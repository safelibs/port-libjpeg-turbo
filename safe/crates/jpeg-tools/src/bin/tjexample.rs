use std::{
    ffi::{c_char, c_int, c_ulong, CStr, CString},
    fs,
    path::Path,
    ptr, slice,
};

use jpeg_core::ported::turbojpeg::{
    tjutil,
    turbojpeg::{
        colorspace_name, scaled, subsamp_name, tjscalingfactor, TJFLAG_ACCURATEDCT, TJFLAG_FASTDCT,
        TJFLAG_FASTUPSAMPLE, TJPF_BGRX, TJPF_GRAY, TJPF_UNKNOWN, TJSAMP_444, TJSAMP_GRAY,
        TJXOPT_CROP, TJXOPT_GRAY, TJXOPT_TRIM, TJXOP_HFLIP, TJXOP_NONE, TJXOP_ROT180, TJXOP_ROT270,
        TJXOP_ROT90, TJXOP_TRANSPOSE, TJXOP_TRANSVERSE, TJXOP_VFLIP, TJ_PIXEL_SIZE,
    },
};
use libturbojpeg_abi::non_jni as tj;

const DEFAULT_SUBSAMP: c_int = TJSAMP_444;
const DEFAULT_QUALITY: c_int = 95;

fn main() {
    let _ = libjpeg_abi::common_exports::jpeg_std_error as *const ();
    let _ = libjpeg_abi::compress::jctrans::jpeg_write_coefficients as *const ();
    let _ = libjpeg_abi::transform::transupp::jtransform_execute_transform as *const ();
    std::process::exit(match run() {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("{error}");
            1
        }
    });
}

