#[allow(warnings, clippy::all)]
mod generated {
    #[path = "../../generated/cdjpeg.rs"]
    pub mod cdjpeg;
    #[path = "../../generated/jpegtran.rs"]
    pub mod jpegtran;
    #[path = "../../generated/rdswitch.rs"]
    pub mod rdswitch;
}

fn main() {
    let _ = libjpeg_abi::common_exports::jpeg_std_error as *const ();
    let _ = libjpeg_abi::compress::jctrans::jpeg_write_coefficients as *const ();
    let _ = libjpeg_abi::transform::transupp::jtransform_execute_transform as *const ();
    generated::jpegtran::main();
}
