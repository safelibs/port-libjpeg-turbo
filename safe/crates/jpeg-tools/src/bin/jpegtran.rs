#[allow(warnings, clippy::all)]
mod generated {
    #[path = "../../generated/cdjpeg.rs"]
    pub mod cdjpeg;
    #[path = "../../generated/jpegtran.rs"]
    pub mod jpegtran;
    #[path = "../../generated/rdswitch.rs"]
    pub mod rdswitch;
}

const TOOL_NAME: &str = "jpegtran";

fn main() {
    let _ = jpeg_tools::packaged_tool_contract(TOOL_NAME);
    let _ = libjpeg_abi::common_exports::jpeg_std_error as *const ();
    let _ = libjpeg_abi::compress::jctrans::jpeg_write_coefficients as *const ();
    let _ = libjpeg_abi::transform::transupp::jtransform_execute_transform as *const ();
    generated::jpegtran::main();
}
