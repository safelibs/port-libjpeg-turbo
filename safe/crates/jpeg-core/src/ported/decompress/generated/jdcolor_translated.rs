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
    fn jsimd_can_ycc_rgb() -> ::core::ffi::c_int;
    fn jsimd_can_ycc_rgb565() -> ::core::ffi::c_int;
    fn jsimd_ycc_rgb_convert(
        cinfo: j_decompress_ptr,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: ::core::ffi::c_int,
    );
    fn jsimd_ycc_rgb565_convert(
        cinfo: j_decompress_ptr,
        input_buf: JSAMPIMAGE,
        input_row: JDIMENSION,
        output_buf: JSAMPARRAY,
        num_rows: ::core::ffi::c_int,
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
pub const JERR_CONVERSION_NOTIMPL: C2RustUnnamed_0 = 28;
pub type my_cconvert_ptr = *mut my_color_deconverter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_color_deconverter {
    pub pub_0: jpeg_color_deconverter,
    pub Cr_r_tab: *mut ::core::ffi::c_int,
    pub Cb_b_tab: *mut ::core::ffi::c_int,
    pub Cr_g_tab: *mut JLONG,
    pub Cb_g_tab: *mut JLONG,
    pub rgb_y_tab: *mut JLONG,
}
pub const JERR_BAD_J_COLORSPACE: C2RustUnnamed_0 = 11;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const JMSG_LASTMSGCODE: C2RustUnnamed_0 = 128;
pub const JWRN_BOGUS_ICC: C2RustUnnamed_0 = 127;
pub const JWRN_TOO_MUCH_DATA: C2RustUnnamed_0 = 126;
pub const JWRN_NOT_SEQUENTIAL: C2RustUnnamed_0 = 125;
pub const JWRN_MUST_RESYNC: C2RustUnnamed_0 = 124;
pub const JWRN_JPEG_EOF: C2RustUnnamed_0 = 123;
pub const JWRN_JFIF_MAJOR: C2RustUnnamed_0 = 122;
pub const JWRN_HUFF_BAD_CODE: C2RustUnnamed_0 = 121;
pub const JWRN_HIT_MARKER: C2RustUnnamed_0 = 120;
pub const JWRN_EXTRANEOUS_DATA: C2RustUnnamed_0 = 119;
pub const JWRN_BOGUS_PROGRESSION: C2RustUnnamed_0 = 118;
pub const JWRN_ARITH_BAD_CODE: C2RustUnnamed_0 = 117;
pub const JWRN_ADOBE_XFORM: C2RustUnnamed_0 = 116;
pub const JTRC_XMS_OPEN: C2RustUnnamed_0 = 115;
pub const JTRC_XMS_CLOSE: C2RustUnnamed_0 = 114;
pub const JTRC_UNKNOWN_IDS: C2RustUnnamed_0 = 113;
pub const JTRC_THUMB_RGB: C2RustUnnamed_0 = 112;
pub const JTRC_THUMB_PALETTE: C2RustUnnamed_0 = 111;
pub const JTRC_THUMB_JPEG: C2RustUnnamed_0 = 110;
pub const JTRC_TFILE_OPEN: C2RustUnnamed_0 = 109;
pub const JTRC_TFILE_CLOSE: C2RustUnnamed_0 = 108;
pub const JTRC_SOS_PARAMS: C2RustUnnamed_0 = 107;
pub const JTRC_SOS_COMPONENT: C2RustUnnamed_0 = 106;
pub const JTRC_SOS: C2RustUnnamed_0 = 105;
pub const JTRC_SOI: C2RustUnnamed_0 = 104;
pub const JTRC_SOF_COMPONENT: C2RustUnnamed_0 = 103;
pub const JTRC_SOF: C2RustUnnamed_0 = 102;
pub const JTRC_SMOOTH_NOTIMPL: C2RustUnnamed_0 = 101;
pub const JTRC_RST: C2RustUnnamed_0 = 100;
pub const JTRC_RECOVERY_ACTION: C2RustUnnamed_0 = 99;
pub const JTRC_QUANT_SELECTED: C2RustUnnamed_0 = 98;
pub const JTRC_QUANT_NCOLORS: C2RustUnnamed_0 = 97;
pub const JTRC_QUANT_3_NCOLORS: C2RustUnnamed_0 = 96;
pub const JTRC_QUANTVALS: C2RustUnnamed_0 = 95;
pub const JTRC_PARMLESS_MARKER: C2RustUnnamed_0 = 94;
pub const JTRC_MISC_MARKER: C2RustUnnamed_0 = 93;
pub const JTRC_JFIF_THUMBNAIL: C2RustUnnamed_0 = 92;
pub const JTRC_JFIF_EXTENSION: C2RustUnnamed_0 = 91;
pub const JTRC_JFIF_BADTHUMBNAILSIZE: C2RustUnnamed_0 = 90;
pub const JTRC_JFIF: C2RustUnnamed_0 = 89;
pub const JTRC_HUFFBITS: C2RustUnnamed_0 = 88;
pub const JTRC_EOI: C2RustUnnamed_0 = 87;
pub const JTRC_EMS_OPEN: C2RustUnnamed_0 = 86;
pub const JTRC_EMS_CLOSE: C2RustUnnamed_0 = 85;
pub const JTRC_DRI: C2RustUnnamed_0 = 84;
pub const JTRC_DQT: C2RustUnnamed_0 = 83;
pub const JTRC_DHT: C2RustUnnamed_0 = 82;
pub const JTRC_DAC: C2RustUnnamed_0 = 81;
pub const JTRC_APP14: C2RustUnnamed_0 = 80;
pub const JTRC_APP0: C2RustUnnamed_0 = 79;
pub const JTRC_ADOBE: C2RustUnnamed_0 = 78;
pub const JTRC_16BIT_TABLES: C2RustUnnamed_0 = 77;
pub const JMSG_VERSION: C2RustUnnamed_0 = 76;
pub const JMSG_COPYRIGHT: C2RustUnnamed_0 = 75;
pub const JERR_XMS_WRITE: C2RustUnnamed_0 = 74;
pub const JERR_XMS_READ: C2RustUnnamed_0 = 73;
pub const JERR_WIDTH_OVERFLOW: C2RustUnnamed_0 = 72;
pub const JERR_VIRTUAL_BUG: C2RustUnnamed_0 = 71;
pub const JERR_UNKNOWN_MARKER: C2RustUnnamed_0 = 70;
pub const JERR_TOO_LITTLE_DATA: C2RustUnnamed_0 = 69;
pub const JERR_TFILE_WRITE: C2RustUnnamed_0 = 68;
pub const JERR_TFILE_SEEK: C2RustUnnamed_0 = 67;
pub const JERR_TFILE_READ: C2RustUnnamed_0 = 66;
pub const JERR_TFILE_CREATE: C2RustUnnamed_0 = 65;
pub const JERR_SOS_NO_SOF: C2RustUnnamed_0 = 64;
pub const JERR_SOI_DUPLICATE: C2RustUnnamed_0 = 63;
pub const JERR_SOF_UNSUPPORTED: C2RustUnnamed_0 = 62;
pub const JERR_SOF_NO_SOS: C2RustUnnamed_0 = 61;
pub const JERR_SOF_DUPLICATE: C2RustUnnamed_0 = 60;
pub const JERR_QUANT_MANY_COLORS: C2RustUnnamed_0 = 59;
pub const JERR_QUANT_FEW_COLORS: C2RustUnnamed_0 = 58;
pub const JERR_QUANT_COMPONENTS: C2RustUnnamed_0 = 57;
pub const JERR_OUT_OF_MEMORY: C2RustUnnamed_0 = 56;
pub const JERR_NO_SOI: C2RustUnnamed_0 = 55;
pub const JERR_NO_QUANT_TABLE: C2RustUnnamed_0 = 54;
pub const JERR_NO_IMAGE: C2RustUnnamed_0 = 53;
pub const JERR_NO_HUFF_TABLE: C2RustUnnamed_0 = 52;
pub const JERR_NO_BACKING_STORE: C2RustUnnamed_0 = 51;
pub const JERR_NO_ARITH_TABLE: C2RustUnnamed_0 = 50;
pub const JERR_NOT_COMPILED: C2RustUnnamed_0 = 49;
pub const JERR_NOTIMPL: C2RustUnnamed_0 = 48;
pub const JERR_MODE_CHANGE: C2RustUnnamed_0 = 47;
pub const JERR_MISSING_DATA: C2RustUnnamed_0 = 46;
pub const JERR_MISMATCHED_QUANT_TABLE: C2RustUnnamed_0 = 45;
pub const JERR_INPUT_EOF: C2RustUnnamed_0 = 44;
pub const JERR_INPUT_EMPTY: C2RustUnnamed_0 = 43;
pub const JERR_IMAGE_TOO_BIG: C2RustUnnamed_0 = 42;
pub const JERR_HUFF_MISSING_CODE: C2RustUnnamed_0 = 41;
pub const JERR_HUFF_CLEN_OVERFLOW: C2RustUnnamed_0 = 40;
pub const JERR_FRACT_SAMPLE_NOTIMPL: C2RustUnnamed_0 = 39;
pub const JERR_FILE_WRITE: C2RustUnnamed_0 = 38;
pub const JERR_FILE_READ: C2RustUnnamed_0 = 37;
pub const JERR_EOI_EXPECTED: C2RustUnnamed_0 = 36;
pub const JERR_EMS_WRITE: C2RustUnnamed_0 = 35;
pub const JERR_EMS_READ: C2RustUnnamed_0 = 34;
pub const JERR_EMPTY_IMAGE: C2RustUnnamed_0 = 33;
pub const JERR_DQT_INDEX: C2RustUnnamed_0 = 32;
pub const JERR_DHT_INDEX: C2RustUnnamed_0 = 31;
pub const JERR_DAC_VALUE: C2RustUnnamed_0 = 30;
pub const JERR_DAC_INDEX: C2RustUnnamed_0 = 29;
pub const JERR_COMPONENT_COUNT: C2RustUnnamed_0 = 27;
pub const JERR_CCIR601_NOTIMPL: C2RustUnnamed_0 = 26;
pub const JERR_CANT_SUSPEND: C2RustUnnamed_0 = 25;
pub const JERR_BUFFER_SIZE: C2RustUnnamed_0 = 24;
pub const JERR_BAD_VIRTUAL_ACCESS: C2RustUnnamed_0 = 23;
pub const JERR_BAD_STRUCT_SIZE: C2RustUnnamed_0 = 22;
pub const JERR_BAD_STATE: C2RustUnnamed_0 = 21;
pub const JERR_BAD_SCAN_SCRIPT: C2RustUnnamed_0 = 20;
pub const JERR_BAD_SAMPLING: C2RustUnnamed_0 = 19;
pub const JERR_BAD_PROG_SCRIPT: C2RustUnnamed_0 = 18;
pub const JERR_BAD_PROGRESSION: C2RustUnnamed_0 = 17;
pub const JERR_BAD_PRECISION: C2RustUnnamed_0 = 16;
pub const JERR_BAD_POOL_ID: C2RustUnnamed_0 = 15;
pub const JERR_BAD_MCU_SIZE: C2RustUnnamed_0 = 14;
pub const JERR_BAD_LIB_VERSION: C2RustUnnamed_0 = 13;
pub const JERR_BAD_LENGTH: C2RustUnnamed_0 = 12;
pub const JERR_BAD_IN_COLORSPACE: C2RustUnnamed_0 = 10;
pub const JERR_BAD_HUFF_TABLE: C2RustUnnamed_0 = 9;
pub const JERR_BAD_DROP_SAMPLING: C2RustUnnamed_0 = 8;
pub const JERR_BAD_DCTSIZE: C2RustUnnamed_0 = 7;
pub const JERR_BAD_DCT_COEF: C2RustUnnamed_0 = 6;
pub const JERR_BAD_CROP_SPEC: C2RustUnnamed_0 = 5;
pub const JERR_BAD_COMPONENT_ID: C2RustUnnamed_0 = 4;
pub const JERR_BAD_BUFFER_MODE: C2RustUnnamed_0 = 3;
pub const JERR_BAD_ALLOC_CHUNK: C2RustUnnamed_0 = 2;
pub const JERR_BAD_ALIGN_TYPE: C2RustUnnamed_0 = 1;
pub const JMSG_NOMESSAGE: C2RustUnnamed_0 = 0;
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
static mut rgb_red: [::core::ffi::c_int; 17] = [
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    RGB_RED,
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    EXT_RGB_RED,
    EXT_RGBX_RED,
    EXT_BGR_RED,
    EXT_BGRX_RED,
    EXT_XBGR_RED,
    EXT_XRGB_RED,
    EXT_RGBX_RED,
    EXT_BGRX_RED,
    EXT_XBGR_RED,
    EXT_XRGB_RED,
    -(1 as ::core::ffi::c_int),
];
static mut rgb_green: [::core::ffi::c_int; 17] = [
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    RGB_GREEN,
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    EXT_RGB_GREEN,
    EXT_RGBX_GREEN,
    EXT_BGR_GREEN,
    EXT_BGRX_GREEN,
    EXT_XBGR_GREEN,
    EXT_XRGB_GREEN,
    EXT_RGBX_GREEN,
    EXT_BGRX_GREEN,
    EXT_XBGR_GREEN,
    EXT_XRGB_GREEN,
    -(1 as ::core::ffi::c_int),
];
static mut rgb_blue: [::core::ffi::c_int; 17] = [
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    RGB_BLUE,
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    EXT_RGB_BLUE,
    EXT_RGBX_BLUE,
    EXT_BGR_BLUE,
    EXT_BGRX_BLUE,
    EXT_XBGR_BLUE,
    EXT_XRGB_BLUE,
    EXT_RGBX_BLUE,
    EXT_BGRX_BLUE,
    EXT_XBGR_BLUE,
    EXT_XRGB_BLUE,
    -(1 as ::core::ffi::c_int),
];
static mut rgb_pixelsize: [::core::ffi::c_int; 17] = [
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    RGB_PIXELSIZE,
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    EXT_RGB_PIXELSIZE,
    EXT_RGBX_PIXELSIZE,
    EXT_BGR_PIXELSIZE,
    EXT_BGRX_PIXELSIZE,
    EXT_XBGR_PIXELSIZE,
    EXT_XRGB_PIXELSIZE,
    EXT_RGBX_PIXELSIZE,
    EXT_BGRX_PIXELSIZE,
    EXT_XBGR_PIXELSIZE,
    EXT_XRGB_PIXELSIZE,
    -(1 as ::core::ffi::c_int),
];
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SCALEBITS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ONE_HALF: JLONG =
    (1 as ::core::ffi::c_int as JLONG) << SCALEBITS - 1 as ::core::ffi::c_int;
pub const R_Y_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_Y_OFF: ::core::ffi::c_int =
    1 as ::core::ffi::c_int * (MAXJSAMPLE + 1 as ::core::ffi::c_int);
pub const B_Y_OFF: ::core::ffi::c_int =
    2 as ::core::ffi::c_int * (MAXJSAMPLE + 1 as ::core::ffi::c_int);
pub const TABLE_SIZE: ::core::ffi::c_int =
    3 as ::core::ffi::c_int * (MAXJSAMPLE + 1 as ::core::ffi::c_int);
#[inline(always)]
unsafe extern "C" fn ycc_rgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh145 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh145;
        col = 0 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as ::core::ffi::c_int;
            cb = *inptr1.offset(col as isize) as ::core::ffi::c_int;
            cr = *inptr2.offset(col as isize) as ::core::ffi::c_int;
            *outptr.offset(RGB_RED as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn gray_rgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh117 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(fresh117 as isize);
        let fresh118 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh118;
        col = 0 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh119 = *outptr.offset(RGB_BLUE as isize);
            *fresh119 = *inptr.offset(col as isize);
            let ref mut fresh120 = *outptr.offset(RGB_GREEN as isize);
            *fresh120 = *fresh119;
            *outptr.offset(RGB_RED as isize) = *fresh120;
            outptr = outptr.offset(RGB_PIXELSIZE as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_rgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh110 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh110;
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE as isize) = *inptr2.offset(col as isize);
            outptr = outptr.offset(RGB_PIXELSIZE as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn ycc_extrgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh151 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh151;
        col = 0 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as ::core::ffi::c_int;
            cb = *inptr1.offset(col as isize) as ::core::ffi::c_int;
            cr = *inptr2.offset(col as isize) as ::core::ffi::c_int;
            *outptr.offset(RGB_RED_5 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_5 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE_5 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_5 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn gray_extrgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh141 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(fresh141 as isize);
        let fresh142 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh142;
        col = 0 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh143 = *outptr.offset(RGB_BLUE_5 as isize);
            *fresh143 = *inptr.offset(col as isize);
            let ref mut fresh144 = *outptr.offset(RGB_GREEN_5 as isize);
            *fresh144 = *fresh143;
            *outptr.offset(RGB_RED_5 as isize) = *fresh144;
            outptr = outptr.offset(RGB_PIXELSIZE_5 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_extrgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh116 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh116;
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_5 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_5 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_5 as isize) = *inptr2.offset(col as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_5 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn ycc_extrgbx_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh150 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh150;
        col = 0 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as ::core::ffi::c_int;
            cb = *inptr1.offset(col as isize) as ::core::ffi::c_int;
            cr = *inptr2.offset(col as isize) as ::core::ffi::c_int;
            *outptr.offset(RGB_RED_4 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_4 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE_4 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            *outptr.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn gray_extrgbx_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh137 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(fresh137 as isize);
        let fresh138 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh138;
        col = 0 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh139 = *outptr.offset(RGB_BLUE_4 as isize);
            *fresh139 = *inptr.offset(col as isize);
            let ref mut fresh140 = *outptr.offset(RGB_GREEN_4 as isize);
            *fresh140 = *fresh139;
            *outptr.offset(RGB_RED_4 as isize) = *fresh140;
            *outptr.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_extrgbx_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh115 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh115;
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_4 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_4 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_4 as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA_2 as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_4 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn ycc_extbgr_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh149 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh149;
        col = 0 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as ::core::ffi::c_int;
            cb = *inptr1.offset(col as isize) as ::core::ffi::c_int;
            cr = *inptr2.offset(col as isize) as ::core::ffi::c_int;
            *outptr.offset(RGB_RED_3 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_3 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE_3 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn gray_extbgr_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh133 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(fresh133 as isize);
        let fresh134 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh134;
        col = 0 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh135 = *outptr.offset(RGB_BLUE_3 as isize);
            *fresh135 = *inptr.offset(col as isize);
            let ref mut fresh136 = *outptr.offset(RGB_GREEN_3 as isize);
            *fresh136 = *fresh135;
            *outptr.offset(RGB_RED_3 as isize) = *fresh136;
            outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_extbgr_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh114 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh114;
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_3 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_3 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_3 as isize) = *inptr2.offset(col as isize);
            outptr = outptr.offset(RGB_PIXELSIZE_3 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn ycc_extbgrx_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh148 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh148;
        col = 0 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as ::core::ffi::c_int;
            cb = *inptr1.offset(col as isize) as ::core::ffi::c_int;
            cr = *inptr2.offset(col as isize) as ::core::ffi::c_int;
            *outptr.offset(RGB_RED_2 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_2 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE_2 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            *outptr.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn gray_extbgrx_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh129 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(fresh129 as isize);
        let fresh130 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh130;
        col = 0 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh131 = *outptr.offset(RGB_BLUE_2 as isize);
            *fresh131 = *inptr.offset(col as isize);
            let ref mut fresh132 = *outptr.offset(RGB_GREEN_2 as isize);
            *fresh132 = *fresh131;
            *outptr.offset(RGB_RED_2 as isize) = *fresh132;
            *outptr.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_extbgrx_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh113 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh113;
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_2 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_2 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_2 as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA_1 as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_2 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn ycc_extxbgr_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh147 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh147;
        col = 0 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as ::core::ffi::c_int;
            cb = *inptr1.offset(col as isize) as ::core::ffi::c_int;
            cr = *inptr2.offset(col as isize) as ::core::ffi::c_int;
            *outptr.offset(RGB_RED_1 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_1 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE_1 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            *outptr.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn gray_extxbgr_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh125 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(fresh125 as isize);
        let fresh126 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh126;
        col = 0 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh127 = *outptr.offset(RGB_BLUE_1 as isize);
            *fresh127 = *inptr.offset(col as isize);
            let ref mut fresh128 = *outptr.offset(RGB_GREEN_1 as isize);
            *fresh128 = *fresh127;
            *outptr.offset(RGB_RED_1 as isize) = *fresh128;
            *outptr.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_extxbgr_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh112 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh112;
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_1 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_1 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_1 as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA_0 as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_1 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn ycc_extxrgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh146 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh146;
        col = 0 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as ::core::ffi::c_int;
            cb = *inptr1.offset(col as isize) as ::core::ffi::c_int;
            cr = *inptr2.offset(col as isize) as ::core::ffi::c_int;
            *outptr.offset(RGB_RED_0 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(RGB_GREEN_0 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            );
            *outptr.offset(RGB_BLUE_0 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            *outptr.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn gray_extxrgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh121 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(fresh121 as isize);
        let fresh122 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh122;
        col = 0 as JDIMENSION;
        while col < num_cols {
            let ref mut fresh123 = *outptr.offset(RGB_BLUE_0 as isize);
            *fresh123 = *inptr.offset(col as isize);
            let ref mut fresh124 = *outptr.offset(RGB_GREEN_0 as isize);
            *fresh124 = *fresh123;
            *outptr.offset(RGB_RED_0 as isize) = *fresh124;
            *outptr.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_extxrgb_convert_internal(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh111 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh111;
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(RGB_RED_0 as isize) = *inptr0.offset(col as isize);
            *outptr.offset(RGB_GREEN_0 as isize) = *inptr1.offset(col as isize);
            *outptr.offset(RGB_BLUE_0 as isize) = *inptr2.offset(col as isize);
            *outptr.offset(RGB_ALPHA as isize) = MAXJSAMPLE as JSAMPLE;
            outptr = outptr.offset(RGB_PIXELSIZE_0 as isize);
            col = col.wrapping_add(1);
        }
    }
}
pub const RGB_RED_5: ::core::ffi::c_int = EXT_RGB_RED;
pub const RGB_GREEN_5: ::core::ffi::c_int = EXT_RGB_GREEN;
pub const RGB_BLUE_5: ::core::ffi::c_int = EXT_RGB_BLUE;
pub const RGB_PIXELSIZE_5: ::core::ffi::c_int = EXT_RGB_PIXELSIZE;
#[inline(always)]
unsafe extern "C" fn ycc_rgb565_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: ::core::ffi::c_uint = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh90 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh90;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh91 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh91 as ::core::ffi::c_int;
            let fresh92 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh92 as ::core::ffi::c_int;
            let fresh93 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh93 as ::core::ffi::c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize)
                as ::core::ffi::c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize)
                as ::core::ffi::c_uint;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh94 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh94 as ::core::ffi::c_int;
            let fresh95 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh95 as ::core::ffi::c_int;
            let fresh96 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh96 as ::core::ffi::c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize)
                as ::core::ffi::c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize)
                as ::core::ffi::c_uint;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            let fresh97 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh97 as ::core::ffi::c_int;
            let fresh98 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh98 as ::core::ffi::c_int;
            let fresh99 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh99 as ::core::ffi::c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize)
                as ::core::ffi::c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize)
                as ::core::ffi::c_uint;
            rgb = ((r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as JLONG
                | rgb;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            y = *inptr0 as ::core::ffi::c_int;
            cb = *inptr1 as ::core::ffi::c_int;
            cr = *inptr2 as ::core::ffi::c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize)
                as ::core::ffi::c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize)
                as ::core::ffi::c_uint;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn ycc_rgb565D_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    let mut d0: JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as JDIMENSION) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: ::core::ffi::c_uint = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh40 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh40;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh41 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh41 as ::core::ffi::c_int;
            let fresh42 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh42 as ::core::ffi::c_int;
            let fresh43 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh43 as ::core::ffi::c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as JLONG
                    + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh44 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh44 as ::core::ffi::c_int;
            let fresh45 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh45 as ::core::ffi::c_int;
            let fresh46 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh46 as ::core::ffi::c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as JLONG
                    + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            let fresh47 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh47 as ::core::ffi::c_int;
            let fresh48 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh48 as ::core::ffi::c_int;
            let fresh49 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh49 as ::core::ffi::c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as JLONG
                    + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            rgb = ((r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as JLONG
                | rgb;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            y = *inptr0 as ::core::ffi::c_int;
            cb = *inptr1 as ::core::ffi::c_int;
            cr = *inptr2 as ::core::ffi::c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as JLONG
                    + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_rgb565_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: ::core::ffi::c_uint = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh60 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh60;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh61 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh61 as ::core::ffi::c_uint;
            let fresh62 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh62 as ::core::ffi::c_uint;
            let fresh63 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh63 as ::core::ffi::c_uint;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh64 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh64 as ::core::ffi::c_uint;
            let fresh65 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh65 as ::core::ffi::c_uint;
            let fresh66 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh66 as ::core::ffi::c_uint;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            let fresh67 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh67 as ::core::ffi::c_uint;
            let fresh68 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh68 as ::core::ffi::c_uint;
            let fresh69 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh69 as ::core::ffi::c_uint;
            rgb = ((r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as JLONG
                | rgb;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            r = *inptr0 as ::core::ffi::c_uint;
            g = *inptr1 as ::core::ffi::c_uint;
            b = *inptr2 as ::core::ffi::c_uint;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_rgb565D_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut d0: JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as JDIMENSION) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: ::core::ffi::c_uint = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh10 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh10;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh11 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset((*fresh11 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            let fresh12 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh12 as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            let fresh13 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset((*fresh13 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh14 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset((*fresh14 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            let fresh15 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh15 as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            let fresh16 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset((*fresh16 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            let fresh17 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset((*fresh17 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            let fresh18 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh18 as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            let fresh19 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset((*fresh19 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            rgb = ((r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as JLONG
                | rgb;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            r = *range_limit.offset((*inptr0 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            g = *range_limit.offset(
                (*inptr1 as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset((*inptr2 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = (r << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | b >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn gray_rgb565_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let fresh80 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(fresh80 as isize);
        let fresh81 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh81;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh82 = inptr;
            inptr = inptr.offset(1);
            g = *fresh82 as ::core::ffi::c_uint;
            rgb = (g << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | g >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh83 = inptr;
            inptr = inptr.offset(1);
            g = *fresh83 as ::core::ffi::c_uint;
            rgb = (g << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | g >> 3 as ::core::ffi::c_int) as JLONG;
            let fresh84 = inptr;
            inptr = inptr.offset(1);
            g = *fresh84 as ::core::ffi::c_uint;
            rgb = ((g << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | g >> 3 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as JLONG
                | rgb;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            g = *inptr as ::core::ffi::c_uint;
            rgb = (g << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | g >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn gray_rgb565D_convert_le(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut d0: JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as JDIMENSION) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let fresh30 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(fresh30 as isize);
        let fresh31 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh31;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh32 = inptr;
            inptr = inptr.offset(1);
            g = *fresh32 as ::core::ffi::c_uint;
            g = *range_limit.offset((g as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = (g << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | g >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh33 = inptr;
            inptr = inptr.offset(1);
            g = *fresh33 as ::core::ffi::c_uint;
            g = *range_limit.offset((g as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = (g << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | g >> 3 as ::core::ffi::c_int) as JLONG;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            let fresh34 = inptr;
            inptr = inptr.offset(1);
            g = *fresh34 as ::core::ffi::c_uint;
            g = *range_limit.offset((g as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = ((g << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | g >> 3 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as JLONG
                | rgb;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            g = *inptr as ::core::ffi::c_uint;
            g = *range_limit.offset((g as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = (g << 8 as ::core::ffi::c_int & 0xf800 as ::core::ffi::c_uint
                | g << 3 as ::core::ffi::c_int & 0x7e0 as ::core::ffi::c_uint
                | g >> 3 as ::core::ffi::c_int) as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn ycc_rgb565_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: ::core::ffi::c_uint = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh100 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh100;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh101 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh101 as ::core::ffi::c_int;
            let fresh102 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh102 as ::core::ffi::c_int;
            let fresh103 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh103 as ::core::ffi::c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize)
                as ::core::ffi::c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize)
                as ::core::ffi::c_uint;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh104 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh104 as ::core::ffi::c_int;
            let fresh105 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh105 as ::core::ffi::c_int;
            let fresh106 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh106 as ::core::ffi::c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize)
                as ::core::ffi::c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize)
                as ::core::ffi::c_uint;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            let fresh107 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh107 as ::core::ffi::c_int;
            let fresh108 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh108 as ::core::ffi::c_int;
            let fresh109 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh109 as ::core::ffi::c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize)
                as ::core::ffi::c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize)
                as ::core::ffi::c_uint;
            rgb = rgb << 16 as ::core::ffi::c_int
                | (r & 0xf8 as ::core::ffi::c_uint
                    | g >> 5 as ::core::ffi::c_int
                    | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                    | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                    as JLONG;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            y = *inptr0 as ::core::ffi::c_int;
            cb = *inptr1 as ::core::ffi::c_int;
            cr = *inptr2 as ::core::ffi::c_int;
            r = *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize)
                as ::core::ffi::c_uint;
            g = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize)
                as ::core::ffi::c_uint;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn ycc_rgb565D_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    let mut d0: JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as JDIMENSION) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: ::core::ffi::c_uint = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh50 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh50;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh51 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh51 as ::core::ffi::c_int;
            let fresh52 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh52 as ::core::ffi::c_int;
            let fresh53 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh53 as ::core::ffi::c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as JLONG
                    + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh54 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh54 as ::core::ffi::c_int;
            let fresh55 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh55 as ::core::ffi::c_int;
            let fresh56 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh56 as ::core::ffi::c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as JLONG
                    + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            let fresh57 = inptr0;
            inptr0 = inptr0.offset(1);
            y = *fresh57 as ::core::ffi::c_int;
            let fresh58 = inptr1;
            inptr1 = inptr1.offset(1);
            cb = *fresh58 as ::core::ffi::c_int;
            let fresh59 = inptr2;
            inptr2 = inptr2.offset(1);
            cr = *fresh59 as ::core::ffi::c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as JLONG
                    + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            rgb = rgb << 16 as ::core::ffi::c_int
                | (r & 0xf8 as ::core::ffi::c_uint
                    | g >> 5 as ::core::ffi::c_int
                    | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                    | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                    as JLONG;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            y = *inptr0 as ::core::ffi::c_int;
            cb = *inptr1 as ::core::ffi::c_int;
            cr = *inptr2 as ::core::ffi::c_int;
            r = *range_limit.offset(
                ((y + *Crrtab.offset(cr as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            g = *range_limit.offset(
                ((y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                    >> 16 as ::core::ffi::c_int) as ::core::ffi::c_int) as JLONG
                    + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset(
                ((y + *Cbbtab.offset(cb as isize)) as JLONG + (d0 & 0xff as JLONG)) as isize,
            ) as ::core::ffi::c_uint;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_rgb565_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: ::core::ffi::c_uint = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh70 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh70;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh71 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh71 as ::core::ffi::c_uint;
            let fresh72 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh72 as ::core::ffi::c_uint;
            let fresh73 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh73 as ::core::ffi::c_uint;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh74 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh74 as ::core::ffi::c_uint;
            let fresh75 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh75 as ::core::ffi::c_uint;
            let fresh76 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh76 as ::core::ffi::c_uint;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            let fresh77 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *fresh77 as ::core::ffi::c_uint;
            let fresh78 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *fresh78 as ::core::ffi::c_uint;
            let fresh79 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *fresh79 as ::core::ffi::c_uint;
            rgb = rgb << 16 as ::core::ffi::c_int
                | (r & 0xf8 as ::core::ffi::c_uint
                    | g >> 5 as ::core::ffi::c_int
                    | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                    | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                    as JLONG;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            r = *inptr0 as ::core::ffi::c_uint;
            g = *inptr1 as ::core::ffi::c_uint;
            b = *inptr2 as ::core::ffi::c_uint;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_rgb565D_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut d0: JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as JDIMENSION) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut r: ::core::ffi::c_uint = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh20 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh20;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh21 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset((*fresh21 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            let fresh22 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh22 as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            let fresh23 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset((*fresh23 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh24 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset((*fresh24 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            let fresh25 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh25 as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            let fresh26 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset((*fresh26 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            let fresh27 = inptr0;
            inptr0 = inptr0.offset(1);
            r = *range_limit.offset((*fresh27 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            let fresh28 = inptr1;
            inptr1 = inptr1.offset(1);
            g = *range_limit.offset(
                (*fresh28 as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            let fresh29 = inptr2;
            inptr2 = inptr2.offset(1);
            b = *range_limit.offset((*fresh29 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            rgb = rgb << 16 as ::core::ffi::c_int
                | (r & 0xf8 as ::core::ffi::c_uint
                    | g >> 5 as ::core::ffi::c_int
                    | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                    | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                    as JLONG;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            r = *range_limit.offset((*inptr0 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            g = *range_limit.offset(
                (*inptr1 as JLONG + ((d0 & 0xff as JLONG) >> 1 as ::core::ffi::c_int)) as isize,
            ) as ::core::ffi::c_uint;
            b = *range_limit.offset((*inptr2 as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = (r & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | b << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn gray_rgb565_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let fresh85 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(fresh85 as isize);
        let fresh86 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh86;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh87 = inptr;
            inptr = inptr.offset(1);
            g = *fresh87 as ::core::ffi::c_uint;
            rgb = (g & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | g << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh88 = inptr;
            inptr = inptr.offset(1);
            g = *fresh88 as ::core::ffi::c_uint;
            rgb = (g & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | g << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            let fresh89 = inptr;
            inptr = inptr.offset(1);
            g = *fresh89 as ::core::ffi::c_uint;
            rgb = rgb << 16 as ::core::ffi::c_int
                | (g & 0xf8 as ::core::ffi::c_uint
                    | g >> 5 as ::core::ffi::c_int
                    | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                    | g << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                    as JLONG;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            g = *inptr as ::core::ffi::c_uint;
            rgb = (g & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | g << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn gray_rgb565D_convert_be(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut d0: JLONG =
        dither_matrix[((*cinfo).output_scanline & DITHER_MASK as JDIMENSION) as usize];
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut rgb: JLONG = 0;
        let mut g: ::core::ffi::c_uint = 0;
        let fresh35 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(fresh35 as isize);
        let fresh36 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh36;
        if outptr as size_t & 3 as size_t != 0 {
            let fresh37 = inptr;
            inptr = inptr.offset(1);
            g = *fresh37 as ::core::ffi::c_uint;
            g = *range_limit.offset((g as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = (g & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | g << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
            outptr = outptr.offset(2 as ::core::ffi::c_int as isize);
            num_cols = num_cols.wrapping_sub(1);
        }
        col = 0 as JDIMENSION;
        while col < num_cols >> 1 as ::core::ffi::c_int {
            let fresh38 = inptr;
            inptr = inptr.offset(1);
            g = *fresh38 as ::core::ffi::c_uint;
            g = *range_limit.offset((g as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = (g & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | g << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            let fresh39 = inptr;
            inptr = inptr.offset(1);
            g = *fresh39 as ::core::ffi::c_uint;
            g = *range_limit.offset((g as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = rgb << 16 as ::core::ffi::c_int
                | (g & 0xf8 as ::core::ffi::c_uint
                    | g >> 5 as ::core::ffi::c_int
                    | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                    | g << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                    as JLONG;
            d0 = (d0 & 0xff as JLONG) << 24 as ::core::ffi::c_int
                | d0 >> 8 as ::core::ffi::c_int & 0xffffff as JLONG;
            *(outptr as *mut ::core::ffi::c_int) = rgb as ::core::ffi::c_int;
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
        if num_cols & 1 as JDIMENSION != 0 {
            g = *inptr as ::core::ffi::c_uint;
            g = *range_limit.offset((g as JLONG + (d0 & 0xff as JLONG)) as isize)
                as ::core::ffi::c_uint;
            rgb = (g & 0xf8 as ::core::ffi::c_uint
                | g >> 5 as ::core::ffi::c_int
                | g << 11 as ::core::ffi::c_int & 0xe000 as ::core::ffi::c_uint
                | g << 5 as ::core::ffi::c_int & 0x1f00 as ::core::ffi::c_uint)
                as JLONG;
            *(outptr as *mut INT16) = rgb as INT16;
        }
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
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut i: ::core::ffi::c_int = 0;
    let mut x: JLONG = 0;
    (*cconvert).Cr_r_tab = Some(
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
    (*cconvert).Cb_b_tab = Some(
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
    (*cconvert).Cr_g_tab = Some(
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
    (*cconvert).Cb_g_tab = Some(
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
        *(*cconvert).Cr_r_tab.offset(i as isize) = ((1.40200f64
            * ((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int) as ::core::ffi::c_double
            + 0.5f64) as JLONG
            * x
            + ((1 as ::core::ffi::c_int as JLONG)
                << 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 16 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        *(*cconvert).Cb_b_tab.offset(i as isize) = ((1.77200f64
            * ((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int) as ::core::ffi::c_double
            + 0.5f64) as JLONG
            * x
            + ((1 as ::core::ffi::c_int as JLONG)
                << 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 16 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        *(*cconvert).Cr_g_tab.offset(i as isize) = -((0.71414f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG)
            * x;
        *(*cconvert).Cb_g_tab.offset(i as isize) = -((0.34414f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG)
            * x
            + ONE_HALF;
        i += 1;
        x += 1;
    }
}
unsafe extern "C" fn ycc_rgb_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    match (*cinfo).out_color_space as ::core::ffi::c_uint {
        6 => {
            ycc_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            ycc_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            ycc_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            ycc_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            ycc_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            ycc_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            ycc_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
unsafe extern "C" fn build_rgb_y_table(mut cinfo: j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut rgb_y_tab: *mut JLONG = ::core::ptr::null_mut::<JLONG>();
    let mut i: JLONG = 0;
    rgb_y_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (TABLE_SIZE as size_t).wrapping_mul(::core::mem::size_of::<JLONG>() as size_t),
    ) as *mut JLONG;
    (*cconvert).rgb_y_tab = rgb_y_tab;
    i = 0 as JLONG;
    while i <= MAXJSAMPLE as JLONG {
        *rgb_y_tab.offset((i + R_Y_OFF as JLONG) as isize) = (0.29900f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG
            * i;
        *rgb_y_tab.offset((i + G_Y_OFF as JLONG) as isize) = (0.58700f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG
            * i;
        *rgb_y_tab.offset((i + B_Y_OFF as JLONG) as isize) = (0.11400f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG
            * i
            + ONE_HALF;
        i += 1;
    }
}
unsafe extern "C" fn rgb_gray_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_y_tab;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh152 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh152;
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr0.offset(col as isize) as ::core::ffi::c_int;
            g = *inptr1.offset(col as isize) as ::core::ffi::c_int;
            b = *inptr2.offset(col as isize) as ::core::ffi::c_int;
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn null_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr3: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_components: ::core::ffi::c_int = (*cinfo).num_components;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut ci: ::core::ffi::c_int = 0;
    if num_components == 3 as ::core::ffi::c_int {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as ::core::ffi::c_int) {
                break;
            }
            inptr0 =
                *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
            inptr1 =
                *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
            inptr2 =
                *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh0 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh0;
            col = 0 as JDIMENSION;
            while col < num_cols {
                let fresh1 = outptr;
                outptr = outptr.offset(1);
                *fresh1 = *inptr0.offset(col as isize);
                let fresh2 = outptr;
                outptr = outptr.offset(1);
                *fresh2 = *inptr1.offset(col as isize);
                let fresh3 = outptr;
                outptr = outptr.offset(1);
                *fresh3 = *inptr2.offset(col as isize);
                col = col.wrapping_add(1);
            }
        }
    } else if num_components == 4 as ::core::ffi::c_int {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as ::core::ffi::c_int) {
                break;
            }
            inptr0 =
                *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
            inptr1 =
                *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
            inptr2 =
                *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
            inptr3 =
                *(*input_buf.offset(3 as ::core::ffi::c_int as isize)).offset(input_row as isize);
            input_row = input_row.wrapping_add(1);
            let fresh4 = output_buf;
            output_buf = output_buf.offset(1);
            outptr = *fresh4;
            col = 0 as JDIMENSION;
            while col < num_cols {
                let fresh5 = outptr;
                outptr = outptr.offset(1);
                *fresh5 = *inptr0.offset(col as isize);
                let fresh6 = outptr;
                outptr = outptr.offset(1);
                *fresh6 = *inptr1.offset(col as isize);
                let fresh7 = outptr;
                outptr = outptr.offset(1);
                *fresh7 = *inptr2.offset(col as isize);
                let fresh8 = outptr;
                outptr = outptr.offset(1);
                *fresh8 = *inptr3.offset(col as isize);
                col = col.wrapping_add(1);
            }
        }
    } else {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as ::core::ffi::c_int) {
                break;
            }
            ci = 0 as ::core::ffi::c_int;
            while ci < num_components {
                inptr = *(*input_buf.offset(ci as isize)).offset(input_row as isize);
                outptr = *output_buf;
                col = 0 as JDIMENSION;
                while col < num_cols {
                    *outptr.offset(ci as isize) = *inptr.offset(col as isize);
                    outptr = outptr.offset(num_components as isize);
                    col = col.wrapping_add(1);
                }
                ci += 1;
            }
            output_buf = output_buf.offset(1);
            input_row = input_row.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn grayscale_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    jcopy_sample_rows(
        *input_buf.offset(0 as ::core::ffi::c_int as isize),
        input_row as ::core::ffi::c_int,
        output_buf,
        0 as ::core::ffi::c_int,
        num_rows,
        (*cinfo).output_width,
    );
}
unsafe extern "C" fn gray_rgb_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    match (*cinfo).out_color_space as ::core::ffi::c_uint {
        6 => {
            gray_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            gray_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            gray_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            gray_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            gray_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            gray_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            gray_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
unsafe extern "C" fn rgb_rgb_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    match (*cinfo).out_color_space as ::core::ffi::c_uint {
        6 => {
            rgb_extrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        7 | 12 => {
            rgb_extrgbx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        8 => {
            rgb_extbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        9 | 13 => {
            rgb_extbgrx_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        10 | 14 => {
            rgb_extxbgr_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        11 | 15 => {
            rgb_extxrgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
        _ => {
            rgb_rgb_convert_internal(cinfo, input_buf, input_row, output_buf, num_rows);
        }
    };
}
unsafe extern "C" fn ycck_cmyk_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: ::core::ffi::c_int = 0;
    let mut cb: ::core::ffi::c_int = 0;
    let mut cr: ::core::ffi::c_int = 0;
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr3: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut ::core::ffi::c_int = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut ::core::ffi::c_int = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut JLONG = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut JLONG = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        inptr3 = *(*input_buf.offset(3 as ::core::ffi::c_int as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh9 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh9;
        col = 0 as JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as ::core::ffi::c_int;
            cb = *inptr1.offset(col as isize) as ::core::ffi::c_int;
            cr = *inptr2.offset(col as isize) as ::core::ffi::c_int;
            *outptr.offset(0 as ::core::ffi::c_int as isize) =
                *range_limit.offset((MAXJSAMPLE - (y + *Crrtab.offset(cr as isize))) as isize);
            *outptr.offset(1 as ::core::ffi::c_int as isize) = *range_limit.offset(
                (MAXJSAMPLE
                    - (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize)
                        >> 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_int)) as isize,
            );
            *outptr.offset(2 as ::core::ffi::c_int as isize) =
                *range_limit.offset((MAXJSAMPLE - (y + *Cbbtab.offset(cb as isize))) as isize);
            *outptr.offset(3 as ::core::ffi::c_int as isize) = *inptr3.offset(col as isize);
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_add(1);
        }
    }
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
unsafe extern "C" fn ycc_rgb565_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    if is_big_endian() != 0 {
        ycc_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        ycc_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn ycc_rgb565D_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    if is_big_endian() != 0 {
        ycc_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        ycc_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn rgb_rgb565_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    if is_big_endian() != 0 {
        rgb_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        rgb_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn rgb_rgb565D_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    if is_big_endian() != 0 {
        rgb_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        rgb_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn gray_rgb565_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    if is_big_endian() != 0 {
        gray_rgb565_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        gray_rgb565_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn gray_rgb565D_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    if is_big_endian() != 0 {
        gray_rgb565D_convert_be(cinfo, input_buf, input_row, output_buf, num_rows);
    } else {
        gray_rgb565D_convert_le(cinfo, input_buf, input_row, output_buf, num_rows);
    };
}
unsafe extern "C" fn start_pass_dcolor(mut cinfo: j_decompress_ptr) {}
pub unsafe extern "C" fn jinit_color_deconverter(mut cinfo: j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = ::core::ptr::null_mut::<my_color_deconverter>();
    let mut ci: ::core::ffi::c_int = 0;
    cconvert = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<my_color_deconverter>() as size_t,
    ) as my_cconvert_ptr;
    (*cinfo).cconvert = cconvert as *mut jpeg_color_deconverter as *mut jpeg_color_deconverter;
    (*cconvert).pub_0.start_pass =
        Some(start_pass_dcolor as unsafe extern "C" fn(j_decompress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
    match (*cinfo).jpeg_color_space as ::core::ffi::c_uint {
        1 => {
            if (*cinfo).num_components != 1 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 | 3 => {
            if (*cinfo).num_components != 3 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 | 5 => {
            if (*cinfo).num_components != 4 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            if (*cinfo).num_components < 1 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    match (*cinfo).out_color_space as ::core::ffi::c_uint {
        1 => {
            (*cinfo).out_color_components = 1 as ::core::ffi::c_int;
            if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                    == JCS_YCbCr as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
                ci = 1 as ::core::ffi::c_int;
                while ci < (*cinfo).num_components {
                    (*(*cinfo).comp_info.offset(ci as isize)).component_needed = FALSE as boolean;
                    ci += 1;
                }
            } else if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_gray_convert
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
                build_rgb_y_table(cinfo);
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            (*cinfo).out_color_components = rgb_pixelsize[(*cinfo).out_color_space as usize];
            if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_YCbCr as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if jsimd_can_ycc_rgb() != 0 {
                    (*cconvert).pub_0.color_convert = Some(
                        jsimd_ycc_rgb_convert
                            as unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                        >;
                } else {
                    (*cconvert).pub_0.color_convert = Some(
                        ycc_rgb_convert
                            as unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                        >;
                    build_ycc_rgb_table(cinfo);
                }
            } else if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    gray_rgb_convert
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if rgb_red[(*cinfo).out_color_space as usize] == 0 as ::core::ffi::c_int
                    && rgb_green[(*cinfo).out_color_space as usize] == 1 as ::core::ffi::c_int
                    && rgb_blue[(*cinfo).out_color_space as usize] == 2 as ::core::ffi::c_int
                    && rgb_pixelsize[(*cinfo).out_color_space as usize] == 3 as ::core::ffi::c_int
                {
                    (*cconvert).pub_0.color_convert = Some(
                        null_convert
                            as unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                        >;
                } else {
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_rgb_convert
                            as unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                        >;
                }
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        16 => {
            (*cinfo).out_color_components = 3 as ::core::ffi::c_int;
            if (*cinfo).dither_mode as ::core::ffi::c_uint
                == JDITHER_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                    == JCS_YCbCr as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if jsimd_can_ycc_rgb565() != 0 {
                        (*cconvert).pub_0.color_convert = Some(
                            jsimd_ycc_rgb565_convert
                                as unsafe extern "C" fn(
                                    j_decompress_ptr,
                                    JSAMPIMAGE,
                                    JDIMENSION,
                                    JSAMPARRAY,
                                    ::core::ffi::c_int,
                                ) -> (),
                        )
                            as Option<
                                unsafe extern "C" fn(
                                    j_decompress_ptr,
                                    JSAMPIMAGE,
                                    JDIMENSION,
                                    JSAMPARRAY,
                                    ::core::ffi::c_int,
                                ) -> (),
                            >;
                    } else {
                        (*cconvert).pub_0.color_convert = Some(
                            ycc_rgb565_convert
                                as unsafe extern "C" fn(
                                    j_decompress_ptr,
                                    JSAMPIMAGE,
                                    JDIMENSION,
                                    JSAMPARRAY,
                                    ::core::ffi::c_int,
                                ) -> (),
                        )
                            as Option<
                                unsafe extern "C" fn(
                                    j_decompress_ptr,
                                    JSAMPIMAGE,
                                    JDIMENSION,
                                    JSAMPARRAY,
                                    ::core::ffi::c_int,
                                ) -> (),
                            >;
                        build_ycc_rgb_table(cinfo);
                    }
                } else if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                    == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*cconvert).pub_0.color_convert = Some(
                        gray_rgb565_convert
                            as unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                        >;
                } else if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                    == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_rgb565_convert
                            as unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                j_decompress_ptr,
                                JSAMPIMAGE,
                                JDIMENSION,
                                JSAMPARRAY,
                                ::core::ffi::c_int,
                            ) -> (),
                        >;
                } else {
                    (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
                }
            } else if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_YCbCr as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    ycc_rgb565D_convert
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
                build_ycc_rgb_table(cinfo);
            } else if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    gray_rgb565D_convert
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_rgb565D_convert
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 => {
            (*cinfo).out_color_components = 4 as ::core::ffi::c_int;
            if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_YCCK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    ycck_cmyk_convert
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
                build_ycc_rgb_table(cinfo);
            } else if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            if (*cinfo).out_color_space as ::core::ffi::c_uint
                == (*cinfo).jpeg_color_space as ::core::ffi::c_uint
            {
                (*cinfo).out_color_components = (*cinfo).num_components;
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_decompress_ptr,
                            JSAMPIMAGE,
                            JDIMENSION,
                            JSAMPARRAY,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    if (*cinfo).quantize_colors != 0 {
        (*cinfo).output_components = 1 as ::core::ffi::c_int;
    } else {
        (*cinfo).output_components = (*cinfo).out_color_components;
    };
}
