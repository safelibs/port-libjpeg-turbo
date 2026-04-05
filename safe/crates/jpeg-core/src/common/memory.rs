use core::{ffi::c_void, mem::size_of, ptr};

use ffi_types::{
    backing_store_info, backing_store_ptr, boolean, int, j_common_ptr, jpeg_memory_mgr,
    jvirt_barray_ptr, jvirt_sarray_ptr, FALSE, JBLOCK, JBLOCKARRAY, JBLOCKROW, JDIMENSION,
    JPOOL_IMAGE, JPOOL_NUMPOOLS, JSAMPARRAY, JSAMPLE, JSAMPROW, J_MESSAGE_CODE, MAX_ALLOC_CHUNK,
    TRUE,
};

use crate::common::error;

const ALIGN_SIZE: usize = 32;
const MIN_SLOP: usize = 50;
const FIRST_POOL_SLOP: [usize; JPOOL_NUMPOOLS] = [1600, 16000];
const EXTRA_POOL_SLOP: [usize; JPOOL_NUMPOOLS] = [0, 5000];

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn getenv(name: *const i8) -> *const i8;
}

#[repr(C)]
struct SmallPoolHdr {
    next: *mut SmallPoolHdr,
    bytes_used: usize,
    bytes_left: usize,
}

#[repr(C)]
struct LargePoolHdr {
    next: *mut LargePoolHdr,
    bytes_used: usize,
    bytes_left: usize,
}

#[repr(C)]
struct JVirtSArrayControl {
    mem_buffer: JSAMPARRAY,
    rows_in_array: JDIMENSION,
    samplesperrow: JDIMENSION,
    maxaccess: JDIMENSION,
    rows_in_mem: JDIMENSION,
    rowsperchunk: JDIMENSION,
    cur_start_row: JDIMENSION,
    first_undef_row: JDIMENSION,
    pre_zero: boolean,
    dirty: boolean,
    b_s_open: boolean,
    next: *mut JVirtSArrayControl,
    b_s_info: backing_store_info,
}

#[repr(C)]
struct JVirtBArrayControl {
    mem_buffer: JBLOCKARRAY,
    rows_in_array: JDIMENSION,
    blocksperrow: JDIMENSION,
    maxaccess: JDIMENSION,
    rows_in_mem: JDIMENSION,
    rowsperchunk: JDIMENSION,
    cur_start_row: JDIMENSION,
    first_undef_row: JDIMENSION,
    pre_zero: boolean,
    dirty: boolean,
    b_s_open: boolean,
    next: *mut JVirtBArrayControl,
    b_s_info: backing_store_info,
}

#[repr(C)]
struct MyMemoryMgr {
    pub_: jpeg_memory_mgr,
    small_list: [*mut SmallPoolHdr; JPOOL_NUMPOOLS],
    large_list: [*mut LargePoolHdr; JPOOL_NUMPOOLS],
    virt_sarray_list: *mut JVirtSArrayControl,
    virt_barray_list: *mut JVirtBArrayControl,
    total_space_allocated: usize,
    last_rowsperchunk: JDIMENSION,
}

#[inline]
unsafe fn mem(cinfo: j_common_ptr) -> *mut MyMemoryMgr {
    (*cinfo).mem as *mut MyMemoryMgr
}

