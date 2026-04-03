#![allow(non_snake_case)]

use core::{
    ffi::{c_char, c_int, c_ulong},
    ptr,
};
use std::cell::RefCell;

use ffi_types::JMSG_LENGTH_MAX;
use jpeg_core::ported::turbojpeg::turbojpeg::{
    self as tj, tjscalingfactor, tjtransform, TjHandle, TjMathError, TJERR_FATAL,
};
#[doc(hidden)]
pub use libjpeg_abi::{common_exports, decompress_exports};

pub const SONAME: &str = "libturbojpeg.so.0";
pub const LINK_NAME: &str = "turbojpeg";

pub const VERSIONED_EXPORTS: &[(&str, &str)] = &[
    ("TJBUFSIZE", "TURBOJPEG_1.0"),
    ("tjCompress", "TURBOJPEG_1.0"),
    ("tjDecompress", "TURBOJPEG_1.0"),
    ("tjDecompressHeader", "TURBOJPEG_1.0"),
    ("tjDestroy", "TURBOJPEG_1.0"),
    ("tjGetErrorStr", "TURBOJPEG_1.0"),
    ("tjInitCompress", "TURBOJPEG_1.0"),
    ("tjInitDecompress", "TURBOJPEG_1.0"),
    ("TJBUFSIZEYUV", "TURBOJPEG_1.1"),
    ("tjDecompressHeader2", "TURBOJPEG_1.1"),
    ("tjDecompressToYUV", "TURBOJPEG_1.1"),
    ("tjEncodeYUV", "TURBOJPEG_1.1"),
    ("tjAlloc", "TURBOJPEG_1.2"),
    ("tjBufSize", "TURBOJPEG_1.2"),
    ("tjBufSizeYUV", "TURBOJPEG_1.2"),
    ("tjCompress2", "TURBOJPEG_1.2"),
    ("tjDecompress2", "TURBOJPEG_1.2"),
    ("tjEncodeYUV2", "TURBOJPEG_1.2"),
    ("tjFree", "TURBOJPEG_1.2"),
    ("tjGetScalingFactors", "TURBOJPEG_1.2"),
    ("tjInitTransform", "TURBOJPEG_1.2"),
    ("tjTransform", "TURBOJPEG_1.2"),
    ("tjBufSizeYUV2", "TURBOJPEG_1.4"),
    ("tjCompressFromYUV", "TURBOJPEG_1.4"),
    ("tjCompressFromYUVPlanes", "TURBOJPEG_1.4"),
    ("tjDecodeYUV", "TURBOJPEG_1.4"),
    ("tjDecodeYUVPlanes", "TURBOJPEG_1.4"),
    ("tjDecompressHeader3", "TURBOJPEG_1.4"),
    ("tjDecompressToYUV2", "TURBOJPEG_1.4"),
    ("tjDecompressToYUVPlanes", "TURBOJPEG_1.4"),
    ("tjEncodeYUV3", "TURBOJPEG_1.4"),
    ("tjEncodeYUVPlanes", "TURBOJPEG_1.4"),
    ("tjPlaneHeight", "TURBOJPEG_1.4"),
    ("tjPlaneSizeYUV", "TURBOJPEG_1.4"),
    ("tjPlaneWidth", "TURBOJPEG_1.4"),
    ("tjGetErrorCode", "TURBOJPEG_2.0"),
    ("tjGetErrorStr2", "TURBOJPEG_2.0"),
    ("tjLoadImage", "TURBOJPEG_2.0"),
    ("tjSaveImage", "TURBOJPEG_2.0"),
];

