#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
BUILD_DIR="$SAFE_ROOT/target/upstream-bootstrap"
SOURCE_ROOT="$SAFE_ROOT/target/upstream-source"
STAGE_DIR="$SAFE_ROOT/stage"
TMP_INSTALL_ROOT="$SAFE_ROOT/target/upstream-install"
TMP_RENDER_ROOT="$SAFE_ROOT/target/rendered"
JAVA_TOOL_ROOT="$SAFE_ROOT/target/java-tools"
JAVA_TOOL_BIN_DIR="$JAVA_TOOL_ROOT/bin"
SYMBOLS_TOOL="$SAFE_ROOT/scripts/debian_symbols.py"
WITH_JAVA_MODE="auto"
CLEAN=0
ARGV=("$@")
JAVA_DOCKER_IMAGE="${LIBJPEG_TURBO_JAVA_BUILD_IMAGE:-libjpeg-turbo-java-build:ubuntu24.04-r2}"

JAVA_BIN=""
JAVAC_BIN=""
JAR_BIN=""
JAVA_INCLUDE_PATH=""
JAVA_INCLUDE_PATH2=""
JAVA_AWT_LIBRARY=""
JAVA_JVM_LIBRARY=""

UPSTREAM_VERSION="2.1.5"
COPYRIGHT_YEAR="1991-2023"
BUILD_STRING="20260403"
LIBJPEG_TURBO_VERSION_NUMBER="2001005"

usage() {
  cat <<'EOF'
usage: stage-install.sh [--build-dir <dir>] [--stage-dir <dir>] [--with-java auto|0|1] [--clean]

Bootstraps the temporary upstream-C bridge for the Rust workspace, then stages
the Debian-style install tree under safe/stage/usr/.

--build-dir overrides the upstream CMake build directory.
--stage-dir overrides the staged install root.
--with-java controls whether the upstream TurboJPEG JNI wrapper is built:
  auto: enable when the local host can satisfy the Java/JNI build or when
        Docker is available as a fallback builder
  0: disable JNI build and export surface
  1: require the local host (or Docker fallback) to satisfy the Java/JNI build
--clean removes the cached bootstrap build and staged output first.
EOF
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

trim_leading_ws() {
  local value="$1"
  value="${value#"${value%%[![:space:]]*}"}"
  printf '%s' "$value"
}

while (($#)); do
  case "$1" in
    --build-dir)
      BUILD_DIR="${2:?missing value for --build-dir}"
      shift 2
      ;;
    --stage-dir)
      STAGE_DIR="${2:?missing value for --stage-dir}"
      shift 2
      ;;
    --with-java)
      WITH_JAVA_MODE="${2:?missing value for --with-java}"
      shift 2
      ;;
    --with-java=*)
      WITH_JAVA_MODE="${1#--with-java=}"
      shift
      ;;
    --clean)
      CLEAN=1
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      printf 'unknown option: %s\n' "$1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

case "$WITH_JAVA_MODE" in
  auto|0|1)
    ;;
  *)
    die "unsupported --with-java mode: $WITH_JAVA_MODE"
    ;;
esac

multiarch() {
  if command -v dpkg-architecture >/dev/null 2>&1; then
    dpkg-architecture -qDEB_HOST_MULTIARCH
  elif command -v gcc >/dev/null 2>&1; then
    gcc -print-multiarch
  else
    printf '%s-linux-gnu\n' "$(uname -m)"
  fi
}

cpu_count() {
  if command -v nproc >/dev/null 2>&1; then
    nproc
  else
    getconf _NPROCESSORS_ONLN
  fi
}

java_bin_path() {
  command -v java 2>/dev/null || true
}

java_module_tool_available() {
  local module_class="$1"
  local java_bin

  java_bin="$(java_bin_path)"
  [[ -n "$java_bin" ]] || return 1
  "$java_bin" --module "$module_class" -version >/dev/null 2>&1
}

have_java_compiler() {
  command -v javac >/dev/null 2>&1 || java_module_tool_available jdk.compiler/com.sun.tools.javac.Main
}

