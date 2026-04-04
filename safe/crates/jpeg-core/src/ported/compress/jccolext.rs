// Upstream includes jccolext.c multiple times from jccolor.c with different
// macro bindings. The translated implementation therefore lives in jccolor.rs.
pub use super::jccolor::*;

pub const JPEG_RS_JCCOLEXT_LINK_ANCHOR: unsafe extern "C" fn(j_compress_ptr) =
    jinit_color_converter;
