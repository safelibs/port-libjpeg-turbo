use ffi_types::{j_decompress_ptr, jpeg_component_info, JCOEFPTR, JDIMENSION, JSAMPARRAY};

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
    include!("generated/jidctfst_translated.rs");
}

#[inline]
unsafe fn translated_cinfo(cinfo: j_decompress_ptr) -> *mut translated::jpeg_decompress_struct {
    cinfo.cast::<translated::jpeg_decompress_struct>()
}

#[inline]
unsafe fn translated_compptr(
    compptr: *mut jpeg_component_info,
) -> *mut translated::jpeg_component_info {
    compptr.cast::<translated::jpeg_component_info>()
}

#[inline]
unsafe fn translated_coef_block(coef_block: JCOEFPTR) -> *mut translated::JCOEF {
    coef_block.cast::<translated::JCOEF>()
}

#[inline]
unsafe fn translated_output_buf(output_buf: JSAMPARRAY) -> *mut translated::JSAMPROW {
    output_buf.cast::<translated::JSAMPROW>()
}

pub unsafe extern "C" fn jpeg_idct_ifast(
    cinfo: j_decompress_ptr,
    compptr: *mut jpeg_component_info,
    coef_block: JCOEFPTR,
    output_buf: JSAMPARRAY,
    output_col: JDIMENSION,
) {
    translated::jpeg_idct_ifast(
        translated_cinfo(cinfo),
        translated_compptr(compptr),
        translated_coef_block(coef_block),
        translated_output_buf(output_buf),
        output_col,
    )
}
