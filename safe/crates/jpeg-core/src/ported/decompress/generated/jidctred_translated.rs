#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}

extern "C" {
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
pub type ISLOW_MULT_TYPE = ::core::ffi::c_short;
pub const MAXJSAMPLE: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const CENTERJSAMPLE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const DCTSIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RANGE_MASK: ::core::ffi::c_int =
    MAXJSAMPLE * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
pub unsafe extern "C" fn jpeg_idct_4x4(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: JLONG = 0;
    let mut tmp2: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut inptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut quantptr: *mut ISLOW_MULT_TYPE = ::core::ptr::null_mut::<ISLOW_MULT_TYPE>();
    let mut wsptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: ::core::ffi::c_int = 0;
    let mut workspace: [::core::ffi::c_int; 32] = [0; 32];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = ::core::ptr::addr_of_mut!(workspace) as *mut ::core::ffi::c_int;
    ctr = DCTSIZE;
    while ctr > 0 as ::core::ffi::c_int {
        if !(ctr == DCTSIZE - 4 as ::core::ffi::c_int) {
            if *inptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && *inptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && *inptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && *inptr.offset((DCTSIZE * 5 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && *inptr.offset((DCTSIZE * 6 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && *inptr.offset((DCTSIZE * 7 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                let mut dcval: ::core::ffi::c_int =
                    (((*inptr.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        * *quantptr
                            .offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                        << 2 as ::core::ffi::c_int) as JLONG
                        as ::core::ffi::c_int;
                *wsptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize) = dcval;
            } else {
                tmp0 = (*inptr.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                tmp0 = ((tmp0 as ::core::ffi::c_ulong)
                    << 13 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as JLONG;
                z2 = (*inptr.offset((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                z3 = (*inptr.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                tmp2 = z2 * 15137 as ::core::ffi::c_int as JLONG
                    + z3 * -(6270 as ::core::ffi::c_int as JLONG);
                tmp10 = tmp0 + tmp2;
                tmp12 = tmp0 - tmp2;
                z1 = (*inptr.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                z2 = (*inptr.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                z3 = (*inptr.offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                z4 = (*inptr.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                tmp0 = z1 * -(1730 as ::core::ffi::c_int as JLONG)
                    + z2 * 11893 as ::core::ffi::c_int as JLONG
                    + z3 * -(17799 as ::core::ffi::c_int as JLONG)
                    + z4 * 8697 as ::core::ffi::c_int as JLONG;
                tmp2 = z1 * -(4176 as ::core::ffi::c_int as JLONG)
                    + z2 * -(4926 as ::core::ffi::c_int as JLONG)
                    + z3 * 7373 as ::core::ffi::c_int as JLONG
                    + z4 * 20995 as ::core::ffi::c_int as JLONG;
                *wsptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize) = (tmp10
                    + tmp2
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                *wsptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize) = (tmp10 - tmp2
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                *wsptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) = (tmp12
                    + tmp0
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                *wsptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize) = (tmp12 - tmp0
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            }
        }
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1;
    }
    wsptr = ::core::ptr::addr_of_mut!(workspace) as *mut ::core::ffi::c_int;
    ctr = 0 as ::core::ffi::c_int;
    while ctr < 4 as ::core::ffi::c_int {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        if *wsptr.offset(1 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
            && *wsptr.offset(2 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
            && *wsptr.offset(3 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
            && *wsptr.offset(5 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
            && *wsptr.offset(6 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
            && *wsptr.offset(7 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        {
            let mut dcval_0: JSAMPLE = *range_limit.offset(
                ((*wsptr.offset(0 as ::core::ffi::c_int as isize) as JLONG
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(0 as ::core::ffi::c_int as isize) = dcval_0;
            *outptr.offset(1 as ::core::ffi::c_int as isize) = dcval_0;
            *outptr.offset(2 as ::core::ffi::c_int as isize) = dcval_0;
            *outptr.offset(3 as ::core::ffi::c_int as isize) = dcval_0;
            wsptr = wsptr.offset(DCTSIZE as isize);
        } else {
            tmp0 = ((*wsptr.offset(0 as ::core::ffi::c_int as isize) as JLONG
                as ::core::ffi::c_ulong)
                << 13 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as JLONG;
            tmp2 = *wsptr.offset(2 as ::core::ffi::c_int as isize) as JLONG
                * 15137 as ::core::ffi::c_int as JLONG
                + *wsptr.offset(6 as ::core::ffi::c_int as isize) as JLONG
                    * -(6270 as ::core::ffi::c_int as JLONG);
            tmp10 = tmp0 + tmp2;
            tmp12 = tmp0 - tmp2;
            z1 = *wsptr.offset(7 as ::core::ffi::c_int as isize) as JLONG;
            z2 = *wsptr.offset(5 as ::core::ffi::c_int as isize) as JLONG;
            z3 = *wsptr.offset(3 as ::core::ffi::c_int as isize) as JLONG;
            z4 = *wsptr.offset(1 as ::core::ffi::c_int as isize) as JLONG;
            tmp0 = z1 * -(1730 as ::core::ffi::c_int as JLONG)
                + z2 * 11893 as ::core::ffi::c_int as JLONG
                + z3 * -(17799 as ::core::ffi::c_int as JLONG)
                + z4 * 8697 as ::core::ffi::c_int as JLONG;
            tmp2 = z1 * -(4176 as ::core::ffi::c_int as JLONG)
                + z2 * -(4926 as ::core::ffi::c_int as JLONG)
                + z3 * 7373 as ::core::ffi::c_int as JLONG
                + z4 * 20995 as ::core::ffi::c_int as JLONG;
            *outptr.offset(0 as ::core::ffi::c_int as isize) = *range_limit.offset(
                ((tmp10
                    + tmp2
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(3 as ::core::ffi::c_int as isize) = *range_limit.offset(
                ((tmp10 - tmp2
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(1 as ::core::ffi::c_int as isize) = *range_limit.offset(
                ((tmp12
                    + tmp0
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(2 as ::core::ffi::c_int as isize) = *range_limit.offset(
                ((tmp12 - tmp0
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    & RANGE_MASK) as isize,
            );
            wsptr = wsptr.offset(DCTSIZE as isize);
        }
        ctr += 1;
    }
}
pub unsafe extern "C" fn jpeg_idct_2x2(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut inptr: JCOEFPTR = ::core::ptr::null_mut::<JCOEF>();
    let mut quantptr: *mut ISLOW_MULT_TYPE = ::core::ptr::null_mut::<ISLOW_MULT_TYPE>();
    let mut wsptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut outptr: JSAMPROW = ::core::ptr::null_mut::<JSAMPLE>();
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: ::core::ffi::c_int = 0;
    let mut workspace: [::core::ffi::c_int; 16] = [0; 16];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    wsptr = ::core::ptr::addr_of_mut!(workspace) as *mut ::core::ffi::c_int;
    ctr = DCTSIZE;
    while ctr > 0 as ::core::ffi::c_int {
        if !(ctr == DCTSIZE - 2 as ::core::ffi::c_int
            || ctr == DCTSIZE - 4 as ::core::ffi::c_int
            || ctr == DCTSIZE - 6 as ::core::ffi::c_int)
        {
            if *inptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && *inptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && *inptr.offset((DCTSIZE * 5 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && *inptr.offset((DCTSIZE * 7 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                let mut dcval: ::core::ffi::c_int =
                    (((*inptr.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        * *quantptr
                            .offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                        << 2 as ::core::ffi::c_int) as JLONG
                        as ::core::ffi::c_int;
                *wsptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize) = dcval;
                *wsptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) = dcval;
            } else {
                z1 = (*inptr.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                tmp10 = ((z1 as ::core::ffi::c_ulong)
                    << 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as JLONG;
                z1 = (*inptr.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                tmp0 = z1 * -(5906 as ::core::ffi::c_int as JLONG);
                z1 = (*inptr.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                tmp0 += z1 * 6967 as ::core::ffi::c_int as JLONG;
                z1 = (*inptr.offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                tmp0 += z1 * -(10426 as ::core::ffi::c_int as JLONG);
                z1 = (*inptr.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * *quantptr.offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int) as JLONG;
                tmp0 += z1 * 29692 as ::core::ffi::c_int as JLONG;
                *wsptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize) = (tmp10
                    + tmp0
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                *wsptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) = (tmp10 - tmp0
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            }
        }
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1;
    }
    wsptr = ::core::ptr::addr_of_mut!(workspace) as *mut ::core::ffi::c_int;
    ctr = 0 as ::core::ffi::c_int;
    while ctr < 2 as ::core::ffi::c_int {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        if *wsptr.offset(1 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
            && *wsptr.offset(3 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
            && *wsptr.offset(5 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
            && *wsptr.offset(7 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int
        {
            let mut dcval_0: JSAMPLE = *range_limit.offset(
                ((*wsptr.offset(0 as ::core::ffi::c_int as isize) as JLONG
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(0 as ::core::ffi::c_int as isize) = dcval_0;
            *outptr.offset(1 as ::core::ffi::c_int as isize) = dcval_0;
            wsptr = wsptr.offset(DCTSIZE as isize);
        } else {
            tmp10 = ((*wsptr.offset(0 as ::core::ffi::c_int as isize) as JLONG
                as ::core::ffi::c_ulong)
                << 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as JLONG;
            tmp0 = *wsptr.offset(7 as ::core::ffi::c_int as isize) as JLONG
                * -(5906 as ::core::ffi::c_int as JLONG)
                + *wsptr.offset(5 as ::core::ffi::c_int as isize) as JLONG
                    * 6967 as ::core::ffi::c_int as JLONG
                + *wsptr.offset(3 as ::core::ffi::c_int as isize) as JLONG
                    * -(10426 as ::core::ffi::c_int as JLONG)
                + *wsptr.offset(1 as ::core::ffi::c_int as isize) as JLONG
                    * 29692 as ::core::ffi::c_int as JLONG;
            *outptr.offset(0 as ::core::ffi::c_int as isize) = *range_limit.offset(
                ((tmp10
                    + tmp0
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as ::core::ffi::c_int
                    & RANGE_MASK) as isize,
            );
            *outptr.offset(1 as ::core::ffi::c_int as isize) = *range_limit.offset(
                ((tmp10 - tmp0
                    + ((1 as ::core::ffi::c_int as JLONG)
                        << 13 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 13 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as ::core::ffi::c_int
                    & RANGE_MASK) as isize,
            );
            wsptr = wsptr.offset(DCTSIZE as isize);
        }
        ctr += 1;
    }
}
pub unsafe extern "C" fn jpeg_idct_1x1(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut dcval: ::core::ffi::c_int = 0;
    let mut quantptr: *mut ISLOW_MULT_TYPE = ::core::ptr::null_mut::<ISLOW_MULT_TYPE>();
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    quantptr = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
    dcval = *coef_block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        * *quantptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    dcval = (dcval as JLONG
        + ((1 as ::core::ffi::c_int as JLONG) << 3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
    *(*output_buf.offset(0 as ::core::ffi::c_int as isize)).offset(output_col as isize) =
        *range_limit.offset((dcval & RANGE_MASK) as isize);
}
