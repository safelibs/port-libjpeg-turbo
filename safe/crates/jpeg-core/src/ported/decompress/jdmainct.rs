use ffi_types::{
    boolean, j_decompress_ptr, jpeg_d_main_controller, jpeg_component_info, JSAMPARRAY,
    JSAMPIMAGE, J_MESSAGE_CODE, JDIMENSION, MAX_COMPONENTS,
};

use crate::common::error;

pub const CTX_PREPARE_FOR_IMCU: i32 = 0;
pub const CTX_PROCESS_IMCU: i32 = 1;
pub const CTX_POSTPONED_ROW: i32 = 2;

#[repr(C)]
pub struct my_main_controller {
    pub pub_: jpeg_d_main_controller,
    pub buffer: [JSAMPARRAY; MAX_COMPONENTS],
    pub buffer_full: boolean,
    pub rowgroup_ctr: JDIMENSION,
    pub xbuffer: [JSAMPIMAGE; 2],
    pub whichptr: i32,
    pub context_state: i32,
    pub rowgroups_avail: JDIMENSION,
    pub iMCU_row_ctr: JDIMENSION,
}

pub unsafe fn set_wraparound_pointers(cinfo: j_decompress_ptr) {
    let main_ptr = (*cinfo).main as *mut my_main_controller;
    let m = (*cinfo).min_DCT_v_scaled_size as isize;

    for ci in 0..(*cinfo).num_components as usize {
        let compptr = (*cinfo).comp_info.add(ci);
        let rgroup = ((*compptr).v_samp_factor * (*compptr).DCT_v_scaled_size)
            / (*cinfo).min_DCT_v_scaled_size;
        let xbuf0 = *(*main_ptr).xbuffer[0].add(ci);
        let xbuf1 = *(*main_ptr).xbuffer[1].add(ci);
        for i in 0..rgroup as isize {
            *xbuf0.offset(i - rgroup as isize) = *xbuf0.offset(rgroup as isize * (m + 1) + i);
            *xbuf1.offset(i - rgroup as isize) = *xbuf1.offset(rgroup as isize * (m + 1) + i);
            *xbuf0.offset(rgroup as isize * (m + 2) + i) = *xbuf0.offset(i);
            *xbuf1.offset(rgroup as isize * (m + 2) + i) = *xbuf1.offset(i);
        }
    }
}

extern "C" {
    #[link_name = "jinit_d_main_controller"]
    fn c_jinit_d_main_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean);
}

pub unsafe fn jinit_d_main_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean) {
    if (*cinfo).min_DCT_v_scaled_size < 1 {
        error::errexit1(
            cinfo as _,
            J_MESSAGE_CODE::JERR_BAD_DCTSIZE,
            (*cinfo).min_DCT_v_scaled_size,
        );
    }
    c_jinit_d_main_controller(cinfo, need_full_buffer)
}
