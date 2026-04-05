pub const COLOR_QUANTIZATION_SWITCHES: &[&str] = &["-colors", "-onepass", "-twopass", "-map"];

pub const SOURCE_FILE: &str = "rdcolmap.c";

pub fn is_color_quantization_switch(flag: &str) -> bool {
    COLOR_QUANTIZATION_SWITCHES.contains(&flag)
}
