use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
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
    runner: Option<fn(&StagePaths, &Path) -> Result<(), String>>,
}

struct StagePaths {
    original_testimages: PathBuf,
    stage_bin: PathBuf,
    stage_lib: PathBuf,
}

static STAGE_PATHS: OnceLock<Result<StagePaths, String>> = OnceLock::new();

fn main() {
    let args = Arguments::from_args();
    let trials = matrix_cases()
        .into_iter()
        .map(|case| {
            let name = case.name;
            Trial::test(name, move || run_case(&case).map_err(Failed::from))
        })
        .collect();
    libtest_mimic::run(&args, trials).exit();
}

fn matrix_cases() -> Vec<MatrixCase> {
    let mut cases = baseline_decode_cases();
    cases.extend(advanced_decode_cases());
    cases.extend(croptest_cases());
    cases
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
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
            runner: None,
        });
    }

    cases
}

fn advanced_decode_cases() -> Vec<MatrixCase> {
    vec![
        MatrixCase {
            name: "advanced-decode-progressive-420-q100-ifast",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-sample",
                        "2x2",
                        "-quality",
                        "100",
                        "-dct",
                        "fast",
                        "-scans",
                        "@ORIG:test.scan",
                        "-outfile",
                        "@TMP:testout_420_q100_ifast_prog.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "fast",
                        "-ppm",
                        "-outfile",
                        "@TMP:testout_420_q100_ifast.ppm",
                        "@TMP:testout_420_q100_ifast_prog.jpg",
                    ],
                    Some((
                        "@TMP:testout_420_q100_ifast.ppm",
                        "5a732542015c278ff43635e473a8a294",
                    )),
                ),
            ],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-progressive-420m-q100-ifast",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-sample",
                        "2x2",
                        "-quality",
                        "100",
                        "-dct",
                        "fast",
                        "-scans",
                        "@ORIG:test.scan",
                        "-outfile",
                        "@TMP:testout_420_q100_ifast_prog.jpg",
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
                        "-ppm",
                        "-outfile",
                        "@TMP:testout_420m_q100_ifast.ppm",
                        "@TMP:testout_420_q100_ifast_prog.jpg",
                    ],
                    Some((
                        "@TMP:testout_420m_q100_ifast.ppm",
                        "ff692ee9323a3b424894862557c092f1",
                    )),
                ),
            ],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-progressive-3x2-ifast",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-sample",
                        "3x2",
                        "-dct",
                        "fast",
                        "-prog",
                        "-outfile",
                        "@TMP:testout_3x2_ifast_prog.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "fast",
                        "-ppm",
                        "-outfile",
                        "@TMP:testout_3x2_ifast.ppm",
                        "@TMP:testout_3x2_ifast_prog.jpg",
                    ],
                    Some(("@TMP:testout_3x2_ifast.ppm", "fd283664b3b49127984af0a7f118fccd")),
                ),
            ],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-arithmetic-420m-ifast-skip",
            commands: vec![cmd(
                "djpeg",
                &[
                    "-fast",
                    "-skip",
                    "1,20",
                    "-ppm",
                    "-outfile",
                    "@TMP:testout_420m_ifast_ari.ppm",
                    "@ORIG:testimgari.jpg",
                ],
                Some(("@TMP:testout_420m_ifast_ari.ppm", "57251da28a35b46eecb7177d82d10e0e")),
            )],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-coefficients-jpegtran-arithmetic",
            commands: vec![cmd(
                "jpegtran",
                &[
                    "-outfile",
                    "@TMP:testout_420_islow.jpg",
                    "@ORIG:testimgari.jpg",
                ],
                Some(("@TMP:testout_420_islow.jpg", "9a68f56bc76e466aa7e52f415d0f4a5f")),
            )],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-skip-420-islow",
            commands: vec![cmd(
                "djpeg",
                &[
                    "-dct",
                    "int",
                    "-skip",
                    "15,31",
                    "-ppm",
                    "-outfile",
                    "@TMP:testout_420_islow_skip15_31.ppm",
                    "@ORIG:testorig.jpg",
                ],
                Some((
                    "@TMP:testout_420_islow_skip15_31.ppm",
                    "c4c65c1e43d7275cd50328a61e6534f0",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-skip-420-ari",
            commands: vec![cmd(
                "djpeg",
                &[
                    "-dct",
                    "int",
                    "-skip",
                    "16,139",
                    "-ppm",
                    "-outfile",
                    "@TMP:testout_420_islow_ari_skip16_139.ppm",
                    "@ORIG:testimgari.jpg",
                ],
                Some((
                    "@TMP:testout_420_islow_ari_skip16_139.ppm",
                    "087c6b123db16ac00cb88c5b590bb74a",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-crop-420-prog",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-dct",
                        "int",
                        "-prog",
                        "-outfile",
                        "@TMP:testout_420_islow_prog.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-crop",
                        "62x62+71+71",
                        "-ppm",
                        "-outfile",
                        "@TMP:testout_420_islow_prog_crop62x62_71_71.ppm",
                        "@TMP:testout_420_islow_prog.jpg",
                    ],
                    Some((
                        "@TMP:testout_420_islow_prog_crop62x62_71_71.ppm",
                        "26eb36ccc7d1f0cb80cdabb0ac8b5d99",
                    )),
                ),
            ],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-crop-420-ari",
            commands: vec![cmd(
                "djpeg",
                &[
                    "-dct",
                    "int",
                    "-crop",
                    "53x53+4+4",
                    "-ppm",
                    "-outfile",
                    "@TMP:testout_420_islow_ari_crop53x53_4_4.ppm",
                    "@ORIG:testimgari.jpg",
                ],
                Some((
                    "@TMP:testout_420_islow_ari_crop53x53_4_4.ppm",
                    "886c6775af22370257122f8b16207e6d",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-skip-444-islow",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-dct",
                        "int",
                        "-sample",
                        "1x1",
                        "-outfile",
                        "@TMP:testout_444_islow.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-skip",
                        "1,6",
                        "-ppm",
                        "-outfile",
                        "@TMP:testout_444_islow_skip1_6.ppm",
                        "@TMP:testout_444_islow.jpg",
                    ],
                    Some((
                        "@TMP:testout_444_islow_skip1_6.ppm",
                        "5606f86874cf26b8fcee1117a0a436a6",
                    )),
                ),
            ],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-crop-444-prog",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-dct",
                        "int",
                        "-prog",
                        "-sample",
                        "1x1",
                        "-outfile",
                        "@TMP:testout_444_islow_prog.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-crop",
                        "98x98+13+13",
                        "-ppm",
                        "-outfile",
                        "@TMP:testout_444_islow_prog_crop98x98_13_13.ppm",
                        "@TMP:testout_444_islow_prog.jpg",
                    ],
                    Some((
                        "@TMP:testout_444_islow_prog_crop98x98_13_13.ppm",
                        "db87dc7ce26bcdc7a6b56239ce2b9d6c",
                    )),
                ),
            ],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-crop-444-ari",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-dct",
                        "int",
                        "-arithmetic",
                        "-sample",
                        "1x1",
                        "-outfile",
                        "@TMP:testout_444_islow_ari.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-dct",
                        "int",
                        "-crop",
                        "37x37+0+0",
                        "-ppm",
                        "-outfile",
                        "@TMP:testout_444_islow_ari_crop37x37_0_0.ppm",
                        "@TMP:testout_444_islow_ari.jpg",
                    ],
                    Some((
                        "@TMP:testout_444_islow_ari_crop37x37_0_0.ppm",
                        "cb57b32bd6d03e35432362f7bf184b6d",
                    )),
                ),
            ],
            runner: None,
        },
    ]
}

