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
  "$SAFE_ROOT/crates/libjpeg-abi/build.rs"
)

if rg -n 'original/.*\.c' "${cargo_bridge_files[@]}"; then
  die "Cargo-side build helpers still reference original/*.c sources"
fi

remaining_c_shim_refs="$(
  rg -n 'c_shim/error_bridge\.c' \
    "$SAFE_ROOT/build.rs" \
    "$SAFE_ROOT/crates" \
    "$SAFE_ROOT/scripts" \
    | grep -Fv "$SAFE_ROOT/crates/libjpeg-abi/build.rs:" \
    | grep -Fv "$SAFE_ROOT/build.rs:" \
    | grep -Fv "$SAFE_ROOT/scripts/audit-unsafe.sh:" \
    || true
)"

if [[ -n "$remaining_c_shim_refs" ]]; then
  printf '%s\n' "$remaining_c_shim_refs" >&2
  die "only libjpeg-abi/build.rs may compile the minimal error_bridge.c shim"
fi

if [[ -e "$SAFE_ROOT/bridge/libjpeg_compat.c" ]]; then
  die "temporary libjpeg compatibility bridge source still exists"
fi

for obsolete_dir in \
  "$SAFE_ROOT/c_shim/tools" \
  "$SAFE_ROOT/c_shim/turbojpeg"
do
  if find "$obsolete_dir" -type f -print -quit 2>/dev/null | grep -q .; then
    die "obsolete staged C frontend sources still exist under $obsolete_dir"
  fi
done

if [[ -e "$SAFE_ROOT/c_shim/jsimd_none.c" ]]; then
  die "obsolete jsimd_none.c C shim still exists"
fi

if rg -n 'bridge/libjpeg_compat\.c|libjpeg_compat\.c' \
  "$SAFE_ROOT/build.rs" \
  "$SAFE_ROOT/crates" \
  "$SAFE_ROOT/scripts/stage-install.sh" \
  "$SAFE_ROOT/README.md"; then
  die "temporary libjpeg compatibility bridge is still referenced"
fi

obsolete_c_frontend_refs="$(
  rg -n 'c_shim/(tools|turbojpeg)/|c_shim/jsimd_none\.c' \
    "$SAFE_ROOT/build.rs" \
    "$SAFE_ROOT/crates" \
    "$SAFE_ROOT/scripts" \
    "$SAFE_ROOT/tests" \
    "$SAFE_ROOT/README.md" \
    | grep -Fv "$SAFE_ROOT/scripts/audit-unsafe.sh:" \
    || true
)"

if [[ -n "$obsolete_c_frontend_refs" ]]; then
  printf '%s\n' "$obsolete_c_frontend_refs" >&2
  die "obsolete C frontend references remain in the committed tree"
fi

matches_any_pattern() {
  local file="$1"
  shift
  local pattern

  for pattern in "$@"; do
    if [[ "$file" =~ $pattern ]]; then
      return 0
    fi
  done

  return 1
}

collect_unexpected_files() {
  local -a allowed_patterns=("$@")
  local -a unexpected=()
  local file

  while IFS= read -r file; do
    [[ -n "$file" ]] || continue
    if ! matches_any_pattern "$file" "${allowed_patterns[@]}"; then
      unexpected+=("$file")
    fi
  done

  printf '%s\n' "${unexpected[@]}"
}

category_match_count() {
  local pattern="$1"
  local matches_file="$2"

  awk -F: '{ print $1 }' "$matches_file" | grep -Ec "$pattern" || true
}

category_file_count() {
  local pattern="$1"
  local matches_file="$2"

  (awk -F: '{ print $1 }' "$matches_file" | grep -E "$pattern" | sort -u | wc -l) || true
}

