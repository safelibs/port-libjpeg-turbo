use core::{ffi::c_void, mem::size_of, ptr};

use ffi_types::{
    boolean, int, j_common_ptr, j_compress_ptr, j_decompress_ptr, jpeg_destination_mgr,
    jpeg_source_mgr, long, size_t, FALSE, FILE, JOCTET, JPEG_EOI, JPOOL_IMAGE, JPOOL_PERMANENT,
    J_MESSAGE_CODE, TRUE,
};

use crate::common::error;

const INPUT_BUF_SIZE: usize = 4096;
const OUTPUT_BUF_SIZE: usize = 4096;

extern "C" {
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fwrite(ptr: *const c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fflush(stream: *mut FILE) -> int;
    fn ferror(stream: *mut FILE) -> int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn jpeg_resync_to_restart(cinfo: j_decompress_ptr, desired: int) -> boolean;
}

#[repr(C)]
struct MySourceMgr {
    pub_: jpeg_source_mgr,
    infile: *mut FILE,
    buffer: *mut JOCTET,
    start_of_file: boolean,
}

#[repr(C)]
struct MyDestinationMgr {
    pub_: jpeg_destination_mgr,
    outfile: *mut FILE,
    buffer: *mut JOCTET,
}

#[repr(C)]
struct MyMemDestinationMgr {
    pub_: jpeg_destination_mgr,
    outbuffer: *mut *mut u8,
    outsize: *mut ffi_types::ulong,
    newbuffer: *mut u8,
    buffer: *mut JOCTET,
    bufsize: size_t,
}

#[inline]
unsafe fn has_stdio_src_manager(src: *mut jpeg_source_mgr) -> bool {
    let expected: unsafe extern "C" fn(j_decompress_ptr) = init_source;
    !src.is_null() && (*src).init_source == Some(expected)
}

#[inline]
unsafe fn has_mem_src_manager(src: *mut jpeg_source_mgr) -> bool {
    let expected: unsafe extern "C" fn(j_decompress_ptr) = init_mem_source;
    !src.is_null() && (*src).init_source == Some(expected)
}

#[inline]
unsafe fn has_stdio_dest_manager(dest: *mut jpeg_destination_mgr) -> bool {
    let expected: unsafe extern "C" fn(j_compress_ptr) = init_destination;
    !dest.is_null() && (*dest).init_destination == Some(expected)
}

#[inline]
unsafe fn has_mem_dest_manager(dest: *mut jpeg_destination_mgr) -> bool {
    let expected: unsafe extern "C" fn(j_compress_ptr) = init_mem_destination;
    !dest.is_null() && (*dest).init_destination == Some(expected)
}

unsafe extern "C" fn init_source(cinfo: j_decompress_ptr) {
    (*((*cinfo).src as *mut MySourceMgr)).start_of_file = TRUE;
}

unsafe extern "C" fn init_mem_source(_cinfo: j_decompress_ptr) {}

unsafe extern "C" fn fill_input_buffer(cinfo: j_decompress_ptr) -> boolean {
    let src = (*cinfo).src as *mut MySourceMgr;
    let mut nbytes = fread(
        (*src).buffer as *mut c_void,
        1,
        INPUT_BUF_SIZE,
        (*src).infile,
    );
    if nbytes == 0 {
        if (*src).start_of_file != FALSE {
            error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_INPUT_EMPTY);
        }
        error::warnms(cinfo as j_common_ptr, J_MESSAGE_CODE::JWRN_JPEG_EOF);
        *(*src).buffer = 0xFF;
        *(*src).buffer.add(1) = JPEG_EOI as JOCTET;
        nbytes = 2;
    }
    (*(*cinfo).src).next_input_byte = (*src).buffer;
    (*(*cinfo).src).bytes_in_buffer = nbytes;
    (*src).start_of_file = FALSE;
    TRUE
}

unsafe extern "C" fn fill_mem_input_buffer(cinfo: j_decompress_ptr) -> boolean {
    static MYBUFFER: [JOCTET; 4] = [0xFF, JPEG_EOI as JOCTET, 0, 0];
    error::warnms(cinfo as j_common_ptr, J_MESSAGE_CODE::JWRN_JPEG_EOF);
    (*(*cinfo).src).next_input_byte = MYBUFFER.as_ptr();
    (*(*cinfo).src).bytes_in_buffer = 2;
    TRUE
}

