use std::{
    env,
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
};

pub mod cdjpeg;
pub mod image_io;
pub mod rdcolmap;
pub mod rdswitch;

pub const INTERNAL_TOOL_DIR: &str = "libexec/libjpeg-turbo-safe";

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

pub const WRAPPER_TOOL_NAMES: &[&str] = &[
    "cjpeg",
    "djpeg",
    "jpegtran",
    "rdjpgcom",
    "wrjpgcom",
    "tjbench",
    "jpegexiforient",
    "tjexample",
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

pub fn internal_tools_dir_from_exe(exe_path: &Path) -> Result<PathBuf, String> {
    if let Some(path) = env::var_os("LIBJPEG_TURBO_SAFE_INTERNAL_TOOLS_DIR") {
        return Ok(PathBuf::from(path));
    }

    let usr_dir = exe_path
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| format!("could not derive usr/ from {}", exe_path.display()))?;
    Ok(usr_dir.join(INTERNAL_TOOL_DIR))
}

pub fn internal_tool_path(tool: &str) -> Result<PathBuf, String> {
    let exe_path = env::current_exe()
        .map_err(|error| format!("failed to resolve current executable: {error}"))?;
    Ok(internal_tools_dir_from_exe(&exe_path)?.join(format!("{tool}-real")))
}

pub fn exec_internal_tool(tool: &str) -> ! {
    let public_argv0 = env::args_os().next();
    let internal = match internal_tool_path(tool) {
        Ok(path) => path,
        Err(error) => {
            eprintln!("{tool}: {error}");
            std::process::exit(127);
        }
    };

    let mut command = Command::new(&internal);
    if let Some(arg0) = public_argv0 {
        command.arg0(arg0);
    }
    command.args(env::args_os().skip(1));

    let error = command.exec();
    eprintln!(
        "{tool}: failed to execute staged internal tool {}: {error}",
        internal.display()
    );
    std::process::exit(127);
}
