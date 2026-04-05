#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::c_void;

pub type boolean = ::core::ffi::c_int;
pub type int = ::core::ffi::c_int;
pub type long = ::core::ffi::c_long;
pub type ulong = ::core::ffi::c_ulong;
pub type size_t = usize;

pub type JSAMPLE = u8;
pub type JCOEF = i16;
pub type JOCTET = u8;
pub type UINT8 = u8;
pub type UINT16 = u16;
pub type INT16 = i16;
pub type INT32 = long;
pub type JDIMENSION = u32;
pub type JLONG = long;
pub type JUINTPTR = usize;

pub const FALSE: boolean = 0;
pub const TRUE: boolean = 1;

pub const JPEG_LIB_VERSION: int = 80;
pub const DCTSIZE: usize = 8;
pub const DCTSIZE2: usize = 64;
pub const NUM_QUANT_TBLS: usize = 4;
pub const NUM_HUFF_TBLS: usize = 4;
pub const NUM_ARITH_TBLS: usize = 16;
pub const MAX_COMPS_IN_SCAN: usize = 4;
pub const MAX_SAMP_FACTOR: usize = 4;
pub const C_MAX_BLOCKS_IN_MCU: usize = 10;
pub const D_MAX_BLOCKS_IN_MCU: usize = 10;
pub const MAX_COMPONENTS: usize = 10;
pub const MAXJSAMPLE: int = 255;
pub const CENTERJSAMPLE: int = 128;
pub const JPEG_MAX_DIMENSION: long = 65500;
pub const JCS_EXTENSIONS: int = 1;
pub const JCS_ALPHA_EXTENSIONS: int = 1;
pub const JMSG_LENGTH_MAX: usize = 200;
pub const JMSG_STR_PARM_MAX: usize = 80;
pub const JPEG_MSG_PARMS_MAX: usize = 8;
pub const JPOOL_PERMANENT: int = 0;
pub const JPOOL_IMAGE: int = 1;
pub const JPOOL_NUMPOOLS: usize = 2;
pub const TEMP_NAME_LENGTH: usize = 64;
pub const MAX_ALLOC_CHUNK: long = 1_000_000_000;

pub const JPEG_APP0: int = 0xE0;
pub const JPEG_EOI: int = 0xD9;

pub type J_COLOR_SPACE = int;
pub const JCS_UNKNOWN: J_COLOR_SPACE = 0;
pub const JCS_GRAYSCALE: J_COLOR_SPACE = 1;
pub const JCS_RGB: J_COLOR_SPACE = 2;
pub const JCS_YCbCr: J_COLOR_SPACE = 3;
pub const JCS_CMYK: J_COLOR_SPACE = 4;
pub const JCS_YCCK: J_COLOR_SPACE = 5;
pub const JCS_EXT_RGB: J_COLOR_SPACE = 6;
pub const JCS_EXT_RGBX: J_COLOR_SPACE = 7;
pub const JCS_EXT_BGR: J_COLOR_SPACE = 8;
pub const JCS_EXT_BGRX: J_COLOR_SPACE = 9;
pub const JCS_EXT_XBGR: J_COLOR_SPACE = 10;
pub const JCS_EXT_XRGB: J_COLOR_SPACE = 11;
pub const JCS_EXT_RGBA: J_COLOR_SPACE = 12;
pub const JCS_EXT_BGRA: J_COLOR_SPACE = 13;
pub const JCS_EXT_ABGR: J_COLOR_SPACE = 14;
pub const JCS_EXT_ARGB: J_COLOR_SPACE = 15;
pub const JCS_RGB565: J_COLOR_SPACE = 16;

pub type J_DCT_METHOD = int;
pub const JDCT_ISLOW: J_DCT_METHOD = 0;
pub const JDCT_IFAST: J_DCT_METHOD = 1;
pub const JDCT_FLOAT: J_DCT_METHOD = 2;
pub const JDCT_DEFAULT: J_DCT_METHOD = JDCT_ISLOW;
pub const JDCT_FASTEST: J_DCT_METHOD = JDCT_IFAST;

