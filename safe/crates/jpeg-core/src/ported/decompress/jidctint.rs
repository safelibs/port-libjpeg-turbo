use core::ptr;

use ffi_types::{j_decompress_ptr, jpeg_component_info, JCOEFPTR, JDIMENSION, JSAMPARRAY, JSAMPLE};

use crate::ported::decompress::jidctred;

#[allow(
    dead_code,
    improper_ctypes,
    improper_ctypes_definitions,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    clippy::all
)]
mod translated {
    include!("generated/jidctint_translated.rs");
}

#[inline]
unsafe fn translated_cinfo(cinfo: j_decompress_ptr) -> *mut translated::jpeg_decompress_struct {
    cinfo.cast::<translated::jpeg_decompress_struct>()
}

#[inline]
unsafe fn translated_compptr(
    compptr: *mut jpeg_component_info,
) -> *mut translated::jpeg_component_info {
    compptr.cast::<translated::jpeg_component_info>()
}

#[inline]
unsafe fn translated_coef_block(coef_block: JCOEFPTR) -> *mut translated::JCOEF {
    coef_block.cast::<translated::JCOEF>()
}

#[inline]
unsafe fn translated_output_buf(output_buf: JSAMPARRAY) -> *mut translated::JSAMPROW {
    output_buf.cast::<translated::JSAMPROW>()
}

macro_rules! square_idct_wrapper {
    ($rust_name:ident) => {
        pub unsafe extern "C" fn $rust_name(
            cinfo: j_decompress_ptr,
            compptr: *mut jpeg_component_info,
            coef_block: JCOEFPTR,
            output_buf: JSAMPARRAY,
            output_col: JDIMENSION,
        ) {
            translated::$rust_name(
                translated_cinfo(cinfo),
                translated_compptr(compptr),
                translated_coef_block(coef_block),
                translated_output_buf(output_buf),
                output_col,
            )
        }
    };
}

square_idct_wrapper!(jpeg_idct_islow);
square_idct_wrapper!(jpeg_idct_3x3);
square_idct_wrapper!(jpeg_idct_5x5);
square_idct_wrapper!(jpeg_idct_6x6);
square_idct_wrapper!(jpeg_idct_7x7);
square_idct_wrapper!(jpeg_idct_9x9);
square_idct_wrapper!(jpeg_idct_10x10);
square_idct_wrapper!(jpeg_idct_11x11);
square_idct_wrapper!(jpeg_idct_12x12);
square_idct_wrapper!(jpeg_idct_13x13);
square_idct_wrapper!(jpeg_idct_14x14);
square_idct_wrapper!(jpeg_idct_15x15);
square_idct_wrapper!(jpeg_idct_16x16);

type InverseDctFn = unsafe extern "C" fn(
    cinfo: j_decompress_ptr,
    compptr: *mut jpeg_component_info,
    coef_block: JCOEFPTR,
    output_buf: JSAMPARRAY,
    output_col: JDIMENSION,
);

fn square_idct_for_size(size: JDIMENSION) -> InverseDctFn {
    match size {
        1 => jidctred::jpeg_idct_1x1,
        2 => jidctred::jpeg_idct_2x2,
        3 => jpeg_idct_3x3,
        4 => jidctred::jpeg_idct_4x4,
        5 => jpeg_idct_5x5,
        6 => jpeg_idct_6x6,
        7 => jpeg_idct_7x7,
        9 => jpeg_idct_9x9,
        10 => jpeg_idct_10x10,
        11 => jpeg_idct_11x11,
        12 => jpeg_idct_12x12,
        13 => jpeg_idct_13x13,
        14 => jpeg_idct_14x14,
        15 => jpeg_idct_15x15,
        16 => jpeg_idct_16x16,
        _ => jpeg_idct_islow,
    }
}

unsafe fn jpeg_idct_rect_bridge(
    cinfo: j_decompress_ptr,
    compptr: *mut jpeg_component_info,
    coef_block: JCOEFPTR,
    output_buf: JSAMPARRAY,
    output_col: JDIMENSION,
    output_width: JDIMENSION,
    output_height: JDIMENSION,
) {
    let square_size = output_width.max(output_height);
    let square_idct = square_idct_for_size(square_size);
    let mut workspace = [[0 as JSAMPLE; 16]; 16];
    let mut row_pointers = [ptr::null_mut(); 16];

    for (row_index, row) in workspace.iter_mut().enumerate() {
        row_pointers[row_index] = row.as_mut_ptr();
    }

    square_idct(cinfo, compptr, coef_block, row_pointers.as_mut_ptr(), 0);

    for row in 0..output_height as usize {
        let output_row = *output_buf.add(row);
        ptr::copy_nonoverlapping(
            workspace[row].as_ptr(),
            output_row.add(output_col as usize),
            output_width as usize,
        );
    }
}

macro_rules! rect_idct_wrapper {
    ($rust_name:ident, $width:expr, $height:expr) => {
        pub unsafe extern "C" fn $rust_name(
            cinfo: j_decompress_ptr,
            compptr: *mut jpeg_component_info,
            coef_block: JCOEFPTR,
            output_buf: JSAMPARRAY,
            output_col: JDIMENSION,
        ) {
            jpeg_idct_rect_bridge(
                cinfo, compptr, coef_block, output_buf, output_col, $width, $height,
            )
        }
    };
}

rect_idct_wrapper!(jpeg_idct_1x2, 1, 2);
rect_idct_wrapper!(jpeg_idct_2x1, 2, 1);
rect_idct_wrapper!(jpeg_idct_2x4, 2, 4);
rect_idct_wrapper!(jpeg_idct_3x6, 3, 6);
rect_idct_wrapper!(jpeg_idct_4x2, 4, 2);
rect_idct_wrapper!(jpeg_idct_4x8, 4, 8);
rect_idct_wrapper!(jpeg_idct_5x10, 5, 10);
rect_idct_wrapper!(jpeg_idct_6x3, 6, 3);
rect_idct_wrapper!(jpeg_idct_6x12, 6, 12);
rect_idct_wrapper!(jpeg_idct_7x14, 7, 14);
rect_idct_wrapper!(jpeg_idct_8x4, 8, 4);
rect_idct_wrapper!(jpeg_idct_8x16, 8, 16);
rect_idct_wrapper!(jpeg_idct_10x5, 10, 5);
rect_idct_wrapper!(jpeg_idct_12x6, 12, 6);
rect_idct_wrapper!(jpeg_idct_14x7, 14, 7);
rect_idct_wrapper!(jpeg_idct_16x8, 16, 8);
