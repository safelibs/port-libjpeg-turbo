use ffi_types::{boolean, int, j_decompress_ptr, jpeg_color_quantizer, jpeg_decomp_master};

#[repr(C)]
pub struct my_decomp_master {
    pub pub_: jpeg_decomp_master,
    pub pass_number: int,
    pub using_merged_upsample: boolean,
    pub quantizer_1pass: *mut jpeg_color_quantizer,
    pub quantizer_2pass: *mut jpeg_color_quantizer,
}

extern "C" {
    pub fn jinit_master_decompress(cinfo: j_decompress_ptr);
}
