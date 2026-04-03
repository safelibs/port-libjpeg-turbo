use core::ffi::{c_int, c_ulong, c_void};

pub type TjHandle = *mut c_void;

pub const TJ_NUMSAMP: c_int = 6;
pub const TJ_NUMPF: c_int = 12;

pub const TJSAMP_444: c_int = 0;
pub const TJSAMP_422: c_int = 1;
pub const TJSAMP_420: c_int = 2;
pub const TJSAMP_GRAY: c_int = 3;
pub const TJSAMP_440: c_int = 4;
pub const TJSAMP_411: c_int = 5;

pub const TJPF_RGB: c_int = 0;
pub const TJPF_BGR: c_int = 1;
pub const TJPF_RGBX: c_int = 2;
pub const TJPF_BGRX: c_int = 3;
pub const TJPF_XBGR: c_int = 4;
pub const TJPF_XRGB: c_int = 5;
pub const TJPF_GRAY: c_int = 6;
pub const TJPF_RGBA: c_int = 7;
pub const TJPF_BGRA: c_int = 8;
pub const TJPF_ABGR: c_int = 9;
pub const TJPF_ARGB: c_int = 10;
pub const TJPF_CMYK: c_int = 11;
pub const TJPF_UNKNOWN: c_int = -1;

pub const TJFLAG_BOTTOMUP: c_int = 2;
pub const TJFLAG_FORCEMMX: c_int = 8;
pub const TJFLAG_FORCESSE: c_int = 16;
pub const TJFLAG_FORCESSE2: c_int = 32;
pub const TJFLAG_FORCESSE3: c_int = 128;
pub const TJFLAG_FASTUPSAMPLE: c_int = 256;
pub const TJFLAG_NOREALLOC: c_int = 1024;
pub const TJFLAG_FASTDCT: c_int = 2048;
pub const TJFLAG_ACCURATEDCT: c_int = 4096;
pub const TJFLAG_STOPONWARNING: c_int = 8192;
pub const TJFLAG_PROGRESSIVE: c_int = 16384;
pub const TJFLAG_LIMITSCANS: c_int = 32768;

pub const TJERR_WARNING: c_int = 0;
pub const TJERR_FATAL: c_int = 1;

pub const TJ_MCU_WIDTH: [c_int; TJ_NUMSAMP as usize] = [8, 16, 16, 8, 8, 32];
pub const TJ_MCU_HEIGHT: [c_int; TJ_NUMSAMP as usize] = [8, 8, 16, 8, 16, 8];

pub const TJ_RED_OFFSET: [c_int; TJ_NUMPF as usize] = [0, 2, 0, 2, 3, 1, 0, 0, 2, 3, 1, -1];
pub const TJ_GREEN_OFFSET: [c_int; TJ_NUMPF as usize] =
    [1, 1, 1, 1, 2, 2, 0, 1, 1, 2, 2, -1];
pub const TJ_BLUE_OFFSET: [c_int; TJ_NUMPF as usize] =
    [2, 0, 2, 0, 1, 3, 0, 2, 0, 1, 3, -1];
pub const TJ_ALPHA_OFFSET: [c_int; TJ_NUMPF as usize] =
    [-1, -1, -1, -1, -1, -1, -1, 3, 3, 0, 0, -1];
pub const TJ_PIXEL_SIZE: [c_int; TJ_NUMPF as usize] = [3, 3, 4, 4, 4, 4, 1, 4, 4, 4, 4, 4];

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct tjscalingfactor {
    pub num: c_int,
    pub denom: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct tjregion {
    pub x: c_int,
    pub y: c_int,
    pub w: c_int,
    pub h: c_int,
}

pub type tjtransform_custom_filter = Option<
    unsafe extern "C" fn(
        coeffs: *mut i16,
        array_region: tjregion,
        plane_region: tjregion,
        component_index: c_int,
        transform_index: c_int,
        transform: *mut tjtransform,
    ) -> c_int,
>;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct tjtransform {
    pub r: tjregion,
    pub op: c_int,
    pub options: c_int,
    pub data: *mut c_void,
    pub custom_filter: tjtransform_custom_filter,
}

impl Default for tjtransform {
    fn default() -> Self {
        Self {
            r: tjregion::default(),
            op: 0,
            options: 0,
            data: core::ptr::null_mut(),
            custom_filter: None,
        }
    }
}

pub type TjBufSizeFn = unsafe extern "C" fn(width: c_int, height: c_int, subsamp: c_int) -> c_ulong;