have_java_archiver() {
  command -v jar >/dev/null 2>&1 || java_module_tool_available jdk.jartool/sun.tools.jar.Main
}

java_home_dir() {
  local java_bin
  java_bin="$(java_bin_path)"
  [[ -n "$java_bin" ]] || return 1
  dirname -- "$(dirname -- "$(readlink -f "$java_bin")")"
}

find_java_tree_file() {
  local name="$1"
  local java_home

  java_home="$(java_home_dir 2>/dev/null || true)"
  if [[ -n "$java_home" ]]; then
    find "$java_home" -name "$name" -print -quit 2>/dev/null && return 0
  fi
  find /usr/lib/jvm -name "$name" -print -quit 2>/dev/null
}

refresh_java_paths() {
  local java_home

  JAVA_BIN="$(java_bin_path)"
  JAVAC_BIN="$(command -v javac 2>/dev/null || true)"
  JAR_BIN="$(command -v jar 2>/dev/null || true)"
  JAVA_INCLUDE_PATH=""
  JAVA_INCLUDE_PATH2=""
  JAVA_AWT_LIBRARY=""
  JAVA_JVM_LIBRARY=""

  if [[ -n "$JAVA_BIN" ]]; then
    java_home="$(java_home_dir 2>/dev/null || true)"
    if [[ -n "$java_home" && -f "$java_home/include/jni.h" ]]; then
      JAVA_INCLUDE_PATH="$java_home/include"
      if [[ -f "$java_home/include/linux/jni_md.h" ]]; then
        JAVA_INCLUDE_PATH2="$java_home/include/linux"
      else
        JAVA_INCLUDE_PATH2="$(find "$java_home/include" -mindepth 1 -maxdepth 1 -type d -print -quit 2>/dev/null || true)"
      fi
    fi
  fi

  JAVA_AWT_LIBRARY="$(find_java_tree_file libjawt.so || true)"
  JAVA_JVM_LIBRARY="$(find_java_tree_file libjvm.so || true)"
}

prepare_java_tool_wrappers() {
  local java_bin
  java_bin="$(java_bin_path)"
  [[ -n "$java_bin" ]] || return 0

  rm -rf "$JAVA_TOOL_ROOT"
  mkdir -p "$JAVA_TOOL_BIN_DIR"

  if ! command -v javac >/dev/null 2>&1 && java_module_tool_available jdk.compiler/com.sun.tools.javac.Main; then
    cat >"$JAVA_TOOL_BIN_DIR/javac" <<EOF
#!/usr/bin/env bash
exec "$java_bin" --module jdk.compiler/com.sun.tools.javac.Main "\$@"
EOF
    chmod +x "$JAVA_TOOL_BIN_DIR/javac"
  fi

  if ! command -v jar >/dev/null 2>&1 && java_module_tool_available jdk.jartool/sun.tools.jar.Main; then
    cat >"$JAVA_TOOL_BIN_DIR/jar" <<EOF
#!/usr/bin/env bash
exec "$java_bin" --module jdk.jartool/sun.tools.jar.Main "\$@"
EOF
    chmod +x "$JAVA_TOOL_BIN_DIR/jar"
  fi

  export PATH="$JAVA_TOOL_BIN_DIR:$PATH"
  refresh_java_paths
}

local_java_build_available() {
  refresh_java_paths
  have_java_compiler &&
    have_java_archiver &&
    [[ -n "$JAVA_INCLUDE_PATH" ]] &&
    [[ -n "$JAVA_INCLUDE_PATH2" ]] &&
    [[ -n "$JAVA_AWT_LIBRARY" ]] &&
    [[ -n "$JAVA_JVM_LIBRARY" ]]
}

