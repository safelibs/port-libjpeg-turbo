use std::os::raw::{c_char, c_int};

#[link(name = "jpeg")]
unsafe extern "C" {}

#[link(name = "jpeg_tools_jpegtran_tool", kind = "static")]
unsafe extern "C" {
    fn safe_jpegtran_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

fn main() {
    jpeg_tools::run_embedded_tool("jpegtran", safe_jpegtran_main);
}
