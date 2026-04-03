fn main() {
    for path in [
        "../../scripts/stage-install.sh",
        "../../scripts/check-symbols.sh",
        "../../../original/CMakeLists.txt",
        "../../../original/turbojpeg-mapfile",
        "../../../original/turbojpeg-mapfile.jni",
        "../../../original/debian/libturbojpeg.symbols",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}