build_java_fallback_image() {
  docker image inspect "$JAVA_DOCKER_IMAGE" >/dev/null 2>&1 && return 0

  docker build -t "$JAVA_DOCKER_IMAGE" - <<'DOCKERFILE'
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_HOME=/opt/cargo
ENV RUSTUP_HOME=/opt/rustup
ENV PATH=/opt/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      build-essential \
      ca-certificates \
      cargo \
      cmake \
      curl \
      nasm \
      openjdk-17-jdk \
      pkg-config \
      python3 \
      rustc \
 && rm -rf /var/lib/apt/lists/* \
 && curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain 1.85.1 \
 && chmod -R a+rX /opt/cargo /opt/rustup
DOCKERFILE
}

reexec_stage_install_in_docker() {
  local docker_home="$SAFE_ROOT/target/docker-home"
  local uid gid

  uid="$(id -u)"
  gid="$(id -g)"
  mkdir -p "$docker_home"
  build_java_fallback_image
  docker run --rm \
    --user "$uid:$gid" \
    -e LIBJPEG_TURBO_STAGE_INSTALL_IN_DOCKER=1 \
    -e HOME="$docker_home" \
    -e CARGO_HOME=/opt/cargo \
    -e RUSTUP_HOME=/opt/rustup \
    -v "$ROOT":"$ROOT" \
    -w "$ROOT" \
    "$JAVA_DOCKER_IMAGE" \
    bash "$SAFE_ROOT/scripts/stage-install.sh" "${ARGV[@]}"
  exit $?
}

maybe_reexec_for_java() {
  [[ "$WITH_JAVA_MODE" != "0" ]] || return 0

  prepare_java_tool_wrappers
  if local_java_build_available; then
    return 0
  fi

  if [[ -n "${LIBJPEG_TURBO_STAGE_INSTALL_IN_DOCKER:-}" ]]; then
    if [[ "$WITH_JAVA_MODE" == "1" ]]; then
      die "--with-java=1 requires Java compiler tools and JNI headers inside the Docker fallback image"
    fi
    return 0
  fi

  if command -v docker >/dev/null 2>&1; then
    reexec_stage_install_in_docker
  fi

  if [[ "$WITH_JAVA_MODE" == "1" ]]; then
    die "--with-java=1 requires Java compiler tools and JNI headers, or Docker for fallback"
  fi
}

resolve_with_java() {
  case "$WITH_JAVA_MODE" in
    auto)
      if local_java_build_available; then
        printf '1\n'
      else
        printf '0\n'
      fi
      ;;
    0|1)
      if [[ "$WITH_JAVA_MODE" == "1" ]] && ! local_java_build_available; then
        die "--with-java=1 requires Java compiler tools and JNI headers"
      fi
      printf '%s\n' "$WITH_JAVA_MODE"
      ;;
  esac
}

prepare_upstream_source_tree() {
  rm -rf "$SOURCE_ROOT"
  cp -a "$ROOT/original" "$SOURCE_ROOT"
  mkdir -p "$SOURCE_ROOT/java"
  cp -a "$SAFE_ROOT/java/." "$SOURCE_ROOT/java/"
  install -m 644 "$SAFE_ROOT/java/tjbenchtest.java.in" "$SOURCE_ROOT/tjbenchtest.java.in"
  install -m 644 "$SAFE_ROOT/java/tjexampletest.java.in" "$SOURCE_ROOT/tjexampletest.java.in"
}

render_jconfig_h() {
  local output="$1"
  cat >"$output" <<EOF
/* Version ID for the JPEG library.
 * Might be useful for tests like "#if JPEG_LIB_VERSION >= 60".
 */
#define JPEG_LIB_VERSION  80

/* libjpeg-turbo version */
#define LIBJPEG_TURBO_VERSION  ${UPSTREAM_VERSION}

/* libjpeg-turbo version in integer form */
#define LIBJPEG_TURBO_VERSION_NUMBER  ${LIBJPEG_TURBO_VERSION_NUMBER}

/* Support arithmetic encoding */
#define C_ARITH_CODING_SUPPORTED 1

/* Support arithmetic decoding */
#define D_ARITH_CODING_SUPPORTED 1

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
EOF
}

