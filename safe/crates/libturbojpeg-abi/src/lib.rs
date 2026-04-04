#![allow(non_snake_case)]
#![allow(clippy::all)]

use core::{
    ffi::{c_char, c_int, c_ulong, c_void},
    ptr,
};
use std::{
    cell::RefCell,
    ffi::{CStr, CString},
    mem::MaybeUninit,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    sync::OnceLock,
};

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

pub const EXPECTED_JNI_SYMBOLS: &[&str] = &[
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_compressFromYUV___3_3B_3II_3III_3BII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3BIIIIII_3BIII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3BIIII_3BIII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3IIIIIII_3BIII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3IIIII_3BIII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_destroy",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3BIIIIII_3_3B_3I_3III",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3BIIII_3BII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3IIIIIII_3_3B_3I_3III",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3IIIII_3BII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_init",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decodeYUV___3_3B_3I_3II_3BIIIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decodeYUV___3_3B_3I_3II_3IIIIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompressHeader",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompressToYUV___3BI_3BI",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompressToYUV___3BI_3_3B_3II_3III",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3BIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3BIIIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3IIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3IIIIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_destroy",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_init",
    "Java_org_libjpegturbo_turbojpeg_TJTransformer_init",
    "Java_org_libjpegturbo_turbojpeg_TJTransformer_transform",
    "Java_org_libjpegturbo_turbojpeg_TJ_bufSize",
    "Java_org_libjpegturbo_turbojpeg_TJ_bufSizeYUV__III",
    "Java_org_libjpegturbo_turbojpeg_TJ_bufSizeYUV__IIII",
    "Java_org_libjpegturbo_turbojpeg_TJ_getScalingFactors",
    "Java_org_libjpegturbo_turbojpeg_TJ_planeHeight__III",
    "Java_org_libjpegturbo_turbojpeg_TJ_planeSizeYUV__IIIII",
    "Java_org_libjpegturbo_turbojpeg_TJ_planeWidth__III",
];

pub const BACKEND_LIBRARY_ENV_VAR: &str = "LIBJPEG_TURBO_BACKEND_LIB";

const RTLD_NOW: c_int = 2;

#[repr(C)]
struct DlInfo {
    dli_fname: *const c_char,
    dli_fbase: *mut c_void,
    dli_sname: *const c_char,
    dli_saddr: *mut c_void,
}

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
    fn dladdr(addr: *const c_void, info: *mut DlInfo) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

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

struct BackendLibrary {
    handle: *mut c_void,
}

unsafe impl Send for BackendLibrary {}
unsafe impl Sync for BackendLibrary {}

static BACKEND_LIBRARY: OnceLock<Result<BackendLibrary, String>> = OnceLock::new();

impl BackendLibrary {
    fn load() -> Result<Self, String> {
        let path = backend_library_path()?;
        let path_c = CString::new(path.as_os_str().as_bytes())
            .map_err(|error| format!("backend path contains NUL: {error}"))?;
        let handle = unsafe { dlopen(path_c.as_ptr(), RTLD_NOW) };
        if handle.is_null() {
            return Err(format!(
                "dlopen {} failed: {}",
                path.display(),
                unsafe { dlerror_message() }
            ));
        }
        Ok(Self { handle })
    }

    unsafe fn symbol<T>(&self, symbol: &'static [u8]) -> Result<T, String> {
        let name = CStr::from_bytes_with_nul(symbol).expect("backend symbol names are NUL-terminated");
        let ptr = unsafe { dlsym(self.handle, name.as_ptr()) };
        if ptr.is_null() {
            return Err(format!(
                "dlsym({}) failed: {}",
                name.to_string_lossy(),
                unsafe { dlerror_message() }
            ));
        }
        Ok(unsafe { std::mem::transmute_copy(&ptr) })
    }
}

fn backend_library() -> Result<&'static BackendLibrary, String> {
    match BACKEND_LIBRARY.get_or_init(BackendLibrary::load) {
        Ok(backend) => Ok(backend),
        Err(error) => Err(error.clone()),
    }
}

unsafe fn dlerror_message() -> String {
    let message = unsafe { dlerror() };
    if message.is_null() {
        "unknown dlerror".to_string()
    } else {
        unsafe { CStr::from_ptr(message) }
            .to_string_lossy()
            .into_owned()
    }
}

