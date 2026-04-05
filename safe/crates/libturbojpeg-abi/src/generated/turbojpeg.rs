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
extern "C" {
    fn toupper(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn ungetc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn jpeg_std_error(err: *mut jpeg_error_mgr) -> *mut jpeg_error_mgr;
    fn jpeg_CreateCompress(cinfo: j_compress_ptr, version: ::core::ffi::c_int, structsize: size_t);
    fn jpeg_CreateDecompress(
        cinfo: j_decompress_ptr,
        version: ::core::ffi::c_int,
        structsize: size_t,
    );
    fn jpeg_destroy_compress(cinfo: j_compress_ptr);
    fn jpeg_destroy_decompress(cinfo: j_decompress_ptr);
    fn jpeg_set_defaults(cinfo: j_compress_ptr);
    fn jpeg_set_colorspace(cinfo: j_compress_ptr, colorspace: J_COLOR_SPACE);
    fn jpeg_set_quality(
        cinfo: j_compress_ptr,
        quality: ::core::ffi::c_int,
        force_baseline: boolean,
    );
    fn jpeg_simple_progression(cinfo: j_compress_ptr);
    fn jpeg_alloc_quant_table(cinfo: j_common_ptr) -> *mut JQUANT_TBL;
    fn jpeg_start_compress(cinfo: j_compress_ptr, write_all_tables: boolean);
    fn jpeg_write_scanlines(
        cinfo: j_compress_ptr,
        scanlines: JSAMPARRAY,
        num_lines: JDIMENSION,
    ) -> JDIMENSION;
    fn jpeg_finish_compress(cinfo: j_compress_ptr);
    fn jpeg_write_raw_data(
        cinfo: j_compress_ptr,
        data: JSAMPIMAGE,
        num_lines: JDIMENSION,
    ) -> JDIMENSION;
    fn jpeg_read_header(cinfo: j_decompress_ptr, require_image: boolean) -> ::core::ffi::c_int;
    fn jpeg_start_decompress(cinfo: j_decompress_ptr) -> boolean;
    fn jpeg_read_scanlines(
        cinfo: j_decompress_ptr,
        scanlines: JSAMPARRAY,
        max_lines: JDIMENSION,
    ) -> JDIMENSION;
    fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean;
    fn jpeg_read_raw_data(
        cinfo: j_decompress_ptr,
        data: JSAMPIMAGE,
        max_lines: JDIMENSION,
    ) -> JDIMENSION;
    fn jpeg_calc_output_dimensions(cinfo: j_decompress_ptr);
    fn jpeg_read_coefficients(cinfo: j_decompress_ptr) -> *mut jvirt_barray_ptr;
    fn jpeg_write_coefficients(cinfo: j_compress_ptr, coef_arrays: *mut jvirt_barray_ptr);
    fn jpeg_copy_critical_parameters(srcinfo: j_decompress_ptr, dstinfo: j_compress_ptr);
    fn jpeg_abort_compress(cinfo: j_compress_ptr);
    fn jpeg_abort_decompress(cinfo: j_decompress_ptr);
    fn jinit_c_master_control(cinfo: j_compress_ptr, transcode_only: boolean);
    fn jinit_color_converter(cinfo: j_compress_ptr);
    fn jinit_downsampler(cinfo: j_compress_ptr);
    fn jinit_master_decompress(cinfo: j_decompress_ptr);
    fn jcopy_sample_rows(
        input_array: JSAMPARRAY,
        source_row: ::core::ffi::c_int,
        output_array: JSAMPARRAY,
        dest_row: ::core::ffi::c_int,
        num_rows: ::core::ffi::c_int,
        num_cols: JDIMENSION,
    );
    fn _setjmp(__env: *mut __jmp_buf_tag) -> ::core::ffi::c_int;
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
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
    fn jinit_read_bmp(cinfo: j_compress_ptr, use_inversion_array: boolean) -> cjpeg_source_ptr;
    fn jinit_write_bmp(
        cinfo: j_decompress_ptr,
        is_os2: boolean,
        use_inversion_array: boolean,
    ) -> djpeg_dest_ptr;
    fn jinit_read_ppm(cinfo: j_compress_ptr) -> cjpeg_source_ptr;
    fn jinit_write_ppm(cinfo: j_decompress_ptr) -> djpeg_dest_ptr;
    fn jpeg_mem_dest_tj(
        _: j_compress_ptr,
        _: *mut *mut ::core::ffi::c_uchar,
        _: *mut ::core::ffi::c_ulong,
        _: boolean,
    );
    fn jpeg_mem_src_tj(
        _: j_decompress_ptr,
        _: *const ::core::ffi::c_uchar,
        _: ::core::ffi::c_ulong,
    );
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
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
pub type JUINTPTR = usize;
pub type __jmp_buf = [::core::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: ::core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type TJSAMP = ::core::ffi::c_uint;
pub const TJSAMP_411: TJSAMP = 5;
pub const TJSAMP_440: TJSAMP = 4;
pub const TJSAMP_GRAY: TJSAMP = 3;
pub const TJSAMP_420: TJSAMP = 2;
pub const TJSAMP_422: TJSAMP = 1;
pub const TJSAMP_444: TJSAMP = 0;
pub type TJPF = ::core::ffi::c_int;
pub const TJPF_UNKNOWN: TJPF = -1;
pub const TJPF_CMYK: TJPF = 11;
pub const TJPF_ARGB: TJPF = 10;
pub const TJPF_ABGR: TJPF = 9;
pub const TJPF_BGRA: TJPF = 8;
pub const TJPF_RGBA: TJPF = 7;
pub const TJPF_GRAY: TJPF = 6;
pub const TJPF_XRGB: TJPF = 5;
pub const TJPF_XBGR: TJPF = 4;
pub const TJPF_BGRX: TJPF = 3;
pub const TJPF_RGBX: TJPF = 2;
pub const TJPF_BGR: TJPF = 1;
pub const TJPF_RGB: TJPF = 0;
pub type TJCS = ::core::ffi::c_uint;
pub const TJCS_YCCK: TJCS = 4;
pub const TJCS_CMYK: TJCS = 3;
pub const TJCS_GRAY: TJCS = 2;
pub const TJCS_YCbCr: TJCS = 1;
pub const TJCS_RGB: TJCS = 0;
pub type TJERR = ::core::ffi::c_uint;
pub const TJERR_FATAL: TJERR = 1;
pub const TJERR_WARNING: TJERR = 0;
pub type TJXOP = ::core::ffi::c_uint;
pub const TJXOP_ROT270: TJXOP = 7;
pub const TJXOP_ROT180: TJXOP = 6;
pub const TJXOP_ROT90: TJXOP = 5;
pub const TJXOP_TRANSVERSE: TJXOP = 4;
pub const TJXOP_TRANSPOSE: TJXOP = 3;
pub const TJXOP_VFLIP: TJXOP = 2;
pub const TJXOP_HFLIP: TJXOP = 1;
pub const TJXOP_NONE: TJXOP = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tjscalingfactor {
    pub num: ::core::ffi::c_int,
    pub denom: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tjregion {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tjtransform {
    pub r: tjregion,
    pub op: ::core::ffi::c_int,
    pub options: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub customFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_short,
            tjregion,
            tjregion,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut tjtransform,
        ) -> ::core::ffi::c_int,
    >,
}
pub type tjhandle = *mut ::core::ffi::c_void;
pub type tjinstance = _tjinstance;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tjinstance {
    pub cinfo: jpeg_compress_struct,
    pub dinfo: jpeg_decompress_struct,
    pub jerr: my_error_mgr,
    pub init: ::core::ffi::c_int,
    pub headerRead: ::core::ffi::c_int,
    pub errStr: [::core::ffi::c_char; 200],
    pub isInstanceError: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_error_mgr {
    pub pub_0: jpeg_error_mgr,
    pub setjmp_buffer: jmp_buf,
    pub emit_message: Option<unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> ()>,
    pub warning: boolean,
    pub stopOnWarning: boolean,
}
pub const COMPRESS: C2RustUnnamed_1 = 1;
pub const JMSG_LASTADDONCODE: C2RustUnnamed_0 = 1028;
pub const JMSG_FIRSTADDONCODE: C2RustUnnamed_0 = 1000;
pub type my_error_ptr = *mut my_error_mgr;
pub const DECOMPRESS: C2RustUnnamed_1 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct my_progress_mgr {
    pub pub_0: jpeg_progress_mgr,
    pub this: *mut tjinstance,
}
pub type my_progress_ptr = *mut my_progress_mgr;
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
pub type JCROP_CODE = ::core::ffi::c_uint;
pub const JCROP_REFLECT: JCROP_CODE = 4;
pub const JCROP_FORCE: JCROP_CODE = 3;
pub const JCROP_NEG: JCROP_CODE = 2;
pub const JCROP_POS: JCROP_CODE = 1;
pub const JCROP_UNSET: JCROP_CODE = 0;
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
pub type JCOPY_OPTION = ::core::ffi::c_uint;
pub const JCOPYOPT_ICC: JCOPY_OPTION = 4;
pub const JCOPYOPT_ALL_EXCEPT_ICC: JCOPY_OPTION = 3;
pub const JCOPYOPT_ALL: JCOPY_OPTION = 2;
pub const JCOPYOPT_COMMENTS: JCOPY_OPTION = 1;
pub const JCOPYOPT_NONE: JCOPY_OPTION = 0;
pub type cjpeg_source_ptr = *mut cjpeg_source_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cjpeg_source_struct {
    pub start_input: Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ()>,
    pub get_pixel_rows:
        Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> JDIMENSION>,
    pub finish_input: Option<unsafe extern "C" fn(j_compress_ptr, cjpeg_source_ptr) -> ()>,
    pub input_file: *mut FILE,
    pub buffer: JSAMPARRAY,
    pub buffer_height: JDIMENSION,
}
pub type djpeg_dest_ptr = *mut djpeg_dest_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct djpeg_dest_struct {
    pub start_output: Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ()>,
    pub put_pixel_rows:
        Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr, JDIMENSION) -> ()>,
    pub finish_output: Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ()>,
    pub calc_buffer_dimensions:
        Option<unsafe extern "C" fn(j_decompress_ptr, djpeg_dest_ptr) -> ()>,
    pub output_file: *mut FILE,
    pub buffer: JSAMPARRAY,
    pub buffer_height: JDIMENSION,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const JERR_UNSUPPORTED_FORMAT: C2RustUnnamed_0 = 1027;
pub const JERR_UNKNOWN_FORMAT: C2RustUnnamed_0 = 1026;
pub const JERR_UNGETC_FAILED: C2RustUnnamed_0 = 1025;
pub const JERR_TOO_MANY_COLORS: C2RustUnnamed_0 = 1024;
pub const JERR_BAD_CMAP_FILE: C2RustUnnamed_0 = 1023;
pub const JERR_TGA_NOTCOMP: C2RustUnnamed_0 = 1022;
pub const JTRC_PPM_TEXT: C2RustUnnamed_0 = 1021;
pub const JTRC_PPM: C2RustUnnamed_0 = 1020;
pub const JTRC_PGM_TEXT: C2RustUnnamed_0 = 1019;
pub const JTRC_PGM: C2RustUnnamed_0 = 1018;
pub const JERR_PPM_OUTOFRANGE: C2RustUnnamed_0 = 1017;
pub const JERR_PPM_NOT: C2RustUnnamed_0 = 1016;
pub const JERR_PPM_NONNUMERIC: C2RustUnnamed_0 = 1015;
pub const JERR_PPM_COLORSPACE: C2RustUnnamed_0 = 1014;
pub const JTRC_BMP_OS2_MAPPED: C2RustUnnamed_0 = 1013;
pub const JTRC_BMP_OS2: C2RustUnnamed_0 = 1012;
pub const JTRC_BMP_MAPPED: C2RustUnnamed_0 = 1011;
pub const JTRC_BMP: C2RustUnnamed_0 = 1010;
pub const JERR_BMP_OUTOFRANGE: C2RustUnnamed_0 = 1009;
pub const JERR_BMP_NOT: C2RustUnnamed_0 = 1008;
pub const JERR_BMP_EMPTY: C2RustUnnamed_0 = 1007;
pub const JERR_BMP_COMPRESSED: C2RustUnnamed_0 = 1006;
pub const JERR_BMP_COLORSPACE: C2RustUnnamed_0 = 1005;
pub const JERR_BMP_BADPLANES: C2RustUnnamed_0 = 1004;
pub const JERR_BMP_BADHEADER: C2RustUnnamed_0 = 1003;
pub const JERR_BMP_BADDEPTH: C2RustUnnamed_0 = 1002;
pub const JERR_BMP_BADCMAP: C2RustUnnamed_0 = 1001;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const JPEG_LIB_VERSION: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[inline(always)]
unsafe extern "C" fn GETENV_S(
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut env: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if buffer.is_null() {
        if buffer_size == 0 as size_t {
            return 0 as ::core::ffi::c_int;
        } else {
            let ref mut fresh0 = *__errno_location();
            *fresh0 = EINVAL;
            return *fresh0;
        }
    }
    if buffer_size == 0 as size_t {
        let ref mut fresh1 = *__errno_location();
        *fresh1 = EINVAL;
        return *fresh1;
    }
    if name.is_null() {
        *buffer = 0 as ::core::ffi::c_char;
        return 0 as ::core::ffi::c_int;
    }
    env = getenv(name);
    if env.is_null() {
        *buffer = 0 as ::core::ffi::c_char;
        return 0 as ::core::ffi::c_int;
    }
    if strlen(env).wrapping_add(1 as size_t) > buffer_size {
        *buffer = 0 as ::core::ffi::c_char;
        return ERANGE;
    }
    strncpy(buffer, env, buffer_size);
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn PUTENV_S(
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if name.is_null() || value.is_null() {
        let ref mut fresh2 = *__errno_location();
        *fresh2 = EINVAL;
        return *fresh2;
    }
    setenv(name, value, 1 as ::core::ffi::c_int);
    return *__errno_location();
}
pub const MAX_COMPONENTS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DCTSIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const DCTSIZE2: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const D_MAX_BLOCKS_IN_MCU: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const JMSG_LENGTH_MAX: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPEG_HEADER_TABLES_ONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const JPEG_REACHED_SOS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CSTATE_START: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const DSTATE_START: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const DSTATE_READY: ::core::ffi::c_int = 202 as ::core::ffi::c_int;
pub const TJ_NUMSAMP: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
static mut tjMCUWidth: [::core::ffi::c_int; 6] = [
    8 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    32 as ::core::ffi::c_int,
];
static mut tjMCUHeight: [::core::ffi::c_int; 6] = [
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
];
pub const TJ_NUMPF: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
static mut tjPixelSize: [::core::ffi::c_int; 12] = [
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
];
pub const TJFLAG_BOTTOMUP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TJFLAG_FASTUPSAMPLE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const TJFLAG_NOREALLOC: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const TJFLAG_FASTDCT: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const TJFLAG_ACCURATEDCT: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const TJFLAG_STOPONWARNING: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const TJFLAG_PROGRESSIVE: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
pub const TJFLAG_LIMITSCANS: ::core::ffi::c_int = 32768 as ::core::ffi::c_int;
pub const TJXOPT_PERFECT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TJXOPT_TRIM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TJXOPT_CROP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TJXOPT_GRAY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const TJXOPT_NOOUTPUT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const TJXOPT_PROGRESSIVE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const TJXOPT_COPYNONE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const TJ_BGR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TJ_ALPHAFIRST: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const TJ_YUV: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const TJFLAG_FORCEMMX: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const TJFLAG_FORCESSE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const TJFLAG_FORCESSE2: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut errStr: [::core::ffi::c_char; 200] = unsafe {
    ::core::mem::transmute::<
        [u8; 200],
        [::core::ffi::c_char; 200],
    >(
        *b"No error\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut turbojpeg_message_table: [*const ::core::ffi::c_char; 29] = [
    ::core::ptr::null::<::core::ffi::c_char>(),
    b"Unsupported BMP colormap format\0" as *const u8 as *const ::core::ffi::c_char,
    b"Only 8-, 24-, and 32-bit BMP files are supported\0" as *const u8
        as *const ::core::ffi::c_char,
    b"Invalid BMP file: bad header length\0" as *const u8 as *const ::core::ffi::c_char,
    b"Invalid BMP file: biPlanes not equal to 1\0" as *const u8 as *const ::core::ffi::c_char,
    b"BMP output must be grayscale or RGB\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sorry, compressed BMPs not yet supported\0" as *const u8 as *const ::core::ffi::c_char,
    b"Empty BMP image\0" as *const u8 as *const ::core::ffi::c_char,
    b"Not a BMP file - does not start with BM\0" as *const u8 as *const ::core::ffi::c_char,
    b"Numeric value out of range in BMP file\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u %d-bit BMP image\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u 8-bit colormapped BMP image\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u %d-bit OS2 BMP image\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u 8-bit colormapped OS2 BMP image\0" as *const u8 as *const ::core::ffi::c_char,
    b"PPM output must be grayscale or RGB\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nonnumeric data in PPM file\0" as *const u8 as *const ::core::ffi::c_char,
    b"Not a PPM/PGM file\0" as *const u8 as *const ::core::ffi::c_char,
    b"Numeric value out of range in PPM file\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u PGM image\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u text PGM image\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u PPM image\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u text PPM image\0" as *const u8 as *const ::core::ffi::c_char,
    b"Targa support was not compiled\0" as *const u8 as *const ::core::ffi::c_char,
    b"Color map file is invalid or of unsupported format\0" as *const u8
        as *const ::core::ffi::c_char,
    b"Output file format cannot handle %d colormap entries\0" as *const u8
        as *const ::core::ffi::c_char,
    b"ungetc failed\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unrecognized input file format\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unsupported output file format\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn my_error_exit(mut cinfo: j_common_ptr) {
    let mut myerr: my_error_ptr = (*cinfo).err as my_error_ptr;
    Some(
        (*(*cinfo).err)
            .output_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    longjmp(
        &raw mut (*myerr).setjmp_buffer as *mut __jmp_buf_tag,
        1 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn my_output_message(mut cinfo: j_common_ptr) {
    Some(
        (*(*cinfo).err)
            .format_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, &raw mut errStr as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn my_emit_message(mut cinfo: j_common_ptr, mut msg_level: ::core::ffi::c_int) {
    let mut myerr: my_error_ptr = (*cinfo).err as my_error_ptr;
    (*myerr).emit_message.expect("non-null function pointer")(cinfo, msg_level);
    if msg_level < 0 as ::core::ffi::c_int {
        (*myerr).warning = TRUE as boolean;
        if (*myerr).stopOnWarning != 0 {
            longjmp(
                &raw mut (*myerr).setjmp_buffer as *mut __jmp_buf_tag,
                1 as ::core::ffi::c_int,
            );
        }
    }
}
unsafe extern "C" fn my_progress_monitor(mut dinfo: j_common_ptr) {
    let mut myerr: my_error_ptr = (*dinfo).err as my_error_ptr;
    let mut myprog: my_progress_ptr = (*dinfo).progress as my_progress_ptr;
    if (*dinfo).is_decompressor != 0 {
        let mut scan_no: ::core::ffi::c_int = (*(dinfo as j_decompress_ptr)).input_scan_number;
        if scan_no > 500 as ::core::ffi::c_int {
            snprintf(
                &raw mut (*(*myprog).this).errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"Progressive JPEG image has more than 500 scans\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"Progressive JPEG image has more than 500 scans\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*(*myprog).this).isInstanceError = TRUE as boolean;
            (*myerr).warning = FALSE as boolean;
            longjmp(
                &raw mut (*myerr).setjmp_buffer as *mut __jmp_buf_tag,
                1 as ::core::ffi::c_int,
            );
        }
    }
}
static mut pixelsize: [::core::ffi::c_int; 6] = [
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
];
static mut xformtypes: [JXFORM_CODE; 8] = [
    JXFORM_NONE,
    JXFORM_FLIP_H,
    JXFORM_FLIP_V,
    JXFORM_TRANSPOSE,
    JXFORM_TRANSVERSE,
    JXFORM_ROT_90,
    JXFORM_ROT_180,
    JXFORM_ROT_270,
];
pub const NUMSF: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
/* timg 1.5.2 starts iterating at factors + count, so keep a guard copy of the
 * smallest factor one slot past the logical end of the table. */
static mut sf: [tjscalingfactor; 17] = [
    tjscalingfactor {
        num: 2 as ::core::ffi::c_int,
        denom: 1 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 15 as ::core::ffi::c_int,
        denom: 8 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 7 as ::core::ffi::c_int,
        denom: 4 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 13 as ::core::ffi::c_int,
        denom: 8 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 3 as ::core::ffi::c_int,
        denom: 2 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 11 as ::core::ffi::c_int,
        denom: 8 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 5 as ::core::ffi::c_int,
        denom: 4 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 9 as ::core::ffi::c_int,
        denom: 8 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 1 as ::core::ffi::c_int,
        denom: 1 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 7 as ::core::ffi::c_int,
        denom: 8 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 3 as ::core::ffi::c_int,
        denom: 4 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 5 as ::core::ffi::c_int,
        denom: 8 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 1 as ::core::ffi::c_int,
        denom: 2 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 3 as ::core::ffi::c_int,
        denom: 8 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 1 as ::core::ffi::c_int,
        denom: 4 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 1 as ::core::ffi::c_int,
        denom: 8 as ::core::ffi::c_int,
    },
    tjscalingfactor {
        num: 1 as ::core::ffi::c_int,
        denom: 8 as ::core::ffi::c_int,
    },
];
static mut pf2cs: [J_COLOR_SPACE; 12] = [
    JCS_EXT_RGB,
    JCS_EXT_BGR,
    JCS_EXT_RGBX,
    JCS_EXT_BGRX,
    JCS_EXT_XBGR,
    JCS_EXT_XRGB,
    JCS_GRAYSCALE,
    JCS_EXT_RGBA,
    JCS_EXT_BGRA,
    JCS_EXT_ABGR,
    JCS_EXT_ARGB,
    JCS_CMYK,
];
static mut cs2pf: [::core::ffi::c_int; 17] = [
    TJPF_UNKNOWN as ::core::ffi::c_int,
    TJPF_GRAY as ::core::ffi::c_int,
    TJPF_RGB as ::core::ffi::c_int,
    TJPF_UNKNOWN as ::core::ffi::c_int,
    TJPF_CMYK as ::core::ffi::c_int,
    TJPF_UNKNOWN as ::core::ffi::c_int,
    TJPF_RGB as ::core::ffi::c_int,
    TJPF_RGBX as ::core::ffi::c_int,
    TJPF_BGR as ::core::ffi::c_int,
    TJPF_BGRX as ::core::ffi::c_int,
    TJPF_XBGR as ::core::ffi::c_int,
    TJPF_XRGB as ::core::ffi::c_int,
    TJPF_RGBA as ::core::ffi::c_int,
    TJPF_BGRA as ::core::ffi::c_int,
    TJPF_ABGR as ::core::ffi::c_int,
    TJPF_ARGB as ::core::ffi::c_int,
    TJPF_UNKNOWN as ::core::ffi::c_int,
];
unsafe extern "C" fn getPixelFormat(
    mut pixelSize: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if pixelSize == 1 as ::core::ffi::c_int {
        return TJPF_GRAY as ::core::ffi::c_int;
    }
    if pixelSize == 3 as ::core::ffi::c_int {
        if flags & TJ_BGR != 0 {
            return TJPF_BGR as ::core::ffi::c_int;
        } else {
            return TJPF_RGB as ::core::ffi::c_int;
        }
    }
    if pixelSize == 4 as ::core::ffi::c_int {
        if flags & TJ_ALPHAFIRST != 0 {
            if flags & TJ_BGR != 0 {
                return TJPF_XBGR as ::core::ffi::c_int;
            } else {
                return TJPF_XRGB as ::core::ffi::c_int;
            }
        } else if flags & TJ_BGR != 0 {
            return TJPF_BGRX as ::core::ffi::c_int;
        } else {
            return TJPF_RGBX as ::core::ffi::c_int;
        }
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn setCompDefaults(
    mut cinfo: *mut jpeg_compress_struct,
    mut pixelFormat: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
    mut jpegQual: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) {
    let mut env: [::core::ffi::c_char; 7] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    (*cinfo).in_color_space = pf2cs[pixelFormat as usize];
    (*cinfo).input_components = tjPixelSize[pixelFormat as usize];
    jpeg_set_defaults(cinfo as j_compress_ptr);
    if GETENV_S(
        &raw mut env as *mut ::core::ffi::c_char,
        7 as size_t,
        b"TJ_OPTIMIZE\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        && strcmp(
            &raw mut env as *mut ::core::ffi::c_char,
            b"1\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*cinfo).optimize_coding = TRUE as boolean;
    }
    if GETENV_S(
        &raw mut env as *mut ::core::ffi::c_char,
        7 as size_t,
        b"TJ_ARITHMETIC\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        && strcmp(
            &raw mut env as *mut ::core::ffi::c_char,
            b"1\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*cinfo).arith_code = TRUE as boolean;
    }
    if GETENV_S(
        &raw mut env as *mut ::core::ffi::c_char,
        7 as size_t,
        b"TJ_RESTART\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        && strlen(&raw mut env as *mut ::core::ffi::c_char) > 0 as size_t
    {
        let mut temp: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut tempc: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
        if sscanf(
            &raw mut env as *mut ::core::ffi::c_char,
            b"%d%c\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut temp,
            &raw mut tempc,
        ) >= 1 as ::core::ffi::c_int
            && temp >= 0 as ::core::ffi::c_int
            && temp <= 65535 as ::core::ffi::c_int
        {
            if toupper(tempc as ::core::ffi::c_int) == 'B' as i32 {
                (*cinfo).restart_interval = temp as ::core::ffi::c_uint;
                (*cinfo).restart_in_rows = 0 as ::core::ffi::c_int;
            } else {
                (*cinfo).restart_in_rows = temp;
            }
        }
    }
    if jpegQual >= 0 as ::core::ffi::c_int {
        jpeg_set_quality(cinfo as j_compress_ptr, jpegQual, TRUE);
        if jpegQual >= 96 as ::core::ffi::c_int || flags & TJFLAG_ACCURATEDCT != 0 {
            (*cinfo).dct_method = JDCT_ISLOW;
        } else {
            (*cinfo).dct_method = JDCT_IFAST;
        }
    }
    if subsamp == TJSAMP_GRAY as ::core::ffi::c_int {
        jpeg_set_colorspace(cinfo as j_compress_ptr, JCS_GRAYSCALE);
    } else if pixelFormat == TJPF_CMYK as ::core::ffi::c_int {
        jpeg_set_colorspace(cinfo as j_compress_ptr, JCS_YCCK);
    } else {
        jpeg_set_colorspace(cinfo as j_compress_ptr, JCS_YCbCr);
    }
    if flags & TJFLAG_PROGRESSIVE != 0 {
        jpeg_simple_progression(cinfo as j_compress_ptr);
    } else if GETENV_S(
        &raw mut env as *mut ::core::ffi::c_char,
        7 as size_t,
        b"TJ_PROGRESSIVE\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        && strcmp(
            &raw mut env as *mut ::core::ffi::c_char,
            b"1\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        jpeg_simple_progression(cinfo as j_compress_ptr);
    }
    (*(*cinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)).h_samp_factor =
        tjMCUWidth[subsamp as usize] / 8 as ::core::ffi::c_int;
    (*(*cinfo).comp_info.offset(1 as ::core::ffi::c_int as isize)).h_samp_factor =
        1 as ::core::ffi::c_int;
    (*(*cinfo).comp_info.offset(2 as ::core::ffi::c_int as isize)).h_samp_factor =
        1 as ::core::ffi::c_int;
    if (*cinfo).num_components > 3 as ::core::ffi::c_int {
        (*(*cinfo).comp_info.offset(3 as ::core::ffi::c_int as isize)).h_samp_factor =
            tjMCUWidth[subsamp as usize] / 8 as ::core::ffi::c_int;
    }
    (*(*cinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)).v_samp_factor =
        tjMCUHeight[subsamp as usize] / 8 as ::core::ffi::c_int;
    (*(*cinfo).comp_info.offset(1 as ::core::ffi::c_int as isize)).v_samp_factor =
        1 as ::core::ffi::c_int;
    (*(*cinfo).comp_info.offset(2 as ::core::ffi::c_int as isize)).v_samp_factor =
        1 as ::core::ffi::c_int;
    if (*cinfo).num_components > 3 as ::core::ffi::c_int {
        (*(*cinfo).comp_info.offset(3 as ::core::ffi::c_int as isize)).v_samp_factor =
            tjMCUHeight[subsamp as usize] / 8 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn getSubsamp(mut dinfo: j_decompress_ptr) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    if (*dinfo).num_components == 1 as ::core::ffi::c_int
        && (*dinfo).jpeg_color_space as ::core::ffi::c_uint
            == JCS_GRAYSCALE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return TJSAMP_GRAY as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < TJ_NUMSAMP {
        if (*dinfo).num_components == pixelsize[i as usize]
            || ((*dinfo).jpeg_color_space as ::core::ffi::c_uint
                == JCS_YCCK as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*dinfo).jpeg_color_space as ::core::ffi::c_uint
                    == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint)
                && pixelsize[i as usize] == 3 as ::core::ffi::c_int
                && (*dinfo).num_components == 4 as ::core::ffi::c_int
        {
            if (*(*dinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)).h_samp_factor
                == tjMCUWidth[i as usize] / 8 as ::core::ffi::c_int
                && (*(*dinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)).v_samp_factor
                    == tjMCUHeight[i as usize] / 8 as ::core::ffi::c_int
            {
                let mut match_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                k = 1 as ::core::ffi::c_int;
                while k < (*dinfo).num_components {
                    let mut href: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    let mut vref: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    if ((*dinfo).jpeg_color_space as ::core::ffi::c_uint
                        == JCS_YCCK as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*dinfo).jpeg_color_space as ::core::ffi::c_uint
                            == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint)
                        && k == 3 as ::core::ffi::c_int
                    {
                        href = tjMCUWidth[i as usize] / 8 as ::core::ffi::c_int;
                        vref = tjMCUHeight[i as usize] / 8 as ::core::ffi::c_int;
                    }
                    if (*(*dinfo).comp_info.offset(k as isize)).h_samp_factor == href
                        && (*(*dinfo).comp_info.offset(k as isize)).v_samp_factor == vref
                    {
                        match_0 += 1;
                    }
                    k += 1;
                }
                if match_0 == (*dinfo).num_components - 1 as ::core::ffi::c_int {
                    retval = i;
                    break;
                }
            }
            if (*(*dinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)).h_samp_factor
                == 2 as ::core::ffi::c_int
                && (*(*dinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)).v_samp_factor
                    == 2 as ::core::ffi::c_int
                && (i == TJSAMP_422 as ::core::ffi::c_int || i == TJSAMP_440 as ::core::ffi::c_int)
            {
                let mut match_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                k = 1 as ::core::ffi::c_int;
                while k < (*dinfo).num_components {
                    let mut href_0: ::core::ffi::c_int =
                        tjMCUHeight[i as usize] / 8 as ::core::ffi::c_int;
                    let mut vref_0: ::core::ffi::c_int =
                        tjMCUWidth[i as usize] / 8 as ::core::ffi::c_int;
                    if ((*dinfo).jpeg_color_space as ::core::ffi::c_uint
                        == JCS_YCCK as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*dinfo).jpeg_color_space as ::core::ffi::c_uint
                            == JCS_CMYK as ::core::ffi::c_int as ::core::ffi::c_uint)
                        && k == 3 as ::core::ffi::c_int
                    {
                        vref_0 = 2 as ::core::ffi::c_int;
                        href_0 = vref_0;
                    }
                    if (*(*dinfo).comp_info.offset(k as isize)).h_samp_factor == href_0
                        && (*(*dinfo).comp_info.offset(k as isize)).v_samp_factor == vref_0
                    {
                        match_1 += 1;
                    }
                    k += 1;
                }
                if match_1 == (*dinfo).num_components - 1 as ::core::ffi::c_int {
                    retval = i;
                    break;
                }
            }
            if (*(*dinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)).h_samp_factor
                * (*(*dinfo).comp_info.offset(0 as ::core::ffi::c_int as isize)).v_samp_factor
                <= D_MAX_BLOCKS_IN_MCU / pixelsize[i as usize]
                && i == TJSAMP_444 as ::core::ffi::c_int
            {
                let mut match_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                k = 1 as ::core::ffi::c_int;
                while k < (*dinfo).num_components {
                    if (*(*dinfo).comp_info.offset(k as isize)).h_samp_factor
                        == (*(*dinfo).comp_info.offset(0 as ::core::ffi::c_int as isize))
                            .h_samp_factor
                        && (*(*dinfo).comp_info.offset(k as isize)).v_samp_factor
                            == (*(*dinfo).comp_info.offset(0 as ::core::ffi::c_int as isize))
                                .v_samp_factor
                    {
                        match_2 += 1;
                    }
                    if match_2 == (*dinfo).num_components - 1 as ::core::ffi::c_int {
                        retval = i;
                        break;
                    } else {
                        k += 1;
                    }
                }
            }
        }
        i += 1;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjGetErrorStr2(mut handle: tjhandle) -> *mut ::core::ffi::c_char {
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    if !this.is_null() && (*this).isInstanceError != 0 {
        (*this).isInstanceError = FALSE as boolean;
        return &raw mut (*this).errStr as *mut ::core::ffi::c_char;
    } else {
        return &raw mut errStr as *mut ::core::ffi::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tjGetErrorStr() -> *mut ::core::ffi::c_char {
    return &raw mut errStr as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn tjGetErrorCode(mut handle: tjhandle) -> ::core::ffi::c_int {
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    if !this.is_null() && (*this).jerr.warning != 0 {
        return TJERR_WARNING as ::core::ffi::c_int;
    } else {
        return TJERR_FATAL as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tjDestroy(mut handle: tjhandle) -> ::core::ffi::c_int {
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut cinfo: j_compress_ptr = ::core::ptr::null_mut::<jpeg_compress_struct>();
    let mut dinfo: j_decompress_ptr = ::core::ptr::null_mut::<jpeg_decompress_struct>();
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    cinfo = &raw mut (*this).cinfo as j_compress_ptr;
    dinfo = &raw mut (*this).dinfo as j_decompress_ptr;
    (*this).jerr.warning = FALSE as boolean;
    (*this).isInstanceError = FALSE as boolean;
    if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if (*this).init & COMPRESS as ::core::ffi::c_int != 0 {
        jpeg_destroy_compress(cinfo);
    }
    if (*this).init & DECOMPRESS as ::core::ffi::c_int != 0 {
        jpeg_destroy_decompress(dinfo);
    }
    free(this as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tjFree(mut buf: *mut ::core::ffi::c_uchar) {
    free(buf as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tjAlloc(mut bytes: ::core::ffi::c_int) -> *mut ::core::ffi::c_uchar {
    return malloc(bytes as size_t) as *mut ::core::ffi::c_uchar;
}
unsafe extern "C" fn _tjInitCompress(mut this: *mut tjinstance) -> tjhandle {
    static mut buffer: [::core::ffi::c_uchar; 1] = [0; 1];
    let mut buf: *mut ::core::ffi::c_uchar = &raw mut buffer as *mut ::core::ffi::c_uchar;
    let mut size: ::core::ffi::c_ulong = 1 as ::core::ffi::c_ulong;
    (*this).cinfo.err = jpeg_std_error(&raw mut (*this).jerr.pub_0);
    (*this).jerr.pub_0.error_exit = Some(my_error_exit as unsafe extern "C" fn(j_common_ptr) -> ())
        as Option<unsafe extern "C" fn(j_common_ptr) -> ()>;
    (*this).jerr.pub_0.output_message =
        Some(my_output_message as unsafe extern "C" fn(j_common_ptr) -> ())
            as Option<unsafe extern "C" fn(j_common_ptr) -> ()>;
    (*this).jerr.emit_message = (*this).jerr.pub_0.emit_message;
    (*this).jerr.pub_0.emit_message =
        Some(my_emit_message as unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> ())
            as Option<unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> ()>;
    (*this).jerr.pub_0.addon_message_table =
        &raw mut turbojpeg_message_table as *mut *const ::core::ffi::c_char;
    (*this).jerr.pub_0.first_addon_message = JMSG_FIRSTADDONCODE as ::core::ffi::c_int;
    (*this).jerr.pub_0.last_addon_message = JMSG_LASTADDONCODE as ::core::ffi::c_int;
    if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
        free(this as *mut ::core::ffi::c_void);
        return NULL;
    }
    jpeg_CreateCompress(
        &raw mut (*this).cinfo,
        JPEG_LIB_VERSION,
        ::core::mem::size_of::<jpeg_compress_struct>(),
    );
    jpeg_mem_dest_tj(
        &raw mut (*this).cinfo,
        &raw mut buf,
        &raw mut size,
        0 as boolean,
    );
    (*this).init |= COMPRESS as ::core::ffi::c_int;
    return this as tjhandle;
}
#[no_mangle]
pub unsafe extern "C" fn tjInitCompress() -> tjhandle {
    let mut this: *mut tjinstance = ::core::ptr::null_mut::<tjinstance>();
    this = malloc(::core::mem::size_of::<tjinstance>() as size_t) as *mut tjinstance;
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"tjInitCompress(): Memory allocation failure\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    memset(
        this as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tjinstance>() as size_t,
    );
    snprintf(
        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
        JMSG_LENGTH_MAX as size_t,
        b"No error\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return _tjInitCompress(this);
}
#[no_mangle]
pub unsafe extern "C" fn tjBufSize(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut jpegSubsamp: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    let mut retval: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
    let mut mcuw: ::core::ffi::c_int = 0;
    let mut mcuh: ::core::ffi::c_int = 0;
    let mut chromasf: ::core::ffi::c_int = 0;
    if width < 1 as ::core::ffi::c_int
        || height < 1 as ::core::ffi::c_int
        || jpegSubsamp < 0 as ::core::ffi::c_int
        || jpegSubsamp >= TJ_NUMSAMP
    {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjBufSize(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
    } else {
        mcuw = tjMCUWidth[jpegSubsamp as usize];
        mcuh = tjMCUHeight[jpegSubsamp as usize];
        chromasf = if jpegSubsamp == TJSAMP_GRAY as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int * 64 as ::core::ffi::c_int / (mcuw * mcuh)
        };
        retval = (((width + mcuw - 1 as ::core::ffi::c_int & !(mcuw - 1 as ::core::ffi::c_int))
            * (height + mcuh - 1 as ::core::ffi::c_int & !(mcuh - 1 as ::core::ffi::c_int)))
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(
                (2 as ::core::ffi::c_ulonglong).wrapping_add(chromasf as ::core::ffi::c_ulonglong),
            )
            .wrapping_add(2048 as ::core::ffi::c_ulonglong);
        if retval > -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong as ::core::ffi::c_ulonglong {
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjBufSize(): Image is too large\0" as *const u8 as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
        }
    }
    return retval as ::core::ffi::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn TJBUFSIZE(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    let mut retval: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
    if width < 1 as ::core::ffi::c_int || height < 1 as ::core::ffi::c_int {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"TJBUFSIZE(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
    } else {
        retval = (((width + 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            & !(16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            * (height + 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                & !(16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)))
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(6 as ::core::ffi::c_ulonglong)
            .wrapping_add(2048 as ::core::ffi::c_ulonglong);
        if retval > -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong as ::core::ffi::c_ulonglong {
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"TJBUFSIZE(): Image is too large\0" as *const u8 as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
        }
    }
    return retval as ::core::ffi::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn tjBufSizeYUV2(
    mut width: ::core::ffi::c_int,
    mut align: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    let mut retval: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
    let mut nc: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if align < 1 as ::core::ffi::c_int
        || !(align & align - 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        || subsamp < 0 as ::core::ffi::c_int
        || subsamp >= TJ_NUMSAMP
    {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjBufSizeYUV2(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
    } else {
        nc = if subsamp == TJSAMP_GRAY as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        };
        i = 0 as ::core::ffi::c_int;
        while i < nc {
            let mut pw: ::core::ffi::c_int = tjPlaneWidth(i, width, subsamp);
            let mut stride: ::core::ffi::c_int =
                pw + align - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
            let mut ph: ::core::ffi::c_int = tjPlaneHeight(i, height, subsamp);
            if pw < 0 as ::core::ffi::c_int || ph < 0 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong;
            } else {
                retval = retval.wrapping_add(
                    (stride as ::core::ffi::c_ulonglong)
                        .wrapping_mul(ph as ::core::ffi::c_ulonglong),
                );
            }
            i += 1;
        }
        if retval > -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong as ::core::ffi::c_ulonglong {
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjBufSizeYUV2(): Image is too large\0" as *const u8 as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
        }
    }
    return retval as ::core::ffi::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn tjBufSizeYUV(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    return tjBufSizeYUV2(width, 4 as ::core::ffi::c_int, height, subsamp);
}
#[no_mangle]
pub unsafe extern "C" fn TJBUFSIZEYUV(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    return tjBufSizeYUV(width, height, subsamp);
}
#[no_mangle]
pub unsafe extern "C" fn tjPlaneWidth(
    mut componentID: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pw: ::core::ffi::c_ulonglong = 0;
    let mut retval: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
    let mut nc: ::core::ffi::c_int = 0;
    if width < 1 as ::core::ffi::c_int || subsamp < 0 as ::core::ffi::c_int || subsamp >= TJ_NUMSAMP
    {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjPlaneWidth(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
    } else {
        nc = if subsamp == TJSAMP_GRAY as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        };
        if componentID < 0 as ::core::ffi::c_int || componentID >= nc {
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjPlaneWidth(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
        } else {
            pw = (width as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    (tjMCUWidth[subsamp as usize] / 8 as ::core::ffi::c_int)
                        as ::core::ffi::c_ulonglong,
                )
                .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                & !(tjMCUWidth[subsamp as usize] / 8 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
            if componentID == 0 as ::core::ffi::c_int {
                retval = pw;
            } else {
                retval = pw
                    .wrapping_mul(8 as ::core::ffi::c_ulonglong)
                    .wrapping_div(tjMCUWidth[subsamp as usize] as ::core::ffi::c_ulonglong);
            }
            if retval > INT_MAX as ::core::ffi::c_ulonglong {
                snprintf(
                    &raw mut errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjPlaneWidth(): Width is too large\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
            }
        }
    }
    return retval as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tjPlaneHeight(
    mut componentID: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ph: ::core::ffi::c_ulonglong = 0;
    let mut retval: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
    let mut nc: ::core::ffi::c_int = 0;
    if height < 1 as ::core::ffi::c_int
        || subsamp < 0 as ::core::ffi::c_int
        || subsamp >= TJ_NUMSAMP
    {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjPlaneHeight(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
    } else {
        nc = if subsamp == TJSAMP_GRAY as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        };
        if componentID < 0 as ::core::ffi::c_int || componentID >= nc {
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjPlaneHeight(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
        } else {
            ph = (height as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    (tjMCUHeight[subsamp as usize] / 8 as ::core::ffi::c_int)
                        as ::core::ffi::c_ulonglong,
                )
                .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                & !(tjMCUHeight[subsamp as usize] / 8 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
            if componentID == 0 as ::core::ffi::c_int {
                retval = ph;
            } else {
                retval = ph
                    .wrapping_mul(8 as ::core::ffi::c_ulonglong)
                    .wrapping_div(tjMCUHeight[subsamp as usize] as ::core::ffi::c_ulonglong);
            }
            if retval > INT_MAX as ::core::ffi::c_ulonglong {
                snprintf(
                    &raw mut errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjPlaneHeight(): Height is too large\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
            }
        }
    }
    return retval as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tjPlaneSizeYUV(
    mut componentID: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    let mut retval: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
    let mut pw: ::core::ffi::c_int = 0;
    let mut ph: ::core::ffi::c_int = 0;
    if width < 1 as ::core::ffi::c_int
        || height < 1 as ::core::ffi::c_int
        || subsamp < 0 as ::core::ffi::c_int
        || subsamp >= TJ_NUMSAMP
    {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjPlaneSizeYUV(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
    } else {
        pw = tjPlaneWidth(componentID, width, subsamp);
        ph = tjPlaneHeight(componentID, height, subsamp);
        if pw < 0 as ::core::ffi::c_int || ph < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong;
        }
        if stride == 0 as ::core::ffi::c_int {
            stride = pw;
        } else {
            stride = abs(stride);
        }
        retval = (stride as ::core::ffi::c_ulonglong)
            .wrapping_mul((ph - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
            .wrapping_add(pw as ::core::ffi::c_ulonglong);
        if retval > -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong as ::core::ffi::c_ulonglong {
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjPlaneSizeYUV(): Image is too large\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
        }
    }
    return retval as ::core::ffi::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn tjCompress2(
    mut handle: tjhandle,
    mut srcBuf: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut pixelFormat: ::core::ffi::c_int,
    mut jpegBuf: *mut *mut ::core::ffi::c_uchar,
    mut jpegSize: *mut ::core::ffi::c_ulong,
    mut jpegSubsamp: ::core::ffi::c_int,
    mut jpegQual: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut alloc: boolean = TRUE;
    let mut row_pointer: *mut JSAMPROW = ::core::ptr::null_mut::<JSAMPROW>();
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut cinfo: j_compress_ptr = ::core::ptr::null_mut::<jpeg_compress_struct>();
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    cinfo = &raw mut (*this).cinfo as j_compress_ptr;
    (*this).jerr.warning = FALSE as boolean;
    (*this).isInstanceError = FALSE as boolean;
    (*this).jerr.stopOnWarning = (if flags & TJFLAG_STOPONWARNING != 0 {
        TRUE
    } else {
        FALSE
    }) as boolean;
    if (*this).init & COMPRESS as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjCompress2(): Instance has not been initialized for compression\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjCompress2(): Instance has not been initialized for compression\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if srcBuf.is_null()
        || width <= 0 as ::core::ffi::c_int
        || pitch < 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
        || pixelFormat < 0 as ::core::ffi::c_int
        || pixelFormat >= TJ_NUMPF
        || jpegBuf.is_null()
        || jpegSize.is_null()
        || jpegSubsamp < 0 as ::core::ffi::c_int
        || jpegSubsamp >= TJ_NUMSAMP
        || jpegQual < 0 as ::core::ffi::c_int
        || jpegQual > 100 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjCompress2(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjCompress2(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        if pitch == 0 as ::core::ffi::c_int {
            pitch = width * tjPixelSize[pixelFormat as usize];
        }
        row_pointer =
            malloc((::core::mem::size_of::<JSAMPROW>() as size_t).wrapping_mul(height as size_t))
                as *mut JSAMPROW;
        if row_pointer.is_null() {
            snprintf(
                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjCompress2(): Memory allocation failure\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*this).isInstanceError = TRUE as boolean;
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjCompress2(): Memory allocation failure\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int);
        } else if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
            retval = -(1 as ::core::ffi::c_int);
        } else {
            (*cinfo).image_width = width as JDIMENSION;
            (*cinfo).image_height = height as JDIMENSION;
            if flags & TJFLAG_FORCEMMX != 0 {
                PUTENV_S(
                    b"JSIMD_FORCEMMX\0" as *const u8 as *const ::core::ffi::c_char,
                    b"1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if flags & TJFLAG_FORCESSE != 0 {
                PUTENV_S(
                    b"JSIMD_FORCESSE\0" as *const u8 as *const ::core::ffi::c_char,
                    b"1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if flags & TJFLAG_FORCESSE2 != 0 {
                PUTENV_S(
                    b"JSIMD_FORCESSE2\0" as *const u8 as *const ::core::ffi::c_char,
                    b"1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if flags & TJFLAG_NOREALLOC != 0 {
                alloc = FALSE as boolean;
                *jpegSize = tjBufSize(width, height, jpegSubsamp);
            }
            jpeg_mem_dest_tj(cinfo, jpegBuf, jpegSize, alloc);
            setCompDefaults(
                cinfo as *mut jpeg_compress_struct,
                pixelFormat,
                jpegSubsamp,
                jpegQual,
                flags,
            );
            jpeg_start_compress(cinfo, TRUE);
            i = 0 as ::core::ffi::c_int;
            while i < height {
                if flags & TJFLAG_BOTTOMUP != 0 {
                    let ref mut fresh3 = *row_pointer.offset(i as isize);
                    *fresh3 = srcBuf.offset(
                        ((height - i - 1 as ::core::ffi::c_int) as size_t)
                            .wrapping_mul(pitch as size_t) as isize,
                    ) as *const ::core::ffi::c_uchar as JSAMPROW;
                } else {
                    let ref mut fresh4 = *row_pointer.offset(i as isize);
                    *fresh4 = srcBuf.offset((i as size_t).wrapping_mul(pitch as size_t) as isize)
                        as *const ::core::ffi::c_uchar as JSAMPROW;
                }
                i += 1;
            }
            while (*cinfo).next_scanline < (*cinfo).image_height {
                jpeg_write_scanlines(
                    cinfo,
                    row_pointer.offset((*cinfo).next_scanline as isize) as JSAMPARRAY,
                    (*cinfo).image_height.wrapping_sub((*cinfo).next_scanline),
                );
            }
            jpeg_finish_compress(cinfo);
        }
    }
    if (*cinfo).global_state > CSTATE_START {
        if alloc != 0 {
            Some(
                (*(*cinfo).dest)
                    .term_destination
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
        jpeg_abort_compress(cinfo);
    }
    free(row_pointer as *mut ::core::ffi::c_void);
    if (*this).jerr.warning != 0 {
        retval = -(1 as ::core::ffi::c_int);
    }
    (*this).jerr.stopOnWarning = FALSE as boolean;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjCompress(
    mut handle: tjhandle,
    mut srcBuf: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut pixelSize: ::core::ffi::c_int,
    mut jpegBuf: *mut ::core::ffi::c_uchar,
    mut jpegSize: *mut ::core::ffi::c_ulong,
    mut jpegSubsamp: ::core::ffi::c_int,
    mut jpegQual: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_ulong = 0;
    if flags & TJ_YUV != 0 {
        size = tjBufSizeYUV(width, height, jpegSubsamp);
        retval = tjEncodeYUV2(
            handle,
            srcBuf,
            width,
            pitch,
            height,
            getPixelFormat(pixelSize, flags),
            jpegBuf,
            jpegSubsamp,
            flags,
        );
    } else {
        retval = tjCompress2(
            handle,
            srcBuf,
            width,
            pitch,
            height,
            getPixelFormat(pixelSize, flags),
            &raw mut jpegBuf,
            &raw mut size,
            jpegSubsamp,
            jpegQual,
            flags | TJFLAG_NOREALLOC,
        );
    }
    *jpegSize = size;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjEncodeYUVPlanes(
    mut handle: tjhandle,
    mut srcBuf: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut pixelFormat: ::core::ffi::c_int,
    mut dstPlanes: *mut *mut ::core::ffi::c_uchar,
    mut strides: *mut ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut row_pointer: *mut JSAMPROW = ::core::ptr::null_mut::<JSAMPROW>();
    let mut _tmpbuf: [*mut JSAMPLE; 10] = [::core::ptr::null_mut::<JSAMPLE>(); 10];
    let mut _tmpbuf2: [*mut JSAMPLE; 10] = [::core::ptr::null_mut::<JSAMPLE>(); 10];
    let mut tmpbuf: [*mut JSAMPROW; 10] = [::core::ptr::null_mut::<JSAMPROW>(); 10];
    let mut tmpbuf2: [*mut JSAMPROW; 10] = [::core::ptr::null_mut::<JSAMPROW>(); 10];
    let mut outbuf: [*mut JSAMPROW; 10] = [::core::ptr::null_mut::<JSAMPROW>(); 10];
    let mut i: ::core::ffi::c_int = 0;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut row: ::core::ffi::c_int = 0;
    let mut pw0: ::core::ffi::c_int = 0;
    let mut ph0: ::core::ffi::c_int = 0;
    let mut pw: [::core::ffi::c_int; 10] = [0; 10];
    let mut ph: [::core::ffi::c_int; 10] = [0; 10];
    let mut ptr: *mut JSAMPLE = ::core::ptr::null_mut::<JSAMPLE>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut cinfo: j_compress_ptr = ::core::ptr::null_mut::<jpeg_compress_struct>();
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    cinfo = &raw mut (*this).cinfo as j_compress_ptr;
    (*this).jerr.warning = FALSE as boolean;
    (*this).isInstanceError = FALSE as boolean;
    (*this).jerr.stopOnWarning = (if flags & TJFLAG_STOPONWARNING != 0 {
        TRUE
    } else {
        FALSE
    }) as boolean;
    i = 0 as ::core::ffi::c_int;
    while i < MAX_COMPONENTS {
        tmpbuf[i as usize] = ::core::ptr::null_mut::<JSAMPROW>();
        _tmpbuf[i as usize] = ::core::ptr::null_mut::<JSAMPLE>();
        tmpbuf2[i as usize] = ::core::ptr::null_mut::<JSAMPROW>();
        _tmpbuf2[i as usize] = ::core::ptr::null_mut::<JSAMPLE>();
        outbuf[i as usize] = ::core::ptr::null_mut::<JSAMPROW>();
        i += 1;
    }
    if (*this).init & COMPRESS as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjEncodeYUVPlanes(): Instance has not been initialized for compression\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjEncodeYUVPlanes(): Instance has not been initialized for compression\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if srcBuf.is_null()
        || width <= 0 as ::core::ffi::c_int
        || pitch < 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
        || pixelFormat < 0 as ::core::ffi::c_int
        || pixelFormat >= TJ_NUMPF
        || dstPlanes.is_null()
        || (*dstPlanes.offset(0 as ::core::ffi::c_int as isize)).is_null()
        || subsamp < 0 as ::core::ffi::c_int
        || subsamp >= TJ_NUMSAMP
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjEncodeYUVPlanes(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjEncodeYUVPlanes(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if subsamp != TJSAMP_GRAY as ::core::ffi::c_int
        && ((*dstPlanes.offset(1 as ::core::ffi::c_int as isize)).is_null()
            || (*dstPlanes.offset(2 as ::core::ffi::c_int as isize)).is_null())
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjEncodeYUVPlanes(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjEncodeYUVPlanes(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if pixelFormat == TJPF_CMYK as ::core::ffi::c_int {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjEncodeYUVPlanes(): Cannot generate YUV images from packed-pixel CMYK images\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjEncodeYUVPlanes(): Cannot generate YUV images from packed-pixel CMYK images\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        if pitch == 0 as ::core::ffi::c_int {
            pitch = width * tjPixelSize[pixelFormat as usize];
        }
        if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
            retval = -(1 as ::core::ffi::c_int);
        } else {
            (*cinfo).image_width = width as JDIMENSION;
            (*cinfo).image_height = height as JDIMENSION;
            if flags & TJFLAG_FORCEMMX != 0 {
                PUTENV_S(
                    b"JSIMD_FORCEMMX\0" as *const u8 as *const ::core::ffi::c_char,
                    b"1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if flags & TJFLAG_FORCESSE != 0 {
                PUTENV_S(
                    b"JSIMD_FORCESSE\0" as *const u8 as *const ::core::ffi::c_char,
                    b"1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if flags & TJFLAG_FORCESSE2 != 0 {
                PUTENV_S(
                    b"JSIMD_FORCESSE2\0" as *const u8 as *const ::core::ffi::c_char,
                    b"1\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            setCompDefaults(
                cinfo as *mut jpeg_compress_struct,
                pixelFormat,
                subsamp,
                -(1 as ::core::ffi::c_int),
                flags,
            );
            if (*cinfo).global_state != CSTATE_START {
                snprintf(
                    &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjEncodeYUVPlanes(): libjpeg API is in the wrong state\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                (*this).isInstanceError = TRUE as boolean;
                snprintf(
                    &raw mut errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjEncodeYUVPlanes(): libjpeg API is in the wrong state\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                retval = -(1 as ::core::ffi::c_int);
            } else {
                Some(
                    (*(*cinfo).err)
                        .reset_error_mgr
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
                jinit_c_master_control(cinfo, FALSE);
                jinit_color_converter(cinfo);
                jinit_downsampler(cinfo);
                Some(
                    (*(*cinfo).cconvert)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                pw0 = width + (*cinfo).max_h_samp_factor - 1 as ::core::ffi::c_int
                    & !((*cinfo).max_h_samp_factor - 1 as ::core::ffi::c_int);
                ph0 = height + (*cinfo).max_v_samp_factor - 1 as ::core::ffi::c_int
                    & !((*cinfo).max_v_samp_factor - 1 as ::core::ffi::c_int);
                row_pointer = malloc(
                    (::core::mem::size_of::<JSAMPROW>() as size_t).wrapping_mul(ph0 as size_t),
                ) as *mut JSAMPROW;
                if row_pointer.is_null() {
                    snprintf(
                        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjEncodeYUVPlanes(): Memory allocation failure\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    (*this).isInstanceError = TRUE as boolean;
                    snprintf(
                        &raw mut errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjEncodeYUVPlanes(): Memory allocation failure\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    retval = -(1 as ::core::ffi::c_int);
                } else {
                    i = 0 as ::core::ffi::c_int;
                    while i < height {
                        if flags & TJFLAG_BOTTOMUP != 0 {
                            let ref mut fresh7 = *row_pointer.offset(i as isize);
                            *fresh7 = srcBuf.offset(
                                ((height - i - 1 as ::core::ffi::c_int) as size_t)
                                    .wrapping_mul(pitch as size_t)
                                    as isize,
                            ) as *const ::core::ffi::c_uchar
                                as JSAMPROW;
                        } else {
                            let ref mut fresh8 = *row_pointer.offset(i as isize);
                            *fresh8 = srcBuf
                                .offset((i as size_t).wrapping_mul(pitch as size_t) as isize)
                                as *const ::core::ffi::c_uchar
                                as JSAMPROW;
                        }
                        i += 1;
                    }
                    if height < ph0 {
                        i = height;
                        while i < ph0 {
                            let ref mut fresh9 = *row_pointer.offset(i as isize);
                            *fresh9 =
                                *row_pointer.offset((height - 1 as ::core::ffi::c_int) as isize);
                            i += 1;
                        }
                    }
                    i = 0 as ::core::ffi::c_int;
                    loop {
                        if !(i < (*cinfo).num_components) {
                            current_block = 10109057886293123569;
                            break;
                        }
                        compptr = (*cinfo).comp_info.offset(i as isize) as *mut jpeg_component_info;
                        _tmpbuf[i as usize] = malloc(
                            ((*compptr)
                                .width_in_blocks
                                .wrapping_mul((*cinfo).max_h_samp_factor as JDIMENSION)
                                .wrapping_mul(8 as JDIMENSION)
                                .wrapping_div((*compptr).h_samp_factor as JDIMENSION)
                                .wrapping_add(32 as JDIMENSION)
                                .wrapping_sub(1 as JDIMENSION)
                                & !(32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as JDIMENSION)
                                .wrapping_mul((*cinfo).max_v_samp_factor as JDIMENSION)
                                .wrapping_add(32 as JDIMENSION)
                                as size_t,
                        ) as *mut JSAMPLE;
                        if _tmpbuf[i as usize].is_null() {
                            snprintf(
                                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                JMSG_LENGTH_MAX as size_t,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                b"tjEncodeYUVPlanes(): Memory allocation failure\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            (*this).isInstanceError = TRUE as boolean;
                            snprintf(
                                &raw mut errStr as *mut ::core::ffi::c_char,
                                JMSG_LENGTH_MAX as size_t,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                b"tjEncodeYUVPlanes(): Memory allocation failure\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            retval = -(1 as ::core::ffi::c_int);
                            current_block = 17012680438162395660;
                            break;
                        } else {
                            tmpbuf[i as usize] = malloc(
                                (::core::mem::size_of::<JSAMPROW>() as size_t)
                                    .wrapping_mul((*cinfo).max_v_samp_factor as size_t),
                            ) as *mut JSAMPROW;
                            if tmpbuf[i as usize].is_null() {
                                snprintf(
                                    &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                    JMSG_LENGTH_MAX as size_t,
                                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"tjEncodeYUVPlanes(): Memory allocation failure\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                (*this).isInstanceError = TRUE as boolean;
                                snprintf(
                                    &raw mut errStr as *mut ::core::ffi::c_char,
                                    JMSG_LENGTH_MAX as size_t,
                                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"tjEncodeYUVPlanes(): Memory allocation failure\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                retval = -(1 as ::core::ffi::c_int);
                                current_block = 17012680438162395660;
                                break;
                            } else {
                                row = 0 as ::core::ffi::c_int;
                                while row < (*cinfo).max_v_samp_factor {
                                    let mut _tmpbuf_aligned: *mut ::core::ffi::c_uchar =
                                        ((_tmpbuf[i as usize] as JUINTPTR)
                                            .wrapping_add(32 as ::core::ffi::c_int as JUINTPTR)
                                            .wrapping_sub(1 as ::core::ffi::c_int as JUINTPTR)
                                            & !(32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                                as JUINTPTR)
                                            as *mut ::core::ffi::c_uchar;
                                    let ref mut fresh10 = *tmpbuf[i as usize].offset(row as isize);
                                    *fresh10 = _tmpbuf_aligned.offset(
                                        ((*compptr)
                                            .width_in_blocks
                                            .wrapping_mul((*cinfo).max_h_samp_factor as JDIMENSION)
                                            .wrapping_mul(8 as JDIMENSION)
                                            .wrapping_div((*compptr).h_samp_factor as JDIMENSION)
                                            .wrapping_add(32 as JDIMENSION)
                                            .wrapping_sub(1 as JDIMENSION)
                                            & !(32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                                as JDIMENSION)
                                            .wrapping_mul(row as JDIMENSION)
                                            as isize,
                                    )
                                        as *mut ::core::ffi::c_uchar
                                        as JSAMPROW;
                                    row += 1;
                                }
                                _tmpbuf2[i as usize] = malloc(
                                    ((*compptr)
                                        .width_in_blocks
                                        .wrapping_mul(8 as JDIMENSION)
                                        .wrapping_add(32 as JDIMENSION)
                                        .wrapping_sub(1 as JDIMENSION)
                                        & !(32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                            as JDIMENSION)
                                        .wrapping_mul((*compptr).v_samp_factor as JDIMENSION)
                                        .wrapping_add(32 as JDIMENSION)
                                        as size_t,
                                )
                                    as *mut JSAMPLE;
                                if _tmpbuf2[i as usize].is_null() {
                                    snprintf(
                                        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                        JMSG_LENGTH_MAX as size_t,
                                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                        b"tjEncodeYUVPlanes(): Memory allocation failure\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    (*this).isInstanceError = TRUE as boolean;
                                    snprintf(
                                        &raw mut errStr as *mut ::core::ffi::c_char,
                                        JMSG_LENGTH_MAX as size_t,
                                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                        b"tjEncodeYUVPlanes(): Memory allocation failure\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    retval = -(1 as ::core::ffi::c_int);
                                    current_block = 17012680438162395660;
                                    break;
                                } else {
                                    tmpbuf2[i as usize] = malloc(
                                        (::core::mem::size_of::<JSAMPROW>() as size_t)
                                            .wrapping_mul((*compptr).v_samp_factor as size_t),
                                    )
                                        as *mut JSAMPROW;
                                    if tmpbuf2[i as usize].is_null() {
                                        snprintf(
                                            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                            JMSG_LENGTH_MAX as size_t,
                                            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                            b"tjEncodeYUVPlanes(): Memory allocation failure\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                        (*this).isInstanceError = TRUE as boolean;
                                        snprintf(
                                            &raw mut errStr as *mut ::core::ffi::c_char,
                                            JMSG_LENGTH_MAX as size_t,
                                            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                            b"tjEncodeYUVPlanes(): Memory allocation failure\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                        retval = -(1 as ::core::ffi::c_int);
                                        current_block = 17012680438162395660;
                                        break;
                                    } else {
                                        row = 0 as ::core::ffi::c_int;
                                        while row < (*compptr).v_samp_factor {
                                            let mut _tmpbuf2_aligned: *mut ::core::ffi::c_uchar =
                                                ((_tmpbuf2[i as usize] as JUINTPTR)
                                                    .wrapping_add(
                                                        32 as ::core::ffi::c_int as JUINTPTR,
                                                    )
                                                    .wrapping_sub(
                                                        1 as ::core::ffi::c_int as JUINTPTR,
                                                    )
                                                    & !(32 as ::core::ffi::c_int
                                                        - 1 as ::core::ffi::c_int)
                                                        as JUINTPTR)
                                                    as *mut ::core::ffi::c_uchar;
                                            let ref mut fresh11 =
                                                *tmpbuf2[i as usize].offset(row as isize);
                                            *fresh11 = _tmpbuf2_aligned.offset(
                                                ((*compptr)
                                                    .width_in_blocks
                                                    .wrapping_mul(8 as JDIMENSION)
                                                    .wrapping_add(32 as JDIMENSION)
                                                    .wrapping_sub(1 as JDIMENSION)
                                                    & !(32 as ::core::ffi::c_int
                                                        - 1 as ::core::ffi::c_int)
                                                        as JDIMENSION)
                                                    .wrapping_mul(row as JDIMENSION)
                                                    as isize,
                                            )
                                                as *mut ::core::ffi::c_uchar
                                                as JSAMPROW;
                                            row += 1;
                                        }
                                        pw[i as usize] = pw0 * (*compptr).h_samp_factor
                                            / (*cinfo).max_h_samp_factor;
                                        ph[i as usize] = ph0 * (*compptr).v_samp_factor
                                            / (*cinfo).max_v_samp_factor;
                                        outbuf[i as usize] = malloc(
                                            (::core::mem::size_of::<JSAMPROW>() as size_t)
                                                .wrapping_mul(ph[i as usize] as size_t),
                                        )
                                            as *mut JSAMPROW;
                                        if outbuf[i as usize].is_null() {
                                            snprintf(
                                                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                                JMSG_LENGTH_MAX as size_t,
                                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                b"tjEncodeYUVPlanes(): Memory allocation failure\0"
                                                    as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                            (*this).isInstanceError = TRUE as boolean;
                                            snprintf(
                                                &raw mut errStr as *mut ::core::ffi::c_char,
                                                JMSG_LENGTH_MAX as size_t,
                                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                b"tjEncodeYUVPlanes(): Memory allocation failure\0"
                                                    as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                            retval = -(1 as ::core::ffi::c_int);
                                            current_block = 17012680438162395660;
                                            break;
                                        } else {
                                            ptr = *dstPlanes.offset(i as isize) as *mut JSAMPLE;
                                            row = 0 as ::core::ffi::c_int;
                                            while row < ph[i as usize] {
                                                let ref mut fresh12 =
                                                    *outbuf[i as usize].offset(row as isize);
                                                *fresh12 = ptr as JSAMPROW;
                                                ptr = ptr.offset(
                                                    (if !strides.is_null()
                                                        && *strides.offset(i as isize)
                                                            != 0 as ::core::ffi::c_int
                                                    {
                                                        *strides.offset(i as isize)
                                                    } else {
                                                        pw[i as usize]
                                                    })
                                                        as isize,
                                                );
                                                row += 1;
                                            }
                                            i += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        17012680438162395660 => {}
                        _ => {
                            if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag)
                                != 0
                            {
                                retval = -(1 as ::core::ffi::c_int);
                            } else {
                                row = 0 as ::core::ffi::c_int;
                                while row < ph0 {
                                    Some(
                                        (*(*cinfo).cconvert)
                                            .color_convert
                                            .expect("non-null function pointer"),
                                    )
                                    .expect("non-null function pointer")(
                                        cinfo,
                                        row_pointer.offset(row as isize) as JSAMPARRAY,
                                        &raw mut tmpbuf as JSAMPIMAGE,
                                        0 as JDIMENSION,
                                        (*cinfo).max_v_samp_factor,
                                    );
                                    (*(*cinfo).downsample)
                                        .downsample
                                        .expect("non-null function pointer")(
                                        cinfo,
                                        &raw mut tmpbuf as JSAMPIMAGE,
                                        0 as JDIMENSION,
                                        &raw mut tmpbuf2 as JSAMPIMAGE,
                                        0 as JDIMENSION,
                                    );
                                    i = 0 as ::core::ffi::c_int;
                                    compptr = (*cinfo).comp_info;
                                    while i < (*cinfo).num_components {
                                        jcopy_sample_rows(
                                            tmpbuf2[i as usize],
                                            0 as ::core::ffi::c_int,
                                            outbuf[i as usize],
                                            row * (*compptr).v_samp_factor
                                                / (*cinfo).max_v_samp_factor,
                                            (*compptr).v_samp_factor,
                                            pw[i as usize] as JDIMENSION,
                                        );
                                        i += 1;
                                        compptr = compptr.offset(1);
                                    }
                                    row += (*cinfo).max_v_samp_factor;
                                }
                                (*cinfo).next_scanline =
                                    (*cinfo).next_scanline.wrapping_add(height as JDIMENSION);
                                jpeg_abort_compress(cinfo);
                            }
                        }
                    }
                }
            }
        }
    }
    if (*cinfo).global_state > CSTATE_START {
        jpeg_abort_compress(cinfo);
    }
    free(row_pointer as *mut ::core::ffi::c_void);
    i = 0 as ::core::ffi::c_int;
    while i < MAX_COMPONENTS {
        free(tmpbuf[i as usize] as *mut ::core::ffi::c_void);
        free(_tmpbuf[i as usize] as *mut ::core::ffi::c_void);
        free(tmpbuf2[i as usize] as *mut ::core::ffi::c_void);
        free(_tmpbuf2[i as usize] as *mut ::core::ffi::c_void);
        free(outbuf[i as usize] as *mut ::core::ffi::c_void);
        i += 1;
    }
    if (*this).jerr.warning != 0 {
        retval = -(1 as ::core::ffi::c_int);
    }
    (*this).jerr.stopOnWarning = FALSE as boolean;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjEncodeYUV3(
    mut handle: tjhandle,
    mut srcBuf: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut pixelFormat: ::core::ffi::c_int,
    mut dstBuf: *mut ::core::ffi::c_uchar,
    mut align: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dstPlanes: [*mut ::core::ffi::c_uchar; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3];
    let mut pw0: ::core::ffi::c_int = 0;
    let mut ph0: ::core::ffi::c_int = 0;
    let mut strides: [::core::ffi::c_int; 3] = [0; 3];
    let mut retval: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjEncodeYUV3(): Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        (*this).isInstanceError = FALSE as boolean;
        if width <= 0 as ::core::ffi::c_int
            || height <= 0 as ::core::ffi::c_int
            || dstBuf.is_null()
            || align < 1 as ::core::ffi::c_int
            || !(align & align - 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            || subsamp < 0 as ::core::ffi::c_int
            || subsamp >= TJ_NUMSAMP
        {
            snprintf(
                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjEncodeYUV3(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
            );
            (*this).isInstanceError = TRUE as boolean;
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjEncodeYUV3(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int);
        } else {
            pw0 = tjPlaneWidth(0 as ::core::ffi::c_int, width, subsamp);
            ph0 = tjPlaneHeight(0 as ::core::ffi::c_int, height, subsamp);
            dstPlanes[0 as ::core::ffi::c_int as usize] = dstBuf;
            strides[0 as ::core::ffi::c_int as usize] =
                pw0 + align - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
            if subsamp == TJSAMP_GRAY as ::core::ffi::c_int {
                strides[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
                strides[1 as ::core::ffi::c_int as usize] =
                    strides[2 as ::core::ffi::c_int as usize];
                dstPlanes[2 as ::core::ffi::c_int as usize] =
                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                dstPlanes[1 as ::core::ffi::c_int as usize] =
                    dstPlanes[2 as ::core::ffi::c_int as usize];
            } else {
                let mut pw1: ::core::ffi::c_int =
                    tjPlaneWidth(1 as ::core::ffi::c_int, width, subsamp);
                let mut ph1: ::core::ffi::c_int =
                    tjPlaneHeight(1 as ::core::ffi::c_int, height, subsamp);
                strides[2 as ::core::ffi::c_int as usize] =
                    pw1 + align - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
                strides[1 as ::core::ffi::c_int as usize] =
                    strides[2 as ::core::ffi::c_int as usize];
                dstPlanes[1 as ::core::ffi::c_int as usize] = dstPlanes
                    [0 as ::core::ffi::c_int as usize]
                    .offset((strides[0 as ::core::ffi::c_int as usize] * ph0) as isize);
                dstPlanes[2 as ::core::ffi::c_int as usize] = dstPlanes
                    [1 as ::core::ffi::c_int as usize]
                    .offset((strides[1 as ::core::ffi::c_int as usize] * ph1) as isize);
            }
            return tjEncodeYUVPlanes(
                handle,
                srcBuf,
                width,
                pitch,
                height,
                pixelFormat,
                &raw mut dstPlanes as *mut *mut ::core::ffi::c_uchar,
                &raw mut strides as *mut ::core::ffi::c_int,
                subsamp,
                flags,
            );
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjEncodeYUV2(
    mut handle: tjhandle,
    mut srcBuf: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut pixelFormat: ::core::ffi::c_int,
    mut dstBuf: *mut ::core::ffi::c_uchar,
    mut subsamp: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return tjEncodeYUV3(
        handle,
        srcBuf,
        width,
        pitch,
        height,
        pixelFormat,
        dstBuf,
        4 as ::core::ffi::c_int,
        subsamp,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tjEncodeYUV(
    mut handle: tjhandle,
    mut srcBuf: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut pixelSize: ::core::ffi::c_int,
    mut dstBuf: *mut ::core::ffi::c_uchar,
    mut subsamp: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return tjEncodeYUV2(
        handle,
        srcBuf,
        width,
        pitch,
        height,
        getPixelFormat(pixelSize, flags),
        dstBuf,
        subsamp,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tjCompressFromYUVPlanes(
    mut handle: tjhandle,
    mut srcPlanes: *mut *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut strides: *const ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
    mut jpegBuf: *mut *mut ::core::ffi::c_uchar,
    mut jpegSize: *mut ::core::ffi::c_ulong,
    mut jpegQual: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut alloc: boolean = TRUE;
    let mut pw: [::core::ffi::c_int; 10] = [0; 10];
    let mut ph: [::core::ffi::c_int; 10] = [0; 10];
    let mut iw: [::core::ffi::c_int; 10] = [0; 10];
    let mut tmpbufsize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut usetmpbuf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut th: [::core::ffi::c_int; 10] = [0; 10];
    let mut _tmpbuf: *mut JSAMPLE = ::core::ptr::null_mut::<JSAMPLE>();
    let mut ptr: *mut JSAMPLE = ::core::ptr::null_mut::<JSAMPLE>();
    let mut inbuf: [*mut JSAMPROW; 10] = [::core::ptr::null_mut::<JSAMPROW>(); 10];
    let mut tmpbuf: [*mut JSAMPROW; 10] = [::core::ptr::null_mut::<JSAMPROW>(); 10];
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut cinfo: j_compress_ptr = ::core::ptr::null_mut::<jpeg_compress_struct>();
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    cinfo = &raw mut (*this).cinfo as j_compress_ptr;
    (*this).jerr.warning = FALSE as boolean;
    (*this).isInstanceError = FALSE as boolean;
    (*this).jerr.stopOnWarning = (if flags & TJFLAG_STOPONWARNING != 0 {
        TRUE
    } else {
        FALSE
    }) as boolean;
    i = 0 as ::core::ffi::c_int;
    while i < MAX_COMPONENTS {
        tmpbuf[i as usize] = ::core::ptr::null_mut::<JSAMPROW>();
        inbuf[i as usize] = ::core::ptr::null_mut::<JSAMPROW>();
        i += 1;
    }
    if (*this).init & COMPRESS as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjCompressFromYUVPlanes(): Instance has not been initialized for compression\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjCompressFromYUVPlanes(): Instance has not been initialized for compression\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if srcPlanes.is_null()
        || (*srcPlanes.offset(0 as ::core::ffi::c_int as isize)).is_null()
        || width <= 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
        || subsamp < 0 as ::core::ffi::c_int
        || subsamp >= TJ_NUMSAMP
        || jpegBuf.is_null()
        || jpegSize.is_null()
        || jpegQual < 0 as ::core::ffi::c_int
        || jpegQual > 100 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjCompressFromYUVPlanes(): Invalid argument\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjCompressFromYUVPlanes(): Invalid argument\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if subsamp != TJSAMP_GRAY as ::core::ffi::c_int
        && ((*srcPlanes.offset(1 as ::core::ffi::c_int as isize)).is_null()
            || (*srcPlanes.offset(2 as ::core::ffi::c_int as isize)).is_null())
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjCompressFromYUVPlanes(): Invalid argument\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjCompressFromYUVPlanes(): Invalid argument\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
        retval = -(1 as ::core::ffi::c_int);
    } else {
        (*cinfo).image_width = width as JDIMENSION;
        (*cinfo).image_height = height as JDIMENSION;
        if flags & TJFLAG_FORCEMMX != 0 {
            PUTENV_S(
                b"JSIMD_FORCEMMX\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if flags & TJFLAG_FORCESSE != 0 {
            PUTENV_S(
                b"JSIMD_FORCESSE\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if flags & TJFLAG_FORCESSE2 != 0 {
            PUTENV_S(
                b"JSIMD_FORCESSE2\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if flags & TJFLAG_NOREALLOC != 0 {
            alloc = FALSE as boolean;
            *jpegSize = tjBufSize(width, height, subsamp);
        }
        jpeg_mem_dest_tj(cinfo, jpegBuf, jpegSize, alloc);
        setCompDefaults(
            cinfo as *mut jpeg_compress_struct,
            TJPF_RGB as ::core::ffi::c_int,
            subsamp,
            jpegQual,
            flags,
        );
        (*cinfo).raw_data_in = TRUE as boolean;
        jpeg_start_compress(cinfo, TRUE);
        i = 0 as ::core::ffi::c_int;
        loop {
            if !(i < (*cinfo).num_components) {
                current_block = 9241535491006583629;
                break;
            }
            let mut compptr: *mut jpeg_component_info =
                (*cinfo).comp_info.offset(i as isize) as *mut jpeg_component_info;
            let mut ih: ::core::ffi::c_int = 0;
            iw[i as usize] = (*compptr)
                .width_in_blocks
                .wrapping_mul(DCTSIZE as JDIMENSION)
                as ::core::ffi::c_int;
            ih = (*compptr)
                .height_in_blocks
                .wrapping_mul(DCTSIZE as JDIMENSION) as ::core::ffi::c_int;
            pw[i as usize] = ((*cinfo)
                .image_width
                .wrapping_add((*cinfo).max_h_samp_factor as JDIMENSION)
                .wrapping_sub(1 as JDIMENSION)
                & !((*cinfo).max_h_samp_factor - 1 as ::core::ffi::c_int) as JDIMENSION)
                .wrapping_mul((*compptr).h_samp_factor as JDIMENSION)
                .wrapping_div((*cinfo).max_h_samp_factor as JDIMENSION)
                as ::core::ffi::c_int;
            ph[i as usize] = ((*cinfo)
                .image_height
                .wrapping_add((*cinfo).max_v_samp_factor as JDIMENSION)
                .wrapping_sub(1 as JDIMENSION)
                & !((*cinfo).max_v_samp_factor - 1 as ::core::ffi::c_int) as JDIMENSION)
                .wrapping_mul((*compptr).v_samp_factor as JDIMENSION)
                .wrapping_div((*cinfo).max_v_samp_factor as JDIMENSION)
                as ::core::ffi::c_int;
            if iw[i as usize] != pw[i as usize] || ih != ph[i as usize] {
                usetmpbuf = 1 as ::core::ffi::c_int;
            }
            th[i as usize] = (*compptr).v_samp_factor * DCTSIZE;
            tmpbufsize += iw[i as usize] * th[i as usize];
            inbuf[i as usize] = malloc(
                (::core::mem::size_of::<JSAMPROW>() as size_t)
                    .wrapping_mul(ph[i as usize] as size_t),
            ) as *mut JSAMPROW;
            if inbuf[i as usize].is_null() {
                snprintf(
                    &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjCompressFromYUVPlanes(): Memory allocation failure\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                (*this).isInstanceError = TRUE as boolean;
                snprintf(
                    &raw mut errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjCompressFromYUVPlanes(): Memory allocation failure\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                retval = -(1 as ::core::ffi::c_int);
                current_block = 15821307919385397474;
                break;
            } else {
                ptr = *srcPlanes.offset(i as isize) as *mut JSAMPLE;
                row = 0 as ::core::ffi::c_int;
                while row < ph[i as usize] {
                    let ref mut fresh5 = *inbuf[i as usize].offset(row as isize);
                    *fresh5 = ptr as JSAMPROW;
                    ptr = ptr.offset(
                        (if !strides.is_null()
                            && *strides.offset(i as isize) != 0 as ::core::ffi::c_int
                        {
                            *strides.offset(i as isize)
                        } else {
                            pw[i as usize]
                        }) as isize,
                    );
                    row += 1;
                }
                i += 1;
            }
        }
        match current_block {
            15821307919385397474 => {}
            _ => {
                if usetmpbuf != 0 {
                    _tmpbuf = malloc(
                        (::core::mem::size_of::<JSAMPLE>() as size_t)
                            .wrapping_mul(tmpbufsize as size_t),
                    ) as *mut JSAMPLE;
                    if _tmpbuf.is_null() {
                        snprintf(
                            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                            JMSG_LENGTH_MAX as size_t,
                            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                            b"tjCompressFromYUVPlanes(): Memory allocation failure\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        (*this).isInstanceError = TRUE as boolean;
                        snprintf(
                            &raw mut errStr as *mut ::core::ffi::c_char,
                            JMSG_LENGTH_MAX as size_t,
                            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                            b"tjCompressFromYUVPlanes(): Memory allocation failure\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        retval = -(1 as ::core::ffi::c_int);
                        current_block = 15821307919385397474;
                    } else {
                        ptr = _tmpbuf;
                        i = 0 as ::core::ffi::c_int;
                        loop {
                            if !(i < (*cinfo).num_components) {
                                current_block = 10435735846551762309;
                                break;
                            }
                            tmpbuf[i as usize] = malloc(
                                (::core::mem::size_of::<JSAMPROW>() as size_t)
                                    .wrapping_mul(th[i as usize] as size_t),
                            ) as *mut JSAMPROW;
                            if tmpbuf[i as usize].is_null() {
                                snprintf(
                                    &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                    JMSG_LENGTH_MAX as size_t,
                                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"tjCompressFromYUVPlanes(): Memory allocation failure\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                (*this).isInstanceError = TRUE as boolean;
                                snprintf(
                                    &raw mut errStr as *mut ::core::ffi::c_char,
                                    JMSG_LENGTH_MAX as size_t,
                                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"tjCompressFromYUVPlanes(): Memory allocation failure\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                retval = -(1 as ::core::ffi::c_int);
                                current_block = 15821307919385397474;
                                break;
                            } else {
                                row = 0 as ::core::ffi::c_int;
                                while row < th[i as usize] {
                                    let ref mut fresh6 = *tmpbuf[i as usize].offset(row as isize);
                                    *fresh6 = ptr as JSAMPROW;
                                    ptr = ptr.offset(iw[i as usize] as isize);
                                    row += 1;
                                }
                                i += 1;
                            }
                        }
                    }
                } else {
                    current_block = 10435735846551762309;
                }
                match current_block {
                    15821307919385397474 => {}
                    _ => {
                        if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
                            retval = -(1 as ::core::ffi::c_int);
                        } else {
                            row = 0 as ::core::ffi::c_int;
                            while row < (*cinfo).image_height as ::core::ffi::c_int {
                                let mut yuvptr: [JSAMPARRAY; 10] =
                                    [::core::ptr::null_mut::<JSAMPROW>(); 10];
                                let mut crow: [::core::ffi::c_int; 10] = [0; 10];
                                i = 0 as ::core::ffi::c_int;
                                while i < (*cinfo).num_components {
                                    let mut compptr_0: *mut jpeg_component_info =
                                        (*cinfo).comp_info.offset(i as isize)
                                            as *mut jpeg_component_info;
                                    crow[i as usize] = row * (*compptr_0).v_samp_factor
                                        / (*cinfo).max_v_samp_factor;
                                    if usetmpbuf != 0 {
                                        let mut j: ::core::ffi::c_int = 0;
                                        let mut k: ::core::ffi::c_int = 0;
                                        j = 0 as ::core::ffi::c_int;
                                        while j
                                            < (if th[i as usize] < ph[i as usize] - crow[i as usize]
                                            {
                                                th[i as usize]
                                            } else {
                                                ph[i as usize] - crow[i as usize]
                                            })
                                        {
                                            memcpy(
                                                *tmpbuf[i as usize].offset(j as isize)
                                                    as *mut ::core::ffi::c_void,
                                                *inbuf[i as usize]
                                                    .offset((crow[i as usize] + j) as isize)
                                                    as *const ::core::ffi::c_void,
                                                pw[i as usize] as size_t,
                                            );
                                            k = pw[i as usize];
                                            while k < iw[i as usize] {
                                                *(*tmpbuf[i as usize].offset(j as isize))
                                                    .offset(k as isize) = *(*tmpbuf[i as usize]
                                                    .offset(j as isize))
                                                .offset(
                                                    (pw[i as usize] - 1 as ::core::ffi::c_int)
                                                        as isize,
                                                );
                                                k += 1;
                                            }
                                            j += 1;
                                        }
                                        j = ph[i as usize] - crow[i as usize];
                                        while j < th[i as usize] {
                                            memcpy(
                                                *tmpbuf[i as usize].offset(j as isize)
                                                    as *mut ::core::ffi::c_void,
                                                *tmpbuf[i as usize].offset(
                                                    (ph[i as usize]
                                                        - crow[i as usize]
                                                        - 1 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as *const ::core::ffi::c_void,
                                                iw[i as usize] as size_t,
                                            );
                                            j += 1;
                                        }
                                        yuvptr[i as usize] = tmpbuf[i as usize] as JSAMPARRAY;
                                    } else {
                                        yuvptr[i as usize] = (*(&raw mut inbuf
                                            as *mut *mut JSAMPROW)
                                            .offset(i as isize))
                                        .offset(
                                            *(&raw mut crow as *mut ::core::ffi::c_int)
                                                .offset(i as isize)
                                                as isize,
                                        )
                                            as *mut JSAMPROW
                                            as JSAMPARRAY;
                                    }
                                    i += 1;
                                }
                                jpeg_write_raw_data(
                                    cinfo,
                                    &raw mut yuvptr as JSAMPIMAGE,
                                    ((*cinfo).max_v_samp_factor * DCTSIZE) as JDIMENSION,
                                );
                                row += (*cinfo).max_v_samp_factor * DCTSIZE;
                            }
                            jpeg_finish_compress(cinfo);
                        }
                    }
                }
            }
        }
    }
    if (*cinfo).global_state > CSTATE_START {
        if alloc != 0 {
            Some(
                (*(*cinfo).dest)
                    .term_destination
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
        jpeg_abort_compress(cinfo);
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAX_COMPONENTS {
        free(tmpbuf[i as usize] as *mut ::core::ffi::c_void);
        free(inbuf[i as usize] as *mut ::core::ffi::c_void);
        i += 1;
    }
    free(_tmpbuf as *mut ::core::ffi::c_void);
    if (*this).jerr.warning != 0 {
        retval = -(1 as ::core::ffi::c_int);
    }
    (*this).jerr.stopOnWarning = FALSE as boolean;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjCompressFromYUV(
    mut handle: tjhandle,
    mut srcBuf: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut align: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
    mut jpegBuf: *mut *mut ::core::ffi::c_uchar,
    mut jpegSize: *mut ::core::ffi::c_ulong,
    mut jpegQual: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut srcPlanes: [*const ::core::ffi::c_uchar; 3] =
        [::core::ptr::null::<::core::ffi::c_uchar>(); 3];
    let mut pw0: ::core::ffi::c_int = 0;
    let mut ph0: ::core::ffi::c_int = 0;
    let mut strides: [::core::ffi::c_int; 3] = [0; 3];
    let mut retval: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjCompressFromYUV(): Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        (*this).isInstanceError = FALSE as boolean;
        if srcBuf.is_null()
            || width <= 0 as ::core::ffi::c_int
            || align < 1 as ::core::ffi::c_int
            || !(align & align - 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            || height <= 0 as ::core::ffi::c_int
            || subsamp < 0 as ::core::ffi::c_int
            || subsamp >= TJ_NUMSAMP
        {
            snprintf(
                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjCompressFromYUV(): Invalid argument\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*this).isInstanceError = TRUE as boolean;
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjCompressFromYUV(): Invalid argument\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int);
        } else {
            pw0 = tjPlaneWidth(0 as ::core::ffi::c_int, width, subsamp);
            ph0 = tjPlaneHeight(0 as ::core::ffi::c_int, height, subsamp);
            srcPlanes[0 as ::core::ffi::c_int as usize] = srcBuf;
            strides[0 as ::core::ffi::c_int as usize] =
                pw0 + align - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
            if subsamp == TJSAMP_GRAY as ::core::ffi::c_int {
                strides[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
                strides[1 as ::core::ffi::c_int as usize] =
                    strides[2 as ::core::ffi::c_int as usize];
                srcPlanes[2 as ::core::ffi::c_int as usize] =
                    ::core::ptr::null::<::core::ffi::c_uchar>();
                srcPlanes[1 as ::core::ffi::c_int as usize] =
                    srcPlanes[2 as ::core::ffi::c_int as usize];
            } else {
                let mut pw1: ::core::ffi::c_int =
                    tjPlaneWidth(1 as ::core::ffi::c_int, width, subsamp);
                let mut ph1: ::core::ffi::c_int =
                    tjPlaneHeight(1 as ::core::ffi::c_int, height, subsamp);
                strides[2 as ::core::ffi::c_int as usize] =
                    pw1 + align - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
                strides[1 as ::core::ffi::c_int as usize] =
                    strides[2 as ::core::ffi::c_int as usize];
                srcPlanes[1 as ::core::ffi::c_int as usize] = srcPlanes
                    [0 as ::core::ffi::c_int as usize]
                    .offset((strides[0 as ::core::ffi::c_int as usize] * ph0) as isize);
                srcPlanes[2 as ::core::ffi::c_int as usize] = srcPlanes
                    [1 as ::core::ffi::c_int as usize]
                    .offset((strides[1 as ::core::ffi::c_int as usize] * ph1) as isize);
            }
            return tjCompressFromYUVPlanes(
                handle,
                &raw mut srcPlanes as *mut *const ::core::ffi::c_uchar,
                width,
                &raw mut strides as *mut ::core::ffi::c_int,
                height,
                subsamp,
                jpegBuf,
                jpegSize,
                jpegQual,
                flags,
            );
        }
    }
    return retval;
}
unsafe extern "C" fn _tjInitDecompress(mut this: *mut tjinstance) -> tjhandle {
    static mut buffer: [::core::ffi::c_uchar; 1] = [0; 1];
    (*this).dinfo.err = jpeg_std_error(&raw mut (*this).jerr.pub_0);
    (*this).jerr.pub_0.error_exit = Some(my_error_exit as unsafe extern "C" fn(j_common_ptr) -> ())
        as Option<unsafe extern "C" fn(j_common_ptr) -> ()>;
    (*this).jerr.pub_0.output_message =
        Some(my_output_message as unsafe extern "C" fn(j_common_ptr) -> ())
            as Option<unsafe extern "C" fn(j_common_ptr) -> ()>;
    (*this).jerr.emit_message = (*this).jerr.pub_0.emit_message;
    (*this).jerr.pub_0.emit_message =
        Some(my_emit_message as unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> ())
            as Option<unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> ()>;
    (*this).jerr.pub_0.addon_message_table =
        &raw mut turbojpeg_message_table as *mut *const ::core::ffi::c_char;
    (*this).jerr.pub_0.first_addon_message = JMSG_FIRSTADDONCODE as ::core::ffi::c_int;
    (*this).jerr.pub_0.last_addon_message = JMSG_LASTADDONCODE as ::core::ffi::c_int;
    if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
        free(this as *mut ::core::ffi::c_void);
        return NULL;
    }
    jpeg_CreateDecompress(
        &raw mut (*this).dinfo,
        JPEG_LIB_VERSION,
        ::core::mem::size_of::<jpeg_decompress_struct>(),
    );
    jpeg_mem_src_tj(
        &raw mut (*this).dinfo,
        &raw mut buffer as *mut ::core::ffi::c_uchar,
        1 as ::core::ffi::c_ulong,
    );
    (*this).init |= DECOMPRESS as ::core::ffi::c_int;
    return this as tjhandle;
}
#[no_mangle]
pub unsafe extern "C" fn tjInitDecompress() -> tjhandle {
    let mut this: *mut tjinstance = ::core::ptr::null_mut::<tjinstance>();
    this = malloc(::core::mem::size_of::<tjinstance>() as size_t) as *mut tjinstance;
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"tjInitDecompress(): Memory allocation failure\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    memset(
        this as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tjinstance>() as size_t,
    );
    snprintf(
        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
        JMSG_LENGTH_MAX as size_t,
        b"No error\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return _tjInitDecompress(this);
}
#[no_mangle]
pub unsafe extern "C" fn tjDecompressHeader3(
    mut handle: tjhandle,
    mut jpegBuf: *const ::core::ffi::c_uchar,
    mut jpegSize: ::core::ffi::c_ulong,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut jpegSubsamp: *mut ::core::ffi::c_int,
    mut jpegColorspace: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut dinfo: j_decompress_ptr = ::core::ptr::null_mut::<jpeg_decompress_struct>();
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    dinfo = &raw mut (*this).dinfo as j_decompress_ptr;
    (*this).jerr.warning = FALSE as boolean;
    (*this).isInstanceError = FALSE as boolean;
    if (*this).init & DECOMPRESS as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompressHeader3(): Instance has not been initialized for decompression\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompressHeader3(): Instance has not been initialized for decompression\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if jpegBuf.is_null()
        || jpegSize <= 0 as ::core::ffi::c_ulong
        || width.is_null()
        || height.is_null()
        || jpegSubsamp.is_null()
        || jpegColorspace.is_null()
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompressHeader3(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompressHeader3(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        jpeg_mem_src_tj(dinfo, jpegBuf, jpegSize);
        if jpeg_read_header(dinfo, FALSE) == JPEG_HEADER_TABLES_ONLY {
            return 0 as ::core::ffi::c_int;
        }
        *width = (*dinfo).image_width as ::core::ffi::c_int;
        *height = (*dinfo).image_height as ::core::ffi::c_int;
        *jpegSubsamp = getSubsamp(dinfo);
        match (*dinfo).jpeg_color_space as ::core::ffi::c_uint {
            1 => {
                *jpegColorspace = TJCS_GRAY as ::core::ffi::c_int;
            }
            2 => {
                *jpegColorspace = TJCS_RGB as ::core::ffi::c_int;
            }
            3 => {
                *jpegColorspace = TJCS_YCbCr as ::core::ffi::c_int;
            }
            4 => {
                *jpegColorspace = TJCS_CMYK as ::core::ffi::c_int;
            }
            5 => {
                *jpegColorspace = TJCS_YCCK as ::core::ffi::c_int;
            }
            _ => {
                *jpegColorspace = -(1 as ::core::ffi::c_int);
            }
        }
        jpeg_abort_decompress(dinfo);
        if *jpegSubsamp < 0 as ::core::ffi::c_int {
            snprintf(
                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecompressHeader3(): Could not determine subsampling type for JPEG image\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            (*this).isInstanceError = TRUE as boolean;
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecompressHeader3(): Could not determine subsampling type for JPEG image\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int);
        } else if *jpegColorspace < 0 as ::core::ffi::c_int {
            snprintf(
                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecompressHeader3(): Could not determine colorspace of JPEG image\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            (*this).isInstanceError = TRUE as boolean;
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecompressHeader3(): Could not determine colorspace of JPEG image\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int);
        } else if *width < 1 as ::core::ffi::c_int || *height < 1 as ::core::ffi::c_int {
            snprintf(
                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecompressHeader3(): Invalid data returned in header\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*this).isInstanceError = TRUE as boolean;
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecompressHeader3(): Invalid data returned in header\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int);
        }
    }
    if (*this).jerr.warning != 0 {
        retval = -(1 as ::core::ffi::c_int);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjDecompressHeader2(
    mut handle: tjhandle,
    mut jpegBuf: *mut ::core::ffi::c_uchar,
    mut jpegSize: ::core::ffi::c_ulong,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut jpegSubsamp: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut jpegColorspace: ::core::ffi::c_int = 0;
    return tjDecompressHeader3(
        handle,
        jpegBuf,
        jpegSize,
        width,
        height,
        jpegSubsamp,
        &raw mut jpegColorspace,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tjDecompressHeader(
    mut handle: tjhandle,
    mut jpegBuf: *mut ::core::ffi::c_uchar,
    mut jpegSize: ::core::ffi::c_ulong,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut jpegSubsamp: ::core::ffi::c_int = 0;
    return tjDecompressHeader2(
        handle,
        jpegBuf,
        jpegSize,
        width,
        height,
        &raw mut jpegSubsamp,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tjGetScalingFactors(
    mut numScalingFactors: *mut ::core::ffi::c_int,
) -> *mut tjscalingfactor {
    if numScalingFactors.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"tjGetScalingFactors(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<tjscalingfactor>();
    }
    *numScalingFactors = NUMSF;
    return &raw const sf as *const tjscalingfactor as *mut tjscalingfactor;
}
#[no_mangle]
pub unsafe extern "C" fn tjDecompress2(
    mut handle: tjhandle,
    mut jpegBuf: *const ::core::ffi::c_uchar,
    mut jpegSize: ::core::ffi::c_ulong,
    mut dstBuf: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut pixelFormat: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut row_pointer: *mut JSAMPROW = ::core::ptr::null_mut::<JSAMPROW>();
    let mut i: ::core::ffi::c_int = 0;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut jpegwidth: ::core::ffi::c_int = 0;
    let mut jpegheight: ::core::ffi::c_int = 0;
    let mut scaledw: ::core::ffi::c_int = 0;
    let mut scaledh: ::core::ffi::c_int = 0;
    let mut progress: my_progress_mgr = my_progress_mgr {
        pub_0: jpeg_progress_mgr {
            progress_monitor: None,
            pass_counter: 0,
            pass_limit: 0,
            completed_passes: 0,
            total_passes: 0,
        },
        this: ::core::ptr::null_mut::<tjinstance>(),
    };
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut dinfo: j_decompress_ptr = ::core::ptr::null_mut::<jpeg_decompress_struct>();
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    dinfo = &raw mut (*this).dinfo as j_decompress_ptr;
    (*this).jerr.warning = FALSE as boolean;
    (*this).isInstanceError = FALSE as boolean;
    (*this).jerr.stopOnWarning = (if flags & TJFLAG_STOPONWARNING != 0 {
        TRUE
    } else {
        FALSE
    }) as boolean;
    if (*this).init & DECOMPRESS as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompress2(): Instance has not been initialized for decompression\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompress2(): Instance has not been initialized for decompression\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if jpegBuf.is_null()
        || jpegSize <= 0 as ::core::ffi::c_ulong
        || dstBuf.is_null()
        || width < 0 as ::core::ffi::c_int
        || pitch < 0 as ::core::ffi::c_int
        || height < 0 as ::core::ffi::c_int
        || pixelFormat < 0 as ::core::ffi::c_int
        || pixelFormat >= TJ_NUMPF
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompress2(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompress2(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        if flags & TJFLAG_FORCEMMX != 0 {
            PUTENV_S(
                b"JSIMD_FORCEMMX\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if flags & TJFLAG_FORCESSE != 0 {
            PUTENV_S(
                b"JSIMD_FORCESSE\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if flags & TJFLAG_FORCESSE2 != 0 {
            PUTENV_S(
                b"JSIMD_FORCESSE2\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if flags & TJFLAG_LIMITSCANS != 0 {
            memset(
                &raw mut progress as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<my_progress_mgr>() as size_t,
            );
            progress.pub_0.progress_monitor =
                Some(my_progress_monitor as unsafe extern "C" fn(j_common_ptr) -> ())
                    as Option<unsafe extern "C" fn(j_common_ptr) -> ()>;
            progress.this = this;
            (*dinfo).progress = &raw mut progress.pub_0;
        } else {
            (*dinfo).progress = ::core::ptr::null_mut::<jpeg_progress_mgr>();
        }
        if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
            retval = -(1 as ::core::ffi::c_int);
        } else {
            jpeg_mem_src_tj(dinfo, jpegBuf, jpegSize);
            jpeg_read_header(dinfo, TRUE);
            (*this).dinfo.out_color_space = pf2cs[pixelFormat as usize];
            if flags & TJFLAG_FASTDCT != 0 {
                (*this).dinfo.dct_method = JDCT_IFAST;
            }
            if flags & TJFLAG_FASTUPSAMPLE != 0 {
                (*dinfo).do_fancy_upsampling = FALSE as boolean;
            }
            jpegwidth = (*dinfo).image_width as ::core::ffi::c_int;
            jpegheight = (*dinfo).image_height as ::core::ffi::c_int;
            if width == 0 as ::core::ffi::c_int {
                width = jpegwidth;
            }
            if height == 0 as ::core::ffi::c_int {
                height = jpegheight;
            }
            i = 0 as ::core::ffi::c_int;
            while i < NUMSF {
                scaledw = (jpegwidth * sf[i as usize].num + sf[i as usize].denom
                    - 1 as ::core::ffi::c_int)
                    / sf[i as usize].denom;
                scaledh = (jpegheight * sf[i as usize].num + sf[i as usize].denom
                    - 1 as ::core::ffi::c_int)
                    / sf[i as usize].denom;
                if scaledw <= width && scaledh <= height {
                    break;
                }
                i += 1;
            }
            if i >= NUMSF {
                snprintf(
                    &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjDecompress2(): Could not scale down to desired image dimensions\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                (*this).isInstanceError = TRUE as boolean;
                snprintf(
                    &raw mut errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjDecompress2(): Could not scale down to desired image dimensions\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                retval = -(1 as ::core::ffi::c_int);
            } else {
                width = scaledw;
                height = scaledh;
                (*dinfo).scale_num = sf[i as usize].num as ::core::ffi::c_uint;
                (*dinfo).scale_denom = sf[i as usize].denom as ::core::ffi::c_uint;
                jpeg_start_decompress(dinfo);
                if pitch == 0 as ::core::ffi::c_int {
                    pitch = (*dinfo)
                        .output_width
                        .wrapping_mul(tjPixelSize[pixelFormat as usize] as JDIMENSION)
                        as ::core::ffi::c_int;
                }
                row_pointer = malloc(
                    (::core::mem::size_of::<JSAMPROW>() as size_t)
                        .wrapping_mul((*dinfo).output_height as size_t),
                ) as *mut JSAMPROW;
                if row_pointer.is_null() {
                    snprintf(
                        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjDecompress2(): Memory allocation failure\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    (*this).isInstanceError = TRUE as boolean;
                    snprintf(
                        &raw mut errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjDecompress2(): Memory allocation failure\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    retval = -(1 as ::core::ffi::c_int);
                } else if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
                    retval = -(1 as ::core::ffi::c_int);
                } else {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*dinfo).output_height as ::core::ffi::c_int {
                        if flags & TJFLAG_BOTTOMUP != 0 {
                            let ref mut fresh13 = *row_pointer.offset(i as isize);
                            *fresh13 = dstBuf.offset(
                                ((*dinfo)
                                    .output_height
                                    .wrapping_sub(i as JDIMENSION)
                                    .wrapping_sub(1 as JDIMENSION)
                                    as size_t)
                                    .wrapping_mul(pitch as size_t)
                                    as isize,
                            ) as *mut ::core::ffi::c_uchar
                                as JSAMPROW;
                        } else {
                            let ref mut fresh14 = *row_pointer.offset(i as isize);
                            *fresh14 = dstBuf
                                .offset((i as size_t).wrapping_mul(pitch as size_t) as isize)
                                as *mut ::core::ffi::c_uchar
                                as JSAMPROW;
                        }
                        i += 1;
                    }
                    while (*dinfo).output_scanline < (*dinfo).output_height {
                        jpeg_read_scanlines(
                            dinfo,
                            row_pointer.offset((*dinfo).output_scanline as isize) as JSAMPARRAY,
                            (*dinfo)
                                .output_height
                                .wrapping_sub((*dinfo).output_scanline),
                        );
                    }
                    jpeg_finish_decompress(dinfo);
                }
            }
        }
    }
    if (*dinfo).global_state > DSTATE_START {
        jpeg_abort_decompress(dinfo);
    }
    free(row_pointer as *mut ::core::ffi::c_void);
    if (*this).jerr.warning != 0 {
        retval = -(1 as ::core::ffi::c_int);
    }
    (*this).jerr.stopOnWarning = FALSE as boolean;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjDecompress(
    mut handle: tjhandle,
    mut jpegBuf: *mut ::core::ffi::c_uchar,
    mut jpegSize: ::core::ffi::c_ulong,
    mut dstBuf: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut pixelSize: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if flags & TJ_YUV != 0 {
        return tjDecompressToYUV(handle, jpegBuf, jpegSize, dstBuf, flags);
    } else {
        return tjDecompress2(
            handle,
            jpegBuf,
            jpegSize,
            dstBuf,
            width,
            pitch,
            height,
            getPixelFormat(pixelSize, flags),
            flags,
        );
    };
}
unsafe extern "C" fn setDecodeDefaults(
    mut dinfo: *mut jpeg_decompress_struct,
    mut pixelFormat: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    (*dinfo).scale_denom = 1 as ::core::ffi::c_uint;
    (*dinfo).scale_num = (*dinfo).scale_denom;
    if subsamp == TJSAMP_GRAY as ::core::ffi::c_int {
        (*dinfo).comps_in_scan = 1 as ::core::ffi::c_int;
        (*dinfo).num_components = (*dinfo).comps_in_scan;
        (*dinfo).jpeg_color_space = JCS_GRAYSCALE;
    } else {
        (*dinfo).comps_in_scan = 3 as ::core::ffi::c_int;
        (*dinfo).num_components = (*dinfo).comps_in_scan;
        (*dinfo).jpeg_color_space = JCS_YCbCr;
    }
    (*dinfo).comp_info = Some(
        (*(*dinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        dinfo as j_common_ptr,
        JPOOL_IMAGE,
        ((*dinfo).num_components as size_t)
            .wrapping_mul(::core::mem::size_of::<jpeg_component_info>() as size_t),
    ) as *mut jpeg_component_info;
    i = 0 as ::core::ffi::c_int;
    while i < (*dinfo).num_components {
        let mut compptr: *mut jpeg_component_info =
            (*dinfo).comp_info.offset(i as isize) as *mut jpeg_component_info;
        (*compptr).h_samp_factor = if i == 0 as ::core::ffi::c_int {
            tjMCUWidth[subsamp as usize] / 8 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        (*compptr).v_samp_factor = if i == 0 as ::core::ffi::c_int {
            tjMCUHeight[subsamp as usize] / 8 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        (*compptr).component_index = i;
        (*compptr).component_id = i + 1 as ::core::ffi::c_int;
        (*compptr).ac_tbl_no = if i == 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        (*compptr).dc_tbl_no = (*compptr).ac_tbl_no;
        (*compptr).quant_tbl_no = (*compptr).dc_tbl_no;
        (*dinfo).cur_comp_info[i as usize] = compptr;
        i += 1;
    }
    (*dinfo).data_precision = 8 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        if (*dinfo).quant_tbl_ptrs[i as usize].is_null() {
            (*dinfo).quant_tbl_ptrs[i as usize] = jpeg_alloc_quant_table(dinfo as j_common_ptr);
        }
        i += 1;
    }
}
unsafe extern "C" fn my_read_markers(mut dinfo: j_decompress_ptr) -> ::core::ffi::c_int {
    return JPEG_REACHED_SOS;
}
unsafe extern "C" fn my_reset_marker_reader(mut dinfo: j_decompress_ptr) {}
#[no_mangle]
pub unsafe extern "C" fn tjDecodeYUVPlanes(
    mut handle: tjhandle,
    mut srcPlanes: *mut *const ::core::ffi::c_uchar,
    mut strides: *const ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
    mut dstBuf: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut pixelFormat: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut row_pointer: *mut JSAMPROW = ::core::ptr::null_mut::<JSAMPROW>();
    let mut _tmpbuf: [*mut JSAMPLE; 10] = [::core::ptr::null_mut::<JSAMPLE>(); 10];
    let mut tmpbuf: [*mut JSAMPROW; 10] = [::core::ptr::null_mut::<JSAMPROW>(); 10];
    let mut inbuf: [*mut JSAMPROW; 10] = [::core::ptr::null_mut::<JSAMPROW>(); 10];
    let mut i: ::core::ffi::c_int = 0;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut row: ::core::ffi::c_int = 0;
    let mut pw0: ::core::ffi::c_int = 0;
    let mut ph0: ::core::ffi::c_int = 0;
    let mut pw: [::core::ffi::c_int; 10] = [0; 10];
    let mut ph: [::core::ffi::c_int; 10] = [0; 10];
    let mut ptr: *mut JSAMPLE = ::core::ptr::null_mut::<JSAMPLE>();
    let mut compptr: *mut jpeg_component_info = ::core::ptr::null_mut::<jpeg_component_info>();
    let mut old_read_markers: Option<unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int> =
        None;
    let mut old_reset_marker_reader: Option<unsafe extern "C" fn(j_decompress_ptr) -> ()> = None;
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut dinfo: j_decompress_ptr = ::core::ptr::null_mut::<jpeg_decompress_struct>();
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    dinfo = &raw mut (*this).dinfo as j_decompress_ptr;
    (*this).jerr.warning = FALSE as boolean;
    (*this).isInstanceError = FALSE as boolean;
    (*this).jerr.stopOnWarning = (if flags & TJFLAG_STOPONWARNING != 0 {
        TRUE
    } else {
        FALSE
    }) as boolean;
    i = 0 as ::core::ffi::c_int;
    while i < MAX_COMPONENTS {
        tmpbuf[i as usize] = ::core::ptr::null_mut::<JSAMPROW>();
        _tmpbuf[i as usize] = ::core::ptr::null_mut::<JSAMPLE>();
        inbuf[i as usize] = ::core::ptr::null_mut::<JSAMPROW>();
        i += 1;
    }
    if (*this).init & DECOMPRESS as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecodeYUVPlanes(): Instance has not been initialized for decompression\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecodeYUVPlanes(): Instance has not been initialized for decompression\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if srcPlanes.is_null()
        || (*srcPlanes.offset(0 as ::core::ffi::c_int as isize)).is_null()
        || subsamp < 0 as ::core::ffi::c_int
        || subsamp >= TJ_NUMSAMP
        || dstBuf.is_null()
        || width <= 0 as ::core::ffi::c_int
        || pitch < 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
        || pixelFormat < 0 as ::core::ffi::c_int
        || pixelFormat >= TJ_NUMPF
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecodeYUVPlanes(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecodeYUVPlanes(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if subsamp != TJSAMP_GRAY as ::core::ffi::c_int
        && ((*srcPlanes.offset(1 as ::core::ffi::c_int as isize)).is_null()
            || (*srcPlanes.offset(2 as ::core::ffi::c_int as isize)).is_null())
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecodeYUVPlanes(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecodeYUVPlanes(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
        retval = -(1 as ::core::ffi::c_int);
    } else if pixelFormat == TJPF_CMYK as ::core::ffi::c_int {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecodeYUVPlanes(): Cannot decode YUV images into packed-pixel CMYK images.\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecodeYUVPlanes(): Cannot decode YUV images into packed-pixel CMYK images.\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        if pitch == 0 as ::core::ffi::c_int {
            pitch = width * tjPixelSize[pixelFormat as usize];
        }
        (*dinfo).image_width = width as JDIMENSION;
        (*dinfo).image_height = height as JDIMENSION;
        if flags & TJFLAG_FORCEMMX != 0 {
            PUTENV_S(
                b"JSIMD_FORCEMMX\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if flags & TJFLAG_FORCESSE != 0 {
            PUTENV_S(
                b"JSIMD_FORCESSE\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if flags & TJFLAG_FORCESSE2 != 0 {
            PUTENV_S(
                b"JSIMD_FORCESSE2\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*(*dinfo).inputctl).has_multiple_scans = FALSE as boolean;
        (*dinfo).progressive_mode = (*(*dinfo).inputctl).has_multiple_scans;
        (*dinfo).Al = 0 as ::core::ffi::c_int;
        (*dinfo).Ah = (*dinfo).Al;
        (*dinfo).Ss = (*dinfo).Ah;
        (*dinfo).Se = DCTSIZE2 - 1 as ::core::ffi::c_int;
        setDecodeDefaults(
            dinfo as *mut jpeg_decompress_struct,
            pixelFormat,
            subsamp,
            flags,
        );
        old_read_markers = (*(*dinfo).marker).read_markers;
        (*(*dinfo).marker).read_markers =
            Some(my_read_markers as unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int)
                as Option<unsafe extern "C" fn(j_decompress_ptr) -> ::core::ffi::c_int>;
        old_reset_marker_reader = (*(*dinfo).marker).reset_marker_reader;
        (*(*dinfo).marker).reset_marker_reader =
            Some(my_reset_marker_reader as unsafe extern "C" fn(j_decompress_ptr) -> ())
                as Option<unsafe extern "C" fn(j_decompress_ptr) -> ()>;
        jpeg_read_header(dinfo, TRUE);
        (*(*dinfo).marker).read_markers = old_read_markers;
        (*(*dinfo).marker).reset_marker_reader = old_reset_marker_reader;
        (*this).dinfo.out_color_space = pf2cs[pixelFormat as usize];
        if flags & TJFLAG_FASTDCT != 0 {
            (*this).dinfo.dct_method = JDCT_IFAST;
        }
        (*dinfo).do_fancy_upsampling = FALSE as boolean;
        (*dinfo).Se = DCTSIZE2 - 1 as ::core::ffi::c_int;
        jinit_master_decompress(dinfo);
        Some(
            (*(*dinfo).upsample)
                .start_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(dinfo);
        pw0 = width + (*dinfo).max_h_samp_factor - 1 as ::core::ffi::c_int
            & !((*dinfo).max_h_samp_factor - 1 as ::core::ffi::c_int);
        ph0 = height + (*dinfo).max_v_samp_factor - 1 as ::core::ffi::c_int
            & !((*dinfo).max_v_samp_factor - 1 as ::core::ffi::c_int);
        if pitch == 0 as ::core::ffi::c_int {
            pitch = (*dinfo)
                .output_width
                .wrapping_mul(tjPixelSize[pixelFormat as usize] as JDIMENSION)
                as ::core::ffi::c_int;
        }
        row_pointer =
            malloc((::core::mem::size_of::<JSAMPROW>() as size_t).wrapping_mul(ph0 as size_t))
                as *mut JSAMPROW;
        if row_pointer.is_null() {
            snprintf(
                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecodeYUVPlanes(): Memory allocation failure\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*this).isInstanceError = TRUE as boolean;
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecodeYUVPlanes(): Memory allocation failure\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int);
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < height {
                if flags & TJFLAG_BOTTOMUP != 0 {
                    let ref mut fresh17 = *row_pointer.offset(i as isize);
                    *fresh17 = dstBuf.offset(
                        ((height - i - 1 as ::core::ffi::c_int) as size_t)
                            .wrapping_mul(pitch as size_t) as isize,
                    ) as *mut ::core::ffi::c_uchar as JSAMPROW;
                } else {
                    let ref mut fresh18 = *row_pointer.offset(i as isize);
                    *fresh18 = dstBuf.offset((i as size_t).wrapping_mul(pitch as size_t) as isize)
                        as *mut ::core::ffi::c_uchar as JSAMPROW;
                }
                i += 1;
            }
            if height < ph0 {
                i = height;
                while i < ph0 {
                    let ref mut fresh19 = *row_pointer.offset(i as isize);
                    *fresh19 = *row_pointer.offset((height - 1 as ::core::ffi::c_int) as isize);
                    i += 1;
                }
            }
            i = 0 as ::core::ffi::c_int;
            loop {
                if !(i < (*dinfo).num_components) {
                    current_block = 10153752038087260855;
                    break;
                }
                compptr = (*dinfo).comp_info.offset(i as isize) as *mut jpeg_component_info;
                _tmpbuf[i as usize] = malloc(
                    ((*compptr)
                        .width_in_blocks
                        .wrapping_mul(8 as JDIMENSION)
                        .wrapping_add(32 as JDIMENSION)
                        .wrapping_sub(1 as JDIMENSION)
                        & !(32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as JDIMENSION)
                        .wrapping_mul((*compptr).v_samp_factor as JDIMENSION)
                        .wrapping_add(32 as JDIMENSION) as size_t,
                ) as *mut JSAMPLE;
                if _tmpbuf[i as usize].is_null() {
                    snprintf(
                        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjDecodeYUVPlanes(): Memory allocation failure\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    (*this).isInstanceError = TRUE as boolean;
                    snprintf(
                        &raw mut errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjDecodeYUVPlanes(): Memory allocation failure\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    retval = -(1 as ::core::ffi::c_int);
                    current_block = 15552504796284461537;
                    break;
                } else {
                    tmpbuf[i as usize] = malloc(
                        (::core::mem::size_of::<JSAMPROW>() as size_t)
                            .wrapping_mul((*compptr).v_samp_factor as size_t),
                    ) as *mut JSAMPROW;
                    if tmpbuf[i as usize].is_null() {
                        snprintf(
                            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                            JMSG_LENGTH_MAX as size_t,
                            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                            b"tjDecodeYUVPlanes(): Memory allocation failure\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        (*this).isInstanceError = TRUE as boolean;
                        snprintf(
                            &raw mut errStr as *mut ::core::ffi::c_char,
                            JMSG_LENGTH_MAX as size_t,
                            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                            b"tjDecodeYUVPlanes(): Memory allocation failure\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        retval = -(1 as ::core::ffi::c_int);
                        current_block = 15552504796284461537;
                        break;
                    } else {
                        row = 0 as ::core::ffi::c_int;
                        while row < (*compptr).v_samp_factor {
                            let mut _tmpbuf_aligned: *mut ::core::ffi::c_uchar = ((_tmpbuf
                                [i as usize]
                                as JUINTPTR)
                                .wrapping_add(32 as ::core::ffi::c_int as JUINTPTR)
                                .wrapping_sub(1 as ::core::ffi::c_int as JUINTPTR)
                                & !(32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as JUINTPTR)
                                as *mut ::core::ffi::c_uchar;
                            let ref mut fresh20 = *tmpbuf[i as usize].offset(row as isize);
                            *fresh20 = _tmpbuf_aligned.offset(
                                ((*compptr)
                                    .width_in_blocks
                                    .wrapping_mul(8 as JDIMENSION)
                                    .wrapping_add(32 as JDIMENSION)
                                    .wrapping_sub(1 as JDIMENSION)
                                    & !(32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                        as JDIMENSION)
                                    .wrapping_mul(row as JDIMENSION)
                                    as isize,
                            ) as *mut ::core::ffi::c_uchar
                                as JSAMPROW;
                            row += 1;
                        }
                        pw[i as usize] =
                            pw0 * (*compptr).h_samp_factor / (*dinfo).max_h_samp_factor;
                        ph[i as usize] =
                            ph0 * (*compptr).v_samp_factor / (*dinfo).max_v_samp_factor;
                        inbuf[i as usize] = malloc(
                            (::core::mem::size_of::<JSAMPROW>() as size_t)
                                .wrapping_mul(ph[i as usize] as size_t),
                        ) as *mut JSAMPROW;
                        if inbuf[i as usize].is_null() {
                            snprintf(
                                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                JMSG_LENGTH_MAX as size_t,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                b"tjDecodeYUVPlanes(): Memory allocation failure\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            (*this).isInstanceError = TRUE as boolean;
                            snprintf(
                                &raw mut errStr as *mut ::core::ffi::c_char,
                                JMSG_LENGTH_MAX as size_t,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                b"tjDecodeYUVPlanes(): Memory allocation failure\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            retval = -(1 as ::core::ffi::c_int);
                            current_block = 15552504796284461537;
                            break;
                        } else {
                            ptr = *srcPlanes.offset(i as isize) as *mut JSAMPLE;
                            row = 0 as ::core::ffi::c_int;
                            while row < ph[i as usize] {
                                let ref mut fresh21 = *inbuf[i as usize].offset(row as isize);
                                *fresh21 = ptr as JSAMPROW;
                                ptr = ptr.offset(
                                    (if !strides.is_null()
                                        && *strides.offset(i as isize) != 0 as ::core::ffi::c_int
                                    {
                                        *strides.offset(i as isize)
                                    } else {
                                        pw[i as usize]
                                    }) as isize,
                                );
                                row += 1;
                            }
                            i += 1;
                        }
                    }
                }
            }
            match current_block {
                15552504796284461537 => {}
                _ => {
                    if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
                        retval = -(1 as ::core::ffi::c_int);
                    } else {
                        row = 0 as ::core::ffi::c_int;
                        while row < ph0 {
                            let mut inrow: JDIMENSION = 0 as JDIMENSION;
                            let mut outrow: JDIMENSION = 0 as JDIMENSION;
                            i = 0 as ::core::ffi::c_int;
                            compptr = (*dinfo).comp_info;
                            while i < (*dinfo).num_components {
                                jcopy_sample_rows(
                                    inbuf[i as usize],
                                    row * (*compptr).v_samp_factor / (*dinfo).max_v_samp_factor,
                                    tmpbuf[i as usize],
                                    0 as ::core::ffi::c_int,
                                    (*compptr).v_samp_factor,
                                    pw[i as usize] as JDIMENSION,
                                );
                                i += 1;
                                compptr = compptr.offset(1);
                            }
                            (*(*dinfo).upsample)
                                .upsample
                                .expect("non-null function pointer")(
                                dinfo,
                                &raw mut tmpbuf as JSAMPIMAGE,
                                &raw mut inrow,
                                (*dinfo).max_v_samp_factor as JDIMENSION,
                                row_pointer.offset(row as isize) as JSAMPARRAY,
                                &raw mut outrow,
                                (*dinfo).max_v_samp_factor as JDIMENSION,
                            );
                            row += (*dinfo).max_v_samp_factor;
                        }
                        jpeg_abort_decompress(dinfo);
                    }
                }
            }
        }
    }
    if (*dinfo).global_state > DSTATE_START {
        jpeg_abort_decompress(dinfo);
    }
    free(row_pointer as *mut ::core::ffi::c_void);
    i = 0 as ::core::ffi::c_int;
    while i < MAX_COMPONENTS {
        free(tmpbuf[i as usize] as *mut ::core::ffi::c_void);
        free(_tmpbuf[i as usize] as *mut ::core::ffi::c_void);
        free(inbuf[i as usize] as *mut ::core::ffi::c_void);
        i += 1;
    }
    if (*this).jerr.warning != 0 {
        retval = -(1 as ::core::ffi::c_int);
    }
    (*this).jerr.stopOnWarning = FALSE as boolean;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjDecodeYUV(
    mut handle: tjhandle,
    mut srcBuf: *const ::core::ffi::c_uchar,
    mut align: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
    mut dstBuf: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut pixelFormat: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut srcPlanes: [*const ::core::ffi::c_uchar; 3] =
        [::core::ptr::null::<::core::ffi::c_uchar>(); 3];
    let mut pw0: ::core::ffi::c_int = 0;
    let mut ph0: ::core::ffi::c_int = 0;
    let mut strides: [::core::ffi::c_int; 3] = [0; 3];
    let mut retval: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecodeYUV(): Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        (*this).isInstanceError = FALSE as boolean;
        if srcBuf.is_null()
            || align < 1 as ::core::ffi::c_int
            || !(align & align - 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            || subsamp < 0 as ::core::ffi::c_int
            || subsamp >= TJ_NUMSAMP
            || width <= 0 as ::core::ffi::c_int
            || height <= 0 as ::core::ffi::c_int
        {
            snprintf(
                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecodeYUV(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
            );
            (*this).isInstanceError = TRUE as boolean;
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecodeYUV(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int);
        } else {
            pw0 = tjPlaneWidth(0 as ::core::ffi::c_int, width, subsamp);
            ph0 = tjPlaneHeight(0 as ::core::ffi::c_int, height, subsamp);
            srcPlanes[0 as ::core::ffi::c_int as usize] = srcBuf;
            strides[0 as ::core::ffi::c_int as usize] =
                pw0 + align - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
            if subsamp == TJSAMP_GRAY as ::core::ffi::c_int {
                strides[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
                strides[1 as ::core::ffi::c_int as usize] =
                    strides[2 as ::core::ffi::c_int as usize];
                srcPlanes[2 as ::core::ffi::c_int as usize] =
                    ::core::ptr::null::<::core::ffi::c_uchar>();
                srcPlanes[1 as ::core::ffi::c_int as usize] =
                    srcPlanes[2 as ::core::ffi::c_int as usize];
            } else {
                let mut pw1: ::core::ffi::c_int =
                    tjPlaneWidth(1 as ::core::ffi::c_int, width, subsamp);
                let mut ph1: ::core::ffi::c_int =
                    tjPlaneHeight(1 as ::core::ffi::c_int, height, subsamp);
                strides[2 as ::core::ffi::c_int as usize] =
                    pw1 + align - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
                strides[1 as ::core::ffi::c_int as usize] =
                    strides[2 as ::core::ffi::c_int as usize];
                srcPlanes[1 as ::core::ffi::c_int as usize] = srcPlanes
                    [0 as ::core::ffi::c_int as usize]
                    .offset((strides[0 as ::core::ffi::c_int as usize] * ph0) as isize);
                srcPlanes[2 as ::core::ffi::c_int as usize] = srcPlanes
                    [1 as ::core::ffi::c_int as usize]
                    .offset((strides[1 as ::core::ffi::c_int as usize] * ph1) as isize);
            }
            return tjDecodeYUVPlanes(
                handle,
                &raw mut srcPlanes as *mut *const ::core::ffi::c_uchar,
                &raw mut strides as *mut ::core::ffi::c_int,
                subsamp,
                dstBuf,
                width,
                pitch,
                height,
                pixelFormat,
                flags,
            );
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjDecompressToYUVPlanes(
    mut handle: tjhandle,
    mut jpegBuf: *const ::core::ffi::c_uchar,
    mut jpegSize: ::core::ffi::c_ulong,
    mut dstPlanes: *mut *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut strides: *mut ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut sfi: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut jpegwidth: ::core::ffi::c_int = 0;
    let mut jpegheight: ::core::ffi::c_int = 0;
    let mut jpegSubsamp: ::core::ffi::c_int = 0;
    let mut scaledw: ::core::ffi::c_int = 0;
    let mut scaledh: ::core::ffi::c_int = 0;
    let mut pw: [::core::ffi::c_int; 10] = [0; 10];
    let mut ph: [::core::ffi::c_int; 10] = [0; 10];
    let mut iw: [::core::ffi::c_int; 10] = [0; 10];
    let mut tmpbufsize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut usetmpbuf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut th: [::core::ffi::c_int; 10] = [0; 10];
    let mut _tmpbuf: *mut JSAMPLE = ::core::ptr::null_mut::<JSAMPLE>();
    let mut ptr: *mut JSAMPLE = ::core::ptr::null_mut::<JSAMPLE>();
    let mut outbuf: [*mut JSAMPROW; 10] = [::core::ptr::null_mut::<JSAMPROW>(); 10];
    let mut tmpbuf: [*mut JSAMPROW; 10] = [::core::ptr::null_mut::<JSAMPROW>(); 10];
    let mut dctsize: ::core::ffi::c_int = 0;
    let mut progress: my_progress_mgr = my_progress_mgr {
        pub_0: jpeg_progress_mgr {
            progress_monitor: None,
            pass_counter: 0,
            pass_limit: 0,
            completed_passes: 0,
            total_passes: 0,
        },
        this: ::core::ptr::null_mut::<tjinstance>(),
    };
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut dinfo: j_decompress_ptr = ::core::ptr::null_mut::<jpeg_decompress_struct>();
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    dinfo = &raw mut (*this).dinfo as j_decompress_ptr;
    (*this).jerr.warning = FALSE as boolean;
    (*this).isInstanceError = FALSE as boolean;
    (*this).jerr.stopOnWarning = (if flags & TJFLAG_STOPONWARNING != 0 {
        TRUE
    } else {
        FALSE
    }) as boolean;
    i = 0 as ::core::ffi::c_int;
    while i < MAX_COMPONENTS {
        tmpbuf[i as usize] = ::core::ptr::null_mut::<JSAMPROW>();
        outbuf[i as usize] = ::core::ptr::null_mut::<JSAMPROW>();
        i += 1;
    }
    if (*this).init & DECOMPRESS as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompressToYUVPlanes(): Instance has not been initialized for decompression\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompressToYUVPlanes(): Instance has not been initialized for decompression\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if jpegBuf.is_null()
        || jpegSize <= 0 as ::core::ffi::c_ulong
        || dstPlanes.is_null()
        || (*dstPlanes.offset(0 as ::core::ffi::c_int as isize)).is_null()
        || width < 0 as ::core::ffi::c_int
        || height < 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompressToYUVPlanes(): Invalid argument\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompressToYUVPlanes(): Invalid argument\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        if flags & TJFLAG_FORCEMMX != 0 {
            PUTENV_S(
                b"JSIMD_FORCEMMX\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if flags & TJFLAG_FORCESSE != 0 {
            PUTENV_S(
                b"JSIMD_FORCESSE\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if flags & TJFLAG_FORCESSE2 != 0 {
            PUTENV_S(
                b"JSIMD_FORCESSE2\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if flags & TJFLAG_LIMITSCANS != 0 {
            memset(
                &raw mut progress as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<my_progress_mgr>() as size_t,
            );
            progress.pub_0.progress_monitor =
                Some(my_progress_monitor as unsafe extern "C" fn(j_common_ptr) -> ())
                    as Option<unsafe extern "C" fn(j_common_ptr) -> ()>;
            progress.this = this;
            (*dinfo).progress = &raw mut progress.pub_0;
        } else {
            (*dinfo).progress = ::core::ptr::null_mut::<jpeg_progress_mgr>();
        }
        if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
            retval = -(1 as ::core::ffi::c_int);
        } else {
            if (*this).headerRead == 0 {
                jpeg_mem_src_tj(dinfo, jpegBuf, jpegSize);
                jpeg_read_header(dinfo, TRUE);
            }
            (*this).headerRead = 0 as ::core::ffi::c_int;
            jpegSubsamp = getSubsamp(dinfo);
            if jpegSubsamp < 0 as ::core::ffi::c_int {
                snprintf(
                    &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjDecompressToYUVPlanes(): Could not determine subsampling type for JPEG image\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                (*this).isInstanceError = TRUE as boolean;
                snprintf(
                    &raw mut errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjDecompressToYUVPlanes(): Could not determine subsampling type for JPEG image\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                retval = -(1 as ::core::ffi::c_int);
            } else if jpegSubsamp != TJSAMP_GRAY as ::core::ffi::c_int
                && ((*dstPlanes.offset(1 as ::core::ffi::c_int as isize)).is_null()
                    || (*dstPlanes.offset(2 as ::core::ffi::c_int as isize)).is_null())
            {
                snprintf(
                    &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjDecompressToYUVPlanes(): Invalid argument\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                (*this).isInstanceError = TRUE as boolean;
                snprintf(
                    &raw mut errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjDecompressToYUVPlanes(): Invalid argument\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                retval = -(1 as ::core::ffi::c_int);
            } else {
                jpegwidth = (*dinfo).image_width as ::core::ffi::c_int;
                jpegheight = (*dinfo).image_height as ::core::ffi::c_int;
                if width == 0 as ::core::ffi::c_int {
                    width = jpegwidth;
                }
                if height == 0 as ::core::ffi::c_int {
                    height = jpegheight;
                }
                i = 0 as ::core::ffi::c_int;
                while i < NUMSF {
                    scaledw = (jpegwidth * sf[i as usize].num + sf[i as usize].denom
                        - 1 as ::core::ffi::c_int)
                        / sf[i as usize].denom;
                    scaledh = (jpegheight * sf[i as usize].num + sf[i as usize].denom
                        - 1 as ::core::ffi::c_int)
                        / sf[i as usize].denom;
                    if scaledw <= width && scaledh <= height {
                        break;
                    }
                    i += 1;
                }
                if i >= NUMSF {
                    snprintf(
                        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjDecompressToYUVPlanes(): Could not scale down to desired image dimensions\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    (*this).isInstanceError = TRUE as boolean;
                    snprintf(
                        &raw mut errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjDecompressToYUVPlanes(): Could not scale down to desired image dimensions\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    retval = -(1 as ::core::ffi::c_int);
                } else if (*dinfo).num_components > 3 as ::core::ffi::c_int {
                    snprintf(
                        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjDecompressToYUVPlanes(): JPEG image must have 3 or fewer components\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    (*this).isInstanceError = TRUE as boolean;
                    snprintf(
                        &raw mut errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjDecompressToYUVPlanes(): JPEG image must have 3 or fewer components\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    retval = -(1 as ::core::ffi::c_int);
                } else {
                    width = scaledw;
                    height = scaledh;
                    (*dinfo).scale_num = sf[i as usize].num as ::core::ffi::c_uint;
                    (*dinfo).scale_denom = sf[i as usize].denom as ::core::ffi::c_uint;
                    sfi = i;
                    jpeg_calc_output_dimensions(dinfo);
                    dctsize = DCTSIZE * sf[sfi as usize].num / sf[sfi as usize].denom;
                    i = 0 as ::core::ffi::c_int;
                    loop {
                        if !(i < (*dinfo).num_components) {
                            current_block = 9505035279996566320;
                            break;
                        }
                        let mut compptr: *mut jpeg_component_info =
                            (*dinfo).comp_info.offset(i as isize) as *mut jpeg_component_info;
                        let mut ih: ::core::ffi::c_int = 0;
                        iw[i as usize] = (*compptr)
                            .width_in_blocks
                            .wrapping_mul(dctsize as JDIMENSION)
                            as ::core::ffi::c_int;
                        ih = (*compptr)
                            .height_in_blocks
                            .wrapping_mul(dctsize as JDIMENSION)
                            as ::core::ffi::c_int;
                        pw[i as usize] = tjPlaneWidth(
                            i,
                            (*dinfo).output_width as ::core::ffi::c_int,
                            jpegSubsamp,
                        );
                        ph[i as usize] = tjPlaneHeight(
                            i,
                            (*dinfo).output_height as ::core::ffi::c_int,
                            jpegSubsamp,
                        );
                        if iw[i as usize] != pw[i as usize] || ih != ph[i as usize] {
                            usetmpbuf = 1 as ::core::ffi::c_int;
                        }
                        th[i as usize] = (*compptr).v_samp_factor * dctsize;
                        tmpbufsize += iw[i as usize] * th[i as usize];
                        outbuf[i as usize] = malloc(
                            (::core::mem::size_of::<JSAMPROW>() as size_t)
                                .wrapping_mul(ph[i as usize] as size_t),
                        ) as *mut JSAMPROW;
                        if outbuf[i as usize].is_null() {
                            snprintf(
                                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                JMSG_LENGTH_MAX as size_t,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                b"tjDecompressToYUVPlanes(): Memory allocation failure\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            (*this).isInstanceError = TRUE as boolean;
                            snprintf(
                                &raw mut errStr as *mut ::core::ffi::c_char,
                                JMSG_LENGTH_MAX as size_t,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                b"tjDecompressToYUVPlanes(): Memory allocation failure\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            retval = -(1 as ::core::ffi::c_int);
                            current_block = 12563316033059927059;
                            break;
                        } else {
                            ptr = *dstPlanes.offset(i as isize) as *mut JSAMPLE;
                            row = 0 as ::core::ffi::c_int;
                            while row < ph[i as usize] {
                                let ref mut fresh15 = *outbuf[i as usize].offset(row as isize);
                                *fresh15 = ptr as JSAMPROW;
                                ptr = ptr.offset(
                                    (if !strides.is_null()
                                        && *strides.offset(i as isize) != 0 as ::core::ffi::c_int
                                    {
                                        *strides.offset(i as isize)
                                    } else {
                                        pw[i as usize]
                                    }) as isize,
                                );
                                row += 1;
                            }
                            i += 1;
                        }
                    }
                    match current_block {
                        12563316033059927059 => {}
                        _ => {
                            if usetmpbuf != 0 {
                                _tmpbuf = malloc(
                                    (::core::mem::size_of::<JSAMPLE>() as size_t)
                                        .wrapping_mul(tmpbufsize as size_t),
                                ) as *mut JSAMPLE;
                                if _tmpbuf.is_null() {
                                    snprintf(
                                        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                        JMSG_LENGTH_MAX as size_t,
                                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                        b"tjDecompressToYUVPlanes(): Memory allocation failure\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    (*this).isInstanceError = TRUE as boolean;
                                    snprintf(
                                        &raw mut errStr as *mut ::core::ffi::c_char,
                                        JMSG_LENGTH_MAX as size_t,
                                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                        b"tjDecompressToYUVPlanes(): Memory allocation failure\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    retval = -(1 as ::core::ffi::c_int);
                                    current_block = 12563316033059927059;
                                } else {
                                    ptr = _tmpbuf;
                                    i = 0 as ::core::ffi::c_int;
                                    loop {
                                        if !(i < (*dinfo).num_components) {
                                            current_block = 16593409533420678784;
                                            break;
                                        }
                                        tmpbuf[i as usize] = malloc(
                                            (::core::mem::size_of::<JSAMPROW>() as size_t)
                                                .wrapping_mul(th[i as usize] as size_t),
                                        )
                                            as *mut JSAMPROW;
                                        if tmpbuf[i as usize].is_null() {
                                            snprintf(
                                                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                                JMSG_LENGTH_MAX as size_t,
                                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                b"tjDecompressToYUVPlanes(): Memory allocation failure\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                            );
                                            (*this).isInstanceError = TRUE as boolean;
                                            snprintf(
                                                &raw mut errStr as *mut ::core::ffi::c_char,
                                                JMSG_LENGTH_MAX as size_t,
                                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                b"tjDecompressToYUVPlanes(): Memory allocation failure\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                            );
                                            retval = -(1 as ::core::ffi::c_int);
                                            current_block = 12563316033059927059;
                                            break;
                                        } else {
                                            row = 0 as ::core::ffi::c_int;
                                            while row < th[i as usize] {
                                                let ref mut fresh16 =
                                                    *tmpbuf[i as usize].offset(row as isize);
                                                *fresh16 = ptr as JSAMPROW;
                                                ptr = ptr.offset(iw[i as usize] as isize);
                                                row += 1;
                                            }
                                            i += 1;
                                        }
                                    }
                                }
                            } else {
                                current_block = 16593409533420678784;
                            }
                            match current_block {
                                12563316033059927059 => {}
                                _ => {
                                    if _setjmp(
                                        &raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag,
                                    ) != 0
                                    {
                                        retval = -(1 as ::core::ffi::c_int);
                                    } else {
                                        if flags & TJFLAG_FASTUPSAMPLE != 0 {
                                            (*dinfo).do_fancy_upsampling = FALSE as boolean;
                                        }
                                        if flags & TJFLAG_FASTDCT != 0 {
                                            (*dinfo).dct_method = JDCT_IFAST;
                                        }
                                        (*dinfo).raw_data_out = TRUE as boolean;
                                        jpeg_start_decompress(dinfo);
                                        row = 0 as ::core::ffi::c_int;
                                        while row < (*dinfo).output_height as ::core::ffi::c_int {
                                            let mut yuvptr: [JSAMPARRAY; 10] =
                                                [::core::ptr::null_mut::<JSAMPROW>(); 10];
                                            let mut crow: [::core::ffi::c_int; 10] = [0; 10];
                                            i = 0 as ::core::ffi::c_int;
                                            while i < (*dinfo).num_components {
                                                let mut compptr_0: *mut jpeg_component_info =
                                                    (*dinfo).comp_info.offset(i as isize)
                                                        as *mut jpeg_component_info;
                                                if jpegSubsamp == TJSAMP_420 as ::core::ffi::c_int {
                                                    (*compptr_0).DCT_h_scaled_size = dctsize;
                                                    (*compptr_0).MCU_sample_width = tjMCUWidth
                                                        [jpegSubsamp as usize]
                                                        * sf[sfi as usize].num
                                                        / sf[sfi as usize].denom
                                                        * (*compptr_0).v_samp_factor
                                                        / (*dinfo).max_v_samp_factor;
                                                    (*(*dinfo).idct).inverse_DCT[i as usize] =
                                                        (*(*dinfo).idct).inverse_DCT
                                                            [0 as ::core::ffi::c_int as usize];
                                                }
                                                crow[i as usize] = row * (*compptr_0).v_samp_factor
                                                    / (*dinfo).max_v_samp_factor;
                                                if usetmpbuf != 0 {
                                                    yuvptr[i as usize] =
                                                        tmpbuf[i as usize] as JSAMPARRAY;
                                                } else {
                                                    yuvptr[i as usize] = (*(&raw mut outbuf
                                                        as *mut *mut JSAMPROW)
                                                        .offset(i as isize))
                                                    .offset(
                                                        *(&raw mut crow as *mut ::core::ffi::c_int)
                                                            .offset(i as isize)
                                                            as isize,
                                                    )
                                                        as *mut JSAMPROW
                                                        as JSAMPARRAY;
                                                }
                                                i += 1;
                                            }
                                            jpeg_read_raw_data(
                                                dinfo,
                                                &raw mut yuvptr as JSAMPIMAGE,
                                                ((*dinfo).max_v_samp_factor
                                                    * (*dinfo).min_DCT_h_scaled_size)
                                                    as JDIMENSION,
                                            );
                                            if usetmpbuf != 0 {
                                                let mut j: ::core::ffi::c_int = 0;
                                                i = 0 as ::core::ffi::c_int;
                                                while i < (*dinfo).num_components {
                                                    j = 0 as ::core::ffi::c_int;
                                                    while j
                                                        < (if th[i as usize]
                                                            < ph[i as usize] - crow[i as usize]
                                                        {
                                                            th[i as usize]
                                                        } else {
                                                            ph[i as usize] - crow[i as usize]
                                                        })
                                                    {
                                                        memcpy(
                                                            *outbuf[i as usize].offset(
                                                                (crow[i as usize] + j) as isize,
                                                            )
                                                                as *mut ::core::ffi::c_void,
                                                            *tmpbuf[i as usize].offset(j as isize)
                                                                as *const ::core::ffi::c_void,
                                                            pw[i as usize] as size_t,
                                                        );
                                                        j += 1;
                                                    }
                                                    i += 1;
                                                }
                                            }
                                            row += (*dinfo).max_v_samp_factor
                                                * (*dinfo).min_DCT_h_scaled_size;
                                        }
                                        jpeg_finish_decompress(dinfo);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if (*dinfo).global_state > DSTATE_START {
        jpeg_abort_decompress(dinfo);
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAX_COMPONENTS {
        free(tmpbuf[i as usize] as *mut ::core::ffi::c_void);
        free(outbuf[i as usize] as *mut ::core::ffi::c_void);
        i += 1;
    }
    free(_tmpbuf as *mut ::core::ffi::c_void);
    if (*this).jerr.warning != 0 {
        retval = -(1 as ::core::ffi::c_int);
    }
    (*this).jerr.stopOnWarning = FALSE as boolean;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjDecompressToYUV2(
    mut handle: tjhandle,
    mut jpegBuf: *const ::core::ffi::c_uchar,
    mut jpegSize: ::core::ffi::c_ulong,
    mut dstBuf: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut align: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dstPlanes: [*mut ::core::ffi::c_uchar; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_uchar>(); 3];
    let mut pw0: ::core::ffi::c_int = 0;
    let mut ph0: ::core::ffi::c_int = 0;
    let mut strides: [::core::ffi::c_int; 3] = [0; 3];
    let mut retval: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut jpegSubsamp: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut jpegwidth: ::core::ffi::c_int = 0;
    let mut jpegheight: ::core::ffi::c_int = 0;
    let mut scaledw: ::core::ffi::c_int = 0;
    let mut scaledh: ::core::ffi::c_int = 0;
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut dinfo: j_decompress_ptr = ::core::ptr::null_mut::<jpeg_decompress_struct>();
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    dinfo = &raw mut (*this).dinfo as j_decompress_ptr;
    (*this).jerr.warning = FALSE as boolean;
    (*this).isInstanceError = FALSE as boolean;
    (*this).jerr.stopOnWarning = (if flags & TJFLAG_STOPONWARNING != 0 {
        TRUE
    } else {
        FALSE
    }) as boolean;
    if jpegBuf.is_null()
        || jpegSize <= 0 as ::core::ffi::c_ulong
        || dstBuf.is_null()
        || width < 0 as ::core::ffi::c_int
        || align < 1 as ::core::ffi::c_int
        || !(align & align - 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        || height < 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompressToYUV2(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjDecompressToYUV2(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        jpeg_mem_src_tj(dinfo, jpegBuf, jpegSize);
        jpeg_read_header(dinfo, TRUE);
        jpegSubsamp = getSubsamp(dinfo);
        if jpegSubsamp < 0 as ::core::ffi::c_int {
            snprintf(
                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecompressToYUV2(): Could not determine subsampling type for JPEG image\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            (*this).isInstanceError = TRUE as boolean;
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjDecompressToYUV2(): Could not determine subsampling type for JPEG image\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int);
        } else {
            jpegwidth = (*dinfo).image_width as ::core::ffi::c_int;
            jpegheight = (*dinfo).image_height as ::core::ffi::c_int;
            if width == 0 as ::core::ffi::c_int {
                width = jpegwidth;
            }
            if height == 0 as ::core::ffi::c_int {
                height = jpegheight;
            }
            i = 0 as ::core::ffi::c_int;
            while i < NUMSF {
                scaledw = (jpegwidth * sf[i as usize].num + sf[i as usize].denom
                    - 1 as ::core::ffi::c_int)
                    / sf[i as usize].denom;
                scaledh = (jpegheight * sf[i as usize].num + sf[i as usize].denom
                    - 1 as ::core::ffi::c_int)
                    / sf[i as usize].denom;
                if scaledw <= width && scaledh <= height {
                    break;
                }
                i += 1;
            }
            if i >= NUMSF {
                snprintf(
                    &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjDecompressToYUV2(): Could not scale down to desired image dimensions\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                (*this).isInstanceError = TRUE as boolean;
                snprintf(
                    &raw mut errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjDecompressToYUV2(): Could not scale down to desired image dimensions\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                retval = -(1 as ::core::ffi::c_int);
            } else {
                width = scaledw;
                height = scaledh;
                pw0 = tjPlaneWidth(0 as ::core::ffi::c_int, width, jpegSubsamp);
                ph0 = tjPlaneHeight(0 as ::core::ffi::c_int, height, jpegSubsamp);
                dstPlanes[0 as ::core::ffi::c_int as usize] = dstBuf;
                strides[0 as ::core::ffi::c_int as usize] =
                    pw0 + align - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
                if jpegSubsamp == TJSAMP_GRAY as ::core::ffi::c_int {
                    strides[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
                    strides[1 as ::core::ffi::c_int as usize] =
                        strides[2 as ::core::ffi::c_int as usize];
                    dstPlanes[2 as ::core::ffi::c_int as usize] =
                        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    dstPlanes[1 as ::core::ffi::c_int as usize] =
                        dstPlanes[2 as ::core::ffi::c_int as usize];
                } else {
                    let mut pw1: ::core::ffi::c_int =
                        tjPlaneWidth(1 as ::core::ffi::c_int, width, jpegSubsamp);
                    let mut ph1: ::core::ffi::c_int =
                        tjPlaneHeight(1 as ::core::ffi::c_int, height, jpegSubsamp);
                    strides[2 as ::core::ffi::c_int as usize] =
                        pw1 + align - 1 as ::core::ffi::c_int & !(align - 1 as ::core::ffi::c_int);
                    strides[1 as ::core::ffi::c_int as usize] =
                        strides[2 as ::core::ffi::c_int as usize];
                    dstPlanes[1 as ::core::ffi::c_int as usize] = dstPlanes
                        [0 as ::core::ffi::c_int as usize]
                        .offset((strides[0 as ::core::ffi::c_int as usize] * ph0) as isize);
                    dstPlanes[2 as ::core::ffi::c_int as usize] = dstPlanes
                        [1 as ::core::ffi::c_int as usize]
                        .offset((strides[1 as ::core::ffi::c_int as usize] * ph1) as isize);
                }
                (*this).headerRead = 1 as ::core::ffi::c_int;
                return tjDecompressToYUVPlanes(
                    handle,
                    jpegBuf,
                    jpegSize,
                    &raw mut dstPlanes as *mut *mut ::core::ffi::c_uchar,
                    width,
                    &raw mut strides as *mut ::core::ffi::c_int,
                    height,
                    flags,
                );
            }
        }
    }
    (*this).jerr.stopOnWarning = FALSE as boolean;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjDecompressToYUV(
    mut handle: tjhandle,
    mut jpegBuf: *mut ::core::ffi::c_uchar,
    mut jpegSize: ::core::ffi::c_ulong,
    mut dstBuf: *mut ::core::ffi::c_uchar,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return tjDecompressToYUV2(
        handle,
        jpegBuf,
        jpegSize,
        dstBuf,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tjInitTransform() -> tjhandle {
    let mut this: *mut tjinstance = ::core::ptr::null_mut::<tjinstance>();
    let mut handle: tjhandle = NULL;
    this = malloc(::core::mem::size_of::<tjinstance>() as size_t) as *mut tjinstance;
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"tjInitTransform(): Memory allocation failure\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    memset(
        this as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tjinstance>() as size_t,
    );
    snprintf(
        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
        JMSG_LENGTH_MAX as size_t,
        b"No error\0" as *const u8 as *const ::core::ffi::c_char,
    );
    handle = _tjInitCompress(this);
    if handle.is_null() {
        return NULL;
    }
    handle = _tjInitDecompress(this);
    return handle;
}
#[no_mangle]
pub unsafe extern "C" fn tjTransform(
    mut handle: tjhandle,
    mut jpegBuf: *const ::core::ffi::c_uchar,
    mut jpegSize: ::core::ffi::c_ulong,
    mut n: ::core::ffi::c_int,
    mut dstBufs: *mut *mut ::core::ffi::c_uchar,
    mut dstSizes: *mut ::core::ffi::c_ulong,
    mut t: *mut tjtransform,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut xinfo: *mut jpeg_transform_info = ::core::ptr::null_mut::<jpeg_transform_info>();
    let mut srccoefs: *mut jvirt_barray_ptr = ::core::ptr::null_mut::<jvirt_barray_ptr>();
    let mut dstcoefs: *mut jvirt_barray_ptr = ::core::ptr::null_mut::<jvirt_barray_ptr>();
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut jpegSubsamp: ::core::ffi::c_int = 0;
    let mut saveMarkers: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut alloc: boolean = TRUE;
    let mut progress: my_progress_mgr = my_progress_mgr {
        pub_0: jpeg_progress_mgr {
            progress_monitor: None,
            pass_counter: 0,
            pass_limit: 0,
            completed_passes: 0,
            total_passes: 0,
        },
        this: ::core::ptr::null_mut::<tjinstance>(),
    };
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut cinfo: j_compress_ptr = ::core::ptr::null_mut::<jpeg_compress_struct>();
    let mut dinfo: j_decompress_ptr = ::core::ptr::null_mut::<jpeg_decompress_struct>();
    if this.is_null() {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"Invalid handle\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    cinfo = &raw mut (*this).cinfo as j_compress_ptr;
    dinfo = &raw mut (*this).dinfo as j_decompress_ptr;
    (*this).jerr.warning = FALSE as boolean;
    (*this).isInstanceError = FALSE as boolean;
    (*this).jerr.stopOnWarning = (if flags & TJFLAG_STOPONWARNING != 0 {
        TRUE
    } else {
        FALSE
    }) as boolean;
    if (*this).init & COMPRESS as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*this).init & DECOMPRESS as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjTransform(): Instance has not been initialized for transformation\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjTransform(): Instance has not been initialized for transformation\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if jpegBuf.is_null()
        || jpegSize <= 0 as ::core::ffi::c_ulong
        || n < 1 as ::core::ffi::c_int
        || dstBufs.is_null()
        || dstSizes.is_null()
        || t.is_null()
        || flags < 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut (*this).errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjTransform(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*this).isInstanceError = TRUE as boolean;
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjTransform(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        if flags & TJFLAG_FORCEMMX != 0 {
            PUTENV_S(
                b"JSIMD_FORCEMMX\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if flags & TJFLAG_FORCESSE != 0 {
            PUTENV_S(
                b"JSIMD_FORCESSE\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if flags & TJFLAG_FORCESSE2 != 0 {
            PUTENV_S(
                b"JSIMD_FORCESSE2\0" as *const u8 as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if flags & TJFLAG_LIMITSCANS != 0 {
            memset(
                &raw mut progress as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<my_progress_mgr>() as size_t,
            );
            progress.pub_0.progress_monitor =
                Some(my_progress_monitor as unsafe extern "C" fn(j_common_ptr) -> ())
                    as Option<unsafe extern "C" fn(j_common_ptr) -> ()>;
            progress.this = this;
            (*dinfo).progress = &raw mut progress.pub_0;
        } else {
            (*dinfo).progress = ::core::ptr::null_mut::<jpeg_progress_mgr>();
        }
        xinfo = malloc(
            (::core::mem::size_of::<jpeg_transform_info>() as size_t).wrapping_mul(n as size_t),
        ) as *mut jpeg_transform_info;
        if xinfo.is_null() {
            snprintf(
                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjTransform(): Memory allocation failure\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*this).isInstanceError = TRUE as boolean;
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjTransform(): Memory allocation failure\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            retval = -(1 as ::core::ffi::c_int);
        } else {
            memset(
                xinfo as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<jpeg_transform_info>() as size_t).wrapping_mul(n as size_t),
            );
            if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
                retval = -(1 as ::core::ffi::c_int);
            } else {
                jpeg_mem_src_tj(dinfo, jpegBuf, jpegSize);
                i = 0 as ::core::ffi::c_int;
                while i < n {
                    (*xinfo.offset(i as isize)).transform =
                        xformtypes[(*t.offset(i as isize)).op as usize];
                    (*xinfo.offset(i as isize)).perfect =
                        (if (*t.offset(i as isize)).options & TJXOPT_PERFECT != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as boolean;
                    (*xinfo.offset(i as isize)).trim =
                        (if (*t.offset(i as isize)).options & TJXOPT_TRIM != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as boolean;
                    (*xinfo.offset(i as isize)).force_grayscale =
                        (if (*t.offset(i as isize)).options & TJXOPT_GRAY != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as boolean;
                    (*xinfo.offset(i as isize)).crop =
                        (if (*t.offset(i as isize)).options & TJXOPT_CROP != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as boolean;
                    if n != 1 as ::core::ffi::c_int
                        && (*t.offset(i as isize)).op == TJXOP_HFLIP as ::core::ffi::c_int
                    {
                        (*xinfo.offset(i as isize)).slow_hflip = 1 as ::core::ffi::c_int as boolean;
                    } else {
                        (*xinfo.offset(i as isize)).slow_hflip = 0 as ::core::ffi::c_int as boolean;
                    }
                    if (*xinfo.offset(i as isize)).crop != 0 {
                        (*xinfo.offset(i as isize)).crop_xoffset =
                            (*t.offset(i as isize)).r.x as JDIMENSION;
                        (*xinfo.offset(i as isize)).crop_xoffset_set = JCROP_POS;
                        (*xinfo.offset(i as isize)).crop_yoffset =
                            (*t.offset(i as isize)).r.y as JDIMENSION;
                        (*xinfo.offset(i as isize)).crop_yoffset_set = JCROP_POS;
                        if (*t.offset(i as isize)).r.w != 0 as ::core::ffi::c_int {
                            (*xinfo.offset(i as isize)).crop_width =
                                (*t.offset(i as isize)).r.w as JDIMENSION;
                            (*xinfo.offset(i as isize)).crop_width_set = JCROP_POS;
                        } else {
                            (*xinfo.offset(i as isize)).crop_width =
                                JCROP_UNSET as ::core::ffi::c_int as JDIMENSION;
                        }
                        if (*t.offset(i as isize)).r.h != 0 as ::core::ffi::c_int {
                            (*xinfo.offset(i as isize)).crop_height =
                                (*t.offset(i as isize)).r.h as JDIMENSION;
                            (*xinfo.offset(i as isize)).crop_height_set = JCROP_POS;
                        } else {
                            (*xinfo.offset(i as isize)).crop_height =
                                JCROP_UNSET as ::core::ffi::c_int as JDIMENSION;
                        }
                    }
                    if (*t.offset(i as isize)).options & TJXOPT_COPYNONE == 0 {
                        saveMarkers = 1 as ::core::ffi::c_int;
                    }
                    i += 1;
                }
                jcopy_markers_setup(
                    dinfo,
                    (if saveMarkers != 0 {
                        JCOPYOPT_ALL as ::core::ffi::c_int
                    } else {
                        JCOPYOPT_NONE as ::core::ffi::c_int
                    }) as JCOPY_OPTION,
                );
                jpeg_read_header(dinfo, TRUE);
                jpegSubsamp = getSubsamp(dinfo);
                if jpegSubsamp < 0 as ::core::ffi::c_int {
                    snprintf(
                        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjTransform(): Could not determine subsampling type for JPEG image\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    (*this).isInstanceError = TRUE as boolean;
                    snprintf(
                        &raw mut errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjTransform(): Could not determine subsampling type for JPEG image\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    retval = -(1 as ::core::ffi::c_int);
                } else {
                    i = 0 as ::core::ffi::c_int;
                    loop {
                        if !(i < n) {
                            current_block = 5854763015135596753;
                            break;
                        }
                        if jtransform_request_workspace(
                            dinfo,
                            xinfo.offset(i as isize) as *mut jpeg_transform_info,
                        ) == 0
                        {
                            snprintf(
                                &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                JMSG_LENGTH_MAX as size_t,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                b"tjTransform(): Transform is not perfect\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            (*this).isInstanceError = TRUE as boolean;
                            snprintf(
                                &raw mut errStr as *mut ::core::ffi::c_char,
                                JMSG_LENGTH_MAX as size_t,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                b"tjTransform(): Transform is not perfect\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            retval = -(1 as ::core::ffi::c_int);
                            current_block = 17324398410423727300;
                            break;
                        } else {
                            if (*xinfo.offset(i as isize)).crop != 0 {
                                if (*t.offset(i as isize)).r.x % tjMCUWidth[jpegSubsamp as usize]
                                    != 0 as ::core::ffi::c_int
                                    || (*t.offset(i as isize)).r.y
                                        % tjMCUHeight[jpegSubsamp as usize]
                                        != 0 as ::core::ffi::c_int
                                {
                                    snprintf(
                                        &raw mut (*this).errStr as *mut ::core::ffi::c_char,
                                        JMSG_LENGTH_MAX as size_t,
                                        b"To crop this JPEG image, x must be a multiple of %d\nand y must be a multiple of %d.\n\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        tjMCUWidth[jpegSubsamp as usize],
                                        tjMCUHeight[jpegSubsamp as usize],
                                    );
                                    (*this).isInstanceError = TRUE as boolean;
                                    retval = -(1 as ::core::ffi::c_int);
                                    current_block = 17324398410423727300;
                                    break;
                                }
                            }
                            i += 1;
                        }
                    }
                    match current_block {
                        17324398410423727300 => {}
                        _ => {
                            srccoefs = jpeg_read_coefficients(dinfo);
                            i = 0 as ::core::ffi::c_int;
                            's_393: loop {
                                if !(i < n) {
                                    current_block = 13823707972521062695;
                                    break;
                                }
                                let mut w: ::core::ffi::c_int = 0;
                                let mut h: ::core::ffi::c_int = 0;
                                if (*xinfo.offset(i as isize)).crop == 0 {
                                    w = (*dinfo).image_width as ::core::ffi::c_int;
                                    h = (*dinfo).image_height as ::core::ffi::c_int;
                                } else {
                                    w = (*xinfo.offset(i as isize)).crop_width
                                        as ::core::ffi::c_int;
                                    h = (*xinfo.offset(i as isize)).crop_height
                                        as ::core::ffi::c_int;
                                }
                                if flags & TJFLAG_NOREALLOC != 0 {
                                    alloc = FALSE as boolean;
                                    *dstSizes.offset(i as isize) = tjBufSize(w, h, jpegSubsamp);
                                }
                                if (*t.offset(i as isize)).options & TJXOPT_NOOUTPUT == 0 {
                                    jpeg_mem_dest_tj(
                                        cinfo,
                                        dstBufs.offset(i as isize)
                                            as *mut *mut ::core::ffi::c_uchar,
                                        dstSizes.offset(i as isize) as *mut ::core::ffi::c_ulong,
                                        alloc,
                                    );
                                }
                                jpeg_copy_critical_parameters(dinfo, cinfo);
                                dstcoefs = jtransform_adjust_parameters(
                                    dinfo,
                                    cinfo,
                                    srccoefs,
                                    xinfo.offset(i as isize) as *mut jpeg_transform_info,
                                );
                                if flags & TJFLAG_PROGRESSIVE != 0
                                    || (*t.offset(i as isize)).options & TJXOPT_PROGRESSIVE != 0
                                {
                                    jpeg_simple_progression(cinfo);
                                }
                                if (*t.offset(i as isize)).options & TJXOPT_NOOUTPUT == 0 {
                                    jpeg_write_coefficients(cinfo, dstcoefs);
                                    jcopy_markers_execute(
                                        dinfo,
                                        cinfo,
                                        (if (*t.offset(i as isize)).options & TJXOPT_COPYNONE != 0 {
                                            JCOPYOPT_NONE as ::core::ffi::c_int
                                        } else {
                                            JCOPYOPT_ALL as ::core::ffi::c_int
                                        }) as JCOPY_OPTION,
                                    );
                                } else {
                                    jinit_c_master_control(cinfo, TRUE);
                                }
                                jtransform_execute_transform(
                                    dinfo,
                                    cinfo,
                                    srccoefs,
                                    xinfo.offset(i as isize) as *mut jpeg_transform_info,
                                );
                                if (*t.offset(i as isize)).customFilter.is_some() {
                                    let mut ci: ::core::ffi::c_int = 0;
                                    let mut y: ::core::ffi::c_int = 0;
                                    let mut by: JDIMENSION = 0;
                                    ci = 0 as ::core::ffi::c_int;
                                    while ci < (*cinfo).num_components {
                                        let mut compptr: *mut jpeg_component_info =
                                            (*cinfo).comp_info.offset(ci as isize)
                                                as *mut jpeg_component_info;
                                        let mut arrayRegion: tjregion = tjregion {
                                            x: 0 as ::core::ffi::c_int,
                                            y: 0 as ::core::ffi::c_int,
                                            w: 0 as ::core::ffi::c_int,
                                            h: 0 as ::core::ffi::c_int,
                                        };
                                        let mut planeRegion: tjregion = tjregion {
                                            x: 0 as ::core::ffi::c_int,
                                            y: 0 as ::core::ffi::c_int,
                                            w: 0 as ::core::ffi::c_int,
                                            h: 0 as ::core::ffi::c_int,
                                        };
                                        arrayRegion.w = (*compptr)
                                            .width_in_blocks
                                            .wrapping_mul(DCTSIZE as JDIMENSION)
                                            as ::core::ffi::c_int;
                                        arrayRegion.h = DCTSIZE;
                                        planeRegion.w = (*compptr)
                                            .width_in_blocks
                                            .wrapping_mul(DCTSIZE as JDIMENSION)
                                            as ::core::ffi::c_int;
                                        planeRegion.h = (*compptr)
                                            .height_in_blocks
                                            .wrapping_mul(DCTSIZE as JDIMENSION)
                                            as ::core::ffi::c_int;
                                        by = 0 as JDIMENSION;
                                        while by < (*compptr).height_in_blocks {
                                            let mut barray: JBLOCKARRAY = (*(*dinfo).mem)
                                                .access_virt_barray
                                                .expect("non-null function pointer")(
                                                dinfo as j_common_ptr,
                                                *dstcoefs.offset(ci as isize),
                                                by,
                                                (*compptr).v_samp_factor as JDIMENSION,
                                                TRUE,
                                            );
                                            y = 0 as ::core::ffi::c_int;
                                            while y < (*compptr).v_samp_factor {
                                                if (*t.offset(i as isize))
                                                    .customFilter
                                                    .expect("non-null function pointer")(
                                                    &raw mut *(*barray.offset(y as isize))
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as *mut ::core::ffi::c_short,
                                                    arrayRegion,
                                                    planeRegion,
                                                    ci,
                                                    i,
                                                    t.offset(i as isize) as *mut tjtransform,
                                                ) == -(1 as ::core::ffi::c_int)
                                                {
                                                    snprintf(
                                                        &raw mut (*this).errStr
                                                            as *mut ::core::ffi::c_char,
                                                        JMSG_LENGTH_MAX as size_t,
                                                        b"%s\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        b"tjTransform(): Error in custom filter\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    (*this).isInstanceError = TRUE as boolean;
                                                    snprintf(
                                                        &raw mut errStr as *mut ::core::ffi::c_char,
                                                        JMSG_LENGTH_MAX as size_t,
                                                        b"%s\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        b"tjTransform(): Error in custom filter\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    retval = -(1 as ::core::ffi::c_int);
                                                    current_block = 17324398410423727300;
                                                    break 's_393;
                                                } else {
                                                    arrayRegion.y += DCTSIZE;
                                                    y += 1;
                                                }
                                            }
                                            by = by.wrapping_add(
                                                (*compptr).v_samp_factor as JDIMENSION,
                                            );
                                        }
                                        ci += 1;
                                    }
                                }
                                if (*t.offset(i as isize)).options & TJXOPT_NOOUTPUT == 0 {
                                    jpeg_finish_compress(cinfo);
                                }
                                i += 1;
                            }
                            match current_block {
                                17324398410423727300 => {}
                                _ => {
                                    jpeg_finish_decompress(dinfo);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if (*cinfo).global_state > CSTATE_START {
        if alloc != 0 {
            Some(
                (*(*cinfo).dest)
                    .term_destination
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
        jpeg_abort_compress(cinfo);
    }
    if (*dinfo).global_state > DSTATE_START {
        jpeg_abort_decompress(dinfo);
    }
    free(xinfo as *mut ::core::ffi::c_void);
    if (*this).jerr.warning != 0 {
        retval = -(1 as ::core::ffi::c_int);
    }
    (*this).jerr.stopOnWarning = FALSE as boolean;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn tjLoadImage(
    mut filename: *const ::core::ffi::c_char,
    mut width: *mut ::core::ffi::c_int,
    mut align: ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut pixelFormat: *mut ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    let mut current_block: u64;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tempc: ::core::ffi::c_int = 0;
    let mut pitch: size_t = 0;
    let mut handle: tjhandle = NULL;
    let mut this: *mut tjinstance = ::core::ptr::null_mut::<tjinstance>();
    let mut cinfo: j_compress_ptr = ::core::ptr::null_mut::<jpeg_compress_struct>();
    let mut src: cjpeg_source_ptr = ::core::ptr::null_mut::<cjpeg_source_struct>();
    let mut dstBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut invert: boolean = 0;
    if filename.is_null()
        || width.is_null()
        || align < 1 as ::core::ffi::c_int
        || height.is_null()
        || pixelFormat.is_null()
        || *pixelFormat < TJPF_UNKNOWN as ::core::ffi::c_int
        || *pixelFormat >= TJ_NUMPF
    {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjLoadImage(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if align & align - 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjLoadImage(): Alignment must be a power of 2\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        handle = tjInitCompress();
        if handle.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        this = handle as *mut tjinstance;
        cinfo = &raw mut (*this).cinfo as j_compress_ptr;
        file = fopen(filename, b"rb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
        if file.is_null() {
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjLoadImage(): Cannot open input file\0" as *const u8
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            retval = -(1 as ::core::ffi::c_int);
        } else {
            tempc = getc(file);
            if tempc < 0 as ::core::ffi::c_int || ungetc(tempc, file) == EOF {
                snprintf(
                    &raw mut errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjLoadImage(): Could not read input file\0" as *const u8
                        as *const ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
                retval = -(1 as ::core::ffi::c_int);
            } else if tempc == EOF {
                snprintf(
                    &raw mut errStr as *mut ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tjLoadImage(): Input file contains no data\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                retval = -(1 as ::core::ffi::c_int);
            } else if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
                retval = -(1 as ::core::ffi::c_int);
            } else {
                if *pixelFormat == TJPF_UNKNOWN as ::core::ffi::c_int {
                    (*cinfo).in_color_space = JCS_UNKNOWN;
                } else {
                    (*cinfo).in_color_space = pf2cs[*pixelFormat as usize];
                }
                if tempc == 'B' as i32 {
                    src = jinit_read_bmp(cinfo, FALSE);
                    if src.is_null() {
                        snprintf(
                            &raw mut errStr as *mut ::core::ffi::c_char,
                            JMSG_LENGTH_MAX as size_t,
                            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                            b"tjLoadImage(): Could not initialize bitmap loader\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        retval = -(1 as ::core::ffi::c_int);
                        current_block = 8000481920919002204;
                    } else {
                        invert = (flags & TJFLAG_BOTTOMUP == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as boolean;
                        current_block = 13131896068329595644;
                    }
                } else if tempc == 'P' as i32 {
                    src = jinit_read_ppm(cinfo);
                    if src.is_null() {
                        snprintf(
                            &raw mut errStr as *mut ::core::ffi::c_char,
                            JMSG_LENGTH_MAX as size_t,
                            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                            b"tjLoadImage(): Could not initialize PPM loader\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        retval = -(1 as ::core::ffi::c_int);
                        current_block = 8000481920919002204;
                    } else {
                        invert = (flags & TJFLAG_BOTTOMUP != 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as boolean;
                        current_block = 13131896068329595644;
                    }
                } else {
                    snprintf(
                        &raw mut errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjLoadImage(): Unsupported file type\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    retval = -(1 as ::core::ffi::c_int);
                    current_block = 8000481920919002204;
                }
                match current_block {
                    8000481920919002204 => {}
                    _ => {
                        (*src).input_file = file;
                        Some((*src).start_input.expect("non-null function pointer"))
                            .expect("non-null function pointer")(cinfo, src);
                        Some(
                            (*(*cinfo).mem)
                                .realize_virt_arrays
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr
                        );
                        *width = (*cinfo).image_width as ::core::ffi::c_int;
                        *height = (*cinfo).image_height as ::core::ffi::c_int;
                        *pixelFormat = cs2pf[(*cinfo).in_color_space as usize];
                        pitch = (*width * tjPixelSize[*pixelFormat as usize] + align
                            - 1 as ::core::ffi::c_int
                            & !(align - 1 as ::core::ffi::c_int))
                            as size_t;
                        if (pitch as ::core::ffi::c_ulonglong)
                            .wrapping_mul(*height as ::core::ffi::c_ulonglong)
                            > -(1 as ::core::ffi::c_int) as size_t as ::core::ffi::c_ulonglong
                            || {
                                dstBuf = malloc(pitch.wrapping_mul(*height as size_t))
                                    as *mut ::core::ffi::c_uchar;
                                dstBuf.is_null()
                            }
                        {
                            snprintf(
                                &raw mut errStr as *mut ::core::ffi::c_char,
                                JMSG_LENGTH_MAX as size_t,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                b"tjLoadImage(): Memory allocation failure\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            retval = -(1 as ::core::ffi::c_int);
                        } else if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag)
                            != 0
                        {
                            retval = -(1 as ::core::ffi::c_int);
                        } else {
                            while (*cinfo).next_scanline < (*cinfo).image_height {
                                let mut i: ::core::ffi::c_int = 0;
                                let mut nlines: ::core::ffi::c_int =
                                    Some((*src).get_pixel_rows.expect("non-null function pointer"))
                                        .expect("non-null function pointer")(
                                        cinfo, src
                                    ) as ::core::ffi::c_int;
                                i = 0 as ::core::ffi::c_int;
                                while i < nlines {
                                    let mut dstptr: *mut ::core::ffi::c_uchar =
                                        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    let mut row: ::core::ffi::c_int = 0;
                                    row = (*cinfo).next_scanline.wrapping_add(i as JDIMENSION)
                                        as ::core::ffi::c_int;
                                    if invert != 0 {
                                        dstptr = dstBuf.offset(
                                            ((*height - row - 1 as ::core::ffi::c_int) as size_t)
                                                .wrapping_mul(pitch)
                                                as isize,
                                        )
                                            as *mut ::core::ffi::c_uchar;
                                    } else {
                                        dstptr = dstBuf
                                            .offset((row as size_t).wrapping_mul(pitch) as isize)
                                            as *mut ::core::ffi::c_uchar;
                                    }
                                    memcpy(
                                        dstptr as *mut ::core::ffi::c_void,
                                        *(*src).buffer.offset(i as isize)
                                            as *const ::core::ffi::c_void,
                                        (*width * tjPixelSize[*pixelFormat as usize]) as size_t,
                                    );
                                    i += 1;
                                }
                                (*cinfo).next_scanline =
                                    (*cinfo).next_scanline.wrapping_add(nlines as JDIMENSION);
                            }
                            Some((*src).finish_input.expect("non-null function pointer"))
                                .expect("non-null function pointer")(
                                cinfo, src
                            );
                        }
                    }
                }
            }
        }
    }
    if !handle.is_null() {
        tjDestroy(handle);
    }
    if !file.is_null() {
        fclose(file);
    }
    if retval < 0 as ::core::ffi::c_int {
        free(dstBuf as *mut ::core::ffi::c_void);
        dstBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    return dstBuf;
}
#[no_mangle]
pub unsafe extern "C" fn tjSaveImage(
    mut filename: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut pitch: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut pixelFormat: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut handle: tjhandle = NULL;
    let mut this: *mut tjinstance = ::core::ptr::null_mut::<tjinstance>();
    let mut dinfo: j_decompress_ptr = ::core::ptr::null_mut::<jpeg_decompress_struct>();
    let mut dst: djpeg_dest_ptr = ::core::ptr::null_mut::<djpeg_dest_struct>();
    let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut invert: boolean = 0;
    if filename.is_null()
        || buffer.is_null()
        || width < 1 as ::core::ffi::c_int
        || pitch < 0 as ::core::ffi::c_int
        || height < 1 as ::core::ffi::c_int
        || pixelFormat < 0 as ::core::ffi::c_int
        || pixelFormat >= TJ_NUMPF
    {
        snprintf(
            &raw mut errStr as *mut ::core::ffi::c_char,
            JMSG_LENGTH_MAX as size_t,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"tjSaveImage(): Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        handle = tjInitDecompress();
        if handle.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        this = handle as *mut tjinstance;
        dinfo = &raw mut (*this).dinfo as j_decompress_ptr;
        file = fopen(filename, b"wb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
        if file.is_null() {
            snprintf(
                &raw mut errStr as *mut ::core::ffi::c_char,
                JMSG_LENGTH_MAX as size_t,
                b"%s\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"tjSaveImage(): Cannot open output file\0" as *const u8
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            retval = -(1 as ::core::ffi::c_int);
        } else if _setjmp(&raw mut (*this).jerr.setjmp_buffer as *mut __jmp_buf_tag) != 0 {
            retval = -(1 as ::core::ffi::c_int);
        } else {
            (*this).dinfo.out_color_space = pf2cs[pixelFormat as usize];
            (*dinfo).image_width = width as JDIMENSION;
            (*dinfo).image_height = height as JDIMENSION;
            (*dinfo).global_state = DSTATE_READY;
            (*dinfo).scale_denom = 1 as ::core::ffi::c_uint;
            (*dinfo).scale_num = (*dinfo).scale_denom;
            ptr = strrchr(filename, '.' as i32);
            if !ptr.is_null()
                && strcasecmp(ptr, b".bmp\0" as *const u8 as *const ::core::ffi::c_char) == 0
            {
                dst = jinit_write_bmp(dinfo, FALSE, FALSE);
                if dst.is_null() {
                    snprintf(
                        &raw mut errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjSaveImage(): Could not initialize bitmap writer\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    retval = -(1 as ::core::ffi::c_int);
                    current_block = 6307062595671882425;
                } else {
                    invert = (flags & TJFLAG_BOTTOMUP == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as boolean;
                    current_block = 14763689060501151050;
                }
            } else {
                dst = jinit_write_ppm(dinfo);
                if dst.is_null() {
                    snprintf(
                        &raw mut errStr as *mut ::core::ffi::c_char,
                        JMSG_LENGTH_MAX as size_t,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"tjSaveImage(): Could not initialize PPM writer\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    retval = -(1 as ::core::ffi::c_int);
                    current_block = 6307062595671882425;
                } else {
                    invert = (flags & TJFLAG_BOTTOMUP != 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as boolean;
                    current_block = 14763689060501151050;
                }
            }
            match current_block {
                6307062595671882425 => {}
                _ => {
                    (*dst).output_file = file;
                    Some((*dst).start_output.expect("non-null function pointer"))
                        .expect("non-null function pointer")(dinfo, dst);
                    Some(
                        (*(*dinfo).mem)
                            .realize_virt_arrays
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(dinfo as j_common_ptr);
                    if pitch == 0 as ::core::ffi::c_int {
                        pitch = width * tjPixelSize[pixelFormat as usize];
                    }
                    while (*dinfo).output_scanline < (*dinfo).output_height {
                        let mut rowptr: *mut ::core::ffi::c_uchar =
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        if invert != 0 {
                            rowptr = buffer.offset(
                                (height as JDIMENSION)
                                    .wrapping_sub((*dinfo).output_scanline)
                                    .wrapping_sub(1 as JDIMENSION)
                                    .wrapping_mul(pitch as JDIMENSION)
                                    as isize,
                            ) as *mut ::core::ffi::c_uchar;
                        } else {
                            rowptr = buffer
                                .offset((*dinfo).output_scanline.wrapping_mul(pitch as JDIMENSION)
                                    as isize)
                                as *mut ::core::ffi::c_uchar;
                        }
                        memcpy(
                            *(*dst).buffer.offset(0 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_void,
                            rowptr as *const ::core::ffi::c_void,
                            (width * tjPixelSize[pixelFormat as usize]) as size_t,
                        );
                        Some((*dst).put_pixel_rows.expect("non-null function pointer"))
                            .expect("non-null function pointer")(
                            dinfo, dst, 1 as JDIMENSION
                        );
                        (*dinfo).output_scanline = (*dinfo).output_scanline.wrapping_add(1);
                    }
                    Some((*dst).finish_output.expect("non-null function pointer"))
                        .expect("non-null function pointer")(dinfo, dst);
                }
            }
        }
    }
    if !handle.is_null() {
        tjDestroy(handle);
    }
    if !file.is_null() {
        fclose(file);
    }
    return retval;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