pub type J_DITHER_MODE = int;
pub const JDITHER_NONE: J_DITHER_MODE = 0;
pub const JDITHER_ORDERED: J_DITHER_MODE = 1;
pub const JDITHER_FS: J_DITHER_MODE = 2;

pub type J_BUF_MODE = int;
pub const JBUF_PASS_THRU: J_BUF_MODE = 0;
pub const JBUF_SAVE_SOURCE: J_BUF_MODE = 1;
pub const JBUF_CRANK_DEST: J_BUF_MODE = 2;
pub const JBUF_SAVE_AND_PASS: J_BUF_MODE = 3;

pub const CSTATE_START: int = 100;
pub const CSTATE_SCANNING: int = 101;
pub const CSTATE_RAW_OK: int = 102;
pub const CSTATE_WRCOEFS: int = 103;
pub const DSTATE_START: int = 200;
pub const DSTATE_INHEADER: int = 201;
pub const DSTATE_READY: int = 202;
pub const DSTATE_PRELOAD: int = 203;
pub const DSTATE_PRESCAN: int = 204;
pub const DSTATE_SCANNING: int = 205;
pub const DSTATE_RAW_OK: int = 206;
pub const DSTATE_BUFIMAGE: int = 207;
pub const DSTATE_BUFPOST: int = 208;
pub const DSTATE_RDCOEFS: int = 209;
pub const DSTATE_STOPPING: int = 210;

pub const JPEG_SUSPENDED: int = 0;
pub const JPEG_HEADER_OK: int = 1;
pub const JPEG_HEADER_TABLES_ONLY: int = 2;
pub const JPEG_REACHED_SOS: int = 1;
pub const JPEG_REACHED_EOI: int = 2;
pub const JPEG_ROW_COMPLETED: int = 3;
pub const JPEG_SCAN_COMPLETED: int = 4;

pub type JSAMPROW = *mut JSAMPLE;
pub type JSAMPARRAY = *mut JSAMPROW;
pub type JSAMPIMAGE = *mut JSAMPARRAY;
pub type JBLOCK = [JCOEF; DCTSIZE2];
pub type JBLOCKROW = *mut JBLOCK;
pub type JBLOCKARRAY = *mut JBLOCKROW;
pub type JBLOCKIMAGE = *mut JBLOCKARRAY;
pub type JCOEFPTR = *mut JCOEF;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct jvirt_sarray_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct jvirt_barray_control {
    _private: [u8; 0],
}

pub type jvirt_sarray_ptr = *mut jvirt_sarray_control;
pub type jvirt_barray_ptr = *mut jvirt_barray_control;

#[repr(C)]
pub struct jpeg_marker_struct {
    pub next: jpeg_saved_marker_ptr,
    pub marker: UINT8,
    pub original_length: ::core::ffi::c_uint,
    pub data_length: ::core::ffi::c_uint,
    pub data: *mut JOCTET,
}

pub type jpeg_saved_marker_ptr = *mut jpeg_marker_struct;

#[repr(C)]
pub struct jpeg_error_mgr_msg_parm {
    pub i: [int; JPEG_MSG_PARMS_MAX],
    pub s: [::core::ffi::c_char; JMSG_STR_PARM_MAX],
}

#[repr(C)]
pub union jpeg_error_mgr_msg_parm_union {
    pub i: [int; JPEG_MSG_PARMS_MAX],
    pub s: [::core::ffi::c_char; JMSG_STR_PARM_MAX],
}

pub type jpeg_marker_parser_method =
    Option<unsafe extern "C" fn(cinfo: j_decompress_ptr) -> boolean>;
pub type inverse_DCT_method_ptr = Option<
    unsafe extern "C" fn(
        cinfo: j_decompress_ptr,
        compptr: *mut jpeg_component_info,
        coef_block: JCOEFPTR,
        output_buf: JSAMPARRAY,
        output_col: JDIMENSION,
    ),
>;

pub type backing_store_ptr = *mut backing_store_info;

