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
    fn jcopy_block_row(input_row: JBLOCKROW, output_row: JBLOCKROW, num_blocks: JDIMENSION);
    fn jzero_far(target: *mut ::core::ffi::c_void, bytestozero: size_t);
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
pub type JLONG = ::core::ffi::c_long;
pub type my_coef_ptr = *mut my_coef_controller;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_coef_controller {
    pub pub_0: jpeg_d_coef_controller,
    pub MCU_ctr: JDIMENSION,
    pub MCU_vert_offset: ::core::ffi::c_int,
    pub MCU_rows_per_iMCU_row: ::core::ffi::c_int,
    pub MCU_buffer: [JBLOCKROW; 10],
    pub workspace: *mut JCOEF,
    pub whole_image: [jvirt_barray_ptr; 10],
    pub coef_bits_latch: *mut ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DCTSIZE2: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const D_MAX_BLOCKS_IN_MCU: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPEG_SUSPENDED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JPEG_ROW_COMPLETED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const JPEG_SCAN_COMPLETED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SAVED_COEFS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
unsafe extern "C" fn start_iMCU_row(mut cinfo: j_decompress_ptr) {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    if (*cinfo).comps_in_scan > 1 as ::core::ffi::c_int {
        (*coef).MCU_rows_per_iMCU_row = 1 as ::core::ffi::c_int;
    } else if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows.wrapping_sub(1 as JDIMENSION) {
        (*coef).MCU_rows_per_iMCU_row =
            (*(*cinfo).cur_comp_info[0 as ::core::ffi::c_int as usize]).v_samp_factor;
    } else {
        (*coef).MCU_rows_per_iMCU_row =
            (*(*cinfo).cur_comp_info[0 as ::core::ffi::c_int as usize]).last_row_height;
    }
    (*coef).MCU_ctr = 0 as JDIMENSION;
    (*coef).MCU_vert_offset = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn start_input_pass(mut cinfo: j_decompress_ptr) {
    (*cinfo).input_iMCU_row = 0 as JDIMENSION;
    start_iMCU_row(cinfo);
}
unsafe extern "C" fn start_output_pass(mut cinfo: j_decompress_ptr) {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    if !(*coef).pub_0.coef_arrays.is_null() {
        if (*cinfo).do_block_smoothing != 0 && smoothing_ok(cinfo) != 0 {
            (*coef).pub_0.decompress_data = Some(
                decompress_smooth_data
                    as unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE) -> ::core::ffi::c_int,
            )
                as Option<unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE) -> ::core::ffi::c_int>;
        } else {
            (*coef).pub_0.decompress_data = Some(
                decompress_data
                    as unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE) -> ::core::ffi::c_int,
            )
                as Option<unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE) -> ::core::ffi::c_int>;
        }
    }
    (*cinfo).output_iMCU_row = 0 as JDIMENSION;
}
unsafe extern "C" fn decompress_onepass(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPIMAGE,
) -> ::core::ffi::c_int {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut MCU_col_num: JDIMENSION = 0;
    let mut last_MCU_col: JDIMENSION = (*cinfo).MCUs_per_row.wrapping_sub(1 as JDIMENSION);
    let mut last_iMCU_row: JDIMENSION = (*cinfo).total_iMCU_rows.wrapping_sub(1 as JDIMENSION);
    let mut blkn: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut xindex: ::core::ffi::c_int = 0;
    let mut yindex: ::core::ffi::c_int = 0;
    let mut yoffset: ::core::ffi::c_int = 0;
    let mut useful_width: ::core::ffi::c_int = 0;
    let mut output_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    let mut start_col: JDIMENSION = 0;
    let mut output_col: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut inverse_DCT: inverse_DCT_method_ptr = None;
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).MCU_ctr;
        while MCU_col_num <= last_MCU_col {
            jzero_far(
                (*coef).MCU_buffer[0 as ::core::ffi::c_int as usize] as *mut ::core::ffi::c_void,
                ((*cinfo).blocks_in_MCU as usize)
                    .wrapping_mul(::core::mem::size_of::<JBLOCK>() as usize),
            );
            if (*(*cinfo).entropy).insufficient_data == 0 {
                (*(*cinfo).master).last_good_iMCU_row = (*cinfo).input_iMCU_row;
            }
            if Some(
                (*(*cinfo).entropy)
                    .decode_mcu
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                ::core::ptr::addr_of_mut!((*coef).MCU_buffer) as *mut JBLOCKROW,
            ) == 0
            {
                (*coef).MCU_vert_offset = yoffset;
                (*coef).MCU_ctr = MCU_col_num;
                return JPEG_SUSPENDED;
            }
            if MCU_col_num >= (*(*cinfo).master).first_iMCU_col
                && MCU_col_num <= (*(*cinfo).master).last_iMCU_col
            {
                blkn = 0 as ::core::ffi::c_int;
                ci = 0 as ::core::ffi::c_int;
                while ci < (*cinfo).comps_in_scan {
                    compptr = (*cinfo).cur_comp_info[ci as usize];
                    if (*compptr).component_needed == 0 {
                        blkn += (*compptr).MCU_blocks;
                    } else {
                        inverse_DCT =
                            (*(*cinfo).idct).inverse_DCT[(*compptr).component_index as usize];
                        useful_width = if MCU_col_num < last_MCU_col {
                            (*compptr).MCU_width
                        } else {
                            (*compptr).last_col_width
                        };
                        output_ptr = (*output_buf.offset((*compptr).component_index as isize))
                            .offset((yoffset * (*compptr).DCT_h_scaled_size) as isize);
                        start_col = MCU_col_num
                            .wrapping_sub((*(*cinfo).master).first_iMCU_col)
                            .wrapping_mul((*compptr).MCU_sample_width as JDIMENSION);
                        yindex = 0 as ::core::ffi::c_int;
                        while yindex < (*compptr).MCU_height {
                            if (*cinfo).input_iMCU_row < last_iMCU_row
                                || yoffset + yindex < (*compptr).last_row_height
                            {
                                output_col = start_col;
                                xindex = 0 as ::core::ffi::c_int;
                                while xindex < useful_width {
                                    Some(inverse_DCT.expect("non-null function pointer"))
                                        .expect("non-null function pointer")(
                                        cinfo,
                                        compptr,
                                        (*coef).MCU_buffer[(blkn + xindex) as usize] as JCOEFPTR,
                                        output_ptr,
                                        output_col,
                                    );
                                    output_col = output_col
                                        .wrapping_add((*compptr).DCT_h_scaled_size as JDIMENSION);
                                    xindex += 1;
                                }
                            }
                            blkn += (*compptr).MCU_width;
                            output_ptr = output_ptr.offset((*compptr).DCT_h_scaled_size as isize);
                            yindex += 1;
                        }
                    }
                    ci += 1;
                }
            }
            MCU_col_num = MCU_col_num.wrapping_add(1);
        }
        (*coef).MCU_ctr = 0 as JDIMENSION;
        yoffset += 1;
    }
    (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row.wrapping_add(1);
    (*cinfo).input_iMCU_row = (*cinfo).input_iMCU_row.wrapping_add(1);
    if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows {
        start_iMCU_row(cinfo);
        return JPEG_ROW_COMPLETED;
    }
    Some(
        (*(*cinfo).inputctl)
            .finish_input_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    return JPEG_SCAN_COMPLETED;
}
unsafe extern "C" fn dummy_consume_data(mut cinfo: j_decompress_ptr) -> ::core::ffi::c_int {
    return JPEG_SUSPENDED;
}
unsafe extern "C" fn consume_data(mut cinfo: j_decompress_ptr) -> ::core::ffi::c_int {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut MCU_col_num: JDIMENSION = 0;
    let mut blkn: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut xindex: ::core::ffi::c_int = 0;
    let mut yindex: ::core::ffi::c_int = 0;
    let mut yoffset: ::core::ffi::c_int = 0;
    let mut start_col: JDIMENSION = 0;
    let mut buffer: [JBLOCKARRAY; 4] = [::core::ptr::null_mut::<JBLOCKROW>(); 4];
    let mut buffer_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    ci = 0 as ::core::ffi::c_int;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        buffer[ci as usize] = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image[(*compptr).component_index as usize],
            (*cinfo)
                .input_iMCU_row
                .wrapping_mul((*compptr).v_samp_factor as JDIMENSION),
            (*compptr).v_samp_factor as JDIMENSION,
            TRUE,
        );
        ci += 1;
    }
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).MCU_ctr;
        while MCU_col_num < (*cinfo).MCUs_per_row {
            blkn = 0 as ::core::ffi::c_int;
            ci = 0 as ::core::ffi::c_int;
            while ci < (*cinfo).comps_in_scan {
                compptr = (*cinfo).cur_comp_info[ci as usize];
                start_col = MCU_col_num.wrapping_mul((*compptr).MCU_width as JDIMENSION);
                yindex = 0 as ::core::ffi::c_int;
                while yindex < (*compptr).MCU_height {
                    buffer_ptr = (*buffer[ci as usize].offset((yindex + yoffset) as isize))
                        .offset(start_col as isize);
                    xindex = 0 as ::core::ffi::c_int;
                    while xindex < (*compptr).MCU_width {
                        let fresh0 = buffer_ptr;
                        buffer_ptr = buffer_ptr.offset(1);
                        let fresh1 = blkn;
                        blkn = blkn + 1;
                        (*coef).MCU_buffer[fresh1 as usize] = fresh0;
                        xindex += 1;
                    }
                    yindex += 1;
                }
                ci += 1;
            }
            if (*(*cinfo).entropy).insufficient_data == 0 {
                (*(*cinfo).master).last_good_iMCU_row = (*cinfo).input_iMCU_row;
            }
            if Some(
                (*(*cinfo).entropy)
                    .decode_mcu
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                ::core::ptr::addr_of_mut!((*coef).MCU_buffer) as *mut JBLOCKROW,
            ) == 0
            {
                (*coef).MCU_vert_offset = yoffset;
                (*coef).MCU_ctr = MCU_col_num;
                return JPEG_SUSPENDED;
            }
            MCU_col_num = MCU_col_num.wrapping_add(1);
        }
        (*coef).MCU_ctr = 0 as JDIMENSION;
        yoffset += 1;
    }
    (*cinfo).input_iMCU_row = (*cinfo).input_iMCU_row.wrapping_add(1);
    if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows {
        start_iMCU_row(cinfo);
        return JPEG_ROW_COMPLETED;
    }
    Some(
        (*(*cinfo).inputctl)
            .finish_input_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    return JPEG_SCAN_COMPLETED;
}
unsafe extern "C" fn decompress_data(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPIMAGE,
) -> ::core::ffi::c_int {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: JDIMENSION = (*cinfo).total_iMCU_rows.wrapping_sub(1 as JDIMENSION);
    let mut block_num: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut block_row: ::core::ffi::c_int = 0;
    let mut block_rows: ::core::ffi::c_int = 0;
    let mut buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut buffer_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut output_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    let mut output_col: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut inverse_DCT: inverse_DCT_method_ptr = None;
    while (*cinfo).input_scan_number < (*cinfo).output_scan_number
        || (*cinfo).input_scan_number == (*cinfo).output_scan_number
            && (*cinfo).input_iMCU_row <= (*cinfo).output_iMCU_row
    {
        if Some(
            (*(*cinfo).inputctl)
                .consume_input
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == JPEG_SUSPENDED
        {
            return JPEG_SUSPENDED;
        }
    }
    ci = 0 as ::core::ffi::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        if !((*compptr).component_needed == 0) {
            buffer = Some(
                (*(*cinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                (*coef).whole_image[ci as usize],
                (*cinfo)
                    .output_iMCU_row
                    .wrapping_mul((*compptr).v_samp_factor as JDIMENSION),
                (*compptr).v_samp_factor as JDIMENSION,
                FALSE,
            );
            if (*cinfo).output_iMCU_row < last_iMCU_row {
                block_rows = (*compptr).v_samp_factor;
            } else {
                block_rows = (*compptr)
                    .height_in_blocks
                    .wrapping_rem((*compptr).v_samp_factor as JDIMENSION)
                    as ::core::ffi::c_int;
                if block_rows == 0 as ::core::ffi::c_int {
                    block_rows = (*compptr).v_samp_factor;
                }
            }
            inverse_DCT = (*(*cinfo).idct).inverse_DCT[ci as usize];
            output_ptr = *output_buf.offset(ci as isize);
            block_row = 0 as ::core::ffi::c_int;
            while block_row < block_rows {
                buffer_ptr = (*buffer.offset(block_row as isize))
                    .offset((*(*cinfo).master).first_MCU_col[ci as usize] as isize);
                output_col = 0 as JDIMENSION;
                block_num = (*(*cinfo).master).first_MCU_col[ci as usize];
                while block_num <= (*(*cinfo).master).last_MCU_col[ci as usize] {
                    Some(inverse_DCT.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        cinfo,
                        compptr,
                        buffer_ptr as JCOEFPTR,
                        output_ptr,
                        output_col,
                    );
                    buffer_ptr = buffer_ptr.offset(1);
                    output_col =
                        output_col.wrapping_add((*compptr).DCT_h_scaled_size as JDIMENSION);
                    block_num = block_num.wrapping_add(1);
                }
                output_ptr = output_ptr.offset((*compptr).DCT_h_scaled_size as isize);
                block_row += 1;
            }
        }
        ci += 1;
        compptr = compptr.offset(1);
    }
    (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row.wrapping_add(1);
    if (*cinfo).output_iMCU_row < (*cinfo).total_iMCU_rows {
        return JPEG_ROW_COMPLETED;
    }
    return JPEG_SCAN_COMPLETED;
}
pub const Q01_POS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Q10_POS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const Q20_POS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const Q11_POS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const Q02_POS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const Q03_POS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const Q12_POS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const Q21_POS: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const Q30_POS: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
unsafe extern "C" fn smoothing_ok(mut cinfo: j_decompress_ptr) -> boolean {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut smoothing_useful: boolean = FALSE;
    let mut ci: ::core::ffi::c_int = 0;
    let mut coefi: ::core::ffi::c_int = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut qtable: *mut JQUANT_TBL = ::core::ptr::null_mut::<JQUANT_TBL>();
    let mut coef_bits: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut prev_coef_bits: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut coef_bits_latch: *mut ::core::ffi::c_int =
        ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut prev_coef_bits_latch: *mut ::core::ffi::c_int =
        ::core::ptr::null_mut::<::core::ffi::c_int>();
    if (*cinfo).progressive_mode == 0 || (*cinfo).coef_bits.is_null() {
        return FALSE;
    }
    if (*coef).coef_bits_latch.is_null() {
        (*coef).coef_bits_latch = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (((*cinfo).num_components * 2 as ::core::ffi::c_int) as size_t).wrapping_mul(
                (SAVED_COEFS as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            ),
        ) as *mut ::core::ffi::c_int;
    }
    coef_bits_latch = (*coef).coef_bits_latch;
    prev_coef_bits_latch = (*coef)
        .coef_bits_latch
        .offset(((*cinfo).num_components * SAVED_COEFS) as isize)
        as *mut ::core::ffi::c_int;
    ci = 0 as ::core::ffi::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        qtable = (*compptr).quant_table;
        if qtable.is_null() {
            return FALSE;
        }
        if (*qtable).quantval[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || (*qtable).quantval[Q01_POS as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*qtable).quantval[Q10_POS as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*qtable).quantval[Q20_POS as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*qtable).quantval[Q11_POS as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*qtable).quantval[Q02_POS as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*qtable).quantval[Q03_POS as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*qtable).quantval[Q12_POS as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*qtable).quantval[Q21_POS as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*qtable).quantval[Q30_POS as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return FALSE;
        }
        coef_bits = ::core::ptr::addr_of_mut!(*(*cinfo).coef_bits.offset(ci as isize)) as *mut ::core::ffi::c_int;
        prev_coef_bits = ::core::ptr::addr_of_mut!(*(*cinfo)
            .coef_bits
            .offset((ci + (*cinfo).num_components) as isize)
           ) as *mut ::core::ffi::c_int;
        if *coef_bits.offset(0 as ::core::ffi::c_int as isize) < 0 as ::core::ffi::c_int {
            return FALSE;
        }
        *coef_bits_latch.offset(0 as ::core::ffi::c_int as isize) =
            *coef_bits.offset(0 as ::core::ffi::c_int as isize);
        coefi = 1 as ::core::ffi::c_int;
        while coefi < SAVED_COEFS {
            if (*cinfo).input_scan_number > 1 as ::core::ffi::c_int {
                *prev_coef_bits_latch.offset(coefi as isize) =
                    *prev_coef_bits.offset(coefi as isize);
            } else {
                *prev_coef_bits_latch.offset(coefi as isize) = -(1 as ::core::ffi::c_int);
            }
            *coef_bits_latch.offset(coefi as isize) = *coef_bits.offset(coefi as isize);
            if *coef_bits.offset(coefi as isize) != 0 as ::core::ffi::c_int {
                smoothing_useful = TRUE as boolean;
            }
            coefi += 1;
        }
        coef_bits_latch = coef_bits_latch.offset(SAVED_COEFS as isize);
        prev_coef_bits_latch = prev_coef_bits_latch.offset(SAVED_COEFS as isize);
        ci += 1;
        compptr = compptr.offset(1);
    }
    return smoothing_useful;
}
unsafe extern "C" fn decompress_smooth_data(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPIMAGE,
) -> ::core::ffi::c_int {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: JDIMENSION = (*cinfo).total_iMCU_rows.wrapping_sub(1 as JDIMENSION);
    let mut block_num: JDIMENSION = 0;
    let mut last_block_column: JDIMENSION = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut block_row: ::core::ffi::c_int = 0;
    let mut block_rows: ::core::ffi::c_int = 0;
    let mut access_rows: ::core::ffi::c_int = 0;
    let mut buffer: JBLOCKARRAY = ::core::ptr::null_mut::<JBLOCKROW>();
    let mut buffer_ptr: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut prev_prev_block_row: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut prev_block_row: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut next_block_row: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut next_next_block_row: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
    let mut output_ptr: JSAMPARRAY = ::core::ptr::null_mut::<JSAMPROW>();
    let mut output_col: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut inverse_DCT: inverse_DCT_method_ptr = None;
    let mut change_dc: boolean = 0;
    let mut workspace: *mut JCOEF = ::core::ptr::null_mut::<JCOEF>();
    let mut coef_bits: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut quanttbl: *mut JQUANT_TBL = ::core::ptr::null_mut::<JQUANT_TBL>();
    let mut Q00: JLONG = 0;
    let mut Q01: JLONG = 0;
    let mut Q02: JLONG = 0;
    let mut Q03: JLONG = 0 as JLONG;
    let mut Q10: JLONG = 0;
    let mut Q11: JLONG = 0;
    let mut Q12: JLONG = 0 as JLONG;
    let mut Q20: JLONG = 0;
    let mut Q21: JLONG = 0 as JLONG;
    let mut Q30: JLONG = 0 as JLONG;
    let mut num: JLONG = 0;
    let mut DC01: ::core::ffi::c_int = 0;
    let mut DC02: ::core::ffi::c_int = 0;
    let mut DC03: ::core::ffi::c_int = 0;
    let mut DC04: ::core::ffi::c_int = 0;
    let mut DC05: ::core::ffi::c_int = 0;
    let mut DC06: ::core::ffi::c_int = 0;
    let mut DC07: ::core::ffi::c_int = 0;
    let mut DC08: ::core::ffi::c_int = 0;
    let mut DC09: ::core::ffi::c_int = 0;
    let mut DC10: ::core::ffi::c_int = 0;
    let mut DC11: ::core::ffi::c_int = 0;
    let mut DC12: ::core::ffi::c_int = 0;
    let mut DC13: ::core::ffi::c_int = 0;
    let mut DC14: ::core::ffi::c_int = 0;
    let mut DC15: ::core::ffi::c_int = 0;
    let mut DC16: ::core::ffi::c_int = 0;
    let mut DC17: ::core::ffi::c_int = 0;
    let mut DC18: ::core::ffi::c_int = 0;
    let mut DC19: ::core::ffi::c_int = 0;
    let mut DC20: ::core::ffi::c_int = 0;
    let mut DC21: ::core::ffi::c_int = 0;
    let mut DC22: ::core::ffi::c_int = 0;
    let mut DC23: ::core::ffi::c_int = 0;
    let mut DC24: ::core::ffi::c_int = 0;
    let mut DC25: ::core::ffi::c_int = 0;
    let mut Al: ::core::ffi::c_int = 0;
    let mut pred: ::core::ffi::c_int = 0;
    workspace = (*coef).workspace;
    while (*cinfo).input_scan_number <= (*cinfo).output_scan_number
        && (*(*cinfo).inputctl).eoi_reached == 0
    {
        if (*cinfo).input_scan_number == (*cinfo).output_scan_number {
            let mut delta: JDIMENSION = (if (*cinfo).Ss == 0 as ::core::ffi::c_int {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as JDIMENSION;
            if (*cinfo).input_iMCU_row > (*cinfo).output_iMCU_row.wrapping_add(delta) {
                break;
            }
        }
        if Some(
            (*(*cinfo).inputctl)
                .consume_input
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == JPEG_SUSPENDED
        {
            return JPEG_SUSPENDED;
        }
    }
    ci = 0 as ::core::ffi::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        if !((*compptr).component_needed == 0) {
            if (*cinfo).output_iMCU_row.wrapping_add(1 as JDIMENSION) < last_iMCU_row {
                block_rows = (*compptr).v_samp_factor;
                access_rows = block_rows * 3 as ::core::ffi::c_int;
            } else if (*cinfo).output_iMCU_row < last_iMCU_row {
                block_rows = (*compptr).v_samp_factor;
                access_rows = block_rows * 2 as ::core::ffi::c_int;
            } else {
                block_rows = (*compptr)
                    .height_in_blocks
                    .wrapping_rem((*compptr).v_samp_factor as JDIMENSION)
                    as ::core::ffi::c_int;
                if block_rows == 0 as ::core::ffi::c_int {
                    block_rows = (*compptr).v_samp_factor;
                }
                access_rows = block_rows;
            }
            if (*cinfo).output_iMCU_row > 1 as JDIMENSION {
                access_rows += 2 as ::core::ffi::c_int * (*compptr).v_samp_factor;
                buffer = Some(
                    (*(*cinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    (*coef).whole_image[ci as usize],
                    (*cinfo)
                        .output_iMCU_row
                        .wrapping_sub(2 as JDIMENSION)
                        .wrapping_mul((*compptr).v_samp_factor as JDIMENSION),
                    access_rows as JDIMENSION,
                    FALSE,
                );
                buffer =
                    buffer.offset((2 as ::core::ffi::c_int * (*compptr).v_samp_factor) as isize);
            } else if (*cinfo).output_iMCU_row > 0 as JDIMENSION {
                buffer = Some(
                    (*(*cinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    (*coef).whole_image[ci as usize],
                    (*cinfo)
                        .output_iMCU_row
                        .wrapping_sub(1 as JDIMENSION)
                        .wrapping_mul((*compptr).v_samp_factor as JDIMENSION),
                    access_rows as JDIMENSION,
                    FALSE,
                );
                buffer = buffer.offset((*compptr).v_samp_factor as isize);
            } else {
                buffer = Some(
                    (*(*cinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    (*coef).whole_image[ci as usize],
                    0 as ::core::ffi::c_int as JDIMENSION,
                    access_rows as JDIMENSION,
                    FALSE,
                );
            }
            if (*cinfo).output_iMCU_row > (*(*cinfo).master).last_good_iMCU_row {
                coef_bits = (*coef)
                    .coef_bits_latch
                    .offset(((ci + (*cinfo).num_components) * SAVED_COEFS) as isize);
            } else {
                coef_bits = (*coef).coef_bits_latch.offset((ci * SAVED_COEFS) as isize);
            }
            change_dc = (*coef_bits.offset(1 as ::core::ffi::c_int as isize)
                == -(1 as ::core::ffi::c_int)
                && *coef_bits.offset(2 as ::core::ffi::c_int as isize)
                    == -(1 as ::core::ffi::c_int)
                && *coef_bits.offset(3 as ::core::ffi::c_int as isize)
                    == -(1 as ::core::ffi::c_int)
                && *coef_bits.offset(4 as ::core::ffi::c_int as isize)
                    == -(1 as ::core::ffi::c_int)
                && *coef_bits.offset(5 as ::core::ffi::c_int as isize)
                    == -(1 as ::core::ffi::c_int)
                && *coef_bits.offset(6 as ::core::ffi::c_int as isize)
                    == -(1 as ::core::ffi::c_int)
                && *coef_bits.offset(7 as ::core::ffi::c_int as isize)
                    == -(1 as ::core::ffi::c_int)
                && *coef_bits.offset(8 as ::core::ffi::c_int as isize)
                    == -(1 as ::core::ffi::c_int)
                && *coef_bits.offset(9 as ::core::ffi::c_int as isize)
                    == -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int
                as boolean;
            quanttbl = (*compptr).quant_table;
            Q00 = (*quanttbl).quantval[0 as ::core::ffi::c_int as usize] as JLONG;
            Q01 = (*quanttbl).quantval[Q01_POS as usize] as JLONG;
            Q10 = (*quanttbl).quantval[Q10_POS as usize] as JLONG;
            Q20 = (*quanttbl).quantval[Q20_POS as usize] as JLONG;
            Q11 = (*quanttbl).quantval[Q11_POS as usize] as JLONG;
            Q02 = (*quanttbl).quantval[Q02_POS as usize] as JLONG;
            if change_dc != 0 {
                Q03 = (*quanttbl).quantval[Q03_POS as usize] as JLONG;
                Q12 = (*quanttbl).quantval[Q12_POS as usize] as JLONG;
                Q21 = (*quanttbl).quantval[Q21_POS as usize] as JLONG;
                Q30 = (*quanttbl).quantval[Q30_POS as usize] as JLONG;
            }
            inverse_DCT = (*(*cinfo).idct).inverse_DCT[ci as usize];
            output_ptr = *output_buf.offset(ci as isize);
            block_row = 0 as ::core::ffi::c_int;
            while block_row < block_rows {
                buffer_ptr = (*buffer.offset(block_row as isize))
                    .offset((*(*cinfo).master).first_MCU_col[ci as usize] as isize);
                if block_row > 0 as ::core::ffi::c_int || (*cinfo).output_iMCU_row > 0 as JDIMENSION
                {
                    prev_block_row = (*buffer
                        .offset((block_row - 1 as ::core::ffi::c_int) as isize))
                    .offset((*(*cinfo).master).first_MCU_col[ci as usize] as isize);
                } else {
                    prev_block_row = buffer_ptr;
                }
                if block_row > 1 as ::core::ffi::c_int || (*cinfo).output_iMCU_row > 1 as JDIMENSION
                {
                    prev_prev_block_row = (*buffer
                        .offset((block_row - 2 as ::core::ffi::c_int) as isize))
                    .offset((*(*cinfo).master).first_MCU_col[ci as usize] as isize);
                } else {
                    prev_prev_block_row = prev_block_row;
                }
                if block_row < block_rows - 1 as ::core::ffi::c_int
                    || (*cinfo).output_iMCU_row < last_iMCU_row
                {
                    next_block_row = (*buffer
                        .offset((block_row + 1 as ::core::ffi::c_int) as isize))
                    .offset((*(*cinfo).master).first_MCU_col[ci as usize] as isize);
                } else {
                    next_block_row = buffer_ptr;
                }
                if block_row < block_rows - 2 as ::core::ffi::c_int
                    || (*cinfo).output_iMCU_row.wrapping_add(1 as JDIMENSION) < last_iMCU_row
                {
                    next_next_block_row = (*buffer
                        .offset((block_row + 2 as ::core::ffi::c_int) as isize))
                    .offset((*(*cinfo).master).first_MCU_col[ci as usize] as isize);
                } else {
                    next_next_block_row = next_block_row;
                }
                DC05 = (*prev_prev_block_row.offset(0 as ::core::ffi::c_int as isize))
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                DC04 = DC05;
                DC03 = DC04;
                DC02 = DC03;
                DC01 = DC02;
                DC10 = (*prev_block_row.offset(0 as ::core::ffi::c_int as isize))
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                DC09 = DC10;
                DC08 = DC09;
                DC07 = DC08;
                DC06 = DC07;
                DC15 = (*buffer_ptr.offset(0 as ::core::ffi::c_int as isize))
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                DC14 = DC15;
                DC13 = DC14;
                DC12 = DC13;
                DC11 = DC12;
                DC20 = (*next_block_row.offset(0 as ::core::ffi::c_int as isize))
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                DC19 = DC20;
                DC18 = DC19;
                DC17 = DC18;
                DC16 = DC17;
                DC25 = (*next_next_block_row.offset(0 as ::core::ffi::c_int as isize))
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                DC24 = DC25;
                DC23 = DC24;
                DC22 = DC23;
                DC21 = DC22;
                output_col = 0 as JDIMENSION;
                last_block_column = (*compptr).width_in_blocks.wrapping_sub(1 as JDIMENSION);
                block_num = (*(*cinfo).master).first_MCU_col[ci as usize];
                while block_num <= (*(*cinfo).master).last_MCU_col[ci as usize] {
                    jcopy_block_row(
                        buffer_ptr,
                        workspace as JBLOCKROW,
                        1 as ::core::ffi::c_int as JDIMENSION,
                    );
                    if block_num == (*(*cinfo).master).first_MCU_col[ci as usize]
                        && block_num < last_block_column
                    {
                        DC04 = (*prev_prev_block_row.offset(1 as ::core::ffi::c_int as isize))
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                        DC09 = (*prev_block_row.offset(1 as ::core::ffi::c_int as isize))
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                        DC14 = (*buffer_ptr.offset(1 as ::core::ffi::c_int as isize))
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                        DC19 = (*next_block_row.offset(1 as ::core::ffi::c_int as isize))
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                        DC24 = (*next_next_block_row.offset(1 as ::core::ffi::c_int as isize))
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                    }
                    if block_num.wrapping_add(1 as JDIMENSION) < last_block_column {
                        DC05 = (*prev_prev_block_row.offset(2 as ::core::ffi::c_int as isize))
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                        DC10 = (*prev_block_row.offset(2 as ::core::ffi::c_int as isize))
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                        DC15 = (*buffer_ptr.offset(2 as ::core::ffi::c_int as isize))
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                        DC20 = (*next_block_row.offset(2 as ::core::ffi::c_int as isize))
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                        DC25 = (*next_next_block_row.offset(2 as ::core::ffi::c_int as isize))
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                    }
                    Al = *coef_bits.offset(1 as ::core::ffi::c_int as isize);
                    if Al != 0 as ::core::ffi::c_int
                        && *workspace.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        num = Q00
                            * (if change_dc != 0 {
                                -DC01 - DC02 + DC04 + DC05 - 3 as ::core::ffi::c_int * DC06
                                    + 13 as ::core::ffi::c_int * DC07
                                    - 13 as ::core::ffi::c_int * DC09
                                    + 3 as ::core::ffi::c_int * DC10
                                    - 3 as ::core::ffi::c_int * DC11
                                    + 38 as ::core::ffi::c_int * DC12
                                    - 38 as ::core::ffi::c_int * DC14
                                    + 3 as ::core::ffi::c_int * DC15
                                    - 3 as ::core::ffi::c_int * DC16
                                    + 13 as ::core::ffi::c_int * DC17
                                    - 13 as ::core::ffi::c_int * DC19
                                    + 3 as ::core::ffi::c_int * DC20
                                    - DC21
                                    - DC22
                                    + DC24
                                    + DC25
                            } else {
                                -(7 as ::core::ffi::c_int) * DC11 + 50 as ::core::ffi::c_int * DC12
                                    - 50 as ::core::ffi::c_int * DC14
                                    + 7 as ::core::ffi::c_int * DC15
                            }) as JLONG;
                        if num >= 0 as JLONG {
                            pred = (((Q01 << 7 as ::core::ffi::c_int) + num)
                                / (Q01 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                            if Al > 0 as ::core::ffi::c_int
                                && pred >= (1 as ::core::ffi::c_int) << Al
                            {
                                pred = ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                            }
                        } else {
                            pred = (((Q01 << 7 as ::core::ffi::c_int) - num)
                                / (Q01 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                            if Al > 0 as ::core::ffi::c_int
                                && pred >= (1 as ::core::ffi::c_int) << Al
                            {
                                pred = ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                            }
                            pred = -pred;
                        }
                        *workspace.offset(1 as ::core::ffi::c_int as isize) = pred as JCOEF;
                    }
                    Al = *coef_bits.offset(2 as ::core::ffi::c_int as isize);
                    if Al != 0 as ::core::ffi::c_int
                        && *workspace.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        num = Q00
                            * (if change_dc != 0 {
                                -DC01
                                    - 3 as ::core::ffi::c_int * DC02
                                    - 3 as ::core::ffi::c_int * DC03
                                    - 3 as ::core::ffi::c_int * DC04
                                    - DC05
                                    - DC06
                                    + 13 as ::core::ffi::c_int * DC07
                                    + 38 as ::core::ffi::c_int * DC08
                                    + 13 as ::core::ffi::c_int * DC09
                                    - DC10
                                    + DC16
                                    - 13 as ::core::ffi::c_int * DC17
                                    - 38 as ::core::ffi::c_int * DC18
                                    - 13 as ::core::ffi::c_int * DC19
                                    + DC20
                                    + DC21
                                    + 3 as ::core::ffi::c_int * DC22
                                    + 3 as ::core::ffi::c_int * DC23
                                    + 3 as ::core::ffi::c_int * DC24
                                    + DC25
                            } else {
                                -(7 as ::core::ffi::c_int) * DC03 + 50 as ::core::ffi::c_int * DC08
                                    - 50 as ::core::ffi::c_int * DC18
                                    + 7 as ::core::ffi::c_int * DC23
                            }) as JLONG;
                        if num >= 0 as JLONG {
                            pred = (((Q10 << 7 as ::core::ffi::c_int) + num)
                                / (Q10 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                            if Al > 0 as ::core::ffi::c_int
                                && pred >= (1 as ::core::ffi::c_int) << Al
                            {
                                pred = ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                            }
                        } else {
                            pred = (((Q10 << 7 as ::core::ffi::c_int) - num)
                                / (Q10 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                            if Al > 0 as ::core::ffi::c_int
                                && pred >= (1 as ::core::ffi::c_int) << Al
                            {
                                pred = ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                            }
                            pred = -pred;
                        }
                        *workspace.offset(8 as ::core::ffi::c_int as isize) = pred as JCOEF;
                    }
                    Al = *coef_bits.offset(3 as ::core::ffi::c_int as isize);
                    if Al != 0 as ::core::ffi::c_int
                        && *workspace.offset(16 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        num = Q00
                            * (if change_dc != 0 {
                                DC03 + 2 as ::core::ffi::c_int * DC07
                                    + 7 as ::core::ffi::c_int * DC08
                                    + 2 as ::core::ffi::c_int * DC09
                                    - 5 as ::core::ffi::c_int * DC12
                                    - 14 as ::core::ffi::c_int * DC13
                                    - 5 as ::core::ffi::c_int * DC14
                                    + 2 as ::core::ffi::c_int * DC17
                                    + 7 as ::core::ffi::c_int * DC18
                                    + 2 as ::core::ffi::c_int * DC19
                                    + DC23
                            } else {
                                -DC03 + 13 as ::core::ffi::c_int * DC08
                                    - 24 as ::core::ffi::c_int * DC13
                                    + 13 as ::core::ffi::c_int * DC18
                                    - DC23
                            }) as JLONG;
                        if num >= 0 as JLONG {
                            pred = (((Q20 << 7 as ::core::ffi::c_int) + num)
                                / (Q20 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                            if Al > 0 as ::core::ffi::c_int
                                && pred >= (1 as ::core::ffi::c_int) << Al
                            {
                                pred = ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                            }
                        } else {
                            pred = (((Q20 << 7 as ::core::ffi::c_int) - num)
                                / (Q20 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                            if Al > 0 as ::core::ffi::c_int
                                && pred >= (1 as ::core::ffi::c_int) << Al
                            {
                                pred = ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                            }
                            pred = -pred;
                        }
                        *workspace.offset(16 as ::core::ffi::c_int as isize) = pred as JCOEF;
                    }
                    Al = *coef_bits.offset(4 as ::core::ffi::c_int as isize);
                    if Al != 0 as ::core::ffi::c_int
                        && *workspace.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        num = Q00
                            * (if change_dc != 0 {
                                -DC01 + DC05 + 9 as ::core::ffi::c_int * DC07
                                    - 9 as ::core::ffi::c_int * DC09
                                    - 9 as ::core::ffi::c_int * DC17
                                    + 9 as ::core::ffi::c_int * DC19
                                    + DC21
                                    - DC25
                            } else {
                                DC10 + DC16 - 10 as ::core::ffi::c_int * DC17
                                    + 10 as ::core::ffi::c_int * DC19
                                    - DC02
                                    - DC20
                                    + DC22
                                    - DC24
                                    + DC04
                                    - DC06
                                    + 10 as ::core::ffi::c_int * DC07
                                    - 10 as ::core::ffi::c_int * DC09
                            }) as JLONG;
                        if num >= 0 as JLONG {
                            pred = (((Q11 << 7 as ::core::ffi::c_int) + num)
                                / (Q11 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                            if Al > 0 as ::core::ffi::c_int
                                && pred >= (1 as ::core::ffi::c_int) << Al
                            {
                                pred = ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                            }
                        } else {
                            pred = (((Q11 << 7 as ::core::ffi::c_int) - num)
                                / (Q11 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                            if Al > 0 as ::core::ffi::c_int
                                && pred >= (1 as ::core::ffi::c_int) << Al
                            {
                                pred = ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                            }
                            pred = -pred;
                        }
                        *workspace.offset(9 as ::core::ffi::c_int as isize) = pred as JCOEF;
                    }
                    Al = *coef_bits.offset(5 as ::core::ffi::c_int as isize);
                    if Al != 0 as ::core::ffi::c_int
                        && *workspace.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        num = Q00
                            * (if change_dc != 0 {
                                2 as ::core::ffi::c_int * DC07 - 5 as ::core::ffi::c_int * DC08
                                    + 2 as ::core::ffi::c_int * DC09
                                    + DC11
                                    + 7 as ::core::ffi::c_int * DC12
                                    - 14 as ::core::ffi::c_int * DC13
                                    + 7 as ::core::ffi::c_int * DC14
                                    + DC15
                                    + 2 as ::core::ffi::c_int * DC17
                                    - 5 as ::core::ffi::c_int * DC18
                                    + 2 as ::core::ffi::c_int * DC19
                            } else {
                                -DC11 + 13 as ::core::ffi::c_int * DC12
                                    - 24 as ::core::ffi::c_int * DC13
                                    + 13 as ::core::ffi::c_int * DC14
                                    - DC15
                            }) as JLONG;
                        if num >= 0 as JLONG {
                            pred = (((Q02 << 7 as ::core::ffi::c_int) + num)
                                / (Q02 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                            if Al > 0 as ::core::ffi::c_int
                                && pred >= (1 as ::core::ffi::c_int) << Al
                            {
                                pred = ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                            }
                        } else {
                            pred = (((Q02 << 7 as ::core::ffi::c_int) - num)
                                / (Q02 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                            if Al > 0 as ::core::ffi::c_int
                                && pred >= (1 as ::core::ffi::c_int) << Al
                            {
                                pred = ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                            }
                            pred = -pred;
                        }
                        *workspace.offset(2 as ::core::ffi::c_int as isize) = pred as JCOEF;
                    }
                    if change_dc != 0 {
                        Al = *coef_bits.offset(6 as ::core::ffi::c_int as isize);
                        if Al != 0 as ::core::ffi::c_int
                            && *workspace.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            num = Q00
                                * (DC07 - DC09 + 2 as ::core::ffi::c_int * DC12
                                    - 2 as ::core::ffi::c_int * DC14
                                    + DC17
                                    - DC19) as JLONG;
                            if num >= 0 as JLONG {
                                pred = (((Q03 << 7 as ::core::ffi::c_int) + num)
                                    / (Q03 << 8 as ::core::ffi::c_int))
                                    as ::core::ffi::c_int;
                                if Al > 0 as ::core::ffi::c_int
                                    && pred >= (1 as ::core::ffi::c_int) << Al
                                {
                                    pred =
                                        ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                                }
                            } else {
                                pred = (((Q03 << 7 as ::core::ffi::c_int) - num)
                                    / (Q03 << 8 as ::core::ffi::c_int))
                                    as ::core::ffi::c_int;
                                if Al > 0 as ::core::ffi::c_int
                                    && pred >= (1 as ::core::ffi::c_int) << Al
                                {
                                    pred =
                                        ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                                }
                                pred = -pred;
                            }
                            *workspace.offset(3 as ::core::ffi::c_int as isize) = pred as JCOEF;
                        }
                        Al = *coef_bits.offset(7 as ::core::ffi::c_int as isize);
                        if Al != 0 as ::core::ffi::c_int
                            && *workspace.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            num = Q00
                                * (DC07 - 3 as ::core::ffi::c_int * DC08 + DC09 - DC17
                                    + 3 as ::core::ffi::c_int * DC18
                                    - DC19) as JLONG;
                            if num >= 0 as JLONG {
                                pred = (((Q12 << 7 as ::core::ffi::c_int) + num)
                                    / (Q12 << 8 as ::core::ffi::c_int))
                                    as ::core::ffi::c_int;
                                if Al > 0 as ::core::ffi::c_int
                                    && pred >= (1 as ::core::ffi::c_int) << Al
                                {
                                    pred =
                                        ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                                }
                            } else {
                                pred = (((Q12 << 7 as ::core::ffi::c_int) - num)
                                    / (Q12 << 8 as ::core::ffi::c_int))
                                    as ::core::ffi::c_int;
                                if Al > 0 as ::core::ffi::c_int
                                    && pred >= (1 as ::core::ffi::c_int) << Al
                                {
                                    pred =
                                        ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                                }
                                pred = -pred;
                            }
                            *workspace.offset(10 as ::core::ffi::c_int as isize) = pred as JCOEF;
                        }
                        Al = *coef_bits.offset(8 as ::core::ffi::c_int as isize);
                        if Al != 0 as ::core::ffi::c_int
                            && *workspace.offset(17 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            num = Q00
                                * (DC07 - DC09 - 3 as ::core::ffi::c_int * DC12
                                    + 3 as ::core::ffi::c_int * DC14
                                    + DC17
                                    - DC19) as JLONG;
                            if num >= 0 as JLONG {
                                pred = (((Q21 << 7 as ::core::ffi::c_int) + num)
                                    / (Q21 << 8 as ::core::ffi::c_int))
                                    as ::core::ffi::c_int;
                                if Al > 0 as ::core::ffi::c_int
                                    && pred >= (1 as ::core::ffi::c_int) << Al
                                {
                                    pred =
                                        ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                                }
                            } else {
                                pred = (((Q21 << 7 as ::core::ffi::c_int) - num)
                                    / (Q21 << 8 as ::core::ffi::c_int))
                                    as ::core::ffi::c_int;
                                if Al > 0 as ::core::ffi::c_int
                                    && pred >= (1 as ::core::ffi::c_int) << Al
                                {
                                    pred =
                                        ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                                }
                                pred = -pred;
                            }
                            *workspace.offset(17 as ::core::ffi::c_int as isize) = pred as JCOEF;
                        }
                        Al = *coef_bits.offset(9 as ::core::ffi::c_int as isize);
                        if Al != 0 as ::core::ffi::c_int
                            && *workspace.offset(24 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            num = Q00
                                * (DC07 + 2 as ::core::ffi::c_int * DC08 + DC09
                                    - DC17
                                    - 2 as ::core::ffi::c_int * DC18
                                    - DC19) as JLONG;
                            if num >= 0 as JLONG {
                                pred = (((Q30 << 7 as ::core::ffi::c_int) + num)
                                    / (Q30 << 8 as ::core::ffi::c_int))
                                    as ::core::ffi::c_int;
                                if Al > 0 as ::core::ffi::c_int
                                    && pred >= (1 as ::core::ffi::c_int) << Al
                                {
                                    pred =
                                        ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                                }
                            } else {
                                pred = (((Q30 << 7 as ::core::ffi::c_int) - num)
                                    / (Q30 << 8 as ::core::ffi::c_int))
                                    as ::core::ffi::c_int;
                                if Al > 0 as ::core::ffi::c_int
                                    && pred >= (1 as ::core::ffi::c_int) << Al
                                {
                                    pred =
                                        ((1 as ::core::ffi::c_int) << Al) - 1 as ::core::ffi::c_int;
                                }
                                pred = -pred;
                            }
                            *workspace.offset(24 as ::core::ffi::c_int as isize) = pred as JCOEF;
                        }
                        num = Q00
                            * (-(2 as ::core::ffi::c_int) * DC01
                                - 6 as ::core::ffi::c_int * DC02
                                - 8 as ::core::ffi::c_int * DC03
                                - 6 as ::core::ffi::c_int * DC04
                                - 2 as ::core::ffi::c_int * DC05
                                - 6 as ::core::ffi::c_int * DC06
                                + 6 as ::core::ffi::c_int * DC07
                                + 42 as ::core::ffi::c_int * DC08
                                + 6 as ::core::ffi::c_int * DC09
                                - 6 as ::core::ffi::c_int * DC10
                                - 8 as ::core::ffi::c_int * DC11
                                + 42 as ::core::ffi::c_int * DC12
                                + 152 as ::core::ffi::c_int * DC13
                                + 42 as ::core::ffi::c_int * DC14
                                - 8 as ::core::ffi::c_int * DC15
                                - 6 as ::core::ffi::c_int * DC16
                                + 6 as ::core::ffi::c_int * DC17
                                + 42 as ::core::ffi::c_int * DC18
                                + 6 as ::core::ffi::c_int * DC19
                                - 6 as ::core::ffi::c_int * DC20
                                - 2 as ::core::ffi::c_int * DC21
                                - 6 as ::core::ffi::c_int * DC22
                                - 8 as ::core::ffi::c_int * DC23
                                - 6 as ::core::ffi::c_int * DC24
                                - 2 as ::core::ffi::c_int * DC25)
                                as JLONG;
                        if num >= 0 as JLONG {
                            pred = (((Q00 << 7 as ::core::ffi::c_int) + num)
                                / (Q00 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                        } else {
                            pred = (((Q00 << 7 as ::core::ffi::c_int) - num)
                                / (Q00 << 8 as ::core::ffi::c_int))
                                as ::core::ffi::c_int;
                            pred = -pred;
                        }
                        *workspace.offset(0 as ::core::ffi::c_int as isize) = pred as JCOEF;
                    }
                    Some(inverse_DCT.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        cinfo, compptr, workspace, output_ptr, output_col,
                    );
                    DC01 = DC02;
                    DC02 = DC03;
                    DC03 = DC04;
                    DC04 = DC05;
                    DC06 = DC07;
                    DC07 = DC08;
                    DC08 = DC09;
                    DC09 = DC10;
                    DC11 = DC12;
                    DC12 = DC13;
                    DC13 = DC14;
                    DC14 = DC15;
                    DC16 = DC17;
                    DC17 = DC18;
                    DC18 = DC19;
                    DC19 = DC20;
                    DC21 = DC22;
                    DC22 = DC23;
                    DC23 = DC24;
                    DC24 = DC25;
                    buffer_ptr = buffer_ptr.offset(1);
                    prev_block_row = prev_block_row.offset(1);
                    next_block_row = next_block_row.offset(1);
                    prev_prev_block_row = prev_prev_block_row.offset(1);
                    next_next_block_row = next_next_block_row.offset(1);
                    output_col =
                        output_col.wrapping_add((*compptr).DCT_h_scaled_size as JDIMENSION);
                    block_num = block_num.wrapping_add(1);
                }
                output_ptr = output_ptr.offset((*compptr).DCT_h_scaled_size as isize);
                block_row += 1;
            }
        }
        ci += 1;
        compptr = compptr.offset(1);
    }
    (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row.wrapping_add(1);
    if (*cinfo).output_iMCU_row < (*cinfo).total_iMCU_rows {
        return JPEG_ROW_COMPLETED;
    }
    return JPEG_SCAN_COMPLETED;
}
pub unsafe extern "C" fn jinit_d_coef_controller(
    mut cinfo: j_decompress_ptr,
    mut need_full_buffer: boolean,
) {
    let mut coef: my_coef_ptr = ::core::ptr::null_mut::<my_coef_controller>();
    coef = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::core::mem::size_of::<my_coef_controller>() as size_t,
    ) as my_coef_ptr;
    (*cinfo).coef = coef as *mut jpeg_d_coef_controller as *mut jpeg_d_coef_controller;
    (*coef).pub_0.start_input_pass =
        Some(start_input_pass as unsafe extern "C" fn(j_decompress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
    (*coef).pub_0.start_output_pass =
        Some(start_output_pass as unsafe extern "C" fn(j_decompress_ptr) -> ())
            as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
    (*coef).coef_bits_latch = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if need_full_buffer != 0 {
        let mut ci: ::core::ffi::c_int = 0;
        let mut access_rows: ::core::ffi::c_int = 0;
        let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
        ci = 0 as ::core::ffi::c_int;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            access_rows = (*compptr).v_samp_factor;
            if (*cinfo).progressive_mode != 0 {
                access_rows *= 5 as ::core::ffi::c_int;
            }
            (*coef).whole_image[ci as usize] = Some(
                (*(*cinfo).mem)
                    .request_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                JPOOL_IMAGE,
                TRUE,
                jround_up(
                    (*compptr).width_in_blocks as ::core::ffi::c_long,
                    (*compptr).h_samp_factor as ::core::ffi::c_long,
                ) as JDIMENSION,
                jround_up(
                    (*compptr).height_in_blocks as ::core::ffi::c_long,
                    (*compptr).v_samp_factor as ::core::ffi::c_long,
                ) as JDIMENSION,
                access_rows as JDIMENSION,
            );
            ci += 1;
            compptr = compptr.offset(1);
        }
        (*coef).pub_0.consume_data =
            Some(consume_data as unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int)
                as Option<unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int>;
        (*coef).pub_0.decompress_data = Some(
            decompress_data
                as unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE) -> ::core::ffi::c_int>;
        (*coef).pub_0.coef_arrays = ::core::ptr::addr_of_mut!((*coef).whole_image) as *mut jvirt_barray_ptr;
    } else {
        let mut buffer: JBLOCKROW = ::core::ptr::null_mut::<JBLOCK>();
        let mut i: ::core::ffi::c_int = 0;
        buffer = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (D_MAX_BLOCKS_IN_MCU as size_t)
                .wrapping_mul(::core::mem::size_of::<JBLOCK>() as size_t),
        ) as JBLOCKROW;
        i = 0 as ::core::ffi::c_int;
        while i < D_MAX_BLOCKS_IN_MCU {
            (*coef).MCU_buffer[i as usize] = buffer.offset(i as isize);
            i += 1;
        }
        (*coef).pub_0.consume_data = Some(
            dummy_consume_data as unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int>;
        (*coef).pub_0.decompress_data = Some(
            decompress_onepass
                as unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(j_decompress_ptr, JSAMPIMAGE) -> ::core::ffi::c_int>;
        (*coef).pub_0.coef_arrays = ::core::ptr::null_mut::<jvirt_barray_ptr>();
    }
    (*coef).workspace = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (::core::mem::size_of::<JCOEF>() as size_t).wrapping_mul(DCTSIZE2 as size_t),
    ) as *mut JCOEF;
}
