use core::ffi::c_char;

use ffi_types::{
    boolean, int, j_common_ptr, j_decompress_ptr, jpeg_common_struct, jpeg_error_mgr, CSTATE_START,
    DSTATE_START, FALSE, JHUFF_TBL, JMSG_LASTMSGCODE, JMSG_LENGTH_MAX, JPEG_STD_MESSAGE_TABLE_LEN,
    JPOOL_NUMPOOLS, JPOOL_PERMANENT, JQUANT_TBL, J_MESSAGE_CODE,
};

use crate::common::{memory, registry};

const EXIT_FAILURE: int = 1;
const JMSG_SCAN_LIMIT: int = JMSG_LASTMSGCODE;

#[no_mangle]
pub static mut jpeg_rs_message_table: [*const c_char; 1] =
    [cstr(b"JPEG image has more than %d scans\0")];

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

#[no_mangle]
pub static mut jpeg_std_message_table: [*const c_char; JPEG_STD_MESSAGE_TABLE_LEN] = [
    cstr(b"Bogus message code %d\0"),
    cstr(b"ALIGN_TYPE is wrong, please fix\0"),
    cstr(b"MAX_ALLOC_CHUNK is wrong, please fix\0"),
    cstr(b"Bogus buffer control mode\0"),
    cstr(b"Invalid component ID %d in SOS\0"),
    cstr(b"Invalid crop request\0"),
    cstr(b"DCT coefficient out of range\0"),
    cstr(b"IDCT output block size %d not supported\0"),
    cstr(b"Component index %d: mismatching sampling ratio %d:%d, %d:%d, %c\0"),
    cstr(b"Bogus Huffman table definition\0"),
    cstr(b"Bogus input colorspace\0"),
    cstr(b"Bogus JPEG colorspace\0"),
    cstr(b"Bogus marker length\0"),
    cstr(b"Wrong JPEG library version: library is %d, caller expects %d\0"),
    cstr(b"Sampling factors too large for interleaved scan\0"),
    cstr(b"Invalid memory pool code %d\0"),
    cstr(b"Unsupported JPEG data precision %d\0"),
    cstr(b"Invalid progressive parameters Ss=%d Se=%d Ah=%d Al=%d\0"),
    cstr(b"Invalid progressive parameters at scan script entry %d\0"),
    cstr(b"Bogus sampling factors\0"),
    cstr(b"Invalid scan script at entry %d\0"),
    cstr(b"Improper call to JPEG library in state %d\0"),
    cstr(b"JPEG parameter struct mismatch: library thinks size is %u, caller expects %u\0"),
    cstr(b"Bogus virtual array access\0"),
    cstr(b"Buffer passed to JPEG library is too small\0"),
    cstr(b"Suspension not allowed here\0"),
    cstr(b"CCIR601 sampling not implemented yet\0"),
    cstr(b"Too many color components: %d, max %d\0"),
    cstr(b"Unsupported color conversion request\0"),
    cstr(b"Bogus DAC index %d\0"),
    cstr(b"Bogus DAC value 0x%x\0"),
    cstr(b"Bogus DHT index %d\0"),
    cstr(b"Bogus DQT index %d\0"),
    cstr(b"Empty JPEG image (DNL not supported)\0"),
    cstr(b"Read from EMS failed\0"),
    cstr(b"Write to EMS failed\0"),
    cstr(b"Didn't expect more than one scan\0"),
    cstr(b"Input file read error\0"),
    cstr(b"Output file write error --- out of disk space?\0"),
    cstr(b"Fractional sampling not implemented yet\0"),
    cstr(b"Huffman code size table overflow\0"),
    cstr(b"Missing Huffman code table entry\0"),
    cstr(b"Maximum supported image dimension is %u pixels\0"),
    cstr(b"Empty input file\0"),
    cstr(b"Premature end of input file\0"),
    cstr(b"Cannot transcode due to multiple use of quantization table %d\0"),
    cstr(b"Scan script does not transmit all data\0"),
    cstr(b"Invalid color quantization mode change\0"),
    cstr(b"Requested features are incompatible\0"),
    cstr(b"Requested feature was omitted at compile time\0"),
    cstr(b"Arithmetic table 0x%02x was not defined\0"),
    cstr(b"Backing store not supported\0"),
    cstr(b"Huffman table 0x%02x was not defined\0"),
    cstr(b"JPEG datastream contains no image\0"),
    cstr(b"Quantization table 0x%02x was not defined\0"),
    cstr(b"Not a JPEG file: starts with 0x%02x 0x%02x\0"),
    cstr(b"Insufficient memory (case %d)\0"),
    cstr(b"Cannot quantize more than %d color components\0"),
    cstr(b"Cannot quantize to fewer than %d colors\0"),
    cstr(b"Cannot quantize to more than %d colors\0"),
    cstr(b"Invalid JPEG file structure: two SOF markers\0"),
    cstr(b"Invalid JPEG file structure: missing SOS marker\0"),
    cstr(b"Unsupported JPEG process: SOF type 0x%02x\0"),
    cstr(b"Invalid JPEG file structure: two SOI markers\0"),
    cstr(b"Invalid JPEG file structure: SOS before SOF\0"),
    cstr(b"Failed to create temporary file %s\0"),
    cstr(b"Read failed on temporary file\0"),
    cstr(b"Seek failed on temporary file\0"),
    cstr(b"Write failed on temporary file --- out of disk space?\0"),
    cstr(b"Application transferred too few scanlines\0"),
    cstr(b"Unsupported marker type 0x%02x\0"),
    cstr(b"Virtual array controller messed up\0"),
    cstr(b"Image too wide for this implementation\0"),
    cstr(b"Read from XMS failed\0"),
    cstr(b"Write to XMS failed\0"),
    cstr(b"Copyright (C) 1991-2023 The libjpeg-turbo Project and many others\0"),
    cstr(b"8d  15-Jan-2012\0"),
    cstr(b"Caution: quantization tables are too coarse for baseline JPEG\0"),
    cstr(b"Adobe APP14 marker: version %d, flags 0x%04x 0x%04x, transform %d\0"),
    cstr(b"Unknown APP0 marker (not JFIF), length %u\0"),
    cstr(b"Unknown APP14 marker (not Adobe), length %u\0"),
    cstr(b"Define Arithmetic Table 0x%02x: 0x%02x\0"),
    cstr(b"Define Huffman Table 0x%02x\0"),
    cstr(b"Define Quantization Table %d  precision %d\0"),
    cstr(b"Define Restart Interval %u\0"),
    cstr(b"Freed EMS handle %u\0"),
    cstr(b"Obtained EMS handle %u\0"),
    cstr(b"End Of Image\0"),
    cstr(b"        %3d %3d %3d %3d %3d %3d %3d %3d\0"),
    cstr(b"JFIF APP0 marker: version %d.%02d, density %dx%d  %d\0"),
    cstr(b"Warning: thumbnail image size does not match data length %u\0"),
    cstr(b"JFIF extension marker: type 0x%02x, length %u\0"),
    cstr(b"    with %d x %d thumbnail image\0"),
    cstr(b"Miscellaneous marker 0x%02x, length %u\0"),
    cstr(b"Unexpected marker 0x%02x\0"),
    cstr(b"        %4u %4u %4u %4u %4u %4u %4u %4u\0"),
    cstr(b"Quantizing to %d = %d*%d*%d colors\0"),
    cstr(b"Quantizing to %d colors\0"),
    cstr(b"Selected %d colors for quantization\0"),
    cstr(b"At marker 0x%02x, recovery action %d\0"),
    cstr(b"RST%d\0"),
    cstr(b"Smoothing not supported with nonstandard sampling ratios\0"),
    cstr(b"Start Of Frame 0x%02x: width=%u, height=%u, components=%d\0"),
    cstr(b"    Component %d: %dhx%dv q=%d\0"),
    cstr(b"Start of Image\0"),
    cstr(b"Start Of Scan: %d components\0"),
    cstr(b"    Component %d: dc=%d ac=%d\0"),
    cstr(b"  Ss=%d, Se=%d, Ah=%d, Al=%d\0"),
    cstr(b"Closed temporary file %s\0"),
    cstr(b"Opened temporary file %s\0"),
    cstr(b"JFIF extension marker: JPEG-compressed thumbnail image, length %u\0"),
    cstr(b"JFIF extension marker: palette thumbnail image, length %u\0"),
    cstr(b"JFIF extension marker: RGB thumbnail image, length %u\0"),
    cstr(b"Unrecognized component IDs %d %d %d, assuming YCbCr\0"),
    cstr(b"Freed XMS handle %u\0"),
    cstr(b"Obtained XMS handle %u\0"),
    cstr(b"Unknown Adobe color transform code %d\0"),
    cstr(b"Corrupt JPEG data: bad arithmetic code\0"),
    cstr(b"Inconsistent progression sequence for component %d coefficient %d\0"),
    cstr(b"Corrupt JPEG data: %u extraneous bytes before marker 0x%02x\0"),
    cstr(b"Corrupt JPEG data: premature end of data segment\0"),
    cstr(b"Corrupt JPEG data: bad Huffman code\0"),
    cstr(b"Warning: unknown JFIF revision number %d.%02d\0"),
    cstr(b"Premature end of JPEG file\0"),
    cstr(b"Corrupt JPEG data: found marker 0x%02x instead of RST%d\0"),
    cstr(b"Invalid SOS parameters for sequential JPEG\0"),
    cstr(b"Application transferred too many scanlines\0"),
    cstr(b"Corrupt JPEG data: bad ICC marker\0"),
    core::ptr::null(),
];

