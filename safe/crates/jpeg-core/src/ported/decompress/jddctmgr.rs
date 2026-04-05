use core::{mem::size_of, ptr};

use ffi_types::{
    j_common_ptr, j_decompress_ptr, jpeg_component_info, jpeg_inverse_dct, DCTSIZE, DCTSIZE2,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JPOOL_IMAGE, JQUANT_TBL, J_MESSAGE_CODE, MAX_COMPONENTS,
};

use crate::{
    common::error,
    ported::decompress::{jidctflt, jidctfst, jidctint, jidctred},
};

type ISLOW_MULT_TYPE = i16;
type IFAST_MULT_TYPE = i16;
type FLOAT_MULT_TYPE = f32;

const IFAST_SCALE_BITS: i32 = 2;

#[repr(C)]
pub struct my_idct_controller {
    pub pub_: jpeg_inverse_dct,
    pub cur_method: [i32; MAX_COMPONENTS],
}

#[repr(C)]
union multiplier_table {
    islow_array: [ISLOW_MULT_TYPE; DCTSIZE2],
    ifast_array: [IFAST_MULT_TYPE; DCTSIZE2],
    float_array: [FLOAT_MULT_TYPE; DCTSIZE2],
}

type InverseDctFn = unsafe extern "C" fn(
    j_decompress_ptr,
    *mut jpeg_component_info,
    ffi_types::JCOEFPTR,
    ffi_types::JSAMPARRAY,
    ffi_types::JDIMENSION,
);

#[inline]
fn descale(x: i64, n: i32) -> i64 {
    (x + (1_i64 << (n - 1))) >> n
}

#[inline]
unsafe fn select_inverse_dct(cinfo: j_decompress_ptr, scaled_size: i32) -> (i32, InverseDctFn) {
    match scaled_size {
        1 => (JDCT_ISLOW, jidctred::jpeg_idct_1x1),
        2 => (JDCT_ISLOW, jidctred::jpeg_idct_2x2),
        3 => (JDCT_ISLOW, jidctint::jpeg_idct_3x3),
        4 => (JDCT_ISLOW, jidctred::jpeg_idct_4x4),
        5 => (JDCT_ISLOW, jidctint::jpeg_idct_5x5),
        6 => (JDCT_ISLOW, jidctint::jpeg_idct_6x6),
        7 => (JDCT_ISLOW, jidctint::jpeg_idct_7x7),
        8 => match (*cinfo).dct_method {
            JDCT_IFAST => (JDCT_IFAST, jidctfst::jpeg_idct_ifast),
            JDCT_FLOAT => (JDCT_FLOAT, jidctflt::jpeg_idct_float),
            _ => (JDCT_ISLOW, jidctint::jpeg_idct_islow),
        },
        9 => (JDCT_ISLOW, jidctint::jpeg_idct_9x9),
        10 => (JDCT_ISLOW, jidctint::jpeg_idct_10x10),
        11 => (JDCT_ISLOW, jidctint::jpeg_idct_11x11),
        12 => (JDCT_ISLOW, jidctint::jpeg_idct_12x12),
        13 => (JDCT_ISLOW, jidctint::jpeg_idct_13x13),
        14 => (JDCT_ISLOW, jidctint::jpeg_idct_14x14),
        15 => (JDCT_ISLOW, jidctint::jpeg_idct_15x15),
        16 => (JDCT_ISLOW, jidctint::jpeg_idct_16x16),
        _ => error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_DCTSIZE,
            scaled_size,
        ),
    }
}

unsafe extern "C" fn start_pass(cinfo: j_decompress_ptr) {
    static AAN_SCALES: [i16; DCTSIZE2] = [
        16384, 22725, 21407, 19266, 16384, 12873, 8867, 4520, 22725, 31521, 29692, 26722, 22725,
        17855, 12299, 6270, 21407, 29692, 27969, 25172, 21407, 16819, 11585, 5906, 19266, 26722,
        25172, 22654, 19266, 15137, 10426, 5315, 16384, 22725, 21407, 19266, 16384, 12873, 8867,
        4520, 12873, 17855, 16819, 15137, 12873, 10114, 6967, 3552, 8867, 12299, 11585, 10426,
        8867, 6967, 4799, 2446, 4520, 6270, 5906, 5315, 4520, 3552, 2446, 1247,
    ];
    static AAN_SCALE_FACTOR: [f64; DCTSIZE] = [
        1.0,
        1.387_039_845,
        1.306_562_965,
        1.175_875_602,
        1.0,
        0.785_694_958,
        0.541_196_1,
        0.275_899_379,
    ];

    let idct = (*cinfo).idct as *mut my_idct_controller;

    for ci in 0..(*cinfo).num_components as usize {
        let compptr = (*cinfo).comp_info.add(ci);
        let scaled_size = (*compptr).DCT_h_scaled_size;
        let (method, method_ptr) = select_inverse_dct(cinfo, scaled_size);
        (*idct).pub_.inverse_DCT[ci] = Some(method_ptr);

        if (*compptr).component_needed == 0 || (*idct).cur_method[ci] == method {
            continue;
        }

        let qtbl = (*compptr).quant_table;
        if qtbl.is_null() {
            continue;
        }

        (*idct).cur_method[ci] = method;
        match method {
            JDCT_ISLOW => {
                let ismtbl = (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
                for i in 0..DCTSIZE2 {
                    *ismtbl.add(i) = (*qtbl).quantval[i] as ISLOW_MULT_TYPE;
                }
            }
            JDCT_IFAST => {
                let ifmtbl = (*compptr).dct_table as *mut IFAST_MULT_TYPE;
                for i in 0..DCTSIZE2 {
                    *ifmtbl.add(i) = descale(
                        ((*qtbl).quantval[i] as i64) * (AAN_SCALES[i] as i64),
                        14 - IFAST_SCALE_BITS,
                    ) as IFAST_MULT_TYPE;
                }
            }
            JDCT_FLOAT => {
                let fmtbl = (*compptr).dct_table as *mut FLOAT_MULT_TYPE;
                let mut i = 0;
                for row in 0..DCTSIZE {
                    for col in 0..DCTSIZE {
                        *fmtbl.add(i) = (*qtbl).quantval[i] as FLOAT_MULT_TYPE
                            * AAN_SCALE_FACTOR[row] as FLOAT_MULT_TYPE
                            * AAN_SCALE_FACTOR[col] as FLOAT_MULT_TYPE;
                        i += 1;
                    }
                }
            }
            _ => error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_NOT_COMPILED),
        }
    }
}

pub unsafe fn jinit_inverse_dct(cinfo: j_decompress_ptr) {
    let alloc_small = (*(*cinfo).mem).alloc_small.unwrap();
    let idct = alloc_small(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        size_of::<my_idct_controller>(),
    ) as *mut my_idct_controller;
    ptr::write_bytes(idct, 0, 1);
    (*cinfo).idct = &mut (*idct).pub_;
    (*idct).pub_.start_pass = Some(start_pass);

    for ci in 0..(*cinfo).num_components as usize {
        let compptr = (*cinfo).comp_info.add(ci);
        (*compptr).dct_table = alloc_small(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            size_of::<multiplier_table>(),
        );
        ptr::write_bytes(
            (*compptr).dct_table as *mut u8,
            0,
            size_of::<multiplier_table>(),
        );
        (*idct).cur_method[ci] = -1;
    }
}