#[inline]
const fn min_u32(a: JDIMENSION, b: JDIMENSION) -> JDIMENSION {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
const fn round_up_pow2(a: usize, b: usize) -> usize {
    (a + b - 1) & !(b - 1)
}

unsafe fn out_of_memory(cinfo: j_common_ptr, which: int) -> ! {
    error::errexit1(cinfo, J_MESSAGE_CODE::JERR_OUT_OF_MEMORY, which)
}

pub unsafe fn jpeg_get_small(_cinfo: j_common_ptr, sizeofobject: usize) -> *mut c_void {
    malloc(sizeofobject)
}

pub unsafe fn jpeg_free_small(_cinfo: j_common_ptr, object: *mut c_void, _sizeofobject: usize) {
    free(object);
}

pub unsafe fn jpeg_get_large(_cinfo: j_common_ptr, sizeofobject: usize) -> *mut c_void {
    malloc(sizeofobject)
}

pub unsafe fn jpeg_free_large(_cinfo: j_common_ptr, object: *mut c_void, _sizeofobject: usize) {
    free(object);
}

pub unsafe fn jpeg_mem_available(
    cinfo: j_common_ptr,
    _min_bytes_needed: usize,
    max_bytes_needed: usize,
    already_allocated: usize,
) -> usize {
    if (*(*cinfo).mem).max_memory_to_use != 0 {
        let limit = (*(*cinfo).mem).max_memory_to_use as usize;
        limit.saturating_sub(already_allocated)
    } else {
        max_bytes_needed
    }
}

pub unsafe fn jpeg_open_backing_store(
    cinfo: j_common_ptr,
    _info: backing_store_ptr,
    _total_bytes_needed: ffi_types::long,
) -> ! {
    error::errexit(cinfo, J_MESSAGE_CODE::JERR_NO_BACKING_STORE)
}

pub unsafe fn jpeg_mem_init(_cinfo: j_common_ptr) -> ffi_types::long {
    0
}

pub unsafe fn jpeg_mem_term(_cinfo: j_common_ptr) {}

unsafe extern "C" fn alloc_small(
    cinfo: j_common_ptr,
    pool_id: int,
    sizeofobject: usize,
) -> *mut c_void {
    let mem = mem(cinfo);
    let mut hdr_ptr = (*mem).small_list[pool_id as usize];
    let mut prev_hdr_ptr: *mut SmallPoolHdr = ptr::null_mut();
    let mut data_ptr: *mut u8;
    let min_request;
    let mut slop;
    let mut sizeofobject = sizeofobject;

    if sizeofobject > MAX_ALLOC_CHUNK as usize {
        out_of_memory(cinfo, 7);
    }
    sizeofobject = round_up_pow2(sizeofobject, ALIGN_SIZE);

    if size_of::<SmallPoolHdr>() + sizeofobject + ALIGN_SIZE - 1 > MAX_ALLOC_CHUNK as usize {
        out_of_memory(cinfo, 1);
    }
    if pool_id < 0 || pool_id as usize >= JPOOL_NUMPOOLS {
        error::errexit1(cinfo, J_MESSAGE_CODE::JERR_BAD_POOL_ID, pool_id);
    }

    while !hdr_ptr.is_null() {
        if (*hdr_ptr).bytes_left >= sizeofobject {
            break;
        }
        prev_hdr_ptr = hdr_ptr;
        hdr_ptr = (*hdr_ptr).next;
    }

    if hdr_ptr.is_null() {
        min_request = size_of::<SmallPoolHdr>() + sizeofobject + ALIGN_SIZE - 1;
        slop = if prev_hdr_ptr.is_null() {
            FIRST_POOL_SLOP[pool_id as usize]
        } else {
            EXTRA_POOL_SLOP[pool_id as usize]
        };
        if slop > MAX_ALLOC_CHUNK as usize - min_request {
            slop = MAX_ALLOC_CHUNK as usize - min_request;
        }
        loop {
            hdr_ptr = jpeg_get_small(cinfo, min_request + slop) as *mut SmallPoolHdr;
            if !hdr_ptr.is_null() {
                break;
            }
            slop /= 2;
            if slop < MIN_SLOP {
                out_of_memory(cinfo, 2);
            }
        }
        (*mem).total_space_allocated += min_request + slop;
        (*hdr_ptr).next = ptr::null_mut();
        (*hdr_ptr).bytes_used = 0;
        (*hdr_ptr).bytes_left = sizeofobject + slop;
        if prev_hdr_ptr.is_null() {
            (*mem).small_list[pool_id as usize] = hdr_ptr;
        } else {
            (*prev_hdr_ptr).next = hdr_ptr;
        }
    }

    data_ptr = (hdr_ptr as *mut u8).add(size_of::<SmallPoolHdr>());
    let misalignment = (data_ptr as usize) % ALIGN_SIZE;
    if misalignment != 0 {
        data_ptr = data_ptr.add(ALIGN_SIZE - misalignment);
    }
    data_ptr = data_ptr.add((*hdr_ptr).bytes_used);
    (*hdr_ptr).bytes_used += sizeofobject;
    (*hdr_ptr).bytes_left -= sizeofobject;
    data_ptr as *mut c_void
}

unsafe extern "C" fn alloc_large(
    cinfo: j_common_ptr,
    pool_id: int,
    sizeofobject: usize,
) -> *mut c_void {
    let mem = mem(cinfo);
    let mut sizeofobject = sizeofobject;

    if sizeofobject > MAX_ALLOC_CHUNK as usize {
        out_of_memory(cinfo, 8);
    }
    sizeofobject = round_up_pow2(sizeofobject, ALIGN_SIZE);
    if size_of::<LargePoolHdr>() + sizeofobject + ALIGN_SIZE - 1 > MAX_ALLOC_CHUNK as usize {
        out_of_memory(cinfo, 3);
    }
    if pool_id < 0 || pool_id as usize >= JPOOL_NUMPOOLS {
        error::errexit1(cinfo, J_MESSAGE_CODE::JERR_BAD_POOL_ID, pool_id);
    }

    let hdr_ptr = jpeg_get_large(
        cinfo,
        sizeofobject + size_of::<LargePoolHdr>() + ALIGN_SIZE - 1,
    ) as *mut LargePoolHdr;
    if hdr_ptr.is_null() {
        out_of_memory(cinfo, 4);
    }
    (*mem).total_space_allocated += sizeofobject + size_of::<LargePoolHdr>() + ALIGN_SIZE - 1;
    (*hdr_ptr).next = (*mem).large_list[pool_id as usize];
    (*hdr_ptr).bytes_used = sizeofobject;
    (*hdr_ptr).bytes_left = 0;
    (*mem).large_list[pool_id as usize] = hdr_ptr;

    let mut data_ptr = (hdr_ptr as *mut u8).add(size_of::<LargePoolHdr>());
    let misalignment = (data_ptr as usize) % ALIGN_SIZE;
    if misalignment != 0 {
        data_ptr = data_ptr.add(ALIGN_SIZE - misalignment);
    }
    data_ptr as *mut c_void
}

unsafe extern "C" fn alloc_sarray(
    cinfo: j_common_ptr,
    pool_id: int,
    samplesperrow: JDIMENSION,
    numrows: JDIMENSION,
) -> JSAMPARRAY {
    let mem = mem(cinfo);
    let mut samplesperrow = samplesperrow;

    if ALIGN_SIZE % size_of::<JSAMPLE>() != 0 {
        out_of_memory(cinfo, 5);
    }
    if samplesperrow as usize > MAX_ALLOC_CHUNK as usize {
        out_of_memory(cinfo, 9);
    }
    samplesperrow = round_up_pow2(
        samplesperrow as usize,
        (2 * ALIGN_SIZE) / size_of::<JSAMPLE>(),
    ) as JDIMENSION;

    let ltemp = ((MAX_ALLOC_CHUNK as usize - size_of::<LargePoolHdr>())
        / (samplesperrow as usize * size_of::<JSAMPLE>())) as ffi_types::long;
    if ltemp <= 0 {
        error::errexit(cinfo, J_MESSAGE_CODE::JERR_WIDTH_OVERFLOW);
    }
    let mut rowsperchunk = if ltemp < numrows as ffi_types::long {
        ltemp as JDIMENSION
    } else {
        numrows
    };
    (*mem).last_rowsperchunk = rowsperchunk;

    let result =
        alloc_small(cinfo, pool_id, numrows as usize * size_of::<JSAMPROW>()) as JSAMPARRAY;

    let mut currow = 0usize;
    while currow < numrows as usize {
        rowsperchunk = min_u32(rowsperchunk, numrows - currow as JDIMENSION);
        let mut workspace = alloc_large(
            cinfo,
            pool_id,
            rowsperchunk as usize * samplesperrow as usize * size_of::<JSAMPLE>(),
        ) as JSAMPROW;
        let mut i = rowsperchunk;
        while i > 0 {
            *result.add(currow) = workspace;
            currow += 1;
            workspace = workspace.add(samplesperrow as usize);
            i -= 1;
        }
    }

    result
}

unsafe extern "C" fn alloc_barray(
    cinfo: j_common_ptr,
    pool_id: int,
    blocksperrow: JDIMENSION,
    numrows: JDIMENSION,
) -> JBLOCKARRAY {
    let mem = mem(cinfo);

    if size_of::<JBLOCK>() % ALIGN_SIZE != 0 {
        out_of_memory(cinfo, 6);
    }
    let ltemp = ((MAX_ALLOC_CHUNK as usize - size_of::<LargePoolHdr>())
        / (blocksperrow as usize * size_of::<JBLOCK>())) as ffi_types::long;
    if ltemp <= 0 {
        error::errexit(cinfo, J_MESSAGE_CODE::JERR_WIDTH_OVERFLOW);
    }
    let mut rowsperchunk = if ltemp < numrows as ffi_types::long {
        ltemp as JDIMENSION
    } else {
        numrows
    };
    (*mem).last_rowsperchunk = rowsperchunk;

    let result =
        alloc_small(cinfo, pool_id, numrows as usize * size_of::<JBLOCKROW>()) as JBLOCKARRAY;
    let mut currow = 0usize;
    while currow < numrows as usize {
        rowsperchunk = min_u32(rowsperchunk, numrows - currow as JDIMENSION);
        let mut workspace = alloc_large(
            cinfo,
            pool_id,
            rowsperchunk as usize * blocksperrow as usize * size_of::<JBLOCK>(),
        ) as JBLOCKROW;
        let mut i = rowsperchunk;
        while i > 0 {
            *result.add(currow) = workspace;
            currow += 1;
            workspace = workspace.add(blocksperrow as usize);
            i -= 1;
        }
    }

    result
}

unsafe extern "C" fn request_virt_sarray(
    cinfo: j_common_ptr,
    pool_id: int,
    pre_zero: boolean,
    samplesperrow: JDIMENSION,
    numrows: JDIMENSION,
    maxaccess: JDIMENSION,
) -> jvirt_sarray_ptr {
    let mem = mem(cinfo);
    if pool_id != JPOOL_IMAGE {
        error::errexit1(cinfo, J_MESSAGE_CODE::JERR_BAD_POOL_ID, pool_id);
    }
    let result =
        alloc_small(cinfo, pool_id, size_of::<JVirtSArrayControl>()) as *mut JVirtSArrayControl;
    ptr::write_bytes(result as *mut u8, 0, size_of::<JVirtSArrayControl>());
    (*result).rows_in_array = numrows;
    (*result).samplesperrow = samplesperrow;
    (*result).maxaccess = maxaccess;
    (*result).pre_zero = pre_zero;
    (*result).b_s_open = FALSE;
    (*result).next = (*mem).virt_sarray_list;
    (*mem).virt_sarray_list = result;
    result as jvirt_sarray_ptr
}

unsafe extern "C" fn request_virt_barray(
    cinfo: j_common_ptr,
    pool_id: int,
    pre_zero: boolean,
    blocksperrow: JDIMENSION,
    numrows: JDIMENSION,
    maxaccess: JDIMENSION,
) -> jvirt_barray_ptr {
    let mem = mem(cinfo);
    if pool_id != JPOOL_IMAGE {
        error::errexit1(cinfo, J_MESSAGE_CODE::JERR_BAD_POOL_ID, pool_id);
    }
    let result =
        alloc_small(cinfo, pool_id, size_of::<JVirtBArrayControl>()) as *mut JVirtBArrayControl;
    ptr::write_bytes(result as *mut u8, 0, size_of::<JVirtBArrayControl>());
    (*result).rows_in_array = numrows;
    (*result).blocksperrow = blocksperrow;
    (*result).maxaccess = maxaccess;
    (*result).pre_zero = pre_zero;
    (*result).b_s_open = FALSE;
    (*result).next = (*mem).virt_barray_list;
    (*mem).virt_barray_list = result;
    result as jvirt_barray_ptr
}

unsafe extern "C" fn realize_virt_arrays(cinfo: j_common_ptr) {
    let mem = mem(cinfo);
    let mut space_per_minheight = 0usize;
    let mut maximum_space = 0usize;

    let mut sptr = (*mem).virt_sarray_list;
    while !sptr.is_null() {
        if (*sptr).mem_buffer.is_null() {
            let new_space = (*sptr).rows_in_array as usize
                * (*sptr).samplesperrow as usize
                * size_of::<JSAMPLE>();
            space_per_minheight +=
                (*sptr).maxaccess as usize * (*sptr).samplesperrow as usize * size_of::<JSAMPLE>();
            if usize::MAX - maximum_space < new_space {
                out_of_memory(cinfo, 10);
            }
            maximum_space += new_space;
        }
        sptr = (*sptr).next;
    }

    let mut bptr = (*mem).virt_barray_list;
    while !bptr.is_null() {
        if (*bptr).mem_buffer.is_null() {
            let new_space = (*bptr).rows_in_array as usize
                * (*bptr).blocksperrow as usize
                * size_of::<JBLOCK>();
            space_per_minheight +=
                (*bptr).maxaccess as usize * (*bptr).blocksperrow as usize * size_of::<JBLOCK>();
            if usize::MAX - maximum_space < new_space {
                out_of_memory(cinfo, 11);
            }
            maximum_space += new_space;
        }
        bptr = (*bptr).next;
    }

    if space_per_minheight == 0 {
        return;
    }

    let avail_mem = jpeg_mem_available(
        cinfo,
        space_per_minheight,
        maximum_space,
        (*mem).total_space_allocated,
    );
    let max_minheights = if avail_mem >= maximum_space {
        1_000_000_000usize
    } else {
        let value = avail_mem / space_per_minheight;
        if value == 0 {
            1
        } else {
            value
        }
    };

    sptr = (*mem).virt_sarray_list;
    while !sptr.is_null() {
        if (*sptr).mem_buffer.is_null() {
            let minheights = ((*sptr).rows_in_array as usize - 1) / (*sptr).maxaccess as usize + 1;
            if minheights <= max_minheights {
                (*sptr).rows_in_mem = (*sptr).rows_in_array;
            } else {
                (*sptr).rows_in_mem = (max_minheights as JDIMENSION) * (*sptr).maxaccess;
                jpeg_open_backing_store(
                    cinfo,
                    &mut (*sptr).b_s_info,
                    (*sptr).rows_in_array as ffi_types::long
                        * (*sptr).samplesperrow as ffi_types::long
                        * size_of::<JSAMPLE>() as ffi_types::long,
                );
                (*sptr).b_s_open = TRUE;
            }
            (*sptr).mem_buffer = alloc_sarray(
                cinfo,
                JPOOL_IMAGE,
                (*sptr).samplesperrow,
                (*sptr).rows_in_mem,
            );
            (*sptr).rowsperchunk = (*mem).last_rowsperchunk;
            (*sptr).cur_start_row = 0;
            (*sptr).first_undef_row = 0;
            (*sptr).dirty = FALSE;
        }
        sptr = (*sptr).next;
    }

    bptr = (*mem).virt_barray_list;
    while !bptr.is_null() {
        if (*bptr).mem_buffer.is_null() {
            let minheights = ((*bptr).rows_in_array as usize - 1) / (*bptr).maxaccess as usize + 1;
            if minheights <= max_minheights {
                (*bptr).rows_in_mem = (*bptr).rows_in_array;
            } else {
                (*bptr).rows_in_mem = (max_minheights as JDIMENSION) * (*bptr).maxaccess;
                jpeg_open_backing_store(
                    cinfo,
                    &mut (*bptr).b_s_info,
                    (*bptr).rows_in_array as ffi_types::long
                        * (*bptr).blocksperrow as ffi_types::long
                        * size_of::<JBLOCK>() as ffi_types::long,
                );
                (*bptr).b_s_open = TRUE;
            }
            (*bptr).mem_buffer = alloc_barray(
                cinfo,
                JPOOL_IMAGE,
                (*bptr).blocksperrow,
                (*bptr).rows_in_mem,
            );
            (*bptr).rowsperchunk = (*mem).last_rowsperchunk;
            (*bptr).cur_start_row = 0;
            (*bptr).first_undef_row = 0;
            (*bptr).dirty = FALSE;
        }
        bptr = (*bptr).next;
    }
}

unsafe fn do_sarray_io(cinfo: j_common_ptr, ptr_: *mut JVirtSArrayControl, writing: bool) {
    let bytesperrow =
        (*ptr_).samplesperrow as ffi_types::long * size_of::<JSAMPLE>() as ffi_types::long;
    let mut file_offset = (*ptr_).cur_start_row as ffi_types::long * bytesperrow;
    let mut i = 0i64;
    while i < (*ptr_).rows_in_mem as i64 {
        let mut rows = min_u32((*ptr_).rowsperchunk, (*ptr_).rows_in_mem - i as JDIMENSION) as i64;
        let thisrow = (*ptr_).cur_start_row as i64 + i;
        rows = rows.min((*ptr_).first_undef_row as i64 - thisrow);
        rows = rows.min((*ptr_).rows_in_array as i64 - thisrow);
        if rows <= 0 {
            break;
        }
        let byte_count = rows * bytesperrow;
        if writing {
            if let Some(write_backing_store) = (*ptr_).b_s_info.write_backing_store {
                write_backing_store(
                    cinfo,
                    &mut (*ptr_).b_s_info,
                    *(*ptr_).mem_buffer.add(i as usize) as *mut c_void,
                    file_offset,
                    byte_count,
                );
            }
        } else if let Some(read_backing_store) = (*ptr_).b_s_info.read_backing_store {
            read_backing_store(
                cinfo,
                &mut (*ptr_).b_s_info,
                *(*ptr_).mem_buffer.add(i as usize) as *mut c_void,
                file_offset,
                byte_count,
            );
        }
        file_offset += byte_count;
        i += (*ptr_).rowsperchunk as i64;
    }
}

unsafe fn do_barray_io(cinfo: j_common_ptr, ptr_: *mut JVirtBArrayControl, writing: bool) {
    let bytesperrow =
        (*ptr_).blocksperrow as ffi_types::long * size_of::<JBLOCK>() as ffi_types::long;
    let mut file_offset = (*ptr_).cur_start_row as ffi_types::long * bytesperrow;
    let mut i = 0i64;
    while i < (*ptr_).rows_in_mem as i64 {
        let mut rows = min_u32((*ptr_).rowsperchunk, (*ptr_).rows_in_mem - i as JDIMENSION) as i64;
        let thisrow = (*ptr_).cur_start_row as i64 + i;
        rows = rows.min((*ptr_).first_undef_row as i64 - thisrow);
        rows = rows.min((*ptr_).rows_in_array as i64 - thisrow);
        if rows <= 0 {
            break;
        }
        let byte_count = rows * bytesperrow;
        if writing {
            if let Some(write_backing_store) = (*ptr_).b_s_info.write_backing_store {
                write_backing_store(
                    cinfo,
                    &mut (*ptr_).b_s_info,
                    *(*ptr_).mem_buffer.add(i as usize) as *mut c_void,
                    file_offset,
                    byte_count,
                );
            }
        } else if let Some(read_backing_store) = (*ptr_).b_s_info.read_backing_store {
            read_backing_store(
                cinfo,
                &mut (*ptr_).b_s_info,
                *(*ptr_).mem_buffer.add(i as usize) as *mut c_void,
                file_offset,
                byte_count,
            );
        }
        file_offset += byte_count;
        i += (*ptr_).rowsperchunk as i64;
    }
}

unsafe extern "C" fn access_virt_sarray(
    cinfo: j_common_ptr,
    ptr_: jvirt_sarray_ptr,
    start_row: JDIMENSION,
    num_rows: JDIMENSION,
    writable: boolean,
) -> JSAMPARRAY {
    let ptr_ = ptr_ as *mut JVirtSArrayControl;
    let mut end_row = start_row + num_rows;

    if end_row > (*ptr_).rows_in_array
        || num_rows > (*ptr_).maxaccess
        || (*ptr_).mem_buffer.is_null()
    {
        error::errexit(cinfo, J_MESSAGE_CODE::JERR_BAD_VIRTUAL_ACCESS);
    }

    if start_row < (*ptr_).cur_start_row || end_row > (*ptr_).cur_start_row + (*ptr_).rows_in_mem {
        if (*ptr_).b_s_open == FALSE {
            error::errexit(cinfo, J_MESSAGE_CODE::JERR_VIRTUAL_BUG);
        }
        if (*ptr_).dirty != FALSE {
            do_sarray_io(cinfo, ptr_, true);
            (*ptr_).dirty = FALSE;
        }
        if start_row > (*ptr_).cur_start_row {
            (*ptr_).cur_start_row = start_row;
        } else {
            let ltemp = (end_row as i64 - (*ptr_).rows_in_mem as i64).max(0);
            (*ptr_).cur_start_row = ltemp as JDIMENSION;
        }
        do_sarray_io(cinfo, ptr_, false);
    }

    if (*ptr_).first_undef_row < end_row {
        let mut undef_row = if (*ptr_).first_undef_row < start_row {
            if writable != FALSE {
                error::errexit(cinfo, J_MESSAGE_CODE::JERR_BAD_VIRTUAL_ACCESS);
            }
            start_row
        } else {
            (*ptr_).first_undef_row
        };
        if writable != FALSE {
            (*ptr_).first_undef_row = end_row;
        }
        if (*ptr_).pre_zero != FALSE {
            let bytesperrow = (*ptr_).samplesperrow as usize * size_of::<JSAMPLE>();
            undef_row -= (*ptr_).cur_start_row;
            end_row -= (*ptr_).cur_start_row;
            while undef_row < end_row {
                crate::common::utils::zero_far(
                    *(*ptr_).mem_buffer.add(undef_row as usize) as *mut c_void,
                    bytesperrow,
                );
                undef_row += 1;
            }
        } else if writable == FALSE {
            error::errexit(cinfo, J_MESSAGE_CODE::JERR_BAD_VIRTUAL_ACCESS);
        }
    }

    if writable != FALSE {
        (*ptr_).dirty = TRUE;
    }
    (*ptr_)
        .mem_buffer
        .add((start_row - (*ptr_).cur_start_row) as usize)
}

unsafe extern "C" fn access_virt_barray(
    cinfo: j_common_ptr,
    ptr_: jvirt_barray_ptr,
    start_row: JDIMENSION,
    num_rows: JDIMENSION,
    writable: boolean,
) -> JBLOCKARRAY {
    let ptr_ = ptr_ as *mut JVirtBArrayControl;
    let mut end_row = start_row + num_rows;

    if end_row > (*ptr_).rows_in_array
        || num_rows > (*ptr_).maxaccess
        || (*ptr_).mem_buffer.is_null()
    {
        error::errexit(cinfo, J_MESSAGE_CODE::JERR_BAD_VIRTUAL_ACCESS);
    }

    if start_row < (*ptr_).cur_start_row || end_row > (*ptr_).cur_start_row + (*ptr_).rows_in_mem {
        if (*ptr_).b_s_open == FALSE {
            error::errexit(cinfo, J_MESSAGE_CODE::JERR_VIRTUAL_BUG);
        }
        if (*ptr_).dirty != FALSE {
            do_barray_io(cinfo, ptr_, true);
            (*ptr_).dirty = FALSE;
        }
        if start_row > (*ptr_).cur_start_row {
            (*ptr_).cur_start_row = start_row;
        } else {
            let ltemp = (end_row as i64 - (*ptr_).rows_in_mem as i64).max(0);
            (*ptr_).cur_start_row = ltemp as JDIMENSION;
        }
        do_barray_io(cinfo, ptr_, false);
    }

    if (*ptr_).first_undef_row < end_row {
        let mut undef_row = if (*ptr_).first_undef_row < start_row {
            if writable != FALSE {
                error::errexit(cinfo, J_MESSAGE_CODE::JERR_BAD_VIRTUAL_ACCESS);
            }
            start_row
        } else {
            (*ptr_).first_undef_row
        };
        if writable != FALSE {
            (*ptr_).first_undef_row = end_row;
        }
        if (*ptr_).pre_zero != FALSE {
            let bytesperrow = (*ptr_).blocksperrow as usize * size_of::<JBLOCK>();
            undef_row -= (*ptr_).cur_start_row;
            end_row -= (*ptr_).cur_start_row;
            while undef_row < end_row {
                crate::common::utils::zero_far(
                    *(*ptr_).mem_buffer.add(undef_row as usize) as *mut c_void,
                    bytesperrow,
                );
                undef_row += 1;
            }
        } else if writable == FALSE {
            error::errexit(cinfo, J_MESSAGE_CODE::JERR_BAD_VIRTUAL_ACCESS);
        }
    }

    if writable != FALSE {
        (*ptr_).dirty = TRUE;
    }
    (*ptr_)
        .mem_buffer
        .add((start_row - (*ptr_).cur_start_row) as usize)
}

unsafe extern "C" fn free_pool(cinfo: j_common_ptr, pool_id: int) {
    let mem = mem(cinfo);
    if pool_id < 0 || pool_id as usize >= JPOOL_NUMPOOLS {
        error::errexit1(cinfo, J_MESSAGE_CODE::JERR_BAD_POOL_ID, pool_id);
    }

    if pool_id == JPOOL_IMAGE {
        let mut sptr = (*mem).virt_sarray_list;
        while !sptr.is_null() {
            if (*sptr).b_s_open != FALSE {
                (*sptr).b_s_open = FALSE;
                if let Some(close_backing_store) = (*sptr).b_s_info.close_backing_store {
                    close_backing_store(cinfo, &mut (*sptr).b_s_info);
                }
            }
            sptr = (*sptr).next;
        }
        (*mem).virt_sarray_list = ptr::null_mut();

        let mut bptr = (*mem).virt_barray_list;
        while !bptr.is_null() {
            if (*bptr).b_s_open != FALSE {
                (*bptr).b_s_open = FALSE;
                if let Some(close_backing_store) = (*bptr).b_s_info.close_backing_store {
                    close_backing_store(cinfo, &mut (*bptr).b_s_info);
                }
            }
            bptr = (*bptr).next;
        }
        (*mem).virt_barray_list = ptr::null_mut();
    }

    let mut lhdr_ptr = (*mem).large_list[pool_id as usize];
    (*mem).large_list[pool_id as usize] = ptr::null_mut();
    while !lhdr_ptr.is_null() {
        let next = (*lhdr_ptr).next;
        let space_freed = (*lhdr_ptr).bytes_used
            + (*lhdr_ptr).bytes_left
            + size_of::<LargePoolHdr>()
            + ALIGN_SIZE
            - 1;
        jpeg_free_large(cinfo, lhdr_ptr as *mut c_void, space_freed);
        (*mem).total_space_allocated -= space_freed;
        lhdr_ptr = next;
    }

    let mut shdr_ptr = (*mem).small_list[pool_id as usize];
    (*mem).small_list[pool_id as usize] = ptr::null_mut();
    while !shdr_ptr.is_null() {
        let next = (*shdr_ptr).next;
        let space_freed = (*shdr_ptr).bytes_used
            + (*shdr_ptr).bytes_left
            + size_of::<SmallPoolHdr>()
            + ALIGN_SIZE
            - 1;
        jpeg_free_small(cinfo, shdr_ptr as *mut c_void, space_freed);
        (*mem).total_space_allocated -= space_freed;
        shdr_ptr = next;
    }
}

unsafe extern "C" fn self_destruct(cinfo: j_common_ptr) {
    let mut pool = JPOOL_NUMPOOLS as int - 1;
    while pool >= ffi_types::JPOOL_PERMANENT {
        free_pool(cinfo, pool);
        pool -= 1;
    }
    jpeg_free_small(cinfo, (*cinfo).mem as *mut c_void, size_of::<MyMemoryMgr>());
    (*cinfo).mem = ptr::null_mut();
    jpeg_mem_term(cinfo);
}

unsafe fn parse_jpegmem() -> Option<ffi_types::long> {
    let value = getenv(b"JPEGMEM\0".as_ptr() as *const i8);
    if value.is_null() || *value == 0 {
        return None;
    }
    let mut cursor = value;
    let mut result: ffi_types::long = 0;
    let mut any = false;
    while *cursor >= b'0' as i8 && *cursor <= b'9' as i8 {
        any = true;
        result = result
            .saturating_mul(10)
            .saturating_add((*cursor - b'0' as i8) as ffi_types::long);
        cursor = cursor.add(1);
    }
    if !any {
        return None;
    }
    if *cursor == b'm' as i8 || *cursor == b'M' as i8 {
        result = result.saturating_mul(1000);
    }
    Some(result.saturating_mul(1000))
}

pub unsafe fn jinit_memory_mgr(cinfo: j_common_ptr) {
    (*cinfo).mem = ptr::null_mut();
    if (ALIGN_SIZE & (ALIGN_SIZE - 1)) != 0 {
        error::errexit(cinfo, J_MESSAGE_CODE::JERR_BAD_ALIGN_TYPE);
    }
    let test_mac = MAX_ALLOC_CHUNK as usize;
    if test_mac as ffi_types::long != MAX_ALLOC_CHUNK
        || (MAX_ALLOC_CHUNK as usize % ALIGN_SIZE) != 0
    {
        error::errexit(cinfo, J_MESSAGE_CODE::JERR_BAD_ALLOC_CHUNK);
    }

    let max_to_use = jpeg_mem_init(cinfo);
    let mem = jpeg_get_small(cinfo, size_of::<MyMemoryMgr>()) as *mut MyMemoryMgr;
    if mem.is_null() {
        jpeg_mem_term(cinfo);
        error::errexit1(cinfo, J_MESSAGE_CODE::JERR_OUT_OF_MEMORY, 0);
    }

    ptr::write_bytes(mem as *mut u8, 0, size_of::<MyMemoryMgr>());
    (*mem).pub_.alloc_small = Some(alloc_small);
    (*mem).pub_.alloc_large = Some(alloc_large);
    (*mem).pub_.alloc_sarray = Some(alloc_sarray);
    (*mem).pub_.alloc_barray = Some(alloc_barray);
    (*mem).pub_.request_virt_sarray = Some(request_virt_sarray);
    (*mem).pub_.request_virt_barray = Some(request_virt_barray);
    (*mem).pub_.realize_virt_arrays = Some(realize_virt_arrays);
    (*mem).pub_.access_virt_sarray = Some(access_virt_sarray);
    (*mem).pub_.access_virt_barray = Some(access_virt_barray);
    (*mem).pub_.free_pool = Some(free_pool);
    (*mem).pub_.self_destruct = Some(self_destruct);
    (*mem).pub_.max_alloc_chunk = MAX_ALLOC_CHUNK;
    (*mem).pub_.max_memory_to_use = max_to_use;
    (*mem).total_space_allocated = size_of::<MyMemoryMgr>();
    (*cinfo).mem = &mut (*mem).pub_;

    if let Some(jpegmem) = parse_jpegmem() {
        (*mem).pub_.max_memory_to_use = jpegmem;
    }
}
