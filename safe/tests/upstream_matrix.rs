#![allow(clippy::all)]

use std::{
    ffi::{CStr, CString, OsString},
    fs,
    mem::MaybeUninit,
    os::raw::{c_char, c_int, c_void},
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::OnceLock,
};

use ffi_types::{
    boolean, int, j_decompress_ptr, jpeg_compress_struct, jpeg_decompress_struct, jpeg_error_mgr,
    jvirt_barray_ptr, ulong, CSTATE_SCANNING, CSTATE_START, CSTATE_WRCOEFS, DSTATE_BUFIMAGE,
    DSTATE_READY, DSTATE_SCANNING, DSTATE_START, FALSE, JCS_RGB, JDCT_IFAST, JDCT_ISLOW,
    JDIMENSION, JPEG_HEADER_OK, JPEG_LIB_VERSION, JPEG_REACHED_EOI, JPEG_REACHED_SOS,
    JPEG_ROW_COMPLETED, JPEG_SCAN_COMPLETED, JSAMPARRAY, TRUE,
};
use libjpeg_abi::{
    common_exports::EXPECTED_COMMON_SYMBOLS,
    decompress_exports::EXPECTED_DECOMPRESS_SYMBOLS,
    transform::transupp::{
        self, jpeg_transform_info, JCOPYOPT_COMMENTS, JCROP_POS, JXFORM_ROT_90, JXFORM_TRANSPOSE,
    },
    EXPECTED_COMPRESS_SYMBOLS,
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

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dlerror() -> *const c_char;
    fn free(ptr: *mut c_void);
}

const RTLD_NOW: c_int = 2;

type JpegStdErrorFn = unsafe extern "C" fn(*mut jpeg_error_mgr) -> *mut jpeg_error_mgr;
type JpegCreateDecompressFn = unsafe extern "C" fn(j_decompress_ptr, int, usize);
type JpegDestroyDecompressFn = unsafe extern "C" fn(j_decompress_ptr);
type JpegMemSrcFn = unsafe extern "C" fn(j_decompress_ptr, *const u8, ulong);
type JpegReadHeaderFn = unsafe extern "C" fn(j_decompress_ptr, boolean) -> int;
type JpegHasMultipleScansFn = unsafe extern "C" fn(j_decompress_ptr) -> boolean;
type JpegStartDecompressFn = unsafe extern "C" fn(j_decompress_ptr) -> boolean;
type JpegConsumeInputFn = unsafe extern "C" fn(j_decompress_ptr) -> int;
type JpegStartOutputFn = unsafe extern "C" fn(j_decompress_ptr, int) -> boolean;
type JpegReadScanlinesFn =
    unsafe extern "C" fn(j_decompress_ptr, JSAMPARRAY, JDIMENSION) -> JDIMENSION;
type JpegFinishOutputFn = unsafe extern "C" fn(j_decompress_ptr) -> boolean;
type JpegInputCompleteFn = unsafe extern "C" fn(j_decompress_ptr) -> boolean;
type JpegFinishDecompressFn = unsafe extern "C" fn(j_decompress_ptr) -> boolean;
type JpegCreateCompressFn = unsafe extern "C" fn(*mut jpeg_compress_struct, int, usize);
type JpegDestroyCompressFn = unsafe extern "C" fn(*mut jpeg_compress_struct);
type JpegMemDestFn = unsafe extern "C" fn(*mut jpeg_compress_struct, *mut *mut u8, *mut ulong);
type JpegSetDefaultsFn = unsafe extern "C" fn(*mut jpeg_compress_struct);
type JpegSetColorspaceFn =
    unsafe extern "C" fn(*mut jpeg_compress_struct, ffi_types::J_COLOR_SPACE);
type JpegStartCompressFn = unsafe extern "C" fn(*mut jpeg_compress_struct, boolean);
type JpegWriteScanlinesFn =
    unsafe extern "C" fn(*mut jpeg_compress_struct, JSAMPARRAY, JDIMENSION) -> JDIMENSION;
type JpegFinishCompressFn = unsafe extern "C" fn(*mut jpeg_compress_struct);
type JpegWriteIccProfileFn =
    unsafe extern "C" fn(*mut jpeg_compress_struct, *const u8, ::core::ffi::c_uint);
type JpegReadCoefficientsFn = unsafe extern "C" fn(j_decompress_ptr) -> *mut jvirt_barray_ptr;
type JpegCopyCriticalParametersFn =
    unsafe extern "C" fn(j_decompress_ptr, *mut jpeg_compress_struct);
type JpegWriteCoefficientsFn =
    unsafe extern "C" fn(*mut jpeg_compress_struct, *mut jvirt_barray_ptr);

struct LoadedLibjpeg {
    handle: *mut c_void,
    jpeg_std_error: JpegStdErrorFn,
    jpeg_create_decompress: JpegCreateDecompressFn,
    jpeg_destroy_decompress: JpegDestroyDecompressFn,
    jpeg_mem_src: JpegMemSrcFn,
    jpeg_read_header: JpegReadHeaderFn,
    jpeg_has_multiple_scans: JpegHasMultipleScansFn,
    jpeg_start_decompress: JpegStartDecompressFn,
    jpeg_consume_input: JpegConsumeInputFn,
    jpeg_start_output: JpegStartOutputFn,
    jpeg_read_scanlines: JpegReadScanlinesFn,
    jpeg_finish_output: JpegFinishOutputFn,
    jpeg_input_complete: JpegInputCompleteFn,
    jpeg_finish_decompress: JpegFinishDecompressFn,
}

