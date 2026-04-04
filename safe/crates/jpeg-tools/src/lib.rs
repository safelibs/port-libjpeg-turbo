use std::{
    ffi::OsString,
    io,
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
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

fn find_safe_root_from(path: &Path) -> Result<PathBuf, String> {
    for ancestor in path.ancestors() {
        if ancestor.join("Cargo.toml").is_file() && ancestor.join("scripts/stage-install.sh").is_file() {
            return Ok(ancestor.to_path_buf());
        }
    }
    Err(format!(
        "could not locate safe/ root from {}",
        path.display()
    ))
}

fn find_stage_libdir(safe_root: &Path) -> Result<PathBuf, String> {
    let lib_root = safe_root.join("stage/usr/lib");
    let entries = std::fs::read_dir(&lib_root)
        .map_err(|error| format!("read_dir {}: {error}", lib_root.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("read_dir {}: {error}", lib_root.display()))?;
        let path = entry.path();
        if path.is_dir()
            && (path.join("libjpeg.so.8").exists() || path.join("libturbojpeg.so.0").exists())
        {
            return Ok(path);
        }
    }
    Err(format!(
        "could not find staged library directory under {}",
        lib_root.display()
    ))
}

fn join_library_path(paths: &[PathBuf]) -> Result<OsString, String> {
    std::env::join_paths(paths).map_err(|error| format!("join LD_LIBRARY_PATH entries: {error}"))
}

fn host_multiarch() -> Option<String> {
    for (program, args) in [
        ("dpkg-architecture", &["-qDEB_HOST_MULTIARCH"][..]),
        ("gcc", &["-print-multiarch"][..]),
    ] {
        if let Ok(output) = std::process::Command::new(program).args(args).output() {
            if output.status.success() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
                if !value.is_empty() {
                    return Some(value);
                }
            }
        }
    }
    None
}

fn find_runtime_backend_dir(safe_root: &Path) -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("LIBJPEG_TURBO_TOOL_BACKEND_DIR") {
        return Ok(PathBuf::from(path));
    }

    let multiarch = host_multiarch().ok_or_else(|| "could not determine host multiarch".to_string())?;
    let path = safe_root.join("runtime").join(multiarch).join("bin");
    if path.is_dir() {
        Ok(path)
    } else {
        Err(format!(
            "could not find packaged tool backend directory under {}",
            path.display()
        ))
    }
}

pub fn exec_packaged_tool_backend(tool: &str) -> ! {
    let exe = match std::env::current_exe() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("{tool}: current_exe failed: {error}");
            std::process::exit(1);
        }
    };
    let safe_root = std::env::var_os("LIBJPEG_TURBO_SAFE_ROOT")
        .map(PathBuf::from)
        .or_else(|| find_safe_root_from(&exe).ok());

    let build_dir = match safe_root.as_ref() {
        Some(root) => match find_runtime_backend_dir(root) {
            Ok(path) => path,
            Err(message) => {
                eprintln!("{tool}: {message}");
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("{tool}: could not locate safe/ root from {}", exe.display());
            std::process::exit(1);
        }
    };
    let backend = build_dir.join(tool);
    if !backend.is_file() {
        eprintln!("{tool}: missing packaged backend tool {}", backend.display());
        std::process::exit(1);
    }

    let mut library_paths = Vec::new();
    if let Some(stage_libdir) = std::env::var_os("LIBJPEG_TURBO_STAGE_LIBDIR").map(PathBuf::from) {
        library_paths.push(stage_libdir);
    } else if let Some(safe_root) = safe_root.as_ref() {
        if let Ok(stage_libdir) = find_stage_libdir(safe_root) {
            library_paths.push(stage_libdir);
        }
    }
    library_paths.push(build_dir.clone());
    if let Some(existing) = std::env::var_os("LD_LIBRARY_PATH") {
        library_paths.extend(std::env::split_paths(&existing));
    }
    let ld_library_path = match join_library_path(&library_paths) {
        Ok(value) => value,
        Err(message) => {
            eprintln!("{tool}: {message}");
            std::process::exit(1);
        }
    };

    let args = std::env::args_os().skip(1);
    let error = Command::new(&backend)
        .arg0(tool)
        .args(args)
        .env("LD_LIBRARY_PATH", ld_library_path)
        .exec();

    report_exec_error(tool, &backend, error)
}

fn report_exec_error(tool: &str, backend: &Path, error: io::Error) -> ! {
    eprintln!("{tool}: exec {} failed: {error}", backend.display());
    std::process::exit(1);
}
