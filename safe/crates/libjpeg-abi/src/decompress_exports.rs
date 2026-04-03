use ffi_types::{
    boolean, int, j_decompress_ptr, JSAMPARRAY, JSAMPIMAGE, JDIMENSION,
};

#[no_mangle]
pub unsafe extern "C" fn jpeg_CreateDecompress(
    cinfo: j_decompress_ptr,
    version: int,
    structsize: usize,
) {
    jpeg_core::ported::decompress::jdapimin::jpeg_CreateDecompress(cinfo, version, structsize)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_destroy_decompress(cinfo: j_decompress_ptr) {
    jpeg_core::ported::decompress::jdapimin::jpeg_destroy_decompress(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_abort_decompress(cinfo: j_decompress_ptr) {
    jpeg_core::ported::decompress::jdapimin::jpeg_abort_decompress(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_read_header(cinfo: j_decompress_ptr, require_image: boolean) -> int {
    jpeg_core::ported::decompress::jdapimin::jpeg_read_header(cinfo, require_image)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_consume_input(cinfo: j_decompress_ptr) -> int {
    jpeg_core::ported::decompress::jdapimin::jpeg_consume_input(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_input_complete(cinfo: j_decompress_ptr) -> boolean {
    jpeg_core::ported::decompress::jdapimin::jpeg_input_complete(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_has_multiple_scans(cinfo: j_decompress_ptr) -> boolean {
    jpeg_core::ported::decompress::jdapimin::jpeg_has_multiple_scans(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean {
    jpeg_core::ported::decompress::jdapimin::jpeg_finish_decompress(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_start_decompress(cinfo: j_decompress_ptr) -> boolean {
    jpeg_core::ported::decompress::jdapistd::jpeg_start_decompress(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_read_scanlines(
    cinfo: j_decompress_ptr,
    scanlines: JSAMPARRAY,
    max_lines: JDIMENSION,
) -> JDIMENSION {
    jpeg_core::ported::decompress::jdapistd::jpeg_read_scanlines(cinfo, scanlines, max_lines)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_crop_scanline(
    cinfo: j_decompress_ptr,
    xoffset: *mut JDIMENSION,
    width: *mut JDIMENSION,
) {
    jpeg_core::ported::decompress::jdapistd::jpeg_crop_scanline(cinfo, xoffset, width)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_skip_scanlines(
    cinfo: j_decompress_ptr,
    num_lines: JDIMENSION,
) -> JDIMENSION {
    jpeg_core::ported::decompress::jdapistd::jpeg_skip_scanlines(cinfo, num_lines)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_read_raw_data(
    cinfo: j_decompress_ptr,
    data: JSAMPIMAGE,
    max_lines: JDIMENSION,
) -> JDIMENSION {
    jpeg_core::ported::decompress::jdapistd::jpeg_read_raw_data(cinfo, data, max_lines)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_start_output(cinfo: j_decompress_ptr, scan_number: int) -> boolean {
    jpeg_core::ported::decompress::jdapistd::jpeg_start_output(cinfo, scan_number)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_finish_output(cinfo: j_decompress_ptr) -> boolean {
    jpeg_core::ported::decompress::jdapistd::jpeg_finish_output(cinfo)
}