#[repr(C)]
pub struct backing_store_info {
    pub read_backing_store: Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            info: backing_store_ptr,
            buffer_address: *mut c_void,
            file_offset: long,
            byte_count: long,
        ),
    >,
    pub write_backing_store: Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            info: backing_store_ptr,
            buffer_address: *mut c_void,
            file_offset: long,
            byte_count: long,
        ),
    >,
    pub close_backing_store:
        Option<unsafe extern "C" fn(cinfo: j_common_ptr, info: backing_store_ptr)>,
    pub temp_file: *mut FILE,
    pub temp_name: [::core::ffi::c_char; TEMP_NAME_LENGTH],
}

#[repr(C)]
pub struct JQUANT_TBL {
    pub quantval: [UINT16; DCTSIZE2],
    pub sent_table: boolean,
}

#[repr(C)]
pub struct JHUFF_TBL {
    pub bits: [UINT8; 17],
    pub huffval: [UINT8; 256],
    pub sent_table: boolean,
}

#[repr(C)]
pub struct jpeg_component_info {
    pub component_id: int,
    pub component_index: int,
    pub h_samp_factor: int,
    pub v_samp_factor: int,
    pub quant_tbl_no: int,
    pub dc_tbl_no: int,
    pub ac_tbl_no: int,
    pub width_in_blocks: JDIMENSION,
    pub height_in_blocks: JDIMENSION,
    pub DCT_h_scaled_size: int,
    pub DCT_v_scaled_size: int,
    pub downsampled_width: JDIMENSION,
    pub downsampled_height: JDIMENSION,
    pub component_needed: boolean,
    pub MCU_width: int,
    pub MCU_height: int,
    pub MCU_blocks: int,
    pub MCU_sample_width: int,
    pub last_col_width: int,
    pub last_row_height: int,
    pub quant_table: *mut JQUANT_TBL,
    pub dct_table: *mut c_void,
}

#[repr(C)]
pub struct jpeg_scan_info {
    pub comps_in_scan: int,
    pub component_index: [int; MAX_COMPS_IN_SCAN],
    pub Ss: int,
    pub Se: int,
    pub Ah: int,
    pub Al: int,
}

#[repr(C)]
pub struct jpeg_error_mgr {
    pub error_exit: Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub emit_message: Option<unsafe extern "C" fn(cinfo: j_common_ptr, msg_level: int)>,
    pub output_message: Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub format_message:
        Option<unsafe extern "C" fn(cinfo: j_common_ptr, buffer: *mut ::core::ffi::c_char)>,
    pub reset_error_mgr: Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub msg_code: int,
    pub msg_parm: jpeg_error_mgr_msg_parm_union,
    pub trace_level: int,
    pub num_warnings: long,
    pub jpeg_message_table: *const *const ::core::ffi::c_char,
    pub last_jpeg_message: int,
    pub addon_message_table: *const *const ::core::ffi::c_char,
    pub first_addon_message: int,
    pub last_addon_message: int,
}

#[repr(C)]
pub struct jpeg_progress_mgr {
    pub progress_monitor: Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub pass_counter: long,
    pub pass_limit: long,
    pub completed_passes: int,
    pub total_passes: int,
}

#[repr(C)]
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut JOCTET,
    pub free_in_buffer: size_t,
    pub init_destination: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub empty_output_buffer: Option<unsafe extern "C" fn(cinfo: j_compress_ptr) -> boolean>,
    pub term_destination: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
}

#[repr(C)]
pub struct jpeg_source_mgr {
    pub next_input_byte: *const JOCTET,
    pub bytes_in_buffer: size_t,
    pub init_source: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub fill_input_buffer: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr) -> boolean>,
    pub skip_input_data: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr, num_bytes: long)>,
    pub resync_to_restart:
        Option<unsafe extern "C" fn(cinfo: j_decompress_ptr, desired: int) -> boolean>,
    pub term_source: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
}

