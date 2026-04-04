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
pub type tga_source_ptr = *mut _tga_source_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tga_source_struct {
    pub pub_0: cjpeg_source_struct,
    pub cinfo: j_compress_ptr,
    pub colormap: JSAMPARRAY,
    pub whole_image: jvirt_sarray_ptr,
    pub current_row: JDIMENSION,
    pub read_pixel: Option<unsafe extern "C" fn(tga_source_ptr) -> ()>,
    pub tga_pixel: [U_CHAR; 4],
    pub pixel_size: ::core::ffi::c_int,
    pub cmap_length: ::core::ffi::c_int,
    pub block_count: ::core::ffi::c_int,
    pub dup_pixel_count: ::core::ffi::c_int,
    pub get_pixel_rows:
        Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>,
}
pub type U_CHAR = ::core::ffi::c_uchar;
pub type tga_source_struct = _tga_source_struct;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut c5to8bits: [UINT8; 32] = [
    0 as ::core::ffi::c_int as UINT8,
    8 as ::core::ffi::c_int as UINT8,
    16 as ::core::ffi::c_int as UINT8,
    25 as ::core::ffi::c_int as UINT8,
    33 as ::core::ffi::c_int as UINT8,
    41 as ::core::ffi::c_int as UINT8,
    49 as ::core::ffi::c_int as UINT8,
    58 as ::core::ffi::c_int as UINT8,
    66 as ::core::ffi::c_int as UINT8,
    74 as ::core::ffi::c_int as UINT8,
    82 as ::core::ffi::c_int as UINT8,
    90 as ::core::ffi::c_int as UINT8,
    99 as ::core::ffi::c_int as UINT8,
    107 as ::core::ffi::c_int as UINT8,
    115 as ::core::ffi::c_int as UINT8,
    123 as ::core::ffi::c_int as UINT8,
    132 as ::core::ffi::c_int as UINT8,
    140 as ::core::ffi::c_int as UINT8,
    148 as ::core::ffi::c_int as UINT8,
    156 as ::core::ffi::c_int as UINT8,
    165 as ::core::ffi::c_int as UINT8,
    173 as ::core::ffi::c_int as UINT8,
    181 as ::core::ffi::c_int as UINT8,
    189 as ::core::ffi::c_int as UINT8,
    197 as ::core::ffi::c_int as UINT8,
    206 as ::core::ffi::c_int as UINT8,
    214 as ::core::ffi::c_int as UINT8,
    222 as ::core::ffi::c_int as UINT8,
    230 as ::core::ffi::c_int as UINT8,
    239 as ::core::ffi::c_int as UINT8,
    247 as ::core::ffi::c_int as UINT8,
    255 as ::core::ffi::c_int as UINT8,
];
unsafe extern "C" fn read_byte(mut sinfo: tga_source_ptr) -> ::core::ffi::c_int {
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
unsafe extern "C" fn read_colormap(
    mut sinfo: tga_source_ptr,
    mut cmaplen: ::core::ffi::c_int,
    mut mapentrysize: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    if mapentrysize != 24 as ::core::ffi::c_int {
        (*(*(*sinfo).cinfo).err).msg_code = JERR_TGA_BADCMAP as ::core::ffi::c_int;
        Some(
            (*(*(*sinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")((*sinfo).cinfo as j_common_ptr);
    }
    i = 0 as ::core::ffi::c_int;
    while i < cmaplen {
        *(*(*sinfo).colormap.offset(2 as ::core::ffi::c_int as isize)).offset(i as isize) =
            read_byte(sinfo) as JSAMPLE;
        *(*(*sinfo).colormap.offset(1 as ::core::ffi::c_int as isize)).offset(i as isize) =
            read_byte(sinfo) as JSAMPLE;
        *(*(*sinfo).colormap.offset(0 as ::core::ffi::c_int as isize)).offset(i as isize) =
            read_byte(sinfo) as JSAMPLE;
        i += 1;
    }
}
unsafe extern "C" fn read_non_rle_pixel(mut sinfo: tga_source_ptr) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*sinfo).pixel_size {
        (*sinfo).tga_pixel[i as usize] = read_byte(sinfo) as U_CHAR;
        i += 1;
    }
}
unsafe extern "C" fn read_rle_pixel(mut sinfo: tga_source_ptr) {
    let mut i: ::core::ffi::c_int = 0;
    if (*sinfo).dup_pixel_count > 0 as ::core::ffi::c_int {
        (*sinfo).dup_pixel_count -= 1;
        return;
    }
    (*sinfo).block_count -= 1;
    if (*sinfo).block_count < 0 as ::core::ffi::c_int {
        i = read_byte(sinfo);
        if i & 0x80 as ::core::ffi::c_int != 0 {
            (*sinfo).dup_pixel_count = i & 0x7f as ::core::ffi::c_int;
            (*sinfo).block_count = 0 as ::core::ffi::c_int;
        } else {
            (*sinfo).block_count = i & 0x7f as ::core::ffi::c_int;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*sinfo).pixel_size {
        (*sinfo).tga_pixel[i as usize] = read_byte(sinfo) as U_CHAR;
        i += 1;
    }
}
unsafe extern "C" fn get_8bit_gray_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    ptr = *(*source)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).image_width;
    while col > 0 as JDIMENSION {
        Some((*source).read_pixel.expect("non-null function pointer"))
            .expect("non-null function pointer")(source);
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        *fresh1 =
            (*source).tga_pixel[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as JSAMPLE;
        col = col.wrapping_sub(1);
    }
    return 1 as JDIMENSION;
}
unsafe extern "C" fn get_8bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut t: ::core::ffi::c_int = 0;
    let mut ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut colormap: JSAMPARRAY = (*source).colormap;
    let mut cmaplen: ::core::ffi::c_int = (*source).cmap_length;
    ptr = *(*source)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).image_width;
    while col > 0 as JDIMENSION {
        Some((*source).read_pixel.expect("non-null function pointer"))
            .expect("non-null function pointer")(source);
        t = (*source).tga_pixel[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        if t >= cmaplen {
            (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        let fresh5 = ptr;
        ptr = ptr.offset(1);
        *fresh5 = *(*colormap.offset(0 as ::core::ffi::c_int as isize)).offset(t as isize);
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        *fresh6 = *(*colormap.offset(1 as ::core::ffi::c_int as isize)).offset(t as isize);
        let fresh7 = ptr;
        ptr = ptr.offset(1);
        *fresh7 = *(*colormap.offset(2 as ::core::ffi::c_int as isize)).offset(t as isize);
        col = col.wrapping_sub(1);
    }
    return 1 as JDIMENSION;
}
unsafe extern "C" fn get_16bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut t: ::core::ffi::c_int = 0;
    let mut ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    ptr = *(*source)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).image_width;
    while col > 0 as JDIMENSION {
        Some((*source).read_pixel.expect("non-null function pointer"))
            .expect("non-null function pointer")(source);
        t = (*source).tga_pixel[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        t += ((*source).tga_pixel[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int;
        *ptr.offset(2 as ::core::ffi::c_int as isize) =
            c5to8bits[(t & 0x1f as ::core::ffi::c_int) as usize];
        t >>= 5 as ::core::ffi::c_int;
        *ptr.offset(1 as ::core::ffi::c_int as isize) =
            c5to8bits[(t & 0x1f as ::core::ffi::c_int) as usize];
        t >>= 5 as ::core::ffi::c_int;
        *ptr.offset(0 as ::core::ffi::c_int as isize) =
            c5to8bits[(t & 0x1f as ::core::ffi::c_int) as usize];
        ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
        col = col.wrapping_sub(1);
    }
    return 1 as JDIMENSION;
}
unsafe extern "C" fn get_24bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    ptr = *(*source)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).image_width;
    while col > 0 as JDIMENSION {
        Some((*source).read_pixel.expect("non-null function pointer"))
            .expect("non-null function pointer")(source);
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        *fresh2 =
            (*source).tga_pixel[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as JSAMPLE;
        let fresh3 = ptr;
        ptr = ptr.offset(1);
        *fresh3 =
            (*source).tga_pixel[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as JSAMPLE;
        let fresh4 = ptr;
        ptr = ptr.offset(1);
        *fresh4 =
            (*source).tga_pixel[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as JSAMPLE;
        col = col.wrapping_sub(1);
    }
    return 1 as JDIMENSION;
}
unsafe extern "C" fn get_memory_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut source_row: JDIMENSION = 0;
    source_row = (*cinfo)
        .image_height
        .wrapping_sub((*source).current_row)
        .wrapping_sub(1 as JDIMENSION);
    (*source).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        (*source).whole_image,
        source_row,
        1 as ::core::ffi::c_int as JDIMENSION,
        FALSE,
    );
    (*source).current_row = (*source).current_row.wrapping_add(1);
    return 1 as JDIMENSION;
}
unsafe extern "C" fn preload_image(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
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
        (*source).pub_0.buffer = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            row,
            1 as ::core::ffi::c_int as JDIMENSION,
            TRUE,
        );
        Some((*source).get_pixel_rows.expect("non-null function pointer"))
            .expect("non-null function pointer")(cinfo, sinfo);
        row = row.wrapping_add(1);
    }
    if !progress.is_null() {
        (*progress).completed_extra_passes += 1;
    }
    (*source).pub_0.get_pixel_rows = Some(
        get_memory_row as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
    )
        as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
    (*source).current_row = 0 as JDIMENSION;
    return get_memory_row(cinfo, sinfo);
}
unsafe extern "C" fn start_input_tga(mut cinfo: j_compress_ptr, mut sinfo: cjpeg_source_ptr) {
    let mut source: tga_source_ptr = sinfo as tga_source_ptr;
    let mut targaheader: [U_CHAR; 18] = [0; 18];
    let mut idlen: ::core::ffi::c_int = 0;
    let mut cmaptype: ::core::ffi::c_int = 0;
    let mut subtype: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut interlace_type: ::core::ffi::c_int = 0;
    let mut components: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut maplen: ::core::ffi::c_uint = 0;
    let mut is_bottom_up: boolean = 0;
    if !(fread(
        &raw mut targaheader as *mut U_CHAR as *mut ::core::ffi::c_void,
        1 as size_t,
        18 as size_t,
        (*source).pub_0.input_file,
    ) as size_t
        == 18 as ::core::ffi::c_int as size_t)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if targaheader[16 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 15 as ::core::ffi::c_int
    {
        targaheader[16 as ::core::ffi::c_int as usize] = 16 as U_CHAR;
    }
    idlen = targaheader[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    cmaptype = targaheader[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    subtype = targaheader[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    maplen = (targaheader[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        as ::core::ffi::c_uint)
        .wrapping_add(
            (targaheader[(5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_int,
        );
    width = (targaheader[12 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        as ::core::ffi::c_uint)
        .wrapping_add(
            (targaheader[(12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_int,
        );
    height = (targaheader[14 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        as ::core::ffi::c_uint)
        .wrapping_add(
            (targaheader[(14 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_int,
        );
    (*source).pixel_size = targaheader[16 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        >> 3 as ::core::ffi::c_int;
    flags = targaheader[17 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    is_bottom_up = (flags & 0x20 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as boolean;
    interlace_type = flags >> 6 as ::core::ffi::c_int;
    if cmaptype > 1 as ::core::ffi::c_int
        || (*source).pixel_size < 1 as ::core::ffi::c_int
        || (*source).pixel_size > 4 as ::core::ffi::c_int
        || targaheader[16 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        || interlace_type != 0 as ::core::ffi::c_int
        || width == 0 as ::core::ffi::c_uint
        || height == 0 as ::core::ffi::c_uint
    {
        (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if subtype > 8 as ::core::ffi::c_int {
        (*source).read_pixel = Some(read_rle_pixel as unsafe extern "C" fn(tga_source_ptr) -> ())
            as Option<unsafe extern "C" fn(tga_source_ptr) -> ()>;
        (*source).dup_pixel_count = 0 as ::core::ffi::c_int;
        (*source).block_count = (*source).dup_pixel_count;
        subtype -= 8 as ::core::ffi::c_int;
    } else {
        (*source).read_pixel =
            Some(read_non_rle_pixel as unsafe extern "C" fn(tga_source_ptr) -> ())
                as Option<unsafe extern "C" fn(tga_source_ptr) -> ()>;
    }
    components = 3 as ::core::ffi::c_int;
    (*cinfo).in_color_space = JCS_RGB;
    match subtype {
        1 => {
            if (*source).pixel_size == 1 as ::core::ffi::c_int
                && cmaptype == 1 as ::core::ffi::c_int
            {
                (*source).get_pixel_rows = Some(
                    get_8bit_row
                        as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
                )
                    as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
            } else {
                (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*(*cinfo).err).msg_code = JTRC_TGA_MAPPED as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                width as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] =
                height as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1 as ::core::ffi::c_int
            );
        }
        2 => {
            match (*source).pixel_size {
                2 => {
                    (*source).get_pixel_rows = Some(
                        get_16bit_row
                            as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
                    )
                        as Option<
                            unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
                        >;
                }
                3 => {
                    (*source).get_pixel_rows = Some(
                        get_24bit_row
                            as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
                    )
                        as Option<
                            unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
                        >;
                }
                4 => {
                    (*source).get_pixel_rows = Some(
                        get_24bit_row
                            as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
                    )
                        as Option<
                            unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
                        >;
                }
                _ => {
                    (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as ::core::ffi::c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
                }
            }
            (*(*cinfo).err).msg_code = JTRC_TGA as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                width as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] =
                height as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1 as ::core::ffi::c_int
            );
        }
        3 => {
            components = 1 as ::core::ffi::c_int;
            (*cinfo).in_color_space = JCS_GRAYSCALE;
            if (*source).pixel_size == 1 as ::core::ffi::c_int {
                (*source).get_pixel_rows = Some(
                    get_8bit_gray_row
                        as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
                )
                    as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
            } else {
                (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*(*cinfo).err).msg_code = JTRC_TGA_GRAY as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                width as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] =
                height as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr, 1 as ::core::ffi::c_int
            );
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    if is_bottom_up != 0 {
        (*source).whole_image = Some(
            (*(*cinfo).mem)
                .request_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            FALSE,
            width.wrapping_mul(components as JDIMENSION),
            height,
            1 as ::core::ffi::c_int as JDIMENSION,
        );
        if !(*cinfo).progress.is_null() {
            let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
            (*progress).total_extra_passes += 1;
        }
        (*source).pub_0.buffer_height = 1 as JDIMENSION;
        (*source).pub_0.get_pixel_rows = Some(
            preload_image as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
        )
            as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
    } else {
        (*source).whole_image = ::core::ptr::null_mut::<jvirt_sarray_control>();
        (*source).pub_0.buffer = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            width.wrapping_mul(components as JDIMENSION),
            1 as ::core::ffi::c_int as JDIMENSION,
        );
        (*source).pub_0.buffer_height = 1 as JDIMENSION;
        (*source).pub_0.get_pixel_rows = (*source).get_pixel_rows;
    }
    loop {
        let fresh0 = idlen;
        idlen = idlen - 1;
        if !(fresh0 != 0) {
            break;
        }
        read_byte(source);
    }
    if maplen > 0 as ::core::ffi::c_uint {
        if maplen > 256 as ::core::ffi::c_uint
            || (targaheader[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_add(
                    (targaheader[(3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                != 0 as ::core::ffi::c_uint
        {
            (*(*cinfo).err).msg_code = JERR_TGA_BADCMAP as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*source).colormap = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            maplen,
            3 as ::core::ffi::c_int as JDIMENSION,
        );
        (*source).cmap_length = maplen as ::core::ffi::c_int;
        read_colormap(
            source,
            maplen as ::core::ffi::c_int,
            targaheader[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        );
    } else {
        if cmaptype != 0 {
            (*(*cinfo).err).msg_code = JERR_TGA_BADPARMS as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*source).colormap = ::core::ptr::null_mut::<JSAMPROW>();
        (*source).cmap_length = 0 as ::core::ffi::c_int;
    }
    (*cinfo).input_components = components;
    (*cinfo).data_precision = 8 as ::core::ffi::c_int;
    (*cinfo).image_width = width as JDIMENSION;
    (*cinfo).image_height = height as JDIMENSION;
}
unsafe extern "C" fn finish_input_tga(mut cinfo: j_compress_ptr, mut sinfo: cjpeg_source_ptr) {}
#[no_mangle]
pub unsafe extern "C" fn jinit_read_targa(mut cinfo: j_compress_ptr) -> cjpeg_source_ptr {
    let mut source: tga_source_ptr = ::core::ptr::null_mut::<_tga_source_struct>();
    source = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<tga_source_struct>() as size_t,
    ) as tga_source_ptr;
    (*source).cinfo = cinfo;
    (*source).pub_0.start_input =
        Some(start_input_tga as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ()>;
    (*source).pub_0.finish_input =
        Some(finish_input_tga as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ()>;
    return source as cjpeg_source_ptr;
}
