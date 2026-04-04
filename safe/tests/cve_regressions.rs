#![allow(clippy::all)]

use std::{
    fs,
    mem::MaybeUninit,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::OnceLock,
};

use ffi_types::{jpeg_decompress_struct, TRUE};

struct StagePaths {
    original_testimages: PathBuf,
    stage_bin: PathBuf,
    stage_lib: PathBuf,
}

static STAGE_PATHS: OnceLock<Result<StagePaths, String>> = OnceLock::new();

#[test]
fn skip_scanlines_rejects_two_pass_quantization() {
    let stage = stage_paths().expect("stage paths");
    let temp_dir = new_temp_dir("skip-quantization").expect("temp dir");
    let output = run_stage_command(
        stage,
        &temp_dir,
        "djpeg",
        vec![
            "-colors".into(),
            "256".into(),
            "-skip".into(),
            "1,6".into(),
            "-ppm".into(),
            "-outfile".into(),
            temp_dir.join("quantized_skip.ppm").into_os_string(),
            stage
                .original_testimages
                .join("testorig.jpg")
                .into_os_string(),
        ],
    )
    .expect("spawn djpeg");

    assert!(!output.status.success(), "{:?}", output.status);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Requested features are incompatible")
            || stderr.contains("Requested feature was omitted at compile time"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn skip_scanlines_handles_merged_upsampling_regression_path() {
    let stage = stage_paths().expect("stage paths");
    let temp_dir = new_temp_dir("skip-merged").expect("temp dir");
    let output_path = temp_dir.join("skip_ari.ppm");
    let output = run_stage_command(
        stage,
        &temp_dir,
        "djpeg",
        vec![
            "-dct".into(),
            "int".into(),
            "-skip".into(),
            "16,139".into(),
            "-ppm".into(),
            "-outfile".into(),
            output_path.clone().into_os_string(),
            stage
                .original_testimages
                .join("testimgari.jpg")
                .into_os_string(),
        ],
    )
    .expect("spawn djpeg");

    assert!(
        output.status.success(),
        "{}",
        command_failure("djpeg", &output)
    );
    assert_eq!(
        md5_file(&output_path).expect("md5"),
        "087c6b123db16ac00cb88c5b590bb74a"
    );
}

#[test]
fn progressive_dimensions_are_rejected_before_allocation() {
    let stage = stage_paths().expect("stage paths");
    let temp_dir = new_temp_dir("progressive-dimensions").expect("temp dir");
    let malformed = build_oversized_progressive_fixture(stage, &temp_dir).expect("fixture");
    let output = run_stage_command(
        stage,
        &temp_dir,
        "djpeg",
        vec![
            "-ppm".into(),
            "-outfile".into(),
            temp_dir.join("oversized.ppm").into_os_string(),
            malformed.into_os_string(),
        ],
    )
    .expect("spawn djpeg");

    assert!(!output.status.success(), "{:?}", output.status);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Maximum supported image dimension is 65500 pixels"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn scan_limit_policy_flags_excessive_scan_counts() {
    let mut cinfo = unsafe { MaybeUninit::<jpeg_decompress_struct>::zeroed().assume_init() };
    unsafe {
        libjpeg_abi::decompress_exports::jpeg_rs_configure_decompress_policy(&mut cinfo, 5, TRUE);
    }
    cinfo.input_scan_number = 6;

    let policy = unsafe { jpeg_core::common::registry::get_decompress_policy(&mut cinfo) }
        .expect("policy must exist");
    assert_eq!(policy.max_scans, Some(5));
    assert!(policy.warnings_fatal);
    assert_eq!(
        unsafe { libjpeg_abi::decompress_exports::jpeg_rs_get_max_scans(&mut cinfo) },
        5
    );
    assert_eq!(
        unsafe { libjpeg_abi::decompress_exports::jpeg_rs_get_warnings_fatal(&mut cinfo) },
        TRUE
    );
    assert_eq!(
        unsafe { jpeg_core::common::registry::decompress_scan_limit_exceeded(&mut cinfo) },
        Some(5)
    );

    unsafe {
        jpeg_core::common::registry::clear_decompress_policy(&mut cinfo);
    }
    assert!(unsafe { jpeg_core::common::registry::get_decompress_policy(&mut cinfo) }.is_none());
}

#[test]
fn progressive_scan_limit_rejects_excessive_scan_script() {
    let stage = stage_paths().expect("stage paths");
    let temp_dir = new_temp_dir("scan-limit").expect("temp dir");
    let malformed = build_multiscan_progressive_fixture(stage, &temp_dir).expect("fixture");
    let output = run_stage_command(
        stage,
        &temp_dir,
        "djpeg",
        vec![
            "-maxscans".into(),
            "5".into(),
            "-ppm".into(),
            "-outfile".into(),
            temp_dir.join("scan_limit.ppm").into_os_string(),
            malformed.into_os_string(),
        ],
    )
    .expect("spawn djpeg");

    assert!(!output.status.success(), "{:?}", output.status);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Scan number 6 exceeds maximum scans (5)")
            || stderr.contains("JPEG image has more than 5 scans"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn strict_mode_promotes_truncation_warning_to_fatal() {
    let stage = stage_paths().expect("stage paths");
    let temp_dir = new_temp_dir("strict-truncation").expect("temp dir");
    let truncated = build_truncated_fixture(stage, &temp_dir).expect("fixture");
    let permissive_output = temp_dir.join("permissive.ppm");
    let permissive = run_stage_command(
        stage,
        &temp_dir,
        "djpeg",
        vec![
            "-ppm".into(),
            "-outfile".into(),
            permissive_output.clone().into_os_string(),
            truncated.clone().into_os_string(),
        ],
    )
    .expect("spawn permissive djpeg");
    assert!(
        permissive.status.code() == Some(2),
        "expected warning exit status 2, got {}",
        command_failure("djpeg", &permissive)
    );
    let permissive_len = fs::metadata(&permissive_output)
        .expect("permissive output metadata")
        .len();

    let strict_output = temp_dir.join("strict.ppm");
    let strict = run_stage_command(
        stage,
        &temp_dir,
        "djpeg",
        vec![
            "-strict".into(),
            "-ppm".into(),
            "-outfile".into(),
            strict_output.clone().into_os_string(),
            truncated.into_os_string(),
        ],
    )
    .expect("spawn strict djpeg");
    assert!(
        strict.status.code() == Some(1),
        "expected fatal exit status 1, got {}",
        command_failure("djpeg", &strict)
    );
    let strict_len = fs::metadata(&strict_output)
        .expect("strict output metadata")
        .len();
    assert!(
        strict_len < permissive_len,
        "strict output {strict_len} should be shorter than permissive output {permissive_len}"
    );
}

fn stage_paths() -> Result<&'static StagePaths, String> {
    STAGE_PATHS
        .get_or_init(|| {
            let safe_root = safe::safe_root().to_path_buf();
            let repo_root = safe::repo_root().to_path_buf();
            let status = Command::new("bash")
                .arg("scripts/stage-install.sh")
                .current_dir(&safe_root)
                .status()
                .map_err(|error| format!("failed to run stage-install.sh: {error}"))?;
            if !status.success() {
                return Err(format!("stage-install.sh exited with status {status}"));
            }

            let stage_bin = safe::stage_usr_root().join("bin");
            let stage_lib = find_stage_libdir()?;
            Ok(StagePaths {
                original_testimages: repo_root.join("original/testimages"),
                stage_bin,
                stage_lib,
            })
        })
        .as_ref()
        .map_err(Clone::clone)
}

fn find_stage_libdir() -> Result<PathBuf, String> {
    let lib_root = safe::stage_usr_root().join("lib");
    for entry in fs::read_dir(&lib_root)
        .map_err(|error| format!("read_dir {}: {error}", lib_root.display()))?
    {
        let entry =
            entry.map_err(|error| format!("read_dir entry {}: {error}", lib_root.display()))?;
        let path = entry.path();
        if path.is_dir() && path.join("libjpeg.so.8").exists() {
            return Ok(path);
        }
    }
    Err(format!(
        "could not find staged libjpeg under {}",
        lib_root.display()
    ))
}

fn run_stage_command(
    stage: &StagePaths,
    temp_dir: &Path,
    tool: &str,
    args: Vec<std::ffi::OsString>,
) -> Result<Output, String> {
    Command::new(stage.stage_bin.join(tool))
        .env("LD_LIBRARY_PATH", &stage.stage_lib)
        .current_dir(temp_dir)
        .args(args)
        .output()
        .map_err(|error| format!("failed to spawn {tool}: {error}"))
}

fn command_failure(tool: &str, output: &Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    format!(
        "{tool} exited with status {}\nstdout:\n{}\nstderr:\n{}",
        output.status, stdout, stderr
    )
}

fn new_temp_dir(name: &str) -> Result<PathBuf, String> {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "libjpeg-cve-regressions-{}-{name}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)
            .map_err(|error| format!("remove_dir_all {}: {error}", path.display()))?;
    }
    fs::create_dir_all(&path)
        .map_err(|error| format!("create_dir_all {}: {error}", path.display()))?;
    Ok(path)
}

fn md5_file(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    Ok(format!("{:x}", md5::compute(bytes)))
}

fn build_oversized_progressive_fixture(
    stage: &StagePaths,
    temp_dir: &Path,
) -> Result<PathBuf, String> {
    let base = temp_dir.join("base_progressive.jpg");
    let output = run_stage_command(
        stage,
        temp_dir,
        "cjpeg",
        vec![
            "-prog".into(),
            "-outfile".into(),
            base.clone().into_os_string(),
            stage
                .original_testimages
                .join("testorig.ppm")
                .into_os_string(),
        ],
    )?;
    if !output.status.success() {
        return Err(command_failure("cjpeg", &output));
    }

    let mut bytes = fs::read(&base).map_err(|error| format!("read {}: {error}", base.display()))?;
    let sof = find_sof_segment(&bytes, 0xC2)?;
    if sof + 9 > bytes.len() {
        return Err("SOF segment is truncated".to_string());
    }
    bytes[sof + 5] = 0xFF;
    bytes[sof + 6] = 0xFF;
    bytes[sof + 7] = 0xFF;
    bytes[sof + 8] = 0xFF;

    let malformed = temp_dir.join("oversized_progressive.jpg");
    fs::write(&malformed, bytes)
        .map_err(|error| format!("write {}: {error}", malformed.display()))?;
    Ok(malformed)
}

fn build_multiscan_progressive_fixture(
    stage: &StagePaths,
    temp_dir: &Path,
) -> Result<PathBuf, String> {
    let output_path = temp_dir.join("multiscan_progressive.jpg");
    let output = run_stage_command(
        stage,
        temp_dir,
        "cjpeg",
        vec![
            "-quality".into(),
            "100".into(),
            "-dct".into(),
            "fast".into(),
            "-scans".into(),
            stage.original_testimages.join("test.scan").into_os_string(),
            "-outfile".into(),
            output_path.clone().into_os_string(),
            stage
                .original_testimages
                .join("testorig.ppm")
                .into_os_string(),
        ],
    )?;
    if !output.status.success() {
        return Err(command_failure("cjpeg", &output));
    }
    Ok(output_path)
}

fn build_truncated_fixture(stage: &StagePaths, temp_dir: &Path) -> Result<PathBuf, String> {
    let input = stage.original_testimages.join("testorig.jpg");
    let mut bytes =
        fs::read(&input).map_err(|error| format!("read {}: {error}", input.display()))?;
    if bytes.len() < 4 {
        return Err(format!("fixture {} is unexpectedly short", input.display()));
    }
    bytes.truncate(bytes.len() - 2);

    let truncated = temp_dir.join("truncated.jpg");
    fs::write(&truncated, bytes)
        .map_err(|error| format!("write {}: {error}", truncated.display()))?;
    Ok(truncated)
}

fn find_sof_segment(bytes: &[u8], marker: u8) -> Result<usize, String> {
    if bytes.len() < 4 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return Err("not a JPEG file".to_string());
    }

    let mut index = 2usize;
    while index + 4 <= bytes.len() {
        if bytes[index] != 0xFF {
            return Err(format!("expected marker at offset {index}"));
        }
        while index < bytes.len() && bytes[index] == 0xFF {
            index += 1;
        }
        if index >= bytes.len() {
            break;
        }

        let current = bytes[index];
        index += 1;
        if current == marker {
            return Ok(index - 2);
        }
        if current == 0xD9 || current == 0xDA {
            break;
        }
        if index + 2 > bytes.len() {
            break;
        }
        let length = u16::from_be_bytes([bytes[index], bytes[index + 1]]) as usize;
        if length < 2 || index + length > bytes.len() {
            break;
        }
        index += length;
    }

    Err(format!("marker 0x{marker:02x} not found"))
}
