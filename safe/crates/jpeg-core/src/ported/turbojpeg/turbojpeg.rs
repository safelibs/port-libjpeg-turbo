use core::ffi::{c_int, c_ulong, c_void};

pub type TjHandle = *mut c_void;

pub const SOURCE_FILE: &str = "turbojpeg.c";
pub const HEADER_FILE: &str = "turbojpeg.h";

pub const TJ_NUMSAMP: c_int = 6;
pub const TJ_NUMPF: c_int = 12;
pub const TJ_NUMCS: c_int = 5;

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

pub const TJCS_RGB: c_int = 0;
pub const TJCS_YCBCR: c_int = 1;
pub const TJCS_GRAY: c_int = 2;
pub const TJCS_CMYK: c_int = 3;
pub const TJCS_YCCK: c_int = 4;

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

pub const TJXOP_NONE: c_int = 0;
pub const TJXOP_HFLIP: c_int = 1;
pub const TJXOP_VFLIP: c_int = 2;
pub const TJXOP_TRANSPOSE: c_int = 3;
pub const TJXOP_TRANSVERSE: c_int = 4;
pub const TJXOP_ROT90: c_int = 5;
pub const TJXOP_ROT180: c_int = 6;
pub const TJXOP_ROT270: c_int = 7;

pub const TJXOPT_PERFECT: c_int = 1;
pub const TJXOPT_TRIM: c_int = 2;
pub const TJXOPT_CROP: c_int = 4;
pub const TJXOPT_GRAY: c_int = 8;
pub const TJXOPT_NOOUTPUT: c_int = 16;
pub const TJXOPT_PROGRESSIVE: c_int = 32;
pub const TJXOPT_COPYNONE: c_int = 64;

pub const TJ_MCU_WIDTH: [c_int; TJ_NUMSAMP as usize] = [8, 16, 16, 8, 8, 32];
pub const TJ_MCU_HEIGHT: [c_int; TJ_NUMSAMP as usize] = [8, 8, 16, 8, 16, 8];

pub const TJ_RED_OFFSET: [c_int; TJ_NUMPF as usize] = [0, 2, 0, 2, 3, 1, 0, 0, 2, 3, 1, -1];
pub const TJ_GREEN_OFFSET: [c_int; TJ_NUMPF as usize] = [1, 1, 1, 1, 2, 2, 0, 1, 1, 2, 2, -1];
pub const TJ_BLUE_OFFSET: [c_int; TJ_NUMPF as usize] = [2, 0, 2, 0, 1, 3, 0, 2, 0, 1, 3, -1];
pub const TJ_ALPHA_OFFSET: [c_int; TJ_NUMPF as usize] =
    [-1, -1, -1, -1, -1, -1, -1, 3, 3, 0, 0, -1];
pub const TJ_PIXEL_SIZE: [c_int; TJ_NUMPF as usize] = [3, 3, 4, 4, 4, 4, 1, 4, 4, 4, 4, 4];
pub const TJ_SUBSAMP_NAMES: [&str; TJ_NUMSAMP as usize] =
    ["4:4:4", "4:2:2", "4:2:0", "Grayscale", "4:4:0", "4:1:1"];
pub const TJ_COLORSPACE_NAMES: [&str; TJ_NUMCS as usize] = ["RGB", "YCbCr", "GRAY", "CMYK", "YCCK"];

pub const NUM_SCALING_FACTORS: c_int = 16;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct tjscalingfactor {
    pub num: c_int,
    pub denom: c_int,
}

pub static SCALING_FACTORS: [tjscalingfactor; NUM_SCALING_FACTORS as usize] = [
    tjscalingfactor { num: 2, denom: 1 },
    tjscalingfactor { num: 15, denom: 8 },
    tjscalingfactor { num: 7, denom: 4 },
    tjscalingfactor { num: 13, denom: 8 },
    tjscalingfactor { num: 3, denom: 2 },
    tjscalingfactor { num: 11, denom: 8 },
    tjscalingfactor { num: 5, denom: 4 },
    tjscalingfactor { num: 9, denom: 8 },
    tjscalingfactor { num: 1, denom: 1 },
    tjscalingfactor { num: 7, denom: 8 },
    tjscalingfactor { num: 3, denom: 4 },
    tjscalingfactor { num: 5, denom: 8 },
    tjscalingfactor { num: 1, denom: 2 },
    tjscalingfactor { num: 3, denom: 8 },
    tjscalingfactor { num: 1, denom: 4 },
    tjscalingfactor { num: 1, denom: 8 },
];

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TjMathError {
    InvalidArgument,
    WidthTooLarge,
    HeightTooLarge,
    ImageTooLarge,
}

const fn pad_u128(value: u128, align: u128) -> u128 {
    (value + align - 1) & !(align - 1)
}

const fn plane_count_for_subsamp(subsamp: c_int) -> Option<c_int> {
    if subsamp < 0 || subsamp >= TJ_NUMSAMP {
        None
    } else if subsamp == TJSAMP_GRAY {
        Some(1)
    } else {
        Some(3)
    }
}

pub fn legacy_buf_size_checked(width: c_int, height: c_int) -> Result<c_ulong, TjMathError> {
    if width < 1 || height < 1 {
        return Err(TjMathError::InvalidArgument);
    }

    let retval = pad_u128(width as u128, 16) * pad_u128(height as u128, 16) * 6 + 2048;
    if retval > c_ulong::MAX as u128 {
        Err(TjMathError::ImageTooLarge)
    } else {
        Ok(retval as c_ulong)
    }
}

