#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
MANIFEST="$SAFE_ROOT/scripts/original-object-groups.json"
STAGE_DIR="$SAFE_ROOT/stage"
BUILD_DIR="$SAFE_ROOT/target/compat-fixtures"

usage() {
  cat <<'EOF'
usage: run-compat-fixtures.sh [--stage-dir <dir>] [--build-dir <dir>] [fixture...]

Compile and run original compatibility fixtures against the staged safe headers
and libraries. Fixture names must match entries in
safe/scripts/original-object-groups.json. Defaults to: jcstest strtest
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

fixtures=()
while (($#)); do
  case "$1" in
    --stage-dir)
      STAGE_DIR="${2:?missing value for --stage-dir}"
      shift 2
      ;;
    --build-dir)
      BUILD_DIR="${2:?missing value for --build-dir}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    -*)
      die "unknown option: $1"
      ;;
    *)
      fixtures+=("$1")
      shift
      ;;
  esac
done

if ((${#fixtures[@]} == 0)); then
  fixtures=(jcstest strtest)
fi

[[ -f "$MANIFEST" ]] || die "missing manifest: $MANIFEST"

refresh_stage

MULTIARCH="$(dpkg-architecture -qDEB_HOST_MULTIARCH 2>/dev/null || gcc -print-multiarch)"
LIB_DIR="$STAGE_DIR/usr/lib/$MULTIARCH"
INC_DIR="$STAGE_DIR/usr/include"
INC_MULTIARCH_DIR="$INC_DIR/$MULTIARCH"

[[ -d "$LIB_DIR" ]] || die "missing staged library directory: $LIB_DIR"
mkdir -p "$BUILD_DIR/$MULTIARCH"
mkdir -p "$BUILD_DIR/$MULTIARCH/bin"

for fixture in "${fixtures[@]}"; do
  jq -e --arg fixture "$fixture" '.targets[$fixture]' "$MANIFEST" >/dev/null \
    || die "unknown fixture: $fixture"

  mapfile -t sources < <(jq -r --arg fixture "$fixture" '.targets[$fixture].sources[]' "$MANIFEST")
  mapfile -t cflags < <(jq -r --arg fixture "$fixture" '.targets[$fixture].cflags[]?' "$MANIFEST")
  mapfile -t libs < <(jq -r --arg fixture "$fixture" '.targets[$fixture].libs[]' "$MANIFEST")
  mapfile -t ldflags < <(jq -r --arg fixture "$fixture" '.targets[$fixture].ldflags[]?' "$MANIFEST")

  [[ ${#sources[@]} -gt 0 ]] || die "fixture $fixture has no sources"
  [[ ${#libs[@]} -gt 0 ]] || die "fixture $fixture has no libraries"

  obj_dir="$BUILD_DIR/$MULTIARCH/obj/$fixture"
  mkdir -p "$obj_dir"
  objects=()
  for source_rel in "${sources[@]}"; do
    source="$ROOT/original/$source_rel"
    [[ -f "$source" ]] || die "missing source: $source"
    obj_name="${source_rel//\//_}"
    obj="$obj_dir/${obj_name%.c}.o"
    gcc -c -O2 -fPIC \
      -I"$INC_MULTIARCH_DIR" \
      -I"$INC_DIR" \
      -I"$ROOT/original" \
      "${cflags[@]}" \
      -o "$obj" \
      "$source"
    objects+=("$obj")
  done

  exe="$BUILD_DIR/$MULTIARCH/bin/$fixture"
  printf 'building %s\n' "$fixture"
  gcc -o "$exe" \
    "${objects[@]}" \
    -L"$LIB_DIR" \
    -Wl,-rpath,"$LIB_DIR" \
    "${ldflags[@]}" \
    $(printf '%s ' "${libs[@]/#/-l}")

  printf 'running %s\n' "$fixture"
  LD_LIBRARY_PATH="$LIB_DIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" "$exe"
done
