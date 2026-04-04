use core::ffi::{c_int, c_ulong};

use super::turbojpeg::{
    tjscalingfactor, SCALING_FACTORS, TJSAMP_420, TJSAMP_422, TJSAMP_444, TJSAMP_GRAY,
    TJ_MCU_HEIGHT, TJ_MCU_WIDTH,
};

pub const SOURCE_FILE: &str = "tjutil.c";

pub const fn pad(value: c_int, align: c_int) -> c_int {
    if align <= 0 {
        value
    } else {
        (value + align - 1) & !(align - 1)
    }
}

pub const fn plane_count(subsamp: c_int) -> c_int {
    if subsamp == TJSAMP_GRAY {
        1
    } else {
        3
    }
}

pub fn plane_width(component: c_int, width: c_int, subsamp: c_int) -> Option<c_int> {
    let hsf = *TJ_MCU_WIDTH.get(subsamp as usize)? / 8;
    let padded = pad(width, hsf);
    Some(if component == 0 { padded } else { padded / hsf })
}

pub fn plane_height(component: c_int, height: c_int, subsamp: c_int) -> Option<c_int> {
    let vsf = *TJ_MCU_HEIGHT.get(subsamp as usize)? / 8;
    let padded = pad(height, vsf);
    Some(if component == 0 { padded } else { padded / vsf })
}

pub fn plane_size(
    component: c_int,
    width: c_int,
    stride: c_int,
    height: c_int,
    subsamp: c_int,
) -> Option<c_ulong> {
    let plane_width = plane_width(component, width, subsamp)? as c_ulong;
    let plane_height = plane_height(component, height, subsamp)? as c_ulong;
    let abs_stride = if stride == 0 {
        plane_width
    } else {
        stride.unsigned_abs() as c_ulong
    };
    plane_height
        .checked_sub(1)
        .and_then(|rows| abs_stride.checked_mul(rows))
        .and_then(|prefix| prefix.checked_add(plane_width))
}

pub fn yuv_size(width: c_int, align: c_int, height: c_int, subsamp: c_int) -> Option<c_ulong> {
    if align <= 0 || (align & (align - 1)) != 0 {
        return None;
    }

    let mut total = 0 as c_ulong;
    let count = plane_count(subsamp);
    let mut component = 0;
    while component < count {
        let plane_width = plane_width(component, width, subsamp)?;
        let stride = pad(plane_width, align);
        let plane_height = plane_height(component, height, subsamp)? as c_ulong;
        total = total.checked_add((stride as c_ulong).checked_mul(plane_height)?)?;
        component += 1;
    }
    Some(total)
}

pub fn parse_subsamp(spec: &str) -> Option<c_int> {
    if spec
        .as_bytes()
        .first()
        .is_some_and(|byte| byte.eq_ignore_ascii_case(&b'g'))
    {
        Some(TJSAMP_GRAY)
    } else if spec == "444" {
        Some(TJSAMP_444)
    } else if spec == "422" {
        Some(TJSAMP_422)
    } else if spec == "420" {
        Some(TJSAMP_420)
    } else {
        None
    }
}

pub fn parse_scaling_factor(spec: &str) -> Option<tjscalingfactor> {
    let (num, denom) = spec.split_once('/')?;
    let num = num.parse::<c_int>().ok()?;
    let denom = denom.parse::<c_int>().ok()?;
    if num <= 0 || denom <= 0 {
        return None;
    }

    SCALING_FACTORS.iter().copied().find(|factor| {
        i64::from(num) * i64::from(factor.denom) == i64::from(factor.num) * i64::from(denom)
    })
}

pub fn format_scaling_factor_list() -> String {
    SCALING_FACTORS
        .iter()
        .map(|factor| format!("{}/{}", factor.num, factor.denom))
        .collect::<Vec<_>>()
        .join(", ")
}
