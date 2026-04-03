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
pub type FSERRPTR = *mut FSERROR;
pub type FSERROR = INT16;
pub type my_cquantize_ptr = *mut my_cquantizer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_cquantizer {
    pub pub_0: jpeg_color_quantizer,
    pub sv_colormap: JSAMPARRAY,
    pub sv_actual: ::core::ffi::c_int,
    pub colorindex: JSAMPARRAY,
    pub is_padded: boolean,
    pub Ncolors: [::core::ffi::c_int; 4],
    pub row_index: ::core::ffi::c_int,
    pub odither: [ODITHER_MATRIX_PTR; 4],
    pub fserrors: [FSERRPTR; 4],
    pub on_odd_row: boolean,
}
pub type ODITHER_MATRIX_PTR = *mut [::core::ffi::c_int; 16];
pub const JTRC_QUANT_NCOLORS: C2RustUnnamed_0 = 97;
pub const JTRC_QUANT_3_NCOLORS: C2RustUnnamed_0 = 96;
pub const JERR_QUANT_FEW_COLORS: C2RustUnnamed_0 = 58;
pub const JERR_QUANT_MANY_COLORS: C2RustUnnamed_0 = 59;
pub const JERR_QUANT_COMPONENTS: C2RustUnnamed_0 = 57;
pub const JERR_MODE_CHANGE: C2RustUnnamed_0 = 47;
pub const JERR_NOT_COMPILED: C2RustUnnamed_0 = 49;
pub type LOCFSERROR = ::core::ffi::c_int;
pub type ODITHER_MATRIX = [[::core::ffi::c_int; 16]; 16];
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
pub const JERR_OUT_OF_MEMORY: C2RustUnnamed_0 = 56;
pub const JERR_NO_SOI: C2RustUnnamed_0 = 55;
pub const JERR_NO_QUANT_TABLE: C2RustUnnamed_0 = 54;
pub const JERR_NO_IMAGE: C2RustUnnamed_0 = 53;
pub const JERR_NO_HUFF_TABLE: C2RustUnnamed_0 = 52;
pub const JERR_NO_BACKING_STORE: C2RustUnnamed_0 = 51;
pub const JERR_NO_ARITH_TABLE: C2RustUnnamed_0 = 50;
pub const JERR_NOTIMPL: C2RustUnnamed_0 = 48;
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
pub const ODITHER_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ODITHER_CELLS: ::core::ffi::c_int = ODITHER_SIZE * ODITHER_SIZE;
pub const ODITHER_MASK: ::core::ffi::c_int = ODITHER_SIZE - 1 as ::core::ffi::c_int;
static mut base_dither_matrix: [[UINT8; 16]; 16] = [
    [
        0 as ::core::ffi::c_int as UINT8,
        192 as ::core::ffi::c_int as UINT8,
        48 as ::core::ffi::c_int as UINT8,
        240 as ::core::ffi::c_int as UINT8,
        12 as ::core::ffi::c_int as UINT8,
        204 as ::core::ffi::c_int as UINT8,
        60 as ::core::ffi::c_int as UINT8,
        252 as ::core::ffi::c_int as UINT8,
        3 as ::core::ffi::c_int as UINT8,
        195 as ::core::ffi::c_int as UINT8,
        51 as ::core::ffi::c_int as UINT8,
        243 as ::core::ffi::c_int as UINT8,
        15 as ::core::ffi::c_int as UINT8,
        207 as ::core::ffi::c_int as UINT8,
        63 as ::core::ffi::c_int as UINT8,
        255 as ::core::ffi::c_int as UINT8,
    ],
    [
        128 as ::core::ffi::c_int as UINT8,
        64 as ::core::ffi::c_int as UINT8,
        176 as ::core::ffi::c_int as UINT8,
        112 as ::core::ffi::c_int as UINT8,
        140 as ::core::ffi::c_int as UINT8,
        76 as ::core::ffi::c_int as UINT8,
        188 as ::core::ffi::c_int as UINT8,
        124 as ::core::ffi::c_int as UINT8,
        131 as ::core::ffi::c_int as UINT8,
        67 as ::core::ffi::c_int as UINT8,
        179 as ::core::ffi::c_int as UINT8,
        115 as ::core::ffi::c_int as UINT8,
        143 as ::core::ffi::c_int as UINT8,
        79 as ::core::ffi::c_int as UINT8,
        191 as ::core::ffi::c_int as UINT8,
        127 as ::core::ffi::c_int as UINT8,
    ],
    [
        32 as ::core::ffi::c_int as UINT8,
        224 as ::core::ffi::c_int as UINT8,
        16 as ::core::ffi::c_int as UINT8,
        208 as ::core::ffi::c_int as UINT8,
        44 as ::core::ffi::c_int as UINT8,
        236 as ::core::ffi::c_int as UINT8,
        28 as ::core::ffi::c_int as UINT8,
        220 as ::core::ffi::c_int as UINT8,
        35 as ::core::ffi::c_int as UINT8,
        227 as ::core::ffi::c_int as UINT8,
        19 as ::core::ffi::c_int as UINT8,
        211 as ::core::ffi::c_int as UINT8,
        47 as ::core::ffi::c_int as UINT8,
        239 as ::core::ffi::c_int as UINT8,
        31 as ::core::ffi::c_int as UINT8,
        223 as ::core::ffi::c_int as UINT8,
    ],
    [
        160 as ::core::ffi::c_int as UINT8,
        96 as ::core::ffi::c_int as UINT8,
        144 as ::core::ffi::c_int as UINT8,
        80 as ::core::ffi::c_int as UINT8,
        172 as ::core::ffi::c_int as UINT8,
        108 as ::core::ffi::c_int as UINT8,
        156 as ::core::ffi::c_int as UINT8,
        92 as ::core::ffi::c_int as UINT8,
        163 as ::core::ffi::c_int as UINT8,
        99 as ::core::ffi::c_int as UINT8,
        147 as ::core::ffi::c_int as UINT8,
        83 as ::core::ffi::c_int as UINT8,
        175 as ::core::ffi::c_int as UINT8,
        111 as ::core::ffi::c_int as UINT8,
        159 as ::core::ffi::c_int as UINT8,
        95 as ::core::ffi::c_int as UINT8,
    ],
    [
        8 as ::core::ffi::c_int as UINT8,
        200 as ::core::ffi::c_int as UINT8,
        56 as ::core::ffi::c_int as UINT8,
        248 as ::core::ffi::c_int as UINT8,
        4 as ::core::ffi::c_int as UINT8,
        196 as ::core::ffi::c_int as UINT8,
        52 as ::core::ffi::c_int as UINT8,
        244 as ::core::ffi::c_int as UINT8,
        11 as ::core::ffi::c_int as UINT8,
        203 as ::core::ffi::c_int as UINT8,
        59 as ::core::ffi::c_int as UINT8,
        251 as ::core::ffi::c_int as UINT8,
        7 as ::core::ffi::c_int as UINT8,
        199 as ::core::ffi::c_int as UINT8,
        55 as ::core::ffi::c_int as UINT8,
        247 as ::core::ffi::c_int as UINT8,
    ],
    [
        136 as ::core::ffi::c_int as UINT8,
        72 as ::core::ffi::c_int as UINT8,
        184 as ::core::ffi::c_int as UINT8,
        120 as ::core::ffi::c_int as UINT8,
        132 as ::core::ffi::c_int as UINT8,
        68 as ::core::ffi::c_int as UINT8,
        180 as ::core::ffi::c_int as UINT8,
        116 as ::core::ffi::c_int as UINT8,
        139 as ::core::ffi::c_int as UINT8,
        75 as ::core::ffi::c_int as UINT8,
        187 as ::core::ffi::c_int as UINT8,
        123 as ::core::ffi::c_int as UINT8,
        135 as ::core::ffi::c_int as UINT8,
        71 as ::core::ffi::c_int as UINT8,
        183 as ::core::ffi::c_int as UINT8,
        119 as ::core::ffi::c_int as UINT8,
    ],
    [
        40 as ::core::ffi::c_int as UINT8,
        232 as ::core::ffi::c_int as UINT8,
        24 as ::core::ffi::c_int as UINT8,
        216 as ::core::ffi::c_int as UINT8,
        36 as ::core::ffi::c_int as UINT8,
        228 as ::core::ffi::c_int as UINT8,
        20 as ::core::ffi::c_int as UINT8,
        212 as ::core::ffi::c_int as UINT8,
        43 as ::core::ffi::c_int as UINT8,
        235 as ::core::ffi::c_int as UINT8,
        27 as ::core::ffi::c_int as UINT8,
        219 as ::core::ffi::c_int as UINT8,
        39 as ::core::ffi::c_int as UINT8,
        231 as ::core::ffi::c_int as UINT8,
        23 as ::core::ffi::c_int as UINT8,
        215 as ::core::ffi::c_int as UINT8,
    ],
    [
        168 as ::core::ffi::c_int as UINT8,
        104 as ::core::ffi::c_int as UINT8,
        152 as ::core::ffi::c_int as UINT8,
        88 as ::core::ffi::c_int as UINT8,
        164 as ::core::ffi::c_int as UINT8,
        100 as ::core::ffi::c_int as UINT8,
        148 as ::core::ffi::c_int as UINT8,
        84 as ::core::ffi::c_int as UINT8,
        171 as ::core::ffi::c_int as UINT8,
        107 as ::core::ffi::c_int as UINT8,
        155 as ::core::ffi::c_int as UINT8,
        91 as ::core::ffi::c_int as UINT8,
        167 as ::core::ffi::c_int as UINT8,
        103 as ::core::ffi::c_int as UINT8,
        151 as ::core::ffi::c_int as UINT8,
        87 as ::core::ffi::c_int as UINT8,
    ],
    [
        2 as ::core::ffi::c_int as UINT8,
        194 as ::core::ffi::c_int as UINT8,
        50 as ::core::ffi::c_int as UINT8,
        242 as ::core::ffi::c_int as UINT8,
        14 as ::core::ffi::c_int as UINT8,
        206 as ::core::ffi::c_int as UINT8,
        62 as ::core::ffi::c_int as UINT8,
        254 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        193 as ::core::ffi::c_int as UINT8,
        49 as ::core::ffi::c_int as UINT8,
        241 as ::core::ffi::c_int as UINT8,
        13 as ::core::ffi::c_int as UINT8,
        205 as ::core::ffi::c_int as UINT8,
        61 as ::core::ffi::c_int as UINT8,
        253 as ::core::ffi::c_int as UINT8,
    ],
    [
        130 as ::core::ffi::c_int as UINT8,
        66 as ::core::ffi::c_int as UINT8,
        178 as ::core::ffi::c_int as UINT8,
        114 as ::core::ffi::c_int as UINT8,
        142 as ::core::ffi::c_int as UINT8,
        78 as ::core::ffi::c_int as UINT8,
        190 as ::core::ffi::c_int as UINT8,
        126 as ::core::ffi::c_int as UINT8,
        129 as ::core::ffi::c_int as UINT8,
        65 as ::core::ffi::c_int as UINT8,
        177 as ::core::ffi::c_int as UINT8,
        113 as ::core::ffi::c_int as UINT8,
        141 as ::core::ffi::c_int as UINT8,
        77 as ::core::ffi::c_int as UINT8,
        189 as ::core::ffi::c_int as UINT8,
        125 as ::core::ffi::c_int as UINT8,
    ],
    [
        34 as ::core::ffi::c_int as UINT8,
        226 as ::core::ffi::c_int as UINT8,
        18 as ::core::ffi::c_int as UINT8,
        210 as ::core::ffi::c_int as UINT8,
        46 as ::core::ffi::c_int as UINT8,
        238 as ::core::ffi::c_int as UINT8,
        30 as ::core::ffi::c_int as UINT8,
        222 as ::core::ffi::c_int as UINT8,
        33 as ::core::ffi::c_int as UINT8,
        225 as ::core::ffi::c_int as UINT8,
        17 as ::core::ffi::c_int as UINT8,
        209 as ::core::ffi::c_int as UINT8,
        45 as ::core::ffi::c_int as UINT8,
        237 as ::core::ffi::c_int as UINT8,
        29 as ::core::ffi::c_int as UINT8,
        221 as ::core::ffi::c_int as UINT8,
    ],
    [
        162 as ::core::ffi::c_int as UINT8,
        98 as ::core::ffi::c_int as UINT8,
        146 as ::core::ffi::c_int as UINT8,
        82 as ::core::ffi::c_int as UINT8,
        174 as ::core::ffi::c_int as UINT8,
        110 as ::core::ffi::c_int as UINT8,
        158 as ::core::ffi::c_int as UINT8,
        94 as ::core::ffi::c_int as UINT8,
        161 as ::core::ffi::c_int as UINT8,
        97 as ::core::ffi::c_int as UINT8,
        145 as ::core::ffi::c_int as UINT8,
        81 as ::core::ffi::c_int as UINT8,
        173 as ::core::ffi::c_int as UINT8,
        109 as ::core::ffi::c_int as UINT8,
        157 as ::core::ffi::c_int as UINT8,
        93 as ::core::ffi::c_int as UINT8,
    ],
    [
        10 as ::core::ffi::c_int as UINT8,
        202 as ::core::ffi::c_int as UINT8,
        58 as ::core::ffi::c_int as UINT8,
        250 as ::core::ffi::c_int as UINT8,
        6 as ::core::ffi::c_int as UINT8,
        198 as ::core::ffi::c_int as UINT8,
        54 as ::core::ffi::c_int as UINT8,
        246 as ::core::ffi::c_int as UINT8,
        9 as ::core::ffi::c_int as UINT8,
        201 as ::core::ffi::c_int as UINT8,
        57 as ::core::ffi::c_int as UINT8,
        249 as ::core::ffi::c_int as UINT8,
        5 as ::core::ffi::c_int as UINT8,
        197 as ::core::ffi::c_int as UINT8,
        53 as ::core::ffi::c_int as UINT8,
        245 as ::core::ffi::c_int as UINT8,
    ],
    [
        138 as ::core::ffi::c_int as UINT8,
        74 as ::core::ffi::c_int as UINT8,
        186 as ::core::ffi::c_int as UINT8,
        122 as ::core::ffi::c_int as UINT8,
        134 as ::core::ffi::c_int as UINT8,
        70 as ::core::ffi::c_int as UINT8,
        182 as ::core::ffi::c_int as UINT8,
        118 as ::core::ffi::c_int as UINT8,
        137 as ::core::ffi::c_int as UINT8,
        73 as ::core::ffi::c_int as UINT8,
        185 as ::core::ffi::c_int as UINT8,
        121 as ::core::ffi::c_int as UINT8,
        133 as ::core::ffi::c_int as UINT8,
        69 as ::core::ffi::c_int as UINT8,
        181 as ::core::ffi::c_int as UINT8,
        117 as ::core::ffi::c_int as UINT8,
    ],
    [
        42 as ::core::ffi::c_int as UINT8,
        234 as ::core::ffi::c_int as UINT8,
        26 as ::core::ffi::c_int as UINT8,
        218 as ::core::ffi::c_int as UINT8,
        38 as ::core::ffi::c_int as UINT8,
        230 as ::core::ffi::c_int as UINT8,
        22 as ::core::ffi::c_int as UINT8,
        214 as ::core::ffi::c_int as UINT8,
        41 as ::core::ffi::c_int as UINT8,
        233 as ::core::ffi::c_int as UINT8,
        25 as ::core::ffi::c_int as UINT8,
        217 as ::core::ffi::c_int as UINT8,
        37 as ::core::ffi::c_int as UINT8,
        229 as ::core::ffi::c_int as UINT8,
        21 as ::core::ffi::c_int as UINT8,
        213 as ::core::ffi::c_int as UINT8,
    ],
    [
        170 as ::core::ffi::c_int as UINT8,
        106 as ::core::ffi::c_int as UINT8,
        154 as ::core::ffi::c_int as UINT8,
        90 as ::core::ffi::c_int as UINT8,
        166 as ::core::ffi::c_int as UINT8,
        102 as ::core::ffi::c_int as UINT8,
        150 as ::core::ffi::c_int as UINT8,
        86 as ::core::ffi::c_int as UINT8,
        169 as ::core::ffi::c_int as UINT8,
        105 as ::core::ffi::c_int as UINT8,
        153 as ::core::ffi::c_int as UINT8,
        89 as ::core::ffi::c_int as UINT8,
        165 as ::core::ffi::c_int as UINT8,
        101 as ::core::ffi::c_int as UINT8,
        149 as ::core::ffi::c_int as UINT8,
        85 as ::core::ffi::c_int as UINT8,
    ],
];
pub const MAX_Q_COMPS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn select_ncolors(
    mut cinfo: j_decompress_ptr,
    mut Ncolors: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nc: ::core::ffi::c_int = (*cinfo).out_color_components;
    let mut max_colors: ::core::ffi::c_int = (*cinfo).desired_number_of_colors;
    let mut total_colors: ::core::ffi::c_int = 0;
    let mut iroot: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut changed: boolean = 0;
    let mut temp: ::core::ffi::c_long = 0;
    let mut RGB_order: [::core::ffi::c_int; 3] = [RGB_GREEN, RGB_RED, RGB_BLUE];
    RGB_order[0 as ::core::ffi::c_int as usize] = rgb_green[(*cinfo).out_color_space
        as usize];
    RGB_order[1 as ::core::ffi::c_int as usize] = rgb_red[(*cinfo).out_color_space
        as usize];
    RGB_order[2 as ::core::ffi::c_int as usize] = rgb_blue[(*cinfo).out_color_space
        as usize];
    iroot = 1 as ::core::ffi::c_int;
    loop {
        iroot += 1;
        temp = iroot as ::core::ffi::c_long;
        i = 1 as ::core::ffi::c_int;
        while i < nc {
            temp *= iroot as ::core::ffi::c_long;
            i += 1;
        }
        if !(temp <= max_colors as ::core::ffi::c_long) {
            break;
        }
    }
    iroot -= 1;
    if iroot < 2 as ::core::ffi::c_int {
        (*(*cinfo).err).msg_code = JERR_QUANT_FEW_COLORS as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = temp
            as ::core::ffi::c_int;
        Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    total_colors = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nc {
        *Ncolors.offset(i as isize) = iroot;
        total_colors *= iroot;
        i += 1;
    }
    loop {
        changed = FALSE as boolean;
        i = 0 as ::core::ffi::c_int;
        while i < nc {
            j = if (*cinfo).out_color_space as ::core::ffi::c_uint
                == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                RGB_order[i as usize]
            } else {
                i
            };
            temp = (total_colors / *Ncolors.offset(j as isize)) as ::core::ffi::c_long;
            temp
                *= (*Ncolors.offset(j as isize) + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_long;
            if temp > max_colors as ::core::ffi::c_long {
                break;
            }
            let ref mut fresh1 = *Ncolors.offset(j as isize);
            *fresh1 += 1;
            total_colors = temp as ::core::ffi::c_int;
            changed = TRUE as boolean;
            i += 1;
        }
        if !(changed != 0) {
            break;
        }
    }
    return total_colors;
}
unsafe extern "C" fn output_value(
    mut cinfo: j_decompress_ptr,
    mut ci: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
    mut maxj: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return ((j as JLONG * MAXJSAMPLE as JLONG
        + (maxj / 2 as ::core::ffi::c_int) as JLONG) / maxj as JLONG)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn largest_input_value(
    mut cinfo: j_decompress_ptr,
    mut ci: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
    mut maxj: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (((2 as ::core::ffi::c_int * j + 1 as ::core::ffi::c_int) as JLONG
        * MAXJSAMPLE as JLONG + maxj as JLONG)
        / (2 as ::core::ffi::c_int * maxj) as JLONG) as ::core::ffi::c_int;
}
unsafe extern "C" fn create_colormap(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut colormap: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    let mut total_colors: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut nci: ::core::ffi::c_int = 0;
    let mut blksize: ::core::ffi::c_int = 0;
    let mut blkdist: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_int = 0;
    total_colors = select_ncolors(
        cinfo,
        &raw mut (*cquantize).Ncolors as *mut ::core::ffi::c_int,
    );
    if (*cinfo).out_color_components == 3 as ::core::ffi::c_int {
        let mut _mp: *mut ::core::ffi::c_int = &raw mut (*(*cinfo).err).msg_parm.i
            as *mut ::core::ffi::c_int;
        *_mp.offset(0 as ::core::ffi::c_int as isize) = total_colors;
        *_mp.offset(1 as ::core::ffi::c_int as isize) = (*cquantize)
            .Ncolors[0 as ::core::ffi::c_int as usize];
        *_mp.offset(2 as ::core::ffi::c_int as isize) = (*cquantize)
            .Ncolors[1 as ::core::ffi::c_int as usize];
        *_mp.offset(3 as ::core::ffi::c_int as isize) = (*cquantize)
            .Ncolors[2 as ::core::ffi::c_int as usize];
        (*(*cinfo).err).msg_code = JTRC_QUANT_3_NCOLORS as ::core::ffi::c_int;
        Some((*(*cinfo).err).emit_message.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    } else {
        (*(*cinfo).err).msg_code = JTRC_QUANT_NCOLORS as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = total_colors;
        Some((*(*cinfo).err).emit_message.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    }
    colormap = Some((*(*cinfo).mem).alloc_sarray.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        total_colors as JDIMENSION,
        (*cinfo).out_color_components as JDIMENSION,
    );
    blkdist = total_colors;
    i = 0 as ::core::ffi::c_int;
    while i < (*cinfo).out_color_components {
        nci = (*cquantize).Ncolors[i as usize];
        blksize = blkdist / nci;
        j = 0 as ::core::ffi::c_int;
        while j < nci {
            val = output_value(cinfo, i, j, nci - 1 as ::core::ffi::c_int);
            ptr = j * blksize;
            while ptr < total_colors {
                k = 0 as ::core::ffi::c_int;
                while k < blksize {
                    *(*colormap.offset(i as isize)).offset((ptr + k) as isize) = val
                        as JSAMPLE;
                    k += 1;
                }
                ptr += blkdist;
            }
            j += 1;
        }
        blkdist = blksize;
        i += 1;
    }
    (*cquantize).sv_colormap = colormap;
    (*cquantize).sv_actual = total_colors;
}
unsafe extern "C" fn create_colorindex(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut indexptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut nci: ::core::ffi::c_int = 0;
    let mut blksize: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_int = 0;
    let mut pad: ::core::ffi::c_int = 0;
    if (*cinfo).dither_mode as ::core::ffi::c_uint
        == JDITHER_ORDERED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        pad = MAXJSAMPLE * 2 as ::core::ffi::c_int;
        (*cquantize).is_padded = TRUE as boolean;
    } else {
        pad = 0 as ::core::ffi::c_int;
        (*cquantize).is_padded = FALSE as boolean;
    }
    (*cquantize).colorindex = Some(
            (*(*cinfo).mem).alloc_sarray.expect("non-null function pointer"),
        )
        .expect(
            "non-null function pointer",
        )(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (MAXJSAMPLE + 1 as ::core::ffi::c_int + pad) as JDIMENSION,
        (*cinfo).out_color_components as JDIMENSION,
    );
    blksize = (*cquantize).sv_actual;
    i = 0 as ::core::ffi::c_int;
    while i < (*cinfo).out_color_components {
        nci = (*cquantize).Ncolors[i as usize];
        blksize = blksize / nci;
        if pad != 0 {
            let ref mut fresh0 = *(*cquantize).colorindex.offset(i as isize);
            *fresh0 = (*fresh0).offset(MAXJSAMPLE as isize);
        }
        indexptr = *(*cquantize).colorindex.offset(i as isize);
        val = 0 as ::core::ffi::c_int;
        k = largest_input_value(
            cinfo,
            i,
            0 as ::core::ffi::c_int,
            nci - 1 as ::core::ffi::c_int,
        );
        j = 0 as ::core::ffi::c_int;
        while j <= MAXJSAMPLE {
            while j > k {
                val += 1;
                k = largest_input_value(cinfo, i, val, nci - 1 as ::core::ffi::c_int);
            }
            *indexptr.offset(j as isize) = (val * blksize) as JSAMPLE;
            j += 1;
        }
        if pad != 0 {
            j = 1 as ::core::ffi::c_int;
            while j <= MAXJSAMPLE {
                *indexptr.offset(-j as isize) = *indexptr
                    .offset(0 as ::core::ffi::c_int as isize);
                *indexptr.offset((MAXJSAMPLE + j) as isize) = *indexptr
                    .offset(MAXJSAMPLE as isize);
                j += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn make_odither_array(
    mut cinfo: j_decompress_ptr,
    mut ncolors: ::core::ffi::c_int,
) -> ODITHER_MATRIX_PTR {
    let mut odither: ODITHER_MATRIX_PTR = ::core::ptr::null_mut::<
        [::core::ffi::c_int; 16],
    >();
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut num: JLONG = 0;
    let mut den: JLONG = 0;
    odither = Some((*(*cinfo).mem).alloc_small.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<ODITHER_MATRIX>() as size_t,
    ) as ODITHER_MATRIX_PTR;
    den = (2 as ::core::ffi::c_int * ODITHER_CELLS) as JLONG
        * (ncolors - 1 as ::core::ffi::c_int) as JLONG;
    j = 0 as ::core::ffi::c_int;
    while j < ODITHER_SIZE {
        k = 0 as ::core::ffi::c_int;
        while k < ODITHER_SIZE {
            num = (ODITHER_CELLS - 1 as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int
                    * base_dither_matrix[j as usize][k as usize] as ::core::ffi::c_int)
                as JLONG * MAXJSAMPLE as JLONG;
            (*odither.offset(j as isize))[k as usize] = (if num < 0 as JLONG {
                -(-num / den)
            } else {
                num / den
            }) as ::core::ffi::c_int;
            k += 1;
        }
        j += 1;
    }
    return odither;
}
unsafe extern "C" fn create_odither_tables(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut odither: ODITHER_MATRIX_PTR = ::core::ptr::null_mut::<
        [::core::ffi::c_int; 16],
    >();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nci: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*cinfo).out_color_components {
        nci = (*cquantize).Ncolors[i as usize];
        odither = ::core::ptr::null_mut::<[::core::ffi::c_int; 16]>();
        j = 0 as ::core::ffi::c_int;
        while j < i {
            if nci == (*cquantize).Ncolors[j as usize] {
                odither = (*cquantize).odither[j as usize];
                break;
            } else {
                j += 1;
            }
        }
        if odither.is_null() {
            odither = make_odither_array(cinfo, nci);
        }
        (*cquantize).odither[i as usize] = odither;
        i += 1;
    }
}
unsafe extern "C" fn color_quantize(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut colorindex: JSAMPARRAY = (*cquantize).colorindex;
    let mut pixcode: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut ptrin: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut ptrout: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut row: ::core::ffi::c_int = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    let mut nc: ::core::ffi::c_int = (*cinfo).out_color_components;
    row = 0 as ::core::ffi::c_int;
    while row < num_rows {
        ptrin = *input_buf.offset(row as isize);
        ptrout = *output_buf.offset(row as isize);
        col = width;
        while col > 0 as JDIMENSION {
            pixcode = 0 as ::core::ffi::c_int;
            ci = 0 as ::core::ffi::c_int;
            while ci < nc {
                let fresh6 = ptrin;
                ptrin = ptrin.offset(1);
                pixcode
                    += *(*colorindex.offset(ci as isize)).offset(*fresh6 as isize)
                        as ::core::ffi::c_int;
                ci += 1;
            }
            let fresh7 = ptrout;
            ptrout = ptrout.offset(1);
            *fresh7 = pixcode as JSAMPLE;
            col = col.wrapping_sub(1);
        }
        row += 1;
    }
}
unsafe extern "C" fn color_quantize3(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut pixcode: ::core::ffi::c_int = 0;
    let mut ptrin: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut ptrout: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut colorindex0: JSAMPROW = *(*cquantize)
        .colorindex
        .offset(0 as ::core::ffi::c_int as isize);
    let mut colorindex1: JSAMPROW = *(*cquantize)
        .colorindex
        .offset(1 as ::core::ffi::c_int as isize);
    let mut colorindex2: JSAMPROW = *(*cquantize)
        .colorindex
        .offset(2 as ::core::ffi::c_int as isize);
    let mut row: ::core::ffi::c_int = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    row = 0 as ::core::ffi::c_int;
    while row < num_rows {
        ptrin = *input_buf.offset(row as isize);
        ptrout = *output_buf.offset(row as isize);
        col = width;
        while col > 0 as JDIMENSION {
            let fresh8 = ptrin;
            ptrin = ptrin.offset(1);
            pixcode = *colorindex0.offset(*fresh8 as isize) as ::core::ffi::c_int;
            let fresh9 = ptrin;
            ptrin = ptrin.offset(1);
            pixcode += *colorindex1.offset(*fresh9 as isize) as ::core::ffi::c_int;
            let fresh10 = ptrin;
            ptrin = ptrin.offset(1);
            pixcode += *colorindex2.offset(*fresh10 as isize) as ::core::ffi::c_int;
            let fresh11 = ptrout;
            ptrout = ptrout.offset(1);
            *fresh11 = pixcode as JSAMPLE;
            col = col.wrapping_sub(1);
        }
        row += 1;
    }
}
unsafe extern "C" fn quantize_ord_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut input_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut output_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut colorindex_ci: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut dither: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut row_index: ::core::ffi::c_int = 0;
    let mut col_index: ::core::ffi::c_int = 0;
    let mut nc: ::core::ffi::c_int = (*cinfo).out_color_components;
    let mut ci: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    row = 0 as ::core::ffi::c_int;
    while row < num_rows {
        jzero_far(
            *output_buf.offset(row as isize) as *mut ::core::ffi::c_void,
            (width as usize).wrapping_mul(::core::mem::size_of::<JSAMPLE>() as usize),
        );
        row_index = (*cquantize).row_index;
        ci = 0 as ::core::ffi::c_int;
        while ci < nc {
            input_ptr = (*input_buf.offset(row as isize)).offset(ci as isize);
            output_ptr = *output_buf.offset(row as isize);
            colorindex_ci = *(*cquantize).colorindex.offset(ci as isize);
            dither = &raw mut *(*(&raw mut (*cquantize).odither
                as *mut ODITHER_MATRIX_PTR)
                .offset(ci as isize))
                .offset(row_index as isize) as *mut ::core::ffi::c_int;
            col_index = 0 as ::core::ffi::c_int;
            col = width;
            while col > 0 as JDIMENSION {
                *output_ptr = (*output_ptr as ::core::ffi::c_int
                    + *colorindex_ci
                        .offset(
                            (*input_ptr as ::core::ffi::c_int
                                + *dither.offset(col_index as isize)) as isize,
                        ) as ::core::ffi::c_int) as JSAMPLE;
                input_ptr = input_ptr.offset(nc as isize);
                output_ptr = output_ptr.offset(1);
                col_index = col_index + 1 as ::core::ffi::c_int & ODITHER_MASK;
                col = col.wrapping_sub(1);
            }
            ci += 1;
        }
        row_index = row_index + 1 as ::core::ffi::c_int & ODITHER_MASK;
        (*cquantize).row_index = row_index;
        row += 1;
    }
}
unsafe extern "C" fn quantize3_ord_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut pixcode: ::core::ffi::c_int = 0;
    let mut input_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut output_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut colorindex0: JSAMPROW = *(*cquantize)
        .colorindex
        .offset(0 as ::core::ffi::c_int as isize);
    let mut colorindex1: JSAMPROW = *(*cquantize)
        .colorindex
        .offset(1 as ::core::ffi::c_int as isize);
    let mut colorindex2: JSAMPROW = *(*cquantize)
        .colorindex
        .offset(2 as ::core::ffi::c_int as isize);
    let mut dither0: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut dither1: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut dither2: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut row_index: ::core::ffi::c_int = 0;
    let mut col_index: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    row = 0 as ::core::ffi::c_int;
    while row < num_rows {
        row_index = (*cquantize).row_index;
        input_ptr = *input_buf.offset(row as isize);
        output_ptr = *output_buf.offset(row as isize);
        dither0 = &raw mut *(*(&raw mut (*cquantize).odither as *mut ODITHER_MATRIX_PTR)
            .offset(0 as ::core::ffi::c_int as isize))
            .offset(row_index as isize) as *mut ::core::ffi::c_int;
        dither1 = &raw mut *(*(&raw mut (*cquantize).odither as *mut ODITHER_MATRIX_PTR)
            .offset(1 as ::core::ffi::c_int as isize))
            .offset(row_index as isize) as *mut ::core::ffi::c_int;
        dither2 = &raw mut *(*(&raw mut (*cquantize).odither as *mut ODITHER_MATRIX_PTR)
            .offset(2 as ::core::ffi::c_int as isize))
            .offset(row_index as isize) as *mut ::core::ffi::c_int;
        col_index = 0 as ::core::ffi::c_int;
        col = width;
        while col > 0 as JDIMENSION {
            let fresh2 = input_ptr;
            input_ptr = input_ptr.offset(1);
            pixcode = *colorindex0
                .offset(
                    (*fresh2 as ::core::ffi::c_int + *dither0.offset(col_index as isize))
                        as isize,
                ) as ::core::ffi::c_int;
            let fresh3 = input_ptr;
            input_ptr = input_ptr.offset(1);
            pixcode
                += *colorindex1
                    .offset(
                        (*fresh3 as ::core::ffi::c_int
                            + *dither1.offset(col_index as isize)) as isize,
                    ) as ::core::ffi::c_int;
            let fresh4 = input_ptr;
            input_ptr = input_ptr.offset(1);
            pixcode
                += *colorindex2
                    .offset(
                        (*fresh4 as ::core::ffi::c_int
                            + *dither2.offset(col_index as isize)) as isize,
                    ) as ::core::ffi::c_int;
            let fresh5 = output_ptr;
            output_ptr = output_ptr.offset(1);
            *fresh5 = pixcode as JSAMPLE;
            col_index = col_index + 1 as ::core::ffi::c_int & ODITHER_MASK;
            col = col.wrapping_sub(1);
        }
        row_index = row_index + 1 as ::core::ffi::c_int & ODITHER_MASK;
        (*cquantize).row_index = row_index;
        row += 1;
    }
}
unsafe extern "C" fn quantize_fs_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut cur: LOCFSERROR = 0;
    let mut belowerr: LOCFSERROR = 0;
    let mut bpreverr: LOCFSERROR = 0;
    let mut bnexterr: LOCFSERROR = 0;
    let mut delta: LOCFSERROR = 0;
    let mut errorptr: FSERRPTR = ::core::ptr::null_mut::<FSERROR>();
    let mut input_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut output_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut colorindex_ci: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut colormap_ci: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut pixcode: ::core::ffi::c_int = 0;
    let mut nc: ::core::ffi::c_int = (*cinfo).out_color_components;
    let mut dir: ::core::ffi::c_int = 0;
    let mut dirnc: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    row = 0 as ::core::ffi::c_int;
    while row < num_rows {
        jzero_far(
            *output_buf.offset(row as isize) as *mut ::core::ffi::c_void,
            (width as usize).wrapping_mul(::core::mem::size_of::<JSAMPLE>() as usize),
        );
        ci = 0 as ::core::ffi::c_int;
        while ci < nc {
            input_ptr = (*input_buf.offset(row as isize)).offset(ci as isize);
            output_ptr = *output_buf.offset(row as isize);
            if (*cquantize).on_odd_row != 0 {
                input_ptr = input_ptr
                    .offset(
                        width
                            .wrapping_sub(1 as JDIMENSION)
                            .wrapping_mul(nc as JDIMENSION) as isize,
                    );
                output_ptr = output_ptr
                    .offset(width.wrapping_sub(1 as JDIMENSION) as isize);
                dir = -(1 as ::core::ffi::c_int);
                dirnc = -nc;
                errorptr = (*cquantize)
                    .fserrors[ci as usize]
                    .offset(width.wrapping_add(1 as JDIMENSION) as isize);
            } else {
                dir = 1 as ::core::ffi::c_int;
                dirnc = nc;
                errorptr = (*cquantize).fserrors[ci as usize];
            }
            colorindex_ci = *(*cquantize).colorindex.offset(ci as isize);
            colormap_ci = *(*cquantize).sv_colormap.offset(ci as isize);
            cur = 0 as ::core::ffi::c_int as LOCFSERROR;
            bpreverr = 0 as ::core::ffi::c_int as LOCFSERROR;
            belowerr = bpreverr;
            col = width;
            while col > 0 as JDIMENSION {
                cur = (cur as ::core::ffi::c_int
                    + *errorptr.offset(dir as isize) as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as LOCFSERROR;
                cur += *input_ptr as ::core::ffi::c_int;
                cur = *range_limit.offset(cur as isize) as LOCFSERROR;
                pixcode = *colorindex_ci.offset(cur as isize) as ::core::ffi::c_int;
                *output_ptr = (*output_ptr as ::core::ffi::c_int
                    + pixcode as JSAMPLE as ::core::ffi::c_int) as JSAMPLE;
                cur -= *colormap_ci.offset(pixcode as isize) as ::core::ffi::c_int;
                bnexterr = cur;
                delta = (cur as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    as LOCFSERROR;
                cur += delta;
                *errorptr.offset(0 as ::core::ffi::c_int as isize) = (bpreverr + cur)
                    as FSERROR;
                cur += delta;
                bpreverr = belowerr + cur;
                belowerr = bnexterr;
                cur += delta;
                input_ptr = input_ptr.offset(dirnc as isize);
                output_ptr = output_ptr.offset(dir as isize);
                errorptr = errorptr.offset(dir as isize);
                col = col.wrapping_sub(1);
            }
            *errorptr.offset(0 as ::core::ffi::c_int as isize) = bpreverr as FSERROR;
            ci += 1;
        }
        (*cquantize).on_odd_row = (if (*cquantize).on_odd_row != 0 {
            FALSE
        } else {
            TRUE
        }) as boolean;
        row += 1;
    }
}
unsafe extern "C" fn alloc_fs_workspace(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut arraysize: size_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    arraysize = ((*cinfo).output_width.wrapping_add(2 as JDIMENSION) as usize)
        .wrapping_mul(::core::mem::size_of::<FSERROR>() as usize);
    i = 0 as ::core::ffi::c_int;
    while i < (*cinfo).out_color_components {
        (*cquantize).fserrors[i as usize] = Some(
                (*(*cinfo).mem).alloc_large.expect("non-null function pointer"),
            )
            .expect(
                "non-null function pointer",
            )(cinfo as j_common_ptr, JPOOL_IMAGE, arraysize) as FSERRPTR;
        i += 1;
    }
}
unsafe extern "C" fn start_pass_1_quant(
    mut cinfo: j_decompress_ptr,
    mut is_pre_scan: boolean,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut arraysize: size_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    (*cinfo).colormap = (*cquantize).sv_colormap;
    (*cinfo).actual_number_of_colors = (*cquantize).sv_actual;
    match (*cinfo).dither_mode as ::core::ffi::c_uint {
        0 => {
            if (*cinfo).out_color_components == 3 as ::core::ffi::c_int {
                (*cquantize).pub_0.color_quantize = Some(
                    color_quantize3
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
                    color_quantize
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
        }
        1 => {
            if (*cinfo).out_color_components == 3 as ::core::ffi::c_int {
                (*cquantize).pub_0.color_quantize = Some(
                    quantize3_ord_dither
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
                    quantize_ord_dither
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
            (*cquantize).row_index = 0 as ::core::ffi::c_int;
            if (*cquantize).is_padded == 0 {
                create_colorindex(cinfo);
            }
            if (*cquantize).odither[0 as ::core::ffi::c_int as usize].is_null() {
                create_odither_tables(cinfo);
            }
        }
        2 => {
            (*cquantize).pub_0.color_quantize = Some(
                quantize_fs_dither
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
            (*cquantize).on_odd_row = FALSE as boolean;
            if (*cquantize).fserrors[0 as ::core::ffi::c_int as usize].is_null() {
                alloc_fs_workspace(cinfo);
            }
            arraysize = ((*cinfo).output_width.wrapping_add(2 as JDIMENSION) as usize)
                .wrapping_mul(::core::mem::size_of::<FSERROR>() as usize);
            i = 0 as ::core::ffi::c_int;
            while i < (*cinfo).out_color_components {
                jzero_far(
                    (*cquantize).fserrors[i as usize] as *mut ::core::ffi::c_void,
                    arraysize,
                );
                i += 1;
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_NOT_COMPILED as ::core::ffi::c_int;
            Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    };
}
unsafe extern "C" fn finish_pass_1_quant(mut cinfo: j_decompress_ptr) {}
unsafe extern "C" fn new_color_map_1_quant(mut cinfo: j_decompress_ptr) {
    (*(*cinfo).err).msg_code = JERR_MODE_CHANGE as ::core::ffi::c_int;
    Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
        .expect("non-null function pointer")(cinfo as j_common_ptr);
}
pub unsafe extern "C" fn jinit_1pass_quantizer(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = ::core::ptr::null_mut::<my_cquantizer>();
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
        start_pass_1_quant as unsafe extern "C" fn(j_decompress_ptr, boolean) -> (),
    ) as Option<unsafe extern "C" fn(j_decompress_ptr, boolean) -> ()>;
    (*cquantize).pub_0.finish_pass = Some(
        finish_pass_1_quant as unsafe extern "C" fn(j_decompress_ptr) -> (),
    ) as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
    (*cquantize).pub_0.new_color_map = Some(
        new_color_map_1_quant as unsafe extern "C" fn(j_decompress_ptr) -> (),
    ) as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
    (*cquantize).fserrors[0 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<
        FSERROR,
    >();
    (*cquantize).odither[0 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<
        [::core::ffi::c_int; 16],
    >();
    if (*cinfo).out_color_components > MAX_Q_COMPS {
        (*(*cinfo).err).msg_code = JERR_QUANT_COMPONENTS as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = 4
            as ::core::ffi::c_int;
        Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).desired_number_of_colors > MAXJSAMPLE + 1 as ::core::ffi::c_int {
        (*(*cinfo).err).msg_code = JERR_QUANT_MANY_COLORS as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = 255
            as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    create_colormap(cinfo);
    create_colorindex(cinfo);
    if (*cinfo).dither_mode as ::core::ffi::c_uint
        == JDITHER_FS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        alloc_fs_workspace(cinfo);
    }
}