unsafe extern "C" fn skip_input_data(cinfo: j_decompress_ptr, mut num_bytes: long) {
    let src = (*cinfo).src;
    if num_bytes <= 0 {
        return;
    }
    while num_bytes > (*src).bytes_in_buffer as long {
        num_bytes -= (*src).bytes_in_buffer as long;
        let _ = ((*src).fill_input_buffer.unwrap())(cinfo);
    }
    (*src).next_input_byte = (*src).next_input_byte.add(num_bytes as usize);
    (*src).bytes_in_buffer -= num_bytes as usize;
}

unsafe extern "C" fn term_source(_cinfo: j_decompress_ptr) {}

unsafe extern "C" fn init_destination(cinfo: j_compress_ptr) {
    let dest = (*cinfo).dest as *mut MyDestinationMgr;
    (*dest).buffer =
        (*(*cinfo).mem).alloc_small.unwrap()(cinfo as j_common_ptr, JPOOL_IMAGE, OUTPUT_BUF_SIZE)
            as *mut JOCTET;
    (*(*cinfo).dest).next_output_byte = (*dest).buffer;
    (*(*cinfo).dest).free_in_buffer = OUTPUT_BUF_SIZE;
}

unsafe extern "C" fn init_mem_destination(_cinfo: j_compress_ptr) {}

unsafe extern "C" fn empty_output_buffer(cinfo: j_compress_ptr) -> boolean {
    let dest = (*cinfo).dest as *mut MyDestinationMgr;
    if fwrite(
        (*dest).buffer as *const c_void,
        1,
        OUTPUT_BUF_SIZE,
        (*dest).outfile,
    ) != OUTPUT_BUF_SIZE
    {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_FILE_WRITE);
    }
    (*(*cinfo).dest).next_output_byte = (*dest).buffer;
    (*(*cinfo).dest).free_in_buffer = OUTPUT_BUF_SIZE;
    TRUE
}

unsafe extern "C" fn empty_mem_output_buffer(cinfo: j_compress_ptr) -> boolean {
    let dest = (*cinfo).dest as *mut MyMemDestinationMgr;
    let nextsize = (*dest).bufsize * 2;
    let nextbuffer = malloc(nextsize) as *mut JOCTET;
    if nextbuffer.is_null() {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_OUT_OF_MEMORY,
            10,
        );
    }
    ptr::copy_nonoverlapping((*dest).buffer, nextbuffer, (*dest).bufsize);
    free((*dest).newbuffer as *mut c_void);
    (*dest).newbuffer = nextbuffer as *mut u8;
    (*(*cinfo).dest).next_output_byte = nextbuffer.add((*dest).bufsize);
    (*(*cinfo).dest).free_in_buffer = (*dest).bufsize;
    (*dest).buffer = nextbuffer;
    (*dest).bufsize = nextsize;
    TRUE
}

unsafe extern "C" fn term_destination(cinfo: j_compress_ptr) {
    let dest = (*cinfo).dest as *mut MyDestinationMgr;
    let datacount = OUTPUT_BUF_SIZE - (*(*cinfo).dest).free_in_buffer;
    if datacount > 0
        && fwrite(
            (*dest).buffer as *const c_void,
            1,
            datacount,
            (*dest).outfile,
        ) != datacount
    {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_FILE_WRITE);
    }
    let _ = fflush((*dest).outfile);
    if ferror((*dest).outfile) != 0 {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_FILE_WRITE);
    }
}

unsafe extern "C" fn term_mem_destination(cinfo: j_compress_ptr) {
    let dest = (*cinfo).dest as *mut MyMemDestinationMgr;
    *(*dest).outbuffer = (*dest).buffer as *mut u8;
    *(*dest).outsize = ((*dest).bufsize - (*(*cinfo).dest).free_in_buffer) as ffi_types::ulong;
}

pub unsafe fn jpeg_stdio_src(cinfo: j_decompress_ptr, infile: *mut FILE) {
    if (*cinfo).src.is_null() {
        let src = (*(*cinfo).mem).alloc_small.unwrap()(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            size_of::<MySourceMgr>(),
        ) as *mut MySourceMgr;
        ptr::write_bytes(src as *mut u8, 0, size_of::<MySourceMgr>());
        (*src).buffer = (*(*cinfo).mem).alloc_small.unwrap()(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            INPUT_BUF_SIZE,
        ) as *mut JOCTET;
        (*cinfo).src = &mut (*src).pub_;
    } else if !has_stdio_src_manager((*cinfo).src) {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_BUFFER_SIZE);
    }

    let src = (*cinfo).src as *mut MySourceMgr;
    (*src).pub_.init_source = Some(init_source);
    (*src).pub_.fill_input_buffer = Some(fill_input_buffer);
    (*src).pub_.skip_input_data = Some(skip_input_data);
    (*src).pub_.resync_to_restart = Some(jpeg_resync_to_restart);
    (*src).pub_.term_source = Some(term_source);
    (*src).infile = infile;
    (*src).pub_.bytes_in_buffer = 0;
    (*src).pub_.next_input_byte = ptr::null();
}