pub fn buf_size_checked(
    width: c_int,
    height: c_int,
    jpeg_subsamp: c_int,
) -> Result<c_ulong, TjMathError> {
    if width < 1 || height < 1 || plane_count_for_subsamp(jpeg_subsamp).is_none() {
        return Err(TjMathError::InvalidArgument);
    }

    let mcu_width = TJ_MCU_WIDTH[jpeg_subsamp as usize] as u128;
    let mcu_height = TJ_MCU_HEIGHT[jpeg_subsamp as usize] as u128;
    let chroma_sf = if jpeg_subsamp == TJSAMP_GRAY {
        0
    } else {
        4 * 64 / (mcu_width * mcu_height)
    };
    let retval =
        pad_u128(width as u128, mcu_width) * pad_u128(height as u128, mcu_height) * (2 + chroma_sf)
            + 2048;

    if retval > c_ulong::MAX as u128 {
        Err(TjMathError::ImageTooLarge)
    } else {
        Ok(retval as c_ulong)
    }
}

pub fn plane_width_checked(
    component_id: c_int,
    width: c_int,
    subsamp: c_int,
) -> Result<c_int, TjMathError> {
    if width < 1 {
        return Err(TjMathError::InvalidArgument);
    }
    let components = plane_count_for_subsamp(subsamp).ok_or(TjMathError::InvalidArgument)?;
    if component_id < 0 || component_id >= components {
        return Err(TjMathError::InvalidArgument);
    }

    let mcu_width = TJ_MCU_WIDTH[subsamp as usize] as u128;
    let padded = pad_u128(width as u128, mcu_width / 8);
    let retval = if component_id == 0 {
        padded
    } else {
        padded * 8 / mcu_width
    };

    if retval > i32::MAX as u128 {
        Err(TjMathError::WidthTooLarge)
    } else {
        Ok(retval as c_int)
    }
}

pub fn plane_height_checked(
    component_id: c_int,
    height: c_int,
    subsamp: c_int,
) -> Result<c_int, TjMathError> {
    if height < 1 {
        return Err(TjMathError::InvalidArgument);
    }
    let components = plane_count_for_subsamp(subsamp).ok_or(TjMathError::InvalidArgument)?;
    if component_id < 0 || component_id >= components {
        return Err(TjMathError::InvalidArgument);
    }

    let mcu_height = TJ_MCU_HEIGHT[subsamp as usize] as u128;
    let padded = pad_u128(height as u128, mcu_height / 8);
    let retval = if component_id == 0 {
        padded
    } else {
        padded * 8 / mcu_height
    };

    if retval > i32::MAX as u128 {
        Err(TjMathError::HeightTooLarge)
    } else {
        Ok(retval as c_int)
    }
}

pub fn plane_size_yuv_checked(
    component_id: c_int,
    width: c_int,
    stride: c_int,
    height: c_int,
    subsamp: c_int,
) -> Result<c_ulong, TjMathError> {
    if width < 1 || height < 1 || plane_count_for_subsamp(subsamp).is_none() {
        return Err(TjMathError::InvalidArgument);
    }

    let plane_width = plane_width_checked(component_id, width, subsamp)? as u128;
    let plane_height = plane_height_checked(component_id, height, subsamp)? as u128;
    let abs_stride = if stride == 0 {
        plane_width
    } else {
        stride.unsigned_abs() as u128
    };
    let retval = abs_stride * (plane_height - 1) + plane_width;

    if retval > c_ulong::MAX as u128 {
        Err(TjMathError::ImageTooLarge)
    } else {
        Ok(retval as c_ulong)
    }
}

pub fn subsamp_name(subsamp: c_int) -> &'static str {
    TJ_SUBSAMP_NAMES
        .get(subsamp as usize)
        .copied()
        .unwrap_or("Unknown")
}

pub fn colorspace_name(colorspace: c_int) -> &'static str {
    TJ_COLORSPACE_NAMES
        .get(colorspace as usize)
        .copied()
        .unwrap_or("Unknown")
}

pub const fn scaled(dimension: c_int, factor: tjscalingfactor) -> c_int {
    (dimension * factor.num + factor.denom - 1) / factor.denom
}

pub fn buf_size_yuv2_checked(
    width: c_int,
    align: c_int,
    height: c_int,
    subsamp: c_int,
) -> Result<c_ulong, TjMathError> {
    if align < 1 || (align & (align - 1)) != 0 {
        return Err(TjMathError::InvalidArgument);
    }

    let components = plane_count_for_subsamp(subsamp).ok_or(TjMathError::InvalidArgument)?;
    let mut retval = 0u128;
    let align = align as u128;

    for component_id in 0..components {
        let plane_width = plane_width_checked(component_id, width, subsamp)? as u128;
        let plane_height = plane_height_checked(component_id, height, subsamp)? as u128;
        retval += pad_u128(plane_width, align) * plane_height;
    }

    if retval > c_ulong::MAX as u128 {
        Err(TjMathError::ImageTooLarge)
    } else {
        Ok(retval as c_ulong)
    }
}

pub fn buf_size_yuv_checked(
    width: c_int,
    height: c_int,
    subsamp: c_int,
) -> Result<c_ulong, TjMathError> {
    buf_size_yuv2_checked(width, 4, height, subsamp)
}
