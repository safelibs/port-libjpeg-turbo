use ffi_types::j_decompress_ptr;

extern "C" {
    pub fn jinit_merged_upsampler(cinfo: j_decompress_ptr);
}
