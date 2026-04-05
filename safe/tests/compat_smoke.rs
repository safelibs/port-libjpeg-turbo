#![allow(clippy::all)]

use std::ffi::{c_char, c_int, c_void, CStr};
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::Mutex;

use ffi_types::{
    boolean, j_compress_ptr, j_decompress_ptr, jpeg_common_struct, jpeg_compress_struct,
    jpeg_decompress_struct, jpeg_error_mgr, jpeg_marker_struct, jpeg_marker_writer,
    CSTATE_SCANNING, CSTATE_START, DSTATE_READY, DSTATE_START, FALSE, FILE, JCS_EXT_RGB,
    JCS_EXT_RGBA, JMSG_LENGTH_MAX, JPEG_LIB_VERSION, JPOOL_IMAGE, J_MESSAGE_CODE, TRUE,
};

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[link(name = "jpeg")]
unsafe extern "C" {
    fn jpeg_std_error(err: *mut jpeg_error_mgr) -> *mut jpeg_error_mgr;
    fn jpeg_CreateCompress(cinfo: j_compress_ptr, version: c_int, structsize: usize);
    fn jpeg_CreateDecompress(cinfo: j_decompress_ptr, version: c_int, structsize: usize);
    fn jpeg_abort_compress(cinfo: j_compress_ptr);
    fn jpeg_abort_decompress(cinfo: j_decompress_ptr);
    fn jpeg_destroy_compress(cinfo: j_compress_ptr);
    fn jpeg_destroy_decompress(cinfo: j_decompress_ptr);
    fn jpeg_mem_src(cinfo: j_decompress_ptr, inbuffer: *const u8, insize: u64);
    fn jpeg_stdio_dest(cinfo: j_compress_ptr, outfile: *mut FILE);
    fn jpeg_stdio_src(cinfo: j_decompress_ptr, infile: *mut FILE);
    fn jpeg_mem_dest(cinfo: j_compress_ptr, outbuffer: *mut *mut u8, outsize: *mut u64);
    fn jpeg_write_icc_profile(cinfo: j_compress_ptr, icc_data_ptr: *const u8, icc_data_len: u32);
    fn jpeg_read_icc_profile(
        cinfo: j_decompress_ptr,
        icc_data_ptr: *mut *mut u8,
        icc_data_len: *mut u32,
    ) -> boolean;
}

#[link(name = "error_bridge", kind = "static")]
unsafe extern "C" {
    fn jpeg_rs_expect_mem_src_error(message: *mut c_char, message_len: usize) -> c_int;
    fn jpeg_rs_probe_default_colorspace(
        color_space: c_int,
        message: *mut c_char,
        message_len: usize,
    ) -> c_int;
    fn jpeg_rs_read_header_info(
        jpeg_buf: *const u8,
        jpeg_size: u64,
        progressive_mode: *mut c_int,
        arith_code: *mut c_int,
        restart_interval: *mut u32,
        message: *mut c_char,
        message_len: usize,
    ) -> c_int;
}

