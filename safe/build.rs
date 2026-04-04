fn main() {
    if let Some(libdir) = staged_libdir() {
        println!("cargo:rustc-link-search=native={}", libdir.display());
        println!("cargo:rustc-link-arg-tests=-Wl,-rpath,{}", libdir.display());
    }
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
        "scripts/audit-unsafe.sh",
        "scripts/run-bench-smoke.sh",
        "README.md",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}

fn staged_libdir() -> Option<std::path::PathBuf> {
    let safe_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let multiarch = multiarch()?;
    let libdir = safe_root.join("stage/usr/lib").join(multiarch);
    if libdir.exists() {
        Some(libdir)
    } else {
        None
    }
}

fn multiarch() -> Option<String> {
    for (program, args) in [
        ("dpkg-architecture", &["-qDEB_HOST_MULTIARCH"][..]),
        ("gcc", &["-print-multiarch"][..]),
    ] {
        if let Ok(output) = std::process::Command::new(program).args(args).output() {
            if output.status.success() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
                if !value.is_empty() {
                    return Some(value);
                }
            }
        }
    }
    None
}
