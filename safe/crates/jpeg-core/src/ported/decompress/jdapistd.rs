use core::ptr;

use ffi_types::{
    boolean, int, j_common_ptr, j_decompress_ptr, long, DSTATE_BUFIMAGE, DSTATE_BUFPOST,
    DSTATE_PRELOAD, DSTATE_PRESCAN, DSTATE_RAW_OK, DSTATE_READY, DSTATE_SCANNING, FALSE,
    JDIMENSION, JPEG_REACHED_EOI, JPEG_REACHED_SOS, JPEG_ROW_COMPLETED, JPEG_SUSPENDED, JSAMPARRAY,
    JSAMPIMAGE, JSAMPLE, J_MESSAGE_CODE, TRUE,
};

use crate::{
    common::{error, utils},
    ported::decompress::{
        jdcoefct::{self, my_coef_controller},
        jdmainct::{self, my_main_controller, CTX_PREPARE_FOR_IMCU},
        jdmaster::{self, my_decomp_master},
        jdmerge::my_merged_upsampler,
        jdsample::{self, my_upsampler},
    },
};

#[inline]
unsafe fn update_output_progress(
    cinfo: j_decompress_ptr,
    pass_counter: JDIMENSION,
    pass_limit: JDIMENSION,
) {
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).pass_counter = pass_counter as long;
        (*(*cinfo).progress).pass_limit = pass_limit as long;
        (*(*cinfo).progress).progress_monitor.unwrap()(cinfo as j_common_ptr);
    }
}

#[inline]
unsafe fn sync_upsampler_rows_to_go(
    cinfo: j_decompress_ptr,
    master: *mut my_decomp_master,
    upsample: *mut my_upsampler,
) {
    if (*master).using_merged_upsample == FALSE {
        (*upsample).rows_to_go = (*cinfo).output_height - (*cinfo).output_scanline;
    }
}

unsafe fn output_pass_setup(cinfo: j_decompress_ptr) -> boolean {
    if (*cinfo).global_state != DSTATE_PRESCAN {
        (*(*cinfo).master).prepare_for_output_pass.unwrap()(cinfo);
        (*cinfo).output_scanline = 0;
        (*cinfo).global_state = DSTATE_PRESCAN;
    }

    while (*(*cinfo).master).is_dummy_pass != FALSE {
        while (*cinfo).output_scanline < (*cinfo).output_height {
            update_output_progress(cinfo, (*cinfo).output_scanline, (*cinfo).output_height);

            let last_scanline = (*cinfo).output_scanline;
            (*(*cinfo).main).process_data.unwrap()(
                cinfo,
                ptr::null_mut(),
                &mut (*cinfo).output_scanline,
                0,
            );
            if (*cinfo).output_scanline == last_scanline {
                return FALSE;
            }
        }

        (*(*cinfo).master).finish_output_pass.unwrap()(cinfo);
        (*(*cinfo).master).prepare_for_output_pass.unwrap()(cinfo);
        (*cinfo).output_scanline = 0;
    }

    (*cinfo).global_state = if (*cinfo).raw_data_out != FALSE {
        DSTATE_RAW_OK
    } else {
        DSTATE_SCANNING
    };
    TRUE
}

pub unsafe fn jpeg_start_decompress(cinfo: j_decompress_ptr) -> boolean {
    if (*cinfo).global_state == DSTATE_READY {
        jdmaster::jinit_master_decompress(cinfo);
        if (*cinfo).buffered_image != FALSE {
            (*cinfo).global_state = DSTATE_BUFIMAGE;
            return TRUE;
        }
        (*cinfo).global_state = DSTATE_PRELOAD;
    }

    if (*cinfo).global_state == DSTATE_PRELOAD {
        if (*(*cinfo).inputctl).has_multiple_scans != FALSE {
            loop {
                if !(*cinfo).progress.is_null() {
                    (*(*cinfo).progress).progress_monitor.unwrap()(cinfo as j_common_ptr);
                }

                let retcode = (*(*cinfo).inputctl).consume_input.unwrap()(cinfo);
                if retcode == JPEG_SUSPENDED {
                    return FALSE;
                }
                if retcode == JPEG_REACHED_EOI {
                    break;
                }
                if !(*cinfo).progress.is_null()
                    && (retcode == JPEG_ROW_COMPLETED || retcode == JPEG_REACHED_SOS)
                {
                    (*(*cinfo).progress).pass_counter += 1;
                    if (*(*cinfo).progress).pass_counter >= (*(*cinfo).progress).pass_limit {
                        (*(*cinfo).progress).pass_limit += (*cinfo).total_iMCU_rows as long;
                    }
                }
            }
        }
        (*cinfo).output_scan_number = (*cinfo).input_scan_number;
    } else if (*cinfo).global_state != DSTATE_PRESCAN {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }

    output_pass_setup(cinfo)
}

