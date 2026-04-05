use core::ptr;

use ffi_types::{int, long, size_t, DCTSIZE2, JBLOCK, JBLOCKROW, JDIMENSION, JSAMPARRAY, JSAMPLE};

#[no_mangle]
pub static jpeg_natural_order: [int; DCTSIZE2 + 16] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59,
    52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63,
];

#[no_mangle]
pub static jpeg_natural_order2: [int; DCTSIZE2 + 16] = [
    0, 1, 8, 9, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63,
];

#[no_mangle]
pub static jpeg_natural_order3: [int; DCTSIZE2 + 16] = [
    0, 1, 8, 16, 9, 2, 10, 17, 18, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63,
];

#[no_mangle]
pub static jpeg_natural_order4: [int; DCTSIZE2 + 16] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 25, 18, 11, 19, 26, 27, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63,
];

#[no_mangle]
pub static jpeg_natural_order5: [int; DCTSIZE2 + 16] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 12, 19, 26, 33, 34, 27, 20, 28, 35, 36,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63,
];

#[no_mangle]
pub static jpeg_natural_order6: [int; DCTSIZE2 + 16] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 41, 34, 27, 20, 13,
    21, 28, 35, 42, 43, 36, 29, 37, 44, 45, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63,
];

#[no_mangle]
pub static jpeg_natural_order7: [int; DCTSIZE2 + 16] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 14, 21, 28, 35, 42, 49, 50, 43, 36, 29, 22, 30, 37, 44, 51, 52, 45, 38, 46, 53, 54, 63,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63,
];

#[inline]
pub unsafe fn div_round_up(a: long, b: long) -> long {
    (a + b - 1) / b
}

#[inline]
pub unsafe fn round_up(a: long, b: long) -> long {
    let adjusted = a + b - 1;
    adjusted - (adjusted % b)
}

pub unsafe fn copy_sample_rows(
    input_array: JSAMPARRAY,
    source_row: int,
    output_array: JSAMPARRAY,
    dest_row: int,
    num_rows: int,
    num_cols: JDIMENSION,
) {
    let mut input = input_array.add(source_row as usize);
    let mut output = output_array.add(dest_row as usize);
    let count = num_cols as usize * core::mem::size_of::<JSAMPLE>();
    let mut row = num_rows;
    while row > 0 {
        // Upstream allows row duplication, so the individual row copies must
        // tolerate aliasing within the caller-provided sample arrays.
        ptr::copy(*input, *output, count);
        input = input.add(1);
        output = output.add(1);
        row -= 1;
    }
}

pub unsafe fn copy_block_row(input_row: JBLOCKROW, output_row: JBLOCKROW, num_blocks: JDIMENSION) {
    ptr::copy_nonoverlapping(
        input_row as *const u8,
        output_row as *mut u8,
        num_blocks as usize * core::mem::size_of::<JBLOCK>(),
    );
}

pub unsafe fn zero_far(target: *mut core::ffi::c_void, bytestozero: size_t) {
    ptr::write_bytes(target, 0, bytestozero);
}
