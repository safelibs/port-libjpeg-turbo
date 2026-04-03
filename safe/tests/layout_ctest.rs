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

fn compile_probe(dir: &Path) -> PathBuf {
    let source = dir.join("probe.c");
    let binary = dir.join("probe");
    let safe_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let include = safe_root.join("stage/usr/include");
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
  OFF(struct jpeg_error_mgr, msg_parm);
  OFF(struct jpeg_error_mgr, jpeg_message_table);
  OFF(struct jpeg_memory_mgr, request_virt_sarray);
  OFF(struct jpeg_memory_mgr, max_memory_to_use);
  OFF(jpeg_component_info, quant_table);
  OFF(struct jpeg_decomp_master, first_MCU_col);
  OFF(struct jpeg_marker_reader, discarded_bytes);
  OFF(struct jpeg_inverse_dct, inverse_DCT);

  CONST(JCS_EXT_RGB);
  CONST(JCS_EXT_RGBA);
  CONST(JDCT_FLOAT);
  CONST(JDITHER_FS);
  CONST(JBUF_SAVE_AND_PASS);
  CONST(CSTATE_START);
  CONST(DSTATE_STOPPING);
  CONST(JPOOL_IMAGE);
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
    let binary = compile_probe(&dir);
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

    check("size:struct jpeg_common_struct", size_of::<jpeg_common_struct>());
    check("size:struct jpeg_compress_struct", size_of::<jpeg_compress_struct>());
    check("size:struct jpeg_decompress_struct", size_of::<jpeg_decompress_struct>());
    check("size:struct jpeg_error_mgr", size_of::<jpeg_error_mgr>());
    check("size:struct jpeg_progress_mgr", size_of::<jpeg_progress_mgr>());
    check("size:struct jpeg_destination_mgr", size_of::<jpeg_destination_mgr>());
    check("size:struct jpeg_source_mgr", size_of::<jpeg_source_mgr>());
    check("size:struct jpeg_memory_mgr", size_of::<jpeg_memory_mgr>());
    check("size:JQUANT_TBL", size_of::<JQUANT_TBL>());
    check("size:JHUFF_TBL", size_of::<JHUFF_TBL>());
    check("size:jpeg_component_info", size_of::<jpeg_component_info>());
    check("size:jpeg_scan_info", size_of::<jpeg_scan_info>());
    check("size:struct jpeg_marker_struct", size_of::<jpeg_marker_struct>());
    check("size:struct jpeg_comp_master", size_of::<jpeg_comp_master>());
    check("size:struct jpeg_c_main_controller", size_of::<jpeg_c_main_controller>());
    check("size:struct jpeg_c_prep_controller", size_of::<jpeg_c_prep_controller>());
    check("size:struct jpeg_c_coef_controller", size_of::<jpeg_c_coef_controller>());
    check("size:struct jpeg_color_converter", size_of::<jpeg_color_converter>());
    check("size:struct jpeg_downsampler", size_of::<jpeg_downsampler>());
    check("size:struct jpeg_forward_dct", size_of::<jpeg_forward_dct>());
    check("size:struct jpeg_entropy_encoder", size_of::<jpeg_entropy_encoder>());
    check("size:struct jpeg_marker_writer", size_of::<jpeg_marker_writer>());
    check("size:struct jpeg_decomp_master", size_of::<jpeg_decomp_master>());
    check("size:struct jpeg_input_controller", size_of::<jpeg_input_controller>());
    check("size:struct jpeg_d_main_controller", size_of::<jpeg_d_main_controller>());
    check("size:struct jpeg_d_coef_controller", size_of::<jpeg_d_coef_controller>());
    check("size:struct jpeg_d_post_controller", size_of::<jpeg_d_post_controller>());
    check("size:struct jpeg_marker_reader", size_of::<jpeg_marker_reader>());
    check("size:struct jpeg_entropy_decoder", size_of::<jpeg_entropy_decoder>());
    check("size:struct jpeg_inverse_dct", size_of::<jpeg_inverse_dct>());
    check("size:struct jpeg_upsampler", size_of::<jpeg_upsampler>());
    check("size:struct jpeg_color_deconverter", size_of::<jpeg_color_deconverter>());
    check("size:struct jpeg_color_quantizer", size_of::<jpeg_color_quantizer>());

    check("align:struct jpeg_common_struct", align_of::<jpeg_common_struct>());
    check("align:struct jpeg_compress_struct", align_of::<jpeg_compress_struct>());
    check("align:struct jpeg_decompress_struct", align_of::<jpeg_decompress_struct>());
    check("align:struct jpeg_error_mgr", align_of::<jpeg_error_mgr>());
    check("align:struct jpeg_memory_mgr", align_of::<jpeg_memory_mgr>());

    check("off:struct jpeg_common_struct.err", offset_of!(jpeg_common_struct, err));
    check("off:struct jpeg_common_struct.global_state", offset_of!(jpeg_common_struct, global_state));
    check("off:struct jpeg_compress_struct.dest", offset_of!(jpeg_compress_struct, dest));
    check("off:struct jpeg_compress_struct.input_gamma", offset_of!(jpeg_compress_struct, input_gamma));
    check("off:struct jpeg_compress_struct.comp_info", offset_of!(jpeg_compress_struct, comp_info));
    check("off:struct jpeg_compress_struct.quant_tbl_ptrs", offset_of!(jpeg_compress_struct, quant_tbl_ptrs));
    check("off:struct jpeg_compress_struct.q_scale_factor", offset_of!(jpeg_compress_struct, q_scale_factor));
    check("off:struct jpeg_compress_struct.arith_dc_L", offset_of!(jpeg_compress_struct, arith_dc_L));
    check("off:struct jpeg_compress_struct.raw_data_in", offset_of!(jpeg_compress_struct, raw_data_in));
    check("off:struct jpeg_compress_struct.restart_interval", offset_of!(jpeg_compress_struct, restart_interval));
    check("off:struct jpeg_compress_struct.next_scanline", offset_of!(jpeg_compress_struct, next_scanline));
    check("off:struct jpeg_compress_struct.total_iMCU_rows", offset_of!(jpeg_compress_struct, total_iMCU_rows));
    check("off:struct jpeg_compress_struct.MCU_membership", offset_of!(jpeg_compress_struct, MCU_membership));
    check("off:struct jpeg_compress_struct.block_size", offset_of!(jpeg_compress_struct, block_size));
    check("off:struct jpeg_compress_struct.master", offset_of!(jpeg_compress_struct, master));
    check("off:struct jpeg_compress_struct.script_space_size", offset_of!(jpeg_compress_struct, script_space_size));
    check("off:struct jpeg_decompress_struct.src", offset_of!(jpeg_decompress_struct, src));
    check("off:struct jpeg_decompress_struct.out_color_space", offset_of!(jpeg_decompress_struct, out_color_space));
    check("off:struct jpeg_decompress_struct.output_gamma", offset_of!(jpeg_decompress_struct, output_gamma));
    check(
        "off:struct jpeg_decompress_struct.desired_number_of_colors",
        offset_of!(jpeg_decompress_struct, desired_number_of_colors),
    );
    check("off:struct jpeg_decompress_struct.output_width", offset_of!(jpeg_decompress_struct, output_width));
    check("off:struct jpeg_decompress_struct.colormap", offset_of!(jpeg_decompress_struct, colormap));
    check("off:struct jpeg_decompress_struct.coef_bits", offset_of!(jpeg_decompress_struct, coef_bits));
    check("off:struct jpeg_decompress_struct.comp_info", offset_of!(jpeg_decompress_struct, comp_info));
    check("off:struct jpeg_decompress_struct.arith_ac_K", offset_of!(jpeg_decompress_struct, arith_ac_K));
    check("off:struct jpeg_decompress_struct.marker_list", offset_of!(jpeg_decompress_struct, marker_list));
    check(
        "off:struct jpeg_decompress_struct.sample_range_limit",
        offset_of!(jpeg_decompress_struct, sample_range_limit),
    );
    check("off:struct jpeg_decompress_struct.MCU_membership", offset_of!(jpeg_decompress_struct, MCU_membership));
    check("off:struct jpeg_decompress_struct.unread_marker", offset_of!(jpeg_decompress_struct, unread_marker));
    check("off:struct jpeg_decompress_struct.idct", offset_of!(jpeg_decompress_struct, idct));
    check("off:struct jpeg_decompress_struct.cquantize", offset_of!(jpeg_decompress_struct, cquantize));
    check("off:struct jpeg_error_mgr.msg_parm", offset_of!(jpeg_error_mgr, msg_parm));
    check(
        "off:struct jpeg_error_mgr.jpeg_message_table",
        offset_of!(jpeg_error_mgr, jpeg_message_table),
    );
    check(
        "off:struct jpeg_memory_mgr.request_virt_sarray",
        offset_of!(jpeg_memory_mgr, request_virt_sarray),
    );
    check(
        "off:struct jpeg_memory_mgr.max_memory_to_use",
        offset_of!(jpeg_memory_mgr, max_memory_to_use),
    );
    check("off:jpeg_component_info.quant_table", offset_of!(jpeg_component_info, quant_table));
    check("off:struct jpeg_decomp_master.first_MCU_col", offset_of!(jpeg_decomp_master, first_MCU_col));
    check("off:struct jpeg_marker_reader.discarded_bytes", offset_of!(jpeg_marker_reader, discarded_bytes));
    check("off:struct jpeg_inverse_dct.inverse_DCT", offset_of!(jpeg_inverse_dct, inverse_DCT));

    check("const:JCS_EXT_RGB", JCS_EXT_RGB as usize);
    check("const:JCS_EXT_RGBA", JCS_EXT_RGBA as usize);
    check("const:JDCT_FLOAT", JDCT_FLOAT as usize);
    check("const:JDITHER_FS", JDITHER_FS as usize);
    check("const:JBUF_SAVE_AND_PASS", JBUF_SAVE_AND_PASS as usize);
    check("const:CSTATE_START", CSTATE_START as usize);
    check("const:DSTATE_STOPPING", DSTATE_STOPPING as usize);
    check("const:JPOOL_IMAGE", JPOOL_IMAGE as usize);
    check("const:TEMP_NAME_LENGTH", TEMP_NAME_LENGTH as usize);
    check("const:JMSG_LASTMSGCODE", JMSG_LASTMSGCODE as usize);
}
