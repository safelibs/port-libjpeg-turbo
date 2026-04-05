#![allow(clippy::all)]

use std::{
    ffi::{CStr, CString, OsString},
    fs,
    os::raw::{c_char, c_int, c_ulong, c_void},
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::{Mutex, OnceLock},
};

use jpeg_core::ported::turbojpeg::{
    tjutil,
    turbojpeg::{tjscalingfactor, TJPF_RGB, TJSAMP_420, TJSAMP_422, TJSAMP_444, TJSAMP_GRAY},
};
use libturbojpeg_abi::EXPECTED_NON_JNI_SYMBOLS;

static ENV_LOCK: Mutex<()> = Mutex::new(());
static STAGE_PATHS: OnceLock<Result<StagePaths, String>> = OnceLock::new();

const RTLD_NOW: c_int = 2;
const BMP_HEADER_BYTES: usize = 54;

struct StagePaths {
    repo_root: PathBuf,
    stage_bin: PathBuf,
    stage_lib: PathBuf,
    relink_dir: PathBuf,
}

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dlerror() -> *const c_char;
}

type TjInitCompressFn = unsafe extern "C" fn() -> *mut c_void;
type TjDestroyFn = unsafe extern "C" fn(handle: *mut c_void) -> c_int;
type TjFreeFn = unsafe extern "C" fn(buffer: *mut u8);
type TjCompress2Fn = unsafe extern "C" fn(
    handle: *mut c_void,
    src_buf: *const u8,
    width: c_int,
    pitch: c_int,
    height: c_int,
    pixel_format: c_int,
    jpeg_buf: *mut *mut u8,
    jpeg_size: *mut c_ulong,
    jpeg_subsamp: c_int,
    jpeg_qual: c_int,
    flags: c_int,
) -> c_int;
type TjGetScalingFactorsFn = unsafe extern "C" fn(num: *mut c_int) -> *mut tjscalingfactor;
type TjBufSizeYUV2Fn =
    unsafe extern "C" fn(width: c_int, align: c_int, height: c_int, subsamp: c_int) -> c_ulong;
type TjPlaneWidthFn = unsafe extern "C" fn(component: c_int, width: c_int, subsamp: c_int) -> c_int;
type TjPlaneHeightFn =
    unsafe extern "C" fn(component: c_int, height: c_int, subsamp: c_int) -> c_int;
type TjPlaneSizeYUVFn = unsafe extern "C" fn(
    component: c_int,
    width: c_int,
    stride: c_int,
    height: c_int,
    subsamp: c_int,
) -> c_ulong;
type TjGetErrorStr2Fn = unsafe extern "C" fn(handle: *mut c_void) -> *mut c_char;
type TjGetErrorCodeFn = unsafe extern "C" fn(handle: *mut c_void) -> c_int;

struct LoadedTurbojpeg {
    handle: *mut c_void,
}

impl LoadedTurbojpeg {
    fn open(path: &Path) -> Result<Self, String> {
        unsafe {
            let path_c = CString::new(path.to_string_lossy().into_owned())
                .map_err(|error| format!("invalid library path {}: {error}", path.display()))?;
            let handle = dlopen(path_c.as_ptr(), RTLD_NOW);
            if handle.is_null() {
                return Err(format!(
                    "dlopen {} failed: {}",
                    path.display(),
                    dlerror_message()
                ));
            }
            Ok(Self { handle })
        }
    }

    unsafe fn symbol<T>(&self, symbol: &'static [u8]) -> Result<T, String> {
        let symbol_name =
            CStr::from_bytes_with_nul(symbol).expect("static symbol names must be NUL-terminated");
        let ptr = dlsym(self.handle, symbol_name.as_ptr());
        if ptr.is_null() {
            return Err(format!(
                "dlsym({}) failed: {}",
                symbol_name.to_string_lossy(),
                dlerror_message()
            ));
        }
        Ok(std::mem::transmute_copy(&ptr))
    }
}

impl Drop for LoadedTurbojpeg {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                let _ = dlclose(self.handle);
            }
        }
    }
}

