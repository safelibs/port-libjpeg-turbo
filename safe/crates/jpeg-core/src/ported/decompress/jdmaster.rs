use core::ptr;

use ffi_types::{
    boolean, int, j_common_ptr, j_decompress_ptr, jpeg_color_quantizer, jpeg_decomp_master,
    JCS_YCbCr, CENTERJSAMPLE, FALSE, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_SAVE_AND_PASS, JCS_CMYK,
    JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA,
    JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_YCCK,
    JDIMENSION, J_MESSAGE_CODE, MAXJSAMPLE, TRUE,
};

use crate::{
    common::error,
    ported::decompress::{
        jdcoefct, jdcolext, jdcolor, jddctmgr, jdhuff, jdmainct, jdmerge, jdsample,
    },
};

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
    include!("generated/jdmaster_translated.rs");
}

#[repr(C)]
pub struct my_decomp_master {
    pub pub_: jpeg_decomp_master,
    pub pass_number: int,
    pub using_merged_upsample: boolean,
    pub quantizer_1pass: *mut jpeg_color_quantizer,
    pub quantizer_2pass: *mut jpeg_color_quantizer,
}

extern "C" {
    #[link_name = "jinit_1pass_quantizer"]
    fn c_jinit_1pass_quantizer(cinfo: j_decompress_ptr);
    #[link_name = "jinit_2pass_quantizer"]
    fn c_jinit_2pass_quantizer(cinfo: j_decompress_ptr);
    #[link_name = "jinit_d_post_controller"]
    fn c_jinit_d_post_controller(cinfo: j_decompress_ptr, need_full_buffer: boolean);
    #[link_name = "jinit_arith_decoder"]
    fn c_jinit_arith_decoder(cinfo: j_decompress_ptr);
    #[link_name = "jinit_phuff_decoder"]
    fn c_jinit_phuff_decoder(cinfo: j_decompress_ptr);
}

pub unsafe extern "C" fn jpeg_calc_output_dimensions(cinfo: j_decompress_ptr) {
    translated::jpeg_calc_output_dimensions(cinfo.cast::<translated::jpeg_decompress_struct>())
}

pub unsafe extern "C" fn jpeg_core_output_dimensions(cinfo: j_decompress_ptr) {
    translated::jpeg_core_output_dimensions(cinfo.cast::<translated::jpeg_decompress_struct>())
}

pub unsafe extern "C" fn jpeg_new_colormap(cinfo: j_decompress_ptr) {
    translated::jpeg_new_colormap(cinfo.cast::<translated::jpeg_decompress_struct>())
}

#[inline]
unsafe fn need_full_image_buffer(cinfo: j_decompress_ptr) -> boolean {
    if (*(*cinfo).inputctl).has_multiple_scans != FALSE || (*cinfo).buffered_image != FALSE {
        TRUE
    } else {
        FALSE
    }
}

unsafe fn use_merged_upsample(cinfo: j_decompress_ptr) -> boolean {
    if (*cinfo).do_fancy_upsampling != FALSE || (*cinfo).CCIR601_sampling != FALSE {
        return FALSE;
    }
    if (*cinfo).jpeg_color_space != JCS_YCbCr || (*cinfo).num_components != 3 {
        return FALSE;
    }

    match (*cinfo).out_color_space {
        JCS_RGB | JCS_RGB565 | JCS_EXT_RGB | JCS_EXT_RGBX | JCS_EXT_BGR | JCS_EXT_BGRX
        | JCS_EXT_XBGR | JCS_EXT_XRGB | JCS_EXT_RGBA | JCS_EXT_BGRA | JCS_EXT_ABGR
        | JCS_EXT_ARGB => {}
        _ => return FALSE,
    }

    let out_idx = (*cinfo).out_color_space as usize;
    let rgb_pixel_size = jdcolext::rgb_pixelsize_for(out_idx);
    if ((*cinfo).out_color_space == JCS_RGB565 && (*cinfo).out_color_components != 3)
        || ((*cinfo).out_color_space != JCS_RGB565
            && ((*cinfo).out_color_components != rgb_pixel_size))
    {
        return FALSE;
    }

    let comp0 = (*cinfo).comp_info.add(0);
    let comp1 = (*cinfo).comp_info.add(1);
    let comp2 = (*cinfo).comp_info.add(2);
    if (*comp0).h_samp_factor != 2
        || (*comp1).h_samp_factor != 1
        || (*comp2).h_samp_factor != 1
        || (*comp0).v_samp_factor > 2
        || (*comp1).v_samp_factor != 1
        || (*comp2).v_samp_factor != 1
    {
        return FALSE;
    }

    if (*comp0).DCT_h_scaled_size != (*cinfo).min_DCT_h_scaled_size
        || (*comp1).DCT_h_scaled_size != (*cinfo).min_DCT_h_scaled_size
        || (*comp2).DCT_h_scaled_size != (*cinfo).min_DCT_h_scaled_size
    {
        return FALSE;
    }

    TRUE
}

