use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const UPSTREAM_VERSION: &str = "2.1.5";
const BUILD_STRING: &str = "20260403";
const LIBJPEG_TURBO_VERSION_NUMBER: &str = "2001005";

struct ToolSpec {
    lib_name: &'static str,
    main_symbol: &'static str,
    sources: &'static [&'static str],
    defines: &'static [&'static str],
}

const TOOL_SPECS: &[ToolSpec] = &[
    ToolSpec {
        lib_name: "jpeg_tools_cjpeg_tool",
        main_symbol: "safe_cjpeg_main",
        sources: &[
            "cjpeg.c",
            "cdjpeg.c",
            "rdswitch.c",
            "rdbmp.c",
            "rdgif.c",
            "rdppm.c",
            "rdtarga.c",
        ],
        defines: &["BMP_SUPPORTED", "GIF_SUPPORTED", "PPM_SUPPORTED", "TARGA_SUPPORTED"],
    },
    ToolSpec {
        lib_name: "jpeg_tools_djpeg_tool",
        main_symbol: "safe_djpeg_main",
        sources: &[
            "djpeg.c",
            "cdjpeg.c",
            "rdswitch.c",
            "rdcolmap.c",
            "wrbmp.c",
            "wrgif.c",
            "wrppm.c",
            "wrtarga.c",
        ],
        defines: &["BMP_SUPPORTED", "GIF_SUPPORTED", "PPM_SUPPORTED", "TARGA_SUPPORTED"],
    },
    ToolSpec {
        lib_name: "jpeg_tools_jpegtran_tool",
        main_symbol: "safe_jpegtran_main",
        sources: &["jpegtran.c", "cdjpeg.c", "rdswitch.c", "transupp.c"],
        defines: &[],
    },
    ToolSpec {
        lib_name: "jpeg_tools_rdjpgcom_tool",
        main_symbol: "safe_rdjpgcom_main",
        sources: &["rdjpgcom.c"],
        defines: &[],
    },
    ToolSpec {
        lib_name: "jpeg_tools_wrjpgcom_tool",
        main_symbol: "safe_wrjpgcom_main",
        sources: &["wrjpgcom.c"],
        defines: &[],
    },
    ToolSpec {
        lib_name: "jpeg_tools_tjbench_tool",
        main_symbol: "safe_tjbench_main",
        sources: &["tjbench.c"],
        defines: &[],
    },
    ToolSpec {
        lib_name: "jpeg_tools_tjexample_tool",
        main_symbol: "safe_tjexample_main",
        sources: &["tjexample.c"],
        defines: &[],
    },
];

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set"));
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set"));
    let safe_root = manifest_dir.join("../..").canonicalize().expect("resolve safe root");
    let original_root = safe_root
        .join("../original")
        .canonicalize()
        .expect("resolve original source tree");
    let bootstrap_dir = safe_root.join("target/upstream-bootstrap");
    let generated_include = out_dir.join("generated-include");

    fs::create_dir_all(&generated_include).expect("create generated include dir");
    render_jconfig_h(&generated_include.join("jconfig.h"));
    render_jconfigint_h(&generated_include.join("jconfigint.h"));
    render_jversion_h(&generated_include.join("jversion.h"));

    for spec in TOOL_SPECS {
        build_tool_archive(&out_dir, &generated_include, &original_root, spec);
    }

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!(
        "cargo:rustc-link-search=native={}",
        bootstrap_dir.display()
    );
    println!("cargo:rustc-link-lib=m");

    for header in [
        "cdjpeg.h",
        "cderror.h",
        "jerror.h",
        "jinclude.h",
        "jmorecfg.h",
        "jpeglib.h",
        "tjutil.h",
        "transupp.h",
        "turbojpeg.h",
    ] {
        println!("cargo:rerun-if-changed=../../../original/{header}");
    }
    for spec in TOOL_SPECS {
        for source in spec.sources {
            println!("cargo:rerun-if-changed=../../../original/{source}");
        }
    }
}

fn build_tool_archive(
    out_dir: &Path,
    generated_include: &Path,
    original_root: &Path,
    spec: &ToolSpec,
) {
    let tool_dir = out_dir.join(spec.lib_name);
    fs::create_dir_all(&tool_dir).expect("create tool build dir");

    let mut objects = Vec::with_capacity(spec.sources.len());
    for (index, source) in spec.sources.iter().enumerate() {
        let source_path = original_root.join(source);
        let object_path = tool_dir.join(format!("{index}-{source}.o"));
        let rename_main = (index == 0).then_some(spec.main_symbol);
        compile_source(
            &source_path,
            &object_path,
            generated_include,
            original_root,
            spec.defines,
            rename_main,
        );
        objects.push(object_path);
    }

    archive_objects(&out_dir.join(format!("lib{}.a", spec.lib_name)), &objects);
}

