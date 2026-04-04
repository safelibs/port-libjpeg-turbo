#[allow(warnings, clippy::all)]
mod generated {
    #[path = "../../generated/cdjpeg.rs"]
    pub mod cdjpeg;
    #[path = "../../generated/jpegtran.rs"]
    pub mod jpegtran;
    #[path = "../../generated/rdswitch.rs"]
    pub mod rdswitch;
    #[path = "../../generated/transupp.rs"]
    pub mod transupp;
}

fn main() {
    let _ = libjpeg_abi::common_exports::jpeg_std_error as *const ();
    generated::jpegtran::main();
}