fn run() -> Result<(), String> {
    let args = std::env::args_os().collect::<Vec<_>>();
    let program = args
        .first()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "tjexample".to_string());
    if args.len() < 3 {
        usage(&program);
        return Err("missing required input/output paths".to_string());
    }

    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);
    let mut scaling_factor = tjscalingfactor { num: 1, denom: 1 };
    let mut out_subsamp = -1;
    let mut out_qual = -1;
    let mut xform = tj::tjtransform {
        r: tj::tjregion {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        },
        op: TJXOP_NONE,
        options: 0,
        data: ptr::null_mut(),
        customFilter: None,
    };
    let mut flags = 0;
    let mut index = 3usize;

    while index < args.len() {
        let option = args[index].to_string_lossy();
        match option.as_ref() {
            "-scale" | "-sc" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    usage(&program);
                    return Err("missing value for -scale".to_string());
                };
                let spec = value.to_string_lossy();
                scaling_factor = tjutil::parse_scaling_factor(&spec).ok_or_else(|| {
                    usage(&program);
                    format!("unsupported scaling factor: {spec}")
                })?;
            }
            "-subsamp" | "-su" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    usage(&program);
                    return Err("missing value for -subsamp".to_string());
                };
                let spec = value.to_string_lossy();
                out_subsamp = tjutil::parse_subsamp(&spec).ok_or_else(|| {
                    usage(&program);
                    format!("unsupported subsampling value: {spec}")
                })?;
            }
            "-q" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    usage(&program);
                    return Err("missing value for -q".to_string());
                };
                out_qual = value
                    .to_string_lossy()
                    .parse::<c_int>()
                    .map_err(|_| format!("invalid quality value: {}", value.to_string_lossy()))?;
                if !(1..=100).contains(&out_qual) {
                    usage(&program);
                    return Err(format!("quality out of range: {out_qual}"));
                }
            }
            "-grayscale" | "-g" => {
                xform.options |= TJXOPT_GRAY;
            }
            "-hflip" => xform.op = TJXOP_HFLIP,
            "-vflip" => xform.op = TJXOP_VFLIP,
            "-transpose" => xform.op = TJXOP_TRANSPOSE,
            "-transverse" => xform.op = TJXOP_TRANSVERSE,
            "-rot90" => xform.op = TJXOP_ROT90,
            "-rot180" => xform.op = TJXOP_ROT180,
            "-rot270" => xform.op = TJXOP_ROT270,
            "-custom" => xform.customFilter = Some(custom_filter),
            "-crop" | "-c" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    usage(&program);
                    return Err("missing value for -crop".to_string());
                };
                let spec = value.to_string_lossy();
                let (w, h, x, y) = parse_crop_spec(&spec).ok_or_else(|| {
                    usage(&program);
                    format!("invalid crop specification: {spec}")
                })?;
                xform.r.w = w;
                xform.r.h = h;
                xform.r.x = x;
                xform.r.y = y;
                xform.options |= TJXOPT_CROP;
            }
            "-fastupsample" => {
                println!("Using fast upsampling code");
                flags |= TJFLAG_FASTUPSAMPLE;
            }
            "-fastdct" => {
                println!("Using fastest DCT/IDCT algorithm");
                flags |= TJFLAG_FASTDCT;
            }
            "-accuratedct" => {
                println!("Using most accurate DCT/IDCT algorithm");
                flags |= TJFLAG_ACCURATEDCT;
            }
            _ => {
                usage(&program);
                return Err(format!("unsupported option: {option}"));
            }
        }
        index += 1;
    }

    let in_format = extension(input_path).ok_or_else(|| {
        usage(&program);
        "missing input file extension".to_string()
    })?;
    let out_format = extension(output_path).ok_or_else(|| {
        usage(&program);
        "missing output file extension".to_string()
    })?;

    let mut width = 0;
    let mut height = 0;
    let mut pixel_format = TJPF_UNKNOWN;
    let mut image: ImageBuffer;

    if is_jpeg_extension(&in_format) {
        let jpeg_bytes = fs::read(input_path)
            .map_err(|error| format!("opening input file {}: {error}", input_path.display()))?;
        if jpeg_bytes.is_empty() {
            return Err("determining input file size: input file contains no data".to_string());
        }

        let do_transform =
            xform.op != TJXOP_NONE || xform.options != 0 || xform.customFilter.is_some();
        let mut jpeg = OwnedTjBuffer::empty();
        let mut jpeg_size = c_ulong::try_from(jpeg_bytes.len())
            .map_err(|_| format!("input file too large: {}", input_path.display()))?;
        let handle = if do_transform {
            let handle =
                TjInstance::new(unsafe { tj::tjInitTransform() }, "initializing transformer")?;
            xform.options |= TJXOPT_TRIM;
            let mut dst_buf = ptr::null_mut();
            let mut dst_size = 0 as c_ulong;
            unsafe {
                if tj::tjTransform(
                    handle.raw(),
                    jpeg_bytes.as_ptr(),
                    jpeg_size,
                    1,
                    &mut dst_buf,
                    &mut dst_size,
                    &mut xform,
                    flags,
                ) < 0
                {
                    return Err(format!(
                        "transforming input image: {}",
                        tj_error(handle.raw())
                    ));
                }
            }
            jpeg = OwnedTjBuffer::from_raw(dst_buf);
            jpeg_size = dst_size;
            handle
        } else {
            TjInstance::new(
                unsafe { tj::tjInitDecompress() },
                "initializing decompressor",
            )?
        };

        let jpeg_ptr = if jpeg.is_empty() {
            jpeg_bytes.as_ptr()
        } else {
            jpeg.as_ptr()
        };
        let mut in_subsamp = 0;
        let mut in_colorspace = 0;
        unsafe {
            if tj::tjDecompressHeader3(
                handle.raw(),
                jpeg_ptr,
                jpeg_size,
                &mut width,
                &mut height,
                &mut in_subsamp,
                &mut in_colorspace,
            ) < 0
            {
                return Err(format!("reading JPEG header: {}", tj_error(handle.raw())));
            }
        }

        println!(
            "{} Image:  {} x {} pixels, {} subsampling, {} colorspace",
            if do_transform { "Transformed" } else { "Input" },
            width,
            height,
            subsamp_name(in_subsamp),
            colorspace_name(in_colorspace),
        );

        if is_jpeg_extension(&out_format)
            && do_transform
            && scaling_factor.num == 1
            && scaling_factor.denom == 1
            && out_subsamp < 0
            && out_qual < 0
        {
            fs::write(output_path, unsafe {
                slice::from_raw_parts(jpeg_ptr, jpeg_size as usize)
            })
            .map_err(|error| format!("writing output file {}: {error}", output_path.display()))?;
            return Ok(());
        }

        width = scaled(width, scaling_factor);
        height = scaled(height, scaling_factor);
        if out_subsamp < 0 {
            out_subsamp = in_subsamp;
        }
        pixel_format = TJPF_BGRX;
        let mut buffer = vec![0u8; checked_image_buffer_len(width, height, pixel_format)?];
        unsafe {
            if tj::tjDecompress2(
                handle.raw(),
                jpeg_ptr,
                jpeg_size,
                buffer.as_mut_ptr(),
                width,
                0,
                height,
                pixel_format,
                flags,
            ) < 0
            {
                return Err(format!(
                    "decompressing JPEG image: {}",
                    tj_error(handle.raw())
                ));
            }
        }
        image = ImageBuffer::Rust(buffer);
    } else {
        let input_c = c_string(input_path)?;
        let raw = unsafe {
            tj::tjLoadImage(
                input_c.as_ptr(),
                &mut width,
                1,
                &mut height,
                &mut pixel_format,
                0,
            )
        };
        if raw.is_null() {
            return Err(format!(
                "loading input image: {}",
                tj_error(ptr::null_mut())
            ));
        }
        image = ImageBuffer::Turbo(OwnedTjBuffer::from_raw(raw));
        if out_subsamp < 0 {
            out_subsamp = if pixel_format == TJPF_GRAY {
                TJSAMP_GRAY
            } else {
                DEFAULT_SUBSAMP
            };
        }
        println!("Input Image:  {width} x {height} pixels");
    }

    print!("Output Image ({out_format}):  {width} x {height} pixels");
    if is_jpeg_extension(&out_format) {
        let mut jpeg_buf = ptr::null_mut();
        let mut jpeg_size = 0 as c_ulong;
        if out_qual < 0 {
            out_qual = DEFAULT_QUALITY;
        }
        println!(
            ", {} subsampling, quality = {}",
            subsamp_name(out_subsamp),
            out_qual
        );
        let handle = TjInstance::new(unsafe { tj::tjInitCompress() }, "initializing compressor")?;
        unsafe {
            if tj::tjCompress2(
                handle.raw(),
                image.as_ptr(),
                width,
                0,
                height,
                pixel_format,
                &mut jpeg_buf,
                &mut jpeg_size,
                out_subsamp,
                out_qual,
                flags,
            ) < 0
            {
                return Err(format!("compressing image: {}", tj_error(handle.raw())));
            }
        }
        let jpeg = OwnedTjBuffer::from_raw(jpeg_buf);
        fs::write(output_path, unsafe {
            slice::from_raw_parts(jpeg.as_ptr(), jpeg_size as usize)
        })
        .map_err(|error| format!("writing output file {}: {error}", output_path.display()))?;
    } else {
        println!();
        let output_c = c_string(output_path)?;
        unsafe {
            if tj::tjSaveImage(
                output_c.as_ptr(),
                image.as_mut_ptr(),
                width,
                0,
                height,
                pixel_format,
                0,
            ) < 0
            {
                return Err(format!(
                    "saving output image: {}",
                    tj_error(ptr::null_mut())
                ));
            }
        }
    }

    Ok(())
}

