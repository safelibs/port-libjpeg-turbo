use ffi_types::j_decompress_ptr;

extern "C" {
    pub fn jinit_input_controller(cinfo: j_decompress_ptr);
}
