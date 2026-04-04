use std::{
    ffi::OsString,
    fs,
    io::{self, Write},
    path::PathBuf,
};

const TOOL_NAME: &str = "jpegexiforient";

fn main() {
    let _ = jpeg_tools::packaged_tool_contract(TOOL_NAME);
    std::process::exit(run());
}

fn run() -> i32 {
    let mut args = std::env::args_os();
    let program = args
        .next()
        .unwrap_or_else(|| OsString::from("jpegexiforient"));
    let program_name = program.to_string_lossy().into_owned();

    let mut no_newline = false;
    let mut set_orientation = None;
    let mut input_path = None;

    for arg in args {
        let arg_text = arg.to_string_lossy();
        if input_path.is_none() && arg_text.starts_with('-') && arg_text.len() > 1 {
            match arg_text.as_ref() {
                "--help" => {
                    usage(&program_name, io::stdout());
                    return 0;
                }
                "--version" => {
                    println!("jpegexiforient");
                    return 0;
                }
                "-n" => {
                    no_newline = true;
                }
                "-1" | "-2" | "-3" | "-4" | "-5" | "-6" | "-7" | "-8" => {
                    set_orientation = arg_text
                        .as_bytes()
                        .get(1)
                        .copied()
                        .map(|value| value - b'0');
                }
                _ => {
                    usage(&program_name, io::stderr());
                    return 1;
                }
            }
        } else if input_path.is_none() {
            input_path = Some(PathBuf::from(arg));
        } else {
            break;
        }
    }

    let Some(path) = input_path else {
        usage(&program_name, io::stderr());
        return 1;
    };

    let mut bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("{program_name}: can't open {}", path.display());
            return 0;
        }
    };

    let Some(tag) = find_orientation_tag(&bytes) else {
        return 0;
    };

    if let Some(value) = set_orientation {
        overwrite_orientation_entry(&mut bytes, &tag, value);
        let _ = fs::write(&path, &bytes);
        return 0;
    }

    if tag.value > 8 {
        return 0;
    }

    if no_newline {
        print!("{}", tag.value);
    } else {
        println!("{}", tag.value);
    }
    let _ = io::stdout().flush();
    0
}

fn usage(program_name: &str, mut out: impl Write) {
    let _ = writeln!(
        out,
        "jpegexiforient reads or writes the Exif Orientation Tag in a JPEG Exif file."
    );
    let _ = writeln!(
        out,
        "The packaged exifautotran wrapper invokes this helper from the same installed directory."
    );
    let _ = writeln!(out, "Usage: {program_name} [switches] jpegfile");
    let _ = writeln!(out, "Switches:");
    let _ = writeln!(out, "  --help     display this help and exit");
    let _ = writeln!(out, "  --version  output version information and exit");
    let _ = writeln!(out, "  -n         Do not output the trailing newline");
    let _ = writeln!(out, "  -1 .. -8   Set orientation value 1 .. 8");
}

#[derive(Clone, Copy)]
enum Endian {
    Little,
    Big,
}

struct OrientationTag {
    entry_offset: usize,
    endian: Endian,
    value: u8,
}

fn find_orientation_tag(bytes: &[u8]) -> Option<OrientationTag> {
    if bytes.len() < 4 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return None;
    }

    let mut offset = 2usize;
    while offset + 4 <= bytes.len() {
        while offset < bytes.len() && bytes[offset] != 0xFF {
            offset += 1;
        }
        while offset < bytes.len() && bytes[offset] == 0xFF {
            offset += 1;
        }
        if offset >= bytes.len() {
            return None;
        }

        let marker = bytes[offset];
        offset += 1;
        match marker {
            0xD9 | 0xDA => return None,
            0x01 | 0xD0..=0xD7 => continue,
            _ => {}
        }
        if offset + 2 > bytes.len() {
            return None;
        }

        let segment_length = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]) as usize;
        if segment_length < 2 {
            return None;
        }
        let segment_data = offset + 2;
        let segment_end = offset + segment_length;
        if segment_end > bytes.len() {
            return None;
        }

        if marker == 0xE1
            && segment_end >= segment_data + 6
            && &bytes[segment_data..segment_data + 6] == b"Exif\0\0"
        {
            return find_orientation_in_tiff(bytes, segment_data + 6, segment_end);
        }

        offset = segment_end;
    }

    None
}

fn find_orientation_in_tiff(
    bytes: &[u8],
    tiff_start: usize,
    tiff_end: usize,
) -> Option<OrientationTag> {
    if tiff_start + 8 > tiff_end {
        return None;
    }

    let endian = match &bytes[tiff_start..tiff_start + 2] {
        b"II" => Endian::Little,
        b"MM" => Endian::Big,
        _ => return None,
    };
    if read_u16(bytes, tiff_start + 2, endian)? != 0x2A {
        return None;
    }

    let ifd_offset = read_u32(bytes, tiff_start + 4, endian)? as usize;
    let ifd_start = tiff_start.checked_add(ifd_offset)?;
    if ifd_start + 2 > tiff_end {
        return None;
    }

    let entry_count = read_u16(bytes, ifd_start, endian)? as usize;
    for index in 0..entry_count {
        let entry_offset = ifd_start + 2 + index * 12;
        if entry_offset + 12 > tiff_end {
            return None;
        }

        let tag = read_u16(bytes, entry_offset, endian)?;
        if tag != 0x0112 {
            continue;
        }

        let value = match endian {
            Endian::Big => {
                if bytes[entry_offset + 8] != 0 {
                    return None;
                }
                bytes[entry_offset + 9]
            }
            Endian::Little => {
                if bytes[entry_offset + 9] != 0 {
                    return None;
                }
                bytes[entry_offset + 8]
            }
        };

        return Some(OrientationTag {
            entry_offset,
            endian,
            value,
        });
    }

    None
}

fn overwrite_orientation_entry(bytes: &mut [u8], tag: &OrientationTag, value: u8) {
    let entry = tag.entry_offset;
    if entry + 12 > bytes.len() {
        return;
    }

    match tag.endian {
        Endian::Big => {
            bytes[entry + 2..entry + 12].copy_from_slice(&[0, 3, 0, 0, 0, 1, 0, value, 0, 0]);
        }
        Endian::Little => {
            bytes[entry + 2..entry + 12].copy_from_slice(&[3, 0, 1, 0, 0, 0, value, 0, 0, 0]);
        }
    }
}

fn read_u16(bytes: &[u8], offset: usize, endian: Endian) -> Option<u16> {
    let slice = bytes.get(offset..offset + 2)?;
    Some(match endian {
        Endian::Little => u16::from_le_bytes([slice[0], slice[1]]),
        Endian::Big => u16::from_be_bytes([slice[0], slice[1]]),
    })
}

fn read_u32(bytes: &[u8], offset: usize, endian: Endian) -> Option<u32> {
    let slice = bytes.get(offset..offset + 4)?;
    Some(match endian {
        Endian::Little => u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]),
        Endian::Big => u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]),
    })
}
