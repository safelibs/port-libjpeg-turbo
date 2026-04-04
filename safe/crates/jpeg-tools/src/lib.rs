pub mod cdjpeg;
pub mod image_io;
pub mod rdcolmap;
pub mod rdswitch;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PackagedToolContract {
    pub binary_name: &'static str,
    pub frontend_source: &'static str,
    pub manpage_source: &'static str,
}

pub const PACKAGED_TOOL_NAMES: &[&str] = &[
    "cjpeg",
    "djpeg",
    "jpegtran",
    "rdjpgcom",
    "wrjpgcom",
    "tjbench",
    "jpegexiforient",
    "exifautotran",
];
pub const TURBOJPEG_DEV_TOOL_NAMES: &[&str] = &["tjexample"];

pub const MANPAGE_NAMES: &[&str] = &[
    "cjpeg.1",
    "djpeg.1",
    "jpegtran.1",
    "rdjpgcom.1",
    "wrjpgcom.1",
    "tjbench.1",
    "jpegexiforient.1",
    "exifautotran.1",
];

pub const PACKAGED_TOOL_CONTRACTS: &[PackagedToolContract] = &[
    PackagedToolContract {
        binary_name: "cjpeg",
        frontend_source: "safe/crates/jpeg-tools/src/bin/cjpeg.rs",
        manpage_source: "safe/debian/cjpeg.1",
    },
    PackagedToolContract {
        binary_name: "djpeg",
        frontend_source: "safe/crates/jpeg-tools/src/bin/djpeg.rs",
        manpage_source: "safe/debian/djpeg.1",
    },
    PackagedToolContract {
        binary_name: "jpegtran",
        frontend_source: "safe/crates/jpeg-tools/src/bin/jpegtran.rs",
        manpage_source: "safe/debian/jpegtran.1",
    },
    PackagedToolContract {
        binary_name: "rdjpgcom",
        frontend_source: "safe/crates/jpeg-tools/src/bin/rdjpgcom.rs",
        manpage_source: "safe/debian/rdjpgcom.1",
    },
    PackagedToolContract {
        binary_name: "wrjpgcom",
        frontend_source: "safe/crates/jpeg-tools/src/bin/wrjpgcom.rs",
        manpage_source: "safe/debian/wrjpgcom.1",
    },
    PackagedToolContract {
        binary_name: "tjbench",
        frontend_source: "safe/crates/jpeg-tools/src/bin/tjbench.rs",
        manpage_source: "safe/debian/tjbench.1.in",
    },
    PackagedToolContract {
        binary_name: "jpegexiforient",
        frontend_source: "safe/crates/jpeg-tools/src/bin/jpegexiforient.rs",
        manpage_source: "safe/debian/extra/jpegexiforient.1",
    },
    PackagedToolContract {
        binary_name: "exifautotran",
        frontend_source: "safe/debian/extra/exifautotran",
        manpage_source: "safe/debian/extra/exifautotran.1",
    },
];

pub fn packaged_tool_contract(binary_name: &str) -> &'static PackagedToolContract {
    PACKAGED_TOOL_CONTRACTS
        .iter()
        .find(|contract| contract.binary_name == binary_name)
        .unwrap_or_else(|| panic!("missing packaged tool contract for {binary_name}"))
}

pub fn staged_tool_only(tool: &str) -> ! {
    eprintln!(
        "{tool}: the packaged frontend is built during safe/scripts/stage-install.sh; use the staged binary under safe/stage/usr/bin/{tool}"
    );
    std::process::exit(1);
}
