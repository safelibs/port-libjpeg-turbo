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
    fn jpeg_alloc_quant_table(cinfo: j_common_ptr) -> *mut JQUANT_TBL;
    fn jpeg_alloc_huff_table(cinfo: j_common_ptr) -> *mut JHUFF_TBL;
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
pub const JERR_BAD_IN_COLORSPACE: C2RustUnnamed_0 = 10;
pub const JERR_BAD_J_COLORSPACE: C2RustUnnamed_0 = 11;
pub const JERR_COMPONENT_COUNT: C2RustUnnamed_0 = 27;
pub const JERR_BAD_STATE: C2RustUnnamed_0 = 21;
pub const JERR_BAD_HUFF_TABLE: C2RustUnnamed_0 = 9;
pub const JERR_DQT_INDEX: C2RustUnnamed_0 = 32;
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
pub const JERR_DHT_INDEX: C2RustUnnamed_0 = 31;
pub const JERR_DAC_VALUE: C2RustUnnamed_0 = 30;
pub const JERR_DAC_INDEX: C2RustUnnamed_0 = 29;
pub const JERR_CONVERSION_NOTIMPL: C2RustUnnamed_0 = 28;
pub const JERR_CCIR601_NOTIMPL: C2RustUnnamed_0 = 26;
pub const JERR_CANT_SUSPEND: C2RustUnnamed_0 = 25;
pub const JERR_BUFFER_SIZE: C2RustUnnamed_0 = 24;
pub const JERR_BAD_VIRTUAL_ACCESS: C2RustUnnamed_0 = 23;
pub const JERR_BAD_STRUCT_SIZE: C2RustUnnamed_0 = 22;
pub const JERR_BAD_SCAN_SCRIPT: C2RustUnnamed_0 = 20;
pub const JERR_BAD_SAMPLING: C2RustUnnamed_0 = 19;
pub const JERR_BAD_PROG_SCRIPT: C2RustUnnamed_0 = 18;
pub const JERR_BAD_PROGRESSION: C2RustUnnamed_0 = 17;
pub const JERR_BAD_PRECISION: C2RustUnnamed_0 = 16;
pub const JERR_BAD_POOL_ID: C2RustUnnamed_0 = 15;
pub const JERR_BAD_MCU_SIZE: C2RustUnnamed_0 = 14;
pub const JERR_BAD_LIB_VERSION: C2RustUnnamed_0 = 13;
pub const JERR_BAD_LENGTH: C2RustUnnamed_0 = 12;
pub const JERR_BAD_DROP_SAMPLING: C2RustUnnamed_0 = 8;
pub const JERR_BAD_DCTSIZE: C2RustUnnamed_0 = 7;
pub const JERR_BAD_DCT_COEF: C2RustUnnamed_0 = 6;
pub const JERR_BAD_CROP_SPEC: C2RustUnnamed_0 = 5;
pub const JERR_BAD_COMPONENT_ID: C2RustUnnamed_0 = 4;
pub const JERR_BAD_BUFFER_MODE: C2RustUnnamed_0 = 3;
pub const JERR_BAD_ALLOC_CHUNK: C2RustUnnamed_0 = 2;
pub const JERR_BAD_ALIGN_TYPE: C2RustUnnamed_0 = 1;
pub const JMSG_NOMESSAGE: C2RustUnnamed_0 = 0;
pub const BITS_IN_JSAMPLE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAX_COMPONENTS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DCTSIZE2: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NUM_QUANT_TBLS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NUM_ARITH_TBLS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MAX_COMPS_IN_SCAN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JPOOL_PERMANENT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CSTATE_START: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
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
        &raw mut (**htblptr).bits as *mut UINT8 as *mut ::core::ffi::c_void,
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
        &raw mut (**htblptr).huffval as *mut UINT8 as *mut ::core::ffi::c_void,
        val as *const ::core::ffi::c_void,
        (nsymbols as size_t).wrapping_mul(::core::mem::size_of::<UINT8>() as size_t),
    );
    memset(
        (&raw mut (**htblptr).huffval as *mut UINT8).offset(nsymbols as isize) as *mut UINT8
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
            &raw mut (*(cinfo as j_decompress_ptr)).dc_huff_tbl_ptrs as *mut *mut JHUFF_TBL;
        ac_huff_tbl_ptrs =
            &raw mut (*(cinfo as j_decompress_ptr)).ac_huff_tbl_ptrs as *mut *mut JHUFF_TBL;
    } else {
        dc_huff_tbl_ptrs =
            &raw mut (*(cinfo as j_compress_ptr)).dc_huff_tbl_ptrs as *mut *mut JHUFF_TBL;
        ac_huff_tbl_ptrs =
            &raw mut (*(cinfo as j_compress_ptr)).ac_huff_tbl_ptrs as *mut *mut JHUFF_TBL;
    }
    add_huff_table(
        cinfo,
        dc_huff_tbl_ptrs.offset(0 as ::core::ffi::c_int as isize) as *mut *mut JHUFF_TBL,
        &raw const bits_dc_luminance as *const UINT8,
        &raw const val_dc_luminance as *const UINT8,
    );
    add_huff_table(
        cinfo,
        ac_huff_tbl_ptrs.offset(0 as ::core::ffi::c_int as isize) as *mut *mut JHUFF_TBL,
        &raw const bits_ac_luminance as *const UINT8,
        &raw const val_ac_luminance as *const UINT8,
    );
    add_huff_table(
        cinfo,
        dc_huff_tbl_ptrs.offset(1 as ::core::ffi::c_int as isize) as *mut *mut JHUFF_TBL,
        &raw const bits_dc_chrominance as *const UINT8,
        &raw const val_dc_chrominance as *const UINT8,
    );
    add_huff_table(
        cinfo,
        ac_huff_tbl_ptrs.offset(1 as ::core::ffi::c_int as isize) as *mut *mut JHUFF_TBL,
        &raw const bits_ac_chrominance as *const UINT8,
        &raw const val_ac_chrominance as *const UINT8,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_add_quant_table(
    mut cinfo: j_compress_ptr,
    mut which_tbl: ::core::ffi::c_int,
    mut basic_table: *const ::core::ffi::c_uint,
    mut scale_factor: ::core::ffi::c_int,
    mut force_baseline: boolean,
) {
    let mut qtblptr: *mut *mut JQUANT_TBL = ::core::ptr::null_mut::<*mut JQUANT_TBL>();
    let mut i: ::core::ffi::c_int = 0;
    let mut temp: ::core::ffi::c_long = 0;
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if which_tbl < 0 as ::core::ffi::c_int || which_tbl >= NUM_QUANT_TBLS {
        (*(*cinfo).err).msg_code = JERR_DQT_INDEX as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = which_tbl;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    qtblptr = (&raw mut (*cinfo).quant_tbl_ptrs as *mut *mut JQUANT_TBL).offset(which_tbl as isize)
        as *mut *mut JQUANT_TBL;
    if (*qtblptr).is_null() {
        *qtblptr = jpeg_alloc_quant_table(cinfo as j_common_ptr);
    }
    i = 0 as ::core::ffi::c_int;
    while i < DCTSIZE2 {
        temp = (*basic_table.offset(i as isize) as ::core::ffi::c_long
            * scale_factor as ::core::ffi::c_long
            + 50 as ::core::ffi::c_long)
            / 100 as ::core::ffi::c_long;
        if temp <= 0 as ::core::ffi::c_long {
            temp = 1 as ::core::ffi::c_long;
        }
        if temp > 32767 as ::core::ffi::c_long {
            temp = 32767 as ::core::ffi::c_long;
        }
        if force_baseline != 0 && temp > 255 as ::core::ffi::c_long {
            temp = 255 as ::core::ffi::c_long;
        }
        (**qtblptr).quantval[i as usize] = temp as UINT16;
        i += 1;
    }
    (**qtblptr).sent_table = FALSE as boolean;
}
static mut std_luminance_quant_tbl: [::core::ffi::c_uint; 64] = [
    16 as ::core::ffi::c_int as ::core::ffi::c_uint,
    11 as ::core::ffi::c_int as ::core::ffi::c_uint,
    10 as ::core::ffi::c_int as ::core::ffi::c_uint,
    16 as ::core::ffi::c_int as ::core::ffi::c_uint,
    24 as ::core::ffi::c_int as ::core::ffi::c_uint,
    40 as ::core::ffi::c_int as ::core::ffi::c_uint,
    51 as ::core::ffi::c_int as ::core::ffi::c_uint,
    61 as ::core::ffi::c_int as ::core::ffi::c_uint,
    12 as ::core::ffi::c_int as ::core::ffi::c_uint,
    12 as ::core::ffi::c_int as ::core::ffi::c_uint,
    14 as ::core::ffi::c_int as ::core::ffi::c_uint,
    19 as ::core::ffi::c_int as ::core::ffi::c_uint,
    26 as ::core::ffi::c_int as ::core::ffi::c_uint,
    58 as ::core::ffi::c_int as ::core::ffi::c_uint,
    60 as ::core::ffi::c_int as ::core::ffi::c_uint,
    55 as ::core::ffi::c_int as ::core::ffi::c_uint,
    14 as ::core::ffi::c_int as ::core::ffi::c_uint,
    13 as ::core::ffi::c_int as ::core::ffi::c_uint,
    16 as ::core::ffi::c_int as ::core::ffi::c_uint,
    24 as ::core::ffi::c_int as ::core::ffi::c_uint,
    40 as ::core::ffi::c_int as ::core::ffi::c_uint,
    57 as ::core::ffi::c_int as ::core::ffi::c_uint,
    69 as ::core::ffi::c_int as ::core::ffi::c_uint,
    56 as ::core::ffi::c_int as ::core::ffi::c_uint,
    14 as ::core::ffi::c_int as ::core::ffi::c_uint,
    17 as ::core::ffi::c_int as ::core::ffi::c_uint,
    22 as ::core::ffi::c_int as ::core::ffi::c_uint,
    29 as ::core::ffi::c_int as ::core::ffi::c_uint,
    51 as ::core::ffi::c_int as ::core::ffi::c_uint,
    87 as ::core::ffi::c_int as ::core::ffi::c_uint,
    80 as ::core::ffi::c_int as ::core::ffi::c_uint,
    62 as ::core::ffi::c_int as ::core::ffi::c_uint,
    18 as ::core::ffi::c_int as ::core::ffi::c_uint,
    22 as ::core::ffi::c_int as ::core::ffi::c_uint,
    37 as ::core::ffi::c_int as ::core::ffi::c_uint,
    56 as ::core::ffi::c_int as ::core::ffi::c_uint,
    68 as ::core::ffi::c_int as ::core::ffi::c_uint,
    109 as ::core::ffi::c_int as ::core::ffi::c_uint,
    103 as ::core::ffi::c_int as ::core::ffi::c_uint,
    77 as ::core::ffi::c_int as ::core::ffi::c_uint,
    24 as ::core::ffi::c_int as ::core::ffi::c_uint,
    35 as ::core::ffi::c_int as ::core::ffi::c_uint,
    55 as ::core::ffi::c_int as ::core::ffi::c_uint,
    64 as ::core::ffi::c_int as ::core::ffi::c_uint,
    81 as ::core::ffi::c_int as ::core::ffi::c_uint,
    104 as ::core::ffi::c_int as ::core::ffi::c_uint,
    113 as ::core::ffi::c_int as ::core::ffi::c_uint,
    92 as ::core::ffi::c_int as ::core::ffi::c_uint,
    49 as ::core::ffi::c_int as ::core::ffi::c_uint,
    64 as ::core::ffi::c_int as ::core::ffi::c_uint,
    78 as ::core::ffi::c_int as ::core::ffi::c_uint,
    87 as ::core::ffi::c_int as ::core::ffi::c_uint,
    103 as ::core::ffi::c_int as ::core::ffi::c_uint,
    121 as ::core::ffi::c_int as ::core::ffi::c_uint,
    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
    101 as ::core::ffi::c_int as ::core::ffi::c_uint,
    72 as ::core::ffi::c_int as ::core::ffi::c_uint,
    92 as ::core::ffi::c_int as ::core::ffi::c_uint,
    95 as ::core::ffi::c_int as ::core::ffi::c_uint,
    98 as ::core::ffi::c_int as ::core::ffi::c_uint,
    112 as ::core::ffi::c_int as ::core::ffi::c_uint,
    100 as ::core::ffi::c_int as ::core::ffi::c_uint,
    103 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
static mut std_chrominance_quant_tbl: [::core::ffi::c_uint; 64] = [
    17 as ::core::ffi::c_int as ::core::ffi::c_uint,
    18 as ::core::ffi::c_int as ::core::ffi::c_uint,
    24 as ::core::ffi::c_int as ::core::ffi::c_uint,
    47 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    18 as ::core::ffi::c_int as ::core::ffi::c_uint,
    21 as ::core::ffi::c_int as ::core::ffi::c_uint,
    26 as ::core::ffi::c_int as ::core::ffi::c_uint,
    66 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    24 as ::core::ffi::c_int as ::core::ffi::c_uint,
    26 as ::core::ffi::c_int as ::core::ffi::c_uint,
    56 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    47 as ::core::ffi::c_int as ::core::ffi::c_uint,
    66 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    99 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
#[no_mangle]
pub unsafe extern "C" fn jpeg_default_qtables(
    mut cinfo: j_compress_ptr,
    mut force_baseline: boolean,
) {
    jpeg_add_quant_table(
        cinfo,
        0 as ::core::ffi::c_int,
        &raw const std_luminance_quant_tbl as *const ::core::ffi::c_uint,
        (*cinfo).q_scale_factor[0 as ::core::ffi::c_int as usize],
        force_baseline,
    );
    jpeg_add_quant_table(
        cinfo,
        1 as ::core::ffi::c_int,
        &raw const std_chrominance_quant_tbl as *const ::core::ffi::c_uint,
        (*cinfo).q_scale_factor[1 as ::core::ffi::c_int as usize],
        force_baseline,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_set_linear_quality(
    mut cinfo: j_compress_ptr,
    mut scale_factor: ::core::ffi::c_int,
    mut force_baseline: boolean,
) {
    jpeg_add_quant_table(
        cinfo,
        0 as ::core::ffi::c_int,
        &raw const std_luminance_quant_tbl as *const ::core::ffi::c_uint,
        scale_factor,
        force_baseline,
    );
    jpeg_add_quant_table(
        cinfo,
        1 as ::core::ffi::c_int,
        &raw const std_chrominance_quant_tbl as *const ::core::ffi::c_uint,
        scale_factor,
        force_baseline,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_quality_scaling(
    mut quality: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if quality <= 0 as ::core::ffi::c_int {
        quality = 1 as ::core::ffi::c_int;
    }
    if quality > 100 as ::core::ffi::c_int {
        quality = 100 as ::core::ffi::c_int;
    }
    if quality < 50 as ::core::ffi::c_int {
        quality = 5000 as ::core::ffi::c_int / quality;
    } else {
        quality = 200 as ::core::ffi::c_int - quality * 2 as ::core::ffi::c_int;
    }
    return quality;
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_set_quality(
    mut cinfo: j_compress_ptr,
    mut quality: ::core::ffi::c_int,
    mut force_baseline: boolean,
) {
    quality = jpeg_quality_scaling(quality);
    jpeg_set_linear_quality(cinfo, quality, force_baseline);
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_set_defaults(mut cinfo: j_compress_ptr) {
    let mut i: ::core::ffi::c_int = 0;
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*cinfo).comp_info.is_null() {
        (*cinfo).comp_info = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            (MAX_COMPONENTS as size_t)
                .wrapping_mul(::core::mem::size_of::<jpeg_component_info>() as size_t),
        ) as *mut jpeg_component_info;
    }
    (*cinfo).scale_num = 1 as ::core::ffi::c_uint;
    (*cinfo).scale_denom = 1 as ::core::ffi::c_uint;
    (*cinfo).data_precision = BITS_IN_JSAMPLE;
    jpeg_set_quality(cinfo, 75 as ::core::ffi::c_int, TRUE);
    std_huff_tables(cinfo as j_common_ptr);
    i = 0 as ::core::ffi::c_int;
    while i < NUM_ARITH_TBLS {
        (*cinfo).arith_dc_L[i as usize] = 0 as UINT8;
        (*cinfo).arith_dc_U[i as usize] = 1 as UINT8;
        (*cinfo).arith_ac_K[i as usize] = 5 as UINT8;
        i += 1;
    }
    (*cinfo).scan_info = ::core::ptr::null::<jpeg_scan_info>();
    (*cinfo).num_scans = 0 as ::core::ffi::c_int;
    (*cinfo).raw_data_in = FALSE as boolean;
    (*cinfo).arith_code = FALSE as boolean;
    (*cinfo).optimize_coding = FALSE as boolean;
    if (*cinfo).data_precision > 8 as ::core::ffi::c_int {
        (*cinfo).optimize_coding = TRUE as boolean;
    }
    (*cinfo).CCIR601_sampling = FALSE as boolean;
    (*cinfo).do_fancy_downsampling = TRUE as boolean;
    (*cinfo).smoothing_factor = 0 as ::core::ffi::c_int;
    (*cinfo).dct_method = JDCT_ISLOW;
    (*cinfo).restart_interval = 0 as ::core::ffi::c_uint;
    (*cinfo).restart_in_rows = 0 as ::core::ffi::c_int;
    (*cinfo).JFIF_major_version = 1 as UINT8;
    (*cinfo).JFIF_minor_version = 1 as UINT8;
    (*cinfo).density_unit = 0 as UINT8;
    (*cinfo).X_density = 1 as UINT16;
    (*cinfo).Y_density = 1 as UINT16;
    jpeg_default_colorspace(cinfo);
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_default_colorspace(mut cinfo: j_compress_ptr) {
    match (*cinfo).in_color_space as ::core::ffi::c_uint {
        1 => {
            jpeg_set_colorspace(cinfo, JCS_GRAYSCALE);
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            jpeg_set_colorspace(cinfo, JCS_YCbCr);
        }
        3 => {
            jpeg_set_colorspace(cinfo, JCS_YCbCr);
        }
        4 => {
            jpeg_set_colorspace(cinfo, JCS_CMYK);
        }
        5 => {
            jpeg_set_colorspace(cinfo, JCS_YCCK);
        }
        0 => {
            jpeg_set_colorspace(cinfo, JCS_UNKNOWN);
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_set_colorspace(
    mut cinfo: j_compress_ptr,
    mut colorspace: J_COLOR_SPACE,
) {
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut ci: ::core::ffi::c_int = 0;
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*cinfo).jpeg_color_space = colorspace;
    (*cinfo).write_JFIF_header = FALSE as boolean;
    (*cinfo).write_Adobe_marker = FALSE as boolean;
    match colorspace as ::core::ffi::c_uint {
        1 => {
            (*cinfo).write_JFIF_header = TRUE as boolean;
            (*cinfo).num_components = 1 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 1 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
        }
        2 => {
            (*cinfo).write_Adobe_marker = TRUE as boolean;
            (*cinfo).num_components = 3 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 0x52 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(1 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 0x47 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(2 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 0x42 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
        }
        3 => {
            (*cinfo).write_JFIF_header = TRUE as boolean;
            (*cinfo).num_components = 3 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 1 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 2 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 2 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(1 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 2 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 1 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 1 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 1 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(2 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 3 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 1 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 1 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 1 as ::core::ffi::c_int;
        }
        4 => {
            (*cinfo).write_Adobe_marker = TRUE as boolean;
            (*cinfo).num_components = 4 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 0x43 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(1 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 0x4d as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(2 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 0x59 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(3 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 0x4b as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
        }
        5 => {
            (*cinfo).write_Adobe_marker = TRUE as boolean;
            (*cinfo).num_components = 4 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 1 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 2 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 2 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(1 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 2 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 1 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 1 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 1 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(2 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 3 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 1 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 1 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 1 as ::core::ffi::c_int;
            compptr = (*cinfo).comp_info.offset(3 as ::core::ffi::c_int as isize)
                as *mut jpeg_component_info;
            (*compptr).component_id = 4 as ::core::ffi::c_int;
            (*compptr).h_samp_factor = 2 as ::core::ffi::c_int;
            (*compptr).v_samp_factor = 2 as ::core::ffi::c_int;
            (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
            (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
        }
        0 => {
            (*cinfo).num_components = (*cinfo).input_components;
            if (*cinfo).num_components < 1 as ::core::ffi::c_int
                || (*cinfo).num_components > MAX_COMPONENTS
            {
                (*(*cinfo).err).msg_code = JERR_COMPONENT_COUNT as ::core::ffi::c_int;
                (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] =
                    (*cinfo).num_components;
                (*(*cinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] =
                    10 as ::core::ffi::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            ci = 0 as ::core::ffi::c_int;
            while ci < (*cinfo).num_components {
                compptr = (*cinfo).comp_info.offset(ci as isize) as *mut jpeg_component_info;
                (*compptr).component_id = ci;
                (*compptr).h_samp_factor = 1 as ::core::ffi::c_int;
                (*compptr).v_samp_factor = 1 as ::core::ffi::c_int;
                (*compptr).quant_tbl_no = 0 as ::core::ffi::c_int;
                (*compptr).dc_tbl_no = 0 as ::core::ffi::c_int;
                (*compptr).ac_tbl_no = 0 as ::core::ffi::c_int;
                ci += 1;
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    };
}
unsafe extern "C" fn fill_a_scan(
    mut scanptr: *mut jpeg_scan_info,
    mut ci: ::core::ffi::c_int,
    mut Ss: ::core::ffi::c_int,
    mut Se: ::core::ffi::c_int,
    mut Ah: ::core::ffi::c_int,
    mut Al: ::core::ffi::c_int,
) -> *mut jpeg_scan_info {
    (*scanptr).comps_in_scan = 1 as ::core::ffi::c_int;
    (*scanptr).component_index[0 as ::core::ffi::c_int as usize] = ci;
    (*scanptr).Ss = Ss;
    (*scanptr).Se = Se;
    (*scanptr).Ah = Ah;
    (*scanptr).Al = Al;
    scanptr = scanptr.offset(1);
    return scanptr;
}
unsafe extern "C" fn fill_scans(
    mut scanptr: *mut jpeg_scan_info,
    mut ncomps: ::core::ffi::c_int,
    mut Ss: ::core::ffi::c_int,
    mut Se: ::core::ffi::c_int,
    mut Ah: ::core::ffi::c_int,
    mut Al: ::core::ffi::c_int,
) -> *mut jpeg_scan_info {
    let mut ci: ::core::ffi::c_int = 0;
    ci = 0 as ::core::ffi::c_int;
    while ci < ncomps {
        (*scanptr).comps_in_scan = 1 as ::core::ffi::c_int;
        (*scanptr).component_index[0 as ::core::ffi::c_int as usize] = ci;
        (*scanptr).Ss = Ss;
        (*scanptr).Se = Se;
        (*scanptr).Ah = Ah;
        (*scanptr).Al = Al;
        scanptr = scanptr.offset(1);
        ci += 1;
    }
    return scanptr;
}
unsafe extern "C" fn fill_dc_scans(
    mut scanptr: *mut jpeg_scan_info,
    mut ncomps: ::core::ffi::c_int,
    mut Ah: ::core::ffi::c_int,
    mut Al: ::core::ffi::c_int,
) -> *mut jpeg_scan_info {
    let mut ci: ::core::ffi::c_int = 0;
    if ncomps <= MAX_COMPS_IN_SCAN {
        (*scanptr).comps_in_scan = ncomps;
        ci = 0 as ::core::ffi::c_int;
        while ci < ncomps {
            (*scanptr).component_index[ci as usize] = ci;
            ci += 1;
        }
        (*scanptr).Se = 0 as ::core::ffi::c_int;
        (*scanptr).Ss = (*scanptr).Se;
        (*scanptr).Ah = Ah;
        (*scanptr).Al = Al;
        scanptr = scanptr.offset(1);
    } else {
        scanptr = fill_scans(
            scanptr,
            ncomps,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            Ah,
            Al,
        );
    }
    return scanptr;
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_simple_progression(mut cinfo: j_compress_ptr) {
    let mut ncomps: ::core::ffi::c_int = (*cinfo).num_components;
    let mut nscans: ::core::ffi::c_int = 0;
    let mut scanptr: *mut jpeg_scan_info = ::core::ptr::null_mut::<jpeg_scan_info>();
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as ::core::ffi::c_int;
        (*(*cinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if ncomps == 3 as ::core::ffi::c_int
        && (*cinfo).jpeg_color_space as ::core::ffi::c_uint
            == JCS_YCbCr as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        nscans = 10 as ::core::ffi::c_int;
    } else if ncomps > MAX_COMPS_IN_SCAN {
        nscans = 6 as ::core::ffi::c_int * ncomps;
    } else {
        nscans = 2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * ncomps;
    }
    if (*cinfo).script_space.is_null() || (*cinfo).script_space_size < nscans {
        (*cinfo).script_space_size = if nscans > 10 as ::core::ffi::c_int {
            nscans
        } else {
            10 as ::core::ffi::c_int
        };
        (*cinfo).script_space = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            ((*cinfo).script_space_size as size_t)
                .wrapping_mul(::core::mem::size_of::<jpeg_scan_info>() as size_t),
        ) as *mut jpeg_scan_info;
    }
    scanptr = (*cinfo).script_space;
    (*cinfo).scan_info = scanptr;
    (*cinfo).num_scans = nscans;
    if ncomps == 3 as ::core::ffi::c_int
        && (*cinfo).jpeg_color_space as ::core::ffi::c_uint
            == JCS_YCbCr as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        scanptr = fill_dc_scans(
            scanptr,
            ncomps,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        scanptr = fill_a_scan(
            scanptr,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            5 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
        scanptr = fill_a_scan(
            scanptr,
            2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            63 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        scanptr = fill_a_scan(
            scanptr,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            63 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        scanptr = fill_a_scan(
            scanptr,
            0 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            63 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
        scanptr = fill_a_scan(
            scanptr,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            63 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        scanptr = fill_dc_scans(
            scanptr,
            ncomps,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        scanptr = fill_a_scan(
            scanptr,
            2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            63 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        scanptr = fill_a_scan(
            scanptr,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            63 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        scanptr = fill_a_scan(
            scanptr,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            63 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    } else {
        scanptr = fill_dc_scans(
            scanptr,
            ncomps,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        scanptr = fill_scans(
            scanptr,
            ncomps,
            1 as ::core::ffi::c_int,
            5 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
        scanptr = fill_scans(
            scanptr,
            ncomps,
            6 as ::core::ffi::c_int,
            63 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
        scanptr = fill_scans(
            scanptr,
            ncomps,
            1 as ::core::ffi::c_int,
            63 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        scanptr = fill_dc_scans(
            scanptr,
            ncomps,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        scanptr = fill_scans(
            scanptr,
            ncomps,
            1 as ::core::ffi::c_int,
            63 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    };
}

pub const JPEG_RS_JCPARAM_LINK_ANCHOR: unsafe extern "C" fn(j_compress_ptr) = jpeg_set_defaults;
