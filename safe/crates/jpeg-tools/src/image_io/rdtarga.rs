use super::ImageFormatContract;

pub const FORMAT_NAME: &str = "Targa";
pub const SOURCE_FILE: &str = "rdtarga.c";
pub const RELEVANT_CVE: &str = "CVE-2018-11813";
pub const EXTENSIONS: &[&str] = &["tga"];
pub const CONTRACT: ImageFormatContract = ImageFormatContract {
    format_name: FORMAT_NAME,
    source_file: SOURCE_FILE,
    extensions: EXTENSIONS,
    relevant_cve: Some(RELEVANT_CVE),
};
