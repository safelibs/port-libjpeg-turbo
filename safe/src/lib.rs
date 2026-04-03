use std::path::{Path, PathBuf};

pub const UPSTREAM_VERSION: &str = "2.1.5";
pub const UBUNTU_DEBIAN_VERSION: &str = "2.1.5-2ubuntu2";
pub const LIBJPEG_SONAME: &str = "libjpeg.so.8";
pub const LIBTURBOJPEG_SONAME: &str = "libturbojpeg.so.0";
pub const MULTIARCH_TRIPLE_ENV: &str = "DEB_HOST_MULTIARCH";

pub fn safe_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn stage_root() -> PathBuf {
    safe_root().join("stage")
}

pub fn stage_usr_root() -> PathBuf {
    stage_root().join("usr")
}