#[repr(C)]
pub struct jpeg_memory_mgr {
    pub alloc_small: Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: int,
            sizeofobject: size_t,
        ) -> *mut c_void,
    >,
    pub alloc_large: Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: int,
            sizeofobject: size_t,
        ) -> *mut c_void,
    >,
    pub alloc_sarray: Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: int,
            samplesperrow: JDIMENSION,
            numrows: JDIMENSION,
        ) -> JSAMPARRAY,
    >,
    pub alloc_barray: Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: int,
            blocksperrow: JDIMENSION,
            numrows: JDIMENSION,
        ) -> JBLOCKARRAY,
    >,
    pub request_virt_sarray: Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: int,
            pre_zero: boolean,
            samplesperrow: JDIMENSION,
            numrows: JDIMENSION,
            maxaccess: JDIMENSION,
        ) -> jvirt_sarray_ptr,
    >,
    pub request_virt_barray: Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: int,
            pre_zero: boolean,
            blocksperrow: JDIMENSION,
            numrows: JDIMENSION,
            maxaccess: JDIMENSION,
        ) -> jvirt_barray_ptr,
    >,
    pub realize_virt_arrays: Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub access_virt_sarray: Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            ptr: jvirt_sarray_ptr,
            start_row: JDIMENSION,
            num_rows: JDIMENSION,
            writable: boolean,
        ) -> JSAMPARRAY,
    >,
    pub access_virt_barray: Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            ptr: jvirt_barray_ptr,
            start_row: JDIMENSION,
            num_rows: JDIMENSION,
            writable: boolean,
        ) -> JBLOCKARRAY,
    >,
    pub free_pool: Option<unsafe extern "C" fn(cinfo: j_common_ptr, pool_id: int)>,
    pub self_destruct: Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub max_memory_to_use: long,
    pub max_alloc_chunk: long,
}

#[repr(C)]
pub struct jpeg_common_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut c_void,
    pub is_decompressor: boolean,
    pub global_state: int,
}

#[repr(C)]
pub struct jpeg_compress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut c_void,
    pub is_decompressor: boolean,
    pub global_state: int,
    pub dest: *mut jpeg_destination_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub input_components: int,
    pub in_color_space: J_COLOR_SPACE,
    pub input_gamma: f64,
    pub scale_num: ::core::ffi::c_uint,
    pub scale_denom: ::core::ffi::c_uint,
    pub jpeg_width: JDIMENSION,
    pub jpeg_height: JDIMENSION,
    pub data_precision: int,
    pub num_components: int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub comp_info: *mut jpeg_component_info,
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; NUM_QUANT_TBLS],
    pub q_scale_factor: [int; NUM_QUANT_TBLS],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; NUM_HUFF_TBLS],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; NUM_HUFF_TBLS],
    pub arith_dc_L: [UINT8; NUM_ARITH_TBLS],
    pub arith_dc_U: [UINT8; NUM_ARITH_TBLS],
    pub arith_ac_K: [UINT8; NUM_ARITH_TBLS],
    pub num_scans: int,
    pub scan_info: *const jpeg_scan_info,
    pub raw_data_in: boolean,
    pub arith_code: boolean,
    pub optimize_coding: boolean,
    pub CCIR601_sampling: boolean,
    pub do_fancy_downsampling: boolean,
    pub smoothing_factor: int,
    pub dct_method: J_DCT_METHOD,
    pub restart_interval: ::core::ffi::c_uint,
    pub restart_in_rows: int,
    pub write_JFIF_header: boolean,
    pub JFIF_major_version: UINT8,
    pub JFIF_minor_version: UINT8,
    pub density_unit: UINT8,
    pub X_density: UINT16,
    pub Y_density: UINT16,
    pub write_Adobe_marker: boolean,
    pub next_scanline: JDIMENSION,
    pub progressive_mode: boolean,
    pub max_h_samp_factor: int,
    pub max_v_samp_factor: int,
    pub min_DCT_h_scaled_size: int,
    pub min_DCT_v_scaled_size: int,
    pub total_iMCU_rows: JDIMENSION,
    pub comps_in_scan: int,
    pub cur_comp_info: [*mut jpeg_component_info; MAX_COMPS_IN_SCAN],
    pub MCUs_per_row: JDIMENSION,
    pub MCU_rows_in_scan: JDIMENSION,
    pub blocks_in_MCU: int,
    pub MCU_membership: [int; C_MAX_BLOCKS_IN_MCU],
    pub Ss: int,
    pub Se: int,
    pub Ah: int,
    pub Al: int,
    pub block_size: int,
    pub natural_order: *const int,
    pub lim_Se: int,
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
    pub script_space_size: int,
}

