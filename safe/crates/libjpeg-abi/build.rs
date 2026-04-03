fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let manifest_dir = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let safe_root = manifest_dir.join("../..").canonicalize().unwrap();
    let multiarch = multiarch();
    let stage_include = safe_root.join("stage/usr/include");
    let stage_multiarch = stage_include.join(&multiarch);
    let bootstrap_build = safe_root.join("target/upstream-bootstrap");
    let original_root = safe_root.join("../original").canonicalize().unwrap();
    let shim_source = safe_root.join("c_shim/error_bridge.c");
    let shim_object = out_dir.join("error_bridge.o");
    let jdapimin_object = out_dir.join("jdapimin_orig.o");
    let jdapistd_object = out_dir.join("jdapistd_orig.o");
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
    compile_original_shim(
        &stage_include,
        &stage_multiarch,
        &bootstrap_build,
        &original_root,
        &original_root.join("jdapimin.c"),
        &jdapimin_object,
        &[
            "jpeg_CreateDecompress=jpeg_orig_CreateDecompress",
            "jpeg_destroy_decompress=jpeg_orig_destroy_decompress",
            "jpeg_abort_decompress=jpeg_orig_abort_decompress",
            "jpeg_read_header=jpeg_orig_read_header",
            "jpeg_consume_input=jpeg_orig_consume_input",
            "jpeg_input_complete=jpeg_orig_input_complete",
            "jpeg_has_multiple_scans=jpeg_orig_has_multiple_scans",
            "jpeg_finish_decompress=jpeg_orig_finish_decompress",
        ],
    );
    compile_original_shim(
        &stage_include,
        &stage_multiarch,
        &bootstrap_build,
        &original_root,
        &original_root.join("jdapistd.c"),
        &jdapistd_object,
        &[
            "jpeg_start_decompress=jpeg_orig_start_decompress",
            "jpeg_crop_scanline=jpeg_orig_crop_scanline",
            "jpeg_read_scanlines=jpeg_orig_read_scanlines",
            "jpeg_skip_scanlines=jpeg_orig_skip_scanlines",
            "jpeg_read_raw_data=jpeg_orig_read_raw_data",
            "jpeg_start_output=jpeg_orig_start_output",
            "jpeg_finish_output=jpeg_orig_finish_output",
        ],
    );
    run(
        std::process::Command::new("ar")
            .arg("crus")
            .arg(&shim_archive)
            .arg(&shim_object),
    );
    run(
        std::process::Command::new("ar")
            .arg("r")
            .arg(&shim_archive)
            .arg(&jdapimin_object)
            .arg(&jdapistd_object),
    );

    for path in [
        "../../scripts/stage-install.sh",
        "../../scripts/check-symbols.sh",
        "../../c_shim/error_bridge.c",
        "../../../original/jdapimin.c",
        "../../../original/jdapistd.c",
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

fn compile_original_shim(
    stage_include: &std::path::Path,
    stage_multiarch: &std::path::Path,
    bootstrap_build: &std::path::Path,
    original_root: &std::path::Path,
    source: &std::path::Path,
    output: &std::path::Path,
    renames: &[&str],
) {
    let mut command = std::process::Command::new("gcc");
    command
        .arg("-std=c99")
        .arg("-O2")
        .arg("-fPIC")
        .arg("-I")
        .arg(stage_include)
        .arg("-I")
        .arg(stage_multiarch)
        .arg("-I")
        .arg(bootstrap_build)
        .arg("-I")
        .arg(original_root)
        .arg("-c");
    for rename in renames {
        command.arg(format!("-D{rename}"));
    }
    command.arg(source).arg("-o").arg(output);
    run(&mut command);
}
