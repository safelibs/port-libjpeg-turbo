use core::ptr;

use ffi_types::{
    boolean, int, j_common_ptr, j_decompress_ptr, long, JSAMPARRAY, JSAMPIMAGE, JDIMENSION,
    DSTATE_PRESCAN, DSTATE_PRELOAD, DSTATE_RAW_OK, DSTATE_READY, DSTATE_SCANNING,
    JPEG_REACHED_EOI, JPEG_REACHED_SOS, JPEG_ROW_COMPLETED, JPEG_SUSPENDED, J_MESSAGE_CODE,
    TRUE, FALSE,
};

use crate::{
    common::error,
    ported::decompress::jdmaster,
};

extern "C" {
    fn jpeg_orig_crop_scanline(
        cinfo: j_decompress_ptr,
        xoffset: *mut JDIMENSION,
        width: *mut JDIMENSION,
    );
    fn jpeg_orig_skip_scanlines(cinfo: j_decompress_ptr, num_lines: JDIMENSION) -> JDIMENSION;
    fn jpeg_orig_read_raw_data(
        cinfo: j_decompress_ptr,
        data: JSAMPIMAGE,
        max_lines: JDIMENSION,
    ) -> JDIMENSION;
    fn jpeg_orig_start_output(cinfo: j_decompress_ptr, scan_number: int) -> boolean;
    fn jpeg_orig_finish_output(cinfo: j_decompress_ptr) -> boolean;
}

unsafe fn output_pass_setup(cinfo: j_decompress_ptr) -> boolean {
    if (*cinfo).global_state != DSTATE_PRESCAN {
        (*(*cinfo).master).prepare_for_output_pass.unwrap()(cinfo);
        (*cinfo).output_scanline = 0;
        (*cinfo).global_state = DSTATE_PRESCAN;
    }

    while (*(*cinfo).master).is_dummy_pass != FALSE {
        while (*cinfo).output_scanline < (*cinfo).output_height {
            if !(*cinfo).progress.is_null() {
                (*(*cinfo).progress).pass_counter = (*cinfo).output_scanline as long;
                (*(*cinfo).progress).pass_limit = (*cinfo).output_height as long;
                (*(*cinfo).progress).progress_monitor.unwrap()(cinfo as j_common_ptr);
            }

            let last_scanline = (*cinfo).output_scanline;
            (*(*cinfo).main).process_data.unwrap()(cinfo, ptr::null_mut(), &mut (*cinfo).output_scanline, 0);
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
            (*cinfo).global_state = ffi_types::DSTATE_BUFIMAGE;
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

    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).pass_counter = (*cinfo).output_scanline as long;
        (*(*cinfo).progress).pass_limit = (*cinfo).output_height as long;
        (*(*cinfo).progress).progress_monitor.unwrap()(cinfo as j_common_ptr);
    }

    let mut row_ctr = 0;
    (*(*cinfo).main).process_data.unwrap()(cinfo, scanlines, &mut row_ctr, max_lines);
    (*cinfo).output_scanline += row_ctr;
    row_ctr
}

pub unsafe fn jpeg_crop_scanline(
    cinfo: j_decompress_ptr,
    xoffset: *mut JDIMENSION,
    width: *mut JDIMENSION,
) {
    jpeg_orig_crop_scanline(cinfo, xoffset, width);
}

pub unsafe fn jpeg_skip_scanlines(cinfo: j_decompress_ptr, num_lines: JDIMENSION) -> JDIMENSION {
    jpeg_orig_skip_scanlines(cinfo, num_lines)
}

pub unsafe fn jpeg_read_raw_data(
    cinfo: j_decompress_ptr,
    data: JSAMPIMAGE,
    max_lines: JDIMENSION,
) -> JDIMENSION {
    jpeg_orig_read_raw_data(cinfo, data, max_lines)
}

pub unsafe fn jpeg_start_output(cinfo: j_decompress_ptr, scan_number: int) -> boolean {
    jpeg_orig_start_output(cinfo, scan_number)
}

pub unsafe fn jpeg_finish_output(cinfo: j_decompress_ptr) -> boolean {
    jpeg_orig_finish_output(cinfo)
}
