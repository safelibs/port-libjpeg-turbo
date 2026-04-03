use ffi_types::{
    j_decompress_ptr, jpeg_component_info, jpeg_upsampler, JSAMPARRAY, JSAMPIMAGE, JDIMENSION,
    MAX_COMPONENTS, UINT8,
};

pub type upsample1_ptr = Option<
    unsafe extern "C" fn(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    ),
>;

#[repr(C)]
pub struct my_upsampler {
    pub pub_: jpeg_upsampler,
    pub color_buf: [JSAMPARRAY; MAX_COMPONENTS],
    pub methods: [upsample1_ptr; MAX_COMPONENTS],
    pub next_row_out: i32,
    pub rows_to_go: JDIMENSION,
    pub rowgroup_height: [i32; MAX_COMPONENTS],
    pub h_expand: [UINT8; MAX_COMPONENTS],
    pub v_expand: [UINT8; MAX_COMPONENTS],
}

extern "C" {
    #[link_name = "jinit_upsampler"]
    fn c_jinit_upsampler(cinfo: j_decompress_ptr);
}

pub unsafe fn jinit_upsampler(cinfo: j_decompress_ptr) {
    c_jinit_upsampler(cinfo)
}
