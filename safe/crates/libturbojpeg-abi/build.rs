fn main() {
    for path in [
        "Cargo.toml",
        "src/lib.rs",
        "../jpeg-core/src/ported/turbojpeg/turbojpeg.rs",
        "../../scripts/stage-install.sh",
        "../../../original/turbojpeg-mapfile",
        "../../../original/turbojpeg-mapfile.jni",
        "../../../original/debian/libturbojpeg.symbols",
        "../../../original/turbojpeg.h",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}
