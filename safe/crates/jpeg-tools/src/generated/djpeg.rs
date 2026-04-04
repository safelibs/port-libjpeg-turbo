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
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
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
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn jpeg_std_error(err: *mut jpeg_error_mgr) -> *mut jpeg_error_mgr;
    fn jpeg_CreateDecompress(
        cinfo: j_decompress_ptr,
        version: ::core::ffi::c_int,
        structsize: size_t,
    );
    fn jpeg_destroy_decompress(cinfo: j_decompress_ptr);
    fn jpeg_stdio_src(cinfo: j_decompress_ptr, infile: *mut FILE);
    fn jpeg_mem_src(
        cinfo: j_decompress_ptr,
        inbuffer: *const ::core::ffi::c_uchar,
        insize: ::core::ffi::c_ulong,
    );
    fn jpeg_read_header(cinfo: j_decompress_ptr, require_image: boolean) -> ::core::ffi::c_int;
    fn jpeg_start_decompress(cinfo: j_decompress_ptr) -> boolean;
    fn jpeg_read_scanlines(
        cinfo: j_decompress_ptr,
        scanlines: JSAMPARRAY,
        max_lines: JDIMENSION,
    ) -> JDIMENSION;
    fn jpeg_skip_scanlines(cinfo: j_decompress_ptr, num_lines: JDIMENSION) -> JDIMENSION;
    fn jpeg_crop_scanline(
        cinfo: j_decompress_ptr,
        xoffset: *mut JDIMENSION,
        width: *mut JDIMENSION,
    );
    fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean;
    fn jpeg_save_markers(
        cinfo: j_decompress_ptr,
        marker_code: ::core::ffi::c_int,
        length_limit: ::core::ffi::c_uint,
    );
    fn jpeg_set_marker_processor(
        cinfo: j_decompress_ptr,
        marker_code: ::core::ffi::c_int,
        routine: jpeg_marker_parser_method,
    );
    fn jpeg_read_icc_profile(
        cinfo: j_decompress_ptr,
        icc_data_ptr: *mut *mut JOCTET,
        icc_data_len: *mut ::core::ffi::c_uint,
    ) -> boolean;
    fn jinit_write_bmp(
        cinfo: j_decompress_ptr,
        is_os2: boolean,
        use_inversion_array: boolean,
    ) -> djpeg_dest_ptr;
    fn jinit_write_gif(cinfo: j_decompress_ptr, is_lzw: boolean) -> djpeg_dest_ptr;
    fn jinit_write_ppm(cinfo: j_decompress_ptr) -> djpeg_dest_ptr;
    fn jinit_write_targa(cinfo: j_decompress_ptr) -> djpeg_dest_ptr;
    fn read_color_map(cinfo: j_decompress_ptr, infile: *mut FILE);
    fn start_progress_monitor(cinfo: j_common_ptr, progress: cd_progress_ptr);
    fn end_progress_monitor(cinfo: j_common_ptr);
    fn keymatch(
        arg: *mut ::core::ffi::c_char,
        keyword: *const ::core::ffi::c_char,
        minchars: ::core::ffi::c_int,
    ) -> boolean;
    fn read_stdin() -> *mut FILE;
    fn write_stdout() -> *mut FILE;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