pub unsafe fn jpeg_crop_scanline(
    cinfo: j_decompress_ptr,
    xoffset: *mut JDIMENSION,
    width: *mut JDIMENSION,
) {
    let mut reinit_upsampler = FALSE;
    let master = (*cinfo).master as *mut my_decomp_master;

    if ((*cinfo).global_state != DSTATE_SCANNING && (*cinfo).global_state != DSTATE_BUFIMAGE)
        || (*cinfo).output_scanline != 0
    {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }
    if xoffset.is_null() || width.is_null() {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_BAD_CROP_SPEC);
    }
    if *width == 0 || *xoffset + *width > (*cinfo).output_width {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_WIDTH_OVERFLOW);
    }
    if *width == (*cinfo).output_width {
        return;
    }

    let align = if (*cinfo).comps_in_scan == 1 && (*cinfo).num_components == 1 {
        (*cinfo).min_DCT_h_scaled_size as JDIMENSION
    } else {
        ((*cinfo).min_DCT_h_scaled_size * (*cinfo).max_h_samp_factor) as JDIMENSION
    };
    let input_xoffset = *xoffset;
    *xoffset = (input_xoffset / align) * align;
    *width = *width + input_xoffset - *xoffset;
    (*cinfo).output_width = *width;

    if (*master).using_merged_upsample != FALSE && (*cinfo).max_v_samp_factor == 2 {
        let upsample = (*cinfo).upsample as *mut my_merged_upsampler;
        (*upsample).out_row_width = (*cinfo).output_width * (*cinfo).out_color_components as u32;
    }

    (*(*cinfo).master).first_iMCU_col = *xoffset / align;
    (*(*cinfo).master).last_iMCU_col = utils::div_round_up(
        (*xoffset + (*cinfo).output_width) as ffi_types::long,
        align as ffi_types::long,
    ) as JDIMENSION
        - 1;

    for ci in 0..(*cinfo).num_components as usize {
        let compptr = (*cinfo).comp_info.add(ci);
        let hsf = if (*cinfo).comps_in_scan == 1 && (*cinfo).num_components == 1 {
            1
        } else {
            (*compptr).h_samp_factor
        };
        let orig_downsampled_width = (*compptr).downsampled_width;
        (*compptr).downsampled_width = utils::div_round_up(
            ((*cinfo).output_width * (*compptr).h_samp_factor as u32) as ffi_types::long,
            (*cinfo).max_h_samp_factor as ffi_types::long,
        ) as _;
        if (*compptr).downsampled_width < 2 && orig_downsampled_width >= 2 {
            reinit_upsampler = TRUE;
        }
        (*(*cinfo).master).first_MCU_col[ci] = ((*xoffset * hsf as u32) / align) as JDIMENSION;
        (*(*cinfo).master).last_MCU_col[ci] = utils::div_round_up(
            ((*xoffset + (*cinfo).output_width) * hsf as u32) as ffi_types::long,
            align as ffi_types::long,
        ) as JDIMENSION
            - 1;
    }

    if reinit_upsampler != FALSE {
        (*(*cinfo).master).jinit_upsampler_no_alloc = TRUE;
        jdsample::jinit_upsampler(cinfo);
        (*(*cinfo).master).jinit_upsampler_no_alloc = FALSE;
    }
}

