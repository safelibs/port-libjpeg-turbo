#![allow(clippy::all)]

pub mod common_exports;
pub mod decompress_exports;
#[allow(warnings)]
mod jsimd_none;

#[doc(hidden)]
pub use jpeg_core::ported::{compress, decompress, transform};

// Keep the single reviewed C longjmp/error bridge as the only propagated native
// build dependency for final binaries that pull in jpeg_core through
// libjpeg-abi.
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

// Anchor the full Rust compression/transcode pipeline from the ported object
// modules themselves so downstream links keep resolving the shared codec core.
#[used]
static JPEG_RS_JCAPIMIN_LINK_GUARD: unsafe extern "C" fn(
    compress::jcapimin::j_compress_ptr,
    ::core::ffi::c_int,
    usize,
) = compress::jcapimin::JPEG_RS_JCAPIMIN_LINK_ANCHOR;
#[used]
static JPEG_RS_JCAPISTD_LINK_GUARD: unsafe extern "C" fn(
    compress::jcapistd::j_compress_ptr,
    compress::jcapistd::boolean,
) = compress::jcapistd::JPEG_RS_JCAPISTD_LINK_ANCHOR;
#[used]
static JPEG_RS_JCINIT_LINK_GUARD: unsafe extern "C" fn(compress::jcinit::j_compress_ptr) =
    compress::jcinit::JPEG_RS_JCINIT_LINK_ANCHOR;
#[used]
static JPEG_RS_JCMASTER_LINK_GUARD: unsafe extern "C" fn(
    compress::jcmaster::j_compress_ptr,
    compress::jcmaster::boolean,
) = compress::jcmaster::JPEG_RS_JCMASTER_LINK_ANCHOR;
#[used]
static JPEG_RS_JCPARAM_LINK_GUARD: unsafe extern "C" fn(compress::jcparam::j_compress_ptr) =
    compress::jcparam::JPEG_RS_JCPARAM_LINK_ANCHOR;
#[used]
static JPEG_RS_JCMAINCT_LINK_GUARD: unsafe extern "C" fn(
    compress::jcmainct::j_compress_ptr,
    compress::jcmainct::boolean,
) = compress::jcmainct::JPEG_RS_JCMAINCT_LINK_ANCHOR;
#[used]
static JPEG_RS_JCPREPCT_LINK_GUARD: unsafe extern "C" fn(
    compress::jcprepct::j_compress_ptr,
    compress::jcprepct::boolean,
) = compress::jcprepct::JPEG_RS_JCPREPCT_LINK_ANCHOR;
#[used]
static JPEG_RS_JCCOLOR_LINK_GUARD: unsafe extern "C" fn(compress::jccolor::j_compress_ptr) =
    compress::jccolor::JPEG_RS_JCCOLOR_LINK_ANCHOR;
#[used]
static JPEG_RS_JCCOLEXT_LINK_GUARD: unsafe extern "C" fn(compress::jccolext::j_compress_ptr) =
    compress::jccolext::JPEG_RS_JCCOLEXT_LINK_ANCHOR;
#[used]
static JPEG_RS_JCSAMPLE_LINK_GUARD: unsafe extern "C" fn(compress::jcsample::j_compress_ptr) =
    compress::jcsample::JPEG_RS_JCSAMPLE_LINK_ANCHOR;
#[used]
static JPEG_RS_JCDCTMGR_LINK_GUARD: unsafe extern "C" fn(compress::jcdctmgr::j_compress_ptr) =
    compress::jcdctmgr::JPEG_RS_JCDCTMGR_LINK_ANCHOR;
#[used]
static JPEG_RS_JFDCTINT_LINK_GUARD: unsafe extern "C" fn(*mut compress::jfdctint::DCTELEM) =
    compress::jfdctint::JPEG_RS_JFDCTINT_LINK_ANCHOR;
#[used]
static JPEG_RS_JFDCTFST_LINK_GUARD: unsafe extern "C" fn(*mut compress::jfdctfst::DCTELEM) =
    compress::jfdctfst::JPEG_RS_JFDCTFST_LINK_ANCHOR;
#[used]
static JPEG_RS_JFDCTFLT_LINK_GUARD: unsafe extern "C" fn(*mut ::core::ffi::c_float) =
    compress::jfdctflt::JPEG_RS_JFDCTFLT_LINK_ANCHOR;
