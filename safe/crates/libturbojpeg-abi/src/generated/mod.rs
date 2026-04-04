#![allow(warnings)]
#![allow(clippy::all)]

// The committed JNI bindings mirror the upstream turbojpeg-jni.c surface so the
// staged and packaged libturbojpeg SONAME can retain the canonical Java export
// names from Debian's libturbojpeg.symbols manifest.
pub mod jdatadst_tj;
pub mod jdatasrc_tj;
pub mod rdbmp;
pub mod rdppm;
pub mod turbojpeg;
pub mod turbojpeg_jni;
pub mod wrbmp;
pub mod wrppm;