pub unsafe fn jpeg_read_scanlines(
    cinfo: j_decompress_ptr,
    scanlines: JSAMPARRAY,
    max_lines: JDIMENSION,
) -> JDIMENSION {
    if (*cinfo).global_state != DSTATE_SCANNING {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }
    if (*cinfo).output_scanline >= (*cinfo).output_height {
        error::warnms(cinfo as j_common_ptr, J_MESSAGE_CODE::JWRN_TOO_MUCH_DATA);
        return 0;
    }

    update_output_progress(cinfo, (*cinfo).output_scanline, (*cinfo).output_height);

    let mut row_ctr = 0;
    (*(*cinfo).main).process_data.unwrap()(cinfo, scanlines, &mut row_ctr, max_lines);
    (*cinfo).output_scanline += row_ctr;
    row_ctr
}

unsafe extern "C" fn noop_convert(
    _cinfo: j_decompress_ptr,
    _input_buf: JSAMPIMAGE,
    _input_row: JDIMENSION,
    _output_buf: JSAMPARRAY,
    _num_rows: int,
) {
}

unsafe extern "C" fn noop_quantize(
    _cinfo: j_decompress_ptr,
    _input_buf: JSAMPARRAY,
    _output_buf: JSAMPARRAY,
    _num_rows: int,
) {
}

unsafe fn read_and_discard_scanlines(cinfo: j_decompress_ptr, num_lines: JDIMENSION) {
    let master = (*cinfo).master as *mut my_decomp_master;
    let mut dummy_sample = [0 as JSAMPLE; 1];
    let mut dummy_row = dummy_sample.as_mut_ptr();
    let mut scanlines: JSAMPARRAY = ptr::null_mut();
    let mut color_convert = None;
    let mut color_quantize = None;

    if !(*cinfo).cconvert.is_null() && (*(*cinfo).cconvert).color_convert.is_some() {
        color_convert = (*(*cinfo).cconvert).color_convert;
        (*(*cinfo).cconvert).color_convert = Some(noop_convert);
        scanlines = &mut dummy_row;
    }
    if !(*cinfo).cquantize.is_null() && (*(*cinfo).cquantize).color_quantize.is_some() {
        color_quantize = (*(*cinfo).cquantize).color_quantize;
        (*(*cinfo).cquantize).color_quantize = Some(noop_quantize);
    }
    if (*master).using_merged_upsample != FALSE && (*cinfo).max_v_samp_factor == 2 {
        let upsample = (*cinfo).upsample as *mut my_merged_upsampler;
        scanlines = (&mut (*upsample).spare_row as *mut _) as JSAMPARRAY;
    }

    for _ in 0..num_lines {
        jpeg_read_scanlines(cinfo, scanlines, 1);
    }

    if color_convert.is_some() {
        (*(*cinfo).cconvert).color_convert = color_convert;
    }
    if color_quantize.is_some() {
        (*(*cinfo).cquantize).color_quantize = color_quantize;
    }
}

unsafe fn increment_simple_rowgroup_ctr(cinfo: j_decompress_ptr, rows: JDIMENSION) {
    let main_ptr = (*cinfo).main as *mut my_main_controller;
    let master = (*cinfo).master as *mut my_decomp_master;

    if (*master).using_merged_upsample != FALSE && (*cinfo).max_v_samp_factor == 2 {
        read_and_discard_scanlines(cinfo, rows);
        return;
    }

    (*main_ptr).rowgroup_ctr += rows / (*cinfo).max_v_samp_factor as u32;
    let rows_left = rows % (*cinfo).max_v_samp_factor as u32;
    (*cinfo).output_scanline += rows - rows_left;
    read_and_discard_scanlines(cinfo, rows_left);
}