render_jconfigint_h() {
  local output="$1"
  cat >"$output" <<EOF
/* libjpeg-turbo build number */
#define BUILD  "${BUILD_STRING}"

/* Compiler's inline keyword */
#undef inline

/* How to obtain function inlining. */
#define INLINE  __inline__ __attribute__((always_inline))

/* How to obtain thread-local storage */
#define THREAD_LOCAL  __thread

/* Define to the full name of this package. */
#define PACKAGE_NAME  "libjpeg-turbo"

/* Version number of package */
#define VERSION  "${UPSTREAM_VERSION}"

/* The size of \`size_t', as computed by sizeof. */
#define SIZEOF_SIZE_T  8

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
EOF
}

render_jversion_h() {
  local output="$1"
  cat >"$output" <<EOF
/*
 * jversion.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-2020, Thomas G. Lane, Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010, 2012-2023, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains software version identification.
 */


#if JPEG_LIB_VERSION >= 80

#define JVERSION        "8d  15-Jan-2012"

#elif JPEG_LIB_VERSION >= 70

#define JVERSION        "7  27-Jun-2009"

#else

#define JVERSION        "6b  27-Mar-1998"

#endif

/*
 * NOTE: It is our convention to place the authors in the following order:
 * - libjpeg-turbo authors (2009-) in descending order of the date of their
 *   most recent contribution to the project, then in ascending order of the
 *   date of their first contribution to the project, then in alphabetical
 *   order
 * - Upstream authors in descending order of the date of the first inclusion of
 *   their code
 */

#define JCOPYRIGHT \\
  "Copyright (C) 2009-2023 D. R. Commander\\n" \\
  "Copyright (C) 2015, 2020 Google, Inc.\\n" \\
  "Copyright (C) 2019-2020 Arm Limited\\n" \\
  "Copyright (C) 2015-2016, 2018 Matthieu Darbois\\n" \\
  "Copyright (C) 2011-2016 Siarhei Siamashka\\n" \\
  "Copyright (C) 2015 Intel Corporation\\n" \\
  "Copyright (C) 2013-2014 Linaro Limited\\n" \\
  "Copyright (C) 2013-2014 MIPS Technologies, Inc.\\n" \\
  "Copyright (C) 2009, 2012 Pierre Ossman for Cendio AB\\n" \\
  "Copyright (C) 2009-2011 Nokia Corporation and/or its subsidiary(-ies)\\n" \\
  "Copyright (C) 1999-2006 MIYASAKA Masaru\\n" \\
  "Copyright (C) 1991-2020 Thomas G. Lane, Guido Vollbeding"

#define JCOPYRIGHT_SHORT \\
  "Copyright (C) ${COPYRIGHT_YEAR} The libjpeg-turbo Project and many others"
EOF
}

render_template() {
  local input="$1"
  local output="$2"
  sed \
    -e "s|@CMAKE_INSTALL_PREFIX@|/usr|g" \
    -e "s|@CMAKE_INSTALL_FULL_LIBDIR@|/usr/lib/${MULTIARCH}|g" \
    -e "s|@CMAKE_INSTALL_FULL_INCLUDEDIR@|/usr/include|g" \
    -e "s|@VERSION@|${UPSTREAM_VERSION}|g" \
    -e "s|@PACKAGE_VERSION@|${UPSTREAM_VERSION}|g" \
    -e "s|@MULTIARCH@|${MULTIARCH}|g" \
    "$input" >"$output"
}

render_version_script() {
  local symbols_file="$1"
  local output="$2"
  shift 2
  python3 "$SYMBOLS_TOOL" render-version-script "$@" "$symbols_file" "$output"
}