unsafe extern "C" fn custom_filter(
    coeffs: *mut i16,
    array_region: tj::tjregion,
    _plane_region: tj::tjregion,
    _component_index: c_int,
    _transform_index: c_int,
    _transform: *mut tj::tjtransform,
) -> c_int {
    let count = (array_region.w * array_region.h).max(0) as usize;
    for index in 0..count {
        let value = coeffs.add(index);
        *value = -*value;
    }
    0
}

fn usage(program: &str) {
    println!();
    println!("USAGE: {program} <Input image> <Output image> [options]");
    println!();
    println!("Input and output images can be JPEG, BMP, or PBMPLUS (PPM/PGM.)");
    println!("Scaling factors: {}", tjutil::format_scaling_factor_list());
    println!("Compression options: -subsamp <444|422|420|gray>, -q <1-100>");
    println!("Transform options: -hflip, -vflip, -transpose, -transverse, -rot90, -rot180, -rot270, -grayscale, -crop WxH+X+Y, -custom");
    println!("General options: -scale M/N, -fastupsample, -fastdct, -accuratedct");
}

fn extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
}

fn is_jpeg_extension(extension: &str) -> bool {
    matches!(extension, "jpg" | "jpeg")
}

fn parse_crop_spec(spec: &str) -> Option<(c_int, c_int, c_int, c_int)> {
    let (size, offsets) = spec.split_once('+')?;
    let (x, y) = offsets.split_once('+')?;
    let (w, h) = size.split_once('x')?;
    let w = w.parse().ok()?;
    let h = h.parse().ok()?;
    let x = x.parse().ok()?;
    let y = y.parse().ok()?;
    (w > 0 && h > 0 && x >= 0 && y >= 0).then_some((w, h, x, y))
}

