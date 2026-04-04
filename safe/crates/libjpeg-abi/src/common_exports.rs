use core::{
    ffi::{c_int, c_void},
    ptr,
};

use ffi_types::{
    j_common_ptr, j_compress_ptr, j_decompress_ptr, jpeg_error_mgr, FILE, JBLOCKROW, JDIMENSION,
    JHUFF_TBL, JOCTET, JQUANT_TBL, JSAMPARRAY,
};

pub const EXPECTED_COMMON_SYMBOLS: &[&str] = &[
    "jpeg_std_error",
    "jpeg_abort",
    "jpeg_destroy",
    "jpeg_alloc_quant_table",
    "jpeg_alloc_huff_table",
    "jcopy_sample_rows",
    "jcopy_block_row",
    "jpeg_stdio_src",
    "jpeg_stdio_dest",
    "jpeg_mem_src",
    "jpeg_mem_dest",
    "jpeg_write_icc_profile",
    "jpeg_read_icc_profile",
];

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
pub unsafe extern "C" fn jpeg_get_small(
    cinfo: j_common_ptr,
    sizeofobject: ffi_types::size_t,
) -> *mut c_void {
    jpeg_core::common::memory::jpeg_get_small(cinfo, sizeofobject)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_free_small(
    cinfo: j_common_ptr,
    object: *mut c_void,
    sizeofobject: ffi_types::size_t,
) {
    jpeg_core::common::memory::jpeg_free_small(cinfo, object, sizeofobject)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_get_large(
    cinfo: j_common_ptr,
    sizeofobject: ffi_types::size_t,
) -> *mut c_void {
    jpeg_core::common::memory::jpeg_get_large(cinfo, sizeofobject)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_free_large(
    cinfo: j_common_ptr,
    object: *mut c_void,
    sizeofobject: ffi_types::size_t,
) {
    jpeg_core::common::memory::jpeg_free_large(cinfo, object, sizeofobject)
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_mem_available(
    cinfo: j_common_ptr,
    min_bytes_needed: ffi_types::size_t,
    max_bytes_needed: ffi_types::size_t,
    already_allocated: ffi_types::size_t,
) -> ffi_types::size_t {
    jpeg_core::common::memory::jpeg_mem_available(
        cinfo,
        min_bytes_needed,
        max_bytes_needed,
        already_allocated,
    )
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
    jpeg_core::common::utils::copy_sample_rows(
        input_array,
        source_row,
        output_array,
        dest_row,
        num_rows,
        num_cols,
    )
}

#[no_mangle]
pub unsafe extern "C" fn jcopy_block_row(
    input_row: JBLOCKROW,
    output_row: JBLOCKROW,
    num_blocks: JDIMENSION,
) {
    jpeg_core::common::utils::copy_block_row(input_row, output_row, num_blocks)
}

#[no_mangle]
pub unsafe extern "C" fn jzero_far(target: *mut c_void, bytestozero: ffi_types::size_t) {
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
pub unsafe extern "C" fn jpeg_mem_src(
    cinfo: j_decompress_ptr,
    inbuffer: *const u8,
    insize: ffi_types::ulong,
) {
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
pub static jpeg_aritab: [ffi_types::JLONG; 114] = [
    0x5a1d0181, 0x2586020e, 0x11140310, 0x080b0412, 0x03d80514, 0x01da0617, 0x00e50719, 0x006f081c,
    0x0036091e, 0x001a0a21, 0x000d0b23, 0x00060c09, 0x00030d0a, 0x00010d0c, 0x5a7f0f8f, 0x3f251024,
    0x2cf21126, 0x207c1227, 0x17b91328, 0x1182142a, 0x0cef152b, 0x09a1162d, 0x072f172e, 0x055c1830,
    0x04061931, 0x03031a33, 0x02401b34, 0x01b11c36, 0x01441d38, 0x00f51e39, 0x00b71f3b, 0x008a203c,
    0x0068213e, 0x004e223f, 0x003b2320, 0x002c0921, 0x5ae125a5, 0x484c2640, 0x3a0d2741, 0x2ef12843,
    0x261f2944, 0x1f332a45, 0x19a82b46, 0x15182c48, 0x11772d49, 0x0e742e4a, 0x0bfb2f4b, 0x09f8304d,
    0x0861314e, 0x0706324f, 0x05cd3330, 0x04de3432, 0x040f3532, 0x03633633, 0x02d43734, 0x025c3835,
    0x01f83936, 0x01a43a37, 0x01603b38, 0x01253c39, 0x00f63d3a, 0x00cb3e3b, 0x00ab3f3d, 0x008f203d,
    0x5b1241c1, 0x4d044250, 0x412c4351, 0x37d84452, 0x2fe84553, 0x293c4654, 0x23794756, 0x1edf4857,
    0x1aa94957, 0x174e4a48, 0x14244b48, 0x119c4c4a, 0x0f6b4d4a, 0x0d514e4b, 0x0bb64f4d, 0x0a40304d,
    0x583251d0, 0x4d1c5258, 0x438e5359, 0x3bdd545a, 0x34ee555b, 0x2eae565c, 0x299a575d, 0x25164756,
    0x557059d8, 0x4ca95a5f, 0x44d95b60, 0x3e225c61, 0x38245d63, 0x32b45e63, 0x2e17565d, 0x56a860df,
    0x4f466165, 0x47e56266, 0x41cf6367, 0x3c3d6468, 0x375e5d63, 0x52316669, 0x4c0f676a, 0x4639686b,
    0x415e6367, 0x56276ae9, 0x50e76b6c, 0x4b85676d, 0x55976d6e, 0x504f6b6f, 0x5a106fee, 0x55226d70,
    0x59eb6ff0, 0x5a1d7171,
];

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
