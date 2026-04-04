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
    include!("generated/jdpostct_translated.rs");
}

#[inline]
unsafe fn translated_cinfo(cinfo: j_decompress_ptr) -> *mut translated::jpeg_decompress_struct {
    cinfo.cast::<translated::jpeg_decompress_struct>()
}

pub unsafe fn jinit_d_post_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean) {
    translated::jinit_d_post_controller(translated_cinfo(cinfo), need_full_buffer)
}
