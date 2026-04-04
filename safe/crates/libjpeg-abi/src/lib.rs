#![allow(clippy::all)]

pub mod common_exports;
pub mod decompress_exports;
#[allow(warnings)]
mod jsimd_none;

#[doc(hidden)]
pub use jpeg_core::ported::{compress, transform};

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