pub const EXPECTED_NON_JNI_SYMBOLS: &[&str] = &[
    "TJBUFSIZE",
    "TJBUFSIZEYUV",
    "tjAlloc",
    "tjBufSize",
    "tjBufSizeYUV",
    "tjBufSizeYUV2",
    "tjCompress",
    "tjCompress2",
    "tjCompressFromYUV",
    "tjCompressFromYUVPlanes",
    "tjDecodeYUV",
    "tjDecodeYUVPlanes",
    "tjDecompress",
    "tjDecompress2",
    "tjDecompressHeader",
    "tjDecompressHeader2",
    "tjDecompressHeader3",
    "tjDecompressToYUV",
    "tjDecompressToYUV2",
    "tjDecompressToYUVPlanes",
    "tjDestroy",
    "tjEncodeYUV",
    "tjEncodeYUV2",
    "tjEncodeYUV3",
    "tjEncodeYUVPlanes",
    "tjFree",
    "tjGetErrorCode",
    "tjGetErrorStr",
    "tjGetErrorStr2",
    "tjGetScalingFactors",
    "tjInitCompress",
    "tjInitDecompress",
    "tjInitTransform",
    "tjLoadImage",
    "tjPlaneHeight",
    "tjPlaneSizeYUV",
    "tjPlaneWidth",
    "tjSaveImage",
    "tjTransform",
];

#[derive(Clone)]
struct GlobalErrorState {
    message: [c_char; JMSG_LENGTH_MAX],
    active: bool,
    code: c_int,
}

impl Default for GlobalErrorState {
    fn default() -> Self {
        Self {
            message: [0; JMSG_LENGTH_MAX],
            active: false,
            code: TJERR_FATAL,
        }
    }
}

thread_local! {
    static GLOBAL_ERROR: RefCell<GlobalErrorState> = RefCell::new(GlobalErrorState::default());
}

macro_rules! forward_backend_fn {
    (backend = $backend:literal; fn $name:ident($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty;) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($($arg: $ty),*) -> $ret {
            clear_global_error();
            unsafe extern "C" {
                #[link_name = $backend]
                fn backend($($arg: $ty),*) -> $ret;
            }
            unsafe { backend($($arg),*) }
        }
    };
    (backend = $backend:literal; fn $name:ident($($arg:ident : $ty:ty),* $(,)?) ;) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($($arg: $ty),*) {
            clear_global_error();
            unsafe extern "C" {
                #[link_name = $backend]
                fn backend($($arg: $ty),*);
            }
            unsafe { backend($($arg),*) }
        }
    };
}

fn clear_global_error() {
    GLOBAL_ERROR.with(|state| {
        let mut state = state.borrow_mut();
        state.active = false;
        state.code = TJERR_FATAL;
        state.message[0] = 0;
    });
}

fn set_global_error(message: &str) {
    GLOBAL_ERROR.with(|state| {
        let mut state = state.borrow_mut();
        state.message.fill(0);
        for (dst, src) in state
            .message
            .iter_mut()
            .take(JMSG_LENGTH_MAX.saturating_sub(1))
            .zip(message.as_bytes().iter().copied())
        {
            *dst = src as c_char;
        }
        state.active = true;
        state.code = TJERR_FATAL;
    });
}

fn global_error_ptr() -> Option<*mut c_char> {
    GLOBAL_ERROR.with(|state| {
        let mut state = state.borrow_mut();
        if state.active {
            Some(state.message.as_mut_ptr())
        } else {
            None
        }
    })
}

fn global_error_code() -> Option<c_int> {
    GLOBAL_ERROR.with(|state| {
        let state = state.borrow();
        state.active.then_some(state.code)
    })
}

fn legacy_buf_size_message(error: TjMathError) -> &'static str {
    match error {
        TjMathError::InvalidArgument => "TJBUFSIZE(): Invalid argument",
        TjMathError::ImageTooLarge => "TJBUFSIZE(): Image is too large",
        TjMathError::WidthTooLarge | TjMathError::HeightTooLarge => {
            "TJBUFSIZE(): Image is too large"
        }
    }
}

fn buf_size_message(error: TjMathError) -> &'static str {
    match error {
        TjMathError::InvalidArgument => "tjBufSize(): Invalid argument",
        TjMathError::ImageTooLarge => "tjBufSize(): Image is too large",
        TjMathError::WidthTooLarge | TjMathError::HeightTooLarge => {
            "tjBufSize(): Image is too large"
        }
    }
}

