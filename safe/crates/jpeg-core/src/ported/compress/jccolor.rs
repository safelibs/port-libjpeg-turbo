#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}

extern "C" {
    fn jsimd_can_rgb_ycc() -> ::core::ffi::c_int;
    fn jsimd_can_rgb_gray() -> ::core::ffi::c_int;
    fn jsimd_rgb_ycc_convert(
        cinfo: j_compress_ptr,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: ::core::ffi::c_int,
    );
    fn jsimd_rgb_gray_convert(
        cinfo: j_compress_ptr,
        input_buf: JSAMPARRAY,
        output_buf: JSAMPIMAGE,
        output_row: JDIMENSION,
        num_rows: ::core::ffi::c_int,
    );
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
pub type my_cconvert_ptr = *mut my_color_converter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_color_converter {
    pub pub_0: jpeg_color_converter,
    pub rgb_ycc_tab: *mut JLONG,
}
pub const JERR_CONVERSION_NOTIMPL: C2RustUnnamed_0 = 28;
pub const JERR_BAD_J_COLORSPACE: C2RustUnnamed_0 = 11;
pub const JERR_BAD_IN_COLORSPACE: C2RustUnnamed_0 = 10;
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
pub const MAXJSAMPLE: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const CENTERJSAMPLE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
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
pub const SCALEBITS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const CBCR_OFFSET: JLONG = (CENTERJSAMPLE as JLONG) << SCALEBITS;
pub const ONE_HALF: JLONG =
    (1 as ::core::ffi::c_int as JLONG) << SCALEBITS - 1 as ::core::ffi::c_int;
pub const R_Y_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_Y_OFF: ::core::ffi::c_int =
    1 as ::core::ffi::c_int * (MAXJSAMPLE + 1 as ::core::ffi::c_int);
pub const B_Y_OFF: ::core::ffi::c_int =
    2 as ::core::ffi::c_int * (MAXJSAMPLE + 1 as ::core::ffi::c_int);
pub const R_CB_OFF: ::core::ffi::c_int =
    3 as ::core::ffi::c_int * (MAXJSAMPLE + 1 as ::core::ffi::c_int);
pub const G_CB_OFF: ::core::ffi::c_int =
    4 as ::core::ffi::c_int * (MAXJSAMPLE + 1 as ::core::ffi::c_int);
pub const B_CB_OFF: ::core::ffi::c_int =
    5 as ::core::ffi::c_int * (MAXJSAMPLE + 1 as ::core::ffi::c_int);
pub const R_CR_OFF: ::core::ffi::c_int = B_CB_OFF;
pub const G_CR_OFF: ::core::ffi::c_int =
    6 as ::core::ffi::c_int * (MAXJSAMPLE + 1 as ::core::ffi::c_int);
pub const B_CR_OFF: ::core::ffi::c_int =
    7 as ::core::ffi::c_int * (MAXJSAMPLE + 1 as ::core::ffi::c_int);
pub const TABLE_SIZE: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * (MAXJSAMPLE + 1 as ::core::ffi::c_int);
#[inline(always)]
unsafe extern "C" fn rgb_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh10 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh10;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh25 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh25;
        outptr =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn rgb_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh17 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh17;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(RGB_RED as isize);
            *outptr1.offset(col as isize) = *inptr.offset(RGB_GREEN as isize);
            *outptr2.offset(col as isize) = *inptr.offset(RGB_BLUE as isize);
            inptr = inptr.offset(RGB_PIXELSIZE as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extrgb_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh16 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh16;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_5 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extrgb_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh31 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh31;
        outptr =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_5 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extrgb_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh23 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh23;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(RGB_RED_5 as isize);
            *outptr1.offset(col as isize) = *inptr.offset(RGB_GREEN_5 as isize);
            *outptr2.offset(col as isize) = *inptr.offset(RGB_BLUE_5 as isize);
            inptr = inptr.offset(RGB_PIXELSIZE_5 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extrgbx_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh15 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh15;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extrgbx_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh30 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh30;
        outptr =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extrgbx_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh22 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh22;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(RGB_RED_4 as isize);
            *outptr1.offset(col as isize) = *inptr.offset(RGB_GREEN_4 as isize);
            *outptr2.offset(col as isize) = *inptr.offset(RGB_BLUE_4 as isize);
            inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extbgr_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh14 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh14;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_3 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extbgr_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh29 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh29;
        outptr =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_3 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extbgr_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh21 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh21;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(RGB_RED_3 as isize);
            *outptr1.offset(col as isize) = *inptr.offset(RGB_GREEN_3 as isize);
            *outptr2.offset(col as isize) = *inptr.offset(RGB_BLUE_3 as isize);
            inptr = inptr.offset(RGB_PIXELSIZE_3 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extbgrx_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh13 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh13;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extbgrx_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh28 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh28;
        outptr =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extbgrx_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh20 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh20;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(RGB_RED_2 as isize);
            *outptr1.offset(col as isize) = *inptr.offset(RGB_GREEN_2 as isize);
            *outptr2.offset(col as isize) = *inptr.offset(RGB_BLUE_2 as isize);
            inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extxbgr_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh12 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh12;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extxbgr_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh27 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh27;
        outptr =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extxbgr_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh19 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh19;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(RGB_RED_1 as isize);
            *outptr1.offset(col as isize) = *inptr.offset(RGB_GREEN_1 as isize);
            *outptr2.offset(col as isize) = *inptr.offset(RGB_BLUE_1 as isize);
            inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extxrgb_ycc_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh11 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh11;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extxrgb_gray_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh26 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh26;
        outptr =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = *inptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
            *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn extxrgb_rgb_convert_internal(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh18 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh18;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr0.offset(col as isize) = *inptr.offset(RGB_RED_0 as isize);
            *outptr1.offset(col as isize) = *inptr.offset(RGB_GREEN_0 as isize);
            *outptr2.offset(col as isize) = *inptr.offset(RGB_BLUE_0 as isize);
            inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
            col = col.wrapping_add(1);
        }
    }
}
pub const RGB_RED_5: ::core::ffi::c_int = EXT_RGB_RED;
pub const RGB_GREEN_5: ::core::ffi::c_int = EXT_RGB_GREEN;
pub const RGB_BLUE_5: ::core::ffi::c_int = EXT_RGB_BLUE;
pub const RGB_PIXELSIZE_5: ::core::ffi::c_int = EXT_RGB_PIXELSIZE;
pub const RGB_RED_4: ::core::ffi::c_int = EXT_RGBX_RED;
pub const RGB_GREEN_4: ::core::ffi::c_int = EXT_RGBX_GREEN;
pub const RGB_BLUE_4: ::core::ffi::c_int = EXT_RGBX_BLUE;
pub const RGB_PIXELSIZE_4: ::core::ffi::c_int = EXT_RGBX_PIXELSIZE;
pub const RGB_RED_3: ::core::ffi::c_int = EXT_BGR_RED;
pub const RGB_GREEN_3: ::core::ffi::c_int = EXT_BGR_GREEN;
pub const RGB_BLUE_3: ::core::ffi::c_int = EXT_BGR_BLUE;
pub const RGB_PIXELSIZE_3: ::core::ffi::c_int = EXT_BGR_PIXELSIZE;
pub const RGB_RED_2: ::core::ffi::c_int = EXT_BGRX_RED;
pub const RGB_GREEN_2: ::core::ffi::c_int = EXT_BGRX_GREEN;
pub const RGB_BLUE_2: ::core::ffi::c_int = EXT_BGRX_BLUE;
pub const RGB_PIXELSIZE_2: ::core::ffi::c_int = EXT_BGRX_PIXELSIZE;
pub const RGB_RED_1: ::core::ffi::c_int = EXT_XBGR_RED;
pub const RGB_GREEN_1: ::core::ffi::c_int = EXT_XBGR_GREEN;
pub const RGB_BLUE_1: ::core::ffi::c_int = EXT_XBGR_BLUE;
pub const RGB_PIXELSIZE_1: ::core::ffi::c_int = EXT_XBGR_PIXELSIZE;
pub const RGB_RED_0: ::core::ffi::c_int = EXT_XRGB_RED;
pub const RGB_GREEN_0: ::core::ffi::c_int = EXT_XRGB_GREEN;
pub const RGB_BLUE_0: ::core::ffi::c_int = EXT_XRGB_BLUE;
pub const RGB_PIXELSIZE_0: ::core::ffi::c_int = EXT_XRGB_PIXELSIZE;
unsafe extern "C" fn rgb_ycc_start(mut cinfo: j_compress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut rgb_ycc_tab: *mut JLONG = ::core::ptr::null_mut::<JLONG>();
    let mut i: JLONG = 0;
    rgb_ycc_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (TABLE_SIZE as size_t).wrapping_mul(::core::mem::size_of::<JLONG>() as size_t),
    ) as *mut JLONG;
    (*cconvert).rgb_ycc_tab = rgb_ycc_tab;
    i = 0 as JLONG;
    while i <= MAXJSAMPLE as JLONG {
        *rgb_ycc_tab.offset((i + R_Y_OFF as JLONG) as isize) = (0.29900f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG
            * i;
        *rgb_ycc_tab.offset((i + G_Y_OFF as JLONG) as isize) = (0.58700f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG
            * i;
        *rgb_ycc_tab.offset((i + B_Y_OFF as JLONG) as isize) = (0.11400f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG
            * i
            + ONE_HALF;
        *rgb_ycc_tab.offset((i + R_CB_OFF as JLONG) as isize) = -((0.16874f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG)
            * i;
        *rgb_ycc_tab.offset((i + G_CB_OFF as JLONG) as isize) = -((0.33126f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG)
            * i;
        *rgb_ycc_tab.offset((i + B_CB_OFF as JLONG) as isize) = (0.50000f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG
            * i
            + CBCR_OFFSET
            + ONE_HALF
            - 1 as JLONG;
        *rgb_ycc_tab.offset((i + G_CR_OFF as JLONG) as isize) = -((0.41869f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG)
            * i;
        *rgb_ycc_tab.offset((i + B_CR_OFF as JLONG) as isize) = -((0.08131f64
            * ((1 as ::core::ffi::c_long) << SCALEBITS) as ::core::ffi::c_double
            + 0.5f64) as JLONG)
            * i;
        i += 1;
    }
}
unsafe extern "C" fn rgb_ycc_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    match (*cinfo).in_color_space as ::core::ffi::c_uint {
        6 => {
            extrgb_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        7 | 12 => {
            extrgbx_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        8 => {
            extbgr_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        9 | 13 => {
            extbgrx_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        10 | 14 => {
            extxbgr_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        11 | 15 => {
            extxrgb_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        _ => {
            rgb_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
    };
}

pub const JPEG_RS_JCCOLOR_LINK_ANCHOR: unsafe extern "C" fn(j_compress_ptr) = jinit_color_converter;
unsafe extern "C" fn rgb_gray_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    match (*cinfo).in_color_space as ::core::ffi::c_uint {
        6 => {
            extrgb_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        7 | 12 => {
            extrgbx_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        8 => {
            extbgr_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        9 | 13 => {
            extbgrx_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        10 | 14 => {
            extxbgr_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        11 | 15 => {
            extxrgb_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        _ => {
            rgb_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
    };
}
unsafe extern "C" fn rgb_rgb_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    match (*cinfo).in_color_space as ::core::ffi::c_uint {
        6 => {
            extrgb_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        7 | 12 => {
            extrgbx_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        8 => {
            extbgr_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        9 | 13 => {
            extbgrx_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        10 | 14 => {
            extxbgr_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        11 | 15 => {
            extxrgb_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        _ => {
            rgb_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
    };
}
unsafe extern "C" fn cmyk_ycck_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: ::core::ffi::c_int = 0;
    let mut g: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut ctab: *mut JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr3: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh9 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh9;
        outptr0 =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr1 =
            *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr2 =
            *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        outptr3 =
            *(*output_buf.offset(3 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            r = MAXJSAMPLE - *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            g = MAXJSAMPLE - *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            b = MAXJSAMPLE - *inptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            *outptr3.offset(col as isize) = *inptr.offset(3 as ::core::ffi::c_int as isize);
            inptr = inptr.offset(4 as ::core::ffi::c_int as isize);
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS) as JSAMPLE;
            col = col.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn grayscale_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    let mut instride: ::core::ffi::c_int = (*cinfo).input_components;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh24 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh24;
        outptr =
            *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(col as isize) = *inptr.offset(0 as ::core::ffi::c_int as isize);
            inptr = inptr.offset(instride as isize);
            col = col.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn null_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr2: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr3: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut col: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut nc: ::core::ffi::c_int = (*cinfo).num_components;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    if nc == 3 as ::core::ffi::c_int {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as ::core::ffi::c_int) {
                break;
            }
            let fresh0 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh0;
            outptr0 =
                *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
            outptr1 =
                *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
            outptr2 =
                *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as JDIMENSION;
            while col < num_cols {
                let fresh1 = inptr;
                inptr = inptr.offset(1);
                *outptr0.offset(col as isize) = *fresh1;
                let fresh2 = inptr;
                inptr = inptr.offset(1);
                *outptr1.offset(col as isize) = *fresh2;
                let fresh3 = inptr;
                inptr = inptr.offset(1);
                *outptr2.offset(col as isize) = *fresh3;
                col = col.wrapping_add(1);
            }
        }
    } else if nc == 4 as ::core::ffi::c_int {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as ::core::ffi::c_int) {
                break;
            }
            let fresh4 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh4;
            outptr0 =
                *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_row as isize);
            outptr1 =
                *(*output_buf.offset(1 as ::core::ffi::c_int as isize)).offset(output_row as isize);
            outptr2 =
                *(*output_buf.offset(2 as ::core::ffi::c_int as isize)).offset(output_row as isize);
            outptr3 =
                *(*output_buf.offset(3 as ::core::ffi::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as JDIMENSION;
            while col < num_cols {
                let fresh5 = inptr;
                inptr = inptr.offset(1);
                *outptr0.offset(col as isize) = *fresh5;
                let fresh6 = inptr;
                inptr = inptr.offset(1);
                *outptr1.offset(col as isize) = *fresh6;
                let fresh7 = inptr;
                inptr = inptr.offset(1);
                *outptr2.offset(col as isize) = *fresh7;
                let fresh8 = inptr;
                inptr = inptr.offset(1);
                *outptr3.offset(col as isize) = *fresh8;
                col = col.wrapping_add(1);
            }
        }
    } else {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as ::core::ffi::c_int) {
                break;
            }
            ci = 0 as ::core::ffi::c_int;
            while ci < nc {
                inptr = *input_buf;
                outptr = *(*output_buf.offset(ci as isize)).offset(output_row as isize);
                col = 0 as JDIMENSION;
                while col < num_cols {
                    *outptr.offset(col as isize) = *inptr.offset(ci as isize);
                    inptr = inptr.offset(nc as isize);
                    col = col.wrapping_add(1);
                }
                ci += 1;
            }
            input_buf = input_buf.offset(1);
            output_row = output_row.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn null_method(mut cinfo: j_compress_ptr) {}
#[no_mangle]
pub unsafe extern "C" fn jinit_color_converter(mut cinfo: j_compress_ptr) {
    let mut cconvert: my_cconvert_ptr = ::core::ptr::null_mut::<my_color_converter>();
    cconvert = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<my_color_converter>() as size_t,
    ) as my_cconvert_ptr;
    (*cinfo).cconvert = cconvert as *mut jpeg_color_converter as *mut jpeg_color_converter;
    (*cconvert).pub_0.start_pass = Some(null_method as unsafe extern "C" fn(j_compress_ptr) -> ())
        as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
    match (*cinfo).in_color_space as ::core::ffi::c_uint {
        1 => {
            if (*cinfo).input_components != 1 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            if (*cinfo).input_components != rgb_pixelsize[(*cinfo).in_color_space as usize] {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        3 => {
            if (*cinfo).input_components != 3 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 | 5 => {
            if (*cinfo).input_components != 4 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            if (*cinfo).input_components < 1 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    match (*cinfo).jpeg_color_space as ::core::ffi::c_uint {
        1 => {
            if (*cinfo).num_components != 1 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_RGBX as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_BGR as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_BGRX as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_XBGR as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_XRGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_BGRA as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_ABGR as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_ARGB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if jsimd_can_rgb_gray() != 0 {
                    (*cconvert).pub_0.color_convert = Some(
                        jsimd_rgb_gray_convert
                            as unsafe extern "C" fn(
                                j_compress_ptr,
                                JSAMPARRAY,
                                JSAMPIMAGE,
                                JDIMENSION,
                                ::core::ffi::c_int,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                j_compress_ptr,
                                JSAMPARRAY,
                                JSAMPIMAGE,
                                JDIMENSION,
                                ::core::ffi::c_int,
                            ) -> (),
                        >;
                } else {
                    (*cconvert).pub_0.start_pass =
                        Some(rgb_ycc_start as unsafe extern "C" fn(j_compress_ptr) -> ())
                            as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_gray_convert
                            as unsafe extern "C" fn(
                                j_compress_ptr,
                                JSAMPARRAY,
                                JSAMPIMAGE,
                                JDIMENSION,
                                ::core::ffi::c_int,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                j_compress_ptr,
                                JSAMPARRAY,
                                JSAMPIMAGE,
                                JDIMENSION,
                                ::core::ffi::c_int,
                            ) -> (),
                        >;
                }
            } else if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_YCbCr as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 => {
            if (*cinfo).num_components != 3 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if rgb_red[(*cinfo).in_color_space as usize] == 0 as ::core::ffi::c_int
                && rgb_green[(*cinfo).in_color_space as usize] == 1 as ::core::ffi::c_int
                && rgb_blue[(*cinfo).in_color_space as usize] == 2 as ::core::ffi::c_int
                && rgb_pixelsize[(*cinfo).in_color_space as usize] == 3 as ::core::ffi::c_int
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_RGBX as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_BGR as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_BGRX as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_XBGR as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_XRGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_BGRA as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_ABGR as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_ARGB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_rgb_convert
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        3 => {
            if (*cinfo).num_components != 3 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_RGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_RGBX as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_BGR as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_BGRX as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_XBGR as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_XRGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_RGBA as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_BGRA as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_ABGR as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cinfo).in_color_space as ::core::ffi::c_uint
                    == JCS_EXT_ARGB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if jsimd_can_rgb_ycc() != 0 {
                    (*cconvert).pub_0.color_convert = Some(
                        jsimd_rgb_ycc_convert
                            as unsafe extern "C" fn(
                                j_compress_ptr,
                                JSAMPARRAY,
                                JSAMPIMAGE,
                                JDIMENSION,
                                ::core::ffi::c_int,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                j_compress_ptr,
                                JSAMPARRAY,
                                JSAMPIMAGE,
                                JDIMENSION,
                                ::core::ffi::c_int,
                            ) -> (),
                        >;
                } else {
                    (*cconvert).pub_0.start_pass =
                        Some(rgb_ycc_start as unsafe extern "C" fn(j_compress_ptr) -> ())
                            as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_ycc_convert
                            as unsafe extern "C" fn(
                                j_compress_ptr,
                                JSAMPARRAY,
                                JSAMPIMAGE,
                                JDIMENSION,
                                ::core::ffi::c_int,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                j_compress_ptr,
                                JSAMPARRAY,
                                JSAMPIMAGE,
                                JDIMENSION,
                                ::core::ffi::c_int,
                            ) -> (),
                        >;
                }
            } else if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_YCbCr as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 => {
            if (*cinfo).num_components != 4 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        5 => {
            if (*cinfo).num_components != 4 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.start_pass =
                    Some(rgb_ycc_start as unsafe extern "C" fn(j_compress_ptr) -> ())
                        as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
                (*cconvert).pub_0.color_convert = Some(
                    cmyk_ycck_convert
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else if (*cinfo).in_color_space as ::core::ffi::c_uint
                == JCS_YCCK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            j_compress_ptr,
                            JSAMPARRAY,
                            JSAMPIMAGE,
                            JDIMENSION,
                            ::core::ffi::c_int,
                        ) -> (),
                    >;
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            if (*cinfo).jpeg_color_space as ::core::ffi::c_uint
                != (*cinfo).in_color_space as ::core::ffi::c_uint
                || (*cinfo).num_components != (*cinfo).input_components
            {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*cconvert).pub_0.color_convert = Some(
                null_convert
                    as unsafe extern "C" fn(
                        j_compress_ptr,
                        JSAMPARRAY,
                        JSAMPIMAGE,
                        JDIMENSION,
                        ::core::ffi::c_int,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        j_compress_ptr,
                        JSAMPARRAY,
                        JSAMPIMAGE,
                        JDIMENSION,
                        ::core::ffi::c_int,
                    ) -> (),
                >;
        }
    };
}
