pub const STRICT_FLAG: &str = "-strict";
pub const MAXSCANS_FLAG: &str = "-maxscans";
pub const LIMITSCANS_FLAG: &str = "-limitscans";

pub const READER_SWITCHES: &[&str] = &[STRICT_FLAG, MAXSCANS_FLAG, LIMITSCANS_FLAG];

pub fn is_reader_switch(flag: &str) -> bool {
    READER_SWITCHES.contains(&flag)
}
