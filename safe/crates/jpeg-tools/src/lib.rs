pub mod cdjpeg;
pub mod image_io;
pub mod rdcolmap;
pub mod rdswitch;

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

pub fn staged_tool_only(tool: &str) -> ! {
    eprintln!(
        "{tool}: the packaged frontend is built during safe/scripts/stage-install.sh; use the staged binary under safe/stage/usr/bin/{tool}"
    );
    std::process::exit(1);
}
