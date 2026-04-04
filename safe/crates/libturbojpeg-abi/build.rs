fn main() {
    for path in [
        "Cargo.toml",
        "src/lib.rs",
        "src/generated",
        "../../scripts/stage-install.sh",
        "../../scripts/debian_symbols.py",
        "../../link/turbojpeg-mapfile.jni",
        "../../debian/libturbojpeg.symbols",
        "../../java",
        "../../../original/turbojpeg-jni.c",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}