impl LoadedLibjpeg {
    fn open(path: &Path) -> Result<Self, String> {
        unsafe {
            let path = CString::new(path.to_string_lossy().into_owned())
                .map_err(|error| format!("invalid dlopen path {}: {error}", path.display()))?;
            let handle = dlopen(path.as_ptr(), RTLD_NOW);
            if handle.is_null() {
                return Err(format!(
                    "dlopen {} failed: {}",
                    path.to_string_lossy(),
                    dlerror_message()
                ));
            }

            Ok(Self {
                handle,
                jpeg_std_error: load_symbol(handle, b"jpeg_std_error\0")?,
                jpeg_create_decompress: load_symbol(handle, b"jpeg_CreateDecompress\0")?,
                jpeg_destroy_decompress: load_symbol(handle, b"jpeg_destroy_decompress\0")?,
                jpeg_mem_src: load_symbol(handle, b"jpeg_mem_src\0")?,
                jpeg_read_header: load_symbol(handle, b"jpeg_read_header\0")?,
                jpeg_has_multiple_scans: load_symbol(handle, b"jpeg_has_multiple_scans\0")?,
                jpeg_start_decompress: load_symbol(handle, b"jpeg_start_decompress\0")?,
                jpeg_consume_input: load_symbol(handle, b"jpeg_consume_input\0")?,
                jpeg_start_output: load_symbol(handle, b"jpeg_start_output\0")?,
                jpeg_read_scanlines: load_symbol(handle, b"jpeg_read_scanlines\0")?,
                jpeg_finish_output: load_symbol(handle, b"jpeg_finish_output\0")?,
                jpeg_input_complete: load_symbol(handle, b"jpeg_input_complete\0")?,
                jpeg_finish_decompress: load_symbol(handle, b"jpeg_finish_decompress\0")?,
            })
        }
    }
}

impl Drop for LoadedLibjpeg {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                let _ = dlclose(self.handle);
            }
        }
    }
}

unsafe fn load_symbol<T>(handle: *mut c_void, symbol: &'static [u8]) -> Result<T, String> {
    let symbol_name =
        CStr::from_bytes_with_nul(symbol).expect("symbol names are static and NUL-terminated");
    let ptr = dlsym(handle, symbol_name.as_ptr());
    if ptr.is_null() {
        return Err(format!(
            "dlsym({}) failed: {}",
            symbol_name.to_string_lossy(),
            dlerror_message()
        ));
    }
    Ok(std::mem::transmute_copy(&ptr))
}

