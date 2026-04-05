#![allow(clippy::all)]

use std::collections::BTreeMap;
use std::fs;
use std::mem::{align_of, offset_of, size_of};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ffi_types::*;

fn multiarch() -> String {
    for (program, args) in [
        ("dpkg-architecture", ["-qDEB_HOST_MULTIARCH"].as_slice()),
        ("gcc", ["-print-multiarch"].as_slice()),
    ] {
        if let Ok(output) = Command::new(program).args(args).output() {
            if output.status.success() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
                if !value.is_empty() {
                    return value;
                }
            }
        }
    }
    format!("{}-linux-gnu", std::env::consts::ARCH)
}

fn tempdir() -> PathBuf {
    let mut dir = std::env::temp_dir();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("libjpeg-layout-{stamp}"));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn ensure_stage_install() -> PathBuf {
    let safe_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let stage_dir = safe_root.join("stage");
    if std::env::var_os("LIBJPEG_TURBO_SKIP_STAGE_REFRESH").is_none() {
        let status = Command::new("bash")
            .env("CARGO_PROFILE_RELEASE_LTO", "false")
            .env("RUSTFLAGS", "-Clinker-plugin-lto=no")
            .arg(safe_root.join("scripts/stage-install.sh"))
            .arg("--clean")
            .arg("--stage-dir")
            .arg(&stage_dir)
            .status()
            .unwrap();
        assert!(status.success(), "failed to refresh staged install");
    }
    assert!(
        stage_dir.join("usr/include/jpeglib.h").is_file(),
        "missing staged headers under {}",
        stage_dir.display()
    );
    stage_dir
}

