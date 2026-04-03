#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}

extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn jpeg_alloc_quant_table(cinfo: j_common_ptr) -> *mut JQUANT_TBL;
    fn jpeg_alloc_huff_table(cinfo: j_common_ptr) -> *mut JHUFF_TBL;
    static jpeg_natural_order: [::core::ffi::c_int; 0];
}
pub type size_t = usize;
pub type JSAMPLE = ::core::ffi::c_uchar;
pub type JCOEF = ::core::ffi::c_short;
pub type JOCTET = ::core::ffi::c_uchar;
pub type UINT8 = ::core::ffi::c_uchar;
pub type UINT16 = ::core::ffi::c_ushort;
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
pub const JERR_UNKNOWN_MARKER: C2RustUnnamed_0 = 70;
pub const M_APP0: C2RustUnnamed_1 = 224;
pub type my_marker_ptr = *mut my_marker_reader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_marker_reader {
    pub pub_0: jpeg_marker_reader,
    pub process_COM: jpeg_marker_parser_method,
    pub process_APPn: [jpeg_marker_parser_method; 16],
    pub length_limit_COM: ::core::ffi::c_uint,
    pub length_limit_APPn: [::core::ffi::c_uint; 16],
    pub cur_marker: jpeg_saved_marker_ptr,
    pub bytes_read: ::core::ffi::c_uint,
}
pub const M_APP15: C2RustUnnamed_1 = 239;
pub const M_COM: C2RustUnnamed_1 = 254;
pub type JLONG = ::core::ffi::c_long;
pub const JTRC_APP14: C2RustUnnamed_0 = 80;
pub const JTRC_ADOBE: C2RustUnnamed_0 = 78;
pub const M_APP14: C2RustUnnamed_1 = 238;
pub const JTRC_APP0: C2RustUnnamed_0 = 79;
pub const JTRC_JFIF_EXTENSION: C2RustUnnamed_0 = 91;
pub const JTRC_THUMB_RGB: C2RustUnnamed_0 = 112;
pub const JTRC_THUMB_PALETTE: C2RustUnnamed_0 = 111;
pub const JTRC_THUMB_JPEG: C2RustUnnamed_0 = 110;
pub const JTRC_JFIF_BADTHUMBNAILSIZE: C2RustUnnamed_0 = 90;
pub const JTRC_JFIF_THUMBNAIL: C2RustUnnamed_0 = 92;
pub const JTRC_JFIF: C2RustUnnamed_0 = 89;
pub const JWRN_JFIF_MAJOR: C2RustUnnamed_0 = 122;
pub const JTRC_MISC_MARKER: C2RustUnnamed_0 = 93;
pub const JWRN_EXTRANEOUS_DATA: C2RustUnnamed_0 = 119;
pub const JTRC_RECOVERY_ACTION: C2RustUnnamed_0 = 99;
pub const M_RST0: C2RustUnnamed_1 = 208;
pub const M_RST7: C2RustUnnamed_1 = 215;
pub const M_SOF0: C2RustUnnamed_1 = 192;
pub const JWRN_MUST_RESYNC: C2RustUnnamed_0 = 124;
pub const JTRC_RST: C2RustUnnamed_0 = 100;
pub const M_DNL: C2RustUnnamed_1 = 220;
pub const JTRC_PARMLESS_MARKER: C2RustUnnamed_0 = 94;
pub const M_TEM: C2RustUnnamed_1 = 1;
pub const M_RST6: C2RustUnnamed_1 = 214;
pub const M_RST5: C2RustUnnamed_1 = 213;
pub const M_RST4: C2RustUnnamed_1 = 212;
pub const M_RST3: C2RustUnnamed_1 = 211;
pub const M_RST2: C2RustUnnamed_1 = 210;
pub const M_RST1: C2RustUnnamed_1 = 209;
pub const M_APP13: C2RustUnnamed_1 = 237;
pub const M_APP12: C2RustUnnamed_1 = 236;
pub const M_APP11: C2RustUnnamed_1 = 235;
pub const M_APP10: C2RustUnnamed_1 = 234;
pub const M_APP9: C2RustUnnamed_1 = 233;
pub const M_APP8: C2RustUnnamed_1 = 232;
pub const M_APP7: C2RustUnnamed_1 = 231;
pub const M_APP6: C2RustUnnamed_1 = 230;
pub const M_APP5: C2RustUnnamed_1 = 229;
pub const M_APP4: C2RustUnnamed_1 = 228;
pub const M_APP3: C2RustUnnamed_1 = 227;
pub const M_APP2: C2RustUnnamed_1 = 226;
pub const M_APP1: C2RustUnnamed_1 = 225;
pub const JTRC_DRI: C2RustUnnamed_0 = 84;
pub const JERR_BAD_LENGTH: C2RustUnnamed_0 = 12;
pub const M_DRI: C2RustUnnamed_1 = 221;
pub const JTRC_QUANTVALS: C2RustUnnamed_0 = 95;
pub const JERR_DQT_INDEX: C2RustUnnamed_0 = 32;
pub const JTRC_DQT: C2RustUnnamed_0 = 83;
pub const M_DQT: C2RustUnnamed_1 = 219;
pub const JERR_DHT_INDEX: C2RustUnnamed_0 = 31;
pub const JERR_BAD_HUFF_TABLE: C2RustUnnamed_0 = 9;
pub const JTRC_HUFFBITS: C2RustUnnamed_0 = 88;
pub const JTRC_DHT: C2RustUnnamed_0 = 82;
pub const M_DHT: C2RustUnnamed_1 = 196;
pub const JERR_DAC_VALUE: C2RustUnnamed_0 = 30;
pub const JERR_DAC_INDEX: C2RustUnnamed_0 = 29;
pub const JTRC_DAC: C2RustUnnamed_0 = 81;
pub const M_DAC: C2RustUnnamed_1 = 204;
pub const JTRC_EOI: C2RustUnnamed_0 = 87;
pub const M_EOI: C2RustUnnamed_1 = 217;
pub const JTRC_SOS_PARAMS: C2RustUnnamed_0 = 107;
pub const JERR_BAD_COMPONENT_ID: C2RustUnnamed_0 = 4;
pub const JTRC_SOS_COMPONENT: C2RustUnnamed_0 = 106;
pub const JTRC_SOS: C2RustUnnamed_0 = 105;
pub const JERR_SOS_NO_SOF: C2RustUnnamed_0 = 64;
pub const M_SOS: C2RustUnnamed_1 = 218;
pub const JERR_SOF_UNSUPPORTED: C2RustUnnamed_0 = 62;
pub const M_SOF15: C2RustUnnamed_1 = 207;
pub const M_SOF14: C2RustUnnamed_1 = 206;
pub const M_SOF13: C2RustUnnamed_1 = 205;
pub const M_SOF11: C2RustUnnamed_1 = 203;
pub const M_JPG: C2RustUnnamed_1 = 200;
pub const M_SOF7: C2RustUnnamed_1 = 199;
pub const M_SOF6: C2RustUnnamed_1 = 198;
pub const M_SOF5: C2RustUnnamed_1 = 197;
pub const M_SOF3: C2RustUnnamed_1 = 195;
pub const JTRC_SOF_COMPONENT: C2RustUnnamed_0 = 103;
pub const JERR_EMPTY_IMAGE: C2RustUnnamed_0 = 33;
pub const JERR_SOF_DUPLICATE: C2RustUnnamed_0 = 60;
pub const JTRC_SOF: C2RustUnnamed_0 = 102;
pub const M_SOF10: C2RustUnnamed_1 = 202;
pub const M_SOF9: C2RustUnnamed_1 = 201;
pub const M_SOF2: C2RustUnnamed_1 = 194;
pub const M_SOF1: C2RustUnnamed_1 = 193;
pub const JERR_SOI_DUPLICATE: C2RustUnnamed_0 = 63;
pub const JTRC_SOI: C2RustUnnamed_0 = 104;
pub const M_SOI: C2RustUnnamed_1 = 216;
pub const JERR_NO_SOI: C2RustUnnamed_0 = 55;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const JMSG_LASTMSGCODE: C2RustUnnamed_0 = 128;
pub const JWRN_BOGUS_ICC: C2RustUnnamed_0 = 127;
pub const JWRN_TOO_MUCH_DATA: C2RustUnnamed_0 = 126;
pub const JWRN_NOT_SEQUENTIAL: C2RustUnnamed_0 = 125;
pub const JWRN_JPEG_EOF: C2RustUnnamed_0 = 123;
pub const JWRN_HUFF_BAD_CODE: C2RustUnnamed_0 = 121;
pub const JWRN_HIT_MARKER: C2RustUnnamed_0 = 120;
pub const JWRN_BOGUS_PROGRESSION: C2RustUnnamed_0 = 118;
pub const JWRN_ARITH_BAD_CODE: C2RustUnnamed_0 = 117;
pub const JWRN_ADOBE_XFORM: C2RustUnnamed_0 = 116;
pub const JTRC_XMS_OPEN: C2RustUnnamed_0 = 115;
pub const JTRC_XMS_CLOSE: C2RustUnnamed_0 = 114;
pub const JTRC_UNKNOWN_IDS: C2RustUnnamed_0 = 113;
pub const JTRC_TFILE_OPEN: C2RustUnnamed_0 = 109;
pub const JTRC_TFILE_CLOSE: C2RustUnnamed_0 = 108;
pub const JTRC_SMOOTH_NOTIMPL: C2RustUnnamed_0 = 101;
pub const JTRC_QUANT_SELECTED: C2RustUnnamed_0 = 98;
pub const JTRC_QUANT_NCOLORS: C2RustUnnamed_0 = 97;
pub const JTRC_QUANT_3_NCOLORS: C2RustUnnamed_0 = 96;
pub const JTRC_EMS_OPEN: C2RustUnnamed_0 = 86;
pub const JTRC_EMS_CLOSE: C2RustUnnamed_0 = 85;
pub const JTRC_16BIT_TABLES: C2RustUnnamed_0 = 77;
pub const JMSG_VERSION: C2RustUnnamed_0 = 76;
pub const JMSG_COPYRIGHT: C2RustUnnamed_0 = 75;
pub const JERR_XMS_WRITE: C2RustUnnamed_0 = 74;
pub const JERR_XMS_READ: C2RustUnnamed_0 = 73;
pub const JERR_WIDTH_OVERFLOW: C2RustUnnamed_0 = 72;
pub const JERR_VIRTUAL_BUG: C2RustUnnamed_0 = 71;
pub const JERR_TOO_LITTLE_DATA: C2RustUnnamed_0 = 69;
pub const JERR_TFILE_WRITE: C2RustUnnamed_0 = 68;
pub const JERR_TFILE_SEEK: C2RustUnnamed_0 = 67;
pub const JERR_TFILE_READ: C2RustUnnamed_0 = 66;
pub const JERR_TFILE_CREATE: C2RustUnnamed_0 = 65;
pub const JERR_SOF_NO_SOS: C2RustUnnamed_0 = 61;
pub const JERR_QUANT_MANY_COLORS: C2RustUnnamed_0 = 59;
pub const JERR_QUANT_FEW_COLORS: C2RustUnnamed_0 = 58;
pub const JERR_QUANT_COMPONENTS: C2RustUnnamed_0 = 57;
pub const JERR_OUT_OF_MEMORY: C2RustUnnamed_0 = 56;
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
pub const JERR_BAD_J_COLORSPACE: C2RustUnnamed_0 = 11;
pub const JERR_BAD_IN_COLORSPACE: C2RustUnnamed_0 = 10;
pub const JERR_BAD_DROP_SAMPLING: C2RustUnnamed_0 = 8;
pub const JERR_BAD_DCTSIZE: C2RustUnnamed_0 = 7;
pub const JERR_BAD_DCT_COEF: C2RustUnnamed_0 = 6;
pub const JERR_BAD_CROP_SPEC: C2RustUnnamed_0 = 5;
pub const JERR_BAD_BUFFER_MODE: C2RustUnnamed_0 = 3;
pub const JERR_BAD_ALLOC_CHUNK: C2RustUnnamed_0 = 2;
pub const JERR_BAD_ALIGN_TYPE: C2RustUnnamed_0 = 1;
pub const JMSG_NOMESSAGE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const M_ERROR: C2RustUnnamed_1 = 256;
pub const M_JPG13: C2RustUnnamed_1 = 253;
pub const M_JPG0: C2RustUnnamed_1 = 240;
pub const M_EXP: C2RustUnnamed_1 = 223;
pub const M_DHP: C2RustUnnamed_1 = 222;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DCTSIZE2: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NUM_QUANT_TBLS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NUM_HUFF_TBLS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NUM_ARITH_TBLS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MAX_COMPS_IN_SCAN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JPOOL_PERMANENT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPEG_SUSPENDED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JPEG_REACHED_SOS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPEG_REACHED_EOI: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn get_soi(mut cinfo: j_decompress_ptr) -> boolean {
    let mut i: ::core::ffi::c_int = 0;
    (*(*cinfo).err).msg_code = JTRC_SOI as ::core::ffi::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    if (*(*cinfo).marker).saw_SOI != 0 {
        (*(*cinfo).err).msg_code = JERR_SOI_DUPLICATE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    i = 0 as ::core::ffi::c_int;
    while i < NUM_ARITH_TBLS {
        (*cinfo).arith_dc_L[i as usize] = 0 as UINT8;
        (*cinfo).arith_dc_U[i as usize] = 1 as UINT8;
        (*cinfo).arith_ac_K[i as usize] = 5 as UINT8;
        i += 1;
    }
    (*cinfo).restart_interval = 0 as ::core::ffi::c_uint;
    (*cinfo).jpeg_color_space = JCS_UNKNOWN;
    (*cinfo).CCIR601_sampling = FALSE as boolean;
    (*cinfo).saw_JFIF_marker = FALSE as boolean;
    (*cinfo).JFIF_major_version = 1 as UINT8;
    (*cinfo).JFIF_minor_version = 1 as UINT8;
    (*cinfo).density_unit = 0 as UINT8;
    (*cinfo).X_density = 1 as UINT16;
    (*cinfo).Y_density = 1 as UINT16;
    (*cinfo).saw_Adobe_marker = FALSE as boolean;
    (*cinfo).Adobe_transform = 0 as UINT8;
    (*(*cinfo).marker).saw_SOI = TRUE as boolean;
    return TRUE;
}
unsafe extern "C" fn get_sof(
    mut cinfo: j_decompress_ptr,
    mut is_prog: boolean,
    mut is_arith: boolean,
) -> boolean {
    let mut length: JLONG = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    let mut next_input_byte: *const JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: size_t = (*datasrc).bytes_in_buffer;
    (*cinfo).progressive_mode = is_prog;
    (*cinfo).arith_code = is_arith;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh39 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh39 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int) as JLONG;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh40 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh40 as JLONG;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh41 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).data_precision = *fresh41 as ::core::ffi::c_int;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh42 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_height =
        ((*fresh42 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int) as JDIMENSION;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh43 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_height = (*cinfo).image_height.wrapping_add(*fresh43 as JDIMENSION);
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh44 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_width =
        ((*fresh44 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int) as JDIMENSION;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh45 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_width = (*cinfo).image_width.wrapping_add(*fresh45 as JDIMENSION);
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh46 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).num_components = *fresh46 as ::core::ffi::c_int;
    length -= 8 as JLONG;
    let mut _mp: *mut ::core::ffi::c_int =
        ::core::ptr::addr_of_mut!((*(*cinfo).err).msg_parm.i) as *mut ::core::ffi::c_int;
    *_mp.offset(0 as ::core::ffi::c_int as isize) = (*cinfo).unread_marker;
    *_mp.offset(1 as ::core::ffi::c_int as isize) = (*cinfo).image_width as ::core::ffi::c_int;
    *_mp.offset(2 as ::core::ffi::c_int as isize) = (*cinfo).image_height as ::core::ffi::c_int;
    *_mp.offset(3 as ::core::ffi::c_int as isize) = (*cinfo).num_components;
    (*(*cinfo).err).msg_code = JTRC_SOF as ::core::ffi::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    if (*(*cinfo).marker).saw_SOF != 0 {
        (*(*cinfo).err).msg_code = JERR_SOF_DUPLICATE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).image_height <= 0 as JDIMENSION
        || (*cinfo).image_width <= 0 as JDIMENSION
        || (*cinfo).num_components <= 0 as ::core::ffi::c_int
    {
        (*(*cinfo).err).msg_code = JERR_EMPTY_IMAGE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if length != ((*cinfo).num_components * 3 as ::core::ffi::c_int) as JLONG {
        (*(*cinfo).err).msg_code = JERR_BAD_LENGTH as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).comp_info.is_null() {
        (*cinfo).comp_info = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ((*cinfo).num_components as size_t)
                .wrapping_mul(::core::mem::size_of::<jpeg_component_info>() as size_t),
        ) as *mut jpeg_component_info;
    }
    ci = 0 as ::core::ffi::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        (*compptr).component_index = ci;
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh47 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        (*compptr).component_id = *fresh47 as ::core::ffi::c_int;
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh48 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        c = *fresh48 as ::core::ffi::c_int;
        (*compptr).h_samp_factor = c >> 4 as ::core::ffi::c_int & 15 as ::core::ffi::c_int;
        (*compptr).v_samp_factor = c & 15 as ::core::ffi::c_int;
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh49 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        (*compptr).quant_tbl_no = *fresh49 as ::core::ffi::c_int;
        let mut _mp_0: *mut ::core::ffi::c_int =
            ::core::ptr::addr_of_mut!((*(*cinfo).err).msg_parm.i) as *mut ::core::ffi::c_int;
        *_mp_0.offset(0 as ::core::ffi::c_int as isize) = (*compptr).component_id;
        *_mp_0.offset(1 as ::core::ffi::c_int as isize) = (*compptr).h_samp_factor;
        *_mp_0.offset(2 as ::core::ffi::c_int as isize) = (*compptr).v_samp_factor;
        *_mp_0.offset(3 as ::core::ffi::c_int as isize) = (*compptr).quant_tbl_no;
        (*(*cinfo).err).msg_code = JTRC_SOF_COMPONENT as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
        ci += 1;
        compptr = compptr.offset(1);
    }
    (*(*cinfo).marker).saw_SOF = TRUE as boolean;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return TRUE;
}
unsafe extern "C" fn get_sos(mut cinfo: j_decompress_ptr) -> boolean {
    let mut length: JLONG = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut cc: ::core::ffi::c_int = 0;
    let mut pi: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    let mut next_input_byte: *const JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: size_t = (*datasrc).bytes_in_buffer;
    if (*(*cinfo).marker).saw_SOF == 0 {
        (*(*cinfo).err).msg_code = JERR_SOS_NO_SOF as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh31 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh31 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int) as JLONG;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh32 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh32 as JLONG;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh33 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    n = *fresh33 as ::core::ffi::c_int;
    (*(*cinfo).err).msg_code = JTRC_SOS as ::core::ffi::c_int;
    (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = n;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    if length != (n * 2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as JLONG
        || n < 1 as ::core::ffi::c_int
        || n > MAX_COMPS_IN_SCAN
    {
        (*(*cinfo).err).msg_code = JERR_BAD_LENGTH as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*cinfo).comps_in_scan = n;
    i = 0 as ::core::ffi::c_int;
    while i < MAX_COMPS_IN_SCAN {
        (*cinfo).cur_comp_info[i as usize] = ::core::ptr::null_mut::<jpeg_component_info>();
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let mut current_block_80: u64;
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh34 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        cc = *fresh34 as ::core::ffi::c_int;
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh35 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        c = *fresh35 as ::core::ffi::c_int;
        ci = 0 as ::core::ffi::c_int;
        compptr = (*cinfo).comp_info;
        loop {
            if !(ci < (*cinfo).num_components && ci < MAX_COMPS_IN_SCAN) {
                current_block_80 = 1724319918354933278;
                break;
            }
            if cc == (*compptr).component_id && (*cinfo).cur_comp_info[ci as usize].is_null() {
                current_block_80 = 14577945223904882978;
                break;
            }
            ci += 1;
            compptr = compptr.offset(1);
        }
        match current_block_80 {
            1724319918354933278 => {
                (*(*cinfo).err).msg_code = JERR_BAD_COMPONENT_ID as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = cc;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            _ => {}
        }
        (*cinfo).cur_comp_info[i as usize] = compptr;
        (*compptr).dc_tbl_no = c >> 4 as ::core::ffi::c_int & 15 as ::core::ffi::c_int;
        (*compptr).ac_tbl_no = c & 15 as ::core::ffi::c_int;
        let mut _mp: *mut ::core::ffi::c_int =
            ::core::ptr::addr_of_mut!((*(*cinfo).err).msg_parm.i) as *mut ::core::ffi::c_int;
        *_mp.offset(0 as ::core::ffi::c_int as isize) = cc;
        *_mp.offset(1 as ::core::ffi::c_int as isize) = (*compptr).dc_tbl_no;
        *_mp.offset(2 as ::core::ffi::c_int as isize) = (*compptr).ac_tbl_no;
        (*(*cinfo).err).msg_code = JTRC_SOS_COMPONENT as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
        pi = 0 as ::core::ffi::c_int;
        while pi < i {
            if (*cinfo).cur_comp_info[pi as usize] == compptr {
                (*(*cinfo).err).msg_code = JERR_BAD_COMPONENT_ID as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = cc;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            pi += 1;
        }
        i += 1;
    }
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh36 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh36 as ::core::ffi::c_int;
    (*cinfo).Ss = c;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh37 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh37 as ::core::ffi::c_int;
    (*cinfo).Se = c;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh38 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh38 as ::core::ffi::c_int;
    (*cinfo).Ah = c >> 4 as ::core::ffi::c_int & 15 as ::core::ffi::c_int;
    (*cinfo).Al = c & 15 as ::core::ffi::c_int;
    let mut _mp_0: *mut ::core::ffi::c_int =
        ::core::ptr::addr_of_mut!((*(*cinfo).err).msg_parm.i) as *mut ::core::ffi::c_int;
    *_mp_0.offset(0 as ::core::ffi::c_int as isize) = (*cinfo).Ss;
    *_mp_0.offset(1 as ::core::ffi::c_int as isize) = (*cinfo).Se;
    *_mp_0.offset(2 as ::core::ffi::c_int as isize) = (*cinfo).Ah;
    *_mp_0.offset(3 as ::core::ffi::c_int as isize) = (*cinfo).Al;
    (*(*cinfo).err).msg_code = JTRC_SOS_PARAMS as ::core::ffi::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    (*(*cinfo).marker).next_restart_num = 0 as ::core::ffi::c_int;
    (*cinfo).input_scan_number += 1;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return TRUE;
}
unsafe extern "C" fn get_dac(mut cinfo: j_decompress_ptr) -> boolean {
    let mut length: JLONG = 0;
    let mut index: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_int = 0;
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    let mut next_input_byte: *const JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh27 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh27 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int) as JLONG;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh28 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh28 as JLONG;
    length -= 2 as JLONG;
    while length > 0 as JLONG {
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh29 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        index = *fresh29 as ::core::ffi::c_int;
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh30 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        val = *fresh30 as ::core::ffi::c_int;
        length -= 2 as JLONG;
        (*(*cinfo).err).msg_code = JTRC_DAC as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = index;
        (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = val;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
        if index < 0 as ::core::ffi::c_int || index >= 2 as ::core::ffi::c_int * NUM_ARITH_TBLS {
            (*(*cinfo).err).msg_code = JERR_DAC_INDEX as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = index;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if index >= NUM_ARITH_TBLS {
            (*cinfo).arith_ac_K[(index - NUM_ARITH_TBLS) as usize] = val as UINT8;
        } else {
            (*cinfo).arith_dc_L[index as usize] = (val & 0xf as ::core::ffi::c_int) as UINT8;
            (*cinfo).arith_dc_U[index as usize] = (val >> 4 as ::core::ffi::c_int) as UINT8;
            if (*cinfo).arith_dc_L[index as usize] as ::core::ffi::c_int
                > (*cinfo).arith_dc_U[index as usize] as ::core::ffi::c_int
            {
                (*(*cinfo).err).msg_code = JERR_DAC_VALUE as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = val;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    if length != 0 as JLONG {
        (*(*cinfo).err).msg_code = JERR_BAD_LENGTH as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return TRUE;
}
unsafe extern "C" fn get_dht(mut cinfo: j_decompress_ptr) -> boolean {
    let mut length: JLONG = 0;
    let mut bits: [UINT8; 17] = [0; 17];
    let mut huffval: [UINT8; 256] = [0; 256];
    let mut i: ::core::ffi::c_int = 0;
    let mut index: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut htblptr: *mut *mut JHUFF_TBL = ::core::ptr::null_mut::<*mut JHUFF_TBL>();
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    let mut next_input_byte: *const JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh22 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh22 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int) as JLONG;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh23 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh23 as JLONG;
    length -= 2 as JLONG;
    while length > 16 as JLONG {
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh24 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        index = *fresh24 as ::core::ffi::c_int;
        (*(*cinfo).err).msg_code = JTRC_DHT as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = index;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
        bits[0 as ::core::ffi::c_int as usize] = 0 as UINT8;
        count = 0 as ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        while i <= 16 as ::core::ffi::c_int {
            if bytes_in_buffer == 0 as size_t {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0 as boolean;
                }
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer;
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh25 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            bits[i as usize] = *fresh25 as UINT8;
            count += bits[i as usize] as ::core::ffi::c_int;
            i += 1;
        }
        length -= (1 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as JLONG;
        let mut _mp: *mut ::core::ffi::c_int =
            ::core::ptr::addr_of_mut!((*(*cinfo).err).msg_parm.i) as *mut ::core::ffi::c_int;
        *_mp.offset(0 as ::core::ffi::c_int as isize) =
            bits[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp.offset(1 as ::core::ffi::c_int as isize) =
            bits[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp.offset(2 as ::core::ffi::c_int as isize) =
            bits[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp.offset(3 as ::core::ffi::c_int as isize) =
            bits[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp.offset(4 as ::core::ffi::c_int as isize) =
            bits[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp.offset(5 as ::core::ffi::c_int as isize) =
            bits[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp.offset(6 as ::core::ffi::c_int as isize) =
            bits[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp.offset(7 as ::core::ffi::c_int as isize) =
            bits[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        (*(*cinfo).err).msg_code = JTRC_HUFFBITS as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 2 as ::core::ffi::c_int);
        let mut _mp_0: *mut ::core::ffi::c_int =
            ::core::ptr::addr_of_mut!((*(*cinfo).err).msg_parm.i) as *mut ::core::ffi::c_int;
        *_mp_0.offset(0 as ::core::ffi::c_int as isize) =
            bits[9 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp_0.offset(1 as ::core::ffi::c_int as isize) =
            bits[10 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp_0.offset(2 as ::core::ffi::c_int as isize) =
            bits[11 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp_0.offset(3 as ::core::ffi::c_int as isize) =
            bits[12 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp_0.offset(4 as ::core::ffi::c_int as isize) =
            bits[13 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp_0.offset(5 as ::core::ffi::c_int as isize) =
            bits[14 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp_0.offset(6 as ::core::ffi::c_int as isize) =
            bits[15 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp_0.offset(7 as ::core::ffi::c_int as isize) =
            bits[16 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        (*(*cinfo).err).msg_code = JTRC_HUFFBITS as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 2 as ::core::ffi::c_int);
        if count > 256 as ::core::ffi::c_int || count as JLONG > length {
            (*(*cinfo).err).msg_code = JERR_BAD_HUFF_TABLE as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        i = 0 as ::core::ffi::c_int;
        while i < count {
            if bytes_in_buffer == 0 as size_t {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0 as boolean;
                }
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer;
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh26 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            huffval[i as usize] = *fresh26 as UINT8;
            i += 1;
        }
        memset(
            (::core::ptr::addr_of_mut!(huffval) as *mut UINT8).offset(count as isize) as *mut UINT8
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((256 as ::core::ffi::c_int - count) as size_t)
                .wrapping_mul(::core::mem::size_of::<UINT8>() as size_t),
        );
        length -= count as JLONG;
        if index & 0x10 as ::core::ffi::c_int != 0 {
            index -= 0x10 as ::core::ffi::c_int;
            if index < 0 as ::core::ffi::c_int || index >= NUM_HUFF_TBLS {
                (*(*cinfo).err).msg_code = JERR_DHT_INDEX as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = index;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            htblptr = (::core::ptr::addr_of_mut!((*cinfo).ac_huff_tbl_ptrs) as *mut *mut JHUFF_TBL)
                .offset(index as isize) as *mut *mut JHUFF_TBL;
        } else {
            if index < 0 as ::core::ffi::c_int || index >= NUM_HUFF_TBLS {
                (*(*cinfo).err).msg_code = JERR_DHT_INDEX as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = index;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            htblptr = (::core::ptr::addr_of_mut!((*cinfo).dc_huff_tbl_ptrs) as *mut *mut JHUFF_TBL)
                .offset(index as isize) as *mut *mut JHUFF_TBL;
        }
        if (*htblptr).is_null() {
            *htblptr = jpeg_alloc_huff_table(cinfo as j_common_ptr);
        }
        memcpy(
            ::core::ptr::addr_of_mut!((**htblptr).bits) as *mut UINT8 as *mut ::core::ffi::c_void,
            ::core::ptr::addr_of_mut!(bits) as *mut UINT8 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[UINT8; 17]>() as size_t,
        );
        memcpy(
            ::core::ptr::addr_of_mut!((**htblptr).huffval) as *mut UINT8 as *mut ::core::ffi::c_void,
            ::core::ptr::addr_of_mut!(huffval) as *mut UINT8 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[UINT8; 256]>() as size_t,
        );
    }
    if length != 0 as JLONG {
        (*(*cinfo).err).msg_code = JERR_BAD_LENGTH as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return TRUE;
}
unsafe extern "C" fn get_dqt(mut cinfo: j_decompress_ptr) -> boolean {
    let mut length: JLONG = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut prec: ::core::ffi::c_int = 0;
    let mut tmp: ::core::ffi::c_uint = 0;
    let mut quant_ptr: *mut JQUANT_TBL = ::core::ptr::null_mut::<JQUANT_TBL>();
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    let mut next_input_byte: *const JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh16 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh16 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int) as JLONG;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh17 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh17 as JLONG;
    length -= 2 as JLONG;
    while length > 0 as JLONG {
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh18 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        n = *fresh18 as ::core::ffi::c_int;
        prec = n >> 4 as ::core::ffi::c_int;
        n &= 0xf as ::core::ffi::c_int;
        (*(*cinfo).err).msg_code = JTRC_DQT as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = n;
        (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = prec;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
        if n >= NUM_QUANT_TBLS {
            (*(*cinfo).err).msg_code = JERR_DQT_INDEX as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = n;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if (*cinfo).quant_tbl_ptrs[n as usize].is_null() {
            (*cinfo).quant_tbl_ptrs[n as usize] = jpeg_alloc_quant_table(cinfo as j_common_ptr);
        }
        quant_ptr = (*cinfo).quant_tbl_ptrs[n as usize];
        i = 0 as ::core::ffi::c_int;
        while i < DCTSIZE2 {
            if prec != 0 {
                if bytes_in_buffer == 0 as size_t {
                    if Some(
                        (*datasrc)
                            .fill_input_buffer
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo)
                        == 0
                    {
                        return 0 as boolean;
                    }
                    next_input_byte = (*datasrc).next_input_byte;
                    bytes_in_buffer = (*datasrc).bytes_in_buffer;
                }
                bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                let fresh19 = next_input_byte;
                next_input_byte = next_input_byte.offset(1);
                tmp = (*fresh19 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int;
                if bytes_in_buffer == 0 as size_t {
                    if Some(
                        (*datasrc)
                            .fill_input_buffer
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo)
                        == 0
                    {
                        return 0 as boolean;
                    }
                    next_input_byte = (*datasrc).next_input_byte;
                    bytes_in_buffer = (*datasrc).bytes_in_buffer;
                }
                bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                let fresh20 = next_input_byte;
                next_input_byte = next_input_byte.offset(1);
                tmp = tmp.wrapping_add(*fresh20 as ::core::ffi::c_uint);
            } else {
                if bytes_in_buffer == 0 as size_t {
                    if Some(
                        (*datasrc)
                            .fill_input_buffer
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo)
                        == 0
                    {
                        return 0 as boolean;
                    }
                    next_input_byte = (*datasrc).next_input_byte;
                    bytes_in_buffer = (*datasrc).bytes_in_buffer;
                }
                bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                let fresh21 = next_input_byte;
                next_input_byte = next_input_byte.offset(1);
                tmp = *fresh21 as ::core::ffi::c_uint;
            }
            (*quant_ptr).quantval[*(::core::ptr::addr_of!(jpeg_natural_order) as *const ::core::ffi::c_int)
                .offset(i as isize) as usize] = tmp as UINT16;
            i += 1;
        }
        if (*(*cinfo).err).trace_level >= 2 as ::core::ffi::c_int {
            i = 0 as ::core::ffi::c_int;
            while i < DCTSIZE2 {
                let mut _mp: *mut ::core::ffi::c_int =
                    ::core::ptr::addr_of_mut!((*(*cinfo).err).msg_parm.i) as *mut ::core::ffi::c_int;
                *_mp.offset(0 as ::core::ffi::c_int as isize) =
                    (*quant_ptr).quantval[i as usize] as ::core::ffi::c_int;
                *_mp.offset(1 as ::core::ffi::c_int as isize) = (*quant_ptr).quantval
                    [(i + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                *_mp.offset(2 as ::core::ffi::c_int as isize) = (*quant_ptr).quantval
                    [(i + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                *_mp.offset(3 as ::core::ffi::c_int as isize) = (*quant_ptr).quantval
                    [(i + 3 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                *_mp.offset(4 as ::core::ffi::c_int as isize) = (*quant_ptr).quantval
                    [(i + 4 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                *_mp.offset(5 as ::core::ffi::c_int as isize) = (*quant_ptr).quantval
                    [(i + 5 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                *_mp.offset(6 as ::core::ffi::c_int as isize) = (*quant_ptr).quantval
                    [(i + 6 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                *_mp.offset(7 as ::core::ffi::c_int as isize) = (*quant_ptr).quantval
                    [(i + 7 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                (*(*cinfo).err).msg_code = JTRC_QUANTVALS as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    2 as ::core::ffi::c_int,
                );
                i += 8 as ::core::ffi::c_int;
            }
        }
        length -= (DCTSIZE2 + 1 as ::core::ffi::c_int) as JLONG;
        if prec != 0 {
            length -= DCTSIZE2 as JLONG;
        }
    }
    if length != 0 as JLONG {
        (*(*cinfo).err).msg_code = JERR_BAD_LENGTH as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return TRUE;
}
unsafe extern "C" fn get_dri(mut cinfo: j_decompress_ptr) -> boolean {
    let mut length: JLONG = 0;
    let mut tmp: ::core::ffi::c_uint = 0;
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    let mut next_input_byte: *const JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh12 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh12 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int) as JLONG;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh13 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh13 as JLONG;
    if length != 4 as JLONG {
        (*(*cinfo).err).msg_code = JERR_BAD_LENGTH as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh14 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    tmp = (*fresh14 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh15 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    tmp = tmp.wrapping_add(*fresh15 as ::core::ffi::c_uint);
    (*(*cinfo).err).msg_code = JTRC_DRI as ::core::ffi::c_int;
    (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = tmp as ::core::ffi::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    (*cinfo).restart_interval = tmp;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return TRUE;
}
pub const APP0_DATA_LEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const APP14_DATA_LEN: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const APPN_DATA_LEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
unsafe extern "C" fn examine_app0(
    mut cinfo: j_decompress_ptr,
    mut data: *mut JOCTET,
    mut datalen: ::core::ffi::c_uint,
    mut remaining: JLONG,
) {
    let mut totallen: JLONG = datalen as JLONG + remaining;
    if datalen >= APP0_DATA_LEN as ::core::ffi::c_uint
        && *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x4a as ::core::ffi::c_int
        && *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x46 as ::core::ffi::c_int
        && *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x49 as ::core::ffi::c_int
        && *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x46 as ::core::ffi::c_int
        && *data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        (*cinfo).saw_JFIF_marker = TRUE as boolean;
        (*cinfo).JFIF_major_version = *data.offset(5 as ::core::ffi::c_int as isize) as UINT8;
        (*cinfo).JFIF_minor_version = *data.offset(6 as ::core::ffi::c_int as isize) as UINT8;
        (*cinfo).density_unit = *data.offset(7 as ::core::ffi::c_int as isize) as UINT8;
        (*cinfo).X_density = (((*data.offset(8 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
            + *data.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as UINT16;
        (*cinfo).Y_density = (((*data.offset(10 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
            + *data.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as UINT16;
        if (*cinfo).JFIF_major_version as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
            (*(*cinfo).err).msg_code = JWRN_JFIF_MAJOR as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                (*cinfo).JFIF_major_version as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] =
                (*cinfo).JFIF_minor_version as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, -(1 as ::core::ffi::c_int)
            );
        }
        let mut _mp: *mut ::core::ffi::c_int =
            ::core::ptr::addr_of_mut!((*(*cinfo).err).msg_parm.i) as *mut ::core::ffi::c_int;
        *_mp.offset(0 as ::core::ffi::c_int as isize) =
            (*cinfo).JFIF_major_version as ::core::ffi::c_int;
        *_mp.offset(1 as ::core::ffi::c_int as isize) =
            (*cinfo).JFIF_minor_version as ::core::ffi::c_int;
        *_mp.offset(2 as ::core::ffi::c_int as isize) = (*cinfo).X_density as ::core::ffi::c_int;
        *_mp.offset(3 as ::core::ffi::c_int as isize) = (*cinfo).Y_density as ::core::ffi::c_int;
        *_mp.offset(4 as ::core::ffi::c_int as isize) = (*cinfo).density_unit as ::core::ffi::c_int;
        (*(*cinfo).err).msg_code = JTRC_JFIF as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
        if *data.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            | *data.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0
        {
            (*(*cinfo).err).msg_code = JTRC_JFIF_THUMBNAIL as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                *data.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] =
                *data.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1 as ::core::ffi::c_int
            );
        }
        totallen -= APP0_DATA_LEN as JLONG;
        if totallen
            != *data.offset(12 as ::core::ffi::c_int as isize) as JLONG
                * *data.offset(13 as ::core::ffi::c_int as isize) as JLONG
                * 3 as ::core::ffi::c_int as JLONG
        {
            (*(*cinfo).err).msg_code = JTRC_JFIF_BADTHUMBNAILSIZE as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                totallen as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1 as ::core::ffi::c_int
            );
        }
    } else if datalen >= 6 as ::core::ffi::c_uint
        && *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x4a as ::core::ffi::c_int
        && *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x46 as ::core::ffi::c_int
        && *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x58 as ::core::ffi::c_int
        && *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x58 as ::core::ffi::c_int
        && *data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        match *data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
            16 => {
                (*(*cinfo).err).msg_code = JTRC_THUMB_JPEG as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                    totallen as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    1 as ::core::ffi::c_int,
                );
            }
            17 => {
                (*(*cinfo).err).msg_code = JTRC_THUMB_PALETTE as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                    totallen as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    1 as ::core::ffi::c_int,
                );
            }
            19 => {
                (*(*cinfo).err).msg_code = JTRC_THUMB_RGB as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                    totallen as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    1 as ::core::ffi::c_int,
                );
            }
            _ => {
                (*(*cinfo).err).msg_code = JTRC_JFIF_EXTENSION as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                    *data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] =
                    totallen as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    1 as ::core::ffi::c_int,
                );
            }
        }
    } else {
        (*(*cinfo).err).msg_code = JTRC_APP0 as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
            totallen as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn examine_app14(
    mut cinfo: j_decompress_ptr,
    mut data: *mut JOCTET,
    mut datalen: ::core::ffi::c_uint,
    mut remaining: JLONG,
) {
    let mut version: ::core::ffi::c_uint = 0;
    let mut flags0: ::core::ffi::c_uint = 0;
    let mut flags1: ::core::ffi::c_uint = 0;
    let mut transform: ::core::ffi::c_uint = 0;
    if datalen >= APP14_DATA_LEN as ::core::ffi::c_uint
        && *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x41 as ::core::ffi::c_int
        && *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x64 as ::core::ffi::c_int
        && *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x6f as ::core::ffi::c_int
        && *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x62 as ::core::ffi::c_int
        && *data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x65 as ::core::ffi::c_int
    {
        version = (((*data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
            + *data.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        flags0 = (((*data.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
            + *data.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        flags1 = (((*data.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
            + *data.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        transform = *data.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        let mut _mp: *mut ::core::ffi::c_int =
            ::core::ptr::addr_of_mut!((*(*cinfo).err).msg_parm.i) as *mut ::core::ffi::c_int;
        *_mp.offset(0 as ::core::ffi::c_int as isize) = version as ::core::ffi::c_int;
        *_mp.offset(1 as ::core::ffi::c_int as isize) = flags0 as ::core::ffi::c_int;
        *_mp.offset(2 as ::core::ffi::c_int as isize) = flags1 as ::core::ffi::c_int;
        *_mp.offset(3 as ::core::ffi::c_int as isize) = transform as ::core::ffi::c_int;
        (*(*cinfo).err).msg_code = JTRC_ADOBE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
        (*cinfo).saw_Adobe_marker = TRUE as boolean;
        (*cinfo).Adobe_transform = transform as UINT8;
    } else {
        (*(*cinfo).err).msg_code = JTRC_APP14 as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
            (datalen as JLONG + remaining) as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn get_interesting_appn(mut cinfo: j_decompress_ptr) -> boolean {
    let mut length: JLONG = 0;
    let mut b: [JOCTET; 14] = [0; 14];
    let mut i: ::core::ffi::c_uint = 0;
    let mut numtoread: ::core::ffi::c_uint = 0;
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    let mut next_input_byte: *const JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh0 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh0 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int) as JLONG;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh1 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh1 as JLONG;
    length -= 2 as JLONG;
    if length >= APPN_DATA_LEN as JLONG {
        numtoread = APPN_DATA_LEN as ::core::ffi::c_uint;
    } else if length > 0 as JLONG {
        numtoread = length as ::core::ffi::c_uint;
    } else {
        numtoread = 0 as ::core::ffi::c_uint;
    }
    i = 0 as ::core::ffi::c_uint;
    while i < numtoread {
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh2 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        b[i as usize] = *fresh2;
        i = i.wrapping_add(1);
    }
    length -= numtoread as JLONG;
    match (*cinfo).unread_marker {
        224 => {
            examine_app0(cinfo, ::core::ptr::addr_of_mut!(b) as *mut JOCTET, numtoread, length);
        }
        238 => {
            examine_app14(cinfo, ::core::ptr::addr_of_mut!(b) as *mut JOCTET, numtoread, length);
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_UNKNOWN_MARKER as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = (*cinfo).unread_marker;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    if length > 0 as JLONG {
        Some(
            (*(*cinfo).src)
                .skip_input_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, length);
    }
    return TRUE;
}
unsafe extern "C" fn save_marker(mut cinfo: j_decompress_ptr) -> boolean {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    let mut cur_marker: jpeg_saved_marker_ptr = (*marker).cur_marker;
    let mut bytes_read: ::core::ffi::c_uint = 0;
    let mut data_length: ::core::ffi::c_uint = 0;
    let mut data: *mut JOCTET = ::core::ptr::null_mut::<JOCTET>();
    let mut length: JLONG = 0 as JLONG;
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    let mut next_input_byte: *const JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: size_t = (*datasrc).bytes_in_buffer;
    if cur_marker.is_null() {
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh5 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        length = ((*fresh5 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int) as JLONG;
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh6 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        length += *fresh6 as JLONG;
        length -= 2 as JLONG;
        if length >= 0 as JLONG {
            let mut limit: ::core::ffi::c_uint = 0;
            if (*cinfo).unread_marker == M_COM as ::core::ffi::c_int {
                limit = (*marker).length_limit_COM;
            } else {
                limit = (*marker).length_limit_APPn
                    [((*cinfo).unread_marker - M_APP0 as ::core::ffi::c_int) as usize];
            }
            if (length as ::core::ffi::c_uint) < limit {
                limit = length as ::core::ffi::c_uint;
            }
            cur_marker = Some(
                (*(*cinfo).mem)
                    .alloc_large
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                (::core::mem::size_of::<jpeg_marker_struct>() as size_t)
                    .wrapping_add(limit as size_t),
            ) as jpeg_saved_marker_ptr;
            (*cur_marker).next = ::core::ptr::null_mut::<jpeg_marker_struct>();
            (*cur_marker).marker = (*cinfo).unread_marker as UINT8;
            (*cur_marker).original_length = length as ::core::ffi::c_uint;
            (*cur_marker).data_length = limit;
            (*cur_marker).data = cur_marker.offset(1 as ::core::ffi::c_int as isize) as *mut JOCTET;
            data = (*cur_marker).data;
            (*marker).cur_marker = cur_marker;
            (*marker).bytes_read = 0 as ::core::ffi::c_uint;
            bytes_read = 0 as ::core::ffi::c_uint;
            data_length = limit;
        } else {
            data_length = 0 as ::core::ffi::c_uint;
            bytes_read = data_length;
            data = ::core::ptr::null_mut::<JOCTET>();
        }
    } else {
        bytes_read = (*marker).bytes_read;
        data_length = (*cur_marker).data_length;
        data = (*cur_marker).data.offset(bytes_read as isize);
    }
    while bytes_read < data_length {
        (*datasrc).next_input_byte = next_input_byte;
        (*datasrc).bytes_in_buffer = bytes_in_buffer;
        (*marker).bytes_read = bytes_read;
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        while bytes_read < data_length && bytes_in_buffer > 0 as size_t {
            let fresh7 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            let fresh8 = data;
            data = data.offset(1);
            *fresh8 = *fresh7;
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            bytes_read = bytes_read.wrapping_add(1);
        }
    }
    if !cur_marker.is_null() {
        if (*cinfo).marker_list.is_null() {
            (*cinfo).marker_list = cur_marker;
        } else {
            let mut prev: jpeg_saved_marker_ptr = (*cinfo).marker_list;
            while !(*prev).next.is_null() {
                prev = (*prev).next;
            }
            (*prev).next = cur_marker;
        }
        data = (*cur_marker).data;
        length = (*cur_marker).original_length.wrapping_sub(data_length) as JLONG;
    }
    (*marker).cur_marker = ::core::ptr::null_mut::<jpeg_marker_struct>();
    match (*cinfo).unread_marker {
        224 => {
            examine_app0(cinfo, data, data_length, length);
        }
        238 => {
            examine_app14(cinfo, data, data_length, length);
        }
        _ => {
            (*(*cinfo).err).msg_code = JTRC_MISC_MARKER as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = (*cinfo).unread_marker;
            (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] =
                (data_length as JLONG + length) as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1 as ::core::ffi::c_int
            );
        }
    }
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    if length > 0 as JLONG {
        Some(
            (*(*cinfo).src)
                .skip_input_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, length);
    }
    return TRUE;
}
unsafe extern "C" fn skip_variable(mut cinfo: j_decompress_ptr) -> boolean {
    let mut length: JLONG = 0;
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    let mut next_input_byte: *const JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh3 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh3 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int) as JLONG;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh4 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh4 as JLONG;
    length -= 2 as JLONG;
    (*(*cinfo).err).msg_code = JTRC_MISC_MARKER as ::core::ffi::c_int;
    (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = (*cinfo).unread_marker;
    (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = length as ::core::ffi::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    if length > 0 as JLONG {
        Some(
            (*(*cinfo).src)
                .skip_input_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, length);
    }
    return TRUE;
}
unsafe extern "C" fn next_marker(mut cinfo: j_decompress_ptr) -> boolean {
    let mut c: ::core::ffi::c_int = 0;
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    let mut next_input_byte: *const JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: size_t = (*datasrc).bytes_in_buffer;
    loop {
        if bytes_in_buffer == 0 as size_t {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as boolean;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer;
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh9 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        c = *fresh9 as ::core::ffi::c_int;
        while c != 0xff as ::core::ffi::c_int {
            (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker).discarded_bytes.wrapping_add(1);
            (*datasrc).next_input_byte = next_input_byte;
            (*datasrc).bytes_in_buffer = bytes_in_buffer;
            if bytes_in_buffer == 0 as size_t {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0 as boolean;
                }
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer;
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh10 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            c = *fresh10 as ::core::ffi::c_int;
        }
        loop {
            if bytes_in_buffer == 0 as size_t {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0 as boolean;
                }
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer;
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh11 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            c = *fresh11 as ::core::ffi::c_int;
            if !(c == 0xff as ::core::ffi::c_int) {
                break;
            }
        }
        if c != 0 as ::core::ffi::c_int {
            break;
        }
        (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker)
            .discarded_bytes
            .wrapping_add(2 as ::core::ffi::c_uint);
        (*datasrc).next_input_byte = next_input_byte;
        (*datasrc).bytes_in_buffer = bytes_in_buffer;
    }
    if (*(*cinfo).marker).discarded_bytes != 0 as ::core::ffi::c_uint {
        (*(*cinfo).err).msg_code = JWRN_EXTRANEOUS_DATA as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
            (*(*cinfo).marker).discarded_bytes as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = c;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr, -(1 as ::core::ffi::c_int)
        );
        (*(*cinfo).marker).discarded_bytes = 0 as ::core::ffi::c_uint;
    }
    (*cinfo).unread_marker = c;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return TRUE;
}
unsafe extern "C" fn first_marker(mut cinfo: j_decompress_ptr) -> boolean {
    let mut c: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    let mut next_input_byte: *const JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh50 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh50 as ::core::ffi::c_int;
    if bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as boolean;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer;
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh51 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c2 = *fresh51 as ::core::ffi::c_int;
    if c != 0xff as ::core::ffi::c_int || c2 != M_SOI as ::core::ffi::c_int {
        (*(*cinfo).err).msg_code = JERR_NO_SOI as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = c;
        (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = c2;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*cinfo).unread_marker = c2;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return TRUE;
}
unsafe extern "C" fn read_markers(mut cinfo: j_decompress_ptr) -> ::core::ffi::c_int {
    loop {
        if (*cinfo).unread_marker == 0 as ::core::ffi::c_int {
            if (*(*cinfo).marker).saw_SOI == 0 {
                if first_marker(cinfo) == 0 {
                    return JPEG_SUSPENDED;
                }
            } else if next_marker(cinfo) == 0 {
                return JPEG_SUSPENDED;
            }
        }
        match (*cinfo).unread_marker {
            216 => {
                if get_soi(cinfo) == 0 {
                    return JPEG_SUSPENDED;
                }
            }
            192 | 193 => {
                if get_sof(cinfo, FALSE, FALSE) == 0 {
                    return JPEG_SUSPENDED;
                }
            }
            194 => {
                if get_sof(cinfo, TRUE, FALSE) == 0 {
                    return JPEG_SUSPENDED;
                }
            }
            201 => {
                if get_sof(cinfo, FALSE, TRUE) == 0 {
                    return JPEG_SUSPENDED;
                }
            }
            202 => {
                if get_sof(cinfo, TRUE, TRUE) == 0 {
                    return JPEG_SUSPENDED;
                }
            }
            195 | 197 | 198 | 199 | 200 | 203 | 205 | 206 | 207 => {
                (*(*cinfo).err).msg_code = JERR_SOF_UNSUPPORTED as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                    (*cinfo).unread_marker;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            218 => {
                if get_sos(cinfo) == 0 {
                    return JPEG_SUSPENDED;
                }
                (*cinfo).unread_marker = 0 as ::core::ffi::c_int;
                return JPEG_REACHED_SOS;
            }
            217 => {
                (*(*cinfo).err).msg_code = JTRC_EOI as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    1 as ::core::ffi::c_int,
                );
                (*cinfo).unread_marker = 0 as ::core::ffi::c_int;
                return JPEG_REACHED_EOI;
            }
            204 => {
                if get_dac(cinfo) == 0 {
                    return JPEG_SUSPENDED;
                }
            }
            196 => {
                if get_dht(cinfo) == 0 {
                    return JPEG_SUSPENDED;
                }
            }
            219 => {
                if get_dqt(cinfo) == 0 {
                    return JPEG_SUSPENDED;
                }
            }
            221 => {
                if get_dri(cinfo) == 0 {
                    return JPEG_SUSPENDED;
                }
            }
            224 | 225 | 226 | 227 | 228 | 229 | 230 | 231 | 232 | 233 | 234 | 235 | 236 | 237
            | 238 | 239 => {
                if Some(
                    (*(::core::ptr::addr_of_mut!((*((*cinfo).marker as my_marker_ptr)).process_APPn
                       ) as *mut jpeg_marker_parser_method)
                        .offset(((*cinfo).unread_marker - M_APP0 as ::core::ffi::c_int) as isize))
                    .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return JPEG_SUSPENDED;
                }
            }
            254 => {
                if Some(
                    (*((*cinfo).marker as my_marker_ptr))
                        .process_COM
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return JPEG_SUSPENDED;
                }
            }
            208 | 209 | 210 | 211 | 212 | 213 | 214 | 215 | 1 => {
                (*(*cinfo).err).msg_code = JTRC_PARMLESS_MARKER as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                    (*cinfo).unread_marker;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    1 as ::core::ffi::c_int,
                );
            }
            220 => {
                if skip_variable(cinfo) == 0 {
                    return JPEG_SUSPENDED;
                }
            }
            _ => {
                (*(*cinfo).err).msg_code = JERR_UNKNOWN_MARKER as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                    (*cinfo).unread_marker;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        (*cinfo).unread_marker = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn read_restart_marker(mut cinfo: j_decompress_ptr) -> boolean {
    if (*cinfo).unread_marker == 0 as ::core::ffi::c_int {
        if next_marker(cinfo) == 0 {
            return FALSE;
        }
    }
    if (*cinfo).unread_marker == M_RST0 as ::core::ffi::c_int + (*(*cinfo).marker).next_restart_num
    {
        (*(*cinfo).err).msg_code = JTRC_RST as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
            (*(*cinfo).marker).next_restart_num;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 3 as ::core::ffi::c_int);
        (*cinfo).unread_marker = 0 as ::core::ffi::c_int;
    } else if Some(
        (*(*cinfo).src)
            .resync_to_restart
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, (*(*cinfo).marker).next_restart_num)
        == 0
    {
        return FALSE;
    }
    (*(*cinfo).marker).next_restart_num =
        (*(*cinfo).marker).next_restart_num + 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int;
    return TRUE;
}
pub unsafe extern "C" fn jpeg_resync_to_restart(
    mut cinfo: j_decompress_ptr,
    mut desired: ::core::ffi::c_int,
) -> boolean {
    let mut marker: ::core::ffi::c_int = (*cinfo).unread_marker;
    let mut action: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    (*(*cinfo).err).msg_code = JWRN_MUST_RESYNC as ::core::ffi::c_int;
    (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = marker;
    (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = desired;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr, -(1 as ::core::ffi::c_int));
    loop {
        if marker < M_SOF0 as ::core::ffi::c_int {
            action = 2 as ::core::ffi::c_int;
        } else if marker < M_RST0 as ::core::ffi::c_int || marker > M_RST7 as ::core::ffi::c_int {
            action = 3 as ::core::ffi::c_int;
        } else if marker
            == M_RST0 as ::core::ffi::c_int
                + (desired + 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            || marker
                == M_RST0 as ::core::ffi::c_int
                    + (desired + 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
        {
            action = 3 as ::core::ffi::c_int;
        } else if marker
            == M_RST0 as ::core::ffi::c_int
                + (desired - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            || marker
                == M_RST0 as ::core::ffi::c_int
                    + (desired - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
        {
            action = 2 as ::core::ffi::c_int;
        } else {
            action = 1 as ::core::ffi::c_int;
        }
        (*(*cinfo).err).msg_code = JTRC_RECOVERY_ACTION as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = marker;
        (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = action;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 4 as ::core::ffi::c_int);
        match action {
            1 => {
                (*cinfo).unread_marker = 0 as ::core::ffi::c_int;
                return TRUE;
            }
            2 => {
                if next_marker(cinfo) == 0 {
                    return FALSE;
                }
                marker = (*cinfo).unread_marker;
            }
            3 => return TRUE,
            _ => {}
        }
    }
}
unsafe extern "C" fn reset_marker_reader(mut cinfo: j_decompress_ptr) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    (*cinfo).comp_info = ::core::ptr::null_mut::<jpeg_component_info>();
    (*cinfo).input_scan_number = 0 as ::core::ffi::c_int;
    (*cinfo).unread_marker = 0 as ::core::ffi::c_int;
    (*marker).pub_0.saw_SOI = FALSE as boolean;
    (*marker).pub_0.saw_SOF = FALSE as boolean;
    (*marker).pub_0.discarded_bytes = 0 as ::core::ffi::c_uint;
    (*marker).cur_marker = ::core::ptr::null_mut::<jpeg_marker_struct>();
}
pub unsafe extern "C" fn jinit_marker_reader(mut cinfo: j_decompress_ptr) {
    let mut marker: my_marker_ptr = ::core::ptr::null_mut::<my_marker_reader>();
    let mut i: ::core::ffi::c_int = 0;
    marker = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_PERMANENT,
        ::core::mem::size_of::<my_marker_reader>() as size_t,
    ) as my_marker_ptr;
    (*cinfo).marker = marker as *mut jpeg_marker_reader as *mut jpeg_marker_reader;
    (*marker).pub_0.reset_marker_reader =
        Some(reset_marker_reader as unsafe extern "C" fn(j_decompress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
    (*marker).pub_0.read_markers =
        Some(read_markers as unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int)
            as Option<unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int>;
    (*marker).pub_0.read_restart_marker =
        Some(read_restart_marker as unsafe extern "C" fn(j_decompress_ptr) -> boolean)
            as jpeg_marker_parser_method;
    (*marker).process_COM = Some(skip_variable as unsafe extern "C" fn(j_decompress_ptr) -> boolean)
        as jpeg_marker_parser_method;
    (*marker).length_limit_COM = 0 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*marker).process_APPn[i as usize] =
            Some(skip_variable as unsafe extern "C" fn(j_decompress_ptr) -> boolean)
                as jpeg_marker_parser_method;
        (*marker).length_limit_APPn[i as usize] = 0 as ::core::ffi::c_uint;
        i += 1;
    }
    (*marker).process_APPn[0 as ::core::ffi::c_int as usize] =
        Some(get_interesting_appn as unsafe extern "C" fn(j_decompress_ptr) -> boolean)
            as jpeg_marker_parser_method;
    (*marker).process_APPn[14 as ::core::ffi::c_int as usize] =
        Some(get_interesting_appn as unsafe extern "C" fn(j_decompress_ptr) -> boolean)
            as jpeg_marker_parser_method;
    reset_marker_reader(cinfo);
}
pub unsafe extern "C" fn jpeg_save_markers(
    mut cinfo: j_decompress_ptr,
    mut marker_code: ::core::ffi::c_int,
    mut length_limit: ::core::ffi::c_uint,
) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    let mut maxlength: ::core::ffi::c_long = 0;
    let mut processor: jpeg_marker_parser_method = None;
    maxlength = ((*(*cinfo).mem).max_alloc_chunk as usize)
        .wrapping_sub(::core::mem::size_of::<jpeg_marker_struct>() as usize)
        as ::core::ffi::c_long;
    if length_limit as ::core::ffi::c_long > maxlength {
        length_limit = maxlength as ::core::ffi::c_uint;
    }
    if length_limit != 0 {
        processor = Some(save_marker as unsafe extern "C" fn(j_decompress_ptr) -> boolean)
            as jpeg_marker_parser_method;
        if marker_code == M_APP0 as ::core::ffi::c_int
            && length_limit < APP0_DATA_LEN as ::core::ffi::c_uint
        {
            length_limit = APP0_DATA_LEN as ::core::ffi::c_uint;
        } else if marker_code == M_APP14 as ::core::ffi::c_int
            && length_limit < APP14_DATA_LEN as ::core::ffi::c_uint
        {
            length_limit = APP14_DATA_LEN as ::core::ffi::c_uint;
        }
    } else {
        processor = Some(skip_variable as unsafe extern "C" fn(j_decompress_ptr) -> boolean)
            as jpeg_marker_parser_method;
        if marker_code == M_APP0 as ::core::ffi::c_int
            || marker_code == M_APP14 as ::core::ffi::c_int
        {
            processor =
                Some(get_interesting_appn as unsafe extern "C" fn(j_decompress_ptr) -> boolean)
                    as jpeg_marker_parser_method;
        }
    }
    if marker_code == M_COM as ::core::ffi::c_int {
        (*marker).process_COM = processor;
        (*marker).length_limit_COM = length_limit;
    } else if marker_code >= M_APP0 as ::core::ffi::c_int
        && marker_code <= M_APP15 as ::core::ffi::c_int
    {
        (*marker).process_APPn[(marker_code - M_APP0 as ::core::ffi::c_int) as usize] = processor;
        (*marker).length_limit_APPn[(marker_code - M_APP0 as ::core::ffi::c_int) as usize] =
            length_limit;
    } else {
        (*(*cinfo).err).msg_code = JERR_UNKNOWN_MARKER as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = marker_code;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    };
}
pub unsafe extern "C" fn jpeg_set_marker_processor(
    mut cinfo: j_decompress_ptr,
    mut marker_code: ::core::ffi::c_int,
    mut routine: jpeg_marker_parser_method,
) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    if marker_code == M_COM as ::core::ffi::c_int {
        (*marker).process_COM = routine;
    } else if marker_code >= M_APP0 as ::core::ffi::c_int
        && marker_code <= M_APP15 as ::core::ffi::c_int
    {
        (*marker).process_APPn[(marker_code - M_APP0 as ::core::ffi::c_int) as usize] = routine;
    } else {
        (*(*cinfo).err).msg_code = JERR_UNKNOWN_MARKER as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = marker_code;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    };
}
