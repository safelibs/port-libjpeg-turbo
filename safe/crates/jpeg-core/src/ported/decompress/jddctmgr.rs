use ffi_types::j_decompress_ptr;

extern "C" {
    pub fn jinit_inverse_dct(cinfo: j_decompress_ptr);
}
