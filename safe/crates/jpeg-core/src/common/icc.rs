use core::{ffi::c_void, ptr};

use ffi_types::{
    boolean, int, j_compress_ptr, j_decompress_ptr, jpeg_saved_marker_ptr, JOCTET, J_MESSAGE_CODE,
    CSTATE_SCANNING, DSTATE_READY, JPEG_APP0, FALSE, TRUE,
};

use crate::common::error;

const ICC_MARKER: int = JPEG_APP0 + 2;
const ICC_OVERHEAD_LEN: usize = 14;
const MAX_BYTES_IN_MARKER: usize = 65533;
const MAX_DATA_BYTES_IN_MARKER: usize = MAX_BYTES_IN_MARKER - ICC_OVERHEAD_LEN;
const MAX_SEQ_NO: usize = 255;

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn jpeg_write_m_header(cinfo: j_compress_ptr, marker: int, datalen: ::core::ffi::c_uint);
    fn jpeg_write_m_byte(cinfo: j_compress_ptr, val: int);
}

#[inline]
unsafe fn marker_is_icc(marker: jpeg_saved_marker_ptr) -> bool {
    !marker.is_null()
        && (*marker).marker as int == ICC_MARKER
        && (*marker).data_length as usize >= ICC_OVERHEAD_LEN
        && *(*marker).data.add(0) == 0x49
        && *(*marker).data.add(1) == 0x43
        && *(*marker).data.add(2) == 0x43
        && *(*marker).data.add(3) == 0x5F
        && *(*marker).data.add(4) == 0x50
        && *(*marker).data.add(5) == 0x52
        && *(*marker).data.add(6) == 0x4F
        && *(*marker).data.add(7) == 0x46
        && *(*marker).data.add(8) == 0x49
        && *(*marker).data.add(9) == 0x4C
        && *(*marker).data.add(10) == 0x45
        && *(*marker).data.add(11) == 0x00
}

pub unsafe fn jpeg_write_icc_profile(
    cinfo: j_compress_ptr,
    mut icc_data_ptr: *const JOCTET,
    mut icc_data_len: ::core::ffi::c_uint,
) {
    if icc_data_ptr.is_null() || icc_data_len == 0 {
        error::errexit(cinfo as ffi_types::j_common_ptr, J_MESSAGE_CODE::JERR_BUFFER_SIZE);
    }
    if (*cinfo).global_state < CSTATE_SCANNING {
        error::errexit1(
            cinfo as ffi_types::j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }

    let mut num_markers = icc_data_len as usize / MAX_DATA_BYTES_IN_MARKER;
    if num_markers * MAX_DATA_BYTES_IN_MARKER != icc_data_len as usize {
        num_markers += 1;
    }
    let mut cur_marker = 1;

    while icc_data_len > 0 {
        let mut length = icc_data_len as usize;
        if length > MAX_DATA_BYTES_IN_MARKER {
            length = MAX_DATA_BYTES_IN_MARKER;
        }
        icc_data_len -= length as ::core::ffi::c_uint;
        jpeg_write_m_header(cinfo, ICC_MARKER, (length + ICC_OVERHEAD_LEN) as ::core::ffi::c_uint);
        for byte in [0x49, 0x43, 0x43, 0x5F, 0x50, 0x52, 0x4F, 0x46, 0x49, 0x4C, 0x45, 0x00] {
            jpeg_write_m_byte(cinfo, byte);
        }
        jpeg_write_m_byte(cinfo, cur_marker);
        jpeg_write_m_byte(cinfo, num_markers as int);
        while length > 0 {
            jpeg_write_m_byte(cinfo, *icc_data_ptr as int);
            icc_data_ptr = icc_data_ptr.add(1);
            length -= 1;
        }
        cur_marker += 1;
    }
}

pub unsafe fn jpeg_read_icc_profile(
    cinfo: j_decompress_ptr,
    icc_data_ptr: *mut *mut JOCTET,
    icc_data_len: *mut ::core::ffi::c_uint,
) -> boolean {
    if icc_data_ptr.is_null() || icc_data_len.is_null() {
        error::errexit(cinfo as ffi_types::j_common_ptr, J_MESSAGE_CODE::JERR_BUFFER_SIZE);
    }
    if (*cinfo).global_state < DSTATE_READY {
        error::errexit1(
            cinfo as ffi_types::j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }

    *icc_data_ptr = ptr::null_mut();
    *icc_data_len = 0;

    let mut num_markers = 0usize;
    let mut marker_present = [0u8; MAX_SEQ_NO + 1];
    let mut data_length = [0u32; MAX_SEQ_NO + 1];
    let mut data_offset = [0u32; MAX_SEQ_NO + 1];

    let mut marker = (*cinfo).marker_list;
    while !marker.is_null() {
        if marker_is_icc(marker) {
            let count = *(*marker).data.add(13) as usize;
            if num_markers == 0 {
                num_markers = count;
            } else if num_markers != count {
                error::warnms(cinfo as ffi_types::j_common_ptr, J_MESSAGE_CODE::JWRN_BOGUS_ICC);
                return FALSE;
            }
            let seq_no = *(*marker).data.add(12) as usize;
            if seq_no == 0 || seq_no > num_markers || marker_present[seq_no] != 0 {
                error::warnms(cinfo as ffi_types::j_common_ptr, J_MESSAGE_CODE::JWRN_BOGUS_ICC);
                return FALSE;
            }
            marker_present[seq_no] = 1;
            data_length[seq_no] = (*marker).data_length - ICC_OVERHEAD_LEN as u32;
        }
        marker = (*marker).next;
    }

    if num_markers == 0 {
        return FALSE;
    }

    let mut total_length = 0u32;
    let mut seq_no = 1usize;
    while seq_no <= num_markers {
        if marker_present[seq_no] == 0 {
            error::warnms(cinfo as ffi_types::j_common_ptr, J_MESSAGE_CODE::JWRN_BOGUS_ICC);
            return FALSE;
        }
        data_offset[seq_no] = total_length;
        total_length = match total_length.checked_add(data_length[seq_no]) {
            Some(length) => length,
            None => {
                error::warnms(cinfo as ffi_types::j_common_ptr, J_MESSAGE_CODE::JWRN_BOGUS_ICC);
                return FALSE;
            }
        };
        seq_no += 1;
    }

    if total_length == 0 {
        error::warnms(cinfo as ffi_types::j_common_ptr, J_MESSAGE_CODE::JWRN_BOGUS_ICC);
        return FALSE;
    }

    let icc_data = malloc(total_length as usize) as *mut JOCTET;
    if icc_data.is_null() {
        error::errexit1(cinfo as ffi_types::j_common_ptr, J_MESSAGE_CODE::JERR_OUT_OF_MEMORY, 11);
    }

    marker = (*cinfo).marker_list;
    while !marker.is_null() {
        if marker_is_icc(marker) {
            let seq_no = *(*marker).data.add(12) as usize;
            let mut dst = icc_data.add(data_offset[seq_no] as usize);
            let mut src = (*marker).data.add(ICC_OVERHEAD_LEN);
            let mut length = data_length[seq_no];
            while length > 0 {
                *dst = *src;
                dst = dst.add(1);
                src = src.add(1);
                length -= 1;
            }
        }
        marker = (*marker).next;
    }

    *icc_data_ptr = icc_data;
    *icc_data_len = total_length;
    TRUE
}