fn compile_probe(dir: &Path, stage_dir: &Path) -> PathBuf {
    let source = dir.join("probe.c");
    let binary = dir.join("probe");
    let include = stage_dir.join("usr/include");
    let include_multiarch = include.join(multiarch());
    let source_text = r#"
#include <stdio.h>
#include <stddef.h>
#define JPEG_INTERNALS
#include "jpeglib.h"
#include "jmemsys.h"

#define SIZEOF(type) do { printf("size:%s=%zu\n", #type, sizeof(type)); printf("align:%s=%zu\n", #type, __alignof__(type)); } while (0)
#define OFF(type, field) printf("off:%s.%s=%zu\n", #type, #field, offsetof(type, field))
#define CONST(name) printf("const:%s=%d\n", #name, (int)(name))

int main(void) {
  SIZEOF(struct jpeg_common_struct);
  SIZEOF(struct jpeg_compress_struct);
  SIZEOF(struct jpeg_decompress_struct);
  SIZEOF(struct jpeg_error_mgr);
  SIZEOF(struct jpeg_progress_mgr);
  SIZEOF(struct jpeg_destination_mgr);
  SIZEOF(struct jpeg_source_mgr);
  SIZEOF(struct jpeg_memory_mgr);
  SIZEOF(JQUANT_TBL);
  SIZEOF(JHUFF_TBL);
  SIZEOF(jpeg_component_info);
  SIZEOF(jpeg_scan_info);
  SIZEOF(struct jpeg_marker_struct);
  SIZEOF(struct jpeg_comp_master);
  SIZEOF(struct jpeg_c_main_controller);
  SIZEOF(struct jpeg_c_prep_controller);
  SIZEOF(struct jpeg_c_coef_controller);
  SIZEOF(struct jpeg_color_converter);
  SIZEOF(struct jpeg_downsampler);
  SIZEOF(struct jpeg_forward_dct);
  SIZEOF(struct jpeg_entropy_encoder);
  SIZEOF(struct jpeg_marker_writer);
  SIZEOF(struct jpeg_decomp_master);
  SIZEOF(struct jpeg_input_controller);
  SIZEOF(struct jpeg_d_main_controller);
  SIZEOF(struct jpeg_d_coef_controller);
  SIZEOF(struct jpeg_d_post_controller);
  SIZEOF(struct jpeg_marker_reader);
  SIZEOF(struct jpeg_entropy_decoder);
  SIZEOF(struct jpeg_inverse_dct);
  SIZEOF(struct jpeg_upsampler);
  SIZEOF(struct jpeg_color_deconverter);
  SIZEOF(struct jpeg_color_quantizer);

  OFF(struct jpeg_common_struct, err);
  OFF(struct jpeg_common_struct, mem);
  OFF(struct jpeg_common_struct, progress);
  OFF(struct jpeg_common_struct, client_data);
  OFF(struct jpeg_common_struct, is_decompressor);
  OFF(struct jpeg_common_struct, global_state);
  OFF(struct jpeg_compress_struct, dest);
  OFF(struct jpeg_compress_struct, input_gamma);
  OFF(struct jpeg_compress_struct, comp_info);
  OFF(struct jpeg_compress_struct, quant_tbl_ptrs);
  OFF(struct jpeg_compress_struct, q_scale_factor);
  OFF(struct jpeg_compress_struct, arith_dc_L);
  OFF(struct jpeg_compress_struct, raw_data_in);
  OFF(struct jpeg_compress_struct, restart_interval);
  OFF(struct jpeg_compress_struct, next_scanline);
  OFF(struct jpeg_compress_struct, total_iMCU_rows);
  OFF(struct jpeg_compress_struct, MCU_membership);
  OFF(struct jpeg_compress_struct, block_size);
  OFF(struct jpeg_compress_struct, master);
  OFF(struct jpeg_compress_struct, script_space_size);
  OFF(struct jpeg_decompress_struct, src);
  OFF(struct jpeg_decompress_struct, out_color_space);
  OFF(struct jpeg_decompress_struct, output_gamma);
  OFF(struct jpeg_decompress_struct, desired_number_of_colors);
  OFF(struct jpeg_decompress_struct, output_width);
  OFF(struct jpeg_decompress_struct, colormap);
  OFF(struct jpeg_decompress_struct, coef_bits);
  OFF(struct jpeg_decompress_struct, comp_info);
  OFF(struct jpeg_decompress_struct, arith_ac_K);
  OFF(struct jpeg_decompress_struct, marker_list);
  OFF(struct jpeg_decompress_struct, sample_range_limit);
  OFF(struct jpeg_decompress_struct, MCU_membership);
  OFF(struct jpeg_decompress_struct, unread_marker);
  OFF(struct jpeg_decompress_struct, idct);
  OFF(struct jpeg_decompress_struct, cquantize);
  OFF(struct jpeg_error_mgr, error_exit);
  OFF(struct jpeg_error_mgr, emit_message);
  OFF(struct jpeg_error_mgr, output_message);
  OFF(struct jpeg_error_mgr, format_message);
  OFF(struct jpeg_error_mgr, reset_error_mgr);
  OFF(struct jpeg_error_mgr, msg_parm);
  OFF(struct jpeg_error_mgr, trace_level);
  OFF(struct jpeg_error_mgr, jpeg_message_table);
  OFF(struct jpeg_destination_mgr, next_output_byte);
  OFF(struct jpeg_destination_mgr, free_in_buffer);
  OFF(struct jpeg_destination_mgr, init_destination);
  OFF(struct jpeg_destination_mgr, empty_output_buffer);
  OFF(struct jpeg_destination_mgr, term_destination);
  OFF(struct jpeg_source_mgr, next_input_byte);
  OFF(struct jpeg_source_mgr, bytes_in_buffer);
  OFF(struct jpeg_source_mgr, init_source);
  OFF(struct jpeg_source_mgr, fill_input_buffer);
  OFF(struct jpeg_source_mgr, skip_input_data);
  OFF(struct jpeg_source_mgr, resync_to_restart);
  OFF(struct jpeg_source_mgr, term_source);
  OFF(struct jpeg_memory_mgr, alloc_small);
  OFF(struct jpeg_memory_mgr, alloc_large);
  OFF(struct jpeg_memory_mgr, alloc_sarray);
  OFF(struct jpeg_memory_mgr, alloc_barray);
  OFF(struct jpeg_memory_mgr, request_virt_sarray);
  OFF(struct jpeg_memory_mgr, request_virt_barray);
  OFF(struct jpeg_memory_mgr, realize_virt_arrays);
  OFF(struct jpeg_memory_mgr, access_virt_sarray);
  OFF(struct jpeg_memory_mgr, access_virt_barray);
  OFF(struct jpeg_memory_mgr, free_pool);
  OFF(struct jpeg_memory_mgr, self_destruct);
  OFF(struct jpeg_memory_mgr, max_memory_to_use);
  OFF(struct jpeg_memory_mgr, max_alloc_chunk);
  OFF(jpeg_component_info, quant_table);
  OFF(struct jpeg_comp_master, prepare_for_pass);
  OFF(struct jpeg_comp_master, pass_startup);
  OFF(struct jpeg_comp_master, finish_pass);
  OFF(struct jpeg_comp_master, call_pass_startup);
  OFF(struct jpeg_c_main_controller, start_pass);
  OFF(struct jpeg_c_main_controller, process_data);
  OFF(struct jpeg_c_prep_controller, start_pass);
  OFF(struct jpeg_c_prep_controller, pre_process_data);
  OFF(struct jpeg_c_coef_controller, start_pass);
  OFF(struct jpeg_c_coef_controller, compress_data);
  OFF(struct jpeg_color_converter, start_pass);
  OFF(struct jpeg_color_converter, color_convert);
  OFF(struct jpeg_downsampler, start_pass);
  OFF(struct jpeg_downsampler, downsample);
  OFF(struct jpeg_downsampler, need_context_rows);
  OFF(struct jpeg_forward_dct, start_pass);
  OFF(struct jpeg_forward_dct, forward_DCT);
  OFF(struct jpeg_entropy_encoder, start_pass);
  OFF(struct jpeg_entropy_encoder, encode_mcu);
  OFF(struct jpeg_entropy_encoder, finish_pass);
  OFF(struct jpeg_marker_writer, write_file_header);
  OFF(struct jpeg_marker_writer, write_marker_header);
  OFF(struct jpeg_marker_writer, write_marker_byte);
  OFF(struct jpeg_decomp_master, prepare_for_output_pass);
  OFF(struct jpeg_decomp_master, first_MCU_col);
  OFF(struct jpeg_decomp_master, last_good_iMCU_row);
  OFF(struct jpeg_input_controller, consume_input);
  OFF(struct jpeg_input_controller, has_multiple_scans);
  OFF(struct jpeg_input_controller, eoi_reached);
  OFF(struct jpeg_d_main_controller, start_pass);
  OFF(struct jpeg_d_main_controller, process_data);
  OFF(struct jpeg_d_coef_controller, start_input_pass);
  OFF(struct jpeg_d_coef_controller, consume_data);
  OFF(struct jpeg_d_coef_controller, decompress_data);
  OFF(struct jpeg_d_coef_controller, coef_arrays);
  OFF(struct jpeg_d_post_controller, start_pass);
  OFF(struct jpeg_d_post_controller, post_process_data);
  OFF(struct jpeg_marker_reader, reset_marker_reader);
  OFF(struct jpeg_marker_reader, read_markers);
  OFF(struct jpeg_marker_reader, read_restart_marker);
  OFF(struct jpeg_marker_reader, discarded_bytes);
  OFF(struct jpeg_entropy_decoder, start_pass);
  OFF(struct jpeg_entropy_decoder, decode_mcu);
  OFF(struct jpeg_entropy_decoder, insufficient_data);
  OFF(struct jpeg_inverse_dct, start_pass);
  OFF(struct jpeg_inverse_dct, inverse_DCT);
  OFF(struct jpeg_upsampler, start_pass);
  OFF(struct jpeg_upsampler, upsample);
  OFF(struct jpeg_upsampler, need_context_rows);
  OFF(struct jpeg_color_deconverter, start_pass);
  OFF(struct jpeg_color_deconverter, color_convert);
  OFF(struct jpeg_color_quantizer, start_pass);
  OFF(struct jpeg_color_quantizer, color_quantize);
  OFF(struct jpeg_color_quantizer, finish_pass);
  OFF(struct jpeg_color_quantizer, new_color_map);

  CONST(JPEG_LIB_VERSION);
  CONST(C_MAX_BLOCKS_IN_MCU);
  CONST(D_MAX_BLOCKS_IN_MCU);
  CONST(JCS_EXT_RGB);
  CONST(JCS_EXT_RGBA);
  CONST(JDCT_FLOAT);
  CONST(JDITHER_FS);
  CONST(JBUF_SAVE_AND_PASS);
  CONST(CSTATE_START);
  CONST(DSTATE_STOPPING);
  CONST(JPOOL_IMAGE);
  CONST(JMSG_LENGTH_MAX);
  CONST(JMSG_STR_PARM_MAX);
  CONST(TEMP_NAME_LENGTH);
  CONST(JMSG_LASTMSGCODE);
  return 0;
}
"#;
    fs::write(&source, source_text).unwrap();
    let status = Command::new("gcc")
        .arg("-std=c11")
        .arg("-I")
        .arg(include_multiarch)
        .arg("-I")
        .arg(include)
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .status()
        .unwrap();
    assert!(status.success(), "failed to compile C layout probe");
    binary
}

