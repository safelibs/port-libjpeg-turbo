#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
BUILD_DIR="$SAFE_ROOT/target/upstream-bootstrap"
STAGE_DIR="$SAFE_ROOT/stage"
TMP_INSTALL_ROOT="$SAFE_ROOT/target/upstream-install"
TMP_RENDER_ROOT="$SAFE_ROOT/target/rendered"
SYMBOLS_TOOL="$SAFE_ROOT/scripts/debian_symbols.py"
WITH_JAVA_MODE="auto"
CLEAN=0

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
  auto: enable only when javac is available
  0: disable JNI build and export surface
  1: require javac and enable JNI build
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

resolve_with_java() {
  case "$WITH_JAVA_MODE" in
    auto)
      if command -v javac >/dev/null 2>&1; then
        printf '1\n'
      else
        printf '0\n'
      fi
      ;;
    0|1)
      if [[ "$WITH_JAVA_MODE" == "1" ]] && ! command -v javac >/dev/null 2>&1; then
        die "--with-java=1 requires javac"
      fi
      printf '%s\n' "$WITH_JAVA_MODE"
      ;;
  esac
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
  local extra_object="${5:-}"
  local argv=()

  mapfile -d '' -t argv < <(
    python3 - "$link_txt" "$output.tmp" "$version_script" "$extra_object" <<'PY'
import shlex
import sys
from pathlib import Path

link_txt = Path(sys.argv[1])
output = sys.argv[2]
version_script = sys.argv[3]
extra_object = sys.argv[4]

args = shlex.split(link_txt.read_text(encoding="utf-8"))
rewritten = []
i = 0

while i < len(args):
    arg = args[i]
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

if extra_object:
    rewritten.append(extra_object)

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

  render_version_script "$ROOT/original/debian/libjpeg-turbo8.symbols" "$version_script"
  gcc -O2 -fPIC -I"$BUILD_DIR" -I"$ROOT/original" -c \
    "$SAFE_ROOT/bridge/libjpeg_compat.c" -o "$bridge_object"

  output="$(shared_library_target "$libdir/libjpeg.so.8")"
  run_relink_from_link_txt "$link_dir" "$link_txt" "$output" "$version_script" "$bridge_object"
}

relink_staged_libturbojpeg() {
  local libdir="$STAGE_DIR/usr/lib/$MULTIARCH"
  local output
  local version_script="$BUILD_DIR/libturbojpeg-bootstrap.map"
  local link_dir="$BUILD_DIR"
  local link_txt="$BUILD_DIR/CMakeFiles/turbojpeg.dir/link.txt"
  local skip_args=()

  if [[ "$WITH_JAVA" == "1" ]]; then
    return
  fi

  skip_args=(--skip-regex '^Java_org_libjpegturbo_turbojpeg_')

  render_version_script "$ROOT/original/debian/libturbojpeg.symbols" \
    "$version_script" "${skip_args[@]}"

  output="$(shared_library_target "$libdir/libturbojpeg.so.0")"
  run_relink_from_link_txt "$link_dir" "$link_txt" "$output" "$version_script"
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
}

install_extra_tools() {
  local bin_dir="$STAGE_DIR/usr/bin"
  local man_dir="$STAGE_DIR/usr/share/man/man1"

  mkdir -p "$bin_dir" "$man_dir"
  gcc -O2 -o "$bin_dir/jpegexiforient" "$SAFE_ROOT/debian/extra/jpegexiforient.c"
  install -m 755 "$SAFE_ROOT/debian/extra/exifautotran" "$bin_dir/exifautotran"
  install -m 644 "$SAFE_ROOT/debian/extra/jpegexiforient.1" "$man_dir/jpegexiforient.1"
  install -m 644 "$SAFE_ROOT/debian/extra/exifautotran.1" "$man_dir/exifautotran.1"
}

if ((CLEAN)); then
  rm -rf "$BUILD_DIR" "$STAGE_DIR" "$TMP_INSTALL_ROOT" "$TMP_RENDER_ROOT"
fi

[[ -d "$ROOT/original" ]] || die "missing original source tree"

MULTIARCH="$(multiarch)"
WITH_JAVA="$(resolve_with_java)"
JOBS="${JOBS:-$(cpu_count)}"

rm -rf "$TMP_INSTALL_ROOT" "$TMP_RENDER_ROOT" "$STAGE_DIR"
mkdir -p "$BUILD_DIR" "$TMP_INSTALL_ROOT" "$TMP_RENDER_ROOT"

cmake \
  -S "$ROOT/original" \
  -B "$BUILD_DIR" \
  -DCMAKE_BUILD_TYPE=Release \
  -DENABLE_SHARED=1 \
  -DENABLE_STATIC=1 \
  -DWITH_ARITH_DEC=1 \
  -DWITH_ARITH_ENC=1 \
  -DWITH_JPEG8=1 \
  -DWITH_JAVA="$WITH_JAVA" \
  -DWITH_TURBOJPEG=1 \
  -DCMAKE_INSTALL_PREFIX=/usr \
  -DCMAKE_INSTALL_BINDIR=/usr/bin \
  -DCMAKE_INSTALL_INCLUDEDIR=/usr/include \
  -DCMAKE_INSTALL_LIBDIR="/usr/lib/$MULTIARCH" \
  -DCMAKE_INSTALL_MANDIR=/usr/share/man

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
install_extra_tools

printf 'staged bootstrap install at %s/usr\n' "$STAGE_DIR"
