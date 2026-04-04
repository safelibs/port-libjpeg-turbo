#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolSourceContract {
    pub tool_name: &'static str,
    pub source_files: &'static [&'static str],
    pub supports_scan_limits: bool,
    pub supports_strict_mode: bool,
}

pub const TOOL_NAMES: &[&str] = &["cjpeg", "djpeg", "jpegtran"];

pub const SOURCE_FILES: &[&str] = &["cdjpeg.c", "cjpeg.c", "djpeg.c", "jpegtran.c"];
pub const TOOL_CONTRACTS: &[ToolSourceContract] = &[
    ToolSourceContract {
        tool_name: "cjpeg",
        source_files: &["cdjpeg.c", "cjpeg.c"],
        supports_scan_limits: false,
        supports_strict_mode: true,
    },
    ToolSourceContract {
        tool_name: "djpeg",
        source_files: &["cdjpeg.c", "djpeg.c"],
        supports_scan_limits: true,
        supports_strict_mode: true,
    },
    ToolSourceContract {
        tool_name: "jpegtran",
        source_files: &["cdjpeg.c", "jpegtran.c"],
        supports_scan_limits: true,
        supports_strict_mode: true,
    },
];

pub fn supports_scan_limits(tool: &str) -> bool {
    matches!(tool, "djpeg" | "jpegtran")
}

pub fn supports_strict_mode(tool: &str) -> bool {
    matches!(tool, "cjpeg" | "djpeg" | "jpegtran")
}
