fn main() {
    for path in [
        "Cargo.toml",
        "src/lib.rs",
        "../jpeg-core/src/ported/turbojpeg/turbojpeg.rs",
        "../../scripts/stage-install.sh",
        "../../link/turbojpeg-mapfile.jni",
        "../../debian/libturbojpeg.symbols",
        "../../java",
        "../../../original/turbojpeg-jni.c",
        "../../../original/java/org_libjpegturbo_turbojpeg_TJ.h",
        "../../../original/java/org_libjpegturbo_turbojpeg_TJCompressor.h",
        "../../../original/java/org_libjpegturbo_turbojpeg_TJDecompressor.h",
        "../../../original/java/org_libjpegturbo_turbojpeg_TJTransformer.h",
        "../../../original/turbojpeg.h",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}
