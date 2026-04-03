pub mod common;
pub mod ported;

pub mod jc {
    pub const PREFIX: &str = "jc";
}

pub mod jd {
    pub const PREFIX: &str = "jd";
}

pub mod transupp {
    pub const FILE_BASENAME: &str = "transupp";
}

pub mod turbojpeg {
    pub const FILE_BASENAME: &str = "turbojpeg";
    pub use crate::ported::turbojpeg::turbojpeg::*;
    pub use crate::ported::turbojpeg::tjutil;
}