#[used]
static JPEG_RS_JCHUFF_LINK_GUARD: unsafe extern "C" fn(compress::jchuff::j_compress_ptr) =
    compress::jchuff::JPEG_RS_JCHUFF_LINK_ANCHOR;
#[used]
static JPEG_RS_JCPHUFF_LINK_GUARD: unsafe extern "C" fn(compress::jcphuff::j_compress_ptr) =
    compress::jcphuff::JPEG_RS_JCPHUFF_LINK_ANCHOR;
#[used]
static JPEG_RS_JCARITH_LINK_GUARD: unsafe extern "C" fn(compress::jcarith::j_compress_ptr) =
    compress::jcarith::JPEG_RS_JCARITH_LINK_ANCHOR;
#[used]
static JPEG_RS_JCMARKER_LINK_GUARD: unsafe extern "C" fn(compress::jcmarker::j_compress_ptr) =
    compress::jcmarker::JPEG_RS_JCMARKER_LINK_ANCHOR;
#[used]
static JPEG_RS_JCTRANS_LINK_GUARD: unsafe extern "C" fn(
    compress::jctrans::j_compress_ptr,
    *mut compress::jctrans::jvirt_barray_ptr,
) = compress::jctrans::JPEG_RS_JCTRANS_LINK_ANCHOR;
#[used]
static JPEG_RS_JCCOEFCT_LINK_GUARD: unsafe extern "C" fn(
    compress::jccoefct::j_compress_ptr,
    compress::jccoefct::boolean,
) = compress::jccoefct::JPEG_RS_JCCOEFCT_LINK_ANCHOR;
#[used]
static JPEG_RS_TRANSUPP_REQUEST_WORKSPACE_LINK_GUARD: unsafe extern "C" fn(
    transform::transupp::j_decompress_ptr,
    *mut transform::transupp::jpeg_transform_info,
) -> ffi_types::boolean = transform::transupp::JPEG_RS_TRANSUPP_REQUEST_WORKSPACE_LINK_ANCHOR;
#[used]
static JPEG_RS_TRANSUPP_ADJUST_PARAMETERS_LINK_GUARD:
    unsafe extern "C" fn(
        transform::transupp::j_decompress_ptr,
        transform::transupp::j_compress_ptr,
        *mut transform::transupp::jvirt_barray_ptr,
        *mut transform::transupp::jpeg_transform_info,
    ) -> *mut transform::transupp::jvirt_barray_ptr =
    transform::transupp::JPEG_RS_TRANSUPP_ADJUST_PARAMETERS_LINK_ANCHOR;
#[used]
static JPEG_RS_TRANSUPP_EXECUTE_LINK_GUARD: unsafe extern "C" fn(
    transform::transupp::j_decompress_ptr,
    transform::transupp::j_compress_ptr,
    *mut transform::transupp::jvirt_barray_ptr,
    *mut transform::transupp::jpeg_transform_info,
) = transform::transupp::JPEG_RS_TRANSUPP_EXECUTE_LINK_ANCHOR;
#[used]
static JPEG_RS_TRANSUPP_PERFECT_LINK_GUARD: unsafe extern "C" fn(
    ffi_types::JDIMENSION,
    ffi_types::JDIMENSION,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    transform::transupp::JXFORM_CODE,
) -> ffi_types::boolean = transform::transupp::JPEG_RS_TRANSUPP_PERFECT_LINK_ANCHOR;
#[used]
static JPEG_RS_TRANSUPP_COPY_SETUP_LINK_GUARD: unsafe extern "C" fn(
    transform::transupp::j_decompress_ptr,
    transform::transupp::JCOPY_OPTION,
) = transform::transupp::JPEG_RS_TRANSUPP_COPY_SETUP_LINK_ANCHOR;
#[used]
static JPEG_RS_TRANSUPP_COPY_EXECUTE_LINK_GUARD: unsafe extern "C" fn(
    transform::transupp::j_decompress_ptr,
    transform::transupp::j_compress_ptr,
    transform::transupp::JCOPY_OPTION,
) = transform::transupp::JPEG_RS_TRANSUPP_COPY_EXECUTE_LINK_ANCHOR;

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