extern "C" {
    fn snprintf(dst: *mut c_char, size: usize, fmt: *const c_char, ...) -> int;
    fn fprintf(stream: *mut ffi_types::FILE, fmt: *const c_char, ...) -> int;
    fn exit(status: int) -> !;
    #[link_name = "abort"]
    fn libc_abort() -> !;
    static mut stderr: *mut ffi_types::FILE;
    fn jpeg_rs_invoke_error_exit(cinfo: j_common_ptr);
}

#[inline]
unsafe fn abort_if_returns() -> ! {
    libc_abort()
}

#[inline]
unsafe fn err(cinfo: j_common_ptr) -> *mut jpeg_error_mgr {
    (*cinfo).err
}

#[inline]
pub unsafe fn set_msg_code(cinfo: j_common_ptr, code: J_MESSAGE_CODE) {
    (*err(cinfo)).msg_code = code as int;
}

#[inline]
unsafe fn set_i(cinfo: j_common_ptr, index: usize, value: int) {
    (*err(cinfo)).msg_parm.i[index] = value;
}

#[inline]
unsafe fn decode_warning_is_fatal(cinfo: j_common_ptr) -> bool {
    (*cinfo).is_decompressor != FALSE
        && registry::decompress_warnings_fatal(cinfo as j_decompress_ptr)
}