fn current_library_path() -> Result<PathBuf, String> {
    let mut info = MaybeUninit::<DlInfo>::zeroed();
    let rc = unsafe { dladdr(current_library_path as *const () as *const c_void, info.as_mut_ptr()) };
    if rc == 0 {
        return Err(unsafe { dlerror_message() });
    }
    let info = unsafe { info.assume_init() };
    if info.dli_fname.is_null() {
        return Err("dladdr returned a null library path".to_string());
    }
    Ok(PathBuf::from(
        unsafe { CStr::from_ptr(info.dli_fname) }
            .to_string_lossy()
            .into_owned(),
    ))
}

fn find_safe_root_from(path: &Path) -> Result<PathBuf, String> {
    for ancestor in path.ancestors() {
        if ancestor.join("Cargo.toml").is_file() && ancestor.join("scripts/stage-install.sh").is_file() {
            return Ok(ancestor.to_path_buf());
        }
    }
    Err(format!(
        "could not locate safe/ root from {}",
        path.display()
    ))
}

fn host_multiarch() -> Option<String> {
    for (program, args) in [
        ("dpkg-architecture", &["-qDEB_HOST_MULTIARCH"][..]),
        ("gcc", &["-print-multiarch"][..]),
    ] {
        if let Ok(output) = std::process::Command::new(program).args(args).output() {
            if output.status.success() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
                if !value.is_empty() {
                    return Some(value);
                }
            }
        }
    }
    None
}

fn backend_library_path() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os(BACKEND_LIBRARY_ENV_VAR) {
        return Ok(PathBuf::from(path));
    }

    let library_path = current_library_path()?;
    let safe_root = find_safe_root_from(&library_path)?;
    let runtime_dir = safe_root
        .join("runtime")
        .join(host_multiarch().ok_or_else(|| "could not determine host multiarch".to_string())?)
        .join("lib");
    for candidate in [
        runtime_dir.join("libturbojpeg.so.0.2.0"),
        runtime_dir.join("libturbojpeg.so.0"),
        runtime_dir.join("libturbojpeg.so"),
    ] {
        if candidate.is_file() {
            return Ok(candidate);
        }
    }

    Err(format!(
        "could not find packaged TurboJPEG backend under {}",
        runtime_dir.display()
    ))
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

fn backend_load_failure(name: &str, error: String) {
    set_global_error(&format!("{name}(): {error}"));
}

macro_rules! forward_backend_fn {
    (fn $name:ident($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty; default $default:expr;) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($($arg: $ty),*) -> $ret {
            clear_global_error();
            let backend = match backend_library() {
                Ok(backend) => backend,
                Err(error) => {
                    backend_load_failure(stringify!($name), error);
                    return $default;
                }
            };
            let func: unsafe extern "C" fn($($ty),*) -> $ret = match unsafe {
                backend.symbol(concat!(stringify!($name), "\0").as_bytes())
            } {
                Ok(func) => func,
                Err(error) => {
                    backend_load_failure(stringify!($name), error);
                    return $default;
                }
            };
            unsafe { func($($arg),*) }
        }
    };
    (fn $name:ident($($arg:ident : $ty:ty),* $(,)?) ; ) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($($arg: $ty),*) {
            clear_global_error();
            let backend = match backend_library() {
                Ok(backend) => backend,
                Err(error) => {
                    backend_load_failure(stringify!($name), error);
                    return;
                }
            };
            let func: unsafe extern "C" fn($($ty),*) = match unsafe {
                backend.symbol(concat!(stringify!($name), "\0").as_bytes())
            } {
                Ok(func) => func,
                Err(error) => {
                    backend_load_failure(stringify!($name), error);
                    return;
                }
            };
            unsafe { func($($arg),*) }
        }
    };
}

