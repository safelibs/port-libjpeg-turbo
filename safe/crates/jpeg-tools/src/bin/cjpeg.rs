#[allow(warnings, clippy::all)]
mod generated {
    #[path = "../../generated/cdjpeg.rs"]
    pub mod cdjpeg;
    #[path = "../../generated/cjpeg.rs"]
    pub mod cjpeg;
    #[path = "../../generated/rdbmp.rs"]
    pub mod rdbmp;
    #[path = "../../generated/rdgif.rs"]
    pub mod rdgif;
    #[path = "../../generated/rdppm.rs"]
    pub mod rdppm;
    #[path = "../../generated/rdswitch.rs"]
    pub mod rdswitch;
    #[path = "../../generated/rdtarga.rs"]
    pub mod rdtarga;
}

const TOOL_NAME: &str = "cjpeg";

fn main() {
    let _ = jpeg_tools::packaged_tool_contract(TOOL_NAME);
    let _ = libjpeg_abi::common_exports::jpeg_std_error as *const ();
    generated::cjpeg::main();
}