#[repr(C)]
pub struct jpeg_decompress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut c_void,
    pub is_decompressor: boolean,
    pub global_state: int,
    pub src: *mut jpeg_source_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub num_components: int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub out_color_space: J_COLOR_SPACE,
    pub scale_num: ::core::ffi::c_uint,
    pub scale_denom: ::core::ffi::c_uint,
    pub output_gamma: f64,
    pub buffered_image: boolean,
    pub raw_data_out: boolean,
    pub dct_method: J_DCT_METHOD,
    pub do_fancy_upsampling: boolean,
    pub do_block_smoothing: boolean,
    pub quantize_colors: boolean,
    pub dither_mode: J_DITHER_MODE,
    pub two_pass_quantize: boolean,
    pub desired_number_of_colors: int,
    pub enable_1pass_quant: boolean,
    pub enable_external_quant: boolean,
    pub enable_2pass_quant: boolean,
    pub output_width: JDIMENSION,
    pub output_height: JDIMENSION,
    pub out_color_components: int,
    pub output_components: int,
    pub rec_outbuf_height: int,
    pub actual_number_of_colors: int,
    pub colormap: JSAMPARRAY,
    pub output_scanline: JDIMENSION,
    pub input_scan_number: int,
    pub input_iMCU_row: JDIMENSION,
    pub output_scan_number: int,
    pub output_iMCU_row: JDIMENSION,
    pub coef_bits: *mut [int; DCTSIZE2],
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; NUM_QUANT_TBLS],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; NUM_HUFF_TBLS],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; NUM_HUFF_TBLS],
    pub data_precision: int,
    pub comp_info: *mut jpeg_component_info,
    pub is_baseline: boolean,
    pub progressive_mode: boolean,
    pub arith_code: boolean,
    pub arith_dc_L: [UINT8; NUM_ARITH_TBLS],
    pub arith_dc_U: [UINT8; NUM_ARITH_TBLS],
    pub arith_ac_K: [UINT8; NUM_ARITH_TBLS],
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
    pub max_h_samp_factor: int,
    pub max_v_samp_factor: int,
    pub min_DCT_h_scaled_size: int,
    pub min_DCT_v_scaled_size: int,
    pub total_iMCU_rows: JDIMENSION,
    pub sample_range_limit: *mut JSAMPLE,
    pub comps_in_scan: int,
    pub cur_comp_info: [*mut jpeg_component_info; MAX_COMPS_IN_SCAN],
    pub MCUs_per_row: JDIMENSION,
    pub MCU_rows_in_scan: JDIMENSION,
    pub blocks_in_MCU: int,
    pub MCU_membership: [int; D_MAX_BLOCKS_IN_MCU],
    pub Ss: int,
    pub Se: int,
    pub Ah: int,
    pub Al: int,
    pub block_size: int,
    pub natural_order: *const int,
    pub lim_Se: int,
    pub unread_marker: int,
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

pub type j_common_ptr = *mut jpeg_common_struct;
pub type j_compress_ptr = *mut jpeg_compress_struct;
pub type j_decompress_ptr = *mut jpeg_decompress_struct;

#[repr(C)]
pub struct jpeg_comp_master {
    pub prepare_for_pass: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub pass_startup: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub finish_pass: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub call_pass_startup: boolean,
    pub is_last_pass: boolean,
}

#[repr(C)]
pub struct jpeg_c_main_controller {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_compress_ptr, pass_mode: J_BUF_MODE)>,
    pub process_data: Option<
        unsafe extern "C" fn(
            cinfo: j_compress_ptr,
            input_buf: JSAMPARRAY,
            in_row_ctr: *mut JDIMENSION,
            in_rows_avail: JDIMENSION,
        ),
    >,
}

#[repr(C)]
pub struct jpeg_c_prep_controller {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_compress_ptr, pass_mode: J_BUF_MODE)>,
    pub pre_process_data: Option<
        unsafe extern "C" fn(
            cinfo: j_compress_ptr,
            input_buf: JSAMPARRAY,
            in_row_ctr: *mut JDIMENSION,
            in_rows_avail: JDIMENSION,
            output_buf: JSAMPIMAGE,
            out_row_group_ctr: *mut JDIMENSION,
            out_row_groups_avail: JDIMENSION,
        ),
    >,
}