fn croptest_cases() -> Vec<MatrixCase> {
    vec![MatrixCase {
        name: "croptest",
        commands: Vec::new(),
        runner: Some(run_croptest_case),
    }]
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

    if let Some(runner) = case.runner {
        return runner(stage, &temp_dir);
    }

    for command in &case.commands {
        run_matrix_command(stage, &temp_dir, command)?;
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

fn run_matrix_command(stage: &StagePaths, temp_dir: &Path, command: &MatrixCommand) -> Result<(), String> {
    let output = run_stage_command(
        stage,
        temp_dir,
        command.tool,
        command
            .args
            .iter()
            .map(|arg| expand_arg(arg, stage, temp_dir))
            .collect::<Vec<_>>(),
    )?;

    if !output.status.success() {
        return Err(command_failure(command.tool, &output));
    }

    if let Some((file, expected_md5)) = command.verify {
        let path = expand_path(file, stage, temp_dir);
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

    Ok(())
}

fn run_stage_command(
    stage: &StagePaths,
    temp_dir: &Path,
    tool: &str,
    args: Vec<OsString>,
) -> Result<Output, String> {
    Command::new(stage.stage_bin.join(tool))
        .env("LD_LIBRARY_PATH", &stage.stage_lib)
        .current_dir(temp_dir)
        .args(args)
        .output()
        .map_err(|error| format!("failed to spawn {tool}: {error}"))
}

fn command_failure(tool: &str, output: &Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    format!(
        "{tool} exited with status {}{}\n{}{}",
        output.status,
        if stdout.is_empty() { "" } else { " (stdout follows)" },
        stdout,
        if stderr.is_empty() {
            String::new()
        } else {
            format!("\n{stderr}")
        }
    )
}

#[derive(Clone)]
struct PpmImage {
    width: usize,
    height: usize,
    maxval: usize,
    data: Vec<u8>,
}

fn run_croptest_case(stage: &StagePaths, temp_dir: &Path) -> Result<(), String> {
    const IMAGE: &str = "vgl_6548_0026a.bmp";
    const WIDTH: usize = 128;
    const HEIGHT: usize = 95;
    const SAMPLES: [(&str, &[&str]); 5] = [
        ("GRAY", &["-grayscale"]),
        ("420", &["-sample", "2x2"]),
        ("422", &["-sample", "2x1"]),
        ("440", &["-sample", "1x2"]),
        ("444", &["-sample", "1x1"]),
    ];
    const NOSMOOTH: [Option<&str>; 2] = [None, Some("-nosmooth")];
    const QUANT_ARGS: [&[&str]; 2] = [&[], &["-colors", "256", "-dither", "none", "-onepass"]];

    let source = stage.original_testimages.join(IMAGE);
    let basename = "vgl_6548_0026a";

    for progressive in [false, true] {
        let prog_tag = if progressive { "prog" } else { "base" };

        for (sample_name, sample_args) in SAMPLES {
            let mut args = Vec::new();
            if progressive {
                args.push(OsString::from("-progressive"));
            }
            args.extend(sample_args.iter().map(|arg| OsString::from(*arg)));
            args.push(OsString::from("-outfile"));
            args.push(temp_dir.join(format!("{basename}_{prog_tag}_{sample_name}.jpg")).into_os_string());
            args.push(source.clone().into_os_string());

            let output = run_stage_command(stage, temp_dir, "cjpeg", args)?;
            if !output.status.success() {
                return Err(command_failure("cjpeg", &output));
            }
        }

        for nosmooth in NOSMOOTH {
            let ns_tag = if nosmooth.is_some() { "nosmooth" } else { "smooth" };
            for quant_args in QUANT_ARGS {
                let quant_tag = if quant_args.is_empty() { "full" } else { "quant256" };

                for (sample_name, _) in SAMPLES {
                    let jpeg_path = temp_dir.join(format!("{basename}_{prog_tag}_{sample_name}.jpg"));
                    let full_path =
                        temp_dir.join(format!("{basename}_{prog_tag}_{ns_tag}_{quant_tag}_{sample_name}_full.ppm"));
                    let mut args = Vec::new();
                    if let Some(flag) = nosmooth {
                        args.push(OsString::from(flag));
                    }
                    args.extend(quant_args.iter().map(|arg| OsString::from(*arg)));
                    args.push(OsString::from("-rgb"));
                    args.push(OsString::from("-outfile"));
                    args.push(full_path.clone().into_os_string());
                    args.push(jpeg_path.clone().into_os_string());

                    let output = run_stage_command(stage, temp_dir, "djpeg", args)?;
                    if !output.status.success() {
                        return Err(command_failure("djpeg", &output));
                    }
                    let full = read_ppm(&full_path)?;

                    for y in 0..=16 {
                        for h in 1..=16 {
                            let x = (y * 16) % WIDTH;
                            let w = WIDTH - x - 7;
                            let y0 = if y <= 15 { y } else { HEIGHT - h };
                            let cropspec = format!("{w}x{h}+{x}+{y0}");
                            let cropped_path = temp_dir.join(format!(
                                "{basename}_{prog_tag}_{ns_tag}_{quant_tag}_{sample_name}_{x}_{y0}_{w}_{h}.ppm"
                            ));

                            let mut args = Vec::new();
                            if let Some(flag) = nosmooth {
                                args.push(OsString::from(flag));
                            }
                            args.extend(quant_args.iter().map(|arg| OsString::from(*arg)));
                            args.push(OsString::from("-crop"));
                            args.push(OsString::from(cropspec.clone()));
                            args.push(OsString::from("-rgb"));
                            args.push(OsString::from("-outfile"));
                            args.push(cropped_path.clone().into_os_string());
                            args.push(jpeg_path.clone().into_os_string());

                            let output = run_stage_command(stage, temp_dir, "djpeg", args)?;
                            if !output.status.success() {
                                return Err(command_failure("djpeg", &output));
                            }

                            let expected = crop_ppm(&full, x, y0, w, h)?;
                            let actual = read_ppm(&cropped_path)?;
                            if expected.width != actual.width
                                || expected.height != actual.height
                                || expected.maxval != actual.maxval
                                || expected.data != actual.data
                            {
                                return Err(format!(
                                    "croptest mismatch for progressive={progressive} nosmooth={:?} quant={} sample={} crop={cropspec}",
                                    nosmooth,
                                    quant_tag,
                                    sample_name
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn read_ppm(path: &Path) -> Result<PpmImage, String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let mut offset = 0usize;
    let magic = next_ppm_token(&bytes, &mut offset)?;
    if magic != "P6" {
        return Err(format!("{} is not a binary PPM file", path.display()));
    }
    let width = next_ppm_token(&bytes, &mut offset)?
        .parse::<usize>()
        .map_err(|error| format!("invalid PPM width in {}: {error}", path.display()))?;
    let height = next_ppm_token(&bytes, &mut offset)?
        .parse::<usize>()
        .map_err(|error| format!("invalid PPM height in {}: {error}", path.display()))?;
    let maxval = next_ppm_token(&bytes, &mut offset)?
        .parse::<usize>()
        .map_err(|error| format!("invalid PPM maxval in {}: {error}", path.display()))?;
    if maxval > 255 {
        return Err(format!("{} uses unsupported PPM maxval {maxval}", path.display()));
    }
    skip_ppm_separators(&bytes, &mut offset);
    let expected_len = width
        .checked_mul(height)
        .and_then(|pixels| pixels.checked_mul(3))
        .ok_or_else(|| format!("PPM dimensions overflow for {}", path.display()))?;
    let data = bytes
        .get(offset..)
        .ok_or_else(|| format!("missing PPM pixel data in {}", path.display()))?
        .to_vec();
    if data.len() != expected_len {
        return Err(format!(
            "PPM pixel length mismatch for {}: expected {}, got {}",
            path.display(),
            expected_len,
            data.len()
        ));
    }
    Ok(PpmImage {
        width,
        height,
        maxval,
        data,
    })
}

fn crop_ppm(image: &PpmImage, x: usize, y: usize, width: usize, height: usize) -> Result<PpmImage, String> {
    if x + width > image.width || y + height > image.height {
        return Err(format!(
            "crop {x},{y} {width}x{height} falls outside {}x{} image",
            image.width,
            image.height
        ));
    }

    let row_stride = image.width * 3;
    let crop_stride = width * 3;
    let mut data = Vec::with_capacity(height * crop_stride);
    for row in y..(y + height) {
        let start = row * row_stride + x * 3;
        let end = start + crop_stride;
        data.extend_from_slice(&image.data[start..end]);
    }

    Ok(PpmImage {
        width,
        height,
        maxval: image.maxval,
        data,
    })
}

fn next_ppm_token(bytes: &[u8], offset: &mut usize) -> Result<String, String> {
    skip_ppm_separators(bytes, offset);
    let start = *offset;
    while *offset < bytes.len()
        && !bytes[*offset].is_ascii_whitespace()
        && bytes[*offset] != b'#'
    {
        *offset += 1;
    }
    if start == *offset {
        return Err("unexpected end of PPM header".to_string());
    }
    String::from_utf8(bytes[start..*offset].to_vec())
        .map_err(|error| format!("invalid PPM header token: {error}"))
}

fn skip_ppm_separators(bytes: &[u8], offset: &mut usize) {
    loop {
        while *offset < bytes.len() && bytes[*offset].is_ascii_whitespace() {
            *offset += 1;
        }
        if *offset < bytes.len() && bytes[*offset] == b'#' {
            while *offset < bytes.len() && bytes[*offset] != b'\n' {
                *offset += 1;
            }
            continue;
        }
        break;
    }
}
