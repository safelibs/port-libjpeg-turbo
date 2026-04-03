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

idct_wrapper!(c_jpeg_idct_islow, jpeg_idct_islow, "jpeg_idct_islow");
idct_wrapper!(c_jpeg_idct_3x3, jpeg_idct_3x3, "jpeg_idct_3x3");
idct_wrapper!(c_jpeg_idct_5x5, jpeg_idct_5x5, "jpeg_idct_5x5");
idct_wrapper!(c_jpeg_idct_6x6, jpeg_idct_6x6, "jpeg_idct_6x6");
idct_wrapper!(c_jpeg_idct_7x7, jpeg_idct_7x7, "jpeg_idct_7x7");
idct_wrapper!(c_jpeg_idct_9x9, jpeg_idct_9x9, "jpeg_idct_9x9");
idct_wrapper!(c_jpeg_idct_10x10, jpeg_idct_10x10, "jpeg_idct_10x10");
idct_wrapper!(c_jpeg_idct_11x11, jpeg_idct_11x11, "jpeg_idct_11x11");
idct_wrapper!(c_jpeg_idct_12x12, jpeg_idct_12x12, "jpeg_idct_12x12");
idct_wrapper!(c_jpeg_idct_13x13, jpeg_idct_13x13, "jpeg_idct_13x13");
idct_wrapper!(c_jpeg_idct_14x14, jpeg_idct_14x14, "jpeg_idct_14x14");
idct_wrapper!(c_jpeg_idct_15x15, jpeg_idct_15x15, "jpeg_idct_15x15");
idct_wrapper!(c_jpeg_idct_16x16, jpeg_idct_16x16, "jpeg_idct_16x16");
