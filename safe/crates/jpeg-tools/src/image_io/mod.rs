pub mod rdbmp;
pub mod rdgif;
pub mod rdppm;
pub mod rdtarga;
pub mod wrbmp;
pub mod wrgif;
pub mod wrppm;
pub mod wrtarga;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageFormatContract {
    pub format_name: &'static str,
    pub source_file: &'static str,
    pub extensions: &'static [&'static str],
    pub relevant_cve: Option<&'static str>,
}

impl ImageFormatContract {
    pub fn matches_extension(&self, extension: &str) -> bool {
        self.extensions
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(extension))
    }
}

pub const INPUT_FORMATS: &[ImageFormatContract] = &[
    rdbmp::CONTRACT,
    rdppm::CONTRACT,
    rdgif::CONTRACT,
    rdtarga::CONTRACT,
];

pub const OUTPUT_FORMATS: &[ImageFormatContract] = &[
    wrbmp::CONTRACT,
    wrppm::CONTRACT,
    wrgif::CONTRACT,
    wrtarga::CONTRACT,
];
