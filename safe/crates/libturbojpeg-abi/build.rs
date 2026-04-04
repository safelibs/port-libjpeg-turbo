fn main() {
    for path in [
        "Cargo.toml",
        "src/lib.rs",
        "../jpeg-core/src/ported/turbojpeg/turbojpeg.rs",
        "../../scripts/stage-install.sh",
        "../../link/turbojpeg-mapfile.jni",
        "../../debian/libturbojpeg.symbols",
        "../../java",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}
