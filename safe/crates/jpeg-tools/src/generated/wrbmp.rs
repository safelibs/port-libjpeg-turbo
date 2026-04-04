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
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
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
pub type bmp_dest_ptr = *mut bmp_dest_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bmp_dest_struct {
    pub pub_0: djpeg_dest_struct,
    pub is_os2: boolean,
    pub whole_image: jvirt_sarray_ptr,
    pub data_width: JDIMENSION,
    pub row_width: JDIMENSION,
    pub pad_bytes: ::core::ffi::c_int,
    pub cur_output_row: JDIMENSION,
    pub use_inversion_array: boolean,
    pub iobuffer: *mut JSAMPLE,
}
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
unsafe extern "C" fn cmyk_to_rgb(
    mut c: JSAMPLE,
    mut m: JSAMPLE,
    mut y: JSAMPLE,
    mut k: JSAMPLE,
    mut r: *mut JSAMPLE,
    mut g: *mut JSAMPLE,
    mut b: *mut JSAMPLE,
) {
    *r = (c as ::core::ffi::c_double * k as ::core::ffi::c_double / 255.0f64 + 0.5f64) as JSAMPLE;
    *g = (m as ::core::ffi::c_double * k as ::core::ffi::c_double / 255.0f64 + 0.5f64) as JSAMPLE;
    *b = (y as ::core::ffi::c_double * k as ::core::ffi::c_double / 255.0f64 + 0.5f64) as JSAMPLE;
}
#[inline(always)]
unsafe extern "C" fn is_big_endian() -> boolean {
    let mut test_value: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if *(&raw mut test_value as *mut ::core::ffi::c_char) as ::core::ffi::c_int
        != 1 as ::core::ffi::c_int
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn put_pixel_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut rows_supplied: JDIMENSION,
) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    let mut image_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut pad: ::core::ffi::c_int = 0;
    if (*dest).use_inversion_array != 0 {
        image_ptr = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*dest).whole_image,
            (*dest).cur_output_row,
            1 as ::core::ffi::c_int as JDIMENSION,
            TRUE,
        );
        (*dest).cur_output_row = (*dest).cur_output_row.wrapping_add(1);
        outptr = *image_ptr.offset(0 as ::core::ffi::c_int as isize);
    } else {
        outptr = (*dest).iobuffer as JSAMPROW;
    }
    inptr = *(*dest)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    if (*cinfo).out_color_space as ::core::ffi::c_uint
        == JCS_EXT_BGR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        memcpy(
            outptr as *mut ::core::ffi::c_void,
            inptr as *const ::core::ffi::c_void,
            (*dest).row_width as size_t,
        );
        outptr = outptr.offset((*cinfo).output_width.wrapping_mul(3 as JDIMENSION) as isize);
    } else if (*cinfo).out_color_space as ::core::ffi::c_uint
        == JCS_RGB565 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut big_endian: boolean = is_big_endian();
        let mut inptr2: *mut ::core::ffi::c_ushort = inptr as *mut ::core::ffi::c_ushort;
        col = (*cinfo).output_width;
        while col > 0 as JDIMENSION {
            if big_endian != 0 {
                *outptr.offset(0 as ::core::ffi::c_int as isize) =
                    (*inptr2 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int) as JSAMPLE;
                *outptr.offset(1 as ::core::ffi::c_int as isize) =
                    ((*inptr2 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        | *inptr2 as ::core::ffi::c_int >> 11 as ::core::ffi::c_int
                            & 0x1c as ::core::ffi::c_int) as JSAMPLE;
                *outptr.offset(2 as ::core::ffi::c_int as isize) =
                    (*inptr2 as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int) as JSAMPLE;
            } else {
                *outptr.offset(0 as ::core::ffi::c_int as isize) =
                    ((*inptr2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int) as JSAMPLE;
                *outptr.offset(1 as ::core::ffi::c_int as isize) =
                    (*inptr2 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
                        & 0xfc as ::core::ffi::c_int) as JSAMPLE;
                *outptr.offset(2 as ::core::ffi::c_int as isize) =
                    (*inptr2 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int) as JSAMPLE;
            }
            outptr = outptr.offset(3 as ::core::ffi::c_int as isize);
            inptr2 = inptr2.offset(1);
            col = col.wrapping_sub(1);
        }
    } else if (*cinfo).out_color_space as ::core::ffi::c_uint
        == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        col = (*cinfo).output_width;
        while col > 0 as JDIMENSION {
            let fresh0 = inptr;
            inptr = inptr.offset(1);
            let mut c: JSAMPLE = *fresh0;
            let fresh1 = inptr;
            inptr = inptr.offset(1);
            let mut m: JSAMPLE = *fresh1;
            let fresh2 = inptr;
            inptr = inptr.offset(1);
            let mut y: JSAMPLE = *fresh2;
            let fresh3 = inptr;
            inptr = inptr.offset(1);
            let mut k: JSAMPLE = *fresh3;
            cmyk_to_rgb(
                c,
                m,
                y,
                k,
                outptr.offset(2 as ::core::ffi::c_int as isize),
                outptr.offset(1 as ::core::ffi::c_int as isize),
                outptr as *mut JSAMPLE,
            );
            outptr = outptr.offset(3 as ::core::ffi::c_int as isize);
            col = col.wrapping_sub(1);
        }
    } else {
        let mut rindex: ::core::ffi::c_int = rgb_red[(*cinfo).out_color_space as usize];
        let mut gindex: ::core::ffi::c_int = rgb_green[(*cinfo).out_color_space as usize];
        let mut bindex: ::core::ffi::c_int = rgb_blue[(*cinfo).out_color_space as usize];
        let mut ps: ::core::ffi::c_int = rgb_pixelsize[(*cinfo).out_color_space as usize];
        col = (*cinfo).output_width;
        while col > 0 as JDIMENSION {
            *outptr.offset(0 as ::core::ffi::c_int as isize) = *inptr.offset(bindex as isize);
            *outptr.offset(1 as ::core::ffi::c_int as isize) = *inptr.offset(gindex as isize);
            *outptr.offset(2 as ::core::ffi::c_int as isize) = *inptr.offset(rindex as isize);
            outptr = outptr.offset(3 as ::core::ffi::c_int as isize);
            inptr = inptr.offset(ps as isize);
            col = col.wrapping_sub(1);
        }
    }
    pad = (*dest).pad_bytes;
    loop {
        pad -= 1;
        if !(pad >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh4 = outptr;
        outptr = outptr.offset(1);
        *fresh4 = 0 as JSAMPLE;
    }
    if (*dest).use_inversion_array == 0 {
        fwrite(
            (*dest).iobuffer as *const ::core::ffi::c_void,
            1 as size_t,
            (*dest).row_width as size_t,
            (*dest).pub_0.output_file,
        );
    }
}
unsafe extern "C" fn put_gray_rows(
    mut cinfo: j_decompress_ptr,
    mut dinfo: djpeg_dest_ptr,
    mut rows_supplied: JDIMENSION,
) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    let mut image_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut pad: ::core::ffi::c_int = 0;
    if (*dest).use_inversion_array != 0 {
        image_ptr = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*dest).whole_image,
            (*dest).cur_output_row,
            1 as ::core::ffi::c_int as JDIMENSION,
            TRUE,
        );
        (*dest).cur_output_row = (*dest).cur_output_row.wrapping_add(1);
        outptr = *image_ptr.offset(0 as ::core::ffi::c_int as isize);
    } else {
        outptr = (*dest).iobuffer as JSAMPROW;
    }
    inptr = *(*dest)
        .pub_0
        .buffer
        .offset(0 as ::core::ffi::c_int as isize);
    memcpy(
        outptr as *mut ::core::ffi::c_void,
        inptr as *const ::core::ffi::c_void,
        (*cinfo).output_width as size_t,
    );
    outptr = outptr.offset((*cinfo).output_width as isize);
    pad = (*dest).pad_bytes;
    loop {
        pad -= 1;
        if !(pad >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh5 = outptr;
        outptr = outptr.offset(1);
        *fresh5 = 0 as JSAMPLE;
    }
    if (*dest).use_inversion_array == 0 {
        fwrite(
            (*dest).iobuffer as *const ::core::ffi::c_void,
            1 as size_t,
            (*dest).row_width as size_t,
            (*dest).pub_0.output_file,
        );
    }
}
unsafe extern "C" fn write_bmp_header(mut cinfo: j_decompress_ptr, mut dest: bmp_dest_ptr) {
    let mut bmpfileheader: [::core::ffi::c_char; 14] = [0; 14];
    let mut bmpinfoheader: [::core::ffi::c_char; 40] = [0; 40];
    let mut headersize: ::core::ffi::c_long = 0;
    let mut bfSize: ::core::ffi::c_long = 0;
    let mut bits_per_pixel: ::core::ffi::c_int = 0;
    let mut cmap_entries: ::core::ffi::c_int = 0;
    if (*cinfo).out_color_space as ::core::ffi::c_uint
        == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cinfo).out_color_space as ::core::ffi::c_uint
            >= JCS_EXT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*cinfo).out_color_space as ::core::ffi::c_uint
                <= JCS_EXT_ARGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*cinfo).quantize_colors != 0 {
            bits_per_pixel = 8 as ::core::ffi::c_int;
            cmap_entries = 256 as ::core::ffi::c_int;
        } else {
            bits_per_pixel = 24 as ::core::ffi::c_int;
            cmap_entries = 0 as ::core::ffi::c_int;
        }
    } else if (*cinfo).out_color_space as ::core::ffi::c_uint
        == JCS_RGB565 as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cinfo).out_color_space as ::core::ffi::c_uint
            == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        bits_per_pixel = 24 as ::core::ffi::c_int;
        cmap_entries = 0 as ::core::ffi::c_int;
    } else {
        bits_per_pixel = 8 as ::core::ffi::c_int;
        cmap_entries = 256 as ::core::ffi::c_int;
    }
    headersize = (14 as ::core::ffi::c_int
        + 40 as ::core::ffi::c_int
        + cmap_entries * 4 as ::core::ffi::c_int) as ::core::ffi::c_long;
    bfSize = headersize
        + (*dest).row_width as ::core::ffi::c_long * (*cinfo).output_height as ::core::ffi::c_long;
    memset(
        &raw mut bmpfileheader as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t,
    );
    memset(
        &raw mut bmpinfoheader as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 40]>() as size_t,
    );
    bmpfileheader[0 as ::core::ffi::c_int as usize] = 0x42 as ::core::ffi::c_char;
    bmpfileheader[1 as ::core::ffi::c_int as usize] = 0x4d as ::core::ffi::c_char;
    bmpfileheader[2 as ::core::ffi::c_int as usize] =
        (bfSize & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
    bmpfileheader[(2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (bfSize >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
    bmpfileheader[(2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        (bfSize >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
    bmpfileheader[(2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (bfSize >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
    bmpfileheader[10 as ::core::ffi::c_int as usize] =
        (headersize & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
    bmpfileheader[(10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (headersize >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
            as ::core::ffi::c_char;
    bmpfileheader[(10 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        (headersize >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
            as ::core::ffi::c_char;
    bmpfileheader[(10 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (headersize >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
            as ::core::ffi::c_char;
    bmpinfoheader[0 as ::core::ffi::c_int as usize] =
        (40 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
    bmpinfoheader[(0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (40 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_char;
    bmpinfoheader[4 as ::core::ffi::c_int as usize] =
        ((*cinfo).output_width & 0xff as JDIMENSION) as ::core::ffi::c_char;
    bmpinfoheader[(4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*cinfo).output_width >> 8 as ::core::ffi::c_int & 0xff as JDIMENSION)
            as ::core::ffi::c_char;
    bmpinfoheader[(4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        ((*cinfo).output_width >> 16 as ::core::ffi::c_int & 0xff as JDIMENSION)
            as ::core::ffi::c_char;
    bmpinfoheader[(4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        ((*cinfo).output_width >> 24 as ::core::ffi::c_int & 0xff as JDIMENSION)
            as ::core::ffi::c_char;
    bmpinfoheader[8 as ::core::ffi::c_int as usize] =
        ((*cinfo).output_height & 0xff as JDIMENSION) as ::core::ffi::c_char;
    bmpinfoheader[(8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*cinfo).output_height >> 8 as ::core::ffi::c_int & 0xff as JDIMENSION)
            as ::core::ffi::c_char;
    bmpinfoheader[(8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        ((*cinfo).output_height >> 16 as ::core::ffi::c_int & 0xff as JDIMENSION)
            as ::core::ffi::c_char;
    bmpinfoheader[(8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        ((*cinfo).output_height >> 24 as ::core::ffi::c_int & 0xff as JDIMENSION)
            as ::core::ffi::c_char;
    bmpinfoheader[12 as ::core::ffi::c_int as usize] =
        (1 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
    bmpinfoheader[(12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (1 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_char;
    bmpinfoheader[14 as ::core::ffi::c_int as usize] =
        (bits_per_pixel & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
    bmpinfoheader[(14 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (bits_per_pixel >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_char;
    if (*cinfo).density_unit as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        bmpinfoheader[24 as ::core::ffi::c_int as usize] =
            (((*cinfo).X_density as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
        bmpinfoheader[(24 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
            (((*cinfo).X_density as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                >> 8 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
        bmpinfoheader[(24 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
            (((*cinfo).X_density as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                >> 16 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
        bmpinfoheader[(24 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
            (((*cinfo).X_density as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                >> 24 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
        bmpinfoheader[28 as ::core::ffi::c_int as usize] =
            (((*cinfo).Y_density as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
        bmpinfoheader[(28 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
            (((*cinfo).Y_density as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                >> 8 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
        bmpinfoheader[(28 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
            (((*cinfo).Y_density as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                >> 16 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
        bmpinfoheader[(28 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
            (((*cinfo).Y_density as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                >> 24 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
    }
    bmpinfoheader[32 as ::core::ffi::c_int as usize] =
        (cmap_entries & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
    bmpinfoheader[(32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (cmap_entries >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_char;
    if fwrite(
        &raw mut bmpfileheader as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as size_t,
        14 as size_t,
        (*dest).pub_0.output_file,
    ) as size_t
        != 14 as ::core::ffi::c_int as size_t
    {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if fwrite(
        &raw mut bmpinfoheader as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as size_t,
        40 as size_t,
        (*dest).pub_0.output_file,
    ) as size_t
        != 40 as ::core::ffi::c_int as size_t
    {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if cmap_entries > 0 as ::core::ffi::c_int {
        write_colormap(cinfo, dest, cmap_entries, 4 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn write_os2_header(mut cinfo: j_decompress_ptr, mut dest: bmp_dest_ptr) {
    let mut bmpfileheader: [::core::ffi::c_char; 14] = [0; 14];
    let mut bmpcoreheader: [::core::ffi::c_char; 12] = [0; 12];
    let mut headersize: ::core::ffi::c_long = 0;
    let mut bfSize: ::core::ffi::c_long = 0;
    let mut bits_per_pixel: ::core::ffi::c_int = 0;
    let mut cmap_entries: ::core::ffi::c_int = 0;
    if (*cinfo).out_color_space as ::core::ffi::c_uint
        == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cinfo).out_color_space as ::core::ffi::c_uint
            >= JCS_EXT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*cinfo).out_color_space as ::core::ffi::c_uint
                <= JCS_EXT_ARGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*cinfo).quantize_colors != 0 {
            bits_per_pixel = 8 as ::core::ffi::c_int;
            cmap_entries = 256 as ::core::ffi::c_int;
        } else {
            bits_per_pixel = 24 as ::core::ffi::c_int;
            cmap_entries = 0 as ::core::ffi::c_int;
        }
    } else if (*cinfo).out_color_space as ::core::ffi::c_uint
        == JCS_RGB565 as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cinfo).out_color_space as ::core::ffi::c_uint
            == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        bits_per_pixel = 24 as ::core::ffi::c_int;
        cmap_entries = 0 as ::core::ffi::c_int;
    } else {
        bits_per_pixel = 8 as ::core::ffi::c_int;
        cmap_entries = 256 as ::core::ffi::c_int;
    }
    headersize = (14 as ::core::ffi::c_int
        + 12 as ::core::ffi::c_int
        + cmap_entries * 3 as ::core::ffi::c_int) as ::core::ffi::c_long;
    bfSize = headersize
        + (*dest).row_width as ::core::ffi::c_long * (*cinfo).output_height as ::core::ffi::c_long;
    memset(
        &raw mut bmpfileheader as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t,
    );
    memset(
        &raw mut bmpcoreheader as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t,
    );
    bmpfileheader[0 as ::core::ffi::c_int as usize] = 0x42 as ::core::ffi::c_char;
    bmpfileheader[1 as ::core::ffi::c_int as usize] = 0x4d as ::core::ffi::c_char;
    bmpfileheader[2 as ::core::ffi::c_int as usize] =
        (bfSize & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
    bmpfileheader[(2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (bfSize >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
    bmpfileheader[(2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        (bfSize >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
    bmpfileheader[(2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (bfSize >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
    bmpfileheader[10 as ::core::ffi::c_int as usize] =
        (headersize & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
    bmpfileheader[(10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (headersize >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
            as ::core::ffi::c_char;
    bmpfileheader[(10 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        (headersize >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
            as ::core::ffi::c_char;
    bmpfileheader[(10 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (headersize >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
            as ::core::ffi::c_char;
    bmpcoreheader[0 as ::core::ffi::c_int as usize] =
        (12 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
    bmpcoreheader[(0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (12 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_char;
    bmpcoreheader[4 as ::core::ffi::c_int as usize] =
        ((*cinfo).output_width & 0xff as JDIMENSION) as ::core::ffi::c_char;
    bmpcoreheader[(4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*cinfo).output_width >> 8 as ::core::ffi::c_int & 0xff as JDIMENSION)
            as ::core::ffi::c_char;
    bmpcoreheader[6 as ::core::ffi::c_int as usize] =
        ((*cinfo).output_height & 0xff as JDIMENSION) as ::core::ffi::c_char;
    bmpcoreheader[(6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*cinfo).output_height >> 8 as ::core::ffi::c_int & 0xff as JDIMENSION)
            as ::core::ffi::c_char;
    bmpcoreheader[8 as ::core::ffi::c_int as usize] =
        (1 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
    bmpcoreheader[(8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (1 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_char;
    bmpcoreheader[10 as ::core::ffi::c_int as usize] =
        (bits_per_pixel & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
    bmpcoreheader[(10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (bits_per_pixel >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_char;
    if fwrite(
        &raw mut bmpfileheader as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as size_t,
        14 as size_t,
        (*dest).pub_0.output_file,
    ) as size_t
        != 14 as ::core::ffi::c_int as size_t
    {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if fwrite(
        &raw mut bmpcoreheader as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as size_t,
        12 as size_t,
        (*dest).pub_0.output_file,
    ) as size_t
        != 12 as ::core::ffi::c_int as size_t
    {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if cmap_entries > 0 as ::core::ffi::c_int {
        write_colormap(cinfo, dest, cmap_entries, 3 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn write_colormap(
    mut cinfo: j_decompress_ptr,
    mut dest: bmp_dest_ptr,
    mut map_colors: ::core::ffi::c_int,
    mut map_entry_size: ::core::ffi::c_int,
) {
    let mut colormap: JSAMPARRAY = (*cinfo).colormap;
    let mut num_colors: ::core::ffi::c_int = (*cinfo).actual_number_of_colors;
    let mut outfile: *mut FILE = (*dest).pub_0.output_file;
    let mut i: ::core::ffi::c_int = 0;
    if !colormap.is_null() {
        if (*cinfo).out_color_components == 3 as ::core::ffi::c_int {
            i = 0 as ::core::ffi::c_int;
            while i < num_colors {
                putc(
                    *(*colormap.offset(2 as ::core::ffi::c_int as isize)).offset(i as isize)
                        as ::core::ffi::c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(1 as ::core::ffi::c_int as isize)).offset(i as isize)
                        as ::core::ffi::c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(0 as ::core::ffi::c_int as isize)).offset(i as isize)
                        as ::core::ffi::c_int,
                    outfile,
                );
                if map_entry_size == 4 as ::core::ffi::c_int {
                    putc(0 as ::core::ffi::c_int, outfile);
                }
                i += 1;
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < num_colors {
                putc(
                    *(*colormap.offset(0 as ::core::ffi::c_int as isize)).offset(i as isize)
                        as ::core::ffi::c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(0 as ::core::ffi::c_int as isize)).offset(i as isize)
                        as ::core::ffi::c_int,
                    outfile,
                );
                putc(
                    *(*colormap.offset(0 as ::core::ffi::c_int as isize)).offset(i as isize)
                        as ::core::ffi::c_int,
                    outfile,
                );
                if map_entry_size == 4 as ::core::ffi::c_int {
                    putc(0 as ::core::ffi::c_int, outfile);
                }
                i += 1;
            }
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            putc(i, outfile);
            putc(i, outfile);
            putc(i, outfile);
            if map_entry_size == 4 as ::core::ffi::c_int {
                putc(0 as ::core::ffi::c_int, outfile);
            }
            i += 1;
        }
    }
    if i > map_colors {
        (*(*cinfo).err).msg_code = JERR_TOO_MANY_COLORS as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = i;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    while i < map_colors {
        putc(0 as ::core::ffi::c_int, outfile);
        putc(0 as ::core::ffi::c_int, outfile);
        putc(0 as ::core::ffi::c_int, outfile);
        if map_entry_size == 4 as ::core::ffi::c_int {
            putc(0 as ::core::ffi::c_int, outfile);
        }
        i += 1;
    }
}
unsafe extern "C" fn start_output_bmp(mut cinfo: j_decompress_ptr, mut dinfo: djpeg_dest_ptr) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    if (*dest).use_inversion_array == 0 {
        if (*dest).is_os2 != 0 {
            write_os2_header(cinfo, dest);
        } else {
            write_bmp_header(cinfo, dest);
        }
    }
}
unsafe extern "C" fn finish_output_bmp(mut cinfo: j_decompress_ptr, mut dinfo: djpeg_dest_ptr) {
    let mut dest: bmp_dest_ptr = dinfo as bmp_dest_ptr;
    let mut outfile: *mut FILE = (*dest).pub_0.output_file;
    let mut image_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    let mut data_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut row: JDIMENSION = 0;
    let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
    if (*dest).use_inversion_array != 0 {
        if (*dest).is_os2 != 0 {
            write_os2_header(cinfo, dest);
        } else {
            write_bmp_header(cinfo, dest);
        }
        row = (*cinfo).output_height;
        while row > 0 as JDIMENSION {
            if !progress.is_null() {
                (*progress).pub_0.pass_counter =
                    (*cinfo).output_height.wrapping_sub(row) as ::core::ffi::c_long;
                (*progress).pub_0.pass_limit = (*cinfo).output_height as ::core::ffi::c_long;
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
                (*dest).whole_image,
                row.wrapping_sub(1 as JDIMENSION),
                1 as ::core::ffi::c_int as JDIMENSION,
                FALSE,
            );
            data_ptr = *image_ptr.offset(0 as ::core::ffi::c_int as isize);
            fwrite(
                data_ptr as *const ::core::ffi::c_void,
                1 as size_t,
                (*dest).row_width as size_t,
                outfile,
            );
            row = row.wrapping_sub(1);
        }
        if !progress.is_null() {
            (*progress).completed_extra_passes += 1;
        }
    }
    fflush(outfile);
    if ferror(outfile) != 0 {
        (*(*cinfo).err).msg_code = JERR_FILE_WRITE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jinit_write_bmp(
    mut cinfo: j_decompress_ptr,
    mut is_os2: boolean,
    mut use_inversion_array: boolean,
) -> djpeg_dest_ptr {
    let mut dest: bmp_dest_ptr = ::core::ptr::null_mut::<bmp_dest_struct>();
    let mut row_width: JDIMENSION = 0;
    dest = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<bmp_dest_struct>() as size_t,
    ) as bmp_dest_ptr;
    (*dest).pub_0.start_output =
        Some(start_output_bmp as unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ())
            as Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ()>;
    (*dest).pub_0.finish_output =
        Some(finish_output_bmp as unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ())
            as Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ()>;
    (*dest).pub_0.calc_buffer_dimensions = None;
    (*dest).is_os2 = is_os2;
    if (*cinfo).out_color_space as ::core::ffi::c_uint
        == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*dest).pub_0.put_pixel_rows = Some(
            put_gray_rows
                as unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> (),
        )
            as Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> ()>;
    } else if (*cinfo).out_color_space as ::core::ffi::c_uint
        == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cinfo).out_color_space as ::core::ffi::c_uint
            >= JCS_EXT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*cinfo).out_color_space as ::core::ffi::c_uint
                <= JCS_EXT_ARGB as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*cinfo).quantize_colors != 0 {
            (*dest).pub_0.put_pixel_rows = Some(
                put_gray_rows
                    as unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> (),
            )
                as Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> ()>;
        } else {
            (*dest).pub_0.put_pixel_rows = Some(
                put_pixel_rows
                    as unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> (),
            )
                as Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> ()>;
        }
    } else if (*cinfo).quantize_colors == 0
        && ((*cinfo).out_color_space as ::core::ffi::c_uint
            == JCS_RGB565 as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*cinfo).out_color_space as ::core::ffi::c_uint
                == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        (*dest).pub_0.put_pixel_rows = Some(
            put_pixel_rows
                as unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> (),
        )
            as Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> ()>;
    } else {
        (*(*cinfo).err).msg_code = JERR_BMP_COLORSPACE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    jpeg_calc_output_dimensions(cinfo);
    if (*cinfo).out_color_space as ::core::ffi::c_uint
        == JCS_RGB565 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        row_width = (*cinfo).output_width.wrapping_mul(2 as JDIMENSION);
        (*dest).data_width = (*cinfo).output_width.wrapping_mul(3 as JDIMENSION);
        (*dest).row_width = (*dest).data_width;
        while row_width & 3 as JDIMENSION != 0 as JDIMENSION {
            row_width = row_width.wrapping_add(1);
        }
    } else if (*cinfo).quantize_colors == 0
        && ((*cinfo).out_color_space as ::core::ffi::c_uint
            == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*cinfo).out_color_space as ::core::ffi::c_uint
                >= JCS_EXT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*cinfo).out_color_space as ::core::ffi::c_uint
                    <= JCS_EXT_ARGB as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*cinfo).out_color_space as ::core::ffi::c_uint
                == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        row_width = (*cinfo)
            .output_width
            .wrapping_mul((*cinfo).output_components as JDIMENSION);
        (*dest).data_width = (*cinfo).output_width.wrapping_mul(3 as JDIMENSION);
        (*dest).row_width = (*dest).data_width;
    } else {
        row_width = (*cinfo)
            .output_width
            .wrapping_mul((*cinfo).output_components as JDIMENSION);
        (*dest).data_width = row_width;
        (*dest).row_width = (*dest).data_width;
    }
    while (*dest).row_width & 3 as JDIMENSION != 0 as JDIMENSION {
        (*dest).row_width = (*dest).row_width.wrapping_add(1);
    }
    (*dest).pad_bytes = (*dest).row_width.wrapping_sub((*dest).data_width) as ::core::ffi::c_int;
    if use_inversion_array != 0 {
        (*dest).whole_image = Some(
            (*(*cinfo).mem)
                .request_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            FALSE,
            (*dest).row_width,
            (*cinfo).output_height,
            1 as ::core::ffi::c_int as JDIMENSION,
        );
        (*dest).cur_output_row = 0 as JDIMENSION;
        if !(*cinfo).progress.is_null() {
            let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
            (*progress).total_extra_passes += 1;
        }
    } else {
        (*dest).iobuffer = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (*dest).row_width as size_t,
        ) as *mut JSAMPLE;
    }
    (*dest).use_inversion_array = use_inversion_array;
    (*dest).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        row_width,
        1 as ::core::ffi::c_int as JDIMENSION,
    );
    (*dest).pub_0.buffer_height = 1 as JDIMENSION;
    return dest as djpeg_dest_ptr;
}