unsafe fn prepare_range_limit_table(cinfo: j_decompress_ptr) {
    let table = (*(*cinfo).mem).alloc_small.unwrap()(
        cinfo as j_common_ptr,
        ffi_types::JPOOL_IMAGE,
        (5 * (MAXJSAMPLE as usize + 1) + CENTERJSAMPLE as usize),
    ) as *mut u8;
    let table = table.add(MAXJSAMPLE as usize + 1);
    (*cinfo).sample_range_limit = table;
    ptr::write_bytes(
        table.sub(MAXJSAMPLE as usize + 1),
        0,
        MAXJSAMPLE as usize + 1,
    );
    for i in 0..=MAXJSAMPLE as usize {
        *table.add(i) = i as u8;
    }
    let table = table.add(CENTERJSAMPLE as usize);
    for i in CENTERJSAMPLE as usize..2 * (MAXJSAMPLE as usize + 1) {
        *table.add(i) = MAXJSAMPLE as u8;
    }
    ptr::write_bytes(
        table.add(2 * (MAXJSAMPLE as usize + 1)),
        0,
        2 * (MAXJSAMPLE as usize + 1) - CENTERJSAMPLE as usize,
    );
    ptr::copy_nonoverlapping(
        (*cinfo).sample_range_limit,
        table.add(4 * (MAXJSAMPLE as usize + 1) - CENTERJSAMPLE as usize),
        CENTERJSAMPLE as usize,
    );
}

unsafe fn master_selection(cinfo: j_decompress_ptr) {
    let master = (*cinfo).master as *mut my_decomp_master;

    jpeg_calc_output_dimensions(cinfo);
    prepare_range_limit_table(cinfo);

    let samplesperrow =
        (*cinfo).output_width as ffi_types::long * (*cinfo).out_color_components as ffi_types::long;
    let jd_samplesperrow = samplesperrow as JDIMENSION;
    if jd_samplesperrow as ffi_types::long != samplesperrow {
        error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_WIDTH_OVERFLOW);
    }

    (*master).pass_number = 0;
    (*master).using_merged_upsample = use_merged_upsample(cinfo);
    (*master).quantizer_1pass = ptr::null_mut();
    (*master).quantizer_2pass = ptr::null_mut();

    if (*cinfo).quantize_colors == FALSE || (*cinfo).buffered_image == FALSE {
        (*cinfo).enable_1pass_quant = FALSE;
        (*cinfo).enable_external_quant = FALSE;
        (*cinfo).enable_2pass_quant = FALSE;
    }

    if (*cinfo).quantize_colors != FALSE {
        if (*cinfo).raw_data_out != FALSE {
            error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_NOTIMPL);
        }
        if (*cinfo).out_color_components != 3 {
            (*cinfo).enable_1pass_quant = TRUE;
            (*cinfo).enable_external_quant = FALSE;
            (*cinfo).enable_2pass_quant = FALSE;
            (*cinfo).colormap = ptr::null_mut();
        } else if !(*cinfo).colormap.is_null() {
            (*cinfo).enable_external_quant = TRUE;
        } else if (*cinfo).two_pass_quantize != FALSE {
            (*cinfo).enable_2pass_quant = TRUE;
        } else {
            (*cinfo).enable_1pass_quant = TRUE;
        }

        if (*cinfo).enable_1pass_quant != FALSE {
            c_jinit_1pass_quantizer(cinfo);
            (*master).quantizer_1pass = (*cinfo).cquantize;
        }

        if (*cinfo).enable_2pass_quant != FALSE || (*cinfo).enable_external_quant != FALSE {
            c_jinit_2pass_quantizer(cinfo);
            (*master).quantizer_2pass = (*cinfo).cquantize;
        }
    }

    if (*cinfo).raw_data_out == FALSE {
        if (*master).using_merged_upsample != FALSE {
            jdmerge::jinit_merged_upsampler(cinfo);
        } else {
            jdcolor::jinit_color_deconverter(cinfo);
            jdsample::jinit_upsampler(cinfo);
        }
        c_jinit_d_post_controller(cinfo, (*cinfo).enable_2pass_quant);
    }

    jddctmgr::jinit_inverse_dct(cinfo);
    if (*cinfo).arith_code != FALSE {
        c_jinit_arith_decoder(cinfo);
    } else if (*cinfo).progressive_mode != FALSE {
        c_jinit_phuff_decoder(cinfo);
    } else {
        jdhuff::jinit_huff_decoder(cinfo);
    }

    jdcoefct::jinit_d_coef_controller(cinfo, need_full_image_buffer(cinfo));

    if (*cinfo).raw_data_out == FALSE {
        jdmainct::jinit_d_main_controller(cinfo, FALSE);
    }

    (*(*cinfo).mem).realize_virt_arrays.unwrap()(cinfo as j_common_ptr);
    (*(*cinfo).inputctl).start_input_pass.unwrap()(cinfo);
    (*(*cinfo).master).first_iMCU_col = 0;
    (*(*cinfo).master).last_iMCU_col = (*cinfo).MCUs_per_row - 1;
    (*(*cinfo).master).last_good_iMCU_row = 0;

    if !(*cinfo).progress.is_null()
        && (*cinfo).buffered_image == FALSE
        && (*(*cinfo).inputctl).has_multiple_scans != FALSE
    {
        let nscans = if (*cinfo).progressive_mode != FALSE {
            2 + 3 * (*cinfo).num_components
        } else {
            (*cinfo).num_components
        };
        (*(*cinfo).progress).pass_counter = 0;
        (*(*cinfo).progress).pass_limit =
            (*cinfo).total_iMCU_rows as ffi_types::long * nscans as ffi_types::long;
        (*(*cinfo).progress).completed_passes = 0;
        (*(*cinfo).progress).total_passes = if (*cinfo).enable_2pass_quant != FALSE {
            3
        } else {
            2
        };
        (*master).pass_number += 1;
    }
}