forward_backend_fn!(fn tjInitCompress() -> TjHandle; default ptr::null_mut(););
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(fn tjInitDecompress() -> TjHandle; default ptr::null_mut(););
forward_backend_fn!(
    fn tjDecompressHeader3(
        handle: TjHandle,
        jpegBuf: *const u8,
        jpegSize: c_ulong,
        width: *mut c_int,
        height: *mut c_int,
        jpegSubsamp: *mut c_int,
        jpegColorspace: *mut c_int,
    ) -> c_int;
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(fn tjInitTransform() -> TjHandle; default ptr::null_mut(););
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
    fn tjLoadImage(
        filename: *const c_char,
        width: *mut c_int,
        align: c_int,
        height: *mut c_int,
        pixelFormat: *mut c_int,
        flags: c_int,
    ) -> *mut u8;
    default ptr::null_mut();
);
forward_backend_fn!(
    fn tjSaveImage(
        filename: *const c_char,
        buffer: *mut u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelFormat: c_int,
        flags: c_int,
    ) -> c_int;
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
    fn tjDecompressHeader(
        handle: TjHandle,
        jpegBuf: *mut u8,
        jpegSize: c_ulong,
        width: *mut c_int,
        height: *mut c_int,
    ) -> c_int;
    default -1;
);
forward_backend_fn!(
    fn tjDecompressHeader2(
        handle: TjHandle,
        jpegBuf: *mut u8,
        jpegSize: c_ulong,
        width: *mut c_int,
        height: *mut c_int,
        jpegSubsamp: *mut c_int,
    ) -> c_int;
    default -1;
);
forward_backend_fn!(
    fn tjDecompressToYUV(
        handle: TjHandle,
        jpegBuf: *mut u8,
        jpegSize: c_ulong,
        dstBuf: *mut u8,
        flags: c_int,
    ) -> c_int;
    default -1;
);
forward_backend_fn!(
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
    default -1;
);
forward_backend_fn!(
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
    default -1;
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
pub unsafe extern "C" fn tjDestroy(handle: TjHandle) -> c_int {
    if handle.is_null() {
        return 0;
    }
    clear_global_error();
    let backend = match backend_library() {
        Ok(backend) => backend,
        Err(error) => {
            backend_load_failure("tjDestroy", error);
            return -1;
        }
    };
    let func: unsafe extern "C" fn(TjHandle) -> c_int = match unsafe {
        backend.symbol(b"tjDestroy\0")
    } {
        Ok(func) => func,
        Err(error) => {
            backend_load_failure("tjDestroy", error);
            return -1;
        }
    };
    unsafe { func(handle) }
}

#[no_mangle]
pub unsafe extern "C" fn tjAlloc(bytes: c_int) -> *mut u8 {
    clear_global_error();
    if bytes < 0 {
        set_global_error("tjAlloc(): Invalid argument");
        return ptr::null_mut();
    }
    unsafe { malloc(bytes as usize) as *mut u8 }
}

#[no_mangle]
pub unsafe extern "C" fn tjFree(buffer: *mut u8) {
    if buffer.is_null() {
        return;
    }
    unsafe { free(buffer.cast::<c_void>()) }
}

#[no_mangle]
pub unsafe extern "C" fn tjGetErrorStr2(handle: TjHandle) -> *mut c_char {
    if handle.is_null() {
        if let Some(error) = global_error_ptr() {
            return error;
        }
    }

    let backend = match backend_library() {
        Ok(backend) => backend,
        Err(error) => {
            backend_load_failure("tjGetErrorStr2", error);
            return global_error_ptr().unwrap_or(ptr::null_mut());
        }
    };
    let func: unsafe extern "C" fn(TjHandle) -> *mut c_char = match unsafe {
        backend.symbol(b"tjGetErrorStr2\0")
    } {
        Ok(func) => func,
        Err(error) => {
            backend_load_failure("tjGetErrorStr2", error);
            return global_error_ptr().unwrap_or(ptr::null_mut());
        }
    };
    unsafe { func(handle) }
}

#[no_mangle]
pub unsafe extern "C" fn tjGetErrorCode(handle: TjHandle) -> c_int {
    if handle.is_null() {
        if let Some(code) = global_error_code() {
            return code;
        }
    }

    let backend = match backend_library() {
        Ok(backend) => backend,
        Err(error) => {
            backend_load_failure("tjGetErrorCode", error);
            return TJERR_FATAL;
        }
    };
    let func: unsafe extern "C" fn(TjHandle) -> c_int = match unsafe {
        backend.symbol(b"tjGetErrorCode\0")
    } {
        Ok(func) => func,
        Err(error) => {
            backend_load_failure("tjGetErrorCode", error);
            return TJERR_FATAL;
        }
    };
    unsafe { func(handle) }
}

#[no_mangle]
pub unsafe extern "C" fn tjGetErrorStr() -> *mut c_char {
    if let Some(error) = global_error_ptr() {
        return error;
    }

    let backend = match backend_library() {
        Ok(backend) => backend,
        Err(error) => {
            backend_load_failure("tjGetErrorStr", error);
            return global_error_ptr().unwrap_or(ptr::null_mut());
        }
    };
    let func: unsafe extern "C" fn() -> *mut c_char = match unsafe { backend.symbol(b"tjGetErrorStr\0") } {
        Ok(func) => func,
        Err(error) => {
            backend_load_failure("tjGetErrorStr", error);
            return global_error_ptr().unwrap_or(ptr::null_mut());
        }
    };
    unsafe { func() }
}
