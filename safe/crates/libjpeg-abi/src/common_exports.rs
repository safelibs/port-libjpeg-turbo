use core::ffi::c_void;

use ffi_types::{
    j_common_ptr, j_compress_ptr, j_decompress_ptr, jpeg_error_mgr, JHUFF_TBL, JQUANT_TBL,
    JSAMPARRAY, JBLOCKROW, JDIMENSION, FILE, JOCTET,
};

#[no_mangle]
pub unsafe extern "C" fn jpeg_std_error(err: *mut jpeg_error_mgr) -> *mut jpeg_error_mgr {
    jpeg_core::common::error::std_error(err)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_abort(cinfo: j_common_ptr) {
    jpeg_core::common::error::abort(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_destroy(cinfo: j_common_ptr) {
    jpeg_core::common::error::destroy(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_alloc_quant_table(cinfo: j_common_ptr) -> *mut JQUANT_TBL {
    jpeg_core::common::error::alloc_quant_table(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_alloc_huff_table(cinfo: j_common_ptr) -> *mut JHUFF_TBL {
    jpeg_core::common::error::alloc_huff_table(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_get_small(cinfo: j_common_ptr, sizeofobject: usize) -> *mut c_void {
    jpeg_core::common::memory::jpeg_get_small(cinfo, sizeofobject)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_free_small(cinfo: j_common_ptr, object: *mut c_void, sizeofobject: usize) {
    jpeg_core::common::memory::jpeg_free_small(cinfo, object, sizeofobject)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_get_large(cinfo: j_common_ptr, sizeofobject: usize) -> *mut c_void {
    jpeg_core::common::memory::jpeg_get_large(cinfo, sizeofobject)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_free_large(cinfo: j_common_ptr, object: *mut c_void, sizeofobject: usize) {
    jpeg_core::common::memory::jpeg_free_large(cinfo, object, sizeofobject)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_mem_available(
    cinfo: j_common_ptr,
    min_bytes_needed: usize,
    max_bytes_needed: usize,
    already_allocated: usize,
) -> usize {
    jpeg_core::common::memory::jpeg_mem_available(cinfo, min_bytes_needed, max_bytes_needed, already_allocated)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_open_backing_store(
    cinfo: j_common_ptr,
    info: ffi_types::backing_store_ptr,
    total_bytes_needed: ffi_types::long,
) {
    jpeg_core::common::memory::jpeg_open_backing_store(cinfo, info, total_bytes_needed)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_mem_init(cinfo: j_common_ptr) -> ffi_types::long {
    jpeg_core::common::memory::jpeg_mem_init(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_mem_term(cinfo: j_common_ptr) {
    jpeg_core::common::memory::jpeg_mem_term(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jinit_memory_mgr(cinfo: j_common_ptr) {
    jpeg_core::common::memory::jinit_memory_mgr(cinfo)
}

#[no_mangle]
pub unsafe extern "C" fn jdiv_round_up(a: ffi_types::long, b: ffi_types::long) -> ffi_types::long {
    jpeg_core::common::utils::div_round_up(a, b)
}

#[no_mangle]
pub unsafe extern "C" fn jround_up(a: ffi_types::long, b: ffi_types::long) -> ffi_types::long {
    jpeg_core::common::utils::round_up(a, b)
}

#[no_mangle]
pub unsafe extern "C" fn jcopy_sample_rows(
    input_array: JSAMPARRAY,
    source_row: ffi_types::int,
    output_array: JSAMPARRAY,
    dest_row: ffi_types::int,
    num_rows: ffi_types::int,
    num_cols: JDIMENSION,
) {
    jpeg_core::common::utils::copy_sample_rows(input_array, source_row, output_array, dest_row, num_rows, num_cols)
}

#[no_mangle]
pub unsafe extern "C" fn jcopy_block_row(input_row: JBLOCKROW, output_row: JBLOCKROW, num_blocks: JDIMENSION) {
    jpeg_core::common::utils::copy_block_row(input_row, output_row, num_blocks)
}

#[no_mangle]
pub unsafe extern "C" fn jzero_far(target: *mut c_void, bytestozero: usize) {
    jpeg_core::common::utils::zero_far(target, bytestozero)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_stdio_src(cinfo: j_decompress_ptr, infile: *mut FILE) {
    jpeg_core::common::source_dest::jpeg_stdio_src(cinfo, infile)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_stdio_dest(cinfo: j_compress_ptr, outfile: *mut FILE) {
    jpeg_core::common::source_dest::jpeg_stdio_dest(cinfo, outfile)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_mem_src(cinfo: j_decompress_ptr, inbuffer: *const u8, insize: ffi_types::ulong) {
    jpeg_core::common::source_dest::jpeg_mem_src(cinfo, inbuffer, insize)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_mem_dest(
    cinfo: j_compress_ptr,
    outbuffer: *mut *mut u8,
    outsize: *mut ffi_types::ulong,
) {
    jpeg_core::common::source_dest::jpeg_mem_dest(cinfo, outbuffer, outsize)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_write_icc_profile(
    cinfo: j_compress_ptr,
    icc_data_ptr: *const JOCTET,
    icc_data_len: ::core::ffi::c_uint,
) {
    jpeg_core::common::icc::jpeg_write_icc_profile(cinfo, icc_data_ptr, icc_data_len)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_read_icc_profile(
    cinfo: j_decompress_ptr,
    icc_data_ptr: *mut *mut JOCTET,
    icc_data_len: *mut ::core::ffi::c_uint,
) -> ffi_types::boolean {
    jpeg_core::common::icc::jpeg_read_icc_profile(cinfo, icc_data_ptr, icc_data_len)
}
