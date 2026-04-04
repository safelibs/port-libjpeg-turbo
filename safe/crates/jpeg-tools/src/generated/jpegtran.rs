#[repr(C)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_entropy_encoder {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_forward_dct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_downsampler {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_color_converter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_marker_writer {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_c_coef_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_c_prep_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_c_main_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_comp_master {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_color_quantizer {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_color_deconverter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_upsampler {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_inverse_dct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_entropy_decoder {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_marker_reader {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_input_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_d_post_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_d_coef_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_d_main_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_decomp_master {
    _unused: [u8; 0],
}
extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn jpeg_std_error(err: *mut jpeg_error_mgr) -> *mut jpeg_error_mgr;
    fn jpeg_CreateCompress(cinfo: j_compress_ptr, version: ::core::ffi::c_int, structsize: size_t);
    fn jpeg_CreateDecompress(
        cinfo: j_decompress_ptr,
        version: ::core::ffi::c_int,
        structsize: size_t,
    );
    fn jpeg_destroy_compress(cinfo: j_compress_ptr);
    fn jpeg_destroy_decompress(cinfo: j_decompress_ptr);
    fn jpeg_stdio_dest(cinfo: j_compress_ptr, outfile: *mut FILE);
    fn jpeg_stdio_src(cinfo: j_decompress_ptr, infile: *mut FILE);
    fn jpeg_simple_progression(cinfo: j_compress_ptr);
    fn jpeg_finish_compress(cinfo: j_compress_ptr);
    fn jpeg_write_icc_profile(
        cinfo: j_compress_ptr,
        icc_data_ptr: *const JOCTET,
        icc_data_len: ::core::ffi::c_uint,
    );
    fn jpeg_read_header(cinfo: j_decompress_ptr, require_image: boolean) -> ::core::ffi::c_int;
    fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean;
    fn jpeg_read_coefficients(cinfo: j_decompress_ptr) -> *mut jvirt_barray_ptr;
    fn jpeg_write_coefficients(cinfo: j_compress_ptr, coef_arrays: *mut jvirt_barray_ptr);
    fn jpeg_copy_critical_parameters(srcinfo: j_decompress_ptr, dstinfo: j_compress_ptr);
    fn read_scan_script(cinfo: j_compress_ptr, filename: *mut ::core::ffi::c_char) -> boolean;
    fn start_progress_monitor(cinfo: j_common_ptr, progress: cd_progress_ptr);
    fn end_progress_monitor(cinfo: j_common_ptr);
    fn keymatch(
        arg: *mut ::core::ffi::c_char,
        keyword: *const ::core::ffi::c_char,
        minchars: ::core::ffi::c_int,
    ) -> boolean;
    fn read_stdin() -> *mut FILE;
    fn write_stdout() -> *mut FILE;
    fn jtransform_parse_crop_spec(
        info: *mut jpeg_transform_info,
        spec: *const ::core::ffi::c_char,
    ) -> boolean;
    fn jtransform_request_workspace(
        srcinfo: j_decompress_ptr,
        info: *mut jpeg_transform_info,
    ) -> boolean;
    fn jtransform_adjust_parameters(
        srcinfo: j_decompress_ptr,
        dstinfo: j_compress_ptr,
        src_coef_arrays: *mut jvirt_barray_ptr,
        info: *mut jpeg_transform_info,
    ) -> *mut jvirt_barray_ptr;
    fn jtransform_execute_transform(
        srcinfo: j_decompress_ptr,
        dstinfo: j_compress_ptr,
        src_coef_arrays: *mut jvirt_barray_ptr,
        info: *mut jpeg_transform_info,
    );
    fn jcopy_markers_setup(srcinfo: j_decompress_ptr, option: JCOPY_OPTION);
    fn jcopy_markers_execute(
        srcinfo: j_decompress_ptr,
        dstinfo: j_compress_ptr,
        option: JCOPY_OPTION,
    );
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type JSAMPLE = ::core::ffi::c_uchar;
pub type JCOEF = ::core::ffi::c_short;
pub type JOCTET = ::core::ffi::c_uchar;
pub type UINT8 = ::core::ffi::c_uchar;
pub type UINT16 = ::core::ffi::c_ushort;
pub type JDIMENSION = ::core::ffi::c_uint;
pub type boolean = ::core::ffi::c_int;
pub type JSAMPROW = *mut JSAMPLE;
pub type JSAMPARRAY = *mut JSAMPROW;
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
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut JOCTET,
    pub free_in_buffer: size_t,
    pub init_destination: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub empty_output_buffer: Option<unsafe extern "C" fn(j_compress_ptr) -> boolean>,
    pub term_destination: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
}
pub type j_compress_ptr = *mut jpeg_compress_struct;
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
pub type j_decompress_ptr = *mut jpeg_decompress_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdjpeg_progress_mgr {
    pub pub_0: jpeg_progress_mgr,
    pub completed_extra_passes: ::core::ffi::c_int,
    pub total_extra_passes: ::core::ffi::c_int,
    pub max_scans: JDIMENSION,
    pub report: boolean,
    pub percent_done: ::core::ffi::c_int,
}
pub type cd_progress_ptr = *mut cdjpeg_progress_mgr;
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
pub type JCOPY_OPTION = ::core::ffi::c_uint;
pub const JCOPYOPT_ICC: JCOPY_OPTION = 4;
pub const JCOPYOPT_ALL_EXCEPT_ICC: JCOPY_OPTION = 3;
pub const JCOPYOPT_ALL: JCOPY_OPTION = 2;
pub const JCOPYOPT_COMMENTS: JCOPY_OPTION = 1;
pub const JCOPYOPT_NONE: JCOPY_OPTION = 0;
pub const JPEG_LIB_VERSION: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const READ_BINARY: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const WRITE_BINARY: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"wb\0") };
pub const EXIT_WARNING: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const JVERSION: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"8d  15-Jan-2012\0") };
pub const JCOPYRIGHT: [::core::ffi::c_char; 547] = unsafe {
    ::core::mem::transmute::<
        [u8; 547],
        [::core::ffi::c_char; 547],
    >(
        *b"Copyright (C) 2009-2023 D. R. Commander\nCopyright (C) 2015, 2020 Google, Inc.\nCopyright (C) 2019-2020 Arm Limited\nCopyright (C) 2015-2016, 2018 Matthieu Darbois\nCopyright (C) 2011-2016 Siarhei Siamashka\nCopyright (C) 2015 Intel Corporation\nCopyright (C) 2013-2014 Linaro Limited\nCopyright (C) 2013-2014 MIPS Technologies, Inc.\nCopyright (C) 2009, 2012 Pierre Ossman for Cendio AB\nCopyright (C) 2009-2011 Nokia Corporation and/or its subsidiary(-ies)\nCopyright (C) 1999-2006 MIYASAKA Masaru\nCopyright (C) 1991-2020 Thomas G. Lane, Guido Vollbeding\0",
    )
};
pub const BUILD: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"20260403\0") };
pub const PACKAGE_NAME: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"libjpeg-turbo\0") };
pub const VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"2.1.5\0") };
static mut progname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut icc_filename: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut max_scans: JDIMENSION = 0;
static mut outfilename: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut dropfilename: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut report: boolean = 0;
#[no_mangle]
pub static mut strict: boolean = 0;
static mut copyoption: JCOPY_OPTION = JCOPYOPT_NONE;
static mut transformoption: jpeg_transform_info = jpeg_transform_info {
    transform: JXFORM_NONE,
    perfect: 0,
    trim: 0,
    force_grayscale: 0,
    crop: 0,
    slow_hflip: 0,
    crop_width: 0,
    crop_width_set: JCROP_UNSET,
    crop_height: 0,
    crop_height_set: JCROP_UNSET,
    crop_xoffset: 0,
    crop_xoffset_set: JCROP_UNSET,
    crop_yoffset: 0,
    crop_yoffset_set: JCROP_UNSET,
    drop_ptr: ::core::ptr::null::<jpeg_decompress_struct>() as *mut jpeg_decompress_struct,
    drop_coef_arrays: ::core::ptr::null::<jvirt_barray_ptr>() as *mut jvirt_barray_ptr,
    num_components: 0,
    workspace_coef_arrays: ::core::ptr::null::<jvirt_barray_ptr>() as *mut jvirt_barray_ptr,
    output_width: 0,
    output_height: 0,
    x_crop_offset: 0,
    y_crop_offset: 0,
    drop_width: 0,
    drop_height: 0,
    iMCU_sample_width: 0,
    iMCU_sample_height: 0,
};
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"usage: %s [switches] \0" as *const u8 as *const ::core::ffi::c_char,
        progname,
    );
    fprintf(
        stderr,
        b"[inputfile]\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"Switches (names may be abbreviated):\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -copy none     Copy no extra markers from source file\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -copy comments Copy only comment markers (default)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -copy icc      Copy only ICC profile markers\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -copy all      Copy all extra markers\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -optimize      Optimize Huffman table (smaller file, but slow compression)\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -progressive   Create progressive JPEG file\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"Switches for modifying the image:\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -crop WxH+X+Y  Crop to a rectangular region\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -drop +X+Y filename          Drop (insert) another image\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -flip [horizontal|vertical]  Mirror image (left-right or top-bottom)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -grayscale     Reduce to grayscale (omit color data)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -perfect       Fail if there is non-transformable edge blocks\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -rotate [90|180|270]         Rotate image (degrees clockwise)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -transpose     Transpose image\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -transverse    Transverse transpose image\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -trim          Drop non-transformable edge blocks\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"                 with -drop: Requantize drop file to match source file\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -wipe WxH+X+Y  Wipe (gray out) a rectangular region\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"Switches for advanced users:\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -arithmetic    Use arithmetic coding\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -icc FILE      Embed ICC profile contained in FILE\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -restart N     Set restart interval in rows, or in blocks with B\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -maxmemory N   Maximum memory to use (in kbytes)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -maxscans N    Maximum number of scans to allow in input file\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -outfile name  Specify name for output file\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -report        Report transformation progress\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -strict        Treat all warnings as fatal\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -verbose  or  -debug   Emit debug output\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -version       Print version information and exit\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"Switches for wizards:\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -scans FILE    Create multi-scan JPEG per script FILE\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    exit(EXIT_FAILURE);
}
unsafe extern "C" fn select_transform(mut transform: JXFORM_CODE) {
    if transformoption.transform as ::core::ffi::c_uint
        == JXFORM_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        || transformoption.transform as ::core::ffi::c_uint == transform as ::core::ffi::c_uint
    {
        transformoption.transform = transform;
    } else {
        fprintf(
            stderr,
            b"%s: can only do one image transformation at a time\n\0" as *const u8
                as *const ::core::ffi::c_char,
            progname,
        );
        usage();
    };
}
unsafe extern "C" fn parse_switches(
    mut cinfo: j_compress_ptr,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut last_file_arg_seen: ::core::ffi::c_int,
    mut for_real: boolean,
) -> ::core::ffi::c_int {
    let mut argn: ::core::ffi::c_int = 0;
    let mut arg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut simple_progressive: boolean = 0;
    let mut scansarg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    simple_progressive = FALSE as boolean;
    icc_filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    max_scans = 0 as JDIMENSION;
    outfilename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    report = FALSE as boolean;
    strict = FALSE as boolean;
    copyoption = JCOPYOPT_COMMENTS;
    transformoption.transform = JXFORM_NONE;
    transformoption.perfect = FALSE as boolean;
    transformoption.trim = FALSE as boolean;
    transformoption.force_grayscale = FALSE as boolean;
    transformoption.crop = FALSE as boolean;
    transformoption.slow_hflip = FALSE as boolean;
    (*(*cinfo).err).trace_level = 0 as ::core::ffi::c_int;
    argn = 1 as ::core::ffi::c_int;
    while argn < argc {
        arg = *argv.offset(argn as isize);
        if *arg as ::core::ffi::c_int != '-' as i32 {
            if !(argn <= last_file_arg_seen) {
                break;
            }
            outfilename = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            arg = arg.offset(1);
            if keymatch(
                arg,
                b"arithmetic\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                (*cinfo).arith_code = TRUE as boolean;
            } else if keymatch(
                arg,
                b"copy\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if keymatch(
                    *argv.offset(argn as isize),
                    b"none\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
                {
                    copyoption = JCOPYOPT_NONE;
                } else if keymatch(
                    *argv.offset(argn as isize),
                    b"comments\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
                {
                    copyoption = JCOPYOPT_COMMENTS;
                } else if keymatch(
                    *argv.offset(argn as isize),
                    b"icc\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
                {
                    copyoption = JCOPYOPT_ICC;
                } else if keymatch(
                    *argv.offset(argn as isize),
                    b"all\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
                {
                    copyoption = JCOPYOPT_ALL;
                } else {
                    usage();
                }
            } else if keymatch(
                arg,
                b"crop\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if transformoption.crop != 0
                    || jtransform_parse_crop_spec(
                        &raw mut transformoption,
                        *argv.offset(argn as isize),
                    ) == 0
                {
                    fprintf(
                        stderr,
                        b"%s: bogus -crop argument '%s'\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        progname,
                        *argv.offset(argn as isize),
                    );
                    exit(EXIT_FAILURE);
                }
            } else if keymatch(
                arg,
                b"drop\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if transformoption.crop != 0
                    || jtransform_parse_crop_spec(
                        &raw mut transformoption,
                        *argv.offset(argn as isize),
                    ) == 0
                    || transformoption.crop_width_set as ::core::ffi::c_uint
                        != JCROP_UNSET as ::core::ffi::c_int as ::core::ffi::c_uint
                    || transformoption.crop_height_set as ::core::ffi::c_uint
                        != JCROP_UNSET as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    fprintf(
                        stderr,
                        b"%s: bogus -drop argument '%s'\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        progname,
                        *argv.offset(argn as isize),
                    );
                    exit(EXIT_FAILURE);
                }
                argn += 1;
                if argn >= argc {
                    usage();
                }
                dropfilename = *argv.offset(argn as isize);
                select_transform(JXFORM_DROP);
            } else if keymatch(
                arg,
                b"debug\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
                || keymatch(
                    arg,
                    b"verbose\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
            {
                static mut printed_version: boolean = FALSE;
                if printed_version == 0 {
                    fprintf(
                        stderr,
                        b"%s version %s (build %s)\n\0" as *const u8 as *const ::core::ffi::c_char,
                        PACKAGE_NAME.as_ptr(),
                        VERSION.as_ptr(),
                        BUILD.as_ptr(),
                    );
                    fprintf(
                        stderr,
                        b"%s\n\n\0" as *const u8 as *const ::core::ffi::c_char,
                        JCOPYRIGHT.as_ptr(),
                    );
                    fprintf(
                        stderr,
                        b"Emulating The Independent JPEG Group's software, version %s\n\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        JVERSION.as_ptr(),
                    );
                    printed_version = TRUE as boolean;
                }
                (*(*cinfo).err).trace_level += 1;
            } else if keymatch(
                arg,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                4 as ::core::ffi::c_int,
            ) != 0
            {
                fprintf(
                    stderr,
                    b"%s version %s (build %s)\n\0" as *const u8 as *const ::core::ffi::c_char,
                    PACKAGE_NAME.as_ptr(),
                    VERSION.as_ptr(),
                    BUILD.as_ptr(),
                );
                exit(EXIT_SUCCESS);
            } else if keymatch(
                arg,
                b"flip\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if keymatch(
                    *argv.offset(argn as isize),
                    b"horizontal\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
                {
                    select_transform(JXFORM_FLIP_H);
                } else if keymatch(
                    *argv.offset(argn as isize),
                    b"vertical\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
                {
                    select_transform(JXFORM_FLIP_V);
                } else {
                    usage();
                }
            } else if keymatch(
                arg,
                b"grayscale\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
                || keymatch(
                    arg,
                    b"greyscale\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
            {
                transformoption.force_grayscale = TRUE as boolean;
            } else if keymatch(
                arg,
                b"icc\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                icc_filename = *argv.offset(argn as isize);
            } else if keymatch(
                arg,
                b"maxmemory\0" as *const u8 as *const ::core::ffi::c_char,
                3 as ::core::ffi::c_int,
            ) != 0
            {
                let mut lval: ::core::ffi::c_long = 0;
                let mut ch: ::core::ffi::c_char = 'x' as i32 as ::core::ffi::c_char;
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if sscanf(
                    *argv.offset(argn as isize),
                    b"%ld%c\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut lval,
                    &raw mut ch,
                ) < 1 as ::core::ffi::c_int
                {
                    usage();
                }
                if ch as ::core::ffi::c_int == 'm' as i32 || ch as ::core::ffi::c_int == 'M' as i32
                {
                    lval *= 1000 as ::core::ffi::c_long;
                }
                (*(*cinfo).mem).max_memory_to_use = lval * 1000 as ::core::ffi::c_long;
            } else if keymatch(
                arg,
                b"maxscans\0" as *const u8 as *const ::core::ffi::c_char,
                4 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if sscanf(
                    *argv.offset(argn as isize),
                    b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut max_scans,
                ) != 1 as ::core::ffi::c_int
                {
                    usage();
                }
            } else if keymatch(
                arg,
                b"optimize\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
                || keymatch(
                    arg,
                    b"optimise\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
            {
                (*cinfo).optimize_coding = TRUE as boolean;
            } else if keymatch(
                arg,
                b"outfile\0" as *const u8 as *const ::core::ffi::c_char,
                4 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                outfilename = *argv.offset(argn as isize);
            } else if keymatch(
                arg,
                b"perfect\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                transformoption.perfect = TRUE as boolean;
            } else if keymatch(
                arg,
                b"progressive\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                simple_progressive = TRUE as boolean;
            } else if keymatch(
                arg,
                b"report\0" as *const u8 as *const ::core::ffi::c_char,
                3 as ::core::ffi::c_int,
            ) != 0
            {
                report = TRUE as boolean;
            } else if keymatch(
                arg,
                b"restart\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                let mut lval_0: ::core::ffi::c_long = 0;
                let mut ch_0: ::core::ffi::c_char = 'x' as i32 as ::core::ffi::c_char;
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if sscanf(
                    *argv.offset(argn as isize),
                    b"%ld%c\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut lval_0,
                    &raw mut ch_0,
                ) < 1 as ::core::ffi::c_int
                {
                    usage();
                }
                if lval_0 < 0 as ::core::ffi::c_long || lval_0 > 65535 as ::core::ffi::c_long {
                    usage();
                }
                if ch_0 as ::core::ffi::c_int == 'b' as i32
                    || ch_0 as ::core::ffi::c_int == 'B' as i32
                {
                    (*cinfo).restart_interval = lval_0 as ::core::ffi::c_uint;
                    (*cinfo).restart_in_rows = 0 as ::core::ffi::c_int;
                } else {
                    (*cinfo).restart_in_rows = lval_0 as ::core::ffi::c_int;
                }
            } else if keymatch(
                arg,
                b"rotate\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if keymatch(
                    *argv.offset(argn as isize),
                    b"90\0" as *const u8 as *const ::core::ffi::c_char,
                    2 as ::core::ffi::c_int,
                ) != 0
                {
                    select_transform(JXFORM_ROT_90);
                } else if keymatch(
                    *argv.offset(argn as isize),
                    b"180\0" as *const u8 as *const ::core::ffi::c_char,
                    3 as ::core::ffi::c_int,
                ) != 0
                {
                    select_transform(JXFORM_ROT_180);
                } else if keymatch(
                    *argv.offset(argn as isize),
                    b"270\0" as *const u8 as *const ::core::ffi::c_char,
                    3 as ::core::ffi::c_int,
                ) != 0
                {
                    select_transform(JXFORM_ROT_270);
                } else {
                    usage();
                }
            } else if keymatch(
                arg,
                b"scans\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                scansarg = *argv.offset(argn as isize);
            } else if keymatch(
                arg,
                b"strict\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                strict = TRUE as boolean;
            } else if keymatch(
                arg,
                b"transpose\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                select_transform(JXFORM_TRANSPOSE);
            } else if keymatch(
                arg,
                b"transverse\0" as *const u8 as *const ::core::ffi::c_char,
                6 as ::core::ffi::c_int,
            ) != 0
            {
                select_transform(JXFORM_TRANSVERSE);
            } else if keymatch(
                arg,
                b"trim\0" as *const u8 as *const ::core::ffi::c_char,
                3 as ::core::ffi::c_int,
            ) != 0
            {
                transformoption.trim = TRUE as boolean;
            } else if keymatch(
                arg,
                b"wipe\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if transformoption.crop != 0
                    || jtransform_parse_crop_spec(
                        &raw mut transformoption,
                        *argv.offset(argn as isize),
                    ) == 0
                {
                    fprintf(
                        stderr,
                        b"%s: bogus -wipe argument '%s'\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        progname,
                        *argv.offset(argn as isize),
                    );
                    exit(EXIT_FAILURE);
                }
                select_transform(JXFORM_WIPE);
            } else {
                usage();
            }
        }
        argn += 1;
    }
    if for_real != 0 {
        if simple_progressive != 0 {
            jpeg_simple_progression(cinfo);
        }
        if !scansarg.is_null() {
            if read_scan_script(cinfo, scansarg) == 0 {
                usage();
            }
        }
    }
    return argn;
}
unsafe extern "C" fn my_emit_message(mut cinfo: j_common_ptr, mut msg_level: ::core::ffi::c_int) {
    if msg_level < 0 as ::core::ffi::c_int {
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo);
    } else if (*(*cinfo).err).trace_level >= msg_level {
        (*(*cinfo).err)
            .output_message
            .expect("non-null function pointer")(cinfo);
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut srcinfo: jpeg_decompress_struct = jpeg_decompress_struct {
        err: ::core::ptr::null_mut::<jpeg_error_mgr>(),
        mem: ::core::ptr::null_mut::<jpeg_memory_mgr>(),
        progress: ::core::ptr::null_mut::<jpeg_progress_mgr>(),
        client_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        is_decompressor: 0,
        global_state: 0,
        src: ::core::ptr::null_mut::<jpeg_source_mgr>(),
        image_width: 0,
        image_height: 0,
        num_components: 0,
        jpeg_color_space: JCS_UNKNOWN,
        out_color_space: JCS_UNKNOWN,
        scale_num: 0,
        scale_denom: 0,
        output_gamma: 0.,
        buffered_image: 0,
        raw_data_out: 0,
        dct_method: JDCT_ISLOW,
        do_fancy_upsampling: 0,
        do_block_smoothing: 0,
        quantize_colors: 0,
        dither_mode: JDITHER_NONE,
        two_pass_quantize: 0,
        desired_number_of_colors: 0,
        enable_1pass_quant: 0,
        enable_external_quant: 0,
        enable_2pass_quant: 0,
        output_width: 0,
        output_height: 0,
        out_color_components: 0,
        output_components: 0,
        rec_outbuf_height: 0,
        actual_number_of_colors: 0,
        colormap: ::core::ptr::null_mut::<JSAMPROW>(),
        output_scanline: 0,
        input_scan_number: 0,
        input_iMCU_row: 0,
        output_scan_number: 0,
        output_iMCU_row: 0,
        coef_bits: ::core::ptr::null_mut::<[::core::ffi::c_int; 64]>(),
        quant_tbl_ptrs: [::core::ptr::null_mut::<JQUANT_TBL>(); 4],
        dc_huff_tbl_ptrs: [::core::ptr::null_mut::<JHUFF_TBL>(); 4],
        ac_huff_tbl_ptrs: [::core::ptr::null_mut::<JHUFF_TBL>(); 4],
        data_precision: 0,
        comp_info: ::core::ptr::null_mut::<jpeg_component_info>(),
        is_baseline: 0,
        progressive_mode: 0,
        arith_code: 0,
        arith_dc_L: [0; 16],
        arith_dc_U: [0; 16],
        arith_ac_K: [0; 16],
        restart_interval: 0,
        saw_JFIF_marker: 0,
        JFIF_major_version: 0,
        JFIF_minor_version: 0,
        density_unit: 0,
        X_density: 0,
        Y_density: 0,
        saw_Adobe_marker: 0,
        Adobe_transform: 0,
        CCIR601_sampling: 0,
        marker_list: ::core::ptr::null_mut::<jpeg_marker_struct>(),
        max_h_samp_factor: 0,
        max_v_samp_factor: 0,
        min_DCT_h_scaled_size: 0,
        min_DCT_v_scaled_size: 0,
        total_iMCU_rows: 0,
        sample_range_limit: ::core::ptr::null_mut::<JSAMPLE>(),
        comps_in_scan: 0,
        cur_comp_info: [::core::ptr::null_mut::<jpeg_component_info>(); 4],
        MCUs_per_row: 0,
        MCU_rows_in_scan: 0,
        blocks_in_MCU: 0,
        MCU_membership: [0; 10],
        Ss: 0,
        Se: 0,
        Ah: 0,
        Al: 0,
        block_size: 0,
        natural_order: ::core::ptr::null::<::core::ffi::c_int>(),
        lim_Se: 0,
        unread_marker: 0,
        master: ::core::ptr::null_mut::<jpeg_decomp_master>(),
        main: ::core::ptr::null_mut::<jpeg_d_main_controller>(),
        coef: ::core::ptr::null_mut::<jpeg_d_coef_controller>(),
        post: ::core::ptr::null_mut::<jpeg_d_post_controller>(),
        inputctl: ::core::ptr::null_mut::<jpeg_input_controller>(),
        marker: ::core::ptr::null_mut::<jpeg_marker_reader>(),
        entropy: ::core::ptr::null_mut::<jpeg_entropy_decoder>(),
        idct: ::core::ptr::null_mut::<jpeg_inverse_dct>(),
        upsample: ::core::ptr::null_mut::<jpeg_upsampler>(),
        cconvert: ::core::ptr::null_mut::<jpeg_color_deconverter>(),
        cquantize: ::core::ptr::null_mut::<jpeg_color_quantizer>(),
    };
    let mut dropinfo: jpeg_decompress_struct = jpeg_decompress_struct {
        err: ::core::ptr::null_mut::<jpeg_error_mgr>(),
        mem: ::core::ptr::null_mut::<jpeg_memory_mgr>(),
        progress: ::core::ptr::null_mut::<jpeg_progress_mgr>(),
        client_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        is_decompressor: 0,
        global_state: 0,
        src: ::core::ptr::null_mut::<jpeg_source_mgr>(),
        image_width: 0,
        image_height: 0,
        num_components: 0,
        jpeg_color_space: JCS_UNKNOWN,
        out_color_space: JCS_UNKNOWN,
        scale_num: 0,
        scale_denom: 0,
        output_gamma: 0.,
        buffered_image: 0,
        raw_data_out: 0,
        dct_method: JDCT_ISLOW,
        do_fancy_upsampling: 0,
        do_block_smoothing: 0,
        quantize_colors: 0,
        dither_mode: JDITHER_NONE,
        two_pass_quantize: 0,
        desired_number_of_colors: 0,
        enable_1pass_quant: 0,
        enable_external_quant: 0,
        enable_2pass_quant: 0,
        output_width: 0,
        output_height: 0,
        out_color_components: 0,
        output_components: 0,
        rec_outbuf_height: 0,
        actual_number_of_colors: 0,
        colormap: ::core::ptr::null_mut::<JSAMPROW>(),
        output_scanline: 0,
        input_scan_number: 0,
        input_iMCU_row: 0,
        output_scan_number: 0,
        output_iMCU_row: 0,
        coef_bits: ::core::ptr::null_mut::<[::core::ffi::c_int; 64]>(),
        quant_tbl_ptrs: [::core::ptr::null_mut::<JQUANT_TBL>(); 4],
        dc_huff_tbl_ptrs: [::core::ptr::null_mut::<JHUFF_TBL>(); 4],
        ac_huff_tbl_ptrs: [::core::ptr::null_mut::<JHUFF_TBL>(); 4],
        data_precision: 0,
        comp_info: ::core::ptr::null_mut::<jpeg_component_info>(),
        is_baseline: 0,
        progressive_mode: 0,
        arith_code: 0,
        arith_dc_L: [0; 16],
        arith_dc_U: [0; 16],
        arith_ac_K: [0; 16],
        restart_interval: 0,
        saw_JFIF_marker: 0,
        JFIF_major_version: 0,
        JFIF_minor_version: 0,
        density_unit: 0,
        X_density: 0,
        Y_density: 0,
        saw_Adobe_marker: 0,
        Adobe_transform: 0,
        CCIR601_sampling: 0,
        marker_list: ::core::ptr::null_mut::<jpeg_marker_struct>(),
        max_h_samp_factor: 0,
        max_v_samp_factor: 0,
        min_DCT_h_scaled_size: 0,
        min_DCT_v_scaled_size: 0,
        total_iMCU_rows: 0,
        sample_range_limit: ::core::ptr::null_mut::<JSAMPLE>(),
        comps_in_scan: 0,
        cur_comp_info: [::core::ptr::null_mut::<jpeg_component_info>(); 4],
        MCUs_per_row: 0,
        MCU_rows_in_scan: 0,
        blocks_in_MCU: 0,
        MCU_membership: [0; 10],
        Ss: 0,
        Se: 0,
        Ah: 0,
        Al: 0,
        block_size: 0,
        natural_order: ::core::ptr::null::<::core::ffi::c_int>(),
        lim_Se: 0,
        unread_marker: 0,
        master: ::core::ptr::null_mut::<jpeg_decomp_master>(),
        main: ::core::ptr::null_mut::<jpeg_d_main_controller>(),
        coef: ::core::ptr::null_mut::<jpeg_d_coef_controller>(),
        post: ::core::ptr::null_mut::<jpeg_d_post_controller>(),
        inputctl: ::core::ptr::null_mut::<jpeg_input_controller>(),
        marker: ::core::ptr::null_mut::<jpeg_marker_reader>(),
        entropy: ::core::ptr::null_mut::<jpeg_entropy_decoder>(),
        idct: ::core::ptr::null_mut::<jpeg_inverse_dct>(),
        upsample: ::core::ptr::null_mut::<jpeg_upsampler>(),
        cconvert: ::core::ptr::null_mut::<jpeg_color_deconverter>(),
        cquantize: ::core::ptr::null_mut::<jpeg_color_quantizer>(),
    };
    let mut jdroperr: jpeg_error_mgr = jpeg_error_mgr {
        error_exit: None,
        emit_message: None,
        output_message: None,
        format_message: None,
        reset_error_mgr: None,
        msg_code: 0,
        msg_parm: C2RustUnnamed { i: [0; 8] },
        trace_level: 0,
        num_warnings: 0,
        jpeg_message_table: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        last_jpeg_message: 0,
        addon_message_table: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        first_addon_message: 0,
        last_addon_message: 0,
    };
    let mut drop_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut dstinfo: jpeg_compress_struct = jpeg_compress_struct {
        err: ::core::ptr::null_mut::<jpeg_error_mgr>(),
        mem: ::core::ptr::null_mut::<jpeg_memory_mgr>(),
        progress: ::core::ptr::null_mut::<jpeg_progress_mgr>(),
        client_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        is_decompressor: 0,
        global_state: 0,
        dest: ::core::ptr::null_mut::<jpeg_destination_mgr>(),
        image_width: 0,
        image_height: 0,
        input_components: 0,
        in_color_space: JCS_UNKNOWN,
        input_gamma: 0.,
        scale_num: 0,
        scale_denom: 0,
        jpeg_width: 0,
        jpeg_height: 0,
        data_precision: 0,
        num_components: 0,
        jpeg_color_space: JCS_UNKNOWN,
        comp_info: ::core::ptr::null_mut::<jpeg_component_info>(),
        quant_tbl_ptrs: [::core::ptr::null_mut::<JQUANT_TBL>(); 4],
        q_scale_factor: [0; 4],
        dc_huff_tbl_ptrs: [::core::ptr::null_mut::<JHUFF_TBL>(); 4],
        ac_huff_tbl_ptrs: [::core::ptr::null_mut::<JHUFF_TBL>(); 4],
        arith_dc_L: [0; 16],
        arith_dc_U: [0; 16],
        arith_ac_K: [0; 16],
        num_scans: 0,
        scan_info: ::core::ptr::null::<jpeg_scan_info>(),
        raw_data_in: 0,
        arith_code: 0,
        optimize_coding: 0,
        CCIR601_sampling: 0,
        do_fancy_downsampling: 0,
        smoothing_factor: 0,
        dct_method: JDCT_ISLOW,
        restart_interval: 0,
        restart_in_rows: 0,
        write_JFIF_header: 0,
        JFIF_major_version: 0,
        JFIF_minor_version: 0,
        density_unit: 0,
        X_density: 0,
        Y_density: 0,
        write_Adobe_marker: 0,
        next_scanline: 0,
        progressive_mode: 0,
        max_h_samp_factor: 0,
        max_v_samp_factor: 0,
        min_DCT_h_scaled_size: 0,
        min_DCT_v_scaled_size: 0,
        total_iMCU_rows: 0,
        comps_in_scan: 0,
        cur_comp_info: [::core::ptr::null_mut::<jpeg_component_info>(); 4],
        MCUs_per_row: 0,
        MCU_rows_in_scan: 0,
        blocks_in_MCU: 0,
        MCU_membership: [0; 10],
        Ss: 0,
        Se: 0,
        Ah: 0,
        Al: 0,
        block_size: 0,
        natural_order: ::core::ptr::null::<::core::ffi::c_int>(),
        lim_Se: 0,
        master: ::core::ptr::null_mut::<jpeg_comp_master>(),
        main: ::core::ptr::null_mut::<jpeg_c_main_controller>(),
        prep: ::core::ptr::null_mut::<jpeg_c_prep_controller>(),
        coef: ::core::ptr::null_mut::<jpeg_c_coef_controller>(),
        marker: ::core::ptr::null_mut::<jpeg_marker_writer>(),
        cconvert: ::core::ptr::null_mut::<jpeg_color_converter>(),
        downsample: ::core::ptr::null_mut::<jpeg_downsampler>(),
        fdct: ::core::ptr::null_mut::<jpeg_forward_dct>(),
        entropy: ::core::ptr::null_mut::<jpeg_entropy_encoder>(),
        script_space: ::core::ptr::null_mut::<jpeg_scan_info>(),
        script_space_size: 0,
    };
    let mut jsrcerr: jpeg_error_mgr = jpeg_error_mgr {
        error_exit: None,
        emit_message: None,
        output_message: None,
        format_message: None,
        reset_error_mgr: None,
        msg_code: 0,
        msg_parm: C2RustUnnamed { i: [0; 8] },
        trace_level: 0,
        num_warnings: 0,
        jpeg_message_table: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        last_jpeg_message: 0,
        addon_message_table: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        first_addon_message: 0,
        last_addon_message: 0,
    };
    let mut jdsterr: jpeg_error_mgr = jpeg_error_mgr {
        error_exit: None,
        emit_message: None,
        output_message: None,
        format_message: None,
        reset_error_mgr: None,
        msg_code: 0,
        msg_parm: C2RustUnnamed { i: [0; 8] },
        trace_level: 0,
        num_warnings: 0,
        jpeg_message_table: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        last_jpeg_message: 0,
        addon_message_table: ::core::ptr::null::<*const ::core::ffi::c_char>(),
        first_addon_message: 0,
        last_addon_message: 0,
    };
    let mut src_progress: cdjpeg_progress_mgr = cdjpeg_progress_mgr {
        pub_0: jpeg_progress_mgr {
            progress_monitor: None,
            pass_counter: 0,
            pass_limit: 0,
            completed_passes: 0,
            total_passes: 0,
        },
        completed_extra_passes: 0,
        total_extra_passes: 0,
        max_scans: 0,
        report: 0,
        percent_done: 0,
    };
    let mut dst_progress: cdjpeg_progress_mgr = cdjpeg_progress_mgr {
        pub_0: jpeg_progress_mgr {
            progress_monitor: None,
            pass_counter: 0,
            pass_limit: 0,
            completed_passes: 0,
            total_passes: 0,
        },
        completed_extra_passes: 0,
        total_extra_passes: 0,
        max_scans: 0,
        report: 0,
        percent_done: 0,
    };
    let mut src_coef_arrays: *mut jvirt_barray_ptr = ::core::ptr::null_mut::<jvirt_barray_ptr>();
    let mut dst_coef_arrays: *mut jvirt_barray_ptr = ::core::ptr::null_mut::<jvirt_barray_ptr>();
    let mut file_index: ::core::ffi::c_int = 0;
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut icc_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut icc_profile: *mut JOCTET = ::core::ptr::null_mut::<JOCTET>();
    let mut icc_len: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    progname = *argv.offset(0 as ::core::ffi::c_int as isize);
    if progname.is_null()
        || *progname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        progname = b"jpegtran\0" as *const u8 as *const ::core::ffi::c_char;
    }
    srcinfo.err = jpeg_std_error(&raw mut jsrcerr);
    jpeg_CreateDecompress(
        &raw mut srcinfo,
        JPEG_LIB_VERSION,
        ::core::mem::size_of::<jpeg_decompress_struct>(),
    );
    dstinfo.err = jpeg_std_error(&raw mut jdsterr);
    jpeg_CreateCompress(
        &raw mut dstinfo,
        JPEG_LIB_VERSION,
        ::core::mem::size_of::<jpeg_compress_struct>(),
    );
    file_index = parse_switches(&raw mut dstinfo, argc, argv, 0 as ::core::ffi::c_int, FALSE);
    jsrcerr.trace_level = jdsterr.trace_level;
    (*srcinfo.mem).max_memory_to_use = (*dstinfo.mem).max_memory_to_use;
    if strict != 0 {
        jsrcerr.emit_message =
            Some(my_emit_message as unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> ())
                as Option<unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> ()>;
    }
    if file_index < argc - 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"%s: only one input file\n\0" as *const u8 as *const ::core::ffi::c_char,
            progname,
        );
        usage();
    }
    if file_index < argc {
        fp = fopen(*argv.offset(file_index as isize), READ_BINARY.as_ptr()) as *mut FILE;
        if fp.is_null() {
            fprintf(
                stderr,
                b"%s: can't open %s for reading\n\0" as *const u8 as *const ::core::ffi::c_char,
                progname,
                *argv.offset(file_index as isize),
            );
            exit(EXIT_FAILURE);
        }
    } else {
        fp = read_stdin();
    }
    if !icc_filename.is_null() {
        icc_file = fopen(icc_filename, READ_BINARY.as_ptr()) as *mut FILE;
        if icc_file.is_null() {
            fprintf(
                stderr,
                b"%s: can't open %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                progname,
                icc_filename,
            );
            exit(EXIT_FAILURE);
        }
        if fseek(icc_file, 0 as ::core::ffi::c_long, SEEK_END) < 0 as ::core::ffi::c_int
            || {
                icc_len = ftell(icc_file);
                icc_len < 1 as ::core::ffi::c_long
            }
            || fseek(icc_file, 0 as ::core::ffi::c_long, SEEK_SET) < 0 as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"%s: can't determine size of %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                progname,
                icc_filename,
            );
            exit(EXIT_FAILURE);
        }
        icc_profile = malloc(icc_len as size_t) as *mut JOCTET;
        if icc_profile.is_null() {
            fprintf(
                stderr,
                b"%s: can't allocate memory for ICC profile\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progname,
            );
            fclose(icc_file);
            exit(EXIT_FAILURE);
        }
        if fread(
            icc_profile as *mut ::core::ffi::c_void,
            icc_len as size_t,
            1 as size_t,
            icc_file,
        ) < 1 as ::core::ffi::c_ulong
        {
            fprintf(
                stderr,
                b"%s: can't read ICC profile from %s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progname,
                icc_filename,
            );
            free(icc_profile as *mut ::core::ffi::c_void);
            fclose(icc_file);
            exit(EXIT_FAILURE);
        }
        fclose(icc_file);
        if copyoption as ::core::ffi::c_uint
            == JCOPYOPT_ALL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            copyoption = JCOPYOPT_ALL_EXCEPT_ICC;
        }
        if copyoption as ::core::ffi::c_uint
            == JCOPYOPT_ICC as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            copyoption = JCOPYOPT_NONE;
        }
    }
    if report != 0 {
        start_progress_monitor(&raw mut dstinfo as j_common_ptr, &raw mut dst_progress);
        dst_progress.report = report;
    }
    if report != 0 || max_scans != 0 as JDIMENSION {
        start_progress_monitor(&raw mut srcinfo as j_common_ptr, &raw mut src_progress);
        src_progress.report = report;
        src_progress.max_scans = max_scans;
    }
    if !dropfilename.is_null() {
        drop_file = fopen(dropfilename, READ_BINARY.as_ptr()) as *mut FILE;
        if drop_file.is_null() {
            fprintf(
                stderr,
                b"%s: can't open %s for reading\n\0" as *const u8 as *const ::core::ffi::c_char,
                progname,
                dropfilename,
            );
            exit(EXIT_FAILURE);
        }
        dropinfo.err = jpeg_std_error(&raw mut jdroperr);
        jpeg_CreateDecompress(
            &raw mut dropinfo,
            JPEG_LIB_VERSION,
            ::core::mem::size_of::<jpeg_decompress_struct>(),
        );
        jpeg_stdio_src(&raw mut dropinfo, drop_file);
    } else {
        drop_file = ::core::ptr::null_mut::<FILE>();
    }
    jpeg_stdio_src(&raw mut srcinfo, fp);
    jcopy_markers_setup(&raw mut srcinfo, copyoption);
    jpeg_read_header(&raw mut srcinfo, TRUE);
    if !dropfilename.is_null() {
        jpeg_read_header(&raw mut dropinfo, TRUE);
        transformoption.crop_width = dropinfo.image_width;
        transformoption.crop_width_set = JCROP_POS;
        transformoption.crop_height = dropinfo.image_height;
        transformoption.crop_height_set = JCROP_POS;
        transformoption.drop_ptr = &raw mut dropinfo as j_decompress_ptr;
    }
    if jtransform_request_workspace(&raw mut srcinfo, &raw mut transformoption) == 0 {
        fprintf(
            stderr,
            b"%s: transformation is not perfect\n\0" as *const u8 as *const ::core::ffi::c_char,
            progname,
        );
        exit(EXIT_FAILURE);
    }
    src_coef_arrays = jpeg_read_coefficients(&raw mut srcinfo);
    if !dropfilename.is_null() {
        transformoption.drop_coef_arrays = jpeg_read_coefficients(&raw mut dropinfo);
    }
    jpeg_copy_critical_parameters(&raw mut srcinfo, &raw mut dstinfo);
    dst_coef_arrays = jtransform_adjust_parameters(
        &raw mut srcinfo,
        &raw mut dstinfo,
        src_coef_arrays,
        &raw mut transformoption,
    );
    if fp != stdin {
        fclose(fp);
    }
    if !outfilename.is_null() {
        fp = fopen(outfilename, WRITE_BINARY.as_ptr()) as *mut FILE;
        if fp.is_null() {
            fprintf(
                stderr,
                b"%s: can't open %s for writing\n\0" as *const u8 as *const ::core::ffi::c_char,
                progname,
                outfilename,
            );
            exit(EXIT_FAILURE);
        }
    } else {
        fp = write_stdout();
    }
    file_index = parse_switches(&raw mut dstinfo, argc, argv, 0 as ::core::ffi::c_int, TRUE);
    jpeg_stdio_dest(&raw mut dstinfo, fp);
    jpeg_write_coefficients(&raw mut dstinfo, dst_coef_arrays);
    jcopy_markers_execute(&raw mut srcinfo, &raw mut dstinfo, copyoption);
    if !icc_profile.is_null() {
        jpeg_write_icc_profile(
            &raw mut dstinfo,
            icc_profile,
            icc_len as ::core::ffi::c_uint,
        );
    }
    jtransform_execute_transform(
        &raw mut srcinfo,
        &raw mut dstinfo,
        src_coef_arrays,
        &raw mut transformoption,
    );
    jpeg_finish_compress(&raw mut dstinfo);
    jpeg_destroy_compress(&raw mut dstinfo);
    if !dropfilename.is_null() {
        jpeg_finish_decompress(&raw mut dropinfo);
        jpeg_destroy_decompress(&raw mut dropinfo);
    }
    jpeg_finish_decompress(&raw mut srcinfo);
    jpeg_destroy_decompress(&raw mut srcinfo);
    if fp != stdout {
        fclose(fp);
    }
    if !drop_file.is_null() {
        fclose(drop_file);
    }
    if report != 0 {
        end_progress_monitor(&raw mut dstinfo as j_common_ptr);
    }
    if report != 0 || max_scans != 0 as JDIMENSION {
        end_progress_monitor(&raw mut srcinfo as j_common_ptr);
    }
    free(icc_profile as *mut ::core::ffi::c_void);
    if !dropfilename.is_null() {
        exit(
            if jsrcerr.num_warnings + jdroperr.num_warnings + jdsterr.num_warnings != 0 {
                EXIT_WARNING
            } else {
                EXIT_SUCCESS
            },
        );
    }
    exit(if jsrcerr.num_warnings + jdsterr.num_warnings != 0 {
        EXIT_WARNING
    } else {
        EXIT_SUCCESS
    });
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
