#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}

extern "C" {
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
pub struct jpeg_scan_info {
    pub comps_in_scan: ::core::ffi::c_int,
    pub component_index: [::core::ffi::c_int; 4],
    pub Ss: ::core::ffi::c_int,
    pub Se: ::core::ffi::c_int,
    pub Ah: ::core::ffi::c_int,
    pub Al: ::core::ffi::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_compress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut ::core::ffi::c_void,
    pub is_decompressor: boolean,
    pub global_state: ::core::ffi::c_int,
    pub dest: *mut jpeg_destination_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub input_components: ::core::ffi::c_int,
    pub in_color_space: J_COLOR_SPACE,
    pub input_gamma: ::core::ffi::c_double,
    pub scale_num: ::core::ffi::c_uint,
    pub scale_denom: ::core::ffi::c_uint,
    pub jpeg_width: JDIMENSION,
    pub jpeg_height: JDIMENSION,
    pub data_precision: ::core::ffi::c_int,
    pub num_components: ::core::ffi::c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub comp_info: *mut jpeg_component_info,
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4],
    pub q_scale_factor: [::core::ffi::c_int; 4],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub arith_dc_L: [UINT8; 16],
    pub arith_dc_U: [UINT8; 16],
    pub arith_ac_K: [UINT8; 16],
    pub num_scans: ::core::ffi::c_int,
    pub scan_info: *const jpeg_scan_info,
    pub raw_data_in: boolean,
    pub arith_code: boolean,
    pub optimize_coding: boolean,
    pub CCIR601_sampling: boolean,
    pub do_fancy_downsampling: boolean,
    pub smoothing_factor: ::core::ffi::c_int,
    pub dct_method: J_DCT_METHOD,
    pub restart_interval: ::core::ffi::c_uint,
    pub restart_in_rows: ::core::ffi::c_int,
    pub write_JFIF_header: boolean,
    pub JFIF_major_version: UINT8,
    pub JFIF_minor_version: UINT8,
    pub density_unit: UINT8,
    pub X_density: UINT16,
    pub Y_density: UINT16,
    pub write_Adobe_marker: boolean,
    pub next_scanline: JDIMENSION,
    pub progressive_mode: boolean,
    pub max_h_samp_factor: ::core::ffi::c_int,
    pub max_v_samp_factor: ::core::ffi::c_int,
    pub min_DCT_h_scaled_size: ::core::ffi::c_int,
    pub min_DCT_v_scaled_size: ::core::ffi::c_int,
    pub total_iMCU_rows: JDIMENSION,
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
    pub master: *mut jpeg_comp_master,
    pub main: *mut jpeg_c_main_controller,
    pub prep: *mut jpeg_c_prep_controller,
    pub coef: *mut jpeg_c_coef_controller,
    pub marker: *mut jpeg_marker_writer,
    pub cconvert: *mut jpeg_color_converter,
    pub downsample: *mut jpeg_downsampler,
    pub fdct: *mut jpeg_forward_dct,
    pub entropy: *mut jpeg_entropy_encoder,
    pub script_space: *mut jpeg_scan_info,
    pub script_space_size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_entropy_encoder {
    pub start_pass: Option<unsafe extern "C" fn(j_compress_ptr, boolean) -> ()>,
    pub encode_mcu: Option<unsafe extern "C" fn(j_compress_ptr, *mut JBLOCKROW) -> boolean>,
    pub finish_pass: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
}
pub type j_compress_ptr = *mut jpeg_compress_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_forward_dct {
    pub start_pass: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub forward_DCT: Option<
        unsafe extern "C" fn(
            j_compress_ptr,
            *mut jpeg_component_info,
            JSAMPARRAY,
            JBLOCKROW,
            JDIMENSION,
            JDIMENSION,
            JDIMENSION,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_downsampler {
    pub start_pass: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub downsample: Option<
        unsafe extern "C" fn(j_compress_ptr, JSAMPIMAGE, JDIMENSION, JSAMPIMAGE, JDIMENSION) -> (),
    >,
    pub need_context_rows: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_color_converter {
    pub start_pass: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub color_convert: Option<
        unsafe extern "C" fn(
            j_compress_ptr,
            JSAMPARRAY,
            JSAMPIMAGE,
            JDIMENSION,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_marker_writer {
    pub write_file_header: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub write_frame_header: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub write_scan_header: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub write_file_trailer: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub write_tables_only: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub write_marker_header:
        Option<unsafe extern "C" fn(j_compress_ptr, ::core::ffi::c_int, ::core::ffi::c_uint) -> ()>,
    pub write_marker_byte: Option<unsafe extern "C" fn(j_compress_ptr, ::core::ffi::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_c_coef_controller {
    pub start_pass: Option<unsafe extern "C" fn(j_compress_ptr, J_BUF_MODE) -> ()>,
    pub compress_data: Option<unsafe extern "C" fn(j_compress_ptr, JSAMPIMAGE) -> boolean>,
}
pub type J_BUF_MODE = ::core::ffi::c_uint;
pub const JBUF_SAVE_AND_PASS: J_BUF_MODE = 3;
pub const JBUF_CRANK_DEST: J_BUF_MODE = 2;
pub const JBUF_SAVE_SOURCE: J_BUF_MODE = 1;
pub const JBUF_PASS_THRU: J_BUF_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_c_prep_controller {
    pub start_pass: Option<unsafe extern "C" fn(j_compress_ptr, J_BUF_MODE) -> ()>,
    pub pre_process_data: Option<
        unsafe extern "C" fn(
            j_compress_ptr,
            JSAMPARRAY,
            *mut JDIMENSION,
            JDIMENSION,
            JSAMPIMAGE,
            *mut JDIMENSION,
            JDIMENSION,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_c_main_controller {
    pub start_pass: Option<unsafe extern "C" fn(j_compress_ptr, J_BUF_MODE) -> ()>,
    pub process_data:
        Option<unsafe extern "C" fn(j_compress_ptr, JSAMPARRAY, *mut JDIMENSION, JDIMENSION) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_comp_master {
    pub prepare_for_pass: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub pass_startup: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub finish_pass: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub call_pass_startup: boolean,
    pub is_last_pass: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut JOCTET,
    pub free_in_buffer: size_t,
    pub init_destination: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub empty_output_buffer: Option<unsafe extern "C" fn(j_compress_ptr) -> boolean>,
    pub term_destination: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
}
pub type my_marker_ptr = *mut my_marker_writer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_marker_writer {
    pub pub_0: jpeg_marker_writer,
    pub last_restart_interval: ::core::ffi::c_uint,
}
pub const JERR_CANT_SUSPEND: C2RustUnnamed_0 = 25;
pub type JPEG_MARKER = ::core::ffi::c_uint;
pub const M_ERROR: JPEG_MARKER = 256;
pub const M_TEM: JPEG_MARKER = 1;
pub const M_COM: JPEG_MARKER = 254;
pub const M_JPG13: JPEG_MARKER = 253;
pub const M_JPG0: JPEG_MARKER = 240;
pub const M_APP15: JPEG_MARKER = 239;
pub const M_APP14: JPEG_MARKER = 238;
pub const M_APP13: JPEG_MARKER = 237;
pub const M_APP12: JPEG_MARKER = 236;
pub const M_APP11: JPEG_MARKER = 235;
pub const M_APP10: JPEG_MARKER = 234;
pub const M_APP9: JPEG_MARKER = 233;
pub const M_APP8: JPEG_MARKER = 232;
pub const M_APP7: JPEG_MARKER = 231;
pub const M_APP6: JPEG_MARKER = 230;
pub const M_APP5: JPEG_MARKER = 229;
pub const M_APP4: JPEG_MARKER = 228;
pub const M_APP3: JPEG_MARKER = 227;
pub const M_APP2: JPEG_MARKER = 226;
pub const M_APP1: JPEG_MARKER = 225;
pub const M_APP0: JPEG_MARKER = 224;
pub const M_EXP: JPEG_MARKER = 223;
pub const M_DHP: JPEG_MARKER = 222;
pub const M_DRI: JPEG_MARKER = 221;
pub const M_DNL: JPEG_MARKER = 220;
pub const M_DQT: JPEG_MARKER = 219;
pub const M_SOS: JPEG_MARKER = 218;
pub const M_EOI: JPEG_MARKER = 217;
pub const M_SOI: JPEG_MARKER = 216;
pub const M_RST7: JPEG_MARKER = 215;
pub const M_RST6: JPEG_MARKER = 214;
pub const M_RST5: JPEG_MARKER = 213;
pub const M_RST4: JPEG_MARKER = 212;
pub const M_RST3: JPEG_MARKER = 211;
pub const M_RST2: JPEG_MARKER = 210;
pub const M_RST1: JPEG_MARKER = 209;
pub const M_RST0: JPEG_MARKER = 208;
pub const M_DAC: JPEG_MARKER = 204;
pub const M_DHT: JPEG_MARKER = 196;
pub const M_SOF15: JPEG_MARKER = 207;
pub const M_SOF14: JPEG_MARKER = 206;
pub const M_SOF13: JPEG_MARKER = 205;
pub const M_SOF11: JPEG_MARKER = 203;
pub const M_SOF10: JPEG_MARKER = 202;
pub const M_SOF9: JPEG_MARKER = 201;
pub const M_JPG: JPEG_MARKER = 200;
pub const M_SOF7: JPEG_MARKER = 199;
pub const M_SOF6: JPEG_MARKER = 198;
pub const M_SOF5: JPEG_MARKER = 197;
pub const M_SOF3: JPEG_MARKER = 195;
pub const M_SOF2: JPEG_MARKER = 194;
pub const M_SOF1: JPEG_MARKER = 193;
pub const M_SOF0: JPEG_MARKER = 192;
pub const JERR_BAD_LENGTH: C2RustUnnamed_0 = 12;
pub const JERR_NO_HUFF_TABLE: C2RustUnnamed_0 = 52;
pub const JERR_NO_QUANT_TABLE: C2RustUnnamed_0 = 54;
pub const JERR_IMAGE_TOO_BIG: C2RustUnnamed_0 = 42;
pub const JTRC_16BIT_TABLES: C2RustUnnamed_0 = 77;
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
pub const JERR_NO_IMAGE: C2RustUnnamed_0 = 53;
pub const JERR_NO_BACKING_STORE: C2RustUnnamed_0 = 51;
pub const JERR_NO_ARITH_TABLE: C2RustUnnamed_0 = 50;
pub const JERR_NOT_COMPILED: C2RustUnnamed_0 = 49;
pub const JERR_NOTIMPL: C2RustUnnamed_0 = 48;
pub const JERR_MODE_CHANGE: C2RustUnnamed_0 = 47;
pub const JERR_MISSING_DATA: C2RustUnnamed_0 = 46;
pub const JERR_MISMATCHED_QUANT_TABLE: C2RustUnnamed_0 = 45;
pub const JERR_INPUT_EOF: C2RustUnnamed_0 = 44;
pub const JERR_INPUT_EMPTY: C2RustUnnamed_0 = 43;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DCTSIZE2: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NUM_QUANT_TBLS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NUM_HUFF_TBLS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NUM_ARITH_TBLS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn emit_byte(mut cinfo: j_compress_ptr, mut val: ::core::ffi::c_int) {
    let mut dest: *mut jpeg_destination_mgr = (*cinfo).dest as *mut jpeg_destination_mgr;
    let fresh0 = (*dest).next_output_byte;
    (*dest).next_output_byte = (*dest).next_output_byte.offset(1);
    *fresh0 = val as JOCTET;
    (*dest).free_in_buffer = (*dest).free_in_buffer.wrapping_sub(1);
    if (*dest).free_in_buffer == 0 as size_t {
        if Some(
            (*dest)
                .empty_output_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            (*(*cinfo).err).msg_code = JERR_CANT_SUSPEND as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
}
unsafe extern "C" fn emit_marker(mut cinfo: j_compress_ptr, mut mark: JPEG_MARKER) {
    emit_byte(cinfo, 0xff as ::core::ffi::c_int);
    emit_byte(cinfo, mark as ::core::ffi::c_int);
}
unsafe extern "C" fn emit_2bytes(mut cinfo: j_compress_ptr, mut value: ::core::ffi::c_int) {
    emit_byte(
        cinfo,
        value >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
    );
    emit_byte(cinfo, value & 0xff as ::core::ffi::c_int);
}
unsafe extern "C" fn emit_dqt(
    mut cinfo: j_compress_ptr,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut qtbl: *mut JQUANT_TBL = (*cinfo).quant_tbl_ptrs[index as usize];
    let mut prec: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if qtbl.is_null() {
        (*(*cinfo).err).msg_code = JERR_NO_QUANT_TABLE as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = index;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    prec = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < DCTSIZE2 {
        if (*qtbl).quantval[i as usize] as ::core::ffi::c_int > 255 as ::core::ffi::c_int {
            prec = 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if (*qtbl).sent_table == 0 {
        emit_marker(cinfo, M_DQT);
        emit_2bytes(
            cinfo,
            if prec != 0 {
                DCTSIZE2 * 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
            } else {
                DCTSIZE2 + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            },
        );
        emit_byte(cinfo, index + (prec << 4 as ::core::ffi::c_int));
        i = 0 as ::core::ffi::c_int;
        while i < DCTSIZE2 {
            let mut qval: ::core::ffi::c_uint =
                (*qtbl).quantval[*(&raw const jpeg_natural_order as *const ::core::ffi::c_int)
                    .offset(i as isize) as usize] as ::core::ffi::c_uint;
            if prec != 0 {
                emit_byte(
                    cinfo,
                    (qval >> 8 as ::core::ffi::c_int) as ::core::ffi::c_int,
                );
            }
            emit_byte(
                cinfo,
                (qval & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_int,
            );
            i += 1;
        }
        (*qtbl).sent_table = TRUE as boolean;
    }
    return prec;
}
unsafe extern "C" fn emit_dht(
    mut cinfo: j_compress_ptr,
    mut index: ::core::ffi::c_int,
    mut is_ac: boolean,
) {
    let mut htbl: *mut JHUFF_TBL = ::core::ptr::null_mut::<JHUFF_TBL>();
    let mut length: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if is_ac != 0 {
        htbl = (*cinfo).ac_huff_tbl_ptrs[index as usize];
        index += 0x10 as ::core::ffi::c_int;
    } else {
        htbl = (*cinfo).dc_huff_tbl_ptrs[index as usize];
    }
    if htbl.is_null() {
        (*(*cinfo).err).msg_code = JERR_NO_HUFF_TABLE as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = index;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*htbl).sent_table == 0 {
        emit_marker(cinfo, M_DHT);
        length = 0 as ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        while i <= 16 as ::core::ffi::c_int {
            length += (*htbl).bits[i as usize] as ::core::ffi::c_int;
            i += 1;
        }
        emit_2bytes(
            cinfo,
            length + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
        );
        emit_byte(cinfo, index);
        i = 1 as ::core::ffi::c_int;
        while i <= 16 as ::core::ffi::c_int {
            emit_byte(cinfo, (*htbl).bits[i as usize] as ::core::ffi::c_int);
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < length {
            emit_byte(cinfo, (*htbl).huffval[i as usize] as ::core::ffi::c_int);
            i += 1;
        }
        (*htbl).sent_table = TRUE as boolean;
    }
}
unsafe extern "C" fn emit_dac(mut cinfo: j_compress_ptr) {
    let mut dc_in_use: [::core::ffi::c_char; 16] = [0; 16];
    let mut ac_in_use: [::core::ffi::c_char; 16] = [0; 16];
    let mut length: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    i = 0 as ::core::ffi::c_int;
    while i < NUM_ARITH_TBLS {
        ac_in_use[i as usize] = 0 as ::core::ffi::c_char;
        dc_in_use[i as usize] = ac_in_use[i as usize];
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[i as usize];
        if (*cinfo).Ss == 0 as ::core::ffi::c_int && (*cinfo).Ah == 0 as ::core::ffi::c_int {
            dc_in_use[(*compptr).dc_tbl_no as usize] = 1 as ::core::ffi::c_char;
        }
        if (*cinfo).Se != 0 {
            ac_in_use[(*compptr).ac_tbl_no as usize] = 1 as ::core::ffi::c_char;
        }
        i += 1;
    }
    length = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < NUM_ARITH_TBLS {
        length += dc_in_use[i as usize] as ::core::ffi::c_int
            + ac_in_use[i as usize] as ::core::ffi::c_int;
        i += 1;
    }
    if length != 0 {
        emit_marker(cinfo, M_DAC);
        emit_2bytes(
            cinfo,
            length * 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
        );
        i = 0 as ::core::ffi::c_int;
        while i < NUM_ARITH_TBLS {
            if dc_in_use[i as usize] != 0 {
                emit_byte(cinfo, i);
                emit_byte(
                    cinfo,
                    (*cinfo).arith_dc_L[i as usize] as ::core::ffi::c_int
                        + (((*cinfo).arith_dc_U[i as usize] as ::core::ffi::c_int)
                            << 4 as ::core::ffi::c_int),
                );
            }
            if ac_in_use[i as usize] != 0 {
                emit_byte(cinfo, i + 0x10 as ::core::ffi::c_int);
                emit_byte(cinfo, (*cinfo).arith_ac_K[i as usize] as ::core::ffi::c_int);
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn emit_dri(mut cinfo: j_compress_ptr) {
    emit_marker(cinfo, M_DRI);
    emit_2bytes(cinfo, 4 as ::core::ffi::c_int);
    emit_2bytes(cinfo, (*cinfo).restart_interval as ::core::ffi::c_int);
}
unsafe extern "C" fn emit_sof(mut cinfo: j_compress_ptr, mut code: JPEG_MARKER) {
    let mut ci: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    emit_marker(cinfo, code);
    emit_2bytes(
        cinfo,
        3 as ::core::ffi::c_int * (*cinfo).num_components
            + 2 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int,
    );
    if (*cinfo).jpeg_height as ::core::ffi::c_long > 65535 as ::core::ffi::c_long
        || (*cinfo).jpeg_width as ::core::ffi::c_long > 65535 as ::core::ffi::c_long
    {
        (*(*cinfo).err).msg_code = JERR_IMAGE_TOO_BIG as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
            65535 as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    emit_byte(cinfo, (*cinfo).data_precision);
    emit_2bytes(cinfo, (*cinfo).jpeg_height as ::core::ffi::c_int);
    emit_2bytes(cinfo, (*cinfo).jpeg_width as ::core::ffi::c_int);
    emit_byte(cinfo, (*cinfo).num_components);
    ci = 0 as ::core::ffi::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        emit_byte(cinfo, (*compptr).component_id);
        emit_byte(
            cinfo,
            ((*compptr).h_samp_factor << 4 as ::core::ffi::c_int) + (*compptr).v_samp_factor,
        );
        emit_byte(cinfo, (*compptr).quant_tbl_no);
        ci += 1;
        compptr = compptr.offset(1);
    }
}
unsafe extern "C" fn emit_sos(mut cinfo: j_compress_ptr) {
    let mut i: ::core::ffi::c_int = 0;
    let mut td: ::core::ffi::c_int = 0;
    let mut ta: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    emit_marker(cinfo, M_SOS);
    emit_2bytes(
        cinfo,
        2 as ::core::ffi::c_int * (*cinfo).comps_in_scan
            + 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int,
    );
    emit_byte(cinfo, (*cinfo).comps_in_scan);
    i = 0 as ::core::ffi::c_int;
    while i < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[i as usize];
        emit_byte(cinfo, (*compptr).component_id);
        td = if (*cinfo).Ss == 0 as ::core::ffi::c_int && (*cinfo).Ah == 0 as ::core::ffi::c_int {
            (*compptr).dc_tbl_no
        } else {
            0 as ::core::ffi::c_int
        };
        ta = if (*cinfo).Se != 0 {
            (*compptr).ac_tbl_no
        } else {
            0 as ::core::ffi::c_int
        };
        emit_byte(cinfo, (td << 4 as ::core::ffi::c_int) + ta);
        i += 1;
    }
    emit_byte(cinfo, (*cinfo).Ss);
    emit_byte(cinfo, (*cinfo).Se);
    emit_byte(
        cinfo,
        ((*cinfo).Ah << 4 as ::core::ffi::c_int) + (*cinfo).Al,
    );
}
unsafe extern "C" fn emit_jfif_app0(mut cinfo: j_compress_ptr) {
    emit_marker(cinfo, M_APP0);
    emit_2bytes(
        cinfo,
        2 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int,
    );
    emit_byte(cinfo, 0x4a as ::core::ffi::c_int);
    emit_byte(cinfo, 0x46 as ::core::ffi::c_int);
    emit_byte(cinfo, 0x49 as ::core::ffi::c_int);
    emit_byte(cinfo, 0x46 as ::core::ffi::c_int);
    emit_byte(cinfo, 0 as ::core::ffi::c_int);
    emit_byte(cinfo, (*cinfo).JFIF_major_version as ::core::ffi::c_int);
    emit_byte(cinfo, (*cinfo).JFIF_minor_version as ::core::ffi::c_int);
    emit_byte(cinfo, (*cinfo).density_unit as ::core::ffi::c_int);
    emit_2bytes(cinfo, (*cinfo).X_density as ::core::ffi::c_int);
    emit_2bytes(cinfo, (*cinfo).Y_density as ::core::ffi::c_int);
    emit_byte(cinfo, 0 as ::core::ffi::c_int);
    emit_byte(cinfo, 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn emit_adobe_app14(mut cinfo: j_compress_ptr) {
    emit_marker(cinfo, M_APP14);
    emit_2bytes(
        cinfo,
        2 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int,
    );
    emit_byte(cinfo, 0x41 as ::core::ffi::c_int);
    emit_byte(cinfo, 0x64 as ::core::ffi::c_int);
    emit_byte(cinfo, 0x6f as ::core::ffi::c_int);
    emit_byte(cinfo, 0x62 as ::core::ffi::c_int);
    emit_byte(cinfo, 0x65 as ::core::ffi::c_int);
    emit_2bytes(cinfo, 100 as ::core::ffi::c_int);
    emit_2bytes(cinfo, 0 as ::core::ffi::c_int);
    emit_2bytes(cinfo, 0 as ::core::ffi::c_int);
    match (*cinfo).jpeg_color_space as ::core::ffi::c_uint {
        3 => {
            emit_byte(cinfo, 1 as ::core::ffi::c_int);
        }
        5 => {
            emit_byte(cinfo, 2 as ::core::ffi::c_int);
        }
        _ => {
            emit_byte(cinfo, 0 as ::core::ffi::c_int);
        }
    };
}
unsafe extern "C" fn write_marker_header(
    mut cinfo: j_compress_ptr,
    mut marker: ::core::ffi::c_int,
    mut datalen: ::core::ffi::c_uint,
) {
    if datalen > 65533 as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*(*cinfo).err).msg_code = JERR_BAD_LENGTH as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    emit_marker(cinfo, marker as JPEG_MARKER);
    emit_2bytes(
        cinfo,
        datalen.wrapping_add(2 as ::core::ffi::c_uint) as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn write_marker_byte(mut cinfo: j_compress_ptr, mut val: ::core::ffi::c_int) {
    emit_byte(cinfo, val);
}
unsafe extern "C" fn write_file_header(mut cinfo: j_compress_ptr) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    emit_marker(cinfo, M_SOI);
    (*marker).last_restart_interval = 0 as ::core::ffi::c_uint;
    if (*cinfo).write_JFIF_header != 0 {
        emit_jfif_app0(cinfo);
    }
    if (*cinfo).write_Adobe_marker != 0 {
        emit_adobe_app14(cinfo);
    }
}
unsafe extern "C" fn write_frame_header(mut cinfo: j_compress_ptr) {
    let mut ci: ::core::ffi::c_int = 0;
    let mut prec: ::core::ffi::c_int = 0;
    let mut is_baseline: boolean = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    prec = 0 as ::core::ffi::c_int;
    ci = 0 as ::core::ffi::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        prec += emit_dqt(cinfo, (*compptr).quant_tbl_no);
        ci += 1;
        compptr = compptr.offset(1);
    }
    if (*cinfo).arith_code != 0
        || (*cinfo).progressive_mode != 0
        || (*cinfo).data_precision != 8 as ::core::ffi::c_int
    {
        is_baseline = FALSE as boolean;
    } else {
        is_baseline = TRUE as boolean;
        ci = 0 as ::core::ffi::c_int;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            if (*compptr).dc_tbl_no > 1 as ::core::ffi::c_int
                || (*compptr).ac_tbl_no > 1 as ::core::ffi::c_int
            {
                is_baseline = FALSE as boolean;
            }
            ci += 1;
            compptr = compptr.offset(1);
        }
        if prec != 0 && is_baseline != 0 {
            is_baseline = FALSE as boolean;
            (*(*cinfo).err).msg_code = JTRC_16BIT_TABLES as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 0 as ::core::ffi::c_int
            );
        }
    }
    if (*cinfo).arith_code != 0 {
        if (*cinfo).progressive_mode != 0 {
            emit_sof(cinfo, M_SOF10);
        } else {
            emit_sof(cinfo, M_SOF9);
        }
    } else if (*cinfo).progressive_mode != 0 {
        emit_sof(cinfo, M_SOF2);
    } else if is_baseline != 0 {
        emit_sof(cinfo, M_SOF0);
    } else {
        emit_sof(cinfo, M_SOF1);
    };
}
unsafe extern "C" fn write_scan_header(mut cinfo: j_compress_ptr) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    let mut i: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    if (*cinfo).arith_code != 0 {
        emit_dac(cinfo);
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < (*cinfo).comps_in_scan {
            compptr = (*cinfo).cur_comp_info[i as usize];
            if (*cinfo).Ss == 0 as ::core::ffi::c_int && (*cinfo).Ah == 0 as ::core::ffi::c_int {
                emit_dht(cinfo, (*compptr).dc_tbl_no, FALSE);
            }
            if (*cinfo).Se != 0 {
                emit_dht(cinfo, (*compptr).ac_tbl_no, TRUE);
            }
            i += 1;
        }
    }
    if (*cinfo).restart_interval != (*marker).last_restart_interval {
        emit_dri(cinfo);
        (*marker).last_restart_interval = (*cinfo).restart_interval;
    }
    emit_sos(cinfo);
}
unsafe extern "C" fn write_file_trailer(mut cinfo: j_compress_ptr) {
    emit_marker(cinfo, M_EOI);
}
unsafe extern "C" fn write_tables_only(mut cinfo: j_compress_ptr) {
    let mut i: ::core::ffi::c_int = 0;
    emit_marker(cinfo, M_SOI);
    i = 0 as ::core::ffi::c_int;
    while i < NUM_QUANT_TBLS {
        if !(*cinfo).quant_tbl_ptrs[i as usize].is_null() {
            emit_dqt(cinfo, i);
        }
        i += 1;
    }
    if (*cinfo).arith_code == 0 {
        i = 0 as ::core::ffi::c_int;
        while i < NUM_HUFF_TBLS {
            if !(*cinfo).dc_huff_tbl_ptrs[i as usize].is_null() {
                emit_dht(cinfo, i, FALSE);
            }
            if !(*cinfo).ac_huff_tbl_ptrs[i as usize].is_null() {
                emit_dht(cinfo, i, TRUE);
            }
            i += 1;
        }
    }
    emit_marker(cinfo, M_EOI);
}
#[no_mangle]
pub unsafe extern "C" fn jinit_marker_writer(mut cinfo: j_compress_ptr) {
    let mut marker: my_marker_ptr = ::core::ptr::null_mut::<my_marker_writer>();
    marker = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<my_marker_writer>() as size_t,
    ) as my_marker_ptr;
    (*cinfo).marker = marker as *mut jpeg_marker_writer as *mut jpeg_marker_writer;
    (*marker).pub_0.write_file_header =
        Some(write_file_header as unsafe extern "C" fn(j_compress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
    (*marker).pub_0.write_frame_header =
        Some(write_frame_header as unsafe extern "C" fn(j_compress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
    (*marker).pub_0.write_scan_header =
        Some(write_scan_header as unsafe extern "C" fn(j_compress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
    (*marker).pub_0.write_file_trailer =
        Some(write_file_trailer as unsafe extern "C" fn(j_compress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
    (*marker).pub_0.write_tables_only =
        Some(write_tables_only as unsafe extern "C" fn(j_compress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
    (*marker).pub_0.write_marker_header = Some(
        write_marker_header
            as unsafe extern "C" fn(j_compress_ptr, ::core::ffi::c_int, ::core::ffi::c_uint) -> (),
    )
        as Option<
            unsafe extern "C" fn(j_compress_ptr, ::core::ffi::c_int, ::core::ffi::c_uint) -> (),
        >;
    (*marker).pub_0.write_marker_byte =
        Some(write_marker_byte as unsafe extern "C" fn(j_compress_ptr, ::core::ffi::c_int) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr, ::core::ffi::c_int) -> ()>;
    (*marker).last_restart_interval = 0 as ::core::ffi::c_uint;
}

pub const JPEG_RS_JCMARKER_LINK_ANCHOR: unsafe extern "C" fn(j_compress_ptr) = jinit_marker_writer;
