#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
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
pub type UJCOEF = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct c_derived_tbl {
    pub ehufco: [::core::ffi::c_uint; 256],
    pub ehufsi: [::core::ffi::c_char; 256],
}
pub type DCTELEM = ::core::ffi::c_short;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_rgb_ycc() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_rgb_gray() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_ycc_rgb() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_ycc_rgb565() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_c_can_null_convert() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_rgb_ycc_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_rgb_gray_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_ycc_rgb_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_ycc_rgb565_convert(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut input_row: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut num_rows: ::core::ffi::c_int,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_c_null_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: ::core::ffi::c_int,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_h2v2_downsample() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_h2v1_downsample() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_h2v2_smooth_downsample() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_h2v2_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_h2v2_smooth_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_h2v1_downsample(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data: JSAMPARRAY,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_h2v2_upsample() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_h2v1_upsample() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_int_upsample() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_int_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_h2v2_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_h2v1_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_h2v2_fancy_upsample() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_h2v1_fancy_upsample() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_h1v2_fancy_upsample() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_h2v2_fancy_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_h2v1_fancy_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_h1v2_fancy_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_h2v2_merged_upsample() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_h2v1_merged_upsample() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_h2v2_merged_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_h2v1_merged_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: JDIMENSION,
    mut output_buf: JSAMPARRAY,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_convsamp() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_convsamp_float() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_convsamp(
    mut sample_data: JSAMPARRAY,
    mut start_col: JDIMENSION,
    mut workspace: *mut DCTELEM,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_convsamp_float(
    mut sample_data: JSAMPARRAY,
    mut start_col: JDIMENSION,
    mut workspace: *mut ::core::ffi::c_float,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_fdct_islow() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_fdct_ifast() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_fdct_float() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_fdct_islow(mut data: *mut DCTELEM) {}
#[no_mangle]
pub unsafe extern "C" fn jsimd_fdct_ifast(mut data: *mut DCTELEM) {}
#[no_mangle]
pub unsafe extern "C" fn jsimd_fdct_float(mut data: *mut ::core::ffi::c_float) {}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_quantize() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_quantize_float() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_quantize(
    mut coef_block: JCOEFPTR,
    mut divisors: *mut DCTELEM,
    mut workspace: *mut DCTELEM,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_quantize_float(
    mut coef_block: JCOEFPTR,
    mut divisors: *mut ::core::ffi::c_float,
    mut workspace: *mut ::core::ffi::c_float,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_idct_2x2() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_idct_4x4() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_idct_6x6() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_idct_12x12() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_idct_2x2(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_idct_4x4(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_idct_6x6(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_idct_12x12(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_idct_islow() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_idct_ifast() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_idct_float() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_idct_islow(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_idct_ifast(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_idct_float(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_huff_encode_one_block() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_huff_encode_one_block(
    mut state: *mut ::core::ffi::c_void,
    mut buffer: *mut JOCTET,
    mut block: JCOEFPTR,
    mut last_dc_val: ::core::ffi::c_int,
    mut dctbl: *mut c_derived_tbl,
    mut actbl: *mut c_derived_tbl,
) -> *mut JOCTET {
    return ::core::ptr::null_mut::<JOCTET>();
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_encode_mcu_AC_first_prepare() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_encode_mcu_AC_first_prepare(
    mut block: *const JCOEF,
    mut jpeg_natural_order_start: *const ::core::ffi::c_int,
    mut Sl: ::core::ffi::c_int,
    mut Al: ::core::ffi::c_int,
    mut values: *mut UJCOEF,
    mut zerobits: *mut size_t,
) {
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_can_encode_mcu_AC_refine_prepare() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jsimd_encode_mcu_AC_refine_prepare(
    mut block: *const JCOEF,
    mut jpeg_natural_order_start: *const ::core::ffi::c_int,
    mut Sl: ::core::ffi::c_int,
    mut Al: ::core::ffi::c_int,
    mut absvalues: *mut UJCOEF,
    mut bits: *mut size_t,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
