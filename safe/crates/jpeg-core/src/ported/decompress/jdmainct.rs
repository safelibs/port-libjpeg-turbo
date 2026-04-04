use ffi_types::{boolean, j_decompress_ptr, J_MESSAGE_CODE};

use crate::common::error;

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
    include!("generated/jdmainct_translated.rs");
}

pub const CTX_PREPARE_FOR_IMCU: i32 = translated::CTX_PREPARE_FOR_IMCU;
pub const CTX_PROCESS_IMCU: i32 = 1;
pub const CTX_POSTPONED_ROW: i32 = 2;

pub use translated::my_main_controller;

#[inline]
unsafe fn translated_cinfo(cinfo: j_decompress_ptr) -> *mut translated::jpeg_decompress_struct {
    cinfo.cast::<translated::jpeg_decompress_struct>()
}

#[inline]
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

pub unsafe fn jinit_d_main_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean) {
    if (*cinfo).min_DCT_v_scaled_size < 1 {
        error::errexit1(
            cinfo as _,
            J_MESSAGE_CODE::JERR_BAD_DCTSIZE,
            (*cinfo).min_DCT_v_scaled_size,
        );
    }
    translated::jinit_d_main_controller(translated_cinfo(cinfo), need_full_buffer)
}