fn buf_size_yuv2_message(error: TjMathError) -> &'static str {
    match error {
        TjMathError::InvalidArgument => "tjBufSizeYUV2(): Invalid argument",
        TjMathError::ImageTooLarge => "tjBufSizeYUV2(): Image is too large",
        TjMathError::WidthTooLarge | TjMathError::HeightTooLarge => {
            "tjBufSizeYUV2(): Image is too large"
        }
    }
}

fn plane_width_message(error: TjMathError) -> &'static str {
    match error {
        TjMathError::InvalidArgument => "tjPlaneWidth(): Invalid argument",
        TjMathError::WidthTooLarge => "tjPlaneWidth(): Width is too large",
        TjMathError::HeightTooLarge | TjMathError::ImageTooLarge => {
            "tjPlaneWidth(): Width is too large"
        }
    }
}

fn plane_height_message(error: TjMathError) -> &'static str {
    match error {
        TjMathError::InvalidArgument => "tjPlaneHeight(): Invalid argument",
        TjMathError::HeightTooLarge => "tjPlaneHeight(): Height is too large",
        TjMathError::WidthTooLarge | TjMathError::ImageTooLarge => {
            "tjPlaneHeight(): Height is too large"
        }
    }
}

fn plane_size_yuv_message(error: TjMathError) -> &'static str {
    match error {
        TjMathError::InvalidArgument => "tjPlaneSizeYUV(): Invalid argument",
        TjMathError::ImageTooLarge => "tjPlaneSizeYUV(): Image is too large",
        TjMathError::WidthTooLarge | TjMathError::HeightTooLarge => {
            "tjPlaneSizeYUV(): Image is too large"
        }
    }
}