#[link(name = "c")]
unsafe extern "C" {
    fn tmpfile() -> *mut FILE;
    fn rewind(stream: *mut FILE);
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fwrite(ptr: *const c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fclose(stream: *mut FILE) -> c_int;
    fn free(ptr: *mut c_void);
}

#[link(name = "turbojpeg")]
unsafe extern "C" {
    fn tjInitCompress() -> *mut c_void;
    fn tjCompress2(
        handle: *mut c_void,
        srcBuf: *const u8,
        width: c_int,
        pitch: c_int,
        height: c_int,
        pixelFormat: c_int,
        jpegBuf: *mut *mut u8,
        jpegSize: *mut u64,
        jpegSubsamp: c_int,
        jpegQual: c_int,
        flags: c_int,
    ) -> c_int;
    fn tjDestroy(handle: *mut c_void) -> c_int;
    fn tjFree(buffer: *mut u8);
    fn tjGetErrorStr2(handle: *mut c_void) -> *mut c_char;
}

const TJSAMP_444: c_int = 0;
const TJPF_RGB: c_int = 0;

fn c_message(buffer: &[c_char]) -> String {
    unsafe { CStr::from_ptr(buffer.as_ptr()) }
        .to_string_lossy()
        .into_owned()
}

unsafe fn init_compress() -> (jpeg_compress_struct, jpeg_error_mgr) {
    let mut cinfo = MaybeUninit::<jpeg_compress_struct>::zeroed().assume_init();
    let mut err = MaybeUninit::<jpeg_error_mgr>::zeroed().assume_init();
    cinfo.err = jpeg_std_error(&mut err);
    jpeg_CreateCompress(
        &mut cinfo,
        JPEG_LIB_VERSION,
        std::mem::size_of::<jpeg_compress_struct>(),
    );
    (cinfo, err)
}

unsafe fn init_decompress() -> (jpeg_decompress_struct, jpeg_error_mgr) {
    let mut cinfo = MaybeUninit::<jpeg_decompress_struct>::zeroed().assume_init();
    let mut err = MaybeUninit::<jpeg_error_mgr>::zeroed().assume_init();
    cinfo.err = jpeg_std_error(&mut err);
    jpeg_CreateDecompress(
        &mut cinfo,
        JPEG_LIB_VERSION,
        std::mem::size_of::<jpeg_decompress_struct>(),
    );
    (cinfo, err)
}

#[test]
fn error_boundary_supports_setjmp_longjmp() {
    let mut message = [0 as c_char; JMSG_LENGTH_MAX];
    let result = unsafe { jpeg_rs_expect_mem_src_error(message.as_mut_ptr(), message.len()) };
    assert_eq!(result, 1);
    assert_eq!(c_message(&message), "Empty input file");
}

#[test]
fn jpeg_std_error_formats_messages() {
    let mut err = unsafe { MaybeUninit::<jpeg_error_mgr>::zeroed().assume_init() };
    let mut cinfo = unsafe { MaybeUninit::<jpeg_common_struct>::zeroed().assume_init() };
    unsafe {
        jpeg_std_error(&mut err);
        cinfo.err = &mut err;
        err.msg_code = J_MESSAGE_CODE::JERR_OUT_OF_MEMORY as c_int;
        err.msg_parm.i[0] = 7;
        let mut buffer = [0 as c_char; JMSG_LENGTH_MAX];
        (err.format_message.unwrap())(&mut cinfo, buffer.as_mut_ptr());
        assert_eq!(c_message(&buffer), "Insufficient memory (case 7)");
        assert_eq!(err.trace_level, 0);
        assert_eq!(err.last_jpeg_message, ffi_types::JMSG_LASTMSGCODE - 1);
    }
}

#[test]
fn jcstest_ported() {
    let mut message = [0 as c_char; JMSG_LENGTH_MAX];
    let rgb = unsafe {
        jpeg_rs_probe_default_colorspace(JCS_EXT_RGB, message.as_mut_ptr(), message.len())
    };
    assert_eq!(rgb, 1, "{}", c_message(&message));

    message.fill(0);
    let rgba = unsafe {
        jpeg_rs_probe_default_colorspace(JCS_EXT_RGBA, message.as_mut_ptr(), message.len())
    };
    assert_eq!(rgba, 1, "{}", c_message(&message));
}

#[test]
fn stdio_and_memory_managers_roundtrip() {
    unsafe {
        let (mut cinfo, _err) = init_compress();
        let file = tmpfile();
        assert!(!file.is_null());

        jpeg_stdio_dest(&mut cinfo, file);
        let dest = cinfo.dest;
        ((*dest).init_destination.unwrap())(&mut cinfo);
        let bytes = [1u8, 2, 3, 4];
        ptr::copy_nonoverlapping(bytes.as_ptr(), (*dest).next_output_byte, bytes.len());
        (*dest).next_output_byte = (*dest).next_output_byte.add(bytes.len());
        (*dest).free_in_buffer -= bytes.len();
        ((*dest).term_destination.unwrap())(&mut cinfo);

        rewind(file);
        let mut read_back = [0u8; 4];
        assert_eq!(
            fread(
                read_back.as_mut_ptr() as *mut c_void,
                1,
                read_back.len(),
                file
            ),
            4
        );
        assert_eq!(read_back, bytes);
        fclose(file);
        jpeg_destroy_compress(&mut cinfo);

        let (mut dinfo, _derr) = init_decompress();
        let src_file = tmpfile();
        assert!(!src_file.is_null());
        let jpeg_bytes = [0xFFu8, 0xD8, 0xFF, 0xD9];
        assert_eq!(
            fwrite(
                jpeg_bytes.as_ptr() as *const c_void,
                1,
                jpeg_bytes.len(),
                src_file
            ),
            jpeg_bytes.len()
        );
        rewind(src_file);
        jpeg_stdio_src(&mut dinfo, src_file);
        let src = dinfo.src;
        ((*src).init_source.unwrap())(&mut dinfo);
        assert_eq!(((*src).fill_input_buffer.unwrap())(&mut dinfo), 1);
        assert_eq!((*src).bytes_in_buffer, jpeg_bytes.len());
        ((*src).skip_input_data.unwrap())(&mut dinfo, 2);
        assert_eq!((*src).bytes_in_buffer, jpeg_bytes.len() - 2);
        assert_eq!(*(*src).next_input_byte, 0xFF);
        fclose(src_file);
        jpeg_destroy_decompress(&mut dinfo);

        let (mut mem_cinfo, _mem_err) = init_compress();
        let mut outbuffer: *mut u8 = ptr::null_mut();
        let mut outsize = 0u64;
        jpeg_mem_dest(&mut mem_cinfo, &mut outbuffer, &mut outsize);
        let mem_dest = mem_cinfo.dest;
        let payload = [9u8, 8, 7];
        ptr::copy_nonoverlapping(
            payload.as_ptr(),
            (*mem_dest).next_output_byte,
            payload.len(),
        );
        (*mem_dest).next_output_byte = (*mem_dest).next_output_byte.add(payload.len());
        (*mem_dest).free_in_buffer -= payload.len();
        ((*mem_dest).term_destination.unwrap())(&mut mem_cinfo);
        assert_eq!(outsize, payload.len() as u64);
        assert_eq!(
            std::slice::from_raw_parts(outbuffer, outsize as usize),
            payload
        );
        free(outbuffer as *mut c_void);
        jpeg_destroy_compress(&mut mem_cinfo);
    }
}

#[test]
fn abort_preserves_permanent_io_managers() {
    unsafe {
        let (mut cinfo, _err) = init_compress();
        let mut outbuffer: *mut u8 = ptr::null_mut();
        let mut outsize = 0u64;
        jpeg_mem_dest(&mut cinfo, &mut outbuffer, &mut outsize);
        let dest = cinfo.dest;
        assert!(!dest.is_null());

        jpeg_abort_compress(&mut cinfo);
        assert_eq!(cinfo.global_state, CSTATE_START);
        assert_eq!(cinfo.dest, dest);

        jpeg_mem_dest(&mut cinfo, &mut outbuffer, &mut outsize);
        assert_eq!(cinfo.dest, dest);
        free(outbuffer as *mut c_void);
        jpeg_destroy_compress(&mut cinfo);

        let (mut dinfo, _derr) = init_decompress();
        let jpeg_bytes = [0xFFu8, 0xD8, 0xFF, 0xD9];
        jpeg_mem_src(&mut dinfo, jpeg_bytes.as_ptr(), jpeg_bytes.len() as u64);
        let src = dinfo.src;
        assert!(!src.is_null());

        jpeg_abort_decompress(&mut dinfo);
        assert_eq!(dinfo.global_state, DSTATE_START);
        assert_eq!(dinfo.src, src);
        assert!(dinfo.marker_list.is_null());

        jpeg_mem_src(&mut dinfo, jpeg_bytes.as_ptr(), jpeg_bytes.len() as u64);
        assert_eq!(dinfo.src, src);
        jpeg_destroy_decompress(&mut dinfo);
    }
}

#[test]
fn virtual_sample_arrays_obey_prezero_and_pool_semantics() {
    unsafe {
        let (mut cinfo, _err) = init_compress();
        let mem = cinfo.mem;
        let request = (*mem).request_virt_sarray.unwrap();
        let realize = (*mem).realize_virt_arrays.unwrap();
        let access = (*mem).access_virt_sarray.unwrap();

        let array = request(
            &mut cinfo as *mut jpeg_compress_struct as *mut jpeg_common_struct,
            JPOOL_IMAGE,
            TRUE,
            4,
            3,
            2,
        );
        realize(&mut cinfo as *mut jpeg_compress_struct as *mut jpeg_common_struct);

        let rows = access(
            &mut cinfo as *mut jpeg_compress_struct as *mut jpeg_common_struct,
            array,
            0,
            2,
            TRUE,
        );
        assert_eq!(std::slice::from_raw_parts(*rows.add(0), 4), &[0, 0, 0, 0]);
        assert_eq!(std::slice::from_raw_parts(*rows.add(1), 4), &[0, 0, 0, 0]);

        *(*rows.add(0)).add(0) = 17;
        *(*rows.add(1)).add(3) = 29;

        let reread = access(
            &mut cinfo as *mut jpeg_compress_struct as *mut jpeg_common_struct,
            array,
            0,
            2,
            FALSE,
        );
        assert_eq!(std::slice::from_raw_parts(*reread.add(0), 4)[0], 17);
        assert_eq!(std::slice::from_raw_parts(*reread.add(1), 4)[3], 29);

        let tail = access(
            &mut cinfo as *mut jpeg_compress_struct as *mut jpeg_common_struct,
            array,
            2,
            1,
            TRUE,
        );
        assert_eq!(std::slice::from_raw_parts(*tail.add(0), 4), &[0, 0, 0, 0]);

        jpeg_abort_compress(&mut cinfo);
        assert_eq!(cinfo.global_state, CSTATE_START);
        jpeg_destroy_compress(&mut cinfo);
    }
}

struct MarkerCapture {
    current: Vec<u8>,
    markers: Vec<Vec<u8>>,
}

unsafe extern "C" fn capture_marker_header(cinfo: j_compress_ptr, marker: c_int, _datalen: u32) {
    let capture = &mut *((*cinfo).client_data as *mut MarkerCapture);
    capture.current.clear();
    capture.current.push(marker as u8);
}

unsafe extern "C" fn capture_marker_byte(cinfo: j_compress_ptr, val: c_int) {
    let capture = &mut *((*cinfo).client_data as *mut MarkerCapture);
    capture.current.push(val as u8);
}

#[test]
fn icc_helpers_work() {
    unsafe {
        let mut capture = MarkerCapture {
            current: Vec::new(),
            markers: Vec::new(),
        };
        let mut cinfo = MaybeUninit::<jpeg_compress_struct>::zeroed().assume_init();
        let mut marker = jpeg_marker_writer {
            write_file_header: None,
            write_frame_header: None,
            write_scan_header: None,
            write_file_trailer: None,
            write_tables_only: None,
            write_marker_header: Some(capture_marker_header),
            write_marker_byte: Some(capture_marker_byte),
        };
        cinfo.client_data = &mut capture as *mut MarkerCapture as *mut c_void;
        cinfo.marker = &mut marker;
        cinfo.global_state = CSTATE_SCANNING;

        let icc = b"sample-icc-profile";
        jpeg_write_icc_profile(&mut cinfo, icc.as_ptr(), icc.len() as u32);
        capture.markers.push(std::mem::take(&mut capture.current));
        assert_eq!(capture.markers.len(), 1);
        assert_eq!(capture.markers[0][0], (ffi_types::JPEG_APP0 + 2) as u8);
        assert_eq!(&capture.markers[0][1..13], b"ICC_PROFILE\0");

        let part1 = b"hello ";
        let part2 = b"world";
        let mut data1 = Vec::from(&b"ICC_PROFILE\0"[..]);
        data1.extend_from_slice(&[1, 2]);
        data1.extend_from_slice(part1);
        let mut data2 = Vec::from(&b"ICC_PROFILE\0"[..]);
        data2.extend_from_slice(&[2, 2]);
        data2.extend_from_slice(part2);

        let mut marker2 = jpeg_marker_struct {
            next: ptr::null_mut(),
            marker: (ffi_types::JPEG_APP0 + 2) as u8,
            original_length: data2.len() as u32,
            data_length: data2.len() as u32,
            data: data2.as_mut_ptr(),
        };
        let mut marker1 = jpeg_marker_struct {
            next: &mut marker2,
            marker: (ffi_types::JPEG_APP0 + 2) as u8,
            original_length: data1.len() as u32,
            data_length: data1.len() as u32,
            data: data1.as_mut_ptr(),
        };
        let mut dinfo = MaybeUninit::<jpeg_decompress_struct>::zeroed().assume_init();
        dinfo.global_state = DSTATE_READY;
        dinfo.marker_list = &mut marker1;
        let mut out: *mut u8 = ptr::null_mut();
        let mut outlen = 0u32;
        assert_eq!(
            jpeg_read_icc_profile(&mut dinfo, &mut out, &mut outlen),
            TRUE
        );
        assert_eq!(outlen, (part1.len() + part2.len()) as u32);
        assert_eq!(
            std::slice::from_raw_parts(out, outlen as usize),
            b"hello world"
        );
        free(out as *mut c_void);
    }
}

#[derive(Clone)]
struct EnvGuard {
    name: &'static str,
    value: Option<String>,
}

impl EnvGuard {
    fn capture(name: &'static str) -> Self {
        Self {
            name,
            value: std::env::var(name).ok(),
        }
    }

    fn restore(&self) {
        match &self.value {
            Some(value) => std::env::set_var(self.name, value),
            None => std::env::remove_var(self.name),
        }
    }
}

fn init_image() -> Vec<u8> {
    let width = 32usize;
    let height = 32usize;
    let mut src = vec![0u8; width * height * 3];
    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) * 3;
            src[index] = if y < height / 2 { 0 } else { 255 };
            src[index + 1] = if x < width / 2 { 64 } else { 192 };
            src[index + 2] = if ((x / 8 + y / 8) & 1) == 1 { 32 } else { 224 };
        }
    }
    src
}

