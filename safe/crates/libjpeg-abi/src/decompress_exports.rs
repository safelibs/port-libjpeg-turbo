use core::ffi::c_uint;

use ffi_types::{
    boolean, int, j_decompress_ptr, jpeg_component_info, jpeg_marker_parser_method,
    jvirt_barray_ptr, JCOEFPTR, JDIMENSION, JSAMPARRAY, JSAMPIMAGE,
};

use crate::decompress;

pub const EXPECTED_DECOMPRESS_SYMBOLS: &[&str] = &[
    "jpeg_CreateDecompress",
    "jpeg_destroy_decompress",
    "jpeg_abort_decompress",
    "jpeg_read_header",
    "jpeg_consume_input",
    "jpeg_input_complete",
    "jpeg_has_multiple_scans",
    "jpeg_finish_decompress",
    "jpeg_start_decompress",
    "jpeg_read_scanlines",
    "jpeg_crop_scanline",
    "jpeg_skip_scanlines",
    "jpeg_read_raw_data",
    "jpeg_start_output",
    "jpeg_finish_output",
    "jpeg_read_coefficients",
    "jpeg_save_markers",
    "jpeg_set_marker_processor",
];

#[no_mangle]
pub unsafe extern "C" fn jpeg_CreateDecompress(
    cinfo: j_decompress_ptr,
    version: int,
    structsize: usize,
) {
    decompress::jdapimin::jpeg_CreateDecompress(cinfo, version, structsize)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_destroy_decompress(cinfo: j_decompress_ptr) {
    decompress::jdapimin::jpeg_destroy_decompress(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_abort_decompress(cinfo: j_decompress_ptr) {
    decompress::jdapimin::jpeg_abort_decompress(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_read_header(cinfo: j_decompress_ptr, require_image: boolean) -> int {
    decompress::jdapimin::jpeg_read_header(cinfo, require_image)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_consume_input(cinfo: j_decompress_ptr) -> int {
    decompress::jdapimin::jpeg_consume_input(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_input_complete(cinfo: j_decompress_ptr) -> boolean {
    decompress::jdapimin::jpeg_input_complete(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_has_multiple_scans(cinfo: j_decompress_ptr) -> boolean {
    decompress::jdapimin::jpeg_has_multiple_scans(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean {
    decompress::jdapimin::jpeg_finish_decompress(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_start_decompress(cinfo: j_decompress_ptr) -> boolean {
    decompress::jdapistd::jpeg_start_decompress(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_read_scanlines(
    cinfo: j_decompress_ptr,
    scanlines: JSAMPARRAY,
    max_lines: JDIMENSION,
) -> JDIMENSION {
    decompress::jdapistd::jpeg_read_scanlines(cinfo, scanlines, max_lines)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_crop_scanline(
    cinfo: j_decompress_ptr,
    xoffset: *mut JDIMENSION,
    width: *mut JDIMENSION,
) {
    decompress::jdapistd::jpeg_crop_scanline(cinfo, xoffset, width)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_skip_scanlines(
    cinfo: j_decompress_ptr,
    num_lines: JDIMENSION,
) -> JDIMENSION {
    decompress::jdapistd::jpeg_skip_scanlines(cinfo, num_lines)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_read_raw_data(
    cinfo: j_decompress_ptr,
    data: JSAMPIMAGE,
    max_lines: JDIMENSION,
) -> JDIMENSION {
    decompress::jdapistd::jpeg_read_raw_data(cinfo, data, max_lines)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_start_output(cinfo: j_decompress_ptr, scan_number: int) -> boolean {
    decompress::jdapistd::jpeg_start_output(cinfo, scan_number)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_finish_output(cinfo: j_decompress_ptr) -> boolean {
    decompress::jdapistd::jpeg_finish_output(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_read_coefficients(cinfo: j_decompress_ptr) -> *mut jvirt_barray_ptr {
    decompress::jdtrans::jpeg_read_coefficients(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_color_deconverter(cinfo: j_decompress_ptr) {
    decompress::jdcolor::jinit_color_deconverter(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_d_coef_controller(
    cinfo: j_decompress_ptr,
    need_full_buffer: boolean,
) {
    decompress::jdcoefct::jinit_d_coef_controller(cinfo, need_full_buffer)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_d_main_controller(
    cinfo: j_decompress_ptr,
    need_full_buffer: boolean,
) {
    decompress::jdmainct::jinit_d_main_controller(cinfo, need_full_buffer)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_huff_decoder(cinfo: j_decompress_ptr) {
    decompress::jdhuff::jinit_huff_decoder(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_phuff_decoder(cinfo: j_decompress_ptr) {
    decompress::jdphuff::jinit_phuff_decoder(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_arith_decoder(cinfo: j_decompress_ptr) {
    decompress::jdarith::jinit_arith_decoder(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_d_post_controller(
    cinfo: j_decompress_ptr,
    need_full_buffer: boolean,
) {
    decompress::jdpostct::jinit_d_post_controller(cinfo, need_full_buffer)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_1pass_quantizer(cinfo: j_decompress_ptr) {
    decompress::jquant1::jinit_1pass_quantizer(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_2pass_quantizer(cinfo: j_decompress_ptr) {
    decompress::jquant2::jinit_2pass_quantizer(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_rs_set_max_scans(cinfo: j_decompress_ptr, max_scans: int) {
    crate::set_decompress_scan_limit(cinfo, max_scans)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_rs_set_warnings_fatal(cinfo: j_decompress_ptr, fatal: boolean) {
    crate::set_decompress_warnings_fatal(cinfo, fatal)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_rs_configure_decompress_policy(
    cinfo: j_decompress_ptr,
    max_scans: int,
    warnings_fatal: boolean,
) {
    crate::configure_decompress_policy(cinfo, max_scans, warnings_fatal)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_rs_get_max_scans(cinfo: j_decompress_ptr) -> int {
    crate::decompress_scan_limit(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_rs_get_warnings_fatal(cinfo: j_decompress_ptr) -> boolean {
    crate::decompress_warnings_fatal(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_make_d_derived_tbl(
    cinfo: j_decompress_ptr,
    is_dc: boolean,
    tblno: int,
    pdtbl: *mut *mut jpeg_core::ported::decompress::jdhuff::d_derived_tbl,
) {
    jpeg_core::ported::decompress::jdhuff::jpeg_make_d_derived_tbl(cinfo, is_dc, tblno, pdtbl)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_fill_bit_buffer(
    state: *mut jpeg_core::ported::decompress::jdhuff::bitread_working_state,
    get_buffer: jpeg_core::ported::decompress::jdhuff::bit_buf_type,
    bits_left: int,
    nbits: int,
) -> boolean {
    jpeg_core::ported::decompress::jdhuff::jpeg_fill_bit_buffer(state, get_buffer, bits_left, nbits)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_huff_decode(
    state: *mut jpeg_core::ported::decompress::jdhuff::bitread_working_state,
    get_buffer: jpeg_core::ported::decompress::jdhuff::bit_buf_type,
    bits_left: int,
    htbl: *mut jpeg_core::ported::decompress::jdhuff::d_derived_tbl,
    min_bits: int,
) -> int {
    jpeg_core::ported::decompress::jdhuff::jpeg_huff_decode(
        state, get_buffer, bits_left, htbl, min_bits,
    )
}

#[no_mangle]
pub unsafe extern "C" fn jinit_input_controller(cinfo: j_decompress_ptr) {
    jpeg_core::ported::decompress::jdinput::jinit_input_controller(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_inverse_dct(cinfo: j_decompress_ptr) {
    jpeg_core::ported::decompress::jddctmgr::jinit_inverse_dct(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_marker_reader(cinfo: j_decompress_ptr) {
    jpeg_core::ported::decompress::jdmarker::jinit_marker_reader(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_resync_to_restart(cinfo: j_decompress_ptr, desired: int) -> boolean {
    jpeg_core::ported::decompress::jdmarker::jpeg_resync_to_restart(cinfo, desired)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_save_markers(
    cinfo: j_decompress_ptr,
    marker_code: int,
    length_limit: c_uint,
) {
    jpeg_core::ported::decompress::jdmarker::jpeg_save_markers(cinfo, marker_code, length_limit)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_set_marker_processor(
    cinfo: j_decompress_ptr,
    marker_code: int,
    routine: jpeg_marker_parser_method,
) {
    jpeg_core::ported::decompress::jdmarker::jpeg_set_marker_processor(cinfo, marker_code, routine)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_master_decompress(cinfo: j_decompress_ptr) {
    jpeg_core::ported::decompress::jdmaster::jinit_master_decompress(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_calc_output_dimensions(cinfo: j_decompress_ptr) {
    jpeg_core::ported::decompress::jdmaster::jpeg_calc_output_dimensions(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_core_output_dimensions(cinfo: j_decompress_ptr) {
    jpeg_core::ported::decompress::jdmaster::jpeg_core_output_dimensions(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_new_colormap(cinfo: j_decompress_ptr) {
    jpeg_core::ported::decompress::jdmaster::jpeg_new_colormap(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_merged_upsampler(cinfo: j_decompress_ptr) {
    jpeg_core::ported::decompress::jdmerge::jinit_merged_upsampler(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_upsampler(cinfo: j_decompress_ptr) {
    jpeg_core::ported::decompress::jdsample::jinit_upsampler(cinfo)
}

macro_rules! export_idct {
    ($name:ident, $module:path) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            cinfo: j_decompress_ptr,
            compptr: *mut jpeg_component_info,
            coef_block: JCOEFPTR,
            output_buf: JSAMPARRAY,
            output_col: JDIMENSION,
        ) {
            $module(cinfo, compptr, coef_block, output_buf, output_col)
        }
    };
}

export_idct!(
    jpeg_idct_1x1,
    jpeg_core::ported::decompress::jidctred::jpeg_idct_1x1
);
export_idct!(
    jpeg_idct_1x2,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_1x2
);
export_idct!(
    jpeg_idct_2x1,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_2x1
);
export_idct!(
    jpeg_idct_2x2,
    jpeg_core::ported::decompress::jidctred::jpeg_idct_2x2
);
export_idct!(
    jpeg_idct_2x4,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_2x4
);
export_idct!(
    jpeg_idct_3x3,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_3x3
);
export_idct!(
    jpeg_idct_3x6,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_3x6
);
export_idct!(
    jpeg_idct_4x2,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_4x2
);
export_idct!(
    jpeg_idct_4x4,
    jpeg_core::ported::decompress::jidctred::jpeg_idct_4x4
);
export_idct!(
    jpeg_idct_4x8,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_4x8
);
export_idct!(
    jpeg_idct_5x5,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_5x5
);
export_idct!(
    jpeg_idct_5x10,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_5x10
);
export_idct!(
    jpeg_idct_6x3,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_6x3
);
export_idct!(
    jpeg_idct_6x6,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_6x6
);
export_idct!(
    jpeg_idct_6x12,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_6x12
);
export_idct!(
    jpeg_idct_7x7,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_7x7
);
export_idct!(
    jpeg_idct_7x14,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_7x14
);
export_idct!(
    jpeg_idct_8x4,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_8x4
);
export_idct!(
    jpeg_idct_8x16,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_8x16
);
export_idct!(
    jpeg_idct_9x9,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_9x9
);
export_idct!(
    jpeg_idct_10x5,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_10x5
);
export_idct!(
    jpeg_idct_10x10,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_10x10
);
export_idct!(
    jpeg_idct_11x11,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_11x11
);
export_idct!(
    jpeg_idct_12x6,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_12x6
);
export_idct!(
    jpeg_idct_12x12,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_12x12
);
export_idct!(
    jpeg_idct_13x13,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_13x13
);
export_idct!(
    jpeg_idct_14x7,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_14x7
);
export_idct!(
    jpeg_idct_14x14,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_14x14
);
export_idct!(
    jpeg_idct_15x15,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_15x15
);
export_idct!(
    jpeg_idct_16x8,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_16x8
);
export_idct!(
    jpeg_idct_16x16,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_16x16
);
export_idct!(
    jpeg_idct_float,
    jpeg_core::ported::decompress::jidctflt::jpeg_idct_float
);
export_idct!(
    jpeg_idct_ifast,
    jpeg_core::ported::decompress::jidctfst::jpeg_idct_ifast
);
export_idct!(
    jpeg_idct_islow,
    jpeg_core::ported::decompress::jidctint::jpeg_idct_islow
);
