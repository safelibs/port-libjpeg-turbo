use std::os::raw::{c_char, c_int};

#[link(name = "turbojpeg")]
unsafe extern "C" {}

#[link(name = "jpeg_tools_tjexample_tool", kind = "static")]
unsafe extern "C" {
    fn safe_tjexample_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

fn main() {
    jpeg_tools::run_embedded_tool("tjexample", safe_tjexample_main);
}