fn compile_source(
    source_path: &Path,
    object_path: &Path,
    generated_include: &Path,
    original_root: &Path,
    defines: &[&str],
    rename_main: Option<&str>,
) {
    let mut command = Command::new("gcc");
    command
        .arg("-std=c99")
        .arg("-O2")
        .arg("-I")
        .arg(generated_include)
        .arg("-I")
        .arg(original_root);
    for define in defines {
        command.arg(format!("-D{define}"));
    }
    if let Some(symbol) = rename_main {
        command.arg(format!("-Dmain={symbol}"));
    }
    command
        .arg("-c")
        .arg(source_path)
        .arg("-o")
        .arg(object_path);
    run(&mut command);
}

fn archive_objects(archive: &Path, objects: &[PathBuf]) {
    if archive.exists() {
        fs::remove_file(archive).expect("remove stale archive");
    }
    let mut command = Command::new("ar");
    command.arg("crus").arg(archive);
    for object in objects {
        command.arg(object);
    }
    run(&mut command);
}

fn run(command: &mut Command) {
    let status = command.status().unwrap_or_else(|error| {
        panic!("failed to run {:?}: {error}", command);
    });
    if !status.success() {
        panic!("command {:?} exited with status {status}", command);
    }
}

fn render_jconfig_h(output: &Path) {
    fs::write(
        output,
        format!(
            "/* Version ID for the JPEG library. */\n\
             #define JPEG_LIB_VERSION  80\n\n\
             /* libjpeg-turbo version */\n\
             #define LIBJPEG_TURBO_VERSION  {UPSTREAM_VERSION}\n\n\
             /* libjpeg-turbo version in integer form */\n\
             #define LIBJPEG_TURBO_VERSION_NUMBER  {LIBJPEG_TURBO_VERSION_NUMBER}\n\n\
             /* Support arithmetic encoding */\n\
             #define C_ARITH_CODING_SUPPORTED 1\n\n\
             /* Support arithmetic decoding */\n\
             #define D_ARITH_CODING_SUPPORTED 1\n\n\
             /* Support in-memory source/destination managers */\n\
             #define MEM_SRCDST_SUPPORTED 1\n\n\
             /* Use accelerated SIMD routines. */\n\
             #define WITH_SIMD 1\n\n\
             #define BITS_IN_JSAMPLE  8\n\n\
             #undef RIGHT_SHIFT_IS_UNSIGNED\n"
        ),
    )
    .expect("write generated jconfig.h");
}

fn render_jconfigint_h(output: &Path) {
    let contents = format!(
        "/* libjpeg-turbo build number */\n\
         #define BUILD  \"{BUILD_STRING}\"\n\n\
         #undef inline\n\n\
         #define INLINE  __inline__ __attribute__((always_inline))\n\n\
         #define THREAD_LOCAL  __thread\n\n\
         #define PACKAGE_NAME  \"libjpeg-turbo\"\n\n\
         #define VERSION  \"{UPSTREAM_VERSION}\"\n\n\
         #define SIZEOF_SIZE_T  {size_t_size}\n\n\
         #define HAVE_BUILTIN_CTZL 1\n\n\
         #undef HAVE_INTRIN_H\n\n\
         #if defined(__has_attribute)\n\
         #if __has_attribute(fallthrough)\n\
         #define FALLTHROUGH  __attribute__((fallthrough));\n\
         #else\n\
         #define FALLTHROUGH\n\
         #endif\n\
         #else\n\
         #define FALLTHROUGH\n\
         #endif\n",
        size_t_size = std::mem::size_of::<usize>()
    );
    fs::write(output, contents).expect("write generated jconfigint.h");
}

fn render_jversion_h(output: &Path) {
    fs::write(
        output,
        "/* Generated for the Rust jpeg-tools crate. */\n\
         #define JVERSION        \"8d  15-Jan-2012\"\n\
         #define JCOPYRIGHT \\\n\
           \"Copyright (C) 2009-2023 D. R. Commander\\n\" \\\n\
           \"Copyright (C) 2015, 2020 Google, Inc.\\n\" \\\n\
           \"Copyright (C) 2019-2020 Arm Limited\\n\" \\\n\
           \"Copyright (C) 2015-2016, 2018 Matthieu Darbois\\n\" \\\n\
           \"Copyright (C) 2011-2016 Siarhei Siamashka\\n\" \\\n\
           \"Copyright (C) 2015 Intel Corporation\\n\" \\\n\
           \"Copyright (C) 2013-2014 Linaro Limited\\n\" \\\n\
           \"Copyright (C) 2013-2014 MIPS Technologies, Inc.\\n\" \\\n\
           \"Copyright (C) 2009, 2012 Pierre Ossman for Cendio AB\\n\" \\\n\
           \"Copyright (C) 2009-2011 Nokia Corporation and/or its subsidiary(-ies)\\n\" \\\n\
           \"Copyright (C) 1999-2006 MIYASAKA Masaru\\n\" \\\n\
           \"Copyright (C) 1991-2020 Thomas G. Lane, Guido Vollbeding\"\n",
    )
    .expect("write generated jversion.h");
}
