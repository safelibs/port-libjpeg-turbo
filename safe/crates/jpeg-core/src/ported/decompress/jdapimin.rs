use core::{mem::size_of, ptr};

use ffi_types::{
    boolean, int, j_common_ptr, j_decompress_ptr, jpeg_decompress_struct, JCS_YCbCr,
    DSTATE_INHEADER, DSTATE_RAW_OK, DSTATE_READY, DSTATE_SCANNING, DSTATE_START, DSTATE_STOPPING,
    FALSE, JCS_CMYK, JCS_GRAYSCALE, JCS_RGB, JCS_UNKNOWN, JCS_YCCK, JDCT_DEFAULT, JDITHER_FS,
    JHUFF_TBL, JPEG_HEADER_OK, JPEG_HEADER_TABLES_ONLY, JPEG_LIB_VERSION, JPEG_REACHED_EOI,
    JPEG_REACHED_SOS, JPEG_SUSPENDED, JPOOL_PERMANENT, J_MESSAGE_CODE, NUM_HUFF_TBLS, TRUE,
};

use crate::{
    common::{error, memory, registry},
    ported::decompress::{jdinput, jdmarker, jdmaster::my_decomp_master},
};

#[allow(non_snake_case)]
pub unsafe fn jpeg_CreateDecompress(cinfo: j_decompress_ptr, version: int, structsize: usize) {
    registry::clear_decompress_policy(cinfo);
    (*cinfo).mem = ptr::null_mut();
    if version != JPEG_LIB_VERSION {
        error::errexit2(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_LIB_VERSION,
            JPEG_LIB_VERSION,
            version,
        );
    }
    if structsize != size_of::<jpeg_decompress_struct>() {
        error::errexit2(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STRUCT_SIZE,
            size_of::<jpeg_decompress_struct>() as int,
            structsize as int,
        );
    }

    let err = (*cinfo).err;
    let client_data = (*cinfo).client_data;
    ptr::write_bytes(cinfo, 0, 1);
    (*cinfo).err = err;
    (*cinfo).client_data = client_data;
    (*cinfo).is_decompressor = TRUE;

    memory::jinit_memory_mgr(cinfo as j_common_ptr);
    (*cinfo).progress = ptr::null_mut();
    (*cinfo).src = ptr::null_mut();

    for slot in &mut (*cinfo).quant_tbl_ptrs {
        *slot = ptr::null_mut();
    }
    for i in 0..NUM_HUFF_TBLS {
        (*cinfo).dc_huff_tbl_ptrs[i] = ptr::null_mut::<JHUFF_TBL>();
        (*cinfo).ac_huff_tbl_ptrs[i] = ptr::null_mut::<JHUFF_TBL>();
    }

    (*cinfo).marker_list = ptr::null_mut();
    jdmarker::jinit_marker_reader(cinfo);
    jdinput::jinit_input_controller(cinfo);
    (*cinfo).global_state = DSTATE_START;

    let alloc_small = (*(*cinfo).mem).alloc_small.unwrap();
    (*cinfo).master = alloc_small(
        cinfo as j_common_ptr,
        JPOOL_PERMANENT,
        size_of::<my_decomp_master>(),
    ) as *mut _;
    ptr::write_bytes((*cinfo).master.cast::<my_decomp_master>(), 0, 1);
}

pub unsafe fn jpeg_destroy_decompress(cinfo: j_decompress_ptr) {
    error::destroy(cinfo as j_common_ptr);
}

pub unsafe fn jpeg_abort_decompress(cinfo: j_decompress_ptr) {
    error::abort(cinfo as j_common_ptr);
}

unsafe fn default_decompress_parms(cinfo: j_decompress_ptr) {
    match (*cinfo).num_components {
        1 => {
            (*cinfo).jpeg_color_space = JCS_GRAYSCALE;
            (*cinfo).out_color_space = JCS_GRAYSCALE;
        }
        3 => {
            if (*cinfo).saw_JFIF_marker != FALSE {
                (*cinfo).jpeg_color_space = JCS_YCbCr;
            } else if (*cinfo).saw_Adobe_marker != FALSE {
                match (*cinfo).Adobe_transform as int {
                    0 => (*cinfo).jpeg_color_space = JCS_RGB,
                    1 => (*cinfo).jpeg_color_space = JCS_YCbCr,
                    other => {
                        error::warnms1(
                            cinfo as j_common_ptr,
                            J_MESSAGE_CODE::JWRN_ADOBE_XFORM,
                            other,
                        );
                        (*cinfo).jpeg_color_space = JCS_YCbCr;
                    }
                }
            } else {
                let cid0 = (*(*cinfo).comp_info.add(0)).component_id;
                let cid1 = (*(*cinfo).comp_info.add(1)).component_id;
                let cid2 = (*(*cinfo).comp_info.add(2)).component_id;
                if cid0 == 1 && cid1 == 2 && cid2 == 3 {
                    (*cinfo).jpeg_color_space = JCS_YCbCr;
                } else if cid0 == 82 && cid1 == 71 && cid2 == 66 {
                    (*cinfo).jpeg_color_space = JCS_RGB;
                } else {
                    error::tracems3(
                        cinfo as j_common_ptr,
                        1,
                        J_MESSAGE_CODE::JTRC_UNKNOWN_IDS,
                        cid0,
                        cid1,
                        cid2,
                    );
                    (*cinfo).jpeg_color_space = JCS_YCbCr;
                }
            }
            (*cinfo).out_color_space = JCS_RGB;
        }
        4 => {
            if (*cinfo).saw_Adobe_marker != FALSE {
                match (*cinfo).Adobe_transform as int {
                    0 => (*cinfo).jpeg_color_space = JCS_CMYK,
                    2 => (*cinfo).jpeg_color_space = JCS_YCCK,
                    other => {
                        error::warnms1(
                            cinfo as j_common_ptr,
                            J_MESSAGE_CODE::JWRN_ADOBE_XFORM,
                            other,
                        );
                        (*cinfo).jpeg_color_space = JCS_YCCK;
                    }
                }
            } else {
                (*cinfo).jpeg_color_space = JCS_CMYK;
            }
            (*cinfo).out_color_space = JCS_CMYK;
        }
        _ => {
            (*cinfo).jpeg_color_space = JCS_UNKNOWN;
            (*cinfo).out_color_space = JCS_UNKNOWN;
        }
    }

    (*cinfo).scale_num = 1;
    (*cinfo).scale_denom = 1;
    (*cinfo).output_gamma = 1.0;
    (*cinfo).buffered_image = FALSE;
    (*cinfo).raw_data_out = FALSE;
    (*cinfo).dct_method = JDCT_DEFAULT;
    (*cinfo).do_fancy_upsampling = TRUE;
    (*cinfo).do_block_smoothing = TRUE;
    (*cinfo).quantize_colors = FALSE;
    (*cinfo).dither_mode = JDITHER_FS;
    (*cinfo).two_pass_quantize = TRUE;
    (*cinfo).desired_number_of_colors = 256;
    (*cinfo).colormap = ptr::null_mut();
    (*cinfo).enable_1pass_quant = FALSE;
    (*cinfo).enable_external_quant = FALSE;
    (*cinfo).enable_2pass_quant = FALSE;
}

