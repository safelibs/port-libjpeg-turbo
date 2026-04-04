#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
STAGE_DIR="$SAFE_ROOT/stage"
TESTS_DIR="$SAFE_ROOT/debian/tests"

usage() {
  cat <<'EOF'
usage: run-debian-autopkgtests.sh [--stage-dir <dir>]

Run the safe Debian dev-package autopkgtests against the staged safe
headers, pkg-config files, and libraries.
EOF
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

refresh_stage() {
  if [[ "${LIBJPEG_TURBO_SKIP_STAGE_REFRESH:-0}" == 1 ]]; then
    [[ -e "$STAGE_DIR/usr/lib" ]] || die "missing staged install: $STAGE_DIR"
    return
  fi

  CARGO_PROFILE_RELEASE_LTO=false \
  RUSTFLAGS=-Clinker-plugin-lto=no \
    bash "$SAFE_ROOT/scripts/stage-install.sh" --clean --stage-dir "$STAGE_DIR"
}

while (($#)); do
  case "$1" in
    --stage-dir)
      STAGE_DIR="${2:?missing value for --stage-dir}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      die "unknown option: $1"
      ;;
  esac
done

[[ -d "$TESTS_DIR" ]] || die "missing Debian tests directory: $TESTS_DIR"

refresh_stage

MULTIARCH="$(dpkg-architecture -qDEB_HOST_MULTIARCH 2>/dev/null || gcc -print-multiarch)"
LIB_DIR="$STAGE_DIR/usr/lib/$MULTIARCH"
PKGCONFIG_DIR="$LIB_DIR/pkgconfig"

[[ -d "$LIB_DIR" ]] || die "missing staged library directory: $LIB_DIR"
[[ -d "$PKGCONFIG_DIR" ]] || die "missing staged pkg-config directory: $PKGCONFIG_DIR"

WORKDIR="$(mktemp -d "$SAFE_ROOT/target/debian-autopkgtests.XXXXXX")"
trap 'rm -rf "$WORKDIR"' EXIT INT QUIT ABRT PIPE TERM

mkdir -p "$WORKDIR/debian/tests"
cp "$TESTS_DIR/control" "$WORKDIR/debian/tests/control"
cp "$TESTS_DIR/libjpeg-turbo8-dev" "$WORKDIR/debian/tests/libjpeg-turbo8-dev"
cp "$TESTS_DIR/libjpeg-turbo8-dev-static" "$WORKDIR/debian/tests/libjpeg-turbo8-dev-static"
chmod +x "$WORKDIR/debian/tests/libjpeg-turbo8-dev" "$WORKDIR/debian/tests/libjpeg-turbo8-dev-static"

run_test() {
  local name="$1"
  printf 'running %s\n' "$name"
  (
    cd "$WORKDIR"
    export PKG_CONFIG_LIBDIR="$PKGCONFIG_DIR"
    export PKG_CONFIG_SYSROOT_DIR="$STAGE_DIR"
    export LD_LIBRARY_PATH="$LIB_DIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
    "./debian/tests/$name"
  )
}

run_test libjpeg-turbo8-dev
run_test libjpeg-turbo8-dev-static