fn main_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.lock().expect("environment mutex poisoned")
}

unsafe fn dlerror_message() -> String {
    let message = dlerror();
    if message.is_null() {
        "unknown dlerror".to_string()
    } else {
        CStr::from_ptr(message).to_string_lossy().into_owned()
    }
}

fn stage_paths() -> Result<&'static StagePaths, String> {
    STAGE_PATHS
        .get_or_init(|| {
            let safe_root = safe::safe_root().to_path_buf();
            let repo_root = safe::repo_root().to_path_buf();

            let stage_status = Command::new("bash")
                .arg("scripts/stage-install.sh")
                .current_dir(&safe_root)
                .status()
                .map_err(|error| format!("failed to run stage-install.sh: {error}"))?;
            if !stage_status.success() {
                return Err(format!(
                    "stage-install.sh exited with status {stage_status}"
                ));
            }

            let relink_status = Command::new("bash")
                .arg("scripts/relink-original-objects.sh")
                .arg("--group")
                .arg("turbojpeg")
                .current_dir(&safe_root)
                .status()
                .map_err(|error| format!("failed to run relink-original-objects.sh: {error}"))?;
            if !relink_status.success() {
                return Err(format!(
                    "relink-original-objects.sh exited with status {relink_status}"
                ));
            }

            let stage_lib = find_stage_libdir()?;
            let relink_dir = find_relink_dir()?;
            Ok(StagePaths {
                repo_root,
                stage_bin: safe::stage_usr_root().join("bin"),
                stage_lib,
                relink_dir,
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
        if path.is_dir() && path.join("libturbojpeg.so.0").exists() {
            return Ok(path);
        }
    }
    Err(format!(
        "could not find staged libturbojpeg under {}",
        lib_root.display()
    ))
}

fn find_relink_dir() -> Result<PathBuf, String> {
    let root = safe::safe_root().join("target/original-relinked");
    for entry in
        fs::read_dir(&root).map_err(|error| format!("read_dir {}: {error}", root.display()))?
    {
        let entry = entry.map_err(|error| format!("read_dir entry {}: {error}", root.display()))?;
        let path = entry.path();
        if path.is_dir() && path.join("tjunittest").exists() {
            return Ok(path);
        }
    }
    Err(format!(
        "could not find relinked TurboJPEG tools under {}",
        root.display()
    ))
}

fn run_stage_command(
    stage: &StagePaths,
    temp_dir: &Path,
    tool: &str,
    args: &[OsString],
) -> Result<Output, String> {
    Command::new(stage.stage_bin.join(tool))
        .env("LD_LIBRARY_PATH", &stage.stage_lib)
        .current_dir(temp_dir)
        .args(args)
        .output()
        .map_err(|error| format!("failed to spawn staged {tool}: {error}"))
}

fn run_relinked_command(
    stage: &StagePaths,
    temp_dir: &Path,
    tool: &str,
    args: &[OsString],
) -> Result<Output, String> {
    Command::new(stage.relink_dir.join(tool))
        .env("LD_LIBRARY_PATH", &stage.stage_lib)
        .current_dir(temp_dir)
        .args(args)
        .output()
        .map_err(|error| format!("failed to spawn relinked {tool}: {error}"))
}

fn run_rust_release_command(
    stage: &StagePaths,
    temp_dir: &Path,
    tool: &str,
    args: &[OsString],
) -> Result<Output, String> {
    let path = safe::safe_root().join("target/release").join(tool);
    Command::new(&path)
        .env("LD_LIBRARY_PATH", &stage.stage_lib)
        .current_dir(temp_dir)
        .args(args)
        .output()
        .map_err(|error| {
            format!(
                "failed to spawn Rust release {tool} at {}: {error}",
                path.display()
            )
        })
}

fn command_failure(tool: &str, output: &Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    format!(
        "{tool} exited with status {}\nstdout:\n{}\nstderr:\n{}",
        output.status, stdout, stderr
    )
}

fn new_temp_dir(name: &str) -> Result<PathBuf, String> {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "libjpeg-turbo-phase6-{}-{name}",
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

fn md5_file(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    Ok(format!("{:x}", md5::compute(bytes)))
}

fn assert_success(tool: &str, output: Output) -> Result<(), String> {
    if output.status.success() {
        Ok(())
    } else {
        Err(command_failure(tool, &output))
    }
}

fn assert_files_identical(left: &Path, right: &Path) -> Result<(), String> {
    let left_bytes = fs::read(left).map_err(|error| format!("read {}: {error}", left.display()))?;
    let right_bytes =
        fs::read(right).map_err(|error| format!("read {}: {error}", right.display()))?;
    if left_bytes == right_bytes {
        Ok(())
    } else {
        Err(format!("{} and {} differ", left.display(), right.display()))
    }
}

fn assert_bmp_payload_identical(left: &Path, right: &Path) -> Result<(), String> {
    let left_bytes = fs::read(left).map_err(|error| format!("read {}: {error}", left.display()))?;
    let right_bytes =
        fs::read(right).map_err(|error| format!("read {}: {error}", right.display()))?;
    if left_bytes.len() < BMP_HEADER_BYTES || right_bytes.len() < BMP_HEADER_BYTES {
        return Err(format!(
            "BMP payload compare requires at least {BMP_HEADER_BYTES} bytes"
        ));
    }
    if left_bytes[BMP_HEADER_BYTES..] == right_bytes[BMP_HEADER_BYTES..] {
        Ok(())
    } else {
        Err(format!(
            "BMP payloads differ for {} and {}",
            left.display(),
            right.display()
        ))
    }
}

fn run_tjexample_subset<F>(stage: &StagePaths, temp_dir: &Path, runner: F) -> Result<(), String>
where
    F: Fn(&StagePaths, &Path, &[OsString]) -> Result<Output, String>,
{
    let image = stage
        .repo_root
        .join("original/testimages/vgl_6548_0026a.bmp");
    let ref_420_fast = temp_dir.join("ref_420_fast_cjpeg.jpg");
    let ref_420_default_bmp = temp_dir.join("ref_420_default_djpeg.bmp");
    let ref_420_nosmooth_bmp = temp_dir.join("ref_420_nosmooth_djpeg.bmp");
    let ref_420_half_bmp = temp_dir.join("ref_420_half_djpeg.bmp");
    let ref_gray_fast = temp_dir.join("ref_gray_fast_cjpeg.jpg");
    let ref_rot90 = temp_dir.join("ref_rot90_jpegtran.jpg");
    let ref_gray_rot90 = temp_dir.join("ref_gray_rot90_jpegtran.jpg");
    let actual_jpeg = temp_dir.join("tjexample_420_fast.jpg");
    let actual_gray_alias_jpeg = temp_dir.join("tjexample_gray_alias.jpg");
    let actual_default_bmp = temp_dir.join("tjexample_default.bmp");
    let actual_nosmooth_bmp = temp_dir.join("tjexample_nosmooth.bmp");
    let actual_full_scale_bmp = temp_dir.join("tjexample_full_scale.bmp");
    let actual_half_bmp = temp_dir.join("tjexample_half.bmp");
    let actual_rot90 = temp_dir.join("tjexample_rot90.jpg");
    let actual_gray_rot90 = temp_dir.join("tjexample_gray_rot90.jpg");

    assert_success(
        "cjpeg",
        run_stage_command(
            stage,
            temp_dir,
            "cjpeg",
            &[
                "-quality".into(),
                "95".into(),
                "-dct".into(),
                "fast".into(),
                "-sample".into(),
                "2x2".into(),
                "-outfile".into(),
                ref_420_fast.clone().into_os_string(),
                image.clone().into_os_string(),
            ],
        )?,
    )?;

    assert_success(
        "cjpeg",
        run_stage_command(
            stage,
            temp_dir,
            "cjpeg",
            &[
                "-quality".into(),
                "95".into(),
                "-dct".into(),
                "fast".into(),
                "-grayscale".into(),
                "-outfile".into(),
                ref_gray_fast.clone().into_os_string(),
                image.clone().into_os_string(),
            ],
        )?,
    )?;

    assert_success(
        "djpeg",
        run_stage_command(
            stage,
            temp_dir,
            "djpeg",
            &[
                "-rgb".into(),
                "-bmp".into(),
                "-outfile".into(),
                ref_420_default_bmp.clone().into_os_string(),
                ref_420_fast.clone().into_os_string(),
            ],
        )?,
    )?;

    assert_success(
        "djpeg",
        run_stage_command(
            stage,
            temp_dir,
            "djpeg",
            &[
                "-nosmooth".into(),
                "-rgb".into(),
                "-bmp".into(),
                "-outfile".into(),
                ref_420_nosmooth_bmp.clone().into_os_string(),
                ref_420_fast.clone().into_os_string(),
            ],
        )?,
    )?;

    assert_success(
        "djpeg",
        run_stage_command(
            stage,
            temp_dir,
            "djpeg",
            &[
                "-rgb".into(),
                "-bmp".into(),
                "-scale".into(),
                "1/2".into(),
                "-outfile".into(),
                ref_420_half_bmp.clone().into_os_string(),
                ref_420_fast.clone().into_os_string(),
            ],
        )?,
    )?;

    assert_success(
        "jpegtran",
        run_stage_command(
            stage,
            temp_dir,
            "jpegtran",
            &[
                "-crop".into(),
                "70x60+16+16".into(),
                "-rotate".into(),
                "90".into(),
                "-trim".into(),
                "-outfile".into(),
                ref_rot90.clone().into_os_string(),
                ref_420_fast.clone().into_os_string(),
            ],
        )?,
    )?;

    assert_success(
        "jpegtran",
        run_stage_command(
            stage,
            temp_dir,
            "jpegtran",
            &[
                "-crop".into(),
                "70x60+16+16".into(),
                "-rotate".into(),
                "90".into(),
                "-trim".into(),
                "-grayscale".into(),
                "-outfile".into(),
                ref_gray_rot90.clone().into_os_string(),
                ref_gray_fast.clone().into_os_string(),
            ],
        )?,
    )?;

    assert_success(
        "tjexample",
        runner(
            stage,
            temp_dir,
            &[
                image.clone().into_os_string(),
                actual_jpeg.clone().into_os_string(),
                "-q".into(),
                "95".into(),
                "-subsamp".into(),
                "420".into(),
                "-fastdct".into(),
            ],
        )?,
    )?;
    assert_files_identical(&actual_jpeg, &ref_420_fast)?;

    assert_success(
        "tjexample",
        runner(
            stage,
            temp_dir,
            &[
                image.clone().into_os_string(),
                actual_gray_alias_jpeg.clone().into_os_string(),
                "-q".into(),
                "95".into(),
                "-subsamp".into(),
                "g".into(),
                "-fastdct".into(),
            ],
        )?,
    )?;
    assert_files_identical(&actual_gray_alias_jpeg, &ref_gray_fast)?;

    assert_success(
        "tjexample",
        runner(
            stage,
            temp_dir,
            &[
                ref_420_fast.clone().into_os_string(),
                actual_default_bmp.clone().into_os_string(),
            ],
        )?,
    )?;
    assert_bmp_payload_identical(&actual_default_bmp, &ref_420_default_bmp)?;

    assert_success(
        "tjexample",
        runner(
            stage,
            temp_dir,
            &[
                ref_420_fast.clone().into_os_string(),
                actual_full_scale_bmp.clone().into_os_string(),
                "-scale".into(),
                "2/2".into(),
            ],
        )?,
    )?;
    assert_bmp_payload_identical(&actual_full_scale_bmp, &ref_420_default_bmp)?;

    assert_success(
        "tjexample",
        runner(
            stage,
            temp_dir,
            &[
                ref_420_fast.clone().into_os_string(),
                actual_nosmooth_bmp.clone().into_os_string(),
                "-fastupsample".into(),
            ],
        )?,
    )?;
    assert_bmp_payload_identical(&actual_nosmooth_bmp, &ref_420_nosmooth_bmp)?;

    assert_success(
        "tjexample",
        runner(
            stage,
            temp_dir,
            &[
                ref_420_fast.clone().into_os_string(),
                actual_half_bmp.clone().into_os_string(),
                "-scale".into(),
                "1/2".into(),
            ],
        )?,
    )?;
    assert_bmp_payload_identical(&actual_half_bmp, &ref_420_half_bmp)?;

    assert_success(
        "tjexample",
        runner(
            stage,
            temp_dir,
            &[
                ref_420_fast.clone().into_os_string(),
                actual_rot90.clone().into_os_string(),
                "-rot90".into(),
                "-crop".into(),
                "70x60+16+16".into(),
            ],
        )?,
    )?;
    assert_files_identical(&actual_rot90, &ref_rot90)?;

    assert_success(
        "tjexample",
        runner(
            stage,
            temp_dir,
            &[
                ref_420_fast.clone().into_os_string(),
                actual_gray_rot90.clone().into_os_string(),
                "-rot90".into(),
                "-grayscale".into(),
                "-crop".into(),
                "70x60+16+16".into(),
            ],
        )?,
    )?;
    assert_files_identical(&actual_gray_rot90, &ref_gray_rot90)?;

    Ok(())
}

#[test]
fn turbojpeg_option_parsers_match_upstream_cli_aliases() {
    let full_scale = tjutil::parse_scaling_factor("2/2").expect("2/2 scaling factor");
    assert_eq!(full_scale.num, 1);
    assert_eq!(full_scale.denom, 1);

    let half_scale = tjutil::parse_scaling_factor("2/4").expect("2/4 scaling factor");
    assert_eq!(half_scale.num, 1);
    assert_eq!(half_scale.denom, 2);

    assert_eq!(tjutil::parse_subsamp("g"), Some(TJSAMP_GRAY));
    assert_eq!(tjutil::parse_subsamp("GRAY"), Some(TJSAMP_GRAY));
    assert_eq!(tjutil::parse_subsamp("grayscale"), Some(TJSAMP_GRAY));
}

#[test]
fn packaged_tool_contracts_reference_committed_safe_sources() {
    let stage = stage_paths().expect("stage paths");

    for contract in jpeg_tools::PACKAGED_TOOL_CONTRACTS {
        assert!(
            stage.repo_root.join(contract.frontend_source).is_file(),
            "missing packaged frontend source for {}: {}",
            contract.binary_name,
            contract.frontend_source,
        );
        assert!(
            stage.repo_root.join(contract.manpage_source).is_file(),
            "missing packaged manpage source for {}: {}",
            contract.binary_name,
            contract.manpage_source,
        );
    }
}

#[test]
fn turbojpeg_exports_and_geometry_match_reference_tables() {
    let _guard = main_lock();
    let stage = stage_paths().expect("stage paths");

    let library = LoadedTurbojpeg::open(&stage.stage_lib.join("libturbojpeg.so.0"))
        .expect("load libturbojpeg");

    unsafe {
        for symbol in EXPECTED_NON_JNI_SYMBOLS {
            let mut name = symbol.as_bytes().to_vec();
            name.push(0);
            let c_name = CStr::from_bytes_with_nul(&name).expect("NUL-terminated symbol");
            let ptr = dlsym(library.handle, c_name.as_ptr());
            assert!(
                !ptr.is_null(),
                "missing staged TurboJPEG export {}: {}",
                symbol,
                dlerror_message()
            );
        }

        let tj_get_scaling_factors: TjGetScalingFactorsFn = library
            .symbol(b"tjGetScalingFactors\0")
            .expect("tjGetScalingFactors");
        let tj_buf_size_yuv2: TjBufSizeYUV2Fn =
            library.symbol(b"tjBufSizeYUV2\0").expect("tjBufSizeYUV2");
        let tj_plane_width: TjPlaneWidthFn =
            library.symbol(b"tjPlaneWidth\0").expect("tjPlaneWidth");
        let tj_plane_height: TjPlaneHeightFn =
            library.symbol(b"tjPlaneHeight\0").expect("tjPlaneHeight");
        let tj_plane_size_yuv: TjPlaneSizeYUVFn =
            library.symbol(b"tjPlaneSizeYUV\0").expect("tjPlaneSizeYUV");
        let tj_init_compress: TjInitCompressFn =
            library.symbol(b"tjInitCompress\0").expect("tjInitCompress");
        let tj_destroy: TjDestroyFn = library.symbol(b"tjDestroy\0").expect("tjDestroy");
        let tj_free: TjFreeFn = library.symbol(b"tjFree\0").expect("tjFree");
        let tj_compress2: TjCompress2Fn = library.symbol(b"tjCompress2\0").expect("tjCompress2");
        let tj_get_error_str2: TjGetErrorStr2Fn =
            library.symbol(b"tjGetErrorStr2\0").expect("tjGetErrorStr2");
        let tj_get_error_code: TjGetErrorCodeFn =
            library.symbol(b"tjGetErrorCode\0").expect("tjGetErrorCode");

        let mut num_scaling_factors = 0;
        let scaling_factors = tj_get_scaling_factors(&mut num_scaling_factors);
        assert!(!scaling_factors.is_null());
        assert_eq!(num_scaling_factors, 16);
        let scaling_slice =
            std::slice::from_raw_parts(scaling_factors, num_scaling_factors as usize);
        let actual_scaling_factors = scaling_slice
            .iter()
            .map(|factor| (factor.num, factor.denom))
            .collect::<Vec<_>>();
        assert_eq!(
            actual_scaling_factors,
            vec![
                (2, 1),
                (15, 8),
                (7, 4),
                (13, 8),
                (3, 2),
                (11, 8),
                (5, 4),
                (9, 8),
                (1, 1),
                (7, 8),
                (3, 4),
                (5, 8),
                (1, 2),
                (3, 8),
                (1, 4),
                (1, 8),
            ]
        );
        let timg_guard = *scaling_factors.add(num_scaling_factors as usize);
        assert_eq!((timg_guard.num, timg_guard.denom), (1, 8));

        for &(width, height, align, subsamp) in &[
            (35, 39, 1, TJSAMP_444),
            (35, 39, 4, TJSAMP_420),
            (17, 19, 4, TJSAMP_422),
            (63, 5, 8, TJSAMP_GRAY),
        ] {
            let expected_size =
                tjutil::yuv_size(width, align, height, subsamp).expect("reference yuv size");
            assert_eq!(
                tj_buf_size_yuv2(width, align, height, subsamp),
                expected_size,
                "tjBufSizeYUV2 mismatch for {width}x{height} align={align} subsamp={subsamp}",
            );

            let plane_count = tjutil::plane_count(subsamp);
            for component in 0..plane_count {
                let expected_width =
                    tjutil::plane_width(component, width, subsamp).expect("reference plane width");
                let expected_height = tjutil::plane_height(component, height, subsamp)
                    .expect("reference plane height");
                let stride = if component == 0 {
                    0
                } else {
                    expected_width + component
                };
                let expected_plane_size =
                    tjutil::plane_size(component, width, stride, height, subsamp)
                        .expect("reference plane size");

                assert_eq!(tj_plane_width(component, width, subsamp), expected_width);
                assert_eq!(tj_plane_height(component, height, subsamp), expected_height);
                assert_eq!(
                    tj_plane_size_yuv(component, width, stride, height, subsamp),
                    expected_plane_size,
                );
            }
        }

        let handle = tj_init_compress();
        assert!(!handle.is_null(), "tjInitCompress returned NULL");

        let width = 16;
        let height = 16;
        let mut src = vec![0u8; (width * height * 3) as usize];
        for (index, byte) in src.iter_mut().enumerate() {
            *byte = (index as u8).wrapping_mul(13);
        }
        let mut jpeg_buf = std::ptr::null_mut();
        let mut jpeg_size = 0 as c_ulong;
        let rc = tj_compress2(
            handle,
            src.as_ptr(),
            width,
            0,
            height,
            TJPF_RGB,
            &mut jpeg_buf,
            &mut jpeg_size,
            TJSAMP_444,
            90,
            0,
        );
        if rc != 0 {
            let message = CStr::from_ptr(tj_get_error_str2(handle))
                .to_string_lossy()
                .into_owned();
            let error_code = tj_get_error_code(handle);
            panic!("tjCompress2 failed ({error_code}): {message}");
        }
        assert!(jpeg_size > 0);
        tj_free(jpeg_buf);
        assert_eq!(tj_destroy(handle), 0);
    }
}

#[test]
fn relinked_tjunittest_variants_pass() {
    let _guard = main_lock();
    let stage = stage_paths().expect("stage paths");
    let temp_dir = new_temp_dir("tjunittest").expect("temp dir");

    for args in [
        Vec::<OsString>::new(),
        vec!["-alloc".into()],
        vec!["-yuv".into()],
        vec!["-yuv".into(), "-alloc".into()],
        vec!["-yuv".into(), "-noyuvpad".into()],
        vec!["-bmp".into()],
    ] {
        let output =
            run_relinked_command(&stage, &temp_dir, "tjunittest", &args).expect("spawn tjunittest");
        assert_success("tjunittest", output).expect("tjunittest variant");
    }
}

#[test]
fn tjbench_tile_regressions_match_upstream_md5s() {
    let _guard = main_lock();
    let stage = stage_paths().expect("stage paths");
    let temp_dir = new_temp_dir("tjbench-tile").expect("temp dir");

    fs::copy(
        stage.repo_root.join("original/testimages/testorig.ppm"),
        temp_dir.join("testout_tile.ppm"),
    )
    .expect("copy testout_tile.ppm");
    fs::copy(
        stage.repo_root.join("original/testimages/testorig.ppm"),
        temp_dir.join("testout_tilem.ppm"),
    )
    .expect("copy testout_tilem.ppm");

    let output = run_stage_command(
        &stage,
        &temp_dir,
        "tjbench",
        &[
            "testout_tile.ppm".into(),
            "95".into(),
            "-rgb".into(),
            "-quiet".into(),
            "-tile".into(),
            "-benchtime".into(),
            "0.01".into(),
            "-warmup".into(),
            "0".into(),
        ],
    )
    .expect("spawn tjbench tile");
    assert_success("tjbench", output).expect("tjbench tile");

    let output = run_stage_command(
        &stage,
        &temp_dir,
        "tjbench",
        &[
            "testout_tilem.ppm".into(),
            "95".into(),
            "-rgb".into(),
            "-fastupsample".into(),
            "-quiet".into(),
            "-tile".into(),
            "-benchtime".into(),
            "0.01".into(),
            "-warmup".into(),
            "0".into(),
        ],
    )
    .expect("spawn tjbench tilem");
    assert_success("tjbench", output).expect("tjbench tilem");

    for (file, expected_md5) in [
        (
            "testout_tile_GRAY_Q95_8x8.ppm",
            "89d3ca21213d9d864b50b4e4e7de4ca6",
        ),
        (
            "testout_tile_GRAY_Q95_16x16.ppm",
            "89d3ca21213d9d864b50b4e4e7de4ca6",
        ),
        (
            "testout_tile_GRAY_Q95_32x32.ppm",
            "89d3ca21213d9d864b50b4e4e7de4ca6",
        ),
        (
            "testout_tile_GRAY_Q95_64x64.ppm",
            "89d3ca21213d9d864b50b4e4e7de4ca6",
        ),
        (
            "testout_tile_GRAY_Q95_128x128.ppm",
            "89d3ca21213d9d864b50b4e4e7de4ca6",
        ),
        (
            "testout_tile_420_Q95_8x8.ppm",
            "847fceab15c5b7b911cb986cf0f71de3",
        ),
        (
            "testout_tile_420_Q95_16x16.ppm",
            "ca45552a93687e078f7137cc4126a7b0",
        ),
        (
            "testout_tile_420_Q95_32x32.ppm",
            "d8676f1d6b68df358353bba9844f4a00",
        ),
        (
            "testout_tile_420_Q95_64x64.ppm",
            "4e4c1a3d7ea4bace4f868bcbe83b7050",
        ),
        (
            "testout_tile_420_Q95_128x128.ppm",
            "f24c3429c52265832beab9df72a0ceae",
        ),
        (
            "testout_tile_422_Q95_8x8.ppm",
            "d83dacd9fc73b0a6f10c09acad64eb1e",
        ),
        (
            "testout_tile_422_Q95_16x16.ppm",
            "35077fb610d72dd743b1eb0cbcfe10fb",
        ),
        (
            "testout_tile_422_Q95_32x32.ppm",
            "e6902ed8a449ecc0f0d6f2bf945f65f7",
        ),
        (
            "testout_tile_422_Q95_64x64.ppm",
            "2b4502a8f316cedbde1da7bce3d2231e",
        ),
        (
            "testout_tile_422_Q95_128x128.ppm",
            "f0b5617d578f5e13c8eee215d64d4877",
        ),
        (
            "testout_tile_444_Q95_8x8.ppm",
            "7964e41e67cfb8d0a587c0aa4798f9c3",
        ),
        (
            "testout_tile_444_Q95_16x16.ppm",
            "7964e41e67cfb8d0a587c0aa4798f9c3",
        ),
        (
            "testout_tile_444_Q95_32x32.ppm",
            "7964e41e67cfb8d0a587c0aa4798f9c3",
        ),
        (
            "testout_tile_444_Q95_64x64.ppm",
            "7964e41e67cfb8d0a587c0aa4798f9c3",
        ),
        (
            "testout_tile_444_Q95_128x128.ppm",
            "7964e41e67cfb8d0a587c0aa4798f9c3",
        ),
        (
            "testout_tilem_420_Q95_8x8.ppm",
            "bc25320e1f4c31ce2e610e43e9fd173c",
        ),
        (
            "testout_tilem_420_Q95_16x16.ppm",
            "75ffdf14602258c5c189522af57fa605",
        ),
        (
            "testout_tilem_420_Q95_32x32.ppm",
            "75ffdf14602258c5c189522af57fa605",
        ),
        (
            "testout_tilem_420_Q95_64x64.ppm",
            "75ffdf14602258c5c189522af57fa605",
        ),
        (
            "testout_tilem_420_Q95_128x128.ppm",
            "75ffdf14602258c5c189522af57fa605",
        ),
        (
            "testout_tilem_422_Q95_8x8.ppm",
            "828941d7f41cd6283abd6beffb7fd51d",
        ),
        (
            "testout_tilem_422_Q95_16x16.ppm",
            "e877ae1324c4a280b95376f7f018172f",
        ),
        (
            "testout_tilem_422_Q95_32x32.ppm",
            "e877ae1324c4a280b95376f7f018172f",
        ),
        (
            "testout_tilem_422_Q95_64x64.ppm",
            "e877ae1324c4a280b95376f7f018172f",
        ),
        (
            "testout_tilem_422_Q95_128x128.ppm",
            "e877ae1324c4a280b95376f7f018172f",
        ),
    ] {
        let actual = md5_file(&temp_dir.join(file)).expect("md5");
        assert_eq!(actual, expected_md5, "{file}");
    }
}

#[test]
fn rust_tjexample_shell_contract_subset_passes() {
    let _guard = main_lock();
    let stage = stage_paths().expect("stage paths");
    let temp_dir = new_temp_dir("rust-tjexample").expect("temp dir");
    run_tjexample_subset(&stage, &temp_dir, |stage, temp_dir, args| {
        run_rust_release_command(stage, temp_dir, "tjexample", args)
    })
    .expect("Rust tjexample subset");
}

#[test]
fn relinked_tjexample_shell_contract_subset_passes() {
    let _guard = main_lock();
    let stage = stage_paths().expect("stage paths");
    let temp_dir = new_temp_dir("relinked-tjexample").expect("temp dir");
    run_tjexample_subset(&stage, &temp_dir, |stage, temp_dir, args| {
        run_relinked_command(stage, temp_dir, "tjexample", args)
    })
    .expect("relinked tjexample subset");
}
