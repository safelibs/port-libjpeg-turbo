use ffi_types::j_decompress_ptr;

pub use crate::ported::decompress::jdcolext::RGB_PIXELSIZE as rgb_pixelsize;

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
    include!("generated/jdcolor_translated.rs");
}

pub use translated::my_color_deconverter;

#[inline]
unsafe fn translated_cinfo(cinfo: j_decompress_ptr) -> *mut translated::jpeg_decompress_struct {
    cinfo.cast::<translated::jpeg_decompress_struct>()
}

pub unsafe fn jinit_color_deconverter(cinfo: j_decompress_ptr) {
    translated::jinit_color_deconverter(translated_cinfo(cinfo))
}
