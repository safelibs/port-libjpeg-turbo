use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
};

use libtest_mimic::{Arguments, Failed, Trial};

#[derive(Clone)]
struct MatrixCommand {
    tool: &'static str,
    args: Vec<&'static str>,
    verify: Option<(&'static str, &'static str)>,
}

#[derive(Clone)]
struct MatrixCase {
    name: &'static str,
    commands: Vec<MatrixCommand>,
}

struct StagePaths {
    original_testimages: PathBuf,
    stage_bin: PathBuf,
    stage_lib: PathBuf,
}

static STAGE_PATHS: OnceLock<Result<StagePaths, String>> = OnceLock::new();

fn main() {
    let args = Arguments::from_args();
    let trials = baseline_decode_cases()
        .into_iter()
        .map(|case| {
            let name = case.name;
            Trial::test(name, move || run_case(&case).map_err(Failed::from))
        })
        .collect();
    libtest_mimic::run(&args, trials).exit();
}

fn baseline_decode_cases() -> Vec<MatrixCase> {
    let mut cases = vec![
        MatrixCase {
            name: "baseline-decode-rgb-islow",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-rgb",
                        "-dct",
                        "int",
                        "-icc",
                        "@ORIG:test1.icc",
                        "-outfile",
                        "@TMP:testout_rgb_islow.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-ppm",
                        "-icc",
                        "@TMP:testout_rgb_islow.icc",
                        "-outfile",
                        "@TMP:testout_rgb_islow.ppm",
                        "@TMP:testout_rgb_islow.jpg",
                    ],
                    Some((
                        "@TMP:testout_rgb_islow.ppm",
                        "00a257f5393fef8821f2b88ac7421291",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-rgb-islow-icc",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-rgb",
                        "-dct",
                        "int",
                        "-icc",
                        "@ORIG:test1.icc",
                        "-outfile",
                        "@TMP:testout_rgb_islow.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-ppm",
                        "-icc",
                        "@TMP:testout_rgb_islow.icc",
                        "-outfile",
                        "@TMP:testout_rgb_islow.ppm",
                        "@TMP:testout_rgb_islow.jpg",
                    ],
                    Some((
                        "@TMP:testout_rgb_islow.icc",
                        "b06a39d730129122e85c1363ed1bbc9e",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-rgb-islow-565",
            commands: vec![
                rgb_islow_setup(),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-rgb565",
                        "-dither",
                        "none",
                        "-bmp",
                        "-outfile",
                        "@TMP:testout_rgb_islow_565.bmp",
                        "@TMP:testout_rgb_islow.jpg",
                    ],
                    Some((
                        "@TMP:testout_rgb_islow_565.bmp",
                        "f07d2e75073e4bb10f6c6f4d36e2e3be",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-rgb-islow-565d",
            commands: vec![
                rgb_islow_setup(),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-rgb565",
                        "-bmp",
                        "-outfile",
                        "@TMP:testout_rgb_islow_565D.bmp",
                        "@TMP:testout_rgb_islow.jpg",
                    ],
                    Some((
                        "@TMP:testout_rgb_islow_565D.bmp",
                        "4cfa0928ef3e6bb626d7728c924cfda4",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-422-ifast",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-sample",
                        "2x1",
                        "-dct",
                        "fast",
                        "-opt",
                        "-outfile",
                        "@TMP:testout_422_ifast_opt.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "fast",
                        "-outfile",
                        "@TMP:testout_422_ifast.ppm",
                        "@TMP:testout_422_ifast_opt.jpg",
                    ],
                    Some((
                        "@TMP:testout_422_ifast.ppm",
                        "35bd6b3f833bad23de82acea847129fa",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-440-islow",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-sample",
                        "1x2",
                        "-dct",
                        "int",
                        "-outfile",
                        "@TMP:testout_440_islow.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-outfile",
                        "@TMP:testout_440_islow.ppm",
                        "@TMP:testout_440_islow.jpg",
                    ],
                    Some((
                        "@TMP:testout_440_islow.ppm",
                        "11e7eab7ef7ef3276934bb7e7b6bb377",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-422m-ifast",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-sample",
                        "2x1",
                        "-dct",
                        "fast",
                        "-opt",
                        "-outfile",
                        "@TMP:testout_422_ifast_opt.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "fast",
                        "-nosmooth",
                        "-outfile",
                        "@TMP:testout_422m_ifast.ppm",
                        "@TMP:testout_422_ifast_opt.jpg",
                    ],
                    Some((
                        "@TMP:testout_422m_ifast.ppm",
                        "8dbc65323d62cca7c91ba02dd1cfa81d",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-422m-ifast-565",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-sample",
                        "2x1",
                        "-dct",
                        "fast",
                        "-opt",
                        "-outfile",
                        "@TMP:testout_422_ifast_opt.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-nosmooth",
                        "-rgb565",
                        "-dither",
                        "none",
                        "-bmp",
                        "-outfile",
                        "@TMP:testout_422m_ifast_565.bmp",
                        "@TMP:testout_422_ifast_opt.jpg",
                    ],
                    Some((
                        "@TMP:testout_422m_ifast_565.bmp",
                        "3294bd4d9a1f2b3d08ea6020d0db7065",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-422m-ifast-565d",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-sample",
                        "2x1",
                        "-dct",
                        "fast",
                        "-opt",
                        "-outfile",
                        "@TMP:testout_422_ifast_opt.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-nosmooth",
                        "-rgb565",
                        "-bmp",
                        "-outfile",
                        "@TMP:testout_422m_ifast_565D.bmp",
                        "@TMP:testout_422_ifast_opt.jpg",
                    ],
                    Some((
                        "@TMP:testout_422m_ifast_565D.bmp",
                        "da98c9c7b6039511be4a79a878a9abc1",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-gray-islow",
            commands: vec![
                gray_islow_setup(),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-outfile",
                        "@TMP:testout_gray_islow.ppm",
                        "@TMP:testout_gray_islow.jpg",
                    ],
                    Some((
                        "@TMP:testout_gray_islow.ppm",
                        "8d3596c56eace32f205deccc229aa5ed",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-gray-islow-rgb",
            commands: vec![
                gray_islow_setup(),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-rgb",
                        "-outfile",
                        "@TMP:testout_gray_islow_rgb.ppm",
                        "@TMP:testout_gray_islow.jpg",
                    ],
                    Some((
                        "@TMP:testout_gray_islow_rgb.ppm",
                        "116424ac07b79e5e801f00508eab48ec",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-gray-islow-565",
            commands: vec![
                gray_islow_setup(),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-rgb565",
                        "-dither",
                        "none",
                        "-bmp",
                        "-outfile",
                        "@TMP:testout_gray_islow_565.bmp",
                        "@TMP:testout_gray_islow.jpg",
                    ],
                    Some((
                        "@TMP:testout_gray_islow_565.bmp",
                        "12f78118e56a2f48b966f792fedf23cc",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-gray-islow-565d",
            commands: vec![
                gray_islow_setup(),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-rgb565",
                        "-bmp",
                        "-outfile",
                        "@TMP:testout_gray_islow_565D.bmp",
                        "@TMP:testout_gray_islow.jpg",
                    ],
                    Some((
                        "@TMP:testout_gray_islow_565D.bmp",
                        "bdbbd616441a24354c98553df5dc82db",
                    )),
                ),
            ],
        },
        MatrixCase {
            name: "baseline-decode-420-islow-256",
            commands: vec![cmd(
                "djpeg",
                &[
                    "-dct",
                    "int",
                    "-colors",
                    "256",
                    "-bmp",
                    "-outfile",
                    "@TMP:testout_420_islow_256.bmp",
                    "@ORIG:testorig.jpg",
                ],
                Some((
                    "@TMP:testout_420_islow_256.bmp",
                    "4980185e3776e89bd931736e1cddeee6",
                )),
            )],
        },
        MatrixCase {
            name: "baseline-decode-420-islow-565",
            commands: vec![cmd(
                "djpeg",
                &[
                    "-dct",
                    "int",
                    "-rgb565",
                    "-dither",
                    "none",
                    "-bmp",
                    "-outfile",
                    "@TMP:testout_420_islow_565.bmp",
                    "@ORIG:testorig.jpg",
                ],
                Some((
                    "@TMP:testout_420_islow_565.bmp",
                    "bf9d13e16c4923b92e1faa604d7922cb",
                )),
            )],
        },
        MatrixCase {
            name: "baseline-decode-420-islow-565d",
            commands: vec![cmd(
                "djpeg",
                &[
                    "-dct",
                    "int",
                    "-rgb565",
                    "-bmp",
                    "-outfile",
                    "@TMP:testout_420_islow_565D.bmp",
                    "@ORIG:testorig.jpg",
                ],
                Some((
                    "@TMP:testout_420_islow_565D.bmp",
                    "6bde71526acc44bcff76f696df8638d2",
                )),
            )],
        },
        MatrixCase {
            name: "baseline-decode-420m-islow-565",
            commands: vec![cmd(
                "djpeg",
                &[
                    "-dct",
                    "int",
                    "-nosmooth",
                    "-rgb565",
                    "-dither",
                    "none",
                    "-bmp",
                    "-outfile",
                    "@TMP:testout_420m_islow_565.bmp",
                    "@ORIG:testorig.jpg",
                ],
                Some((
                    "@TMP:testout_420m_islow_565.bmp",
                    "8dc0185245353cfa32ad97027342216f",
                )),
            )],
        },
        MatrixCase {
            name: "baseline-decode-420m-islow-565d",
            commands: vec![cmd(
                "djpeg",
                &[
                    "-dct",
                    "int",
                    "-nosmooth",
                    "-rgb565",
                    "-bmp",
                    "-outfile",
                    "@TMP:testout_420m_islow_565D.bmp",
                    "@ORIG:testorig.jpg",
                ],
                Some((
                    "@TMP:testout_420m_islow_565D.bmp",
                    "ce034037d212bc403330df6f915c161b",
                )),
            )],
        },
    ];

    for (scale, md5) in [
        ("2/1", "9f9de8c0612f8d06869b960b05abf9c9"),
        ("15/8", "b6875bc070720b899566cc06459b63b7"),
        ("13/8", "bc3452573c8152f6ae552939ee19f82f"),
        ("11/8", "d8cc73c0aaacd4556569b59437ba00a5"),
        ("9/8", "d25e61bc7eac0002f5b393aa223747b6"),
        ("7/8", "ddb564b7c74a09494016d6cd7502a946"),
        ("3/4", "8ed8e68808c3fbc4ea764fc9d2968646"),
        ("5/8", "a3363274999da2366a024efae6d16c9b"),
        ("1/2", "e692a315cea26b988c8e8b29a5dbcd81"),
        ("3/8", "79eca9175652ced755155c90e785a996"),
        ("1/4", "79cd778f8bf1a117690052cacdd54eca"),
        ("1/8", "391b3d4aca640c8567d6f8745eb2142f"),
    ] {
        let scale_file = scale.replace('/', "_");
        cases.push(MatrixCase {
            name: Box::leak(format!("baseline-decode-420m-islow-{scale_file}").into_boxed_str()),
            commands: vec![cmd(
                "djpeg",
                &[
                    "-dct",
                    "int",
                    "-scale",
                    Box::leak(scale.to_string().into_boxed_str()),
                    "-nosmooth",
                    "-ppm",
                    "-outfile",
                    Box::leak(format!("@TMP:testout_420m_islow_{scale_file}.ppm").into_boxed_str()),
                    "@ORIG:testorig.jpg",
                ],
                Some((
                    Box::leak(format!("@TMP:testout_420m_islow_{scale_file}.ppm").into_boxed_str()),
                    md5,
                )),
            )],
        });
    }

    cases
}

fn rgb_islow_setup() -> MatrixCommand {
    cmd(
        "cjpeg",
        &[
            "-rgb",
            "-dct",
            "int",
            "-icc",
            "@ORIG:test1.icc",
            "-outfile",
            "@TMP:testout_rgb_islow.jpg",
            "@ORIG:testorig.ppm",
        ],
        None,
    )
}

fn gray_islow_setup() -> MatrixCommand {
    cmd(
        "cjpeg",
        &[
            "-gray",
            "-dct",
            "int",
            "-outfile",
            "@TMP:testout_gray_islow.jpg",
            "@ORIG:testorig.ppm",
        ],
        None,
    )
}

fn cmd(
    tool: &'static str,
    args: &[&'static str],
    verify: Option<(&'static str, &'static str)>,
) -> MatrixCommand {
    MatrixCommand {
        tool,
        args: args.to_vec(),
        verify,
    }
}

fn run_case(case: &MatrixCase) -> Result<(), String> {
    let stage = stage_paths()?;
    let temp_dir = new_temp_dir(case.name)?;

    for command in &case.commands {
        let status = Command::new(stage.stage_bin.join(command.tool))
            .env("LD_LIBRARY_PATH", &stage.stage_lib)
            .current_dir(&temp_dir)
            .args(
                command
                    .args
                    .iter()
                    .map(|arg| expand_arg(arg, stage, &temp_dir)),
            )
            .status()
            .map_err(|error| format!("failed to spawn {}: {error}", command.tool))?;
        if !status.success() {
            return Err(format!("{} exited with status {status}", command.tool));
        }

        if let Some((file, expected_md5)) = command.verify {
            let path = expand_path(file, stage, &temp_dir);
            let digest = md5_file(&path)?;
            if digest != expected_md5 {
                return Err(format!(
                    "md5 mismatch for {}: expected {}, got {}",
                    path.display(),
                    expected_md5,
                    digest
                ));
            }
        }
    }

    Ok(())
}

fn stage_paths() -> Result<&'static StagePaths, String> {
    STAGE_PATHS
        .get_or_init(|| {
            let safe_root = safe::safe_root().to_path_buf();
            let repo_root = safe_root
                .parent()
                .ok_or_else(|| "safe root has no parent".to_string())?
                .to_path_buf();
            let status = Command::new("bash")
                .arg("scripts/stage-install.sh")
                .current_dir(&safe_root)
                .status()
                .map_err(|error| format!("failed to run stage-install.sh: {error}"))?;
            if !status.success() {
                return Err(format!("stage-install.sh exited with status {status}"));
            }

            let stage_bin = safe::stage_usr_root().join("bin");
            let stage_lib = find_stage_libdir()?;
            Ok(StagePaths {
                original_testimages: repo_root.join("original/testimages"),
                stage_bin,
                stage_lib,
            })
        })
        .as_ref()
        .map_err(Clone::clone)
}

fn find_stage_libdir() -> Result<PathBuf, String> {
    let lib_root = safe::stage_usr_root().join("lib");
    for entry in fs::read_dir(&lib_root)
        .map_err(|error| format!("read_dir {}: {error}", lib_root.display()))?
    {
        let entry =
            entry.map_err(|error| format!("read_dir entry {}: {error}", lib_root.display()))?;
        let path = entry.path();
        if path.is_dir() && path.join("libjpeg.so.8").exists() {
            return Ok(path);
        }
    }
    Err(format!(
        "could not find staged libjpeg under {}",
        lib_root.display()
    ))
}

fn expand_arg(arg: &str, stage: &StagePaths, temp_dir: &Path) -> OsString {
    if let Some(rest) = arg.strip_prefix("@ORIG:") {
        stage.original_testimages.join(rest).into_os_string()
    } else if let Some(rest) = arg.strip_prefix("@TMP:") {
        temp_dir.join(rest).into_os_string()
    } else {
        OsString::from(arg)
    }
}

fn expand_path(arg: &str, stage: &StagePaths, temp_dir: &Path) -> PathBuf {
    if let Some(rest) = arg.strip_prefix("@ORIG:") {
        stage.original_testimages.join(rest)
    } else if let Some(rest) = arg.strip_prefix("@TMP:") {
        temp_dir.join(rest)
    } else {
        PathBuf::from(arg)
    }
}

fn new_temp_dir(name: &str) -> Result<PathBuf, String> {
    let mut path = std::env::temp_dir();
    let salt = format!(
        "libjpeg-turbo-safe-{}-{}-{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("main"),
        name
    );
    path.push(salt.replace(['/', ':'], "_"));
    if path.exists() {
        fs::remove_dir_all(&path)
            .map_err(|error| format!("remove_dir_all {}: {error}", path.display()))?;
    }
    fs::create_dir_all(&path)
        .map_err(|error| format!("create_dir_all {}: {error}", path.display()))?;
    Ok(path)
}

fn md5_file(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    Ok(format!("{:x}", md5::compute(bytes)))
}
