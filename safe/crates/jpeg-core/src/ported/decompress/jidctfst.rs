use ffi_types::{j_decompress_ptr, jpeg_component_info, JCOEFPTR, JSAMPARRAY, JDIMENSION};

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

pub unsafe extern "C" fn jpeg_idct_ifast(
    cinfo: j_decompress_ptr,
    compptr: *mut jpeg_component_info,
    coef_block: JCOEFPTR,
    output_buf: JSAMPARRAY,
    output_col: JDIMENSION,
) {
    translated::jpeg_idct_ifast(
        cinfo.cast::<translated::jpeg_decompress_struct>(),
        compptr.cast::<translated::jpeg_component_info>(),
        coef_block.cast::<translated::JCOEF>(),
        output_buf.cast::<translated::JSAMPROW>(),
        output_col,
    )
}
