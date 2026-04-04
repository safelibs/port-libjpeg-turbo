#![allow(clippy::all)]

pub mod common_exports;
pub mod decompress_exports;
#[allow(warnings)]
mod jsimd_none;

#[doc(hidden)]
pub use jpeg_core::ported::{compress, decompress, transform};

// Keep the minimal C longjmp/error bridge as a propagated native link
// dependency for final binaries that pull in jpeg_core through libjpeg-abi.
#[allow(improper_ctypes)]
#[link(name = "error_bridge", kind = "static")]
unsafe extern "C" {
    pub fn jpeg_rs_invoke_error_exit(cinfo: ffi_types::j_common_ptr);
}

#[used]
static JPEG_RS_ERROR_BRIDGE_LINK_GUARD: unsafe extern "C" fn(ffi_types::j_common_ptr) =
    jpeg_rs_invoke_error_exit;

pub const SONAME: &str = "libjpeg.so.8";
pub const LINK_NAME: &str = "jpeg";
pub const EXPECTED_COMPRESS_SYMBOLS: &[&str] = &[
    "jpeg_CreateCompress",
    "jpeg_destroy_compress",
    "jpeg_abort_compress",
    "jpeg_finish_compress",
    "jpeg_start_compress",
    "jpeg_write_scanlines",
    "jpeg_write_raw_data",
    "jpeg_write_tables",
    "jpeg_write_marker",
    "jpeg_write_m_header",
    "jpeg_write_m_byte",
    "jpeg_set_defaults",
    "jpeg_default_colorspace",
    "jpeg_set_colorspace",
    "jpeg_set_quality",
    "jpeg_set_linear_quality",
    "jpeg_simple_progression",
    "jpeg_suppress_tables",
    "jpeg_write_coefficients",
    "jpeg_copy_critical_parameters",
];

// Keep one symbol per encoder/transcode object file anchored in libjpeg-abi so
// downstream link modes continue to resolve the shared Rust codec core without
// needing jpegtran's generated transupp copy.
#[used]
static JPEG_RS_JCAPIMIN_LINK_GUARD: unsafe extern "C" fn(
    compress::jcapimin::j_compress_ptr,
    ::core::ffi::c_int,
    usize,
) = compress::jcapimin::jpeg_CreateCompress;
#[used]
static JPEG_RS_JCAPISTD_LINK_GUARD: unsafe extern "C" fn(
    compress::jcapistd::j_compress_ptr,
    compress::jcapistd::boolean,
) = compress::jcapistd::jpeg_start_compress;
#[used]
static JPEG_RS_JCPARAM_LINK_GUARD: unsafe extern "C" fn(compress::jcparam::j_compress_ptr) =
    compress::jcparam::jpeg_set_defaults;
#[used]
static JPEG_RS_JCTRANS_LINK_GUARD: unsafe extern "C" fn(
    compress::jctrans::j_compress_ptr,
    *mut compress::jctrans::jvirt_barray_ptr,
) = compress::jctrans::jpeg_write_coefficients;
#[used]
static JPEG_RS_TRANSUPP_LINK_GUARD: unsafe extern "C" fn(
    transform::transupp::j_decompress_ptr,
    transform::transupp::j_compress_ptr,
    *mut transform::transupp::jvirt_barray_ptr,
    *mut transform::transupp::jpeg_transform_info,
) = transform::transupp::jtransform_execute_transform;

#[inline]
pub unsafe fn configure_decompress_policy(
    cinfo: ffi_types::j_decompress_ptr,
    max_scans: ffi_types::int,
    warnings_fatal: ffi_types::boolean,
) {
    jpeg_core::common::registry::configure_decompress_policy(cinfo, max_scans, warnings_fatal)
}

#[inline]
pub unsafe fn set_decompress_scan_limit(
    cinfo: ffi_types::j_decompress_ptr,
    max_scans: ffi_types::int,
) {
    jpeg_core::common::registry::set_decompress_scan_limit(cinfo, max_scans)
}

#[inline]
pub unsafe fn decompress_scan_limit(cinfo: ffi_types::j_decompress_ptr) -> ffi_types::int {
    jpeg_core::common::registry::decompress_scan_limit(cinfo).unwrap_or(0)
}

#[inline]
pub unsafe fn set_decompress_warnings_fatal(
    cinfo: ffi_types::j_decompress_ptr,
    fatal: ffi_types::boolean,
) {
    jpeg_core::common::registry::set_decompress_warnings_fatal(cinfo, fatal)
}

#[inline]
pub unsafe fn decompress_warnings_fatal(cinfo: ffi_types::j_decompress_ptr) -> ffi_types::boolean {
    jpeg_core::common::registry::decompress_warnings_fatal_flag(cinfo)
}
