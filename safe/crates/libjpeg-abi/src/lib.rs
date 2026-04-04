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
