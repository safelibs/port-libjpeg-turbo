use core::ffi::c_uint;

use ffi_types::{boolean, int, j_decompress_ptr, jpeg_marker_parser_method};

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
    include!("generated/jdmarker_translated.rs");
}

#[inline]
unsafe fn translated_cinfo(cinfo: j_decompress_ptr) -> *mut translated::jpeg_decompress_struct {
    cinfo.cast::<translated::jpeg_decompress_struct>()
}

pub unsafe fn jinit_marker_reader(cinfo: j_decompress_ptr) {
    translated::jinit_marker_reader(translated_cinfo(cinfo))
}

pub unsafe extern "C" fn jpeg_resync_to_restart(cinfo: j_decompress_ptr, desired: int) -> boolean {
    translated::jpeg_resync_to_restart(translated_cinfo(cinfo), desired)
}

pub unsafe extern "C" fn jpeg_save_markers(
    cinfo: j_decompress_ptr,
    marker_code: int,
    length_limit: c_uint,
) {
    translated::jpeg_save_markers(translated_cinfo(cinfo), marker_code, length_limit)
}

pub unsafe extern "C" fn jpeg_set_marker_processor(
    cinfo: j_decompress_ptr,
    marker_code: int,
    routine: jpeg_marker_parser_method,
) {
    translated::jpeg_set_marker_processor(
        cinfo.cast::<translated::jpeg_decompress_struct>(),
        marker_code,
        routine.map(|func| unsafe {
            core::mem::transmute::<
                unsafe extern "C" fn(j_decompress_ptr) -> boolean,
                unsafe extern "C" fn(
                    *mut translated::jpeg_decompress_struct,
                ) -> translated::boolean,
            >(func)
        }),
    )
}
