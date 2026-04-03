#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}

extern "C" {
    fn jround_up(a: ::core::ffi::c_long, b: ::core::ffi::c_long) -> ::core::ffi::c_long;
    fn jcopy_sample_rows(
        input_array: JSAMPARRAY,
        source_row: ::core::ffi::c_int,
        output_array: JSAMPARRAY,
        dest_row: ::core::ffi::c_int,
        num_rows: ::core::ffi::c_int,
        num_cols: JDIMENSION,
    );
    fn jsimd_can_h2v2_upsample() -> ::core::ffi::c_int;
    fn jsimd_can_h2v1_upsample() -> ::core::ffi::c_int;
    fn jsimd_h2v2_upsample(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    );
    fn jsimd_h2v1_upsample(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    );
    fn jsimd_can_h2v2_fancy_upsample() -> ::core::ffi::c_int;
    fn jsimd_can_h2v1_fancy_upsample() -> ::core::ffi::c_int;
    fn jsimd_h2v2_fancy_upsample(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
    );
    fn jsimd_h2v1_fancy_upsample(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        input_data: JSAMPARRAY,
        output_data_ptr: *mut JSAMPARRAY,
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
pub type my_upsample_ptr = *mut my_upsampler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_upsampler {
    pub pub_0: jpeg_upsampler,
    pub color_buf: [JSAMPARRAY; 10],
    pub methods: [upsample1_ptr; 10],
    pub next_row_out: ::core::ffi::c_int,
    pub rows_to_go: JDIMENSION,
    pub rowgroup_height: [::core::ffi::c_int; 10],
    pub h_expand: [UINT8; 10],
    pub v_expand: [UINT8; 10],
}
pub type upsample1_ptr = Option<
    unsafe extern "C" fn(
        j_decompress_ptr,
        *mut jpeg_component_info,
        JSAMPARRAY,
        *mut JSAMPARRAY,
    ) -> (),
>;
pub const JERR_FRACT_SAMPLE_NOTIMPL: C2RustUnnamed_0 = 39;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn start_pass_upsample(mut cinfo: j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    (*upsample).next_row_out = (*cinfo).max_v_samp_factor;
    (*upsample).rows_to_go = (*cinfo).output_height;
}
unsafe extern "C" fn sep_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: *mut JDIMENSION,
    mut in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut ci: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut num_rows: JDIMENSION = 0;
    if (*upsample).next_row_out >= (*cinfo).max_v_samp_factor {
        ci = 0 as ::core::ffi::c_int;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            Some(
                (*(::core::ptr::addr_of_mut!((*upsample).methods) as *mut upsample1_ptr).offset(ci as isize))
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                compptr,
                (*input_buf.offset(ci as isize)).offset(
                    (*in_row_group_ctr)
                        .wrapping_mul((*upsample).rowgroup_height[ci as usize] as JDIMENSION)
                        as isize,
                ),
                (::core::ptr::addr_of_mut!((*upsample).color_buf) as *mut JSAMPARRAY).offset(ci as isize),
            );
            ci += 1;
            compptr = compptr.offset(1);
        }
        (*upsample).next_row_out = 0 as ::core::ffi::c_int;
    }
    num_rows = ((*cinfo).max_v_samp_factor - (*upsample).next_row_out) as JDIMENSION;
    if num_rows > (*upsample).rows_to_go {
        num_rows = (*upsample).rows_to_go;
    }
    out_rows_avail = out_rows_avail.wrapping_sub(*out_row_ctr);
    if num_rows > out_rows_avail {
        num_rows = out_rows_avail;
    }
    Some(
        (*(*cinfo).cconvert)
            .color_convert
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        ::core::ptr::addr_of_mut!((*upsample).color_buf) as JSAMPIMAGE,
        (*upsample).next_row_out as JDIMENSION,
        output_buf.offset(*out_row_ctr as isize),
        num_rows as ::core::ffi::c_int,
    );
    *out_row_ctr = (*out_row_ctr).wrapping_add(num_rows);
    (*upsample).rows_to_go = (*upsample).rows_to_go.wrapping_sub(num_rows);
    (*upsample).next_row_out = ((*upsample).next_row_out as JDIMENSION).wrapping_add(num_rows)
        as ::core::ffi::c_int as ::core::ffi::c_int;
    if (*upsample).next_row_out >= (*cinfo).max_v_samp_factor {
        *in_row_group_ctr = (*in_row_group_ctr).wrapping_add(1);
    }
}
unsafe extern "C" fn fullsize_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    *output_data_ptr = input_data;
}
unsafe extern "C" fn noop_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    *output_data_ptr = ::core::ptr::null_mut::<JSAMPROW>();
}
unsafe extern "C" fn int_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut output_data: JSAMPARRAY = *output_data_ptr;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut invalue: JSAMPLE = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut outend: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut h_expand: ::core::ffi::c_int = 0;
    let mut v_expand: ::core::ffi::c_int = 0;
    let mut inrow: ::core::ffi::c_int = 0;
    let mut outrow: ::core::ffi::c_int = 0;
    h_expand = (*upsample).h_expand[(*compptr).component_index as usize] as ::core::ffi::c_int;
    v_expand = (*upsample).v_expand[(*compptr).component_index as usize] as ::core::ffi::c_int;
    outrow = 0 as ::core::ffi::c_int;
    inrow = outrow;
    while outrow < (*cinfo).max_v_samp_factor {
        inptr = *input_data.offset(inrow as isize);
        outptr = *output_data.offset(outrow as isize);
        outend = outptr.offset((*cinfo).output_width as isize);
        while outptr < outend {
            let fresh0 = inptr;
            inptr = inptr.offset(1);
            invalue = *fresh0;
            h = h_expand;
            while h > 0 as ::core::ffi::c_int {
                let fresh1 = outptr;
                outptr = outptr.offset(1);
                *fresh1 = invalue;
                h -= 1;
            }
        }
        if v_expand > 1 as ::core::ffi::c_int {
            jcopy_sample_rows(
                output_data,
                outrow,
                output_data,
                outrow + 1 as ::core::ffi::c_int,
                v_expand - 1 as ::core::ffi::c_int,
                (*cinfo).output_width,
            );
        }
        inrow += 1;
        outrow += v_expand;
    }
}
unsafe extern "C" fn h2v1_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    let mut output_data: JSAMPARRAY = *output_data_ptr;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut invalue: JSAMPLE = 0;
    let mut outend: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inrow: ::core::ffi::c_int = 0;
    inrow = 0 as ::core::ffi::c_int;
    while inrow < (*cinfo).max_v_samp_factor {
        inptr = *input_data.offset(inrow as isize);
        outptr = *output_data.offset(inrow as isize);
        outend = outptr.offset((*cinfo).output_width as isize);
        while outptr < outend {
            let fresh22 = inptr;
            inptr = inptr.offset(1);
            invalue = *fresh22;
            let fresh23 = outptr;
            outptr = outptr.offset(1);
            *fresh23 = invalue;
            let fresh24 = outptr;
            outptr = outptr.offset(1);
            *fresh24 = invalue;
        }
        inrow += 1;
    }
}
unsafe extern "C" fn h2v2_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    let mut output_data: JSAMPARRAY = *output_data_ptr;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut invalue: JSAMPLE = 0;
    let mut outend: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inrow: ::core::ffi::c_int = 0;
    let mut outrow: ::core::ffi::c_int = 0;
    outrow = 0 as ::core::ffi::c_int;
    inrow = outrow;
    while outrow < (*cinfo).max_v_samp_factor {
        inptr = *input_data.offset(inrow as isize);
        outptr = *output_data.offset(outrow as isize);
        outend = outptr.offset((*cinfo).output_width as isize);
        while outptr < outend {
            let fresh2 = inptr;
            inptr = inptr.offset(1);
            invalue = *fresh2;
            let fresh3 = outptr;
            outptr = outptr.offset(1);
            *fresh3 = invalue;
            let fresh4 = outptr;
            outptr = outptr.offset(1);
            *fresh4 = invalue;
        }
        jcopy_sample_rows(
            output_data,
            outrow,
            output_data,
            outrow + 1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            (*cinfo).output_width,
        );
        inrow += 1;
        outrow += 2 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn h2v1_fancy_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    let mut output_data: JSAMPARRAY = *output_data_ptr;
    let mut inptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut invalue: ::core::ffi::c_int = 0;
    let mut colctr: JDIMENSION = 0;
    let mut inrow: ::core::ffi::c_int = 0;
    inrow = 0 as ::core::ffi::c_int;
    while inrow < (*cinfo).max_v_samp_factor {
        inptr = *input_data.offset(inrow as isize);
        outptr = *output_data.offset(inrow as isize);
        let fresh25 = inptr;
        inptr = inptr.offset(1);
        invalue = *fresh25 as ::core::ffi::c_int;
        let fresh26 = outptr;
        outptr = outptr.offset(1);
        *fresh26 = invalue as JSAMPLE;
        let fresh27 = outptr;
        outptr = outptr.offset(1);
        *fresh27 = (invalue * 3 as ::core::ffi::c_int
            + *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as JSAMPLE;
        colctr = (*compptr).downsampled_width.wrapping_sub(2 as JDIMENSION);
        while colctr > 0 as JDIMENSION {
            let fresh28 = inptr;
            inptr = inptr.offset(1);
            invalue = *fresh28 as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
            let fresh29 = outptr;
            outptr = outptr.offset(1);
            *fresh29 = (invalue
                + *inptr.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as JSAMPLE;
            let fresh30 = outptr;
            outptr = outptr.offset(1);
            *fresh30 = (invalue
                + *inptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as JSAMPLE;
            colctr = colctr.wrapping_sub(1);
        }
        invalue = *inptr as ::core::ffi::c_int;
        let fresh31 = outptr;
        outptr = outptr.offset(1);
        *fresh31 = (invalue * 3 as ::core::ffi::c_int
            + *inptr.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as JSAMPLE;
        let fresh32 = outptr;
        outptr = outptr.offset(1);
        *fresh32 = invalue as JSAMPLE;
        inrow += 1;
    }
}
unsafe extern "C" fn h1v2_fancy_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    let mut output_data: JSAMPARRAY = *output_data_ptr;
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut thiscolsum: ::core::ffi::c_int = 0;
    let mut bias: ::core::ffi::c_int = 0;
    let mut colctr: JDIMENSION = 0;
    let mut inrow: ::core::ffi::c_int = 0;
    let mut outrow: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    outrow = 0 as ::core::ffi::c_int;
    inrow = outrow;
    while outrow < (*cinfo).max_v_samp_factor {
        v = 0 as ::core::ffi::c_int;
        while v < 2 as ::core::ffi::c_int {
            inptr0 = *input_data.offset(inrow as isize);
            if v == 0 as ::core::ffi::c_int {
                inptr1 = *input_data.offset((inrow - 1 as ::core::ffi::c_int) as isize);
                bias = 1 as ::core::ffi::c_int;
            } else {
                inptr1 = *input_data.offset((inrow + 1 as ::core::ffi::c_int) as isize);
                bias = 2 as ::core::ffi::c_int;
            }
            let fresh18 = outrow;
            outrow = outrow + 1;
            outptr = *output_data.offset(fresh18 as isize);
            colctr = 0 as JDIMENSION;
            while colctr < (*compptr).downsampled_width {
                let fresh19 = inptr0;
                inptr0 = inptr0.offset(1);
                let fresh20 = inptr1;
                inptr1 = inptr1.offset(1);
                thiscolsum = *fresh19 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                    + *fresh20 as ::core::ffi::c_int;
                let fresh21 = outptr;
                outptr = outptr.offset(1);
                *fresh21 = (thiscolsum + bias >> 2 as ::core::ffi::c_int) as JSAMPLE;
                colctr = colctr.wrapping_add(1);
            }
            v += 1;
        }
        inrow += 1;
    }
}
unsafe extern "C" fn h2v2_fancy_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    let mut output_data: JSAMPARRAY = *output_data_ptr;
    let mut inptr0: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inptr1: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut thiscolsum: ::core::ffi::c_int = 0;
    let mut lastcolsum: ::core::ffi::c_int = 0;
    let mut nextcolsum: ::core::ffi::c_int = 0;
    let mut colctr: JDIMENSION = 0;
    let mut inrow: ::core::ffi::c_int = 0;
    let mut outrow: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    outrow = 0 as ::core::ffi::c_int;
    inrow = outrow;
    while outrow < (*cinfo).max_v_samp_factor {
        v = 0 as ::core::ffi::c_int;
        while v < 2 as ::core::ffi::c_int {
            inptr0 = *input_data.offset(inrow as isize);
            if v == 0 as ::core::ffi::c_int {
                inptr1 = *input_data.offset((inrow - 1 as ::core::ffi::c_int) as isize);
            } else {
                inptr1 = *input_data.offset((inrow + 1 as ::core::ffi::c_int) as isize);
            }
            let fresh5 = outrow;
            outrow = outrow + 1;
            outptr = *output_data.offset(fresh5 as isize);
            let fresh6 = inptr0;
            inptr0 = inptr0.offset(1);
            let fresh7 = inptr1;
            inptr1 = inptr1.offset(1);
            thiscolsum = *fresh6 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                + *fresh7 as ::core::ffi::c_int;
            let fresh8 = inptr0;
            inptr0 = inptr0.offset(1);
            let fresh9 = inptr1;
            inptr1 = inptr1.offset(1);
            nextcolsum = *fresh8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                + *fresh9 as ::core::ffi::c_int;
            let fresh10 = outptr;
            outptr = outptr.offset(1);
            *fresh10 = (thiscolsum * 4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as JSAMPLE;
            let fresh11 = outptr;
            outptr = outptr.offset(1);
            *fresh11 = (thiscolsum * 3 as ::core::ffi::c_int + nextcolsum + 7 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as JSAMPLE;
            lastcolsum = thiscolsum;
            thiscolsum = nextcolsum;
            colctr = (*compptr).downsampled_width.wrapping_sub(2 as JDIMENSION);
            while colctr > 0 as JDIMENSION {
                let fresh12 = inptr0;
                inptr0 = inptr0.offset(1);
                let fresh13 = inptr1;
                inptr1 = inptr1.offset(1);
                nextcolsum = *fresh12 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                    + *fresh13 as ::core::ffi::c_int;
                let fresh14 = outptr;
                outptr = outptr.offset(1);
                *fresh14 =
                    (thiscolsum * 3 as ::core::ffi::c_int + lastcolsum + 8 as ::core::ffi::c_int
                        >> 4 as ::core::ffi::c_int) as JSAMPLE;
                let fresh15 = outptr;
                outptr = outptr.offset(1);
                *fresh15 =
                    (thiscolsum * 3 as ::core::ffi::c_int + nextcolsum + 7 as ::core::ffi::c_int
                        >> 4 as ::core::ffi::c_int) as JSAMPLE;
                lastcolsum = thiscolsum;
                thiscolsum = nextcolsum;
                colctr = colctr.wrapping_sub(1);
            }
            let fresh16 = outptr;
            outptr = outptr.offset(1);
            *fresh16 = (thiscolsum * 3 as ::core::ffi::c_int + lastcolsum + 8 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as JSAMPLE;
            let fresh17 = outptr;
            outptr = outptr.offset(1);
            *fresh17 = (thiscolsum * 4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as JSAMPLE;
            v += 1;
        }
        inrow += 1;
    }
}
pub unsafe extern "C" fn jinit_upsampler(mut cinfo: j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = ::core::ptr::null_mut::<my_upsampler>();
    let mut ci: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut need_buffer: boolean = 0;
    let mut do_fancy: boolean = 0;
    let mut h_in_group: ::core::ffi::c_int = 0;
    let mut v_in_group: ::core::ffi::c_int = 0;
    let mut h_out_group: ::core::ffi::c_int = 0;
    let mut v_out_group: ::core::ffi::c_int = 0;
    if (*(*cinfo).master).jinit_upsampler_no_alloc == 0 {
        upsample = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ::core::mem::size_of::<my_upsampler>() as size_t,
        ) as my_upsample_ptr;
        (*cinfo).upsample = upsample as *mut jpeg_upsampler as *mut jpeg_upsampler;
        (*upsample).pub_0.start_pass =
            Some(start_pass_upsample as unsafe extern "C" fn(j_decompress_ptr) -> ())
                as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
        (*upsample).pub_0.upsample = Some(
            sep_upsample
                as unsafe extern "C" fn(
                    j_decompress_ptr,
                    JSAMPIMAGE,
                    *mut JDIMENSION,
                    JDIMENSION,
                    JSAMPARRAY,
                    *mut JDIMENSION,
                    JDIMENSION,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    j_decompress_ptr,
                    JSAMPIMAGE,
                    *mut JDIMENSION,
                    JDIMENSION,
                    JSAMPARRAY,
                    *mut JDIMENSION,
                    JDIMENSION,
                ) -> (),
            >;
        (*upsample).pub_0.need_context_rows = FALSE as boolean;
    } else {
        upsample = (*cinfo).upsample as my_upsample_ptr;
    }
    if (*cinfo).CCIR601_sampling != 0 {
        (*(*cinfo).err).msg_code = JERR_CCIR601_NOTIMPL as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    do_fancy = ((*cinfo).do_fancy_upsampling != 0
        && (*cinfo).min_DCT_h_scaled_size > 1 as ::core::ffi::c_int)
        as ::core::ffi::c_int as boolean;
    ci = 0 as ::core::ffi::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        h_in_group = (*compptr).h_samp_factor * (*compptr).DCT_h_scaled_size
            / (*cinfo).min_DCT_h_scaled_size;
        v_in_group = (*compptr).v_samp_factor * (*compptr).DCT_h_scaled_size
            / (*cinfo).min_DCT_h_scaled_size;
        h_out_group = (*cinfo).max_h_samp_factor;
        v_out_group = (*cinfo).max_v_samp_factor;
        (*upsample).rowgroup_height[ci as usize] = v_in_group;
        need_buffer = TRUE as boolean;
        if (*compptr).component_needed == 0 {
            (*upsample).methods[ci as usize] = Some(
                noop_upsample
                    as unsafe extern "C" fn(
                        j_decompress_ptr,
                        *mut jpeg_component_info,
                        JSAMPARRAY,
                        *mut JSAMPARRAY,
                    ) -> (),
            ) as upsample1_ptr;
            need_buffer = FALSE as boolean;
        } else if h_in_group == h_out_group && v_in_group == v_out_group {
            (*upsample).methods[ci as usize] = Some(
                fullsize_upsample
                    as unsafe extern "C" fn(
                        j_decompress_ptr,
                        *mut jpeg_component_info,
                        JSAMPARRAY,
                        *mut JSAMPARRAY,
                    ) -> (),
            ) as upsample1_ptr;
            need_buffer = FALSE as boolean;
        } else if h_in_group * 2 as ::core::ffi::c_int == h_out_group && v_in_group == v_out_group {
            if do_fancy != 0 && (*compptr).downsampled_width > 2 as JDIMENSION {
                if jsimd_can_h2v1_fancy_upsample() != 0 {
                    (*upsample).methods[ci as usize] = Some(
                        jsimd_h2v1_fancy_upsample
                            as unsafe extern "C" fn(
                                j_decompress_ptr,
                                *mut jpeg_component_info,
                                JSAMPARRAY,
                                *mut JSAMPARRAY,
                            ) -> (),
                    ) as upsample1_ptr;
                } else {
                    (*upsample).methods[ci as usize] = Some(
                        h2v1_fancy_upsample
                            as unsafe extern "C" fn(
                                j_decompress_ptr,
                                *mut jpeg_component_info,
                                JSAMPARRAY,
                                *mut JSAMPARRAY,
                            ) -> (),
                    ) as upsample1_ptr;
                }
            } else if jsimd_can_h2v1_upsample() != 0 {
                (*upsample).methods[ci as usize] = Some(
                    jsimd_h2v1_upsample
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            *mut jpeg_component_info,
                            JSAMPARRAY,
                            *mut JSAMPARRAY,
                        ) -> (),
                ) as upsample1_ptr;
            } else {
                (*upsample).methods[ci as usize] = Some(
                    h2v1_upsample
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            *mut jpeg_component_info,
                            JSAMPARRAY,
                            *mut JSAMPARRAY,
                        ) -> (),
                ) as upsample1_ptr;
            }
        } else if h_in_group == h_out_group
            && v_in_group * 2 as ::core::ffi::c_int == v_out_group
            && do_fancy != 0
        {
            (*upsample).methods[ci as usize] = Some(
                h1v2_fancy_upsample
                    as unsafe extern "C" fn(
                        j_decompress_ptr,
                        *mut jpeg_component_info,
                        JSAMPARRAY,
                        *mut JSAMPARRAY,
                    ) -> (),
            ) as upsample1_ptr;
            (*upsample).pub_0.need_context_rows = TRUE as boolean;
        } else if h_in_group * 2 as ::core::ffi::c_int == h_out_group
            && v_in_group * 2 as ::core::ffi::c_int == v_out_group
        {
            if do_fancy != 0 && (*compptr).downsampled_width > 2 as JDIMENSION {
                if jsimd_can_h2v2_fancy_upsample() != 0 {
                    (*upsample).methods[ci as usize] = Some(
                        jsimd_h2v2_fancy_upsample
                            as unsafe extern "C" fn(
                                j_decompress_ptr,
                                *mut jpeg_component_info,
                                JSAMPARRAY,
                                *mut JSAMPARRAY,
                            ) -> (),
                    ) as upsample1_ptr;
                } else {
                    (*upsample).methods[ci as usize] = Some(
                        h2v2_fancy_upsample
                            as unsafe extern "C" fn(
                                j_decompress_ptr,
                                *mut jpeg_component_info,
                                JSAMPARRAY,
                                *mut JSAMPARRAY,
                            ) -> (),
                    ) as upsample1_ptr;
                }
                (*upsample).pub_0.need_context_rows = TRUE as boolean;
            } else if jsimd_can_h2v2_upsample() != 0 {
                (*upsample).methods[ci as usize] = Some(
                    jsimd_h2v2_upsample
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            *mut jpeg_component_info,
                            JSAMPARRAY,
                            *mut JSAMPARRAY,
                        ) -> (),
                ) as upsample1_ptr;
            } else {
                (*upsample).methods[ci as usize] = Some(
                    h2v2_upsample
                        as unsafe extern "C" fn(
                            j_decompress_ptr,
                            *mut jpeg_component_info,
                            JSAMPARRAY,
                            *mut JSAMPARRAY,
                        ) -> (),
                ) as upsample1_ptr;
            }
        } else if h_out_group % h_in_group == 0 as ::core::ffi::c_int
            && v_out_group % v_in_group == 0 as ::core::ffi::c_int
        {
            (*upsample).methods[ci as usize] = Some(
                int_upsample
                    as unsafe extern "C" fn(
                        j_decompress_ptr,
                        *mut jpeg_component_info,
                        JSAMPARRAY,
                        *mut JSAMPARRAY,
                    ) -> (),
            ) as upsample1_ptr;
            (*upsample).h_expand[ci as usize] = (h_out_group / h_in_group) as UINT8;
            (*upsample).v_expand[ci as usize] = (v_out_group / v_in_group) as UINT8;
        } else {
            (*(*cinfo).err).msg_code = JERR_FRACT_SAMPLE_NOTIMPL as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if need_buffer != 0 && (*(*cinfo).master).jinit_upsampler_no_alloc == 0 {
            (*upsample).color_buf[ci as usize] = Some(
                (*(*cinfo).mem)
                    .alloc_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                jround_up(
                    (*cinfo).output_width as ::core::ffi::c_long,
                    (*cinfo).max_h_samp_factor as ::core::ffi::c_long,
                ) as JDIMENSION,
                (*cinfo).max_v_samp_factor as JDIMENSION,
            );
        }
        ci += 1;
        compptr = compptr.offset(1);
    }
}
