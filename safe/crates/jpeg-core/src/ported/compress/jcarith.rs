#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}

extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    static jpeg_natural_order: [::core::ffi::c_int; 0];
    static jpeg_aritab: [JLONG; 0];
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
pub type JLONG = ::core::ffi::c_long;
pub type arith_entropy_ptr = *mut arith_entropy_encoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arith_entropy_encoder {
    pub pub_0: jpeg_entropy_encoder,
    pub c: JLONG,
    pub a: JLONG,
    pub sc: JLONG,
    pub zc: JLONG,
    pub ct: ::core::ffi::c_int,
    pub buffer: ::core::ffi::c_int,
    pub last_dc_val: [::core::ffi::c_int; 4],
    pub dc_context: [::core::ffi::c_int; 4],
    pub restarts_to_go: ::core::ffi::c_uint,
    pub next_restart_num: ::core::ffi::c_int,
    pub dc_stats: [*mut ::core::ffi::c_uchar; 16],
    pub ac_stats: [*mut ::core::ffi::c_uchar; 16],
    pub fixed_bin: [::core::ffi::c_uchar; 4],
}
pub const JERR_CANT_SUSPEND: C2RustUnnamed_0 = 25;
pub const JERR_NO_ARITH_TABLE: C2RustUnnamed_0 = 50;
pub const JERR_NOTIMPL: C2RustUnnamed_0 = 48;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DCTSIZE2: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NUM_ARITH_TBLS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPEG_RST0: ::core::ffi::c_int = 0xd0 as ::core::ffi::c_int;
pub const DC_STAT_BINS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const AC_STAT_BINS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
unsafe extern "C" fn emit_byte(mut val: ::core::ffi::c_int, mut cinfo: j_compress_ptr) {
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
unsafe extern "C" fn finish_pass(mut cinfo: j_compress_ptr) {
    let mut e: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut temp: JLONG = 0;
    temp = (((*e).a - 1 as JLONG + (*e).c) as ::core::ffi::c_ulong
        & 0xffff0000 as ::core::ffi::c_ulong) as JLONG;
    if temp < (*e).c {
        (*e).c = (temp as ::core::ffi::c_long + 0x8000 as ::core::ffi::c_long) as JLONG;
    } else {
        (*e).c = temp;
    }
    (*e).c <<= (*e).ct;
    if (*e).c as ::core::ffi::c_ulong & 0xf8000000 as ::core::ffi::c_ulong != 0 {
        if (*e).buffer >= 0 as ::core::ffi::c_int {
            if (*e).zc != 0 {
                loop {
                    emit_byte(0 as ::core::ffi::c_int, cinfo);
                    (*e).zc -= 1;
                    if !((*e).zc != 0) {
                        break;
                    }
                }
            }
            emit_byte((*e).buffer + 1 as ::core::ffi::c_int, cinfo);
            if (*e).buffer + 1 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
                emit_byte(0 as ::core::ffi::c_int, cinfo);
            }
        }
        (*e).zc += (*e).sc;
        (*e).sc = 0 as JLONG;
    } else {
        if (*e).buffer == 0 as ::core::ffi::c_int {
            (*e).zc += 1;
        } else if (*e).buffer >= 0 as ::core::ffi::c_int {
            if (*e).zc != 0 {
                loop {
                    emit_byte(0 as ::core::ffi::c_int, cinfo);
                    (*e).zc -= 1;
                    if !((*e).zc != 0) {
                        break;
                    }
                }
            }
            emit_byte((*e).buffer, cinfo);
        }
        if (*e).sc != 0 {
            if (*e).zc != 0 {
                loop {
                    emit_byte(0 as ::core::ffi::c_int, cinfo);
                    (*e).zc -= 1;
                    if !((*e).zc != 0) {
                        break;
                    }
                }
            }
            loop {
                emit_byte(0xff as ::core::ffi::c_int, cinfo);
                emit_byte(0 as ::core::ffi::c_int, cinfo);
                (*e).sc -= 1;
                if !((*e).sc != 0) {
                    break;
                }
            }
        }
    }
    if (*e).c as ::core::ffi::c_long & 0x7fff800 as ::core::ffi::c_long != 0 {
        if (*e).zc != 0 {
            loop {
                emit_byte(0 as ::core::ffi::c_int, cinfo);
                (*e).zc -= 1;
                if !((*e).zc != 0) {
                    break;
                }
            }
        }
        emit_byte(
            ((*e).c >> 19 as ::core::ffi::c_int & 0xff as JLONG) as ::core::ffi::c_int,
            cinfo,
        );
        if (*e).c >> 19 as ::core::ffi::c_int & 0xff as JLONG == 0xff as JLONG {
            emit_byte(0 as ::core::ffi::c_int, cinfo);
        }
        if (*e).c as ::core::ffi::c_long & 0x7f800 as ::core::ffi::c_long != 0 {
            emit_byte(
                ((*e).c >> 11 as ::core::ffi::c_int & 0xff as JLONG) as ::core::ffi::c_int,
                cinfo,
            );
            if (*e).c >> 11 as ::core::ffi::c_int & 0xff as JLONG == 0xff as JLONG {
                emit_byte(0 as ::core::ffi::c_int, cinfo);
            }
        }
    }
}
unsafe extern "C" fn arith_encode(
    mut cinfo: j_compress_ptr,
    mut st: *mut ::core::ffi::c_uchar,
    mut val: ::core::ffi::c_int,
) {
    let mut e: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut nl: ::core::ffi::c_uchar = 0;
    let mut nm: ::core::ffi::c_uchar = 0;
    let mut qe: JLONG = 0;
    let mut temp: JLONG = 0;
    let mut sv: ::core::ffi::c_int = 0;
    sv = *st as ::core::ffi::c_int;
    qe = *(&raw const jpeg_aritab as *const JLONG)
        .offset((sv & 0x7f as ::core::ffi::c_int) as isize);
    nl = (qe & 0xff as JLONG) as ::core::ffi::c_uchar;
    qe >>= 8 as ::core::ffi::c_int;
    nm = (qe & 0xff as JLONG) as ::core::ffi::c_uchar;
    qe >>= 8 as ::core::ffi::c_int;
    (*e).a -= qe;
    if val != sv >> 7 as ::core::ffi::c_int {
        if (*e).a >= qe {
            (*e).c += (*e).a;
            (*e).a = qe;
        }
        *st = (sv & 0x80 as ::core::ffi::c_int ^ nl as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    } else {
        if (*e).a >= 0x8000 as ::core::ffi::c_long {
            return;
        }
        if (*e).a < qe {
            (*e).c += (*e).a;
            (*e).a = qe;
        }
        *st = (sv & 0x80 as ::core::ffi::c_int ^ nm as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    }
    loop {
        (*e).a <<= 1 as ::core::ffi::c_int;
        (*e).c <<= 1 as ::core::ffi::c_int;
        (*e).ct -= 1;
        if (*e).ct == 0 as ::core::ffi::c_int {
            temp = (*e).c >> 19 as ::core::ffi::c_int;
            if temp > 0xff as JLONG {
                if (*e).buffer >= 0 as ::core::ffi::c_int {
                    if (*e).zc != 0 {
                        loop {
                            emit_byte(0 as ::core::ffi::c_int, cinfo);
                            (*e).zc -= 1;
                            if !((*e).zc != 0) {
                                break;
                            }
                        }
                    }
                    emit_byte((*e).buffer + 1 as ::core::ffi::c_int, cinfo);
                    if (*e).buffer + 1 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
                        emit_byte(0 as ::core::ffi::c_int, cinfo);
                    }
                }
                (*e).zc += (*e).sc;
                (*e).sc = 0 as JLONG;
                (*e).buffer = (temp & 0xff as JLONG) as ::core::ffi::c_int;
            } else if temp == 0xff as JLONG {
                (*e).sc += 1;
            } else {
                if (*e).buffer == 0 as ::core::ffi::c_int {
                    (*e).zc += 1;
                } else if (*e).buffer >= 0 as ::core::ffi::c_int {
                    if (*e).zc != 0 {
                        loop {
                            emit_byte(0 as ::core::ffi::c_int, cinfo);
                            (*e).zc -= 1;
                            if !((*e).zc != 0) {
                                break;
                            }
                        }
                    }
                    emit_byte((*e).buffer, cinfo);
                }
                if (*e).sc != 0 {
                    if (*e).zc != 0 {
                        loop {
                            emit_byte(0 as ::core::ffi::c_int, cinfo);
                            (*e).zc -= 1;
                            if !((*e).zc != 0) {
                                break;
                            }
                        }
                    }
                    loop {
                        emit_byte(0xff as ::core::ffi::c_int, cinfo);
                        emit_byte(0 as ::core::ffi::c_int, cinfo);
                        (*e).sc -= 1;
                        if !((*e).sc != 0) {
                            break;
                        }
                    }
                }
                (*e).buffer = (temp & 0xff as JLONG) as ::core::ffi::c_int;
            }
            (*e).c &= 0x7ffff as ::core::ffi::c_long;
            (*e).ct += 8 as ::core::ffi::c_int;
        }
        if !((*e).a < 0x8000 as ::core::ffi::c_long) {
            break;
        }
    }
}
unsafe extern "C" fn emit_restart(mut cinfo: j_compress_ptr, mut restart_num: ::core::ffi::c_int) {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut ci: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    finish_pass(cinfo);
    emit_byte(0xff as ::core::ffi::c_int, cinfo);
    emit_byte(JPEG_RST0 + restart_num, cinfo);
    ci = 0 as ::core::ffi::c_int;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        if (*cinfo).progressive_mode == 0 as ::core::ffi::c_int
            || (*cinfo).Ss == 0 as ::core::ffi::c_int && (*cinfo).Ah == 0 as ::core::ffi::c_int
        {
            memset(
                (*entropy).dc_stats[(*compptr).dc_tbl_no as usize] as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                DC_STAT_BINS as size_t,
            );
            (*entropy).last_dc_val[ci as usize] = 0 as ::core::ffi::c_int;
            (*entropy).dc_context[ci as usize] = 0 as ::core::ffi::c_int;
        }
        if (*cinfo).progressive_mode == 0 as ::core::ffi::c_int || (*cinfo).Se != 0 {
            memset(
                (*entropy).ac_stats[(*compptr).ac_tbl_no as usize] as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                AC_STAT_BINS as size_t,
            );
        }
        ci += 1;
    }
    (*entropy).c = 0 as JLONG;
    (*entropy).a = 0x10000 as ::core::ffi::c_long as JLONG;
    (*entropy).sc = 0 as JLONG;
    (*entropy).zc = 0 as JLONG;
    (*entropy).ct = 11 as ::core::ffi::c_int;
    (*entropy).buffer = -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn encode_mcu_DC_first(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut block: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut st: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut blkn: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut tbl: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    let mut v2: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as ::core::ffi::c_uint {
            emit_restart(cinfo, (*entropy).next_restart_num);
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as ::core::ffi::c_int;
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    }
    blkn = 0 as ::core::ffi::c_int;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        ci = (*cinfo).MCU_membership[blkn as usize];
        tbl = (*(*cinfo).cur_comp_info[ci as usize]).dc_tbl_no;
        m = (*block)[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int >> (*cinfo).Al;
        st = (*entropy).dc_stats[tbl as usize].offset((*entropy).dc_context[ci as usize] as isize);
        v = m - (*entropy).last_dc_val[ci as usize];
        if v == 0 as ::core::ffi::c_int {
            arith_encode(cinfo, st, 0 as ::core::ffi::c_int);
            (*entropy).dc_context[ci as usize] = 0 as ::core::ffi::c_int;
        } else {
            (*entropy).last_dc_val[ci as usize] = m;
            arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
            if v > 0 as ::core::ffi::c_int {
                arith_encode(
                    cinfo,
                    st.offset(1 as ::core::ffi::c_int as isize),
                    0 as ::core::ffi::c_int,
                );
                st = st.offset(2 as ::core::ffi::c_int as isize);
                (*entropy).dc_context[ci as usize] = 4 as ::core::ffi::c_int;
            } else {
                v = -v;
                arith_encode(
                    cinfo,
                    st.offset(1 as ::core::ffi::c_int as isize),
                    1 as ::core::ffi::c_int,
                );
                st = st.offset(3 as ::core::ffi::c_int as isize);
                (*entropy).dc_context[ci as usize] = 8 as ::core::ffi::c_int;
            }
            m = 0 as ::core::ffi::c_int;
            v -= 1 as ::core::ffi::c_int;
            if v != 0 {
                arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
                m = 1 as ::core::ffi::c_int;
                v2 = v;
                st = (*entropy).dc_stats[tbl as usize].offset(20 as ::core::ffi::c_int as isize);
                loop {
                    v2 >>= 1 as ::core::ffi::c_int;
                    if !(v2 != 0) {
                        break;
                    }
                    arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
                    m <<= 1 as ::core::ffi::c_int;
                    st = st.offset(1 as ::core::ffi::c_int as isize);
                }
            }
            arith_encode(cinfo, st, 0 as ::core::ffi::c_int);
            if m < ((1 as ::core::ffi::c_long)
                << (*cinfo).arith_dc_L[tbl as usize] as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int
            {
                (*entropy).dc_context[ci as usize] = 0 as ::core::ffi::c_int;
            } else if m
                > ((1 as ::core::ffi::c_long)
                    << (*cinfo).arith_dc_U[tbl as usize] as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int
            {
                (*entropy).dc_context[ci as usize] += 8 as ::core::ffi::c_int;
            }
            st = st.offset(14 as ::core::ffi::c_int as isize);
            loop {
                m >>= 1 as ::core::ffi::c_int;
                if !(m != 0) {
                    break;
                }
                arith_encode(
                    cinfo,
                    st,
                    if m & v != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    },
                );
            }
        }
        blkn += 1;
    }
    return TRUE;
}
unsafe extern "C" fn encode_mcu_AC_first(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut block: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut st: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut tbl: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ke: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    let mut v2: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as ::core::ffi::c_uint {
            emit_restart(cinfo, (*entropy).next_restart_num);
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as ::core::ffi::c_int;
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    }
    block = *MCU_data.offset(0 as ::core::ffi::c_int as isize);
    tbl = (*(*cinfo).cur_comp_info[0 as ::core::ffi::c_int as usize]).ac_tbl_no;
    ke = (*cinfo).Se;
    while ke > 0 as ::core::ffi::c_int {
        v = (*block)[*(&raw const jpeg_natural_order as *const ::core::ffi::c_int)
            .offset(ke as isize) as usize] as ::core::ffi::c_int;
        if v >= 0 as ::core::ffi::c_int {
            v >>= (*cinfo).Al;
            if v != 0 {
                break;
            }
        } else {
            v = -v;
            v >>= (*cinfo).Al;
            if v != 0 {
                break;
            }
        }
        ke -= 1;
    }
    k = (*cinfo).Ss;
    while k <= ke {
        st = (*entropy).ac_stats[tbl as usize]
            .offset((3 as ::core::ffi::c_int * (k - 1 as ::core::ffi::c_int)) as isize);
        arith_encode(cinfo, st, 0 as ::core::ffi::c_int);
        loop {
            v = (*block)[*(&raw const jpeg_natural_order as *const ::core::ffi::c_int)
                .offset(k as isize) as usize] as ::core::ffi::c_int;
            if v >= 0 as ::core::ffi::c_int {
                v >>= (*cinfo).Al;
                if v != 0 {
                    arith_encode(
                        cinfo,
                        st.offset(1 as ::core::ffi::c_int as isize),
                        1 as ::core::ffi::c_int,
                    );
                    arith_encode(
                        cinfo,
                        &raw mut (*entropy).fixed_bin as *mut ::core::ffi::c_uchar,
                        0 as ::core::ffi::c_int,
                    );
                    break;
                }
            } else {
                v = -v;
                v >>= (*cinfo).Al;
                if v != 0 {
                    arith_encode(
                        cinfo,
                        st.offset(1 as ::core::ffi::c_int as isize),
                        1 as ::core::ffi::c_int,
                    );
                    arith_encode(
                        cinfo,
                        &raw mut (*entropy).fixed_bin as *mut ::core::ffi::c_uchar,
                        1 as ::core::ffi::c_int,
                    );
                    break;
                }
            }
            arith_encode(
                cinfo,
                st.offset(1 as ::core::ffi::c_int as isize),
                0 as ::core::ffi::c_int,
            );
            st = st.offset(3 as ::core::ffi::c_int as isize);
            k += 1;
        }
        st = st.offset(2 as ::core::ffi::c_int as isize);
        m = 0 as ::core::ffi::c_int;
        v -= 1 as ::core::ffi::c_int;
        if v != 0 {
            arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
            m = 1 as ::core::ffi::c_int;
            v2 = v;
            v2 >>= 1 as ::core::ffi::c_int;
            if v2 != 0 {
                arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
                m <<= 1 as ::core::ffi::c_int;
                st = (*entropy).ac_stats[tbl as usize].offset(
                    (if k <= (*cinfo).arith_ac_K[tbl as usize] as ::core::ffi::c_int {
                        189 as ::core::ffi::c_int
                    } else {
                        217 as ::core::ffi::c_int
                    }) as isize,
                );
                loop {
                    v2 >>= 1 as ::core::ffi::c_int;
                    if !(v2 != 0) {
                        break;
                    }
                    arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
                    m <<= 1 as ::core::ffi::c_int;
                    st = st.offset(1 as ::core::ffi::c_int as isize);
                }
            }
        }
        arith_encode(cinfo, st, 0 as ::core::ffi::c_int);
        st = st.offset(14 as ::core::ffi::c_int as isize);
        loop {
            m >>= 1 as ::core::ffi::c_int;
            if !(m != 0) {
                break;
            }
            arith_encode(
                cinfo,
                st,
                if m & v != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                },
            );
        }
        k += 1;
    }
    if k <= (*cinfo).Se {
        st = (*entropy).ac_stats[tbl as usize]
            .offset((3 as ::core::ffi::c_int * (k - 1 as ::core::ffi::c_int)) as isize);
        arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
    }
    return TRUE;
}
unsafe extern "C" fn encode_mcu_DC_refine(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut st: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut Al: ::core::ffi::c_int = 0;
    let mut blkn: ::core::ffi::c_int = 0;
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as ::core::ffi::c_uint {
            emit_restart(cinfo, (*entropy).next_restart_num);
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as ::core::ffi::c_int;
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    }
    st = &raw mut (*entropy).fixed_bin as *mut ::core::ffi::c_uchar;
    Al = (*cinfo).Al;
    blkn = 0 as ::core::ffi::c_int;
    while blkn < (*cinfo).blocks_in_MCU {
        arith_encode(
            cinfo,
            st,
            (*(*MCU_data.offset(blkn as isize)).offset(0 as ::core::ffi::c_int as isize))
                [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                >> Al
                & 1 as ::core::ffi::c_int,
        );
        blkn += 1;
    }
    return TRUE;
}
unsafe extern "C" fn encode_mcu_AC_refine(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut block: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut st: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut tbl: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ke: ::core::ffi::c_int = 0;
    let mut kex: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as ::core::ffi::c_uint {
            emit_restart(cinfo, (*entropy).next_restart_num);
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as ::core::ffi::c_int;
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    }
    block = *MCU_data.offset(0 as ::core::ffi::c_int as isize);
    tbl = (*(*cinfo).cur_comp_info[0 as ::core::ffi::c_int as usize]).ac_tbl_no;
    ke = (*cinfo).Se;
    while ke > 0 as ::core::ffi::c_int {
        v = (*block)[*(&raw const jpeg_natural_order as *const ::core::ffi::c_int)
            .offset(ke as isize) as usize] as ::core::ffi::c_int;
        if v >= 0 as ::core::ffi::c_int {
            v >>= (*cinfo).Al;
            if v != 0 {
                break;
            }
        } else {
            v = -v;
            v >>= (*cinfo).Al;
            if v != 0 {
                break;
            }
        }
        ke -= 1;
    }
    kex = ke;
    while kex > 0 as ::core::ffi::c_int {
        v = (*block)[*(&raw const jpeg_natural_order as *const ::core::ffi::c_int)
            .offset(kex as isize) as usize] as ::core::ffi::c_int;
        if v >= 0 as ::core::ffi::c_int {
            v >>= (*cinfo).Ah;
            if v != 0 {
                break;
            }
        } else {
            v = -v;
            v >>= (*cinfo).Ah;
            if v != 0 {
                break;
            }
        }
        kex -= 1;
    }
    k = (*cinfo).Ss;
    while k <= ke {
        st = (*entropy).ac_stats[tbl as usize]
            .offset((3 as ::core::ffi::c_int * (k - 1 as ::core::ffi::c_int)) as isize);
        if k > kex {
            arith_encode(cinfo, st, 0 as ::core::ffi::c_int);
        }
        loop {
            v = (*block)[*(&raw const jpeg_natural_order as *const ::core::ffi::c_int)
                .offset(k as isize) as usize] as ::core::ffi::c_int;
            if v >= 0 as ::core::ffi::c_int {
                v >>= (*cinfo).Al;
                if v != 0 {
                    if v >> 1 as ::core::ffi::c_int != 0 {
                        arith_encode(
                            cinfo,
                            st.offset(2 as ::core::ffi::c_int as isize),
                            v & 1 as ::core::ffi::c_int,
                        );
                    } else {
                        arith_encode(
                            cinfo,
                            st.offset(1 as ::core::ffi::c_int as isize),
                            1 as ::core::ffi::c_int,
                        );
                        arith_encode(
                            cinfo,
                            &raw mut (*entropy).fixed_bin as *mut ::core::ffi::c_uchar,
                            0 as ::core::ffi::c_int,
                        );
                    }
                    break;
                }
            } else {
                v = -v;
                v >>= (*cinfo).Al;
                if v != 0 {
                    if v >> 1 as ::core::ffi::c_int != 0 {
                        arith_encode(
                            cinfo,
                            st.offset(2 as ::core::ffi::c_int as isize),
                            v & 1 as ::core::ffi::c_int,
                        );
                    } else {
                        arith_encode(
                            cinfo,
                            st.offset(1 as ::core::ffi::c_int as isize),
                            1 as ::core::ffi::c_int,
                        );
                        arith_encode(
                            cinfo,
                            &raw mut (*entropy).fixed_bin as *mut ::core::ffi::c_uchar,
                            1 as ::core::ffi::c_int,
                        );
                    }
                    break;
                }
            }
            arith_encode(
                cinfo,
                st.offset(1 as ::core::ffi::c_int as isize),
                0 as ::core::ffi::c_int,
            );
            st = st.offset(3 as ::core::ffi::c_int as isize);
            k += 1;
        }
        k += 1;
    }
    if k <= (*cinfo).Se {
        st = (*entropy).ac_stats[tbl as usize]
            .offset((3 as ::core::ffi::c_int * (k - 1 as ::core::ffi::c_int)) as isize);
        arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
    }
    return TRUE;
}
unsafe extern "C" fn encode_mcu(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut block: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut st: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut blkn: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut tbl: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ke: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    let mut v2: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as ::core::ffi::c_uint {
            emit_restart(cinfo, (*entropy).next_restart_num);
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as ::core::ffi::c_int;
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    }
    blkn = 0 as ::core::ffi::c_int;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        ci = (*cinfo).MCU_membership[blkn as usize];
        compptr = (*cinfo).cur_comp_info[ci as usize];
        tbl = (*compptr).dc_tbl_no;
        st = (*entropy).dc_stats[tbl as usize].offset((*entropy).dc_context[ci as usize] as isize);
        v = (*block)[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            - (*entropy).last_dc_val[ci as usize];
        if v == 0 as ::core::ffi::c_int {
            arith_encode(cinfo, st, 0 as ::core::ffi::c_int);
            (*entropy).dc_context[ci as usize] = 0 as ::core::ffi::c_int;
        } else {
            (*entropy).last_dc_val[ci as usize] =
                (*block)[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
            arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
            if v > 0 as ::core::ffi::c_int {
                arith_encode(
                    cinfo,
                    st.offset(1 as ::core::ffi::c_int as isize),
                    0 as ::core::ffi::c_int,
                );
                st = st.offset(2 as ::core::ffi::c_int as isize);
                (*entropy).dc_context[ci as usize] = 4 as ::core::ffi::c_int;
            } else {
                v = -v;
                arith_encode(
                    cinfo,
                    st.offset(1 as ::core::ffi::c_int as isize),
                    1 as ::core::ffi::c_int,
                );
                st = st.offset(3 as ::core::ffi::c_int as isize);
                (*entropy).dc_context[ci as usize] = 8 as ::core::ffi::c_int;
            }
            m = 0 as ::core::ffi::c_int;
            v -= 1 as ::core::ffi::c_int;
            if v != 0 {
                arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
                m = 1 as ::core::ffi::c_int;
                v2 = v;
                st = (*entropy).dc_stats[tbl as usize].offset(20 as ::core::ffi::c_int as isize);
                loop {
                    v2 >>= 1 as ::core::ffi::c_int;
                    if !(v2 != 0) {
                        break;
                    }
                    arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
                    m <<= 1 as ::core::ffi::c_int;
                    st = st.offset(1 as ::core::ffi::c_int as isize);
                }
            }
            arith_encode(cinfo, st, 0 as ::core::ffi::c_int);
            if m < ((1 as ::core::ffi::c_long)
                << (*cinfo).arith_dc_L[tbl as usize] as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int
            {
                (*entropy).dc_context[ci as usize] = 0 as ::core::ffi::c_int;
            } else if m
                > ((1 as ::core::ffi::c_long)
                    << (*cinfo).arith_dc_U[tbl as usize] as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int
            {
                (*entropy).dc_context[ci as usize] += 8 as ::core::ffi::c_int;
            }
            st = st.offset(14 as ::core::ffi::c_int as isize);
            loop {
                m >>= 1 as ::core::ffi::c_int;
                if !(m != 0) {
                    break;
                }
                arith_encode(
                    cinfo,
                    st,
                    if m & v != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    },
                );
            }
        }
        tbl = (*compptr).ac_tbl_no;
        ke = DCTSIZE2 - 1 as ::core::ffi::c_int;
        while ke > 0 as ::core::ffi::c_int {
            if (*block)[*(&raw const jpeg_natural_order as *const ::core::ffi::c_int)
                .offset(ke as isize) as usize]
                != 0
            {
                break;
            }
            ke -= 1;
        }
        k = 1 as ::core::ffi::c_int;
        while k <= ke {
            st = (*entropy).ac_stats[tbl as usize]
                .offset((3 as ::core::ffi::c_int * (k - 1 as ::core::ffi::c_int)) as isize);
            arith_encode(cinfo, st, 0 as ::core::ffi::c_int);
            loop {
                v = (*block)[*(&raw const jpeg_natural_order as *const ::core::ffi::c_int)
                    .offset(k as isize) as usize] as ::core::ffi::c_int;
                if !(v == 0 as ::core::ffi::c_int) {
                    break;
                }
                arith_encode(
                    cinfo,
                    st.offset(1 as ::core::ffi::c_int as isize),
                    0 as ::core::ffi::c_int,
                );
                st = st.offset(3 as ::core::ffi::c_int as isize);
                k += 1;
            }
            arith_encode(
                cinfo,
                st.offset(1 as ::core::ffi::c_int as isize),
                1 as ::core::ffi::c_int,
            );
            if v > 0 as ::core::ffi::c_int {
                arith_encode(
                    cinfo,
                    &raw mut (*entropy).fixed_bin as *mut ::core::ffi::c_uchar,
                    0 as ::core::ffi::c_int,
                );
            } else {
                v = -v;
                arith_encode(
                    cinfo,
                    &raw mut (*entropy).fixed_bin as *mut ::core::ffi::c_uchar,
                    1 as ::core::ffi::c_int,
                );
            }
            st = st.offset(2 as ::core::ffi::c_int as isize);
            m = 0 as ::core::ffi::c_int;
            v -= 1 as ::core::ffi::c_int;
            if v != 0 {
                arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
                m = 1 as ::core::ffi::c_int;
                v2 = v;
                v2 >>= 1 as ::core::ffi::c_int;
                if v2 != 0 {
                    arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
                    m <<= 1 as ::core::ffi::c_int;
                    st = (*entropy).ac_stats[tbl as usize].offset(
                        (if k <= (*cinfo).arith_ac_K[tbl as usize] as ::core::ffi::c_int {
                            189 as ::core::ffi::c_int
                        } else {
                            217 as ::core::ffi::c_int
                        }) as isize,
                    );
                    loop {
                        v2 >>= 1 as ::core::ffi::c_int;
                        if !(v2 != 0) {
                            break;
                        }
                        arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
                        m <<= 1 as ::core::ffi::c_int;
                        st = st.offset(1 as ::core::ffi::c_int as isize);
                    }
                }
            }
            arith_encode(cinfo, st, 0 as ::core::ffi::c_int);
            st = st.offset(14 as ::core::ffi::c_int as isize);
            loop {
                m >>= 1 as ::core::ffi::c_int;
                if !(m != 0) {
                    break;
                }
                arith_encode(
                    cinfo,
                    st,
                    if m & v != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    },
                );
            }
            k += 1;
        }
        if k <= DCTSIZE2 - 1 as ::core::ffi::c_int {
            st = (*entropy).ac_stats[tbl as usize]
                .offset((3 as ::core::ffi::c_int * (k - 1 as ::core::ffi::c_int)) as isize);
            arith_encode(cinfo, st, 1 as ::core::ffi::c_int);
        }
        blkn += 1;
    }
    return TRUE;
}
unsafe extern "C" fn start_pass(mut cinfo: j_compress_ptr, mut gather_statistics: boolean) {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut ci: ::core::ffi::c_int = 0;
    let mut tbl: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    if gather_statistics != 0 {
        (*(*cinfo).err).msg_code = JERR_NOTIMPL as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).progressive_mode != 0 {
        if (*cinfo).Ah == 0 as ::core::ffi::c_int {
            if (*cinfo).Ss == 0 as ::core::ffi::c_int {
                (*entropy).pub_0.encode_mcu = Some(
                    encode_mcu_DC_first
                        as unsafe extern "C" fn(j_compress_ptr, *mut JBLOCKROW) -> boolean,
                )
                    as Option<unsafe extern "C" fn(j_compress_ptr, *mut JBLOCKROW) -> boolean>;
            } else {
                (*entropy).pub_0.encode_mcu = Some(
                    encode_mcu_AC_first
                        as unsafe extern "C" fn(j_compress_ptr, *mut JBLOCKROW) -> boolean,
                )
                    as Option<unsafe extern "C" fn(j_compress_ptr, *mut JBLOCKROW) -> boolean>;
            }
        } else if (*cinfo).Ss == 0 as ::core::ffi::c_int {
            (*entropy).pub_0.encode_mcu = Some(
                encode_mcu_DC_refine
                    as unsafe extern "C" fn(j_compress_ptr, *mut JBLOCKROW) -> boolean,
            )
                as Option<unsafe extern "C" fn(j_compress_ptr, *mut JBLOCKROW) -> boolean>;
        } else {
            (*entropy).pub_0.encode_mcu = Some(
                encode_mcu_AC_refine
                    as unsafe extern "C" fn(j_compress_ptr, *mut JBLOCKROW) -> boolean,
            )
                as Option<unsafe extern "C" fn(j_compress_ptr, *mut JBLOCKROW) -> boolean>;
        }
    } else {
        (*entropy).pub_0.encode_mcu =
            Some(encode_mcu as unsafe extern "C" fn(j_compress_ptr, *mut JBLOCKROW) -> boolean)
                as Option<unsafe extern "C" fn(j_compress_ptr, *mut JBLOCKROW) -> boolean>;
    }
    ci = 0 as ::core::ffi::c_int;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        if (*cinfo).progressive_mode == 0 as ::core::ffi::c_int
            || (*cinfo).Ss == 0 as ::core::ffi::c_int && (*cinfo).Ah == 0 as ::core::ffi::c_int
        {
            tbl = (*compptr).dc_tbl_no;
            if tbl < 0 as ::core::ffi::c_int || tbl >= NUM_ARITH_TBLS {
                (*(*cinfo).err).msg_code = JERR_NO_ARITH_TABLE as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = tbl;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*entropy).dc_stats[tbl as usize].is_null() {
                (*entropy).dc_stats[tbl as usize] = Some(
                    (*(*cinfo).mem)
                        .alloc_small
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    JPOOL_IMAGE,
                    DC_STAT_BINS as size_t,
                ) as *mut ::core::ffi::c_uchar;
            }
            memset(
                (*entropy).dc_stats[tbl as usize] as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                DC_STAT_BINS as size_t,
            );
            (*entropy).last_dc_val[ci as usize] = 0 as ::core::ffi::c_int;
            (*entropy).dc_context[ci as usize] = 0 as ::core::ffi::c_int;
        }
        if (*cinfo).progressive_mode == 0 as ::core::ffi::c_int || (*cinfo).Se != 0 {
            tbl = (*compptr).ac_tbl_no;
            if tbl < 0 as ::core::ffi::c_int || tbl >= NUM_ARITH_TBLS {
                (*(*cinfo).err).msg_code = JERR_NO_ARITH_TABLE as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = tbl;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*entropy).ac_stats[tbl as usize].is_null() {
                (*entropy).ac_stats[tbl as usize] = Some(
                    (*(*cinfo).mem)
                        .alloc_small
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    JPOOL_IMAGE,
                    AC_STAT_BINS as size_t,
                ) as *mut ::core::ffi::c_uchar;
            }
            memset(
                (*entropy).ac_stats[tbl as usize] as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                AC_STAT_BINS as size_t,
            );
        }
        ci += 1;
    }
    (*entropy).c = 0 as JLONG;
    (*entropy).a = 0x10000 as ::core::ffi::c_long as JLONG;
    (*entropy).sc = 0 as JLONG;
    (*entropy).zc = 0 as JLONG;
    (*entropy).ct = 11 as ::core::ffi::c_int;
    (*entropy).buffer = -(1 as ::core::ffi::c_int);
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    (*entropy).next_restart_num = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jinit_arith_encoder(mut cinfo: j_compress_ptr) {
    let mut entropy: arith_entropy_ptr = ::core::ptr::null_mut::<arith_entropy_encoder>();
    let mut i: ::core::ffi::c_int = 0;
    entropy = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<arith_entropy_encoder>() as size_t,
    ) as arith_entropy_ptr;
    (*cinfo).entropy = entropy as *mut jpeg_entropy_encoder as *mut jpeg_entropy_encoder;
    (*entropy).pub_0.start_pass =
        Some(start_pass as unsafe extern "C" fn(j_compress_ptr, boolean) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr, boolean) -> ()>;
    (*entropy).pub_0.finish_pass = Some(finish_pass as unsafe extern "C" fn(j_compress_ptr) -> ())
        as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
    i = 0 as ::core::ffi::c_int;
    while i < NUM_ARITH_TBLS {
        (*entropy).dc_stats[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        (*entropy).ac_stats[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        i += 1;
    }
    (*entropy).fixed_bin[0 as ::core::ffi::c_int as usize] = 113 as ::core::ffi::c_uchar;
}

pub const JPEG_RS_JCARITH_LINK_ANCHOR: unsafe extern "C" fn(j_compress_ptr) = jinit_arith_encoder;
