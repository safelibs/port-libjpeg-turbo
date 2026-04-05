fn main() {
    println!("cargo:rerun-if-env-changed=DEB_HOST_MULTIARCH");
    println!("cargo:rerun-if-env-changed=LIBJPEG_TURBO_STAGE_ROOT");
    println!("cargo:rerun-if-changed={}", stage_root().display());
    if let Some(libdir) = staged_libdir() {
        println!("cargo:rustc-link-search=native={}", libdir.display());
        println!("cargo:rustc-link-arg-tests=-Wl,-rpath,{}", libdir.display());
    }
    for path in [
        "Cargo.toml",
        "debian",
        "c_shim/error_bridge.c",
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
        "scripts/run-dependent-regressions.sh",
        "scripts/audit-unsafe.sh",
        "scripts/run-bench-smoke.sh",
        "README.md",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}

fn stage_root() -> std::path::PathBuf {
    std::env::var_os("LIBJPEG_TURBO_STAGE_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("stage"))
}

fn staged_libdir() -> Option<std::path::PathBuf> {
    let stage_root = stage_root();
    let multiarch = multiarch()?;
    let libdir = stage_root.join("usr/lib").join(multiarch);
    if libdir.exists() {
        Some(libdir)
    } else {
        None
    }
}

fn multiarch() -> Option<String> {
    if let Ok(value) = std::env::var("DEB_HOST_MULTIARCH") {
        let value = value.trim().to_owned();
        if !value.is_empty() {
            return Some(value);
        }
    }

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
