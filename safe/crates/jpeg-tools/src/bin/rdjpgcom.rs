#[allow(warnings, clippy::all)]
mod generated {
    #[path = "../../generated/rdjpgcom.rs"]
    pub mod rdjpgcom;
}

fn main() {
    generated::rdjpgcom::main();
}
