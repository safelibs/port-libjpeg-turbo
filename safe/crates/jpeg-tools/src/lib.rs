use std::{
    ffi::CString,
    os::{
        raw::{c_char, c_int},
        unix::ffi::OsStringExt,
    },
};

pub mod cdjpeg;
pub mod image_io;
pub mod rdcolmap;
pub mod rdswitch;

pub const PACKAGED_TOOL_NAMES: &[&str] = &[
    "cjpeg",
    "djpeg",
    "jpegtran",
    "rdjpgcom",
    "wrjpgcom",
    "tjbench",
    "jpegexiforient",
    "exifautotran",
];

pub const MANPAGE_NAMES: &[&str] = &[
    "cjpeg.1",
    "djpeg.1",
    "jpegtran.1",
    "rdjpgcom.1",
    "wrjpgcom.1",
    "tjbench.1",
    "jpegexiforient.1",
    "exifautotran.1",
];

pub type EmbeddedToolMain = unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int;

pub fn run_embedded_tool(tool: &str, entry: EmbeddedToolMain) -> ! {
    let args = match std::env::args_os()
        .map(|arg| CString::new(arg.into_vec()))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(args) => args,
        Err(_) => {
            eprintln!("{tool}: arguments contain an unexpected NUL byte");
            std::process::exit(1);
        }
    };

    let mut argv = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .collect::<Vec<_>>();
    argv.push(std::ptr::null_mut());

    let argc = c_int::try_from(args.len()).unwrap_or(c_int::MAX);
    let code = unsafe { entry(argc, argv.as_mut_ptr()) };
    std::process::exit(code);
}
