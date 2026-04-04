#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

require_command() {
  local name="$1"
  command -v "$name" >/dev/null 2>&1 || die "missing required command: $name"
}

require_command rg
require_command awk
require_command sort

declare -a cargo_bridge_files=(
  "$SAFE_ROOT/build.rs"
  "$SAFE_ROOT/crates/jpeg-tools/build.rs"
  "$SAFE_ROOT/crates/libjpeg-abi/build.rs"
  "$SAFE_ROOT/crates/libturbojpeg-abi/build.rs"
)

if rg -n 'original/.*\.c' "${cargo_bridge_files[@]}"; then
  die "Cargo-side build helpers still reference original/*.c sources"
fi

if [[ -e "$SAFE_ROOT/bridge/libjpeg_compat.c" ]]; then
  die "temporary libjpeg compatibility bridge source still exists"
fi

if rg -n 'bridge/libjpeg_compat\.c|libjpeg_compat\.c' \
  "$SAFE_ROOT/build.rs" \
  "$SAFE_ROOT/crates" \
  "$SAFE_ROOT/scripts/stage-install.sh" \
  "$SAFE_ROOT/README.md"; then
  die "temporary libjpeg compatibility bridge is still referenced"
fi

unsafe_report="$(mktemp)"
trap 'rm -f "$unsafe_report"' EXIT

rg -n -o '\bunsafe\b' "$SAFE_ROOT/crates" "$SAFE_ROOT/tests" \
  | awk -F: '{ count[$1] += 1 } END { for (file in count) printf "%5d %s\n", count[file], file }' \
  | sort -nr >"$unsafe_report"

printf 'Unsafe footprint by file:\n'
cat "$unsafe_report"

unexpected_files="$(
  rg --files-with-matches '\bunsafe\b' "$SAFE_ROOT/crates" "$SAFE_ROOT/tests" \
    | sort \
    | grep -Ev '^'"$SAFE_ROOT"'/crates/(ffi-types|jpeg-core|libjpeg-abi|libturbojpeg-abi)/|^'"$SAFE_ROOT"'/tests/(compat_smoke|cve_regressions|turbojpeg_suite|upstream_matrix)\.rs$' \
    || true
)"

if [[ -n "$unexpected_files" ]]; then
  printf '\nUnexpected unsafe outside the reviewed ABI/core boundary:\n%s\n' "$unexpected_files" >&2
  exit 1
fi

printf '\nResidual compatibility bridge references:\n'
rg -n 'LIBJPEG_TURBO_BACKEND_LIB|target/upstream-bootstrap|dlopen\(|dlsym\(' \
  "$SAFE_ROOT/crates/libturbojpeg-abi/src/lib.rs" \
  "$SAFE_ROOT/crates/jpeg-tools/src/lib.rs" \
  "$SAFE_ROOT/scripts/stage-install.sh" \
  "$SAFE_ROOT/tests/turbojpeg_suite.rs" \
  "$SAFE_ROOT/tests/upstream_matrix.rs" \
  || true

printf '\naudit-unsafe: ok\n'
