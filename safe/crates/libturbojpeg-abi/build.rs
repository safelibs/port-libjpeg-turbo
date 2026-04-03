use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const BACKEND_SOURCES: &[&str] = &[
    "turbojpeg.c",
    "transupp.c",
    "jdatadst-tj.c",
    "jdatasrc-tj.c",
    "rdbmp.c",
    "rdppm.c",
    "wrbmp.c",
    "wrppm.c",
];

const PREFIXED_EXPORTS: &[&str] = &[
    "TJBUFSIZE",
    "tjCompress",
    "tjDecompress",
    "tjDecompressHeader",
    "tjDestroy",
    "tjGetErrorStr",
    "tjInitCompress",
    "tjInitDecompress",
    "TJBUFSIZEYUV",
    "tjDecompressHeader2",
    "tjDecompressToYUV",
    "tjEncodeYUV",
    "tjAlloc",
    "tjBufSize",
    "tjBufSizeYUV",
    "tjCompress2",
    "tjDecompress2",
    "tjEncodeYUV2",
    "tjFree",
    "tjGetScalingFactors",
    "tjInitTransform",
    "tjTransform",
    "tjBufSizeYUV2",
    "tjCompressFromYUV",
    "tjCompressFromYUVPlanes",
    "tjDecodeYUV",
    "tjDecodeYUVPlanes",
    "tjDecompressHeader3",
    "tjDecompressToYUV2",
    "tjDecompressToYUVPlanes",
    "tjEncodeYUV3",
    "tjEncodeYUVPlanes",
    "tjPlaneHeight",
    "tjPlaneSizeYUV",
    "tjPlaneWidth",
    "tjGetErrorCode",
    "tjGetErrorStr2",
    "tjLoadImage",
    "tjSaveImage",
    "jcopy_markers_execute",
    "jcopy_markers_setup",
    "jtransform_adjust_parameters",
    "jtransform_execute_transform",
    "jtransform_parse_crop_spec",
    "jtransform_perfect_transform",
    "jtransform_request_workspace",
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
    let generated_include = out_dir.join("generated-include");
    let prefix_header = out_dir.join("turbojpeg-prefix.h");
    let archive = out_dir.join("libturbojpeg_backend.a");

    fs::create_dir_all(&generated_include).expect("create generated include dir");
    render_jconfig_h(&generated_include.join("jconfig.h"));
    render_jconfigint_h(&generated_include.join("jconfigint.h"));
    render_prefix_header(&prefix_header);

    let objects = compile_backend(&out_dir, &generated_include, &prefix_header, &original_root);
    archive_objects(&archive, &objects);

    for path in [
        "../../scripts/stage-install.sh",
        "../../../original/CMakeLists.txt",
        "../../../original/turbojpeg-mapfile",
        "../../../original/turbojpeg-mapfile.jni",
        "../../../original/debian/libturbojpeg.symbols",
        "../../../original/turbojpeg.h",
        "../../../original/turbojpeg.c",
        "../../../original/tjutil.h",
        "../../../original/transupp.h",
        "../../../original/transupp.c",
        "../../../original/jdatadst-tj.c",
        "../../../original/jdatasrc-tj.c",
        "../../../original/rdbmp.c",
        "../../../original/rdppm.c",
        "../../../original/wrbmp.c",
        "../../../original/wrppm.c",
        "../../../original/cdjpeg.h",
        "../../../original/cderror.h",
        "../../../original/jinclude.h",
        "../../../original/jpeglib.h",
        "../../../original/jerror.h",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=turbojpeg_backend");
}

fn render_jconfig_h(output: &Path) {
    fs::write(
        output,
        r##"/* Version ID for the JPEG library.
 * Might be useful for tests like "#if JPEG_LIB_VERSION >= 60".
 */
#define JPEG_LIB_VERSION  80

/* libjpeg-turbo version */
#define LIBJPEG_TURBO_VERSION  2.1.5

/* libjpeg-turbo version in integer form */
#define LIBJPEG_TURBO_VERSION_NUMBER  2001005

/* Support arithmetic encoding */
#define C_ARITH_CODING_SUPPORTED 1

/* Support arithmetic decoding */
#define D_ARITH_CODING_SUPPORTED 1

/* Support in-memory source/destination managers */
#define MEM_SRCDST_SUPPORTED 1

/* Use accelerated SIMD routines. */
#define WITH_SIMD 1

/*
 * Define BITS_IN_JSAMPLE as either
 *   8   for 8-bit sample values (the usual setting)
 *   12  for 12-bit sample values
 * Only 8 and 12 are legal data precisions for lossy JPEG according to the
 * JPEG standard, and the IJG code does not support anything else!
 * We do not support run-time selection of data precision, sorry.
 */

#define BITS_IN_JSAMPLE  8

/* Define if your (broken) compiler shifts signed values as if they were
   unsigned. */
#undef RIGHT_SHIFT_IS_UNSIGNED
"##,
    )
    .expect("write generated jconfig.h");
}

fn render_jconfigint_h(output: &Path) {
    let contents = format!(
        r#"/* libjpeg-turbo build number */
#define BUILD  "20260403"

/* Compiler's inline keyword */
#undef inline

/* How to obtain function inlining. */
#define INLINE  __inline__ __attribute__((always_inline))

/* How to obtain thread-local storage */
#define THREAD_LOCAL  __thread

/* Define to the full name of this package. */
#define PACKAGE_NAME  "libjpeg-turbo"

/* Version number of package */
#define VERSION  "2.1.5"

/* The size of `size_t', as computed by sizeof. */
#define SIZEOF_SIZE_T  {size_t_size}

/* Define if your compiler has __builtin_ctzl() and sizeof(unsigned long) == sizeof(size_t). */
#define HAVE_BUILTIN_CTZL 1

/* Define to 1 if you have the <intrin.h> header file. */
#undef HAVE_INTRIN_H

#if defined(_MSC_VER) && defined(HAVE_INTRIN_H)
#if (SIZEOF_SIZE_T == 8)
#define HAVE_BITSCANFORWARD64
#elif (SIZEOF_SIZE_T == 4)
#define HAVE_BITSCANFORWARD
#endif
#endif

#if defined(__has_attribute)
#if __has_attribute(fallthrough)
#define FALLTHROUGH  __attribute__((fallthrough));
#else
#define FALLTHROUGH
#endif
#else
#define FALLTHROUGH
#endif
"#,
        size_t_size = std::mem::size_of::<usize>()
    );
    fs::write(output, contents).expect("write generated jconfigint.h");
}

fn render_prefix_header(output: &Path) {
    let mut header = String::new();
    for symbol in PREFIXED_EXPORTS {
        header.push_str("#define ");
        header.push_str(symbol);
        header.push_str(" rs_backend_");
        header.push_str(symbol);
        header.push('\n');
    }
    fs::write(output, header).expect("write TurboJPEG prefix header");
}

fn compile_backend(
    out_dir: &Path,
    generated_include: &Path,
    prefix_header: &Path,
    original_root: &Path,
) -> Vec<PathBuf> {
    BACKEND_SOURCES
        .iter()
        .map(|source| {
            let source_path = original_root.join(source);
            let object_path = out_dir.join(format!("{source}.o"));
            run(
                Command::new("gcc")
                    .arg("-std=c99")
                    .arg("-O2")
                    .arg("-fPIC")
                    .arg("-DBMP_SUPPORTED")
                    .arg("-DPPM_SUPPORTED")
                    .arg("-I")
                    .arg(generated_include)
                    .arg("-I")
                    .arg(original_root)
                    .arg("-include")
                    .arg(prefix_header)
                    .arg("-c")
                    .arg(&source_path)
                    .arg("-o")
                    .arg(&object_path),
            );
            object_path
        })
        .collect()
}

fn archive_objects(archive: &Path, objects: &[PathBuf]) {
    let mut command = Command::new("ar");
    command.arg("crus").arg(archive);
    for object in objects {
        command.arg(object);
    }
    run(&mut command);
}

fn run(command: &mut Command) {
    let status = command.status().expect("failed to run build helper");
    if !status.success() {
        panic!("build helper exited with status {status}");
    }
}
