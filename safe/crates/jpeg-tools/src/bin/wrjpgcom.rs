#[allow(warnings, clippy::all)]
mod generated {
    #[path = "../../generated/wrjpgcom.rs"]
    pub mod wrjpgcom;
}

const TOOL_NAME: &str = "wrjpgcom";

fn main() {
    let _ = jpeg_tools::packaged_tool_contract(TOOL_NAME);
    generated::wrjpgcom::main();
}
