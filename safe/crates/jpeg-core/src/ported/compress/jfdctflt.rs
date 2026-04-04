pub const DCTSIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn jpeg_fdct_float(mut data: *mut ::core::ffi::c_float) {
    let mut tmp0: ::core::ffi::c_float = 0.;
    let mut tmp1: ::core::ffi::c_float = 0.;
    let mut tmp2: ::core::ffi::c_float = 0.;
    let mut tmp3: ::core::ffi::c_float = 0.;
    let mut tmp4: ::core::ffi::c_float = 0.;
    let mut tmp5: ::core::ffi::c_float = 0.;
    let mut tmp6: ::core::ffi::c_float = 0.;
    let mut tmp7: ::core::ffi::c_float = 0.;
    let mut tmp10: ::core::ffi::c_float = 0.;
    let mut tmp11: ::core::ffi::c_float = 0.;
    let mut tmp12: ::core::ffi::c_float = 0.;
    let mut tmp13: ::core::ffi::c_float = 0.;
    let mut z1: ::core::ffi::c_float = 0.;
    let mut z2: ::core::ffi::c_float = 0.;
    let mut z3: ::core::ffi::c_float = 0.;
    let mut z4: ::core::ffi::c_float = 0.;
    let mut z5: ::core::ffi::c_float = 0.;
    let mut z11: ::core::ffi::c_float = 0.;
    let mut z13: ::core::ffi::c_float = 0.;
    let mut dataptr: *mut ::core::ffi::c_float = ::core::ptr::null_mut::<::core::ffi::c_float>();
    let mut ctr: ::core::ffi::c_int = 0;
    dataptr = data;
    ctr = DCTSIZE - 1 as ::core::ffi::c_int;
    while ctr >= 0 as ::core::ffi::c_int {
        tmp0 = *dataptr.offset(0 as ::core::ffi::c_int as isize)
            + *dataptr.offset(7 as ::core::ffi::c_int as isize);
        tmp7 = *dataptr.offset(0 as ::core::ffi::c_int as isize)
            - *dataptr.offset(7 as ::core::ffi::c_int as isize);
        tmp1 = *dataptr.offset(1 as ::core::ffi::c_int as isize)
            + *dataptr.offset(6 as ::core::ffi::c_int as isize);
        tmp6 = *dataptr.offset(1 as ::core::ffi::c_int as isize)
            - *dataptr.offset(6 as ::core::ffi::c_int as isize);
        tmp2 = *dataptr.offset(2 as ::core::ffi::c_int as isize)
            + *dataptr.offset(5 as ::core::ffi::c_int as isize);
        tmp5 = *dataptr.offset(2 as ::core::ffi::c_int as isize)
            - *dataptr.offset(5 as ::core::ffi::c_int as isize);
        tmp3 = *dataptr.offset(3 as ::core::ffi::c_int as isize)
            + *dataptr.offset(4 as ::core::ffi::c_int as isize);
        tmp4 = *dataptr.offset(3 as ::core::ffi::c_int as isize)
            - *dataptr.offset(4 as ::core::ffi::c_int as isize);
        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;
        *dataptr.offset(0 as ::core::ffi::c_int as isize) = tmp10 + tmp11;
        *dataptr.offset(4 as ::core::ffi::c_int as isize) = tmp10 - tmp11;
        z1 = (tmp12 + tmp13) * 0.707106781f64 as ::core::ffi::c_float;
        *dataptr.offset(2 as ::core::ffi::c_int as isize) = tmp13 + z1;
        *dataptr.offset(6 as ::core::ffi::c_int as isize) = tmp13 - z1;
        tmp10 = tmp4 + tmp5;
        tmp11 = tmp5 + tmp6;
        tmp12 = tmp6 + tmp7;
        z5 = (tmp10 - tmp12) * 0.382683433f64 as ::core::ffi::c_float;
        z2 = 0.541196100f64 as ::core::ffi::c_float * tmp10 + z5;
        z4 = 1.306562965f64 as ::core::ffi::c_float * tmp12 + z5;
        z3 = tmp11 * 0.707106781f64 as ::core::ffi::c_float;
        z11 = tmp7 + z3;
        z13 = tmp7 - z3;
        *dataptr.offset(5 as ::core::ffi::c_int as isize) = z13 + z2;
        *dataptr.offset(3 as ::core::ffi::c_int as isize) = z13 - z2;
        *dataptr.offset(1 as ::core::ffi::c_int as isize) = z11 + z4;
        *dataptr.offset(7 as ::core::ffi::c_int as isize) = z11 - z4;
        dataptr = dataptr.offset(DCTSIZE as isize);
        ctr -= 1;
    }
    dataptr = data;
    ctr = DCTSIZE - 1 as ::core::ffi::c_int;
    while ctr >= 0 as ::core::ffi::c_int {
        tmp0 = *dataptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize)
            + *dataptr.offset((DCTSIZE * 7 as ::core::ffi::c_int) as isize);
        tmp7 = *dataptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize)
            - *dataptr.offset((DCTSIZE * 7 as ::core::ffi::c_int) as isize);
        tmp1 = *dataptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize)
            + *dataptr.offset((DCTSIZE * 6 as ::core::ffi::c_int) as isize);
        tmp6 = *dataptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize)
            - *dataptr.offset((DCTSIZE * 6 as ::core::ffi::c_int) as isize);
        tmp2 = *dataptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize)
            + *dataptr.offset((DCTSIZE * 5 as ::core::ffi::c_int) as isize);
        tmp5 = *dataptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize)
            - *dataptr.offset((DCTSIZE * 5 as ::core::ffi::c_int) as isize);
        tmp3 = *dataptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize)
            + *dataptr.offset((DCTSIZE * 4 as ::core::ffi::c_int) as isize);
        tmp4 = *dataptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize)
            - *dataptr.offset((DCTSIZE * 4 as ::core::ffi::c_int) as isize);
        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;
        *dataptr.offset((DCTSIZE * 0 as ::core::ffi::c_int) as isize) = tmp10 + tmp11;
        *dataptr.offset((DCTSIZE * 4 as ::core::ffi::c_int) as isize) = tmp10 - tmp11;
        z1 = (tmp12 + tmp13) * 0.707106781f64 as ::core::ffi::c_float;
        *dataptr.offset((DCTSIZE * 2 as ::core::ffi::c_int) as isize) = tmp13 + z1;
        *dataptr.offset((DCTSIZE * 6 as ::core::ffi::c_int) as isize) = tmp13 - z1;
        tmp10 = tmp4 + tmp5;
        tmp11 = tmp5 + tmp6;
        tmp12 = tmp6 + tmp7;
        z5 = (tmp10 - tmp12) * 0.382683433f64 as ::core::ffi::c_float;
        z2 = 0.541196100f64 as ::core::ffi::c_float * tmp10 + z5;
        z4 = 1.306562965f64 as ::core::ffi::c_float * tmp12 + z5;
        z3 = tmp11 * 0.707106781f64 as ::core::ffi::c_float;
        z11 = tmp7 + z3;
        z13 = tmp7 - z3;
        *dataptr.offset((DCTSIZE * 5 as ::core::ffi::c_int) as isize) = z13 + z2;
        *dataptr.offset((DCTSIZE * 3 as ::core::ffi::c_int) as isize) = z13 - z2;
        *dataptr.offset((DCTSIZE * 1 as ::core::ffi::c_int) as isize) = z11 + z4;
        *dataptr.offset((DCTSIZE * 7 as ::core::ffi::c_int) as isize) = z11 - z4;
        dataptr = dataptr.offset(1);
        ctr -= 1;
    }
}

pub const JPEG_RS_JFDCTFLT_LINK_ANCHOR: unsafe extern "C" fn(*mut ::core::ffi::c_float) =
    jpeg_fdct_float;
