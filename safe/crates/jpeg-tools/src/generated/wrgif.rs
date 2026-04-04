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
pub struct jpeg_color_quantizer {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_color_deconverter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_upsampler {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_inverse_dct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_entropy_decoder {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_marker_reader {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_input_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_d_post_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_d_coef_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_d_main_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_decomp_master {
    _unused: [u8; 0],
}
extern "C" {
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn jpeg_calc_output_dimensions(cinfo: j_decompress_ptr);
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
pub type INT16 = ::core::ffi::c_short;
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
pub type j_decompress_ptr = *mut jpeg_decompress_struct;
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
pub struct djpeg_dest_struct {
    pub start_output: Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ()>,
    pub put_pixel_rows:
        Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> ()>,
    pub finish_output: Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ()>,
    pub calc_buffer_dimensions:
        Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ()>,
    pub output_file: *mut FILE,
    pub buffer: JSAMPARRAY,
    pub buffer_height: JDIMENSION,
}
pub type djpeg_dest_ptr = *mut djpeg_dest_struct;
pub type gif_dest_ptr = *mut gif_dest_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gif_dest_struct {
    pub pub_0: djpeg_dest_struct,
    pub cinfo: j_decompress_ptr,
    pub n_bits: ::core::ffi::c_int,
    pub maxcode: code_int,
    pub init_bits: ::core::ffi::c_int,
    pub cur_accum: ::core::ffi::c_int,
    pub cur_bits: ::core::ffi::c_int,
    pub waiting_code: code_int,
    pub first_byte: boolean,
    pub ClearCode: code_int,
    pub EOFCode: code_int,
    pub free_code: code_int,
    pub code_counter: code_int,
    pub hash_code: *mut code_int,
    pub hash_value: *mut hash_entry,
    pub bytesinpkt: ::core::ffi::c_int,
    pub packetbuf: [::core::ffi::c_char; 256],
}
pub type hash_entry = ::core::ffi::c_int;
pub type code_int = INT16;
pub type hash_int = ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const CENTERJSAMPLE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAX_LZW_BITS: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const LZW_TABLE_SIZE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int as code_int as ::core::ffi::c_int) << MAX_LZW_BITS;
pub const HSIZE: ::core::ffi::c_int = 5003 as ::core::ffi::c_int;
unsafe extern "C" fn flush_packet(mut dinfo: gif_dest_ptr) {
    if (*dinfo).bytesinpkt > 0 as ::core::ffi::c_int {
        let fresh1 = (*dinfo).bytesinpkt;
        (*dinfo).bytesinpkt = (*dinfo).bytesinpkt + 1;
        (*dinfo).packetbuf[0 as ::core::ffi::c_int as usize] = fresh1 as ::core::ffi::c_char;
        if fwrite(
            &raw mut (*dinfo).packetbuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as size_t,
            (*dinfo).bytesinpkt as size_t,
            (*dinfo).pub_0.output_file,
        ) as size_t
            != (*dinfo).bytesinpkt as size_t
        {
            (*(*(*dinfo).cinfo).err).msg_code = JERR_FILE_WRITE as ::core::ffi::c_int;
            Some(
                (*(*(*dinfo).cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")((*dinfo).cinfo as j_common_ptr);
        }
        (*dinfo).bytesinpkt = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn output(mut dinfo: gif_dest_ptr, mut code: code_int) {
    (*dinfo).cur_accum = ((*dinfo).cur_accum as ::core::ffi::c_long
        | (code as ::core::ffi::c_long) << (*dinfo).cur_bits)
        as ::core::ffi::c_int;
    (*dinfo).cur_bits += (*dinfo).n_bits;
    while (*dinfo).cur_bits >= 8 as ::core::ffi::c_int {
        (*dinfo).bytesinpkt += 1;
        (*dinfo).packetbuf[(*dinfo).bytesinpkt as usize] =
            ((*dinfo).cur_accum & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
        if (*dinfo).bytesinpkt >= 255 as ::core::ffi::c_int {
            flush_packet(dinfo);
        }
        (*dinfo).cur_accum >>= 8 as ::core::ffi::c_int;
        (*dinfo).cur_bits -= 8 as ::core::ffi::c_int;
    }
    if (*dinfo).free_code as ::core::ffi::c_int > (*dinfo).maxcode as ::core::ffi::c_int {
        (*dinfo).n_bits += 1;
        if (*dinfo).n_bits == MAX_LZW_BITS {
            (*dinfo).maxcode = LZW_TABLE_SIZE as code_int;
        } else {
            (*dinfo).maxcode = (((1 as ::core::ffi::c_int as code_int as ::core::ffi::c_int)
                << (*dinfo).n_bits)
                - 1 as ::core::ffi::c_int) as code_int;
        }
    }
}
unsafe extern "C" fn clear_hash(mut dinfo: gif_dest_ptr) {
    memset(
        (*dinfo).hash_code as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (HSIZE as size_t).wrapping_mul(::core::mem::size_of::<code_int>() as size_t),
    );
}
unsafe extern "C" fn clear_block(mut dinfo: gif_dest_ptr) {
    clear_hash(dinfo);
    (*dinfo).free_code =
        ((*dinfo).ClearCode as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as code_int;
    output(dinfo, (*dinfo).ClearCode);
    (*dinfo).n_bits = (*dinfo).init_bits;
    (*dinfo).maxcode = (((1 as ::core::ffi::c_int as code_int as ::core::ffi::c_int)
        << (*dinfo).n_bits)
        - 1 as ::core::ffi::c_int) as code_int;
}
unsafe extern "C" fn compress_init(mut dinfo: gif_dest_ptr, mut i_bits: ::core::ffi::c_int) {
    (*dinfo).init_bits = i_bits;
    (*dinfo).n_bits = (*dinfo).init_bits;
    (*dinfo).maxcode = (((1 as ::core::ffi::c_int as code_int as ::core::ffi::c_int)
        << (*dinfo).n_bits)
        - 1 as ::core::ffi::c_int) as code_int;
    (*dinfo).ClearCode = ((1 as ::core::ffi::c_int as code_int as ::core::ffi::c_int)
        << i_bits - 1 as ::core::ffi::c_int) as code_int;
    (*dinfo).EOFCode =
        ((*dinfo).ClearCode as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as code_int;
    (*dinfo).free_code =
        ((*dinfo).ClearCode as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as code_int;
    (*dinfo).code_counter = (*dinfo).free_code;
    (*dinfo).first_byte = TRUE as boolean;
    (*dinfo).bytesinpkt = 0 as ::core::ffi::c_int;
    (*dinfo).cur_accum = 0 as ::core::ffi::c_int;
    (*dinfo).cur_bits = 0 as ::core::ffi::c_int;
    if !(*dinfo).hash_code.is_null() {
        clear_hash(dinfo);
    }
    output(dinfo, (*dinfo).ClearCode);
}
unsafe extern "C" fn compress_term(mut dinfo: gif_dest_ptr) {
    if (*dinfo).first_byte == 0 {
        output(dinfo, (*dinfo).waiting_code);
    }
    output(dinfo, (*dinfo).EOFCode);
    if (*dinfo).cur_bits > 0 as ::core::ffi::c_int {
        (*dinfo).bytesinpkt += 1;
        (*dinfo).packetbuf[(*dinfo).bytesinpkt as usize] =
            ((*dinfo).cur_accum & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
        if (*dinfo).bytesinpkt >= 255 as ::core::ffi::c_int {
            flush_packet(dinfo);
        }
    }
    flush_packet(dinfo);
}
unsafe extern "C" fn put_word(mut dinfo: gif_dest_ptr, mut w: ::core::ffi::c_uint) {
    putc(
        (w & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (*dinfo).pub_0.output_file,
    );
    putc(
        (w >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_int,
        (*dinfo).pub_0.output_file,
    );
}
unsafe extern "C" fn put_3bytes(mut dinfo: gif_dest_ptr, mut val: ::core::ffi::c_int) {
    putc(val, (*dinfo).pub_0.output_file);
    putc(val, (*dinfo).pub_0.output_file);
    putc(val, (*dinfo).pub_0.output_file);
}
unsafe extern "C" fn emit_header(
    mut dinfo: gif_dest_ptr,
    mut num_colors: ::core::ffi::c_int,
    mut colormap: JSAMPARRAY,
) {
    let mut BitsPerPixel: ::core::ffi::c_int = 0;
    let mut ColorMapSize: ::core::ffi::c_int = 0;
    let mut InitCodeSize: ::core::ffi::c_int = 0;
    let mut FlagByte: ::core::ffi::c_int = 0;
    let mut cshift: ::core::ffi::c_int = (*(*dinfo).cinfo).data_precision - 8 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    if num_colors > 256 as ::core::ffi::c_int {
        (*(*(*dinfo).cinfo).err).msg_code = JERR_TOO_MANY_COLORS as ::core::ffi::c_int;
        (*(*(*dinfo).cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = num_colors;
        Some(
            (*(*(*dinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")((*dinfo).cinfo as j_common_ptr);
    }
    BitsPerPixel = 1 as ::core::ffi::c_int;
    while num_colors > (1 as ::core::ffi::c_int) << BitsPerPixel {
        BitsPerPixel += 1;
    }
    ColorMapSize = (1 as ::core::ffi::c_int) << BitsPerPixel;
    if BitsPerPixel <= 1 as ::core::ffi::c_int {
        InitCodeSize = 2 as ::core::ffi::c_int;
    } else {
        InitCodeSize = BitsPerPixel;
    }
    putc('G' as i32, (*dinfo).pub_0.output_file);
    putc('I' as i32, (*dinfo).pub_0.output_file);
    putc('F' as i32, (*dinfo).pub_0.output_file);
    putc('8' as i32, (*dinfo).pub_0.output_file);
    putc('7' as i32, (*dinfo).pub_0.output_file);
    putc('a' as i32, (*dinfo).pub_0.output_file);
    put_word(dinfo, (*(*dinfo).cinfo).output_width);
    put_word(dinfo, (*(*dinfo).cinfo).output_height);
    FlagByte = 0x80 as ::core::ffi::c_int;
    FlagByte |= (BitsPerPixel - 1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
    FlagByte |= BitsPerPixel - 1 as ::core::ffi::c_int;
    putc(FlagByte, (*dinfo).pub_0.output_file);
    putc(0 as ::core::ffi::c_int, (*dinfo).pub_0.output_file);
    putc(0 as ::core::ffi::c_int, (*dinfo).pub_0.output_file);
    i = 0 as ::core::ffi::c_int;
    while i < ColorMapSize {
        if i < num_colors {
            if !colormap.is_null() {
                if (*(*dinfo).cinfo).out_color_space as ::core::ffi::c_uint
                    == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    putc(
                        *(*colormap.offset(0 as ::core::ffi::c_int as isize)).offset(i as isize)
                            as ::core::ffi::c_int
                            >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                    putc(
                        *(*colormap.offset(1 as ::core::ffi::c_int as isize)).offset(i as isize)
                            as ::core::ffi::c_int
                            >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                    putc(
                        *(*colormap.offset(2 as ::core::ffi::c_int as isize)).offset(i as isize)
                            as ::core::ffi::c_int
                            >> cshift,
                        (*dinfo).pub_0.output_file,
                    );
                } else {
                    put_3bytes(
                        dinfo,
                        *(*colormap.offset(0 as ::core::ffi::c_int as isize)).offset(i as isize)
                            as ::core::ffi::c_int
                            >> cshift,
                    );
                }
            } else {
                put_3bytes(
                    dinfo,
                    (i * 255 as ::core::ffi::c_int
                        + (num_colors - 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
                        / (num_colors - 1 as ::core::ffi::c_int),
                );
            }
        } else {
            put_3bytes(dinfo, CENTERJSAMPLE >> cshift);
        }
        i += 1;
    }
    putc(',' as i32, (*dinfo).pub_0.output_file);
    put_word(dinfo, 0 as ::core::ffi::c_uint);
    put_word(dinfo, 0 as ::core::ffi::c_uint);
    put_word(dinfo, (*(*dinfo).cinfo).output_width);
    put_word(dinfo, (*(*dinfo).cinfo).output_height);
    putc(0 as ::core::ffi::c_int, (*dinfo).pub_0.output_file);
    putc(InitCodeSize, (*dinfo).pub_0.output_file);
    compress_init(dinfo, InitCodeSize + 1 as ::core::ffi::c_int);
}
unsafe extern "C" fn start_output_gif(mut cinfo: j_decompress_ptr, mut dinfo: djpeg_dest_ptr) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    if (*cinfo).quantize_colors != 0 {
        emit_header(dest, (*cinfo).actual_number_of_colors, (*cinfo).colormap);
    } else {
        emit_header(dest, 256 as ::core::ffi::c_int, NULL as JSAMPARRAY);
    };
}
unsafe extern "C" fn put_LZW_pixel_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut rows_supplied: JDIMENSION,
) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    let mut ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut c: code_int = 0;
    let mut i: hash_int = 0;
    let mut disp: hash_int = 0;
    let mut probe_value: hash_entry = 0;
    ptr = *(*dest)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width;
    while col > 0 as JDIMENSION {
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        c = *fresh2 as code_int;
        if (*dest).first_byte != 0 {
            (*dest).waiting_code = c;
            (*dest).first_byte = FALSE as boolean;
        } else {
            i = (((c as ::core::ffi::c_int) << MAX_LZW_BITS - 8 as ::core::ffi::c_int)
                + (*dest).waiting_code as ::core::ffi::c_int) as hash_int;
            if i >= HSIZE {
                i -= HSIZE;
            }
            probe_value = (((*dest).waiting_code as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | c as ::core::ffi::c_int) as hash_entry;
            if *(*dest).hash_code.offset(i as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                output(dest, (*dest).waiting_code);
                if ((*dest).free_code as ::core::ffi::c_int) < LZW_TABLE_SIZE {
                    let fresh3 = (*dest).free_code;
                    (*dest).free_code = (*dest).free_code + 1;
                    *(*dest).hash_code.offset(i as isize) = fresh3;
                    *(*dest).hash_value.offset(i as isize) = probe_value;
                } else {
                    clear_block(dest);
                }
                (*dest).waiting_code = c;
            } else if *(*dest).hash_value.offset(i as isize) == probe_value {
                (*dest).waiting_code = *(*dest).hash_code.offset(i as isize);
            } else {
                if i == 0 as ::core::ffi::c_int {
                    disp = 1 as ::core::ffi::c_int as hash_int;
                } else {
                    disp = HSIZE - i;
                }
                loop {
                    i -= disp;
                    if i < 0 as ::core::ffi::c_int {
                        i += HSIZE;
                    }
                    if *(*dest).hash_code.offset(i as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        output(dest, (*dest).waiting_code);
                        if ((*dest).free_code as ::core::ffi::c_int) < LZW_TABLE_SIZE {
                            let fresh4 = (*dest).free_code;
                            (*dest).free_code = (*dest).free_code + 1;
                            *(*dest).hash_code.offset(i as isize) = fresh4;
                            *(*dest).hash_value.offset(i as isize) = probe_value;
                        } else {
                            clear_block(dest);
                        }
                        (*dest).waiting_code = c;
                        break;
                    } else {
                        if !(*(*dest).hash_value.offset(i as isize) == probe_value) {
                            continue;
                        }
                        (*dest).waiting_code = *(*dest).hash_code.offset(i as isize);
                        break;
                    }
                }
            }
        }
        col = col.wrapping_sub(1);
    }
}
unsafe extern "C" fn put_raw_pixel_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut rows_supplied: JDIMENSION,
) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    let mut ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut c: code_int = 0;
    ptr = *(*dest)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    col = (*cinfo).output_width;
    while col > 0 as JDIMENSION {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        c = *fresh0 as code_int;
        output(dest, c);
        if ((*dest).code_counter as ::core::ffi::c_int) < (*dest).maxcode as ::core::ffi::c_int {
            (*dest).code_counter += 1;
        } else {
            output(dest, (*dest).ClearCode);
            (*dest).code_counter =
                ((*dest).ClearCode as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as code_int;
        }
        col = col.wrapping_sub(1);
    }
}
unsafe extern "C" fn finish_output_gif(mut cinfo: j_decompress_ptr, mut dinfo: djpeg_dest_ptr) {
    let mut dest: gif_dest_ptr = dinfo as gif_dest_ptr;
    compress_term(dest);
    putc(0 as ::core::ffi::c_int, (*dest).pub_0.output_file);
    putc(';' as i32, (*dest).pub_0.output_file);
    fflush((*dest).pub_0.output_file);
    if ferror((*dest).pub_0.output_file) != 0 {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
}
unsafe extern "C" fn calc_buffer_dimensions_gif(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jinit_write_gif(
    mut cinfo: j_decompress_ptr,
    mut is_lzw: boolean,
) -> djpeg_dest_ptr {
    let mut dest: gif_dest_ptr = ::core::ptr::null_mut::<gif_dest_struct>();
    dest = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<gif_dest_struct>() as size_t,
    ) as gif_dest_ptr;
    (*dest).cinfo = cinfo;
    (*dest).pub_0.start_output =
        Some(start_output_gif as unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ())
            as Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ()>;
    (*dest).pub_0.finish_output =
        Some(finish_output_gif as unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ())
            as Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ()>;
    (*dest).pub_0.calc_buffer_dimensions = Some(
        calc_buffer_dimensions_gif as unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> (),
    )
        as Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ()>;
    if (*cinfo).out_color_space as ::core::ffi::c_uint
        != JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cinfo).out_color_space as ::core::ffi::c_uint
            != JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*(*cinfo).err).msg_code = JERR_GIF_COLORSPACE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).out_color_space as ::core::ffi::c_uint
        != JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cinfo).data_precision > 8 as ::core::ffi::c_int
    {
        (*cinfo).quantize_colors = TRUE as boolean;
        if (*cinfo).desired_number_of_colors > 256 as ::core::ffi::c_int {
            (*cinfo).desired_number_of_colors = 256 as ::core::ffi::c_int;
        }
    }
    jpeg_calc_output_dimensions(cinfo);
    if (*cinfo).output_components != 1 as ::core::ffi::c_int {
        (*(*cinfo).err).msg_code = JERR_GIF_BUG as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*dest).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (*cinfo).output_width,
        1 as ::core::ffi::c_int as JDIMENSION,
    );
    (*dest).pub_0.buffer_height = 1 as JDIMENSION;
    if is_lzw != 0 {
        (*dest).pub_0.put_pixel_rows = Some(
            put_LZW_pixel_rows
                as unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> (),
        )
            as Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> ()>;
        (*dest).hash_code = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (HSIZE as size_t).wrapping_mul(::core::mem::size_of::<code_int>() as size_t),
        ) as *mut code_int;
        (*dest).hash_value = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (HSIZE as size_t).wrapping_mul(::core::mem::size_of::<hash_entry>() as size_t),
        ) as *mut hash_entry;
    } else {
        (*dest).pub_0.put_pixel_rows = Some(
            put_raw_pixel_rows
                as unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> (),
        )
            as Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> ()>;
        (*dest).hash_code = ::core::ptr::null_mut::<code_int>();
        (*dest).hash_value = ::core::ptr::null_mut::<hash_entry>();
    }
    return dest as djpeg_dest_ptr;
}
