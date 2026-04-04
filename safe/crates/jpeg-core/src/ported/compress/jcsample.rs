#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}

extern "C" {
    fn jcopy_sample_rows(
        input_array: JSAMPARRAY,
        source_row: ::core::ffi::c_int,
        output_array: JSAMPARRAY,
        dest_row: ::core::ffi::c_int,
        num_rows: ::core::ffi::c_int,
        num_cols: JDIMENSION,
    );
    fn jsimd_can_h2v2_downsample() -> ::core::ffi::c_int;
    fn jsimd_can_h2v1_downsample() -> ::core::ffi::c_int;
    fn jsimd_h2v2_downsample(
        cinfo: j_compress_ptr,
        compptr: *mut jpeg_component_info,
        input_data: JSAMPARRAY,
        output_data: JSAMPARRAY,
    );
    fn jsimd_h2v1_downsample(
        cinfo: j_compress_ptr,
        compptr: *mut jpeg_component_info,
        input_data: JSAMPARRAY,
        output_data: JSAMPARRAY,
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
pub const JTRC_SMOOTH_NOTIMPL: C2RustUnnamed_0 = 101;
pub const JERR_FRACT_SAMPLE_NOTIMPL: C2RustUnnamed_0 = 39;
pub type downsample1_ptr = Option<
    unsafe extern "C" fn(j_compress_ptr, *mut jpeg_component_info, JSAMPARRAY, JSAMPARRAY) -> (),
>;
pub type my_downsample_ptr = *mut my_downsampler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_downsampler {
    pub pub_0: jpeg_downsampler,
    pub methods: [downsample1_ptr; 10],
}
pub const JERR_CCIR601_NOTIMPL: C2RustUnnamed_0 = 26;
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
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DCTSIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn start_pass_downsample(mut cinfo: j_compress_ptr) {}
unsafe extern "C" fn expand_right_edge(
    mut image_data: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
    mut input_cols: JDIMENSION,
    mut output_cols: JDIMENSION,
) {
    let mut ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut pixval: JSAMPLE = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut numcols: ::core::ffi::c_int =
        output_cols.wrapping_sub(input_cols) as ::core::ffi::c_int;
    if numcols > 0 as ::core::ffi::c_int {
        row = 0 as ::core::ffi::c_int;
        while row < num_rows {
            ptr = (*image_data.offset(row as isize)).offset(input_cols as isize);
            pixval = *ptr.offset(-(1 as ::core::ffi::c_int) as isize);
            count = numcols;
            while count > 0 as ::core::ffi::c_int {
                let fresh2 = ptr;
                ptr = ptr.offset(1);
                *fresh2 = pixval;
                count -= 1;
            }
            row += 1;
        }
    }
}
unsafe extern "C" fn sep_downsample(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_index: JDIMENSION,
    mut output_buf: JSAMPIMAGE,
    mut out_row_group_index: JDIMENSION,
) {
    let mut downsample: my_downsample_ptr = (*cinfo).downsample as my_downsample_ptr;
    let mut ci: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut in_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    let mut out_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    ci = 0 as ::core::ffi::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        in_ptr = (*input_buf.offset(ci as isize)).offset(in_row_index as isize);
        out_ptr = (*output_buf.offset(ci as isize)).offset(
            out_row_group_index.wrapping_mul((*compptr).v_samp_factor as JDIMENSION) as isize,
        );
        Some(
            (*(&raw mut (*downsample).methods as *mut downsample1_ptr).offset(ci as isize))
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, compptr, in_ptr, out_ptr);
        ci += 1;
        compptr = compptr.offset(1);
    }
}
unsafe extern "C" fn int_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    let mut inrow: ::core::ffi::c_int = 0;
    let mut outrow: ::core::ffi::c_int = 0;
    let mut h_expand: ::core::ffi::c_int = 0;
    let mut v_expand: ::core::ffi::c_int = 0;
    let mut numpix: ::core::ffi::c_int = 0;
    let mut numpix2: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    let mut outcol: JDIMENSION = 0;
    let mut outcol_h: JDIMENSION = 0;
    let mut output_cols: JDIMENSION = (*compptr)
        .width_in_blocks
        .wrapping_mul(DCTSIZE as JDIMENSION);
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outvalue: JLONG = 0;
    h_expand = (*cinfo).max_h_samp_factor / (*compptr).h_samp_factor;
    v_expand = (*cinfo).max_v_samp_factor / (*compptr).v_samp_factor;
    numpix = h_expand * v_expand;
    numpix2 = numpix / 2 as ::core::ffi::c_int;
    expand_right_edge(
        input_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        output_cols.wrapping_mul(h_expand as JDIMENSION),
    );
    inrow = 0 as ::core::ffi::c_int;
    outrow = 0 as ::core::ffi::c_int;
    while outrow < (*compptr).v_samp_factor {
        outptr = *output_data.offset(outrow as isize);
        outcol = 0 as JDIMENSION;
        outcol_h = 0 as JDIMENSION;
        while outcol < output_cols {
            outvalue = 0 as JLONG;
            v = 0 as ::core::ffi::c_int;
            while v < v_expand {
                inptr = (*input_data.offset((inrow + v) as isize)).offset(outcol_h as isize);
                h = 0 as ::core::ffi::c_int;
                while h < h_expand {
                    let fresh0 = inptr;
                    inptr = inptr.offset(1);
                    outvalue += *fresh0 as JLONG;
                    h += 1;
                }
                v += 1;
            }
            let fresh1 = outptr;
            outptr = outptr.offset(1);
            *fresh1 = ((outvalue + numpix2 as JLONG) / numpix as JLONG) as JSAMPLE;
            outcol = outcol.wrapping_add(1);
            outcol_h = outcol_h.wrapping_add(h_expand as JDIMENSION);
        }
        inrow += v_expand;
        outrow += 1;
    }
}
unsafe extern "C" fn fullsize_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    jcopy_sample_rows(
        input_data,
        0 as ::core::ffi::c_int,
        output_data,
        0 as ::core::ffi::c_int,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
    );
    expand_right_edge(
        output_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        (*compptr)
            .width_in_blocks
            .wrapping_mul(DCTSIZE as JDIMENSION),
    );
}
unsafe extern "C" fn h2v1_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    let mut outrow: ::core::ffi::c_int = 0;
    let mut outcol: JDIMENSION = 0;
    let mut output_cols: JDIMENSION = (*compptr)
        .width_in_blocks
        .wrapping_mul(DCTSIZE as JDIMENSION);
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut bias: ::core::ffi::c_int = 0;
    expand_right_edge(
        input_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        output_cols.wrapping_mul(2 as JDIMENSION),
    );
    outrow = 0 as ::core::ffi::c_int;
    while outrow < (*compptr).v_samp_factor {
        outptr = *output_data.offset(outrow as isize);
        inptr = *input_data.offset(outrow as isize);
        bias = 0 as ::core::ffi::c_int;
        outcol = 0 as JDIMENSION;
        while outcol < output_cols {
            let fresh6 = outptr;
            outptr = outptr.offset(1);
            *fresh6 = (*inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *inptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + bias
                >> 1 as ::core::ffi::c_int) as JSAMPLE;
            bias ^= 1 as ::core::ffi::c_int;
            inptr = inptr.offset(2 as ::core::ffi::c_int as isize);
            outcol = outcol.wrapping_add(1);
        }
        outrow += 1;
    }
}
unsafe extern "C" fn h2v2_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    let mut inrow: ::core::ffi::c_int = 0;
    let mut outrow: ::core::ffi::c_int = 0;
    let mut outcol: JDIMENSION = 0;
    let mut output_cols: JDIMENSION = (*compptr)
        .width_in_blocks
        .wrapping_mul(DCTSIZE as JDIMENSION);
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut bias: ::core::ffi::c_int = 0;
    expand_right_edge(
        input_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        output_cols.wrapping_mul(2 as JDIMENSION),
    );
    inrow = 0 as ::core::ffi::c_int;
    outrow = 0 as ::core::ffi::c_int;
    while outrow < (*compptr).v_samp_factor {
        outptr = *output_data.offset(outrow as isize);
        inptr0 = *input_data.offset(inrow as isize);
        inptr1 = *input_data.offset((inrow + 1 as ::core::ffi::c_int) as isize);
        bias = 1 as ::core::ffi::c_int;
        outcol = 0 as JDIMENSION;
        while outcol < output_cols {
            let fresh3 = outptr;
            outptr = outptr.offset(1);
            *fresh3 = (*inptr0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *inptr0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *inptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *inptr1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + bias
                >> 2 as ::core::ffi::c_int) as JSAMPLE;
            bias ^= 3 as ::core::ffi::c_int;
            inptr0 = inptr0.offset(2 as ::core::ffi::c_int as isize);
            inptr1 = inptr1.offset(2 as ::core::ffi::c_int as isize);
            outcol = outcol.wrapping_add(1);
        }
        inrow += 2 as ::core::ffi::c_int;
        outrow += 1;
    }
}
unsafe extern "C" fn h2v2_smooth_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    let mut inrow: ::core::ffi::c_int = 0;
    let mut outrow: ::core::ffi::c_int = 0;
    let mut colctr: JDIMENSION = 0;
    let mut output_cols: JDIMENSION = (*compptr)
        .width_in_blocks
        .wrapping_mul(DCTSIZE as JDIMENSION);
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut above_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut below_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut membersum: JLONG = 0;
    let mut neighsum: JLONG = 0;
    let mut memberscale: JLONG = 0;
    let mut neighscale: JLONG = 0;
    expand_right_edge(
        input_data.offset(-(1 as ::core::ffi::c_int as isize)),
        (*cinfo).max_v_samp_factor + 2 as ::core::ffi::c_int,
        (*cinfo).image_width,
        output_cols.wrapping_mul(2 as JDIMENSION),
    );
    memberscale = (16384 as ::core::ffi::c_int
        - (*cinfo).smoothing_factor * 80 as ::core::ffi::c_int) as JLONG;
    neighscale = ((*cinfo).smoothing_factor * 16 as ::core::ffi::c_int) as JLONG;
    inrow = 0 as ::core::ffi::c_int;
    outrow = 0 as ::core::ffi::c_int;
    while outrow < (*compptr).v_samp_factor {
        outptr = *output_data.offset(outrow as isize);
        inptr0 = *input_data.offset(inrow as isize);
        inptr1 = *input_data.offset((inrow + 1 as ::core::ffi::c_int) as isize);
        above_ptr = *input_data.offset((inrow - 1 as ::core::ffi::c_int) as isize);
        below_ptr = *input_data.offset((inrow + 2 as ::core::ffi::c_int) as isize);
        membersum = (*inptr0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        neighsum = (*above_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *above_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *below_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *below_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        neighsum += neighsum;
        neighsum += (*above_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *above_ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *below_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *below_ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        membersum = membersum * memberscale + neighsum * neighscale;
        let fresh4 = outptr;
        outptr = outptr.offset(1);
        *fresh4 = (membersum + 32768 as JLONG >> 16 as ::core::ffi::c_int) as JSAMPLE;
        inptr0 = inptr0.offset(2 as ::core::ffi::c_int as isize);
        inptr1 = inptr1.offset(2 as ::core::ffi::c_int as isize);
        above_ptr = above_ptr.offset(2 as ::core::ffi::c_int as isize);
        below_ptr = below_ptr.offset(2 as ::core::ffi::c_int as isize);
        colctr = output_cols.wrapping_sub(2 as JDIMENSION);
        while colctr > 0 as JDIMENSION {
            membersum = (*inptr0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *inptr0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *inptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *inptr1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as JLONG;
            neighsum = (*above_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *above_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *below_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *below_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *inptr0.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *inptr0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *inptr1.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *inptr1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as JLONG;
            neighsum += neighsum;
            neighsum += (*above_ptr.offset(-(1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *above_ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *below_ptr.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *below_ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as JLONG;
            membersum = membersum * memberscale + neighsum * neighscale;
            let fresh5 = outptr;
            outptr = outptr.offset(1);
            *fresh5 = (membersum + 32768 as JLONG >> 16 as ::core::ffi::c_int) as JSAMPLE;
            inptr0 = inptr0.offset(2 as ::core::ffi::c_int as isize);
            inptr1 = inptr1.offset(2 as ::core::ffi::c_int as isize);
            above_ptr = above_ptr.offset(2 as ::core::ffi::c_int as isize);
            below_ptr = below_ptr.offset(2 as ::core::ffi::c_int as isize);
            colctr = colctr.wrapping_sub(1);
        }
        membersum = (*inptr0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        neighsum = (*above_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *above_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *below_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *below_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr0.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *inptr0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr1.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *inptr1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        neighsum += neighsum;
        neighsum += (*above_ptr.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *above_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *below_ptr.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *below_ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        membersum = membersum * memberscale + neighsum * neighscale;
        *outptr = (membersum + 32768 as JLONG >> 16 as ::core::ffi::c_int) as JSAMPLE;
        inrow += 2 as ::core::ffi::c_int;
        outrow += 1;
    }
}
unsafe extern "C" fn fullsize_smooth_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
    let mut outrow: ::core::ffi::c_int = 0;
    let mut colctr: JDIMENSION = 0;
    let mut output_cols: JDIMENSION = (*compptr)
        .width_in_blocks
        .wrapping_mul(DCTSIZE as JDIMENSION);
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut above_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut below_ptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut membersum: JLONG = 0;
    let mut neighsum: JLONG = 0;
    let mut memberscale: JLONG = 0;
    let mut neighscale: JLONG = 0;
    let mut colsum: ::core::ffi::c_int = 0;
    let mut lastcolsum: ::core::ffi::c_int = 0;
    let mut nextcolsum: ::core::ffi::c_int = 0;
    expand_right_edge(
        input_data.offset(-(1 as ::core::ffi::c_int as isize)),
        (*cinfo).max_v_samp_factor + 2 as ::core::ffi::c_int,
        (*cinfo).image_width,
        output_cols,
    );
    memberscale = (65536 as ::core::ffi::c_long
        - (*cinfo).smoothing_factor as ::core::ffi::c_long * 512 as ::core::ffi::c_long)
        as JLONG;
    neighscale = ((*cinfo).smoothing_factor * 64 as ::core::ffi::c_int) as JLONG;
    outrow = 0 as ::core::ffi::c_int;
    while outrow < (*compptr).v_samp_factor {
        outptr = *output_data.offset(outrow as isize);
        inptr = *input_data.offset(outrow as isize);
        above_ptr = *input_data.offset((outrow - 1 as ::core::ffi::c_int) as isize);
        below_ptr = *input_data.offset((outrow + 1 as ::core::ffi::c_int) as isize);
        let fresh7 = above_ptr;
        above_ptr = above_ptr.offset(1);
        let fresh8 = below_ptr;
        below_ptr = below_ptr.offset(1);
        colsum = *fresh7 as ::core::ffi::c_int
            + *fresh8 as ::core::ffi::c_int
            + *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let fresh9 = inptr;
        inptr = inptr.offset(1);
        membersum = *fresh9 as JLONG;
        nextcolsum = *above_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *below_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        neighsum = colsum as JLONG + (colsum as JLONG - membersum) + nextcolsum as JLONG;
        membersum = membersum * memberscale + neighsum * neighscale;
        let fresh10 = outptr;
        outptr = outptr.offset(1);
        *fresh10 = (membersum + 32768 as JLONG >> 16 as ::core::ffi::c_int) as JSAMPLE;
        lastcolsum = colsum;
        colsum = nextcolsum;
        colctr = output_cols.wrapping_sub(2 as JDIMENSION);
        while colctr > 0 as JDIMENSION {
            let fresh11 = inptr;
            inptr = inptr.offset(1);
            membersum = *fresh11 as JLONG;
            above_ptr = above_ptr.offset(1);
            below_ptr = below_ptr.offset(1);
            nextcolsum = *above_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *below_ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            neighsum = lastcolsum as JLONG + (colsum as JLONG - membersum) + nextcolsum as JLONG;
            membersum = membersum * memberscale + neighsum * neighscale;
            let fresh12 = outptr;
            outptr = outptr.offset(1);
            *fresh12 = (membersum + 32768 as JLONG >> 16 as ::core::ffi::c_int) as JSAMPLE;
            lastcolsum = colsum;
            colsum = nextcolsum;
            colctr = colctr.wrapping_sub(1);
        }
        membersum = *inptr as JLONG;
        neighsum = lastcolsum as JLONG + (colsum as JLONG - membersum) + colsum as JLONG;
        membersum = membersum * memberscale + neighsum * neighscale;
        *outptr = (membersum + 32768 as JLONG >> 16 as ::core::ffi::c_int) as JSAMPLE;
        outrow += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn jinit_downsampler(mut cinfo: j_compress_ptr) {
    let mut downsample: my_downsample_ptr = ::core::ptr::null_mut::<my_downsampler>();
    let mut ci: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut smoothok: boolean = TRUE;
    downsample = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<my_downsampler>() as size_t,
    ) as my_downsample_ptr;
    (*cinfo).downsample = downsample as *mut jpeg_downsampler as *mut jpeg_downsampler;
    (*downsample).pub_0.start_pass =
        Some(start_pass_downsample as unsafe extern "C" fn(j_compress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_compress_ptr) -> ()>;
    (*downsample).pub_0.downsample = Some(
        sep_downsample
            as unsafe extern "C" fn(
                j_compress_ptr,
                JSAMPIMAGE,
                JDIMENSION,
                JSAMPIMAGE,
                JDIMENSION,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                j_compress_ptr,
                JSAMPIMAGE,
                JDIMENSION,
                JSAMPIMAGE,
                JDIMENSION,
            ) -> (),
        >;
    (*downsample).pub_0.need_context_rows = FALSE as boolean;
    if (*cinfo).CCIR601_sampling != 0 {
        (*(*cinfo).err).msg_code = JERR_CCIR601_NOTIMPL as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    ci = 0 as ::core::ffi::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        if (*compptr).h_samp_factor == (*cinfo).max_h_samp_factor
            && (*compptr).v_samp_factor == (*cinfo).max_v_samp_factor
        {
            if (*cinfo).smoothing_factor != 0 {
                (*downsample).methods[ci as usize] = Some(
                    fullsize_smooth_downsample
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            *mut jpeg_component_info,
                            JSAMPARRAY,
                            JSAMPARRAY,
                        ) -> (),
                ) as downsample1_ptr;
                (*downsample).pub_0.need_context_rows = TRUE as boolean;
            } else {
                (*downsample).methods[ci as usize] = Some(
                    fullsize_downsample
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            *mut jpeg_component_info,
                            JSAMPARRAY,
                            JSAMPARRAY,
                        ) -> (),
                ) as downsample1_ptr;
            }
        } else if (*compptr).h_samp_factor * 2 as ::core::ffi::c_int == (*cinfo).max_h_samp_factor
            && (*compptr).v_samp_factor == (*cinfo).max_v_samp_factor
        {
            smoothok = FALSE as boolean;
            if jsimd_can_h2v1_downsample() != 0 {
                (*downsample).methods[ci as usize] = Some(
                    jsimd_h2v1_downsample
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            *mut jpeg_component_info,
                            JSAMPARRAY,
                            JSAMPARRAY,
                        ) -> (),
                ) as downsample1_ptr;
            } else {
                (*downsample).methods[ci as usize] = Some(
                    h2v1_downsample
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            *mut jpeg_component_info,
                            JSAMPARRAY,
                            JSAMPARRAY,
                        ) -> (),
                ) as downsample1_ptr;
            }
        } else if (*compptr).h_samp_factor * 2 as ::core::ffi::c_int == (*cinfo).max_h_samp_factor
            && (*compptr).v_samp_factor * 2 as ::core::ffi::c_int == (*cinfo).max_v_samp_factor
        {
            if (*cinfo).smoothing_factor != 0 {
                (*downsample).methods[ci as usize] = Some(
                    h2v2_smooth_downsample
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            *mut jpeg_component_info,
                            JSAMPARRAY,
                            JSAMPARRAY,
                        ) -> (),
                ) as downsample1_ptr;
                (*downsample).pub_0.need_context_rows = TRUE as boolean;
            } else if jsimd_can_h2v2_downsample() != 0 {
                (*downsample).methods[ci as usize] = Some(
                    jsimd_h2v2_downsample
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            *mut jpeg_component_info,
                            JSAMPARRAY,
                            JSAMPARRAY,
                        ) -> (),
                ) as downsample1_ptr;
            } else {
                (*downsample).methods[ci as usize] = Some(
                    h2v2_downsample
                        as unsafe extern "C" fn(
                            j_compress_ptr,
                            *mut jpeg_component_info,
                            JSAMPARRAY,
                            JSAMPARRAY,
                        ) -> (),
                ) as downsample1_ptr;
            }
        } else if (*cinfo).max_h_samp_factor % (*compptr).h_samp_factor == 0 as ::core::ffi::c_int
            && (*cinfo).max_v_samp_factor % (*compptr).v_samp_factor == 0 as ::core::ffi::c_int
        {
            smoothok = FALSE as boolean;
            (*downsample).methods[ci as usize] = Some(
                int_downsample
                    as unsafe extern "C" fn(
                        j_compress_ptr,
                        *mut jpeg_component_info,
                        JSAMPARRAY,
                        JSAMPARRAY,
                    ) -> (),
            ) as downsample1_ptr;
        } else {
            (*(*cinfo).err).msg_code = JERR_FRACT_SAMPLE_NOTIMPL as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        ci += 1;
        compptr = compptr.offset(1);
    }
    if (*cinfo).smoothing_factor != 0 && smoothok == 0 {
        (*(*cinfo).err).msg_code = JTRC_SMOOTH_NOTIMPL as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, 0 as ::core::ffi::c_int);
    }
}

pub const JPEG_RS_JCSAMPLE_LINK_ANCHOR: unsafe extern "C" fn(j_compress_ptr) = jinit_downsampler;
