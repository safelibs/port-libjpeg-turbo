use ffi_types::{
    JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA,
    JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_RGB,
};

pub const JPEG_NUMCS: usize = 17;

pub const RGB_PIXELSIZE: [i32; JPEG_NUMCS] = {
    let mut table = [-1; JPEG_NUMCS];
    table[JCS_RGB as usize] = 3;
    table[JCS_EXT_RGB as usize] = 3;
    table[JCS_EXT_RGBX as usize] = 4;
    table[JCS_EXT_BGR as usize] = 3;
    table[JCS_EXT_BGRX as usize] = 4;
    table[JCS_EXT_XBGR as usize] = 4;
    table[JCS_EXT_XRGB as usize] = 4;
    table[JCS_EXT_RGBA as usize] = 4;
    table[JCS_EXT_BGRA as usize] = 4;
    table[JCS_EXT_ABGR as usize] = 4;
    table[JCS_EXT_ARGB as usize] = 4;
    table
};

#[inline]
pub const fn rgb_pixelsize_for(color_space: usize) -> i32 {
    if color_space < JPEG_NUMCS {
        RGB_PIXELSIZE[color_space]
    } else {
        -1
    }
}
