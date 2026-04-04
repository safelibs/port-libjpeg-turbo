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

if [[ -e "$SAFE_ROOT/runtime" ]]; then
  die "obsolete safe/runtime compatibility artifacts still exist"
fi

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

bootstrap_refs="$(
  rg -n 'LIBJPEG_TURBO_UPSTREAM_BUILD_DIR|target/upstream-bootstrap' \
    "$ROOT/test-original.sh" \
    "$SAFE_ROOT/crates" \
    "$SAFE_ROOT/tests" \
    "$SAFE_ROOT/scripts" \
    "$SAFE_ROOT/README.md" \
    | grep -Fv "$SAFE_ROOT/scripts/audit-unsafe.sh:" \
    || true
)"

if [[ -n "$bootstrap_refs" ]]; then
  printf '%s\n' "$bootstrap_refs" >&2
  die "obsolete upstream-bootstrap references remain in the committed tree"
fi

legacy_backend_refs="$(
  rg -n \
    'LIBJPEG_TURBO_BACKEND_LIB|safe/runtime|libturbojpeg_backend|libjpeg-turbo-tools|exec_packaged_tool_backend|dlopen\(|dlsym\(' \
    "$ROOT/test-original.sh" \
    "$SAFE_ROOT/crates" \
    "$SAFE_ROOT/tests" \
    "$SAFE_ROOT/scripts" \
    "$SAFE_ROOT/README.md" \
    | grep -Ev 'tests/(turbojpeg_suite|upstream_matrix)\.rs:' \
    | grep -Fv "$SAFE_ROOT/scripts/audit-unsafe.sh:" \
    || true
)"

if [[ -n "$legacy_backend_refs" ]]; then
  printf '%s\n' "$legacy_backend_refs" >&2
  die "obsolete runtime/backend bridge references remain in the committed tree"
fi

printf '\naudit-unsafe: ok\n'