unsafe fn dlerror_message() -> String {
    let message = dlerror();
    if message.is_null() {
        "unknown dlerror".to_string()
    } else {
        CStr::from_ptr(message).to_string_lossy().into_owned()
    }
}

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
    cases.extend(encode_transcode_cases());
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
            name: "baseline-decode-direct-api-sequential",
            commands: Vec::new(),
            runner: Some(run_sequential_api_case),
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
            name: "advanced-decode-progressive-420-q100-ifast-maxscans",
            commands: vec![
                cmd(
                    "cjpeg",
                    &[
                        "-quality",
                        "100",
                        "-dct",
                        "fast",
                        "-scans",
                        "@ORIG:test.scan",
                        "-outfile",
                        "@TMP:testout_420_q100_ifast_limitscans_prog.jpg",
                        "@ORIG:testorig.ppm",
                    ],
                    None,
                ),
                cmd(
                    "djpeg",
                    &[
                        "-maxscans",
                        "64",
                        "-dct",
                        "fast",
                        "-ppm",
                        "-outfile",
                        "@TMP:testout_420_q100_ifast_limitscans.ppm",
                        "@TMP:testout_420_q100_ifast_limitscans_prog.jpg",
                    ],
                    Some((
                        "@TMP:testout_420_q100_ifast_limitscans.ppm",
                        "5a732542015c278ff43635e473a8a294",
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
                    Some((
                        "@TMP:testout_3x2_ifast.ppm",
                        "fd283664b3b49127984af0a7f118fccd",
                    )),
                ),
            ],
            runner: None,
        },
        MatrixCase {
            name: "advanced-decode-buffered-image-progressive",
            commands: Vec::new(),
            runner: Some(run_buffered_image_case),
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
                Some((
                    "@TMP:testout_420m_ifast_ari.ppm",
                    "57251da28a35b46eecb7177d82d10e0e",
                )),
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
                Some((
                    "@TMP:testout_420_islow.jpg",
                    "9a68f56bc76e466aa7e52f415d0f4a5f",
                )),
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

fn encode_transcode_cases() -> Vec<MatrixCase> {
    vec![
        MatrixCase {
            name: "encode-transcode-libjpeg-symbol-surface",
            commands: Vec::new(),
            runner: Some(run_encode_transcode_symbol_surface_case),
        },
        MatrixCase {
            name: "encode-transcode-libjpeg-api-rgb-islow",
            commands: Vec::new(),
            runner: Some(run_encode_transcode_api_rgb_islow_case),
        },
        MatrixCase {
            name: "encode-transcode-libjpeg-api-transform-crop",
            commands: Vec::new(),
            runner: Some(run_encode_transcode_api_transform_crop_case),
        },
        MatrixCase {
            name: "encode-transcode-cjpeg-rgb-islow",
            commands: vec![cmd(
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
                Some((
                    "@TMP:testout_rgb_islow.jpg",
                    "1d44a406f61da743b5fd31c0a9abdca3",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-jpegtran-icc",
            commands: vec![
                rgb_islow_setup(),
                cmd(
                    "jpegtran",
                    &[
                        "-copy",
                        "all",
                        "-icc",
                        "@ORIG:test2.icc",
                        "-outfile",
                        "@TMP:testout_rgb_islow2.jpg",
                        "@TMP:testout_rgb_islow.jpg",
                    ],
                    Some((
                        "@TMP:testout_rgb_islow2.jpg",
                        "31d121e57b6c2934c890a7fc7763bcd4",
                    )),
                ),
            ],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-cjpeg-422-ifast-opt",
            commands: vec![cmd(
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
                Some((
                    "@TMP:testout_422_ifast_opt.jpg",
                    "2540287b79d913f91665e660303ab2c8",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-cjpeg-440-islow",
            commands: vec![cmd(
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
                Some((
                    "@TMP:testout_440_islow.jpg",
                    "538bc02bd4b4658fd85de6ece6cbeda6",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-cjpeg-420-q100-ifast-prog",
            commands: vec![cmd(
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
                Some((
                    "@TMP:testout_420_q100_ifast_prog.jpg",
                    "0ba15f9dab81a703505f835f9dbbac6d",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-cjpeg-gray-islow",
            commands: vec![cmd(
                "cjpeg",
                &[
                    "-gray",
                    "-dct",
                    "int",
                    "-outfile",
                    "@TMP:testout_gray_islow.jpg",
                    "@ORIG:testorig.ppm",
                ],
                Some((
                    "@TMP:testout_gray_islow.jpg",
                    "72b51f894b8f4a10b3ee3066770aa38d",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-cjpeg-420s-ifast-opt",
            commands: vec![cmd(
                "cjpeg",
                &[
                    "-sample",
                    "2x2",
                    "-smooth",
                    "1",
                    "-dct",
                    "int",
                    "-opt",
                    "-outfile",
                    "@TMP:testout_420s_ifast_opt.jpg",
                    "@ORIG:testorig.ppm",
                ],
                Some((
                    "@TMP:testout_420s_ifast_opt.jpg",
                    "388708217ac46273ca33086b22827ed8",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-cjpeg-3x2-ifast-prog",
            commands: vec![cmd(
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
                Some((
                    "@TMP:testout_3x2_ifast_prog.jpg",
                    "1ee5d2c1a77f2da495f993c8c7cceca5",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-cjpeg-420-islow-ari",
            commands: vec![cmd(
                "cjpeg",
                &[
                    "-dct",
                    "int",
                    "-arithmetic",
                    "-outfile",
                    "@TMP:testout_420_islow_ari.jpg",
                    "@ORIG:testorig.ppm",
                ],
                Some((
                    "@TMP:testout_420_islow_ari.jpg",
                    "e986fb0a637a8d833d96e8a6d6d84ea1",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-jpegtran-420-islow-ari",
            commands: vec![cmd(
                "jpegtran",
                &[
                    "-arithmetic",
                    "-outfile",
                    "@TMP:testout_420_islow_ari2.jpg",
                    "@ORIG:testimgint.jpg",
                ],
                Some((
                    "@TMP:testout_420_islow_ari2.jpg",
                    "e986fb0a637a8d833d96e8a6d6d84ea1",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-cjpeg-444-islow-progari",
            commands: vec![cmd(
                "cjpeg",
                &[
                    "-sample",
                    "1x1",
                    "-dct",
                    "int",
                    "-prog",
                    "-arithmetic",
                    "-outfile",
                    "@TMP:testout_444_islow_progari.jpg",
                    "@ORIG:testorig.ppm",
                ],
                Some((
                    "@TMP:testout_444_islow_progari.jpg",
                    "0a8f1c8f66e113c3cf635df0a475a617",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-jpegtran-420-islow",
            commands: vec![cmd(
                "jpegtran",
                &[
                    "-outfile",
                    "@TMP:testout_420_islow.jpg",
                    "@ORIG:testimgari.jpg",
                ],
                Some((
                    "@TMP:testout_420_islow.jpg",
                    "9a68f56bc76e466aa7e52f415d0f4a5f",
                )),
            )],
            runner: None,
        },
        MatrixCase {
            name: "encode-transcode-jpegtran-crop",
            commands: vec![cmd(
                "jpegtran",
                &[
                    "-crop",
                    "120x90+20+50",
                    "-transpose",
                    "-perfect",
                    "-outfile",
                    "@TMP:testout_crop.jpg",
                    "@ORIG:testorig.jpg",
                ],
                Some(("@TMP:testout_crop.jpg", "b4197f377e621c4e9b1d20471432610d")),
            )],
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
            let stage_root = new_temp_dir("stage-install")?;
            let status = Command::new("bash")
                .arg("scripts/stage-install.sh")
                .arg("--stage-dir")
                .arg(&stage_root)
                .current_dir(&safe_root)
                .status()
                .map_err(|error| format!("failed to run stage-install.sh: {error}"))?;
            if !status.success() {
                return Err(format!("stage-install.sh exited with status {status}"));
            }

            let stage_usr_root = stage_root.join("usr");
            let stage_bin = stage_usr_root.join("bin");
            let stage_lib = find_stage_libdir(&stage_usr_root)?;
            Ok(StagePaths {
                original_testimages: repo_root.join("original/testimages"),
                stage_bin,
                stage_lib,
            })
        })
        .as_ref()
        .map_err(Clone::clone)
}

fn find_stage_libdir(stage_usr_root: &Path) -> Result<PathBuf, String> {
    let lib_root = stage_usr_root.join("lib");
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

fn run_matrix_command(
    stage: &StagePaths,
    temp_dir: &Path,
    command: &MatrixCommand,
) -> Result<(), String> {
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
        .env("LIBJPEG_TURBO_STAGE_LIBDIR", &stage.stage_lib)
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
        if stdout.is_empty() {
            ""
        } else {
            " (stdout follows)"
        },
        stdout,
        if stderr.is_empty() {
            String::new()
        } else {
            format!("\n{stderr}")
        }
    )
}

#[derive(Clone, Eq, PartialEq)]
struct PpmImage {
    width: usize,
    height: usize,
    maxval: usize,
    data: Vec<u8>,
}

fn ppm_md5(image: &PpmImage) -> String {
    let mut bytes =
        format!("P6\n{} {}\n{}\n", image.width, image.height, image.maxval).into_bytes();
    bytes.extend_from_slice(&image.data);
    format!("{:x}", md5::compute(bytes))
}

fn md5_bytes(bytes: &[u8]) -> String {
    format!("{:x}", md5::compute(bytes))
}

unsafe fn take_output_buffer(outbuffer: *mut u8, outsize: ulong) -> Result<Vec<u8>, String> {
    if outbuffer.is_null() {
        return Err("jpeg_mem_dest returned a null output buffer".to_string());
    }
    let bytes = std::slice::from_raw_parts(outbuffer, outsize as usize).to_vec();
    free(outbuffer.cast());
    Ok(bytes)
}

fn run_sequential_api_case(stage: &StagePaths, temp_dir: &Path) -> Result<(), String> {
    let jpeg_path = temp_dir.join("sequential_api_422_ifast.jpg");

    let output = run_stage_command(
        stage,
        temp_dir,
        "cjpeg",
        vec![
            OsString::from("-sample"),
            OsString::from("2x1"),
            OsString::from("-dct"),
            OsString::from("fast"),
            OsString::from("-opt"),
            OsString::from("-outfile"),
            jpeg_path.clone().into_os_string(),
            stage
                .original_testimages
                .join("testorig.ppm")
                .into_os_string(),
        ],
    )?;
    if !output.status.success() {
        return Err(command_failure("cjpeg", &output));
    }

    let jpeg_bytes =
        fs::read(&jpeg_path).map_err(|error| format!("read {}: {error}", jpeg_path.display()))?;
    let image = decode_sequential_image(stage, &jpeg_bytes)?;
    let digest = ppm_md5(&image);
    if digest != "8dbc65323d62cca7c91ba02dd1cfa81d" {
        return Err(format!(
            "sequential ABI decode md5 mismatch: expected 8dbc65323d62cca7c91ba02dd1cfa81d, got {digest}"
        ));
    }

    Ok(())
}

fn run_buffered_image_case(stage: &StagePaths, temp_dir: &Path) -> Result<(), String> {
    let jpeg_path = temp_dir.join("buffered_prog.jpg");
    let reference_path = temp_dir.join("buffered_prog_reference.ppm");

    let output = run_stage_command(
        stage,
        temp_dir,
        "cjpeg",
        vec![
            OsString::from("-quality"),
            OsString::from("100"),
            OsString::from("-dct"),
            OsString::from("fast"),
            OsString::from("-scans"),
            stage.original_testimages.join("test.scan").into_os_string(),
            OsString::from("-outfile"),
            jpeg_path.clone().into_os_string(),
            stage
                .original_testimages
                .join("testorig.ppm")
                .into_os_string(),
        ],
    )?;
    if !output.status.success() {
        return Err(command_failure("cjpeg", &output));
    }

    let output = run_stage_command(
        stage,
        temp_dir,
        "djpeg",
        vec![
            OsString::from("-dct"),
            OsString::from("fast"),
            OsString::from("-ppm"),
            OsString::from("-outfile"),
            reference_path.clone().into_os_string(),
            jpeg_path.clone().into_os_string(),
        ],
    )?;
    if !output.status.success() {
        return Err(command_failure("djpeg", &output));
    }

    let jpeg_bytes =
        fs::read(&jpeg_path).map_err(|error| format!("read {}: {error}", jpeg_path.display()))?;
    let reference = read_ppm(&reference_path)?;
    let (first_pass, final_pass, passes) = decode_buffered_image_passes(stage, &jpeg_bytes)?;

    if passes < 2 {
        return Err(format!(
            "buffered-image decode completed in only {passes} pass(es)"
        ));
    }
    if first_pass == final_pass {
        return Err("buffered-image first pass unexpectedly matched final output".to_string());
    }
    if final_pass != reference {
        return Err("buffered-image final pass did not match normal decode output".to_string());
    }

    Ok(())
}

fn decode_buffered_image_passes(
    stage: &StagePaths,
    jpeg_bytes: &[u8],
) -> Result<(PpmImage, PpmImage, usize), String> {
    unsafe {
        let libjpeg = LoadedLibjpeg::open(&stage.stage_lib.join("libjpeg.so.8"))?;
        let mut cinfo = MaybeUninit::<jpeg_decompress_struct>::zeroed().assume_init();
        let mut err = MaybeUninit::<jpeg_error_mgr>::zeroed().assume_init();
        cinfo.err = (libjpeg.jpeg_std_error)(&mut err);
        (libjpeg.jpeg_create_decompress)(
            &mut cinfo,
            JPEG_LIB_VERSION,
            std::mem::size_of::<jpeg_decompress_struct>(),
        );

        let result = (|| -> Result<(PpmImage, PpmImage, usize), String> {
            (libjpeg.jpeg_mem_src)(&mut cinfo, jpeg_bytes.as_ptr(), jpeg_bytes.len() as _);
            let header_status = (libjpeg.jpeg_read_header)(&mut cinfo, TRUE);
            if header_status != JPEG_HEADER_OK {
                return Err(format!(
                    "jpeg_read_header returned unexpected status {header_status}"
                ));
            }
            if (libjpeg.jpeg_has_multiple_scans)(&mut cinfo) == 0 {
                return Err(
                    "progressive buffered-image input did not report multiple scans".into(),
                );
            }

            cinfo.buffered_image = TRUE;
            cinfo.dct_method = JDCT_IFAST;
            if (libjpeg.jpeg_start_decompress)(&mut cinfo) == 0 {
                return Err("jpeg_start_decompress suspended unexpectedly".into());
            }
            if cinfo.global_state != DSTATE_BUFIMAGE {
                return Err(format!(
                    "jpeg_start_decompress left unexpected state {}",
                    cinfo.global_state
                ));
            }

            let mut first_pass = None;
            let mut final_pass: Option<PpmImage>;
            let mut passes = 0usize;

            loop {
                loop {
                    let ret = (libjpeg.jpeg_consume_input)(&mut cinfo);
                    if ret != JPEG_ROW_COMPLETED && ret != JPEG_SCAN_COMPLETED {
                        if ret != JPEG_REACHED_SOS && ret != JPEG_REACHED_EOI {
                            return Err(format!("unexpected jpeg_consume_input return {ret}"));
                        }
                        break;
                    }
                }

                passes += 1;
                if passes > 64 {
                    return Err("buffered-image decode exceeded 64 output passes".into());
                }

                if (libjpeg.jpeg_start_output)(&mut cinfo, cinfo.input_scan_number) == 0 {
                    return Err("jpeg_start_output suspended unexpectedly".into());
                }
                if cinfo.global_state != DSTATE_SCANNING {
                    return Err(format!(
                        "jpeg_start_output left unexpected state {}",
                        cinfo.global_state
                    ));
                }

                let image = read_scanline_output(&libjpeg, &mut cinfo)?;
                if first_pass.is_none() {
                    first_pass = Some(image.clone());
                }
                final_pass = Some(image);

                if (libjpeg.jpeg_finish_output)(&mut cinfo) == 0 {
                    return Err("jpeg_finish_output suspended unexpectedly".into());
                }
                if cinfo.global_state != DSTATE_BUFIMAGE {
                    return Err(format!(
                        "jpeg_finish_output left unexpected state {}",
                        cinfo.global_state
                    ));
                }

                if (libjpeg.jpeg_input_complete)(&mut cinfo) != 0
                    && cinfo.input_scan_number == cinfo.output_scan_number
                {
                    break;
                }
            }

            if (libjpeg.jpeg_finish_decompress)(&mut cinfo) == 0 {
                return Err("jpeg_finish_decompress suspended unexpectedly".into());
            }

            Ok((
                first_pass.expect("buffered-image decode must produce a first pass"),
                final_pass.expect("buffered-image decode must produce a final pass"),
                passes,
            ))
        })();

        (libjpeg.jpeg_destroy_decompress)(&mut cinfo);
        result
    }
}

fn decode_sequential_image(stage: &StagePaths, jpeg_bytes: &[u8]) -> Result<PpmImage, String> {
    unsafe {
        let libjpeg = LoadedLibjpeg::open(&stage.stage_lib.join("libjpeg.so.8"))?;
        let mut cinfo = MaybeUninit::<jpeg_decompress_struct>::zeroed().assume_init();
        let mut err = MaybeUninit::<jpeg_error_mgr>::zeroed().assume_init();
        cinfo.err = (libjpeg.jpeg_std_error)(&mut err);
        (libjpeg.jpeg_create_decompress)(
            &mut cinfo,
            JPEG_LIB_VERSION,
            std::mem::size_of::<jpeg_decompress_struct>(),
        );

        let result = (|| -> Result<PpmImage, String> {
            (libjpeg.jpeg_mem_src)(&mut cinfo, jpeg_bytes.as_ptr(), jpeg_bytes.len() as _);
            let header_status = (libjpeg.jpeg_read_header)(&mut cinfo, TRUE);
            if header_status != JPEG_HEADER_OK {
                return Err(format!(
                    "jpeg_read_header returned unexpected status {header_status}"
                ));
            }
            if cinfo.global_state != DSTATE_READY {
                return Err(format!(
                    "jpeg_read_header left unexpected state {}",
                    cinfo.global_state
                ));
            }
            if (libjpeg.jpeg_has_multiple_scans)(&mut cinfo) != 0 {
                return Err("baseline input unexpectedly reported multiple scans".into());
            }
            if (libjpeg.jpeg_input_complete)(&mut cinfo) != FALSE {
                return Err("baseline input unexpectedly reported complete before output".into());
            }

            cinfo.dct_method = JDCT_IFAST;
            cinfo.do_fancy_upsampling = FALSE;
            if (libjpeg.jpeg_start_decompress)(&mut cinfo) == 0 {
                return Err("jpeg_start_decompress suspended unexpectedly".into());
            }
            if cinfo.global_state != DSTATE_SCANNING {
                return Err(format!(
                    "jpeg_start_decompress left unexpected state {}",
                    cinfo.global_state
                ));
            }
            if cinfo.output_scanline != 0 {
                return Err(format!(
                    "jpeg_start_decompress left unexpected output_scanline {}",
                    cinfo.output_scanline
                ));
            }

            let image = read_scanline_output(&libjpeg, &mut cinfo)?;
            if cinfo.output_scanline != cinfo.output_height {
                return Err(format!(
                    "sequential decode stopped at output_scanline {} of {}",
                    cinfo.output_scanline, cinfo.output_height
                ));
            }

            if (libjpeg.jpeg_finish_decompress)(&mut cinfo) == 0 {
                return Err("jpeg_finish_decompress suspended unexpectedly".into());
            }
            if cinfo.global_state != DSTATE_START {
                return Err(format!(
                    "jpeg_finish_decompress left unexpected state {}",
                    cinfo.global_state
                ));
            }
            if (libjpeg.jpeg_input_complete)(&mut cinfo) == 0 {
                return Err("baseline input did not report complete after finish".into());
            }

            Ok(image)
        })();

        (libjpeg.jpeg_destroy_decompress)(&mut cinfo);
        result
    }
}

fn read_scanline_output(
    libjpeg: &LoadedLibjpeg,
    cinfo: &mut jpeg_decompress_struct,
) -> Result<PpmImage, String> {
    let width = cinfo.output_width as usize;
    let height = cinfo.output_height as usize;
    let components = cinfo.output_components as usize;
    if width == 0 || height == 0 || components == 0 {
        return Err(format!(
            "invalid buffered-image dimensions {width}x{height}x{components}"
        ));
    }

    let row_stride = width
        .checked_mul(components)
        .ok_or_else(|| "buffered-image row stride overflow".to_string())?;
    let total_bytes = row_stride
        .checked_mul(height)
        .ok_or_else(|| "buffered-image output size overflow".to_string())?;
    let mut data = vec![0u8; total_bytes];
    let mut row = vec![0u8; row_stride];
    let mut scanlines = [row.as_mut_ptr()];
    let mut offset = 0usize;

    while cinfo.output_scanline < cinfo.output_height {
        let before = cinfo.output_scanline;
        let rows_read = unsafe { (libjpeg.jpeg_read_scanlines)(cinfo, scanlines.as_mut_ptr(), 1) };
        if rows_read == 0 {
            return Err(format!(
                "jpeg_read_scanlines returned 0 at output scanline {before}"
            ));
        }

        let bytes_read = row_stride
            .checked_mul(rows_read as usize)
            .ok_or_else(|| "buffered-image bytes_read overflow".to_string())?;
        let end = offset
            .checked_add(bytes_read)
            .ok_or_else(|| "buffered-image output offset overflow".to_string())?;
        data[offset..end].copy_from_slice(&row[..bytes_read]);
        offset = end;
    }

    Ok(PpmImage {
        width,
        height,
        maxval: 255,
        data,
    })
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
            args.push(
                temp_dir
                    .join(format!("{basename}_{prog_tag}_{sample_name}.jpg"))
                    .into_os_string(),
            );
            args.push(source.clone().into_os_string());

            let output = run_stage_command(stage, temp_dir, "cjpeg", args)?;
            if !output.status.success() {
                return Err(command_failure("cjpeg", &output));
            }
        }

        for nosmooth in NOSMOOTH {
            let ns_tag = if nosmooth.is_some() {
                "nosmooth"
            } else {
                "smooth"
            };
            for quant_args in QUANT_ARGS {
                let quant_tag = if quant_args.is_empty() {
                    "full"
                } else {
                    "quant256"
                };

                for (sample_name, _) in SAMPLES {
                    let jpeg_path =
                        temp_dir.join(format!("{basename}_{prog_tag}_{sample_name}.jpg"));
                    let full_path = temp_dir.join(format!(
                        "{basename}_{prog_tag}_{ns_tag}_{quant_tag}_{sample_name}_full.ppm"
                    ));
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

fn run_encode_transcode_api_rgb_islow_case(
    stage: &StagePaths,
    _temp_dir: &Path,
) -> Result<(), String> {
    let ppm = read_ppm(&stage.original_testimages.join("testorig.ppm"))?;
    let icc = fs::read(stage.original_testimages.join("test1.icc")).map_err(|error| {
        format!(
            "read {}: {error}",
            stage.original_testimages.join("test1.icc").display()
        )
    })?;
    let jpeg = encode_rgb_islow_via_abi(stage, &ppm, &icc)?;
    let digest = md5_bytes(&jpeg);
    if digest != "1d44a406f61da743b5fd31c0a9abdca3" {
        return Err(format!(
            "libjpeg ABI rgb/islow md5 mismatch: expected 1d44a406f61da743b5fd31c0a9abdca3, got {digest}"
        ));
    }
    Ok(())
}

fn encode_rgb_islow_via_abi(
    stage: &StagePaths,
    ppm: &PpmImage,
    icc_profile: &[u8],
) -> Result<Vec<u8>, String> {
    unsafe {
        if ppm.maxval != 255 {
            return Err(format!("unsupported PPM maxval {}", ppm.maxval));
        }

        let libjpeg = LoadedLibjpeg::open(&stage.stage_lib.join("libjpeg.so.8"))?;
        let jpeg_create_compress: JpegCreateCompressFn =
            load_symbol(libjpeg.handle, b"jpeg_CreateCompress\0")?;
        let jpeg_destroy_compress: JpegDestroyCompressFn =
            load_symbol(libjpeg.handle, b"jpeg_destroy_compress\0")?;
        let jpeg_mem_dest: JpegMemDestFn = load_symbol(libjpeg.handle, b"jpeg_mem_dest\0")?;
        let jpeg_set_defaults: JpegSetDefaultsFn =
            load_symbol(libjpeg.handle, b"jpeg_set_defaults\0")?;
        let jpeg_set_colorspace: JpegSetColorspaceFn =
            load_symbol(libjpeg.handle, b"jpeg_set_colorspace\0")?;
        let jpeg_start_compress: JpegStartCompressFn =
            load_symbol(libjpeg.handle, b"jpeg_start_compress\0")?;
        let jpeg_write_scanlines: JpegWriteScanlinesFn =
            load_symbol(libjpeg.handle, b"jpeg_write_scanlines\0")?;
        let jpeg_finish_compress: JpegFinishCompressFn =
            load_symbol(libjpeg.handle, b"jpeg_finish_compress\0")?;
        let jpeg_write_icc_profile: JpegWriteIccProfileFn =
            load_symbol(libjpeg.handle, b"jpeg_write_icc_profile\0")?;

        let mut cinfo = MaybeUninit::<jpeg_compress_struct>::zeroed().assume_init();
        let mut err = MaybeUninit::<jpeg_error_mgr>::zeroed().assume_init();
        let mut outbuffer: *mut u8 = std::ptr::null_mut();
        let mut outsize: ulong = 0;

        cinfo.err = (libjpeg.jpeg_std_error)(&mut err);
        jpeg_create_compress(
            &mut cinfo,
            JPEG_LIB_VERSION,
            std::mem::size_of::<jpeg_compress_struct>(),
        );

        let result = (|| -> Result<Vec<u8>, String> {
            let row_stride = ppm
                .width
                .checked_mul(3)
                .ok_or_else(|| "encode row stride overflow".to_string())?;

            jpeg_mem_dest(&mut cinfo, &mut outbuffer, &mut outsize);
            if cinfo.global_state != CSTATE_START {
                return Err(format!(
                    "jpeg_CreateCompress left unexpected state {}",
                    cinfo.global_state
                ));
            }

            cinfo.image_width = ppm.width as JDIMENSION;
            cinfo.image_height = ppm.height as JDIMENSION;
            cinfo.input_components = 3;
            cinfo.in_color_space = JCS_RGB;

            jpeg_set_defaults(&mut cinfo);
            jpeg_set_colorspace(&mut cinfo, JCS_RGB);
            cinfo.dct_method = JDCT_ISLOW;

            jpeg_start_compress(&mut cinfo, TRUE);
            if cinfo.global_state != CSTATE_SCANNING {
                return Err(format!(
                    "jpeg_start_compress left unexpected state {}",
                    cinfo.global_state
                ));
            }

            jpeg_write_icc_profile(
                &mut cinfo,
                icc_profile.as_ptr(),
                icc_profile.len() as ::core::ffi::c_uint,
            );

            while cinfo.next_scanline < cinfo.image_height {
                let offset = row_stride
                    .checked_mul(cinfo.next_scanline as usize)
                    .ok_or_else(|| "encode output offset overflow".to_string())?;
                let mut row = [ppm.data.as_ptr().add(offset) as *mut u8];
                let written = jpeg_write_scanlines(&mut cinfo, row.as_mut_ptr(), 1);
                if written != 1 {
                    return Err(format!(
                        "jpeg_write_scanlines wrote {written} row(s) at scanline {}",
                        cinfo.next_scanline
                    ));
                }
            }

            jpeg_finish_compress(&mut cinfo);
            if cinfo.global_state != CSTATE_START {
                return Err(format!(
                    "jpeg_finish_compress left unexpected state {}",
                    cinfo.global_state
                ));
            }

            let bytes = take_output_buffer(outbuffer, outsize)?;
            outbuffer = std::ptr::null_mut();
            Ok(bytes)
        })();

        if !outbuffer.is_null() {
            free(outbuffer.cast());
        }
        jpeg_destroy_compress(&mut cinfo);
        result
    }
}

fn run_encode_transcode_api_transform_crop_case(
    stage: &StagePaths,
    _temp_dir: &Path,
) -> Result<(), String> {
    let jpeg_bytes = fs::read(stage.original_testimages.join("testorig.jpg")).map_err(|error| {
        format!(
            "read {}: {error}",
            stage.original_testimages.join("testorig.jpg").display()
        )
    })?;
    let jpeg = transcode_crop_transpose_via_abi(stage, &jpeg_bytes)?;
    let digest = md5_bytes(&jpeg);
    if digest != "b4197f377e621c4e9b1d20471432610d" {
        return Err(format!(
            "libjpeg ABI transform md5 mismatch: expected b4197f377e621c4e9b1d20471432610d, got {digest}"
        ));
    }
    Ok(())
}

fn transcode_crop_transpose_via_abi(
    stage: &StagePaths,
    jpeg_bytes: &[u8],
) -> Result<Vec<u8>, String> {
    unsafe {
        let libjpeg = LoadedLibjpeg::open(&stage.stage_lib.join("libjpeg.so.8"))?;
        let jpeg_create_compress: JpegCreateCompressFn =
            load_symbol(libjpeg.handle, b"jpeg_CreateCompress\0")?;
        let jpeg_destroy_compress: JpegDestroyCompressFn =
            load_symbol(libjpeg.handle, b"jpeg_destroy_compress\0")?;
        let jpeg_mem_dest: JpegMemDestFn = load_symbol(libjpeg.handle, b"jpeg_mem_dest\0")?;
        let jpeg_read_coefficients: JpegReadCoefficientsFn =
            load_symbol(libjpeg.handle, b"jpeg_read_coefficients\0")?;
        let jpeg_copy_critical_parameters: JpegCopyCriticalParametersFn =
            load_symbol(libjpeg.handle, b"jpeg_copy_critical_parameters\0")?;
        let jpeg_write_coefficients: JpegWriteCoefficientsFn =
            load_symbol(libjpeg.handle, b"jpeg_write_coefficients\0")?;
        let jpeg_finish_compress: JpegFinishCompressFn =
            load_symbol(libjpeg.handle, b"jpeg_finish_compress\0")?;

        let mut srcinfo = MaybeUninit::<jpeg_decompress_struct>::zeroed().assume_init();
        let mut srcerr = MaybeUninit::<jpeg_error_mgr>::zeroed().assume_init();
        let mut dstinfo = MaybeUninit::<jpeg_compress_struct>::zeroed().assume_init();
        let mut dsterr = MaybeUninit::<jpeg_error_mgr>::zeroed().assume_init();
        let mut outbuffer: *mut u8 = std::ptr::null_mut();
        let mut outsize: ulong = 0;

        srcinfo.err = (libjpeg.jpeg_std_error)(&mut srcerr);
        (libjpeg.jpeg_create_decompress)(
            &mut srcinfo,
            JPEG_LIB_VERSION,
            std::mem::size_of::<jpeg_decompress_struct>(),
        );
        dstinfo.err = (libjpeg.jpeg_std_error)(&mut dsterr);
        jpeg_create_compress(
            &mut dstinfo,
            JPEG_LIB_VERSION,
            std::mem::size_of::<jpeg_compress_struct>(),
        );

        let result = (|| -> Result<Vec<u8>, String> {
            (libjpeg.jpeg_mem_src)(&mut srcinfo, jpeg_bytes.as_ptr(), jpeg_bytes.len() as _);
            transupp::jcopy_markers_setup(
                &mut srcinfo as *mut _ as transupp::j_decompress_ptr,
                JCOPYOPT_COMMENTS,
            );
            let header_status = (libjpeg.jpeg_read_header)(&mut srcinfo, TRUE);
            if header_status != JPEG_HEADER_OK {
                return Err(format!(
                    "jpeg_read_header returned unexpected status {header_status}"
                ));
            }

            if transupp::jtransform_perfect_transform(17, 17, 16, 16, JXFORM_ROT_90) != FALSE {
                return Err("jtransform_perfect_transform accepted an imperfect rotate".into());
            }
            if transupp::jtransform_perfect_transform(32, 32, 16, 16, JXFORM_ROT_90) == FALSE {
                return Err("jtransform_perfect_transform rejected a perfect rotate".into());
            }

            let mut transform = MaybeUninit::<jpeg_transform_info>::zeroed().assume_init();
            transform.transform = JXFORM_TRANSPOSE;
            transform.perfect = TRUE;
            transform.crop = TRUE;
            transform.crop_width = 120;
            transform.crop_width_set = JCROP_POS;
            transform.crop_height = 90;
            transform.crop_height_set = JCROP_POS;
            transform.crop_xoffset = 20;
            transform.crop_xoffset_set = JCROP_POS;
            transform.crop_yoffset = 50;
            transform.crop_yoffset_set = JCROP_POS;

            if transupp::jtransform_request_workspace(
                &mut srcinfo as *mut _ as transupp::j_decompress_ptr,
                &mut transform,
            ) == 0
            {
                return Err("jtransform_request_workspace rejected a valid transpose crop".into());
            }

            let src_coef_arrays = jpeg_read_coefficients(&mut srcinfo);
            if src_coef_arrays.is_null() {
                return Err("jpeg_read_coefficients returned null".into());
            }

            jpeg_copy_critical_parameters(&mut srcinfo, &mut dstinfo);
            let dst_coef_arrays = transupp::jtransform_adjust_parameters(
                &mut srcinfo as *mut _ as transupp::j_decompress_ptr,
                &mut dstinfo as *mut _ as transupp::j_compress_ptr,
                src_coef_arrays as *mut transupp::jvirt_barray_ptr,
                &mut transform,
            );
            if dst_coef_arrays.is_null() {
                return Err("jtransform_adjust_parameters returned null".into());
            }
            if transform.output_width != 124 || transform.output_height != 92 {
                return Err(format!(
                    "unexpected transform output dimensions {}x{}",
                    transform.output_width, transform.output_height
                ));
            }

            jpeg_mem_dest(&mut dstinfo, &mut outbuffer, &mut outsize);
            if dstinfo.global_state != CSTATE_START {
                return Err(format!(
                    "jpeg_copy_critical_parameters left unexpected dst state {}",
                    dstinfo.global_state
                ));
            }

            jpeg_write_coefficients(
                &mut dstinfo,
                dst_coef_arrays as *mut ffi_types::jvirt_barray_ptr,
            );
            if dstinfo.global_state != CSTATE_WRCOEFS {
                return Err(format!(
                    "jpeg_write_coefficients left unexpected dst state {}",
                    dstinfo.global_state
                ));
            }

            transupp::jcopy_markers_execute(
                &mut srcinfo as *mut _ as transupp::j_decompress_ptr,
                &mut dstinfo as *mut _ as transupp::j_compress_ptr,
                JCOPYOPT_COMMENTS,
            );
            transupp::jtransform_execute_transform(
                &mut srcinfo as *mut _ as transupp::j_decompress_ptr,
                &mut dstinfo as *mut _ as transupp::j_compress_ptr,
                src_coef_arrays as *mut transupp::jvirt_barray_ptr,
                &mut transform,
            );

            jpeg_finish_compress(&mut dstinfo);
            if dstinfo.global_state != CSTATE_START {
                return Err(format!(
                    "jpeg_finish_compress left unexpected dst state {}",
                    dstinfo.global_state
                ));
            }
            if (libjpeg.jpeg_finish_decompress)(&mut srcinfo) == 0 {
                return Err("jpeg_finish_decompress suspended unexpectedly".into());
            }

            let bytes = take_output_buffer(outbuffer, outsize)?;
            outbuffer = std::ptr::null_mut();
            Ok(bytes)
        })();

        if !outbuffer.is_null() {
            free(outbuffer.cast());
        }
        jpeg_destroy_compress(&mut dstinfo);
        (libjpeg.jpeg_destroy_decompress)(&mut srcinfo);
        result
    }
}

fn run_encode_transcode_symbol_surface_case(
    stage: &StagePaths,
    _temp_dir: &Path,
) -> Result<(), String> {
    unsafe {
        let path = stage.stage_lib.join("libjpeg.so.8");
        let path_c = CString::new(path.to_string_lossy().into_owned())
            .map_err(|error| format!("invalid dlopen path {}: {error}", path.display()))?;
        let handle = dlopen(path_c.as_ptr(), RTLD_NOW);
        if handle.is_null() {
            return Err(format!(
                "dlopen {} failed: {}",
                path.display(),
                dlerror_message()
            ));
        }

        let result = (|| -> Result<(), String> {
            for symbol in EXPECTED_COMMON_SYMBOLS
                .iter()
                .chain(EXPECTED_DECOMPRESS_SYMBOLS.iter())
                .chain(EXPECTED_COMPRESS_SYMBOLS.iter())
            {
                let symbol_c =
                    CString::new(*symbol).expect("static symbol names never contain NUL");
                if dlsym(handle, symbol_c.as_ptr()).is_null() {
                    return Err(format!(
                        "staged libjpeg is missing symbol {symbol}: {}",
                        dlerror_message()
                    ));
                }
            }
            Ok(())
        })();

        let _ = dlclose(handle);
        result
    }
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
        return Err(format!(
            "{} uses unsupported PPM maxval {maxval}",
            path.display()
        ));
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

fn crop_ppm(
    image: &PpmImage,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Result<PpmImage, String> {
    if x + width > image.width || y + height > image.height {
        return Err(format!(
            "crop {x},{y} {width}x{height} falls outside {}x{} image",
            image.width, image.height
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
    while *offset < bytes.len() && !bytes[*offset].is_ascii_whitespace() && bytes[*offset] != b'#' {
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
