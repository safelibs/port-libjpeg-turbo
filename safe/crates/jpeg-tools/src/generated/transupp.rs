#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}
extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn jpeg_set_colorspace(cinfo: j_compress_ptr, colorspace: J_COLOR_SPACE);
    fn jpeg_write_marker(
        cinfo: j_compress_ptr,
        marker: ::core::ffi::c_int,
        dataptr: *const JOCTET,
        datalen: ::core::ffi::c_uint,
    );
    fn jpeg_core_output_dimensions(cinfo: j_decompress_ptr);
    fn jpeg_save_markers(
        cinfo: j_decompress_ptr,
        marker_code: ::core::ffi::c_int,
        length_limit: ::core::ffi::c_uint,
    );
    fn jdiv_round_up(a: ::core::ffi::c_long, b: ::core::ffi::c_long) -> ::core::ffi::c_long;
    fn jcopy_block_row(input_row: JBLOCKROW, output_row: JBLOCKROW, num_blocks: JDIMENSION);
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
pub type JXFORM_CODE = ::core::ffi::c_uint;
pub const JXFORM_DROP: JXFORM_CODE = 9;
pub const JXFORM_WIPE: JXFORM_CODE = 8;
pub const JXFORM_ROT_270: JXFORM_CODE = 7;
pub const JXFORM_ROT_180: JXFORM_CODE = 6;
pub const JXFORM_ROT_90: JXFORM_CODE = 5;
pub const JXFORM_TRANSVERSE: JXFORM_CODE = 4;
pub const JXFORM_TRANSPOSE: JXFORM_CODE = 3;
pub const JXFORM_FLIP_V: JXFORM_CODE = 2;
pub const JXFORM_FLIP_H: JXFORM_CODE = 1;
pub const JXFORM_NONE: JXFORM_CODE = 0;
pub type JCROP_CODE = ::core::ffi::c_uint;
pub const JCROP_REFLECT: JCROP_CODE = 4;
pub const JCROP_FORCE: JCROP_CODE = 3;
pub const JCROP_NEG: JCROP_CODE = 2;
pub const JCROP_POS: JCROP_CODE = 1;
pub const JCROP_UNSET: JCROP_CODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_transform_info {
    pub transform: JXFORM_CODE,
    pub perfect: boolean,
    pub trim: boolean,
    pub force_grayscale: boolean,
    pub crop: boolean,
    pub slow_hflip: boolean,
    pub crop_width: JDIMENSION,
    pub crop_width_set: JCROP_CODE,
    pub crop_height: JDIMENSION,
    pub crop_height_set: JCROP_CODE,
    pub crop_xoffset: JDIMENSION,
    pub crop_xoffset_set: JCROP_CODE,
    pub crop_yoffset: JDIMENSION,
    pub crop_yoffset_set: JCROP_CODE,
    pub drop_ptr: j_decompress_ptr,
    pub drop_coef_arrays: *mut jvirt_barray_ptr,
    pub num_components: ::core::ffi::c_int,
    pub workspace_coef_arrays: *mut jvirt_barray_ptr,
    pub output_width: JDIMENSION,
    pub output_height: JDIMENSION,
    pub x_crop_offset: JDIMENSION,
    pub y_crop_offset: JDIMENSION,
    pub drop_width: JDIMENSION,
    pub drop_height: JDIMENSION,
    pub iMCU_sample_width: ::core::ffi::c_int,
    pub iMCU_sample_height: ::core::ffi::c_int,
}
pub const _ISdigit: C2RustUnnamed_1 = 2048;
pub type JCOPY_OPTION = ::core::ffi::c_uint;
pub const JCOPYOPT_ICC: JCOPY_OPTION = 4;
pub const JCOPYOPT_ALL_EXCEPT_ICC: JCOPY_OPTION = 3;
pub const JCOPYOPT_ALL: JCOPY_OPTION = 2;
pub const JCOPYOPT_COMMENTS: JCOPY_OPTION = 1;
pub const JCOPYOPT_NONE: JCOPY_OPTION = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_1 = 8;
pub const _ISpunct: C2RustUnnamed_1 = 4;
pub const _IScntrl: C2RustUnnamed_1 = 2;
pub const _ISblank: C2RustUnnamed_1 = 1;
pub const _ISgraph: C2RustUnnamed_1 = 32768;
pub const _ISprint: C2RustUnnamed_1 = 16384;
pub const _ISspace: C2RustUnnamed_1 = 8192;
pub const _ISxdigit: C2RustUnnamed_1 = 4096;
pub const _ISalpha: C2RustUnnamed_1 = 1024;
pub const _ISlower: C2RustUnnamed_1 = 512;
pub const _ISupper: C2RustUnnamed_1 = 256;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DCTSIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const DCTSIZE2: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NUM_QUANT_TBLS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPEG_APP0: ::core::ffi::c_int = 0xe0 as ::core::ffi::c_int;
pub const JPEG_COM: ::core::ffi::c_int = 0xfe as ::core::ffi::c_int;
unsafe extern "C" fn dequant_comp(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_array: jvirt_barray_ptr,
    mut qtblptr1: *mut JQUANT_TBL,
) {
    let mut blk_x: JDIMENSION = 0;
    let mut blk_y: JDIMENSION = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut qtblptr: *mut JQUANT_TBL = ::core::ptr::null_mut::<JQUANT_TBL>();
    let mut buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut block: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    qtblptr = (*compptr).quant_table;
    blk_y = 0 as JDIMENSION;
    while blk_y < (*compptr).height_in_blocks {
        buffer = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            coef_array,
            blk_y,
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        );
        offset_y = 0 as ::core::ffi::c_int;
        while offset_y < (*compptr).v_samp_factor {
            block = *buffer.offset(offset_y as isize);
            blk_x = 0 as JDIMENSION;
            while blk_x < (*compptr).width_in_blocks {
                ptr = &raw mut *block.offset(blk_x as isize) as *mut JCOEF as JCOEFPTR;
                k = 0 as ::core::ffi::c_int;
                while k < DCTSIZE2 {
                    if (*qtblptr).quantval[k as usize] as ::core::ffi::c_int
                        != (*qtblptr1).quantval[k as usize] as ::core::ffi::c_int
                    {
                        let ref mut fresh1 = *ptr.offset(k as isize);
                        *fresh1 = (*fresh1 as ::core::ffi::c_int
                            * ((*qtblptr).quantval[k as usize] as ::core::ffi::c_int
                                / (*qtblptr1).quantval[k as usize] as ::core::ffi::c_int))
                            as JCOEF;
                    }
                    k += 1;
                }
                blk_x = blk_x.wrapping_add(1);
            }
            offset_y += 1;
        }
        blk_y = blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
    }
}
unsafe extern "C" fn requant_comp(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_array: jvirt_barray_ptr,
    mut qtblptr1: *mut JQUANT_TBL,
) {
    let mut blk_x: JDIMENSION = 0;
    let mut blk_y: JDIMENSION = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut qtblptr: *mut JQUANT_TBL = ::core::ptr::null_mut::<JQUANT_TBL>();
    let mut buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut block: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut temp: JCOEF = 0;
    let mut qval: JCOEF = 0;
    qtblptr = (*compptr).quant_table;
    blk_y = 0 as JDIMENSION;
    while blk_y < (*compptr).height_in_blocks {
        buffer = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            coef_array,
            blk_y,
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        );
        offset_y = 0 as ::core::ffi::c_int;
        while offset_y < (*compptr).v_samp_factor {
            block = *buffer.offset(offset_y as isize);
            blk_x = 0 as JDIMENSION;
            while blk_x < (*compptr).width_in_blocks {
                ptr = &raw mut *block.offset(blk_x as isize) as *mut JCOEF as JCOEFPTR;
                k = 0 as ::core::ffi::c_int;
                while k < DCTSIZE2 {
                    temp = (*qtblptr).quantval[k as usize] as JCOEF;
                    qval = (*qtblptr1).quantval[k as usize] as JCOEF;
                    if temp as ::core::ffi::c_int != qval as ::core::ffi::c_int
                        && qval as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    {
                        temp = (temp as ::core::ffi::c_int
                            * *ptr.offset(k as isize) as ::core::ffi::c_int)
                            as JCOEF;
                        if (temp as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                            temp = -(temp as ::core::ffi::c_int) as JCOEF;
                            temp = (temp as ::core::ffi::c_int
                                + (qval as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                                as JCOEF;
                            if temp as ::core::ffi::c_int >= qval as ::core::ffi::c_int {
                                temp = (temp as ::core::ffi::c_int / qval as ::core::ffi::c_int)
                                    as JCOEF;
                            } else {
                                temp = 0 as JCOEF;
                            }
                            temp = -(temp as ::core::ffi::c_int) as JCOEF;
                        } else {
                            temp = (temp as ::core::ffi::c_int
                                + (qval as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                                as JCOEF;
                            if temp as ::core::ffi::c_int >= qval as ::core::ffi::c_int {
                                temp = (temp as ::core::ffi::c_int / qval as ::core::ffi::c_int)
                                    as JCOEF;
                            } else {
                                temp = 0 as JCOEF;
                            }
                        }
                        *ptr.offset(k as isize) = temp;
                    }
                    k += 1;
                }
                blk_x = blk_x.wrapping_add(1);
            }
            offset_y += 1;
        }
        blk_y = blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
    }
}
unsafe extern "C" fn largest_common_denominator(mut a: JCOEF, mut b: JCOEF) -> JCOEF {
    let mut c: JCOEF = 0;
    loop {
        c = (a as ::core::ffi::c_int % b as ::core::ffi::c_int) as JCOEF;
        a = b;
        b = c;
        if !(c != 0) {
            break;
        }
    }
    return a;
}
unsafe extern "C" fn adjust_quant(
    mut srcinfo: j_decompress_ptr,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dropinfo: j_decompress_ptr,
    mut drop_coef_arrays: *mut jvirt_barray_ptr,
    mut trim: boolean,
    mut dstinfo: j_compress_ptr,
) {
    let mut compptr1: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut compptr2: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut qtblptr1: *mut JQUANT_TBL = ::core::ptr::null_mut::<JQUANT_TBL>();
    let mut qtblptr2: *mut JQUANT_TBL = ::core::ptr::null_mut::<JQUANT_TBL>();
    let mut qtblptr3: *mut JQUANT_TBL = ::core::ptr::null_mut::<JQUANT_TBL>();
    let mut ci: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components && ci < (*dropinfo).num_components {
        compptr1 = (*srcinfo).comp_info.offset(ci as isize);
        compptr2 = (*dropinfo).comp_info.offset(ci as isize);
        qtblptr1 = (*compptr1).quant_table;
        qtblptr2 = (*compptr2).quant_table;
        k = 0 as ::core::ffi::c_int;
        while k < DCTSIZE2 {
            if (*qtblptr1).quantval[k as usize] as ::core::ffi::c_int
                != (*qtblptr2).quantval[k as usize] as ::core::ffi::c_int
            {
                if trim != 0 {
                    requant_comp(
                        dropinfo,
                        compptr2,
                        *drop_coef_arrays.offset(ci as isize),
                        qtblptr1,
                    );
                } else {
                    qtblptr3 = (*dstinfo).quant_tbl_ptrs[(*compptr1).quant_tbl_no as usize];
                    k = 0 as ::core::ffi::c_int;
                    while k < DCTSIZE2 {
                        if (*qtblptr1).quantval[k as usize] as ::core::ffi::c_int
                            != (*qtblptr2).quantval[k as usize] as ::core::ffi::c_int
                        {
                            (*qtblptr3).quantval[k as usize] = largest_common_denominator(
                                (*qtblptr1).quantval[k as usize] as JCOEF,
                                (*qtblptr2).quantval[k as usize] as JCOEF,
                            )
                                as UINT16;
                        }
                        k += 1;
                    }
                    dequant_comp(
                        srcinfo,
                        compptr1,
                        *src_coef_arrays.offset(ci as isize),
                        qtblptr3,
                    );
                    dequant_comp(
                        dropinfo,
                        compptr2,
                        *drop_coef_arrays.offset(ci as isize),
                        qtblptr3,
                    );
                }
                break;
            } else {
                k += 1;
            }
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_drop(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dropinfo: j_decompress_ptr,
    mut drop_coef_arrays: *mut jvirt_barray_ptr,
    mut drop_width: JDIMENSION,
    mut drop_height: JDIMENSION,
) {
    let mut comp_width: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut blk_y: JDIMENSION = 0;
    let mut x_drop_blocks: JDIMENSION = 0;
    let mut y_drop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = drop_width.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        comp_height = drop_height.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        x_drop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_drop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        blk_y = 0 as JDIMENSION;
        while blk_y < comp_height {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                blk_y.wrapping_add(y_drop_blocks),
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            if ci < (*dropinfo).num_components {
                src_buffer = Some(
                    (*(*dropinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    dropinfo as j_common_ptr,
                    *drop_coef_arrays.offset(ci as isize),
                    blk_y,
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                );
                offset_y = 0 as ::core::ffi::c_int;
                while offset_y < (*compptr).v_samp_factor {
                    jcopy_block_row(
                        *src_buffer.offset(offset_y as isize),
                        (*dst_buffer.offset(offset_y as isize)).offset(x_drop_blocks as isize),
                        comp_width,
                    );
                    offset_y += 1;
                }
            } else {
                offset_y = 0 as ::core::ffi::c_int;
                while offset_y < (*compptr).v_samp_factor {
                    memset(
                        (*dst_buffer.offset(offset_y as isize)).offset(x_drop_blocks as isize)
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (comp_width as size_t)
                            .wrapping_mul(::core::mem::size_of::<JBLOCK>() as size_t),
                    );
                    offset_y += 1;
                }
            }
            blk_y = blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_crop(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        dst_blk_y = 0 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            src_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                dst_blk_y.wrapping_add(y_crop_blocks),
                (*compptr).v_samp_factor as JDIMENSION,
                FALSE,
            );
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                jcopy_block_row(
                    (*src_buffer.offset(offset_y as isize)).offset(x_crop_blocks as isize),
                    *dst_buffer.offset(offset_y as isize),
                    (*compptr).width_in_blocks,
                );
                offset_y += 1;
            }
            dst_blk_y = dst_blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_crop_ext_zero(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut MCU_cols: JDIMENSION = 0;
    let mut MCU_rows: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    MCU_cols = (*srcinfo).output_width.wrapping_div(
        ((*dstinfo).max_h_samp_factor * (*dstinfo).min_DCT_h_scaled_size) as JDIMENSION,
    );
    MCU_rows = (*srcinfo).output_height.wrapping_div(
        ((*dstinfo).max_v_samp_factor * (*dstinfo).min_DCT_v_scaled_size) as JDIMENSION,
    );
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        comp_height = MCU_rows.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        let mut current_block_30: u64;
        dst_blk_y = 0 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            if (*dstinfo).jpeg_height > (*srcinfo).output_height {
                if dst_blk_y < y_crop_blocks || dst_blk_y >= y_crop_blocks.wrapping_add(comp_height)
                {
                    offset_y = 0 as ::core::ffi::c_int;
                    while offset_y < (*compptr).v_samp_factor {
                        memset(
                            *dst_buffer.offset(offset_y as isize) as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ((*compptr).width_in_blocks as size_t)
                                .wrapping_mul(::core::mem::size_of::<JBLOCK>() as size_t),
                        );
                        offset_y += 1;
                    }
                    current_block_30 = 8515828400728868193;
                } else {
                    src_buffer = Some(
                        (*(*srcinfo).mem)
                            .access_virt_barray
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        srcinfo as j_common_ptr,
                        *src_coef_arrays.offset(ci as isize),
                        dst_blk_y.wrapping_sub(y_crop_blocks),
                        (*compptr).v_samp_factor as JDIMENSION,
                        FALSE,
                    );
                    current_block_30 = 6057473163062296781;
                }
            } else {
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    dst_blk_y.wrapping_add(y_crop_blocks),
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                );
                current_block_30 = 6057473163062296781;
            }
            match current_block_30 {
                6057473163062296781 => {
                    offset_y = 0 as ::core::ffi::c_int;
                    while offset_y < (*compptr).v_samp_factor {
                        if (*dstinfo).jpeg_width > (*srcinfo).output_width {
                            if x_crop_blocks > 0 as JDIMENSION {
                                memset(
                                    *dst_buffer.offset(offset_y as isize)
                                        as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    (x_crop_blocks as size_t).wrapping_mul(::core::mem::size_of::<
                                        JBLOCK,
                                    >(
                                    )
                                        as size_t),
                                );
                            }
                            jcopy_block_row(
                                *src_buffer.offset(offset_y as isize),
                                (*dst_buffer.offset(offset_y as isize))
                                    .offset(x_crop_blocks as isize),
                                comp_width,
                            );
                            if (*compptr).width_in_blocks > x_crop_blocks.wrapping_add(comp_width) {
                                memset(
                                    (*dst_buffer.offset(offset_y as isize))
                                        .offset(x_crop_blocks as isize)
                                        .offset(comp_width as isize)
                                        as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    ((*compptr)
                                        .width_in_blocks
                                        .wrapping_sub(x_crop_blocks)
                                        .wrapping_sub(comp_width)
                                        as size_t)
                                        .wrapping_mul(::core::mem::size_of::<JBLOCK>() as size_t),
                                );
                            }
                        } else {
                            jcopy_block_row(
                                (*src_buffer.offset(offset_y as isize))
                                    .offset(x_crop_blocks as isize),
                                *dst_buffer.offset(offset_y as isize),
                                (*compptr).width_in_blocks,
                            );
                        }
                        offset_y += 1;
                    }
                }
                _ => {}
            }
            dst_blk_y = dst_blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_crop_ext_flat(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut MCU_cols: JDIMENSION = 0;
    let mut MCU_rows: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut dc: JCOEF = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    MCU_cols = (*srcinfo).output_width.wrapping_div(
        ((*dstinfo).max_h_samp_factor * (*dstinfo).min_DCT_h_scaled_size) as JDIMENSION,
    );
    MCU_rows = (*srcinfo).output_height.wrapping_div(
        ((*dstinfo).max_v_samp_factor * (*dstinfo).min_DCT_v_scaled_size) as JDIMENSION,
    );
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        comp_height = MCU_rows.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        let mut current_block_36: u64;
        dst_blk_y = 0 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            if (*dstinfo).jpeg_height > (*srcinfo).output_height {
                if dst_blk_y < y_crop_blocks || dst_blk_y >= y_crop_blocks.wrapping_add(comp_height)
                {
                    offset_y = 0 as ::core::ffi::c_int;
                    while offset_y < (*compptr).v_samp_factor {
                        memset(
                            *dst_buffer.offset(offset_y as isize) as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ((*compptr).width_in_blocks as size_t)
                                .wrapping_mul(::core::mem::size_of::<JBLOCK>() as size_t),
                        );
                        offset_y += 1;
                    }
                    current_block_36 = 5720623009719927633;
                } else {
                    src_buffer = Some(
                        (*(*srcinfo).mem)
                            .access_virt_barray
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        srcinfo as j_common_ptr,
                        *src_coef_arrays.offset(ci as isize),
                        dst_blk_y.wrapping_sub(y_crop_blocks),
                        (*compptr).v_samp_factor as JDIMENSION,
                        FALSE,
                    );
                    current_block_36 = 13242334135786603907;
                }
            } else {
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    dst_blk_y.wrapping_add(y_crop_blocks),
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                );
                current_block_36 = 13242334135786603907;
            }
            match current_block_36 {
                13242334135786603907 => {
                    offset_y = 0 as ::core::ffi::c_int;
                    while offset_y < (*compptr).v_samp_factor {
                        if x_crop_blocks > 0 as JDIMENSION {
                            memset(
                                *dst_buffer.offset(offset_y as isize) as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                (x_crop_blocks as size_t)
                                    .wrapping_mul(::core::mem::size_of::<JBLOCK>() as size_t),
                            );
                            dc = (*(*src_buffer.offset(offset_y as isize))
                                .offset(0 as ::core::ffi::c_int as isize))
                                [0 as ::core::ffi::c_int as usize];
                            dst_blk_x = 0 as JDIMENSION;
                            while dst_blk_x < x_crop_blocks {
                                (*(*dst_buffer.offset(offset_y as isize))
                                    .offset(dst_blk_x as isize))
                                    [0 as ::core::ffi::c_int as usize] = dc;
                                dst_blk_x = dst_blk_x.wrapping_add(1);
                            }
                        }
                        jcopy_block_row(
                            *src_buffer.offset(offset_y as isize),
                            (*dst_buffer.offset(offset_y as isize)).offset(x_crop_blocks as isize),
                            comp_width,
                        );
                        if (*compptr).width_in_blocks > x_crop_blocks.wrapping_add(comp_width) {
                            memset(
                                (*dst_buffer.offset(offset_y as isize))
                                    .offset(x_crop_blocks as isize)
                                    .offset(comp_width as isize)
                                    as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ((*compptr)
                                    .width_in_blocks
                                    .wrapping_sub(x_crop_blocks)
                                    .wrapping_sub(comp_width)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<JBLOCK>() as size_t),
                            );
                            dc = (*(*src_buffer.offset(offset_y as isize))
                                .offset(comp_width.wrapping_sub(1 as JDIMENSION) as isize))
                                [0 as ::core::ffi::c_int as usize];
                            dst_blk_x = x_crop_blocks.wrapping_add(comp_width);
                            while dst_blk_x < (*compptr).width_in_blocks {
                                (*(*dst_buffer.offset(offset_y as isize))
                                    .offset(dst_blk_x as isize))
                                    [0 as ::core::ffi::c_int as usize] = dc;
                                dst_blk_x = dst_blk_x.wrapping_add(1);
                            }
                        }
                        offset_y += 1;
                    }
                }
                _ => {}
            }
            dst_blk_y = dst_blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_crop_ext_reflect(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut MCU_cols: JDIMENSION = 0;
    let mut MCU_rows: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut src_blk_x: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut src_row_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut dst_row_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut src_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut dst_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    MCU_cols = (*srcinfo).output_width.wrapping_div(
        ((*dstinfo).max_h_samp_factor * (*dstinfo).min_DCT_h_scaled_size) as JDIMENSION,
    );
    MCU_rows = (*srcinfo).output_height.wrapping_div(
        ((*dstinfo).max_v_samp_factor * (*dstinfo).min_DCT_v_scaled_size) as JDIMENSION,
    );
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        comp_height = MCU_rows.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        let mut current_block_54: u64;
        dst_blk_y = 0 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            if (*dstinfo).jpeg_height > (*srcinfo).output_height {
                if dst_blk_y < y_crop_blocks || dst_blk_y >= y_crop_blocks.wrapping_add(comp_height)
                {
                    offset_y = 0 as ::core::ffi::c_int;
                    while offset_y < (*compptr).v_samp_factor {
                        memset(
                            *dst_buffer.offset(offset_y as isize) as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ((*compptr).width_in_blocks as size_t)
                                .wrapping_mul(::core::mem::size_of::<JBLOCK>() as size_t),
                        );
                        offset_y += 1;
                    }
                    current_block_54 = 13183875560443969876;
                } else {
                    src_buffer = Some(
                        (*(*srcinfo).mem)
                            .access_virt_barray
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        srcinfo as j_common_ptr,
                        *src_coef_arrays.offset(ci as isize),
                        dst_blk_y.wrapping_sub(y_crop_blocks),
                        (*compptr).v_samp_factor as JDIMENSION,
                        FALSE,
                    );
                    current_block_54 = 12124785117276362961;
                }
            } else {
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    dst_blk_y.wrapping_add(y_crop_blocks),
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                );
                current_block_54 = 12124785117276362961;
            }
            match current_block_54 {
                12124785117276362961 => {
                    offset_y = 0 as ::core::ffi::c_int;
                    while offset_y < (*compptr).v_samp_factor {
                        jcopy_block_row(
                            *src_buffer.offset(offset_y as isize),
                            (*dst_buffer.offset(offset_y as isize)).offset(x_crop_blocks as isize),
                            comp_width,
                        );
                        if x_crop_blocks > 0 as JDIMENSION {
                            dst_row_ptr = (*dst_buffer.offset(offset_y as isize))
                                .offset(x_crop_blocks as isize);
                            dst_blk_x = x_crop_blocks;
                            while dst_blk_x > 0 as JDIMENSION {
                                src_row_ptr = dst_row_ptr;
                                src_blk_x = comp_width;
                                while src_blk_x > 0 as JDIMENSION && dst_blk_x > 0 as JDIMENSION {
                                    dst_row_ptr = dst_row_ptr.offset(-1);
                                    dst_ptr = &raw mut *dst_row_ptr as *mut JCOEF as JCOEFPTR;
                                    let fresh40 = src_row_ptr;
                                    src_row_ptr = src_row_ptr.offset(1);
                                    src_ptr = &raw mut *fresh40 as *mut JCOEF as JCOEFPTR;
                                    k = 0 as ::core::ffi::c_int;
                                    while k < DCTSIZE2 {
                                        let fresh41 = src_ptr;
                                        src_ptr = src_ptr.offset(1);
                                        let fresh42 = dst_ptr;
                                        dst_ptr = dst_ptr.offset(1);
                                        *fresh42 = *fresh41;
                                        let fresh43 = src_ptr;
                                        src_ptr = src_ptr.offset(1);
                                        let fresh44 = dst_ptr;
                                        dst_ptr = dst_ptr.offset(1);
                                        *fresh44 = -(*fresh43 as ::core::ffi::c_int) as JCOEF;
                                        k += 2 as ::core::ffi::c_int;
                                    }
                                    src_blk_x = src_blk_x.wrapping_sub(1);
                                    dst_blk_x = dst_blk_x.wrapping_sub(1);
                                }
                            }
                        }
                        if (*compptr).width_in_blocks > x_crop_blocks.wrapping_add(comp_width) {
                            dst_row_ptr = (*dst_buffer.offset(offset_y as isize))
                                .offset(x_crop_blocks as isize)
                                .offset(comp_width as isize);
                            dst_blk_x = (*compptr)
                                .width_in_blocks
                                .wrapping_sub(x_crop_blocks)
                                .wrapping_sub(comp_width);
                            while dst_blk_x > 0 as JDIMENSION {
                                src_row_ptr = dst_row_ptr;
                                src_blk_x = comp_width;
                                while src_blk_x > 0 as JDIMENSION && dst_blk_x > 0 as JDIMENSION {
                                    let fresh45 = dst_row_ptr;
                                    dst_row_ptr = dst_row_ptr.offset(1);
                                    dst_ptr = &raw mut *fresh45 as *mut JCOEF as JCOEFPTR;
                                    src_row_ptr = src_row_ptr.offset(-1);
                                    src_ptr = &raw mut *src_row_ptr as *mut JCOEF as JCOEFPTR;
                                    k = 0 as ::core::ffi::c_int;
                                    while k < DCTSIZE2 {
                                        let fresh46 = src_ptr;
                                        src_ptr = src_ptr.offset(1);
                                        let fresh47 = dst_ptr;
                                        dst_ptr = dst_ptr.offset(1);
                                        *fresh47 = *fresh46;
                                        let fresh48 = src_ptr;
                                        src_ptr = src_ptr.offset(1);
                                        let fresh49 = dst_ptr;
                                        dst_ptr = dst_ptr.offset(1);
                                        *fresh49 = -(*fresh48 as ::core::ffi::c_int) as JCOEF;
                                        k += 2 as ::core::ffi::c_int;
                                    }
                                    src_blk_x = src_blk_x.wrapping_sub(1);
                                    dst_blk_x = dst_blk_x.wrapping_sub(1);
                                }
                            }
                        }
                        offset_y += 1;
                    }
                }
                _ => {}
            }
            dst_blk_y = dst_blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_wipe(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut drop_width: JDIMENSION,
    mut drop_height: JDIMENSION,
) {
    let mut x_wipe_blocks: JDIMENSION = 0;
    let mut wipe_width: JDIMENSION = 0;
    let mut y_wipe_blocks: JDIMENSION = 0;
    let mut wipe_bottom: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        x_wipe_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        wipe_width = drop_width.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_wipe_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        wipe_bottom = drop_height
            .wrapping_mul((*compptr).v_samp_factor as JDIMENSION)
            .wrapping_add(y_wipe_blocks);
        while y_wipe_blocks < wipe_bottom {
            buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                y_wipe_blocks,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                memset(
                    (*buffer.offset(offset_y as isize)).offset(x_wipe_blocks as isize)
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (wipe_width as size_t).wrapping_mul(::core::mem::size_of::<JBLOCK>() as size_t),
                );
                offset_y += 1;
            }
            y_wipe_blocks = y_wipe_blocks.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_flatten(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut drop_width: JDIMENSION,
    mut drop_height: JDIMENSION,
) {
    let mut x_wipe_blocks: JDIMENSION = 0;
    let mut wipe_width: JDIMENSION = 0;
    let mut wipe_right: JDIMENSION = 0;
    let mut y_wipe_blocks: JDIMENSION = 0;
    let mut wipe_bottom: JDIMENSION = 0;
    let mut blk_x: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut dc_left_value: ::core::ffi::c_int = 0;
    let mut dc_right_value: ::core::ffi::c_int = 0;
    let mut average: ::core::ffi::c_int = 0;
    let mut buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        x_wipe_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        wipe_width = drop_width.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        wipe_right = wipe_width.wrapping_add(x_wipe_blocks);
        y_wipe_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        wipe_bottom = drop_height
            .wrapping_mul((*compptr).v_samp_factor as JDIMENSION)
            .wrapping_add(y_wipe_blocks);
        while y_wipe_blocks < wipe_bottom {
            buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                y_wipe_blocks,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            let mut current_block_23: u64;
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                memset(
                    (*buffer.offset(offset_y as isize)).offset(x_wipe_blocks as isize)
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (wipe_width as size_t).wrapping_mul(::core::mem::size_of::<JBLOCK>() as size_t),
                );
                if x_wipe_blocks > 0 as JDIMENSION {
                    dc_left_value = (*(*buffer.offset(offset_y as isize))
                        .offset(x_wipe_blocks.wrapping_sub(1 as JDIMENSION) as isize))
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                    if wipe_right < (*compptr).width_in_blocks {
                        dc_right_value = (*(*buffer.offset(offset_y as isize))
                            .offset(wipe_right as isize))
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                        average = dc_left_value + dc_right_value >> 1 as ::core::ffi::c_int;
                    } else {
                        average = dc_left_value;
                    }
                    current_block_23 = 224731115979188411;
                } else if wipe_right < (*compptr).width_in_blocks {
                    average = (*(*buffer.offset(offset_y as isize)).offset(wipe_right as isize))
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                    current_block_23 = 224731115979188411;
                } else {
                    current_block_23 = 3640593987805443782;
                }
                match current_block_23 {
                    224731115979188411 => {
                        blk_x = x_wipe_blocks;
                        while blk_x < wipe_right {
                            (*(*buffer.offset(offset_y as isize)).offset(blk_x as isize))
                                [0 as ::core::ffi::c_int as usize] = average as JCOEF;
                            blk_x = blk_x.wrapping_add(1);
                        }
                    }
                    _ => {}
                }
                offset_y += 1;
            }
            y_wipe_blocks = y_wipe_blocks.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_reflect(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut drop_width: JDIMENSION,
    mut drop_height: JDIMENSION,
) {
    let mut x_wipe_blocks: JDIMENSION = 0;
    let mut wipe_width: JDIMENSION = 0;
    let mut y_wipe_blocks: JDIMENSION = 0;
    let mut wipe_bottom: JDIMENSION = 0;
    let mut src_blk_x: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut src_row_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut dst_row_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut src_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut dst_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        x_wipe_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        wipe_width = drop_width.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        wipe_bottom = drop_height.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        y_wipe_blocks = 0 as JDIMENSION;
        while y_wipe_blocks < wipe_bottom {
            buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                y_wipe_blocks,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                if x_wipe_blocks > 0 as JDIMENSION {
                    dst_row_ptr =
                        (*buffer.offset(offset_y as isize)).offset(x_wipe_blocks as isize);
                    dst_blk_x = wipe_width;
                    while dst_blk_x > 0 as JDIMENSION {
                        src_row_ptr = dst_row_ptr;
                        src_blk_x = x_wipe_blocks;
                        while src_blk_x > 0 as JDIMENSION && dst_blk_x > 0 as JDIMENSION {
                            let fresh2 = dst_row_ptr;
                            dst_row_ptr = dst_row_ptr.offset(1);
                            dst_ptr = &raw mut *fresh2 as *mut JCOEF as JCOEFPTR;
                            src_row_ptr = src_row_ptr.offset(-1);
                            src_ptr = &raw mut *src_row_ptr as *mut JCOEF as JCOEFPTR;
                            k = 0 as ::core::ffi::c_int;
                            while k < DCTSIZE2 {
                                let fresh3 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh4 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh4 = *fresh3;
                                let fresh5 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh6 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh6 = -(*fresh5 as ::core::ffi::c_int) as JCOEF;
                                k += 2 as ::core::ffi::c_int;
                            }
                            src_blk_x = src_blk_x.wrapping_sub(1);
                            dst_blk_x = dst_blk_x.wrapping_sub(1);
                        }
                    }
                } else if (*compptr).width_in_blocks > x_wipe_blocks.wrapping_add(wipe_width) {
                    dst_row_ptr = (*buffer.offset(offset_y as isize))
                        .offset(x_wipe_blocks as isize)
                        .offset(wipe_width as isize);
                    dst_blk_x = wipe_width;
                    while dst_blk_x > 0 as JDIMENSION {
                        src_row_ptr = dst_row_ptr;
                        src_blk_x = (*compptr)
                            .width_in_blocks
                            .wrapping_sub(x_wipe_blocks)
                            .wrapping_sub(wipe_width);
                        while src_blk_x > 0 as JDIMENSION && dst_blk_x > 0 as JDIMENSION {
                            dst_row_ptr = dst_row_ptr.offset(-1);
                            dst_ptr = &raw mut *dst_row_ptr as *mut JCOEF as JCOEFPTR;
                            let fresh7 = src_row_ptr;
                            src_row_ptr = src_row_ptr.offset(1);
                            src_ptr = &raw mut *fresh7 as *mut JCOEF as JCOEFPTR;
                            k = 0 as ::core::ffi::c_int;
                            while k < DCTSIZE2 {
                                let fresh8 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh9 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh9 = *fresh8;
                                let fresh10 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh11 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh11 = -(*fresh10 as ::core::ffi::c_int) as JCOEF;
                                k += 2 as ::core::ffi::c_int;
                            }
                            src_blk_x = src_blk_x.wrapping_sub(1);
                            dst_blk_x = dst_blk_x.wrapping_sub(1);
                        }
                    }
                } else {
                    memset(
                        (*buffer.offset(offset_y as isize)).offset(x_wipe_blocks as isize)
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (wipe_width as size_t)
                            .wrapping_mul(::core::mem::size_of::<JBLOCK>() as size_t),
                    );
                }
                offset_y += 1;
            }
            y_wipe_blocks = y_wipe_blocks.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_flip_h_no_crop(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut MCU_cols: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut blk_x: JDIMENSION = 0;
    let mut blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut ptr1: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut ptr2: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut temp1: JCOEF = 0;
    let mut temp2: JCOEF = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    MCU_cols = (*srcinfo).output_width.wrapping_div(
        ((*dstinfo).max_h_samp_factor * (*dstinfo).min_DCT_h_scaled_size) as JDIMENSION,
    );
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        blk_y = 0 as JDIMENSION;
        while blk_y < (*compptr).height_in_blocks {
            buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                blk_x = 0 as JDIMENSION;
                while blk_x.wrapping_mul(2 as JDIMENSION) < comp_width {
                    ptr1 = &raw mut *(*buffer.offset(offset_y as isize)).offset(blk_x as isize)
                        as *mut JCOEF as JCOEFPTR;
                    ptr2 =
                        &raw mut *(*buffer.offset(offset_y as isize))
                            .offset(comp_width.wrapping_sub(blk_x).wrapping_sub(1 as JDIMENSION)
                                as isize) as *mut JCOEF as JCOEFPTR;
                    k = 0 as ::core::ffi::c_int;
                    while k < DCTSIZE2 {
                        temp1 = *ptr1;
                        temp2 = *ptr2;
                        let fresh32 = ptr1;
                        ptr1 = ptr1.offset(1);
                        *fresh32 = temp2;
                        let fresh33 = ptr2;
                        ptr2 = ptr2.offset(1);
                        *fresh33 = temp1;
                        temp1 = *ptr1;
                        temp2 = *ptr2;
                        let fresh34 = ptr1;
                        ptr1 = ptr1.offset(1);
                        *fresh34 = -(temp2 as ::core::ffi::c_int) as JCOEF;
                        let fresh35 = ptr2;
                        ptr2 = ptr2.offset(1);
                        *fresh35 = -(temp1 as ::core::ffi::c_int) as JCOEF;
                        k += 2 as ::core::ffi::c_int;
                    }
                    blk_x = blk_x.wrapping_add(1);
                }
                if x_crop_blocks > 0 as JDIMENSION {
                    blk_x = 0 as JDIMENSION;
                    while blk_x < (*compptr).width_in_blocks {
                        jcopy_block_row(
                            (*buffer.offset(offset_y as isize))
                                .offset(blk_x as isize)
                                .offset(x_crop_blocks as isize),
                            (*buffer.offset(offset_y as isize)).offset(blk_x as isize),
                            1 as ::core::ffi::c_int as JDIMENSION,
                        );
                        blk_x = blk_x.wrapping_add(1);
                    }
                }
                offset_y += 1;
            }
            blk_y = blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_flip_h(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut MCU_cols: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut src_row_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut dst_row_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut src_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut dst_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    MCU_cols = (*srcinfo).output_width.wrapping_div(
        ((*dstinfo).max_h_samp_factor * (*dstinfo).min_DCT_h_scaled_size) as JDIMENSION,
    );
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        dst_blk_y = 0 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            src_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *src_coef_arrays.offset(ci as isize),
                dst_blk_y.wrapping_add(y_crop_blocks),
                (*compptr).v_samp_factor as JDIMENSION,
                FALSE,
            );
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                dst_row_ptr = *dst_buffer.offset(offset_y as isize);
                src_row_ptr = *src_buffer.offset(offset_y as isize);
                dst_blk_x = 0 as JDIMENSION;
                while dst_blk_x < (*compptr).width_in_blocks {
                    if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                        dst_ptr = &raw mut *dst_row_ptr.offset(dst_blk_x as isize) as *mut JCOEF
                            as JCOEFPTR;
                        src_ptr = &raw mut *src_row_ptr.offset(
                            comp_width
                                .wrapping_sub(x_crop_blocks)
                                .wrapping_sub(dst_blk_x)
                                .wrapping_sub(1 as JDIMENSION) as isize,
                        ) as *mut JCOEF as JCOEFPTR;
                        k = 0 as ::core::ffi::c_int;
                        while k < DCTSIZE2 {
                            let fresh36 = src_ptr;
                            src_ptr = src_ptr.offset(1);
                            let fresh37 = dst_ptr;
                            dst_ptr = dst_ptr.offset(1);
                            *fresh37 = *fresh36;
                            let fresh38 = src_ptr;
                            src_ptr = src_ptr.offset(1);
                            let fresh39 = dst_ptr;
                            dst_ptr = dst_ptr.offset(1);
                            *fresh39 = -(*fresh38 as ::core::ffi::c_int) as JCOEF;
                            k += 2 as ::core::ffi::c_int;
                        }
                    } else {
                        jcopy_block_row(
                            src_row_ptr
                                .offset(dst_blk_x as isize)
                                .offset(x_crop_blocks as isize),
                            dst_row_ptr.offset(dst_blk_x as isize),
                            1 as ::core::ffi::c_int as JDIMENSION,
                        );
                    }
                    dst_blk_x = dst_blk_x.wrapping_add(1);
                }
                offset_y += 1;
            }
            dst_blk_y = dst_blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_flip_v(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut MCU_rows: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut src_row_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut dst_row_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut src_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut dst_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    MCU_rows = (*srcinfo).output_height.wrapping_div(
        ((*dstinfo).max_v_samp_factor * (*dstinfo).min_DCT_v_scaled_size) as JDIMENSION,
    );
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_height = MCU_rows.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        dst_blk_y = 0 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    comp_height
                        .wrapping_sub(y_crop_blocks)
                        .wrapping_sub(dst_blk_y)
                        .wrapping_sub((*compptr).v_samp_factor as JDIMENSION),
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                );
            } else {
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    dst_blk_y.wrapping_add(y_crop_blocks),
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                );
            }
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                    dst_row_ptr = *dst_buffer.offset(offset_y as isize);
                    src_row_ptr = *src_buffer.offset(
                        ((*compptr).v_samp_factor - offset_y - 1 as ::core::ffi::c_int) as isize,
                    );
                    src_row_ptr = src_row_ptr.offset(x_crop_blocks as isize);
                    dst_blk_x = 0 as JDIMENSION;
                    while dst_blk_x < (*compptr).width_in_blocks {
                        dst_ptr = &raw mut *dst_row_ptr.offset(dst_blk_x as isize) as *mut JCOEF
                            as JCOEFPTR;
                        src_ptr = &raw mut *src_row_ptr.offset(dst_blk_x as isize) as *mut JCOEF
                            as JCOEFPTR;
                        i = 0 as ::core::ffi::c_int;
                        while i < DCTSIZE {
                            j = 0 as ::core::ffi::c_int;
                            while j < DCTSIZE {
                                let fresh28 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh29 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh29 = *fresh28;
                                j += 1;
                            }
                            j = 0 as ::core::ffi::c_int;
                            while j < DCTSIZE {
                                let fresh30 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh31 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh31 = -(*fresh30 as ::core::ffi::c_int) as JCOEF;
                                j += 1;
                            }
                            i += 2 as ::core::ffi::c_int;
                        }
                        dst_blk_x = dst_blk_x.wrapping_add(1);
                    }
                } else {
                    jcopy_block_row(
                        (*src_buffer.offset(offset_y as isize)).offset(x_crop_blocks as isize),
                        *dst_buffer.offset(offset_y as isize),
                        (*compptr).width_in_blocks,
                    );
                }
                offset_y += 1;
            }
            dst_blk_y = dst_blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_transpose(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut offset_x: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut src_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut dst_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        dst_blk_y = 0 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                dst_blk_x = 0 as JDIMENSION;
                while dst_blk_x < (*compptr).width_in_blocks {
                    src_buffer = Some(
                        (*(*srcinfo).mem)
                            .access_virt_barray
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        srcinfo as j_common_ptr,
                        *src_coef_arrays.offset(ci as isize),
                        dst_blk_x.wrapping_add(x_crop_blocks),
                        (*compptr).h_samp_factor as JDIMENSION,
                        FALSE,
                    );
                    offset_x = 0 as ::core::ffi::c_int;
                    while offset_x < (*compptr).h_samp_factor {
                        dst_ptr = &raw mut *(*dst_buffer.offset(offset_y as isize))
                            .offset(dst_blk_x.wrapping_add(offset_x as JDIMENSION) as isize)
                            as *mut JCOEF as JCOEFPTR;
                        src_ptr = &raw mut *(*src_buffer.offset(offset_x as isize)).offset(
                            dst_blk_y
                                .wrapping_add(offset_y as JDIMENSION)
                                .wrapping_add(y_crop_blocks) as isize,
                        ) as *mut JCOEF as JCOEFPTR;
                        i = 0 as ::core::ffi::c_int;
                        while i < DCTSIZE {
                            j = 0 as ::core::ffi::c_int;
                            while j < DCTSIZE {
                                *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                    *src_ptr.offset((i * DCTSIZE + j) as isize);
                                j += 1;
                            }
                            i += 1;
                        }
                        offset_x += 1;
                    }
                    dst_blk_x = dst_blk_x.wrapping_add((*compptr).h_samp_factor as JDIMENSION);
                }
                offset_y += 1;
            }
            dst_blk_y = dst_blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_rot_90(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut MCU_cols: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut offset_x: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut src_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut dst_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    MCU_cols = (*srcinfo).output_height.wrapping_div(
        ((*dstinfo).max_h_samp_factor * (*dstinfo).min_DCT_h_scaled_size) as JDIMENSION,
    );
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        dst_blk_y = 0 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                dst_blk_x = 0 as JDIMENSION;
                while dst_blk_x < (*compptr).width_in_blocks {
                    if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            comp_width
                                .wrapping_sub(x_crop_blocks)
                                .wrapping_sub(dst_blk_x)
                                .wrapping_sub((*compptr).h_samp_factor as JDIMENSION),
                            (*compptr).h_samp_factor as JDIMENSION,
                            FALSE,
                        );
                    } else {
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            dst_blk_x.wrapping_add(x_crop_blocks),
                            (*compptr).h_samp_factor as JDIMENSION,
                            FALSE,
                        );
                    }
                    offset_x = 0 as ::core::ffi::c_int;
                    while offset_x < (*compptr).h_samp_factor {
                        dst_ptr = &raw mut *(*dst_buffer.offset(offset_y as isize))
                            .offset(dst_blk_x.wrapping_add(offset_x as JDIMENSION) as isize)
                            as *mut JCOEF as JCOEFPTR;
                        if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                            src_ptr = &raw mut *(*src_buffer.offset(
                                ((*compptr).h_samp_factor - offset_x - 1 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .offset(
                                dst_blk_y
                                    .wrapping_add(offset_y as JDIMENSION)
                                    .wrapping_add(y_crop_blocks)
                                    as isize,
                            ) as *mut JCOEF as JCOEFPTR;
                            i = 0 as ::core::ffi::c_int;
                            while i < DCTSIZE {
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1;
                                }
                                i += 1;
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) = -(*src_ptr
                                        .offset((i * DCTSIZE + j) as isize)
                                        as ::core::ffi::c_int)
                                        as JCOEF;
                                    j += 1;
                                }
                                i += 1;
                            }
                        } else {
                            src_ptr = &raw mut *(*src_buffer.offset(offset_x as isize)).offset(
                                dst_blk_y
                                    .wrapping_add(offset_y as JDIMENSION)
                                    .wrapping_add(y_crop_blocks)
                                    as isize,
                            ) as *mut JCOEF as JCOEFPTR;
                            i = 0 as ::core::ffi::c_int;
                            while i < DCTSIZE {
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1;
                                }
                                i += 1;
                            }
                        }
                        offset_x += 1;
                    }
                    dst_blk_x = dst_blk_x.wrapping_add((*compptr).h_samp_factor as JDIMENSION);
                }
                offset_y += 1;
            }
            dst_blk_y = dst_blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_rot_270(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut MCU_rows: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut offset_x: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut src_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut dst_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    MCU_rows = (*srcinfo).output_width.wrapping_div(
        ((*dstinfo).max_v_samp_factor * (*dstinfo).min_DCT_v_scaled_size) as JDIMENSION,
    );
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_height = MCU_rows.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        dst_blk_y = 0 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                dst_blk_x = 0 as JDIMENSION;
                while dst_blk_x < (*compptr).width_in_blocks {
                    src_buffer = Some(
                        (*(*srcinfo).mem)
                            .access_virt_barray
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        srcinfo as j_common_ptr,
                        *src_coef_arrays.offset(ci as isize),
                        dst_blk_x.wrapping_add(x_crop_blocks),
                        (*compptr).h_samp_factor as JDIMENSION,
                        FALSE,
                    );
                    offset_x = 0 as ::core::ffi::c_int;
                    while offset_x < (*compptr).h_samp_factor {
                        dst_ptr = &raw mut *(*dst_buffer.offset(offset_y as isize))
                            .offset(dst_blk_x.wrapping_add(offset_x as JDIMENSION) as isize)
                            as *mut JCOEF as JCOEFPTR;
                        if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                            src_ptr = &raw mut *(*src_buffer.offset(offset_x as isize)).offset(
                                comp_height
                                    .wrapping_sub(y_crop_blocks)
                                    .wrapping_sub(dst_blk_y)
                                    .wrapping_sub(offset_y as JDIMENSION)
                                    .wrapping_sub(1 as JDIMENSION)
                                    as isize,
                            ) as *mut JCOEF as JCOEFPTR;
                            i = 0 as ::core::ffi::c_int;
                            while i < DCTSIZE {
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1;
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) = -(*src_ptr
                                        .offset((i * DCTSIZE + j) as isize)
                                        as ::core::ffi::c_int)
                                        as JCOEF;
                                    j += 1;
                                }
                                i += 1;
                            }
                        } else {
                            src_ptr = &raw mut *(*src_buffer.offset(offset_x as isize)).offset(
                                dst_blk_y
                                    .wrapping_add(offset_y as JDIMENSION)
                                    .wrapping_add(y_crop_blocks)
                                    as isize,
                            ) as *mut JCOEF as JCOEFPTR;
                            i = 0 as ::core::ffi::c_int;
                            while i < DCTSIZE {
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1;
                                }
                                i += 1;
                            }
                        }
                        offset_x += 1;
                    }
                    dst_blk_x = dst_blk_x.wrapping_add((*compptr).h_samp_factor as JDIMENSION);
                }
                offset_y += 1;
            }
            dst_blk_y = dst_blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_rot_180(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut MCU_cols: JDIMENSION = 0;
    let mut MCU_rows: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut src_row_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut dst_row_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut src_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut dst_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    MCU_cols = (*srcinfo).output_width.wrapping_div(
        ((*dstinfo).max_h_samp_factor * (*dstinfo).min_DCT_h_scaled_size) as JDIMENSION,
    );
    MCU_rows = (*srcinfo).output_height.wrapping_div(
        ((*dstinfo).max_v_samp_factor * (*dstinfo).min_DCT_v_scaled_size) as JDIMENSION,
    );
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        comp_height = MCU_rows.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        dst_blk_y = 0 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    comp_height
                        .wrapping_sub(y_crop_blocks)
                        .wrapping_sub(dst_blk_y)
                        .wrapping_sub((*compptr).v_samp_factor as JDIMENSION),
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                );
            } else {
                src_buffer = Some(
                    (*(*srcinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    srcinfo as j_common_ptr,
                    *src_coef_arrays.offset(ci as isize),
                    dst_blk_y.wrapping_add(y_crop_blocks),
                    (*compptr).v_samp_factor as JDIMENSION,
                    FALSE,
                );
            }
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                dst_row_ptr = *dst_buffer.offset(offset_y as isize);
                if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                    src_row_ptr = *src_buffer.offset(
                        ((*compptr).v_samp_factor - offset_y - 1 as ::core::ffi::c_int) as isize,
                    );
                    dst_blk_x = 0 as JDIMENSION;
                    while dst_blk_x < (*compptr).width_in_blocks {
                        dst_ptr = &raw mut *dst_row_ptr.offset(dst_blk_x as isize) as *mut JCOEF
                            as JCOEFPTR;
                        if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                            src_ptr = &raw mut *src_row_ptr.offset(
                                comp_width
                                    .wrapping_sub(x_crop_blocks)
                                    .wrapping_sub(dst_blk_x)
                                    .wrapping_sub(1 as JDIMENSION)
                                    as isize,
                            ) as *mut JCOEF as JCOEFPTR;
                            i = 0 as ::core::ffi::c_int;
                            while i < DCTSIZE {
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    let fresh12 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh13 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh13 = *fresh12;
                                    let fresh14 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh15 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh15 = -(*fresh14 as ::core::ffi::c_int) as JCOEF;
                                    j += 2 as ::core::ffi::c_int;
                                }
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    let fresh16 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh17 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh17 = -(*fresh16 as ::core::ffi::c_int) as JCOEF;
                                    let fresh18 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh19 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh19 = *fresh18;
                                    j += 2 as ::core::ffi::c_int;
                                }
                                i += 2 as ::core::ffi::c_int;
                            }
                        } else {
                            src_ptr = &raw mut *src_row_ptr
                                .offset(x_crop_blocks.wrapping_add(dst_blk_x) as isize)
                                as *mut JCOEF as JCOEFPTR;
                            i = 0 as ::core::ffi::c_int;
                            while i < DCTSIZE {
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    let fresh20 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh21 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh21 = *fresh20;
                                    j += 1;
                                }
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    let fresh22 = src_ptr;
                                    src_ptr = src_ptr.offset(1);
                                    let fresh23 = dst_ptr;
                                    dst_ptr = dst_ptr.offset(1);
                                    *fresh23 = -(*fresh22 as ::core::ffi::c_int) as JCOEF;
                                    j += 1;
                                }
                                i += 2 as ::core::ffi::c_int;
                            }
                        }
                        dst_blk_x = dst_blk_x.wrapping_add(1);
                    }
                } else {
                    src_row_ptr = *src_buffer.offset(offset_y as isize);
                    dst_blk_x = 0 as JDIMENSION;
                    while dst_blk_x < (*compptr).width_in_blocks {
                        if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                            dst_ptr = &raw mut *dst_row_ptr.offset(dst_blk_x as isize) as *mut JCOEF
                                as JCOEFPTR;
                            src_ptr = &raw mut *src_row_ptr.offset(
                                comp_width
                                    .wrapping_sub(x_crop_blocks)
                                    .wrapping_sub(dst_blk_x)
                                    .wrapping_sub(1 as JDIMENSION)
                                    as isize,
                            ) as *mut JCOEF as JCOEFPTR;
                            i = 0 as ::core::ffi::c_int;
                            while i < DCTSIZE2 {
                                let fresh24 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh25 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh25 = *fresh24;
                                let fresh26 = src_ptr;
                                src_ptr = src_ptr.offset(1);
                                let fresh27 = dst_ptr;
                                dst_ptr = dst_ptr.offset(1);
                                *fresh27 = -(*fresh26 as ::core::ffi::c_int) as JCOEF;
                                i += 2 as ::core::ffi::c_int;
                            }
                        } else {
                            jcopy_block_row(
                                src_row_ptr
                                    .offset(dst_blk_x as isize)
                                    .offset(x_crop_blocks as isize),
                                dst_row_ptr.offset(dst_blk_x as isize),
                                1 as ::core::ffi::c_int as JDIMENSION,
                            );
                        }
                        dst_blk_x = dst_blk_x.wrapping_add(1);
                    }
                }
                offset_y += 1;
            }
            dst_blk_y = dst_blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn do_transverse(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut x_crop_offset: JDIMENSION,
    mut y_crop_offset: JDIMENSION,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut dst_coef_arrays: *mut jvirt_barray_ptr,
) {
    let mut MCU_cols: JDIMENSION = 0;
    let mut MCU_rows: JDIMENSION = 0;
    let mut comp_width: JDIMENSION = 0;
    let mut comp_height: JDIMENSION = 0;
    let mut dst_blk_x: JDIMENSION = 0;
    let mut dst_blk_y: JDIMENSION = 0;
    let mut x_crop_blocks: JDIMENSION = 0;
    let mut y_crop_blocks: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut offset_x: ::core::ffi::c_int = 0;
    let mut offset_y: ::core::ffi::c_int = 0;
    let mut src_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut dst_buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut src_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut dst_ptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    MCU_cols = (*srcinfo).output_height.wrapping_div(
        ((*dstinfo).max_h_samp_factor * (*dstinfo).min_DCT_h_scaled_size) as JDIMENSION,
    );
    MCU_rows = (*srcinfo).output_width.wrapping_div(
        ((*dstinfo).max_v_samp_factor * (*dstinfo).min_DCT_v_scaled_size) as JDIMENSION,
    );
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        comp_width = MCU_cols.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        comp_height = MCU_rows.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        x_crop_blocks = x_crop_offset.wrapping_mul((*compptr).h_samp_factor as JDIMENSION);
        y_crop_blocks = y_crop_offset.wrapping_mul((*compptr).v_samp_factor as JDIMENSION);
        dst_blk_y = 0 as JDIMENSION;
        while dst_blk_y < (*compptr).height_in_blocks {
            dst_buffer = Some(
                (*(*srcinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                *dst_coef_arrays.offset(ci as isize),
                dst_blk_y,
                (*compptr).v_samp_factor as JDIMENSION,
                TRUE,
            );
            offset_y = 0 as ::core::ffi::c_int;
            while offset_y < (*compptr).v_samp_factor {
                dst_blk_x = 0 as JDIMENSION;
                while dst_blk_x < (*compptr).width_in_blocks {
                    if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            comp_width
                                .wrapping_sub(x_crop_blocks)
                                .wrapping_sub(dst_blk_x)
                                .wrapping_sub((*compptr).h_samp_factor as JDIMENSION),
                            (*compptr).h_samp_factor as JDIMENSION,
                            FALSE,
                        );
                    } else {
                        src_buffer = Some(
                            (*(*srcinfo).mem)
                                .access_virt_barray
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            srcinfo as j_common_ptr,
                            *src_coef_arrays.offset(ci as isize),
                            dst_blk_x.wrapping_add(x_crop_blocks),
                            (*compptr).h_samp_factor as JDIMENSION,
                            FALSE,
                        );
                    }
                    offset_x = 0 as ::core::ffi::c_int;
                    while offset_x < (*compptr).h_samp_factor {
                        dst_ptr = &raw mut *(*dst_buffer.offset(offset_y as isize))
                            .offset(dst_blk_x.wrapping_add(offset_x as JDIMENSION) as isize)
                            as *mut JCOEF as JCOEFPTR;
                        if y_crop_blocks.wrapping_add(dst_blk_y) < comp_height {
                            if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                                src_ptr = &raw mut *(*src_buffer.offset(
                                    ((*compptr).h_samp_factor - offset_x - 1 as ::core::ffi::c_int)
                                        as isize,
                                ))
                                .offset(
                                    comp_height
                                        .wrapping_sub(y_crop_blocks)
                                        .wrapping_sub(dst_blk_y)
                                        .wrapping_sub(offset_y as JDIMENSION)
                                        .wrapping_sub(1 as JDIMENSION)
                                        as isize,
                                ) as *mut JCOEF
                                    as JCOEFPTR;
                                i = 0 as ::core::ffi::c_int;
                                while i < DCTSIZE {
                                    j = 0 as ::core::ffi::c_int;
                                    while j < DCTSIZE {
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                            *src_ptr.offset((i * DCTSIZE + j) as isize);
                                        j += 1;
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) = -(*src_ptr
                                            .offset((i * DCTSIZE + j) as isize)
                                            as ::core::ffi::c_int)
                                            as JCOEF;
                                        j += 1;
                                    }
                                    i += 1;
                                    j = 0 as ::core::ffi::c_int;
                                    while j < DCTSIZE {
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) = -(*src_ptr
                                            .offset((i * DCTSIZE + j) as isize)
                                            as ::core::ffi::c_int)
                                            as JCOEF;
                                        j += 1;
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                            *src_ptr.offset((i * DCTSIZE + j) as isize);
                                        j += 1;
                                    }
                                    i += 1;
                                }
                            } else {
                                src_ptr = &raw mut *(*src_buffer.offset(offset_x as isize)).offset(
                                    comp_height
                                        .wrapping_sub(y_crop_blocks)
                                        .wrapping_sub(dst_blk_y)
                                        .wrapping_sub(offset_y as JDIMENSION)
                                        .wrapping_sub(1 as JDIMENSION)
                                        as isize,
                                ) as *mut JCOEF
                                    as JCOEFPTR;
                                i = 0 as ::core::ffi::c_int;
                                while i < DCTSIZE {
                                    j = 0 as ::core::ffi::c_int;
                                    while j < DCTSIZE {
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                            *src_ptr.offset((i * DCTSIZE + j) as isize);
                                        j += 1;
                                        *dst_ptr.offset((j * DCTSIZE + i) as isize) = -(*src_ptr
                                            .offset((i * DCTSIZE + j) as isize)
                                            as ::core::ffi::c_int)
                                            as JCOEF;
                                        j += 1;
                                    }
                                    i += 1;
                                }
                            }
                        } else if x_crop_blocks.wrapping_add(dst_blk_x) < comp_width {
                            src_ptr = &raw mut *(*src_buffer.offset(
                                ((*compptr).h_samp_factor - offset_x - 1 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .offset(
                                dst_blk_y
                                    .wrapping_add(offset_y as JDIMENSION)
                                    .wrapping_add(y_crop_blocks)
                                    as isize,
                            ) as *mut JCOEF as JCOEFPTR;
                            i = 0 as ::core::ffi::c_int;
                            while i < DCTSIZE {
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1;
                                }
                                i += 1;
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) = -(*src_ptr
                                        .offset((i * DCTSIZE + j) as isize)
                                        as ::core::ffi::c_int)
                                        as JCOEF;
                                    j += 1;
                                }
                                i += 1;
                            }
                        } else {
                            src_ptr = &raw mut *(*src_buffer.offset(offset_x as isize)).offset(
                                dst_blk_y
                                    .wrapping_add(offset_y as JDIMENSION)
                                    .wrapping_add(y_crop_blocks)
                                    as isize,
                            ) as *mut JCOEF as JCOEFPTR;
                            i = 0 as ::core::ffi::c_int;
                            while i < DCTSIZE {
                                j = 0 as ::core::ffi::c_int;
                                while j < DCTSIZE {
                                    *dst_ptr.offset((j * DCTSIZE + i) as isize) =
                                        *src_ptr.offset((i * DCTSIZE + j) as isize);
                                    j += 1;
                                }
                                i += 1;
                            }
                        }
                        offset_x += 1;
                    }
                    dst_blk_x = dst_blk_x.wrapping_add((*compptr).h_samp_factor as JDIMENSION);
                }
                offset_y += 1;
            }
            dst_blk_y = dst_blk_y.wrapping_add((*compptr).v_samp_factor as JDIMENSION);
        }
        ci += 1;
    }
}
unsafe extern "C" fn jt_read_integer(
    mut strptr: *mut *const ::core::ffi::c_char,
    mut result: *mut JDIMENSION,
) -> boolean {
    let mut ptr: *const ::core::ffi::c_char = *strptr;
    let mut val: JDIMENSION = 0 as JDIMENSION;
    while *(*__ctype_b_loc()).offset(*ptr as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        val = val
            .wrapping_mul(10 as JDIMENSION)
            .wrapping_add((*ptr as ::core::ffi::c_int - '0' as i32) as JDIMENSION);
        ptr = ptr.offset(1);
    }
    *result = val;
    if ptr == *strptr {
        return FALSE;
    }
    *strptr = ptr;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn jtransform_parse_crop_spec(
    mut info: *mut jpeg_transform_info,
    mut spec: *const ::core::ffi::c_char,
) -> boolean {
    (*info).crop = FALSE as boolean;
    (*info).crop_width_set = JCROP_UNSET;
    (*info).crop_height_set = JCROP_UNSET;
    (*info).crop_xoffset_set = JCROP_UNSET;
    (*info).crop_yoffset_set = JCROP_UNSET;
    if *(*__ctype_b_loc()).offset(*spec as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        if jt_read_integer(&raw mut spec, &raw mut (*info).crop_width) == 0 {
            return FALSE;
        }
        if *spec as ::core::ffi::c_int == 'f' as i32 || *spec as ::core::ffi::c_int == 'F' as i32 {
            spec = spec.offset(1);
            (*info).crop_width_set = JCROP_FORCE;
        } else if *spec as ::core::ffi::c_int == 'r' as i32
            || *spec as ::core::ffi::c_int == 'R' as i32
        {
            spec = spec.offset(1);
            (*info).crop_width_set = JCROP_REFLECT;
        } else {
            (*info).crop_width_set = JCROP_POS;
        }
    }
    if *spec as ::core::ffi::c_int == 'x' as i32 || *spec as ::core::ffi::c_int == 'X' as i32 {
        spec = spec.offset(1);
        if jt_read_integer(&raw mut spec, &raw mut (*info).crop_height) == 0 {
            return FALSE;
        }
        if *spec as ::core::ffi::c_int == 'f' as i32 || *spec as ::core::ffi::c_int == 'F' as i32 {
            spec = spec.offset(1);
            (*info).crop_height_set = JCROP_FORCE;
        } else if *spec as ::core::ffi::c_int == 'r' as i32
            || *spec as ::core::ffi::c_int == 'R' as i32
        {
            spec = spec.offset(1);
            (*info).crop_height_set = JCROP_REFLECT;
        } else {
            (*info).crop_height_set = JCROP_POS;
        }
    }
    if *spec as ::core::ffi::c_int == '+' as i32 || *spec as ::core::ffi::c_int == '-' as i32 {
        (*info).crop_xoffset_set = (if *spec as ::core::ffi::c_int == '-' as i32 {
            JCROP_NEG as ::core::ffi::c_int
        } else {
            JCROP_POS as ::core::ffi::c_int
        }) as JCROP_CODE;
        spec = spec.offset(1);
        if jt_read_integer(&raw mut spec, &raw mut (*info).crop_xoffset) == 0 {
            return FALSE;
        }
    }
    if *spec as ::core::ffi::c_int == '+' as i32 || *spec as ::core::ffi::c_int == '-' as i32 {
        (*info).crop_yoffset_set = (if *spec as ::core::ffi::c_int == '-' as i32 {
            JCROP_NEG as ::core::ffi::c_int
        } else {
            JCROP_POS as ::core::ffi::c_int
        }) as JCROP_CODE;
        spec = spec.offset(1);
        if jt_read_integer(&raw mut spec, &raw mut (*info).crop_yoffset) == 0 {
            return FALSE;
        }
    }
    if *spec as ::core::ffi::c_int != '\0' as i32 {
        return FALSE;
    }
    (*info).crop = TRUE as boolean;
    return TRUE;
}
unsafe extern "C" fn trim_right_edge(
    mut info: *mut jpeg_transform_info,
    mut full_width: JDIMENSION,
) {
    let mut MCU_cols: JDIMENSION = 0;
    MCU_cols = (*info)
        .output_width
        .wrapping_div((*info).iMCU_sample_width as JDIMENSION);
    if MCU_cols > 0 as JDIMENSION
        && (*info).x_crop_offset.wrapping_add(MCU_cols)
            == full_width.wrapping_div((*info).iMCU_sample_width as JDIMENSION)
    {
        (*info).output_width = MCU_cols.wrapping_mul((*info).iMCU_sample_width as JDIMENSION);
    }
}
unsafe extern "C" fn trim_bottom_edge(
    mut info: *mut jpeg_transform_info,
    mut full_height: JDIMENSION,
) {
    let mut MCU_rows: JDIMENSION = 0;
    MCU_rows = (*info)
        .output_height
        .wrapping_div((*info).iMCU_sample_height as JDIMENSION);
    if MCU_rows > 0 as JDIMENSION
        && (*info).y_crop_offset.wrapping_add(MCU_rows)
            == full_height.wrapping_div((*info).iMCU_sample_height as JDIMENSION)
    {
        (*info).output_height = MCU_rows.wrapping_mul((*info).iMCU_sample_height as JDIMENSION);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jtransform_request_workspace(
    mut srcinfo: j_decompress_ptr,
    mut info: *mut jpeg_transform_info,
) -> boolean {
    let mut coef_arrays: *mut jvirt_barray_ptr = ::core::ptr::null_mut::<jvirt_barray_ptr>();
    let mut need_workspace: boolean = 0;
    let mut transpose_it: boolean = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut xoffset: JDIMENSION = 0;
    let mut yoffset: JDIMENSION = 0;
    let mut dtemp: JDIMENSION = 0;
    let mut width_in_iMCUs: JDIMENSION = 0;
    let mut height_in_iMCUs: JDIMENSION = 0;
    let mut width_in_blocks: JDIMENSION = 0;
    let mut height_in_blocks: JDIMENSION = 0;
    let mut itemp: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut h_samp_factor: ::core::ffi::c_int = 0;
    let mut v_samp_factor: ::core::ffi::c_int = 0;
    if (*info).force_grayscale != 0
        && (*srcinfo).jpeg_color_space as ::core::ffi::c_uint
            == JCS_YCbCr as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*srcinfo).num_components == 3 as ::core::ffi::c_int
    {
        (*info).num_components = 1 as ::core::ffi::c_int;
    } else {
        (*info).num_components = (*srcinfo).num_components;
    }
    jpeg_core_output_dimensions(srcinfo);
    if (*info).perfect != 0 {
        if (*info).num_components == 1 as ::core::ffi::c_int {
            if jtransform_perfect_transform(
                (*srcinfo).output_width,
                (*srcinfo).output_height,
                (*srcinfo).min_DCT_h_scaled_size,
                (*srcinfo).min_DCT_v_scaled_size,
                (*info).transform,
            ) == 0
            {
                return FALSE;
            }
        } else if jtransform_perfect_transform(
            (*srcinfo).output_width,
            (*srcinfo).output_height,
            (*srcinfo).max_h_samp_factor * (*srcinfo).min_DCT_h_scaled_size,
            (*srcinfo).max_v_samp_factor * (*srcinfo).min_DCT_v_scaled_size,
            (*info).transform,
        ) == 0
        {
            return FALSE;
        }
    }
    match (*info).transform as ::core::ffi::c_uint {
        3 | 4 | 5 | 7 => {
            (*info).output_width = (*srcinfo).output_height;
            (*info).output_height = (*srcinfo).output_width;
            if (*info).num_components == 1 as ::core::ffi::c_int {
                (*info).iMCU_sample_width = (*srcinfo).min_DCT_v_scaled_size;
                (*info).iMCU_sample_height = (*srcinfo).min_DCT_h_scaled_size;
            } else {
                (*info).iMCU_sample_width =
                    (*srcinfo).max_v_samp_factor * (*srcinfo).min_DCT_v_scaled_size;
                (*info).iMCU_sample_height =
                    (*srcinfo).max_h_samp_factor * (*srcinfo).min_DCT_h_scaled_size;
            }
        }
        _ => {
            (*info).output_width = (*srcinfo).output_width;
            (*info).output_height = (*srcinfo).output_height;
            if (*info).num_components == 1 as ::core::ffi::c_int {
                (*info).iMCU_sample_width = (*srcinfo).min_DCT_h_scaled_size;
                (*info).iMCU_sample_height = (*srcinfo).min_DCT_v_scaled_size;
            } else {
                (*info).iMCU_sample_width =
                    (*srcinfo).max_h_samp_factor * (*srcinfo).min_DCT_h_scaled_size;
                (*info).iMCU_sample_height =
                    (*srcinfo).max_v_samp_factor * (*srcinfo).min_DCT_v_scaled_size;
            }
        }
    }
    if (*info).crop != 0 {
        if (*info).crop_xoffset_set as ::core::ffi::c_uint
            == JCROP_UNSET as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*info).crop_xoffset = 0 as JDIMENSION;
        }
        if (*info).crop_yoffset_set as ::core::ffi::c_uint
            == JCROP_UNSET as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*info).crop_yoffset = 0 as JDIMENSION;
        }
        if (*info).crop_width_set as ::core::ffi::c_uint
            == JCROP_UNSET as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (*info).crop_xoffset >= (*info).output_width {
                (*(*srcinfo).err).msg_code = JERR_BAD_CROP_SPEC as ::core::ffi::c_int;
                Some(
                    (*(*srcinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(srcinfo as j_common_ptr);
            }
            (*info).crop_width = (*info).output_width.wrapping_sub((*info).crop_xoffset);
        } else if (*info).crop_width > (*info).output_width {
            if (*info).transform as ::core::ffi::c_uint
                != JXFORM_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*info).crop_xoffset >= (*info).crop_width
                || (*info).crop_xoffset > (*info).crop_width.wrapping_sub((*info).output_width)
            {
                (*(*srcinfo).err).msg_code = JERR_BAD_CROP_SPEC as ::core::ffi::c_int;
                Some(
                    (*(*srcinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(srcinfo as j_common_ptr);
            }
        } else if (*info).crop_xoffset >= (*info).output_width
            || (*info).crop_width <= 0 as JDIMENSION
            || (*info).crop_xoffset > (*info).output_width.wrapping_sub((*info).crop_width)
        {
            (*(*srcinfo).err).msg_code = JERR_BAD_CROP_SPEC as ::core::ffi::c_int;
            Some(
                (*(*srcinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(srcinfo as j_common_ptr);
        }
        if (*info).crop_height_set as ::core::ffi::c_uint
            == JCROP_UNSET as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (*info).crop_yoffset >= (*info).output_height {
                (*(*srcinfo).err).msg_code = JERR_BAD_CROP_SPEC as ::core::ffi::c_int;
                Some(
                    (*(*srcinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(srcinfo as j_common_ptr);
            }
            (*info).crop_height = (*info).output_height.wrapping_sub((*info).crop_yoffset);
        } else if (*info).crop_height > (*info).output_height {
            if (*info).transform as ::core::ffi::c_uint
                != JXFORM_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*info).crop_yoffset >= (*info).crop_height
                || (*info).crop_yoffset > (*info).crop_height.wrapping_sub((*info).output_height)
            {
                (*(*srcinfo).err).msg_code = JERR_BAD_CROP_SPEC as ::core::ffi::c_int;
                Some(
                    (*(*srcinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(srcinfo as j_common_ptr);
            }
        } else if (*info).crop_yoffset >= (*info).output_height
            || (*info).crop_height <= 0 as JDIMENSION
            || (*info).crop_yoffset > (*info).output_height.wrapping_sub((*info).crop_height)
        {
            (*(*srcinfo).err).msg_code = JERR_BAD_CROP_SPEC as ::core::ffi::c_int;
            Some(
                (*(*srcinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(srcinfo as j_common_ptr);
        }
        if (*info).crop_xoffset_set as ::core::ffi::c_uint
            != JCROP_NEG as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xoffset = (*info).crop_xoffset;
        } else if (*info).crop_width > (*info).output_width {
            xoffset = (*info)
                .crop_width
                .wrapping_sub((*info).output_width)
                .wrapping_sub((*info).crop_xoffset);
        } else {
            xoffset = (*info)
                .output_width
                .wrapping_sub((*info).crop_width)
                .wrapping_sub((*info).crop_xoffset);
        }
        if (*info).crop_yoffset_set as ::core::ffi::c_uint
            != JCROP_NEG as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            yoffset = (*info).crop_yoffset;
        } else if (*info).crop_height > (*info).output_height {
            yoffset = (*info)
                .crop_height
                .wrapping_sub((*info).output_height)
                .wrapping_sub((*info).crop_yoffset);
        } else {
            yoffset = (*info)
                .output_height
                .wrapping_sub((*info).crop_height)
                .wrapping_sub((*info).crop_yoffset);
        }
        match (*info).transform as ::core::ffi::c_uint {
            9 => {
                itemp = (*info).iMCU_sample_width;
                dtemp = ((itemp - 1 as ::core::ffi::c_int) as JDIMENSION).wrapping_sub(
                    xoffset
                        .wrapping_add(itemp as JDIMENSION)
                        .wrapping_sub(1 as JDIMENSION)
                        .wrapping_rem(itemp as JDIMENSION),
                );
                xoffset = xoffset.wrapping_add(dtemp);
                if (*info).crop_width <= dtemp {
                    (*info).drop_width = 0 as JDIMENSION;
                } else if xoffset.wrapping_add((*info).crop_width).wrapping_sub(dtemp)
                    == (*info).output_width
                {
                    (*info).drop_width = (*info)
                        .crop_width
                        .wrapping_sub(dtemp)
                        .wrapping_add(itemp as JDIMENSION)
                        .wrapping_sub(1 as JDIMENSION)
                        .wrapping_div(itemp as JDIMENSION);
                } else {
                    (*info).drop_width = (*info)
                        .crop_width
                        .wrapping_sub(dtemp)
                        .wrapping_div(itemp as JDIMENSION);
                }
                itemp = (*info).iMCU_sample_height;
                dtemp = ((itemp - 1 as ::core::ffi::c_int) as JDIMENSION).wrapping_sub(
                    yoffset
                        .wrapping_add(itemp as JDIMENSION)
                        .wrapping_sub(1 as JDIMENSION)
                        .wrapping_rem(itemp as JDIMENSION),
                );
                yoffset = yoffset.wrapping_add(dtemp);
                if (*info).crop_height <= dtemp {
                    (*info).drop_height = 0 as JDIMENSION;
                } else if yoffset
                    .wrapping_add((*info).crop_height)
                    .wrapping_sub(dtemp)
                    == (*info).output_height
                {
                    (*info).drop_height = (*info)
                        .crop_height
                        .wrapping_sub(dtemp)
                        .wrapping_add(itemp as JDIMENSION)
                        .wrapping_sub(1 as JDIMENSION)
                        .wrapping_div(itemp as JDIMENSION);
                } else {
                    (*info).drop_height = (*info)
                        .crop_height
                        .wrapping_sub(dtemp)
                        .wrapping_div(itemp as JDIMENSION);
                }
                if (*info).drop_width != 0 as JDIMENSION && (*info).drop_height != 0 as JDIMENSION {
                    ci = 0 as ::core::ffi::c_int;
                    while ci < (*info).num_components && ci < (*(*info).drop_ptr).num_components {
                        if (*(*(*info).drop_ptr).comp_info.offset(ci as isize)).h_samp_factor
                            * (*srcinfo).max_h_samp_factor
                            != (*(*srcinfo).comp_info.offset(ci as isize)).h_samp_factor
                                * (*(*info).drop_ptr).max_h_samp_factor
                        {
                            (*(*srcinfo).err).msg_code =
                                JERR_BAD_DROP_SAMPLING as ::core::ffi::c_int;
                            (*(*srcinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = ci;
                            (*(*srcinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] =
                                (*(*(*info).drop_ptr).comp_info.offset(ci as isize)).h_samp_factor;
                            (*(*srcinfo).err).msg_parm.i[2 as ::core::ffi::c_int as usize] =
                                (*(*info).drop_ptr).max_h_samp_factor;
                            (*(*srcinfo).err).msg_parm.i[3 as ::core::ffi::c_int as usize] =
                                (*(*srcinfo).comp_info.offset(ci as isize)).h_samp_factor;
                            (*(*srcinfo).err).msg_parm.i[4 as ::core::ffi::c_int as usize] =
                                (*srcinfo).max_h_samp_factor;
                            (*(*srcinfo).err).msg_parm.i[5 as ::core::ffi::c_int as usize] =
                                'h' as i32;
                            Some(
                                (*(*srcinfo).err)
                                    .error_exit
                                    .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                srcinfo as j_common_ptr
                            );
                        }
                        if (*(*(*info).drop_ptr).comp_info.offset(ci as isize)).v_samp_factor
                            * (*srcinfo).max_v_samp_factor
                            != (*(*srcinfo).comp_info.offset(ci as isize)).v_samp_factor
                                * (*(*info).drop_ptr).max_v_samp_factor
                        {
                            (*(*srcinfo).err).msg_code =
                                JERR_BAD_DROP_SAMPLING as ::core::ffi::c_int;
                            (*(*srcinfo).err).msg_parm.i[0 as ::core::ffi::c_int as usize] = ci;
                            (*(*srcinfo).err).msg_parm.i[1 as ::core::ffi::c_int as usize] =
                                (*(*(*info).drop_ptr).comp_info.offset(ci as isize)).v_samp_factor;
                            (*(*srcinfo).err).msg_parm.i[2 as ::core::ffi::c_int as usize] =
                                (*(*info).drop_ptr).max_v_samp_factor;
                            (*(*srcinfo).err).msg_parm.i[3 as ::core::ffi::c_int as usize] =
                                (*(*srcinfo).comp_info.offset(ci as isize)).v_samp_factor;
                            (*(*srcinfo).err).msg_parm.i[4 as ::core::ffi::c_int as usize] =
                                (*srcinfo).max_v_samp_factor;
                            (*(*srcinfo).err).msg_parm.i[5 as ::core::ffi::c_int as usize] =
                                'v' as i32;
                            Some(
                                (*(*srcinfo).err)
                                    .error_exit
                                    .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                srcinfo as j_common_ptr
                            );
                        }
                        ci += 1;
                    }
                }
            }
            8 => {
                (*info).drop_width = jdiv_round_up(
                    (*info)
                        .crop_width
                        .wrapping_add(xoffset.wrapping_rem((*info).iMCU_sample_width as JDIMENSION))
                        as ::core::ffi::c_long,
                    (*info).iMCU_sample_width as ::core::ffi::c_long,
                ) as JDIMENSION;
                (*info).drop_height = jdiv_round_up(
                    (*info).crop_height.wrapping_add(
                        yoffset.wrapping_rem((*info).iMCU_sample_height as JDIMENSION),
                    ) as ::core::ffi::c_long,
                    (*info).iMCU_sample_height as ::core::ffi::c_long,
                ) as JDIMENSION;
            }
            _ => {
                if (*info).crop_width_set as ::core::ffi::c_uint
                    == JCROP_FORCE as ::core::ffi::c_int as ::core::ffi::c_uint
                    || (*info).crop_width > (*info).output_width
                {
                    (*info).output_width = (*info).crop_width;
                } else {
                    (*info).output_width = (*info).crop_width.wrapping_add(
                        xoffset.wrapping_rem((*info).iMCU_sample_width as JDIMENSION),
                    );
                }
                if (*info).crop_height_set as ::core::ffi::c_uint
                    == JCROP_FORCE as ::core::ffi::c_int as ::core::ffi::c_uint
                    || (*info).crop_height > (*info).output_height
                {
                    (*info).output_height = (*info).crop_height;
                } else {
                    (*info).output_height = (*info).crop_height.wrapping_add(
                        yoffset.wrapping_rem((*info).iMCU_sample_height as JDIMENSION),
                    );
                }
            }
        }
        (*info).x_crop_offset = xoffset.wrapping_div((*info).iMCU_sample_width as JDIMENSION);
        (*info).y_crop_offset = yoffset.wrapping_div((*info).iMCU_sample_height as JDIMENSION);
    } else {
        (*info).x_crop_offset = 0 as JDIMENSION;
        (*info).y_crop_offset = 0 as JDIMENSION;
    }
    need_workspace = FALSE as boolean;
    transpose_it = FALSE as boolean;
    match (*info).transform as ::core::ffi::c_uint {
        0 => {
            if (*info).x_crop_offset != 0 as JDIMENSION
                || (*info).y_crop_offset != 0 as JDIMENSION
                || (*info).output_width > (*srcinfo).output_width
                || (*info).output_height > (*srcinfo).output_height
            {
                need_workspace = TRUE as boolean;
            }
        }
        1 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_width);
            }
            if (*info).y_crop_offset != 0 as JDIMENSION || (*info).slow_hflip != 0 {
                need_workspace = TRUE as boolean;
            }
        }
        2 => {
            if (*info).trim != 0 {
                trim_bottom_edge(info, (*srcinfo).output_height);
            }
            need_workspace = TRUE as boolean;
        }
        3 => {
            need_workspace = TRUE as boolean;
            transpose_it = TRUE as boolean;
        }
        4 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_height);
                trim_bottom_edge(info, (*srcinfo).output_width);
            }
            need_workspace = TRUE as boolean;
            transpose_it = TRUE as boolean;
        }
        5 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_height);
            }
            need_workspace = TRUE as boolean;
            transpose_it = TRUE as boolean;
        }
        6 => {
            if (*info).trim != 0 {
                trim_right_edge(info, (*srcinfo).output_width);
                trim_bottom_edge(info, (*srcinfo).output_height);
            }
            need_workspace = TRUE as boolean;
        }
        7 => {
            if (*info).trim != 0 {
                trim_bottom_edge(info, (*srcinfo).output_width);
            }
            need_workspace = TRUE as boolean;
            transpose_it = TRUE as boolean;
        }
        8 | 9 | _ => {}
    }
    if need_workspace != 0 {
        coef_arrays = Some(
            (*(*srcinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            srcinfo as j_common_ptr,
            JPOOL_IMAGE,
            (::core::mem::size_of::<jvirt_barray_ptr>() as size_t)
                .wrapping_mul((*info).num_components as size_t),
        ) as *mut jvirt_barray_ptr;
        width_in_iMCUs = jdiv_round_up(
            (*info).output_width as ::core::ffi::c_long,
            (*info).iMCU_sample_width as ::core::ffi::c_long,
        ) as JDIMENSION;
        height_in_iMCUs = jdiv_round_up(
            (*info).output_height as ::core::ffi::c_long,
            (*info).iMCU_sample_height as ::core::ffi::c_long,
        ) as JDIMENSION;
        ci = 0 as ::core::ffi::c_int;
        while ci < (*info).num_components {
            compptr = (*srcinfo).comp_info.offset(ci as isize);
            if (*info).num_components == 1 as ::core::ffi::c_int {
                v_samp_factor = 1 as ::core::ffi::c_int;
                h_samp_factor = v_samp_factor;
            } else if transpose_it != 0 {
                h_samp_factor = (*compptr).v_samp_factor;
                v_samp_factor = (*compptr).h_samp_factor;
            } else {
                h_samp_factor = (*compptr).h_samp_factor;
                v_samp_factor = (*compptr).v_samp_factor;
            }
            width_in_blocks = width_in_iMCUs.wrapping_mul(h_samp_factor as JDIMENSION);
            height_in_blocks = height_in_iMCUs.wrapping_mul(v_samp_factor as JDIMENSION);
            let ref mut fresh0 = *coef_arrays.offset(ci as isize);
            *fresh0 = Some(
                (*(*srcinfo).mem)
                    .request_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                srcinfo as j_common_ptr,
                JPOOL_IMAGE,
                FALSE,
                width_in_blocks,
                height_in_blocks,
                v_samp_factor as JDIMENSION,
            );
            ci += 1;
        }
        (*info).workspace_coef_arrays = coef_arrays;
    } else {
        (*info).workspace_coef_arrays = ::core::ptr::null_mut::<jvirt_barray_ptr>();
    }
    return TRUE;
}
unsafe extern "C" fn transpose_critical_parameters(mut dstinfo: j_compress_ptr) {
    let mut tblno: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut itemp: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut qtblptr: *mut JQUANT_TBL = ::core::ptr::null_mut::<JQUANT_TBL>();
    let mut jtemp: JDIMENSION = 0;
    let mut qtemp: UINT16 = 0;
    jtemp = (*dstinfo).image_width;
    (*dstinfo).image_width = (*dstinfo).image_height;
    (*dstinfo).image_height = jtemp;
    itemp = (*dstinfo).min_DCT_h_scaled_size;
    (*dstinfo).min_DCT_h_scaled_size = (*dstinfo).min_DCT_v_scaled_size;
    (*dstinfo).min_DCT_v_scaled_size = itemp;
    ci = 0 as ::core::ffi::c_int;
    while ci < (*dstinfo).num_components {
        compptr = (*dstinfo).comp_info.offset(ci as isize);
        itemp = (*compptr).h_samp_factor;
        (*compptr).h_samp_factor = (*compptr).v_samp_factor;
        (*compptr).v_samp_factor = itemp;
        ci += 1;
    }
    tblno = 0 as ::core::ffi::c_int;
    while tblno < NUM_QUANT_TBLS {
        qtblptr = (*dstinfo).quant_tbl_ptrs[tblno as usize];
        if !qtblptr.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < DCTSIZE {
                j = 0 as ::core::ffi::c_int;
                while j < i {
                    qtemp = (*qtblptr).quantval[(i * DCTSIZE + j) as usize];
                    (*qtblptr).quantval[(i * DCTSIZE + j) as usize] =
                        (*qtblptr).quantval[(j * DCTSIZE + i) as usize];
                    (*qtblptr).quantval[(j * DCTSIZE + i) as usize] = qtemp;
                    j += 1;
                }
                i += 1;
            }
        }
        tblno += 1;
    }
}
unsafe extern "C" fn adjust_exif_parameters(
    mut data: *mut JOCTET,
    mut length: ::core::ffi::c_uint,
    mut new_width: JDIMENSION,
    mut new_height: JDIMENSION,
) {
    let mut is_motorola: boolean = 0;
    let mut number_of_tags: ::core::ffi::c_uint = 0;
    let mut tagnum: ::core::ffi::c_uint = 0;
    let mut firstoffset: ::core::ffi::c_uint = 0;
    let mut offset: ::core::ffi::c_uint = 0;
    let mut new_value: JDIMENSION = 0;
    if length < 12 as ::core::ffi::c_uint {
        return;
    }
    if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0x49 as ::core::ffi::c_int
        && *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x49 as ::core::ffi::c_int
    {
        is_motorola = FALSE as boolean;
    } else if *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0x4d as ::core::ffi::c_int
        && *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x4d as ::core::ffi::c_int
    {
        is_motorola = TRUE as boolean;
    } else {
        return;
    }
    if is_motorola != 0 {
        if *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return;
        }
        if *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0x2a as ::core::ffi::c_int
        {
            return;
        }
    } else {
        if *data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return;
        }
        if *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0x2a as ::core::ffi::c_int
        {
            return;
        }
    }
    if is_motorola != 0 {
        if *data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return;
        }
        if *data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return;
        }
        firstoffset = *data.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        firstoffset <<= 8 as ::core::ffi::c_int;
        firstoffset = firstoffset
            .wrapping_add(*data.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    } else {
        if *data.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return;
        }
        if *data.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return;
        }
        firstoffset = *data.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
        firstoffset <<= 8 as ::core::ffi::c_int;
        firstoffset = firstoffset
            .wrapping_add(*data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint);
    }
    if firstoffset > length.wrapping_sub(2 as ::core::ffi::c_uint) {
        return;
    }
    if is_motorola != 0 {
        number_of_tags = *data.offset(firstoffset as isize) as ::core::ffi::c_uint;
        number_of_tags <<= 8 as ::core::ffi::c_int;
        number_of_tags = number_of_tags.wrapping_add(
            *data.offset(firstoffset.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                as ::core::ffi::c_uint,
        );
    } else {
        number_of_tags = *data.offset(firstoffset.wrapping_add(1 as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_uint;
        number_of_tags <<= 8 as ::core::ffi::c_int;
        number_of_tags =
            number_of_tags.wrapping_add(*data.offset(firstoffset as isize) as ::core::ffi::c_uint);
    }
    if number_of_tags == 0 as ::core::ffi::c_uint {
        return;
    }
    firstoffset = firstoffset.wrapping_add(2 as ::core::ffi::c_uint);
    loop {
        if firstoffset > length.wrapping_sub(12 as ::core::ffi::c_uint) {
            return;
        }
        if is_motorola != 0 {
            tagnum = *data.offset(firstoffset as isize) as ::core::ffi::c_uint;
            tagnum <<= 8 as ::core::ffi::c_int;
            tagnum = tagnum.wrapping_add(
                *data.offset(firstoffset.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                    as ::core::ffi::c_uint,
            );
        } else {
            tagnum = *data.offset(firstoffset.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                as ::core::ffi::c_uint;
            tagnum <<= 8 as ::core::ffi::c_int;
            tagnum = tagnum.wrapping_add(*data.offset(firstoffset as isize) as ::core::ffi::c_uint);
        }
        if tagnum == 0x8769 as ::core::ffi::c_uint {
            break;
        }
        number_of_tags = number_of_tags.wrapping_sub(1);
        if number_of_tags == 0 as ::core::ffi::c_uint {
            return;
        }
        firstoffset = firstoffset.wrapping_add(12 as ::core::ffi::c_uint);
    }
    if is_motorola != 0 {
        if *data.offset(firstoffset.wrapping_add(8 as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return;
        }
        if *data.offset(firstoffset.wrapping_add(9 as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return;
        }
        offset = *data.offset(firstoffset.wrapping_add(10 as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_uint;
        offset <<= 8 as ::core::ffi::c_int;
        offset = offset.wrapping_add(
            *data.offset(firstoffset.wrapping_add(11 as ::core::ffi::c_uint) as isize)
                as ::core::ffi::c_uint,
        );
    } else {
        if *data.offset(firstoffset.wrapping_add(11 as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return;
        }
        if *data.offset(firstoffset.wrapping_add(10 as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return;
        }
        offset = *data.offset(firstoffset.wrapping_add(9 as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_uint;
        offset <<= 8 as ::core::ffi::c_int;
        offset = offset.wrapping_add(
            *data.offset(firstoffset.wrapping_add(8 as ::core::ffi::c_uint) as isize)
                as ::core::ffi::c_uint,
        );
    }
    if offset > length.wrapping_sub(2 as ::core::ffi::c_uint) {
        return;
    }
    if is_motorola != 0 {
        number_of_tags = *data.offset(offset as isize) as ::core::ffi::c_uint;
        number_of_tags <<= 8 as ::core::ffi::c_int;
        number_of_tags = number_of_tags.wrapping_add(
            *data.offset(offset.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                as ::core::ffi::c_uint,
        );
    } else {
        number_of_tags = *data.offset(offset.wrapping_add(1 as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_uint;
        number_of_tags <<= 8 as ::core::ffi::c_int;
        number_of_tags =
            number_of_tags.wrapping_add(*data.offset(offset as isize) as ::core::ffi::c_uint);
    }
    if number_of_tags < 2 as ::core::ffi::c_uint {
        return;
    }
    offset = offset.wrapping_add(2 as ::core::ffi::c_uint);
    loop {
        if offset > length.wrapping_sub(12 as ::core::ffi::c_uint) {
            return;
        }
        if is_motorola != 0 {
            tagnum = *data.offset(offset as isize) as ::core::ffi::c_uint;
            tagnum <<= 8 as ::core::ffi::c_int;
            tagnum = tagnum.wrapping_add(
                *data.offset(offset.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                    as ::core::ffi::c_uint,
            );
        } else {
            tagnum = *data.offset(offset.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                as ::core::ffi::c_uint;
            tagnum <<= 8 as ::core::ffi::c_int;
            tagnum = tagnum.wrapping_add(*data.offset(offset as isize) as ::core::ffi::c_uint);
        }
        if tagnum == 0xa002 as ::core::ffi::c_uint || tagnum == 0xa003 as ::core::ffi::c_uint {
            if tagnum == 0xa002 as ::core::ffi::c_uint {
                new_value = new_width;
            } else {
                new_value = new_height;
            }
            if is_motorola != 0 {
                *data.offset(offset.wrapping_add(2 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
                *data.offset(offset.wrapping_add(3 as ::core::ffi::c_uint) as isize) = 4 as JOCTET;
                *data.offset(offset.wrapping_add(4 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
                *data.offset(offset.wrapping_add(5 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
                *data.offset(offset.wrapping_add(6 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
                *data.offset(offset.wrapping_add(7 as ::core::ffi::c_uint) as isize) = 1 as JOCTET;
                *data.offset(offset.wrapping_add(8 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
                *data.offset(offset.wrapping_add(9 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
                *data.offset(offset.wrapping_add(10 as ::core::ffi::c_uint) as isize) =
                    (new_value >> 8 as ::core::ffi::c_int & 0xff as JDIMENSION) as JOCTET;
                *data.offset(offset.wrapping_add(11 as ::core::ffi::c_uint) as isize) =
                    (new_value & 0xff as JDIMENSION) as JOCTET;
            } else {
                *data.offset(offset.wrapping_add(2 as ::core::ffi::c_uint) as isize) = 4 as JOCTET;
                *data.offset(offset.wrapping_add(3 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
                *data.offset(offset.wrapping_add(4 as ::core::ffi::c_uint) as isize) = 1 as JOCTET;
                *data.offset(offset.wrapping_add(5 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
                *data.offset(offset.wrapping_add(6 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
                *data.offset(offset.wrapping_add(7 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
                *data.offset(offset.wrapping_add(8 as ::core::ffi::c_uint) as isize) =
                    (new_value & 0xff as JDIMENSION) as JOCTET;
                *data.offset(offset.wrapping_add(9 as ::core::ffi::c_uint) as isize) =
                    (new_value >> 8 as ::core::ffi::c_int & 0xff as JDIMENSION) as JOCTET;
                *data.offset(offset.wrapping_add(10 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
                *data.offset(offset.wrapping_add(11 as ::core::ffi::c_uint) as isize) = 0 as JOCTET;
            }
        }
        offset = offset.wrapping_add(12 as ::core::ffi::c_uint);
        number_of_tags = number_of_tags.wrapping_sub(1);
        if !(number_of_tags != 0) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn jtransform_adjust_parameters(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut info: *mut jpeg_transform_info,
) -> *mut jvirt_barray_ptr {
    if (*info).force_grayscale != 0 {
        if ((*dstinfo).jpeg_color_space as ::core::ffi::c_uint
            == JCS_YCbCr as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*dstinfo).num_components == 3 as ::core::ffi::c_int
            || (*dstinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*dstinfo).num_components == 1 as ::core::ffi::c_int)
            && (*(*srcinfo)
                .comp_info
                .offset(0 as ::core::ffi::c_int as isize))
            .h_samp_factor
                == (*srcinfo).max_h_samp_factor
            && (*(*srcinfo)
                .comp_info
                .offset(0 as ::core::ffi::c_int as isize))
            .v_samp_factor
                == (*srcinfo).max_v_samp_factor
        {
            let mut sv_quant_tbl_no: ::core::ffi::c_int = (*(*dstinfo)
                .comp_info
                .offset(0 as ::core::ffi::c_int as isize))
            .quant_tbl_no;
            jpeg_set_colorspace(dstinfo, JCS_GRAYSCALE);
            (*(*dstinfo)
                .comp_info
                .offset(0 as ::core::ffi::c_int as isize))
            .quant_tbl_no = sv_quant_tbl_no;
        } else {
            (*(*dstinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as ::core::ffi::c_int;
            Some(
                (*(*dstinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(dstinfo as j_common_ptr);
        }
    } else if (*info).num_components == 1 as ::core::ffi::c_int {
        (*(*dstinfo)
            .comp_info
            .offset(0 as ::core::ffi::c_int as isize))
        .h_samp_factor = 1 as ::core::ffi::c_int;
        (*(*dstinfo)
            .comp_info
            .offset(0 as ::core::ffi::c_int as isize))
        .v_samp_factor = 1 as ::core::ffi::c_int;
    }
    (*dstinfo).jpeg_width = (*info).output_width;
    (*dstinfo).jpeg_height = (*info).output_height;
    match (*info).transform as ::core::ffi::c_uint {
        3 | 4 | 5 | 7 => {
            transpose_critical_parameters(dstinfo);
        }
        9 => {
            if (*info).drop_width != 0 as JDIMENSION && (*info).drop_height != 0 as JDIMENSION {
                adjust_quant(
                    srcinfo,
                    src_coef_arrays,
                    (*info).drop_ptr,
                    (*info).drop_coef_arrays,
                    (*info).trim,
                    dstinfo,
                );
            }
        }
        _ => {}
    }
    if !(*srcinfo).marker_list.is_null()
        && (*(*srcinfo).marker_list).marker as ::core::ffi::c_int
            == JPEG_APP0 + 1 as ::core::ffi::c_int
        && (*(*srcinfo).marker_list).data_length >= 6 as ::core::ffi::c_uint
        && *(*(*srcinfo).marker_list)
            .data
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x45 as ::core::ffi::c_int
        && *(*(*srcinfo).marker_list)
            .data
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x78 as ::core::ffi::c_int
        && *(*(*srcinfo).marker_list)
            .data
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x69 as ::core::ffi::c_int
        && *(*(*srcinfo).marker_list)
            .data
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x66 as ::core::ffi::c_int
        && *(*(*srcinfo).marker_list)
            .data
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        && *(*(*srcinfo).marker_list)
            .data
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        (*dstinfo).write_JFIF_header = FALSE as boolean;
        if (*dstinfo).jpeg_width != (*srcinfo).image_width
            || (*dstinfo).jpeg_height != (*srcinfo).image_height
        {
            adjust_exif_parameters(
                (*(*srcinfo).marker_list)
                    .data
                    .offset(6 as ::core::ffi::c_int as isize),
                (*(*srcinfo).marker_list)
                    .data_length
                    .wrapping_sub(6 as ::core::ffi::c_uint),
                (*dstinfo).jpeg_width,
                (*dstinfo).jpeg_height,
            );
        }
    }
    if !(*info).workspace_coef_arrays.is_null() {
        return (*info).workspace_coef_arrays;
    }
    return src_coef_arrays;
}
#[no_mangle]
pub unsafe extern "C" fn jtransform_execute_transform(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut src_coef_arrays: *mut jvirt_barray_ptr,
    mut info: *mut jpeg_transform_info,
) {
    let mut dst_coef_arrays: *mut jvirt_barray_ptr = (*info).workspace_coef_arrays;
    match (*info).transform as ::core::ffi::c_uint {
        0 => {
            if (*info).output_width > (*srcinfo).output_width
                || (*info).output_height > (*srcinfo).output_height
            {
                if (*info).output_width > (*srcinfo).output_width
                    && (*info).crop_width_set as ::core::ffi::c_uint
                        == JCROP_REFLECT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    do_crop_ext_reflect(
                        srcinfo,
                        dstinfo,
                        (*info).x_crop_offset,
                        (*info).y_crop_offset,
                        src_coef_arrays,
                        dst_coef_arrays,
                    );
                } else if (*info).output_width > (*srcinfo).output_width
                    && (*info).crop_width_set as ::core::ffi::c_uint
                        == JCROP_FORCE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    do_crop_ext_flat(
                        srcinfo,
                        dstinfo,
                        (*info).x_crop_offset,
                        (*info).y_crop_offset,
                        src_coef_arrays,
                        dst_coef_arrays,
                    );
                } else {
                    do_crop_ext_zero(
                        srcinfo,
                        dstinfo,
                        (*info).x_crop_offset,
                        (*info).y_crop_offset,
                        src_coef_arrays,
                        dst_coef_arrays,
                    );
                }
            } else if (*info).x_crop_offset != 0 as JDIMENSION
                || (*info).y_crop_offset != 0 as JDIMENSION
            {
                do_crop(
                    srcinfo,
                    dstinfo,
                    (*info).x_crop_offset,
                    (*info).y_crop_offset,
                    src_coef_arrays,
                    dst_coef_arrays,
                );
            }
        }
        1 => {
            if (*info).y_crop_offset != 0 as JDIMENSION || (*info).slow_hflip != 0 {
                do_flip_h(
                    srcinfo,
                    dstinfo,
                    (*info).x_crop_offset,
                    (*info).y_crop_offset,
                    src_coef_arrays,
                    dst_coef_arrays,
                );
            } else {
                do_flip_h_no_crop(srcinfo, dstinfo, (*info).x_crop_offset, src_coef_arrays);
            }
        }
        2 => {
            do_flip_v(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        3 => {
            do_transpose(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        4 => {
            do_transverse(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        5 => {
            do_rot_90(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        6 => {
            do_rot_180(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        7 => {
            do_rot_270(
                srcinfo,
                dstinfo,
                (*info).x_crop_offset,
                (*info).y_crop_offset,
                src_coef_arrays,
                dst_coef_arrays,
            );
        }
        8 => {
            if (*info).crop_width_set as ::core::ffi::c_uint
                == JCROP_REFLECT as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*info).y_crop_offset == 0 as JDIMENSION
                && (*info).drop_height
                    == jdiv_round_up(
                        (*info).output_height as ::core::ffi::c_long,
                        (*info).iMCU_sample_height as ::core::ffi::c_long,
                    ) as JDIMENSION
                && ((*info).x_crop_offset == 0 as JDIMENSION
                    || (*info).x_crop_offset.wrapping_add((*info).drop_width)
                        == jdiv_round_up(
                            (*info).output_width as ::core::ffi::c_long,
                            (*info).iMCU_sample_width as ::core::ffi::c_long,
                        ) as JDIMENSION)
            {
                do_reflect(
                    srcinfo,
                    dstinfo,
                    (*info).x_crop_offset,
                    src_coef_arrays,
                    (*info).drop_width,
                    (*info).drop_height,
                );
            } else if (*info).crop_width_set as ::core::ffi::c_uint
                == JCROP_FORCE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                do_flatten(
                    srcinfo,
                    dstinfo,
                    (*info).x_crop_offset,
                    (*info).y_crop_offset,
                    src_coef_arrays,
                    (*info).drop_width,
                    (*info).drop_height,
                );
            } else {
                do_wipe(
                    srcinfo,
                    dstinfo,
                    (*info).x_crop_offset,
                    (*info).y_crop_offset,
                    src_coef_arrays,
                    (*info).drop_width,
                    (*info).drop_height,
                );
            }
        }
        9 => {
            if (*info).drop_width != 0 as JDIMENSION && (*info).drop_height != 0 as JDIMENSION {
                do_drop(
                    srcinfo,
                    dstinfo,
                    (*info).x_crop_offset,
                    (*info).y_crop_offset,
                    src_coef_arrays,
                    (*info).drop_ptr,
                    (*info).drop_coef_arrays,
                    (*info).drop_width,
                    (*info).drop_height,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn jtransform_perfect_transform(
    mut image_width: JDIMENSION,
    mut image_height: JDIMENSION,
    mut MCU_width: ::core::ffi::c_int,
    mut MCU_height: ::core::ffi::c_int,
    mut transform: JXFORM_CODE,
) -> boolean {
    let mut result: boolean = TRUE;
    match transform as ::core::ffi::c_uint {
        1 | 7 => {
            if image_width.wrapping_rem(MCU_width as JDIMENSION) != 0 {
                result = FALSE as boolean;
            }
        }
        2 | 5 => {
            if image_height.wrapping_rem(MCU_height as JDIMENSION) != 0 {
                result = FALSE as boolean;
            }
        }
        4 | 6 => {
            if image_width.wrapping_rem(MCU_width as JDIMENSION) != 0 {
                result = FALSE as boolean;
            }
            if image_height.wrapping_rem(MCU_height as JDIMENSION) != 0 {
                result = FALSE as boolean;
            }
        }
        _ => {}
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn jcopy_markers_setup(
    mut srcinfo: j_decompress_ptr,
    mut option: JCOPY_OPTION,
) {
    let mut m: ::core::ffi::c_int = 0;
    if option as ::core::ffi::c_uint != JCOPYOPT_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        && option as ::core::ffi::c_uint
            != JCOPYOPT_ICC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        jpeg_save_markers(srcinfo, JPEG_COM, 0xffff as ::core::ffi::c_uint);
    }
    if option as ::core::ffi::c_uint == JCOPYOPT_ALL as ::core::ffi::c_int as ::core::ffi::c_uint
        || option as ::core::ffi::c_uint
            == JCOPYOPT_ALL_EXCEPT_ICC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        m = 0 as ::core::ffi::c_int;
        while m < 16 as ::core::ffi::c_int {
            if !(option as ::core::ffi::c_uint
                == JCOPYOPT_ALL_EXCEPT_ICC as ::core::ffi::c_int as ::core::ffi::c_uint
                && m == 2 as ::core::ffi::c_int)
            {
                jpeg_save_markers(srcinfo, JPEG_APP0 + m, 0xffff as ::core::ffi::c_uint);
            }
            m += 1;
        }
    }
    if option as ::core::ffi::c_uint == JCOPYOPT_ICC as ::core::ffi::c_int as ::core::ffi::c_uint {
        jpeg_save_markers(
            srcinfo,
            JPEG_APP0 + 2 as ::core::ffi::c_int,
            0xffff as ::core::ffi::c_uint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn jcopy_markers_execute(
    mut srcinfo: j_decompress_ptr,
    mut dstinfo: j_compress_ptr,
    mut option: JCOPY_OPTION,
) {
    let mut marker: jpeg_saved_marker_ptr = ::core::ptr::null_mut::<jpeg_marker_struct>();
    marker = (*srcinfo).marker_list;
    while !marker.is_null() {
        if !((*dstinfo).write_JFIF_header != 0
            && (*marker).marker as ::core::ffi::c_int == JPEG_APP0
            && (*marker).data_length >= 5 as ::core::ffi::c_uint
            && *(*marker).data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0x4a as ::core::ffi::c_int
            && *(*marker).data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0x46 as ::core::ffi::c_int
            && *(*marker).data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0x49 as ::core::ffi::c_int
            && *(*marker).data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0x46 as ::core::ffi::c_int
            && *(*marker).data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int)
        {
            if !((*dstinfo).write_Adobe_marker != 0
                && (*marker).marker as ::core::ffi::c_int == JPEG_APP0 + 14 as ::core::ffi::c_int
                && (*marker).data_length >= 5 as ::core::ffi::c_uint
                && *(*marker).data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x41 as ::core::ffi::c_int
                && *(*marker).data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x64 as ::core::ffi::c_int
                && *(*marker).data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x6f as ::core::ffi::c_int
                && *(*marker).data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x62 as ::core::ffi::c_int
                && *(*marker).data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0x65 as ::core::ffi::c_int)
            {
                jpeg_write_marker(
                    dstinfo,
                    (*marker).marker as ::core::ffi::c_int,
                    (*marker).data,
                    (*marker).data_length,
                );
            }
        }
        marker = (*marker).next;
    }
}
