fn main() {
    for path in [
        "../../scripts/stage-install.sh",
        "../../scripts/check-symbols.sh",
        "../../../original/CMakeLists.txt",
        "../../../original/sharedlib/CMakeLists.txt",
        "../../../original/libjpeg.map.in",
        "../../../original/debian/libjpeg-turbo8.symbols",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}

