#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}

extern "C" {
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
    fn jpeg_alloc_huff_table(cinfo: j_common_ptr) -> *mut JHUFF_TBL;
    static jpeg_natural_order: [::core::ffi::c_int; 0];
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
pub struct jpeg_scan_info {
    pub comps_in_scan: ::core::ffi::c_int,
    pub component_index: [::core::ffi::c_int; 4],
    pub Ss: ::core::ffi::c_int,
    pub Se: ::core::ffi::c_int,
    pub Ah: ::core::ffi::c_int,
    pub Al: ::core::ffi::c_int,
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
pub type JLONG = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct d_derived_tbl {
    pub maxcode: [JLONG; 18],
    pub valoffset: [JLONG; 18],
    pub pub_0: *mut JHUFF_TBL,
    pub lookup: [::core::ffi::c_int; 256],
}
pub type huff_entropy_ptr = *mut huff_entropy_decoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huff_entropy_decoder {
    pub pub_0: jpeg_entropy_decoder,
    pub bitstate: bitread_perm_state,
    pub saved: savable_state,
    pub restarts_to_go: ::core::ffi::c_uint,
    pub dc_derived_tbls: [*mut d_derived_tbl; 4],
    pub ac_derived_tbls: [*mut d_derived_tbl; 4],
    pub dc_cur_tbls: [*mut d_derived_tbl; 10],
    pub ac_cur_tbls: [*mut d_derived_tbl; 10],
    pub dc_needed: [boolean; 10],
    pub ac_needed: [boolean; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct savable_state {
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
pub const JWRN_HIT_MARKER: C2RustUnnamed_0 = 120;
pub const JWRN_HUFF_BAD_CODE: C2RustUnnamed_0 = 121;
pub const JERR_BAD_HUFF_TABLE: C2RustUnnamed_0 = 9;
pub const JERR_NO_HUFF_TABLE: C2RustUnnamed_0 = 52;
pub const JWRN_NOT_SEQUENTIAL: C2RustUnnamed_0 = 125;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const JMSG_LASTMSGCODE: C2RustUnnamed_0 = 128;
pub const JWRN_BOGUS_ICC: C2RustUnnamed_0 = 127;
pub const JWRN_TOO_MUCH_DATA: C2RustUnnamed_0 = 126;
pub const JWRN_MUST_RESYNC: C2RustUnnamed_0 = 124;
pub const JWRN_JPEG_EOF: C2RustUnnamed_0 = 123;
pub const JWRN_JFIF_MAJOR: C2RustUnnamed_0 = 122;
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
pub const DCTSIZE2: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NUM_HUFF_TBLS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HUFF_LOOKAHEAD: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const BIT_BUF_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
unsafe extern "C" fn add_huff_table(
    mut cinfo: j_common_ptr,
    mut htblptr: *mut *mut JHUFF_TBL,
    mut bits: *const UINT8,
    mut val: *const UINT8,
) {
    let mut nsymbols: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    if (*htblptr).is_null() {
        *htblptr = jpeg_alloc_huff_table(cinfo);
    } else {
        return;
    }
    memcpy(
        ::core::ptr::addr_of_mut!((**htblptr).bits) as *mut UINT8 as *mut ::core::ffi::c_void,
        bits as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[UINT8; 17]>() as size_t,
    );
    nsymbols = 0 as ::core::ffi::c_int;
    len = 1 as ::core::ffi::c_int;
    while len <= 16 as ::core::ffi::c_int {
        nsymbols += *bits.offset(len as isize) as ::core::ffi::c_int;
        len += 1;
    }
    if nsymbols < 1 as ::core::ffi::c_int || nsymbols > 256 as ::core::ffi::c_int {
        (*(*cinfo).err).msg_code = JERR_BAD_HUFF_TABLE as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    memcpy(
        ::core::ptr::addr_of_mut!((**htblptr).huffval) as *mut UINT8 as *mut ::core::ffi::c_void,
        val as *const ::core::ffi::c_void,
        (nsymbols as size_t).wrapping_mul(::core::mem::size_of::<UINT8>() as size_t),
    );
    memset(
        (::core::ptr::addr_of_mut!((**htblptr).huffval) as *mut UINT8).offset(nsymbols as isize) as *mut UINT8
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ((256 as ::core::ffi::c_int - nsymbols) as size_t)
            .wrapping_mul(::core::mem::size_of::<UINT8>() as size_t),
    );
    (**htblptr).sent_table = FALSE as boolean;
}
unsafe extern "C" fn std_huff_tables(mut cinfo: j_common_ptr) {
    let mut dc_huff_tbl_ptrs: *mut *mut JHUFF_TBL = ::core::ptr::null_mut::<*mut JHUFF_TBL>();
    let mut ac_huff_tbl_ptrs: *mut *mut JHUFF_TBL = ::core::ptr::null_mut::<*mut JHUFF_TBL>();
    static mut bits_dc_luminance: [UINT8; 17] = [
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        5 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
    ];
    static mut val_dc_luminance: [UINT8; 12] = [
        0 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        2 as ::core::ffi::c_int as UINT8,
        3 as ::core::ffi::c_int as UINT8,
        4 as ::core::ffi::c_int as UINT8,
        5 as ::core::ffi::c_int as UINT8,
        6 as ::core::ffi::c_int as UINT8,
        7 as ::core::ffi::c_int as UINT8,
        8 as ::core::ffi::c_int as UINT8,
        9 as ::core::ffi::c_int as UINT8,
        10 as ::core::ffi::c_int as UINT8,
        11 as ::core::ffi::c_int as UINT8,
    ];
    static mut bits_dc_chrominance: [UINT8; 17] = [
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        3 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
    ];
    static mut val_dc_chrominance: [UINT8; 12] = [
        0 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        2 as ::core::ffi::c_int as UINT8,
        3 as ::core::ffi::c_int as UINT8,
        4 as ::core::ffi::c_int as UINT8,
        5 as ::core::ffi::c_int as UINT8,
        6 as ::core::ffi::c_int as UINT8,
        7 as ::core::ffi::c_int as UINT8,
        8 as ::core::ffi::c_int as UINT8,
        9 as ::core::ffi::c_int as UINT8,
        10 as ::core::ffi::c_int as UINT8,
        11 as ::core::ffi::c_int as UINT8,
    ];
    static mut bits_ac_luminance: [UINT8; 17] = [
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        2 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        3 as ::core::ffi::c_int as UINT8,
        3 as ::core::ffi::c_int as UINT8,
        2 as ::core::ffi::c_int as UINT8,
        4 as ::core::ffi::c_int as UINT8,
        3 as ::core::ffi::c_int as UINT8,
        5 as ::core::ffi::c_int as UINT8,
        5 as ::core::ffi::c_int as UINT8,
        4 as ::core::ffi::c_int as UINT8,
        4 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        0x7d as ::core::ffi::c_int as UINT8,
    ];
    static mut val_ac_luminance: [UINT8; 162] = [
        0x1 as ::core::ffi::c_int as UINT8,
        0x2 as ::core::ffi::c_int as UINT8,
        0x3 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        0x4 as ::core::ffi::c_int as UINT8,
        0x11 as ::core::ffi::c_int as UINT8,
        0x5 as ::core::ffi::c_int as UINT8,
        0x12 as ::core::ffi::c_int as UINT8,
        0x21 as ::core::ffi::c_int as UINT8,
        0x31 as ::core::ffi::c_int as UINT8,
        0x41 as ::core::ffi::c_int as UINT8,
        0x6 as ::core::ffi::c_int as UINT8,
        0x13 as ::core::ffi::c_int as UINT8,
        0x51 as ::core::ffi::c_int as UINT8,
        0x61 as ::core::ffi::c_int as UINT8,
        0x7 as ::core::ffi::c_int as UINT8,
        0x22 as ::core::ffi::c_int as UINT8,
        0x71 as ::core::ffi::c_int as UINT8,
        0x14 as ::core::ffi::c_int as UINT8,
        0x32 as ::core::ffi::c_int as UINT8,
        0x81 as ::core::ffi::c_int as UINT8,
        0x91 as ::core::ffi::c_int as UINT8,
        0xa1 as ::core::ffi::c_int as UINT8,
        0x8 as ::core::ffi::c_int as UINT8,
        0x23 as ::core::ffi::c_int as UINT8,
        0x42 as ::core::ffi::c_int as UINT8,
        0xb1 as ::core::ffi::c_int as UINT8,
        0xc1 as ::core::ffi::c_int as UINT8,
        0x15 as ::core::ffi::c_int as UINT8,
        0x52 as ::core::ffi::c_int as UINT8,
        0xd1 as ::core::ffi::c_int as UINT8,
        0xf0 as ::core::ffi::c_int as UINT8,
        0x24 as ::core::ffi::c_int as UINT8,
        0x33 as ::core::ffi::c_int as UINT8,
        0x62 as ::core::ffi::c_int as UINT8,
        0x72 as ::core::ffi::c_int as UINT8,
        0x82 as ::core::ffi::c_int as UINT8,
        0x9 as ::core::ffi::c_int as UINT8,
        0xa as ::core::ffi::c_int as UINT8,
        0x16 as ::core::ffi::c_int as UINT8,
        0x17 as ::core::ffi::c_int as UINT8,
        0x18 as ::core::ffi::c_int as UINT8,
        0x19 as ::core::ffi::c_int as UINT8,
        0x1a as ::core::ffi::c_int as UINT8,
        0x25 as ::core::ffi::c_int as UINT8,
        0x26 as ::core::ffi::c_int as UINT8,
        0x27 as ::core::ffi::c_int as UINT8,
        0x28 as ::core::ffi::c_int as UINT8,
        0x29 as ::core::ffi::c_int as UINT8,
        0x2a as ::core::ffi::c_int as UINT8,
        0x34 as ::core::ffi::c_int as UINT8,
        0x35 as ::core::ffi::c_int as UINT8,
        0x36 as ::core::ffi::c_int as UINT8,
        0x37 as ::core::ffi::c_int as UINT8,
        0x38 as ::core::ffi::c_int as UINT8,
        0x39 as ::core::ffi::c_int as UINT8,
        0x3a as ::core::ffi::c_int as UINT8,
        0x43 as ::core::ffi::c_int as UINT8,
        0x44 as ::core::ffi::c_int as UINT8,
        0x45 as ::core::ffi::c_int as UINT8,
        0x46 as ::core::ffi::c_int as UINT8,
        0x47 as ::core::ffi::c_int as UINT8,
        0x48 as ::core::ffi::c_int as UINT8,
        0x49 as ::core::ffi::c_int as UINT8,
        0x4a as ::core::ffi::c_int as UINT8,
        0x53 as ::core::ffi::c_int as UINT8,
        0x54 as ::core::ffi::c_int as UINT8,
        0x55 as ::core::ffi::c_int as UINT8,
        0x56 as ::core::ffi::c_int as UINT8,
        0x57 as ::core::ffi::c_int as UINT8,
        0x58 as ::core::ffi::c_int as UINT8,
        0x59 as ::core::ffi::c_int as UINT8,
        0x5a as ::core::ffi::c_int as UINT8,
        0x63 as ::core::ffi::c_int as UINT8,
        0x64 as ::core::ffi::c_int as UINT8,
        0x65 as ::core::ffi::c_int as UINT8,
        0x66 as ::core::ffi::c_int as UINT8,
        0x67 as ::core::ffi::c_int as UINT8,
        0x68 as ::core::ffi::c_int as UINT8,
        0x69 as ::core::ffi::c_int as UINT8,
        0x6a as ::core::ffi::c_int as UINT8,
        0x73 as ::core::ffi::c_int as UINT8,
        0x74 as ::core::ffi::c_int as UINT8,
        0x75 as ::core::ffi::c_int as UINT8,
        0x76 as ::core::ffi::c_int as UINT8,
        0x77 as ::core::ffi::c_int as UINT8,
        0x78 as ::core::ffi::c_int as UINT8,
        0x79 as ::core::ffi::c_int as UINT8,
        0x7a as ::core::ffi::c_int as UINT8,
        0x83 as ::core::ffi::c_int as UINT8,
        0x84 as ::core::ffi::c_int as UINT8,
        0x85 as ::core::ffi::c_int as UINT8,
        0x86 as ::core::ffi::c_int as UINT8,
        0x87 as ::core::ffi::c_int as UINT8,
        0x88 as ::core::ffi::c_int as UINT8,
        0x89 as ::core::ffi::c_int as UINT8,
        0x8a as ::core::ffi::c_int as UINT8,
        0x92 as ::core::ffi::c_int as UINT8,
        0x93 as ::core::ffi::c_int as UINT8,
        0x94 as ::core::ffi::c_int as UINT8,
        0x95 as ::core::ffi::c_int as UINT8,
        0x96 as ::core::ffi::c_int as UINT8,
        0x97 as ::core::ffi::c_int as UINT8,
        0x98 as ::core::ffi::c_int as UINT8,
        0x99 as ::core::ffi::c_int as UINT8,
        0x9a as ::core::ffi::c_int as UINT8,
        0xa2 as ::core::ffi::c_int as UINT8,
        0xa3 as ::core::ffi::c_int as UINT8,
        0xa4 as ::core::ffi::c_int as UINT8,
        0xa5 as ::core::ffi::c_int as UINT8,
        0xa6 as ::core::ffi::c_int as UINT8,
        0xa7 as ::core::ffi::c_int as UINT8,
        0xa8 as ::core::ffi::c_int as UINT8,
        0xa9 as ::core::ffi::c_int as UINT8,
        0xaa as ::core::ffi::c_int as UINT8,
        0xb2 as ::core::ffi::c_int as UINT8,
        0xb3 as ::core::ffi::c_int as UINT8,
        0xb4 as ::core::ffi::c_int as UINT8,
        0xb5 as ::core::ffi::c_int as UINT8,
        0xb6 as ::core::ffi::c_int as UINT8,
        0xb7 as ::core::ffi::c_int as UINT8,
        0xb8 as ::core::ffi::c_int as UINT8,
        0xb9 as ::core::ffi::c_int as UINT8,
        0xba as ::core::ffi::c_int as UINT8,
        0xc2 as ::core::ffi::c_int as UINT8,
        0xc3 as ::core::ffi::c_int as UINT8,
        0xc4 as ::core::ffi::c_int as UINT8,
        0xc5 as ::core::ffi::c_int as UINT8,
        0xc6 as ::core::ffi::c_int as UINT8,
        0xc7 as ::core::ffi::c_int as UINT8,
        0xc8 as ::core::ffi::c_int as UINT8,
        0xc9 as ::core::ffi::c_int as UINT8,
        0xca as ::core::ffi::c_int as UINT8,
        0xd2 as ::core::ffi::c_int as UINT8,
        0xd3 as ::core::ffi::c_int as UINT8,
        0xd4 as ::core::ffi::c_int as UINT8,
        0xd5 as ::core::ffi::c_int as UINT8,
        0xd6 as ::core::ffi::c_int as UINT8,
        0xd7 as ::core::ffi::c_int as UINT8,
        0xd8 as ::core::ffi::c_int as UINT8,
        0xd9 as ::core::ffi::c_int as UINT8,
        0xda as ::core::ffi::c_int as UINT8,
        0xe1 as ::core::ffi::c_int as UINT8,
        0xe2 as ::core::ffi::c_int as UINT8,
        0xe3 as ::core::ffi::c_int as UINT8,
        0xe4 as ::core::ffi::c_int as UINT8,
        0xe5 as ::core::ffi::c_int as UINT8,
        0xe6 as ::core::ffi::c_int as UINT8,
        0xe7 as ::core::ffi::c_int as UINT8,
        0xe8 as ::core::ffi::c_int as UINT8,
        0xe9 as ::core::ffi::c_int as UINT8,
        0xea as ::core::ffi::c_int as UINT8,
        0xf1 as ::core::ffi::c_int as UINT8,
        0xf2 as ::core::ffi::c_int as UINT8,
        0xf3 as ::core::ffi::c_int as UINT8,
        0xf4 as ::core::ffi::c_int as UINT8,
        0xf5 as ::core::ffi::c_int as UINT8,
        0xf6 as ::core::ffi::c_int as UINT8,
        0xf7 as ::core::ffi::c_int as UINT8,
        0xf8 as ::core::ffi::c_int as UINT8,
        0xf9 as ::core::ffi::c_int as UINT8,
        0xfa as ::core::ffi::c_int as UINT8,
    ];
    static mut bits_ac_chrominance: [UINT8; 17] = [
        0 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        2 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        2 as ::core::ffi::c_int as UINT8,
        4 as ::core::ffi::c_int as UINT8,
        4 as ::core::ffi::c_int as UINT8,
        3 as ::core::ffi::c_int as UINT8,
        4 as ::core::ffi::c_int as UINT8,
        7 as ::core::ffi::c_int as UINT8,
        5 as ::core::ffi::c_int as UINT8,
        4 as ::core::ffi::c_int as UINT8,
        4 as ::core::ffi::c_int as UINT8,
        0 as ::core::ffi::c_int as UINT8,
        1 as ::core::ffi::c_int as UINT8,
        2 as ::core::ffi::c_int as UINT8,
        0x77 as ::core::ffi::c_int as UINT8,
    ];
    static mut val_ac_chrominance: [UINT8; 162] = [
        0 as ::core::ffi::c_int as UINT8,
        0x1 as ::core::ffi::c_int as UINT8,
        0x2 as ::core::ffi::c_int as UINT8,
        0x3 as ::core::ffi::c_int as UINT8,
        0x11 as ::core::ffi::c_int as UINT8,
        0x4 as ::core::ffi::c_int as UINT8,
        0x5 as ::core::ffi::c_int as UINT8,
        0x21 as ::core::ffi::c_int as UINT8,
        0x31 as ::core::ffi::c_int as UINT8,
        0x6 as ::core::ffi::c_int as UINT8,
        0x12 as ::core::ffi::c_int as UINT8,
        0x41 as ::core::ffi::c_int as UINT8,
        0x51 as ::core::ffi::c_int as UINT8,
        0x7 as ::core::ffi::c_int as UINT8,
        0x61 as ::core::ffi::c_int as UINT8,
        0x71 as ::core::ffi::c_int as UINT8,
        0x13 as ::core::ffi::c_int as UINT8,
        0x22 as ::core::ffi::c_int as UINT8,
        0x32 as ::core::ffi::c_int as UINT8,
        0x81 as ::core::ffi::c_int as UINT8,
        0x8 as ::core::ffi::c_int as UINT8,
        0x14 as ::core::ffi::c_int as UINT8,
        0x42 as ::core::ffi::c_int as UINT8,
        0x91 as ::core::ffi::c_int as UINT8,
        0xa1 as ::core::ffi::c_int as UINT8,
        0xb1 as ::core::ffi::c_int as UINT8,
        0xc1 as ::core::ffi::c_int as UINT8,
        0x9 as ::core::ffi::c_int as UINT8,
        0x23 as ::core::ffi::c_int as UINT8,
        0x33 as ::core::ffi::c_int as UINT8,
        0x52 as ::core::ffi::c_int as UINT8,
        0xf0 as ::core::ffi::c_int as UINT8,
        0x15 as ::core::ffi::c_int as UINT8,
        0x62 as ::core::ffi::c_int as UINT8,
        0x72 as ::core::ffi::c_int as UINT8,
        0xd1 as ::core::ffi::c_int as UINT8,
        0xa as ::core::ffi::c_int as UINT8,
        0x16 as ::core::ffi::c_int as UINT8,
        0x24 as ::core::ffi::c_int as UINT8,
        0x34 as ::core::ffi::c_int as UINT8,
        0xe1 as ::core::ffi::c_int as UINT8,
        0x25 as ::core::ffi::c_int as UINT8,
        0xf1 as ::core::ffi::c_int as UINT8,
        0x17 as ::core::ffi::c_int as UINT8,
        0x18 as ::core::ffi::c_int as UINT8,
        0x19 as ::core::ffi::c_int as UINT8,
        0x1a as ::core::ffi::c_int as UINT8,
        0x26 as ::core::ffi::c_int as UINT8,
        0x27 as ::core::ffi::c_int as UINT8,
        0x28 as ::core::ffi::c_int as UINT8,
        0x29 as ::core::ffi::c_int as UINT8,
        0x2a as ::core::ffi::c_int as UINT8,
        0x35 as ::core::ffi::c_int as UINT8,
        0x36 as ::core::ffi::c_int as UINT8,
        0x37 as ::core::ffi::c_int as UINT8,
        0x38 as ::core::ffi::c_int as UINT8,
        0x39 as ::core::ffi::c_int as UINT8,
        0x3a as ::core::ffi::c_int as UINT8,
        0x43 as ::core::ffi::c_int as UINT8,
        0x44 as ::core::ffi::c_int as UINT8,
        0x45 as ::core::ffi::c_int as UINT8,
        0x46 as ::core::ffi::c_int as UINT8,
        0x47 as ::core::ffi::c_int as UINT8,
        0x48 as ::core::ffi::c_int as UINT8,
        0x49 as ::core::ffi::c_int as UINT8,
        0x4a as ::core::ffi::c_int as UINT8,
        0x53 as ::core::ffi::c_int as UINT8,
        0x54 as ::core::ffi::c_int as UINT8,
        0x55 as ::core::ffi::c_int as UINT8,
        0x56 as ::core::ffi::c_int as UINT8,
        0x57 as ::core::ffi::c_int as UINT8,
        0x58 as ::core::ffi::c_int as UINT8,
        0x59 as ::core::ffi::c_int as UINT8,
        0x5a as ::core::ffi::c_int as UINT8,
        0x63 as ::core::ffi::c_int as UINT8,
        0x64 as ::core::ffi::c_int as UINT8,
        0x65 as ::core::ffi::c_int as UINT8,
        0x66 as ::core::ffi::c_int as UINT8,
        0x67 as ::core::ffi::c_int as UINT8,
        0x68 as ::core::ffi::c_int as UINT8,
        0x69 as ::core::ffi::c_int as UINT8,
        0x6a as ::core::ffi::c_int as UINT8,
        0x73 as ::core::ffi::c_int as UINT8,
        0x74 as ::core::ffi::c_int as UINT8,
        0x75 as ::core::ffi::c_int as UINT8,
        0x76 as ::core::ffi::c_int as UINT8,
        0x77 as ::core::ffi::c_int as UINT8,
        0x78 as ::core::ffi::c_int as UINT8,
        0x79 as ::core::ffi::c_int as UINT8,
        0x7a as ::core::ffi::c_int as UINT8,
        0x82 as ::core::ffi::c_int as UINT8,
        0x83 as ::core::ffi::c_int as UINT8,
        0x84 as ::core::ffi::c_int as UINT8,
        0x85 as ::core::ffi::c_int as UINT8,
        0x86 as ::core::ffi::c_int as UINT8,
        0x87 as ::core::ffi::c_int as UINT8,
        0x88 as ::core::ffi::c_int as UINT8,
        0x89 as ::core::ffi::c_int as UINT8,
        0x8a as ::core::ffi::c_int as UINT8,
        0x92 as ::core::ffi::c_int as UINT8,
        0x93 as ::core::ffi::c_int as UINT8,
        0x94 as ::core::ffi::c_int as UINT8,
        0x95 as ::core::ffi::c_int as UINT8,
        0x96 as ::core::ffi::c_int as UINT8,
        0x97 as ::core::ffi::c_int as UINT8,
        0x98 as ::core::ffi::c_int as UINT8,
        0x99 as ::core::ffi::c_int as UINT8,
        0x9a as ::core::ffi::c_int as UINT8,
        0xa2 as ::core::ffi::c_int as UINT8,
        0xa3 as ::core::ffi::c_int as UINT8,
        0xa4 as ::core::ffi::c_int as UINT8,
        0xa5 as ::core::ffi::c_int as UINT8,
        0xa6 as ::core::ffi::c_int as UINT8,
        0xa7 as ::core::ffi::c_int as UINT8,
        0xa8 as ::core::ffi::c_int as UINT8,
        0xa9 as ::core::ffi::c_int as UINT8,
        0xaa as ::core::ffi::c_int as UINT8,
        0xb2 as ::core::ffi::c_int as UINT8,
        0xb3 as ::core::ffi::c_int as UINT8,
        0xb4 as ::core::ffi::c_int as UINT8,
        0xb5 as ::core::ffi::c_int as UINT8,
        0xb6 as ::core::ffi::c_int as UINT8,
        0xb7 as ::core::ffi::c_int as UINT8,
        0xb8 as ::core::ffi::c_int as UINT8,
        0xb9 as ::core::ffi::c_int as UINT8,
        0xba as ::core::ffi::c_int as UINT8,
        0xc2 as ::core::ffi::c_int as UINT8,
        0xc3 as ::core::ffi::c_int as UINT8,
        0xc4 as ::core::ffi::c_int as UINT8,
        0xc5 as ::core::ffi::c_int as UINT8,
        0xc6 as ::core::ffi::c_int as UINT8,
        0xc7 as ::core::ffi::c_int as UINT8,
        0xc8 as ::core::ffi::c_int as UINT8,
        0xc9 as ::core::ffi::c_int as UINT8,
        0xca as ::core::ffi::c_int as UINT8,
        0xd2 as ::core::ffi::c_int as UINT8,
        0xd3 as ::core::ffi::c_int as UINT8,
        0xd4 as ::core::ffi::c_int as UINT8,
        0xd5 as ::core::ffi::c_int as UINT8,
        0xd6 as ::core::ffi::c_int as UINT8,
        0xd7 as ::core::ffi::c_int as UINT8,
        0xd8 as ::core::ffi::c_int as UINT8,
        0xd9 as ::core::ffi::c_int as UINT8,
        0xda as ::core::ffi::c_int as UINT8,
        0xe2 as ::core::ffi::c_int as UINT8,
        0xe3 as ::core::ffi::c_int as UINT8,
        0xe4 as ::core::ffi::c_int as UINT8,
        0xe5 as ::core::ffi::c_int as UINT8,
        0xe6 as ::core::ffi::c_int as UINT8,
        0xe7 as ::core::ffi::c_int as UINT8,
        0xe8 as ::core::ffi::c_int as UINT8,
        0xe9 as ::core::ffi::c_int as UINT8,
        0xea as ::core::ffi::c_int as UINT8,
        0xf2 as ::core::ffi::c_int as UINT8,
        0xf3 as ::core::ffi::c_int as UINT8,
        0xf4 as ::core::ffi::c_int as UINT8,
        0xf5 as ::core::ffi::c_int as UINT8,
        0xf6 as ::core::ffi::c_int as UINT8,
        0xf7 as ::core::ffi::c_int as UINT8,
        0xf8 as ::core::ffi::c_int as UINT8,
        0xf9 as ::core::ffi::c_int as UINT8,
        0xfa as ::core::ffi::c_int as UINT8,
    ];
    if (*cinfo).is_decompressor != 0 {
        dc_huff_tbl_ptrs =
            ::core::ptr::addr_of_mut!((*(cinfo as j_decompress_ptr)).dc_huff_tbl_ptrs) as *mut *mut JHUFF_TBL;
        ac_huff_tbl_ptrs =
            ::core::ptr::addr_of_mut!((*(cinfo as j_decompress_ptr)).ac_huff_tbl_ptrs) as *mut *mut JHUFF_TBL;
    } else {
        dc_huff_tbl_ptrs =
            ::core::ptr::addr_of_mut!((*(cinfo as j_compress_ptr)).dc_huff_tbl_ptrs) as *mut *mut JHUFF_TBL;
        ac_huff_tbl_ptrs =
            ::core::ptr::addr_of_mut!((*(cinfo as j_compress_ptr)).ac_huff_tbl_ptrs) as *mut *mut JHUFF_TBL;
    }
    add_huff_table(
        cinfo,
        dc_huff_tbl_ptrs.offset(0 as ::core::ffi::c_int as isize) as *mut *mut JHUFF_TBL,
        ::core::ptr::addr_of!(bits_dc_luminance) as *const UINT8,
        ::core::ptr::addr_of!(val_dc_luminance) as *const UINT8,
    );
    add_huff_table(
        cinfo,
        ac_huff_tbl_ptrs.offset(0 as ::core::ffi::c_int as isize) as *mut *mut JHUFF_TBL,
        ::core::ptr::addr_of!(bits_ac_luminance) as *const UINT8,
        ::core::ptr::addr_of!(val_ac_luminance) as *const UINT8,
    );
    add_huff_table(
        cinfo,
        dc_huff_tbl_ptrs.offset(1 as ::core::ffi::c_int as isize) as *mut *mut JHUFF_TBL,
        ::core::ptr::addr_of!(bits_dc_chrominance) as *const UINT8,
        ::core::ptr::addr_of!(val_dc_chrominance) as *const UINT8,
    );
    add_huff_table(
        cinfo,
        ac_huff_tbl_ptrs.offset(1 as ::core::ffi::c_int as isize) as *mut *mut JHUFF_TBL,
        ::core::ptr::addr_of!(bits_ac_chrominance) as *const UINT8,
        ::core::ptr::addr_of!(val_ac_chrominance) as *const UINT8,
    );
}
unsafe extern "C" fn start_pass_huff_decoder(mut cinfo: j_decompress_ptr) {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut ci: ::core::ffi::c_int = 0;
    let mut blkn: ::core::ffi::c_int = 0;
    let mut dctbl: ::core::ffi::c_int = 0;
    let mut actbl: ::core::ffi::c_int = 0;
    let mut pdtbl: *mut *mut d_derived_tbl = ::core::ptr::null_mut::<*mut d_derived_tbl>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    if (*cinfo).Ss != 0 as ::core::ffi::c_int
        || (*cinfo).Se != DCTSIZE2 - 1 as ::core::ffi::c_int
        || (*cinfo).Ah != 0 as ::core::ffi::c_int
        || (*cinfo).Al != 0 as ::core::ffi::c_int
    {
        (*(*cinfo).err).msg_code = JWRN_NOT_SEQUENTIAL as ::core::ffi::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr, -(1 as ::core::ffi::c_int)
        );
    }
    ci = 0 as ::core::ffi::c_int;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        dctbl = (*compptr).dc_tbl_no;
        actbl = (*compptr).ac_tbl_no;
        pdtbl =
            (::core::ptr::addr_of_mut!((*entropy).dc_derived_tbls) as *mut *mut d_derived_tbl).offset(dctbl as isize);
        jpeg_make_d_derived_tbl(cinfo, TRUE, dctbl, pdtbl);
        pdtbl =
            (::core::ptr::addr_of_mut!((*entropy).ac_derived_tbls) as *mut *mut d_derived_tbl).offset(actbl as isize);
        jpeg_make_d_derived_tbl(cinfo, FALSE, actbl, pdtbl);
        (*entropy).saved.last_dc_val[ci as usize] = 0 as ::core::ffi::c_int;
        ci += 1;
    }
    blkn = 0 as ::core::ffi::c_int;
    while blkn < (*cinfo).blocks_in_MCU {
        ci = (*cinfo).MCU_membership[blkn as usize];
        compptr = (*cinfo).cur_comp_info[ci as usize];
        (*entropy).dc_cur_tbls[blkn as usize] =
            (*entropy).dc_derived_tbls[(*compptr).dc_tbl_no as usize];
        (*entropy).ac_cur_tbls[blkn as usize] =
            (*entropy).ac_derived_tbls[(*compptr).ac_tbl_no as usize];
        if (*compptr).component_needed != 0 {
            (*entropy).dc_needed[blkn as usize] = TRUE as boolean;
            (*entropy).ac_needed[blkn as usize] = ((*compptr).DCT_h_scaled_size
                > 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int as boolean;
        } else {
            (*entropy).ac_needed[blkn as usize] = FALSE as boolean;
            (*entropy).dc_needed[blkn as usize] = (*entropy).ac_needed[blkn as usize];
        }
        blkn += 1;
    }
    (*entropy).bitstate.bits_left = 0 as ::core::ffi::c_int;
    (*entropy).bitstate.get_buffer = 0 as bit_buf_type;
    (*entropy).pub_0.insufficient_data = FALSE as boolean;
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
}
pub unsafe extern "C" fn jpeg_make_d_derived_tbl(
    mut cinfo: j_decompress_ptr,
    mut isDC: boolean,
    mut tblno: ::core::ffi::c_int,
    mut pdtbl: *mut *mut d_derived_tbl,
) {
    let mut htbl: *mut JHUFF_TBL = ::core::ptr::null_mut::<JHUFF_TBL>();
    let mut dtbl: *mut d_derived_tbl = ::core::ptr::null_mut::<d_derived_tbl>();
    let mut p: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut si: ::core::ffi::c_int = 0;
    let mut numsymbols: ::core::ffi::c_int = 0;
    let mut lookbits: ::core::ffi::c_int = 0;
    let mut ctr: ::core::ffi::c_int = 0;
    let mut huffsize: [::core::ffi::c_char; 257] = [0; 257];
    let mut huffcode: [::core::ffi::c_uint; 257] = [0; 257];
    let mut code: ::core::ffi::c_uint = 0;
    if tblno < 0 as ::core::ffi::c_int || tblno >= NUM_HUFF_TBLS {
        (*(*cinfo).err).msg_code = JERR_NO_HUFF_TABLE as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = tblno;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    htbl = if isDC != 0 {
        (*cinfo).dc_huff_tbl_ptrs[tblno as usize]
    } else {
        (*cinfo).ac_huff_tbl_ptrs[tblno as usize]
    };
    if htbl.is_null() {
        (*(*cinfo).err).msg_code = JERR_NO_HUFF_TABLE as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = tblno;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*pdtbl).is_null() {
        *pdtbl = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            ::core::mem::size_of::<d_derived_tbl>() as size_t,
        ) as *mut d_derived_tbl;
    }
    dtbl = *pdtbl;
    (*dtbl).pub_0 = htbl;
    p = 0 as ::core::ffi::c_int;
    l = 1 as ::core::ffi::c_int;
    while l <= 16 as ::core::ffi::c_int {
        i = (*htbl).bits[l as usize] as ::core::ffi::c_int;
        if i < 0 as ::core::ffi::c_int || p + i > 256 as ::core::ffi::c_int {
            (*(*cinfo).err).msg_code = JERR_BAD_HUFF_TABLE as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        loop {
            let fresh38 = i;
            i = i - 1;
            if !(fresh38 != 0) {
                break;
            }
            let fresh39 = p;
            p = p + 1;
            huffsize[fresh39 as usize] = l as ::core::ffi::c_char;
        }
        l += 1;
    }
    huffsize[p as usize] = 0 as ::core::ffi::c_char;
    numsymbols = p;
    code = 0 as ::core::ffi::c_uint;
    si = huffsize[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    p = 0 as ::core::ffi::c_int;
    while huffsize[p as usize] != 0 {
        while huffsize[p as usize] as ::core::ffi::c_int == si {
            let fresh40 = p;
            p = p + 1;
            huffcode[fresh40 as usize] = code;
            code = code.wrapping_add(1);
        }
        if code as JLONG >= (1 as ::core::ffi::c_int as JLONG) << si {
            (*(*cinfo).err).msg_code = JERR_BAD_HUFF_TABLE as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        code <<= 1 as ::core::ffi::c_int;
        si += 1;
    }
    p = 0 as ::core::ffi::c_int;
    l = 1 as ::core::ffi::c_int;
    while l <= 16 as ::core::ffi::c_int {
        if (*htbl).bits[l as usize] != 0 {
            (*dtbl).valoffset[l as usize] = p as JLONG - huffcode[p as usize] as JLONG;
            p += (*htbl).bits[l as usize] as ::core::ffi::c_int;
            (*dtbl).maxcode[l as usize] = huffcode[(p - 1 as ::core::ffi::c_int) as usize] as JLONG;
        } else {
            (*dtbl).maxcode[l as usize] = -(1 as ::core::ffi::c_int) as JLONG;
        }
        l += 1;
    }
    (*dtbl).valoffset[17 as ::core::ffi::c_int as usize] = 0 as JLONG;
    (*dtbl).maxcode[17 as ::core::ffi::c_int as usize] = 0xfffff as ::core::ffi::c_long as JLONG;
    i = 0 as ::core::ffi::c_int;
    while i < (1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD {
        (*dtbl).lookup[i as usize] = (HUFF_LOOKAHEAD + 1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD;
        i += 1;
    }
    p = 0 as ::core::ffi::c_int;
    l = 1 as ::core::ffi::c_int;
    while l <= HUFF_LOOKAHEAD {
        i = 1 as ::core::ffi::c_int;
        while i <= (*htbl).bits[l as usize] as ::core::ffi::c_int {
            lookbits = (huffcode[p as usize] << HUFF_LOOKAHEAD - l) as ::core::ffi::c_int;
            ctr = (1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD - l;
            while ctr > 0 as ::core::ffi::c_int {
                (*dtbl).lookup[lookbits as usize] =
                    l << HUFF_LOOKAHEAD | (*htbl).huffval[p as usize] as ::core::ffi::c_int;
                lookbits += 1;
                ctr -= 1;
            }
            i += 1;
            p += 1;
        }
        l += 1;
    }
    if isDC != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < numsymbols {
            let mut sym: ::core::ffi::c_int = (*htbl).huffval[i as usize] as ::core::ffi::c_int;
            if sym < 0 as ::core::ffi::c_int || sym > 15 as ::core::ffi::c_int {
                (*(*cinfo).err).msg_code = JERR_BAD_HUFF_TABLE as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            i += 1;
        }
    }
}
pub const MIN_GET_BITS: ::core::ffi::c_int = BIT_BUF_SIZE - 7 as ::core::ffi::c_int;
pub unsafe extern "C" fn jpeg_fill_bit_buffer(
    mut state: *mut bitread_working_state,
    mut get_buffer: bit_buf_type,
    mut bits_left: ::core::ffi::c_int,
    mut nbits: ::core::ffi::c_int,
) -> boolean {
    let mut next_input_byte: *const JOCTET = (*state).next_input_byte;
    let mut bytes_in_buffer: size_t = (*state).bytes_in_buffer;
    let mut cinfo: j_decompress_ptr = (*state).cinfo;
    let mut current_block_30: u64;
    if (*cinfo).unread_marker == 0 as ::core::ffi::c_int {
        loop {
            if !(bits_left < MIN_GET_BITS) {
                current_block_30 = 11459959175219260272;
                break;
            }
            let mut c: ::core::ffi::c_int = 0;
            if bytes_in_buffer == 0 as size_t {
                if Some(
                    (*(*cinfo).src)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return FALSE;
                }
                next_input_byte = (*(*cinfo).src).next_input_byte;
                bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh0 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            c = *fresh0 as ::core::ffi::c_int;
            if c == 0xff as ::core::ffi::c_int {
                loop {
                    if bytes_in_buffer == 0 as size_t {
                        if Some(
                            (*(*cinfo).src)
                                .fill_input_buffer
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(cinfo)
                            == 0
                        {
                            return FALSE;
                        }
                        next_input_byte = (*(*cinfo).src).next_input_byte;
                        bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
                    }
                    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                    let fresh1 = next_input_byte;
                    next_input_byte = next_input_byte.offset(1);
                    c = *fresh1 as ::core::ffi::c_int;
                    if !(c == 0xff as ::core::ffi::c_int) {
                        break;
                    }
                }
                if c == 0 as ::core::ffi::c_int {
                    c = 0xff as ::core::ffi::c_int;
                } else {
                    (*cinfo).unread_marker = c;
                    current_block_30 = 7526322600627785187;
                    break;
                }
            }
            get_buffer = get_buffer << 8 as ::core::ffi::c_int | c as bit_buf_type;
            bits_left += 8 as ::core::ffi::c_int;
        }
    } else {
        current_block_30 = 7526322600627785187;
    }
    match current_block_30 {
        7526322600627785187 => {
            if nbits > bits_left {
                if (*(*cinfo).entropy).insufficient_data == 0 {
                    (*(*cinfo).err).msg_code = JWRN_HIT_MARKER as ::core::ffi::c_int;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        -(1 as ::core::ffi::c_int),
                    );
                    (*(*cinfo).entropy).insufficient_data = TRUE as boolean;
                }
                get_buffer <<= MIN_GET_BITS - bits_left;
                bits_left = MIN_GET_BITS;
            }
        }
        _ => {}
    }
    (*state).next_input_byte = next_input_byte;
    (*state).bytes_in_buffer = bytes_in_buffer;
    (*state).get_buffer = get_buffer;
    (*state).bits_left = bits_left;
    return TRUE;
}
pub unsafe extern "C" fn jpeg_huff_decode(
    mut state: *mut bitread_working_state,
    mut get_buffer: bit_buf_type,
    mut bits_left: ::core::ffi::c_int,
    mut htbl: *mut d_derived_tbl,
    mut min_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut l: ::core::ffi::c_int = min_bits;
    let mut code: JLONG = 0;
    if bits_left < l {
        if jpeg_fill_bit_buffer(state, get_buffer, bits_left, l) == 0 {
            return -(1 as ::core::ffi::c_int);
        }
        get_buffer = (*state).get_buffer;
        bits_left = (*state).bits_left;
    }
    bits_left -= l;
    code = ((get_buffer >> bits_left) as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << l) - 1 as ::core::ffi::c_int) as JLONG;
    while code > (*htbl).maxcode[l as usize] {
        code <<= 1 as ::core::ffi::c_int;
        if bits_left < 1 as ::core::ffi::c_int {
            if jpeg_fill_bit_buffer(state, get_buffer, bits_left, 1 as ::core::ffi::c_int) == 0 {
                return -(1 as ::core::ffi::c_int);
            }
            get_buffer = (*state).get_buffer;
            bits_left = (*state).bits_left;
        }
        bits_left -= 1 as ::core::ffi::c_int;
        code |= ((get_buffer >> bits_left) as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
            as JLONG;
        l += 1;
    }
    (*state).get_buffer = get_buffer;
    (*state).bits_left = bits_left;
    if l > 16 as ::core::ffi::c_int {
        (*(*(*state).cinfo).err).msg_code = JWRN_HUFF_BAD_CODE as ::core::ffi::c_int;
        Some(
            (*(*(*state).cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*state).cinfo as j_common_ptr,
            -(1 as ::core::ffi::c_int),
        );
        return 0 as ::core::ffi::c_int;
    }
    return (*(*htbl).pub_0).huffval
        [(code + (*htbl).valoffset[l as usize]) as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int;
}
pub const NEG_1: ::core::ffi::c_uint = -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
unsafe extern "C" fn process_restart(mut cinfo: j_decompress_ptr) -> boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut ci: ::core::ffi::c_int = 0;
    (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker).discarded_bytes.wrapping_add(
        ((*entropy).bitstate.bits_left / 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    (*entropy).bitstate.bits_left = 0 as ::core::ffi::c_int;
    if Some(
        (*(*cinfo).marker)
            .read_restart_marker
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo)
        == 0
    {
        return FALSE;
    }
    ci = 0 as ::core::ffi::c_int;
    while ci < (*cinfo).comps_in_scan {
        (*entropy).saved.last_dc_val[ci as usize] = 0 as ::core::ffi::c_int;
        ci += 1;
    }
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    if (*cinfo).unread_marker == 0 as ::core::ffi::c_int {
        (*entropy).pub_0.insufficient_data = FALSE as boolean;
    }
    return TRUE;
}
unsafe extern "C" fn decode_mcu_slow(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: ::core::ffi::c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: ::core::ptr::null::<JOCTET>(),
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: ::core::ptr::null_mut::<jpeg_decompress_struct>(),
    };
    let mut blkn: ::core::ffi::c_int = 0;
    let mut state: savable_state = savable_state {
        last_dc_val: [0; 4],
    };
    br_state.cinfo = cinfo;
    br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
    br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
    get_buffer = (*entropy).bitstate.get_buffer;
    bits_left = (*entropy).bitstate.bits_left;
    state = (*entropy).saved;
    blkn = 0 as ::core::ffi::c_int;
    while blkn < (*cinfo).blocks_in_MCU {
        let mut block: JBLOCKROW = if !MCU_data.is_null() {
            *MCU_data.offset(blkn as isize)
        } else {
            ::core::ptr::null_mut::<JBLOCK>()
        };
        let mut dctbl: *mut d_derived_tbl = (*entropy).dc_cur_tbls[blkn as usize];
        let mut actbl: *mut d_derived_tbl = (*entropy).ac_cur_tbls[blkn as usize];
        let mut s: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut r: ::core::ffi::c_int = 0;
        let mut current_block_22: u64;
        let mut nb: ::core::ffi::c_int = 0;
        let mut look: ::core::ffi::c_int = 0;
        if bits_left < HUFF_LOOKAHEAD {
            if jpeg_fill_bit_buffer(
                ::core::ptr::addr_of_mut!(br_state),
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
                current_block_22 = 11532744695490468833;
            } else {
                current_block_22 = 15976848397966268834;
            }
        } else {
            current_block_22 = 15976848397966268834;
        }
        match current_block_22 {
            15976848397966268834 => {
                look = (get_buffer >> bits_left - 8 as ::core::ffi::c_int) as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int;
                nb = (*dctbl).lookup[look as usize] >> HUFF_LOOKAHEAD;
                if nb <= HUFF_LOOKAHEAD {
                    bits_left -= nb;
                    s = (*dctbl).lookup[look as usize]
                        & ((1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD) - 1 as ::core::ffi::c_int;
                    current_block_22 = 15768484401365413375;
                } else {
                    current_block_22 = 11532744695490468833;
                }
            }
            _ => {}
        }
        match current_block_22 {
            11532744695490468833 => {
                s = jpeg_huff_decode(::core::ptr::addr_of_mut!(br_state), get_buffer, bits_left, dctbl, nb);
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
                if jpeg_fill_bit_buffer(::core::ptr::addr_of_mut!(br_state), get_buffer, bits_left, s) == 0 {
                    return 0 as boolean;
                }
                get_buffer = br_state.get_buffer;
                bits_left = br_state.bits_left;
            }
            bits_left -= s;
            r = (get_buffer >> bits_left) as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << s) - 1 as ::core::ffi::c_int;
            s = (r as ::core::ffi::c_uint).wrapping_add(
                (r - ((1 as ::core::ffi::c_int) << s - 1 as ::core::ffi::c_int)
                    >> 31 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    & ((-(1 as ::core::ffi::c_int) as ::core::ffi::c_uint) << s)
                        .wrapping_add(1 as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int;
        }
        if (*entropy).dc_needed[blkn as usize] != 0 {
            let mut ci: ::core::ffi::c_int = (*cinfo).MCU_membership[blkn as usize];
            s += state.last_dc_val[ci as usize];
            state.last_dc_val[ci as usize] = s;
            if !block.is_null() {
                (*block)[0 as ::core::ffi::c_int as usize] = s as JCOEF;
            }
        }
        if (*entropy).ac_needed[blkn as usize] != 0 && !block.is_null() {
            k = 1 as ::core::ffi::c_int;
            while k < DCTSIZE2 {
                let mut current_block_60: u64;
                let mut nb_0: ::core::ffi::c_int = 0;
                let mut look_0: ::core::ffi::c_int = 0;
                if bits_left < HUFF_LOOKAHEAD {
                    if jpeg_fill_bit_buffer(
                        ::core::ptr::addr_of_mut!(br_state),
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
                        nb_0 = 1 as ::core::ffi::c_int;
                        current_block_60 = 5481584536774418473;
                    } else {
                        current_block_60 = 5141539773904409130;
                    }
                } else {
                    current_block_60 = 5141539773904409130;
                }
                match current_block_60 {
                    5141539773904409130 => {
                        look_0 = (get_buffer >> bits_left - 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int;
                        nb_0 = (*actbl).lookup[look_0 as usize] >> HUFF_LOOKAHEAD;
                        if nb_0 <= HUFF_LOOKAHEAD {
                            bits_left -= nb_0;
                            s = (*actbl).lookup[look_0 as usize]
                                & ((1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD)
                                    - 1 as ::core::ffi::c_int;
                            current_block_60 = 168769493162332264;
                        } else {
                            current_block_60 = 5481584536774418473;
                        }
                    }
                    _ => {}
                }
                match current_block_60 {
                    5481584536774418473 => {
                        s = jpeg_huff_decode(::core::ptr::addr_of_mut!(br_state), get_buffer, bits_left, actbl, nb_0);
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
                        if jpeg_fill_bit_buffer(::core::ptr::addr_of_mut!(br_state), get_buffer, bits_left, s) == 0 {
                            return 0 as boolean;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left;
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << s) - 1 as ::core::ffi::c_int;
                    s = (r as ::core::ffi::c_uint).wrapping_add(
                        (r - ((1 as ::core::ffi::c_int) << s - 1 as ::core::ffi::c_int)
                            >> 31 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            & ((-(1 as ::core::ffi::c_int) as ::core::ffi::c_uint) << s)
                                .wrapping_add(1 as ::core::ffi::c_uint),
                    ) as ::core::ffi::c_int;
                    (*block)[*(::core::ptr::addr_of!(jpeg_natural_order) as *const ::core::ffi::c_int)
                        .offset(k as isize) as usize] = s as JCOEF;
                } else {
                    if r != 15 as ::core::ffi::c_int {
                        break;
                    }
                    k += 15 as ::core::ffi::c_int;
                }
                k += 1;
            }
        } else {
            k = 1 as ::core::ffi::c_int;
            while k < DCTSIZE2 {
                let mut current_block_97: u64;
                let mut nb_1: ::core::ffi::c_int = 0;
                let mut look_1: ::core::ffi::c_int = 0;
                if bits_left < HUFF_LOOKAHEAD {
                    if jpeg_fill_bit_buffer(
                        ::core::ptr::addr_of_mut!(br_state),
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
                        nb_1 = 1 as ::core::ffi::c_int;
                        current_block_97 = 2861742418894916635;
                    } else {
                        current_block_97 = 15855550149339537395;
                    }
                } else {
                    current_block_97 = 15855550149339537395;
                }
                match current_block_97 {
                    15855550149339537395 => {
                        look_1 = (get_buffer >> bits_left - 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int;
                        nb_1 = (*actbl).lookup[look_1 as usize] >> HUFF_LOOKAHEAD;
                        if nb_1 <= HUFF_LOOKAHEAD {
                            bits_left -= nb_1;
                            s = (*actbl).lookup[look_1 as usize]
                                & ((1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD)
                                    - 1 as ::core::ffi::c_int;
                            current_block_97 = 17769492591016358583;
                        } else {
                            current_block_97 = 2861742418894916635;
                        }
                    }
                    _ => {}
                }
                match current_block_97 {
                    2861742418894916635 => {
                        s = jpeg_huff_decode(::core::ptr::addr_of_mut!(br_state), get_buffer, bits_left, actbl, nb_1);
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
                        if jpeg_fill_bit_buffer(::core::ptr::addr_of_mut!(br_state), get_buffer, bits_left, s) == 0 {
                            return 0 as boolean;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left;
                    }
                    bits_left -= s;
                } else {
                    if r != 15 as ::core::ffi::c_int {
                        break;
                    }
                    k += 15 as ::core::ffi::c_int;
                }
                k += 1;
            }
        }
        blkn += 1;
    }
    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
    (*entropy).bitstate.get_buffer = get_buffer;
    (*entropy).bitstate.bits_left = bits_left;
    (*entropy).saved = state;
    return TRUE;
}
unsafe extern "C" fn decode_mcu_fast(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: ::core::ffi::c_int = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: ::core::ptr::null::<JOCTET>(),
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: ::core::ptr::null_mut::<jpeg_decompress_struct>(),
    };
    let mut buffer: *mut JOCTET = ::core::ptr::null_mut::<JOCTET>();
    let mut blkn: ::core::ffi::c_int = 0;
    let mut state: savable_state = savable_state {
        last_dc_val: [0; 4],
    };
    br_state.cinfo = cinfo;
    br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
    br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
    get_buffer = (*entropy).bitstate.get_buffer;
    bits_left = (*entropy).bitstate.bits_left;
    buffer = br_state.next_input_byte as *mut JOCTET;
    state = (*entropy).saved;
    blkn = 0 as ::core::ffi::c_int;
    while blkn < (*cinfo).blocks_in_MCU {
        let mut block: JBLOCKROW = if !MCU_data.is_null() {
            *MCU_data.offset(blkn as isize)
        } else {
            ::core::ptr::null_mut::<JBLOCK>()
        };
        let mut dctbl: *mut d_derived_tbl = (*entropy).dc_cur_tbls[blkn as usize];
        let mut actbl: *mut d_derived_tbl = (*entropy).ac_cur_tbls[blkn as usize];
        let mut s: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut r: ::core::ffi::c_int = 0;
        let mut l: ::core::ffi::c_int = 0;
        if bits_left <= 16 as ::core::ffi::c_int {
            let mut c0: ::core::ffi::c_int = 0;
            let mut c1: ::core::ffi::c_int = 0;
            let fresh2 = buffer;
            buffer = buffer.offset(1);
            c0 = *fresh2 as ::core::ffi::c_int;
            c1 = *buffer as ::core::ffi::c_int;
            get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0 as bit_buf_type;
            bits_left += 8 as ::core::ffi::c_int;
            if c0 == 0xff as ::core::ffi::c_int {
                buffer = buffer.offset(1);
                if c1 != 0 as ::core::ffi::c_int {
                    (*cinfo).unread_marker = c1;
                    buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                    get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                }
            }
            let mut c0_0: ::core::ffi::c_int = 0;
            let mut c1_0: ::core::ffi::c_int = 0;
            let fresh3 = buffer;
            buffer = buffer.offset(1);
            c0_0 = *fresh3 as ::core::ffi::c_int;
            c1_0 = *buffer as ::core::ffi::c_int;
            get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_0 as bit_buf_type;
            bits_left += 8 as ::core::ffi::c_int;
            if c0_0 == 0xff as ::core::ffi::c_int {
                buffer = buffer.offset(1);
                if c1_0 != 0 as ::core::ffi::c_int {
                    (*cinfo).unread_marker = c1_0;
                    buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                    get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                }
            }
            let mut c0_1: ::core::ffi::c_int = 0;
            let mut c1_1: ::core::ffi::c_int = 0;
            let fresh4 = buffer;
            buffer = buffer.offset(1);
            c0_1 = *fresh4 as ::core::ffi::c_int;
            c1_1 = *buffer as ::core::ffi::c_int;
            get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_1 as bit_buf_type;
            bits_left += 8 as ::core::ffi::c_int;
            if c0_1 == 0xff as ::core::ffi::c_int {
                buffer = buffer.offset(1);
                if c1_1 != 0 as ::core::ffi::c_int {
                    (*cinfo).unread_marker = c1_1;
                    buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                    get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                }
            }
            let mut c0_2: ::core::ffi::c_int = 0;
            let mut c1_2: ::core::ffi::c_int = 0;
            let fresh5 = buffer;
            buffer = buffer.offset(1);
            c0_2 = *fresh5 as ::core::ffi::c_int;
            c1_2 = *buffer as ::core::ffi::c_int;
            get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_2 as bit_buf_type;
            bits_left += 8 as ::core::ffi::c_int;
            if c0_2 == 0xff as ::core::ffi::c_int {
                buffer = buffer.offset(1);
                if c1_2 != 0 as ::core::ffi::c_int {
                    (*cinfo).unread_marker = c1_2;
                    buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                    get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                }
            }
            let mut c0_3: ::core::ffi::c_int = 0;
            let mut c1_3: ::core::ffi::c_int = 0;
            let fresh6 = buffer;
            buffer = buffer.offset(1);
            c0_3 = *fresh6 as ::core::ffi::c_int;
            c1_3 = *buffer as ::core::ffi::c_int;
            get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_3 as bit_buf_type;
            bits_left += 8 as ::core::ffi::c_int;
            if c0_3 == 0xff as ::core::ffi::c_int {
                buffer = buffer.offset(1);
                if c1_3 != 0 as ::core::ffi::c_int {
                    (*cinfo).unread_marker = c1_3;
                    buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                    get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                }
            }
            let mut c0_4: ::core::ffi::c_int = 0;
            let mut c1_4: ::core::ffi::c_int = 0;
            let fresh7 = buffer;
            buffer = buffer.offset(1);
            c0_4 = *fresh7 as ::core::ffi::c_int;
            c1_4 = *buffer as ::core::ffi::c_int;
            get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_4 as bit_buf_type;
            bits_left += 8 as ::core::ffi::c_int;
            if c0_4 == 0xff as ::core::ffi::c_int {
                buffer = buffer.offset(1);
                if c1_4 != 0 as ::core::ffi::c_int {
                    (*cinfo).unread_marker = c1_4;
                    buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                    get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                }
            }
        }
        s = (get_buffer >> bits_left - 8 as ::core::ffi::c_int) as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
        s = (*dctbl).lookup[s as usize];
        l = s >> HUFF_LOOKAHEAD;
        bits_left -= l;
        s = s & ((1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD) - 1 as ::core::ffi::c_int;
        if l > HUFF_LOOKAHEAD {
            s = (get_buffer >> bits_left
                & (((1 as ::core::ffi::c_int) << l) - 1 as ::core::ffi::c_int) as bit_buf_type)
                as ::core::ffi::c_int;
            while s as JLONG > (*dctbl).maxcode[l as usize] {
                s <<= 1 as ::core::ffi::c_int;
                bits_left -= 1 as ::core::ffi::c_int;
                s |= (get_buffer >> bits_left) as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int;
                l += 1;
            }
            if l > 16 as ::core::ffi::c_int {
                s = 0 as ::core::ffi::c_int;
            } else {
                s = (*(*dctbl).pub_0).huffval[((s as JLONG + (*dctbl).valoffset[l as usize])
                    as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    as usize] as ::core::ffi::c_int;
            }
        }
        if s != 0 {
            if bits_left <= 16 as ::core::ffi::c_int {
                let mut c0_5: ::core::ffi::c_int = 0;
                let mut c1_5: ::core::ffi::c_int = 0;
                let fresh8 = buffer;
                buffer = buffer.offset(1);
                c0_5 = *fresh8 as ::core::ffi::c_int;
                c1_5 = *buffer as ::core::ffi::c_int;
                get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_5 as bit_buf_type;
                bits_left += 8 as ::core::ffi::c_int;
                if c0_5 == 0xff as ::core::ffi::c_int {
                    buffer = buffer.offset(1);
                    if c1_5 != 0 as ::core::ffi::c_int {
                        (*cinfo).unread_marker = c1_5;
                        buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                        get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                    }
                }
                let mut c0_6: ::core::ffi::c_int = 0;
                let mut c1_6: ::core::ffi::c_int = 0;
                let fresh9 = buffer;
                buffer = buffer.offset(1);
                c0_6 = *fresh9 as ::core::ffi::c_int;
                c1_6 = *buffer as ::core::ffi::c_int;
                get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_6 as bit_buf_type;
                bits_left += 8 as ::core::ffi::c_int;
                if c0_6 == 0xff as ::core::ffi::c_int {
                    buffer = buffer.offset(1);
                    if c1_6 != 0 as ::core::ffi::c_int {
                        (*cinfo).unread_marker = c1_6;
                        buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                        get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                    }
                }
                let mut c0_7: ::core::ffi::c_int = 0;
                let mut c1_7: ::core::ffi::c_int = 0;
                let fresh10 = buffer;
                buffer = buffer.offset(1);
                c0_7 = *fresh10 as ::core::ffi::c_int;
                c1_7 = *buffer as ::core::ffi::c_int;
                get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_7 as bit_buf_type;
                bits_left += 8 as ::core::ffi::c_int;
                if c0_7 == 0xff as ::core::ffi::c_int {
                    buffer = buffer.offset(1);
                    if c1_7 != 0 as ::core::ffi::c_int {
                        (*cinfo).unread_marker = c1_7;
                        buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                        get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                    }
                }
                let mut c0_8: ::core::ffi::c_int = 0;
                let mut c1_8: ::core::ffi::c_int = 0;
                let fresh11 = buffer;
                buffer = buffer.offset(1);
                c0_8 = *fresh11 as ::core::ffi::c_int;
                c1_8 = *buffer as ::core::ffi::c_int;
                get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_8 as bit_buf_type;
                bits_left += 8 as ::core::ffi::c_int;
                if c0_8 == 0xff as ::core::ffi::c_int {
                    buffer = buffer.offset(1);
                    if c1_8 != 0 as ::core::ffi::c_int {
                        (*cinfo).unread_marker = c1_8;
                        buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                        get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                    }
                }
                let mut c0_9: ::core::ffi::c_int = 0;
                let mut c1_9: ::core::ffi::c_int = 0;
                let fresh12 = buffer;
                buffer = buffer.offset(1);
                c0_9 = *fresh12 as ::core::ffi::c_int;
                c1_9 = *buffer as ::core::ffi::c_int;
                get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_9 as bit_buf_type;
                bits_left += 8 as ::core::ffi::c_int;
                if c0_9 == 0xff as ::core::ffi::c_int {
                    buffer = buffer.offset(1);
                    if c1_9 != 0 as ::core::ffi::c_int {
                        (*cinfo).unread_marker = c1_9;
                        buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                        get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                    }
                }
                let mut c0_10: ::core::ffi::c_int = 0;
                let mut c1_10: ::core::ffi::c_int = 0;
                let fresh13 = buffer;
                buffer = buffer.offset(1);
                c0_10 = *fresh13 as ::core::ffi::c_int;
                c1_10 = *buffer as ::core::ffi::c_int;
                get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_10 as bit_buf_type;
                bits_left += 8 as ::core::ffi::c_int;
                if c0_10 == 0xff as ::core::ffi::c_int {
                    buffer = buffer.offset(1);
                    if c1_10 != 0 as ::core::ffi::c_int {
                        (*cinfo).unread_marker = c1_10;
                        buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                        get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                    }
                }
            }
            bits_left -= s;
            r = (get_buffer >> bits_left) as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << s) - 1 as ::core::ffi::c_int;
            s = (r as ::core::ffi::c_uint).wrapping_add(
                (r - ((1 as ::core::ffi::c_int) << s - 1 as ::core::ffi::c_int)
                    >> 31 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    & ((-(1 as ::core::ffi::c_int) as ::core::ffi::c_uint) << s)
                        .wrapping_add(1 as ::core::ffi::c_uint),
            ) as ::core::ffi::c_int;
        }
        if (*entropy).dc_needed[blkn as usize] != 0 {
            let mut ci: ::core::ffi::c_int = (*cinfo).MCU_membership[blkn as usize];
            s += state.last_dc_val[ci as usize];
            state.last_dc_val[ci as usize] = s;
            if !block.is_null() {
                (*block)[0 as ::core::ffi::c_int as usize] = s as JCOEF;
            }
        }
        if (*entropy).ac_needed[blkn as usize] != 0 && !block.is_null() {
            k = 1 as ::core::ffi::c_int;
            while k < DCTSIZE2 {
                if bits_left <= 16 as ::core::ffi::c_int {
                    let mut c0_11: ::core::ffi::c_int = 0;
                    let mut c1_11: ::core::ffi::c_int = 0;
                    let fresh14 = buffer;
                    buffer = buffer.offset(1);
                    c0_11 = *fresh14 as ::core::ffi::c_int;
                    c1_11 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_11 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_11 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_11 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_11;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                    let mut c0_12: ::core::ffi::c_int = 0;
                    let mut c1_12: ::core::ffi::c_int = 0;
                    let fresh15 = buffer;
                    buffer = buffer.offset(1);
                    c0_12 = *fresh15 as ::core::ffi::c_int;
                    c1_12 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_12 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_12 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_12 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_12;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                    let mut c0_13: ::core::ffi::c_int = 0;
                    let mut c1_13: ::core::ffi::c_int = 0;
                    let fresh16 = buffer;
                    buffer = buffer.offset(1);
                    c0_13 = *fresh16 as ::core::ffi::c_int;
                    c1_13 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_13 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_13 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_13 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_13;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                    let mut c0_14: ::core::ffi::c_int = 0;
                    let mut c1_14: ::core::ffi::c_int = 0;
                    let fresh17 = buffer;
                    buffer = buffer.offset(1);
                    c0_14 = *fresh17 as ::core::ffi::c_int;
                    c1_14 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_14 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_14 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_14 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_14;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                    let mut c0_15: ::core::ffi::c_int = 0;
                    let mut c1_15: ::core::ffi::c_int = 0;
                    let fresh18 = buffer;
                    buffer = buffer.offset(1);
                    c0_15 = *fresh18 as ::core::ffi::c_int;
                    c1_15 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_15 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_15 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_15 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_15;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                    let mut c0_16: ::core::ffi::c_int = 0;
                    let mut c1_16: ::core::ffi::c_int = 0;
                    let fresh19 = buffer;
                    buffer = buffer.offset(1);
                    c0_16 = *fresh19 as ::core::ffi::c_int;
                    c1_16 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_16 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_16 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_16 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_16;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                }
                s = (get_buffer >> bits_left - 8 as ::core::ffi::c_int) as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int;
                s = (*actbl).lookup[s as usize];
                l = s >> HUFF_LOOKAHEAD;
                bits_left -= l;
                s = s & ((1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD) - 1 as ::core::ffi::c_int;
                if l > HUFF_LOOKAHEAD {
                    s = (get_buffer >> bits_left
                        & (((1 as ::core::ffi::c_int) << l) - 1 as ::core::ffi::c_int)
                            as bit_buf_type) as ::core::ffi::c_int;
                    while s as JLONG > (*actbl).maxcode[l as usize] {
                        s <<= 1 as ::core::ffi::c_int;
                        bits_left -= 1 as ::core::ffi::c_int;
                        s |= (get_buffer >> bits_left) as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int;
                        l += 1;
                    }
                    if l > 16 as ::core::ffi::c_int {
                        s = 0 as ::core::ffi::c_int;
                    } else {
                        s = (*(*actbl).pub_0).huffval[((s as JLONG + (*actbl).valoffset[l as usize])
                            as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int;
                    }
                }
                r = s >> 4 as ::core::ffi::c_int;
                s &= 15 as ::core::ffi::c_int;
                if s != 0 {
                    k += r;
                    if bits_left <= 16 as ::core::ffi::c_int {
                        let mut c0_17: ::core::ffi::c_int = 0;
                        let mut c1_17: ::core::ffi::c_int = 0;
                        let fresh20 = buffer;
                        buffer = buffer.offset(1);
                        c0_17 = *fresh20 as ::core::ffi::c_int;
                        c1_17 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_17 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_17 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_17 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_17;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                        let mut c0_18: ::core::ffi::c_int = 0;
                        let mut c1_18: ::core::ffi::c_int = 0;
                        let fresh21 = buffer;
                        buffer = buffer.offset(1);
                        c0_18 = *fresh21 as ::core::ffi::c_int;
                        c1_18 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_18 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_18 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_18 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_18;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                        let mut c0_19: ::core::ffi::c_int = 0;
                        let mut c1_19: ::core::ffi::c_int = 0;
                        let fresh22 = buffer;
                        buffer = buffer.offset(1);
                        c0_19 = *fresh22 as ::core::ffi::c_int;
                        c1_19 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_19 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_19 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_19 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_19;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                        let mut c0_20: ::core::ffi::c_int = 0;
                        let mut c1_20: ::core::ffi::c_int = 0;
                        let fresh23 = buffer;
                        buffer = buffer.offset(1);
                        c0_20 = *fresh23 as ::core::ffi::c_int;
                        c1_20 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_20 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_20 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_20 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_20;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                        let mut c0_21: ::core::ffi::c_int = 0;
                        let mut c1_21: ::core::ffi::c_int = 0;
                        let fresh24 = buffer;
                        buffer = buffer.offset(1);
                        c0_21 = *fresh24 as ::core::ffi::c_int;
                        c1_21 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_21 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_21 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_21 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_21;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                        let mut c0_22: ::core::ffi::c_int = 0;
                        let mut c1_22: ::core::ffi::c_int = 0;
                        let fresh25 = buffer;
                        buffer = buffer.offset(1);
                        c0_22 = *fresh25 as ::core::ffi::c_int;
                        c1_22 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_22 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_22 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_22 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_22;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << s) - 1 as ::core::ffi::c_int;
                    s = (r as ::core::ffi::c_uint).wrapping_add(
                        (r - ((1 as ::core::ffi::c_int) << s - 1 as ::core::ffi::c_int)
                            >> 31 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            & ((-(1 as ::core::ffi::c_int) as ::core::ffi::c_uint) << s)
                                .wrapping_add(1 as ::core::ffi::c_uint),
                    ) as ::core::ffi::c_int;
                    (*block)[*(::core::ptr::addr_of!(jpeg_natural_order) as *const ::core::ffi::c_int)
                        .offset(k as isize) as usize] = s as JCOEF;
                } else {
                    if r != 15 as ::core::ffi::c_int {
                        break;
                    }
                    k += 15 as ::core::ffi::c_int;
                }
                k += 1;
            }
        } else {
            k = 1 as ::core::ffi::c_int;
            while k < DCTSIZE2 {
                if bits_left <= 16 as ::core::ffi::c_int {
                    let mut c0_23: ::core::ffi::c_int = 0;
                    let mut c1_23: ::core::ffi::c_int = 0;
                    let fresh26 = buffer;
                    buffer = buffer.offset(1);
                    c0_23 = *fresh26 as ::core::ffi::c_int;
                    c1_23 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_23 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_23 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_23 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_23;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                    let mut c0_24: ::core::ffi::c_int = 0;
                    let mut c1_24: ::core::ffi::c_int = 0;
                    let fresh27 = buffer;
                    buffer = buffer.offset(1);
                    c0_24 = *fresh27 as ::core::ffi::c_int;
                    c1_24 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_24 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_24 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_24 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_24;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                    let mut c0_25: ::core::ffi::c_int = 0;
                    let mut c1_25: ::core::ffi::c_int = 0;
                    let fresh28 = buffer;
                    buffer = buffer.offset(1);
                    c0_25 = *fresh28 as ::core::ffi::c_int;
                    c1_25 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_25 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_25 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_25 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_25;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                    let mut c0_26: ::core::ffi::c_int = 0;
                    let mut c1_26: ::core::ffi::c_int = 0;
                    let fresh29 = buffer;
                    buffer = buffer.offset(1);
                    c0_26 = *fresh29 as ::core::ffi::c_int;
                    c1_26 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_26 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_26 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_26 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_26;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                    let mut c0_27: ::core::ffi::c_int = 0;
                    let mut c1_27: ::core::ffi::c_int = 0;
                    let fresh30 = buffer;
                    buffer = buffer.offset(1);
                    c0_27 = *fresh30 as ::core::ffi::c_int;
                    c1_27 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_27 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_27 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_27 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_27;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                    let mut c0_28: ::core::ffi::c_int = 0;
                    let mut c1_28: ::core::ffi::c_int = 0;
                    let fresh31 = buffer;
                    buffer = buffer.offset(1);
                    c0_28 = *fresh31 as ::core::ffi::c_int;
                    c1_28 = *buffer as ::core::ffi::c_int;
                    get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_28 as bit_buf_type;
                    bits_left += 8 as ::core::ffi::c_int;
                    if c0_28 == 0xff as ::core::ffi::c_int {
                        buffer = buffer.offset(1);
                        if c1_28 != 0 as ::core::ffi::c_int {
                            (*cinfo).unread_marker = c1_28;
                            buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                            get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                        }
                    }
                }
                s = (get_buffer >> bits_left - 8 as ::core::ffi::c_int) as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int;
                s = (*actbl).lookup[s as usize];
                l = s >> HUFF_LOOKAHEAD;
                bits_left -= l;
                s = s & ((1 as ::core::ffi::c_int) << HUFF_LOOKAHEAD) - 1 as ::core::ffi::c_int;
                if l > HUFF_LOOKAHEAD {
                    s = (get_buffer >> bits_left
                        & (((1 as ::core::ffi::c_int) << l) - 1 as ::core::ffi::c_int)
                            as bit_buf_type) as ::core::ffi::c_int;
                    while s as JLONG > (*actbl).maxcode[l as usize] {
                        s <<= 1 as ::core::ffi::c_int;
                        bits_left -= 1 as ::core::ffi::c_int;
                        s |= (get_buffer >> bits_left) as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int;
                        l += 1;
                    }
                    if l > 16 as ::core::ffi::c_int {
                        s = 0 as ::core::ffi::c_int;
                    } else {
                        s = (*(*actbl).pub_0).huffval[((s as JLONG + (*actbl).valoffset[l as usize])
                            as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int;
                    }
                }
                r = s >> 4 as ::core::ffi::c_int;
                s &= 15 as ::core::ffi::c_int;
                if s != 0 {
                    k += r;
                    if bits_left <= 16 as ::core::ffi::c_int {
                        let mut c0_29: ::core::ffi::c_int = 0;
                        let mut c1_29: ::core::ffi::c_int = 0;
                        let fresh32 = buffer;
                        buffer = buffer.offset(1);
                        c0_29 = *fresh32 as ::core::ffi::c_int;
                        c1_29 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_29 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_29 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_29 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_29;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                        let mut c0_30: ::core::ffi::c_int = 0;
                        let mut c1_30: ::core::ffi::c_int = 0;
                        let fresh33 = buffer;
                        buffer = buffer.offset(1);
                        c0_30 = *fresh33 as ::core::ffi::c_int;
                        c1_30 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_30 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_30 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_30 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_30;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                        let mut c0_31: ::core::ffi::c_int = 0;
                        let mut c1_31: ::core::ffi::c_int = 0;
                        let fresh34 = buffer;
                        buffer = buffer.offset(1);
                        c0_31 = *fresh34 as ::core::ffi::c_int;
                        c1_31 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_31 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_31 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_31 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_31;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                        let mut c0_32: ::core::ffi::c_int = 0;
                        let mut c1_32: ::core::ffi::c_int = 0;
                        let fresh35 = buffer;
                        buffer = buffer.offset(1);
                        c0_32 = *fresh35 as ::core::ffi::c_int;
                        c1_32 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_32 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_32 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_32 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_32;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                        let mut c0_33: ::core::ffi::c_int = 0;
                        let mut c1_33: ::core::ffi::c_int = 0;
                        let fresh36 = buffer;
                        buffer = buffer.offset(1);
                        c0_33 = *fresh36 as ::core::ffi::c_int;
                        c1_33 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_33 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_33 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_33 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_33;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                        let mut c0_34: ::core::ffi::c_int = 0;
                        let mut c1_34: ::core::ffi::c_int = 0;
                        let fresh37 = buffer;
                        buffer = buffer.offset(1);
                        c0_34 = *fresh37 as ::core::ffi::c_int;
                        c1_34 = *buffer as ::core::ffi::c_int;
                        get_buffer = get_buffer << 8 as ::core::ffi::c_int | c0_34 as bit_buf_type;
                        bits_left += 8 as ::core::ffi::c_int;
                        if c0_34 == 0xff as ::core::ffi::c_int {
                            buffer = buffer.offset(1);
                            if c1_34 != 0 as ::core::ffi::c_int {
                                (*cinfo).unread_marker = c1_34;
                                buffer = buffer.offset(-(2 as ::core::ffi::c_int as isize));
                                get_buffer &= !(0xff as ::core::ffi::c_int) as bit_buf_type;
                            }
                        }
                    }
                    bits_left -= s;
                } else {
                    if r != 15 as ::core::ffi::c_int {
                        break;
                    }
                    k += 15 as ::core::ffi::c_int;
                }
                k += 1;
            }
        }
        blkn += 1;
    }
    if (*cinfo).unread_marker != 0 as ::core::ffi::c_int {
        (*cinfo).unread_marker = 0 as ::core::ffi::c_int;
        return FALSE;
    }
    br_state.bytes_in_buffer = br_state.bytes_in_buffer.wrapping_sub(
        buffer.offset_from(br_state.next_input_byte) as ::core::ffi::c_long as size_t,
    );
    br_state.next_input_byte = buffer;
    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
    (*entropy).bitstate.get_buffer = get_buffer;
    (*entropy).bitstate.bits_left = bits_left;
    (*entropy).saved = state;
    return TRUE;
}
pub const BUFSIZE: ::core::ffi::c_int = DCTSIZE2 * 8 as ::core::ffi::c_int;
unsafe extern "C" fn decode_mcu(
    mut cinfo: j_decompress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut usefast: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as ::core::ffi::c_uint {
            if process_restart(cinfo) == 0 {
                return FALSE;
            }
        }
        usefast = 0 as ::core::ffi::c_int;
    }
    if (*(*cinfo).src).bytes_in_buffer
        < (BUFSIZE as size_t).wrapping_mul((*cinfo).blocks_in_MCU as size_t)
        || (*cinfo).unread_marker != 0 as ::core::ffi::c_int
    {
        usefast = 0 as ::core::ffi::c_int;
    }
    if (*entropy).pub_0.insufficient_data == 0 {
        let mut current_block_9: u64;
        if usefast != 0 {
            if decode_mcu_fast(cinfo, MCU_data) == 0 {
                current_block_9 = 4973859941700582218;
            } else {
                current_block_9 = 1841672684692190573;
            }
        } else {
            current_block_9 = 4973859941700582218;
        }
        match current_block_9 {
            4973859941700582218 => {
                if decode_mcu_slow(cinfo, MCU_data) == 0 {
                    return FALSE;
                }
            }
            _ => {}
        }
    }
    if (*cinfo).restart_interval != 0 {
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    }
    return TRUE;
}
pub unsafe extern "C" fn jinit_huff_decoder(mut cinfo: j_decompress_ptr) {
    let mut entropy: huff_entropy_ptr = ::core::ptr::null_mut::<huff_entropy_decoder>();
    let mut i: ::core::ffi::c_int = 0;
    std_huff_tables(cinfo as j_common_ptr);
    entropy = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<huff_entropy_decoder>() as size_t,
    ) as huff_entropy_ptr;
    (*cinfo).entropy = entropy as *mut jpeg_entropy_decoder as *mut jpeg_entropy_decoder;
    (*entropy).pub_0.start_pass =
        Some(start_pass_huff_decoder as unsafe extern "C" fn(j_decompress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
    (*entropy).pub_0.decode_mcu =
        Some(decode_mcu as unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean)
            as Option<unsafe extern "C" fn(j_decompress_ptr, *mut JBLOCKROW) -> boolean>;
    i = 0 as ::core::ffi::c_int;
    while i < NUM_HUFF_TBLS {
        (*entropy).ac_derived_tbls[i as usize] = ::core::ptr::null_mut::<d_derived_tbl>();
        (*entropy).dc_derived_tbls[i as usize] = (*entropy).ac_derived_tbls[i as usize];
        i += 1;
    }
}
