#[allow(warnings, clippy::all)]
mod generated {
    #[path = "../../generated/rdjpgcom.rs"]
    pub mod rdjpgcom;
}

const TOOL_NAME: &str = "rdjpgcom";

fn main() {
    let _ = jpeg_tools::packaged_tool_contract(TOOL_NAME);
    generated::rdjpgcom::main();
}
