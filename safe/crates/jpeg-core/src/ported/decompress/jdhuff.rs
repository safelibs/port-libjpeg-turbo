use ffi_types::j_decompress_ptr;

extern "C" {
    pub fn jinit_huff_decoder(cinfo: j_decompress_ptr);
}
