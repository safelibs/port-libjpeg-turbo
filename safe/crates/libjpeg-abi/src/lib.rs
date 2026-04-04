#![allow(clippy::all)]

pub mod common_exports;
pub mod decompress_exports;

#[doc(hidden)]
pub use jpeg_core::ported::{compress, transform};

pub const SONAME: &str = "libjpeg.so.8";
pub const LINK_NAME: &str = "jpeg";