run_relink_from_link_txt() {
  local link_dir="$1"
  local link_txt="$2"
  local output="$3"
  local version_script="$4"
  local skip_basenames="${5:-}"
  if (($# >= 5)); then
    shift 5
  else
    shift "$#"
  fi
  local extra_args=("$@")
  local argv=()

  mapfile -d '' -t argv < <(
    python3 - "$link_txt" "$output.tmp" "$version_script" "$skip_basenames" "${extra_args[@]}" <<'PY'
import shlex
import sys
from pathlib import Path

link_txt = Path(sys.argv[1])
output = sys.argv[2]
version_script = sys.argv[3]
skip_basenames = {name for name in sys.argv[4].split(",") if name}
extra_args = sys.argv[5:]

args = shlex.split(link_txt.read_text(encoding="utf-8"))
rewritten = []
i = 0

while i < len(args):
    arg = args[i]
    if Path(arg).name in skip_basenames:
        i += 1
        continue
    if arg == "-o":
        rewritten.extend(["-o", output])
        i += 2
        continue
    if arg.startswith("-Wl,--version-script,"):
        rewritten.append(f"-Wl,--version-script,{version_script}")
        i += 1
        continue
    rewritten.append(arg)
    i += 1

rewritten.extend(extra_args)

sys.stdout.write("\0".join(rewritten))
sys.stdout.write("\0")
PY
  )

  (
    cd "$link_dir"
    "${argv[@]}"
  )
  mv "$output.tmp" "$output"
}

ensure_rust_libjpeg_staticlib() {
  local staticlib="$SAFE_ROOT/target/release/liblibjpeg_abi.a"
  cargo build --manifest-path "$SAFE_ROOT/Cargo.toml" -p libjpeg-abi --release >/dev/null
  printf '%s\n' "$staticlib"
}

ensure_rust_libturbojpeg_staticlib() {
  local staticlib="$SAFE_ROOT/target/release/liblibturbojpeg_abi.a"
  cargo build --manifest-path "$SAFE_ROOT/Cargo.toml" -p libturbojpeg-abi --release >/dev/null
  printf '%s\n' "$staticlib"
}

shared_library_target() {
  local symlink_path="$1"
  local target

  target="$(readlink "$symlink_path")"
  if [[ -n "$target" ]]; then
    printf '%s\n' "$(dirname -- "$symlink_path")/$target"
  else
    printf '%s\n' "$symlink_path"
  fi
}

relink_staged_libjpeg() {
  local libdir="$STAGE_DIR/usr/lib/$MULTIARCH"
  local output
  local version_script="$BUILD_DIR/libjpeg-bootstrap.map"
  local bridge_object="$BUILD_DIR/libjpeg_compat.o"
  local link_dir="$BUILD_DIR/sharedlib"
  local link_txt="$BUILD_DIR/sharedlib/CMakeFiles/jpeg.dir/link.txt"
  local rust_staticlib
  local skip_basenames

  render_version_script "$SAFE_ROOT/debian/libjpeg-turbo8.symbols" "$version_script"
  gcc -O2 -fPIC -I"$BUILD_DIR" -I"$SOURCE_ROOT" -c \
    "$SAFE_ROOT/bridge/libjpeg_compat.c" -o "$bridge_object"
  rust_staticlib="$(ensure_rust_libjpeg_staticlib)"
  skip_basenames="jcomapi.c.o,jerror.c.o,jutils.c.o,jmemmgr.c.o,jmemnobs.c.o,jdatasrc.c.o,jdatadst.c.o,jcicc.c.o,jdicc.c.o,jcapimin.c.o,jcapistd.c.o,jcarith.c.o,jccoefct.c.o,jccolor.c.o,jcdctmgr.c.o,jchuff.c.o,jcinit.c.o,jcmainct.c.o,jcmarker.c.o,jcmaster.c.o,jcparam.c.o,jcphuff.c.o,jcprepct.c.o,jcsample.c.o,jctrans.c.o,jdapimin.c.o,jdapistd.c.o,jdarith.c.o,jdcoefct.c.o,jdpostct.c.o,jdinput.c.o,jdmarker.c.o,jdhuff.c.o,jdphuff.c.o,jdmainct.c.o,jdmaster.c.o,jdmerge.c.o,jdsample.c.o,jdcolor.c.o,jddctmgr.c.o,jdtrans.c.o,jquant1.c.o,jquant2.c.o,jfdctint.c.o,jfdctfst.c.o,jfdctflt.c.o,jidctint.c.o,jidctfst.c.o,jidctflt.c.o,jidctred.c.o"

  output="$(shared_library_target "$libdir/libjpeg.so.8")"
  run_relink_from_link_txt \
    "$link_dir" \
    "$link_txt" \
    "$output" \
    "$version_script" \
    "$skip_basenames" \
    "$bridge_object" \
    -Wl,--whole-archive \
    "$rust_staticlib" \
    -Wl,--no-whole-archive \
    -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc
}

relink_staged_libturbojpeg() {
  local libdir="$STAGE_DIR/usr/lib/$MULTIARCH"
  local output
  local version_script="$BUILD_DIR/libturbojpeg-bootstrap.map"
  local link_dir="$BUILD_DIR"
  local link_txt="$BUILD_DIR/CMakeFiles/turbojpeg.dir/link.txt"
  local rust_staticlib
  local skip_basenames="jcomapi.c.o,jerror.c.o,jutils.c.o,jmemmgr.c.o,jmemnobs.c.o,jdatasrc.c.o,jdatadst.c.o,jcicc.c.o,jdicc.c.o,jcapimin.c.o,jcapistd.c.o,jcarith.c.o,jccoefct.c.o,jccolor.c.o,jcdctmgr.c.o,jchuff.c.o,jcinit.c.o,jcmainct.c.o,jcmarker.c.o,jcmaster.c.o,jcparam.c.o,jcphuff.c.o,jcprepct.c.o,jcsample.c.o,jctrans.c.o,jdapimin.c.o,jdapistd.c.o,jdarith.c.o,jdcoefct.c.o,jdpostct.c.o,jdinput.c.o,jdmarker.c.o,jdhuff.c.o,jdphuff.c.o,jdmainct.c.o,jdmaster.c.o,jdmerge.c.o,jdsample.c.o,jdcolor.c.o,jddctmgr.c.o,jdtrans.c.o,jquant1.c.o,jquant2.c.o,jfdctint.c.o,jfdctfst.c.o,jfdctflt.c.o,jidctint.c.o,jidctfst.c.o,jidctflt.c.o,jidctred.c.o,turbojpeg.c.o,transupp.c.o,jdatadst-tj.c.o,jdatasrc-tj.c.o,rdbmp.c.o,rdppm.c.o,wrbmp.c.o,wrppm.c.o"

  if [[ "$WITH_JAVA" == "1" ]]; then
    version_script="$SAFE_ROOT/link/turbojpeg-mapfile.jni"
  else
    render_version_script "$SAFE_ROOT/debian/libturbojpeg.symbols" \
      "$version_script" --skip-regex '^Java_org_libjpegturbo_turbojpeg_'
  fi

  rust_staticlib="$(ensure_rust_libturbojpeg_staticlib)"
  output="$(shared_library_target "$libdir/libturbojpeg.so.0")"
  run_relink_from_link_txt \
    "$link_dir" \
    "$link_txt" \
    "$output" \
    "$version_script" \
    "$skip_basenames" \
    -Wl,--whole-archive \
    "$rust_staticlib" \
    -Wl,--no-whole-archive \
    -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc
}

install_committed_headers() {
  local manifest="$SAFE_ROOT/include/install-manifest.txt"
  local header_path source_path generated_path

  while IFS= read -r header_path; do
    [[ -z "$header_path" || "$header_path" =~ ^# ]] && continue
    header_path="${header_path//@multiarch@/$MULTIARCH}"
    generated_path="$STAGE_DIR/$header_path"
    mkdir -p "$(dirname -- "$generated_path")"
    case "$(basename -- "$header_path")" in
      jconfig.h)
        render_jconfig_h "$generated_path"
        ;;
      jconfigint.h)
        render_jconfigint_h "$generated_path"
        ;;
      jversion.h)
        render_jversion_h "$generated_path"
        ;;
      *)
        source_path="$ROOT/original/$(basename -- "$header_path")"
        [[ -f "$source_path" ]] || die "missing upstream header source for $header_path"
        install -m 644 "$source_path" "$generated_path"
        ;;
    esac
  done <"$manifest"
}

