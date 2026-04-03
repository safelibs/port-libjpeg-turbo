fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let manifest_dir = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let safe_root = manifest_dir.join("../..").canonicalize().unwrap();
    let multiarch = multiarch();
    let stage_include = safe_root.join("stage/usr/include");
    let stage_multiarch = stage_include.join(&multiarch);
    let shim_source = safe_root.join("c_shim/error_bridge.c");
    let shim_object = out_dir.join("error_bridge.o");
    let shim_archive = out_dir.join("libjpeg_compat_shims.a");

    run(
        std::process::Command::new("gcc")
            .arg("-std=c99")
            .arg("-O2")
            .arg("-fPIC")
            .arg("-I")
            .arg(&stage_include)
            .arg("-I")
            .arg(&stage_multiarch)
            .arg("-c")
            .arg(&shim_source)
            .arg("-o")
            .arg(&shim_object),
    );
    run(
        std::process::Command::new("ar")
            .arg("crus")
            .arg(&shim_archive)
            .arg(&shim_object),
    );

    for path in [
        "../../scripts/stage-install.sh",
        "../../scripts/check-symbols.sh",
        "../../c_shim/error_bridge.c",
        "../../../original/CMakeLists.txt",
        "../../../original/sharedlib/CMakeLists.txt",
        "../../../original/libjpeg.map.in",
        "../../../original/debian/libjpeg-turbo8.symbols",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=jpeg_compat_shims");
}

fn multiarch() -> String {
    for (program, args) in [
        ("dpkg-architecture", &["-qDEB_HOST_MULTIARCH"][..]),
        ("gcc", &["-print-multiarch"][..]),
    ] {
        let output = std::process::Command::new(program).args(args).output();
        if let Ok(output) = output {
            if output.status.success() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
                if !value.is_empty() {
                    return value;
                }
            }
        }
    }
    format!("{}-linux-gnu", std::env::consts::ARCH)
}

fn run(command: &mut std::process::Command) {
    let status = command.status().expect("failed to run build helper");
    if !status.success() {
        panic!("build helper exited with status {status}");
    }
}