fn try_compress_test_jpeg() -> Result<Vec<u8>, String> {
    unsafe {
        let handle = tjInitCompress();
        if handle.is_null() {
            return Err(CStr::from_ptr(tjGetErrorStr2(ptr::null_mut()))
                .to_string_lossy()
                .into_owned());
        }

        let src = init_image();
        let mut jpeg_buf: *mut u8 = ptr::null_mut();
        let mut jpeg_size = 0u64;
        let rc = tjCompress2(
            handle,
            src.as_ptr(),
            32,
            0,
            32,
            TJPF_RGB,
            &mut jpeg_buf,
            &mut jpeg_size,
            TJSAMP_444,
            75,
            0,
        );
        let result = if rc == -1 {
            Err(CStr::from_ptr(tjGetErrorStr2(handle))
                .to_string_lossy()
                .into_owned())
        } else {
            let bytes = std::slice::from_raw_parts(jpeg_buf, jpeg_size as usize).to_vec();
            tjFree(jpeg_buf);
            Ok(bytes)
        };
        let _ = tjDestroy(handle);
        result
    }
}

fn read_header_info(jpeg: &[u8]) -> Result<(i32, i32, u32), String> {
    let mut progressive = 0;
    let mut arith = 0;
    let mut restart = 0;
    let mut message = [0 as c_char; JMSG_LENGTH_MAX];
    let rc = unsafe {
        jpeg_rs_read_header_info(
            jpeg.as_ptr(),
            jpeg.len() as u64,
            &mut progressive,
            &mut arith,
            &mut restart,
            message.as_mut_ptr(),
            message.len(),
        )
    };
    if rc == 0 {
        Ok((progressive, arith, restart))
    } else {
        Err(c_message(&message))
    }
}

