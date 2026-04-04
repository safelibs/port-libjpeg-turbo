use core::{
    ffi::{c_int, c_void},
    ptr,
};

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

type DctElem = ::core::ffi::c_short;

const JPEG_COMPAT_MAX_BLOCK: usize = 16;

#[no_mangle]
pub static mut auxv: *mut c_void = ptr::null_mut();

#[no_mangle]
pub extern "C" fn libjpeg_general_init() -> c_int {
    0
}

unsafe fn jpeg_fdct_rect_bridge(data: *mut DctElem, width: usize, height: usize) {
    let mut tmp = [0 as DctElem; ffi_types::DCTSIZE2];
    let copy_width = width.min(ffi_types::DCTSIZE);
    let copy_height = height.min(ffi_types::DCTSIZE);

    debug_assert!(width <= JPEG_COMPAT_MAX_BLOCK);
    debug_assert!(height <= JPEG_COMPAT_MAX_BLOCK);

    for row in 0..copy_height {
        ptr::copy_nonoverlapping(
            data.add(row * width),
            tmp.as_mut_ptr().add(row * ffi_types::DCTSIZE),
            copy_width,
        );
    }

    jpeg_core::ported::compress::jfdctint::jpeg_fdct_islow(tmp.as_mut_ptr());
    ptr::write_bytes(data, 0, width * height);

    for row in 0..copy_height {
        ptr::copy_nonoverlapping(
            tmp.as_ptr().add(row * ffi_types::DCTSIZE),
            data.add(row * width),
            copy_width,
        );
    }
}

macro_rules! export_fdct_rect {
    ($name:ident, $width:expr, $height:expr) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(data: *mut DctElem) {
            jpeg_fdct_rect_bridge(data, $width, $height)
        }
    };
}

export_fdct_rect!(jpeg_fdct_1x1, 1, 1);
export_fdct_rect!(jpeg_fdct_1x2, 1, 2);
export_fdct_rect!(jpeg_fdct_2x1, 2, 1);
export_fdct_rect!(jpeg_fdct_2x2, 2, 2);
export_fdct_rect!(jpeg_fdct_2x4, 2, 4);
export_fdct_rect!(jpeg_fdct_3x3, 3, 3);
export_fdct_rect!(jpeg_fdct_3x6, 3, 6);
export_fdct_rect!(jpeg_fdct_4x2, 4, 2);
export_fdct_rect!(jpeg_fdct_4x4, 4, 4);
export_fdct_rect!(jpeg_fdct_4x8, 4, 8);
export_fdct_rect!(jpeg_fdct_5x5, 5, 5);
export_fdct_rect!(jpeg_fdct_5x10, 5, 10);
export_fdct_rect!(jpeg_fdct_6x3, 6, 3);
export_fdct_rect!(jpeg_fdct_6x6, 6, 6);
export_fdct_rect!(jpeg_fdct_6x12, 6, 12);
export_fdct_rect!(jpeg_fdct_7x7, 7, 7);
export_fdct_rect!(jpeg_fdct_7x14, 7, 14);
export_fdct_rect!(jpeg_fdct_8x4, 8, 4);
export_fdct_rect!(jpeg_fdct_8x16, 8, 16);
export_fdct_rect!(jpeg_fdct_9x9, 9, 9);
export_fdct_rect!(jpeg_fdct_10x5, 10, 5);
export_fdct_rect!(jpeg_fdct_10x10, 10, 10);
export_fdct_rect!(jpeg_fdct_11x11, 11, 11);
export_fdct_rect!(jpeg_fdct_12x6, 12, 6);
export_fdct_rect!(jpeg_fdct_12x12, 12, 12);
export_fdct_rect!(jpeg_fdct_13x13, 13, 13);
export_fdct_rect!(jpeg_fdct_14x7, 14, 7);
export_fdct_rect!(jpeg_fdct_14x14, 14, 14);
export_fdct_rect!(jpeg_fdct_15x15, 15, 15);
export_fdct_rect!(jpeg_fdct_16x8, 16, 8);
export_fdct_rect!(jpeg_fdct_16x16, 16, 16);
