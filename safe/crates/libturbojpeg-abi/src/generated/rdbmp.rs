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
extern "C" {
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
pub const JMSG_LASTADDONCODE: C2RustUnnamed_1 = 1028;
pub const JERR_UNSUPPORTED_FORMAT: C2RustUnnamed_1 = 1027;
pub const JERR_UNKNOWN_FORMAT: C2RustUnnamed_1 = 1026;
pub const JERR_UNGETC_FAILED: C2RustUnnamed_1 = 1025;
pub const JERR_TOO_MANY_COLORS: C2RustUnnamed_1 = 1024;
pub const JERR_BAD_CMAP_FILE: C2RustUnnamed_1 = 1023;
pub const JERR_TGA_NOTCOMP: C2RustUnnamed_1 = 1022;
pub const JTRC_PPM_TEXT: C2RustUnnamed_1 = 1021;
pub const JTRC_PPM: C2RustUnnamed_1 = 1020;
pub const JTRC_PGM_TEXT: C2RustUnnamed_1 = 1019;
pub const JTRC_PGM: C2RustUnnamed_1 = 1018;
pub const JERR_PPM_OUTOFRANGE: C2RustUnnamed_1 = 1017;
pub const JERR_PPM_NOT: C2RustUnnamed_1 = 1016;
pub const JERR_PPM_NONNUMERIC: C2RustUnnamed_1 = 1015;
pub const JERR_PPM_COLORSPACE: C2RustUnnamed_1 = 1014;
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
pub type bmp_source_ptr = *mut _bmp_source_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _bmp_source_struct {
    pub pub_0: cjpeg_source_struct,
    pub cinfo: j_compress_ptr,
    pub colormap: JSAMPARRAY,
    pub whole_image: jvirt_sarray_ptr,
    pub source_row: JDIMENSION,
    pub row_width: JDIMENSION,
    pub bits_per_pixel: ::core::ffi::c_int,
    pub cmap_length: ::core::ffi::c_int,
    pub use_inversion_array: boolean,
    pub iobuffer: *mut U_CHAR,
}
pub type U_CHAR = ::core::ffi::c_uchar;
pub type bmp_source_struct = _bmp_source_struct;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
#[inline(always)]
unsafe extern "C" fn rgb_to_cmyk(
    mut r: JSAMPLE,
    mut g: JSAMPLE,
    mut b: JSAMPLE,
    mut c: *mut JSAMPLE,
    mut m: *mut JSAMPLE,
    mut y: *mut JSAMPLE,
    mut k: *mut JSAMPLE,
) {
    let mut ctmp: ::core::ffi::c_double = 1.0f64 - r as ::core::ffi::c_double / 255.0f64;
    let mut mtmp: ::core::ffi::c_double = 1.0f64 - g as ::core::ffi::c_double / 255.0f64;
    let mut ytmp: ::core::ffi::c_double = 1.0f64 - b as ::core::ffi::c_double / 255.0f64;
    let mut ktmp: ::core::ffi::c_double = if (if ctmp < mtmp { ctmp } else { mtmp }) < ytmp {
        if ctmp < mtmp {
            ctmp
        } else {
            mtmp
        }
    } else {
        ytmp
    };
    if ktmp == 1.0f64 {
        ytmp = 0.0f64;
        mtmp = ytmp;
        ctmp = mtmp;
    } else {
        ctmp = (ctmp - ktmp) / (1.0f64 - ktmp);
        mtmp = (mtmp - ktmp) / (1.0f64 - ktmp);
        ytmp = (ytmp - ktmp) / (1.0f64 - ktmp);
    }
    *c = (255.0f64 - ctmp * 255.0f64 + 0.5f64) as JSAMPLE;
    *m = (255.0f64 - mtmp * 255.0f64 + 0.5f64) as JSAMPLE;
    *y = (255.0f64 - ytmp * 255.0f64 + 0.5f64) as JSAMPLE;
    *k = (255.0f64 - ktmp * 255.0f64 + 0.5f64) as JSAMPLE;
}
static mut alpha_index: [::core::ffi::c_int; 17] = [
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    -(1 as ::core::ffi::c_int),
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    -(1 as ::core::ffi::c_int),
];
unsafe extern "C" fn read_byte(mut sinfo: bmp_source_ptr) -> ::core::ffi::c_int {
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
    mut sinfo: bmp_source_ptr,
    mut cmaplen: ::core::ffi::c_int,
    mut mapentrysize: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut gray: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    match mapentrysize {
        3 => {
            i = 0 as ::core::ffi::c_int;
            while i < cmaplen {
                *(*(*sinfo).colormap.offset(2 as ::core::ffi::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(1 as ::core::ffi::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(0 as ::core::ffi::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                if *(*(*sinfo).colormap.offset(2 as ::core::ffi::c_int as isize)).offset(i as isize)
                    as ::core::ffi::c_int
                    != *(*(*sinfo).colormap.offset(1 as ::core::ffi::c_int as isize))
                        .offset(i as isize) as ::core::ffi::c_int
                    || *(*(*sinfo).colormap.offset(1 as ::core::ffi::c_int as isize))
                        .offset(i as isize) as ::core::ffi::c_int
                        != *(*(*sinfo).colormap.offset(0 as ::core::ffi::c_int as isize))
                            .offset(i as isize) as ::core::ffi::c_int
                {
                    gray = 0 as ::core::ffi::c_int;
                }
                i += 1;
            }
        }
        4 => {
            i = 0 as ::core::ffi::c_int;
            while i < cmaplen {
                *(*(*sinfo).colormap.offset(2 as ::core::ffi::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(1 as ::core::ffi::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(0 as ::core::ffi::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                read_byte(sinfo);
                if *(*(*sinfo).colormap.offset(2 as ::core::ffi::c_int as isize)).offset(i as isize)
                    as ::core::ffi::c_int
                    != *(*(*sinfo).colormap.offset(1 as ::core::ffi::c_int as isize))
                        .offset(i as isize) as ::core::ffi::c_int
                    || *(*(*sinfo).colormap.offset(1 as ::core::ffi::c_int as isize))
                        .offset(i as isize) as ::core::ffi::c_int
                        != *(*(*sinfo).colormap.offset(0 as ::core::ffi::c_int as isize))
                            .offset(i as isize) as ::core::ffi::c_int
                {
                    gray = 0 as ::core::ffi::c_int;
                }
                i += 1;
            }
        }
        _ => {
            (*(*(*sinfo).cinfo).err).msg_code = JERR_BMP_BADCMAP as ::core::ffi::c_int;
            Some(
                (*(*(*sinfo).cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")((*sinfo).cinfo as j_common_ptr);
        }
    }
    if ((*(*sinfo).cinfo).in_color_space as ::core::ffi::c_uint
        == JCS_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*(*sinfo).cinfo).in_color_space as ::core::ffi::c_uint
            == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint)
        && gray != 0
    {
        (*(*sinfo).cinfo).in_color_space = JCS_GRAYSCALE;
    }
    if (*(*sinfo).cinfo).in_color_space as ::core::ffi::c_uint
        == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
        && gray == 0
    {
        (*(*(*sinfo).cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as ::core::ffi::c_int;
        Some(
            (*(*(*sinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")((*sinfo).cinfo as j_common_ptr);
    }
}
unsafe extern "C" fn get_8bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut colormap: JSAMPARRAY = (*source).colormap;
    let mut cmaplen: ::core::ffi::c_int = (*source).cmap_length;
    let mut image_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    let mut t: ::core::ffi::c_int = 0;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    if (*source).use_inversion_array != 0 {
        (*source).source_row = (*source).source_row.wrapping_sub(1);
        image_ptr = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1 as ::core::ffi::c_int as JDIMENSION,
            FALSE,
        );
        inptr = *image_ptr.offset(0 as ::core::ffi::c_int as isize);
    } else {
        if !(fread(
            (*source).iobuffer as *mut ::core::ffi::c_void,
            1 as size_t,
            (*source).row_width as size_t,
            (*source).pub_0.input_file,
        ) as size_t
            == (*source).row_width as size_t)
        {
            (*(*cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        inptr = (*source).iobuffer as JSAMPROW;
    }
    outptr = *(*source)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    if (*cinfo).in_color_space as ::core::ffi::c_uint
        == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        col = (*cinfo).image_width;
        while col > 0 as JDIMENSION {
            let fresh19 = inptr;
            inptr = inptr.offset(1);
            t = *fresh19 as ::core::ffi::c_int;
            if t >= cmaplen {
                (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            let fresh20 = outptr;
            outptr = outptr.offset(1);
            *fresh20 = *(*colormap.offset(0 as ::core::ffi::c_int as isize)).offset(t as isize);
            col = col.wrapping_sub(1);
        }
    } else if (*cinfo).in_color_space as ::core::ffi::c_uint
        == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        col = (*cinfo).image_width;
        while col > 0 as JDIMENSION {
            let fresh21 = inptr;
            inptr = inptr.offset(1);
            t = *fresh21 as ::core::ffi::c_int;
            if t >= cmaplen {
                (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            rgb_to_cmyk(
                *(*colormap.offset(0 as ::core::ffi::c_int as isize)).offset(t as isize),
                *(*colormap.offset(1 as ::core::ffi::c_int as isize)).offset(t as isize),
                *(*colormap.offset(2 as ::core::ffi::c_int as isize)).offset(t as isize),
                outptr as *mut JSAMPLE,
                outptr.offset(1 as ::core::ffi::c_int as isize),
                outptr.offset(2 as ::core::ffi::c_int as isize),
                outptr.offset(3 as ::core::ffi::c_int as isize),
            );
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_sub(1);
        }
    } else {
        let mut rindex: ::core::ffi::c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: ::core::ffi::c_int = rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: ::core::ffi::c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: ::core::ffi::c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: ::core::ffi::c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0 as ::core::ffi::c_int {
            col = (*cinfo).image_width;
            while col > 0 as JDIMENSION {
                let fresh22 = inptr;
                inptr = inptr.offset(1);
                t = *fresh22 as ::core::ffi::c_int;
                if t >= cmaplen {
                    (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as ::core::ffi::c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
                }
                *outptr.offset(rindex as isize) =
                    *(*colormap.offset(0 as ::core::ffi::c_int as isize)).offset(t as isize);
                *outptr.offset(gindex as isize) =
                    *(*colormap.offset(1 as ::core::ffi::c_int as isize)).offset(t as isize);
                *outptr.offset(bindex as isize) =
                    *(*colormap.offset(2 as ::core::ffi::c_int as isize)).offset(t as isize);
                *outptr.offset(aindex as isize) = 0xff as JSAMPLE;
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1);
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0 as JDIMENSION {
                let fresh23 = inptr;
                inptr = inptr.offset(1);
                t = *fresh23 as ::core::ffi::c_int;
                if t >= cmaplen {
                    (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as ::core::ffi::c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
                }
                *outptr.offset(rindex as isize) =
                    *(*colormap.offset(0 as ::core::ffi::c_int as isize)).offset(t as isize);
                *outptr.offset(gindex as isize) =
                    *(*colormap.offset(1 as ::core::ffi::c_int as isize)).offset(t as isize);
                *outptr.offset(bindex as isize) =
                    *(*colormap.offset(2 as ::core::ffi::c_int as isize)).offset(t as isize);
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1);
            }
        }
    }
    return 1 as JDIMENSION;
}
unsafe extern "C" fn get_24bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut image_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    if (*source).use_inversion_array != 0 {
        (*source).source_row = (*source).source_row.wrapping_sub(1);
        image_ptr = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1 as ::core::ffi::c_int as JDIMENSION,
            FALSE,
        );
        inptr = *image_ptr.offset(0 as ::core::ffi::c_int as isize);
    } else {
        if !(fread(
            (*source).iobuffer as *mut ::core::ffi::c_void,
            1 as size_t,
            (*source).row_width as size_t,
            (*source).pub_0.input_file,
        ) as size_t
            == (*source).row_width as size_t)
        {
            (*(*cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        inptr = (*source).iobuffer as JSAMPROW;
    }
    outptr = *(*source)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    if (*cinfo).in_color_space as ::core::ffi::c_uint
        == JCS_EXT_BGR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        memcpy(
            outptr as *mut ::core::ffi::c_void,
            inptr as *const ::core::ffi::c_void,
            (*source).row_width as size_t,
        );
    } else if (*cinfo).in_color_space as ::core::ffi::c_uint
        == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        col = (*cinfo).image_width;
        while col > 0 as JDIMENSION {
            let fresh10 = inptr;
            inptr = inptr.offset(1);
            let mut b: JSAMPLE = *fresh10;
            let fresh11 = inptr;
            inptr = inptr.offset(1);
            let mut g: JSAMPLE = *fresh11;
            let fresh12 = inptr;
            inptr = inptr.offset(1);
            let mut r: JSAMPLE = *fresh12;
            rgb_to_cmyk(
                r,
                g,
                b,
                outptr as *mut JSAMPLE,
                outptr.offset(1 as ::core::ffi::c_int as isize),
                outptr.offset(2 as ::core::ffi::c_int as isize),
                outptr.offset(3 as ::core::ffi::c_int as isize),
            );
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_sub(1);
        }
    } else {
        let mut rindex: ::core::ffi::c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: ::core::ffi::c_int = rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: ::core::ffi::c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: ::core::ffi::c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: ::core::ffi::c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0 as ::core::ffi::c_int {
            col = (*cinfo).image_width;
            while col > 0 as JDIMENSION {
                let fresh13 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh13;
                let fresh14 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh14;
                let fresh15 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh15;
                *outptr.offset(aindex as isize) = 0xff as JSAMPLE;
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1);
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0 as JDIMENSION {
                let fresh16 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh16;
                let fresh17 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh17;
                let fresh18 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh18;
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1);
            }
        }
    }
    return 1 as JDIMENSION;
}
unsafe extern "C" fn get_32bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut image_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    if (*source).use_inversion_array != 0 {
        (*source).source_row = (*source).source_row.wrapping_sub(1);
        image_ptr = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1 as ::core::ffi::c_int as JDIMENSION,
            FALSE,
        );
        inptr = *image_ptr.offset(0 as ::core::ffi::c_int as isize);
    } else {
        if !(fread(
            (*source).iobuffer as *mut ::core::ffi::c_void,
            1 as size_t,
            (*source).row_width as size_t,
            (*source).pub_0.input_file,
        ) as size_t
            == (*source).row_width as size_t)
        {
            (*(*cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        inptr = (*source).iobuffer as JSAMPROW;
    }
    outptr = *(*source)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    if (*cinfo).in_color_space as ::core::ffi::c_uint
        == JCS_EXT_BGRX as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cinfo).in_color_space as ::core::ffi::c_uint
            == JCS_EXT_BGRA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        memcpy(
            outptr as *mut ::core::ffi::c_void,
            inptr as *const ::core::ffi::c_void,
            (*source).row_width as size_t,
        );
    } else if (*cinfo).in_color_space as ::core::ffi::c_uint
        == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        col = (*cinfo).image_width;
        while col > 0 as JDIMENSION {
            let fresh0 = inptr;
            inptr = inptr.offset(1);
            let mut b: JSAMPLE = *fresh0;
            let fresh1 = inptr;
            inptr = inptr.offset(1);
            let mut g: JSAMPLE = *fresh1;
            let fresh2 = inptr;
            inptr = inptr.offset(1);
            let mut r: JSAMPLE = *fresh2;
            rgb_to_cmyk(
                r,
                g,
                b,
                outptr as *mut JSAMPLE,
                outptr.offset(1 as ::core::ffi::c_int as isize),
                outptr.offset(2 as ::core::ffi::c_int as isize),
                outptr.offset(3 as ::core::ffi::c_int as isize),
            );
            inptr = inptr.offset(1);
            outptr = outptr.offset(4 as ::core::ffi::c_int as isize);
            col = col.wrapping_sub(1);
        }
    } else {
        let mut rindex: ::core::ffi::c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: ::core::ffi::c_int = rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: ::core::ffi::c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: ::core::ffi::c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: ::core::ffi::c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0 as ::core::ffi::c_int {
            col = (*cinfo).image_width;
            while col > 0 as JDIMENSION {
                let fresh3 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh3;
                let fresh4 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh4;
                let fresh5 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh5;
                let fresh6 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(aindex as isize) = *fresh6;
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1);
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0 as JDIMENSION {
                let fresh7 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh7;
                let fresh8 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh8;
                let fresh9 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh9;
                inptr = inptr.offset(1);
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1);
            }
        }
    }
    return 1 as JDIMENSION;
}
unsafe extern "C" fn preload_image(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    let mut out_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut image_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
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
        image_ptr = Some(
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
        out_ptr = *image_ptr.offset(0 as ::core::ffi::c_int as isize);
        if fread(
            out_ptr as *mut ::core::ffi::c_void,
            1 as size_t,
            (*source).row_width as size_t,
            infile,
        ) != (*source).row_width as ::core::ffi::c_ulong
        {
            if feof(infile) != 0 {
                (*(*cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            } else {
                (*(*cinfo).err).msg_code = JERR_FILE_READ as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        row = row.wrapping_add(1);
    }
    if !progress.is_null() {
        (*progress).completed_extra_passes += 1;
    }
    match (*source).bits_per_pixel {
        8 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_8bit_row
                    as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
            )
                as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
        }
        24 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_24bit_row
                    as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
            )
                as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
        }
        32 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_32bit_row
                    as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
            )
                as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    (*source).source_row = (*cinfo).image_height;
    return Some(
        (*source)
            .pub_0
            .get_pixel_rows
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, sinfo);
}
unsafe extern "C" fn start_input_bmp(mut cinfo: j_compress_ptr, mut sinfo: cjpeg_source_ptr) {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut bmpfileheader: [U_CHAR; 14] = [0; 14];
    let mut bmpinfoheader: [U_CHAR; 64] = [0; 64];
    let mut bfOffBits: ::core::ffi::c_int = 0;
    let mut headerSize: ::core::ffi::c_int = 0;
    let mut biWidth: ::core::ffi::c_int = 0;
    let mut biHeight: ::core::ffi::c_int = 0;
    let mut biPlanes: ::core::ffi::c_ushort = 0;
    let mut biCompression: ::core::ffi::c_uint = 0;
    let mut biXPelsPerMeter: ::core::ffi::c_int = 0;
    let mut biYPelsPerMeter: ::core::ffi::c_int = 0;
    let mut biClrUsed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mapentrysize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bPad: ::core::ffi::c_int = 0;
    let mut row_width: JDIMENSION = 0 as JDIMENSION;
    if !(fread(
        &raw mut bmpfileheader as *mut U_CHAR as *mut ::core::ffi::c_void,
        1 as size_t,
        14 as size_t,
        (*source).pub_0.input_file,
    ) as size_t
        == 14 as ::core::ffi::c_int as size_t)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if bmpfileheader[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        as ::core::ffi::c_ushort as ::core::ffi::c_int
        + ((bmpfileheader[(0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
        != 0x4d42 as ::core::ffi::c_int
    {
        (*(*cinfo).err).msg_code = JERR_BMP_NOT as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    bfOffBits = (bmpfileheader[10 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        as ::core::ffi::c_uint)
        .wrapping_add(
            (bmpfileheader[(10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (bmpfileheader[(10 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                << 16 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (bmpfileheader[(10 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                << 24 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
    if !(fread(
        &raw mut bmpinfoheader as *mut U_CHAR as *mut ::core::ffi::c_void,
        1 as size_t,
        4 as size_t,
        (*source).pub_0.input_file,
    ) as size_t
        == 4 as ::core::ffi::c_int as size_t)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    headerSize = (bmpinfoheader[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        as ::core::ffi::c_uint)
        .wrapping_add(
            (bmpinfoheader[(0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (bmpinfoheader[(0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                << 16 as ::core::ffi::c_int,
        )
        .wrapping_add(
            (bmpinfoheader[(0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                << 24 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
    if headerSize < 12 as ::core::ffi::c_int
        || headerSize > 64 as ::core::ffi::c_int
        || headerSize + 14 as ::core::ffi::c_int > bfOffBits
    {
        (*(*cinfo).err).msg_code = JERR_BMP_BADHEADER as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if !(fread(
        (&raw mut bmpinfoheader as *mut U_CHAR).offset(4 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        1 as size_t,
        (headerSize - 4 as ::core::ffi::c_int) as size_t,
        (*source).pub_0.input_file,
    ) as size_t
        == (headerSize - 4 as ::core::ffi::c_int) as size_t)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    match headerSize {
        12 => {
            biWidth = bmpinfoheader[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                as ::core::ffi::c_ushort as ::core::ffi::c_int
                + ((bmpinfoheader[(4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int);
            biHeight = bmpinfoheader[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                as ::core::ffi::c_ushort as ::core::ffi::c_int
                + ((bmpinfoheader[(6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int);
            biPlanes = (bmpinfoheader[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                as ::core::ffi::c_ushort as ::core::ffi::c_int
                + ((bmpinfoheader[(8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int)) as ::core::ffi::c_ushort;
            (*source).bits_per_pixel =
                bmpinfoheader[10 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    as ::core::ffi::c_ushort as ::core::ffi::c_int
                    + ((bmpinfoheader[(10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int);
            match (*source).bits_per_pixel {
                8 => {
                    mapentrysize = 3 as ::core::ffi::c_int;
                    (*(*cinfo).err).msg_code = JTRC_BMP_OS2_MAPPED as ::core::ffi::c_int;
                    (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = biHeight;
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
                24 | 32 => {
                    let mut _mp: *mut ::core::ffi::c_int =
                        &raw mut (*(*cinfo).err).msg_parm.i as *mut ::core::ffi::c_int;
                    *_mp.offset(0 as ::core::ffi::c_int as isize) = biWidth;
                    *_mp.offset(1 as ::core::ffi::c_int as isize) = biHeight;
                    *_mp.offset(2 as ::core::ffi::c_int as isize) = (*source).bits_per_pixel;
                    (*(*cinfo).err).msg_code = JTRC_BMP_OS2 as ::core::ffi::c_int;
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
                    (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as ::core::ffi::c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
                }
            }
        }
        40 | 64 => {
            biWidth = (bmpinfoheader[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
            biHeight = (bmpinfoheader[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
            biPlanes = (bmpinfoheader[12 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                as ::core::ffi::c_ushort as ::core::ffi::c_int
                + ((bmpinfoheader[(12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int)) as ::core::ffi::c_ushort;
            (*source).bits_per_pixel =
                bmpinfoheader[14 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    as ::core::ffi::c_ushort as ::core::ffi::c_int
                    + ((bmpinfoheader[(14 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int);
            biCompression = (bmpinfoheader[16 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
            biXPelsPerMeter = (bmpinfoheader[24 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(24 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(24 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(24 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
            biYPelsPerMeter = (bmpinfoheader[28 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(28 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(28 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(28 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
            biClrUsed = (bmpinfoheader[32 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
            match (*source).bits_per_pixel {
                8 => {
                    mapentrysize = 4 as ::core::ffi::c_int;
                    (*(*cinfo).err).msg_code = JTRC_BMP_MAPPED as ::core::ffi::c_int;
                    (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = biHeight;
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
                24 | 32 => {
                    let mut _mp_0: *mut ::core::ffi::c_int =
                        &raw mut (*(*cinfo).err).msg_parm.i as *mut ::core::ffi::c_int;
                    *_mp_0.offset(0 as ::core::ffi::c_int as isize) = biWidth;
                    *_mp_0.offset(1 as ::core::ffi::c_int as isize) = biHeight;
                    *_mp_0.offset(2 as ::core::ffi::c_int as isize) = (*source).bits_per_pixel;
                    (*(*cinfo).err).msg_code = JTRC_BMP as ::core::ffi::c_int;
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
                    (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as ::core::ffi::c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
                }
            }
            if biCompression != 0 as ::core::ffi::c_uint {
                (*(*cinfo).err).msg_code = JERR_BMP_COMPRESSED as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if biXPelsPerMeter > 0 as ::core::ffi::c_int
                && biYPelsPerMeter > 0 as ::core::ffi::c_int
            {
                (*cinfo).X_density = (biXPelsPerMeter / 100 as ::core::ffi::c_int) as UINT16;
                (*cinfo).Y_density = (biYPelsPerMeter / 100 as ::core::ffi::c_int) as UINT16;
                (*cinfo).density_unit = 2 as UINT8;
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BMP_BADHEADER as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
            return;
        }
    }
    if biWidth <= 0 as ::core::ffi::c_int || biHeight <= 0 as ::core::ffi::c_int {
        (*(*cinfo).err).msg_code = JERR_BMP_EMPTY as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if biPlanes as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
        (*(*cinfo).err).msg_code = JERR_BMP_BADPLANES as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    bPad = bfOffBits - (headerSize + 14 as ::core::ffi::c_int);
    if mapentrysize > 0 as ::core::ffi::c_int {
        if biClrUsed <= 0 as ::core::ffi::c_int {
            biClrUsed = 256 as ::core::ffi::c_int;
        } else if biClrUsed > 256 as ::core::ffi::c_int {
            (*(*cinfo).err).msg_code = JERR_BMP_BADCMAP as ::core::ffi::c_int;
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
            biClrUsed as JDIMENSION,
            3 as ::core::ffi::c_int as JDIMENSION,
        );
        (*source).cmap_length = biClrUsed;
        read_colormap(source, biClrUsed, mapentrysize);
        bPad -= biClrUsed * mapentrysize;
    }
    if bPad < 0 as ::core::ffi::c_int {
        (*(*cinfo).err).msg_code = JERR_BMP_BADHEADER as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    loop {
        bPad -= 1;
        if !(bPad >= 0 as ::core::ffi::c_int) {
            break;
        }
        read_byte(source);
    }
    match (*source).bits_per_pixel {
        8 => {
            if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cinfo).in_color_space = JCS_EXT_RGB;
            }
            if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    >= JCS_EXT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*cinfo).in_color_space as ::core::ffi::c_uint
                        <= JCS_EXT_ARGB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize];
            } else if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cinfo).input_components = 1 as ::core::ffi::c_int;
            } else if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cinfo).input_components = 4 as ::core::ffi::c_int;
            } else {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            row_width = biWidth as JDIMENSION;
        }
        24 => {
            if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cinfo).in_color_space = JCS_EXT_BGR;
            }
            if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    >= JCS_EXT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*cinfo).in_color_space as ::core::ffi::c_uint
                        <= JCS_EXT_ARGB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize];
            } else if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cinfo).input_components = 4 as ::core::ffi::c_int;
            } else {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (biWidth as ::core::ffi::c_ulonglong).wrapping_mul(3 as ::core::ffi::c_ulonglong)
                > 0xffffffff as ::core::ffi::c_ulonglong
            {
                (*(*cinfo).err).msg_code = JERR_WIDTH_OVERFLOW as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            row_width = (biWidth as JDIMENSION).wrapping_mul(3 as JDIMENSION);
        }
        32 => {
            if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cinfo).in_color_space = JCS_EXT_BGRA;
            }
            if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    >= JCS_EXT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*cinfo).in_color_space as ::core::ffi::c_uint
                        <= JCS_EXT_ARGB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize];
            } else if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cinfo).input_components = 4 as ::core::ffi::c_int;
            } else {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (biWidth as ::core::ffi::c_ulonglong).wrapping_mul(4 as ::core::ffi::c_ulonglong)
                > 0xffffffff as ::core::ffi::c_ulonglong
            {
                (*(*cinfo).err).msg_code = JERR_WIDTH_OVERFLOW as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            row_width = (biWidth as JDIMENSION).wrapping_mul(4 as JDIMENSION);
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    while row_width & 3 as JDIMENSION != 0 as JDIMENSION {
        row_width = row_width.wrapping_add(1);
    }
    (*source).row_width = row_width;
    if (*source).use_inversion_array != 0 {
        (*source).whole_image = Some(
            (*(*cinfo).mem)
                .request_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            FALSE,
            row_width,
            biHeight as JDIMENSION,
            1 as ::core::ffi::c_int as JDIMENSION,
        );
        (*source).pub_0.get_pixel_rows = Some(
            preload_image as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
        )
            as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
        if !(*cinfo).progress.is_null() {
            let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
            (*progress).total_extra_passes += 1;
        }
    } else {
        (*source).iobuffer = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            row_width as size_t,
        ) as *mut U_CHAR;
        match (*source).bits_per_pixel {
            8 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_8bit_row
                        as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
                )
                    as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
            }
            24 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_24bit_row
                        as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
                )
                    as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
            }
            32 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_32bit_row
                        as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION,
                )
                    as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>;
            }
            _ => {
                (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    if (biWidth as ::core::ffi::c_ulonglong)
        .wrapping_mul((*cinfo).input_components as ::core::ffi::c_ulonglong)
        > 0xffffffff as ::core::ffi::c_ulonglong
    {
        (*(*cinfo).err).msg_code = JERR_WIDTH_OVERFLOW as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*source).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (biWidth as JDIMENSION).wrapping_mul((*cinfo).input_components as JDIMENSION),
        1 as ::core::ffi::c_int as JDIMENSION,
    );
    (*source).pub_0.buffer_height = 1 as JDIMENSION;
    (*cinfo).data_precision = 8 as ::core::ffi::c_int;
    (*cinfo).image_width = biWidth as JDIMENSION;
    (*cinfo).image_height = biHeight as JDIMENSION;
}
unsafe extern "C" fn finish_input_bmp(mut cinfo: j_compress_ptr, mut sinfo: cjpeg_source_ptr) {}
#[no_mangle]
pub unsafe extern "C" fn jinit_read_bmp(
    mut cinfo: j_compress_ptr,
    mut use_inversion_array: boolean,
) -> cjpeg_source_ptr {
    let mut source: bmp_source_ptr = ::core::ptr::null_mut::<_bmp_source_struct>();
    source = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<bmp_source_struct>() as size_t,
    ) as bmp_source_ptr;
    (*source).cinfo = cinfo;
    (*source).pub_0.start_input =
        Some(start_input_bmp as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ()>;
    (*source).pub_0.finish_input =
        Some(finish_input_bmp as unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ()>;
    (*source).use_inversion_array = use_inversion_array;
    return source as cjpeg_source_ptr;
}
