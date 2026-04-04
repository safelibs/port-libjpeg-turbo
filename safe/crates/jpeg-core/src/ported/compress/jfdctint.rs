pub type JLONG = ::core::ffi::c_long;
pub type DCTELEM = ::core::ffi::c_short;
pub const DCTSIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn jpeg_fdct_islow(mut data: *mut DCTELEM) {
    let mut tmp0: JLONG = 0;
    let mut tmp1: JLONG = 0;
    let mut tmp2: JLONG = 0;
    let mut tmp3: JLONG = 0;
    let mut tmp4: JLONG = 0;
    let mut tmp5: JLONG = 0;
    let mut tmp6: JLONG = 0;
    let mut tmp7: JLONG = 0;
    let mut tmp10: JLONG = 0;
    let mut tmp11: JLONG = 0;
    let mut tmp12: JLONG = 0;
    let mut tmp13: JLONG = 0;
    let mut z1: JLONG = 0;
    let mut z2: JLONG = 0;
    let mut z3: JLONG = 0;
    let mut z4: JLONG = 0;
    let mut z5: JLONG = 0;
    let mut dataptr: *mut DCTELEM = ::core::ptr::null_mut::<DCTELEM>();
    let mut ctr: ::core::ffi::c_int = 0;
    dataptr = data;
    ctr = DCTSIZE - 1 as ::core::ffi::c_int;
    while ctr >= 0 as ::core::ffi::c_int {
        tmp0 = (*dataptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *dataptr.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp7 = (*dataptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *dataptr.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp1 = (*dataptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *dataptr.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp6 = (*dataptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *dataptr.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp2 = (*dataptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *dataptr.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp5 = (*dataptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *dataptr.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp3 = (*dataptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *dataptr.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp4 = (*dataptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *dataptr.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;
        *dataptr.offset(0 as ::core::ffi::c_int as isize) =
            (((tmp10 + tmp11) as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int) as JLONG
                as DCTELEM;
        *dataptr.offset(4 as ::core::ffi::c_int as isize) =
            (((tmp10 - tmp11) as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int) as JLONG
                as DCTELEM;
        z1 = (tmp12 + tmp13) * 4433 as ::core::ffi::c_int as JLONG;
        *dataptr.offset(2 as ::core::ffi::c_int as isize) = (z1
            + tmp13 * 6270 as ::core::ffi::c_int as JLONG
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
            as DCTELEM;
        *dataptr.offset(6 as ::core::ffi::c_int as isize) = (z1
            + tmp12 * -(15137 as ::core::ffi::c_int as JLONG)
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
            as DCTELEM;
        z1 = tmp4 + tmp7;
        z2 = tmp5 + tmp6;
        z3 = tmp4 + tmp6;
        z4 = tmp5 + tmp7;
        z5 = (z3 + z4) * 9633 as ::core::ffi::c_int as JLONG;
        tmp4 = tmp4 * 2446 as ::core::ffi::c_int as JLONG;
        tmp5 = tmp5 * 16819 as ::core::ffi::c_int as JLONG;
        tmp6 = tmp6 * 25172 as ::core::ffi::c_int as JLONG;
        tmp7 = tmp7 * 12299 as ::core::ffi::c_int as JLONG;
        z1 = z1 * -(7373 as ::core::ffi::c_int as JLONG);
        z2 = z2 * -(20995 as ::core::ffi::c_int as JLONG);
        z3 = z3 * -(16069 as ::core::ffi::c_int as JLONG);
        z4 = z4 * -(3196 as ::core::ffi::c_int as JLONG);
        z3 += z5;
        z4 += z5;
        *dataptr.offset(7 as ::core::ffi::c_int as isize) = (tmp4
            + z1
            + z3
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
            as DCTELEM;
        *dataptr.offset(5 as ::core::ffi::c_int as isize) = (tmp5
            + z2
            + z4
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
            as DCTELEM;
        *dataptr.offset(3 as ::core::ffi::c_int as isize) = (tmp6
            + z2
            + z3
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
            as DCTELEM;
        *dataptr.offset(1 as ::core::ffi::c_int as isize) = (tmp7
            + z1
            + z4
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
            as DCTELEM;
        dataptr = dataptr.offset(DCTSIZE as isize);
        ctr -= 1;
    }
    dataptr = data;
    ctr = DCTSIZE - 1 as ::core::ffi::c_int;
    while ctr >= 0 as ::core::ffi::c_int {
        tmp0 = (*dataptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *dataptr.offset((DCTSIZE * 7 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp7 = (*dataptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            - *dataptr.offset((DCTSIZE * 7 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp1 = (*dataptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *dataptr.offset((DCTSIZE * 6 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp6 = (*dataptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            - *dataptr.offset((DCTSIZE * 6 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp2 = (*dataptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *dataptr.offset((DCTSIZE * 5 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp5 = (*dataptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            - *dataptr.offset((DCTSIZE * 5 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp3 = (*dataptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *dataptr.offset((DCTSIZE * 4 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp4 = (*dataptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            - *dataptr.offset((DCTSIZE * 4 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as JLONG;
        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;
        *dataptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize) = (tmp10
            + tmp11
            + ((1 as ::core::ffi::c_int as JLONG)
                << 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int)
            as DCTELEM;
        *dataptr.offset((DCTSIZE * 4 as ::core::ffi::c_int) as isize) = (tmp10 - tmp11
            + ((1 as ::core::ffi::c_int as JLONG)
                << 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int)
            as DCTELEM;
        z1 = (tmp12 + tmp13) * 4433 as ::core::ffi::c_int as JLONG;
        *dataptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize) = (z1
            + tmp13 * 6270 as ::core::ffi::c_int as JLONG
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as DCTELEM;
        *dataptr.offset((DCTSIZE * 6 as ::core::ffi::c_int) as isize) = (z1
            + tmp12 * -(15137 as ::core::ffi::c_int as JLONG)
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as DCTELEM;
        z1 = tmp4 + tmp7;
        z2 = tmp5 + tmp6;
        z3 = tmp4 + tmp6;
        z4 = tmp5 + tmp7;
        z5 = (z3 + z4) * 9633 as ::core::ffi::c_int as JLONG;
        tmp4 = tmp4 * 2446 as ::core::ffi::c_int as JLONG;
        tmp5 = tmp5 * 16819 as ::core::ffi::c_int as JLONG;
        tmp6 = tmp6 * 25172 as ::core::ffi::c_int as JLONG;
        tmp7 = tmp7 * 12299 as ::core::ffi::c_int as JLONG;
        z1 = z1 * -(7373 as ::core::ffi::c_int as JLONG);
        z2 = z2 * -(20995 as ::core::ffi::c_int as JLONG);
        z3 = z3 * -(16069 as ::core::ffi::c_int as JLONG);
        z4 = z4 * -(3196 as ::core::ffi::c_int as JLONG);
        z3 += z5;
        z4 += z5;
        *dataptr.offset((DCTSIZE * 7 as ::core::ffi::c_int) as isize) = (tmp4
            + z1
            + z3
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as DCTELEM;
        *dataptr.offset((DCTSIZE * 5 as ::core::ffi::c_int) as isize) = (tmp5
            + z2
            + z4
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as DCTELEM;
        *dataptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize) = (tmp6
            + z2
            + z3
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as DCTELEM;
        *dataptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) = (tmp7
            + z1
            + z4
            + ((1 as ::core::ffi::c_int as JLONG)
                << 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as DCTELEM;
        dataptr = dataptr.offset(1);
        ctr -= 1;
    }
}

pub const JPEG_RS_JFDCTINT_LINK_ANCHOR: unsafe extern "C" fn(*mut DCTELEM) = jpeg_fdct_islow;
