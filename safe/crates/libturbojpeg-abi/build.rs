fn main() {
    for path in [
        "Cargo.toml",
        "src/lib.rs",
        "src/generated",
        "../../scripts/stage-install.sh",
        "../../link/turbojpeg-mapfile.jni",
        "../../debian/libturbojpeg.symbols",
        "../../java",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}
