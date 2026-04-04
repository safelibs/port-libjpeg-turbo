use super::ImageFormatContract;

pub const FORMAT_NAME: &str = "BMP";
pub const SOURCE_FILE: &str = "wrbmp.c";
pub const EXTENSIONS: &[&str] = &["bmp"];
pub const CONTRACT: ImageFormatContract = ImageFormatContract {
    format_name: FORMAT_NAME,
    source_file: SOURCE_FILE,
    extensions: EXTENSIONS,
    relevant_cve: None,
};
