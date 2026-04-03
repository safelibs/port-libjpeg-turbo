use ffi_types::{j_decompress_ptr, jpeg_component_info, JCOEFPTR, JSAMPARRAY, JDIMENSION};

extern "C" {
    #[link_name = "jpeg_idct_ifast"]
    fn c_jpeg_idct_ifast(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    );
}

pub unsafe extern "C" fn jpeg_idct_ifast(
    cinfo: j_decompress_ptr,
    compptr: *mut jpeg_component_info,
    coef_block: JCOEFPTR,
    output_buf: JSAMPARRAY,
    output_col: JDIMENSION,
) {
    c_jpeg_idct_ifast(cinfo, compptr, coef_block, output_buf, output_col)
}
