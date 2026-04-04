#[repr(C)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_entropy_encoder {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_forward_dct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_downsampler {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_color_converter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_marker_writer {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_c_coef_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_c_prep_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_c_main_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_comp_master {
    _unused: [u8; 0],
}
extern "C" {
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type JSAMPLE = ::core::ffi::c_uchar;
pub type JCOEF = ::core::ffi::c_short;
pub type JOCTET = ::core::ffi::c_uchar;
pub type UINT8 = ::core::ffi::c_uchar;
pub type UINT16 = ::core::ffi::c_ushort;
pub type JDIMENSION = ::core::ffi::c_uint;
pub type boolean = ::core::ffi::c_int;
pub type JSAMPROW = *mut JSAMPLE;
pub type JSAMPARRAY = *mut JSAMPROW;
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
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut JOCTET,
    pub free_in_buffer: size_t,
    pub init_destination: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub empty_output_buffer: Option<unsafe extern "C" fn(j_compress_ptr) -> boolean>,
    pub term_destination: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
}
pub type j_compress_ptr = *mut jpeg_compress_struct;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const JMSG_LASTADDONCODE: C2RustUnnamed_1 = 1047;
pub const JERR_UNSUPPORTED_FORMAT: C2RustUnnamed_1 = 1046;
pub const JERR_UNKNOWN_FORMAT: C2RustUnnamed_1 = 1045;
pub const JERR_UNGETC_FAILED: C2RustUnnamed_1 = 1044;
pub const JERR_TOO_MANY_COLORS: C2RustUnnamed_1 = 1043;
pub const JERR_BAD_CMAP_FILE: C2RustUnnamed_1 = 1042;
pub const JTRC_TGA_MAPPED: C2RustUnnamed_1 = 1041;
pub const JTRC_TGA_GRAY: C2RustUnnamed_1 = 1040;
pub const JTRC_TGA: C2RustUnnamed_1 = 1039;
pub const JERR_TGA_COLORSPACE: C2RustUnnamed_1 = 1038;
pub const JERR_TGA_BADPARMS: C2RustUnnamed_1 = 1037;
pub const JERR_TGA_BADCMAP: C2RustUnnamed_1 = 1036;
pub const JTRC_PPM_TEXT: C2RustUnnamed_1 = 1035;
pub const JTRC_PPM: C2RustUnnamed_1 = 1034;
pub const JTRC_PGM_TEXT: C2RustUnnamed_1 = 1033;
pub const JTRC_PGM: C2RustUnnamed_1 = 1032;
pub const JERR_PPM_OUTOFRANGE: C2RustUnnamed_1 = 1031;
pub const JERR_PPM_NOT: C2RustUnnamed_1 = 1030;
pub const JERR_PPM_NONNUMERIC: C2RustUnnamed_1 = 1029;
pub const JERR_PPM_COLORSPACE: C2RustUnnamed_1 = 1028;
pub const JWRN_GIF_NOMOREDATA: C2RustUnnamed_1 = 1027;
pub const JWRN_GIF_ENDCODE: C2RustUnnamed_1 = 1026;
pub const JWRN_GIF_CHAR: C2RustUnnamed_1 = 1025;
pub const JWRN_GIF_BADDATA: C2RustUnnamed_1 = 1024;
pub const JTRC_GIF_NONSQUARE: C2RustUnnamed_1 = 1023;
pub const JTRC_GIF_EXTENSION: C2RustUnnamed_1 = 1022;
pub const JTRC_GIF_BADVERSION: C2RustUnnamed_1 = 1021;
pub const JTRC_GIF: C2RustUnnamed_1 = 1020;
pub const JERR_GIF_NOT: C2RustUnnamed_1 = 1019;
pub const JERR_GIF_IMAGENOTFOUND: C2RustUnnamed_1 = 1018;
pub const JERR_GIF_EMPTY: C2RustUnnamed_1 = 1017;
pub const JERR_GIF_COLORSPACE: C2RustUnnamed_1 = 1016;
pub const JERR_GIF_CODESIZE: C2RustUnnamed_1 = 1015;
pub const JERR_GIF_BUG: C2RustUnnamed_1 = 1014;
pub const JTRC_BMP_OS2_MAPPED: C2RustUnnamed_1 = 1013;
pub const JTRC_BMP_OS2: C2RustUnnamed_1 = 1012;
pub const JTRC_BMP_MAPPED: C2RustUnnamed_1 = 1011;
pub const JTRC_BMP: C2RustUnnamed_1 = 1010;
pub const JERR_BMP_OUTOFRANGE: C2RustUnnamed_1 = 1009;
pub const JERR_BMP_NOT: C2RustUnnamed_1 = 1008;
pub const JERR_BMP_EMPTY: C2RustUnnamed_1 = 1007;
pub const JERR_BMP_COMPRESSED: C2RustUnnamed_1 = 1006;
pub const JERR_BMP_COLORSPACE: C2RustUnnamed_1 = 1005;
pub const JERR_BMP_BADPLANES: C2RustUnnamed_1 = 1004;
pub const JERR_BMP_BADHEADER: C2RustUnnamed_1 = 1003;
pub const JERR_BMP_BADDEPTH: C2RustUnnamed_1 = 1002;
pub const JERR_BMP_BADCMAP: C2RustUnnamed_1 = 1001;
pub const JMSG_FIRSTADDONCODE: C2RustUnnamed_1 = 1000;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cjpeg_source_struct {
    pub start_input: Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ()>,
    pub get_pixel_rows:
        Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>,
    pub finish_input: Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ()>,
    pub input_file: *mut FILE,
    pub buffer: JSAMPARRAY,
    pub buffer_height: JDIMENSION,
}
pub type cjpeg_source_ptr = *mut cjpeg_source_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdjpeg_progress_mgr {
    pub pub_0: jpeg_progress_mgr,
    pub completed_extra_passes: ::core::ffi::c_int,
    pub total_extra_passes: ::core::ffi::c_int,
    pub max_scans: JDIMENSION,
    pub report: boolean,
    pub percent_done: ::core::ffi::c_int,
}
pub type cd_progress_ptr = *mut cdjpeg_progress_mgr;
pub type gif_source_ptr = *mut gif_source_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gif_source_struct {
    pub pub_0: cjpeg_source_struct,
    pub cinfo: j_compress_ptr,
    pub colormap: JSAMPARRAY,
    pub code_buf: [U_CHAR; 260],
    pub last_byte: ::core::ffi::c_int,
    pub last_bit: ::core::ffi::c_int,
    pub cur_bit: ::core::ffi::c_int,
    pub first_time: boolean,
    pub out_of_blocks: boolean,
    pub input_code_size: ::core::ffi::c_int,
    pub clear_code: ::core::ffi::c_int,
    pub end_code: ::core::ffi::c_int,
    pub code_size: ::core::ffi::c_int,
    pub limit_code: ::core::ffi::c_int,
    pub max_code: ::core::ffi::c_int,
    pub oldcode: ::core::ffi::c_int,
    pub firstcode: ::core::ffi::c_int,
    pub symbol_head: *mut UINT16,
    pub symbol_tail: *mut UINT8,
    pub symbol_stack: *mut UINT8,
    pub sp: *mut UINT8,
    pub is_interlaced: boolean,
    pub interlaced_image: jvirt_sarray_ptr,
    pub cur_row_number: JDIMENSION,
    pub pass2_offset: JDIMENSION,
    pub pass3_offset: JDIMENSION,
    pub pass4_offset: JDIMENSION,
}
pub type U_CHAR = ::core::ffi::c_uchar;
pub const BITS_IN_JSAMPLE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const CENTERJSAMPLE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAXCOLORMAPSIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NUMCOLORS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CM_RED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CM_GREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CM_BLUE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAX_LZW_BITS: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const LZW_TABLE_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << MAX_LZW_BITS;
unsafe extern "C" fn ReadByte(mut sinfo: gif_source_ptr) -> ::core::ffi::c_int {
    let mut infile: *mut FILE = (*sinfo).pub_0.input_file;
    let mut c: ::core::ffi::c_int = 0;
    c = getc(infile);
    if c == EOF {
        (*(*(*sinfo).cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
        Some(
            (*(*(*sinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")((*sinfo).cinfo as j_common_ptr);
    }
    return c;
}
unsafe extern "C" fn GetDataBlock(
    mut sinfo: gif_source_ptr,
    mut buf: *mut U_CHAR,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0;
    count = ReadByte(sinfo);
    if count > 0 as ::core::ffi::c_int {
        if !(fread(
            buf as *mut ::core::ffi::c_void,
            1 as size_t,
            count as size_t,
            (*sinfo).pub_0.input_file,
        ) as size_t
            == count as size_t)
        {
            (*(*(*sinfo).cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
            Some(
                (*(*(*sinfo).cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")((*sinfo).cinfo as j_common_ptr);
        }
    }
    return count;
}
unsafe extern "C" fn SkipDataBlocks(mut sinfo: gif_source_ptr) {
    let mut buf: [U_CHAR; 256] = [0; 256];
    while GetDataBlock(sinfo, &raw mut buf as *mut U_CHAR) > 0 as ::core::ffi::c_int {}
}
unsafe extern "C" fn ReInitLZW(mut sinfo: gif_source_ptr) {
    (*sinfo).code_size = (*sinfo).input_code_size + 1 as ::core::ffi::c_int;
    (*sinfo).limit_code = (*sinfo).clear_code << 1 as ::core::ffi::c_int;
    (*sinfo).max_code = (*sinfo).clear_code + 2 as ::core::ffi::c_int;
    (*sinfo).sp = (*sinfo).symbol_stack;
}
unsafe extern "C" fn InitLZWCode(mut sinfo: gif_source_ptr) {
    (*sinfo).last_byte = 2 as ::core::ffi::c_int;
    (*sinfo).code_buf[0 as ::core::ffi::c_int as usize] = 0 as U_CHAR;
    (*sinfo).code_buf[1 as ::core::ffi::c_int as usize] = 0 as U_CHAR;
    (*sinfo).last_bit = 0 as ::core::ffi::c_int;
    (*sinfo).cur_bit = 0 as ::core::ffi::c_int;
    (*sinfo).first_time = TRUE as boolean;
    (*sinfo).out_of_blocks = FALSE as boolean;
    (*sinfo).clear_code = (1 as ::core::ffi::c_int) << (*sinfo).input_code_size;
    (*sinfo).end_code = (*sinfo).clear_code + 1 as ::core::ffi::c_int;
    ReInitLZW(sinfo);
}
unsafe extern "C" fn GetCode(mut sinfo: gif_source_ptr) -> ::core::ffi::c_int {
    let mut accum: ::core::ffi::c_int = 0;
    let mut offs: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    while (*sinfo).cur_bit + (*sinfo).code_size > (*sinfo).last_bit {
        if (*sinfo).first_time != 0 {
            (*sinfo).first_time = FALSE as boolean;
            return (*sinfo).clear_code;
        }
        if (*sinfo).out_of_blocks != 0 {
            (*(*(*sinfo).cinfo).err).msg_code = JWRN_GIF_NOMOREDATA as ::core::ffi::c_int;
            Some(
                (*(*(*sinfo).cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*sinfo).cinfo as j_common_ptr,
                -(1 as ::core::ffi::c_int),
            );
            return (*sinfo).end_code;
        }
        (*sinfo).code_buf[0 as ::core::ffi::c_int as usize] =
            (*sinfo).code_buf[((*sinfo).last_byte - 2 as ::core::ffi::c_int) as usize];
        (*sinfo).code_buf[1 as ::core::ffi::c_int as usize] =
            (*sinfo).code_buf[((*sinfo).last_byte - 1 as ::core::ffi::c_int) as usize];
        count = GetDataBlock(
            sinfo,
            (&raw mut (*sinfo).code_buf as *mut U_CHAR).offset(2 as ::core::ffi::c_int as isize)
                as *mut U_CHAR,
        );
        if count == 0 as ::core::ffi::c_int {
            (*sinfo).out_of_blocks = TRUE as boolean;
            (*(*(*sinfo).cinfo).err).msg_code = JWRN_GIF_NOMOREDATA as ::core::ffi::c_int;
            Some(
                (*(*(*sinfo).cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*sinfo).cinfo as j_common_ptr,
                -(1 as ::core::ffi::c_int),
            );
            return (*sinfo).end_code;
        }
        (*sinfo).cur_bit = (*sinfo).cur_bit - (*sinfo).last_bit + 16 as ::core::ffi::c_int;
        (*sinfo).last_byte = 2 as ::core::ffi::c_int + count;
        (*sinfo).last_bit = (*sinfo).last_byte * 8 as ::core::ffi::c_int;
    }
    offs = (*sinfo).cur_bit >> 3 as ::core::ffi::c_int;
    accum = (*sinfo).code_buf[(offs + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
    accum <<= 8 as ::core::ffi::c_int;
    accum |= (*sinfo).code_buf[(offs + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
    accum <<= 8 as ::core::ffi::c_int;
    accum |= (*sinfo).code_buf[offs as usize] as ::core::ffi::c_int;
    accum >>= (*sinfo).cur_bit & 7 as ::core::ffi::c_int;
    (*sinfo).cur_bit += (*sinfo).code_size;
    return accum & ((1 as ::core::ffi::c_int) << (*sinfo).code_size) - 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn LZWReadByte(mut sinfo: gif_source_ptr) -> ::core::ffi::c_int {
    let mut code: ::core::ffi::c_int = 0;
    let mut incode: ::core::ffi::c_int = 0;
    if (*sinfo).sp > (*sinfo).symbol_stack {
        (*sinfo).sp = (*sinfo).sp.offset(-1);
        return *(*sinfo).sp as ::core::ffi::c_int;
    }
    code = GetCode(sinfo);
    if code == (*sinfo).clear_code {
        ReInitLZW(sinfo);
        loop {
            code = GetCode(sinfo);
            if !(code == (*sinfo).clear_code) {
                break;
            }
        }
        if code > (*sinfo).clear_code {
            (*(*(*sinfo).cinfo).err).msg_code = JWRN_GIF_BADDATA as ::core::ffi::c_int;
            Some(
                (*(*(*sinfo).cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*sinfo).cinfo as j_common_ptr,
                -(1 as ::core::ffi::c_int),
            );
            code = 0 as ::core::ffi::c_int;
        }
        (*sinfo).oldcode = code;
        (*sinfo).firstcode = (*sinfo).oldcode;
        return code;
    }
    if code == (*sinfo).end_code {
        if (*sinfo).out_of_blocks == 0 {
            SkipDataBlocks(sinfo);
            (*sinfo).out_of_blocks = TRUE as boolean;
        }
        (*(*(*sinfo).cinfo).err).msg_code = JWRN_GIF_ENDCODE as ::core::ffi::c_int;
        Some(
            (*(*(*sinfo).cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*sinfo).cinfo as j_common_ptr,
            -(1 as ::core::ffi::c_int),
        );
        return 0 as ::core::ffi::c_int;
    }
    incode = code;
    if code >= (*sinfo).max_code {
        if code > (*sinfo).max_code {
            (*(*(*sinfo).cinfo).err).msg_code = JWRN_GIF_BADDATA as ::core::ffi::c_int;
            Some(
                (*(*(*sinfo).cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*sinfo).cinfo as j_common_ptr,
                -(1 as ::core::ffi::c_int),
            );
            incode = 0 as ::core::ffi::c_int;
        }
        let fresh6 = (*sinfo).sp;
        (*sinfo).sp = (*sinfo).sp.offset(1);
        *fresh6 = (*sinfo).firstcode as UINT8;
        code = (*sinfo).oldcode;
    }
    while code >= (*sinfo).clear_code {
        let fresh7 = (*sinfo).sp;
        (*sinfo).sp = (*sinfo).sp.offset(1);
        *fresh7 = *(*sinfo).symbol_tail.offset(code as isize);
        code = *(*sinfo).symbol_head.offset(code as isize) as ::core::ffi::c_int;
    }
    (*sinfo).firstcode = code;
    code = (*sinfo).max_code;
    if code < LZW_TABLE_SIZE {
        *(*sinfo).symbol_head.offset(code as isize) = (*sinfo).oldcode as UINT16;
        *(*sinfo).symbol_tail.offset(code as isize) = (*sinfo).firstcode as UINT8;
        (*sinfo).max_code += 1;
        if (*sinfo).max_code >= (*sinfo).limit_code && (*sinfo).code_size < MAX_LZW_BITS {
            (*sinfo).code_size += 1;
            (*sinfo).limit_code <<= 1 as ::core::ffi::c_int;
        }
    }
    (*sinfo).oldcode = incode;
    return (*sinfo).firstcode;
}
unsafe extern "C" fn ReadColorMap(
    mut sinfo: gif_source_ptr,
    mut cmaplen: ::core::ffi::c_int,
    mut cmap: JSAMPARRAY,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut gray: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < cmaplen {
        *(*cmap.offset(CM_RED as isize)).offset(i as isize) = ReadByte(sinfo) as JSAMPLE;
        *(*cmap.offset(CM_GREEN as isize)).offset(i as isize) = ReadByte(sinfo) as JSAMPLE;
        *(*cmap.offset(CM_BLUE as isize)).offset(i as isize) = ReadByte(sinfo) as JSAMPLE;
        if *(*cmap.offset(CM_RED as isize)).offset(i as isize) as ::core::ffi::c_int
            != *(*cmap.offset(CM_GREEN as isize)).offset(i as isize) as ::core::ffi::c_int
            || *(*cmap.offset(CM_GREEN as isize)).offset(i as isize) as ::core::ffi::c_int
                != *(*cmap.offset(CM_BLUE as isize)).offset(i as isize) as ::core::ffi::c_int
        {
            gray = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if (*(*sinfo).cinfo).in_color_space as ::core::ffi::c_uint
        == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
        && gray != 0
    {
        (*(*sinfo).cinfo).in_color_space = JCS_GRAYSCALE;
        (*(*sinfo).cinfo).input_components = 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn DoExtension(mut sinfo: gif_source_ptr) {
    let mut extlabel: ::core::ffi::c_int = 0;
    extlabel = ReadByte(sinfo);
    (*(*(*sinfo).cinfo).err).msg_code = JTRC_GIF_EXTENSION as ::core::ffi::c_int;
    (*(*(*sinfo).cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = extlabel;
    Some(
        (*(*(*sinfo).cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        (*sinfo).cinfo as j_common_ptr, 1 as ::core::ffi::c_int
    );
    SkipDataBlocks(sinfo);
}
unsafe extern "C" fn start_input_gif(mut cinfo: j_compress_ptr, mut sinfo: cjpeg_source_ptr) {
    let mut source: gif_source_ptr = sinfo as gif_source_ptr;
    let mut hdrbuf: [U_CHAR; 10] = [0; 10];
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut colormaplen: ::core::ffi::c_int = 0;
    let mut aspectRatio: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    if !(fread(
        &raw mut hdrbuf as *mut U_CHAR as *mut ::core::ffi::c_void,
        1 as size_t,
        6 as size_t,
        (*source).pub_0.input_file,
    ) as size_t
        == 6 as ::core::ffi::c_int as size_t)
    {
        (*(*cinfo).err).msg_code = JERR_GIF_NOT as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if hdrbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 'G' as i32
        || hdrbuf[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 'I' as i32
        || hdrbuf[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 'F' as i32
    {
        (*(*cinfo).err).msg_code = JERR_GIF_NOT as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (hdrbuf[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '8' as i32
        || hdrbuf[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '7' as i32
        || hdrbuf[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 'a' as i32)
        && (hdrbuf[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '8' as i32
            || hdrbuf[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '9' as i32
            || hdrbuf[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 'a' as i32)
    {
        let mut _mp: *mut ::core::ffi::c_int =
            &raw mut (*(*cinfo).err).msg_parm.i as *mut ::core::ffi::c_int;
        *_mp.offset(0 as ::core::ffi::c_int as isize) =
            hdrbuf[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp.offset(1 as ::core::ffi::c_int as isize) =
            hdrbuf[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        *_mp.offset(2 as ::core::ffi::c_int as isize) =
            hdrbuf[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        (*(*cinfo).err).msg_code = JTRC_GIF_BADVERSION as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    }
    if !(fread(
        &raw mut hdrbuf as *mut U_CHAR as *mut ::core::ffi::c_void,
        1 as size_t,
        7 as size_t,
        (*source).pub_0.input_file,
    ) as size_t
        == 7 as ::core::ffi::c_int as size_t)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    width = (hdrbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_add(
            (hdrbuf[(0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_int,
        );
    height = (hdrbuf[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        as ::core::ffi::c_uint)
        .wrapping_add(
            (hdrbuf[(2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_int,
        );
    if width == 0 as ::core::ffi::c_uint || height == 0 as ::core::ffi::c_uint {
        (*(*cinfo).err).msg_code = JERR_GIF_EMPTY as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    aspectRatio = hdrbuf[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    if aspectRatio != 0 as ::core::ffi::c_int && aspectRatio != 49 as ::core::ffi::c_int {
        (*(*cinfo).err).msg_code = JTRC_GIF_NONSQUARE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
    }
    (*source).colormap = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        MAXCOLORMAPSIZE as JDIMENSION,
        NUMCOLORS as JDIMENSION,
    );
    colormaplen = 0 as ::core::ffi::c_int;
    if hdrbuf[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
        != 0
    {
        colormaplen = (2 as ::core::ffi::c_int)
            << (hdrbuf[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & 0x7 as ::core::ffi::c_int);
        ReadColorMap(source, colormaplen, (*source).colormap);
    }
    loop {
        c = ReadByte(source);
        if c == ';' as i32 {
            (*(*cinfo).err).msg_code = JERR_GIF_IMAGENOTFOUND as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if c == '!' as i32 {
            DoExtension(source);
        } else if c != ',' as i32 {
            (*(*cinfo).err).msg_code = JWRN_GIF_CHAR as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = c;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, -(1 as ::core::ffi::c_int)
            );
        } else {
            if !(fread(
                &raw mut hdrbuf as *mut U_CHAR as *mut ::core::ffi::c_void,
                1 as size_t,
                9 as size_t,
                (*source).pub_0.input_file,
            ) as size_t
                == 9 as ::core::ffi::c_int as size_t)
            {
                (*(*cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            width = (hdrbuf[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_add(
                    (hdrbuf[(4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
            height = (hdrbuf[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_add(
                    (hdrbuf[(6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
            if width == 0 as ::core::ffi::c_uint || height == 0 as ::core::ffi::c_uint {
                (*(*cinfo).err).msg_code = JERR_GIF_EMPTY as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*source).is_interlaced =
                (hdrbuf[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    & 0x40 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as boolean;
            if hdrbuf[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                colormaplen = (2 as ::core::ffi::c_int)
                    << (hdrbuf[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        & 0x7 as ::core::ffi::c_int);
                ReadColorMap(source, colormaplen, (*source).colormap);
            }
            (*source).input_code_size = ReadByte(source);
            if (*source).input_code_size < 2 as ::core::ffi::c_int
                || (*source).input_code_size > 8 as ::core::ffi::c_int
            {
                (*(*cinfo).err).msg_code = JERR_GIF_CODESIZE as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                    (*source).input_code_size;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            break;
        }
    }
    (*source).symbol_head = Some(
        (*(*cinfo).mem)
            .alloc_large
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (LZW_TABLE_SIZE as size_t).wrapping_mul(::core::mem::size_of::<UINT16>() as size_t),
    ) as *mut UINT16;
    (*source).symbol_tail = Some(
        (*(*cinfo).mem)
            .alloc_large
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (LZW_TABLE_SIZE as size_t).wrapping_mul(::core::mem::size_of::<UINT8>() as size_t),
    ) as *mut UINT8;
    (*source).symbol_stack = Some(
        (*(*cinfo).mem)
            .alloc_large
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (LZW_TABLE_SIZE as size_t).wrapping_mul(::core::mem::size_of::<UINT8>() as size_t),
    ) as *mut UINT8;
    InitLZWCode(source);
    if (*source).is_interlaced != 0 {
        (*source).interlaced_image = Some(
            (*(*cinfo).mem)
                .request_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            FALSE,
            width,
            height,
            1 as ::core::ffi::c_int as JDIMENSION,
        );
        if !(*cinfo).progress.is_null() {
            let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
            (*progress).total_extra_passes += 1;
        }
        (*source).pub_0.get_pixel_rows = Some(
            load_interlaced_image
                as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
        )
            as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
    } else {
        (*source).pub_0.get_pixel_rows = Some(
            get_pixel_rows as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
        )
            as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
    }
    if (*cinfo).in_color_space as ::core::ffi::c_uint
        != JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cinfo).in_color_space = JCS_RGB;
        (*cinfo).input_components = NUMCOLORS;
    }
    (*source).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        width.wrapping_mul((*cinfo).input_components as JDIMENSION),
        1 as ::core::ffi::c_int as JDIMENSION,
    );
    (*source).pub_0.buffer_height = 1 as JDIMENSION;
    c = colormaplen;
    while c < (*source).clear_code {
        let ref mut fresh0 = *(*(*source).colormap.offset(CM_BLUE as isize)).offset(c as isize);
        *fresh0 = CENTERJSAMPLE as JSAMPLE;
        let ref mut fresh1 = *(*(*source).colormap.offset(CM_GREEN as isize)).offset(c as isize);
        *fresh1 = *fresh0;
        *(*(*source).colormap.offset(CM_RED as isize)).offset(c as isize) = *fresh1;
        c += 1;
    }
    (*cinfo).data_precision = BITS_IN_JSAMPLE;
    (*cinfo).image_width = width as JDIMENSION;
    (*cinfo).image_height = height as JDIMENSION;
    let mut _mp_0: *mut ::core::ffi::c_int =
        &raw mut (*(*cinfo).err).msg_parm.i as *mut ::core::ffi::c_int;
    *_mp_0.offset(0 as ::core::ffi::c_int as isize) = width as ::core::ffi::c_int;
    *_mp_0.offset(1 as ::core::ffi::c_int as isize) = height as ::core::ffi::c_int;
    *_mp_0.offset(2 as ::core::ffi::c_int as isize) = colormaplen;
    (*(*cinfo).err).msg_code = JTRC_GIF as ::core::ffi::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as ::core::ffi::c_int);
}
unsafe extern "C" fn get_pixel_rows(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: gif_source_ptr = sinfo as gif_source_ptr;
    let mut c: ::core::ffi::c_int = 0;
    let mut ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut colormap: JSAMPARRAY = (*source).colormap;
    ptr = *(*source)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    if (*cinfo).in_color_space as ::core::ffi::c_uint
        == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        col = (*cinfo).image_width;
        while col > 0 as JDIMENSION {
            c = LZWReadByte(source);
            let fresh2 = ptr;
            ptr = ptr.offset(1);
            *fresh2 = *(*colormap.offset(CM_RED as isize)).offset(c as isize);
            col = col.wrapping_sub(1);
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0 as JDIMENSION {
            c = LZWReadByte(source);
            let fresh3 = ptr;
            ptr = ptr.offset(1);
            *fresh3 = *(*colormap.offset(CM_RED as isize)).offset(c as isize);
            let fresh4 = ptr;
            ptr = ptr.offset(1);
            *fresh4 = *(*colormap.offset(CM_GREEN as isize)).offset(c as isize);
            let fresh5 = ptr;
            ptr = ptr.offset(1);
            *fresh5 = *(*colormap.offset(CM_BLUE as isize)).offset(c as isize);
            col = col.wrapping_sub(1);
        }
    }
    return 1 as JDIMENSION;
}
unsafe extern "C" fn load_interlaced_image(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: gif_source_ptr = sinfo as gif_source_ptr;
    let mut sptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut row: JDIMENSION = 0;
    let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
    row = 0 as JDIMENSION;
    while row < (*cinfo).image_height {
        if !progress.is_null() {
            (*progress).pub_0.pass_counter = row as ::core::ffi::c_long;
            (*progress).pub_0.pass_limit = (*cinfo).image_height as ::core::ffi::c_long;
            Some(
                (*progress)
                    .pub_0
                    .progress_monitor
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        sptr = *Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).interlaced_image,
            row,
            1 as ::core::ffi::c_int as JDIMENSION,
            TRUE,
        );
        col = (*cinfo).image_width;
        while col > 0 as JDIMENSION {
            let fresh8 = sptr;
            sptr = sptr.offset(1);
            *fresh8 = LZWReadByte(source) as JSAMPLE;
            col = col.wrapping_sub(1);
        }
        row = row.wrapping_add(1);
    }
    if !progress.is_null() {
        (*progress).completed_extra_passes += 1;
    }
    (*source).pub_0.get_pixel_rows = Some(
        get_interlaced_row as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
    )
        as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
    (*source).cur_row_number = 0 as JDIMENSION;
    (*source).pass2_offset = (*cinfo)
        .image_height
        .wrapping_add(7 as JDIMENSION)
        .wrapping_div(8 as JDIMENSION);
    (*source).pass3_offset = (*source).pass2_offset.wrapping_add(
        (*cinfo)
            .image_height
            .wrapping_add(3 as JDIMENSION)
            .wrapping_div(8 as JDIMENSION),
    );
    (*source).pass4_offset = (*source).pass3_offset.wrapping_add(
        (*cinfo)
            .image_height
            .wrapping_add(1 as JDIMENSION)
            .wrapping_div(4 as JDIMENSION),
    );
    return get_interlaced_row(cinfo, sinfo);
}
unsafe extern "C" fn get_interlaced_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: gif_source_ptr = sinfo as gif_source_ptr;
    let mut c: ::core::ffi::c_int = 0;
    let mut sptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut colormap: JSAMPARRAY = (*source).colormap;
    let mut irow: JDIMENSION = 0;
    match ((*source).cur_row_number & 7 as JDIMENSION) as ::core::ffi::c_int {
        0 => {
            irow = (*source).cur_row_number >> 3 as ::core::ffi::c_int;
        }
        4 => {
            irow = ((*source).cur_row_number >> 3 as ::core::ffi::c_int)
                .wrapping_add((*source).pass2_offset);
        }
        2 | 6 => {
            irow = ((*source).cur_row_number >> 2 as ::core::ffi::c_int)
                .wrapping_add((*source).pass3_offset);
        }
        _ => {
            irow = ((*source).cur_row_number >> 1 as ::core::ffi::c_int)
                .wrapping_add((*source).pass4_offset);
        }
    }
    sptr = *Some(
        (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        (*source).interlaced_image,
        irow,
        1 as ::core::ffi::c_int as JDIMENSION,
        FALSE,
    );
    ptr = *(*source)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    if (*cinfo).in_color_space as ::core::ffi::c_uint
        == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        col = (*cinfo).image_width;
        while col > 0 as JDIMENSION {
            let fresh9 = sptr;
            sptr = sptr.offset(1);
            c = *fresh9 as ::core::ffi::c_int;
            let fresh10 = ptr;
            ptr = ptr.offset(1);
            *fresh10 = *(*colormap.offset(CM_RED as isize)).offset(c as isize);
            col = col.wrapping_sub(1);
        }
    } else {
        col = (*cinfo).image_width;
        while col > 0 as JDIMENSION {
            let fresh11 = sptr;
            sptr = sptr.offset(1);
            c = *fresh11 as ::core::ffi::c_int;
            let fresh12 = ptr;
            ptr = ptr.offset(1);
            *fresh12 = *(*colormap.offset(CM_RED as isize)).offset(c as isize);
            let fresh13 = ptr;
            ptr = ptr.offset(1);
            *fresh13 = *(*colormap.offset(CM_GREEN as isize)).offset(c as isize);
            let fresh14 = ptr;
            ptr = ptr.offset(1);
            *fresh14 = *(*colormap.offset(CM_BLUE as isize)).offset(c as isize);
            col = col.wrapping_sub(1);
        }
    }
    (*source).cur_row_number = (*source).cur_row_number.wrapping_add(1);
    return 1 as JDIMENSION;
}
unsafe extern "C" fn finish_input_gif(mut cinfo: j_compress_ptr, mut sinfo: cjpeg_source_ptr) {}
#[no_mangle]
pub unsafe extern "C" fn jinit_read_gif(mut cinfo: j_compress_ptr) -> cjpeg_source_ptr {
    let mut source: gif_source_ptr = ::core::ptr::null_mut::<gif_source_struct>();
    source = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<gif_source_struct>() as size_t,
    ) as gif_source_ptr;
    (*source).cinfo = cinfo;
    (*source).pub_0.start_input =
        Some(start_input_gif as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ()>;
    (*source).pub_0.finish_input =
        Some(finish_input_gif as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ()>;
    return source as cjpeg_source_ptr;
}
