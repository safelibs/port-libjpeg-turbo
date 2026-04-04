use ffi_types::{boolean, int, j_decompress_ptr};

#[allow(
    dead_code,
    improper_ctypes,
    improper_ctypes_definitions,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    clippy::all
)]
mod translated {
    include!("generated/jdhuff_translated.rs");
}

pub use translated::{bit_buf_type, bitread_working_state, d_derived_tbl};

#[inline]
unsafe fn translated_cinfo(cinfo: j_decompress_ptr) -> *mut translated::jpeg_decompress_struct {
    cinfo.cast::<translated::jpeg_decompress_struct>()
}

pub unsafe fn jinit_huff_decoder(cinfo: j_decompress_ptr) {
    translated::jinit_huff_decoder(translated_cinfo(cinfo))
}

pub unsafe extern "C" fn jpeg_make_d_derived_tbl(
    cinfo: j_decompress_ptr,
    is_dc: boolean,
    tblno: int,
    pdtbl: *mut *mut d_derived_tbl,
) {
    translated::jpeg_make_d_derived_tbl(translated_cinfo(cinfo), is_dc, tblno, pdtbl)
}

pub unsafe extern "C" fn jpeg_fill_bit_buffer(
    state: *mut bitread_working_state,
    get_buffer: bit_buf_type,
    bits_left: int,
    nbits: int,
) -> boolean {
    translated::jpeg_fill_bit_buffer(state, get_buffer, bits_left, nbits)
}

pub unsafe extern "C" fn jpeg_huff_decode(
    state: *mut bitread_working_state,
    get_buffer: bit_buf_type,
    bits_left: int,
    htbl: *mut d_derived_tbl,
    min_bits: int,
) -> int {
    translated::jpeg_huff_decode(state, get_buffer, bits_left, htbl, min_bits)
}
