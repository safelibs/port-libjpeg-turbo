#[allow(warnings, clippy::all)]
mod generated {
    #[path = "../../generated/tjbench.rs"]
    pub mod tjbench;
}

fn main() {
    let _ = libjpeg_abi::common_exports::jpeg_std_error as *const ();
    let _ = libturbojpeg_abi::non_jni::tjInitCompress as *const ();
    generated::tjbench::main();
}