unsafe extern "C" fn prepare_for_output_pass(cinfo: j_decompress_ptr) {
    let master = (*cinfo).master as *mut my_decomp_master;

    if (*master).pub_.is_dummy_pass != FALSE {
        (*master).pub_.is_dummy_pass = FALSE;
        (*(*cinfo).cquantize).start_pass.unwrap()(cinfo, FALSE);
        (*(*cinfo).post).start_pass.unwrap()(cinfo, JBUF_CRANK_DEST);
        (*(*cinfo).main).start_pass.unwrap()(cinfo, JBUF_CRANK_DEST);
    } else {
        if (*cinfo).quantize_colors != FALSE && (*cinfo).colormap.is_null() {
            if (*cinfo).two_pass_quantize != FALSE && (*cinfo).enable_2pass_quant != FALSE {
                (*cinfo).cquantize = (*master).quantizer_2pass;
                (*master).pub_.is_dummy_pass = TRUE;
            } else if (*cinfo).enable_1pass_quant != FALSE {
                (*cinfo).cquantize = (*master).quantizer_1pass;
            } else {
                error::errexit(cinfo as j_common_ptr, J_MESSAGE_CODE::JERR_MODE_CHANGE);
            }
        }

        (*(*cinfo).idct).start_pass.unwrap()(cinfo);
        (*(*cinfo).coef).start_output_pass.unwrap()(cinfo);
        if (*cinfo).raw_data_out == FALSE {
            if (*master).using_merged_upsample == FALSE {
                (*(*cinfo).cconvert).start_pass.unwrap()(cinfo);
            }
            (*(*cinfo).upsample).start_pass.unwrap()(cinfo);
            if (*cinfo).quantize_colors != FALSE {
                (*(*cinfo).cquantize).start_pass.unwrap()(cinfo, (*master).pub_.is_dummy_pass);
            }
            (*(*cinfo).post).start_pass.unwrap()(
                cinfo,
                if (*master).pub_.is_dummy_pass != FALSE {
                    JBUF_SAVE_AND_PASS
                } else {
                    JBUF_PASS_THRU
                },
            );
            (*(*cinfo).main).start_pass.unwrap()(cinfo, JBUF_PASS_THRU);
        }
    }

    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).completed_passes = (*master).pass_number;
        (*(*cinfo).progress).total_passes = (*master).pass_number
            + if (*master).pub_.is_dummy_pass != FALSE {
                2
            } else {
                1
            };
        if (*cinfo).buffered_image != FALSE && (*(*cinfo).inputctl).eoi_reached == FALSE {
            (*(*cinfo).progress).total_passes += if (*cinfo).enable_2pass_quant != FALSE {
                2
            } else {
                1
            };
        }
    }
}

unsafe extern "C" fn finish_output_pass(cinfo: j_decompress_ptr) {
    let master = (*cinfo).master as *mut my_decomp_master;
    if (*cinfo).quantize_colors != FALSE {
        (*(*cinfo).cquantize).finish_pass.unwrap()(cinfo);
    }
    (*master).pass_number += 1;
}

pub unsafe fn jinit_master_decompress(cinfo: j_decompress_ptr) {
    let master = (*cinfo).master as *mut my_decomp_master;
    (*master).pub_.prepare_for_output_pass = Some(prepare_for_output_pass);
    (*master).pub_.finish_output_pass = Some(finish_output_pass);
    (*master).pub_.is_dummy_pass = FALSE;
    (*master).pub_.jinit_upsampler_no_alloc = FALSE;
    master_selection(cinfo);
}
