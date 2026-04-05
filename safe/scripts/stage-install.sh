#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
STAGE_DIR="${LIBJPEG_TURBO_STAGE_ROOT:-$SAFE_ROOT/stage}"
TMP_RENDER_ROOT="$SAFE_ROOT/target/rendered"
JAVA_TOOL_ROOT="$SAFE_ROOT/target/java-tools"
JAVA_TOOL_BIN_DIR="$JAVA_TOOL_ROOT/bin"
SYMBOLS_TOOL="$SAFE_ROOT/scripts/debian_symbols.py"
STAGE_INSTALL_LOCK="$SAFE_ROOT/target/stage-install.lock"
WITH_JAVA_MODE="auto"
CLEAN=0
IGNORED_BUILD_DIR=""
ARGV=("$@")
JAVA_DOCKER_IMAGE="${LIBJPEG_TURBO_JAVA_BUILD_IMAGE:-libjpeg-turbo-java-build:ubuntu24.04-r2}"

UPSTREAM_VERSION="2.1.5"
COPYRIGHT_YEAR="1991-2023"
BUILD_STRING="20260403"
LIBJPEG_TURBO_VERSION_NUMBER="2001005"
LIBJPEG_REALNAME="libjpeg.so.8.2.2"
LIBTURBOJPEG_REALNAME="libturbojpeg.so.0.2.0"

