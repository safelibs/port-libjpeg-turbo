use core::{mem::size_of, ptr};

use ffi_types::{
    boolean, int, j_common_ptr, j_decompress_ptr, jpeg_input_controller, DCTSIZE, DCTSIZE2,
    D_MAX_BLOCKS_IN_MCU, FALSE, JPEG_MAX_DIMENSION, JPEG_REACHED_EOI, JPEG_REACHED_SOS,
    JPEG_SUSPENDED, JPOOL_IMAGE, JPOOL_PERMANENT, JQUANT_TBL, J_MESSAGE_CODE, MAX_COMPONENTS,
    MAX_COMPS_IN_SCAN, MAX_SAMP_FACTOR, NUM_QUANT_TBLS, TRUE,
};

use crate::common::{error, registry, utils};

const BITS_IN_JSAMPLE: int = 8;

#[repr(C)]
pub struct my_input_controller {
    pub pub_: jpeg_input_controller,
    pub inheaders: boolean,
}

extern "C" {
    #[link_name = "jpeg_natural_order"]
    static c_jpeg_natural_order: [int; DCTSIZE2 + 16];
}

#[inline]
unsafe fn detect_multiple_scans(cinfo: j_decompress_ptr) -> boolean {
    if (*cinfo).comps_in_scan < (*cinfo).num_components || (*cinfo).progressive_mode != FALSE {
        TRUE
    } else {
        FALSE
    }
}

unsafe fn initial_setup(cinfo: j_decompress_ptr) {
    if (*cinfo).image_height as ffi_types::long > JPEG_MAX_DIMENSION
        || (*cinfo).image_width as ffi_types::long > JPEG_MAX_DIMENSION
    {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_IMAGE_TOO_BIG,
            JPEG_MAX_DIMENSION as int,
        );
    }

    if (*cinfo).data_precision != BITS_IN_JSAMPLE {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            (*cinfo).data_precision,
        );
    }

    if (*cinfo).num_components > MAX_COMPONENTS as int {
        error::errexit2(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_COMPONENT_COUNT,
            (*cinfo).num_components,
            MAX_COMPONENTS as int,
        );
    }

    (*cinfo).max_h_samp_factor = 1;
    (*cinfo).max_v_samp_factor = 1;
    for ci in 0..(*cinfo).num_components as usize {
        let compptr = (*cinfo).comp_info.add(ci);
        if (*compptr).h_samp_factor <= 0
            || (*compptr).h_samp_factor > MAX_SAMP_FACTOR as int
            || (*compptr).v_samp_factor <= 0
            || (*compptr).v_samp_factor > MAX_SAMP_FACTOR as int
        {
            error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_BAD_SAMPLING);
        }
        (*cinfo).max_h_samp_factor = (*cinfo).max_h_samp_factor.max((*compptr).h_samp_factor);
        (*cinfo).max_v_samp_factor = (*cinfo).max_v_samp_factor.max((*compptr).v_samp_factor);
    }

    (*cinfo).block_size = DCTSIZE as int;
    (*cinfo).natural_order = c_jpeg_natural_order.as_ptr();
    (*cinfo).lim_Se = DCTSIZE2 as int - 1;
    (*cinfo).min_DCT_h_scaled_size = DCTSIZE as int;
    (*cinfo).min_DCT_v_scaled_size = DCTSIZE as int;

    for ci in 0..(*cinfo).num_components as usize {
        let compptr = (*cinfo).comp_info.add(ci);
        (*compptr).DCT_h_scaled_size = DCTSIZE as int;
        (*compptr).DCT_v_scaled_size = DCTSIZE as int;
        (*compptr).width_in_blocks = utils::div_round_up(
            (*cinfo).image_width as ffi_types::long * (*compptr).h_samp_factor as ffi_types::long,
            ((*cinfo).max_h_samp_factor as usize * DCTSIZE) as ffi_types::long,
        ) as _;
        (*compptr).height_in_blocks = utils::div_round_up(
            (*cinfo).image_height as ffi_types::long * (*compptr).v_samp_factor as ffi_types::long,
            ((*cinfo).max_v_samp_factor as usize * DCTSIZE) as ffi_types::long,
        ) as _;
        (*(*cinfo).master).first_MCU_col[ci] = 0;
        (*(*cinfo).master).last_MCU_col[ci] = (*compptr).width_in_blocks - 1;
        (*compptr).downsampled_width = utils::div_round_up(
            (*cinfo).image_width as ffi_types::long * (*compptr).h_samp_factor as ffi_types::long,
            (*cinfo).max_h_samp_factor as ffi_types::long,
        ) as _;
        (*compptr).downsampled_height = utils::div_round_up(
            (*cinfo).image_height as ffi_types::long * (*compptr).v_samp_factor as ffi_types::long,
            (*cinfo).max_v_samp_factor as ffi_types::long,
        ) as _;
        (*compptr).component_needed = TRUE;
        (*compptr).quant_table = ptr::null_mut();
    }

    (*cinfo).total_iMCU_rows = utils::div_round_up(
        (*cinfo).image_height as ffi_types::long,
        ((*cinfo).max_v_samp_factor as usize * DCTSIZE) as ffi_types::long,
    ) as _;

    (*(*cinfo).inputctl).has_multiple_scans = detect_multiple_scans(cinfo);
}