install_committed_metadata() {
  local cmake_dir="$STAGE_DIR/usr/lib/$MULTIARCH/cmake/libjpeg-turbo"
  local pc_dir="$STAGE_DIR/usr/lib/$MULTIARCH/pkgconfig"
  local doc_dir="$STAGE_DIR/usr/share/doc/libjpeg-turbo"
  local man_dir="$STAGE_DIR/usr/share/man/man1"
  local bin_dir="$STAGE_DIR/usr/bin"

  mkdir -p "$cmake_dir" "$pc_dir" "$doc_dir" "$man_dir" "$bin_dir"

  render_template "$SAFE_ROOT/pkgconfig/libjpeg.pc.in" "$pc_dir/libjpeg.pc"
  render_template "$SAFE_ROOT/pkgconfig/libturbojpeg.pc.in" "$pc_dir/libturbojpeg.pc"
  render_template "$SAFE_ROOT/cmake/libjpeg-turboConfig.cmake.in" "$cmake_dir/libjpeg-turboConfig.cmake"
  render_template "$SAFE_ROOT/cmake/libjpeg-turboConfigVersion.cmake.in" "$cmake_dir/libjpeg-turboConfigVersion.cmake"
  render_template "$SAFE_ROOT/cmake/libjpeg-turboTargets.cmake.in" "$cmake_dir/libjpeg-turboTargets.cmake"
  rm -f "$cmake_dir/libjpeg-turboTargets-release.cmake"

  for doc in README.ijg README.md libjpeg.txt usage.txt wizard.txt example.txt structure.txt tjexample.c; do
    install -m 644 "$SAFE_ROOT/$doc" "$doc_dir/$doc"
  done

  for page in cjpeg.1 djpeg.1 jpegtran.1 rdjpgcom.1 wrjpgcom.1; do
    install -m 644 "$SAFE_ROOT/debian/$page" "$man_dir/$page"
  done

  render_tjbench_manpage "$TMP_RENDER_ROOT/tjbench.1"
  install -m 644 "$TMP_RENDER_ROOT/tjbench.1" "$man_dir/tjbench.1"
}