pub type jpeg_marker_parser_method = Option<unsafe extern "C" fn(j_decompress_ptr) -> boolean>;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const JMSG_LASTADDONCODE: C2RustUnnamed_1 = 1047;
pub const JERR_UNSUPPORTED_FORMAT: C2RustUnnamed_1 = 1046;
pub const JERR_UNKNOWN_FORMAT: C2RustUnnamed_1 = 1045;
pub const JERR_UNGETC_FAILED: C2RustUnnamed_1 = 1044;
pub const JERR_TOO_MANY_COLORS: C2RustUnnamed_1 = 1043;
pub const JERR_BAD_CMAP_FILE: C2RustUnnamed_1 = 1042;
pub const JTRC_TGA_MAPPED: C2RustUnnamed_1 = 1041;
pub const JTRC_TGA_GRAY: C2RustUnnamed_1 = 1040;
pub const JTRC_TGA: C2RustUnnamed_1 = 1039;
pub const JERR_TGA_COLORSPACE: C2RustUnnamed_1 = 1038;
pub const JERR_TGA_BADPARMS: C2RustUnnamed_1 = 1037;
pub const JERR_TGA_BADCMAP: C2RustUnnamed_1 = 1036;
pub const JTRC_PPM_TEXT: C2RustUnnamed_1 = 1035;
pub const JTRC_PPM: C2RustUnnamed_1 = 1034;
pub const JTRC_PGM_TEXT: C2RustUnnamed_1 = 1033;
pub const JTRC_PGM: C2RustUnnamed_1 = 1032;
pub const JERR_PPM_OUTOFRANGE: C2RustUnnamed_1 = 1031;
pub const JERR_PPM_NOT: C2RustUnnamed_1 = 1030;
pub const JERR_PPM_NONNUMERIC: C2RustUnnamed_1 = 1029;
pub const JERR_PPM_COLORSPACE: C2RustUnnamed_1 = 1028;
pub const JWRN_GIF_NOMOREDATA: C2RustUnnamed_1 = 1027;
pub const JWRN_GIF_ENDCODE: C2RustUnnamed_1 = 1026;
pub const JWRN_GIF_CHAR: C2RustUnnamed_1 = 1025;
pub const JWRN_GIF_BADDATA: C2RustUnnamed_1 = 1024;
pub const JTRC_GIF_NONSQUARE: C2RustUnnamed_1 = 1023;
pub const JTRC_GIF_EXTENSION: C2RustUnnamed_1 = 1022;
pub const JTRC_GIF_BADVERSION: C2RustUnnamed_1 = 1021;
pub const JTRC_GIF: C2RustUnnamed_1 = 1020;
pub const JERR_GIF_NOT: C2RustUnnamed_1 = 1019;
pub const JERR_GIF_IMAGENOTFOUND: C2RustUnnamed_1 = 1018;
pub const JERR_GIF_EMPTY: C2RustUnnamed_1 = 1017;
pub const JERR_GIF_COLORSPACE: C2RustUnnamed_1 = 1016;
pub const JERR_GIF_CODESIZE: C2RustUnnamed_1 = 1015;
pub const JERR_GIF_BUG: C2RustUnnamed_1 = 1014;
pub const JTRC_BMP_OS2_MAPPED: C2RustUnnamed_1 = 1013;
pub const JTRC_BMP_OS2: C2RustUnnamed_1 = 1012;
pub const JTRC_BMP_MAPPED: C2RustUnnamed_1 = 1011;
pub const JTRC_BMP: C2RustUnnamed_1 = 1010;
pub const JERR_BMP_OUTOFRANGE: C2RustUnnamed_1 = 1009;
pub const JERR_BMP_NOT: C2RustUnnamed_1 = 1008;
pub const JERR_BMP_EMPTY: C2RustUnnamed_1 = 1007;
pub const JERR_BMP_COMPRESSED: C2RustUnnamed_1 = 1006;
pub const JERR_BMP_COLORSPACE: C2RustUnnamed_1 = 1005;
pub const JERR_BMP_BADPLANES: C2RustUnnamed_1 = 1004;
pub const JERR_BMP_BADHEADER: C2RustUnnamed_1 = 1003;
pub const JERR_BMP_BADDEPTH: C2RustUnnamed_1 = 1002;
pub const JERR_BMP_BADCMAP: C2RustUnnamed_1 = 1001;
pub const JMSG_FIRSTADDONCODE: C2RustUnnamed_1 = 1000;
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
pub type djpeg_dest_ptr = *mut djpeg_dest_struct;
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
pub type IMAGE_FORMATS = ::core::ffi::c_uint;
pub const FMT_TIFF: IMAGE_FORMATS = 6;
pub const FMT_TARGA: IMAGE_FORMATS = 5;
pub const FMT_PPM: IMAGE_FORMATS = 4;
pub const FMT_OS2: IMAGE_FORMATS = 3;
pub const FMT_GIF0: IMAGE_FORMATS = 2;
pub const FMT_GIF: IMAGE_FORMATS = 1;
pub const FMT_BMP: IMAGE_FORMATS = 0;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPEG_APP0: ::core::ffi::c_int = 0xe0 as ::core::ffi::c_int;
pub const JPEG_COM: ::core::ffi::c_int = 0xfe as ::core::ffi::c_int;
pub const READ_BINARY: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const WRITE_BINARY: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"wb\0") };
pub const EXIT_WARNING: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BUILD: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"20260403\0") };
pub const PACKAGE_NAME: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"libjpeg-turbo\0") };
pub const VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"2.1.5\0") };
static mut cdjpeg_message_table: [*const ::core::ffi::c_char; 48] = [
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
    b"GIF output got confused\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bogus GIF codesize %d\0" as *const u8 as *const ::core::ffi::c_char,
    b"GIF output must be grayscale or RGB\0" as *const u8 as *const ::core::ffi::c_char,
    b"Empty GIF image\0" as *const u8 as *const ::core::ffi::c_char,
    b"Too few images in GIF file\0" as *const u8 as *const ::core::ffi::c_char,
    b"Not a GIF file\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%ux%d GIF image\0" as *const u8 as *const ::core::ffi::c_char,
    b"Warning: unexpected GIF version number '%c%c%c'\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ignoring GIF extension block of type 0x%02x\0" as *const u8 as *const ::core::ffi::c_char,
    b"Caution: nonsquare pixels in input\0" as *const u8 as *const ::core::ffi::c_char,
    b"Corrupt data in GIF file\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bogus char 0x%02x in GIF file, ignoring\0" as *const u8 as *const ::core::ffi::c_char,
    b"Premature end of GIF image\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ran out of GIF bits\0" as *const u8 as *const ::core::ffi::c_char,
    b"PPM output must be grayscale or RGB\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nonnumeric data in PPM file\0" as *const u8 as *const ::core::ffi::c_char,
    b"Not a PPM/PGM file\0" as *const u8 as *const ::core::ffi::c_char,
    b"Numeric value out of range in PPM file\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u PGM image\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u text PGM image\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u PPM image\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u text PPM image\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unsupported Targa colormap format\0" as *const u8 as *const ::core::ffi::c_char,
    b"Invalid or unsupported Targa file\0" as *const u8 as *const ::core::ffi::c_char,
    b"Targa output must be grayscale or RGB\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u RGB Targa image\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u grayscale Targa image\0" as *const u8 as *const ::core::ffi::c_char,
    b"%ux%u colormapped Targa image\0" as *const u8 as *const ::core::ffi::c_char,
    b"Color map file is invalid or of unsupported format\0" as *const u8
        as *const ::core::ffi::c_char,
    b"Output file format cannot handle %d colormap entries\0" as *const u8
        as *const ::core::ffi::c_char,
    b"ungetc failed\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unrecognized input file format --- perhaps you need -targa\0" as *const u8
        as *const ::core::ffi::c_char,
    b"Unsupported output file format\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
pub const JPEG_LIB_VERSION: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
static mut requested_fmt: IMAGE_FORMATS = FMT_BMP;
static mut progname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut icc_filename: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut max_scans: JDIMENSION = 0;
static mut outfilename: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut memsrc: boolean = 0;
#[no_mangle]
pub static mut report: boolean = 0;
#[no_mangle]
pub static mut skip: boolean = 0;
#[no_mangle]
pub static mut crop: boolean = 0;
#[no_mangle]
pub static mut skip_end: JDIMENSION = 0;
#[no_mangle]
pub static mut skip_start: JDIMENSION = 0;
#[no_mangle]
pub static mut crop_x: JDIMENSION = 0;
#[no_mangle]
pub static mut crop_y: JDIMENSION = 0;
#[no_mangle]
pub static mut crop_width: JDIMENSION = 0;
#[no_mangle]
pub static mut crop_height: JDIMENSION = 0;
#[no_mangle]
pub static mut strict: boolean = 0;
pub const INPUT_BUF_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
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
        b"  -colors N      Reduce image to no more than N colors\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -fast          Fast, low-quality processing\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -grayscale     Force grayscale output\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -rgb           Force RGB output\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -rgb565        Force RGB565 output\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -scale M/N     Scale output image by fraction M/N, eg, 1/8\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -bmp           Select BMP output format (Windows style)%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if FMT_PPM as ::core::ffi::c_int == FMT_BMP as ::core::ffi::c_int {
            b" (default)\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b"  -gif           Select GIF output format (LZW-compressed)%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if FMT_PPM as ::core::ffi::c_int == FMT_GIF as ::core::ffi::c_int {
            b" (default)\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b"  -gif0          Select GIF output format (uncompressed)%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if FMT_PPM as ::core::ffi::c_int == FMT_GIF0 as ::core::ffi::c_int {
            b" (default)\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b"  -os2           Select BMP output format (OS/2 style)%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if FMT_PPM as ::core::ffi::c_int == FMT_OS2 as ::core::ffi::c_int {
            b" (default)\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b"  -pnm           Select PBMPLUS (PPM/PGM) output format%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if FMT_PPM as ::core::ffi::c_int == FMT_PPM as ::core::ffi::c_int {
            b" (default)\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b"  -targa         Select Targa output format%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if FMT_PPM as ::core::ffi::c_int == FMT_TARGA as ::core::ffi::c_int {
            b" (default)\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b"Switches for advanced users:\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -dct int       Use accurate integer DCT method%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if JDCT_ISLOW as ::core::ffi::c_int == JDCT_ISLOW as ::core::ffi::c_int {
            b" (default)\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b"  -dct fast      Use less accurate integer DCT method [legacy feature]%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if JDCT_ISLOW as ::core::ffi::c_int == JDCT_IFAST as ::core::ffi::c_int {
            b" (default)\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b"  -dct float     Use floating-point DCT method [legacy feature]%s\n\0" as *const u8
            as *const ::core::ffi::c_char,
        if JDCT_ISLOW as ::core::ffi::c_int == JDCT_FLOAT as ::core::ffi::c_int {
            b" (default)\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b"  -dither fs     Use F-S dithering (default)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -dither none   Don't use dithering in quantization\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -dither ordered  Use ordered dither (medium speed, quality)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -icc FILE      Extract ICC profile to FILE\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -map FILE      Map to colors used in named image file\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -nosmooth      Don't use high-quality upsampling\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -onepass       Use 1-pass quantization (fast, low quality)\n\0" as *const u8
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
        b"  -memsrc        Load input file into memory before decompressing\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -report        Report decompression progress\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -skip Y0,Y1    Decompress all rows except those between Y0 and Y1 (inclusive)\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -crop WxH+X+Y  Decompress only a rectangular subregion of the image\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"                 [requires PBMPLUS (PPM/PGM), GIF, or Targa output format]\n\0"
            as *const u8 as *const ::core::ffi::c_char,
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
    exit(EXIT_FAILURE);
}
unsafe extern "C" fn parse_switches(
    mut cinfo: j_decompress_ptr,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut last_file_arg_seen: ::core::ffi::c_int,
    mut for_real: boolean,
) -> ::core::ffi::c_int {
    let mut argn: ::core::ffi::c_int = 0;
    let mut arg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    requested_fmt = FMT_PPM;
    icc_filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    max_scans = 0 as JDIMENSION;
    outfilename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    memsrc = FALSE as boolean;
    report = FALSE as boolean;
    skip = FALSE as boolean;
    crop = FALSE as boolean;
    strict = FALSE as boolean;
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
                b"bmp\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                requested_fmt = FMT_BMP;
            } else if keymatch(
                arg,
                b"colors\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
                || keymatch(
                    arg,
                    b"colours\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
                || keymatch(
                    arg,
                    b"quantize\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
                || keymatch(
                    arg,
                    b"quantise\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
            {
                let mut val: ::core::ffi::c_int = 0;
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if sscanf(
                    *argv.offset(argn as isize),
                    b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut val,
                ) != 1 as ::core::ffi::c_int
                {
                    usage();
                }
                (*cinfo).desired_number_of_colors = val;
                (*cinfo).quantize_colors = TRUE as boolean;
            } else if keymatch(
                arg,
                b"dct\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if keymatch(
                    *argv.offset(argn as isize),
                    b"int\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
                {
                    (*cinfo).dct_method = JDCT_ISLOW;
                } else if keymatch(
                    *argv.offset(argn as isize),
                    b"fast\0" as *const u8 as *const ::core::ffi::c_char,
                    2 as ::core::ffi::c_int,
                ) != 0
                {
                    (*cinfo).dct_method = JDCT_IFAST;
                } else if keymatch(
                    *argv.offset(argn as isize),
                    b"float\0" as *const u8 as *const ::core::ffi::c_char,
                    2 as ::core::ffi::c_int,
                ) != 0
                {
                    (*cinfo).dct_method = JDCT_FLOAT;
                } else {
                    usage();
                }
            } else if keymatch(
                arg,
                b"dither\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if keymatch(
                    *argv.offset(argn as isize),
                    b"fs\0" as *const u8 as *const ::core::ffi::c_char,
                    2 as ::core::ffi::c_int,
                ) != 0
                {
                    (*cinfo).dither_mode = JDITHER_FS;
                } else if keymatch(
                    *argv.offset(argn as isize),
                    b"none\0" as *const u8 as *const ::core::ffi::c_char,
                    2 as ::core::ffi::c_int,
                ) != 0
                {
                    (*cinfo).dither_mode = JDITHER_NONE;
                } else if keymatch(
                    *argv.offset(argn as isize),
                    b"ordered\0" as *const u8 as *const ::core::ffi::c_char,
                    2 as ::core::ffi::c_int,
                ) != 0
                {
                    (*cinfo).dither_mode = JDITHER_ORDERED;
                } else {
                    usage();
                }
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
                b"fast\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                (*cinfo).two_pass_quantize = FALSE as boolean;
                (*cinfo).dither_mode = JDITHER_ORDERED;
                if (*cinfo).quantize_colors == 0 {
                    (*cinfo).desired_number_of_colors = 216 as ::core::ffi::c_int;
                }
                (*cinfo).dct_method = JDCT_IFAST;
                (*cinfo).do_fancy_upsampling = FALSE as boolean;
            } else if keymatch(
                arg,
                b"gif\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                requested_fmt = FMT_GIF;
            } else if keymatch(
                arg,
                b"gif0\0" as *const u8 as *const ::core::ffi::c_char,
                4 as ::core::ffi::c_int,
            ) != 0
            {
                requested_fmt = FMT_GIF0;
            } else if keymatch(
                arg,
                b"grayscale\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
                || keymatch(
                    arg,
                    b"greyscale\0" as *const u8 as *const ::core::ffi::c_char,
                    2 as ::core::ffi::c_int,
                ) != 0
            {
                (*cinfo).out_color_space = JCS_GRAYSCALE;
            } else if keymatch(
                arg,
                b"rgb\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                (*cinfo).out_color_space = JCS_RGB;
            } else if keymatch(
                arg,
                b"rgb565\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                (*cinfo).out_color_space = JCS_RGB565;
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
                jpeg_save_markers(
                    cinfo,
                    JPEG_APP0 + 2 as ::core::ffi::c_int,
                    0xffff as ::core::ffi::c_uint,
                );
            } else if keymatch(
                arg,
                b"map\0" as *const u8 as *const ::core::ffi::c_char,
                3 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if for_real != 0 {
                    let mut mapfile: *mut FILE = ::core::ptr::null_mut::<FILE>();
                    mapfile = fopen(*argv.offset(argn as isize), READ_BINARY.as_ptr()) as *mut FILE;
                    if mapfile.is_null() {
                        fprintf(
                            stderr,
                            b"%s: can't open %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                            progname,
                            *argv.offset(argn as isize),
                        );
                        exit(EXIT_FAILURE);
                    }
                    read_color_map(cinfo, mapfile);
                    fclose(mapfile);
                    (*cinfo).quantize_colors = TRUE as boolean;
                }
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
                b"nosmooth\0" as *const u8 as *const ::core::ffi::c_char,
                3 as ::core::ffi::c_int,
            ) != 0
            {
                (*cinfo).do_fancy_upsampling = FALSE as boolean;
            } else if keymatch(
                arg,
                b"onepass\0" as *const u8 as *const ::core::ffi::c_char,
                3 as ::core::ffi::c_int,
            ) != 0
            {
                (*cinfo).two_pass_quantize = FALSE as boolean;
            } else if keymatch(
                arg,
                b"os2\0" as *const u8 as *const ::core::ffi::c_char,
                3 as ::core::ffi::c_int,
            ) != 0
            {
                requested_fmt = FMT_OS2;
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
                b"memsrc\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                memsrc = TRUE as boolean;
            } else if keymatch(
                arg,
                b"pnm\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
                || keymatch(
                    arg,
                    b"ppm\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) != 0
            {
                requested_fmt = FMT_PPM;
            } else if keymatch(
                arg,
                b"report\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                report = TRUE as boolean;
            } else if keymatch(
                arg,
                b"scale\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if sscanf(
                    *argv.offset(argn as isize),
                    b"%u/%u\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut (*cinfo).scale_num,
                    &raw mut (*cinfo).scale_denom,
                ) != 2 as ::core::ffi::c_int
                {
                    usage();
                }
            } else if keymatch(
                arg,
                b"skip\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if sscanf(
                    *argv.offset(argn as isize),
                    b"%u,%u\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut skip_start,
                    &raw mut skip_end,
                ) != 2 as ::core::ffi::c_int
                    || skip_start > skip_end
                {
                    usage();
                }
                skip = TRUE as boolean;
            } else if keymatch(
                arg,
                b"crop\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                let mut c: ::core::ffi::c_char = 0;
                argn += 1;
                if argn >= argc {
                    usage();
                }
                if sscanf(
                    *argv.offset(argn as isize),
                    b"%u%c%u+%u+%u\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut crop_width,
                    &raw mut c,
                    &raw mut crop_height,
                    &raw mut crop_x,
                    &raw mut crop_y,
                ) != 5 as ::core::ffi::c_int
                    || c as ::core::ffi::c_int != 'X' as i32
                        && c as ::core::ffi::c_int != 'x' as i32
                    || crop_width < 1 as JDIMENSION
                    || crop_height < 1 as JDIMENSION
                {
                    usage();
                }
                crop = TRUE as boolean;
            } else if keymatch(
                arg,
                b"strict\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) != 0
            {
                strict = TRUE as boolean;
            } else if keymatch(
                arg,
                b"targa\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                requested_fmt = FMT_TARGA;
            } else {
                usage();
            }
        }
        argn += 1;
    }
    return argn;
}
unsafe extern "C" fn jpeg_getc(mut cinfo: j_decompress_ptr) -> ::core::ffi::c_uint {
    let mut datasrc: *mut jpeg_source_mgr = (*cinfo).src as *mut jpeg_source_mgr;
    if (*datasrc).bytes_in_buffer == 0 as size_t {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            (*(*cinfo).err).msg_code = JERR_CANT_SUSPEND as ::core::ffi::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    (*datasrc).bytes_in_buffer = (*datasrc).bytes_in_buffer.wrapping_sub(1);
    let fresh0 = (*datasrc).next_input_byte;
    (*datasrc).next_input_byte = (*datasrc).next_input_byte.offset(1);
    return *fresh0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn print_text_marker(mut cinfo: j_decompress_ptr) -> boolean {
    let mut traceit: boolean =
        ((*(*cinfo).err).trace_level >= 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut length: ::core::ffi::c_long = 0;
    let mut ch: ::core::ffi::c_uint = 0;
    let mut lastch: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    length = (jpeg_getc(cinfo) << 8 as ::core::ffi::c_int) as ::core::ffi::c_long;
    length += jpeg_getc(cinfo) as ::core::ffi::c_long;
    length -= 2 as ::core::ffi::c_long;
    if traceit != 0 {
        if (*cinfo).unread_marker == JPEG_COM {
            fprintf(
                stderr,
                b"Comment, length %ld:\n\0" as *const u8 as *const ::core::ffi::c_char,
                length,
            );
        } else {
            fprintf(
                stderr,
                b"APP%d, length %ld:\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*cinfo).unread_marker - JPEG_APP0,
                length,
            );
        }
    }
    loop {
        length -= 1;
        if !(length >= 0 as ::core::ffi::c_long) {
            break;
        }
        ch = jpeg_getc(cinfo);
        if traceit != 0 {
            if ch == '\r' as i32 as ::core::ffi::c_uint {
                fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            } else if ch == '\n' as i32 as ::core::ffi::c_uint {
                if lastch != '\r' as i32 as ::core::ffi::c_uint {
                    fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                }
            } else if ch == '\\' as i32 as ::core::ffi::c_uint {
                fprintf(stderr, b"\\\\\0" as *const u8 as *const ::core::ffi::c_char);
            } else if *(*__ctype_b_loc()).offset(ch as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISprint as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
            {
                putc(ch as ::core::ffi::c_int, stderr);
            } else {
                fprintf(
                    stderr,
                    b"\\%03o\0" as *const u8 as *const ::core::ffi::c_char,
                    ch,
                );
            }
            lastch = ch;
        }
    }
    if traceit != 0 {
        fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return TRUE;
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
    let mut cinfo: jpeg_decompress_struct = jpeg_decompress_struct {
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
    let mut jerr: jpeg_error_mgr = jpeg_error_mgr {
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
    let mut progress: cdjpeg_progress_mgr = cdjpeg_progress_mgr {
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
    let mut file_index: ::core::ffi::c_int = 0;
    let mut dest_mgr: djpeg_dest_ptr = ::core::ptr::null_mut::<djpeg_dest_struct>();
    let mut input_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut output_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut inbuffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut insize: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut num_scanlines: JDIMENSION = 0;
    progname = *argv.offset(0 as ::core::ffi::c_int as isize);
    if progname.is_null()
        || *progname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        progname = b"djpeg\0" as *const u8 as *const ::core::ffi::c_char;
    }
    cinfo.err = jpeg_std_error(&raw mut jerr);
    jpeg_CreateDecompress(
        &raw mut cinfo,
        JPEG_LIB_VERSION,
        ::core::mem::size_of::<jpeg_decompress_struct>(),
    );
    jerr.addon_message_table = &raw const cdjpeg_message_table as *const *const ::core::ffi::c_char;
    jerr.first_addon_message = JMSG_FIRSTADDONCODE as ::core::ffi::c_int;
    jerr.last_addon_message = JMSG_LASTADDONCODE as ::core::ffi::c_int;
    jpeg_set_marker_processor(
        &raw mut cinfo,
        JPEG_COM,
        Some(print_text_marker as unsafe extern "C" fn(j_decompress_ptr) -> boolean),
    );
    jpeg_set_marker_processor(
        &raw mut cinfo,
        JPEG_APP0 + 12 as ::core::ffi::c_int,
        Some(print_text_marker as unsafe extern "C" fn(j_decompress_ptr) -> boolean),
    );
    file_index = parse_switches(&raw mut cinfo, argc, argv, 0 as ::core::ffi::c_int, FALSE);
    if strict != 0 {
        jerr.emit_message =
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
        input_file = fopen(*argv.offset(file_index as isize), READ_BINARY.as_ptr()) as *mut FILE;
        if input_file.is_null() {
            fprintf(
                stderr,
                b"%s: can't open %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                progname,
                *argv.offset(file_index as isize),
            );
            exit(EXIT_FAILURE);
        }
    } else {
        input_file = read_stdin();
    }
    if !outfilename.is_null() {
        output_file = fopen(outfilename, WRITE_BINARY.as_ptr()) as *mut FILE;
        if output_file.is_null() {
            fprintf(
                stderr,
                b"%s: can't open %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                progname,
                outfilename,
            );
            exit(EXIT_FAILURE);
        }
    } else {
        output_file = write_stdout();
    }
    if report != 0 || max_scans != 0 as JDIMENSION {
        start_progress_monitor(&raw mut cinfo as j_common_ptr, &raw mut progress);
        progress.report = report;
        progress.max_scans = max_scans;
    }
    if memsrc != 0 {
        let mut nbytes: size_t = 0;
        loop {
            inbuffer = realloc(
                inbuffer as *mut ::core::ffi::c_void,
                (insize as size_t).wrapping_add(INPUT_BUF_SIZE as size_t),
            ) as *mut ::core::ffi::c_uchar;
            if inbuffer.is_null() {
                fprintf(
                    stderr,
                    b"%s: memory allocation failure\n\0" as *const u8 as *const ::core::ffi::c_char,
                    progname,
                );
                exit(EXIT_FAILURE);
            }
            nbytes = fread(
                inbuffer.offset(insize as isize) as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                1 as size_t,
                INPUT_BUF_SIZE as size_t,
                input_file,
            ) as size_t;
            if nbytes < INPUT_BUF_SIZE as size_t && ferror(input_file) != 0 {
                if file_index < argc {
                    fprintf(
                        stderr,
                        b"%s: can't read from %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                        progname,
                        *argv.offset(file_index as isize),
                    );
                } else {
                    fprintf(
                        stderr,
                        b"%s: can't read from stdin\n\0" as *const u8 as *const ::core::ffi::c_char,
                        progname,
                    );
                }
            }
            insize = insize.wrapping_add(nbytes as ::core::ffi::c_ulong);
            if !(nbytes == INPUT_BUF_SIZE as size_t) {
                break;
            }
        }
        fprintf(
            stderr,
            b"Compressed size:  %lu bytes\n\0" as *const u8 as *const ::core::ffi::c_char,
            insize,
        );
        jpeg_mem_src(&raw mut cinfo, inbuffer, insize);
    } else {
        jpeg_stdio_src(&raw mut cinfo, input_file);
    }
    jpeg_read_header(&raw mut cinfo, TRUE);
    file_index = parse_switches(&raw mut cinfo, argc, argv, 0 as ::core::ffi::c_int, TRUE);
    match requested_fmt as ::core::ffi::c_uint {
        0 => {
            dest_mgr = jinit_write_bmp(&raw mut cinfo, FALSE, TRUE);
        }
        3 => {
            dest_mgr = jinit_write_bmp(&raw mut cinfo, TRUE, TRUE);
        }
        1 => {
            dest_mgr = jinit_write_gif(&raw mut cinfo, TRUE);
        }
        2 => {
            dest_mgr = jinit_write_gif(&raw mut cinfo, FALSE);
        }
        4 => {
            dest_mgr = jinit_write_ppm(&raw mut cinfo);
        }
        5 => {
            dest_mgr = jinit_write_targa(&raw mut cinfo);
        }
        _ => {
            (*cinfo.err).msg_code = JERR_UNSUPPORTED_FORMAT as ::core::ffi::c_int;
            Some((*cinfo.err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(&raw mut cinfo as j_common_ptr);
        }
    }
    (*dest_mgr).output_file = output_file;
    jpeg_start_decompress(&raw mut cinfo);
    if skip != 0 {
        let mut tmp: JDIMENSION = 0;
        if skip_end > cinfo.output_height.wrapping_sub(1 as JDIMENSION) {
            fprintf(
                stderr,
                b"%s: skip region exceeds image height %u\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progname,
                cinfo.output_height,
            );
            exit(EXIT_FAILURE);
        }
        tmp = cinfo.output_height;
        cinfo.output_height = cinfo.output_height.wrapping_sub(
            skip_end
                .wrapping_sub(skip_start)
                .wrapping_add(1 as JDIMENSION),
        );
        Some((*dest_mgr).start_output.expect("non-null function pointer"))
            .expect("non-null function pointer")(&raw mut cinfo, dest_mgr);
        cinfo.output_height = tmp;
        while cinfo.output_scanline < skip_start {
            num_scanlines = jpeg_read_scanlines(
                &raw mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                &raw mut cinfo, dest_mgr, num_scanlines
            );
        }
        tmp = jpeg_skip_scanlines(
            &raw mut cinfo,
            skip_end
                .wrapping_sub(skip_start)
                .wrapping_add(1 as JDIMENSION),
        );
        if tmp
            != skip_end
                .wrapping_sub(skip_start)
                .wrapping_add(1 as JDIMENSION)
        {
            fprintf(
                stderr,
                b"%s: jpeg_skip_scanlines() returned %u rather than %u\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progname,
                tmp,
                skip_end
                    .wrapping_sub(skip_start)
                    .wrapping_add(1 as JDIMENSION),
            );
            exit(EXIT_FAILURE);
        }
        while cinfo.output_scanline < cinfo.output_height {
            num_scanlines = jpeg_read_scanlines(
                &raw mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                &raw mut cinfo, dest_mgr, num_scanlines
            );
        }
    } else if crop != 0 {
        let mut tmp_0: JDIMENSION = 0;
        if crop_x.wrapping_add(crop_width) > cinfo.output_width
            || crop_y.wrapping_add(crop_height) > cinfo.output_height
        {
            fprintf(
                stderr,
                b"%s: crop dimensions exceed image dimensions %u x %u\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progname,
                cinfo.output_width,
                cinfo.output_height,
            );
            exit(EXIT_FAILURE);
        }
        jpeg_crop_scanline(&raw mut cinfo, &raw mut crop_x, &raw mut crop_width);
        if (*dest_mgr).calc_buffer_dimensions.is_some() {
            Some(
                (*dest_mgr)
                    .calc_buffer_dimensions
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(&raw mut cinfo, dest_mgr);
        } else {
            (*cinfo.err).msg_code = JERR_UNSUPPORTED_FORMAT as ::core::ffi::c_int;
            Some((*cinfo.err).error_exit.expect("non-null function pointer"))
                .expect("non-null function pointer")(&raw mut cinfo as j_common_ptr);
        }
        tmp_0 = cinfo.output_height;
        cinfo.output_height = crop_height;
        Some((*dest_mgr).start_output.expect("non-null function pointer"))
            .expect("non-null function pointer")(&raw mut cinfo, dest_mgr);
        cinfo.output_height = tmp_0;
        tmp_0 = jpeg_skip_scanlines(&raw mut cinfo, crop_y);
        if tmp_0 != crop_y {
            fprintf(
                stderr,
                b"%s: jpeg_skip_scanlines() returned %u rather than %u\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progname,
                tmp_0,
                crop_y,
            );
            exit(EXIT_FAILURE);
        }
        while cinfo.output_scanline < crop_y.wrapping_add(crop_height) {
            num_scanlines = jpeg_read_scanlines(
                &raw mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                &raw mut cinfo, dest_mgr, num_scanlines
            );
        }
        tmp_0 = jpeg_skip_scanlines(
            &raw mut cinfo,
            cinfo
                .output_height
                .wrapping_sub(crop_y)
                .wrapping_sub(crop_height),
        );
        if tmp_0
            != cinfo
                .output_height
                .wrapping_sub(crop_y)
                .wrapping_sub(crop_height)
        {
            fprintf(
                stderr,
                b"%s: jpeg_skip_scanlines() returned %u rather than %u\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progname,
                tmp_0,
                cinfo
                    .output_height
                    .wrapping_sub(crop_y)
                    .wrapping_sub(crop_height),
            );
            exit(EXIT_FAILURE);
        }
    } else {
        Some((*dest_mgr).start_output.expect("non-null function pointer"))
            .expect("non-null function pointer")(&raw mut cinfo, dest_mgr);
        while cinfo.output_scanline < cinfo.output_height {
            num_scanlines = jpeg_read_scanlines(
                &raw mut cinfo,
                (*dest_mgr).buffer,
                (*dest_mgr).buffer_height,
            );
            Some(
                (*dest_mgr)
                    .put_pixel_rows
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                &raw mut cinfo, dest_mgr, num_scanlines
            );
        }
    }
    if report != 0 || max_scans != 0 as JDIMENSION {
        progress.pub_0.completed_passes = progress.pub_0.total_passes;
    }
    if !icc_filename.is_null() {
        let mut icc_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut icc_profile: *mut JOCTET = ::core::ptr::null_mut::<JOCTET>();
        let mut icc_len: ::core::ffi::c_uint = 0;
        icc_file = fopen(icc_filename, WRITE_BINARY.as_ptr()) as *mut FILE;
        if icc_file.is_null() {
            fprintf(
                stderr,
                b"%s: can't open %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                progname,
                icc_filename,
            );
            exit(EXIT_FAILURE);
        }
        if jpeg_read_icc_profile(&raw mut cinfo, &raw mut icc_profile, &raw mut icc_len) != 0 {
            if fwrite(
                icc_profile as *const ::core::ffi::c_void,
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
            free(icc_profile as *mut ::core::ffi::c_void);
            fclose(icc_file);
        } else if (*cinfo.err).msg_code != JWRN_BOGUS_ICC as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"%s: no ICC profile data in JPEG file\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                progname,
            );
        }
    }
    Some(
        (*dest_mgr)
            .finish_output
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(&raw mut cinfo, dest_mgr);
    jpeg_finish_decompress(&raw mut cinfo);
    jpeg_destroy_decompress(&raw mut cinfo);
    if input_file != stdin {
        fclose(input_file);
    }
    if output_file != stdout {
        fclose(output_file);
    }
    if report != 0 || max_scans != 0 as JDIMENSION {
        end_progress_monitor(&raw mut cinfo as j_common_ptr);
    }
    if memsrc != 0 {
        free(inbuffer as *mut ::core::ffi::c_void);
    }
    exit(if jerr.num_warnings != 0 {
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