unsafe fn per_scan_setup(cinfo: j_decompress_ptr) {
    if (*cinfo).comps_in_scan == 1 {
        let compptr = (*cinfo).cur_comp_info[0];
        (*cinfo).MCUs_per_row = (*compptr).width_in_blocks;
        (*cinfo).MCU_rows_in_scan = (*compptr).height_in_blocks;
        (*compptr).MCU_width = 1;
        (*compptr).MCU_height = 1;
        (*compptr).MCU_blocks = 1;
        (*compptr).MCU_sample_width = (*compptr).DCT_h_scaled_size;
        (*compptr).last_col_width = 1;
        let mut tmp = ((*compptr).height_in_blocks % (*compptr).v_samp_factor as u32) as int;
        if tmp == 0 {
            tmp = (*compptr).v_samp_factor;
        }
        (*compptr).last_row_height = tmp;
        (*cinfo).blocks_in_MCU = 1;
        (*cinfo).MCU_membership[0] = 0;
        return;
    }

    if (*cinfo).comps_in_scan <= 0 || (*cinfo).comps_in_scan > MAX_COMPS_IN_SCAN as int {
        error::errexit2(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_COMPONENT_COUNT,
            (*cinfo).comps_in_scan,
            MAX_COMPS_IN_SCAN as int,
        );
    }

    (*cinfo).MCUs_per_row = utils::div_round_up(
        (*cinfo).image_width as ffi_types::long,
        ((*cinfo).max_h_samp_factor as usize * DCTSIZE) as ffi_types::long,
    ) as _;
    (*cinfo).MCU_rows_in_scan = utils::div_round_up(
        (*cinfo).image_height as ffi_types::long,
        ((*cinfo).max_v_samp_factor as usize * DCTSIZE) as ffi_types::long,
    ) as _;
    (*cinfo).blocks_in_MCU = 0;

    for ci in 0..(*cinfo).comps_in_scan as usize {
        let compptr = (*cinfo).cur_comp_info[ci];
        (*compptr).MCU_width = (*compptr).h_samp_factor;
        (*compptr).MCU_height = (*compptr).v_samp_factor;
        (*compptr).MCU_blocks = (*compptr).MCU_width * (*compptr).MCU_height;
        (*compptr).MCU_sample_width = (*compptr).MCU_width * (*compptr).DCT_h_scaled_size;

        let mut tmp = ((*compptr).width_in_blocks % (*compptr).MCU_width as u32) as int;
        if tmp == 0 {
            tmp = (*compptr).MCU_width;
        }
        (*compptr).last_col_width = tmp;
        tmp = ((*compptr).height_in_blocks % (*compptr).MCU_height as u32) as int;
        if tmp == 0 {
            tmp = (*compptr).MCU_height;
        }
        (*compptr).last_row_height = tmp;

        let mut mcublks = (*compptr).MCU_blocks;
        if (*cinfo).blocks_in_MCU + mcublks > D_MAX_BLOCKS_IN_MCU as int {
            error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_BAD_MCU_SIZE);
        }
        while mcublks > 0 {
            (*cinfo).MCU_membership[(*cinfo).blocks_in_MCU as usize] = ci as int;
            (*cinfo).blocks_in_MCU += 1;
            mcublks -= 1;
        }
    }
}