extract_tjbench_section() {
  local section="$1"
  awk -v section="[$section]" '
    $0 == section { emit = 1; next }
    /^\[[A-Z]+\]$/ { emit = 0 }
    emit { print }
  ' "$SAFE_ROOT/debian/tjbench.1.in"
}

render_tjbench_manpage() {
  local output="$1"
  local description comment copyright

  description="$(extract_tjbench_section DESCRIPTION)"
  comment="$(extract_tjbench_section COMMENT)"
  copyright="$(extract_tjbench_section COPYRIGHT)"

  cat >"$output" <<EOF
.TH TJBENCH 1 "03 April 2026" "libjpeg-turbo" "User Commands"
.SH NAME
tjbench \\- JPEG compression/decompression benchmark
.SH SYNOPSIS
.B tjbench
.I input-image
.I quality-or-output-format
.RI [ options ]
.SH DESCRIPTION
$description
.PP
This rendered page is generated from the committed Debian template in
\fBsafe/debian/tjbench.1.in\fR during staging.
.SH NOTES
.TP
\fB-limitscans\fR
Propagate the libjpeg/libturbojpeg scan-limit checks during benchmarked
decompression and transforms.
.TP
\fB-progressive\fR
Generate progressive JPEG output during compression benchmarks.
.TP
\fB-fastupsample\fR, \fB-fastdct\fR, \fB-accuratedct\fR
Select the decompression upsampling path and DCT quality/performance tradeoff.
.TP
\fB-tile\fR
Exercise the tiled encode/decode paths used by the upstream regression suite.
.TP
\fB-benchtime\fR, \fB-warmup\fR, \fB-quiet\fR
Control iteration timing and output verbosity for scripted use.
.SH AUTHOR
$comment
.SH COPYRIGHT
$copyright
EOF
}

build_rust_tools() {
  cargo build --manifest-path "$SAFE_ROOT/Cargo.toml" -p jpeg-tools --release --bins >/dev/null
}