#[repr(C)]
pub struct jpeg_c_coef_controller {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_compress_ptr, pass_mode: J_BUF_MODE)>,
    pub compress_data:
        Option<unsafe extern "C" fn(cinfo: j_compress_ptr, input_buf: JSAMPIMAGE) -> boolean>,
}

#[repr(C)]
pub struct jpeg_color_converter {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub color_convert: Option<
        unsafe extern "C" fn(
            cinfo: j_compress_ptr,
            input_buf: JSAMPARRAY,
            output_buf: JSAMPIMAGE,
            output_row: JDIMENSION,
            num_rows: int,
        ),
    >,
}

#[repr(C)]
pub struct jpeg_downsampler {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub downsample: Option<
        unsafe extern "C" fn(
            cinfo: j_compress_ptr,
            input_buf: JSAMPIMAGE,
            in_row_index: JDIMENSION,
            output_buf: JSAMPIMAGE,
            out_row_group_index: JDIMENSION,
        ),
    >,
    pub need_context_rows: boolean,
}

#[repr(C)]
pub struct jpeg_forward_dct {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub forward_DCT: Option<
        unsafe extern "C" fn(
            cinfo: j_compress_ptr,
            compptr: *mut jpeg_component_info,
            sample_data: JSAMPARRAY,
            coef_blocks: JBLOCKROW,
            start_row: JDIMENSION,
            start_col: JDIMENSION,
            num_blocks: JDIMENSION,
        ),
    >,
}

#[repr(C)]
pub struct jpeg_entropy_encoder {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_compress_ptr, gather_statistics: boolean)>,
    pub encode_mcu:
        Option<unsafe extern "C" fn(cinfo: j_compress_ptr, MCU_data: *mut JBLOCKROW) -> boolean>,
    pub finish_pass: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
}

#[repr(C)]
pub struct jpeg_marker_writer {
    pub write_file_header: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub write_frame_header: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub write_scan_header: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub write_file_trailer: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub write_tables_only: Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub write_marker_header: Option<
        unsafe extern "C" fn(cinfo: j_compress_ptr, marker: int, datalen: ::core::ffi::c_uint),
    >,
    pub write_marker_byte: Option<unsafe extern "C" fn(cinfo: j_compress_ptr, val: int)>,
}

#[repr(C)]
pub struct jpeg_decomp_master {
    pub prepare_for_output_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub finish_output_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub is_dummy_pass: boolean,
    pub first_iMCU_col: JDIMENSION,
    pub last_iMCU_col: JDIMENSION,
    pub first_MCU_col: [JDIMENSION; MAX_COMPONENTS],
    pub last_MCU_col: [JDIMENSION; MAX_COMPONENTS],
    pub jinit_upsampler_no_alloc: boolean,
    pub last_good_iMCU_row: JDIMENSION,
}

#[repr(C)]
pub struct jpeg_input_controller {
    pub consume_input: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr) -> int>,
    pub reset_input_controller: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub start_input_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub finish_input_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub has_multiple_scans: boolean,
    pub eoi_reached: boolean,
}

#[repr(C)]
pub struct jpeg_d_main_controller {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr, pass_mode: J_BUF_MODE)>,
    pub process_data: Option<
        unsafe extern "C" fn(
            cinfo: j_decompress_ptr,
            output_buf: JSAMPARRAY,
            out_row_ctr: *mut JDIMENSION,
            out_rows_avail: JDIMENSION,
        ),
    >,
}

#[repr(C)]
pub struct jpeg_d_coef_controller {
    pub start_input_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub consume_data: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr) -> int>,
    pub start_output_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub decompress_data:
        Option<unsafe extern "C" fn(cinfo: j_decompress_ptr, output_buf: JSAMPIMAGE) -> int>,
    pub coef_arrays: *mut jvirt_barray_ptr,
}