#[inline]
pub unsafe fn warnms(cinfo: j_common_ptr, code: J_MESSAGE_CODE) {
    set_msg_code(cinfo, code);
    if let Some(emit) = (*err(cinfo)).emit_message {
        emit(cinfo, -1);
    }
}

pub unsafe fn warnms1(cinfo: j_common_ptr, code: J_MESSAGE_CODE, p1: int) {
    set_msg_code(cinfo, code);
    set_i(cinfo, 0, p1);
    if let Some(emit) = (*err(cinfo)).emit_message {
        emit(cinfo, -1);
    }
}

pub unsafe fn tracems3(
    cinfo: j_common_ptr,
    msg_level: int,
    code: J_MESSAGE_CODE,
    p1: int,
    p2: int,
    p3: int,
) {
    set_msg_code(cinfo, code);
    set_i(cinfo, 0, p1);
    set_i(cinfo, 1, p2);
    set_i(cinfo, 2, p3);
    let err = err(cinfo);
    if (*err).trace_level >= msg_level {
        if let Some(output_message) = (*err).output_message {
            output_message(cinfo);
        }
    }
}

pub unsafe fn errexit(cinfo: j_common_ptr, code: J_MESSAGE_CODE) -> ! {
    set_msg_code(cinfo, code);
    jpeg_rs_invoke_error_exit(cinfo);
    abort_if_returns()
}

