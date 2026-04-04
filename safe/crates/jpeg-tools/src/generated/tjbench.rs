#[repr(C)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
extern "C" {
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn atof(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn toupper(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn tjInitCompress() -> tjhandle;
    fn tjCompress2(
        handle: tjhandle,
        srcBuf: *const ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        pitch: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        pixelFormat: ::core::ffi::c_int,
        jpegBuf: *mut *mut ::core::ffi::c_uchar,
        jpegSize: *mut ::core::ffi::c_ulong,
        jpegSubsamp: ::core::ffi::c_int,
        jpegQual: ::core::ffi::c_int,
        flags_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjCompressFromYUV(
        handle: tjhandle,
        srcBuf: *const ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        align: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
        jpegBuf: *mut *mut ::core::ffi::c_uchar,
        jpegSize: *mut ::core::ffi::c_ulong,
        jpegQual: ::core::ffi::c_int,
        flags_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjBufSize(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        jpegSubsamp: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn tjBufSizeYUV2(
        width: ::core::ffi::c_int,
        align: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn tjEncodeYUV3(
        handle: tjhandle,
        srcBuf: *const ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        pitch: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        pixelFormat: ::core::ffi::c_int,
        dstBuf: *mut ::core::ffi::c_uchar,
        align: ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
        flags_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjInitDecompress() -> tjhandle;
    fn tjDecompressHeader3(
        handle: tjhandle,
        jpegBuf: *const ::core::ffi::c_uchar,
        jpegSize: ::core::ffi::c_ulong,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
        jpegSubsamp: *mut ::core::ffi::c_int,
        jpegColorspace: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjGetScalingFactors(numScalingFactors: *mut ::core::ffi::c_int) -> *mut tjscalingfactor;
    fn tjDecompress2(
        handle: tjhandle,
        jpegBuf: *const ::core::ffi::c_uchar,
        jpegSize: ::core::ffi::c_ulong,
        dstBuf: *mut ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        pitch: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        pixelFormat: ::core::ffi::c_int,
        flags_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjDecompressToYUV2(
        handle: tjhandle,
        jpegBuf: *const ::core::ffi::c_uchar,
        jpegSize: ::core::ffi::c_ulong,
        dstBuf: *mut ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        align: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        flags_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjDecodeYUV(
        handle: tjhandle,
        srcBuf: *const ::core::ffi::c_uchar,
        align: ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
        dstBuf: *mut ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        pitch: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        pixelFormat: ::core::ffi::c_int,
        flags_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjInitTransform() -> tjhandle;
    fn tjTransform(
        handle: tjhandle,
        jpegBuf: *const ::core::ffi::c_uchar,
        jpegSize: ::core::ffi::c_ulong,
        n: ::core::ffi::c_int,
        dstBufs: *mut *mut ::core::ffi::c_uchar,
        dstSizes: *mut ::core::ffi::c_ulong,
        transforms: *mut tjtransform,
        flags_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjDestroy(handle: tjhandle) -> ::core::ffi::c_int;
    fn tjAlloc(bytes: ::core::ffi::c_int) -> *mut ::core::ffi::c_uchar;
    fn tjLoadImage(
        filename: *const ::core::ffi::c_char,
        width: *mut ::core::ffi::c_int,
        align: ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
        pixelFormat: *mut ::core::ffi::c_int,
        flags_0: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_uchar;
    fn tjSaveImage(
        filename: *const ::core::ffi::c_char,
        buffer: *mut ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        pitch: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        pixelFormat: ::core::ffi::c_int,
        flags_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjFree(buffer: *mut ::core::ffi::c_uchar);
    fn tjGetErrorStr2(handle: tjhandle) -> *mut ::core::ffi::c_char;
    fn tjGetErrorCode(handle: tjhandle) -> ::core::ffi::c_int;
    fn tjGetErrorStr() -> *mut ::core::ffi::c_char;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type TJSAMP = ::core::ffi::c_uint;
pub const TJSAMP_411: TJSAMP = 5;
pub const TJSAMP_440: TJSAMP = 4;
pub const TJSAMP_GRAY: TJSAMP = 3;
pub const TJSAMP_420: TJSAMP = 2;
pub const TJSAMP_422: TJSAMP = 1;
pub const TJSAMP_444: TJSAMP = 0;
pub type TJPF = ::core::ffi::c_int;
pub const TJPF_UNKNOWN: TJPF = -1;
pub const TJPF_CMYK: TJPF = 11;
pub const TJPF_ARGB: TJPF = 10;
pub const TJPF_ABGR: TJPF = 9;
pub const TJPF_BGRA: TJPF = 8;
pub const TJPF_RGBA: TJPF = 7;
pub const TJPF_GRAY: TJPF = 6;
pub const TJPF_XRGB: TJPF = 5;
pub const TJPF_XBGR: TJPF = 4;
pub const TJPF_BGRX: TJPF = 3;
pub const TJPF_RGBX: TJPF = 2;
pub const TJPF_BGR: TJPF = 1;
pub const TJPF_RGB: TJPF = 0;
pub type TJCS = ::core::ffi::c_uint;
pub const TJCS_YCCK: TJCS = 4;
pub const TJCS_CMYK: TJCS = 3;
pub const TJCS_GRAY: TJCS = 2;
pub const TJCS_YCbCr: TJCS = 1;
pub const TJCS_RGB: TJCS = 0;
pub type TJERR = ::core::ffi::c_uint;
pub const TJERR_FATAL: TJERR = 1;
pub const TJERR_WARNING: TJERR = 0;
pub type TJXOP = ::core::ffi::c_uint;
pub const TJXOP_ROT270: TJXOP = 7;
pub const TJXOP_ROT180: TJXOP = 6;
pub const TJXOP_ROT90: TJXOP = 5;
pub const TJXOP_TRANSVERSE: TJXOP = 4;
pub const TJXOP_TRANSPOSE: TJXOP = 3;
pub const TJXOP_VFLIP: TJXOP = 2;
pub const TJXOP_HFLIP: TJXOP = 1;
pub const TJXOP_NONE: TJXOP = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tjscalingfactor {
    pub num: ::core::ffi::c_int,
    pub denom: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tjregion {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tjtransform {
    pub r: tjregion,
    pub op: ::core::ffi::c_int,
    pub options: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub customFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_short,
            tjregion,
            tjregion,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut tjtransform,
        ) -> ::core::ffi::c_int,
    >,
}
pub type tjhandle = *mut ::core::ffi::c_void;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const JMSG_LENGTH_MAX: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const TJ_NUMSAMP: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
static mut tjMCUWidth: [::core::ffi::c_int; 6] = [
    8 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    32 as ::core::ffi::c_int,
];
static mut tjMCUHeight: [::core::ffi::c_int; 6] = [
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
];
static mut tjRedOffset: [::core::ffi::c_int; 12] = [
    0 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    -(1 as ::core::ffi::c_int),
    0 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    -(1 as ::core::ffi::c_int),
];
static mut tjGreenOffset: [::core::ffi::c_int; 12] = [
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    -(1 as ::core::ffi::c_int),
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    -(1 as ::core::ffi::c_int),
];
static mut tjBlueOffset: [::core::ffi::c_int; 12] = [
    2 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    -(1 as ::core::ffi::c_int),
    2 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    -(1 as ::core::ffi::c_int),
];
static mut tjPixelSize: [::core::ffi::c_int; 12] = [
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
];
pub const TJFLAG_BOTTOMUP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TJFLAG_FASTUPSAMPLE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const TJFLAG_NOREALLOC: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const TJFLAG_FASTDCT: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const TJFLAG_ACCURATEDCT: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const TJFLAG_STOPONWARNING: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const TJFLAG_PROGRESSIVE: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
pub const TJFLAG_LIMITSCANS: ::core::ffi::c_int = 32768 as ::core::ffi::c_int;
pub const TJXOPT_TRIM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TJXOPT_CROP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TJXOPT_GRAY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const TJXOPT_NOOUTPUT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const TJXOPT_PROGRESSIVE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const TJXOPT_COPYNONE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
#[no_mangle]
pub static mut tjErrorStr: [::core::ffi::c_char; 200] = unsafe {
    ::core::mem::transmute::<
        [u8; 200],
        [::core::ffi::c_char; 200],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
#[no_mangle]
pub static mut tjErrorMsg: [::core::ffi::c_char; 200] = unsafe {
    ::core::mem::transmute::<
        [u8; 200],
        [::core::ffi::c_char; 200],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
#[no_mangle]
pub static mut tjErrorCode: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub static mut tjErrorLine: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub static mut decompOnly: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut doYUV: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut doWrite: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut yuvAlign: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut pf: ::core::ffi::c_int = TJPF_BGR as ::core::ffi::c_int;
#[no_mangle]
pub static mut doTile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut quiet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut compOnly: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut flags: ::core::ffi::c_int = TJFLAG_NOREALLOC;
#[no_mangle]
pub static mut ext: *mut ::core::ffi::c_char =
    b"ppm\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut pixFormatStr: [*const ::core::ffi::c_char; 12] = [
    b"RGB\0" as *const u8 as *const ::core::ffi::c_char,
    b"BGR\0" as *const u8 as *const ::core::ffi::c_char,
    b"RGBX\0" as *const u8 as *const ::core::ffi::c_char,
    b"BGRX\0" as *const u8 as *const ::core::ffi::c_char,
    b"XBGR\0" as *const u8 as *const ::core::ffi::c_char,
    b"XRGB\0" as *const u8 as *const ::core::ffi::c_char,
    b"GRAY\0" as *const u8 as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"CMYK\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub static mut subNameLong: [*const ::core::ffi::c_char; 6] = [
    b"4:4:4\0" as *const u8 as *const ::core::ffi::c_char,
    b"4:2:2\0" as *const u8 as *const ::core::ffi::c_char,
    b"4:2:0\0" as *const u8 as *const ::core::ffi::c_char,
    b"GRAY\0" as *const u8 as *const ::core::ffi::c_char,
    b"4:4:0\0" as *const u8 as *const ::core::ffi::c_char,
    b"4:1:1\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub static mut csName: [*const ::core::ffi::c_char; 5] = [
    b"RGB\0" as *const u8 as *const ::core::ffi::c_char,
    b"YCbCr\0" as *const u8 as *const ::core::ffi::c_char,
    b"GRAY\0" as *const u8 as *const ::core::ffi::c_char,
    b"CMYK\0" as *const u8 as *const ::core::ffi::c_char,
    b"YCCK\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub static mut subName: [*const ::core::ffi::c_char; 6] = [
    b"444\0" as *const u8 as *const ::core::ffi::c_char,
    b"422\0" as *const u8 as *const ::core::ffi::c_char,
    b"420\0" as *const u8 as *const ::core::ffi::c_char,
    b"GRAY\0" as *const u8 as *const ::core::ffi::c_char,
    b"440\0" as *const u8 as *const ::core::ffi::c_char,
    b"411\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub static mut sf: tjscalingfactor = tjscalingfactor {
    num: 1 as ::core::ffi::c_int,
    denom: 1 as ::core::ffi::c_int,
};
#[no_mangle]
pub static mut scalingFactors: *mut tjscalingfactor =
    ::core::ptr::null::<tjscalingfactor>() as *mut tjscalingfactor;
#[no_mangle]
pub static mut xformOpt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xformOp: ::core::ffi::c_int = TJXOP_NONE as ::core::ffi::c_int;
#[no_mangle]
pub static mut nsf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut customFilter: Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_short,
        tjregion,
        tjregion,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut tjtransform,
    ) -> ::core::ffi::c_int,
> = None;
#[no_mangle]
pub static mut warmup: ::core::ffi::c_double = 1.0f64;
#[no_mangle]
pub static mut benchTime: ::core::ffi::c_double = 5.0f64;
unsafe extern "C" fn getTime() -> ::core::ffi::c_double {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if gettimeofday(&raw mut tv, NULL) < 0 as ::core::ffi::c_int {
        return 0.0f64;
    }
    return tv.tv_sec as ::core::ffi::c_double + tv.tv_usec as ::core::ffi::c_double / 1000000.0f64;
}
unsafe extern "C" fn formatName(
    mut subsamp: ::core::ffi::c_int,
    mut cs: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if cs == TJCS_YCbCr as ::core::ffi::c_int {
        return subNameLong[subsamp as usize] as *mut ::core::ffi::c_char;
    } else if cs == TJCS_YCCK as ::core::ffi::c_int || cs == TJCS_CMYK as ::core::ffi::c_int {
        snprintf(
            buf,
            80 as size_t,
            b"%s %s\0" as *const u8 as *const ::core::ffi::c_char,
            csName[cs as usize],
            subNameLong[subsamp as usize],
        );
        return buf;
    } else {
        return csName[cs as usize] as *mut ::core::ffi::c_char;
    };
}
unsafe extern "C" fn sigfig(
    mut val: ::core::ffi::c_double,
    mut figs: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut format: [::core::ffi::c_char; 80] = [0; 80];
    let mut digitsAfterDecimal: ::core::ffi::c_int =
        figs - ceil(log10(fabs(val))) as ::core::ffi::c_int;
    if digitsAfterDecimal < 1 as ::core::ffi::c_int {
        snprintf(
            &raw mut format as *mut ::core::ffi::c_char,
            80 as size_t,
            b"%%.0f\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        snprintf(
            &raw mut format as *mut ::core::ffi::c_char,
            80 as size_t,
            b"%%.%df\0" as *const u8 as *const ::core::ffi::c_char,
            digitsAfterDecimal,
        );
    }
    snprintf(
        buf,
        len as size_t,
        &raw mut format as *mut ::core::ffi::c_char,
        val,
    );
    return buf;
}
unsafe extern "C" fn dummyDCTFilter(
    mut coeffs: *mut ::core::ffi::c_short,
    mut arrayRegion: tjregion,
    mut planeRegion: tjregion,
    mut componentIndex: ::core::ffi::c_int,
    mut transformIndex: ::core::ffi::c_int,
    mut transform: *mut tjtransform,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < arrayRegion.w * arrayRegion.h {
        *coeffs.offset(i as isize) =
            -(*coeffs.offset(i as isize) as ::core::ffi::c_int) as ::core::ffi::c_short;
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn decomp(
    mut srcBuf: *mut ::core::ffi::c_uchar,
    mut jpegBuf: *mut *mut ::core::ffi::c_uchar,
    mut jpegSize: *mut ::core::ffi::c_ulong,
    mut dstBuf: *mut ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
    mut jpegQual: ::core::ffi::c_int,
    mut fileName: *mut ::core::ffi::c_char,
    mut tilew: ::core::ffi::c_int,
    mut tileh: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut tempStr: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut sizeStr: [::core::ffi::c_char; 24] =
        ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
    let mut qualStr: [::core::ffi::c_char; 13] = ::core::mem::transmute::<
        [u8; 13],
        [::core::ffi::c_char; 13],
    >(*b"\0\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut handle: tjhandle = NULL;
    let mut row: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    let mut iter: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dstBufAlloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut elapsed: ::core::ffi::c_double = 0.;
    let mut elapsedDecode: ::core::ffi::c_double = 0.;
    let mut ps: ::core::ffi::c_int = tjPixelSize[pf as usize];
    let mut scaledw: ::core::ffi::c_int =
        (w * sf.num + sf.denom - 1 as ::core::ffi::c_int) / sf.denom;
    let mut scaledh: ::core::ffi::c_int =
        (h * sf.num + sf.denom - 1 as ::core::ffi::c_int) / sf.denom;
    let mut pitch: ::core::ffi::c_int = scaledw * ps;
    let mut ntilesw: ::core::ffi::c_int = (w + tilew - 1 as ::core::ffi::c_int) / tilew;
    let mut ntilesh: ::core::ffi::c_int = (h + tileh - 1 as ::core::ffi::c_int) / tileh;
    let mut dstPtr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dstPtr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut yuvBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if jpegQual > 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut qualStr as *mut ::core::ffi::c_char,
            13 as size_t,
            b"_Q%d\0" as *const u8 as *const ::core::ffi::c_char,
            jpegQual,
        );
        qualStr[12 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    }
    handle = tjInitDecompress();
    if handle.is_null() {
        let mut _tjErrorCode: ::core::ffi::c_int = tjGetErrorCode(handle);
        let mut _tjErrorStr: *mut ::core::ffi::c_char = tjGetErrorStr2(handle);
        if flags & TJFLAG_STOPONWARNING == 0 && _tjErrorCode == TJERR_WARNING as ::core::ffi::c_int
        {
            if strncmp(
                &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                _tjErrorStr,
                JMSG_LENGTH_MAX as size_t,
            ) != 0
                || strncmp(
                    &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                    b"executing tjInitDecompress()\0" as *const u8 as *const ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                ) != 0
                || tjErrorCode != _tjErrorCode
                || tjErrorLine != 220 as ::core::ffi::c_int
            {
                strncpy(
                    &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                    _tjErrorStr,
                    JMSG_LENGTH_MAX as size_t,
                );
                tjErrorStr[(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                    '\0' as i32 as ::core::ffi::c_char;
                strncpy(
                    &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                    b"executing tjInitDecompress()\0" as *const u8 as *const ::core::ffi::c_char,
                    JMSG_LENGTH_MAX as size_t,
                );
                tjErrorMsg[(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                    '\0' as i32 as ::core::ffi::c_char;
                tjErrorCode = _tjErrorCode;
                tjErrorLine = 220 as ::core::ffi::c_int;
                printf(
                    b"WARNING in line %d while %s:\n%s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    220 as ::core::ffi::c_int,
                    b"executing tjInitDecompress()\0" as *const u8 as *const ::core::ffi::c_char,
                    _tjErrorStr,
                );
            }
            current_block = 12124785117276362961;
        } else {
            printf(
                b"%s in line %d while %s:\n%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                if _tjErrorCode == TJERR_WARNING as ::core::ffi::c_int {
                    b"WARNING\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"ERROR\0" as *const u8 as *const ::core::ffi::c_char
                },
                220 as ::core::ffi::c_int,
                b"executing tjInitDecompress()\0" as *const u8 as *const ::core::ffi::c_char,
                _tjErrorStr,
            );
            retval = -(1 as ::core::ffi::c_int);
            current_block = 16014433924338067226;
        }
    } else {
        current_block = 12124785117276362961;
    }
    match current_block {
        12124785117276362961 => {
            if dstBuf.is_null() {
                if (pitch as ::core::ffi::c_ulonglong)
                    .wrapping_mul(scaledh as ::core::ffi::c_ulonglong)
                    > -(1 as ::core::ffi::c_int) as size_t as ::core::ffi::c_ulonglong
                {
                    printf(
                        b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        225 as ::core::ffi::c_int,
                        b"allocating destination buffer\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"Image is too large\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    retval = -(1 as ::core::ffi::c_int);
                    current_block = 16014433924338067226;
                } else {
                    dstBuf = malloc((pitch as size_t).wrapping_mul(scaledh as size_t))
                        as *mut ::core::ffi::c_uchar;
                    if dstBuf.is_null() {
                        printf(
                            b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            227 as ::core::ffi::c_int,
                            b"allocating destination buffer\0" as *const u8
                                as *const ::core::ffi::c_char,
                            strerror(*__errno_location()),
                        );
                        retval = -(1 as ::core::ffi::c_int);
                        current_block = 16014433924338067226;
                    } else {
                        dstBufAlloc = 1 as ::core::ffi::c_int;
                        current_block = 15345278821338558188;
                    }
                }
            } else {
                current_block = 15345278821338558188;
            }
            match current_block {
                16014433924338067226 => {}
                _ => {
                    memset(
                        dstBuf as *mut ::core::ffi::c_void,
                        127 as ::core::ffi::c_int,
                        (pitch as size_t).wrapping_mul(scaledh as size_t),
                    );
                    if doYUV != 0 {
                        let mut width: ::core::ffi::c_int =
                            if doTile != 0 { tilew } else { scaledw };
                        let mut height: ::core::ffi::c_int =
                            if doTile != 0 { tileh } else { scaledh };
                        let mut yuvSize: ::core::ffi::c_ulong =
                            tjBufSizeYUV2(width, yuvAlign, height, subsamp);
                        if yuvSize == -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong {
                            let mut _tjErrorCode_0: ::core::ffi::c_int = tjGetErrorCode(handle);
                            let mut _tjErrorStr_0: *mut ::core::ffi::c_char =
                                tjGetErrorStr2(handle);
                            if flags & TJFLAG_STOPONWARNING == 0
                                && _tjErrorCode_0 == TJERR_WARNING as ::core::ffi::c_int
                            {
                                if strncmp(
                                    &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                    _tjErrorStr_0,
                                    JMSG_LENGTH_MAX as size_t,
                                ) != 0
                                    || strncmp(
                                        &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                        b"allocating YUV buffer\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        JMSG_LENGTH_MAX as size_t,
                                    ) != 0
                                    || tjErrorCode != _tjErrorCode_0
                                    || tjErrorLine != 240 as ::core::ffi::c_int
                                {
                                    strncpy(
                                        &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                        _tjErrorStr_0,
                                        JMSG_LENGTH_MAX as size_t,
                                    );
                                    tjErrorStr
                                        [(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                                        '\0' as i32 as ::core::ffi::c_char;
                                    strncpy(
                                        &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                        b"allocating YUV buffer\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        JMSG_LENGTH_MAX as size_t,
                                    );
                                    tjErrorMsg
                                        [(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                                        '\0' as i32 as ::core::ffi::c_char;
                                    tjErrorCode = _tjErrorCode_0;
                                    tjErrorLine = 240 as ::core::ffi::c_int;
                                    printf(
                                        b"WARNING in line %d while %s:\n%s\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        240 as ::core::ffi::c_int,
                                        b"allocating YUV buffer\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        _tjErrorStr_0,
                                    );
                                }
                                current_block = 10758786907990354186;
                            } else {
                                printf(
                                    b"%s in line %d while %s:\n%s\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    if _tjErrorCode_0 == TJERR_WARNING as ::core::ffi::c_int {
                                        b"WARNING\0" as *const u8 as *const ::core::ffi::c_char
                                    } else {
                                        b"ERROR\0" as *const u8 as *const ::core::ffi::c_char
                                    },
                                    240 as ::core::ffi::c_int,
                                    b"allocating YUV buffer\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    _tjErrorStr_0,
                                );
                                retval = -(1 as ::core::ffi::c_int);
                                current_block = 16014433924338067226;
                            }
                        } else {
                            current_block = 10758786907990354186;
                        }
                        match current_block {
                            16014433924338067226 => {}
                            _ => {
                                yuvBuf = malloc(yuvSize as size_t) as *mut ::core::ffi::c_uchar;
                                if yuvBuf.is_null() {
                                    printf(
                                        b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        242 as ::core::ffi::c_int,
                                        b"allocating YUV buffer\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        strerror(*__errno_location()),
                                    );
                                    retval = -(1 as ::core::ffi::c_int);
                                    current_block = 16014433924338067226;
                                } else {
                                    memset(
                                        yuvBuf as *mut ::core::ffi::c_void,
                                        127 as ::core::ffi::c_int,
                                        yuvSize as size_t,
                                    );
                                    current_block = 14775119014532381840;
                                }
                            }
                        }
                    } else {
                        current_block = 14775119014532381840;
                    }
                    match current_block {
                        16014433924338067226 => {}
                        _ => {
                            iter = -(1 as ::core::ffi::c_int);
                            elapsedDecode = 0.0f64;
                            elapsed = elapsedDecode;
                            's_248: loop {
                                let mut tile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut start: ::core::ffi::c_double = getTime();
                                row = 0 as ::core::ffi::c_int;
                                dstPtr = dstBuf;
                                while row < ntilesh {
                                    col = 0 as ::core::ffi::c_int;
                                    dstPtr2 = dstPtr;
                                    while col < ntilesw {
                                        let mut width_0: ::core::ffi::c_int = if doTile != 0 {
                                            if tilew < w - col * tilew {
                                                tilew
                                            } else {
                                                w - col * tilew
                                            }
                                        } else {
                                            scaledw
                                        };
                                        let mut height_0: ::core::ffi::c_int = if doTile != 0 {
                                            if tileh < h - row * tileh {
                                                tileh
                                            } else {
                                                h - row * tileh
                                            }
                                        } else {
                                            scaledh
                                        };
                                        if doYUV != 0 {
                                            let mut startDecode: ::core::ffi::c_double = 0.;
                                            if tjDecompressToYUV2(
                                                handle,
                                                *jpegBuf.offset(tile as isize),
                                                *jpegSize.offset(tile as isize),
                                                yuvBuf,
                                                width_0,
                                                yuvAlign,
                                                height_0,
                                                flags,
                                            ) == -(1 as ::core::ffi::c_int)
                                            {
                                                let mut _tjErrorCode_1: ::core::ffi::c_int =
                                                    tjGetErrorCode(handle);
                                                let mut _tjErrorStr_1: *mut ::core::ffi::c_char =
                                                    tjGetErrorStr2(handle);
                                                if flags & TJFLAG_STOPONWARNING == 0
                                                    && _tjErrorCode_1
                                                        == TJERR_WARNING as ::core::ffi::c_int
                                                {
                                                    if strncmp(
                                                        &raw mut tjErrorStr
                                                            as *mut ::core::ffi::c_char,
                                                        _tjErrorStr_1,
                                                        JMSG_LENGTH_MAX as size_t,
                                                    ) != 0
                                                        || strncmp(
                                                            &raw mut tjErrorMsg
                                                                as *mut ::core::ffi::c_char,
                                                            b"executing tjDecompressToYUV2()\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            JMSG_LENGTH_MAX as size_t,
                                                        ) != 0
                                                        || tjErrorCode != _tjErrorCode_1
                                                        || tjErrorLine != 265 as ::core::ffi::c_int
                                                    {
                                                        strncpy(
                                                            &raw mut tjErrorStr
                                                                as *mut ::core::ffi::c_char,
                                                            _tjErrorStr_1,
                                                            JMSG_LENGTH_MAX as size_t,
                                                        );
                                                        tjErrorStr[(JMSG_LENGTH_MAX
                                                            - 1 as ::core::ffi::c_int)
                                                            as usize] =
                                                            '\0' as i32 as ::core::ffi::c_char;
                                                        strncpy(
                                                            &raw mut tjErrorMsg
                                                                as *mut ::core::ffi::c_char,
                                                            b"executing tjDecompressToYUV2()\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            JMSG_LENGTH_MAX as size_t,
                                                        );
                                                        tjErrorMsg[(JMSG_LENGTH_MAX
                                                            - 1 as ::core::ffi::c_int)
                                                            as usize] =
                                                            '\0' as i32 as ::core::ffi::c_char;
                                                        tjErrorCode = _tjErrorCode_1;
                                                        tjErrorLine = 265 as ::core::ffi::c_int;
                                                        printf(
                                                            b"WARNING in line %d while %s:\n%s\n\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            265 as ::core::ffi::c_int,
                                                            b"executing tjDecompressToYUV2()\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            _tjErrorStr_1,
                                                        );
                                                    }
                                                } else {
                                                    printf(
                                                        b"%s in line %d while %s:\n%s\n\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        if _tjErrorCode_1
                                                            == TJERR_WARNING as ::core::ffi::c_int
                                                        {
                                                            b"WARNING\0" as *const u8
                                                                as *const ::core::ffi::c_char
                                                        } else {
                                                            b"ERROR\0" as *const u8
                                                                as *const ::core::ffi::c_char
                                                        },
                                                        265 as ::core::ffi::c_int,
                                                        b"executing tjDecompressToYUV2()\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        _tjErrorStr_1,
                                                    );
                                                    retval = -(1 as ::core::ffi::c_int);
                                                    current_block = 16014433924338067226;
                                                    break 's_248;
                                                }
                                            }
                                            startDecode = getTime();
                                            if tjDecodeYUV(
                                                handle, yuvBuf, yuvAlign, subsamp, dstPtr2,
                                                width_0, pitch, height_0, pf, flags,
                                            ) == -(1 as ::core::ffi::c_int)
                                            {
                                                let mut _tjErrorCode_2: ::core::ffi::c_int =
                                                    tjGetErrorCode(handle);
                                                let mut _tjErrorStr_2: *mut ::core::ffi::c_char =
                                                    tjGetErrorStr2(handle);
                                                if flags & TJFLAG_STOPONWARNING == 0
                                                    && _tjErrorCode_2
                                                        == TJERR_WARNING as ::core::ffi::c_int
                                                {
                                                    if strncmp(
                                                        &raw mut tjErrorStr
                                                            as *mut ::core::ffi::c_char,
                                                        _tjErrorStr_2,
                                                        JMSG_LENGTH_MAX as size_t,
                                                    ) != 0
                                                        || strncmp(
                                                            &raw mut tjErrorMsg
                                                                as *mut ::core::ffi::c_char,
                                                            b"executing tjDecodeYUV()\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            JMSG_LENGTH_MAX as size_t,
                                                        ) != 0
                                                        || tjErrorCode != _tjErrorCode_2
                                                        || tjErrorLine != 269 as ::core::ffi::c_int
                                                    {
                                                        strncpy(
                                                            &raw mut tjErrorStr
                                                                as *mut ::core::ffi::c_char,
                                                            _tjErrorStr_2,
                                                            JMSG_LENGTH_MAX as size_t,
                                                        );
                                                        tjErrorStr[(JMSG_LENGTH_MAX
                                                            - 1 as ::core::ffi::c_int)
                                                            as usize] =
                                                            '\0' as i32 as ::core::ffi::c_char;
                                                        strncpy(
                                                            &raw mut tjErrorMsg
                                                                as *mut ::core::ffi::c_char,
                                                            b"executing tjDecodeYUV()\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            JMSG_LENGTH_MAX as size_t,
                                                        );
                                                        tjErrorMsg[(JMSG_LENGTH_MAX
                                                            - 1 as ::core::ffi::c_int)
                                                            as usize] =
                                                            '\0' as i32 as ::core::ffi::c_char;
                                                        tjErrorCode = _tjErrorCode_2;
                                                        tjErrorLine = 269 as ::core::ffi::c_int;
                                                        printf(
                                                            b"WARNING in line %d while %s:\n%s\n\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            269 as ::core::ffi::c_int,
                                                            b"executing tjDecodeYUV()\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            _tjErrorStr_2,
                                                        );
                                                    }
                                                } else {
                                                    printf(
                                                        b"%s in line %d while %s:\n%s\n\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        if _tjErrorCode_2
                                                            == TJERR_WARNING as ::core::ffi::c_int
                                                        {
                                                            b"WARNING\0" as *const u8
                                                                as *const ::core::ffi::c_char
                                                        } else {
                                                            b"ERROR\0" as *const u8
                                                                as *const ::core::ffi::c_char
                                                        },
                                                        269 as ::core::ffi::c_int,
                                                        b"executing tjDecodeYUV()\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        _tjErrorStr_2,
                                                    );
                                                    retval = -(1 as ::core::ffi::c_int);
                                                    current_block = 16014433924338067226;
                                                    break 's_248;
                                                }
                                            }
                                            if iter >= 0 as ::core::ffi::c_int {
                                                elapsedDecode += getTime() - startDecode;
                                            }
                                        } else if tjDecompress2(
                                            handle,
                                            *jpegBuf.offset(tile as isize),
                                            *jpegSize.offset(tile as isize),
                                            dstPtr2,
                                            width_0,
                                            pitch,
                                            height_0,
                                            pf,
                                            flags,
                                        ) == -(1 as ::core::ffi::c_int)
                                        {
                                            let mut _tjErrorCode_3: ::core::ffi::c_int =
                                                tjGetErrorCode(handle);
                                            let mut _tjErrorStr_3: *mut ::core::ffi::c_char =
                                                tjGetErrorStr2(handle);
                                            if flags & TJFLAG_STOPONWARNING == 0
                                                && _tjErrorCode_3
                                                    == TJERR_WARNING as ::core::ffi::c_int
                                            {
                                                if strncmp(
                                                    &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                                    _tjErrorStr_3,
                                                    JMSG_LENGTH_MAX as size_t,
                                                ) != 0
                                                    || strncmp(
                                                        &raw mut tjErrorMsg
                                                            as *mut ::core::ffi::c_char,
                                                        b"executing tjDecompress2()\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        JMSG_LENGTH_MAX as size_t,
                                                    ) != 0
                                                    || tjErrorCode != _tjErrorCode_3
                                                    || tjErrorLine != 274 as ::core::ffi::c_int
                                                {
                                                    strncpy(
                                                        &raw mut tjErrorStr
                                                            as *mut ::core::ffi::c_char,
                                                        _tjErrorStr_3,
                                                        JMSG_LENGTH_MAX as size_t,
                                                    );
                                                    tjErrorStr[(JMSG_LENGTH_MAX
                                                        - 1 as ::core::ffi::c_int)
                                                        as usize] =
                                                        '\0' as i32 as ::core::ffi::c_char;
                                                    strncpy(
                                                        &raw mut tjErrorMsg
                                                            as *mut ::core::ffi::c_char,
                                                        b"executing tjDecompress2()\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        JMSG_LENGTH_MAX as size_t,
                                                    );
                                                    tjErrorMsg[(JMSG_LENGTH_MAX
                                                        - 1 as ::core::ffi::c_int)
                                                        as usize] =
                                                        '\0' as i32 as ::core::ffi::c_char;
                                                    tjErrorCode = _tjErrorCode_3;
                                                    tjErrorLine = 274 as ::core::ffi::c_int;
                                                    printf(
                                                        b"WARNING in line %d while %s:\n%s\n\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        274 as ::core::ffi::c_int,
                                                        b"executing tjDecompress2()\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        _tjErrorStr_3,
                                                    );
                                                }
                                            } else {
                                                printf(
                                                    b"%s in line %d while %s:\n%s\n\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    if _tjErrorCode_3
                                                        == TJERR_WARNING as ::core::ffi::c_int
                                                    {
                                                        b"WARNING\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    } else {
                                                        b"ERROR\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    },
                                                    274 as ::core::ffi::c_int,
                                                    b"executing tjDecompress2()\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    _tjErrorStr_3,
                                                );
                                                retval = -(1 as ::core::ffi::c_int);
                                                current_block = 16014433924338067226;
                                                break 's_248;
                                            }
                                        }
                                        col += 1;
                                        tile += 1;
                                        dstPtr2 = dstPtr2.offset((ps * tilew) as isize);
                                    }
                                    row += 1;
                                    dstPtr = dstPtr
                                        .offset((pitch as size_t).wrapping_mul(tileh as size_t)
                                            as isize);
                                }
                                elapsed += getTime() - start;
                                if iter >= 0 as ::core::ffi::c_int {
                                    iter += 1;
                                    if elapsed >= benchTime {
                                        current_block = 7739940392431776979;
                                        break;
                                    }
                                } else if elapsed >= warmup {
                                    iter = 0 as ::core::ffi::c_int;
                                    elapsedDecode = 0.0f64;
                                    elapsed = elapsedDecode;
                                }
                            }
                            match current_block {
                                16014433924338067226 => {}
                                _ => {
                                    if doYUV != 0 {
                                        elapsed -= elapsedDecode;
                                    }
                                    if tjDestroy(handle) == -(1 as ::core::ffi::c_int) {
                                        let mut _tjErrorCode_4: ::core::ffi::c_int =
                                            tjGetErrorCode(handle);
                                        let mut _tjErrorStr_4: *mut ::core::ffi::c_char =
                                            tjGetErrorStr2(handle);
                                        if flags & TJFLAG_STOPONWARNING == 0
                                            && _tjErrorCode_4 == TJERR_WARNING as ::core::ffi::c_int
                                        {
                                            if strncmp(
                                                &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                                _tjErrorStr_4,
                                                JMSG_LENGTH_MAX as size_t,
                                            ) != 0
                                                || strncmp(
                                                    &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                                    b"executing tjDestroy()\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    JMSG_LENGTH_MAX as size_t,
                                                ) != 0
                                                || tjErrorCode != _tjErrorCode_4
                                                || tjErrorLine != 288 as ::core::ffi::c_int
                                            {
                                                strncpy(
                                                    &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                                    _tjErrorStr_4,
                                                    JMSG_LENGTH_MAX as size_t,
                                                );
                                                tjErrorStr[(JMSG_LENGTH_MAX
                                                    - 1 as ::core::ffi::c_int)
                                                    as usize] = '\0' as i32 as ::core::ffi::c_char;
                                                strncpy(
                                                    &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                                    b"executing tjDestroy()\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    JMSG_LENGTH_MAX as size_t,
                                                );
                                                tjErrorMsg[(JMSG_LENGTH_MAX
                                                    - 1 as ::core::ffi::c_int)
                                                    as usize] = '\0' as i32 as ::core::ffi::c_char;
                                                tjErrorCode = _tjErrorCode_4;
                                                tjErrorLine = 288 as ::core::ffi::c_int;
                                                printf(
                                                    b"WARNING in line %d while %s:\n%s\n\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    288 as ::core::ffi::c_int,
                                                    b"executing tjDestroy()\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    _tjErrorStr_4,
                                                );
                                            }
                                            current_block = 12963528325254160332;
                                        } else {
                                            printf(
                                                b"%s in line %d while %s:\n%s\n\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                if _tjErrorCode_4
                                                    == TJERR_WARNING as ::core::ffi::c_int
                                                {
                                                    b"WARNING\0" as *const u8
                                                        as *const ::core::ffi::c_char
                                                } else {
                                                    b"ERROR\0" as *const u8
                                                        as *const ::core::ffi::c_char
                                                },
                                                288 as ::core::ffi::c_int,
                                                b"executing tjDestroy()\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                _tjErrorStr_4,
                                            );
                                            retval = -(1 as ::core::ffi::c_int);
                                            current_block = 16014433924338067226;
                                        }
                                    } else {
                                        current_block = 12963528325254160332;
                                    }
                                    match current_block {
                                        16014433924338067226 => {}
                                        _ => {
                                            handle = NULL as tjhandle;
                                            if quiet != 0 {
                                                printf(
                                                    b"%-6s%s\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    sigfig(
                                                        (w * h) as ::core::ffi::c_double
                                                            / 1000000.0f64
                                                            * iter as ::core::ffi::c_double
                                                            / elapsed,
                                                        4 as ::core::ffi::c_int,
                                                        &raw mut tempStr
                                                            as *mut ::core::ffi::c_char,
                                                        1024 as ::core::ffi::c_int,
                                                    ),
                                                    if quiet == 2 as ::core::ffi::c_int {
                                                        b"\n\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    } else {
                                                        b"  \0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    },
                                                );
                                                if doYUV != 0 {
                                                    printf(
                                                        b"%s\n\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        sigfig(
                                                            (w * h) as ::core::ffi::c_double
                                                                / 1000000.0f64
                                                                * iter as ::core::ffi::c_double
                                                                / elapsedDecode,
                                                            4 as ::core::ffi::c_int,
                                                            &raw mut tempStr
                                                                as *mut ::core::ffi::c_char,
                                                            1024 as ::core::ffi::c_int,
                                                        ),
                                                    );
                                                } else if quiet != 2 as ::core::ffi::c_int {
                                                    printf(
                                                        b"\n\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                }
                                            } else {
                                                printf(
                                                    b"%s --> Frame rate:         %f fps\n\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    if doYUV != 0 {
                                                        b"Decomp to YUV\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    } else {
                                                        b"Decompress   \0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    },
                                                    iter as ::core::ffi::c_double / elapsed,
                                                );
                                                printf(
                                                    b"                  Throughput:         %f Megapixels/sec\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char,
                                                    (w * h) as ::core::ffi::c_double / 1000000.0f64
                                                        * iter as ::core::ffi::c_double / elapsed,
                                                );
                                                if doYUV != 0 {
                                                    printf(
                                                        b"YUV Decode    --> Frame rate:         %f fps\n\0"
                                                            as *const u8 as *const ::core::ffi::c_char,
                                                        iter as ::core::ffi::c_double / elapsedDecode,
                                                    );
                                                    printf(
                                                        b"                  Throughput:         %f Megapixels/sec\n\0"
                                                            as *const u8 as *const ::core::ffi::c_char,
                                                        (w * h) as ::core::ffi::c_double / 1000000.0f64
                                                            * iter as ::core::ffi::c_double / elapsedDecode,
                                                    );
                                                }
                                            }
                                            if !(doWrite == 0) {
                                                if sf.num != 1 as ::core::ffi::c_int
                                                    || sf.denom != 1 as ::core::ffi::c_int
                                                {
                                                    snprintf(
                                                        &raw mut sizeStr
                                                            as *mut ::core::ffi::c_char,
                                                        24 as size_t,
                                                        b"%d_%d\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        sf.num,
                                                        sf.denom,
                                                    );
                                                } else if tilew != w || tileh != h {
                                                    snprintf(
                                                        &raw mut sizeStr
                                                            as *mut ::core::ffi::c_char,
                                                        24 as size_t,
                                                        b"%dx%d\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        tilew,
                                                        tileh,
                                                    );
                                                } else {
                                                    snprintf(
                                                        &raw mut sizeStr
                                                            as *mut ::core::ffi::c_char,
                                                        24 as size_t,
                                                        b"full\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                }
                                                if decompOnly != 0 {
                                                    snprintf(
                                                        &raw mut tempStr
                                                            as *mut ::core::ffi::c_char,
                                                        1024 as size_t,
                                                        b"%s_%s.%s\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        fileName,
                                                        &raw mut sizeStr
                                                            as *mut ::core::ffi::c_char,
                                                        ext,
                                                    );
                                                } else {
                                                    snprintf(
                                                        &raw mut tempStr
                                                            as *mut ::core::ffi::c_char,
                                                        1024 as size_t,
                                                        b"%s_%s%s_%s.%s\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        fileName,
                                                        subName[subsamp as usize],
                                                        &raw mut qualStr
                                                            as *mut ::core::ffi::c_char,
                                                        &raw mut sizeStr
                                                            as *mut ::core::ffi::c_char,
                                                        ext,
                                                    );
                                                }
                                                if tjSaveImage(
                                                    &raw mut tempStr as *mut ::core::ffi::c_char,
                                                    dstBuf,
                                                    scaledw,
                                                    0 as ::core::ffi::c_int,
                                                    scaledh,
                                                    pf,
                                                    flags,
                                                ) == -(1 as ::core::ffi::c_int)
                                                {
                                                    printf(
                                                        b"ERROR in line %d while %s:\n%s\n\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        328 as ::core::ffi::c_int,
                                                        b"saving output image\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        tjGetErrorStr2(NULL),
                                                    );
                                                    retval = -(1 as ::core::ffi::c_int);
                                                } else {
                                                    ptr = strrchr(
                                                        &raw mut tempStr
                                                            as *mut ::core::ffi::c_char,
                                                        '.' as i32,
                                                    );
                                                    snprintf(
                                                        ptr,
                                                        (1024 as ::core::ffi::c_long
                                                            - ptr.offset_from(
                                                                &raw mut tempStr
                                                                    as *mut ::core::ffi::c_char,
                                                            )
                                                                as ::core::ffi::c_long)
                                                            as size_t,
                                                        b"-err.%s\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        ext,
                                                    );
                                                    if !srcBuf.is_null()
                                                        && sf.num == 1 as ::core::ffi::c_int
                                                        && sf.denom == 1 as ::core::ffi::c_int
                                                    {
                                                        if quiet == 0 {
                                                            printf(
                                                                b"Compression error written to %s.\n\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                &raw mut tempStr as *mut ::core::ffi::c_char,
                                                            );
                                                        }
                                                        if subsamp
                                                            == TJSAMP_GRAY as ::core::ffi::c_int
                                                        {
                                                            let mut index: ::core::ffi::c_ulong = 0;
                                                            let mut index2: ::core::ffi::c_ulong =
                                                                0;
                                                            row = 0 as ::core::ffi::c_int;
                                                            index = 0 as ::core::ffi::c_ulong;
                                                            while row < h {
                                                                col = 0 as ::core::ffi::c_int;
                                                                index2 = index;
                                                                while col < w {
                                                                    let mut rindex: ::core::ffi::c_ulong = index2
                                                                        .wrapping_add(
                                                                            tjRedOffset[pf as usize] as ::core::ffi::c_ulong,
                                                                        );
                                                                    let mut gindex: ::core::ffi::c_ulong = index2
                                                                        .wrapping_add(
                                                                            tjGreenOffset[pf as usize] as ::core::ffi::c_ulong,
                                                                        );
                                                                    let mut bindex: ::core::ffi::c_ulong = index2
                                                                        .wrapping_add(
                                                                            tjBlueOffset[pf as usize] as ::core::ffi::c_ulong,
                                                                        );
                                                                    let mut y: ::core::ffi::c_int = (*srcBuf
                                                                        .offset(rindex as isize) as ::core::ffi::c_double * 0.299f64
                                                                        + *srcBuf.offset(gindex as isize) as ::core::ffi::c_double
                                                                            * 0.587f64
                                                                        + *srcBuf.offset(bindex as isize) as ::core::ffi::c_double
                                                                            * 0.114f64 + 0.5f64) as ::core::ffi::c_int;
                                                                    if y > 255 as ::core::ffi::c_int
                                                                    {
                                                                        y = 255
                                                                            as ::core::ffi::c_int;
                                                                    }
                                                                    if y < 0 as ::core::ffi::c_int {
                                                                        y = 0 as ::core::ffi::c_int;
                                                                    }
                                                                    *dstBuf
                                                                        .offset(rindex as isize) =
                                                                        abs(*dstBuf
                                                                            .offset(rindex as isize)
                                                                            as ::core::ffi::c_int
                                                                            - y)
                                                                            as ::core::ffi::c_uchar;
                                                                    *dstBuf
                                                                        .offset(gindex as isize) =
                                                                        abs(*dstBuf
                                                                            .offset(gindex as isize)
                                                                            as ::core::ffi::c_int
                                                                            - y)
                                                                            as ::core::ffi::c_uchar;
                                                                    *dstBuf
                                                                        .offset(bindex as isize) =
                                                                        abs(*dstBuf
                                                                            .offset(bindex as isize)
                                                                            as ::core::ffi::c_int
                                                                            - y)
                                                                            as ::core::ffi::c_uchar;
                                                                    col += 1;
                                                                    index2 = index2.wrapping_add(
                                                                        ps as ::core::ffi::c_ulong,
                                                                    );
                                                                }
                                                                row += 1;
                                                                index = index.wrapping_add(
                                                                    pitch as ::core::ffi::c_ulong,
                                                                );
                                                            }
                                                        } else {
                                                            row = 0 as ::core::ffi::c_int;
                                                            while row < h {
                                                                col = 0 as ::core::ffi::c_int;
                                                                while col < w * ps {
                                                                    *dstBuf.offset(
                                                                        (pitch * row + col)
                                                                            as isize,
                                                                    ) = abs(*dstBuf.offset(
                                                                        (pitch * row + col)
                                                                            as isize,
                                                                    )
                                                                        as ::core::ffi::c_int
                                                                        - *srcBuf.offset(
                                                                            (pitch * row + col)
                                                                                as isize,
                                                                        )
                                                                            as ::core::ffi::c_int)
                                                                        as ::core::ffi::c_uchar;
                                                                    col += 1;
                                                                }
                                                                row += 1;
                                                            }
                                                        }
                                                        if tjSaveImage(
                                                            &raw mut tempStr
                                                                as *mut ::core::ffi::c_char,
                                                            dstBuf,
                                                            w,
                                                            0 as ::core::ffi::c_int,
                                                            h,
                                                            pf,
                                                            flags,
                                                        ) == -(1 as ::core::ffi::c_int)
                                                        {
                                                            printf(
                                                                b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                360 as ::core::ffi::c_int,
                                                                b"saving output image\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                tjGetErrorStr2(NULL),
                                                            );
                                                            retval = -(1 as ::core::ffi::c_int);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !file.is_null() {
        fclose(file);
    }
    if !handle.is_null() {
        tjDestroy(handle);
    }
    if dstBufAlloc != 0 {
        free(dstBuf as *mut ::core::ffi::c_void);
    }
    free(yuvBuf as *mut ::core::ffi::c_void);
    return retval;
}
unsafe extern "C" fn fullTest(
    mut srcBuf: *mut ::core::ffi::c_uchar,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut subsamp: ::core::ffi::c_int,
    mut jpegQual: ::core::ffi::c_int,
    mut fileName: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tempStr: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tempStr2: [::core::ffi::c_char; 80] = [0; 80];
    let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut handle: tjhandle = NULL;
    let mut jpegBuf: *mut *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
    let mut yuvBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut tmpBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut srcPtr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut srcPtr2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut start: ::core::ffi::c_double = 0.;
    let mut elapsed: ::core::ffi::c_double = 0.;
    let mut elapsedEncode: ::core::ffi::c_double = 0.;
    let mut totalJpegSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut row: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut tilew: ::core::ffi::c_int = w;
    let mut tileh: ::core::ffi::c_int = h;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iter: ::core::ffi::c_int = 0;
    let mut jpegSize: *mut ::core::ffi::c_ulong = ::core::ptr::null_mut::<::core::ffi::c_ulong>();
    let mut yuvSize: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut ps: ::core::ffi::c_int = tjPixelSize[pf as usize];
    let mut ntilesw: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut ntilesh: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut pitch: ::core::ffi::c_int = w * ps;
    let mut pfStr: *const ::core::ffi::c_char = pixFormatStr[pf as usize];
    if (pitch as ::core::ffi::c_ulonglong).wrapping_mul(h as ::core::ffi::c_ulonglong)
        > -(1 as ::core::ffi::c_int) as size_t as ::core::ffi::c_ulonglong
    {
        printf(
            b"ERROR in line %d while %s:\n%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            390 as ::core::ffi::c_int,
            b"allocating temporary image buffer\0" as *const u8 as *const ::core::ffi::c_char,
            b"Image is too large\0" as *const u8 as *const ::core::ffi::c_char,
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        tmpBuf = malloc((pitch as size_t).wrapping_mul(h as size_t)) as *mut ::core::ffi::c_uchar;
        if tmpBuf.is_null() {
            printf(
                b"ERROR in line %d while %s:\n%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                392 as ::core::ffi::c_int,
                b"allocating temporary image buffer\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            retval = -(1 as ::core::ffi::c_int);
        } else {
            if quiet == 0 {
                printf(
                    b">>>>>  %s (%s) <--> JPEG %s Q%d  <<<<<\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    pfStr,
                    if flags & TJFLAG_BOTTOMUP != 0 {
                        b"Bottom-up\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"Top-down\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    subNameLong[subsamp as usize],
                    jpegQual,
                );
            }
            tilew = (if doTile != 0 {
                8 as ::core::ffi::c_int
            } else {
                w
            });
            tileh = (if doTile != 0 {
                8 as ::core::ffi::c_int
            } else {
                h
            });
            's_71: loop {
                if tilew > w {
                    tilew = w;
                }
                if tileh > h {
                    tileh = h;
                }
                ntilesw = (w + tilew - 1 as ::core::ffi::c_int) / tilew;
                ntilesh = (h + tileh - 1 as ::core::ffi::c_int) / tileh;
                jpegBuf = malloc(
                    (::core::mem::size_of::<*mut ::core::ffi::c_uchar>() as size_t)
                        .wrapping_mul(ntilesw as size_t)
                        .wrapping_mul(ntilesh as size_t),
                ) as *mut *mut ::core::ffi::c_uchar;
                if jpegBuf.is_null() {
                    printf(
                        b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        408 as ::core::ffi::c_int,
                        b"allocating JPEG tile array\0" as *const u8 as *const ::core::ffi::c_char,
                        strerror(*__errno_location()),
                    );
                    retval = -(1 as ::core::ffi::c_int);
                    break;
                } else {
                    memset(
                        jpegBuf as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (::core::mem::size_of::<*mut ::core::ffi::c_uchar>() as size_t)
                            .wrapping_mul(ntilesw as size_t)
                            .wrapping_mul(ntilesh as size_t),
                    );
                    jpegSize = malloc(
                        (::core::mem::size_of::<::core::ffi::c_ulong>() as size_t)
                            .wrapping_mul(ntilesw as size_t)
                            .wrapping_mul(ntilesh as size_t),
                    ) as *mut ::core::ffi::c_ulong;
                    if jpegSize.is_null() {
                        printf(
                            b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            412 as ::core::ffi::c_int,
                            b"allocating JPEG size array\0" as *const u8
                                as *const ::core::ffi::c_char,
                            strerror(*__errno_location()),
                        );
                        retval = -(1 as ::core::ffi::c_int);
                        break;
                    } else {
                        memset(
                            jpegSize as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (::core::mem::size_of::<::core::ffi::c_ulong>() as size_t)
                                .wrapping_mul(ntilesw as size_t)
                                .wrapping_mul(ntilesh as size_t),
                        );
                        if flags & TJFLAG_NOREALLOC != 0 as ::core::ffi::c_int {
                            i = 0 as ::core::ffi::c_int;
                            while i < ntilesw * ntilesh {
                                if tjBufSize(tilew, tileh, subsamp)
                                    > INT_MAX as ::core::ffi::c_ulong
                                {
                                    printf(
                                        b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        418 as ::core::ffi::c_int,
                                        b"getting buffer size\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        b"Image is too large\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    retval = -(1 as ::core::ffi::c_int);
                                    break 's_71;
                                } else {
                                    let ref mut fresh0 = *jpegBuf.offset(i as isize);
                                    *fresh0 = tjAlloc(
                                        tjBufSize(tilew, tileh, subsamp) as ::core::ffi::c_int
                                    );
                                    if (*fresh0).is_null() {
                                        printf(
                                            b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            421 as ::core::ffi::c_int,
                                            b"allocating JPEG tiles\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            strerror(*__errno_location()),
                                        );
                                        retval = -(1 as ::core::ffi::c_int);
                                        break 's_71;
                                    } else {
                                        i += 1;
                                    }
                                }
                            }
                        }
                        if quiet == 1 as ::core::ffi::c_int {
                            printf(
                                b"%-4s (%s)  %-5s    %-3d   \0" as *const u8
                                    as *const ::core::ffi::c_char,
                                pfStr,
                                if flags & TJFLAG_BOTTOMUP != 0 {
                                    b"BU\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"TD\0" as *const u8 as *const ::core::ffi::c_char
                                },
                                subNameLong[subsamp as usize],
                                jpegQual,
                            );
                        }
                        i = 0 as ::core::ffi::c_int;
                        while i < h {
                            memcpy(
                                tmpBuf.offset((pitch * i) as isize) as *mut ::core::ffi::c_uchar
                                    as *mut ::core::ffi::c_void,
                                srcBuf.offset((w * ps * i) as isize) as *mut ::core::ffi::c_uchar
                                    as *const ::core::ffi::c_void,
                                (w * ps) as size_t,
                            );
                            i += 1;
                        }
                        handle = tjInitCompress();
                        if handle.is_null() {
                            let mut _tjErrorCode: ::core::ffi::c_int = tjGetErrorCode(handle);
                            let mut _tjErrorStr: *mut ::core::ffi::c_char = tjGetErrorStr2(handle);
                            if flags & TJFLAG_STOPONWARNING == 0
                                && _tjErrorCode == TJERR_WARNING as ::core::ffi::c_int
                            {
                                if strncmp(
                                    &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                    _tjErrorStr,
                                    JMSG_LENGTH_MAX as size_t,
                                ) != 0
                                    || strncmp(
                                        &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                        b"executing tjInitCompress()\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        JMSG_LENGTH_MAX as size_t,
                                    ) != 0
                                    || tjErrorCode != _tjErrorCode
                                    || tjErrorLine != 432 as ::core::ffi::c_int
                                {
                                    strncpy(
                                        &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                        _tjErrorStr,
                                        JMSG_LENGTH_MAX as size_t,
                                    );
                                    tjErrorStr
                                        [(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                                        '\0' as i32 as ::core::ffi::c_char;
                                    strncpy(
                                        &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                        b"executing tjInitCompress()\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        JMSG_LENGTH_MAX as size_t,
                                    );
                                    tjErrorMsg
                                        [(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                                        '\0' as i32 as ::core::ffi::c_char;
                                    tjErrorCode = _tjErrorCode;
                                    tjErrorLine = 432 as ::core::ffi::c_int;
                                    printf(
                                        b"WARNING in line %d while %s:\n%s\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        432 as ::core::ffi::c_int,
                                        b"executing tjInitCompress()\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        _tjErrorStr,
                                    );
                                }
                            } else {
                                printf(
                                    b"%s in line %d while %s:\n%s\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    if _tjErrorCode == TJERR_WARNING as ::core::ffi::c_int {
                                        b"WARNING\0" as *const u8 as *const ::core::ffi::c_char
                                    } else {
                                        b"ERROR\0" as *const u8 as *const ::core::ffi::c_char
                                    },
                                    432 as ::core::ffi::c_int,
                                    b"executing tjInitCompress()\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    _tjErrorStr,
                                );
                                retval = -(1 as ::core::ffi::c_int);
                                break;
                            }
                        }
                        if doYUV != 0 {
                            yuvSize = tjBufSizeYUV2(tilew, yuvAlign, tileh, subsamp);
                            if yuvSize == -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong {
                                let mut _tjErrorCode_0: ::core::ffi::c_int = tjGetErrorCode(handle);
                                let mut _tjErrorStr_0: *mut ::core::ffi::c_char =
                                    tjGetErrorStr2(handle);
                                if flags & TJFLAG_STOPONWARNING == 0
                                    && _tjErrorCode_0 == TJERR_WARNING as ::core::ffi::c_int
                                {
                                    if strncmp(
                                        &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                        _tjErrorStr_0,
                                        JMSG_LENGTH_MAX as size_t,
                                    ) != 0
                                        || strncmp(
                                            &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                            b"allocating YUV buffer\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            JMSG_LENGTH_MAX as size_t,
                                        ) != 0
                                        || tjErrorCode != _tjErrorCode_0
                                        || tjErrorLine != 437 as ::core::ffi::c_int
                                    {
                                        strncpy(
                                            &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                            _tjErrorStr_0,
                                            JMSG_LENGTH_MAX as size_t,
                                        );
                                        tjErrorStr[(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int)
                                            as usize] = '\0' as i32 as ::core::ffi::c_char;
                                        strncpy(
                                            &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                            b"allocating YUV buffer\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            JMSG_LENGTH_MAX as size_t,
                                        );
                                        tjErrorMsg[(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int)
                                            as usize] = '\0' as i32 as ::core::ffi::c_char;
                                        tjErrorCode = _tjErrorCode_0;
                                        tjErrorLine = 437 as ::core::ffi::c_int;
                                        printf(
                                            b"WARNING in line %d while %s:\n%s\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            437 as ::core::ffi::c_int,
                                            b"allocating YUV buffer\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            _tjErrorStr_0,
                                        );
                                    }
                                } else {
                                    printf(
                                        b"%s in line %d while %s:\n%s\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        if _tjErrorCode_0 == TJERR_WARNING as ::core::ffi::c_int {
                                            b"WARNING\0" as *const u8 as *const ::core::ffi::c_char
                                        } else {
                                            b"ERROR\0" as *const u8 as *const ::core::ffi::c_char
                                        },
                                        437 as ::core::ffi::c_int,
                                        b"allocating YUV buffer\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        _tjErrorStr_0,
                                    );
                                    retval = -(1 as ::core::ffi::c_int);
                                    break;
                                }
                            }
                            yuvBuf = malloc(yuvSize as size_t) as *mut ::core::ffi::c_uchar;
                            if yuvBuf.is_null() {
                                printf(
                                    b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    439 as ::core::ffi::c_int,
                                    b"allocating YUV buffer\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    strerror(*__errno_location()),
                                );
                                retval = -(1 as ::core::ffi::c_int);
                                break;
                            } else {
                                memset(
                                    yuvBuf as *mut ::core::ffi::c_void,
                                    127 as ::core::ffi::c_int,
                                    yuvSize as size_t,
                                );
                            }
                        }
                        iter = -(1 as ::core::ffi::c_int);
                        elapsedEncode = 0.0f64;
                        elapsed = elapsedEncode;
                        loop {
                            let mut tile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            totalJpegSize = 0 as ::core::ffi::c_int;
                            start = getTime();
                            row = 0 as ::core::ffi::c_int;
                            srcPtr = srcBuf;
                            while row < ntilesh {
                                col = 0 as ::core::ffi::c_int;
                                srcPtr2 = srcPtr;
                                while col < ntilesw {
                                    let mut width: ::core::ffi::c_int = if tilew < w - col * tilew {
                                        tilew
                                    } else {
                                        w - col * tilew
                                    };
                                    let mut height: ::core::ffi::c_int = if tileh < h - row * tileh
                                    {
                                        tileh
                                    } else {
                                        h - row * tileh
                                    };
                                    if doYUV != 0 {
                                        let mut startEncode: ::core::ffi::c_double = getTime();
                                        if tjEncodeYUV3(
                                            handle, srcPtr2, width, pitch, height, pf, yuvBuf,
                                            yuvAlign, subsamp, flags,
                                        ) == -(1 as ::core::ffi::c_int)
                                        {
                                            let mut _tjErrorCode_1: ::core::ffi::c_int =
                                                tjGetErrorCode(handle);
                                            let mut _tjErrorStr_1: *mut ::core::ffi::c_char =
                                                tjGetErrorStr2(handle);
                                            if flags & TJFLAG_STOPONWARNING == 0
                                                && _tjErrorCode_1
                                                    == TJERR_WARNING as ::core::ffi::c_int
                                            {
                                                if strncmp(
                                                    &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                                    _tjErrorStr_1,
                                                    JMSG_LENGTH_MAX as size_t,
                                                ) != 0
                                                    || strncmp(
                                                        &raw mut tjErrorMsg
                                                            as *mut ::core::ffi::c_char,
                                                        b"executing tjEncodeYUV3()\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        JMSG_LENGTH_MAX as size_t,
                                                    ) != 0
                                                    || tjErrorCode != _tjErrorCode_1
                                                    || tjErrorLine != 463 as ::core::ffi::c_int
                                                {
                                                    strncpy(
                                                        &raw mut tjErrorStr
                                                            as *mut ::core::ffi::c_char,
                                                        _tjErrorStr_1,
                                                        JMSG_LENGTH_MAX as size_t,
                                                    );
                                                    tjErrorStr[(JMSG_LENGTH_MAX
                                                        - 1 as ::core::ffi::c_int)
                                                        as usize] =
                                                        '\0' as i32 as ::core::ffi::c_char;
                                                    strncpy(
                                                        &raw mut tjErrorMsg
                                                            as *mut ::core::ffi::c_char,
                                                        b"executing tjEncodeYUV3()\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        JMSG_LENGTH_MAX as size_t,
                                                    );
                                                    tjErrorMsg[(JMSG_LENGTH_MAX
                                                        - 1 as ::core::ffi::c_int)
                                                        as usize] =
                                                        '\0' as i32 as ::core::ffi::c_char;
                                                    tjErrorCode = _tjErrorCode_1;
                                                    tjErrorLine = 463 as ::core::ffi::c_int;
                                                    printf(
                                                        b"WARNING in line %d while %s:\n%s\n\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        463 as ::core::ffi::c_int,
                                                        b"executing tjEncodeYUV3()\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        _tjErrorStr_1,
                                                    );
                                                }
                                            } else {
                                                printf(
                                                    b"%s in line %d while %s:\n%s\n\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    if _tjErrorCode_1
                                                        == TJERR_WARNING as ::core::ffi::c_int
                                                    {
                                                        b"WARNING\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    } else {
                                                        b"ERROR\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    },
                                                    463 as ::core::ffi::c_int,
                                                    b"executing tjEncodeYUV3()\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    _tjErrorStr_1,
                                                );
                                                retval = -(1 as ::core::ffi::c_int);
                                                break 's_71;
                                            }
                                        }
                                        if iter >= 0 as ::core::ffi::c_int {
                                            elapsedEncode += getTime() - startEncode;
                                        }
                                        if tjCompressFromYUV(
                                            handle,
                                            yuvBuf,
                                            width,
                                            yuvAlign,
                                            height,
                                            subsamp,
                                            jpegBuf.offset(tile as isize)
                                                as *mut *mut ::core::ffi::c_uchar,
                                            jpegSize.offset(tile as isize)
                                                as *mut ::core::ffi::c_ulong,
                                            jpegQual,
                                            flags,
                                        ) == -(1 as ::core::ffi::c_int)
                                        {
                                            let mut _tjErrorCode_2: ::core::ffi::c_int =
                                                tjGetErrorCode(handle);
                                            let mut _tjErrorStr_2: *mut ::core::ffi::c_char =
                                                tjGetErrorStr2(handle);
                                            if flags & TJFLAG_STOPONWARNING == 0
                                                && _tjErrorCode_2
                                                    == TJERR_WARNING as ::core::ffi::c_int
                                            {
                                                if strncmp(
                                                    &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                                    _tjErrorStr_2,
                                                    JMSG_LENGTH_MAX as size_t,
                                                ) != 0
                                                    || strncmp(
                                                        &raw mut tjErrorMsg
                                                            as *mut ::core::ffi::c_char,
                                                        b"executing tjCompressFromYUV()\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        JMSG_LENGTH_MAX as size_t,
                                                    ) != 0
                                                    || tjErrorCode != _tjErrorCode_2
                                                    || tjErrorLine != 468 as ::core::ffi::c_int
                                                {
                                                    strncpy(
                                                        &raw mut tjErrorStr
                                                            as *mut ::core::ffi::c_char,
                                                        _tjErrorStr_2,
                                                        JMSG_LENGTH_MAX as size_t,
                                                    );
                                                    tjErrorStr[(JMSG_LENGTH_MAX
                                                        - 1 as ::core::ffi::c_int)
                                                        as usize] =
                                                        '\0' as i32 as ::core::ffi::c_char;
                                                    strncpy(
                                                        &raw mut tjErrorMsg
                                                            as *mut ::core::ffi::c_char,
                                                        b"executing tjCompressFromYUV()\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        JMSG_LENGTH_MAX as size_t,
                                                    );
                                                    tjErrorMsg[(JMSG_LENGTH_MAX
                                                        - 1 as ::core::ffi::c_int)
                                                        as usize] =
                                                        '\0' as i32 as ::core::ffi::c_char;
                                                    tjErrorCode = _tjErrorCode_2;
                                                    tjErrorLine = 468 as ::core::ffi::c_int;
                                                    printf(
                                                        b"WARNING in line %d while %s:\n%s\n\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        468 as ::core::ffi::c_int,
                                                        b"executing tjCompressFromYUV()\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        _tjErrorStr_2,
                                                    );
                                                }
                                            } else {
                                                printf(
                                                    b"%s in line %d while %s:\n%s\n\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    if _tjErrorCode_2
                                                        == TJERR_WARNING as ::core::ffi::c_int
                                                    {
                                                        b"WARNING\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    } else {
                                                        b"ERROR\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    },
                                                    468 as ::core::ffi::c_int,
                                                    b"executing tjCompressFromYUV()\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    _tjErrorStr_2,
                                                );
                                                retval = -(1 as ::core::ffi::c_int);
                                                break 's_71;
                                            }
                                        }
                                    } else if tjCompress2(
                                        handle,
                                        srcPtr2,
                                        width,
                                        pitch,
                                        height,
                                        pf,
                                        jpegBuf.offset(tile as isize)
                                            as *mut *mut ::core::ffi::c_uchar,
                                        jpegSize.offset(tile as isize) as *mut ::core::ffi::c_ulong,
                                        subsamp,
                                        jpegQual,
                                        flags,
                                    ) == -(1 as ::core::ffi::c_int)
                                    {
                                        let mut _tjErrorCode_3: ::core::ffi::c_int =
                                            tjGetErrorCode(handle);
                                        let mut _tjErrorStr_3: *mut ::core::ffi::c_char =
                                            tjGetErrorStr2(handle);
                                        if flags & TJFLAG_STOPONWARNING == 0
                                            && _tjErrorCode_3 == TJERR_WARNING as ::core::ffi::c_int
                                        {
                                            if strncmp(
                                                &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                                _tjErrorStr_3,
                                                JMSG_LENGTH_MAX as size_t,
                                            ) != 0
                                                || strncmp(
                                                    &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                                    b"executing tjCompress2()\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    JMSG_LENGTH_MAX as size_t,
                                                ) != 0
                                                || tjErrorCode != _tjErrorCode_3
                                                || tjErrorLine != 473 as ::core::ffi::c_int
                                            {
                                                strncpy(
                                                    &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                                    _tjErrorStr_3,
                                                    JMSG_LENGTH_MAX as size_t,
                                                );
                                                tjErrorStr[(JMSG_LENGTH_MAX
                                                    - 1 as ::core::ffi::c_int)
                                                    as usize] = '\0' as i32 as ::core::ffi::c_char;
                                                strncpy(
                                                    &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                                    b"executing tjCompress2()\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    JMSG_LENGTH_MAX as size_t,
                                                );
                                                tjErrorMsg[(JMSG_LENGTH_MAX
                                                    - 1 as ::core::ffi::c_int)
                                                    as usize] = '\0' as i32 as ::core::ffi::c_char;
                                                tjErrorCode = _tjErrorCode_3;
                                                tjErrorLine = 473 as ::core::ffi::c_int;
                                                printf(
                                                    b"WARNING in line %d while %s:\n%s\n\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    473 as ::core::ffi::c_int,
                                                    b"executing tjCompress2()\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    _tjErrorStr_3,
                                                );
                                            }
                                        } else {
                                            printf(
                                                b"%s in line %d while %s:\n%s\n\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                if _tjErrorCode_3
                                                    == TJERR_WARNING as ::core::ffi::c_int
                                                {
                                                    b"WARNING\0" as *const u8
                                                        as *const ::core::ffi::c_char
                                                } else {
                                                    b"ERROR\0" as *const u8
                                                        as *const ::core::ffi::c_char
                                                },
                                                473 as ::core::ffi::c_int,
                                                b"executing tjCompress2()\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                _tjErrorStr_3,
                                            );
                                            retval = -(1 as ::core::ffi::c_int);
                                            break 's_71;
                                        }
                                    }
                                    totalJpegSize = (totalJpegSize as ::core::ffi::c_ulong)
                                        .wrapping_add(*jpegSize.offset(tile as isize))
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_int;
                                    col += 1;
                                    tile += 1;
                                    srcPtr2 = srcPtr2.offset((ps * tilew) as isize);
                                }
                                row += 1;
                                srcPtr = srcPtr.offset((pitch * tileh) as isize);
                            }
                            elapsed += getTime() - start;
                            if iter >= 0 as ::core::ffi::c_int {
                                iter += 1;
                                if elapsed >= benchTime {
                                    break;
                                }
                            } else if elapsed >= warmup {
                                iter = 0 as ::core::ffi::c_int;
                                elapsedEncode = 0.0f64;
                                elapsed = elapsedEncode;
                            }
                        }
                        if doYUV != 0 {
                            elapsed -= elapsedEncode;
                        }
                        if tjDestroy(handle) == -(1 as ::core::ffi::c_int) {
                            let mut _tjErrorCode_4: ::core::ffi::c_int = tjGetErrorCode(handle);
                            let mut _tjErrorStr_4: *mut ::core::ffi::c_char =
                                tjGetErrorStr2(handle);
                            if flags & TJFLAG_STOPONWARNING == 0
                                && _tjErrorCode_4 == TJERR_WARNING as ::core::ffi::c_int
                            {
                                if strncmp(
                                    &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                    _tjErrorStr_4,
                                    JMSG_LENGTH_MAX as size_t,
                                ) != 0
                                    || strncmp(
                                        &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                        b"executing tjDestroy()\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        JMSG_LENGTH_MAX as size_t,
                                    ) != 0
                                    || tjErrorCode != _tjErrorCode_4
                                    || tjErrorLine != 489 as ::core::ffi::c_int
                                {
                                    strncpy(
                                        &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                        _tjErrorStr_4,
                                        JMSG_LENGTH_MAX as size_t,
                                    );
                                    tjErrorStr
                                        [(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                                        '\0' as i32 as ::core::ffi::c_char;
                                    strncpy(
                                        &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                        b"executing tjDestroy()\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        JMSG_LENGTH_MAX as size_t,
                                    );
                                    tjErrorMsg
                                        [(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                                        '\0' as i32 as ::core::ffi::c_char;
                                    tjErrorCode = _tjErrorCode_4;
                                    tjErrorLine = 489 as ::core::ffi::c_int;
                                    printf(
                                        b"WARNING in line %d while %s:\n%s\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        489 as ::core::ffi::c_int,
                                        b"executing tjDestroy()\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        _tjErrorStr_4,
                                    );
                                }
                            } else {
                                printf(
                                    b"%s in line %d while %s:\n%s\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    if _tjErrorCode_4 == TJERR_WARNING as ::core::ffi::c_int {
                                        b"WARNING\0" as *const u8 as *const ::core::ffi::c_char
                                    } else {
                                        b"ERROR\0" as *const u8 as *const ::core::ffi::c_char
                                    },
                                    489 as ::core::ffi::c_int,
                                    b"executing tjDestroy()\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    _tjErrorStr_4,
                                );
                                retval = -(1 as ::core::ffi::c_int);
                                break;
                            }
                        }
                        handle = NULL as tjhandle;
                        if quiet == 1 as ::core::ffi::c_int {
                            printf(
                                b"%-5d  %-5d   \0" as *const u8 as *const ::core::ffi::c_char,
                                tilew,
                                tileh,
                            );
                        }
                        if quiet != 0 {
                            if doYUV != 0 {
                                printf(
                                    b"%-6s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                    sigfig(
                                        (w * h) as ::core::ffi::c_double / 1000000.0f64
                                            * iter as ::core::ffi::c_double
                                            / elapsedEncode,
                                        4 as ::core::ffi::c_int,
                                        &raw mut tempStr as *mut ::core::ffi::c_char,
                                        1024 as ::core::ffi::c_int,
                                    ),
                                    if quiet == 2 as ::core::ffi::c_int {
                                        b"\n\0" as *const u8 as *const ::core::ffi::c_char
                                    } else {
                                        b"  \0" as *const u8 as *const ::core::ffi::c_char
                                    },
                                );
                            }
                            printf(
                                b"%-6s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                sigfig(
                                    (w * h) as ::core::ffi::c_double / 1000000.0f64
                                        * iter as ::core::ffi::c_double
                                        / elapsed,
                                    4 as ::core::ffi::c_int,
                                    &raw mut tempStr as *mut ::core::ffi::c_char,
                                    1024 as ::core::ffi::c_int,
                                ),
                                if quiet == 2 as ::core::ffi::c_int {
                                    b"\n\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"  \0" as *const u8 as *const ::core::ffi::c_char
                                },
                            );
                            printf(
                                b"%-6s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                sigfig(
                                    (w * h * ps) as ::core::ffi::c_double
                                        / totalJpegSize as ::core::ffi::c_double,
                                    4 as ::core::ffi::c_int,
                                    &raw mut tempStr2 as *mut ::core::ffi::c_char,
                                    80 as ::core::ffi::c_int,
                                ),
                                if quiet == 2 as ::core::ffi::c_int {
                                    b"\n\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"  \0" as *const u8 as *const ::core::ffi::c_char
                                },
                            );
                        } else {
                            printf(
                                b"\n%s size: %d x %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                if doTile != 0 {
                                    b"Tile\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"Image\0" as *const u8 as *const ::core::ffi::c_char
                                },
                                tilew,
                                tileh,
                            );
                            if doYUV != 0 {
                                printf(
                                    b"Encode YUV    --> Frame rate:         %f fps\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    iter as ::core::ffi::c_double / elapsedEncode,
                                );
                                printf(
                                    b"                  Output image size:  %lu bytes\n\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    yuvSize,
                                );
                                printf(
                                    b"                  Compression ratio:  %f:1\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    (w * h * ps) as ::core::ffi::c_double
                                        / yuvSize as ::core::ffi::c_double,
                                );
                                printf(
                                    b"                  Throughput:         %f Megapixels/sec\n\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    (w * h) as ::core::ffi::c_double / 1000000.0f64
                                        * iter as ::core::ffi::c_double
                                        / elapsedEncode,
                                );
                                printf(
                                    b"                  Output bit stream:  %f Megabits/sec\n\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    yuvSize as ::core::ffi::c_double * 8.0f64 / 1000000.0f64
                                        * iter as ::core::ffi::c_double
                                        / elapsedEncode,
                                );
                            }
                            printf(
                                b"%s --> Frame rate:         %f fps\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                if doYUV != 0 {
                                    b"Comp from YUV\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"Compress     \0" as *const u8 as *const ::core::ffi::c_char
                                },
                                iter as ::core::ffi::c_double / elapsed,
                            );
                            printf(
                                b"                  Output image size:  %d bytes\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                totalJpegSize,
                            );
                            printf(
                                b"                  Compression ratio:  %f:1\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (w * h * ps) as ::core::ffi::c_double
                                    / totalJpegSize as ::core::ffi::c_double,
                            );
                            printf(
                                b"                  Throughput:         %f Megapixels/sec\n\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                (w * h) as ::core::ffi::c_double / 1000000.0f64
                                    * iter as ::core::ffi::c_double
                                    / elapsed,
                            );
                            printf(
                                b"                  Output bit stream:  %f Megabits/sec\n\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                totalJpegSize as ::core::ffi::c_double * 8.0f64 / 1000000.0f64
                                    * iter as ::core::ffi::c_double
                                    / elapsed,
                            );
                        }
                        if tilew == w && tileh == h && doWrite != 0 {
                            snprintf(
                                &raw mut tempStr as *mut ::core::ffi::c_char,
                                1024 as size_t,
                                b"%s_%s_Q%d.jpg\0" as *const u8 as *const ::core::ffi::c_char,
                                fileName,
                                subName[subsamp as usize],
                                jpegQual,
                            );
                            file = fopen(
                                &raw mut tempStr as *mut ::core::ffi::c_char,
                                b"wb\0" as *const u8 as *const ::core::ffi::c_char,
                            ) as *mut FILE;
                            if file.is_null() {
                                printf(
                                    b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    536 as ::core::ffi::c_int,
                                    b"opening reference image\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    strerror(*__errno_location()),
                                );
                                retval = -(1 as ::core::ffi::c_int);
                                break;
                            } else if fwrite(
                                *jpegBuf.offset(0 as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                *jpegSize.offset(0 as ::core::ffi::c_int as isize) as size_t,
                                1 as size_t,
                                file,
                            ) != 1 as ::core::ffi::c_ulong
                            {
                                printf(
                                    b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    538 as ::core::ffi::c_int,
                                    b"writing reference image\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    strerror(*__errno_location()),
                                );
                                retval = -(1 as ::core::ffi::c_int);
                                break;
                            } else {
                                fclose(file);
                                file = ::core::ptr::null_mut::<FILE>();
                                if quiet == 0 {
                                    printf(
                                        b"Reference image written to %s\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        &raw mut tempStr as *mut ::core::ffi::c_char,
                                    );
                                }
                            }
                        }
                        if compOnly == 0 {
                            if decomp(
                                srcBuf, jpegBuf, jpegSize, tmpBuf, w, h, subsamp, jpegQual,
                                fileName, tilew, tileh,
                            ) == -(1 as ::core::ffi::c_int)
                            {
                                break;
                            }
                        } else if quiet == 1 as ::core::ffi::c_int {
                            printf(b"N/A\n\0" as *const u8 as *const ::core::ffi::c_char);
                        }
                        i = 0 as ::core::ffi::c_int;
                        while i < ntilesw * ntilesh {
                            tjFree(*jpegBuf.offset(i as isize));
                            let ref mut fresh1 = *jpegBuf.offset(i as isize);
                            *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            i += 1;
                        }
                        free(jpegBuf as *mut ::core::ffi::c_void);
                        jpegBuf = ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
                        free(jpegSize as *mut ::core::ffi::c_void);
                        jpegSize = ::core::ptr::null_mut::<::core::ffi::c_ulong>();
                        if doYUV != 0 {
                            free(yuvBuf as *mut ::core::ffi::c_void);
                            yuvBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        }
                        if tilew == w && tileh == h {
                            break;
                        }
                        tilew *= 2 as ::core::ffi::c_int;
                        tileh *= 2 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    if !file.is_null() {
        fclose(file);
    }
    if !jpegBuf.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < ntilesw * ntilesh {
            tjFree(*jpegBuf.offset(i as isize));
            i += 1;
        }
    }
    free(jpegBuf as *mut ::core::ffi::c_void);
    free(yuvBuf as *mut ::core::ffi::c_void);
    free(jpegSize as *mut ::core::ffi::c_void);
    free(tmpBuf as *mut ::core::ffi::c_void);
    if !handle.is_null() {
        tjDestroy(handle);
    }
    return retval;
}
unsafe extern "C" fn decompTest(mut fileName: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut handle: tjhandle = NULL;
    let mut jpegBuf: *mut *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
    let mut srcBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut jpegSize: *mut ::core::ffi::c_ulong = ::core::ptr::null_mut::<::core::ffi::c_ulong>();
    let mut srcSize: ::core::ffi::c_ulong = 0;
    let mut totalJpegSize: ::core::ffi::c_ulong = 0;
    let mut t: *mut tjtransform = ::core::ptr::null_mut::<tjtransform>();
    let mut start: ::core::ffi::c_double = 0.;
    let mut elapsed: ::core::ffi::c_double = 0.;
    let mut ps: ::core::ffi::c_int = tjPixelSize[pf as usize];
    let mut tile: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut iter: ::core::ffi::c_int = 0;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut decompsrc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut temp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tempStr: [::core::ffi::c_char; 80] = [0; 80];
    let mut tempStr2: [::core::ffi::c_char; 80] = [0; 80];
    let mut w: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut h: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tilew: ::core::ffi::c_int = 0;
    let mut tileh: ::core::ffi::c_int = 0;
    let mut ntilesw: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut ntilesh: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut subsamp: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut cs: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut tw: ::core::ffi::c_int = 0;
    let mut th: ::core::ffi::c_int = 0;
    let mut ttilew: ::core::ffi::c_int = 0;
    let mut ttileh: ::core::ffi::c_int = 0;
    let mut tntilesw: ::core::ffi::c_int = 0;
    let mut tntilesh: ::core::ffi::c_int = 0;
    let mut tsubsamp: ::core::ffi::c_int = 0;
    file = fopen(fileName, b"rb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if file.is_null() {
        printf(
            b"ERROR in line %d while %s:\n%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            595 as ::core::ffi::c_int,
            b"opening file\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        retval = -(1 as ::core::ffi::c_int);
    } else if fseek(file, 0 as ::core::ffi::c_long, SEEK_END) < 0 as ::core::ffi::c_int || {
        srcSize = ftell(file) as ::core::ffi::c_ulong;
        srcSize == -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong
    } {
        printf(
            b"ERROR in line %d while %s:\n%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            598 as ::core::ffi::c_int,
            b"determining file size\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        srcBuf = malloc(srcSize as size_t) as *mut ::core::ffi::c_uchar;
        if srcBuf.is_null() {
            printf(
                b"ERROR in line %d while %s:\n%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                600 as ::core::ffi::c_int,
                b"allocating memory\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            retval = -(1 as ::core::ffi::c_int);
        } else if fseek(file, 0 as ::core::ffi::c_long, SEEK_SET) < 0 as ::core::ffi::c_int {
            printf(
                b"ERROR in line %d while %s:\n%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                602 as ::core::ffi::c_int,
                b"setting file position\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            retval = -(1 as ::core::ffi::c_int);
        } else if fread(
            srcBuf as *mut ::core::ffi::c_void,
            srcSize as size_t,
            1 as size_t,
            file,
        ) < 1 as ::core::ffi::c_ulong
        {
            printf(
                b"ERROR in line %d while %s:\n%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                604 as ::core::ffi::c_int,
                b"reading JPEG data\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            retval = -(1 as ::core::ffi::c_int);
        } else {
            fclose(file);
            file = ::core::ptr::null_mut::<FILE>();
            temp = strrchr(fileName, '.' as i32);
            if !temp.is_null() {
                *temp = '\0' as i32 as ::core::ffi::c_char;
            }
            handle = tjInitTransform();
            if handle.is_null() {
                let mut _tjErrorCode: ::core::ffi::c_int = tjGetErrorCode(handle);
                let mut _tjErrorStr: *mut ::core::ffi::c_char = tjGetErrorStr2(handle);
                if flags & TJFLAG_STOPONWARNING == 0
                    && _tjErrorCode == TJERR_WARNING as ::core::ffi::c_int
                {
                    if strncmp(
                        &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                        _tjErrorStr,
                        JMSG_LENGTH_MAX as size_t,
                    ) != 0
                        || strncmp(
                            &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                            b"executing tjInitTransform()\0" as *const u8
                                as *const ::core::ffi::c_char,
                            JMSG_LENGTH_MAX as size_t,
                        ) != 0
                        || tjErrorCode != _tjErrorCode
                        || tjErrorLine != 611 as ::core::ffi::c_int
                    {
                        strncpy(
                            &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                            _tjErrorStr,
                            JMSG_LENGTH_MAX as size_t,
                        );
                        tjErrorStr[(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                            '\0' as i32 as ::core::ffi::c_char;
                        strncpy(
                            &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                            b"executing tjInitTransform()\0" as *const u8
                                as *const ::core::ffi::c_char,
                            JMSG_LENGTH_MAX as size_t,
                        );
                        tjErrorMsg[(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                            '\0' as i32 as ::core::ffi::c_char;
                        tjErrorCode = _tjErrorCode;
                        tjErrorLine = 611 as ::core::ffi::c_int;
                        printf(
                            b"WARNING in line %d while %s:\n%s\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            611 as ::core::ffi::c_int,
                            b"executing tjInitTransform()\0" as *const u8
                                as *const ::core::ffi::c_char,
                            _tjErrorStr,
                        );
                    }
                    current_block = 7828949454673616476;
                } else {
                    printf(
                        b"%s in line %d while %s:\n%s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        if _tjErrorCode == TJERR_WARNING as ::core::ffi::c_int {
                            b"WARNING\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"ERROR\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        611 as ::core::ffi::c_int,
                        b"executing tjInitTransform()\0" as *const u8 as *const ::core::ffi::c_char,
                        _tjErrorStr,
                    );
                    retval = -(1 as ::core::ffi::c_int);
                    current_block = 17728147293744707814;
                }
            } else {
                current_block = 7828949454673616476;
            }
            match current_block {
                17728147293744707814 => {}
                _ => {
                    if tjDecompressHeader3(
                        handle,
                        srcBuf,
                        srcSize,
                        &raw mut w,
                        &raw mut h,
                        &raw mut subsamp,
                        &raw mut cs,
                    ) == -(1 as ::core::ffi::c_int)
                    {
                        let mut _tjErrorCode_0: ::core::ffi::c_int = tjGetErrorCode(handle);
                        let mut _tjErrorStr_0: *mut ::core::ffi::c_char = tjGetErrorStr2(handle);
                        if flags & TJFLAG_STOPONWARNING == 0
                            && _tjErrorCode_0 == TJERR_WARNING as ::core::ffi::c_int
                        {
                            if strncmp(
                                &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                _tjErrorStr_0,
                                JMSG_LENGTH_MAX as size_t,
                            ) != 0
                                || strncmp(
                                    &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                    b"executing tjDecompressHeader3()\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    JMSG_LENGTH_MAX as size_t,
                                ) != 0
                                || tjErrorCode != _tjErrorCode_0
                                || tjErrorLine != 614 as ::core::ffi::c_int
                            {
                                strncpy(
                                    &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                    _tjErrorStr_0,
                                    JMSG_LENGTH_MAX as size_t,
                                );
                                tjErrorStr[(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                                    '\0' as i32 as ::core::ffi::c_char;
                                strncpy(
                                    &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                    b"executing tjDecompressHeader3()\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    JMSG_LENGTH_MAX as size_t,
                                );
                                tjErrorMsg[(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int) as usize] =
                                    '\0' as i32 as ::core::ffi::c_char;
                                tjErrorCode = _tjErrorCode_0;
                                tjErrorLine = 614 as ::core::ffi::c_int;
                                printf(
                                    b"WARNING in line %d while %s:\n%s\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    614 as ::core::ffi::c_int,
                                    b"executing tjDecompressHeader3()\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    _tjErrorStr_0,
                                );
                            }
                            current_block = 12497913735442871383;
                        } else {
                            printf(
                                b"%s in line %d while %s:\n%s\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                if _tjErrorCode_0 == TJERR_WARNING as ::core::ffi::c_int {
                                    b"WARNING\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"ERROR\0" as *const u8 as *const ::core::ffi::c_char
                                },
                                614 as ::core::ffi::c_int,
                                b"executing tjDecompressHeader3()\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                _tjErrorStr_0,
                            );
                            retval = -(1 as ::core::ffi::c_int);
                            current_block = 17728147293744707814;
                        }
                    } else {
                        current_block = 12497913735442871383;
                    }
                    match current_block {
                        17728147293744707814 => {}
                        _ => {
                            if w < 1 as ::core::ffi::c_int || h < 1 as ::core::ffi::c_int {
                                printf(
                                    b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    616 as ::core::ffi::c_int,
                                    b"reading JPEG header\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"Invalid image dimensions\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                retval = -(1 as ::core::ffi::c_int);
                            } else {
                                if cs == TJCS_YCCK as ::core::ffi::c_int
                                    || cs == TJCS_CMYK as ::core::ffi::c_int
                                {
                                    pf = TJPF_CMYK as ::core::ffi::c_int;
                                    ps = tjPixelSize[pf as usize];
                                }
                                if quiet == 1 as ::core::ffi::c_int {
                                    printf(
                                        b"All performance values in Mpixels/sec\n\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    printf(
                                        b"Pixel      JPEG   JPEG     %s  %s   Xform   Comp    Decomp  \0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        if doTile != 0 {
                                            b"Tile \0" as *const u8 as *const ::core::ffi::c_char
                                        } else {
                                            b"Image\0" as *const u8 as *const ::core::ffi::c_char
                                        },
                                        if doTile != 0 {
                                            b"Tile \0" as *const u8 as *const ::core::ffi::c_char
                                        } else {
                                            b"Image\0" as *const u8 as *const ::core::ffi::c_char
                                        },
                                    );
                                    if doYUV != 0 {
                                        printf(
                                            b"Decode\0" as *const u8 as *const ::core::ffi::c_char,
                                        );
                                    }
                                    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                                    printf(
                                        b"Format     CS     Subsamp  Width  Height  Perf    Ratio   Perf    \0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                    );
                                    if doYUV != 0 {
                                        printf(
                                            b"Perf\0" as *const u8 as *const ::core::ffi::c_char,
                                        );
                                    }
                                    printf(b"\n\n\0" as *const u8 as *const ::core::ffi::c_char);
                                } else if quiet == 0 {
                                    printf(
                                        b">>>>>  JPEG %s --> %s (%s)  <<<<<\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        formatName(
                                            subsamp,
                                            cs,
                                            &raw mut tempStr as *mut ::core::ffi::c_char,
                                        ),
                                        pixFormatStr[pf as usize],
                                        if flags & TJFLAG_BOTTOMUP != 0 {
                                            b"Bottom-up\0" as *const u8
                                                as *const ::core::ffi::c_char
                                        } else {
                                            b"Top-down\0" as *const u8 as *const ::core::ffi::c_char
                                        },
                                    );
                                }
                                tilew = (if doTile != 0 {
                                    16 as ::core::ffi::c_int
                                } else {
                                    w
                                });
                                tileh = (if doTile != 0 {
                                    16 as ::core::ffi::c_int
                                } else {
                                    h
                                });
                                's_330: loop {
                                    if tilew > w {
                                        tilew = w;
                                    }
                                    if tileh > h {
                                        tileh = h;
                                    }
                                    ntilesw = (w + tilew - 1 as ::core::ffi::c_int) / tilew;
                                    ntilesh = (h + tileh - 1 as ::core::ffi::c_int) / tileh;
                                    jpegBuf = malloc(
                                        (::core::mem::size_of::<*mut ::core::ffi::c_uchar>()
                                            as size_t)
                                            .wrapping_mul(ntilesw as size_t)
                                            .wrapping_mul(ntilesh as size_t),
                                    )
                                        as *mut *mut ::core::ffi::c_uchar;
                                    if jpegBuf.is_null() {
                                        printf(
                                            b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            644 as ::core::ffi::c_int,
                                            b"allocating JPEG tile array\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            strerror(*__errno_location()),
                                        );
                                        retval = -(1 as ::core::ffi::c_int);
                                        break;
                                    } else {
                                        memset(
                                            jpegBuf as *mut ::core::ffi::c_void,
                                            0 as ::core::ffi::c_int,
                                            (::core::mem::size_of::<*mut ::core::ffi::c_uchar>()
                                                as size_t)
                                                .wrapping_mul(ntilesw as size_t)
                                                .wrapping_mul(ntilesh as size_t),
                                        );
                                        jpegSize = malloc(
                                            (::core::mem::size_of::<::core::ffi::c_ulong>()
                                                as size_t)
                                                .wrapping_mul(ntilesw as size_t)
                                                .wrapping_mul(ntilesh as size_t),
                                        )
                                            as *mut ::core::ffi::c_ulong;
                                        if jpegSize.is_null() {
                                            printf(
                                                b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                648 as ::core::ffi::c_int,
                                                b"allocating JPEG size array\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                strerror(*__errno_location()),
                                            );
                                            retval = -(1 as ::core::ffi::c_int);
                                            break;
                                        } else {
                                            memset(
                                                jpegSize as *mut ::core::ffi::c_void,
                                                0 as ::core::ffi::c_int,
                                                (::core::mem::size_of::<::core::ffi::c_ulong>()
                                                    as size_t)
                                                    .wrapping_mul(ntilesw as size_t)
                                                    .wrapping_mul(ntilesh as size_t),
                                            );
                                            if flags & TJFLAG_NOREALLOC != 0 as ::core::ffi::c_int
                                                && (doTile != 0
                                                    || xformOp != TJXOP_NONE as ::core::ffi::c_int
                                                    || xformOpt != 0 as ::core::ffi::c_int
                                                    || customFilter.is_some())
                                            {
                                                i = 0 as ::core::ffi::c_int;
                                                while i < ntilesw * ntilesh {
                                                    if tjBufSize(tilew, tileh, subsamp)
                                                        > INT_MAX as ::core::ffi::c_ulong
                                                    {
                                                        printf(
                                                            b"ERROR in line %d while %s:\n%s\n\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            655 as ::core::ffi::c_int,
                                                            b"getting buffer size\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            b"Image is too large\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        retval = -(1 as ::core::ffi::c_int);
                                                        break 's_330;
                                                    } else {
                                                        let ref mut fresh2 =
                                                            *jpegBuf.offset(i as isize);
                                                        *fresh2 = tjAlloc(tjBufSize(
                                                            tilew, tileh, subsamp,
                                                        )
                                                            as ::core::ffi::c_int);
                                                        if (*fresh2).is_null() {
                                                            printf(
                                                                b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                658 as ::core::ffi::c_int,
                                                                b"allocating JPEG tiles\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                strerror(*__errno_location()),
                                                            );
                                                            retval = -(1 as ::core::ffi::c_int);
                                                            break 's_330;
                                                        } else {
                                                            i += 1;
                                                        }
                                                    }
                                                }
                                            }
                                            tw = w;
                                            th = h;
                                            ttilew = tilew;
                                            ttileh = tileh;
                                            if quiet == 0 {
                                                printf(
                                                    b"\n%s size: %d x %d\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    if doTile != 0 {
                                                        b"Tile\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    } else {
                                                        b"Image\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    },
                                                    ttilew,
                                                    ttileh,
                                                );
                                                if sf.num != 1 as ::core::ffi::c_int
                                                    || sf.denom != 1 as ::core::ffi::c_int
                                                {
                                                    printf(
                                                        b" --> %d x %d\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        (tw * sf.num + sf.denom
                                                            - 1 as ::core::ffi::c_int)
                                                            / sf.denom,
                                                        (th * sf.num + sf.denom
                                                            - 1 as ::core::ffi::c_int)
                                                            / sf.denom,
                                                    );
                                                }
                                                printf(
                                                    b"\n\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                            } else if quiet == 1 as ::core::ffi::c_int {
                                                printf(
                                                    b"%-4s (%s)  %-5s  %-5s    \0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    pixFormatStr[pf as usize],
                                                    if flags & TJFLAG_BOTTOMUP != 0 {
                                                        b"BU\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    } else {
                                                        b"TD\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                    },
                                                    csName[cs as usize],
                                                    subNameLong[subsamp as usize],
                                                );
                                                printf(
                                                    b"%-5d  %-5d   \0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    tilew,
                                                    tileh,
                                                );
                                            }
                                            tsubsamp = subsamp;
                                            if doTile != 0
                                                || xformOp != TJXOP_NONE as ::core::ffi::c_int
                                                || xformOpt != 0 as ::core::ffi::c_int
                                                || customFilter.is_some()
                                            {
                                                t = malloc(
                                                    (::core::mem::size_of::<tjtransform>()
                                                        as size_t)
                                                        .wrapping_mul(ntilesw as size_t)
                                                        .wrapping_mul(ntilesh as size_t),
                                                )
                                                    as *mut tjtransform;
                                                if t.is_null() {
                                                    printf(
                                                        b"ERROR in line %d while %s:\n%s\n\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        678 as ::core::ffi::c_int,
                                                        b"allocating image transform array\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        strerror(*__errno_location()),
                                                    );
                                                    retval = -(1 as ::core::ffi::c_int);
                                                    break;
                                                } else {
                                                    if xformOp
                                                        == TJXOP_TRANSPOSE as ::core::ffi::c_int
                                                        || xformOp
                                                            == TJXOP_TRANSVERSE
                                                                as ::core::ffi::c_int
                                                        || xformOp
                                                            == TJXOP_ROT90 as ::core::ffi::c_int
                                                        || xformOp
                                                            == TJXOP_ROT270 as ::core::ffi::c_int
                                                    {
                                                        tw = h;
                                                        th = w;
                                                        ttilew = tileh;
                                                        ttileh = tilew;
                                                    }
                                                    if xformOpt & TJXOPT_GRAY != 0 {
                                                        tsubsamp =
                                                            TJSAMP_GRAY as ::core::ffi::c_int;
                                                    }
                                                    if xformOp == TJXOP_HFLIP as ::core::ffi::c_int
                                                        || xformOp
                                                            == TJXOP_ROT180 as ::core::ffi::c_int
                                                    {
                                                        tw =
                                                            tw - tw % tjMCUWidth[tsubsamp as usize];
                                                    }
                                                    if xformOp == TJXOP_VFLIP as ::core::ffi::c_int
                                                        || xformOp
                                                            == TJXOP_ROT180 as ::core::ffi::c_int
                                                    {
                                                        th = th
                                                            - th % tjMCUHeight[tsubsamp as usize];
                                                    }
                                                    if xformOp
                                                        == TJXOP_TRANSVERSE as ::core::ffi::c_int
                                                        || xformOp
                                                            == TJXOP_ROT90 as ::core::ffi::c_int
                                                    {
                                                        tw = tw
                                                            - tw % tjMCUHeight[tsubsamp as usize];
                                                    }
                                                    if xformOp
                                                        == TJXOP_TRANSVERSE as ::core::ffi::c_int
                                                        || xformOp
                                                            == TJXOP_ROT270 as ::core::ffi::c_int
                                                    {
                                                        th =
                                                            th - th % tjMCUWidth[tsubsamp as usize];
                                                    }
                                                    tntilesw = (tw + ttilew
                                                        - 1 as ::core::ffi::c_int)
                                                        / ttilew;
                                                    tntilesh = (th + ttileh
                                                        - 1 as ::core::ffi::c_int)
                                                        / ttileh;
                                                    if xformOp
                                                        == TJXOP_TRANSPOSE as ::core::ffi::c_int
                                                        || xformOp
                                                            == TJXOP_TRANSVERSE
                                                                as ::core::ffi::c_int
                                                        || xformOp
                                                            == TJXOP_ROT90 as ::core::ffi::c_int
                                                        || xformOp
                                                            == TJXOP_ROT270 as ::core::ffi::c_int
                                                    {
                                                        if tsubsamp
                                                            == TJSAMP_422 as ::core::ffi::c_int
                                                        {
                                                            tsubsamp =
                                                                TJSAMP_440 as ::core::ffi::c_int;
                                                        } else if tsubsamp
                                                            == TJSAMP_440 as ::core::ffi::c_int
                                                        {
                                                            tsubsamp =
                                                                TJSAMP_422 as ::core::ffi::c_int;
                                                        }
                                                    }
                                                    row = 0 as ::core::ffi::c_int;
                                                    tile = 0 as ::core::ffi::c_int;
                                                    while row < tntilesh {
                                                        col = 0 as ::core::ffi::c_int;
                                                        while col < tntilesw {
                                                            (*t.offset(tile as isize)).r.w =
                                                                if ttilew < tw - col * ttilew {
                                                                    ttilew
                                                                } else {
                                                                    tw - col * ttilew
                                                                };
                                                            (*t.offset(tile as isize)).r.h =
                                                                if ttileh < th - row * ttileh {
                                                                    ttileh
                                                                } else {
                                                                    th - row * ttileh
                                                                };
                                                            (*t.offset(tile as isize)).r.x =
                                                                col * ttilew;
                                                            (*t.offset(tile as isize)).r.y =
                                                                row * ttileh;
                                                            (*t.offset(tile as isize)).op = xformOp;
                                                            (*t.offset(tile as isize)).options =
                                                                xformOpt | TJXOPT_TRIM;
                                                            let ref mut fresh3 = (*t
                                                                .offset(tile as isize))
                                                            .customFilter;
                                                            *fresh3 = customFilter
                                                                as Option<
                                                                    unsafe extern "C" fn(
                                                                        *mut ::core::ffi::c_short,
                                                                        tjregion,
                                                                        tjregion,
                                                                        ::core::ffi::c_int,
                                                                        ::core::ffi::c_int,
                                                                        *mut tjtransform,
                                                                    ) -> ::core::ffi::c_int,
                                                                >;
                                                            if (*t.offset(tile as isize)).options
                                                                & TJXOPT_NOOUTPUT
                                                                != 0
                                                                && !(*jpegBuf.offset(tile as isize))
                                                                    .is_null()
                                                            {
                                                                tjFree(
                                                                    *jpegBuf.offset(tile as isize),
                                                                );
                                                                let ref mut fresh4 =
                                                                    *jpegBuf.offset(tile as isize);
                                                                *fresh4 = ::core::ptr::null_mut::<
                                                                    ::core::ffi::c_uchar,
                                                                >(
                                                                );
                                                            }
                                                            col += 1;
                                                            tile += 1;
                                                        }
                                                        row += 1;
                                                    }
                                                    iter = -(1 as ::core::ffi::c_int);
                                                    elapsed = 0.0f64;
                                                    loop {
                                                        start = getTime();
                                                        if tjTransform(
                                                            handle,
                                                            srcBuf,
                                                            srcSize,
                                                            tntilesw * tntilesh,
                                                            jpegBuf,
                                                            jpegSize,
                                                            t,
                                                            flags,
                                                        ) == -(1 as ::core::ffi::c_int)
                                                        {
                                                            let mut _tjErrorCode_1: ::core::ffi::c_int = tjGetErrorCode(
                                                                handle,
                                                            );
                                                            let mut _tjErrorStr_1: *mut ::core::ffi::c_char = tjGetErrorStr2(
                                                                handle,
                                                            );
                                                            if flags & TJFLAG_STOPONWARNING == 0
                                                                && _tjErrorCode_1
                                                                    == TJERR_WARNING
                                                                        as ::core::ffi::c_int
                                                            {
                                                                if strncmp(
                                                                    &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                                                    _tjErrorStr_1,
                                                                    JMSG_LENGTH_MAX as size_t,
                                                                ) != 0
                                                                    || strncmp(
                                                                        &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                                                        b"executing tjTransform()\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        JMSG_LENGTH_MAX as size_t,
                                                                    ) != 0 || tjErrorCode != _tjErrorCode_1
                                                                    || tjErrorLine != 724 as ::core::ffi::c_int
                                                                {
                                                                    strncpy(
                                                                        &raw mut tjErrorStr as *mut ::core::ffi::c_char,
                                                                        _tjErrorStr_1,
                                                                        JMSG_LENGTH_MAX as size_t,
                                                                    );
                                                                    tjErrorStr[(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int)
                                                                        as usize] = '\0' as i32 as ::core::ffi::c_char;
                                                                    strncpy(
                                                                        &raw mut tjErrorMsg as *mut ::core::ffi::c_char,
                                                                        b"executing tjTransform()\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        JMSG_LENGTH_MAX as size_t,
                                                                    );
                                                                    tjErrorMsg[(JMSG_LENGTH_MAX - 1 as ::core::ffi::c_int)
                                                                        as usize] = '\0' as i32 as ::core::ffi::c_char;
                                                                    tjErrorCode = _tjErrorCode_1;
                                                                    tjErrorLine = 724 as ::core::ffi::c_int;
                                                                    printf(
                                                                        b"WARNING in line %d while %s:\n%s\n\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        724 as ::core::ffi::c_int,
                                                                        b"executing tjTransform()\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        _tjErrorStr_1,
                                                                    );
                                                                }
                                                            } else {
                                                                printf(
                                                                    b"%s in line %d while %s:\n%s\n\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                    if _tjErrorCode_1 == TJERR_WARNING as ::core::ffi::c_int {
                                                                        b"WARNING\0" as *const u8 as *const ::core::ffi::c_char
                                                                    } else {
                                                                        b"ERROR\0" as *const u8 as *const ::core::ffi::c_char
                                                                    },
                                                                    724 as ::core::ffi::c_int,
                                                                    b"executing tjTransform()\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                    _tjErrorStr_1,
                                                                );
                                                                retval = -(1 as ::core::ffi::c_int);
                                                                break 's_330;
                                                            }
                                                        }
                                                        elapsed += getTime() - start;
                                                        if iter >= 0 as ::core::ffi::c_int {
                                                            iter += 1;
                                                            if elapsed >= benchTime {
                                                                break;
                                                            }
                                                        } else if elapsed >= warmup {
                                                            iter = 0 as ::core::ffi::c_int;
                                                            elapsed = 0.0f64;
                                                        }
                                                    }
                                                    free(t as *mut ::core::ffi::c_void);
                                                    t = ::core::ptr::null_mut::<tjtransform>();
                                                    tile = 0 as ::core::ffi::c_int;
                                                    totalJpegSize = 0 as ::core::ffi::c_ulong;
                                                    while tile < tntilesw * tntilesh {
                                                        totalJpegSize = totalJpegSize.wrapping_add(
                                                            *jpegSize.offset(tile as isize),
                                                        );
                                                        tile += 1;
                                                    }
                                                    if quiet != 0 {
                                                        printf(
                                                            b"%-6s%s%-6s%s\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            sigfig(
                                                                (w * h) as ::core::ffi::c_double
                                                                    / 1000000.0f64
                                                                    / elapsed,
                                                                4 as ::core::ffi::c_int,
                                                                &raw mut tempStr
                                                                    as *mut ::core::ffi::c_char,
                                                                80 as ::core::ffi::c_int,
                                                            ),
                                                            if quiet == 2 as ::core::ffi::c_int {
                                                                b"\n\0" as *const u8
                                                                    as *const ::core::ffi::c_char
                                                            } else {
                                                                b"  \0" as *const u8
                                                                    as *const ::core::ffi::c_char
                                                            },
                                                            sigfig(
                                                                (w * h * ps)
                                                                    as ::core::ffi::c_double
                                                                    / totalJpegSize
                                                                        as ::core::ffi::c_double,
                                                                4 as ::core::ffi::c_int,
                                                                &raw mut tempStr2
                                                                    as *mut ::core::ffi::c_char,
                                                                80 as ::core::ffi::c_int,
                                                            ),
                                                            if quiet == 2 as ::core::ffi::c_int {
                                                                b"\n\0" as *const u8
                                                                    as *const ::core::ffi::c_char
                                                            } else {
                                                                b"  \0" as *const u8
                                                                    as *const ::core::ffi::c_char
                                                            },
                                                        );
                                                    } else {
                                                        printf(
                                                            b"Transform     --> Frame rate:         %f fps\n\0"
                                                                as *const u8 as *const ::core::ffi::c_char,
                                                            1.0f64 / elapsed,
                                                        );
                                                        printf(
                                                            b"                  Output image size:  %lu bytes\n\0"
                                                                as *const u8 as *const ::core::ffi::c_char,
                                                            totalJpegSize,
                                                        );
                                                        printf(
                                                            b"                  Compression ratio:  %f:1\n\0"
                                                                as *const u8 as *const ::core::ffi::c_char,
                                                            (w * h * ps) as ::core::ffi::c_double
                                                                / totalJpegSize as ::core::ffi::c_double,
                                                        );
                                                        printf(
                                                            b"                  Throughput:         %f Megapixels/sec\n\0"
                                                                as *const u8 as *const ::core::ffi::c_char,
                                                            (w * h) as ::core::ffi::c_double / 1000000.0f64 / elapsed,
                                                        );
                                                        printf(
                                                            b"                  Output bit stream:  %f Megabits/sec\n\0"
                                                                as *const u8 as *const ::core::ffi::c_char,
                                                            totalJpegSize as ::core::ffi::c_double * 8.0f64
                                                                / 1000000.0f64 / elapsed,
                                                        );
                                                    }
                                                }
                                            } else {
                                                if quiet == 1 as ::core::ffi::c_int {
                                                    printf(
                                                        b"N/A     N/A     \0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                }
                                                tjFree(
                                                    *jpegBuf
                                                        .offset(0 as ::core::ffi::c_int as isize),
                                                );
                                                let ref mut fresh5 = *jpegBuf
                                                    .offset(0 as ::core::ffi::c_int as isize);
                                                *fresh5 =
                                                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                decompsrc = 1 as ::core::ffi::c_int;
                                            }
                                            if w == tilew {
                                                ttilew = tw;
                                            }
                                            if h == tileh {
                                                ttileh = th;
                                            }
                                            if xformOpt & TJXOPT_NOOUTPUT == 0 {
                                                if decomp(
                                                    ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                                    (if decompsrc != 0 {
                                                        &raw mut srcBuf
                                                    } else {
                                                        jpegBuf
                                                    }),
                                                    (if decompsrc != 0 {
                                                        &raw mut srcSize
                                                    } else {
                                                        jpegSize
                                                    }),
                                                    ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                                    tw,
                                                    th,
                                                    tsubsamp,
                                                    0 as ::core::ffi::c_int,
                                                    fileName,
                                                    ttilew,
                                                    ttileh,
                                                ) == -(1 as ::core::ffi::c_int)
                                                {
                                                    break;
                                                }
                                            } else if quiet == 1 as ::core::ffi::c_int {
                                                printf(
                                                    b"N/A\n\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                            }
                                            i = 0 as ::core::ffi::c_int;
                                            while i < ntilesw * ntilesh {
                                                tjFree(*jpegBuf.offset(i as isize));
                                                let ref mut fresh6 = *jpegBuf.offset(i as isize);
                                                *fresh6 =
                                                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                i += 1;
                                            }
                                            free(jpegBuf as *mut ::core::ffi::c_void);
                                            jpegBuf =
                                                ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>(
                                                );
                                            free(jpegSize as *mut ::core::ffi::c_void);
                                            jpegSize =
                                                ::core::ptr::null_mut::<::core::ffi::c_ulong>();
                                            if tilew == w && tileh == h {
                                                break;
                                            }
                                            tilew *= 2 as ::core::ffi::c_int;
                                            tileh *= 2 as ::core::ffi::c_int;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !file.is_null() {
        fclose(file);
    }
    if !jpegBuf.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < ntilesw * ntilesh {
            tjFree(*jpegBuf.offset(i as isize));
            i += 1;
        }
    }
    free(jpegBuf as *mut ::core::ffi::c_void);
    free(jpegSize as *mut ::core::ffi::c_void);
    free(srcBuf as *mut ::core::ffi::c_void);
    free(t as *mut ::core::ffi::c_void);
    if !handle.is_null() {
        tjDestroy(handle);
        handle = NULL as tjhandle;
    }
    return retval;
}
unsafe extern "C" fn usage(mut progName: *mut ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    printf(
        b"USAGE: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        progName,
    );
    printf(
        b"       <Inputimage (BMP|PPM)> <Quality> [options]\n\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"       %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        progName,
    );
    printf(b"       <Inputimage (JPG)> [options]\n\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(b"Options:\n\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"-alloc = Dynamically allocate JPEG buffers\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-bmp = Use Windows Bitmap format for output images [default = PPM]\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-bottomup = Use bottom-up row order for packed-pixel source/destination buffers\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"-tile = Compress/transform the input image into separate JPEG tiles of varying\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"     sizes (useful for measuring JPEG overhead)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-rgb, -bgr, -rgbx, -bgrx, -xbgr, -xrgb =\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"     Use the specified pixel format for packed-pixel source/destination buffers\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(b"     [default = BGR]\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"-cmyk = Indirectly test YCCK JPEG compression/decompression\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"     (use the CMYK pixel format for packed-pixel source/destination buffers)\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"-fastupsample = Use the fastest chrominance upsampling algorithm available\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"-fastdct = Use the fastest DCT/IDCT algorithm available\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-accuratedct = Use the most accurate DCT/IDCT algorithm available\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-progressive = Use progressive entropy coding in JPEG images generated by\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"     compression and transform operations\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"-subsamp <s> = When compressing, use the specified level of chrominance\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"     subsampling (<s> = 444, 422, 440, 420, 411, or GRAY) [default = test\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"     Grayscale, 4:2:0, 4:2:2, and 4:4:4 in sequence]\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-quiet = Output results in tabular rather than verbose format\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-yuv = Compress from/decompress to intermediate planar YUV images\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-yuvpad <p> = The number of bytes by which each row in each plane of an\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"     intermediate YUV image is evenly divisible (must be a power of 2)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(b"     [default = 1]\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"-scale M/N = When decompressing, scale the width/height of the JPEG image by a\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(b"     factor of M/N (M/N = \0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as ::core::ffi::c_int;
    while i < nsf {
        printf(
            b"%d/%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*scalingFactors.offset(i as isize)).num,
            (*scalingFactors.offset(i as isize)).denom,
        );
        if nsf == 2 as ::core::ffi::c_int && i != nsf - 1 as ::core::ffi::c_int {
            printf(b" or \0" as *const u8 as *const ::core::ffi::c_char);
        } else if nsf > 2 as ::core::ffi::c_int {
            if i != nsf - 1 as ::core::ffi::c_int {
                printf(b", \0" as *const u8 as *const ::core::ffi::c_char);
            }
            if i == nsf - 2 as ::core::ffi::c_int {
                printf(b"or \0" as *const u8 as *const ::core::ffi::c_char);
            }
        }
        if i % 8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int && i != 0 as ::core::ffi::c_int {
            printf(b"\n     \0" as *const u8 as *const ::core::ffi::c_char);
        }
        i += 1;
    }
    printf(b")\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"-hflip, -vflip, -transpose, -transverse, -rot90, -rot180, -rot270 =\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"     Perform the specified lossless transform operation on the input image\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"     prior to decompression (these operations are mutually exclusive)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-grayscale = Transform the input image into a grayscale JPEG image prior to\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"     decompression (can be combined with the other transform operations above)\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"-copynone = Do not copy any extra markers (including EXIF and ICC profile data)\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"     when transforming the input image\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"-benchtime <t> = Run each benchmark for at least <t> seconds [default = 5.0]\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"-warmup <t> = Run each benchmark for <t> seconds [default = 1.0] prior to\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"     starting the timer, in order to prime the caches and thus improve the\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"     consistency of the benchmark results\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"-componly = Stop after running compression tests.  Do not test decompression.\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"-nowrite = Do not write reference or output images (improves consistency of\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(b"     benchmark results)\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"-limitscans = Refuse to decompress or transform progressive JPEG images that\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"     have an unreasonably large number of scans\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-stoponwarning = Immediately discontinue the current\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"     compression/decompression/transform operation if a warning (non-fatal\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    printf(b"     error) occurs\n\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"NOTE:  If the quality is specified as a range (e.g. 90-100), a separate\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"test will be performed for all quality values in the range.\n\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    exit(1 as ::core::ffi::c_int);
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut srcBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut w: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut h: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut minQual: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut maxQual: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut temp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut minArg: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut subsamp: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    scalingFactors = tjGetScalingFactors(&raw mut nsf);
    if scalingFactors.is_null() || nsf == 0 as ::core::ffi::c_int {
        printf(
            b"ERROR in line %d while %s:\n%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            877 as ::core::ffi::c_int,
            b"executing tjGetScalingFactors()\0" as *const u8 as *const ::core::ffi::c_char,
            tjGetErrorStr(),
        );
        retval = -(1 as ::core::ffi::c_int);
    } else {
        if argc < minArg {
            usage(*argv.offset(0 as ::core::ffi::c_int as isize));
        }
        temp = strrchr(*argv.offset(1 as ::core::ffi::c_int as isize), '.' as i32);
        if !temp.is_null() {
            if strcasecmp(temp, b".bmp\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                ext =
                    b"bmp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            if strcasecmp(temp, b".jpg\0" as *const u8 as *const ::core::ffi::c_char) == 0
                || strcasecmp(temp, b".jpeg\0" as *const u8 as *const ::core::ffi::c_char) == 0
            {
                decompOnly = 1 as ::core::ffi::c_int;
            }
        }
        printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        if decompOnly == 0 {
            minArg = 3 as ::core::ffi::c_int;
            if argc < minArg {
                usage(*argv.offset(0 as ::core::ffi::c_int as isize));
            }
            minQual = atoi(*argv.offset(2 as ::core::ffi::c_int as isize));
            if minQual < 1 as ::core::ffi::c_int || minQual > 100 as ::core::ffi::c_int {
                puts(
                    b"ERROR: Quality must be between 1 and 100.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            temp = strchr(*argv.offset(2 as ::core::ffi::c_int as isize), '-' as i32);
            if !(!temp.is_null()
                && strlen(temp) > 1 as size_t
                && sscanf(
                    temp.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
                    b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut maxQual,
                ) == 1 as ::core::ffi::c_int
                && maxQual > minQual
                && maxQual >= 1 as ::core::ffi::c_int
                && maxQual <= 100 as ::core::ffi::c_int)
            {
                maxQual = minQual;
            }
        }
        if argc > minArg {
            i = minArg;
            while i < argc {
                if strcasecmp(
                    *argv.offset(i as isize),
                    b"-tile\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    doTile = 1 as ::core::ffi::c_int;
                    xformOpt |= TJXOPT_CROP;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-fastupsample\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    printf(
                        b"Using fastest upsampling algorithm\n\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    flags |= TJFLAG_FASTUPSAMPLE;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-fastdct\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    printf(
                        b"Using fastest DCT/IDCT algorithm\n\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    flags |= TJFLAG_FASTDCT;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-accuratedct\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    printf(
                        b"Using most accurate DCT/IDCT algorithm\n\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    flags |= TJFLAG_ACCURATEDCT;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-progressive\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    printf(
                        b"Using progressive entropy coding\n\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    flags |= TJFLAG_PROGRESSIVE;
                    xformOpt |= TJXOPT_PROGRESSIVE;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-rgb\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    pf = TJPF_RGB as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-rgbx\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    pf = TJPF_RGBX as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-bgr\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    pf = TJPF_BGR as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-bgrx\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    pf = TJPF_BGRX as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-xbgr\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    pf = TJPF_XBGR as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-xrgb\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    pf = TJPF_XRGB as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-cmyk\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    pf = TJPF_CMYK as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-bottomup\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    flags |= TJFLAG_BOTTOMUP;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-quiet\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    quiet = 1 as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-qq\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    quiet = 2 as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-scale\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                    && i < argc - 1 as ::core::ffi::c_int
                {
                    let mut temp1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut temp2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut match_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    i += 1;
                    if sscanf(
                        *argv.offset(i as isize),
                        b"%d/%d\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut temp1,
                        &raw mut temp2,
                    ) == 2 as ::core::ffi::c_int
                    {
                        j = 0 as ::core::ffi::c_int;
                        while j < nsf {
                            if temp1 as ::core::ffi::c_double / temp2 as ::core::ffi::c_double
                                == (*scalingFactors.offset(j as isize)).num as ::core::ffi::c_double
                                    / (*scalingFactors.offset(j as isize)).denom
                                        as ::core::ffi::c_double
                            {
                                sf = *scalingFactors.offset(j as isize);
                                match_0 = 1 as ::core::ffi::c_int;
                                break;
                            } else {
                                j += 1;
                            }
                        }
                        if match_0 == 0 {
                            usage(*argv.offset(0 as ::core::ffi::c_int as isize));
                        }
                    } else {
                        usage(*argv.offset(0 as ::core::ffi::c_int as isize));
                    }
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-hflip\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    xformOp = TJXOP_HFLIP as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-vflip\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    xformOp = TJXOP_VFLIP as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-transpose\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    xformOp = TJXOP_TRANSPOSE as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-transverse\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    xformOp = TJXOP_TRANSVERSE as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-rot90\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    xformOp = TJXOP_ROT90 as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-rot180\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    xformOp = TJXOP_ROT180 as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-rot270\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    xformOp = TJXOP_ROT270 as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-grayscale\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    xformOpt |= TJXOPT_GRAY;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-custom\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    customFilter = Some(
                        dummyDCTFilter
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_short,
                                tjregion,
                                tjregion,
                                ::core::ffi::c_int,
                                ::core::ffi::c_int,
                                *mut tjtransform,
                            )
                                -> ::core::ffi::c_int,
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_short,
                                tjregion,
                                tjregion,
                                ::core::ffi::c_int,
                                ::core::ffi::c_int,
                                *mut tjtransform,
                            ) -> ::core::ffi::c_int,
                        >;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-nooutput\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    xformOpt |= TJXOPT_NOOUTPUT;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-copynone\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    xformOpt |= TJXOPT_COPYNONE;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-benchtime\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                    && i < argc - 1 as ::core::ffi::c_int
                {
                    i += 1;
                    let mut tempd: ::core::ffi::c_double = atof(*argv.offset(i as isize));
                    if tempd > 0.0f64 {
                        benchTime = tempd;
                    } else {
                        usage(*argv.offset(0 as ::core::ffi::c_int as isize));
                    }
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-warmup\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                    && i < argc - 1 as ::core::ffi::c_int
                {
                    i += 1;
                    let mut tempd_0: ::core::ffi::c_double = atof(*argv.offset(i as isize));
                    if tempd_0 >= 0.0f64 {
                        warmup = tempd_0;
                    } else {
                        usage(*argv.offset(0 as ::core::ffi::c_int as isize));
                    }
                    printf(
                        b"Warmup time = %.1f seconds\n\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        warmup,
                    );
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-alloc\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    flags &= !TJFLAG_NOREALLOC;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-bmp\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    ext = b"bmp\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-yuv\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    printf(
                        b"Testing planar YUV encoding/decoding\n\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    doYUV = 1 as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-yuvpad\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                    && i < argc - 1 as ::core::ffi::c_int
                {
                    i += 1;
                    let mut tempi: ::core::ffi::c_int = atoi(*argv.offset(i as isize));
                    if tempi >= 1 as ::core::ffi::c_int
                        && tempi & tempi - 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        yuvAlign = tempi;
                    } else {
                        usage(*argv.offset(0 as ::core::ffi::c_int as isize));
                    }
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-subsamp\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                    && i < argc - 1 as ::core::ffi::c_int
                {
                    i += 1;
                    if toupper(
                        *(*argv.offset(i as isize)).offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int,
                    ) == 'G' as i32
                    {
                        subsamp = TJSAMP_GRAY as ::core::ffi::c_int;
                    } else {
                        let mut tempi_0: ::core::ffi::c_int = atoi(*argv.offset(i as isize));
                        match tempi_0 {
                            444 => {
                                subsamp = TJSAMP_444 as ::core::ffi::c_int;
                            }
                            422 => {
                                subsamp = TJSAMP_422 as ::core::ffi::c_int;
                            }
                            440 => {
                                subsamp = TJSAMP_440 as ::core::ffi::c_int;
                            }
                            420 => {
                                subsamp = TJSAMP_420 as ::core::ffi::c_int;
                            }
                            411 => {
                                subsamp = TJSAMP_411 as ::core::ffi::c_int;
                            }
                            _ => {
                                usage(*argv.offset(0 as ::core::ffi::c_int as isize));
                            }
                        }
                    }
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-componly\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    compOnly = 1 as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-nowrite\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    doWrite = 0 as ::core::ffi::c_int;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-limitscans\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    flags |= TJFLAG_LIMITSCANS;
                } else if strcasecmp(
                    *argv.offset(i as isize),
                    b"-stoponwarning\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
                {
                    flags |= TJFLAG_STOPONWARNING;
                } else {
                    usage(*argv.offset(0 as ::core::ffi::c_int as isize));
                }
                i += 1;
            }
        }
        if (sf.num != 1 as ::core::ffi::c_int || sf.denom != 1 as ::core::ffi::c_int) && doTile != 0
        {
            printf(
                b"Disabling tiled compression/decompression tests, because those tests do not\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            printf(
                b"work when scaled decompression is enabled.\n\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            doTile = 0 as ::core::ffi::c_int;
            xformOpt &= !TJXOPT_CROP;
        }
        if flags & TJFLAG_NOREALLOC == 0 as ::core::ffi::c_int && doTile != 0 {
            printf(
                b"Disabling tiled compression/decompression tests, because those tests do not\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            printf(
                b"work when dynamic JPEG buffer allocation is enabled.\n\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            doTile = 0 as ::core::ffi::c_int;
            xformOpt &= !TJXOPT_CROP;
        }
        if decompOnly == 0 {
            srcBuf = tjLoadImage(
                *argv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut w,
                1 as ::core::ffi::c_int,
                &raw mut h,
                &raw mut pf,
                flags,
            );
            if srcBuf.is_null() {
                printf(
                    b"ERROR in line %d while %s:\n%s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1040 as ::core::ffi::c_int,
                    b"loading input image\0" as *const u8 as *const ::core::ffi::c_char,
                    tjGetErrorStr2(NULL),
                );
                retval = -(1 as ::core::ffi::c_int);
                current_block = 17745071501396862765;
            } else {
                temp = strrchr(*argv.offset(1 as ::core::ffi::c_int as isize), '.' as i32);
                if !temp.is_null() {
                    *temp = '\0' as i32 as ::core::ffi::c_char;
                }
                current_block = 11735322225073324345;
            }
        } else {
            current_block = 11735322225073324345;
        }
        match current_block {
            17745071501396862765 => {}
            _ => {
                if quiet == 1 as ::core::ffi::c_int && decompOnly == 0 {
                    printf(
                        b"All performance values in Mpixels/sec\n\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    printf(
                        b"Pixel      JPEG     JPEG  %s  %s   \0" as *const u8
                            as *const ::core::ffi::c_char,
                        if doTile != 0 {
                            b"Tile \0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"Image\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        if doTile != 0 {
                            b"Tile \0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"Image\0" as *const u8 as *const ::core::ffi::c_char
                        },
                    );
                    if doYUV != 0 {
                        printf(b"Encode  \0" as *const u8 as *const ::core::ffi::c_char);
                    }
                    printf(
                        b"Comp    Comp    Decomp  \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if doYUV != 0 {
                        printf(b"Decode\0" as *const u8 as *const ::core::ffi::c_char);
                    }
                    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                    printf(
                        b"Format     Subsamp  Qual  Width  Height  \0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    if doYUV != 0 {
                        printf(b"Perf    \0" as *const u8 as *const ::core::ffi::c_char);
                    }
                    printf(
                        b"Perf    Ratio   Perf    \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if doYUV != 0 {
                        printf(b"Perf\0" as *const u8 as *const ::core::ffi::c_char);
                    }
                    printf(b"\n\n\0" as *const u8 as *const ::core::ffi::c_char);
                }
                if decompOnly != 0 {
                    decompTest(*argv.offset(1 as ::core::ffi::c_int as isize));
                    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                } else if subsamp >= 0 as ::core::ffi::c_int && subsamp < TJ_NUMSAMP {
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            subsamp,
                            i,
                            *argv.offset(1 as ::core::ffi::c_int as isize),
                        );
                        i -= 1;
                    }
                    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                } else {
                    if pf != TJPF_CMYK as ::core::ffi::c_int {
                        i = maxQual;
                        while i >= minQual {
                            fullTest(
                                srcBuf,
                                w,
                                h,
                                TJSAMP_GRAY as ::core::ffi::c_int,
                                i,
                                *argv.offset(1 as ::core::ffi::c_int as isize),
                            );
                            i -= 1;
                        }
                        printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                    }
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            TJSAMP_420 as ::core::ffi::c_int,
                            i,
                            *argv.offset(1 as ::core::ffi::c_int as isize),
                        );
                        i -= 1;
                    }
                    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            TJSAMP_422 as ::core::ffi::c_int,
                            i,
                            *argv.offset(1 as ::core::ffi::c_int as isize),
                        );
                        i -= 1;
                    }
                    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                    i = maxQual;
                    while i >= minQual {
                        fullTest(
                            srcBuf,
                            w,
                            h,
                            TJSAMP_444 as ::core::ffi::c_int,
                            i,
                            *argv.offset(1 as ::core::ffi::c_int as isize),
                        );
                        i -= 1;
                    }
                    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                }
            }
        }
    }
    tjFree(srcBuf);
    return retval;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