usage() {
  cat <<'EOF'
usage: stage-install.sh [--build-dir <dir>] [--stage-dir <dir>] [--with-java auto|0|1] [--clean]

Stage the self-contained install tree under safe/stage/usr/.  libjpeg is linked
from the Rust workspace, and libturbojpeg plus the packaged command-line tools
are built from the committed Rust workspace crates against the staged Rust
libjpeg core.

--build-dir is retained as a compatibility no-op for older harnesses.
--stage-dir overrides the staged install root.
--with-java controls whether turbojpeg.jar is built from the committed Java
sources:
  auto: enable when Java compiler/jar tooling is available locally or via the Docker fallback
  0: disable turbojpeg.jar generation
  1: require the local host (or Docker fallback) to provide Java compiler/jar tooling
--clean removes the rendered staging output first.
EOF
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

objcopy_bin() {
  if command -v objcopy >/dev/null 2>&1; then
    printf 'objcopy\n'
  elif command -v llvm-objcopy >/dev/null 2>&1; then
    printf 'llvm-objcopy\n'
  else
    die "missing required command: objcopy or llvm-objcopy"
  fi
}

acquire_stage_install_lock() {
  mkdir -p "$(dirname "$STAGE_INSTALL_LOCK")"
  exec {stage_install_lock_fd}>"$STAGE_INSTALL_LOCK"
  flock "$stage_install_lock_fd"
}

clear_dir() {
  local path="$1"

  if [[ -d "$path" && ! -L "$path" ]]; then
    find "$path" -mindepth 1 -maxdepth 1 -exec rm -rf -- {} +
  else
    rm -rf "$path"
    mkdir -p "$path"
    return
  fi

  mkdir -p "$path"
}

fresh_writable_dir() {
  local preferred="$1"
  local label="$2"
  local parent fallback

  parent="$(dirname "$preferred")"
  mkdir -p "$parent"

  if rm -rf "$preferred" 2>/dev/null; then
    mkdir -p "$preferred" || die "unable to create $label directory at $preferred"
    if [[ -d "$preferred" && -w "$preferred" ]]; then
      printf '%s\n' "$preferred"
      return 0
    fi
  fi

  fallback="$(mktemp -d "$parent/$(basename "$preferred").tmp.XXXXXX")" \
    || die "unable to create fallback $label directory under $parent"
  printf 'warning: using %s for %s because %s is not removable by uid %s\n' \
    "$fallback" "$label" "$preferred" "$(id -u)" >&2
  printf '%s\n' "$fallback"
}

while (($#)); do
  case "$1" in
    --build-dir)
      IGNORED_BUILD_DIR="${2:?missing value for --build-dir}"
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

prepare_java_tool_wrappers() {
  local java_bin
  java_bin="$(java_bin_path)"
  [[ -n "$java_bin" ]] || return 0

  JAVA_TOOL_ROOT="$(fresh_writable_dir "$JAVA_TOOL_ROOT" "Java tool wrappers")"
  JAVA_TOOL_BIN_DIR="$JAVA_TOOL_ROOT/bin"
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
}

local_java_build_available() {
  [[ -n "$(java_bin_path)" ]] && have_java_compiler && have_java_archiver
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
      curl \
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
  local root_real stage_real
  local uid gid
  local -a extra_mounts=()

  uid="$(id -u)"
  gid="$(id -g)"
  docker_home="$(fresh_writable_dir "$docker_home" "Docker home")"
  mkdir -p "$STAGE_DIR"
  root_real="$(readlink -f "$ROOT")"
  stage_real="$(readlink -f "$STAGE_DIR")"

  case "$stage_real" in
    "$root_real"|"$root_real"/*)
      ;;
    *)
      extra_mounts+=(-v "$STAGE_DIR":"$STAGE_DIR")
      ;;
  esac

  build_java_fallback_image
  docker run --rm \
    --user "$uid:$gid" \
    -e LIBJPEG_TURBO_STAGE_INSTALL_IN_DOCKER=1 \
    -e HOME="$docker_home" \
    -e CARGO_HOME=/opt/cargo \
    -e RUSTUP_HOME=/opt/rustup \
    -v "$ROOT":"$ROOT" \
    "${extra_mounts[@]}" \
    -w "$ROOT" \
    "$JAVA_DOCKER_IMAGE" \
    bash "$SAFE_ROOT/scripts/stage-install.sh" "${ARGV[@]}"
  exit $?
}

maybe_reexec_for_java() {
  prepare_java_tool_wrappers
  if local_java_build_available; then
    return 0
  fi

  if [[ -n "${LIBJPEG_TURBO_STAGE_INSTALL_IN_DOCKER:-}" ]]; then
    if [[ "$WITH_JAVA_MODE" == "1" ]]; then
      die "--with-java=1 requires Java compiler/jar tooling inside the Docker fallback image"
    fi
    return 0
  fi

  if command -v docker >/dev/null 2>&1; then
    reexec_stage_install_in_docker
  fi

  die "Java compiler/jar tooling is required to build turbojpeg.jar, or Docker for fallback"
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
        die "--with-java=1 requires Java compiler/jar tooling"
      fi
      printf '%s\n' "$WITH_JAVA_MODE"
      ;;
  esac
}

run_javac() {
  if command -v javac >/dev/null 2>&1; then
    javac "$@"
  else
    local java_bin
    java_bin="$(java_bin_path)"
    "$java_bin" --module jdk.compiler/com.sun.tools.javac.Main "$@"
  fi
}

run_jar() {
  if command -v jar >/dev/null 2>&1; then
    jar "$@"
  else
    local java_bin
    java_bin="$(java_bin_path)"
    "$java_bin" --module jdk.jartool/sun.tools.jar.Main "$@"
  fi
}

cargo_release_build() {
  local rustflags="${RUSTFLAGS:-}"
  local cargo_profile_release_lto="${CARGO_PROFILE_RELEASE_LTO:-false}"

  [[ "$rustflags" == *"-Clinker-plugin-lto=no"* ]] \
    || rustflags="${rustflags:+$rustflags }-Clinker-plugin-lto=no"
  [[ "$rustflags" == *"-Cembed-bitcode=no"* ]] \
    || rustflags="${rustflags:+$rustflags }-Cembed-bitcode=no"

  CARGO_PROFILE_RELEASE_LTO="$cargo_profile_release_lto" \
    RUSTFLAGS="$rustflags" \
    cargo "$@"
}

sanitize_archive_for_system_linker() {
  local source="$1"
  local dest="$2"
  local tool

  mkdir -p "$(dirname -- "$dest")"
  cp "$source" "$dest"
  tool="$(objcopy_bin)"
  "$tool" \
    --remove-section=.llvmbc \
    --remove-section=.llvmcmd \
    "$dest"
  ranlib "$dest"
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
  shift 2
  local replacement key value
  local -a sed_args=(
    -e "s|@CMAKE_INSTALL_PREFIX@|/usr|g"
    -e "s|@CMAKE_INSTALL_DEFAULT_PREFIX@|/usr|g"
    -e "s|@CMAKE_INSTALL_FULL_LIBDIR@|/usr/lib/${MULTIARCH}|g"
    -e "s|@CMAKE_INSTALL_FULL_INCLUDEDIR@|/usr/include|g"
    -e "s|@VERSION@|${UPSTREAM_VERSION}|g"
    -e "s|@PACKAGE_VERSION@|${UPSTREAM_VERSION}|g"
    -e "s|@MULTIARCH@|${MULTIARCH}|g"
  )

  for replacement in "$@"; do
    key="${replacement%%=*}"
    value="${replacement#*=}"
    value="${value//\\/\\\\}"
    value="${value//&/\\&}"
    value="${value//|/\\|}"
    sed_args+=(-e "s|${key}|${value}|g")
  done

  sed \
    "${sed_args[@]}" \
    "$input" >"$output"
}

render_version_script() {
  local symbols_file="$1"
  local output="$2"
  shift 2
  python3 "$SYMBOLS_TOOL" render-version-script "$@" "$symbols_file" "$output"
}

java_home_dir() {
  local java_home

  java_home="$(
    java -XshowSettings:properties -version 2>&1 \
      | awk -F'= ' '/^[[:space:]]*java.home = / { print $2; exit }'
  )"
  [[ -n "$java_home" ]] || die "could not determine java.home"
  [[ -d "$java_home" ]] || die "reported java.home does not exist: $java_home"
  printf '%s\n' "$java_home"
}

ensure_rust_libjpeg_staticlib() {
  local staticlib="$SAFE_ROOT/target/release/liblibjpeg_abi.a"
  cargo_release_build build --manifest-path "$SAFE_ROOT/Cargo.toml" -p libjpeg-abi --release >/dev/null \
    || die "failed to build libjpeg-abi release staticlib"
  printf '%s\n' "$staticlib"
}

ensure_rust_libturbojpeg_staticlib() {
  local staticlib="$SAFE_ROOT/target/release/liblibturbojpeg_abi.a"
  cargo_release_build build --manifest-path "$SAFE_ROOT/Cargo.toml" -p libturbojpeg-abi --release >/dev/null \
    || die "failed to build libturbojpeg-abi release staticlib"
  printf '%s\n' "$staticlib"
}

ensure_packaged_tool_binaries() {
  cargo_release_build build --manifest-path "$SAFE_ROOT/Cargo.toml" -p jpeg-tools --release \
    --bin cjpeg \
    --bin djpeg \
    --bin jpegtran \
    --bin rdjpgcom \
    --bin wrjpgcom \
    --bin tjbench \
    --bin tjexample \
    --bin jpegexiforient >/dev/null \
    || die "failed to build staged jpeg-tools binaries"
}

link_rust_libjpeg() {
  local libdir="$STAGE_DIR/usr/lib/$MULTIARCH"
  local output="$libdir/$LIBJPEG_REALNAME"
  local staticlib sanitized_staticlib version_script

  mkdir -p "$libdir"
  staticlib="$(ensure_rust_libjpeg_staticlib)"
  sanitized_staticlib="$TMP_RENDER_ROOT/libjpeg.a"
  sanitize_archive_for_system_linker "$staticlib" "$sanitized_staticlib"
  version_script="$TMP_RENDER_ROOT/libjpeg.map"
  render_version_script "$SAFE_ROOT/debian/libjpeg-turbo8.symbols" "$version_script"

  gcc -shared \
    -fno-lto \
    -Wl,-soname,libjpeg.so.8 \
    -Wl,--version-script,"$version_script" \
    -o "$output" \
    -Wl,--whole-archive "$sanitized_staticlib" -Wl,--no-whole-archive \
    -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc

  install -m 644 "$sanitized_staticlib" "$libdir/libjpeg.a"
  ranlib "$libdir/libjpeg.a"
  ln -sfn "$LIBJPEG_REALNAME" "$libdir/libjpeg.so.8"
  ln -sfn libjpeg.so.8 "$libdir/libjpeg.so"
}

build_staged_libturbojpeg() {
  local libdir="$STAGE_DIR/usr/lib/$MULTIARCH"
  local output="$libdir/$LIBTURBOJPEG_REALNAME"
  local jpeg_staticlib turbojpeg_staticlib sanitized_jpeg_staticlib sanitized_turbojpeg_staticlib version_script

  jpeg_staticlib="$(ensure_rust_libjpeg_staticlib)"
  turbojpeg_staticlib="$(ensure_rust_libturbojpeg_staticlib)"
  sanitized_jpeg_staticlib="$TMP_RENDER_ROOT/libjpeg-for-libturbojpeg.a"
  sanitized_turbojpeg_staticlib="$TMP_RENDER_ROOT/libturbojpeg.a"
  sanitize_archive_for_system_linker "$jpeg_staticlib" "$sanitized_jpeg_staticlib"
  sanitize_archive_for_system_linker "$turbojpeg_staticlib" "$sanitized_turbojpeg_staticlib"
  version_script="$TMP_RENDER_ROOT/turbojpeg-mapfile.jni"
  mkdir -p "$libdir"
  # Render from the committed Debian symbols manifest so the staged SONAME and
  # package ABI contract stay aligned, including the canonical JNI exports.
  render_version_script "$SAFE_ROOT/debian/libturbojpeg.symbols" "$version_script"

  gcc -shared \
    -fno-lto \
    -Wl,-soname,libturbojpeg.so.0 \
    -Wl,--version-script,"$version_script" \
    -o "$output" \
    -Wl,--whole-archive "$sanitized_turbojpeg_staticlib" -Wl,--no-whole-archive \
    "$sanitized_jpeg_staticlib" \
    -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc

  install -m 644 "$sanitized_turbojpeg_staticlib" "$libdir/libturbojpeg.a"
  ranlib "$libdir/libturbojpeg.a"
  ln -sfn "$LIBTURBOJPEG_REALNAME" "$libdir/libturbojpeg.so.0"
  ln -sfn libturbojpeg.so.0 "$libdir/libturbojpeg.so"
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

install_committed_metadata() {
  local cmake_dir="$STAGE_DIR/usr/lib/$MULTIARCH/cmake/libjpeg-turbo"
  local pc_dir="$STAGE_DIR/usr/lib/$MULTIARCH/pkgconfig"
  local doc_dir="$STAGE_DIR/usr/share/doc/libjpeg-turbo"
  local man_dir="$STAGE_DIR/usr/share/man/man1"

  mkdir -p "$cmake_dir" "$pc_dir" "$doc_dir" "$man_dir"

  render_template "$SAFE_ROOT/pkgconfig/libjpeg.pc.in" "$pc_dir/libjpeg.pc"
  render_template "$SAFE_ROOT/pkgconfig/libturbojpeg.pc.in" "$pc_dir/libturbojpeg.pc"
  render_template "$SAFE_ROOT/cmake/libjpeg-turboConfig.cmake.in" "$cmake_dir/libjpeg-turboConfig.cmake"
  render_template "$SAFE_ROOT/cmake/libjpeg-turboConfigVersion.cmake.in" "$cmake_dir/libjpeg-turboConfigVersion.cmake"
  render_template "$SAFE_ROOT/cmake/libjpeg-turboTargets.cmake.in" "$cmake_dir/libjpeg-turboTargets.cmake"
  rm -f "$cmake_dir/libjpeg-turboTargets-release.cmake"

  install -m 644 "$ROOT/original/LICENSE.md" "$doc_dir/LICENSE.md"
  for doc in README.ijg README.md libjpeg.txt usage.txt wizard.txt example.txt structure.txt tjexample.c; do
    install -m 644 "$SAFE_ROOT/$doc" "$doc_dir/$doc"
  done

  for page in cjpeg.1 djpeg.1 jpegtran.1 rdjpgcom.1 wrjpgcom.1; do
    install -m 644 "$SAFE_ROOT/debian/$page" "$man_dir/$page"
  done

  render_tjbench_manpage "$TMP_RENDER_ROOT/tjbench.1"
  install -m 644 "$TMP_RENDER_ROOT/tjbench.1" "$man_dir/tjbench.1"
  install -m 644 "$SAFE_ROOT/debian/extra/jpegexiforient.1" "$man_dir/jpegexiforient.1"
  install -m 644 "$SAFE_ROOT/debian/extra/exifautotran.1" "$man_dir/exifautotran.1"
}

install_packaged_tools() {
  local bin_dir man_dir build_dir

  bin_dir="$STAGE_DIR/usr/bin"
  man_dir="$STAGE_DIR/usr/share/man/man1"
  build_dir="$SAFE_ROOT/target/release"
  ensure_packaged_tool_binaries

  mkdir -p "$bin_dir" "$man_dir"

  for tool in cjpeg djpeg jpegtran rdjpgcom wrjpgcom tjbench; do
    install -m 755 "$build_dir/$tool" "$bin_dir/$tool"
  done

  install -m 755 "$build_dir/jpegexiforient" "$bin_dir/jpegexiforient"
  install -m 755 "$SAFE_ROOT/debian/extra/exifautotran" "$bin_dir/exifautotran"
}

build_java_jar() {
  local work_root src_root classes_dir jar_dir
  local -a java_sources=()

  [[ "$WITH_JAVA" == "1" ]] || return 0

  work_root="$TMP_RENDER_ROOT/java"
  src_root="$work_root/src"
  classes_dir="$work_root/classes"
  jar_dir="$STAGE_DIR/usr/share/java"

  rm -rf "$work_root"
  mkdir -p "$src_root" "$classes_dir" "$jar_dir"
  cp -a "$SAFE_ROOT/java/." "$src_root/"
  render_template \
    "$SAFE_ROOT/java/org/libjpegturbo/turbojpeg/TJLoader-unix.java.in" \
    "$src_root/org/libjpegturbo/turbojpeg/TJLoader.java" \
    "@CMAKE_INSTALL_FULL_LIBDIR@=/usr/lib/$MULTIARCH" \
    "@CMAKE_INSTALL_DEFAULT_PREFIX@=/usr"

  mapfile -t java_sources < <(find "$src_root" -name '*.java' -print | sort)
  ((${#java_sources[@]} > 0)) || die "no Java sources found under $src_root"

  run_javac -encoding UTF-8 -d "$classes_dir" "${java_sources[@]}"
  rm -f "$jar_dir/turbojpeg.jar"
  (
    cd "$classes_dir"
    run_jar cfm "$jar_dir/turbojpeg.jar" "$SAFE_ROOT/java/MANIFEST.MF" .
  )
}

if ((CLEAN)); then
  clear_dir "$STAGE_DIR"
  clear_dir "$TMP_RENDER_ROOT"
  rm -rf "$JAVA_TOOL_ROOT" 2>/dev/null || true
fi

[[ -d "$ROOT/original" ]] || die "missing original source tree"
[[ -d "$SAFE_ROOT/java" ]] || die "missing safe/java source tree"

MULTIARCH="$(multiarch)"
maybe_reexec_for_java
WITH_JAVA="$(resolve_with_java)"
acquire_stage_install_lock

clear_dir "$TMP_RENDER_ROOT"
clear_dir "$STAGE_DIR"

if [[ -n "$IGNORED_BUILD_DIR" ]]; then
  mkdir -p "$IGNORED_BUILD_DIR"
fi

install_committed_headers
link_rust_libjpeg
build_staged_libturbojpeg
install_committed_metadata
install_packaged_tools
build_java_jar

printf 'staged install at %s/usr\n' "$STAGE_DIR"
