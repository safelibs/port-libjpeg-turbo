fn main() {
    for path in [
        "Cargo.toml",
        "debian",
        "include/install-manifest.txt",
        "link/libjpeg.map",
        "link/turbojpeg-mapfile",
        "link/turbojpeg-mapfile.jni",
        "pkgconfig/libjpeg.pc.in",
        "pkgconfig/libturbojpeg.pc.in",
        "cmake/libjpeg-turboConfig.cmake.in",
        "cmake/libjpeg-turboConfigVersion.cmake.in",
        "cmake/libjpeg-turboTargets.cmake.in",
        "scripts/stage-install.sh",
        "scripts/check-symbols.sh",
        "scripts/relink-original-objects.sh",
        "scripts/original-object-groups.json",
        "scripts/run-dependent-subset.sh",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}

