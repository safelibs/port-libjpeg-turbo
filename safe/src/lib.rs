use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub const UPSTREAM_VERSION: &str = "2.1.5";
pub const UBUNTU_DEBIAN_VERSION: &str = "2.1.5-2ubuntu2";
pub const LIBJPEG_SONAME: &str = "libjpeg.so.8";
pub const LIBTURBOJPEG_SONAME: &str = "libturbojpeg.so.0";
pub const MULTIARCH_TRIPLE_ENV: &str = "DEB_HOST_MULTIARCH";
pub const STAGE_ROOT_ENV: &str = "LIBJPEG_TURBO_STAGE_ROOT";

pub fn safe_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn repo_root() -> &'static Path {
    safe_root().parent().expect("safe root has parent")
}

pub fn stage_root() -> PathBuf {
    env::var_os(STAGE_ROOT_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|| safe_root().join("stage"))
}

pub fn stage_usr_root() -> PathBuf {
    stage_root().join("usr")
}

pub fn multiarch() -> Option<String> {
    if let Ok(value) = env::var(MULTIARCH_TRIPLE_ENV) {
        let value = value.trim().to_owned();
        if !value.is_empty() {
            return Some(value);
        }
    }

    probe_multiarch("dpkg-architecture", &["-qDEB_HOST_MULTIARCH"])
        .or_else(|| probe_multiarch("gcc", &["-print-multiarch"]))
}

pub fn stage_lib_root() -> PathBuf {
    stage_usr_root().join("lib")
}

pub fn stage_libdir() -> Option<PathBuf> {
    Some(stage_lib_root().join(multiarch()?))
}

pub fn stage_multiarch_include_dir() -> Option<PathBuf> {
    Some(stage_usr_root().join("include").join(multiarch()?))
}

fn probe_multiarch(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}
