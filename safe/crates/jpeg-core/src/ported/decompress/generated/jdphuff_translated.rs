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
    fn jpeg_make_d_derived_tbl(
        cinfo: j_decompress_ptr,
        isDC: boolean,
        tblno: ::core::ffi::c_int,
        pdtbl: *mut *mut d_derived_tbl,
    );
    fn jpeg_fill_bit_buffer(
        state: *mut bitread_working_state,
        get_buffer: bit_buf_type,
        bits_left: ::core::ffi::c_int,
        nbits: ::core::ffi::c_int,
    ) -> boolean;
    fn jpeg_huff_decode(
        state: *mut bitread_working_state,
        get_buffer: bit_buf_type,
        bits_left: ::core::ffi::c_int,
        htbl: *mut d_derived_tbl,
        min_bits: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub alloc_large: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            size_t,
        ) -> *mut ::core::ffi::c_void,
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
    pub emit_message: Option<
        unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> (),
    >,
    pub output_message: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub format_message: Option<
        unsafe extern "C" fn(j_common_ptr, *mut ::core::ffi::c_char) -> (),
    >,
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
        unsafe extern "C" fn(
            j_decompress_ptr,
            JSAMPARRAY,
            JSAMPARRAY,
            ::core::ffi::c_int,
        ) -> (),
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
    pub decode_mcu: Option<
        unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean,
    >,
    pub insufficient_data: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_marker_reader {
    pub reset_marker_reader: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub read_markers: Option<
        unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int,
    >,
    pub read_restart_marker: jpeg_marker_parser_method,
    pub saw_SOI: boolean,
    pub saw_SOF: boolean,
    pub next_restart_num: ::core::ffi::c_int,
    pub discarded_bytes: ::core::ffi::c_uint,
}
pub type jpeg_marker_parser_method = Option<
    unsafe extern "C" fn(j_decompress_ptr) -> boolean,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_input_controller {
    pub consume_input: Option<
        unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int,
    >,
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
    pub consume_data: Option<
        unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int,
    >,
    pub start_output_pass: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub decompress_data: Option<
        unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE) -> ::core::ffi::c_int,
    >,
    pub coef_arrays: *mut jvirt_barray_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_d_main_controller {
    pub start_pass: Option<unsafe extern "C" fn(j_decompress_ptr, J_BUF_MODE) -> ()>,
    pub process_data: Option<
        unsafe extern "C" fn(
            j_decompress_ptr,
            JSAMPARRAY,
            *mut JDIMENSION,
            JDIMENSION,
        ) -> (),
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
    pub skip_input_data: Option<
        unsafe extern "C" fn(j_decompress_ptr, ::core::ffi::c_long) -> (),
    >,
    pub resync_to_restart: Option<
        unsafe extern "C" fn(j_decompress_ptr, ::core::ffi::c_int) -> boolean,
    >,
    pub term_source: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
}
pub type JLONG = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct d_derived_tbl {
    pub maxcode: [JLONG; 18],
    pub valoffset: [JLONG; 18],
    pub pub_0: *mut JHUFF_TBL,
    pub lookup: [::core::ffi::c_int; 256],
}
pub type phuff_entropy_ptr = *mut phuff_entropy_decoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct phuff_entropy_decoder {
    pub pub_0: jpeg_entropy_decoder,
    pub bitstate: bitread_perm_state,
    pub saved: savable_state,
    pub restarts_to_go: ::core::ffi::c_uint,
    pub derived_tbls: [*mut d_derived_tbl; 4],
    pub ac_derived_tbl: *mut d_derived_tbl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct savable_state {
    pub EOBRUN: ::core::ffi::c_uint,
    pub last_dc_val: [::core::ffi::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bitread_perm_state {
    pub get_buffer: bit_buf_type,
    pub bits_left: ::core::ffi::c_int,
}
pub type bit_buf_type = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bitread_working_state {
    pub next_input_byte: *const JOCTET,
    pub bytes_in_buffer: size_t,
    pub get_buffer: bit_buf_type,
    pub bits_left: ::core::ffi::c_int,
    pub cinfo: j_decompress_ptr,
}
pub const JWRN_HUFF_BAD_CODE: C2RustUnnamed_0 = 121;
pub const JERR_BAD_DCT_COEF: C2RustUnnamed_0 = 6;
pub const JWRN_BOGUS_PROGRESSION: C2RustUnnamed_0 = 118;
pub const JERR_BAD_PROGRESSION: C2RustUnnamed_0 = 17;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const JMSG_LASTMSGCODE: C2RustUnnamed_0 = 128;
pub const JWRN_BOGUS_ICC: C2RustUnnamed_0 = 127;
pub const JWRN_TOO_MUCH_DATA: C2RustUnnamed_0 = 126;
pub const JWRN_NOT_SEQUENTIAL: C2RustUnnamed_0 = 125;
pub const JWRN_MUST_RESYNC: C2RustUnnamed_0 = 124;
pub const JWRN_JPEG_EOF: C2RustUnnamed_0 = 123;
pub const JWRN_JFIF_MAJOR: C2RustUnnamed_0 = 122;
pub const JWRN_HIT_MARKER: C2RustUnnamed_0 = 120;
pub const JWRN_EXTRANEOUS_DATA: C2RustUnnamed_0 = 119;
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
pub const JERR_BAD_CROP_SPEC: C2RustUnnamed_0 = 5;
pub const JERR_BAD_COMPONENT_ID: C2RustUnnamed_0 = 4;
pub const JERR_BAD_BUFFER_MODE: C2RustUnnamed_0 = 3;
pub const JERR_BAD_ALLOC_CHUNK: C2RustUnnamed_0 = 2;
pub const JERR_BAD_ALIGN_TYPE: C2RustUnnamed_0 = 1;
pub const JMSG_NOMESSAGE: C2RustUnnamed_0 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DCTSIZE2: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NUM_HUFF_TBLS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HUFF_LOOKAHEAD: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
unsafe extern "C" fn start_pass_phuff_decoder(mut cinfo: j_decompress_ptr) {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut is_DC_band: boolean = 0;
    let mut bad: boolean = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut coefi: ::core::ffi::c_int = 0;
    let mut tbl: ::core::ffi::c_int = 0;
    let mut pdtbl: *mut *mut d_derived_tbl = ::core::ptr::null_mut::<
        *mut d_derived_tbl,
    >();
    let mut coef_bit_ptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut prev_coef_bit_ptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<
        jpeg_component_info,
    >();
    is_DC_band = ((*cinfo).Ss == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as boolean;
    bad = FALSE as boolean;
    if is_DC_band != 0 {
        if (*cinfo).Se != 0 as ::core::ffi::c_int {
            bad = TRUE as boolean;
        }
    } else {
        if (*cinfo).Ss > (*cinfo).Se || (*cinfo).Se >= DCTSIZE2 {
            bad = TRUE as boolean;
        }
        if (*cinfo).comps_in_scan != 1 as ::core::ffi::c_int {
            bad = TRUE as boolean;
        }
    }
    if (*cinfo).Ah != 0 as ::core::ffi::c_int {
        if (*cinfo).Al != (*cinfo).Ah - 1 as ::core::ffi::c_int {
            bad = TRUE as boolean;
        }
    }
    if (*cinfo).Al > 13 as ::core::ffi::c_int {
        bad = TRUE as boolean;
    }
    if bad != 0 {
        (*(*cinfo).err).msg_code = JERR_BAD_PROGRESSION as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = (*cinfo).Ss;
        (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = (*cinfo).Se;
        (*(*cinfo).err).msg_parm.i[2 as ::core::ffi::c_int as usize] = (*cinfo).Ah;
        (*(*cinfo).err).msg_parm.i[3 as ::core::ffi::c_int as usize] = (*cinfo).Al;
        Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    ci = 0 as ::core::ffi::c_int;
    while ci < (*cinfo).comps_in_scan {
        let mut cindex: ::core::ffi::c_int = (*(*cinfo).cur_comp_info[ci as usize])
            .component_index;
        coef_bit_ptr = (&raw mut *(*cinfo).coef_bits.offset(cindex as isize)
            as *mut ::core::ffi::c_int)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
        prev_coef_bit_ptr = (&raw mut *(*cinfo)
            .coef_bits
            .offset((cindex + (*cinfo).num_components) as isize)
            as *mut ::core::ffi::c_int)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
        if is_DC_band == 0
            && *coef_bit_ptr.offset(0 as ::core::ffi::c_int as isize)
                < 0 as ::core::ffi::c_int
        {
            (*(*cinfo).err).msg_code = JWRN_BOGUS_PROGRESSION as ::core::ffi::c_int;
            (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = cindex;
            (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = 0
                as ::core::ffi::c_int;
            Some((*(*cinfo).err).emit_message.expect("non-null function pointer"))
                .expect(
                    "non-null function pointer",
                )(cinfo as j_common_ptr, -(1 as ::core::ffi::c_int));
        }
        coefi = if (*cinfo).Ss < 1 as ::core::ffi::c_int {
            (*cinfo).Ss
        } else {
            1 as ::core::ffi::c_int
        };
        while coefi
            <= (if (*cinfo).Se > 9 as ::core::ffi::c_int {
                (*cinfo).Se
            } else {
                9 as ::core::ffi::c_int
            })
        {
            if (*cinfo).input_scan_number > 1 as ::core::ffi::c_int {
                *prev_coef_bit_ptr.offset(coefi as isize) = *coef_bit_ptr
                    .offset(coefi as isize);
            } else {
                *prev_coef_bit_ptr.offset(coefi as isize) = 0 as ::core::ffi::c_int;
            }
            coefi += 1;
        }
        coefi = (*cinfo).Ss;
        while coefi <= (*cinfo).Se {
            let mut expected: ::core::ffi::c_int = if *coef_bit_ptr
                .offset(coefi as isize) < 0 as ::core::ffi::c_int
            {
                0 as ::core::ffi::c_int
            } else {
                *coef_bit_ptr.offset(coefi as isize)
            };
            if (*cinfo).Ah != expected {
                (*(*cinfo).err).msg_code = JWRN_BOGUS_PROGRESSION as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = cindex;
                (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] = coefi;
                Some((*(*cinfo).err).emit_message.expect("non-null function pointer"))
                    .expect(
                        "non-null function pointer",
                    )(cinfo as j_common_ptr, -(1 as ::core::ffi::c_int));
            }
            *coef_bit_ptr.offset(coefi as isize) = (*cinfo).Al;
            coefi += 1;
        }
        ci += 1;
    }
    if (*cinfo).Ah == 0 as ::core::ffi::c_int {
        if is_DC_band != 0 {
            (*entropy).pub_0.decode_mcu = Some(
                decode_mcu_DC_first
                    as unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean,
            )
                as Option<
                    unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean,
                >;
        } else {
            (*entropy).pub_0.decode_mcu = Some(
                decode_mcu_AC_first
                    as unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean,
            )
                as Option<
                    unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean,
                >;
        }
    } else if is_DC_band != 0 {
        (*entropy).pub_0.decode_mcu = Some(
            decode_mcu_DC_refine
                as unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean,
        ) as Option<unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean>;
    } else {
        (*entropy).pub_0.decode_mcu = Some(
            decode_mcu_AC_refine
                as unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean,
        ) as Option<unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean>;
    }
    ci = 0 as ::core::ffi::c_int;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        if is_DC_band != 0 {
            if (*cinfo).Ah == 0 as ::core::ffi::c_int {
                tbl = (*compptr).dc_tbl_no;
                pdtbl = (&raw mut (*entropy).derived_tbls as *mut *mut d_derived_tbl)
                    .offset(tbl as isize);
                jpeg_make_d_derived_tbl(cinfo, TRUE, tbl, pdtbl);
            }
        } else {
            tbl = (*compptr).ac_tbl_no;
            pdtbl = (&raw mut (*entropy).derived_tbls as *mut *mut d_derived_tbl)
                .offset(tbl as isize);
            jpeg_make_d_derived_tbl(cinfo, FALSE, tbl, pdtbl);
            (*entropy).ac_derived_tbl = (*entropy).derived_tbls[tbl as usize];
        }
        (*entropy).saved.last_dc_val[ci as usize] = 0 as ::core::ffi::c_int;
        ci += 1;
    }
    (*entropy).bitstate.bits_left = 0 as ::core::ffi::c_int;
    (*entropy).bitstate.get_buffer = 0 as bit_buf_type;
    (*entropy).pub_0.insufficient_data = FALSE as boolean;
    (*entropy).saved.EOBRUN = 0 as ::core::ffi::c_uint;
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
}
pub const NEG_1: ::core::ffi::c_uint = -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
unsafe extern "C" fn process_restart(mut cinfo: j_decompress_ptr) -> boolean {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut ci: ::core::ffi::c_int = 0;
    (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker)
        .discarded_bytes
        .wrapping_add(
            ((*entropy).bitstate.bits_left / 8 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
    (*entropy).bitstate.bits_left = 0 as ::core::ffi::c_int;
    if Some((*(*cinfo).marker).read_restart_marker.expect("non-null function pointer"))
        .expect("non-null function pointer")(cinfo) == 0
    {
        return FALSE;
    }
    ci = 0 as ::core::ffi::c_int;
    while ci < (*cinfo).comps_in_scan {
        (*entropy).saved.last_dc_val[ci as usize] = 0 as ::core::ffi::c_int;
        ci += 1;
    }
    (*entropy).saved.EOBRUN = 0 as ::core::ffi::c_uint;
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    if (*cinfo).unread_marker == 0 as ::core::ffi::c_int {
        (*entropy).pub_0.insufficient_data = FALSE as boolean;
    }
    return TRUE;
}
unsafe extern "C" fn decode_mcu_DC_first(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut Al: ::core::ffi::c_int = (*cinfo).Al;
    let mut s: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut blkn: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut block: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: ::core::ffi::c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: ::core::ptr::null::<JOCTET>(),
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: ::core::ptr::null_mut::<jpeg_decompress_struct>(),
    };
    let mut state: savable_state = savable_state {
        EOBRUN: 0,
        last_dc_val: [0; 4],
    };
    let mut tbl: *mut d_derived_tbl = ::core::ptr::null_mut::<d_derived_tbl>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<
        jpeg_component_info,
    >();
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as ::core::ffi::c_uint {
            if process_restart(cinfo) == 0 {
                return FALSE;
            }
        }
    }
    if (*entropy).pub_0.insufficient_data == 0 {
        br_state.cinfo = cinfo;
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        get_buffer = (*entropy).bitstate.get_buffer;
        bits_left = (*entropy).bitstate.bits_left;
        state = (*entropy).saved;
        blkn = 0 as ::core::ffi::c_int;
        while blkn < (*cinfo).blocks_in_MCU {
            block = *MCU_data.offset(blkn as isize);
            ci = (*cinfo).MCU_membership[blkn as usize];
            compptr = (*cinfo).cur_comp_info[ci as usize];
            tbl = (*entropy).derived_tbls[(*compptr).dc_tbl_no as usize];
            let mut current_block_31: u64;
            let mut nb: ::core::ffi::c_int = 0;
            let mut look: ::core::ffi::c_int = 0;
            if bits_left < HUFF_LOOKAHEAD {
                if jpeg_fill_bit_buffer(
                    &raw mut br_state,
                    get_buffer,
                    bits_left,
                    0 as ::core::ffi::c_int,
                ) == 0
                {
                    return 0 as boolean;
                }
                get_buffer = br_state.get_buffer;
                bits_left = br_state.bits_left;
                if bits_left < HUFF_LOOKAHEAD {
                    nb = 1 as ::core::ffi::c_int;
                    current_block_31 = 12818387049849721854;
                } else {
                    current_block_31 = 17478428563724192186;
                }
            } else {
                current_block_31 = 17478428563724192186;
            }
            match current_block_31 {
                17478428563724192186 => {
                    look = (get_buffer >> bits_left - 8 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int;
                    nb = (*tbl).lookup[look as usize] >> HUFF_LOOKAHEAD;
                    if nb <= HUFF_LOOKAHEAD {
                        bits_left -= nb;
                        s = (*tbl).lookup[look as usize]
                            & ((1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD)
                                - 1 as ::core::ffi::c_int;
                        current_block_31 = 18377268871191777778;
                    } else {
                        current_block_31 = 12818387049849721854;
                    }
                }
                _ => {}
            }
            match current_block_31 {
                12818387049849721854 => {
                    s = jpeg_huff_decode(
                        &raw mut br_state,
                        get_buffer,
                        bits_left,
                        tbl,
                        nb,
                    );
                    if s < 0 as ::core::ffi::c_int {
                        return 0 as boolean;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left;
                }
                _ => {}
            }
            if s != 0 {
                if bits_left < s {
                    if jpeg_fill_bit_buffer(&raw mut br_state, get_buffer, bits_left, s)
                        == 0
                    {
                        return 0 as boolean;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left;
                }
                bits_left -= s;
                r = (get_buffer >> bits_left) as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << s) - 1 as ::core::ffi::c_int;
                s = (if r < (1 as ::core::ffi::c_int) << s - 1 as ::core::ffi::c_int {
                    (r as ::core::ffi::c_uint)
                        .wrapping_add(
                            ((-(1 as ::core::ffi::c_int) as ::core::ffi::c_uint) << s)
                                .wrapping_add(1 as ::core::ffi::c_uint),
                        )
                } else {
                    r as ::core::ffi::c_uint
                }) as ::core::ffi::c_int;
            }
            if state.last_dc_val[ci as usize] >= 0 as ::core::ffi::c_int
                && s > INT_MAX - state.last_dc_val[ci as usize]
                || state.last_dc_val[ci as usize] < 0 as ::core::ffi::c_int
                    && s < INT_MIN - state.last_dc_val[ci as usize]
            {
                (*(*cinfo).err).msg_code = JERR_BAD_DCT_COEF as ::core::ffi::c_int;
                Some((*(*cinfo).err).error_exit.expect("non-null function pointer"))
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            s += state.last_dc_val[ci as usize];
            state.last_dc_val[ci as usize] = s;
            (*block)[0 as ::core::ffi::c_int as usize] = ((s as ::core::ffi::c_ulong)
                << Al) as JLONG as JCOEF;
            blkn += 1;
        }
        (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
        (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
        (*entropy).bitstate.get_buffer = get_buffer;
        (*entropy).bitstate.bits_left = bits_left;
        (*entropy).saved = state;
    }
    if (*cinfo).restart_interval != 0 {
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    }
    return TRUE;
}
unsafe extern "C" fn decode_mcu_AC_first(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut Se: ::core::ffi::c_int = (*cinfo).Se;
    let mut Al: ::core::ffi::c_int = (*cinfo).Al;
    let mut s: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut EOBRUN: ::core::ffi::c_uint = 0;
    let mut block: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: ::core::ffi::c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: ::core::ptr::null::<JOCTET>(),
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: ::core::ptr::null_mut::<jpeg_decompress_struct>(),
    };
    let mut tbl: *mut d_derived_tbl = ::core::ptr::null_mut::<d_derived_tbl>();
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as ::core::ffi::c_uint {
            if process_restart(cinfo) == 0 {
                return FALSE;
            }
        }
    }
    if (*entropy).pub_0.insufficient_data == 0 {
        EOBRUN = (*entropy).saved.EOBRUN;
        if EOBRUN > 0 as ::core::ffi::c_uint {
            EOBRUN = EOBRUN.wrapping_sub(1);
        } else {
            br_state.cinfo = cinfo;
            br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
            br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
            get_buffer = (*entropy).bitstate.get_buffer;
            bits_left = (*entropy).bitstate.bits_left;
            block = *MCU_data.offset(0 as ::core::ffi::c_int as isize);
            tbl = (*entropy).ac_derived_tbl;
            k = (*cinfo).Ss;
            while k <= Se {
                let mut current_block_30: u64;
                let mut nb: ::core::ffi::c_int = 0;
                let mut look: ::core::ffi::c_int = 0;
                if bits_left < HUFF_LOOKAHEAD {
                    if jpeg_fill_bit_buffer(
                        &raw mut br_state,
                        get_buffer,
                        bits_left,
                        0 as ::core::ffi::c_int,
                    ) == 0
                    {
                        return 0 as boolean;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left;
                    if bits_left < HUFF_LOOKAHEAD {
                        nb = 1 as ::core::ffi::c_int;
                        current_block_30 = 11616111767368389001;
                    } else {
                        current_block_30 = 13472856163611868459;
                    }
                } else {
                    current_block_30 = 13472856163611868459;
                }
                match current_block_30 {
                    13472856163611868459 => {
                        look = (get_buffer >> bits_left - 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int;
                        nb = (*tbl).lookup[look as usize] >> HUFF_LOOKAHEAD;
                        if nb <= HUFF_LOOKAHEAD {
                            bits_left -= nb;
                            s = (*tbl).lookup[look as usize]
                                & ((1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD)
                                    - 1 as ::core::ffi::c_int;
                            current_block_30 = 3275366147856559585;
                        } else {
                            current_block_30 = 11616111767368389001;
                        }
                    }
                    _ => {}
                }
                match current_block_30 {
                    11616111767368389001 => {
                        s = jpeg_huff_decode(
                            &raw mut br_state,
                            get_buffer,
                            bits_left,
                            tbl,
                            nb,
                        );
                        if s < 0 as ::core::ffi::c_int {
                            return 0 as boolean;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left;
                    }
                    _ => {}
                }
                r = s >> 4 as ::core::ffi::c_int;
                s &= 15 as ::core::ffi::c_int;
                if s != 0 {
                    k += r;
                    if bits_left < s {
                        if jpeg_fill_bit_buffer(
                            &raw mut br_state,
                            get_buffer,
                            bits_left,
                            s,
                        ) == 0
                        {
                            return 0 as boolean;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left;
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << s) - 1 as ::core::ffi::c_int;
                    s = (if r < (1 as ::core::ffi::c_int) << s - 1 as ::core::ffi::c_int
                    {
                        (r as ::core::ffi::c_uint)
                            .wrapping_add(
                                ((-(1 as ::core::ffi::c_int) as ::core::ffi::c_uint) << s)
                                    .wrapping_add(1 as ::core::ffi::c_uint),
                            )
                    } else {
                        r as ::core::ffi::c_uint
                    }) as ::core::ffi::c_int;
                    (*block)[*(&raw const jpeg_natural_order
                        as *const ::core::ffi::c_int)
                        .offset(k as isize) as usize] = ((s as ::core::ffi::c_ulong)
                        << Al) as JLONG as JCOEF;
                } else if r == 15 as ::core::ffi::c_int {
                    k += 15 as ::core::ffi::c_int;
                } else {
                    EOBRUN = ((1 as ::core::ffi::c_int) << r) as ::core::ffi::c_uint;
                    if r != 0 {
                        if bits_left < r {
                            if jpeg_fill_bit_buffer(
                                &raw mut br_state,
                                get_buffer,
                                bits_left,
                                r,
                            ) == 0
                            {
                                return 0 as boolean;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left;
                        }
                        bits_left -= r;
                        r = (get_buffer >> bits_left) as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << r) - 1 as ::core::ffi::c_int;
                        EOBRUN = EOBRUN.wrapping_add(r as ::core::ffi::c_uint);
                    }
                    EOBRUN = EOBRUN.wrapping_sub(1);
                    break;
                }
                k += 1;
            }
            (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
            (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
            (*entropy).bitstate.get_buffer = get_buffer;
            (*entropy).bitstate.bits_left = bits_left;
        }
        (*entropy).saved.EOBRUN = EOBRUN;
    }
    if (*cinfo).restart_interval != 0 {
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    }
    return TRUE;
}
unsafe extern "C" fn decode_mcu_DC_refine(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut p1: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << (*cinfo).Al;
    let mut blkn: ::core::ffi::c_int = 0;
    let mut block: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: ::core::ffi::c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: ::core::ptr::null::<JOCTET>(),
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: ::core::ptr::null_mut::<jpeg_decompress_struct>(),
    };
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as ::core::ffi::c_uint {
            if process_restart(cinfo) == 0 {
                return FALSE;
            }
        }
    }
    br_state.cinfo = cinfo;
    br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
    br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
    get_buffer = (*entropy).bitstate.get_buffer;
    bits_left = (*entropy).bitstate.bits_left;
    blkn = 0 as ::core::ffi::c_int;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        if bits_left < 1 as ::core::ffi::c_int {
            if jpeg_fill_bit_buffer(
                &raw mut br_state,
                get_buffer,
                bits_left,
                1 as ::core::ffi::c_int,
            ) == 0
            {
                return 0 as boolean;
            }
            get_buffer = br_state.get_buffer;
            bits_left = br_state.bits_left;
        }
        bits_left -= 1 as ::core::ffi::c_int;
        if (get_buffer >> bits_left) as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int != 0
        {
            (*block)[0 as ::core::ffi::c_int as usize] = ((*block)[0
                as ::core::ffi::c_int as usize] as ::core::ffi::c_int | p1) as JCOEF;
        }
        blkn += 1;
    }
    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
    (*entropy).bitstate.get_buffer = get_buffer;
    (*entropy).bitstate.bits_left = bits_left;
    if (*cinfo).restart_interval != 0 {
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    }
    return TRUE;
}
unsafe extern "C" fn decode_mcu_AC_refine(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut current_block: u64;
    let mut entropy: phuff_entropy_ptr = (*cinfo).entropy as phuff_entropy_ptr;
    let mut Se: ::core::ffi::c_int = (*cinfo).Se;
    let mut p1: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << (*cinfo).Al;
    let mut m1: ::core::ffi::c_int = ((-(1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
        << (*cinfo).Al) as ::core::ffi::c_int;
    let mut s: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut EOBRUN: ::core::ffi::c_uint = 0;
    let mut block: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut thiscoef: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: ::core::ffi::c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: ::core::ptr::null::<JOCTET>(),
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: ::core::ptr::null_mut::<jpeg_decompress_struct>(),
    };
    let mut tbl: *mut d_derived_tbl = ::core::ptr::null_mut::<d_derived_tbl>();
    let mut num_newnz: ::core::ffi::c_int = 0;
    let mut newnz_pos: [::core::ffi::c_int; 64] = [0; 64];
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as ::core::ffi::c_uint {
            if process_restart(cinfo) == 0 {
                return FALSE;
            }
        }
    }
    if (*entropy).pub_0.insufficient_data == 0 {
        br_state.cinfo = cinfo;
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        get_buffer = (*entropy).bitstate.get_buffer;
        bits_left = (*entropy).bitstate.bits_left;
        EOBRUN = (*entropy).saved.EOBRUN;
        block = *MCU_data.offset(0 as ::core::ffi::c_int as isize);
        tbl = (*entropy).ac_derived_tbl;
        num_newnz = 0 as ::core::ffi::c_int;
        k = (*cinfo).Ss;
        if EOBRUN == 0 as ::core::ffi::c_uint {
            current_block = 8457315219000651999;
        } else {
            current_block = 15514718523126015390;
        }
        's_90: loop {
            match current_block {
                15514718523126015390 => {
                    if EOBRUN > 0 as ::core::ffi::c_uint {
                        current_block = 6033931424626438518;
                        break;
                    } else {
                        current_block = 17395932908762866334;
                        break;
                    }
                }
                _ => {
                    if !(k <= Se) {
                        current_block = 15514718523126015390;
                        continue;
                    }
                    let mut nb: ::core::ffi::c_int = 0;
                    let mut look: ::core::ffi::c_int = 0;
                    if bits_left < HUFF_LOOKAHEAD {
                        if jpeg_fill_bit_buffer(
                            &raw mut br_state,
                            get_buffer,
                            bits_left,
                            0 as ::core::ffi::c_int,
                        ) == 0
                        {
                            current_block = 7898524873119221764;
                            break;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left;
                        if bits_left < HUFF_LOOKAHEAD {
                            nb = 1 as ::core::ffi::c_int;
                            current_block = 12246339261953109688;
                        } else {
                            current_block = 9828876828309294594;
                        }
                    } else {
                        current_block = 9828876828309294594;
                    }
                    match current_block {
                        9828876828309294594 => {
                            look = (get_buffer >> bits_left - 8 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int;
                            nb = (*tbl).lookup[look as usize] >> HUFF_LOOKAHEAD;
                            if nb <= HUFF_LOOKAHEAD {
                                bits_left -= nb;
                                s = (*tbl).lookup[look as usize]
                                    & ((1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD)
                                        - 1 as ::core::ffi::c_int;
                                current_block = 8704759739624374314;
                            } else {
                                current_block = 12246339261953109688;
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        12246339261953109688 => {
                            s = jpeg_huff_decode(
                                &raw mut br_state,
                                get_buffer,
                                bits_left,
                                tbl,
                                nb,
                            );
                            if s < 0 as ::core::ffi::c_int {
                                current_block = 7898524873119221764;
                                break;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left;
                        }
                        _ => {}
                    }
                    r = s >> 4 as ::core::ffi::c_int;
                    s &= 15 as ::core::ffi::c_int;
                    if s != 0 {
                        if s != 1 as ::core::ffi::c_int {
                            (*(*cinfo).err).msg_code = JWRN_HUFF_BAD_CODE
                                as ::core::ffi::c_int;
                            Some(
                                    (*(*cinfo).err)
                                        .emit_message
                                        .expect("non-null function pointer"),
                                )
                                .expect(
                                    "non-null function pointer",
                                )(cinfo as j_common_ptr, -(1 as ::core::ffi::c_int));
                        }
                        if bits_left < 1 as ::core::ffi::c_int {
                            if jpeg_fill_bit_buffer(
                                &raw mut br_state,
                                get_buffer,
                                bits_left,
                                1 as ::core::ffi::c_int,
                            ) == 0
                            {
                                current_block = 7898524873119221764;
                                break;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left;
                        }
                        bits_left -= 1 as ::core::ffi::c_int;
                        if (get_buffer >> bits_left) as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int != 0
                        {
                            s = p1;
                        } else {
                            s = m1;
                        }
                    } else if r != 15 as ::core::ffi::c_int {
                        EOBRUN = ((1 as ::core::ffi::c_int) << r) as ::core::ffi::c_uint;
                        if !(r != 0) {
                            current_block = 15514718523126015390;
                            continue;
                        }
                        if bits_left < r {
                            if jpeg_fill_bit_buffer(
                                &raw mut br_state,
                                get_buffer,
                                bits_left,
                                r,
                            ) == 0
                            {
                                current_block = 7898524873119221764;
                                break;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left;
                        }
                        bits_left -= r;
                        r = (get_buffer >> bits_left) as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << r) - 1 as ::core::ffi::c_int;
                        EOBRUN = EOBRUN.wrapping_add(r as ::core::ffi::c_uint);
                        current_block = 15514718523126015390;
                        continue;
                    }
                    loop {
                        thiscoef = (&raw mut *block as *mut JCOEF)
                            .offset(
                                *(&raw const jpeg_natural_order
                                    as *const ::core::ffi::c_int)
                                    .offset(k as isize) as isize,
                            ) as JCOEFPTR;
                        if *thiscoef as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            if bits_left < 1 as ::core::ffi::c_int {
                                if jpeg_fill_bit_buffer(
                                    &raw mut br_state,
                                    get_buffer,
                                    bits_left,
                                    1 as ::core::ffi::c_int,
                                ) == 0
                                {
                                    current_block = 7898524873119221764;
                                    break 's_90;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left;
                            }
                            bits_left -= 1 as ::core::ffi::c_int;
                            if (get_buffer >> bits_left) as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int != 0
                            {
                                if *thiscoef as ::core::ffi::c_int & p1
                                    == 0 as ::core::ffi::c_int
                                {
                                    if *thiscoef as ::core::ffi::c_int
                                        >= 0 as ::core::ffi::c_int
                                    {
                                        *thiscoef = (*thiscoef as ::core::ffi::c_int
                                            + p1 as JCOEF as ::core::ffi::c_int) as JCOEF;
                                    } else {
                                        *thiscoef = (*thiscoef as ::core::ffi::c_int
                                            + m1 as JCOEF as ::core::ffi::c_int) as JCOEF;
                                    }
                                }
                            }
                        } else {
                            r -= 1;
                            if r < 0 as ::core::ffi::c_int {
                                break;
                            }
                        }
                        k += 1;
                        if !(k <= Se) {
                            break;
                        }
                    }
                    if s != 0 {
                        let mut pos: ::core::ffi::c_int = *(&raw const jpeg_natural_order
                            as *const ::core::ffi::c_int)
                            .offset(k as isize);
                        (*block)[pos as usize] = s as JCOEF;
                        let fresh1 = num_newnz;
                        num_newnz = num_newnz + 1;
                        newnz_pos[fresh1 as usize] = pos;
                    }
                    k += 1;
                    current_block = 8457315219000651999;
                }
            }
        }
        loop {
            match current_block {
                7898524873119221764 => {
                    while num_newnz > 0 as ::core::ffi::c_int {
                        num_newnz -= 1;
                        (*block)[newnz_pos[num_newnz as usize] as usize] = 0 as JCOEF;
                    }
                    return FALSE;
                }
                6033931424626438518 => {
                    if k <= Se {
                        thiscoef = (&raw mut *block as *mut JCOEF)
                            .offset(
                                *(&raw const jpeg_natural_order
                                    as *const ::core::ffi::c_int)
                                    .offset(k as isize) as isize,
                            ) as JCOEFPTR;
                        if *thiscoef as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            if bits_left < 1 as ::core::ffi::c_int {
                                if jpeg_fill_bit_buffer(
                                    &raw mut br_state,
                                    get_buffer,
                                    bits_left,
                                    1 as ::core::ffi::c_int,
                                ) == 0
                                {
                                    current_block = 7898524873119221764;
                                    continue;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left;
                            }
                            bits_left -= 1 as ::core::ffi::c_int;
                            if (get_buffer >> bits_left) as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int != 0
                            {
                                if *thiscoef as ::core::ffi::c_int & p1
                                    == 0 as ::core::ffi::c_int
                                {
                                    if *thiscoef as ::core::ffi::c_int
                                        >= 0 as ::core::ffi::c_int
                                    {
                                        *thiscoef = (*thiscoef as ::core::ffi::c_int
                                            + p1 as JCOEF as ::core::ffi::c_int) as JCOEF;
                                    } else {
                                        *thiscoef = (*thiscoef as ::core::ffi::c_int
                                            + m1 as JCOEF as ::core::ffi::c_int) as JCOEF;
                                    }
                                }
                            }
                        }
                        k += 1;
                        current_block = 6033931424626438518;
                    } else {
                        EOBRUN = EOBRUN.wrapping_sub(1);
                        current_block = 17395932908762866334;
                    }
                }
                _ => {
                    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
                    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
                    (*entropy).bitstate.get_buffer = get_buffer;
                    (*entropy).bitstate.bits_left = bits_left;
                    (*entropy).saved.EOBRUN = EOBRUN;
                    break;
                }
            }
        }
    }
    if (*cinfo).restart_interval != 0 {
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    }
    return TRUE;
}
pub unsafe extern "C" fn jinit_phuff_decoder(mut cinfo: j_decompress_ptr) {
    let mut entropy: phuff_entropy_ptr = ::core::ptr::null_mut::<
        phuff_entropy_decoder,
    >();
    let mut coef_bit_ptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut ci: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    entropy = Some((*(*cinfo).mem).alloc_small.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<phuff_entropy_decoder>() as size_t,
    ) as phuff_entropy_ptr;
    (*cinfo).entropy = entropy as *mut jpeg_entropy_decoder as *mut jpeg_entropy_decoder;
    (*entropy).pub_0.start_pass = Some(
        start_pass_phuff_decoder as unsafe extern "C" fn(j_decompress_ptr) -> (),
    ) as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
    i = 0 as ::core::ffi::c_int;
    while i < NUM_HUFF_TBLS {
        (*entropy).derived_tbls[i as usize] = ::core::ptr::null_mut::<d_derived_tbl>();
        i += 1;
    }
    (*cinfo).coef_bits = Some(
            (*(*cinfo).mem).alloc_small.expect("non-null function pointer"),
        )
        .expect(
            "non-null function pointer",
        )(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (((*cinfo).num_components * 2 as ::core::ffi::c_int * DCTSIZE2) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut [::core::ffi::c_int; 64];
    coef_bit_ptr = (&raw mut *(*cinfo).coef_bits.offset(0 as ::core::ffi::c_int as isize)
        as *mut ::core::ffi::c_int)
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    ci = 0 as ::core::ffi::c_int;
    while ci < (*cinfo).num_components {
        i = 0 as ::core::ffi::c_int;
        while i < DCTSIZE2 {
            let fresh0 = coef_bit_ptr;
            coef_bit_ptr = coef_bit_ptr.offset(1);
            *fresh0 = -(1 as ::core::ffi::c_int);
            i += 1;
        }
        ci += 1;
    }
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
