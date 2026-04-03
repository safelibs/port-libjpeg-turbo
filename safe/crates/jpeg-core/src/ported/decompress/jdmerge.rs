use ffi_types::{boolean, j_decompress_ptr, jpeg_upsampler, JLONG, JSAMPARRAY, JSAMPROW, JSAMPIMAGE, JDIMENSION};

pub type merged_upsample_ptr = Option<
    unsafe extern "C" fn(
        cinfo: j_decompress_ptr,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    ),
>;

#[repr(C)]
pub struct my_merged_upsampler {
    pub pub_: jpeg_upsampler,
    pub upmethod: merged_upsample_ptr,
    pub Cr_r_tab: *mut i32,
    pub Cb_b_tab: *mut i32,
    pub Cr_g_tab: *mut JLONG,
    pub Cb_g_tab: *mut JLONG,
    pub spare_row: JSAMPROW,
    pub spare_full: boolean,
    pub out_row_width: JDIMENSION,
    pub rows_to_go: JDIMENSION,
}

extern "C" {
    #[link_name = "jinit_merged_upsampler"]
    fn c_jinit_merged_upsampler(cinfo: j_decompress_ptr);
}

pub unsafe fn jinit_merged_upsampler(cinfo: j_decompress_ptr) {
    c_jinit_merged_upsampler(cinfo)
}
