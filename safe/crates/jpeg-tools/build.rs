fn main() {
    for path in [
        "Cargo.toml",
        "src/lib.rs",
        "src/bin/cjpeg.rs",
        "src/bin/djpeg.rs",
        "src/bin/jpegexiforient.rs",
        "src/bin/jpegtran.rs",
        "src/bin/rdjpgcom.rs",
        "src/bin/tjbench.rs",
        "src/bin/tjexample.rs",
        "src/bin/wrjpgcom.rs",
        "../../scripts/stage-install.sh",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}