install_packaged_tools() {
  local bin_dir="$STAGE_DIR/usr/bin"
  local man_dir="$STAGE_DIR/usr/share/man/man1"

  mkdir -p "$bin_dir" "$man_dir"
  build_rust_tools

  for tool in cjpeg djpeg jpegtran rdjpgcom wrjpgcom tjbench jpegexiforient; do
    install -m 755 "$SAFE_ROOT/target/release/$tool" "$bin_dir/$tool"
  done

  install -m 755 "$SAFE_ROOT/debian/extra/exifautotran" "$bin_dir/exifautotran"
  install -m 644 "$SAFE_ROOT/debian/extra/jpegexiforient.1" "$man_dir/jpegexiforient.1"
  install -m 644 "$SAFE_ROOT/debian/extra/exifautotran.1" "$man_dir/exifautotran.1"
}

if ((CLEAN)); then
  rm -rf "$BUILD_DIR" "$SOURCE_ROOT" "$STAGE_DIR" "$TMP_INSTALL_ROOT" "$TMP_RENDER_ROOT" \
    "$JAVA_TOOL_ROOT"
fi

[[ -d "$ROOT/original" ]] || die "missing original source tree"
[[ -d "$SAFE_ROOT/java" ]] || die "missing safe/java source tree"

MULTIARCH="$(multiarch)"
maybe_reexec_for_java
WITH_JAVA="$(resolve_with_java)"
JOBS="${JOBS:-$(cpu_count)}"

rm -rf "$BUILD_DIR" "$TMP_INSTALL_ROOT" "$TMP_RENDER_ROOT" "$STAGE_DIR"
mkdir -p "$BUILD_DIR" "$TMP_INSTALL_ROOT" "$TMP_RENDER_ROOT"
prepare_upstream_source_tree

cmake_args=(
  -S "$SOURCE_ROOT"
  -B "$BUILD_DIR"
  -DCMAKE_BUILD_TYPE=Release
  -DENABLE_SHARED=1
  -DENABLE_STATIC=1
  -DWITH_ARITH_DEC=1
  -DWITH_ARITH_ENC=1
  -DWITH_JPEG8=1
  -DWITH_JAVA="$WITH_JAVA"
  -DWITH_TURBOJPEG=1
  -DCMAKE_INSTALL_PREFIX=/usr
  -DCMAKE_INSTALL_BINDIR=/usr/bin
  -DCMAKE_INSTALL_INCLUDEDIR=/usr/include
  -DCMAKE_INSTALL_LIBDIR="/usr/lib/$MULTIARCH"
  -DCMAKE_INSTALL_MANDIR=/usr/share/man
)

if [[ "$WITH_JAVA" == "1" ]]; then
  refresh_java_paths
  cmake_args+=(
    -DJava_JAVA_EXECUTABLE="$JAVA_BIN"
    -DJava_JAVAC_EXECUTABLE="$JAVAC_BIN"
    -DJava_JAR_EXECUTABLE="$JAR_BIN"
    -DJAVA_AWT_LIBRARY="$JAVA_AWT_LIBRARY"
    -DJAVA_JVM_LIBRARY="$JAVA_JVM_LIBRARY"
    -DJAVA_INCLUDE_PATH="$JAVA_INCLUDE_PATH"
    -DJAVA_INCLUDE_PATH2="$JAVA_INCLUDE_PATH2"
  )
fi

cmake "${cmake_args[@]}"

cmake --build "$BUILD_DIR" --parallel "$JOBS"
DESTDIR="$TMP_INSTALL_ROOT" cmake --install "$BUILD_DIR"

mkdir -p "$STAGE_DIR"
cp -a "$TMP_INSTALL_ROOT/." "$STAGE_DIR/"

relink_staged_libjpeg
relink_staged_libturbojpeg

rm -f "$STAGE_DIR/usr/include/jconfig.h"
mkdir -p "$STAGE_DIR/usr/include/$MULTIARCH"
install_committed_headers
install_committed_metadata
install_packaged_tools

printf 'staged bootstrap install at %s/usr\n' "$STAGE_DIR"