#[repr(C)]
pub struct jpeg_d_post_controller {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr, pass_mode: J_BUF_MODE)>,
    pub post_process_data: Option<
        unsafe extern "C" fn(
            cinfo: j_decompress_ptr,
            input_buf: JSAMPIMAGE,
            in_row_group_ctr: *mut JDIMENSION,
            in_row_groups_avail: JDIMENSION,
            output_buf: JSAMPARRAY,
            out_row_ctr: *mut JDIMENSION,
            out_rows_avail: JDIMENSION,
        ),
    >,
}

#[repr(C)]
pub struct jpeg_marker_reader {
    pub reset_marker_reader: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub read_markers: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr) -> int>,
    pub read_restart_marker: jpeg_marker_parser_method,
    pub saw_SOI: boolean,
    pub saw_SOF: boolean,
    pub next_restart_num: int,
    pub discarded_bytes: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct jpeg_entropy_decoder {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub decode_mcu:
        Option<unsafe extern "C" fn(cinfo: j_decompress_ptr, MCU_data: *mut JBLOCKROW) -> boolean>,
    pub insufficient_data: boolean,
}

#[repr(C)]
pub struct jpeg_inverse_dct {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub inverse_DCT: [inverse_DCT_method_ptr; MAX_COMPONENTS],
}

#[repr(C)]
pub struct jpeg_upsampler {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub upsample: Option<
        unsafe extern "C" fn(
            cinfo: j_decompress_ptr,
            input_buf: JSAMPIMAGE,
            in_row_group_ctr: *mut JDIMENSION,
            in_row_groups_avail: JDIMENSION,
            output_buf: JSAMPARRAY,
            out_row_ctr: *mut JDIMENSION,
            out_rows_avail: JDIMENSION,
        ),
    >,
    pub need_context_rows: boolean,
}

#[repr(C)]
pub struct jpeg_color_deconverter {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub color_convert: Option<
        unsafe extern "C" fn(
            cinfo: j_decompress_ptr,
            input_buf: JSAMPIMAGE,
            input_row: JDIMENSION,
            output_buf: JSAMPARRAY,
            num_rows: int,
        ),
    >,
}

#[repr(C)]
pub struct jpeg_color_quantizer {
    pub start_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr, is_pre_scan: boolean)>,
    pub color_quantize: Option<
        unsafe extern "C" fn(
            cinfo: j_decompress_ptr,
            input_buf: JSAMPARRAY,
            output_buf: JSAMPARRAY,
            num_rows: int,
        ),
    >,
    pub finish_pass: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub new_color_map: Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
}