pub unsafe fn jpeg_skip_scanlines(
    cinfo: j_decompress_ptr,
    mut num_lines: JDIMENSION,
) -> JDIMENSION {
    let main_ptr = (*cinfo).main as *mut my_main_controller;
    let coef = (*cinfo).coef as *mut my_coef_controller;
    let master = (*cinfo).master as *mut my_decomp_master;
    let upsample = (*cinfo).upsample as *mut my_upsampler;

    if (*cinfo).quantize_colors != FALSE && (*cinfo).two_pass_quantize != FALSE {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_NOTIMPL);
    }
    if (*cinfo).global_state != DSTATE_SCANNING {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }

    if (*cinfo).output_scanline + num_lines >= (*cinfo).output_height {
        num_lines = (*cinfo).output_height - (*cinfo).output_scanline;
        (*cinfo).output_scanline = (*cinfo).output_height;
        (*(*cinfo).inputctl).finish_input_pass.unwrap()(cinfo);
        (*(*cinfo).inputctl).eoi_reached = TRUE;
        return num_lines;
    }
    if num_lines == 0 {
        return 0;
    }

    let lines_per_iMCU_row =
        ((*cinfo).min_DCT_v_scaled_size * (*cinfo).max_v_samp_factor) as JDIMENSION;
    let lines_left_in_iMCU_row =
        (lines_per_iMCU_row - ((*cinfo).output_scanline % lines_per_iMCU_row)) % lines_per_iMCU_row;
    let mut lines_after_iMCU_row = num_lines - lines_left_in_iMCU_row;

    if (*(*cinfo).upsample).need_context_rows != FALSE {
        if (num_lines < lines_left_in_iMCU_row + 1)
            || (lines_left_in_iMCU_row <= 1
                && (*main_ptr).buffer_full != FALSE
                && lines_after_iMCU_row < lines_per_iMCU_row + 1)
        {
            read_and_discard_scanlines(cinfo, num_lines);
            return num_lines;
        }

        if lines_left_in_iMCU_row <= 1 && (*main_ptr).buffer_full != FALSE {
            (*cinfo).output_scanline += lines_left_in_iMCU_row + lines_per_iMCU_row;
            lines_after_iMCU_row -= lines_per_iMCU_row;
        } else {
            (*cinfo).output_scanline += lines_left_in_iMCU_row;
        }

        if (*main_ptr).iMCU_row_ctr == 0
            || ((*main_ptr).iMCU_row_ctr == 1 && lines_left_in_iMCU_row > 2)
        {
            jdmainct::set_wraparound_pointers(cinfo);
        }
        (*main_ptr).buffer_full = FALSE;
        (*main_ptr).rowgroup_ctr = 0;
        (*main_ptr).context_state = CTX_PREPARE_FOR_IMCU;
        if (*master).using_merged_upsample == FALSE {
            (*upsample).next_row_out = (*cinfo).max_v_samp_factor;
        }
        sync_upsampler_rows_to_go(cinfo, master, upsample);
    } else if num_lines < lines_left_in_iMCU_row {
        increment_simple_rowgroup_ctr(cinfo, num_lines);
        return num_lines;
    } else {
        (*cinfo).output_scanline += lines_left_in_iMCU_row;
        (*main_ptr).buffer_full = FALSE;
        (*main_ptr).rowgroup_ctr = 0;
        if (*master).using_merged_upsample == FALSE {
            (*upsample).next_row_out = (*cinfo).max_v_samp_factor;
        }
        sync_upsampler_rows_to_go(cinfo, master, upsample);
    }

    let lines_to_skip = if (*(*cinfo).upsample).need_context_rows != FALSE {
        ((lines_after_iMCU_row - 1) / lines_per_iMCU_row) * lines_per_iMCU_row
    } else {
        (lines_after_iMCU_row / lines_per_iMCU_row) * lines_per_iMCU_row
    };
    let lines_to_read = lines_after_iMCU_row - lines_to_skip;

    if (*(*cinfo).inputctl).has_multiple_scans != FALSE || (*cinfo).buffered_image != FALSE {
        if (*(*cinfo).upsample).need_context_rows != FALSE {
            (*cinfo).output_scanline += lines_to_skip;
            (*cinfo).output_iMCU_row += lines_to_skip / lines_per_iMCU_row;
            (*main_ptr).iMCU_row_ctr += lines_to_skip / lines_per_iMCU_row;
            read_and_discard_scanlines(cinfo, lines_to_read);
        } else {
            (*cinfo).output_scanline += lines_to_skip;
            (*cinfo).output_iMCU_row += lines_to_skip / lines_per_iMCU_row;
            increment_simple_rowgroup_ctr(cinfo, lines_to_read);
        }
        sync_upsampler_rows_to_go(cinfo, master, upsample);
        return num_lines;
    }

    for _ in (0..lines_to_skip).step_by(lines_per_iMCU_row as usize) {
        for _ in 0..(*coef).MCU_rows_per_iMCU_row {
            for _ in 0..(*cinfo).MCUs_per_row {
                if (*(*cinfo).entropy).insufficient_data == FALSE {
                    (*(*cinfo).master).last_good_iMCU_row = (*cinfo).input_iMCU_row;
                }
                (*(*cinfo).entropy).decode_mcu.unwrap()(cinfo, ptr::null_mut());
            }
        }
        (*cinfo).input_iMCU_row += 1;
        (*cinfo).output_iMCU_row += 1;
        if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows {
            jdcoefct::start_iMCU_row(cinfo);
        } else {
            (*(*cinfo).inputctl).finish_input_pass.unwrap()(cinfo);
        }
    }
    (*cinfo).output_scanline += lines_to_skip;

    if (*(*cinfo).upsample).need_context_rows != FALSE {
        (*main_ptr).iMCU_row_ctr += lines_to_skip / lines_per_iMCU_row;
        read_and_discard_scanlines(cinfo, lines_to_read);
    } else {
        increment_simple_rowgroup_ctr(cinfo, lines_to_read);
    }

    sync_upsampler_rows_to_go(cinfo, master, upsample);

    num_lines
}