pub unsafe fn jpeg_mem_src(cinfo: j_decompress_ptr, inbuffer: *const u8, insize: ffi_types::ulong) {
    if inbuffer.is_null() || insize == 0 {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_INPUT_EMPTY);
    }
    if (*cinfo).src.is_null() {
        let src = (*(*cinfo).mem).alloc_small.unwrap()(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            size_of::<jpeg_source_mgr>(),
        ) as *mut jpeg_source_mgr;
        ptr::write_bytes(src as *mut u8, 0, size_of::<jpeg_source_mgr>());
        (*cinfo).src = src;
    } else if !has_mem_src_manager((*cinfo).src) {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_BUFFER_SIZE);
    }

    let src = (*cinfo).src;
    (*src).init_source = Some(init_mem_source);
    (*src).fill_input_buffer = Some(fill_mem_input_buffer);
    (*src).skip_input_data = Some(skip_input_data);
    (*src).resync_to_restart = Some(jpeg_resync_to_restart);
    (*src).term_source = Some(term_source);
    (*src).bytes_in_buffer = insize as usize;
    (*src).next_input_byte = inbuffer;
}

pub unsafe fn jpeg_stdio_dest(cinfo: j_compress_ptr, outfile: *mut FILE) {
    if (*cinfo).dest.is_null() {
        let dest = (*(*cinfo).mem).alloc_small.unwrap()(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            size_of::<MyDestinationMgr>(),
        ) as *mut MyDestinationMgr;
        ptr::write_bytes(dest as *mut u8, 0, size_of::<MyDestinationMgr>());
        (*cinfo).dest = &mut (*dest).pub_;
    } else if !has_stdio_dest_manager((*cinfo).dest) {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_BUFFER_SIZE);
    }

    let dest = (*cinfo).dest as *mut MyDestinationMgr;
    (*dest).pub_.init_destination = Some(init_destination);
    (*dest).pub_.empty_output_buffer = Some(empty_output_buffer);
    (*dest).pub_.term_destination = Some(term_destination);
    (*dest).outfile = outfile;
}

pub unsafe fn jpeg_mem_dest(
    cinfo: j_compress_ptr,
    outbuffer: *mut *mut u8,
    outsize: *mut ffi_types::ulong,
) {
    if outbuffer.is_null() || outsize.is_null() {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_BUFFER_SIZE);
    }
    if (*cinfo).dest.is_null() {
        let dest = (*(*cinfo).mem).alloc_small.unwrap()(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            size_of::<MyMemDestinationMgr>(),
        ) as *mut MyMemDestinationMgr;
        ptr::write_bytes(dest as *mut u8, 0, size_of::<MyMemDestinationMgr>());
        (*cinfo).dest = &mut (*dest).pub_;
    } else if !has_mem_dest_manager((*cinfo).dest) {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_BUFFER_SIZE);
    }

    let dest = (*cinfo).dest as *mut MyMemDestinationMgr;
    (*dest).pub_.init_destination = Some(init_mem_destination);
    (*dest).pub_.empty_output_buffer = Some(empty_mem_output_buffer);
    (*dest).pub_.term_destination = Some(term_mem_destination);
    (*dest).outbuffer = outbuffer;
    (*dest).outsize = outsize;
    (*dest).newbuffer = ptr::null_mut();

    if (*outbuffer).is_null() || *outsize == 0 {
        (*dest).newbuffer = malloc(OUTPUT_BUF_SIZE) as *mut u8;
        if (*dest).newbuffer.is_null() {
            error::errexit1(
                cinfo as j_common_ptr,
                J_MESSAGE_CODE::JERR_OUT_OF_MEMORY,
                10,
            );
        }
        *outbuffer = (*dest).newbuffer;
        *outsize = OUTPUT_BUF_SIZE as ffi_types::ulong;
    }

    (*dest).buffer = *outbuffer as *mut JOCTET;
    (*dest).bufsize = *outsize as usize;
    (*dest).pub_.next_output_byte = (*dest).buffer;
    (*dest).pub_.free_in_buffer = (*dest).bufsize;
}
