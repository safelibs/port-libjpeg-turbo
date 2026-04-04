use ffi_types::j_decompress_ptr;

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
    include!("generated/jdmerge_translated.rs");
}

pub use translated::my_merged_upsampler;

#[inline]
unsafe fn translated_cinfo(cinfo: j_decompress_ptr) -> *mut translated::jpeg_decompress_struct {
    cinfo.cast::<translated::jpeg_decompress_struct>()
}

pub unsafe fn jinit_merged_upsampler(cinfo: j_decompress_ptr) {
    translated::jinit_merged_upsampler(translated_cinfo(cinfo))
}