fn checked_image_buffer_len(
    width: c_int,
    height: c_int,
    pixel_format: c_int,
) -> Result<usize, String> {
    let pixel_size = *TJ_PIXEL_SIZE
        .get(pixel_format as usize)
        .ok_or_else(|| format!("unsupported pixel format: {pixel_format}"))?;
    usize::try_from(width)
        .ok()
        .and_then(|w| usize::try_from(height).ok().map(|h| (w, h)))
        .and_then(|(w, h)| w.checked_mul(h))
        .and_then(|pixels| pixels.checked_mul(pixel_size as usize))
        .ok_or_else(|| format!("image buffer overflow for {width}x{height}"))
}

fn c_string(path: &Path) -> Result<CString, String> {
    CString::new(path.to_string_lossy().into_owned())
        .map_err(|_| format!("path contains an interior NUL byte: {}", path.display()))
}

fn tj_error(handle: tj::tjhandle) -> String {
    unsafe {
        let message = tj::tjGetErrorStr2(handle);
        if message.is_null() {
            "unknown TurboJPEG error".to_string()
        } else {
            CStr::from_ptr(message as *const c_char)
                .to_string_lossy()
                .into_owned()
        }
    }
}

struct TjInstance(tj::tjhandle);

impl TjInstance {
    fn new(raw: tj::tjhandle, action: &str) -> Result<Self, String> {
        if raw.is_null() {
            Err(format!("{action}: {}", tj_error(ptr::null_mut())))
        } else {
            Ok(Self(raw))
        }
    }

    fn raw(&self) -> tj::tjhandle {
        self.0
    }
}

impl Drop for TjInstance {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                let _ = tj::tjDestroy(self.0);
            }
        }
    }
}

struct OwnedTjBuffer(*mut u8);

impl OwnedTjBuffer {
    fn empty() -> Self {
        Self(ptr::null_mut())
    }

    fn from_raw(raw: *mut u8) -> Self {
        Self(raw)
    }

    fn is_empty(&self) -> bool {
        self.0.is_null()
    }

    fn as_ptr(&self) -> *const u8 {
        self.0
    }
}

impl Drop for OwnedTjBuffer {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                tj::tjFree(self.0);
            }
        }
    }
}

enum ImageBuffer {
    Rust(Vec<u8>),
    Turbo(OwnedTjBuffer),
}

impl ImageBuffer {
    fn as_ptr(&self) -> *const u8 {
        match self {
            Self::Rust(buffer) => buffer.as_ptr(),
            Self::Turbo(buffer) => buffer.as_ptr(),
        }
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        match self {
            Self::Rust(buffer) => buffer.as_mut_ptr(),
            Self::Turbo(buffer) => buffer.0,
        }
    }
}
