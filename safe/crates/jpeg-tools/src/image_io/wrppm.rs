use super::ImageFormatContract;

pub const FORMAT_NAME: &str = "PPM/PGM";
pub const SOURCE_FILE: &str = "wrppm.c";
pub const EXTENSIONS: &[&str] = &["pbm", "pgm", "ppm"];
pub const CONTRACT: ImageFormatContract = ImageFormatContract {
    format_name: FORMAT_NAME,
    source_file: SOURCE_FILE,
    extensions: EXTENSIONS,
    relevant_cve: None,
};