unsafe fn latch_quant_tables(cinfo: j_decompress_ptr) {
    for ci in 0..(*cinfo).comps_in_scan as usize {
        let compptr = (*cinfo).cur_comp_info[ci];
        if !(*compptr).quant_table.is_null() {
            continue;
        }
        let qtblno = (*compptr).quant_tbl_no;
        if qtblno < 0
            || qtblno >= NUM_QUANT_TBLS as int
            || (*cinfo).quant_tbl_ptrs[qtblno as usize].is_null()
        {
            error::errexit1(
                cinfo as j_common_ptr,
                J_MESSAGE_CODE::JERR_NO_QUANT_TABLE,
                qtblno,
            );
        }
        let qtbl = (*(*cinfo).mem).alloc_small.unwrap()(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            size_of::<JQUANT_TBL>(),
        ) as *mut JQUANT_TBL;
        ptr::copy_nonoverlapping((*cinfo).quant_tbl_ptrs[qtblno as usize], qtbl, 1);
        (*compptr).quant_table = qtbl;
    }
}

unsafe extern "C" fn start_input_pass(cinfo: j_decompress_ptr) {
    per_scan_setup(cinfo);
    latch_quant_tables(cinfo);
    (*(*cinfo).entropy).start_pass.unwrap()(cinfo);
    (*(*cinfo).coef).start_input_pass.unwrap()(cinfo);
    (*(*cinfo).inputctl).consume_input = (*(*cinfo).coef).consume_data;
}

unsafe extern "C" fn finish_input_pass(cinfo: j_decompress_ptr) {
    (*(*cinfo).inputctl).consume_input = Some(consume_markers);
}

unsafe extern "C" fn consume_markers(cinfo: j_decompress_ptr) -> int {
    let inputctl = (*cinfo).inputctl as *mut my_input_controller;

    if (*(*cinfo).inputctl).eoi_reached != FALSE {
        return JPEG_REACHED_EOI;
    }

    let val = (*(*cinfo).marker).read_markers.unwrap()(cinfo);
    match val {
        JPEG_REACHED_SOS => {
            if (*inputctl).inheaders != FALSE {
                initial_setup(cinfo);
                (*inputctl).inheaders = FALSE;
            } else {
                if (*(*cinfo).inputctl).has_multiple_scans == FALSE {
                    error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_EOI_EXPECTED);
                }
                start_input_pass(cinfo);
            }
            if let Some(limit) = registry::decompress_scan_limit_exceeded(cinfo) {
                error::errexit_scan_limit(cinfo as j_common_ptr, limit);
            }
        }
        JPEG_REACHED_EOI => {
            (*(*cinfo).inputctl).eoi_reached = TRUE;
            if (*inputctl).inheaders != FALSE {
                if (*(*cinfo).marker).saw_SOF != FALSE {
                    error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_SOF_NO_SOS);
                }
            } else if (*cinfo).output_scan_number > (*cinfo).input_scan_number {
                (*cinfo).output_scan_number = (*cinfo).input_scan_number;
            }
        }
        JPEG_SUSPENDED => {}
        _ => {}
    }

    val
}

unsafe extern "C" fn reset_input_controller(cinfo: j_decompress_ptr) {
    let inputctl = (*cinfo).inputctl as *mut my_input_controller;
    (*inputctl).pub_.consume_input = Some(consume_markers);
    (*inputctl).pub_.has_multiple_scans = FALSE;
    (*inputctl).pub_.eoi_reached = FALSE;
    (*inputctl).inheaders = TRUE;
    (*(*cinfo).err).reset_error_mgr.unwrap()(cinfo as j_common_ptr);
    (*(*cinfo).marker).reset_marker_reader.unwrap()(cinfo);
    (*cinfo).coef_bits = ptr::null_mut();
}

pub unsafe fn jinit_input_controller(cinfo: j_decompress_ptr) {
    let alloc_small = (*(*cinfo).mem).alloc_small.unwrap();
    let inputctl = alloc_small(
        cinfo as j_common_ptr,
        JPOOL_PERMANENT,
        size_of::<my_input_controller>(),
    ) as *mut my_input_controller;
    ptr::write_bytes(inputctl, 0, 1);
    (*cinfo).inputctl = &mut (*inputctl).pub_;
    (*inputctl).pub_.consume_input = Some(consume_markers);
    (*inputctl).pub_.reset_input_controller = Some(reset_input_controller);
    (*inputctl).pub_.start_input_pass = Some(start_input_pass);
    (*inputctl).pub_.finish_input_pass = Some(finish_input_pass);
    (*inputctl).pub_.has_multiple_scans = FALSE;
    (*inputctl).pub_.eoi_reached = FALSE;
    (*inputctl).inheaders = TRUE;
}