fn verify_header(
    expected_progressive: i32,
    expected_arith: i32,
    expected_restart: u32,
    unsupported_error: Option<&str>,
) {
    match try_compress_test_jpeg() {
        Err(error) => {
            if unsupported_error == Some(error.as_str()) {
                return;
            }
            panic!("TurboJPEG ERROR: {error}");
        }
        Ok(jpeg) => match read_header_info(&jpeg) {
            Ok((progressive, arith, restart)) => {
                assert_eq!(progressive, expected_progressive);
                assert_eq!(arith, expected_arith);
                assert_eq!(restart, expected_restart);
            }
            Err(error) => panic!("libjpeg ERROR: {error}"),
        },
    }
}

#[test]
fn strtest_ported() {
    let _guard = ENV_LOCK.lock().unwrap();
    let saved = [
        EnvGuard::capture("TJ_OPTIMIZE"),
        EnvGuard::capture("TJ_ARITHMETIC"),
        EnvGuard::capture("TJ_RESTART"),
        EnvGuard::capture("TJ_PROGRESSIVE"),
    ];

    let restore = || {
        for guard in &saved {
            guard.restore();
        }
    };

    std::env::remove_var("TJ_OPTIMIZE");
    std::env::remove_var("TJ_ARITHMETIC");
    std::env::remove_var("TJ_RESTART");
    std::env::remove_var("TJ_PROGRESSIVE");
    verify_header(0, 0, 0, None);

    let default = try_compress_test_jpeg().unwrap();
    std::env::set_var("TJ_OPTIMIZE", "1");
    let optimized = try_compress_test_jpeg().unwrap();
    assert!(optimized.len() < default.len());

    std::env::remove_var("TJ_OPTIMIZE");
    std::env::remove_var("TJ_ARITHMETIC");
    std::env::remove_var("TJ_PROGRESSIVE");
    std::env::set_var("TJ_RESTART", "8B");
    verify_header(0, 0, 8, None);

    std::env::remove_var("TJ_OPTIMIZE");
    std::env::remove_var("TJ_ARITHMETIC");
    std::env::remove_var("TJ_PROGRESSIVE");
    std::env::set_var("TJ_RESTART", "1");
    verify_header(0, 0, 4, None);

    std::env::remove_var("TJ_OPTIMIZE");
    std::env::remove_var("TJ_ARITHMETIC");
    std::env::remove_var("TJ_RESTART");
    std::env::set_var("TJ_PROGRESSIVE", "1");
    verify_header(
        1,
        0,
        0,
        Some("Requested feature was omitted at compile time"),
    );

    std::env::remove_var("TJ_OPTIMIZE");
    std::env::remove_var("TJ_PROGRESSIVE");
    std::env::remove_var("TJ_RESTART");
    std::env::set_var("TJ_ARITHMETIC", "1");
    verify_header(0, 1, 0, Some("Sorry, arithmetic coding is not implemented"));

    restore();
}