fn probe_values() -> BTreeMap<String, usize> {
    let dir = tempdir();
    let stage_dir = ensure_stage_install();
    let binary = compile_probe(&dir, &stage_dir);
    let output = Command::new(&binary).output().unwrap();
    assert!(output.status.success(), "layout probe failed");
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
        .lines()
        .map(|line| {
            let (key, value) = line.split_once('=').unwrap();
            (key.to_owned(), value.parse::<usize>().unwrap())
        })
        .collect()
}

#[test]
fn abi_layouts_match_headers() {
    let c = probe_values();
    let check = |key: &str, value: usize| {
        assert_eq!(c[key], value, "{key}");
    };

    macro_rules! check_layout {
        ($label:literal, $ty:ty) => {{
            check(concat!("size:", $label), size_of::<$ty>());
            check(concat!("align:", $label), align_of::<$ty>());
        }};
    }

    macro_rules! check_off {
        ($label:literal, $ty:ty, $field:tt) => {
            check(
                concat!("off:", $label, ".", stringify!($field)),
                offset_of!($ty, $field),
            );
        };
    }

    macro_rules! check_const {
        ($name:ident) => {
            check(concat!("const:", stringify!($name)), $name as usize);
        };
    }

    check_layout!("struct jpeg_common_struct", jpeg_common_struct);
    check_layout!("struct jpeg_compress_struct", jpeg_compress_struct);
    check_layout!("struct jpeg_decompress_struct", jpeg_decompress_struct);
    check_layout!("struct jpeg_error_mgr", jpeg_error_mgr);
    check_layout!("struct jpeg_progress_mgr", jpeg_progress_mgr);
    check_layout!("struct jpeg_destination_mgr", jpeg_destination_mgr);
    check_layout!("struct jpeg_source_mgr", jpeg_source_mgr);
    check_layout!("struct jpeg_memory_mgr", jpeg_memory_mgr);
    check_layout!("JQUANT_TBL", JQUANT_TBL);
    check_layout!("JHUFF_TBL", JHUFF_TBL);
    check_layout!("jpeg_component_info", jpeg_component_info);
    check_layout!("jpeg_scan_info", jpeg_scan_info);
    check_layout!("struct jpeg_marker_struct", jpeg_marker_struct);
    check_layout!("struct jpeg_comp_master", jpeg_comp_master);
    check_layout!("struct jpeg_c_main_controller", jpeg_c_main_controller);
    check_layout!("struct jpeg_c_prep_controller", jpeg_c_prep_controller);
    check_layout!("struct jpeg_c_coef_controller", jpeg_c_coef_controller);
    check_layout!("struct jpeg_color_converter", jpeg_color_converter);
    check_layout!("struct jpeg_downsampler", jpeg_downsampler);
    check_layout!("struct jpeg_forward_dct", jpeg_forward_dct);
    check_layout!("struct jpeg_entropy_encoder", jpeg_entropy_encoder);
    check_layout!("struct jpeg_marker_writer", jpeg_marker_writer);
    check_layout!("struct jpeg_decomp_master", jpeg_decomp_master);
    check_layout!("struct jpeg_input_controller", jpeg_input_controller);
    check_layout!("struct jpeg_d_main_controller", jpeg_d_main_controller);
    check_layout!("struct jpeg_d_coef_controller", jpeg_d_coef_controller);
    check_layout!("struct jpeg_d_post_controller", jpeg_d_post_controller);
    check_layout!("struct jpeg_marker_reader", jpeg_marker_reader);
    check_layout!("struct jpeg_entropy_decoder", jpeg_entropy_decoder);
    check_layout!("struct jpeg_inverse_dct", jpeg_inverse_dct);
    check_layout!("struct jpeg_upsampler", jpeg_upsampler);
    check_layout!("struct jpeg_color_deconverter", jpeg_color_deconverter);
    check_layout!("struct jpeg_color_quantizer", jpeg_color_quantizer);

    check_off!("struct jpeg_common_struct", jpeg_common_struct, err);
    check_off!("struct jpeg_common_struct", jpeg_common_struct, mem);
    check_off!("struct jpeg_common_struct", jpeg_common_struct, progress);
    check_off!("struct jpeg_common_struct", jpeg_common_struct, client_data);
    check_off!(
        "struct jpeg_common_struct",
        jpeg_common_struct,
        is_decompressor
    );
    check_off!(
        "struct jpeg_common_struct",
        jpeg_common_struct,
        global_state
    );
    check_off!("struct jpeg_compress_struct", jpeg_compress_struct, dest);
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        input_gamma
    );
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        comp_info
    );
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        quant_tbl_ptrs
    );
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        q_scale_factor
    );
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        arith_dc_L
    );
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        raw_data_in
    );
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        restart_interval
    );
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        next_scanline
    );
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        total_iMCU_rows
    );
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        MCU_membership
    );
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        block_size
    );
    check_off!("struct jpeg_compress_struct", jpeg_compress_struct, master);
    check_off!(
        "struct jpeg_compress_struct",
        jpeg_compress_struct,
        script_space_size
    );
    check_off!("struct jpeg_decompress_struct", jpeg_decompress_struct, src);
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        out_color_space
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        output_gamma
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        desired_number_of_colors
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        output_width
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        colormap
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        coef_bits
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        comp_info
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        arith_ac_K
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        marker_list
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        sample_range_limit
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        MCU_membership
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        unread_marker
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        idct
    );
    check_off!(
        "struct jpeg_decompress_struct",
        jpeg_decompress_struct,
        cquantize
    );
    check_off!("struct jpeg_error_mgr", jpeg_error_mgr, error_exit);
    check_off!("struct jpeg_error_mgr", jpeg_error_mgr, emit_message);
    check_off!("struct jpeg_error_mgr", jpeg_error_mgr, output_message);
    check_off!("struct jpeg_error_mgr", jpeg_error_mgr, format_message);
    check_off!("struct jpeg_error_mgr", jpeg_error_mgr, reset_error_mgr);
    check_off!("struct jpeg_error_mgr", jpeg_error_mgr, msg_parm);
    check_off!("struct jpeg_error_mgr", jpeg_error_mgr, trace_level);
    check_off!("struct jpeg_error_mgr", jpeg_error_mgr, jpeg_message_table);
    check_off!(
        "struct jpeg_destination_mgr",
        jpeg_destination_mgr,
        next_output_byte
    );
    check_off!(
        "struct jpeg_destination_mgr",
        jpeg_destination_mgr,
        free_in_buffer
    );
    check_off!(
        "struct jpeg_destination_mgr",
        jpeg_destination_mgr,
        init_destination
    );
    check_off!(
        "struct jpeg_destination_mgr",
        jpeg_destination_mgr,
        empty_output_buffer
    );
    check_off!(
        "struct jpeg_destination_mgr",
        jpeg_destination_mgr,
        term_destination
    );
    check_off!("struct jpeg_source_mgr", jpeg_source_mgr, next_input_byte);
    check_off!("struct jpeg_source_mgr", jpeg_source_mgr, bytes_in_buffer);
    check_off!("struct jpeg_source_mgr", jpeg_source_mgr, init_source);
    check_off!("struct jpeg_source_mgr", jpeg_source_mgr, fill_input_buffer);
    check_off!("struct jpeg_source_mgr", jpeg_source_mgr, skip_input_data);
    check_off!("struct jpeg_source_mgr", jpeg_source_mgr, resync_to_restart);
    check_off!("struct jpeg_source_mgr", jpeg_source_mgr, term_source);
    check_off!("struct jpeg_memory_mgr", jpeg_memory_mgr, alloc_small);
    check_off!("struct jpeg_memory_mgr", jpeg_memory_mgr, alloc_large);
    check_off!("struct jpeg_memory_mgr", jpeg_memory_mgr, alloc_sarray);
    check_off!("struct jpeg_memory_mgr", jpeg_memory_mgr, alloc_barray);
    check_off!(
        "struct jpeg_memory_mgr",
        jpeg_memory_mgr,
        request_virt_sarray
    );
    check_off!(
        "struct jpeg_memory_mgr",
        jpeg_memory_mgr,
        request_virt_barray
    );
    check_off!(
        "struct jpeg_memory_mgr",
        jpeg_memory_mgr,
        realize_virt_arrays
    );
    check_off!(
        "struct jpeg_memory_mgr",
        jpeg_memory_mgr,
        access_virt_sarray
    );
    check_off!(
        "struct jpeg_memory_mgr",
        jpeg_memory_mgr,
        access_virt_barray
    );
    check_off!("struct jpeg_memory_mgr", jpeg_memory_mgr, free_pool);
    check_off!("struct jpeg_memory_mgr", jpeg_memory_mgr, self_destruct);
    check_off!("struct jpeg_memory_mgr", jpeg_memory_mgr, max_memory_to_use);
    check_off!("struct jpeg_memory_mgr", jpeg_memory_mgr, max_alloc_chunk);
    check_off!("jpeg_component_info", jpeg_component_info, quant_table);
    check_off!(
        "struct jpeg_comp_master",
        jpeg_comp_master,
        prepare_for_pass
    );
    check_off!("struct jpeg_comp_master", jpeg_comp_master, pass_startup);
    check_off!("struct jpeg_comp_master", jpeg_comp_master, finish_pass);
    check_off!(
        "struct jpeg_comp_master",
        jpeg_comp_master,
        call_pass_startup
    );
    check_off!(
        "struct jpeg_c_main_controller",
        jpeg_c_main_controller,
        start_pass
    );
    check_off!(
        "struct jpeg_c_main_controller",
        jpeg_c_main_controller,
        process_data
    );
    check_off!(
        "struct jpeg_c_prep_controller",
        jpeg_c_prep_controller,
        start_pass
    );
    check_off!(
        "struct jpeg_c_prep_controller",
        jpeg_c_prep_controller,
        pre_process_data
    );
    check_off!(
        "struct jpeg_c_coef_controller",
        jpeg_c_coef_controller,
        start_pass
    );
    check_off!(
        "struct jpeg_c_coef_controller",
        jpeg_c_coef_controller,
        compress_data
    );
    check_off!(
        "struct jpeg_color_converter",
        jpeg_color_converter,
        start_pass
    );
    check_off!(
        "struct jpeg_color_converter",
        jpeg_color_converter,
        color_convert
    );
    check_off!("struct jpeg_downsampler", jpeg_downsampler, start_pass);
    check_off!("struct jpeg_downsampler", jpeg_downsampler, downsample);
    check_off!(
        "struct jpeg_downsampler",
        jpeg_downsampler,
        need_context_rows
    );
    check_off!("struct jpeg_forward_dct", jpeg_forward_dct, start_pass);
    check_off!("struct jpeg_forward_dct", jpeg_forward_dct, forward_DCT);
    check_off!(
        "struct jpeg_entropy_encoder",
        jpeg_entropy_encoder,
        start_pass
    );
    check_off!(
        "struct jpeg_entropy_encoder",
        jpeg_entropy_encoder,
        encode_mcu
    );
    check_off!(
        "struct jpeg_entropy_encoder",
        jpeg_entropy_encoder,
        finish_pass
    );
    check_off!(
        "struct jpeg_marker_writer",
        jpeg_marker_writer,
        write_file_header
    );
    check_off!(
        "struct jpeg_marker_writer",
        jpeg_marker_writer,
        write_marker_header
    );
    check_off!(
        "struct jpeg_marker_writer",
        jpeg_marker_writer,
        write_marker_byte
    );
    check_off!(
        "struct jpeg_decomp_master",
        jpeg_decomp_master,
        prepare_for_output_pass
    );
    check_off!(
        "struct jpeg_decomp_master",
        jpeg_decomp_master,
        first_MCU_col
    );
    check_off!(
        "struct jpeg_decomp_master",
        jpeg_decomp_master,
        last_good_iMCU_row
    );
    check_off!(
        "struct jpeg_input_controller",
        jpeg_input_controller,
        consume_input
    );
    check_off!(
        "struct jpeg_input_controller",
        jpeg_input_controller,
        has_multiple_scans
    );
    check_off!(
        "struct jpeg_input_controller",
        jpeg_input_controller,
        eoi_reached
    );
    check_off!(
        "struct jpeg_d_main_controller",
        jpeg_d_main_controller,
        start_pass
    );
    check_off!(
        "struct jpeg_d_main_controller",
        jpeg_d_main_controller,
        process_data
    );
    check_off!(
        "struct jpeg_d_coef_controller",
        jpeg_d_coef_controller,
        start_input_pass
    );
    check_off!(
        "struct jpeg_d_coef_controller",
        jpeg_d_coef_controller,
        consume_data
    );
    check_off!(
        "struct jpeg_d_coef_controller",
        jpeg_d_coef_controller,
        decompress_data
    );
    check_off!(
        "struct jpeg_d_coef_controller",
        jpeg_d_coef_controller,
        coef_arrays
    );
    check_off!(
        "struct jpeg_d_post_controller",
        jpeg_d_post_controller,
        start_pass
    );
    check_off!(
        "struct jpeg_d_post_controller",
        jpeg_d_post_controller,
        post_process_data
    );
    check_off!(
        "struct jpeg_marker_reader",
        jpeg_marker_reader,
        reset_marker_reader
    );
    check_off!(
        "struct jpeg_marker_reader",
        jpeg_marker_reader,
        read_markers
    );
    check_off!(
        "struct jpeg_marker_reader",
        jpeg_marker_reader,
        read_restart_marker
    );
    check_off!(
        "struct jpeg_marker_reader",
        jpeg_marker_reader,
        discarded_bytes
    );
    check_off!(
        "struct jpeg_entropy_decoder",
        jpeg_entropy_decoder,
        start_pass
    );
    check_off!(
        "struct jpeg_entropy_decoder",
        jpeg_entropy_decoder,
        decode_mcu
    );
    check_off!(
        "struct jpeg_entropy_decoder",
        jpeg_entropy_decoder,
        insufficient_data
    );
    check_off!("struct jpeg_inverse_dct", jpeg_inverse_dct, start_pass);
    check_off!("struct jpeg_inverse_dct", jpeg_inverse_dct, inverse_DCT);
    check_off!("struct jpeg_upsampler", jpeg_upsampler, start_pass);
    check_off!("struct jpeg_upsampler", jpeg_upsampler, upsample);
    check_off!("struct jpeg_upsampler", jpeg_upsampler, need_context_rows);
    check_off!(
        "struct jpeg_color_deconverter",
        jpeg_color_deconverter,
        start_pass
    );
    check_off!(
        "struct jpeg_color_deconverter",
        jpeg_color_deconverter,
        color_convert
    );
    check_off!(
        "struct jpeg_color_quantizer",
        jpeg_color_quantizer,
        start_pass
    );
    check_off!(
        "struct jpeg_color_quantizer",
        jpeg_color_quantizer,
        color_quantize
    );
    check_off!(
        "struct jpeg_color_quantizer",
        jpeg_color_quantizer,
        finish_pass
    );
    check_off!(
        "struct jpeg_color_quantizer",
        jpeg_color_quantizer,
        new_color_map
    );

    check_const!(JPEG_LIB_VERSION);
    check_const!(C_MAX_BLOCKS_IN_MCU);
    check_const!(D_MAX_BLOCKS_IN_MCU);
    check_const!(JCS_EXT_RGB);
    check_const!(JCS_EXT_RGBA);
    check_const!(JDCT_FLOAT);
    check_const!(JDITHER_FS);
    check_const!(JBUF_SAVE_AND_PASS);
    check_const!(CSTATE_START);
    check_const!(DSTATE_STOPPING);
    check_const!(JPOOL_IMAGE);
    check_const!(JMSG_LENGTH_MAX);
    check_const!(JMSG_STR_PARM_MAX);
    check_const!(TEMP_NAME_LENGTH);
    check_const!(JMSG_LASTMSGCODE);
}
