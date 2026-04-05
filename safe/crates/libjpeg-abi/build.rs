fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let manifest_dir = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let safe_root = manifest_dir.join("../..").canonicalize().unwrap();
    let repo_root = safe_root.parent().unwrap().to_path_buf();
    let original_root = repo_root.join("original");
    let multiarch = multiarch();
    let generated_include = out_dir.join("shim-include");
    let generated_multiarch = generated_include.join(&multiarch);
    std::fs::create_dir_all(&generated_multiarch).unwrap();
    write_generated_headers(&generated_include, &generated_multiarch);
    let shim_sources = [("error_bridge.c", "error_bridge.o")];
    let shim_archive = out_dir.join("liberror_bridge.a");

    let mut objects = Vec::new();
    for (source_name, object_name) in shim_sources {
        let shim_source = safe_root.join("c_shim").join(source_name);
        let shim_object = out_dir.join(object_name);
        run(std::process::Command::new("gcc")
            .arg("-std=c99")
            .arg("-O2")
            .arg("-fPIC")
            .arg("-I")
            .arg(&generated_include)
            .arg("-I")
            .arg(&generated_multiarch)
            .arg("-I")
            .arg(&original_root)
            .arg("-c")
            .arg(&shim_source)
            .arg("-o")
            .arg(&shim_object));
        objects.push(shim_object);
    }
    let mut archive = std::process::Command::new("ar");
    archive.arg("crus").arg(&shim_archive);
    for object in &objects {
        archive.arg(object);
    }
    run(&mut archive);

    for path in [
        "../../scripts/stage-install.sh",
        "../../scripts/check-symbols.sh",
        "../../c_shim/error_bridge.c",
        "../../debian/libjpeg-turbo8.symbols",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=error_bridge");
}

