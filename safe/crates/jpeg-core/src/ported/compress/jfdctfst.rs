pub type JLONG = ::core::ffi::c_long;
pub type DCTELEM = ::core::ffi::c_short;
pub const DCTSIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn jpeg_fdct_ifast(mut data: *mut DCTELEM) {
    let mut tmp0: DCTELEM = 0;
    let mut tmp1: DCTELEM = 0;
    let mut tmp2: DCTELEM = 0;
    let mut tmp3: DCTELEM = 0;
    let mut tmp4: DCTELEM = 0;
    let mut tmp5: DCTELEM = 0;
    let mut tmp6: DCTELEM = 0;
    let mut tmp7: DCTELEM = 0;
    let mut tmp10: DCTELEM = 0;
    let mut tmp11: DCTELEM = 0;
    let mut tmp12: DCTELEM = 0;
    let mut tmp13: DCTELEM = 0;
    let mut z1: DCTELEM = 0;
    let mut z2: DCTELEM = 0;
    let mut z3: DCTELEM = 0;
    let mut z4: DCTELEM = 0;
    let mut z5: DCTELEM = 0;
    let mut z11: DCTELEM = 0;
    let mut z13: DCTELEM = 0;
    let mut dataptr: *mut DCTELEM = ::core::ptr::null_mut::<DCTELEM>();
    let mut ctr: ::core::ffi::c_int = 0;
    dataptr = data;
    ctr = DCTSIZE - 1 as ::core::ffi::c_int;
    while ctr >= 0 as ::core::ffi::c_int {
        tmp0 = (*dataptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *dataptr.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp7 = (*dataptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *dataptr.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp1 = (*dataptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *dataptr.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp6 = (*dataptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *dataptr.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp2 = (*dataptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *dataptr.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp5 = (*dataptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *dataptr.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp3 = (*dataptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *dataptr.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp4 = (*dataptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *dataptr.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp10 = (tmp0 as ::core::ffi::c_int + tmp3 as ::core::ffi::c_int) as DCTELEM;
        tmp13 = (tmp0 as ::core::ffi::c_int - tmp3 as ::core::ffi::c_int) as DCTELEM;
        tmp11 = (tmp1 as ::core::ffi::c_int + tmp2 as ::core::ffi::c_int) as DCTELEM;
        tmp12 = (tmp1 as ::core::ffi::c_int - tmp2 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset(0 as ::core::ffi::c_int as isize) =
            (tmp10 as ::core::ffi::c_int + tmp11 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset(4 as ::core::ffi::c_int as isize) =
            (tmp10 as ::core::ffi::c_int - tmp11 as ::core::ffi::c_int) as DCTELEM;
        z1 = ((tmp12 as ::core::ffi::c_int + tmp13 as ::core::ffi::c_int) as JLONG
            * 181 as ::core::ffi::c_int as JLONG
            >> 8 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset(2 as ::core::ffi::c_int as isize) =
            (tmp13 as ::core::ffi::c_int + z1 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset(6 as ::core::ffi::c_int as isize) =
            (tmp13 as ::core::ffi::c_int - z1 as ::core::ffi::c_int) as DCTELEM;
        tmp10 = (tmp4 as ::core::ffi::c_int + tmp5 as ::core::ffi::c_int) as DCTELEM;
        tmp11 = (tmp5 as ::core::ffi::c_int + tmp6 as ::core::ffi::c_int) as DCTELEM;
        tmp12 = (tmp6 as ::core::ffi::c_int + tmp7 as ::core::ffi::c_int) as DCTELEM;
        z5 = ((tmp10 as ::core::ffi::c_int - tmp12 as ::core::ffi::c_int) as JLONG
            * 98 as ::core::ffi::c_int as JLONG
            >> 8 as ::core::ffi::c_int) as DCTELEM;
        z2 = ((tmp10 as JLONG * 139 as ::core::ffi::c_int as JLONG >> 8 as ::core::ffi::c_int)
            as DCTELEM as ::core::ffi::c_int
            + z5 as ::core::ffi::c_int) as DCTELEM;
        z4 = ((tmp12 as JLONG * 334 as ::core::ffi::c_int as JLONG >> 8 as ::core::ffi::c_int)
            as DCTELEM as ::core::ffi::c_int
            + z5 as ::core::ffi::c_int) as DCTELEM;
        z3 = (tmp11 as JLONG * 181 as ::core::ffi::c_int as JLONG >> 8 as ::core::ffi::c_int)
            as DCTELEM;
        z11 = (tmp7 as ::core::ffi::c_int + z3 as ::core::ffi::c_int) as DCTELEM;
        z13 = (tmp7 as ::core::ffi::c_int - z3 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset(5 as ::core::ffi::c_int as isize) =
            (z13 as ::core::ffi::c_int + z2 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset(3 as ::core::ffi::c_int as isize) =
            (z13 as ::core::ffi::c_int - z2 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset(1 as ::core::ffi::c_int as isize) =
            (z11 as ::core::ffi::c_int + z4 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset(7 as ::core::ffi::c_int as isize) =
            (z11 as ::core::ffi::c_int - z4 as ::core::ffi::c_int) as DCTELEM;
        dataptr = dataptr.offset(DCTSIZE as isize);
        ctr -= 1;
    }
    dataptr = data;
    ctr = DCTSIZE - 1 as ::core::ffi::c_int;
    while ctr >= 0 as ::core::ffi::c_int {
        tmp0 = (*dataptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *dataptr.offset((DCTSIZE * 7 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp7 = (*dataptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            - *dataptr.offset((DCTSIZE * 7 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp1 = (*dataptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *dataptr.offset((DCTSIZE * 6 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp6 = (*dataptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            - *dataptr.offset((DCTSIZE * 6 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp2 = (*dataptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *dataptr.offset((DCTSIZE * 5 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp5 = (*dataptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            - *dataptr.offset((DCTSIZE * 5 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp3 = (*dataptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            + *dataptr.offset((DCTSIZE * 4 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp4 = (*dataptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            - *dataptr.offset((DCTSIZE * 4 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            as DCTELEM;
        tmp10 = (tmp0 as ::core::ffi::c_int + tmp3 as ::core::ffi::c_int) as DCTELEM;
        tmp13 = (tmp0 as ::core::ffi::c_int - tmp3 as ::core::ffi::c_int) as DCTELEM;
        tmp11 = (tmp1 as ::core::ffi::c_int + tmp2 as ::core::ffi::c_int) as DCTELEM;
        tmp12 = (tmp1 as ::core::ffi::c_int - tmp2 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize) =
            (tmp10 as ::core::ffi::c_int + tmp11 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 4 as ::core::ffi::c_int) as isize) =
            (tmp10 as ::core::ffi::c_int - tmp11 as ::core::ffi::c_int) as DCTELEM;
        z1 = ((tmp12 as ::core::ffi::c_int + tmp13 as ::core::ffi::c_int) as JLONG
            * 181 as ::core::ffi::c_int as JLONG
            >> 8 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize) =
            (tmp13 as ::core::ffi::c_int + z1 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 6 as ::core::ffi::c_int) as isize) =
            (tmp13 as ::core::ffi::c_int - z1 as ::core::ffi::c_int) as DCTELEM;
        tmp10 = (tmp4 as ::core::ffi::c_int + tmp5 as ::core::ffi::c_int) as DCTELEM;
        tmp11 = (tmp5 as ::core::ffi::c_int + tmp6 as ::core::ffi::c_int) as DCTELEM;
        tmp12 = (tmp6 as ::core::ffi::c_int + tmp7 as ::core::ffi::c_int) as DCTELEM;
        z5 = ((tmp10 as ::core::ffi::c_int - tmp12 as ::core::ffi::c_int) as JLONG
            * 98 as ::core::ffi::c_int as JLONG
            >> 8 as ::core::ffi::c_int) as DCTELEM;
        z2 = ((tmp10 as JLONG * 139 as ::core::ffi::c_int as JLONG >> 8 as ::core::ffi::c_int)
            as DCTELEM as ::core::ffi::c_int
            + z5 as ::core::ffi::c_int) as DCTELEM;
        z4 = ((tmp12 as JLONG * 334 as ::core::ffi::c_int as JLONG >> 8 as ::core::ffi::c_int)
            as DCTELEM as ::core::ffi::c_int
            + z5 as ::core::ffi::c_int) as DCTELEM;
        z3 = (tmp11 as JLONG * 181 as ::core::ffi::c_int as JLONG >> 8 as ::core::ffi::c_int)
            as DCTELEM;
        z11 = (tmp7 as ::core::ffi::c_int + z3 as ::core::ffi::c_int) as DCTELEM;
        z13 = (tmp7 as ::core::ffi::c_int - z3 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 5 as ::core::ffi::c_int) as isize) =
            (z13 as ::core::ffi::c_int + z2 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize) =
            (z13 as ::core::ffi::c_int - z2 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) =
            (z11 as ::core::ffi::c_int + z4 as ::core::ffi::c_int) as DCTELEM;
        *dataptr.offset((DCTSIZE * 7 as ::core::ffi::c_int) as isize) =
            (z11 as ::core::ffi::c_int - z4 as ::core::ffi::c_int) as DCTELEM;
        dataptr = dataptr.offset(1);
        ctr -= 1;
    }
}

pub const JPEG_RS_JFDCTFST_LINK_ANCHOR: unsafe extern "C" fn(*mut DCTELEM) = jpeg_fdct_ifast;
