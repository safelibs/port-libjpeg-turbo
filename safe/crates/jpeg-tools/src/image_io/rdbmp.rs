use super::ImageFormatContract;

pub const FORMAT_NAME: &str = "BMP";
pub const SOURCE_FILE: &str = "rdbmp.c";
pub const RELEVANT_CVE: &str = "CVE-2018-1152";
pub const EXTENSIONS: &[&str] = &["bmp"];
pub const CONTRACT: ImageFormatContract = ImageFormatContract {
    format_name: FORMAT_NAME,
    source_file: SOURCE_FILE,
    extensions: EXTENSIONS,
    relevant_cve: Some(RELEVANT_CVE),
};
