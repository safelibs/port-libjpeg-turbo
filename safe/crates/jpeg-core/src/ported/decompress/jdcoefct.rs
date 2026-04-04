use ffi_types::{boolean, j_decompress_ptr};

#[allow(
    dead_code,
    improper_ctypes,
    improper_ctypes_definitions,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    clippy::all
)]
mod translated {
    include!("generated/jdcoefct_translated.rs");
}

pub use translated::my_coef_controller;

#[inline]
unsafe fn translated_cinfo(cinfo: j_decompress_ptr) -> *mut translated::jpeg_decompress_struct {
    cinfo.cast::<translated::jpeg_decompress_struct>()
}

#[inline]
pub unsafe fn start_iMCU_row(cinfo: j_decompress_ptr) {
    let coef = (*cinfo).coef as *mut my_coef_controller;
    if (*cinfo).comps_in_scan > 1 {
        (*coef).MCU_rows_per_iMCU_row = 1;
    } else if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows - 1 {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0]).v_samp_factor;
    } else {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0]).last_row_height;
    }

    (*coef).MCU_ctr = 0;
    (*coef).MCU_vert_offset = 0;
}

pub unsafe fn jinit_d_coef_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean) {
    translated::jinit_d_coef_controller(translated_cinfo(cinfo), need_full_buffer)
}
