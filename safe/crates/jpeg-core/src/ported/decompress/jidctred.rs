use ffi_types::{j_decompress_ptr, jpeg_component_info, JCOEFPTR, JSAMPARRAY, JDIMENSION};

macro_rules! idct_wrapper {
    ($alias:ident, $rust_name:ident, $link_name:literal) => {
        extern "C" {
            #[link_name = $link_name]
            fn $alias(
                cinfo: j_decompress_ptr,
                compptr: *mut jpeg_component_info,
                coef_block: JCOEFPTR,
                output_buf: JSAMPARRAY,
                output_col: JDIMENSION,
            );
        }

        pub unsafe extern "C" fn $rust_name(
            cinfo: j_decompress_ptr,
            compptr: *mut jpeg_component_info,
            coef_block: JCOEFPTR,
            output_buf: JSAMPARRAY,
            output_col: JDIMENSION,
        ) {
            $alias(cinfo, compptr, coef_block, output_buf, output_col)
        }
    };
}

idct_wrapper!(c_jpeg_idct_1x1, jpeg_idct_1x1, "jpeg_idct_1x1");
idct_wrapper!(c_jpeg_idct_2x2, jpeg_idct_2x2, "jpeg_idct_2x2");
idct_wrapper!(c_jpeg_idct_4x4, jpeg_idct_4x4, "jpeg_idct_4x4");