#[repr(i32)]
pub enum J_MESSAGE_CODE {
    JMSG_NOMESSAGE = 0,
    JERR_BAD_ALIGN_TYPE,
    JERR_BAD_ALLOC_CHUNK,
    JERR_BAD_BUFFER_MODE,
    JERR_BAD_COMPONENT_ID,
    JERR_BAD_CROP_SPEC,
    JERR_BAD_DCT_COEF,
    JERR_BAD_DCTSIZE,
    JERR_BAD_DROP_SAMPLING,
    JERR_BAD_HUFF_TABLE,
    JERR_BAD_IN_COLORSPACE,
    JERR_BAD_J_COLORSPACE,
    JERR_BAD_LENGTH,
    JERR_BAD_LIB_VERSION,
    JERR_BAD_MCU_SIZE,
    JERR_BAD_POOL_ID,
    JERR_BAD_PRECISION,
    JERR_BAD_PROGRESSION,
    JERR_BAD_PROG_SCRIPT,
    JERR_BAD_SAMPLING,
    JERR_BAD_SCAN_SCRIPT,
    JERR_BAD_STATE,
    JERR_BAD_STRUCT_SIZE,
    JERR_BAD_VIRTUAL_ACCESS,
    JERR_BUFFER_SIZE,
    JERR_CANT_SUSPEND,
    JERR_CCIR601_NOTIMPL,
    JERR_COMPONENT_COUNT,
    JERR_CONVERSION_NOTIMPL,
    JERR_DAC_INDEX,
    JERR_DAC_VALUE,
    JERR_DHT_INDEX,
    JERR_DQT_INDEX,
    JERR_EMPTY_IMAGE,
    JERR_EMS_READ,
    JERR_EMS_WRITE,
    JERR_EOI_EXPECTED,
    JERR_FILE_READ,
    JERR_FILE_WRITE,
    JERR_FRACT_SAMPLE_NOTIMPL,
    JERR_HUFF_CLEN_OVERFLOW,
    JERR_HUFF_MISSING_CODE,
    JERR_IMAGE_TOO_BIG,
    JERR_INPUT_EMPTY,
    JERR_INPUT_EOF,
    JERR_MISMATCHED_QUANT_TABLE,
    JERR_MISSING_DATA,
    JERR_MODE_CHANGE,
    JERR_NOTIMPL,
    JERR_NOT_COMPILED,
    JERR_NO_ARITH_TABLE,
    JERR_NO_BACKING_STORE,
    JERR_NO_HUFF_TABLE,
    JERR_NO_IMAGE,
    JERR_NO_QUANT_TABLE,
    JERR_NO_SOI,
    JERR_OUT_OF_MEMORY,
    JERR_QUANT_COMPONENTS,
    JERR_QUANT_FEW_COLORS,
    JERR_QUANT_MANY_COLORS,
    JERR_SOF_DUPLICATE,
    JERR_SOF_NO_SOS,
    JERR_SOF_UNSUPPORTED,
    JERR_SOI_DUPLICATE,
    JERR_SOS_NO_SOF,
    JERR_TFILE_CREATE,
    JERR_TFILE_READ,
    JERR_TFILE_SEEK,
    JERR_TFILE_WRITE,
    JERR_TOO_LITTLE_DATA,
    JERR_UNKNOWN_MARKER,
    JERR_VIRTUAL_BUG,
    JERR_WIDTH_OVERFLOW,
    JERR_XMS_READ,
    JERR_XMS_WRITE,
    JMSG_COPYRIGHT,
    JMSG_VERSION,
    JTRC_16BIT_TABLES,
    JTRC_ADOBE,
    JTRC_APP0,
    JTRC_APP14,
    JTRC_DAC,
    JTRC_DHT,
    JTRC_DQT,
    JTRC_DRI,
    JTRC_EMS_CLOSE,
    JTRC_EMS_OPEN,
    JTRC_EOI,
    JTRC_HUFFBITS,
    JTRC_JFIF,
    JTRC_JFIF_BADTHUMBNAILSIZE,
    JTRC_JFIF_EXTENSION,
    JTRC_JFIF_THUMBNAIL,
    JTRC_MISC_MARKER,
    JTRC_PARMLESS_MARKER,
    JTRC_QUANTVALS,
    JTRC_QUANT_3_NCOLORS,
    JTRC_QUANT_NCOLORS,
    JTRC_QUANT_SELECTED,
    JTRC_RECOVERY_ACTION,
    JTRC_RST,
    JTRC_SMOOTH_NOTIMPL,
    JTRC_SOF,
    JTRC_SOF_COMPONENT,
    JTRC_SOI,
    JTRC_SOS,
    JTRC_SOS_COMPONENT,
    JTRC_SOS_PARAMS,
    JTRC_TFILE_CLOSE,
    JTRC_TFILE_OPEN,
    JTRC_THUMB_JPEG,
    JTRC_THUMB_PALETTE,
    JTRC_THUMB_RGB,
    JTRC_UNKNOWN_IDS,
    JTRC_XMS_CLOSE,
    JTRC_XMS_OPEN,
    JWRN_ADOBE_XFORM,
    JWRN_ARITH_BAD_CODE,
    JWRN_BOGUS_PROGRESSION,
    JWRN_EXTRANEOUS_DATA,
    JWRN_HIT_MARKER,
    JWRN_HUFF_BAD_CODE,
    JWRN_JFIF_MAJOR,
    JWRN_JPEG_EOF,
    JWRN_MUST_RESYNC,
    JWRN_NOT_SEQUENTIAL,
    JWRN_TOO_MUCH_DATA,
    JWRN_BOGUS_ICC,
    JMSG_LASTMSGCODE,
}

pub const JMSG_LASTMSGCODE: int = J_MESSAGE_CODE::JMSG_LASTMSGCODE as int;
pub const JPEG_STD_MESSAGE_TABLE_LEN: usize = JMSG_LASTMSGCODE as usize + 1;
