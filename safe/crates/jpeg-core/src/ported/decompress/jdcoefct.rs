use ffi_types::{boolean, j_decompress_ptr};

extern "C" {
    pub fn jinit_d_coef_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean);
}
