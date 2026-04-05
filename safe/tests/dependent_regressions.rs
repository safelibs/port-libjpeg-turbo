#![allow(clippy::all)]

use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::OnceLock,
};

struct StagePaths {
    stage_cmake_dir: PathBuf,
}

static STAGE_PATHS: OnceLock<Result<StagePaths, String>> = OnceLock::new();

#[test]
fn turbojpeg_cmake_target_configures_krita_style_probe() {
    let stage = stage_paths().expect("stage paths");
    let temp_dir = new_temp_dir("krita-cmake").expect("temp dir");

    fs::write(
        temp_dir.join("CMakeLists.txt"),
        format!(
            concat!(
                "cmake_minimum_required(VERSION 3.16)\n",
                "project(krita_turbojpeg_probe C)\n",
                "find_package(libjpeg-turbo CONFIG REQUIRED PATHS \"{}\" NO_DEFAULT_PATH)\n",
                "add_executable(probe probe.c)\n",
                "target_link_libraries(probe PRIVATE turbojpeg)\n"
            ),
            stage.stage_cmake_dir.display()
        ),
    )
    .expect("write CMakeLists.txt");
    fs::write(
        temp_dir.join("probe.c"),
        concat!(
            "#include <turbojpeg.h>\n",
            "int main(void) { return TJPF_RGB; }\n",
        ),
    )
    .expect("write probe.c");

    let output = Command::new("cmake")
        .arg("-S")
        .arg(&temp_dir)
        .arg("-B")
        .arg(temp_dir.join("build"))
        .output()
        .expect("spawn cmake");

    assert!(
        output.status.success(),
        "cmake failed\n{}",
        command_failure("cmake", &output)
    );

    let build_output = Command::new("cmake")
        .arg("--build")
        .arg(temp_dir.join("build"))
        .output()
        .expect("spawn cmake --build");
    assert!(
        build_output.status.success(),
        "cmake --build failed\n{}",
        command_failure("cmake --build", &build_output)
    );
}

fn stage_paths() -> Result<&'static StagePaths, String> {
    STAGE_PATHS
        .get_or_init(|| {
            let safe_root = safe::safe_root().to_path_buf();
            let status = Command::new("bash")
                .arg("scripts/stage-install.sh")
                .current_dir(&safe_root)
                .status()
                .map_err(|error| format!("failed to run stage-install.sh: {error}"))?;
            if !status.success() {
                return Err(format!("stage-install.sh exited with status {status}"));
            }

            let stage_libdir = safe::stage_libdir()
                .ok_or_else(|| "unable to locate staged multiarch libdir".to_string())?;
            let stage_cmake_dir = stage_libdir.join("cmake/libjpeg-turbo");
            if !stage_cmake_dir.join("libjpeg-turboConfig.cmake").is_file() {
                return Err(format!(
                    "missing staged CMake config at {}",
                    stage_cmake_dir.display()
                ));
            }

            Ok(StagePaths { stage_cmake_dir })
        })
        .as_ref()
        .map_err(Clone::clone)
}

fn new_temp_dir(name: &str) -> Result<PathBuf, String> {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "libjpeg-dependent-regressions-{}-{name}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)
            .map_err(|error| format!("remove_dir_all {}: {error}", path.display()))?;
    }
    fs::create_dir_all(&path)
        .map_err(|error| format!("create_dir_all {}: {error}", path.display()))?;
    Ok(path)
}

fn command_failure(tool: &str, output: &Output) -> String {
    format!(
        "{tool} exited with status {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}
