#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}

extern "C" {
    fn jpeg_fdct_islow(data: *mut DCTELEM);
    fn jpeg_fdct_ifast(data: *mut DCTELEM);
    fn jpeg_fdct_float(data: *mut ::core::ffi::c_float);
    fn jsimd_can_convsamp() -> ::core::ffi::c_int;
    fn jsimd_can_convsamp_float() -> ::core::ffi::c_int;
    fn jsimd_convsamp(sample_data: JSAMPARRAY, start_col: JDIMENSION, workspace: *mut DCTELEM);
    fn jsimd_convsamp_float(
        sample_data: JSAMPARRAY,
        start_col: JDIMENSION,
        workspace: *mut ::core::ffi::c_float,
    );
    fn jsimd_can_fdct_islow() -> ::core::ffi::c_int;
    fn jsimd_can_fdct_ifast() -> ::core::ffi::c_int;
    fn jsimd_can_fdct_float() -> ::core::ffi::c_int;
    fn jsimd_fdct_islow(data: *mut DCTELEM);
    fn jsimd_fdct_ifast(data: *mut DCTELEM);
    fn jsimd_fdct_float(data: *mut ::core::ffi::c_float);
    fn jsimd_can_quantize() -> ::core::ffi::c_int;
    fn jsimd_can_quantize_float() -> ::core::ffi::c_int;
    fn jsimd_quantize(coef_block: JCOEFPTR, divisors: *mut DCTELEM, workspace: *mut DCTELEM);
    fn jsimd_quantize_float(
        coef_block: JCOEFPTR,
        divisors: *mut ::core::ffi::c_float,
        workspace: *mut ::core::ffi::c_float,
    );
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
pub type my_fdct_ptr = *mut my_fdct_controller;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_fdct_controller {
    pub pub_0: jpeg_forward_dct,
    pub dct: forward_DCT_method_ptr,
    pub convsamp: convsamp_method_ptr,
    pub quantize: quantize_method_ptr,
    pub divisors: [*mut DCTELEM; 4],
    pub workspace: *mut DCTELEM,
    pub float_dct: float_DCT_method_ptr,
    pub float_convsamp: float_convsamp_method_ptr,
    pub float_quantize: float_quantize_method_ptr,
    pub float_divisors: [*mut ::core::ffi::c_float; 4],
    pub float_workspace: *mut ::core::ffi::c_float,
}
pub type float_quantize_method_ptr = Option<
    unsafe extern "C" fn(JCOEFPTR, *mut ::core::ffi::c_float, *mut ::core::ffi::c_float) -> (),
>;
pub type float_convsamp_method_ptr =
    Option<unsafe extern "C" fn(JSAMPARRAY, JDIMENSION, *mut ::core::ffi::c_float) -> ()>;
pub type float_DCT_method_ptr = Option<unsafe extern "C" fn(*mut ::core::ffi::c_float) -> ()>;
pub type DCTELEM = ::core::ffi::c_short;
pub type quantize_method_ptr =
    Option<unsafe extern "C" fn(JCOEFPTR, *mut DCTELEM, *mut DCTELEM) -> ()>;
pub type convsamp_method_ptr =
    Option<unsafe extern "C" fn(JSAMPARRAY, JDIMENSION, *mut DCTELEM) -> ()>;
pub type forward_DCT_method_ptr = Option<unsafe extern "C" fn(*mut DCTELEM) -> ()>;
pub const JERR_NOT_COMPILED: C2RustUnnamed_0 = 49;
pub type UDCTELEM2 = ::core::ffi::c_uint;
pub type UDCTELEM = ::core::ffi::c_ushort;
pub const JERR_NO_QUANT_TABLE: C2RustUnnamed_0 = 54;
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
pub const JERR_NO_IMAGE: C2RustUnnamed_0 = 53;
pub const JERR_NO_HUFF_TABLE: C2RustUnnamed_0 = 52;
pub const JERR_NO_BACKING_STORE: C2RustUnnamed_0 = 51;
pub const JERR_NO_ARITH_TABLE: C2RustUnnamed_0 = 50;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const CENTERJSAMPLE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const DCTSIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const DCTSIZE2: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NUM_QUANT_TBLS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn flss(mut val: UINT16) -> ::core::ffi::c_int {
    let mut bit: ::core::ffi::c_int = 0;
    bit = 16 as ::core::ffi::c_int;
    if val == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if val as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int == 0 {
        bit -= 8 as ::core::ffi::c_int;
        val = ((val as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UINT16;
    }
    if val as ::core::ffi::c_int & 0xf000 as ::core::ffi::c_int == 0 {
        bit -= 4 as ::core::ffi::c_int;
        val = ((val as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as UINT16;
    }
    if val as ::core::ffi::c_int & 0xc000 as ::core::ffi::c_int == 0 {
        bit -= 2 as ::core::ffi::c_int;
        val = ((val as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as UINT16;
    }
    if val as ::core::ffi::c_int & 0x8000 as ::core::ffi::c_int == 0 {
        bit -= 1 as ::core::ffi::c_int;
        val = ((val as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as UINT16;
    }
    return bit;
}
unsafe extern "C" fn compute_reciprocal(
    mut divisor: UINT16,
    mut dtbl: *mut DCTELEM,
) -> ::core::ffi::c_int {
    let mut fq: UDCTELEM2 = 0;
    let mut fr: UDCTELEM2 = 0;
    let mut c: UDCTELEM = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    if divisor as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        *dtbl.offset((DCTSIZE2 * 0 as ::core::ffi::c_int) as isize) =
            1 as ::core::ffi::c_int as DCTELEM;
        *dtbl.offset((DCTSIZE2 * 1 as ::core::ffi::c_int) as isize) =
            0 as ::core::ffi::c_int as DCTELEM;
        *dtbl.offset((DCTSIZE2 * 2 as ::core::ffi::c_int) as isize) =
            1 as ::core::ffi::c_int as DCTELEM;
        *dtbl.offset((DCTSIZE2 * 3 as ::core::ffi::c_int) as isize) =
            -((::core::mem::size_of::<DCTELEM>() as usize).wrapping_mul(8 as usize) as DCTELEM
                as ::core::ffi::c_int) as DCTELEM;
        return 0 as ::core::ffi::c_int;
    }
    b = flss(divisor) - 1 as ::core::ffi::c_int;
    r = (::core::mem::size_of::<DCTELEM>() as usize)
        .wrapping_mul(8 as usize)
        .wrapping_add(b as usize) as ::core::ffi::c_int;
    fq = ((1 as ::core::ffi::c_int as UDCTELEM2) << r).wrapping_div(divisor as UDCTELEM2);
    fr = ((1 as ::core::ffi::c_int as UDCTELEM2) << r).wrapping_rem(divisor as UDCTELEM2);
    c = (divisor as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as UDCTELEM;
    if fr == 0 as UDCTELEM2 {
        fq >>= 1 as ::core::ffi::c_int;
        r -= 1;
    } else if fr <= (divisor as ::core::ffi::c_uint).wrapping_div(2 as ::core::ffi::c_uint) {
        c = c.wrapping_add(1);
    } else {
        fq = fq.wrapping_add(1);
    }
    *dtbl.offset((DCTSIZE2 * 0 as ::core::ffi::c_int) as isize) = fq as DCTELEM;
    *dtbl.offset((DCTSIZE2 * 1 as ::core::ffi::c_int) as isize) = c as DCTELEM;
    *dtbl.offset((DCTSIZE2 * 2 as ::core::ffi::c_int) as isize) = ((1 as ::core::ffi::c_int)
        << (::core::mem::size_of::<DCTELEM>() as usize)
            .wrapping_mul(8 as usize)
            .wrapping_mul(2 as usize)
            .wrapping_sub(r as usize))
        as DCTELEM;
    *dtbl.offset((DCTSIZE2 * 3 as ::core::ffi::c_int) as isize) = (r as DCTELEM as usize)
        .wrapping_sub((::core::mem::size_of::<DCTELEM>() as usize).wrapping_mul(8 as usize))
        as DCTELEM;
    if r <= 16 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    } else {
        return 1 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn start_pass_fdctmgr(mut cinfo: j_compress_ptr) {
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr;
    let mut ci: ::core::ffi::c_int = 0;
    let mut qtblno: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut qtbl: *mut JQUANT_TBL = ::core::ptr::null_mut::<JQUANT_TBL>();
    let mut dtbl: *mut DCTELEM = ::core::ptr::null_mut::<DCTELEM>();
    ci = 0 as ::core::ffi::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        qtblno = (*compptr).quant_tbl_no;
        if qtblno < 0 as ::core::ffi::c_int
            || qtblno >= NUM_QUANT_TBLS
            || (*cinfo).quant_tbl_ptrs[qtblno as usize].is_null()
        {
            (*(*cinfo).err).msg_code = JERR_NO_QUANT_TABLE as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = qtblno;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        qtbl = (*cinfo).quant_tbl_ptrs[qtblno as usize];
        match (*cinfo).dct_method as ::core::ffi::c_uint {
            0 => {
                if (*fdct).divisors[qtblno as usize].is_null() {
                    (*fdct).divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        JPOOL_IMAGE,
                        ((DCTSIZE2 * 4 as ::core::ffi::c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<DCTELEM>() as size_t),
                    ) as *mut DCTELEM;
                }
                dtbl = (*fdct).divisors[qtblno as usize];
                i = 0 as ::core::ffi::c_int;
                while i < DCTSIZE2 {
                    if compute_reciprocal(
                        (((*qtbl).quantval[i as usize] as ::core::ffi::c_int)
                            << 3 as ::core::ffi::c_int) as UINT16,
                        dtbl.offset(i as isize) as *mut DCTELEM,
                    ) == 0
                        && (*fdct).quantize
                            == Some(
                                jsimd_quantize
                                    as unsafe extern "C" fn(
                                        JCOEFPTR,
                                        *mut DCTELEM,
                                        *mut DCTELEM,
                                    )
                                        -> (),
                            )
                    {
                        (*fdct).quantize = Some(
                            quantize
                                as unsafe extern "C" fn(JCOEFPTR, *mut DCTELEM, *mut DCTELEM) -> (),
                        ) as quantize_method_ptr;
                    }
                    i += 1;
                }
            }
            1 => {
                static mut aanscales: [INT16; 64] = [
                    16384 as ::core::ffi::c_int as INT16,
                    22725 as ::core::ffi::c_int as INT16,
                    21407 as ::core::ffi::c_int as INT16,
                    19266 as ::core::ffi::c_int as INT16,
                    16384 as ::core::ffi::c_int as INT16,
                    12873 as ::core::ffi::c_int as INT16,
                    8867 as ::core::ffi::c_int as INT16,
                    4520 as ::core::ffi::c_int as INT16,
                    22725 as ::core::ffi::c_int as INT16,
                    31521 as ::core::ffi::c_int as INT16,
                    29692 as ::core::ffi::c_int as INT16,
                    26722 as ::core::ffi::c_int as INT16,
                    22725 as ::core::ffi::c_int as INT16,
                    17855 as ::core::ffi::c_int as INT16,
                    12299 as ::core::ffi::c_int as INT16,
                    6270 as ::core::ffi::c_int as INT16,
                    21407 as ::core::ffi::c_int as INT16,
                    29692 as ::core::ffi::c_int as INT16,
                    27969 as ::core::ffi::c_int as INT16,
                    25172 as ::core::ffi::c_int as INT16,
                    21407 as ::core::ffi::c_int as INT16,
                    16819 as ::core::ffi::c_int as INT16,
                    11585 as ::core::ffi::c_int as INT16,
                    5906 as ::core::ffi::c_int as INT16,
                    19266 as ::core::ffi::c_int as INT16,
                    26722 as ::core::ffi::c_int as INT16,
                    25172 as ::core::ffi::c_int as INT16,
                    22654 as ::core::ffi::c_int as INT16,
                    19266 as ::core::ffi::c_int as INT16,
                    15137 as ::core::ffi::c_int as INT16,
                    10426 as ::core::ffi::c_int as INT16,
                    5315 as ::core::ffi::c_int as INT16,
                    16384 as ::core::ffi::c_int as INT16,
                    22725 as ::core::ffi::c_int as INT16,
                    21407 as ::core::ffi::c_int as INT16,
                    19266 as ::core::ffi::c_int as INT16,
                    16384 as ::core::ffi::c_int as INT16,
                    12873 as ::core::ffi::c_int as INT16,
                    8867 as ::core::ffi::c_int as INT16,
                    4520 as ::core::ffi::c_int as INT16,
                    12873 as ::core::ffi::c_int as INT16,
                    17855 as ::core::ffi::c_int as INT16,
                    16819 as ::core::ffi::c_int as INT16,
                    15137 as ::core::ffi::c_int as INT16,
                    12873 as ::core::ffi::c_int as INT16,
                    10114 as ::core::ffi::c_int as INT16,
                    6967 as ::core::ffi::c_int as INT16,
                    3552 as ::core::ffi::c_int as INT16,
                    8867 as ::core::ffi::c_int as INT16,
                    12299 as ::core::ffi::c_int as INT16,
                    11585 as ::core::ffi::c_int as INT16,
                    10426 as ::core::ffi::c_int as INT16,
                    8867 as ::core::ffi::c_int as INT16,
                    6967 as ::core::ffi::c_int as INT16,
                    4799 as ::core::ffi::c_int as INT16,
                    2446 as ::core::ffi::c_int as INT16,
                    4520 as ::core::ffi::c_int as INT16,
                    6270 as ::core::ffi::c_int as INT16,
                    5906 as ::core::ffi::c_int as INT16,
                    5315 as ::core::ffi::c_int as INT16,
                    4520 as ::core::ffi::c_int as INT16,
                    3552 as ::core::ffi::c_int as INT16,
                    2446 as ::core::ffi::c_int as INT16,
                    1247 as ::core::ffi::c_int as INT16,
                ];
                if (*fdct).divisors[qtblno as usize].is_null() {
                    (*fdct).divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        JPOOL_IMAGE,
                        ((DCTSIZE2 * 4 as ::core::ffi::c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<DCTELEM>() as size_t),
                    ) as *mut DCTELEM;
                }
                dtbl = (*fdct).divisors[qtblno as usize];
                i = 0 as ::core::ffi::c_int;
                while i < DCTSIZE2 {
                    if compute_reciprocal(
                        ((*qtbl).quantval[i as usize] as JLONG * aanscales[i as usize] as JLONG
                            + ((1 as ::core::ffi::c_int as JLONG)
                                << 14 as ::core::ffi::c_int
                                    - 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                            >> 14 as ::core::ffi::c_int - 3 as ::core::ffi::c_int)
                            as UINT16,
                        dtbl.offset(i as isize) as *mut DCTELEM,
                    ) == 0
                        && (*fdct).quantize
                            == Some(
                                jsimd_quantize
                                    as unsafe extern "C" fn(
                                        JCOEFPTR,
                                        *mut DCTELEM,
                                        *mut DCTELEM,
                                    )
                                        -> (),
                            )
                    {
                        (*fdct).quantize = Some(
                            quantize
                                as unsafe extern "C" fn(JCOEFPTR, *mut DCTELEM, *mut DCTELEM) -> (),
                        ) as quantize_method_ptr;
                    }
                    i += 1;
                }
            }
            2 => {
                let mut fdtbl: *mut ::core::ffi::c_float =
                    ::core::ptr::null_mut::<::core::ffi::c_float>();
                let mut row: ::core::ffi::c_int = 0;
                let mut col: ::core::ffi::c_int = 0;
                static mut aanscalefactor: [::core::ffi::c_double; 8] = [
                    1.0f64,
                    1.387039845f64,
                    1.306562965f64,
                    1.175875602f64,
                    1.0f64,
                    0.785694958f64,
                    0.541196100f64,
                    0.275899379f64,
                ];
                if (*fdct).float_divisors[qtblno as usize].is_null() {
                    (*fdct).float_divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        JPOOL_IMAGE,
                        (DCTSIZE2 as size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t),
                    )
                        as *mut ::core::ffi::c_float;
                }
                fdtbl = (*fdct).float_divisors[qtblno as usize];
                i = 0 as ::core::ffi::c_int;
                row = 0 as ::core::ffi::c_int;
                while row < DCTSIZE {
                    col = 0 as ::core::ffi::c_int;
                    while col < DCTSIZE {
                        *fdtbl.offset(i as isize) = (1.0f64
                            / ((*qtbl).quantval[i as usize] as ::core::ffi::c_double
                                * aanscalefactor[row as usize]
                                * aanscalefactor[col as usize]
                                * 8.0f64))
                            as ::core::ffi::c_float;
                        i += 1;
                        col += 1;
                    }
                    row += 1;
                }
            }
            _ => {
                (*(*cinfo).err).msg_code = JERR_NOT_COMPILED as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        ci += 1;
        compptr = compptr.offset(1);
    }
}
unsafe extern "C" fn convsamp(
    mut sample_data: JSAMPARRAY,
    mut start_col: JDIMENSION,
    mut workspace: *mut DCTELEM,
) {
    let mut workspaceptr: *mut DCTELEM = ::core::ptr::null_mut::<DCTELEM>();
    let mut elemptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut elemr: ::core::ffi::c_int = 0;
    workspaceptr = workspace;
    elemr = 0 as ::core::ffi::c_int;
    while elemr < DCTSIZE {
        elemptr = (*sample_data.offset(elemr as isize)).offset(start_col as isize);
        let fresh16 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh17 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh17 = (*fresh16 as ::core::ffi::c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh18 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh19 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh19 = (*fresh18 as ::core::ffi::c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh20 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh21 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh21 = (*fresh20 as ::core::ffi::c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh22 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh23 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh23 = (*fresh22 as ::core::ffi::c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh24 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh25 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh25 = (*fresh24 as ::core::ffi::c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh26 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh27 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh27 = (*fresh26 as ::core::ffi::c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh28 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh29 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh29 = (*fresh28 as ::core::ffi::c_int - CENTERJSAMPLE) as DCTELEM;
        let fresh30 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh31 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh31 = (*fresh30 as ::core::ffi::c_int - CENTERJSAMPLE) as DCTELEM;
        elemr += 1;
    }
}
unsafe extern "C" fn quantize(
    mut coef_block: JCOEFPTR,
    mut divisors: *mut DCTELEM,
    mut workspace: *mut DCTELEM,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut temp: DCTELEM = 0;
    let mut output_ptr: JCOEFPTR = coef_block;
    let mut recip: UDCTELEM = 0;
    let mut corr: UDCTELEM = 0;
    let mut shift: ::core::ffi::c_int = 0;
    let mut product: UDCTELEM2 = 0;
    i = 0 as ::core::ffi::c_int;
    while i < DCTSIZE2 {
        temp = *workspace.offset(i as isize);
        recip = *divisors.offset((i + DCTSIZE2 * 0 as ::core::ffi::c_int) as isize) as UDCTELEM;
        corr = *divisors.offset((i + DCTSIZE2 * 1 as ::core::ffi::c_int) as isize) as UDCTELEM;
        shift = *divisors.offset((i + DCTSIZE2 * 3 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        if (temp as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            temp = -(temp as ::core::ffi::c_int) as DCTELEM;
            product = ((temp as ::core::ffi::c_int + corr as ::core::ffi::c_int) as UDCTELEM2)
                .wrapping_mul(recip as UDCTELEM2);
            product >>= (shift as usize).wrapping_add(
                (::core::mem::size_of::<DCTELEM>() as usize).wrapping_mul(8 as usize),
            );
            temp = product as DCTELEM;
            temp = -(temp as ::core::ffi::c_int) as DCTELEM;
        } else {
            product = ((temp as ::core::ffi::c_int + corr as ::core::ffi::c_int) as UDCTELEM2)
                .wrapping_mul(recip as UDCTELEM2);
            product >>= (shift as usize).wrapping_add(
                (::core::mem::size_of::<DCTELEM>() as usize).wrapping_mul(8 as usize),
            );
            temp = product as DCTELEM;
        }
        *output_ptr.offset(i as isize) = temp;
        i += 1;
    }
}
unsafe extern "C" fn forward_DCT(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut sample_data: JSAMPARRAY,
    mut coef_blocks: JBLOCKROW,
    mut start_row: JDIMENSION,
    mut start_col: JDIMENSION,
    mut num_blocks: JDIMENSION,
) {
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr;
    let mut divisors: *mut DCTELEM = (*fdct).divisors[(*compptr).quant_tbl_no as usize];
    let mut workspace: *mut DCTELEM = ::core::ptr::null_mut::<DCTELEM>();
    let mut bi: JDIMENSION = 0;
    let mut do_dct: forward_DCT_method_ptr = (*fdct).dct;
    let mut do_convsamp: convsamp_method_ptr = (*fdct).convsamp;
    let mut do_quantize: quantize_method_ptr = (*fdct).quantize;
    workspace = (*fdct).workspace;
    sample_data = sample_data.offset(start_row as isize);
    bi = 0 as JDIMENSION;
    while bi < num_blocks {
        Some(do_convsamp.expect("non-null function pointer")).expect("non-null function pointer")(
            sample_data,
            start_col,
            workspace,
        );
        Some(do_dct.expect("non-null function pointer")).expect("non-null function pointer")(
            workspace,
        );
        Some(do_quantize.expect("non-null function pointer")).expect("non-null function pointer")(
            &raw mut *coef_blocks.offset(bi as isize) as JCOEFPTR,
            divisors,
            workspace,
        );
        bi = bi.wrapping_add(1);
        start_col = start_col.wrapping_add(DCTSIZE as JDIMENSION);
    }
}
unsafe extern "C" fn convsamp_float(
    mut sample_data: JSAMPARRAY,
    mut start_col: JDIMENSION,
    mut workspace: *mut ::core::ffi::c_float,
) {
    let mut workspaceptr: *mut ::core::ffi::c_float =
        ::core::ptr::null_mut::<::core::ffi::c_float>();
    let mut elemptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut elemr: ::core::ffi::c_int = 0;
    workspaceptr = workspace;
    elemr = 0 as ::core::ffi::c_int;
    while elemr < DCTSIZE {
        elemptr = (*sample_data.offset(elemr as isize)).offset(start_col as isize);
        let fresh0 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh1 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh1 = (*fresh0 as ::core::ffi::c_int - CENTERJSAMPLE) as ::core::ffi::c_float;
        let fresh2 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh3 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh3 = (*fresh2 as ::core::ffi::c_int - CENTERJSAMPLE) as ::core::ffi::c_float;
        let fresh4 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh5 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh5 = (*fresh4 as ::core::ffi::c_int - CENTERJSAMPLE) as ::core::ffi::c_float;
        let fresh6 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh7 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh7 = (*fresh6 as ::core::ffi::c_int - CENTERJSAMPLE) as ::core::ffi::c_float;
        let fresh8 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh9 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh9 = (*fresh8 as ::core::ffi::c_int - CENTERJSAMPLE) as ::core::ffi::c_float;
        let fresh10 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh11 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh11 = (*fresh10 as ::core::ffi::c_int - CENTERJSAMPLE) as ::core::ffi::c_float;
        let fresh12 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh13 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh13 = (*fresh12 as ::core::ffi::c_int - CENTERJSAMPLE) as ::core::ffi::c_float;
        let fresh14 = elemptr;
        elemptr = elemptr.offset(1);
        let fresh15 = workspaceptr;
        workspaceptr = workspaceptr.offset(1);
        *fresh15 = (*fresh14 as ::core::ffi::c_int - CENTERJSAMPLE) as ::core::ffi::c_float;
        elemr += 1;
    }
}
unsafe extern "C" fn quantize_float(
    mut coef_block: JCOEFPTR,
    mut divisors: *mut ::core::ffi::c_float,
    mut workspace: *mut ::core::ffi::c_float,
) {
    let mut temp: ::core::ffi::c_float = 0.;
    let mut i: ::core::ffi::c_int = 0;
    let mut output_ptr: JCOEFPTR = coef_block;
    i = 0 as ::core::ffi::c_int;
    while i < DCTSIZE2 {
        temp = *workspace.offset(i as isize) * *divisors.offset(i as isize);
        *output_ptr.offset(i as isize) = ((temp + 16384.5f64 as ::core::ffi::c_float)
            as ::core::ffi::c_int
            - 16384 as ::core::ffi::c_int) as JCOEF;
        i += 1;
    }
}
unsafe extern "C" fn forward_DCT_float(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut sample_data: JSAMPARRAY,
    mut coef_blocks: JBLOCKROW,
    mut start_row: JDIMENSION,
    mut start_col: JDIMENSION,
    mut num_blocks: JDIMENSION,
) {
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr;
    let mut divisors: *mut ::core::ffi::c_float =
        (*fdct).float_divisors[(*compptr).quant_tbl_no as usize];
    let mut workspace: *mut ::core::ffi::c_float = ::core::ptr::null_mut::<::core::ffi::c_float>();
    let mut bi: JDIMENSION = 0;
    let mut do_dct: float_DCT_method_ptr = (*fdct).float_dct;
    let mut do_convsamp: float_convsamp_method_ptr = (*fdct).float_convsamp;
    let mut do_quantize: float_quantize_method_ptr = (*fdct).float_quantize;
    workspace = (*fdct).float_workspace;
    sample_data = sample_data.offset(start_row as isize);
    bi = 0 as JDIMENSION;
    while bi < num_blocks {
        Some(do_convsamp.expect("non-null function pointer")).expect("non-null function pointer")(
            sample_data,
            start_col,
            workspace,
        );
        Some(do_dct.expect("non-null function pointer")).expect("non-null function pointer")(
            workspace,
        );
        Some(do_quantize.expect("non-null function pointer")).expect("non-null function pointer")(
            &raw mut *coef_blocks.offset(bi as isize) as JCOEFPTR,
            divisors,
            workspace,
        );
        bi = bi.wrapping_add(1);
        start_col = start_col.wrapping_add(DCTSIZE as JDIMENSION);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jinit_forward_dct(mut cinfo: j_compress_ptr) {
    let mut fdct: my_fdct_ptr = ::core::ptr::null_mut::<my_fdct_controller>();
    let mut i: ::core::ffi::c_int = 0;
    fdct = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<my_fdct_controller>() as size_t,
    ) as my_fdct_ptr;
    (*cinfo).fdct = fdct as *mut jpeg_forward_dct as *mut jpeg_forward_dct;
    (*fdct).pub_0.start_pass =
        Some(start_pass_fdctmgr as unsafe extern "C" fn(j_compress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
    match (*cinfo).dct_method as ::core::ffi::c_uint {
        0 => {
            (*fdct).pub_0.forward_DCT = Some(
                forward_DCT
                    as unsafe extern "C" fn(
                        j_compress_ptr,
                        *mut jpeg_component_info,
                        JSAMPARRAY,
                        JBLOCKROW,
                        JDIMENSION,
                        JDIMENSION,
                        JDIMENSION,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        j_compress_ptr,
                        *mut jpeg_component_info,
                        JSAMPARRAY,
                        JBLOCKROW,
                        JDIMENSION,
                        JDIMENSION,
                        JDIMENSION,
                    ) -> (),
                >;
            if jsimd_can_fdct_islow() != 0 {
                (*fdct).dct = Some(jsimd_fdct_islow as unsafe extern "C" fn(*mut DCTELEM) -> ())
                    as forward_DCT_method_ptr;
            } else {
                (*fdct).dct = Some(jpeg_fdct_islow as unsafe extern "C" fn(*mut DCTELEM) -> ())
                    as forward_DCT_method_ptr;
            }
        }
        1 => {
            (*fdct).pub_0.forward_DCT = Some(
                forward_DCT
                    as unsafe extern "C" fn(
                        j_compress_ptr,
                        *mut jpeg_component_info,
                        JSAMPARRAY,
                        JBLOCKROW,
                        JDIMENSION,
                        JDIMENSION,
                        JDIMENSION,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        j_compress_ptr,
                        *mut jpeg_component_info,
                        JSAMPARRAY,
                        JBLOCKROW,
                        JDIMENSION,
                        JDIMENSION,
                        JDIMENSION,
                    ) -> (),
                >;
            if jsimd_can_fdct_ifast() != 0 {
                (*fdct).dct = Some(jsimd_fdct_ifast as unsafe extern "C" fn(*mut DCTELEM) -> ())
                    as forward_DCT_method_ptr;
            } else {
                (*fdct).dct = Some(jpeg_fdct_ifast as unsafe extern "C" fn(*mut DCTELEM) -> ())
                    as forward_DCT_method_ptr;
            }
        }
        2 => {
            (*fdct).pub_0.forward_DCT = Some(
                forward_DCT_float
                    as unsafe extern "C" fn(
                        j_compress_ptr,
                        *mut jpeg_component_info,
                        JSAMPARRAY,
                        JBLOCKROW,
                        JDIMENSION,
                        JDIMENSION,
                        JDIMENSION,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        j_compress_ptr,
                        *mut jpeg_component_info,
                        JSAMPARRAY,
                        JBLOCKROW,
                        JDIMENSION,
                        JDIMENSION,
                        JDIMENSION,
                    ) -> (),
                >;
            if jsimd_can_fdct_float() != 0 {
                (*fdct).float_dct =
                    Some(jsimd_fdct_float as unsafe extern "C" fn(*mut ::core::ffi::c_float) -> ())
                        as float_DCT_method_ptr;
            } else {
                (*fdct).float_dct =
                    Some(jpeg_fdct_float as unsafe extern "C" fn(*mut ::core::ffi::c_float) -> ())
                        as float_DCT_method_ptr;
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_NOT_COMPILED as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    match (*cinfo).dct_method as ::core::ffi::c_uint {
        0 | 1 => {
            if jsimd_can_convsamp() != 0 {
                (*fdct).convsamp = Some(
                    jsimd_convsamp
                        as unsafe extern "C" fn(JSAMPARRAY, JDIMENSION, *mut DCTELEM) -> (),
                ) as convsamp_method_ptr;
            } else {
                (*fdct).convsamp = Some(
                    convsamp as unsafe extern "C" fn(JSAMPARRAY, JDIMENSION, *mut DCTELEM) -> (),
                ) as convsamp_method_ptr;
            }
            if jsimd_can_quantize() != 0 {
                (*fdct).quantize = Some(
                    jsimd_quantize
                        as unsafe extern "C" fn(JCOEFPTR, *mut DCTELEM, *mut DCTELEM) -> (),
                ) as quantize_method_ptr;
            } else {
                (*fdct).quantize = Some(
                    quantize as unsafe extern "C" fn(JCOEFPTR, *mut DCTELEM, *mut DCTELEM) -> (),
                ) as quantize_method_ptr;
            }
        }
        2 => {
            if jsimd_can_convsamp_float() != 0 {
                (*fdct).float_convsamp = Some(
                    jsimd_convsamp_float
                        as unsafe extern "C" fn(
                            JSAMPARRAY,
                            JDIMENSION,
                            *mut ::core::ffi::c_float,
                        ) -> (),
                ) as float_convsamp_method_ptr;
            } else {
                (*fdct).float_convsamp = Some(
                    convsamp_float
                        as unsafe extern "C" fn(
                            JSAMPARRAY,
                            JDIMENSION,
                            *mut ::core::ffi::c_float,
                        ) -> (),
                ) as float_convsamp_method_ptr;
            }
            if jsimd_can_quantize_float() != 0 {
                (*fdct).float_quantize = Some(
                    jsimd_quantize_float
                        as unsafe extern "C" fn(
                            JCOEFPTR,
                            *mut ::core::ffi::c_float,
                            *mut ::core::ffi::c_float,
                        ) -> (),
                ) as float_quantize_method_ptr;
            } else {
                (*fdct).float_quantize = Some(
                    quantize_float
                        as unsafe extern "C" fn(
                            JCOEFPTR,
                            *mut ::core::ffi::c_float,
                            *mut ::core::ffi::c_float,
                        ) -> (),
                ) as float_quantize_method_ptr;
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_NOT_COMPILED as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    if (*cinfo).dct_method as ::core::ffi::c_uint
        == JDCT_FLOAT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*fdct).float_workspace = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (::core::mem::size_of::<::core::ffi::c_float>() as size_t)
                .wrapping_mul(DCTSIZE2 as size_t),
        ) as *mut ::core::ffi::c_float;
    } else {
        (*fdct).workspace = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (::core::mem::size_of::<DCTELEM>() as size_t).wrapping_mul(DCTSIZE2 as size_t),
        ) as *mut DCTELEM;
    }
    i = 0 as ::core::ffi::c_int;
    while i < NUM_QUANT_TBLS {
        (*fdct).divisors[i as usize] = ::core::ptr::null_mut::<DCTELEM>();
        (*fdct).float_divisors[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_float>();
        i += 1;
    }
}

pub const JPEG_RS_JCDCTMGR_LINK_ANCHOR: unsafe extern "C" fn(j_compress_ptr) = jinit_forward_dct;
