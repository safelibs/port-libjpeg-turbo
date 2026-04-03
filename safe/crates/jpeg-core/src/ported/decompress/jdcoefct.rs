use ffi_types::{
    boolean, int, j_decompress_ptr, jvirt_barray_ptr, jpeg_d_coef_controller, JBLOCKROW, JCOEF,
    JDIMENSION, D_MAX_BLOCKS_IN_MCU, MAX_COMPONENTS,
};

#[repr(C)]
pub struct my_coef_controller {
    pub pub_: jpeg_d_coef_controller,
    pub MCU_ctr: JDIMENSION,
    pub MCU_vert_offset: int,
    pub MCU_rows_per_iMCU_row: int,
    pub MCU_buffer: [JBLOCKROW; D_MAX_BLOCKS_IN_MCU],
    pub workspace: *mut JCOEF,
    pub whole_image: [jvirt_barray_ptr; MAX_COMPONENTS],
    pub coef_bits_latch: *mut int,
}

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

extern "C" {
    #[link_name = "jinit_d_coef_controller"]
    fn c_jinit_d_coef_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean);
}

pub unsafe fn jinit_d_coef_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean) {
    c_jinit_d_coef_controller(cinfo, need_full_buffer)
}