pub unsafe fn errexit1(cinfo: j_common_ptr, code: J_MESSAGE_CODE, p1: int) -> ! {
    set_msg_code(cinfo, code);
    set_i(cinfo, 0, p1);
    jpeg_rs_invoke_error_exit(cinfo);
    abort_if_returns()
}

pub unsafe fn errexit2(cinfo: j_common_ptr, code: J_MESSAGE_CODE, p1: int, p2: int) -> ! {
    set_msg_code(cinfo, code);
    set_i(cinfo, 0, p1);
    set_i(cinfo, 1, p2);
    jpeg_rs_invoke_error_exit(cinfo);
    abort_if_returns()
}

pub unsafe fn errexit_scan_limit(cinfo: j_common_ptr, limit: int) -> ! {
    (*err(cinfo)).msg_code = JMSG_SCAN_LIMIT;
    set_i(cinfo, 0, limit);
    jpeg_rs_invoke_error_exit(cinfo);
    abort_if_returns()
}

unsafe extern "C" fn error_exit(cinfo: j_common_ptr) {
    if let Some(output_message) = (*err(cinfo)).output_message {
        output_message(cinfo);
    }
    destroy(cinfo);
    exit(EXIT_FAILURE);
}

unsafe extern "C" fn output_message(cinfo: j_common_ptr) {
    let mut buffer = [0 as c_char; JMSG_LENGTH_MAX];
    if let Some(format_message) = (*err(cinfo)).format_message {
        format_message(cinfo, buffer.as_mut_ptr());
    }
    let _ = fprintf(stderr, cstr(b"%s\n\0"), buffer.as_ptr());
}

unsafe extern "C" fn emit_message(cinfo: j_common_ptr, msg_level: int) {
    let err = err(cinfo);
    if msg_level < 0 {
        if (*err).num_warnings == 0 || (*err).trace_level >= 3 {
            if let Some(output_message) = (*err).output_message {
                output_message(cinfo);
            }
        }
        (*err).num_warnings += 1;
        if decode_warning_is_fatal(cinfo) {
            jpeg_rs_invoke_error_exit(cinfo);
            abort_if_returns();
        }
    } else if (*err).trace_level >= msg_level {
        if let Some(output_message) = (*err).output_message {
            output_message(cinfo);
        }
    }
}

unsafe extern "C" fn format_message(cinfo: j_common_ptr, buffer: *mut c_char) {
    let err = err(cinfo);
    let msg_code = (*err).msg_code;
    let mut msgtext = core::ptr::null::<c_char>();

    if msg_code > 0 && msg_code <= (*err).last_jpeg_message {
        msgtext = *(*err).jpeg_message_table.add(msg_code as usize);
    } else if !(*err).addon_message_table.is_null()
        && msg_code >= (*err).first_addon_message
        && msg_code <= (*err).last_addon_message
    {
        msgtext = *(*err)
            .addon_message_table
            .add((msg_code - (*err).first_addon_message) as usize);
    }

    if msgtext.is_null() {
        (*err).msg_parm.i[0] = msg_code;
        msgtext = *(*err).jpeg_message_table;
    }

    let mut is_string = false;
    let mut cursor = msgtext;
    while *cursor != 0 {
        if *cursor == b'%' as c_char {
            if *cursor.add(1) == b's' as c_char {
                is_string = true;
            }
            break;
        }
        cursor = cursor.add(1);
    }

    if is_string {
        let _ = snprintf(buffer, JMSG_LENGTH_MAX, msgtext, (*err).msg_parm.s.as_ptr());
    } else {
        let i = (*err).msg_parm.i;
        let _ = snprintf(
            buffer,
            JMSG_LENGTH_MAX,
            msgtext,
            i[0],
            i[1],
            i[2],
            i[3],
            i[4],
            i[5],
            i[6],
            i[7],
        );
    }
}