fn multiarch() -> String {
    for (program, args) in [
        ("dpkg-architecture", &["-qDEB_HOST_MULTIARCH"][..]),
        ("gcc", &["-print-multiarch"][..]),
    ] {
        let output = std::process::Command::new(program).args(args).output();
        if let Ok(output) = output {
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

fn write_generated_headers(
    generated_include: &std::path::Path,
    generated_multiarch: &std::path::Path,
) {
    const UPSTREAM_VERSION: &str = "2.1.5";
    const LIBJPEG_TURBO_VERSION_NUMBER: &str = "2001005";
    const BUILD_STRING: &str = "20260403";
    const COPYRIGHT_YEAR: &str = "1991-2023";

    std::fs::write(
        generated_multiarch.join("jconfig.h"),
        format!(
            "/* Version ID for the JPEG library.\n\
             * Might be useful for tests like \"#if JPEG_LIB_VERSION >= 60\".\n\
             */\n\
             #define JPEG_LIB_VERSION  80\n\n\
             /* libjpeg-turbo version */\n\
             #define LIBJPEG_TURBO_VERSION  {UPSTREAM_VERSION}\n\n\
             /* libjpeg-turbo version in integer form */\n\
             #define LIBJPEG_TURBO_VERSION_NUMBER  {LIBJPEG_TURBO_VERSION_NUMBER}\n\n\
             /* Support arithmetic encoding */\n\
             #define C_ARITH_CODING_SUPPORTED 1\n\n\
             /* Support arithmetic decoding */\n\
             #define D_ARITH_CODING_SUPPORTED 1\n\n\
             /* Use accelerated SIMD routines. */\n\
             #define WITH_SIMD 1\n\n\
             /*\n\
              * Define BITS_IN_JSAMPLE as either\n\
              *   8   for 8-bit sample values (the usual setting)\n\
              *   12  for 12-bit sample values\n\
              * Only 8 and 12 are legal data precisions for lossy JPEG according to the\n\
              * JPEG standard, and the IJG code does not support anything else!\n\
              * We do not support run-time selection of data precision, sorry.\n\
              */\n\n\
             #define BITS_IN_JSAMPLE  8\n\n\
             /* Define if your (broken) compiler shifts signed values as if they were\n\
                unsigned. */\n\
             #undef RIGHT_SHIFT_IS_UNSIGNED\n"
        ),
    )
    .unwrap();

    std::fs::write(
        generated_include.join("jconfigint.h"),
        format!(
            "/* libjpeg-turbo build number */\n\
             #define BUILD  \"{BUILD_STRING}\"\n\n\
             /* Compiler's inline keyword */\n\
             #undef inline\n\n\
             /* How to obtain function inlining. */\n\
             #define INLINE  __inline__ __attribute__((always_inline))\n\n\
             /* How to obtain thread-local storage */\n\
             #define THREAD_LOCAL  __thread\n\n\
             /* Define to the full name of this package. */\n\
             #define PACKAGE_NAME  \"libjpeg-turbo\"\n\n\
             /* Version number of package */\n\
             #define VERSION  \"{UPSTREAM_VERSION}\"\n\n\
             /* The size of `size_t', as computed by sizeof. */\n\
             #define SIZEOF_SIZE_T  8\n\n\
             /* Define if your compiler has __builtin_ctzl() and sizeof(unsigned long) == sizeof(size_t). */\n\
             #define HAVE_BUILTIN_CTZL 1\n\n\
             /* Define to 1 if you have the <intrin.h> header file. */\n\
             #undef HAVE_INTRIN_H\n\n\
             #if defined(_MSC_VER) && defined(HAVE_INTRIN_H)\n\
             #if (SIZEOF_SIZE_T == 8)\n\
             #define HAVE_BITSCANFORWARD64\n\
             #elif (SIZEOF_SIZE_T == 4)\n\
             #define HAVE_BITSCANFORWARD\n\
             #endif\n\
             #endif\n\n\
             #if defined(__has_attribute)\n\
             #if __has_attribute(fallthrough)\n\
             #define FALLTHROUGH  __attribute__((fallthrough));\n\
             #else\n\
             #define FALLTHROUGH\n\
             #endif\n\
             #else\n\
             #define FALLTHROUGH\n\
             #endif\n"
        ),
    )
    .unwrap();

    std::fs::write(
        generated_include.join("jversion.h"),
        format!(
            "/*\n\
             * jversion.h\n\
             *\n\
             * This file was part of the Independent JPEG Group's software:\n\
             * Copyright (C) 1991-2020, Thomas G. Lane, Guido Vollbeding.\n\
             * libjpeg-turbo Modifications:\n\
             * Copyright (C) 2010, 2012-2023, D. R. Commander.\n\
             * For conditions of distribution and use, see the accompanying README.ijg\n\
             * file.\n\
             *\n\
             * This file contains software version identification.\n\
             */\n\n\
             #if JPEG_LIB_VERSION >= 80\n\n\
             #define JVERSION        \"8d  15-Jan-2012\"\n\n\
             #elif JPEG_LIB_VERSION >= 70\n\n\
             #define JVERSION        \"7  27-Jun-2009\"\n\n\
             #else\n\n\
             #define JVERSION        \"6b  27-Mar-1998\"\n\n\
             #endif\n\n\
             #define JCOPYRIGHT \\\n\
               \"Copyright (C) 2009-2023 D. R. Commander\\\\n\" \\\n\
               \"Copyright (C) 2015, 2020 Google, Inc.\\\\n\" \\\n\
               \"Copyright (C) 2019-2020 Arm Limited\\\\n\" \\\n\
               \"Copyright (C) 2015-2016, 2018 Matthieu Darbois\\\\n\" \\\n\
               \"Copyright (C) 2011-2016 Siarhei Siamashka\\\\n\" \\\n\
               \"Copyright (C) 2015 Intel Corporation\\\\n\" \\\n\
               \"Copyright (C) 2013-2014 Linaro Limited\\\\n\" \\\n\
               \"Copyright (C) 2013-2014 MIPS Technologies, Inc.\\\\n\" \\\n\
               \"Copyright (C) 2009, 2012 Pierre Ossman for Cendio AB\\\\n\" \\\n\
               \"Copyright (C) 2009-2011 Nokia Corporation and/or its subsidiary(-ies)\\\\n\" \\\n\
               \"Copyright (C) 1999-2006 MIYASAKA Masaru\\\\n\" \\\n\
               \"Copyright (C) 1991-2020 Thomas G. Lane, Guido Vollbeding\"\n\n\
             #define JCOPYRIGHT_SHORT \\\n\
               \"Copyright (C) {COPYRIGHT_YEAR} The libjpeg-turbo Project and many others\"\n"
        ),
    )
    .unwrap();
}

fn run(command: &mut std::process::Command) {
    let status = command.status().expect("failed to run build helper");
    if !status.success() {
        panic!("build helper exited with status {status}");
    }
}
