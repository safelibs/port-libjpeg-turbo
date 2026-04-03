use ffi_types::{j_decompress_ptr, jpeg_color_deconverter, JLONG};

pub use crate::ported::decompress::jdcolext::RGB_PIXELSIZE as rgb_pixelsize;

#[repr(C)]
pub struct my_color_deconverter {
    pub pub_: jpeg_color_deconverter,
    pub Cr_r_tab: *mut i32,
    pub Cb_b_tab: *mut i32,
    pub Cr_g_tab: *mut JLONG,
    pub Cb_g_tab: *mut JLONG,
    pub rgb_y_tab: *mut JLONG,
}

extern "C" {
    #[link_name = "jinit_color_deconverter"]
    fn c_jinit_color_deconverter(cinfo: j_decompress_ptr);
}

pub unsafe fn jinit_color_deconverter(cinfo: j_decompress_ptr) {
    c_jinit_color_deconverter(cinfo)
}
