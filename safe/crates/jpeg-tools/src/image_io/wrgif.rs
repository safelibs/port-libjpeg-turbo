use super::ImageFormatContract;

pub const FORMAT_NAME: &str = "GIF";
pub const SOURCE_FILE: &str = "wrgif.c";
pub const EXTENSIONS: &[&str] = &["gif"];
pub const CONTRACT: ImageFormatContract = ImageFormatContract {
    format_name: FORMAT_NAME,
    source_file: SOURCE_FILE,
    extensions: EXTENSIONS,
    relevant_cve: None,
};
