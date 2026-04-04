#[allow(warnings, clippy::all)]
mod generated {
    #[path = "../../generated/cdjpeg.rs"]
    pub mod cdjpeg;
    #[path = "../../generated/djpeg.rs"]
    pub mod djpeg;
    #[path = "../../generated/rdcolmap.rs"]
    pub mod rdcolmap;
    #[path = "../../generated/rdswitch.rs"]
    pub mod rdswitch;
    #[path = "../../generated/wrbmp.rs"]
    pub mod wrbmp;
    #[path = "../../generated/wrgif.rs"]
    pub mod wrgif;
    #[path = "../../generated/wrppm.rs"]
    pub mod wrppm;
    #[path = "../../generated/wrtarga.rs"]
    pub mod wrtarga;
}

const TOOL_NAME: &str = "djpeg";

fn main() {
    let _ = jpeg_tools::packaged_tool_contract(TOOL_NAME);
    let _ = libjpeg_abi::common_exports::jpeg_std_error as *const ();
    generated::djpeg::main();
}
