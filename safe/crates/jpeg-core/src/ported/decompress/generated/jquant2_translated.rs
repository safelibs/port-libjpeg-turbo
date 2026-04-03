#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}

extern "C" {
    fn jzero_far(target: *mut ::core::ffi::c_void, bytestozero: size_t);
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
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub alloc_large: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            size_t,
        ) -> *mut ::core::ffi::c_void,
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
    pub emit_message: Option<
        unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> (),
    >,
    pub output_message: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub format_message: Option<
        unsafe extern "C" fn(j_common_ptr, *mut ::core::ffi::c_char) -> (),
    >,
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
        unsafe extern "C" fn(
            j_decompress_ptr,
            JSAMPARRAY,
            JSAMPARRAY,
            ::core::ffi::c_int,
        ) -> (),
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
    pub decode_mcu: Option<
        unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean,
    >,
    pub insufficient_data: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_marker_reader {
    pub reset_marker_reader: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub read_markers: Option<
        unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int,
    >,
    pub read_restart_marker: jpeg_marker_parser_method,
    pub saw_SOI: boolean,
    pub saw_SOF: boolean,
    pub next_restart_num: ::core::ffi::c_int,
    pub discarded_bytes: ::core::ffi::c_uint,
}
pub type jpeg_marker_parser_method = Option<
    unsafe extern "C" fn(j_decompress_ptr) -> boolean,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_input_controller {
    pub consume_input: Option<
        unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int,
    >,
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
    pub consume_data: Option<
        unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int,
    >,
    pub start_output_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub decompress_data: Option<
        unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE) -> ::core::ffi::c_int,
    >,
    pub coef_arrays: *mut jvirt_barray_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_d_main_controller {
    pub start_pass: Option<unsafe extern "C" fn(j_decompress_ptr, J_BUF_MODE) -> ()>,
    pub process_data: Option<
        unsafe extern "C" fn(
            j_decompress_ptr,
            JSAMPARRAY,
            *mut JDIMENSION,
            JDIMENSION,
        ) -> (),
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
    pub skip_input_data: Option<
        unsafe extern "C" fn(j_decompress_ptr, ::core::ffi::c_long) -> (),
    >,
    pub resync_to_restart: Option<
        unsafe extern "C" fn(j_decompress_ptr, ::core::ffi::c_int) -> boolean,
    >,
    pub term_source: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
}
pub type JLONG = ::core::ffi::c_long;
pub type my_cquantize_ptr = *mut my_cquantizer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_cquantizer {
    pub pub_0: jpeg_color_quantizer,
    pub sv_colormap: JSAMPARRAY,
    pub desired: ::core::ffi::c_int,
    pub histogram: hist3d,
    pub needs_zeroed: boolean,
    pub fserrors: FSERRPTR,
    pub on_odd_row: boolean,
    pub error_limiter: *mut ::core::ffi::c_int,
}
pub type FSERRPTR = *mut FSERROR;
pub type FSERROR = INT16;
pub type hist3d = *mut hist2d;
pub type hist2d = *mut hist1d;
pub type hist1d = [histcell; 32];
pub type histcell = UINT16;
pub const JERR_QUANT_MANY_COLORS: C2RustUnnamed_0 = 59;
pub const JERR_QUANT_FEW_COLORS: C2RustUnnamed_0 = 58;
pub const JERR_NOTIMPL: C2RustUnnamed_0 = 48;
pub type histptr = *mut histcell;
pub type LOCFSERROR = ::core::ffi::c_int;
pub const JTRC_QUANT_SELECTED: C2RustUnnamed_0 = 98;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct box_0 {
    pub c0min: ::core::ffi::c_int,
    pub c0max: ::core::ffi::c_int,
    pub c1min: ::core::ffi::c_int,
    pub c1max: ::core::ffi::c_int,
    pub c2min: ::core::ffi::c_int,
    pub c2max: ::core::ffi::c_int,
    pub volume: JLONG,
    pub colorcount: ::core::ffi::c_long,
}
pub type boxptr = *mut box_0;
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
pub const JERR_QUANT_COMPONENTS: C2RustUnnamed_0 = 57;
pub const JERR_OUT_OF_MEMORY: C2RustUnnamed_0 = 56;
pub const JERR_NO_SOI: C2RustUnnamed_0 = 55;
pub const JERR_NO_QUANT_TABLE: C2RustUnnamed_0 = 54;
pub const JERR_NO_IMAGE: C2RustUnnamed_0 = 53;
pub const JERR_NO_HUFF_TABLE: C2RustUnnamed_0 = 52;
pub const JERR_NO_BACKING_STORE: C2RustUnnamed_0 = 51;
pub const JERR_NO_ARITH_TABLE: C2RustUnnamed_0 = 50;
pub const JERR_NOT_COMPILED: C2RustUnnamed_0 = 49;
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
pub const JERR_CONVERSION_NOTIMPL: C2RustUnnamed_0 = 28;
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
pub const JERR_BAD_J_COLORSPACE: C2RustUnnamed_0 = 11;
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
pub const BITS_IN_JSAMPLE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const MAXJSAMPLE: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RGB_RED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RGB_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RGB_BLUE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_RGB_RED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXT_RGB_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_RGB_BLUE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_RGBX_RED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXT_RGBX_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_RGBX_BLUE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_BGR_RED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_BGR_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_BGR_BLUE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXT_BGRX_RED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_BGRX_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_BGRX_BLUE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EXT_XBGR_RED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EXT_XBGR_GREEN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_XBGR_BLUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_XRGB_RED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXT_XRGB_GREEN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXT_XRGB_BLUE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
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
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const R_SCALE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_SCALE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const B_SCALE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut c_scales: [::core::ffi::c_int; 3] = [R_SCALE, G_SCALE, B_SCALE];
pub const MAXNUMCOLORS: ::core::ffi::c_int = MAXJSAMPLE + 1 as ::core::ffi::c_int;
pub const HIST_C0_BITS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const HIST_C1_BITS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const HIST_C2_BITS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const HIST_C0_ELEMS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << HIST_C0_BITS;
pub const HIST_C1_ELEMS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << HIST_C1_BITS;
pub const HIST_C2_ELEMS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << HIST_C2_BITS;
pub const C0_SHIFT: ::core::ffi::c_int = BITS_IN_JSAMPLE - HIST_C0_BITS;
pub const C1_SHIFT: ::core::ffi::c_int = BITS_IN_JSAMPLE - HIST_C1_BITS;
pub const C2_SHIFT: ::core::ffi::c_int = BITS_IN_JSAMPLE - HIST_C2_BITS;
unsafe extern "C" fn prescan_quantize(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut histp: histptr = ::core::ptr::null_mut::<histcell>();
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut row: ::core::ffi::c_int = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    row = 0 as ::core::ffi::c_int;
    while row < num_rows {
        ptr = *input_buf.offset(row as isize);
        col = width;
        while col > 0 as JDIMENSION {
            histp = (&raw mut *(*histogram
                .offset(
                    (*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        >> C0_SHIFT) as isize,
                ))
                .offset(
                    (*ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        >> C1_SHIFT) as isize,
                ) as *mut histcell)
                .offset(
                    (*ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        >> C2_SHIFT) as isize,
                ) as *mut histcell as histptr;
            *histp = (*histp).wrapping_add(1);
            if *histp as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                *histp = (*histp).wrapping_sub(1);
            }
            ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
            col = col.wrapping_sub(1);
        }
        row += 1;
    }
}
unsafe extern "C" fn find_biggest_color_pop(
    mut boxlist: boxptr,
    mut numboxes: ::core::ffi::c_int,
) -> boxptr {
    let mut boxp: boxptr = ::core::ptr::null_mut::<box_0>();
    let mut i: ::core::ffi::c_int = 0;
    let mut maxc: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut which: boxptr = ::core::ptr::null_mut::<box_0>();
    i = 0 as ::core::ffi::c_int;
    boxp = boxlist;
    while i < numboxes {
        if (*boxp).colorcount > maxc && (*boxp).volume > 0 as JLONG {
            which = boxp;
            maxc = (*boxp).colorcount;
        }
        i += 1;
        boxp = boxp.offset(1);
    }
    return which;
}
unsafe extern "C" fn find_biggest_volume(
    mut boxlist: boxptr,
    mut numboxes: ::core::ffi::c_int,
) -> boxptr {
    let mut boxp: boxptr = ::core::ptr::null_mut::<box_0>();
    let mut i: ::core::ffi::c_int = 0;
    let mut maxv: JLONG = 0 as JLONG;
    let mut which: boxptr = ::core::ptr::null_mut::<box_0>();
    i = 0 as ::core::ffi::c_int;
    boxp = boxlist;
    while i < numboxes {
        if (*boxp).volume > maxv {
            which = boxp;
            maxv = (*boxp).volume;
        }
        i += 1;
        boxp = boxp.offset(1);
    }
    return which;
}
unsafe extern "C" fn update_box(mut cinfo: j_decompress_ptr, mut boxp: boxptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut histp: histptr = ::core::ptr::null_mut::<histcell>();
    let mut c0: ::core::ffi::c_int = 0;
    let mut c1: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    let mut c0min: ::core::ffi::c_int = 0;
    let mut c0max: ::core::ffi::c_int = 0;
    let mut c1min: ::core::ffi::c_int = 0;
    let mut c1max: ::core::ffi::c_int = 0;
    let mut c2min: ::core::ffi::c_int = 0;
    let mut c2max: ::core::ffi::c_int = 0;
    let mut dist0: JLONG = 0;
    let mut dist1: JLONG = 0;
    let mut dist2: JLONG = 0;
    let mut ccount: ::core::ffi::c_long = 0;
    c0min = (*boxp).c0min;
    c0max = (*boxp).c0max;
    c1min = (*boxp).c1min;
    c1max = (*boxp).c1max;
    c2min = (*boxp).c2min;
    c2max = (*boxp).c2max;
    if c0max > c0min {
        c0 = c0min;
        's_36: while c0 <= c0max {
            c1 = c1min;
            while c1 <= c1max {
                histp = (&raw mut *(*histogram.offset(c0 as isize)).offset(c1 as isize)
                    as *mut histcell)
                    .offset(c2min as isize) as *mut histcell as histptr;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh10 = histp;
                    histp = histp.offset(1);
                    if *fresh10 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        c0min = c0;
                        (*boxp).c0min = c0min;
                        break 's_36;
                    } else {
                        c2 += 1;
                    }
                }
                c1 += 1;
            }
            c0 += 1;
        }
    }
    if c0max > c0min {
        c0 = c0max;
        's_86: while c0 >= c0min {
            c1 = c1min;
            while c1 <= c1max {
                histp = (&raw mut *(*histogram.offset(c0 as isize)).offset(c1 as isize)
                    as *mut histcell)
                    .offset(c2min as isize) as *mut histcell as histptr;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh11 = histp;
                    histp = histp.offset(1);
                    if *fresh11 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        c0max = c0;
                        (*boxp).c0max = c0max;
                        break 's_86;
                    } else {
                        c2 += 1;
                    }
                }
                c1 += 1;
            }
            c0 -= 1;
        }
    }
    if c1max > c1min {
        c1 = c1min;
        's_138: while c1 <= c1max {
            c0 = c0min;
            while c0 <= c0max {
                histp = (&raw mut *(*histogram.offset(c0 as isize)).offset(c1 as isize)
                    as *mut histcell)
                    .offset(c2min as isize) as *mut histcell as histptr;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh12 = histp;
                    histp = histp.offset(1);
                    if *fresh12 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        c1min = c1;
                        (*boxp).c1min = c1min;
                        break 's_138;
                    } else {
                        c2 += 1;
                    }
                }
                c0 += 1;
            }
            c1 += 1;
        }
    }
    if c1max > c1min {
        c1 = c1max;
        's_190: while c1 >= c1min {
            c0 = c0min;
            while c0 <= c0max {
                histp = (&raw mut *(*histogram.offset(c0 as isize)).offset(c1 as isize)
                    as *mut histcell)
                    .offset(c2min as isize) as *mut histcell as histptr;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh13 = histp;
                    histp = histp.offset(1);
                    if *fresh13 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        c1max = c1;
                        (*boxp).c1max = c1max;
                        break 's_190;
                    } else {
                        c2 += 1;
                    }
                }
                c0 += 1;
            }
            c1 -= 1;
        }
    }
    if c2max > c2min {
        c2 = c2min;
        's_242: while c2 <= c2max {
            c0 = c0min;
            while c0 <= c0max {
                histp = (&raw mut *(*histogram.offset(c0 as isize))
                    .offset(c1min as isize) as *mut histcell)
                    .offset(c2 as isize) as *mut histcell as histptr;
                c1 = c1min;
                while c1 <= c1max {
                    if *histp as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        c2min = c2;
                        (*boxp).c2min = c2min;
                        break 's_242;
                    } else {
                        c1 += 1;
                        histp = histp.offset(HIST_C2_ELEMS as isize);
                    }
                }
                c0 += 1;
            }
            c2 += 1;
        }
    }
    if c2max > c2min {
        c2 = c2max;
        's_294: while c2 >= c2min {
            c0 = c0min;
            while c0 <= c0max {
                histp = (&raw mut *(*histogram.offset(c0 as isize))
                    .offset(c1min as isize) as *mut histcell)
                    .offset(c2 as isize) as *mut histcell as histptr;
                c1 = c1min;
                while c1 <= c1max {
                    if *histp as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        c2max = c2;
                        (*boxp).c2max = c2max;
                        break 's_294;
                    } else {
                        c1 += 1;
                        histp = histp.offset(HIST_C2_ELEMS as isize);
                    }
                }
                c0 += 1;
            }
            c2 -= 1;
        }
    }
    dist0 = ((c0max - c0min << C0_SHIFT)
        * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]) as JLONG;
    dist1 = ((c1max - c1min << C1_SHIFT)
        * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize]) as JLONG;
    dist2 = ((c2max - c2min << C2_SHIFT)
        * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize]) as JLONG;
    (*boxp).volume = dist0 * dist0 + dist1 * dist1 + dist2 * dist2;
    ccount = 0 as ::core::ffi::c_long;
    c0 = c0min;
    while c0 <= c0max {
        c1 = c1min;
        while c1 <= c1max {
            histp = (&raw mut *(*histogram.offset(c0 as isize)).offset(c1 as isize)
                as *mut histcell)
                .offset(c2min as isize) as *mut histcell as histptr;
            c2 = c2min;
            while c2 <= c2max {
                if *histp as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    ccount += 1;
                }
                c2 += 1;
                histp = histp.offset(1);
            }
            c1 += 1;
        }
        c0 += 1;
    }
    (*boxp).colorcount = ccount;
}
unsafe extern "C" fn median_cut(
    mut cinfo: j_decompress_ptr,
    mut boxlist: boxptr,
    mut numboxes: ::core::ffi::c_int,
    mut desired_colors: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut lb: ::core::ffi::c_int = 0;
    let mut c0: ::core::ffi::c_int = 0;
    let mut c1: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    let mut cmax: ::core::ffi::c_int = 0;
    let mut b1: boxptr = ::core::ptr::null_mut::<box_0>();
    let mut b2: boxptr = ::core::ptr::null_mut::<box_0>();
    while numboxes < desired_colors {
        if numboxes * 2 as ::core::ffi::c_int <= desired_colors {
            b1 = find_biggest_color_pop(boxlist, numboxes);
        } else {
            b1 = find_biggest_volume(boxlist, numboxes);
        }
        if b1.is_null() {
            break;
        }
        b2 = boxlist.offset(numboxes as isize) as *mut box_0 as boxptr;
        (*b2).c0max = (*b1).c0max;
        (*b2).c1max = (*b1).c1max;
        (*b2).c2max = (*b1).c2max;
        (*b2).c0min = (*b1).c0min;
        (*b2).c1min = (*b1).c1min;
        (*b2).c2min = (*b1).c2min;
        c0 = ((*b1).c0max - (*b1).c0min << C0_SHIFT)
            * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize];
        c1 = ((*b1).c1max - (*b1).c1min << C1_SHIFT)
            * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize];
        c2 = ((*b1).c2max - (*b1).c2min << C2_SHIFT)
            * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize];
        if rgb_red[(*cinfo).out_color_space as usize] == 0 as ::core::ffi::c_int {
            cmax = c1;
            n = 1 as ::core::ffi::c_int;
            if c0 > cmax {
                cmax = c0;
                n = 0 as ::core::ffi::c_int;
            }
            if c2 > cmax {
                n = 2 as ::core::ffi::c_int;
            }
        } else {
            cmax = c1;
            n = 1 as ::core::ffi::c_int;
            if c2 > cmax {
                cmax = c2;
                n = 2 as ::core::ffi::c_int;
            }
            if c0 > cmax {
                n = 0 as ::core::ffi::c_int;
            }
        }
        match n {
            0 => {
                lb = ((*b1).c0max + (*b1).c0min) / 2 as ::core::ffi::c_int;
                (*b1).c0max = lb;
                (*b2).c0min = lb + 1 as ::core::ffi::c_int;
            }
            1 => {
                lb = ((*b1).c1max + (*b1).c1min) / 2 as ::core::ffi::c_int;
                (*b1).c1max = lb;
                (*b2).c1min = lb + 1 as ::core::ffi::c_int;
            }
            2 => {
                lb = ((*b1).c2max + (*b1).c2min) / 2 as ::core::ffi::c_int;
                (*b1).c2max = lb;
                (*b2).c2min = lb + 1 as ::core::ffi::c_int;
            }
            _ => {}
        }
        update_box(cinfo, b1);
        update_box(cinfo, b2);
        numboxes += 1;
    }
    return numboxes;
}
unsafe extern "C" fn compute_color(
    mut cinfo: j_decompress_ptr,
    mut boxp: boxptr,
    mut icolor: ::core::ffi::c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut histp: histptr = ::core::ptr::null_mut::<histcell>();
    let mut c0: ::core::ffi::c_int = 0;
    let mut c1: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    let mut c0min: ::core::ffi::c_int = 0;
    let mut c0max: ::core::ffi::c_int = 0;
    let mut c1min: ::core::ffi::c_int = 0;
    let mut c1max: ::core::ffi::c_int = 0;
    let mut c2min: ::core::ffi::c_int = 0;
    let mut c2max: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_long = 0;
    let mut total: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut c0total: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut c1total: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut c2total: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    c0min = (*boxp).c0min;
    c0max = (*boxp).c0max;
    c1min = (*boxp).c1min;
    c1max = (*boxp).c1max;
    c2min = (*boxp).c2min;
    c2max = (*boxp).c2max;
    c0 = c0min;
    while c0 <= c0max {
        c1 = c1min;
        while c1 <= c1max {
            histp = (&raw mut *(*histogram.offset(c0 as isize)).offset(c1 as isize)
                as *mut histcell)
                .offset(c2min as isize) as *mut histcell as histptr;
            c2 = c2min;
            while c2 <= c2max {
                let fresh9 = histp;
                histp = histp.offset(1);
                count = *fresh9 as ::core::ffi::c_long;
                if count != 0 as ::core::ffi::c_long {
                    total += count;
                    c0total
                        += ((c0 << C0_SHIFT)
                            + ((1 as ::core::ffi::c_int) << C0_SHIFT
                                >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long * count;
                    c1total
                        += ((c1 << C1_SHIFT)
                            + ((1 as ::core::ffi::c_int) << C1_SHIFT
                                >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long * count;
                    c2total
                        += ((c2 << C2_SHIFT)
                            + ((1 as ::core::ffi::c_int) << C2_SHIFT
                                >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long * count;
                }
                c2 += 1;
            }
            c1 += 1;
        }
        c0 += 1;
    }
    *(*(*cinfo).colormap.offset(0 as ::core::ffi::c_int as isize))
        .offset(icolor as isize) = ((c0total + (total >> 1 as ::core::ffi::c_int))
        / total) as JSAMPLE;
    *(*(*cinfo).colormap.offset(1 as ::core::ffi::c_int as isize))
        .offset(icolor as isize) = ((c1total + (total >> 1 as ::core::ffi::c_int))
        / total) as JSAMPLE;
    *(*(*cinfo).colormap.offset(2 as ::core::ffi::c_int as isize))
        .offset(icolor as isize) = ((c2total + (total >> 1 as ::core::ffi::c_int))
        / total) as JSAMPLE;
}
unsafe extern "C" fn select_colors(
    mut cinfo: j_decompress_ptr,
    mut desired_colors: ::core::ffi::c_int,
) {
    let mut boxlist: boxptr = ::core::ptr::null_mut::<box_0>();
    let mut numboxes: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    boxlist = Some((*(*cinfo).mem).alloc_small.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (desired_colors as size_t)
            .wrapping_mul(::core::mem::size_of::<box_0>() as size_t),
    ) as boxptr;
    numboxes = 1 as ::core::ffi::c_int;
    (*boxlist.offset(0 as ::core::ffi::c_int as isize)).c0min = 0 as ::core::ffi::c_int;
    (*boxlist.offset(0 as ::core::ffi::c_int as isize)).c0max = MAXJSAMPLE >> C0_SHIFT;
    (*boxlist.offset(0 as ::core::ffi::c_int as isize)).c1min = 0 as ::core::ffi::c_int;
    (*boxlist.offset(0 as ::core::ffi::c_int as isize)).c1max = MAXJSAMPLE >> C1_SHIFT;
    (*boxlist.offset(0 as ::core::ffi::c_int as isize)).c2min = 0 as ::core::ffi::c_int;
    (*boxlist.offset(0 as ::core::ffi::c_int as isize)).c2max = MAXJSAMPLE >> C2_SHIFT;
    update_box(cinfo, boxlist.offset(0 as ::core::ffi::c_int as isize) as boxptr);
    numboxes = median_cut(cinfo, boxlist, numboxes, desired_colors);
    i = 0 as ::core::ffi::c_int;
    while i < numboxes {
        compute_color(cinfo, boxlist.offset(i as isize) as boxptr, i);
        i += 1;
    }
    (*cinfo).actual_number_of_colors = numboxes;
    (*(*cinfo).err).msg_code = JTRC_QUANT_SELECTED as ::core::ffi::c_int;
    (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = numboxes;
    Some((*(*cinfo).err).emit_message.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
}
pub const BOX_C0_LOG: ::core::ffi::c_int = HIST_C0_BITS - 3 as ::core::ffi::c_int;
pub const BOX_C1_LOG: ::core::ffi::c_int = HIST_C1_BITS - 3 as ::core::ffi::c_int;
pub const BOX_C2_LOG: ::core::ffi::c_int = HIST_C2_BITS - 3 as ::core::ffi::c_int;
pub const BOX_C0_ELEMS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BOX_C0_LOG;
pub const BOX_C1_ELEMS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BOX_C1_LOG;
pub const BOX_C2_ELEMS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << BOX_C2_LOG;
pub const BOX_C0_SHIFT: ::core::ffi::c_int = C0_SHIFT + BOX_C0_LOG;
pub const BOX_C1_SHIFT: ::core::ffi::c_int = C1_SHIFT + BOX_C1_LOG;
pub const BOX_C2_SHIFT: ::core::ffi::c_int = C2_SHIFT + BOX_C2_LOG;
unsafe extern "C" fn find_nearby_colors(
    mut cinfo: j_decompress_ptr,
    mut minc0: ::core::ffi::c_int,
    mut minc1: ::core::ffi::c_int,
    mut minc2: ::core::ffi::c_int,
    mut colorlist: *mut JSAMPLE,
) -> ::core::ffi::c_int {
    let mut numcolors: ::core::ffi::c_int = (*cinfo).actual_number_of_colors;
    let mut maxc0: ::core::ffi::c_int = 0;
    let mut maxc1: ::core::ffi::c_int = 0;
    let mut maxc2: ::core::ffi::c_int = 0;
    let mut centerc0: ::core::ffi::c_int = 0;
    let mut centerc1: ::core::ffi::c_int = 0;
    let mut centerc2: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut ncolors: ::core::ffi::c_int = 0;
    let mut minmaxdist: JLONG = 0;
    let mut min_dist: JLONG = 0;
    let mut max_dist: JLONG = 0;
    let mut tdist: JLONG = 0;
    let mut mindist: [JLONG; 256] = [0; 256];
    maxc0 = minc0
        + (((1 as ::core::ffi::c_int) << BOX_C0_SHIFT)
            - ((1 as ::core::ffi::c_int) << C0_SHIFT));
    centerc0 = minc0 + maxc0 >> 1 as ::core::ffi::c_int;
    maxc1 = minc1
        + (((1 as ::core::ffi::c_int) << BOX_C1_SHIFT)
            - ((1 as ::core::ffi::c_int) << C1_SHIFT));
    centerc1 = minc1 + maxc1 >> 1 as ::core::ffi::c_int;
    maxc2 = minc2
        + (((1 as ::core::ffi::c_int) << BOX_C2_SHIFT)
            - ((1 as ::core::ffi::c_int) << C2_SHIFT));
    centerc2 = minc2 + maxc2 >> 1 as ::core::ffi::c_int;
    minmaxdist = 0x7fffffff as ::core::ffi::c_long as JLONG;
    i = 0 as ::core::ffi::c_int;
    while i < numcolors {
        x = *(*(*cinfo).colormap.offset(0 as ::core::ffi::c_int as isize))
            .offset(i as isize) as ::core::ffi::c_int;
        if x < minc0 {
            tdist = ((x - minc0)
                * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist = tdist * tdist;
            tdist = ((x - maxc0)
                * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist = tdist * tdist;
        } else if x > maxc0 {
            tdist = ((x - maxc0)
                * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist = tdist * tdist;
            tdist = ((x - minc0)
                * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist = tdist * tdist;
        } else {
            min_dist = 0 as JLONG;
            if x <= centerc0 {
                tdist = ((x - maxc0)
                    * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                    as JLONG;
                max_dist = tdist * tdist;
            } else {
                tdist = ((x - minc0)
                    * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                    as JLONG;
                max_dist = tdist * tdist;
            }
        }
        x = *(*(*cinfo).colormap.offset(1 as ::core::ffi::c_int as isize))
            .offset(i as isize) as ::core::ffi::c_int;
        if x < minc1 {
            tdist = ((x - minc1)
                * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - maxc1)
                * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist;
        } else if x > maxc1 {
            tdist = ((x - maxc1)
                * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - minc1)
                * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist;
        } else if x <= centerc1 {
            tdist = ((x - maxc1)
                * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist;
        } else {
            tdist = ((x - minc1)
                * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist;
        }
        x = *(*(*cinfo).colormap.offset(2 as ::core::ffi::c_int as isize))
            .offset(i as isize) as ::core::ffi::c_int;
        if x < minc2 {
            tdist = ((x - minc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - maxc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist;
        } else if x > maxc2 {
            tdist = ((x - maxc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            min_dist += tdist * tdist;
            tdist = ((x - minc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist;
        } else if x <= centerc2 {
            tdist = ((x - maxc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist;
        } else {
            tdist = ((x - minc2)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize])
                as JLONG;
            max_dist += tdist * tdist;
        }
        mindist[i as usize] = min_dist;
        if max_dist < minmaxdist {
            minmaxdist = max_dist;
        }
        i += 1;
    }
    ncolors = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < numcolors {
        if mindist[i as usize] <= minmaxdist {
            let fresh8 = ncolors;
            ncolors = ncolors + 1;
            *colorlist.offset(fresh8 as isize) = i as JSAMPLE;
        }
        i += 1;
    }
    return ncolors;
}
unsafe extern "C" fn find_best_colors(
    mut cinfo: j_decompress_ptr,
    mut minc0: ::core::ffi::c_int,
    mut minc1: ::core::ffi::c_int,
    mut minc2: ::core::ffi::c_int,
    mut numcolors: ::core::ffi::c_int,
    mut colorlist: *mut JSAMPLE,
    mut bestcolor: *mut JSAMPLE,
) {
    let mut ic0: ::core::ffi::c_int = 0;
    let mut ic1: ::core::ffi::c_int = 0;
    let mut ic2: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut icolor: ::core::ffi::c_int = 0;
    let mut bptr: *mut JLONG = ::core::ptr::null_mut::<JLONG>();
    let mut cptr: *mut JSAMPLE = ::core::ptr::null_mut::<JSAMPLE>();
    let mut dist0: JLONG = 0;
    let mut dist1: JLONG = 0;
    let mut dist2: JLONG = 0;
    let mut xx0: JLONG = 0;
    let mut xx1: JLONG = 0;
    let mut xx2: JLONG = 0;
    let mut inc0: JLONG = 0;
    let mut inc1: JLONG = 0;
    let mut inc2: JLONG = 0;
    let mut bestdist: [JLONG; 128] = [0; 128];
    bptr = &raw mut bestdist as *mut JLONG;
    i = BOX_C0_ELEMS * BOX_C1_ELEMS * BOX_C2_ELEMS - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let fresh7 = bptr;
        bptr = bptr.offset(1);
        *fresh7 = 0x7fffffff as ::core::ffi::c_long as JLONG;
        i -= 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < numcolors {
        icolor = *colorlist.offset(i as isize) as ::core::ffi::c_int;
        inc0 = ((minc0
            - *(*(*cinfo).colormap.offset(0 as ::core::ffi::c_int as isize))
                .offset(icolor as isize) as ::core::ffi::c_int)
            * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]) as JLONG;
        dist0 = inc0 * inc0;
        inc1 = ((minc1
            - *(*(*cinfo).colormap.offset(1 as ::core::ffi::c_int as isize))
                .offset(icolor as isize) as ::core::ffi::c_int)
            * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize]) as JLONG;
        dist0 += inc1 * inc1;
        inc2 = ((minc2
            - *(*(*cinfo).colormap.offset(2 as ::core::ffi::c_int as isize))
                .offset(icolor as isize) as ::core::ffi::c_int)
            * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize]) as JLONG;
        dist0 += inc2 * inc2;
        inc0 = inc0
            * (2 as ::core::ffi::c_int
                * (((1 as ::core::ffi::c_int) << C0_SHIFT)
                    * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]))
                as JLONG
            + (((1 as ::core::ffi::c_int) << C0_SHIFT)
                * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]
                * (((1 as ::core::ffi::c_int) << C0_SHIFT)
                    * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]))
                as JLONG;
        inc1 = inc1
            * (2 as ::core::ffi::c_int
                * (((1 as ::core::ffi::c_int) << C1_SHIFT)
                    * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize]))
                as JLONG
            + (((1 as ::core::ffi::c_int) << C1_SHIFT)
                * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize]
                * (((1 as ::core::ffi::c_int) << C1_SHIFT)
                    * c_scales[rgb_green[(*cinfo).out_color_space as usize] as usize]))
                as JLONG;
        inc2 = inc2
            * (2 as ::core::ffi::c_int
                * (((1 as ::core::ffi::c_int) << C2_SHIFT)
                    * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize]))
                as JLONG
            + (((1 as ::core::ffi::c_int) << C2_SHIFT)
                * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize]
                * (((1 as ::core::ffi::c_int) << C2_SHIFT)
                    * c_scales[rgb_blue[(*cinfo).out_color_space as usize] as usize]))
                as JLONG;
        bptr = &raw mut bestdist as *mut JLONG;
        cptr = bestcolor as *mut JSAMPLE;
        xx0 = inc0;
        ic0 = BOX_C0_ELEMS - 1 as ::core::ffi::c_int;
        while ic0 >= 0 as ::core::ffi::c_int {
            dist1 = dist0;
            xx1 = inc1;
            ic1 = BOX_C1_ELEMS - 1 as ::core::ffi::c_int;
            while ic1 >= 0 as ::core::ffi::c_int {
                dist2 = dist1;
                xx2 = inc2;
                ic2 = BOX_C2_ELEMS - 1 as ::core::ffi::c_int;
                while ic2 >= 0 as ::core::ffi::c_int {
                    if dist2 < *bptr {
                        *bptr = dist2;
                        *cptr = icolor as JSAMPLE;
                    }
                    dist2 += xx2;
                    xx2
                        += (2 as ::core::ffi::c_int
                            * (((1 as ::core::ffi::c_int) << C2_SHIFT)
                                * c_scales[rgb_blue[(*cinfo).out_color_space as usize]
                                    as usize])
                            * (((1 as ::core::ffi::c_int) << C2_SHIFT)
                                * c_scales[rgb_blue[(*cinfo).out_color_space as usize]
                                    as usize])) as JLONG;
                    bptr = bptr.offset(1);
                    cptr = cptr.offset(1);
                    ic2 -= 1;
                }
                dist1 += xx1;
                xx1
                    += (2 as ::core::ffi::c_int
                        * (((1 as ::core::ffi::c_int) << C1_SHIFT)
                            * c_scales[rgb_green[(*cinfo).out_color_space as usize]
                                as usize])
                        * (((1 as ::core::ffi::c_int) << C1_SHIFT)
                            * c_scales[rgb_green[(*cinfo).out_color_space as usize]
                                as usize])) as JLONG;
                ic1 -= 1;
            }
            dist0 += xx0;
            xx0
                += (2 as ::core::ffi::c_int
                    * (((1 as ::core::ffi::c_int) << C0_SHIFT)
                        * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize])
                    * (((1 as ::core::ffi::c_int) << C0_SHIFT)
                        * c_scales[rgb_red[(*cinfo).out_color_space as usize] as usize]))
                    as JLONG;
            ic0 -= 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn fill_inverse_cmap(
    mut cinfo: j_decompress_ptr,
    mut c0: ::core::ffi::c_int,
    mut c1: ::core::ffi::c_int,
    mut c2: ::core::ffi::c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut minc0: ::core::ffi::c_int = 0;
    let mut minc1: ::core::ffi::c_int = 0;
    let mut minc2: ::core::ffi::c_int = 0;
    let mut ic0: ::core::ffi::c_int = 0;
    let mut ic1: ::core::ffi::c_int = 0;
    let mut ic2: ::core::ffi::c_int = 0;
    let mut cptr: *mut JSAMPLE = ::core::ptr::null_mut::<JSAMPLE>();
    let mut cachep: histptr = ::core::ptr::null_mut::<histcell>();
    let mut colorlist: [JSAMPLE; 256] = [0; 256];
    let mut numcolors: ::core::ffi::c_int = 0;
    let mut bestcolor: [JSAMPLE; 128] = [0; 128];
    c0 >>= BOX_C0_LOG;
    c1 >>= BOX_C1_LOG;
    c2 >>= BOX_C2_LOG;
    minc0 = (c0 << BOX_C0_SHIFT)
        + ((1 as ::core::ffi::c_int) << C0_SHIFT >> 1 as ::core::ffi::c_int);
    minc1 = (c1 << BOX_C1_SHIFT)
        + ((1 as ::core::ffi::c_int) << C1_SHIFT >> 1 as ::core::ffi::c_int);
    minc2 = (c2 << BOX_C2_SHIFT)
        + ((1 as ::core::ffi::c_int) << C2_SHIFT >> 1 as ::core::ffi::c_int);
    numcolors = find_nearby_colors(
        cinfo,
        minc0,
        minc1,
        minc2,
        &raw mut colorlist as *mut JSAMPLE,
    );
    find_best_colors(
        cinfo,
        minc0,
        minc1,
        minc2,
        numcolors,
        &raw mut colorlist as *mut JSAMPLE,
        &raw mut bestcolor as *mut JSAMPLE,
    );
    c0 <<= BOX_C0_LOG;
    c1 <<= BOX_C1_LOG;
    c2 <<= BOX_C2_LOG;
    cptr = &raw mut bestcolor as *mut JSAMPLE;
    ic0 = 0 as ::core::ffi::c_int;
    while ic0 < BOX_C0_ELEMS {
        ic1 = 0 as ::core::ffi::c_int;
        while ic1 < BOX_C1_ELEMS {
            cachep = (&raw mut *(*histogram.offset((c0 + ic0) as isize))
                .offset((c1 + ic1) as isize) as *mut histcell)
                .offset(c2 as isize) as *mut histcell as histptr;
            ic2 = 0 as ::core::ffi::c_int;
            while ic2 < BOX_C2_ELEMS {
                let fresh5 = cptr;
                cptr = cptr.offset(1);
                let fresh6 = cachep;
                cachep = cachep.offset(1);
                *fresh6 = (*fresh5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as histcell;
                ic2 += 1;
            }
            ic1 += 1;
        }
        ic0 += 1;
    }
}
unsafe extern "C" fn pass2_no_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut cachep: histptr = ::core::ptr::null_mut::<histcell>();
    let mut c0: ::core::ffi::c_int = 0;
    let mut c1: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    row = 0 as ::core::ffi::c_int;
    while row < num_rows {
        inptr = *input_buf.offset(row as isize);
        outptr = *output_buf.offset(row as isize);
        col = width;
        while col > 0 as JDIMENSION {
            let fresh1 = inptr;
            inptr = inptr.offset(1);
            c0 = *fresh1 as ::core::ffi::c_int >> C0_SHIFT;
            let fresh2 = inptr;
            inptr = inptr.offset(1);
            c1 = *fresh2 as ::core::ffi::c_int >> C1_SHIFT;
            let fresh3 = inptr;
            inptr = inptr.offset(1);
            c2 = *fresh3 as ::core::ffi::c_int >> C2_SHIFT;
            cachep = (&raw mut *(*histogram.offset(c0 as isize)).offset(c1 as isize)
                as *mut histcell)
                .offset(c2 as isize) as *mut histcell as histptr;
            if *cachep as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                fill_inverse_cmap(cinfo, c0, c1, c2);
            }
            let fresh4 = outptr;
            outptr = outptr.offset(1);
            *fresh4 = (*cachep as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as JSAMPLE;
            col = col.wrapping_sub(1);
        }
        row += 1;
    }
}
unsafe extern "C" fn pass2_fs_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut cur0: LOCFSERROR = 0;
    let mut cur1: LOCFSERROR = 0;
    let mut cur2: LOCFSERROR = 0;
    let mut belowerr0: LOCFSERROR = 0;
    let mut belowerr1: LOCFSERROR = 0;
    let mut belowerr2: LOCFSERROR = 0;
    let mut bpreverr0: LOCFSERROR = 0;
    let mut bpreverr1: LOCFSERROR = 0;
    let mut bpreverr2: LOCFSERROR = 0;
    let mut errorptr: FSERRPTR = ::core::ptr::null_mut::<FSERROR>();
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut cachep: histptr = ::core::ptr::null_mut::<histcell>();
    let mut dir: ::core::ffi::c_int = 0;
    let mut dir3: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut error_limit: *mut ::core::ffi::c_int = (*cquantize).error_limiter;
    let mut colormap0: JSAMPROW = *(*cinfo)
        .colormap
        .offset(0 as ::core::ffi::c_int as isize);
    let mut colormap1: JSAMPROW = *(*cinfo)
        .colormap
        .offset(1 as ::core::ffi::c_int as isize);
    let mut colormap2: JSAMPROW = *(*cinfo)
        .colormap
        .offset(2 as ::core::ffi::c_int as isize);
    row = 0 as ::core::ffi::c_int;
    while row < num_rows {
        inptr = *input_buf.offset(row as isize);
        outptr = *output_buf.offset(row as isize);
        if (*cquantize).on_odd_row != 0 {
            inptr = inptr
                .offset(
                    width.wrapping_sub(1 as JDIMENSION).wrapping_mul(3 as JDIMENSION)
                        as isize,
                );
            outptr = outptr.offset(width.wrapping_sub(1 as JDIMENSION) as isize);
            dir = -(1 as ::core::ffi::c_int);
            dir3 = -(3 as ::core::ffi::c_int);
            errorptr = (*cquantize)
                .fserrors
                .offset(
                    width.wrapping_add(1 as JDIMENSION).wrapping_mul(3 as JDIMENSION)
                        as isize,
                );
            (*cquantize).on_odd_row = FALSE as boolean;
        } else {
            dir = 1 as ::core::ffi::c_int;
            dir3 = 3 as ::core::ffi::c_int;
            errorptr = (*cquantize).fserrors;
            (*cquantize).on_odd_row = TRUE as boolean;
        }
        cur2 = 0 as ::core::ffi::c_int as LOCFSERROR;
        cur1 = cur2;
        cur0 = cur1;
        belowerr2 = 0 as ::core::ffi::c_int as LOCFSERROR;
        belowerr1 = belowerr2;
        belowerr0 = belowerr1;
        bpreverr2 = 0 as ::core::ffi::c_int as LOCFSERROR;
        bpreverr1 = bpreverr2;
        bpreverr0 = bpreverr1;
        col = width;
        while col > 0 as JDIMENSION {
            cur0 = (cur0 as ::core::ffi::c_int
                + *errorptr.offset((dir3 + 0 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as LOCFSERROR;
            cur1 = (cur1 as ::core::ffi::c_int
                + *errorptr.offset((dir3 + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as LOCFSERROR;
            cur2 = (cur2 as ::core::ffi::c_int
                + *errorptr.offset((dir3 + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as LOCFSERROR;
            cur0 = *error_limit.offset(cur0 as isize) as LOCFSERROR;
            cur1 = *error_limit.offset(cur1 as isize) as LOCFSERROR;
            cur2 = *error_limit.offset(cur2 as isize) as LOCFSERROR;
            cur0
                += *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            cur1
                += *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            cur2
                += *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            cur0 = *range_limit.offset(cur0 as isize) as LOCFSERROR;
            cur1 = *range_limit.offset(cur1 as isize) as LOCFSERROR;
            cur2 = *range_limit.offset(cur2 as isize) as LOCFSERROR;
            cachep = (&raw mut *(*histogram.offset((cur0 >> C0_SHIFT) as isize))
                .offset((cur1 >> C1_SHIFT) as isize) as *mut histcell)
                .offset((cur2 >> C2_SHIFT) as isize) as *mut histcell as histptr;
            if *cachep as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                fill_inverse_cmap(
                    cinfo,
                    cur0 as ::core::ffi::c_int >> C0_SHIFT,
                    cur1 as ::core::ffi::c_int >> C1_SHIFT,
                    cur2 as ::core::ffi::c_int >> C2_SHIFT,
                );
            }
            let mut pixcode: ::core::ffi::c_int = *cachep as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int;
            *outptr = pixcode as JSAMPLE;
            cur0 -= *colormap0.offset(pixcode as isize) as ::core::ffi::c_int;
            cur1 -= *colormap1.offset(pixcode as isize) as ::core::ffi::c_int;
            cur2 -= *colormap2.offset(pixcode as isize) as ::core::ffi::c_int;
            let mut bnexterr: LOCFSERROR = 0;
            bnexterr = cur0;
            *errorptr.offset(0 as ::core::ffi::c_int as isize) = (bpreverr0
                as ::core::ffi::c_int
                + cur0 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as FSERROR;
            bpreverr0 = (belowerr0 as ::core::ffi::c_int
                + cur0 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as LOCFSERROR;
            belowerr0 = bnexterr;
            cur0 *= 7 as ::core::ffi::c_int;
            bnexterr = cur1;
            *errorptr.offset(1 as ::core::ffi::c_int as isize) = (bpreverr1
                as ::core::ffi::c_int
                + cur1 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as FSERROR;
            bpreverr1 = (belowerr1 as ::core::ffi::c_int
                + cur1 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as LOCFSERROR;
            belowerr1 = bnexterr;
            cur1 *= 7 as ::core::ffi::c_int;
            bnexterr = cur2;
            *errorptr.offset(2 as ::core::ffi::c_int as isize) = (bpreverr2
                as ::core::ffi::c_int
                + cur2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as FSERROR;
            bpreverr2 = (belowerr2 as ::core::ffi::c_int
                + cur2 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as LOCFSERROR;
            belowerr2 = bnexterr;
            cur2 *= 7 as ::core::ffi::c_int;
            inptr = inptr.offset(dir3 as isize);
            outptr = outptr.offset(dir as isize);
            errorptr = errorptr.offset(dir3 as isize);
            col = col.wrapping_sub(1);
        }
        *errorptr.offset(0 as ::core::ffi::c_int as isize) = bpreverr0 as FSERROR;
        *errorptr.offset(1 as ::core::ffi::c_int as isize) = bpreverr1 as FSERROR;
        *errorptr.offset(2 as ::core::ffi::c_int as isize) = bpreverr2 as FSERROR;
        row += 1;
    }
}
unsafe extern "C" fn init_error_limit(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut table: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut in_0: ::core::ffi::c_int = 0;
    let mut out: ::core::ffi::c_int = 0;
    table = Some((*(*cinfo).mem).alloc_small.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((MAXJSAMPLE * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    table = table.offset(MAXJSAMPLE as isize);
    (*cquantize).error_limiter = table;
    out = 0 as ::core::ffi::c_int;
    in_0 = 0 as ::core::ffi::c_int;
    while in_0 < STEPSIZE {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1;
        out += 1;
    }
    while in_0 < STEPSIZE * 3 as ::core::ffi::c_int {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1;
        out
            += (if in_0 & 1 as ::core::ffi::c_int != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            });
    }
    while in_0 <= MAXJSAMPLE {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1;
    }
}
pub const STEPSIZE: ::core::ffi::c_int = (MAXJSAMPLE + 1 as ::core::ffi::c_int)
    / 16 as ::core::ffi::c_int;
unsafe extern "C" fn finish_pass1(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    (*cinfo).colormap = (*cquantize).sv_colormap;
    select_colors(cinfo, (*cquantize).desired);
    (*cquantize).needs_zeroed = TRUE as boolean;
}
unsafe extern "C" fn finish_pass2(mut cinfo: j_decompress_ptr) {}
unsafe extern "C" fn start_pass_2_quant(
    mut cinfo: j_decompress_ptr,
    mut is_pre_scan: boolean,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut i: ::core::ffi::c_int = 0;
    if (*cinfo).dither_mode as ::core::ffi::c_uint
        != JDITHER_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cinfo).dither_mode = JDITHER_FS;
    }
    if is_pre_scan != 0 {
        (*cquantize).pub_0.color_quantize = Some(
            prescan_quantize
                as unsafe extern "C" fn(
                    j_decompress_ptr,
                    JSAMPARRAY,
                    JSAMPARRAY,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    j_decompress_ptr,
                    JSAMPARRAY,
                    JSAMPARRAY,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        (*cquantize).pub_0.finish_pass = Some(
            finish_pass1 as unsafe extern "C" fn(j_decompress_ptr) -> (),
        ) as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
        (*cquantize).needs_zeroed = TRUE as boolean;
    } else {
        if (*cinfo).dither_mode as ::core::ffi::c_uint
            == JDITHER_FS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*cquantize).pub_0.color_quantize = Some(
                pass2_fs_dither
                    as unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPARRAY,
                        JSAMPARRAY,
                        ::core::ffi::c_int,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPARRAY,
                        JSAMPARRAY,
                        ::core::ffi::c_int,
                    ) -> (),
                >;
        } else {
            (*cquantize).pub_0.color_quantize = Some(
                pass2_no_dither
                    as unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPARRAY,
                        JSAMPARRAY,
                        ::core::ffi::c_int,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        j_decompress_ptr,
                        JSAMPARRAY,
                        JSAMPARRAY,
                        ::core::ffi::c_int,
                    ) -> (),
                >;
        }
        (*cquantize).pub_0.finish_pass = Some(
            finish_pass2 as unsafe extern "C" fn(j_decompress_ptr) -> (),
        ) as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
        i = (*cinfo).actual_number_of_colors;
        if i < 1 as ::core::ffi::c_int {
            (*(*cinfo).err).msg_code = JERR_QUANT_FEW_COLORS as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = 1
                as ::core::ffi::c_int;
            Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if i > MAXNUMCOLORS {
            (*(*cinfo).err).msg_code = JERR_QUANT_MANY_COLORS as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = 255
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if (*cinfo).dither_mode as ::core::ffi::c_uint
            == JDITHER_FS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut arraysize: size_t = ((*cinfo)
                .output_width
                .wrapping_add(2 as JDIMENSION) as usize)
                .wrapping_mul(
                    (3 as usize).wrapping_mul(::core::mem::size_of::<FSERROR>() as usize),
                );
            if (*cquantize).fserrors.is_null() {
                (*cquantize).fserrors = Some(
                        (*(*cinfo).mem).alloc_large.expect("non-null function pointer"),
                    )
                    .expect(
                        "non-null function pointer",
                    )(cinfo as j_common_ptr, JPOOL_IMAGE, arraysize) as FSERRPTR;
            }
            jzero_far((*cquantize).fserrors as *mut ::core::ffi::c_void, arraysize);
            if (*cquantize).error_limiter.is_null() {
                init_error_limit(cinfo);
            }
            (*cquantize).on_odd_row = FALSE as boolean;
        }
    }
    if (*cquantize).needs_zeroed != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < HIST_C0_ELEMS {
            jzero_far(
                *histogram.offset(i as isize) as *mut ::core::ffi::c_void,
                ((HIST_C1_ELEMS * HIST_C2_ELEMS) as size_t)
                    .wrapping_mul(::core::mem::size_of::<histcell>() as size_t),
            );
            i += 1;
        }
        (*cquantize).needs_zeroed = FALSE as boolean;
    }
}
unsafe extern "C" fn new_color_map_2_quant(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    (*cquantize).needs_zeroed = TRUE as boolean;
}
pub unsafe extern "C" fn jinit_2pass_quantizer(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = ::core::ptr::null_mut::<my_cquantizer>();
    let mut i: ::core::ffi::c_int = 0;
    cquantize = Some((*(*cinfo).mem).alloc_small.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<my_cquantizer>() as size_t,
    ) as my_cquantize_ptr;
    (*cinfo).cquantize = cquantize as *mut jpeg_color_quantizer
        as *mut jpeg_color_quantizer;
    (*cquantize).pub_0.start_pass = Some(
        start_pass_2_quant as unsafe extern "C" fn(j_decompress_ptr, boolean) -> (),
    ) as Option<unsafe extern "C" fn(j_decompress_ptr, boolean) -> ()>;
    (*cquantize).pub_0.new_color_map = Some(
        new_color_map_2_quant as unsafe extern "C" fn(j_decompress_ptr) -> (),
    ) as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
    (*cquantize).fserrors = ::core::ptr::null_mut::<FSERROR>();
    (*cquantize).error_limiter = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if (*cinfo).out_color_components != 3 as ::core::ffi::c_int {
        (*(*cinfo).err).msg_code = JERR_NOTIMPL as ::core::ffi::c_int;
        Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*cquantize).histogram = Some(
            (*(*cinfo).mem).alloc_small.expect("non-null function pointer"),
        )
        .expect(
            "non-null function pointer",
        )(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (HIST_C0_ELEMS as size_t)
            .wrapping_mul(::core::mem::size_of::<hist2d>() as size_t),
    ) as hist3d;
    i = 0 as ::core::ffi::c_int;
    while i < HIST_C0_ELEMS {
        let ref mut fresh0 = *(*cquantize).histogram.offset(i as isize);
        *fresh0 = Some((*(*cinfo).mem).alloc_large.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((HIST_C1_ELEMS * HIST_C2_ELEMS) as size_t)
                .wrapping_mul(::core::mem::size_of::<histcell>() as size_t),
        ) as hist2d;
        i += 1;
    }
    (*cquantize).needs_zeroed = TRUE as boolean;
    if (*cinfo).enable_2pass_quant != 0 {
        let mut desired: ::core::ffi::c_int = (*cinfo).desired_number_of_colors;
        if desired < 8 as ::core::ffi::c_int {
            (*(*cinfo).err).msg_code = JERR_QUANT_FEW_COLORS as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = 8
                as ::core::ffi::c_int;
            Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if desired > MAXNUMCOLORS {
            (*(*cinfo).err).msg_code = JERR_QUANT_MANY_COLORS as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = 255
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*cquantize).sv_colormap = Some(
                (*(*cinfo).mem).alloc_sarray.expect("non-null function pointer"),
            )
            .expect(
                "non-null function pointer",
            )(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            desired as JDIMENSION,
            3 as ::core::ffi::c_int as JDIMENSION,
        );
        (*cquantize).desired = desired;
    } else {
        (*cquantize).sv_colormap = ::core::ptr::null_mut::<JSAMPROW>();
    }
    if (*cinfo).dither_mode as ::core::ffi::c_uint
        != JDITHER_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cinfo).dither_mode = JDITHER_FS;
    }
    if (*cinfo).dither_mode as ::core::ffi::c_uint
        == JDITHER_FS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cquantize).fserrors = Some(
                (*(*cinfo).mem).alloc_large.expect("non-null function pointer"),
            )
            .expect(
                "non-null function pointer",
            )(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((*cinfo).output_width.wrapping_add(2 as JDIMENSION) as usize)
                .wrapping_mul(
                    (3 as usize).wrapping_mul(::core::mem::size_of::<FSERROR>() as usize),
                ),
        ) as FSERRPTR;
        init_error_limit(cinfo);
    }
}
