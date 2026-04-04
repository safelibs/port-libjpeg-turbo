use super::ImageFormatContract;

pub const FORMAT_NAME: &str = "GIF";
pub const SOURCE_FILE: &str = "rdgif.c";
pub const RELEVANT_CVE: &str = "CVE-2021-20205";
pub const EXTENSIONS: &[&str] = &["gif"];
pub const CONTRACT: ImageFormatContract = ImageFormatContract {
    format_name: FORMAT_NAME,
    source_file: SOURCE_FILE,
    extensions: EXTENSIONS,
    relevant_cve: Some(RELEVANT_CVE),
};
