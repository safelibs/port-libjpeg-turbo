use ffi_types::j_decompress_ptr;

extern "C" {
    pub fn jinit_color_deconverter(cinfo: j_decompress_ptr);
}
