#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}

extern "C" {
    fn jcopy_sample_rows(
        input_array: JSAMPARRAY,
        source_row: ::core::ffi::c_int,
        output_array: JSAMPARRAY,
        dest_row: ::core::ffi::c_int,
        num_rows: ::core::ffi::c_int,
        num_cols: JDIMENSION,
    );
    fn jsimd_can_h2v2_merged_upsample() -> ::core::ffi::c_int;
    fn jsimd_can_h2v1_merged_upsample() -> ::core::ffi::c_int;
    fn jsimd_h2v2_merged_upsample(
        cinfo: j_decompress_ptr,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );
    fn jsimd_h2v1_merged_upsample(
        cinfo: j_decompress_ptr,
        input_buf: JSAMPIMAGE,
        in_row_group_ctr: JDIMENSION,
        output_buf: JSAMPARRAY,
    );
}
pub type size_t = usize;
pub type JSAMPLE = ::core::ffi::c_uchar;
pub type JCOEF = ::core::ffi::c_short;
pub type JOCTET = ::core::ffi::c_uchar;
pub type UINT8 = ::core::ffi::c_uchar;
pub type UINT16 = ::core::ffi::c_ushort;
pub type INT16 = ::core::ffi::c_short;
pub type JDIMENSION = ::core::ffi::c_uint;
pub type boolean = ::core::ffi::c_int;
pub type JSAMPROW = *mut JSAMPLE;
pub type JSAMPARRAY = *mut JSAMPROW;
pub type JSAMPIMAGE = *mut JSAMPARRAY;
pub type JBLOCK = [JCOEF; 64];
pub type JBLOCKROW = *mut JBLOCK;
pub type JBLOCKARRAY = *mut JBLOCKROW;
pub type JCOEFPTR = *mut JCOEF;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JQUANT_TBL {
    pub quantval: [UINT16; 64],
    pub sent_table: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JHUFF_TBL {
    pub bits: [UINT8; 17],
    pub huffval: [UINT8; 256],
    pub sent_table: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_component_info {
    pub component_id: ::core::ffi::c_int,
    pub component_index: ::core::ffi::c_int,
    pub h_samp_factor: ::core::ffi::c_int,
    pub v_samp_factor: ::core::ffi::c_int,
    pub quant_tbl_no: ::core::ffi::c_int,
    pub dc_tbl_no: ::core::ffi::c_int,
    pub ac_tbl_no: ::core::ffi::c_int,
    pub width_in_blocks: JDIMENSION,
    pub height_in_blocks: JDIMENSION,
    pub DCT_h_scaled_size: ::core::ffi::c_int,
    pub DCT_v_scaled_size: ::core::ffi::c_int,
    pub downsampled_width: JDIMENSION,
    pub downsampled_height: JDIMENSION,
    pub component_needed: boolean,
    pub MCU_width: ::core::ffi::c_int,
    pub MCU_height: ::core::ffi::c_int,
    pub MCU_blocks: ::core::ffi::c_int,
    pub MCU_sample_width: ::core::ffi::c_int,
    pub last_col_width: ::core::ffi::c_int,
    pub last_row_height: ::core::ffi::c_int,
    pub quant_table: *mut JQUANT_TBL,
    pub dct_table: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_marker_struct {
    pub next: jpeg_saved_marker_ptr,
    pub marker: UINT8,
    pub original_length: ::core::ffi::c_uint,
    pub data_length: ::core::ffi::c_uint,
    pub data: *mut JOCTET,
}
pub type jpeg_saved_marker_ptr = *mut jpeg_marker_struct;
pub type J_COLOR_SPACE = ::core::ffi::c_uint;
pub const JCS_RGB565: J_COLOR_SPACE = 16;
pub const JCS_EXT_ARGB: J_COLOR_SPACE = 15;
pub const JCS_EXT_ABGR: J_COLOR_SPACE = 14;
pub const JCS_EXT_BGRA: J_COLOR_SPACE = 13;
pub const JCS_EXT_RGBA: J_COLOR_SPACE = 12;
pub const JCS_EXT_XRGB: J_COLOR_SPACE = 11;
pub const JCS_EXT_XBGR: J_COLOR_SPACE = 10;
pub const JCS_EXT_BGRX: J_COLOR_SPACE = 9;
pub const JCS_EXT_BGR: J_COLOR_SPACE = 8;
pub const JCS_EXT_RGBX: J_COLOR_SPACE = 7;
pub const JCS_EXT_RGB: J_COLOR_SPACE = 6;
pub const JCS_YCCK: J_COLOR_SPACE = 5;
pub const JCS_CMYK: J_COLOR_SPACE = 4;
pub const JCS_YCbCr: J_COLOR_SPACE = 3;
pub const JCS_RGB: J_COLOR_SPACE = 2;
pub const JCS_GRAYSCALE: J_COLOR_SPACE = 1;
pub const JCS_UNKNOWN: J_COLOR_SPACE = 0;
pub type J_DCT_METHOD = ::core::ffi::c_uint;
pub const JDCT_FLOAT: J_DCT_METHOD = 2;
pub const JDCT_IFAST: J_DCT_METHOD = 1;
pub const JDCT_ISLOW: J_DCT_METHOD = 0;
pub type J_DITHER_MODE = ::core::ffi::c_uint;
pub const JDITHER_FS: J_DITHER_MODE = 2;
pub const JDITHER_ORDERED: J_DITHER_MODE = 1;
pub const JDITHER_NONE: J_DITHER_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_common_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut ::core::ffi::c_void,
    pub is_decompressor: boolean,
    pub global_state: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_progress_mgr {
    pub progress_monitor: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub pass_counter: ::core::ffi::c_long,
    pub pass_limit: ::core::ffi::c_long,
    pub completed_passes: ::core::ffi::c_int,
    pub total_passes: ::core::ffi::c_int,
}
pub type j_common_ptr = *mut jpeg_common_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_memory_mgr {
    pub alloc_small: Option<
        unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int, size_t) -> *mut ::core::ffi::c_void,
    >,
    pub alloc_large: Option<
        unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int, size_t) -> *mut ::core::ffi::c_void,
    >,
    pub alloc_sarray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            JDIMENSION,
            JDIMENSION,
        ) -> JSAMPARRAY,
    >,
    pub alloc_barray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            JDIMENSION,
            JDIMENSION,
        ) -> JBLOCKARRAY,
    >,
    pub request_virt_sarray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            boolean,
            JDIMENSION,
            JDIMENSION,
            JDIMENSION,
        ) -> jvirt_sarray_ptr,
    >,
    pub request_virt_barray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            boolean,
            JDIMENSION,
            JDIMENSION,
            JDIMENSION,
        ) -> jvirt_barray_ptr,
    >,
    pub realize_virt_arrays: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub access_virt_sarray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            jvirt_sarray_ptr,
            JDIMENSION,
            JDIMENSION,
            boolean,
        ) -> JSAMPARRAY,
    >,
    pub access_virt_barray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            jvirt_barray_ptr,
            JDIMENSION,
            JDIMENSION,
            boolean,
        ) -> JBLOCKARRAY,
    >,
    pub free_pool: Option<unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> ()>,
    pub self_destruct: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub max_memory_to_use: ::core::ffi::c_long,
    pub max_alloc_chunk: ::core::ffi::c_long,
}
pub type jvirt_barray_ptr = *mut jvirt_barray_control;
pub type jvirt_sarray_ptr = *mut jvirt_sarray_control;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_error_mgr {
    pub error_exit: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub emit_message: Option<unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> ()>,
    pub output_message: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub format_message: Option<unsafe extern "C" fn(j_common_ptr, *mut ::core::ffi::c_char) -> ()>,
    pub reset_error_mgr: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub msg_code: ::core::ffi::c_int,
    pub msg_parm: C2RustUnnamed,
    pub trace_level: ::core::ffi::c_int,
    pub num_warnings: ::core::ffi::c_long,
    pub jpeg_message_table: *const *const ::core::ffi::c_char,
    pub last_jpeg_message: ::core::ffi::c_int,
    pub addon_message_table: *const *const ::core::ffi::c_char,
    pub first_addon_message: ::core::ffi::c_int,
    pub last_addon_message: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i: [::core::ffi::c_int; 8],
    pub s: [::core::ffi::c_char; 80],
}
pub type J_BUF_MODE = ::core::ffi::c_uint;
pub const JBUF_SAVE_AND_PASS: J_BUF_MODE = 3;
pub const JBUF_CRANK_DEST: J_BUF_MODE = 2;
pub const JBUF_SAVE_SOURCE: J_BUF_MODE = 1;
pub const JBUF_PASS_THRU: J_BUF_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_decompress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut ::core::ffi::c_void,
    pub is_decompressor: boolean,
    pub global_state: ::core::ffi::c_int,
    pub src: *mut jpeg_source_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub num_components: ::core::ffi::c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub out_color_space: J_COLOR_SPACE,
    pub scale_num: ::core::ffi::c_uint,
    pub scale_denom: ::core::ffi::c_uint,
    pub output_gamma: ::core::ffi::c_double,
    pub buffered_image: boolean,
    pub raw_data_out: boolean,
    pub dct_method: J_DCT_METHOD,
    pub do_fancy_upsampling: boolean,
    pub do_block_smoothing: boolean,
    pub quantize_colors: boolean,
    pub dither_mode: J_DITHER_MODE,
    pub two_pass_quantize: boolean,
    pub desired_number_of_colors: ::core::ffi::c_int,
    pub enable_1pass_quant: boolean,
    pub enable_external_quant: boolean,
    pub enable_2pass_quant: boolean,
    pub output_width: JDIMENSION,
    pub output_height: JDIMENSION,
    pub out_color_components: ::core::ffi::c_int,
    pub output_components: ::core::ffi::c_int,
    pub rec_outbuf_height: ::core::ffi::c_int,
    pub actual_number_of_colors: ::core::ffi::c_int,
    pub colormap: JSAMPARRAY,
    pub output_scanline: JDIMENSION,
    pub input_scan_number: ::core::ffi::c_int,
    pub input_iMCU_row: JDIMENSION,
    pub output_scan_number: ::core::ffi::c_int,
    pub output_iMCU_row: JDIMENSION,
    pub coef_bits: *mut [::core::ffi::c_int; 64],
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub data_precision: ::core::ffi::c_int,
    pub comp_info: *mut jpeg_component_info,
    pub is_baseline: boolean,
    pub progressive_mode: boolean,
    pub arith_code: boolean,
    pub arith_dc_L: [UINT8; 16],
    pub arith_dc_U: [UINT8; 16],
    pub arith_ac_K: [UINT8; 16],
    pub restart_interval: ::core::ffi::c_uint,
    pub saw_JFIF_marker: boolean,
    pub JFIF_major_version: UINT8,
    pub JFIF_minor_version: UINT8,
    pub density_unit: UINT8,
    pub X_density: UINT16,
    pub Y_density: UINT16,
    pub saw_Adobe_marker: boolean,
    pub Adobe_transform: UINT8,
    pub CCIR601_sampling: boolean,
    pub marker_list: jpeg_saved_marker_ptr,
    pub max_h_samp_factor: ::core::ffi::c_int,
    pub max_v_samp_factor: ::core::ffi::c_int,
    pub min_DCT_h_scaled_size: ::core::ffi::c_int,
    pub min_DCT_v_scaled_size: ::core::ffi::c_int,
    pub total_iMCU_rows: JDIMENSION,
    pub sample_range_limit: *mut JSAMPLE,
    pub comps_in_scan: ::core::ffi::c_int,
    pub cur_comp_info: [*mut jpeg_component_info; 4],
    pub MCUs_per_row: JDIMENSION,
    pub MCU_rows_in_scan: JDIMENSION,
    pub blocks_in_MCU: ::core::ffi::c_int,
    pub MCU_membership: [::core::ffi::c_int; 10],
    pub Ss: ::core::ffi::c_int,
    pub Se: ::core::ffi::c_int,
    pub Ah: ::core::ffi::c_int,
    pub Al: ::core::ffi::c_int,
    pub block_size: ::core::ffi::c_int,
    pub natural_order: *const ::core::ffi::c_int,
    pub lim_Se: ::core::ffi::c_int,
    pub unread_marker: ::core::ffi::c_int,
    pub master: *mut jpeg_decomp_master,
    pub main: *mut jpeg_d_main_controller,
    pub coef: *mut jpeg_d_coef_controller,
    pub post: *mut jpeg_d_post_controller,
    pub inputctl: *mut jpeg_input_controller,
    pub marker: *mut jpeg_marker_reader,
    pub entropy: *mut jpeg_entropy_decoder,
    pub idct: *mut jpeg_inverse_dct,
    pub upsample: *mut jpeg_upsampler,
    pub cconvert: *mut jpeg_color_deconverter,
    pub cquantize: *mut jpeg_color_quantizer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_color_quantizer {
    pub start_pass: Option<unsafe extern "C" fn(j_decompress_ptr, boolean) -> ()>,
    pub color_quantize: Option<
        unsafe extern "C" fn(j_decompress_ptr, JSAMPARRAY, JSAMPARRAY, ::core::ffi::c_int) -> (),
    >,
    pub finish_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub new_color_map: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
}
pub type j_decompress_ptr = *mut jpeg_decompress_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_color_deconverter {
    pub start_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub color_convert: Option<
        unsafe extern "C" fn(
            j_decompress_ptr,
            JSAMPIMAGE,
            JDIMENSION,
            JSAMPARRAY,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_upsampler {
    pub start_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub upsample: Option<
        unsafe extern "C" fn(
            j_decompress_ptr,
            JSAMPIMAGE,
            *mut JDIMENSION,
            JDIMENSION,
            JSAMPARRAY,
            *mut JDIMENSION,
            JDIMENSION,
        ) -> (),
    >,
    pub need_context_rows: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_inverse_dct {
    pub start_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub inverse_DCT: [inverse_DCT_method_ptr; 10],
}
pub type inverse_DCT_method_ptr = Option<
    unsafe extern "C" fn(
        j_decompress_ptr,
        *mut jpeg_component_info,
        JCOEFPTR,
        JSAMPARRAY,
        JDIMENSION,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_entropy_decoder {
    pub start_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub decode_mcu: Option<unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean>,
    pub insufficient_data: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_marker_reader {
    pub reset_marker_reader: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub read_markers: Option<unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int>,
    pub read_restart_marker: jpeg_marker_parser_method,
    pub saw_SOI: boolean,
    pub saw_SOF: boolean,
    pub next_restart_num: ::core::ffi::c_int,
    pub discarded_bytes: ::core::ffi::c_uint,
}
pub type jpeg_marker_parser_method = Option<unsafe extern "C" fn(j_decompress_ptr) -> boolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_input_controller {
    pub consume_input: Option<unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int>,
    pub reset_input_controller: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub start_input_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub finish_input_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub has_multiple_scans: boolean,
    pub eoi_reached: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_d_post_controller {
    pub start_pass: Option<unsafe extern "C" fn(j_decompress_ptr, J_BUF_MODE) -> ()>,
    pub post_process_data: Option<
        unsafe extern "C" fn(
            j_decompress_ptr,
            JSAMPIMAGE,
            *mut JDIMENSION,
            JDIMENSION,
            JSAMPARRAY,
            *mut JDIMENSION,
            JDIMENSION,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_d_coef_controller {
    pub start_input_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub consume_data: Option<unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int>,
    pub start_output_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub decompress_data:
        Option<unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE) -> ::core::ffi::c_int>,
    pub coef_arrays: *mut jvirt_barray_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_d_main_controller {
    pub start_pass: Option<unsafe extern "C" fn(j_decompress_ptr, J_BUF_MODE) -> ()>,
    pub process_data: Option<
        unsafe extern "C" fn(j_decompress_ptr, JSAMPARRAY, *mut JDIMENSION, JDIMENSION) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_decomp_master {
    pub prepare_for_output_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub finish_output_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub is_dummy_pass: boolean,
    pub first_iMCU_col: JDIMENSION,
    pub last_iMCU_col: JDIMENSION,
    pub first_MCU_col: [JDIMENSION; 10],
    pub last_MCU_col: [JDIMENSION; 10],
    pub jinit_upsampler_no_alloc: boolean,
    pub last_good_iMCU_row: JDIMENSION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_source_mgr {
    pub next_input_byte: *const JOCTET,
    pub bytes_in_buffer: size_t,
    pub init_source: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub fill_input_buffer: Option<unsafe extern "C" fn(j_decompress_ptr) -> boolean>,
    pub skip_input_data: Option<unsafe extern "C" fn(j_decompress_ptr, ::core::ffi::c_long) -> ()>,
    pub resync_to_restart:
        Option<unsafe extern "C" fn(j_decompress_ptr, ::core::ffi::c_int) -> boolean>,
    pub term_source: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
}
pub type JLONG = ::core::ffi::c_long;
pub type my_merged_upsample_ptr = *mut my_merged_upsampler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_merged_upsampler {
    pub pub_0: jpeg_upsampler,
    pub upmethod:
        Option<unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE, JDIMENSION, JSAMPARRAY) -> ()>,
    pub Cr_r_tab: *mut ::core::ffi::c_int,
    pub Cb_b_tab: *mut ::core::ffi::c_int,
    pub Cr_g_tab: *mut JLONG,
    pub Cb_g_tab: *mut JLONG,
    pub spare_row: JSAMPROW,
    pub spare_full: boolean,
    pub out_row_width: JDIMENSION,
    pub rows_to_go: JDIMENSION,
}
pub const MAXJSAMPLE: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const CENTERJSAMPLE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RGB_RED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RGB_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RGB_BLUE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const RGB_PIXELSIZE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EXT_RGB_RED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXT_RGB_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_RGB_BLUE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_RGB_PIXELSIZE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EXT_RGBX_RED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXT_RGBX_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_RGBX_BLUE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_RGBX_PIXELSIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EXT_BGR_RED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_BGR_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_BGR_BLUE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXT_BGR_PIXELSIZE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EXT_BGRX_RED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_BGRX_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_BGRX_BLUE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXT_BGRX_PIXELSIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EXT_XBGR_RED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EXT_XBGR_GREEN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_XBGR_BLUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_XBGR_PIXELSIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EXT_XRGB_RED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_XRGB_GREEN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_XRGB_BLUE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EXT_XRGB_PIXELSIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SCALEBITS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ONE_HALF: JLONG =
    (1 as ::core::ffi::c_int as JLONG) << SCALEBITS - 1 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr0 =
        *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh16 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh16 as ::core::ffi::c_int;
        let fresh17 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh17 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh18 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh18 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE as isize);
        let fresh19 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh19 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
    }
}
#[inline(always)]
unsafe extern "C" fn h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr00: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr01: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr00 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as JDIMENSION) as isize);
    inptr01 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as JDIMENSION)
            .wrapping_add(1 as JDIMENSION) as isize,
    );
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    outptr1 = *output_buf.offset(1 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh68 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh68 as ::core::ffi::c_int;
        let fresh69 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh69 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh70 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh70 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE as isize);
        let fresh71 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh71 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE as isize);
        let fresh72 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh72 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE as isize);
        let fresh73 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh73 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
        y = *inptr01 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE as isize) = *range_limit.offset((y + cblue) as isize);
    }
}
#[inline(always)]
unsafe extern "C" fn extrgb_h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr0 =
        *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh40 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh40 as ::core::ffi::c_int;
        let fresh41 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh41 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh42 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh42 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_5 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_5 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_5 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE_5 as isize);
        let fresh43 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh43 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_5 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_5 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_5 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE_5 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_5 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_5 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_5 as isize) = *range_limit.offset((y + cblue) as isize);
    }
}
#[inline(always)]
unsafe extern "C" fn extrgb_h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr00: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr01: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr00 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as JDIMENSION) as isize);
    inptr01 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as JDIMENSION)
            .wrapping_add(1 as JDIMENSION) as isize,
    );
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    outptr1 = *output_buf.offset(1 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh104 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh104 as ::core::ffi::c_int;
        let fresh105 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh105 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh106 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh106 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_5 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_5 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_5 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE_5 as isize);
        let fresh107 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh107 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_5 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_5 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_5 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE_5 as isize);
        let fresh108 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh108 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_5 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_5 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_5 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE_5 as isize);
        let fresh109 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh109 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_5 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_5 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_5 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE_5 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_5 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_5 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_5 as isize) = *range_limit.offset((y + cblue) as isize);
        y = *inptr01 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_5 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_5 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_5 as isize) = *range_limit.offset((y + cblue) as isize);
    }
}
#[inline(always)]
unsafe extern "C" fn extrgbx_h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr0 =
        *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh36 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh36 as ::core::ffi::c_int;
        let fresh37 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh37 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh38 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh38 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
        let fresh39 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh39 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
    }
}
#[inline(always)]
unsafe extern "C" fn extrgbx_h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr00: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr01: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr00 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as JDIMENSION) as isize);
    inptr01 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as JDIMENSION)
            .wrapping_add(1 as JDIMENSION) as isize,
    );
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    outptr1 = *output_buf.offset(1 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh98 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh98 as ::core::ffi::c_int;
        let fresh99 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh99 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh100 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh100 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_4 as isize);
        let fresh101 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh101 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_4 as isize);
        let fresh102 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh102 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_4 as isize);
        let fresh103 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh103 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_4 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
        y = *inptr01 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_4 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_4 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_4 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
    }
}
#[inline(always)]
unsafe extern "C" fn extbgr_h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr0 =
        *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh32 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh32 as ::core::ffi::c_int;
        let fresh33 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh33 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh34 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh34 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
        let fresh35 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh35 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
    }
}
#[inline(always)]
unsafe extern "C" fn extbgr_h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr00: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr01: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr00 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as JDIMENSION) as isize);
    inptr01 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as JDIMENSION)
            .wrapping_add(1 as JDIMENSION) as isize,
    );
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    outptr1 = *output_buf.offset(1 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh92 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh92 as ::core::ffi::c_int;
        let fresh93 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh93 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh94 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh94 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE_3 as isize);
        let fresh95 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh95 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(RGB_PIXELSIZE_3 as isize);
        let fresh96 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh96 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE_3 as isize);
        let fresh97 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh97 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(RGB_PIXELSIZE_3 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
        y = *inptr01 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_3 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_3 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_3 as isize) = *range_limit.offset((y + cblue) as isize);
    }
}
#[inline(always)]
unsafe extern "C" fn extbgrx_h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr0 =
        *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh28 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh28 as ::core::ffi::c_int;
        let fresh29 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh29 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh30 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh30 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
        let fresh31 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh31 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
    }
}
#[inline(always)]
unsafe extern "C" fn extbgrx_h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr00: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr01: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr00 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as JDIMENSION) as isize);
    inptr01 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as JDIMENSION)
            .wrapping_add(1 as JDIMENSION) as isize,
    );
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    outptr1 = *output_buf.offset(1 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh86 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh86 as ::core::ffi::c_int;
        let fresh87 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh87 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh88 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh88 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_2 as isize);
        let fresh89 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh89 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_2 as isize);
        let fresh90 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh90 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_2 as isize);
        let fresh91 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh91 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_2 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
        y = *inptr01 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_2 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_2 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_2 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
    }
}
#[inline(always)]
unsafe extern "C" fn extxbgr_h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr0 =
        *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh24 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh24 as ::core::ffi::c_int;
        let fresh25 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh25 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh26 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh26 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
        let fresh27 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh27 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
    }
}
#[inline(always)]
unsafe extern "C" fn extxbgr_h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr00: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr01: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr00 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as JDIMENSION) as isize);
    inptr01 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as JDIMENSION)
            .wrapping_add(1 as JDIMENSION) as isize,
    );
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    outptr1 = *output_buf.offset(1 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh80 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh80 as ::core::ffi::c_int;
        let fresh81 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh81 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh82 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh82 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_1 as isize);
        let fresh83 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh83 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_1 as isize);
        let fresh84 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh84 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_1 as isize);
        let fresh85 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh85 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_1 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
        y = *inptr01 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_1 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_1 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_1 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
    }
}
#[inline(always)]
unsafe extern "C" fn extxrgb_h2v1_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr0 =
        *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh20 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh20 as ::core::ffi::c_int;
        let fresh21 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh21 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh22 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh22 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
        outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
        let fresh23 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh23 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
        outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as ::core::ffi::c_int;
        *outptr.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
    }
}
#[inline(always)]
unsafe extern "C" fn extxrgb_h2v2_merged_upsample_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr00: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr01: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    inptr00 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as JDIMENSION) as isize);
    inptr01 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as JDIMENSION)
            .wrapping_add(1 as JDIMENSION) as isize,
    );
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    outptr1 = *output_buf.offset(1 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh74 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh74 as ::core::ffi::c_int;
        let fresh75 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh75 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh76 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh76 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_0 as isize);
        let fresh77 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh77 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
        outptr0 = outptr0.offset(RGB_PIXELSIZE_0 as isize);
        let fresh78 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh78 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_0 as isize);
        let fresh79 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh79 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
        outptr1 = outptr1.offset(RGB_PIXELSIZE_0 as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as ::core::ffi::c_int;
        *outptr0.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr0.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
        y = *inptr01 as ::core::ffi::c_int;
        *outptr1.offset(RGB_RED_0 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(RGB_GREEN_0 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(RGB_BLUE_0 as isize) = *range_limit.offset((y + cblue) as isize);
        *outptr1.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
    }
}
pub const RGB_RED_5: ::core::ffi::c_int = EXT_RGB_RED;
pub const RGB_GREEN_5: ::core::ffi::c_int = EXT_RGB_GREEN;
pub const RGB_BLUE_5: ::core::ffi::c_int = EXT_RGB_BLUE;
pub const RGB_PIXELSIZE_5: ::core::ffi::c_int = EXT_RGB_PIXELSIZE;
#[inline(always)]
unsafe extern "C" fn h2v1_merged_upsample_565_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut r: ::core::ffi::c_uint = 0;
    let mut g: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr0 =
        *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh0 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh0 as ::core::ffi::c_int;
        let fresh1 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh1 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh2 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh2 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        let fresh3 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh3 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = ((r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as JLONG
            | rgb;
        *(outptr as *mut INT16).offset(0 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr as *mut INT16).offset(1 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        *(outptr as *mut INT16) = rgb as INT16;
    }
}
#[inline(always)]
unsafe extern "C" fn h2v1_merged_upsample_565D_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut d0: JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as JDIMENSION) as usize];
    let mut r: ::core::ffi::c_uint = 0;
    let mut g: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr0 =
        *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh8 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh8 as ::core::ffi::c_int;
        let fresh9 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh9 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh10 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh10 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        let fresh11 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh11 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = ((r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as JLONG
            | rgb;
        *(outptr as *mut INT16).offset(0 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr as *mut INT16).offset(1 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        *(outptr as *mut INT16) = rgb as INT16;
    }
}
#[inline(always)]
unsafe extern "C" fn h2v2_merged_upsample_565_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr00: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr01: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut r: ::core::ffi::c_uint = 0;
    let mut g: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr00 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as JDIMENSION) as isize);
    inptr01 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as JDIMENSION)
            .wrapping_add(1 as JDIMENSION) as isize,
    );
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    outptr1 = *output_buf.offset(1 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh44 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh44 as ::core::ffi::c_int;
        let fresh45 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh45 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh46 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh46 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        let fresh47 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh47 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = ((r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as JLONG
            | rgb;
        *(outptr0 as *mut INT16).offset(0 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr0 as *mut INT16).offset(1 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr0 = outptr0.offset(4 as ::core::ffi::c_int as isize);
        let fresh48 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh48 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        let fresh49 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh49 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = ((r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as JLONG
            | rgb;
        *(outptr1 as *mut INT16).offset(0 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr1 as *mut INT16).offset(1 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr1 = outptr1.offset(4 as ::core::ffi::c_int as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        *(outptr0 as *mut INT16) = rgb as INT16;
        y = *inptr01 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        *(outptr1 as *mut INT16) = rgb as INT16;
    }
}
#[inline(always)]
unsafe extern "C" fn h2v2_merged_upsample_565D_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr00: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr01: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut d0: JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as JDIMENSION) as usize];
    let mut d1: JLONG = dither_matrix[((*cinfo).output_scanline.wrapping_add(1 as JDIMENSION)
        & DITHER_MASK as JDIMENSION) as usize];
    let mut r: ::core::ffi::c_uint = 0;
    let mut g: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr00 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as JDIMENSION) as isize);
    inptr01 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as JDIMENSION)
            .wrapping_add(1 as JDIMENSION) as isize,
    );
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    outptr1 = *output_buf.offset(1 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh56 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh56 as ::core::ffi::c_int;
        let fresh57 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh57 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh58 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh58 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        let fresh59 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh59 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = ((r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as JLONG
            | rgb;
        *(outptr0 as *mut INT16).offset(0 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr0 as *mut INT16).offset(1 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr0 = outptr0.offset(4 as ::core::ffi::c_int as isize);
        let fresh60 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh60 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d1 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d1 = (d1 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d1 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        let fresh61 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh61 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d1 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d1 = (d1 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d1 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = ((r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as JLONG
            | rgb;
        *(outptr1 as *mut INT16).offset(0 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr1 as *mut INT16).offset(1 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr1 = outptr1.offset(4 as ::core::ffi::c_int as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        *(outptr0 as *mut INT16) = rgb as INT16;
        y = *inptr01 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d1 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
            | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
            | b >> 3 as ::core::ffi::c_int) as JLONG;
        *(outptr1 as *mut INT16) = rgb as INT16;
    }
}
#[inline(always)]
unsafe extern "C" fn h2v1_merged_upsample_565_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut r: ::core::ffi::c_uint = 0;
    let mut g: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr0 =
        *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh4 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh4 as ::core::ffi::c_int;
        let fresh5 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh5 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh6 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh6 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        let fresh7 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh7 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = rgb << 16 as ::core::ffi::c_int
            | (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
        *(outptr as *mut INT16).offset(1 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr as *mut INT16).offset(0 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        *(outptr as *mut INT16) = rgb as INT16;
    }
}
#[inline(always)]
unsafe extern "C" fn h2v1_merged_upsample_565D_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut d0: JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as JDIMENSION) as usize];
    let mut r: ::core::ffi::c_uint = 0;
    let mut g: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr0 =
        *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh12 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh12 as ::core::ffi::c_int;
        let fresh13 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh13 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh14 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh14 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        let fresh15 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh15 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = rgb << 16 as ::core::ffi::c_int
            | (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
        *(outptr as *mut INT16).offset(1 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr as *mut INT16).offset(0 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        *(outptr as *mut INT16) = rgb as INT16;
    }
}
#[inline(always)]
unsafe extern "C" fn h2v2_merged_upsample_565_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr00: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr01: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut r: ::core::ffi::c_uint = 0;
    let mut g: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr00 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as JDIMENSION) as isize);
    inptr01 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as JDIMENSION)
            .wrapping_add(1 as JDIMENSION) as isize,
    );
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    outptr1 = *output_buf.offset(1 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh50 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh50 as ::core::ffi::c_int;
        let fresh51 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh51 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh52 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh52 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        let fresh53 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh53 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = rgb << 16 as ::core::ffi::c_int
            | (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
        *(outptr0 as *mut INT16).offset(1 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr0 as *mut INT16).offset(0 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr0 = outptr0.offset(4 as ::core::ffi::c_int as isize);
        let fresh54 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh54 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        let fresh55 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh55 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = rgb << 16 as ::core::ffi::c_int
            | (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
        *(outptr1 as *mut INT16).offset(1 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr1 as *mut INT16).offset(0 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr1 = outptr1.offset(4 as ::core::ffi::c_int as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        *(outptr0 as *mut INT16) = rgb as INT16;
        y = *inptr01 as ::core::ffi::c_int;
        r = *range_limit.offset((y + cred) as isize) as ::core::ffi::c_uint;
        g = *range_limit.offset((y + cgreen) as isize) as ::core::ffi::c_uint;
        b = *range_limit.offset((y + cblue) as isize) as ::core::ffi::c_uint;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        *(outptr1 as *mut INT16) = rgb as INT16;
    }
}
#[inline(always)]
unsafe extern "C" fn h2v2_merged_upsample_565D_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cred: ::core::ffi::c_int = 0;
    let mut cgreen: ::core::ffi::c_int = 0;
    let mut cblue: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr00: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr01: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*upsample).Cb_g_tab;
    let mut d0: JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as JDIMENSION) as usize];
    let mut d1: JLONG = dither_matrix[((*cinfo).output_scanline.wrapping_add(1 as JDIMENSION)
        & DITHER_MASK as JDIMENSION) as usize];
    let mut r: ::core::ffi::c_uint = 0;
    let mut g: ::core::ffi::c_uint = 0;
    let mut b: ::core::ffi::c_uint = 0;
    let mut rgb: JLONG = 0;
    inptr00 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as JDIMENSION) as isize);
    inptr01 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as JDIMENSION)
            .wrapping_add(1 as JDIMENSION) as isize,
    );
    inptr1 =
        *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    inptr2 =
        *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as ::core::ffi::c_int as isize);
    outptr1 = *output_buf.offset(1 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width >> 1 as ::core::ffi::c_int;
    while col > 0 as JDIMENSION {
        let fresh62 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh62 as ::core::ffi::c_int;
        let fresh63 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh63 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        let fresh64 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh64 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        let fresh65 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh65 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = rgb << 16 as ::core::ffi::c_int
            | (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
        *(outptr0 as *mut INT16).offset(1 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr0 as *mut INT16).offset(0 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr0 = outptr0.offset(4 as ::core::ffi::c_int as isize);
        let fresh66 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh66 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d1 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d1 = (d1 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d1 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        let fresh67 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh67 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d1 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        d1 = (d1 & 0xff as JLONG) << 24 as ::core::ffi::c_int
            | d1 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
        rgb = rgb << 16 as ::core::ffi::c_int
            | (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
        *(outptr1 as *mut INT16).offset(1 as ::core::ffi::c_int as isize) = rgb as INT16;
        *(outptr1 as *mut INT16).offset(0 as ::core::ffi::c_int as isize) =
            (rgb >> 16 as ::core::ffi::c_int) as INT16;
        outptr1 = outptr1.offset(4 as ::core::ffi::c_int as isize);
        col = col.wrapping_sub(1);
    }
    if (*cinfo).output_width & 1 as JDIMENSION != 0 {
        cb = *inptr1 as ::core::ffi::c_int;
        cr = *inptr2 as ::core::ffi::c_int;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
            >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d0 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        *(outptr0 as *mut INT16) = rgb as INT16;
        y = *inptr01 as ::core::ffi::c_int;
        r = *range_limit.offset(((y + cred) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        g = *range_limit.offset(
            ((y + cgreen) as JLONG + ((d1 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
        ) as ::core::ffi::c_uint;
        b = *range_limit.offset(((y + cblue) as JLONG + (d1 & 0xff as JLONG)) as isize)
            as ::core::ffi::c_uint;
        rgb = (r & 0xf8 as ::core::ffi::c_uint
            | g >> 5 as ::core::ffi::c_int
            | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
            | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint) as JLONG;
        *(outptr1 as *mut INT16) = rgb as INT16;
    }
}
pub const RGB_RED_4: ::core::ffi::c_int = EXT_RGBX_RED;
pub const RGB_GREEN_4: ::core::ffi::c_int = EXT_RGBX_GREEN;
pub const RGB_BLUE_4: ::core::ffi::c_int = EXT_RGBX_BLUE;
pub const RGB_ALPHA_2: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const RGB_PIXELSIZE_4: ::core::ffi::c_int = EXT_RGBX_PIXELSIZE;
pub const RGB_RED_3: ::core::ffi::c_int = EXT_BGR_RED;
pub const RGB_GREEN_3: ::core::ffi::c_int = EXT_BGR_GREEN;
pub const RGB_BLUE_3: ::core::ffi::c_int = EXT_BGR_BLUE;
pub const RGB_PIXELSIZE_3: ::core::ffi::c_int = EXT_BGR_PIXELSIZE;
pub const RGB_RED_2: ::core::ffi::c_int = EXT_BGRX_RED;
pub const RGB_GREEN_2: ::core::ffi::c_int = EXT_BGRX_GREEN;
pub const RGB_BLUE_2: ::core::ffi::c_int = EXT_BGRX_BLUE;
pub const RGB_ALPHA_1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const RGB_PIXELSIZE_2: ::core::ffi::c_int = EXT_BGRX_PIXELSIZE;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const RGB_RED_1: ::core::ffi::c_int = EXT_XBGR_RED;
pub const RGB_GREEN_1: ::core::ffi::c_int = EXT_XBGR_GREEN;
pub const RGB_BLUE_1: ::core::ffi::c_int = EXT_XBGR_BLUE;
pub const RGB_ALPHA_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RGB_PIXELSIZE_1: ::core::ffi::c_int = EXT_XBGR_PIXELSIZE;
pub const RGB_RED_0: ::core::ffi::c_int = EXT_XRGB_RED;
pub const RGB_GREEN_0: ::core::ffi::c_int = EXT_XRGB_GREEN;
pub const RGB_BLUE_0: ::core::ffi::c_int = EXT_XRGB_BLUE;
pub const RGB_ALPHA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RGB_PIXELSIZE_0: ::core::ffi::c_int = EXT_XRGB_PIXELSIZE;
unsafe extern "C" fn build_ycc_rgb_table(mut cinfo: j_decompress_ptr) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut i: ::core::ffi::c_int = 0;
    let mut x: JLONG = 0;
    (*upsample).Cr_r_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    (*upsample).Cb_b_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    (*upsample).Cr_g_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<JLONG>() as size_t),
    ) as *mut JLONG;
    (*upsample).Cb_g_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<JLONG>() as size_t),
    ) as *mut JLONG;
    i = 0 as ::core::ffi::c_int;
    x = -CENTERJSAMPLE as JLONG;
    while i <= MAXJSAMPLE {
        *(*upsample).Cr_r_tab.offset(i as isize) = ((1.40200f64
            * ((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int) as ::core::ffi::c_double
            + 0.5f64) as JLONG
            * x
            + ((1 as ::core::ffi::c_int as JLONG)
                << 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 16 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        *(*upsample).Cb_b_tab.offset(i as isize) = ((1.77200f64
            * ((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int) as ::core::ffi::c_double
            + 0.5f64) as JLONG
            * x
            + ((1 as ::core::ffi::c_int as JLONG)
                << 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 16 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        *(*upsample).Cr_g_tab.offset(i as isize) = -((0.71414f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG)
            * x;
        *(*upsample).Cb_g_tab.offset(i as isize) = -((0.34414f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG)
            * x
            + ONE_HALF;
        i += 1;
        x += 1;
    }
}
unsafe extern "C" fn start_pass_merged_upsample(mut cinfo: j_decompress_ptr) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    (*upsample).spare_full = FALSE as boolean;
    (*upsample).rows_to_go = (*cinfo).output_height;
}
unsafe extern "C" fn merged_2v_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: *mut JDIMENSION,
    mut in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    let mut work_ptrs: [JSAMPROW; 2] = [::core::ptr::null_mut::<JSAMPLE>(); 2];
    let mut num_rows: JDIMENSION = 0;
    if (*upsample).spare_full != 0 {
        let mut size: JDIMENSION = (*upsample).out_row_width;
        if (*cinfo).out_color_space as ::core::ffi::c_uint
            == JCS_RGB565 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            size = (*cinfo).output_width.wrapping_mul(2 as JDIMENSION);
        }
        jcopy_sample_rows(
            ::core::ptr::addr_of_mut!((*upsample).spare_row),
            0 as ::core::ffi::c_int,
            output_buf.offset(*out_row_ctr as isize),
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            size,
        );
        num_rows = 1 as JDIMENSION;
        (*upsample).spare_full = FALSE as boolean;
    } else {
        num_rows = 2 as JDIMENSION;
        if num_rows > (*upsample).rows_to_go {
            num_rows = (*upsample).rows_to_go;
        }
        out_rows_avail = out_rows_avail.wrapping_sub(*out_row_ctr);
        if num_rows > out_rows_avail {
            num_rows = out_rows_avail;
        }
        work_ptrs[0 as ::core::ffi::c_int as usize] = *output_buf.offset(*out_row_ctr as isize);
        if num_rows > 1 as JDIMENSION {
            work_ptrs[1 as ::core::ffi::c_int as usize] =
                *output_buf.offset((*out_row_ctr).wrapping_add(1 as JDIMENSION) as isize);
        } else {
            work_ptrs[1 as ::core::ffi::c_int as usize] = (*upsample).spare_row;
            (*upsample).spare_full = TRUE as boolean;
        }
        Some((*upsample).upmethod.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            cinfo,
            input_buf,
            *in_row_group_ctr,
            ::core::ptr::addr_of_mut!(work_ptrs) as JSAMPARRAY,
        );
    }
    *out_row_ctr = (*out_row_ctr).wrapping_add(num_rows);
    (*upsample).rows_to_go = (*upsample).rows_to_go.wrapping_sub(num_rows);
    if (*upsample).spare_full == 0 {
        *in_row_group_ctr = (*in_row_group_ctr).wrapping_add(1);
    }
}
unsafe extern "C" fn merged_1v_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: *mut JDIMENSION,
    mut in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut upsample: my_merged_upsample_ptr = (*cinfo).upsample as my_merged_upsample_ptr;
    Some((*upsample).upmethod.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        cinfo,
        input_buf,
        *in_row_group_ctr,
        output_buf.offset(*out_row_ctr as isize),
    );
    *out_row_ctr = (*out_row_ctr).wrapping_add(1);
    *in_row_group_ctr = (*in_row_group_ctr).wrapping_add(1);
}
unsafe extern "C" fn h2v1_merged_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    match (*cinfo).out_color_space as ::core::ffi::c_uint {
        6 => {
            extrgb_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        7 | 12 => {
            extrgbx_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        8 => {
            extbgr_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        9 | 13 => {
            extbgrx_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        10 | 14 => {
            extxbgr_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        11 | 15 => {
            extxrgb_h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        _ => {
            h2v1_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
    };
}
unsafe extern "C" fn h2v2_merged_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    match (*cinfo).out_color_space as ::core::ffi::c_uint {
        6 => {
            extrgb_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        7 | 12 => {
            extrgbx_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        8 => {
            extbgr_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        9 | 13 => {
            extbgrx_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        10 | 14 => {
            extxbgr_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        11 | 15 => {
            extxrgb_h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
        _ => {
            h2v2_merged_upsample_internal(cinfo, input_buf, in_row_group_ctr, output_buf);
        }
    };
}
pub const DITHER_MASK: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
static mut dither_matrix: [JLONG; 4] = [
    0x8020a as ::core::ffi::c_int as JLONG,
    0xc040e06 as ::core::ffi::c_int as JLONG,
    0x30b0109 as ::core::ffi::c_int as JLONG,
    0xf070d05 as ::core::ffi::c_int as JLONG,
];
#[inline(always)]
unsafe extern "C" fn is_big_endian() -> boolean {
    let mut test_value: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if *(::core::ptr::addr_of_mut!(test_value) as *mut ::core::ffi::c_char) as ::core::ffi::c_int
        != 1 as ::core::ffi::c_int
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn h2v1_merged_upsample_565(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    if is_big_endian() != 0 {
        h2v1_merged_upsample_565_be(cinfo, input_buf, in_row_group_ctr, output_buf);
    } else {
        h2v1_merged_upsample_565_le(cinfo, input_buf, in_row_group_ctr, output_buf);
    };
}
unsafe extern "C" fn h2v1_merged_upsample_565D(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    if is_big_endian() != 0 {
        h2v1_merged_upsample_565D_be(cinfo, input_buf, in_row_group_ctr, output_buf);
    } else {
        h2v1_merged_upsample_565D_le(cinfo, input_buf, in_row_group_ctr, output_buf);
    };
}
unsafe extern "C" fn h2v2_merged_upsample_565(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    if is_big_endian() != 0 {
        h2v2_merged_upsample_565_be(cinfo, input_buf, in_row_group_ctr, output_buf);
    } else {
        h2v2_merged_upsample_565_le(cinfo, input_buf, in_row_group_ctr, output_buf);
    };
}
unsafe extern "C" fn h2v2_merged_upsample_565D(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
    if is_big_endian() != 0 {
        h2v2_merged_upsample_565D_be(cinfo, input_buf, in_row_group_ctr, output_buf);
    } else {
        h2v2_merged_upsample_565D_le(cinfo, input_buf, in_row_group_ctr, output_buf);
    };
}
pub unsafe extern "C" fn jinit_merged_upsampler(mut cinfo: j_decompress_ptr) {
    let mut upsample: my_merged_upsample_ptr = ::core::ptr::null_mut::<my_merged_upsampler>();
    upsample = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<my_merged_upsampler>() as size_t,
    ) as my_merged_upsample_ptr;
    (*cinfo).upsample = upsample as *mut jpeg_upsampler as *mut jpeg_upsampler;
    (*upsample).pub_0.start_pass =
        Some(start_pass_merged_upsample as unsafe extern "C" fn(j_decompress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
    (*upsample).pub_0.need_context_rows = FALSE as boolean;
    (*upsample).out_row_width = (*cinfo)
        .output_width
        .wrapping_mul((*cinfo).out_color_components as JDIMENSION);
    if (*cinfo).max_v_samp_factor == 2 as ::core::ffi::c_int {
        (*upsample).pub_0.upsample = Some(
            merged_2v_upsample
                as unsafe extern "C" fn(
                    j_decompress_ptr,
                    JSAMPIMAGE,
                    *mut JDIMENSION,
                    JDIMENSION,
                    JSAMPARRAY,
                    *mut JDIMENSION,
                    JDIMENSION,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    j_decompress_ptr,
                    JSAMPIMAGE,
                    *mut JDIMENSION,
                    JDIMENSION,
                    JSAMPARRAY,
                    *mut JDIMENSION,
                    JDIMENSION,
                ) -> (),
            >;
        if jsimd_can_h2v2_merged_upsample() != 0 {
            (*upsample).upmethod = Some(
                jsimd_h2v2_merged_upsample
                    as unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPIMAGE,
                        JDIMENSION,
                        JSAMPARRAY,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPIMAGE,
                        JDIMENSION,
                        JSAMPARRAY,
                    ) -> (),
                >;
        } else {
            (*upsample).upmethod = Some(
                h2v2_merged_upsample
                    as unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPIMAGE,
                        JDIMENSION,
                        JSAMPARRAY,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPIMAGE,
                        JDIMENSION,
                        JSAMPARRAY,
                    ) -> (),
                >;
        }
        if (*cinfo).out_color_space as ::core::ffi::c_uint
            == JCS_RGB565 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (*cinfo).dither_mode as ::core::ffi::c_uint
                != JDITHER_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*upsample).upmethod = Some(
                    h2v2_merged_upsample_565D
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                        ) -> (),
                    >;
            } else {
                (*upsample).upmethod = Some(
                    h2v2_merged_upsample_565
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                        ) -> (),
                    >;
            }
        }
        (*upsample).spare_row = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((*upsample).out_row_width as usize)
                .wrapping_mul(::core::mem::size_of::<JSAMPLE>() as usize),
        ) as JSAMPROW;
    } else {
        (*upsample).pub_0.upsample = Some(
            merged_1v_upsample
                as unsafe extern "C" fn(
                    j_decompress_ptr,
                    JSAMPIMAGE,
                    *mut JDIMENSION,
                    JDIMENSION,
                    JSAMPARRAY,
                    *mut JDIMENSION,
                    JDIMENSION,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    j_decompress_ptr,
                    JSAMPIMAGE,
                    *mut JDIMENSION,
                    JDIMENSION,
                    JSAMPARRAY,
                    *mut JDIMENSION,
                    JDIMENSION,
                ) -> (),
            >;
        if jsimd_can_h2v1_merged_upsample() != 0 {
            (*upsample).upmethod = Some(
                jsimd_h2v1_merged_upsample
                    as unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPIMAGE,
                        JDIMENSION,
                        JSAMPARRAY,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPIMAGE,
                        JDIMENSION,
                        JSAMPARRAY,
                    ) -> (),
                >;
        } else {
            (*upsample).upmethod = Some(
                h2v1_merged_upsample
                    as unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPIMAGE,
                        JDIMENSION,
                        JSAMPARRAY,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPIMAGE,
                        JDIMENSION,
                        JSAMPARRAY,
                    ) -> (),
                >;
        }
        if (*cinfo).out_color_space as ::core::ffi::c_uint
            == JCS_RGB565 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (*cinfo).dither_mode as ::core::ffi::c_uint
                != JDITHER_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*upsample).upmethod = Some(
                    h2v1_merged_upsample_565D
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                        ) -> (),
                    >;
            } else {
                (*upsample).upmethod = Some(
                    h2v1_merged_upsample_565
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                        ) -> (),
                    >;
            }
        }
        (*upsample).spare_row = ::core::ptr::null_mut::<JSAMPLE>();
    }
    build_ycc_rgb_table(cinfo);
}