unsafe extern "C" fn reset_error_mgr(cinfo: j_common_ptr) {
    (*err(cinfo)).num_warnings = 0;
    (*err(cinfo)).msg_code = 0;
}

pub unsafe fn std_error(err: *mut jpeg_error_mgr) -> *mut jpeg_error_mgr {
    (*err).error_exit = Some(error_exit);
    (*err).emit_message = Some(emit_message);
    (*err).output_message = Some(output_message);
    (*err).format_message = Some(format_message);
    (*err).reset_error_mgr = Some(reset_error_mgr);
    (*err).trace_level = 0;
    (*err).num_warnings = 0;
    (*err).msg_code = 0;
    (*err).jpeg_message_table = core::ptr::addr_of!(jpeg_std_message_table) as *const *const c_char;
    (*err).last_jpeg_message = JMSG_LASTMSGCODE - 1;
    (*err).addon_message_table = core::ptr::addr_of!(jpeg_rs_message_table) as *const *const c_char;
    (*err).first_addon_message = JMSG_SCAN_LIMIT;
    (*err).last_addon_message = JMSG_SCAN_LIMIT;
    err
}

pub unsafe fn abort(cinfo: j_common_ptr) {
    if (*cinfo).mem.is_null() {
        return;
    }

    let free_pool = (*(*cinfo).mem).free_pool.unwrap();
    let mut pool = JPOOL_NUMPOOLS as int - 1;
    while pool > JPOOL_PERMANENT {
        free_pool(cinfo, pool);
        pool -= 1;
    }

    if (*cinfo).is_decompressor != FALSE {
        (*cinfo).global_state = DSTATE_START;
        (*(cinfo as j_decompress_ptr)).marker_list = core::ptr::null_mut();
    } else {
        (*cinfo).global_state = CSTATE_START;
    }
}

pub unsafe fn destroy(cinfo: j_common_ptr) {
    if (*cinfo).is_decompressor != FALSE {
        registry::clear_decompress_policy(cinfo as j_decompress_ptr);
    }
    if !(*cinfo).mem.is_null() {
        let self_destruct = (*(*cinfo).mem).self_destruct.unwrap();
        self_destruct(cinfo);
    }
    (*cinfo).mem = core::ptr::null_mut();
    (*cinfo).global_state = 0;
}

pub unsafe fn alloc_quant_table(cinfo: j_common_ptr) -> *mut JQUANT_TBL {
    let alloc_small = (*(*cinfo).mem).alloc_small.unwrap();
    let tbl = alloc_small(
        cinfo,
        ffi_types::JPOOL_PERMANENT,
        core::mem::size_of::<JQUANT_TBL>(),
    ) as *mut JQUANT_TBL;
    (*tbl).sent_table = FALSE as boolean;
    tbl
}

pub unsafe fn alloc_huff_table(cinfo: j_common_ptr) -> *mut JHUFF_TBL {
    let alloc_small = (*(*cinfo).mem).alloc_small.unwrap();
    let tbl = alloc_small(
        cinfo,
        ffi_types::JPOOL_PERMANENT,
        core::mem::size_of::<JHUFF_TBL>(),
    ) as *mut JHUFF_TBL;
    (*tbl).sent_table = FALSE as boolean;
    tbl
}

pub unsafe fn init_memory_manager(cinfo: j_common_ptr) {
    memory::jinit_memory_mgr(cinfo)
}

pub unsafe fn as_common(cinfo: *mut jpeg_common_struct) -> j_common_ptr {
    cinfo
}