pub unsafe fn jpeg_read_raw_data(
    cinfo: j_decompress_ptr,
    data: JSAMPIMAGE,
    max_lines: JDIMENSION,
) -> JDIMENSION {
    if (*cinfo).global_state != DSTATE_RAW_OK {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }
    if (*cinfo).output_scanline >= (*cinfo).output_height {
        error::warnms(cinfo as j_common_ptr, J_MESSAGE_CODE::JWRN_TOO_MUCH_DATA);
        return 0;
    }

    update_output_progress(cinfo, (*cinfo).output_scanline, (*cinfo).output_height);

    let lines_per_iMCU_row =
        ((*cinfo).max_v_samp_factor * (*cinfo).min_DCT_v_scaled_size) as JDIMENSION;
    if max_lines < lines_per_iMCU_row {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_BUFFER_SIZE);
    }

    if (*(*cinfo).coef).decompress_data.unwrap()(cinfo, data) == 0 {
        return 0;
    }

    (*cinfo).output_scanline += lines_per_iMCU_row;
    lines_per_iMCU_row
}

pub unsafe fn jpeg_start_output(cinfo: j_decompress_ptr, mut scan_number: int) -> boolean {
    if (*cinfo).global_state != DSTATE_BUFIMAGE && (*cinfo).global_state != DSTATE_PRESCAN {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }
    if scan_number <= 0 {
        scan_number = 1;
    }
    if (*(*cinfo).inputctl).eoi_reached != FALSE && scan_number > (*cinfo).input_scan_number {
        scan_number = (*cinfo).input_scan_number;
    }
    (*cinfo).output_scan_number = scan_number;
    output_pass_setup(cinfo)
}

pub unsafe fn jpeg_finish_output(cinfo: j_decompress_ptr) -> boolean {
    if ((*cinfo).global_state == DSTATE_SCANNING || (*cinfo).global_state == DSTATE_RAW_OK)
        && (*cinfo).buffered_image != FALSE
    {
        (*(*cinfo).master).finish_output_pass.unwrap()(cinfo);
        (*cinfo).global_state = DSTATE_BUFPOST;
    } else if (*cinfo).global_state != DSTATE_BUFPOST {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }

    while (*cinfo).input_scan_number <= (*cinfo).output_scan_number
        && (*(*cinfo).inputctl).eoi_reached == FALSE
    {
        if (*(*cinfo).inputctl).consume_input.unwrap()(cinfo) == JPEG_SUSPENDED {
            return FALSE;
        }
    }
    (*cinfo).global_state = DSTATE_BUFIMAGE;
    TRUE
}