#[inline]
unsafe fn consume_input_until_ready(cinfo: j_decompress_ptr) -> int {
    let retcode = (*(*cinfo).inputctl).consume_input.unwrap()(cinfo);
    if retcode == JPEG_REACHED_SOS {
        default_decompress_parms(cinfo);
        (*cinfo).global_state = DSTATE_READY;
    }
    retcode
}

pub unsafe fn jpeg_read_header(cinfo: j_decompress_ptr, require_image: boolean) -> int {
    if (*cinfo).global_state != DSTATE_START && (*cinfo).global_state != DSTATE_INHEADER {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }

    let mut retcode = jpeg_consume_input(cinfo);
    match retcode {
        JPEG_REACHED_SOS => {
            retcode = JPEG_HEADER_OK;
        }
        JPEG_REACHED_EOI => {
            if require_image != FALSE {
                error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_NO_IMAGE);
            }
            error::abort(cinfo as j_common_ptr);
            retcode = JPEG_HEADER_TABLES_ONLY;
        }
        JPEG_SUSPENDED => {}
        _ => {}
    }
    retcode
}

pub unsafe fn jpeg_consume_input(cinfo: j_decompress_ptr) -> int {
    match (*cinfo).global_state {
        DSTATE_START => {
            (*(*cinfo).inputctl).reset_input_controller.unwrap()(cinfo);
            (*(*cinfo).src).init_source.unwrap()(cinfo);
            (*cinfo).global_state = DSTATE_INHEADER;
            consume_input_until_ready(cinfo)
        }
        DSTATE_INHEADER => consume_input_until_ready(cinfo),
        DSTATE_READY => JPEG_REACHED_SOS,
        ffi_types::DSTATE_PRELOAD
        | ffi_types::DSTATE_PRESCAN
        | DSTATE_SCANNING
        | DSTATE_RAW_OK
        | ffi_types::DSTATE_BUFIMAGE
        | ffi_types::DSTATE_BUFPOST
        | DSTATE_STOPPING => (*(*cinfo).inputctl).consume_input.unwrap()(cinfo),
        _ => error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        ),
    }
}

pub unsafe fn jpeg_input_complete(cinfo: j_decompress_ptr) -> boolean {
    if (*cinfo).global_state < DSTATE_START || (*cinfo).global_state > DSTATE_STOPPING {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }
    (*(*cinfo).inputctl).eoi_reached
}

pub unsafe fn jpeg_has_multiple_scans(cinfo: j_decompress_ptr) -> boolean {
    if (*cinfo).global_state < DSTATE_READY || (*cinfo).global_state > DSTATE_STOPPING {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }
    (*(*cinfo).inputctl).has_multiple_scans
}

pub unsafe fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean {
    if ((*cinfo).global_state == DSTATE_SCANNING || (*cinfo).global_state == DSTATE_RAW_OK)
        && (*cinfo).buffered_image == FALSE
    {
        if (*cinfo).output_scanline < (*cinfo).output_height {
            error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_TOO_LITTLE_DATA);
        }
        (*(*cinfo).master).finish_output_pass.unwrap()(cinfo);
        (*cinfo).global_state = DSTATE_STOPPING;
    } else if (*cinfo).global_state == ffi_types::DSTATE_BUFIMAGE {
        (*cinfo).global_state = DSTATE_STOPPING;
    } else if (*cinfo).global_state != DSTATE_STOPPING {
        error::errexit1(
            cinfo as j_common_ptr,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            (*cinfo).global_state,
        );
    }

    while (*(*cinfo).inputctl).eoi_reached == FALSE {
        if (*(*cinfo).inputctl).consume_input.unwrap()(cinfo) == JPEG_SUSPENDED {
            return FALSE;
        }
    }

    (*(*cinfo).src).term_source.unwrap()(cinfo);
    error::abort(cinfo as j_common_ptr);
    TRUE
}