forward_backend_fn!(backend = "rs_backend_tjInitCompress"; fn tjInitCompress() -> TjHandle;);
forward_backend_fn!(
    backend = "rs_backend_tjCompress2";
    fn tjCompress2(
        handle: TjHandle,
        srcBuf: *const u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelFormat: c_int,
        jpegBuf: *mut *mut u8,
        jpegSize: *mut c_ulong,
        jpegSubsamp: c_int,
        jpegQual: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjCompressFromYUV";
    fn tjCompressFromYUV(
        handle: TjHandle,
        srcBuf: *const u8,
        width: c_int,
        align: c_int,
        height: c_int,
        subsamp: c_int,
        jpegBuf: *mut *mut u8,
        jpegSize: *mut c_ulong,
        jpegQual: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjCompressFromYUVPlanes";
    fn tjCompressFromYUVPlanes(
        handle: TjHandle,
        srcPlanes: *const *const u8,
        width: c_int,
        strides: *const c_int,
        height: c_int,
        subsamp: c_int,
        jpegBuf: *mut *mut u8,
        jpegSize: *mut c_ulong,
        jpegQual: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjEncodeYUV3";
    fn tjEncodeYUV3(
        handle: TjHandle,
        srcBuf: *const u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelFormat: c_int,
        dstBuf: *mut u8,
        align: c_int,
        subsamp: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjEncodeYUVPlanes";
    fn tjEncodeYUVPlanes(
        handle: TjHandle,
        srcBuf: *const u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelFormat: c_int,
        dstPlanes: *mut *mut u8,
        strides: *mut c_int,
        subsamp: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(backend = "rs_backend_tjInitDecompress"; fn tjInitDecompress() -> TjHandle;);
forward_backend_fn!(
    backend = "rs_backend_tjDecompressHeader3";
    fn tjDecompressHeader3(
        handle: TjHandle,
        jpegBuf: *const u8,
        jpegSize: c_ulong,
        width: *mut c_int,
        height: *mut c_int,
        jpegSubsamp: *mut c_int,
        jpegColorspace: *mut c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjDecompress2";
    fn tjDecompress2(
        handle: TjHandle,
        jpegBuf: *const u8,
        jpegSize: c_ulong,
        dstBuf: *mut u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelFormat: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjDecompressToYUV2";
    fn tjDecompressToYUV2(
        handle: TjHandle,
        jpegBuf: *const u8,
        jpegSize: c_ulong,
        dstBuf: *mut u8,
        width: c_int,
        align: c_int,
        height: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjDecompressToYUVPlanes";
    fn tjDecompressToYUVPlanes(
        handle: TjHandle,
        jpegBuf: *const u8,
        jpegSize: c_ulong,
        dstPlanes: *mut *mut u8,
        width: c_int,
        strides: *mut c_int,
        height: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjDecodeYUV";
    fn tjDecodeYUV(
        handle: TjHandle,
        srcBuf: *const u8,
        align: c_int,
        subsamp: c_int,
        dstBuf: *mut u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelFormat: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjDecodeYUVPlanes";
    fn tjDecodeYUVPlanes(
        handle: TjHandle,
        srcPlanes: *const *const u8,
        strides: *const c_int,
        subsamp: c_int,
        dstBuf: *mut u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelFormat: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(backend = "rs_backend_tjInitTransform"; fn tjInitTransform() -> TjHandle;);
forward_backend_fn!(
    backend = "rs_backend_tjTransform";
    fn tjTransform(
        handle: TjHandle,
        jpegBuf: *const u8,
        jpegSize: c_ulong,
        n: c_int,
        dstBufs: *mut *mut u8,
        dstSizes: *mut c_ulong,
        transforms: *mut tjtransform,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(backend = "rs_backend_tjDestroy"; fn tjDestroy(handle: TjHandle) -> c_int;);
forward_backend_fn!(backend = "rs_backend_tjAlloc"; fn tjAlloc(bytes: c_int) -> *mut u8;);
forward_backend_fn!(
    backend = "rs_backend_tjLoadImage";
    fn tjLoadImage(
        filename: *const c_char,
        width: *mut c_int,
        align: c_int,
        height: *mut c_int,
        pixelFormat: *mut c_int,
        flags: c_int,
    ) -> *mut u8;
);
forward_backend_fn!(
    backend = "rs_backend_tjSaveImage";
    fn tjSaveImage(
        filename: *const c_char,
        buffer: *mut u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelFormat: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(backend = "rs_backend_tjFree"; fn tjFree(buffer: *mut u8););
forward_backend_fn!(
    backend = "rs_backend_tjCompress";
    fn tjCompress(
        handle: TjHandle,
        srcBuf: *mut u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelSize: c_int,
        dstBuf: *mut u8,
        compressedSize: *mut c_ulong,
        jpegSubsamp: c_int,
        jpegQual: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjDecompress";
    fn tjDecompress(
        handle: TjHandle,
        jpegBuf: *mut u8,
        jpegSize: c_ulong,
        dstBuf: *mut u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelSize: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjDecompressHeader";
    fn tjDecompressHeader(
        handle: TjHandle,
        jpegBuf: *mut u8,
        jpegSize: c_ulong,
        width: *mut c_int,
        height: *mut c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjDecompressHeader2";
    fn tjDecompressHeader2(
        handle: TjHandle,
        jpegBuf: *mut u8,
        jpegSize: c_ulong,
        width: *mut c_int,
        height: *mut c_int,
        jpegSubsamp: *mut c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjDecompressToYUV";
    fn tjDecompressToYUV(
        handle: TjHandle,
        jpegBuf: *mut u8,
        jpegSize: c_ulong,
        dstBuf: *mut u8,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjEncodeYUV";
    fn tjEncodeYUV(
        handle: TjHandle,
        srcBuf: *mut u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelSize: c_int,
        dstBuf: *mut u8,
        subsamp: c_int,
        flags: c_int,
    ) -> c_int;
);
forward_backend_fn!(
    backend = "rs_backend_tjEncodeYUV2";
    fn tjEncodeYUV2(
        handle: TjHandle,
        srcBuf: *mut u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelFormat: c_int,
        dstBuf: *mut u8,
        subsamp: c_int,
        flags: c_int,
    ) -> c_int;
);

#[no_mangle]
pub unsafe extern "C" fn tjBufSize(width: c_int, height: c_int, jpegSubsamp: c_int) -> c_ulong {
    clear_global_error();
    match tj::buf_size_checked(width, height, jpegSubsamp) {
        Ok(size) => size,
        Err(error) => {
            set_global_error(buf_size_message(error));
            c_ulong::MAX
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn TJBUFSIZE(width: c_int, height: c_int) -> c_ulong {
    clear_global_error();
    match tj::legacy_buf_size_checked(width, height) {
        Ok(size) => size,
        Err(error) => {
            set_global_error(legacy_buf_size_message(error));
            c_ulong::MAX
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tjBufSizeYUV2(
    width: c_int,
    align: c_int,
    height: c_int,
    subsamp: c_int,
) -> c_ulong {
    clear_global_error();
    match tj::buf_size_yuv2_checked(width, align, height, subsamp) {
        Ok(size) => size,
        Err(error) => {
            set_global_error(buf_size_yuv2_message(error));
            c_ulong::MAX
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tjBufSizeYUV(width: c_int, height: c_int, subsamp: c_int) -> c_ulong {
    unsafe { tjBufSizeYUV2(width, 4, height, subsamp) }
}

#[no_mangle]
pub unsafe extern "C" fn TJBUFSIZEYUV(width: c_int, height: c_int, subsamp: c_int) -> c_ulong {
    unsafe { tjBufSizeYUV(width, height, subsamp) }
}

#[no_mangle]
pub unsafe extern "C" fn tjPlaneWidth(
    componentID: c_int,
    width: c_int,
    subsamp: c_int,
) -> c_int {
    clear_global_error();
    match tj::plane_width_checked(componentID, width, subsamp) {
        Ok(width) => width,
        Err(error) => {
            set_global_error(plane_width_message(error));
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tjPlaneHeight(
    componentID: c_int,
    height: c_int,
    subsamp: c_int,
) -> c_int {
    clear_global_error();
    match tj::plane_height_checked(componentID, height, subsamp) {
        Ok(height) => height,
        Err(error) => {
            set_global_error(plane_height_message(error));
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tjPlaneSizeYUV(
    componentID: c_int,
    width: c_int,
    stride: c_int,
    height: c_int,
    subsamp: c_int,
) -> c_ulong {
    clear_global_error();
    match tj::plane_size_yuv_checked(componentID, width, stride, height, subsamp) {
        Ok(size) => size,
        Err(error) => {
            set_global_error(plane_size_yuv_message(error));
            c_ulong::MAX
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tjGetScalingFactors(
    numScalingFactors: *mut c_int,
) -> *mut tjscalingfactor {
    clear_global_error();
    if numScalingFactors.is_null() {
        set_global_error("tjGetScalingFactors(): Invalid argument");
        return ptr::null_mut();
    }

    unsafe {
        *numScalingFactors = tj::NUM_SCALING_FACTORS;
    }
    tj::SCALING_FACTORS.as_ptr() as *mut tjscalingfactor
}

#[no_mangle]
pub unsafe extern "C" fn tjGetErrorStr2(handle: TjHandle) -> *mut c_char {
    if handle.is_null() {
        if let Some(error) = global_error_ptr() {
            return error;
        }
    }

    unsafe extern "C" {
        #[link_name = "rs_backend_tjGetErrorStr2"]
        fn backend(handle: TjHandle) -> *mut c_char;
    }
    unsafe { backend(handle) }
}

#[no_mangle]
pub unsafe extern "C" fn tjGetErrorCode(handle: TjHandle) -> c_int {
    if handle.is_null() {
        if let Some(code) = global_error_code() {
            return code;
        }
    }

    unsafe extern "C" {
        #[link_name = "rs_backend_tjGetErrorCode"]
        fn backend(handle: TjHandle) -> c_int;
    }
    unsafe { backend(handle) }
}

#[no_mangle]
pub unsafe extern "C" fn tjGetErrorStr() -> *mut c_char {
    if let Some(error) = global_error_ptr() {
        return error;
    }

    unsafe extern "C" {
        #[link_name = "rs_backend_tjGetErrorStr"]
        fn backend() -> *mut c_char;
    }
    unsafe { backend() }
}
