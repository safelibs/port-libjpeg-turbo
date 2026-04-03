pub const TOOL_NAMES: &[&str] = &["cjpeg", "djpeg", "jpegtran"];

pub const SOURCE_FILES: &[&str] = &["cdjpeg.c", "cjpeg.c", "djpeg.c", "jpegtran.c"];

pub fn supports_scan_limits(tool: &str) -> bool {
    matches!(tool, "djpeg" | "jpegtran")
}

pub fn supports_strict_mode(tool: &str) -> bool {
    matches!(tool, "cjpeg" | "djpeg" | "jpegtran")
}