print_category_summary() {
  local title="$1"
  local matches_file="$2"
  shift 2

  printf '%s\n' "$title"

  while (($#)); do
    local label="$1"
    local pattern="$2"
    shift 2

    local marker_count
    local file_count
    marker_count="$(category_match_count "$pattern" "$matches_file")"
    file_count="$(category_file_count "$pattern" "$matches_file")"
    printf '  - %s: %s markers across %s files\n' "$label" "$marker_count" "$file_count"
  done
}

declare -a reviewed_unsafe_operation_patterns=(
  '^'"$SAFE_ROOT"'/crates/ffi-types/src/lib\.rs$'
  '^'"$SAFE_ROOT"'/crates/jpeg-core/src/common/(error|icc|memory|registry|source_dest|utils)\.rs$'
  '^'"$SAFE_ROOT"'/crates/jpeg-core/src/ported/compress/(jcapimin|jcapistd|jcarith|jccoefct|jccolor|jcdctmgr|jchuff|jcinit|jcmainct|jcmarker|jcmaster|jcparam|jcphuff|jcprepct|jcsample|jctrans|jfdctflt|jfdctfst|jfdctint)\.rs$'
  '^'"$SAFE_ROOT"'/crates/jpeg-core/src/ported/decompress/(jdapimin|jdapistd|jdarith|jdcoefct|jdcolor|jddctmgr|jdhuff|jdinput|jdmainct|jdmarker|jdmaster|jdmerge|jdphuff|jdpostct|jdsample|jdtrans|jidctflt|jidctfst|jidctint|jidctred|jquant1|jquant2)\.rs$'
  '^'"$SAFE_ROOT"'/crates/jpeg-core/src/ported/decompress/generated/[^/]+_translated\.rs$'
  '^'"$SAFE_ROOT"'/crates/jpeg-core/src/ported/transform/transupp\.rs$'
  '^'"$SAFE_ROOT"'/crates/jpeg-core/src/ported/turbojpeg/turbojpeg\.rs$'
  '^'"$SAFE_ROOT"'/crates/jpeg-tools/src/bin/tjexample\.rs$'
  '^'"$SAFE_ROOT"'/crates/jpeg-tools/src/generated/[^/]+\.rs$'
  '^'"$SAFE_ROOT"'/crates/libjpeg-abi/src/(common_exports|decompress_exports|jsimd_none|lib)\.rs$'
  '^'"$SAFE_ROOT"'/crates/libturbojpeg-abi/src/generated/[^/]+\.rs$'
  '^'"$SAFE_ROOT"'/tests/(compat_smoke|cve_regressions|turbojpeg_suite|upstream_matrix)\.rs$'
)

declare -a reviewed_unsafe_extern_block_patterns=(
  '^'"$SAFE_ROOT"'/crates/libjpeg-abi/src/lib\.rs$'
  '^'"$SAFE_ROOT"'/tests/(compat_smoke|turbojpeg_suite|upstream_matrix)\.rs$'
)

unsafe_operation_matches="$(mktemp)"
unsafe_extern_block_matches="$(mktemp)"
trap 'rm -f "$unsafe_operation_matches" "$unsafe_extern_block_matches"' EXIT

rg -n -o \
  -e 'unsafe\s*\{' \
  -e '^\s*(pub\s+)?unsafe\s+(extern\s+"C"\s+)?fn' \
  "$SAFE_ROOT/crates" "$SAFE_ROOT/tests" \
  | sort >"$unsafe_operation_matches"

rg -n -o \
  -e '^\s*unsafe\s+extern\s+"C"\s*\{' \
  "$SAFE_ROOT/crates" "$SAFE_ROOT/tests" \
  | sort >"$unsafe_extern_block_matches"

printf 'Executable unsafe excludes callback type signatures such as Option<unsafe extern "C" fn>.\n'

print_category_summary \
  'Unsafe function/block boundary summary:' \
  "$unsafe_operation_matches" \
  'ffi-types ABI declarations' '^'"$SAFE_ROOT"'/crates/ffi-types/src/lib\.rs$' \
  'jpeg-core raw-pointer runtime support' '^'"$SAFE_ROOT"'/crates/jpeg-core/src/common/' \
  'jpeg-core ported codec kernels' '^'"$SAFE_ROOT"'/crates/jpeg-core/src/ported/' \
  'libjpeg ABI exports and jsimd fallback hooks' '^'"$SAFE_ROOT"'/crates/libjpeg-abi/src/(common_exports|decompress_exports|jsimd_none|lib)\.rs$' \
  'TurboJPEG/JNI frontend translations' '^'"$SAFE_ROOT"'/crates/libturbojpeg-abi/src/generated/' \
  'CLI frontend translations' '^'"$SAFE_ROOT"'/crates/jpeg-tools/src/(generated/|bin/tjexample\.rs$)' \
  'integration-test harnesses' '^'"$SAFE_ROOT"'/tests/'

print_category_summary \
  'Unsafe extern block summary:' \
  "$unsafe_extern_block_matches" \
  'libjpeg native link declaration' '^'"$SAFE_ROOT"'/crates/libjpeg-abi/src/lib\.rs$' \
  'integration-test extern bindings' '^'"$SAFE_ROOT"'/tests/'

unsafe_operation_files="$(awk -F: '{ print $1 }' "$unsafe_operation_matches" | sort -u)"
unexpected_operation_files="$(
  collect_unexpected_files "${reviewed_unsafe_operation_patterns[@]}" <<< "$unsafe_operation_files"
)"

if [[ -n "$unexpected_operation_files" ]]; then
  printf '\nUnexpected unsafe function/block files outside the reviewed boundary:\n%s\n' "$unexpected_operation_files" >&2
  exit 1
fi

unsafe_extern_block_files="$(awk -F: '{ print $1 }' "$unsafe_extern_block_matches" | sort -u)"
unexpected_extern_block_files="$(
  collect_unexpected_files "${reviewed_unsafe_extern_block_patterns[@]}" <<< "$unsafe_extern_block_files"
)"

if [[ -n "$unexpected_extern_block_files" ]]; then
  printf '\nUnexpected unsafe extern blocks outside the reviewed boundary:\n%s\n' "$unexpected_extern_block_files" >&2
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
