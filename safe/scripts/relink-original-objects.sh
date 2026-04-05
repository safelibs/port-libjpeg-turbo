#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
MANIFEST="${LIBJPEG_TURBO_OBJECT_GROUPS_MANIFEST:-$SAFE_ROOT/scripts/original-object-groups.json}"
STAGE_DIR="${LIBJPEG_TURBO_STAGE_ROOT:-$SAFE_ROOT/target/relink-stage}"
CACHE_DIR="$SAFE_ROOT/target/original-object-cache"
OUT_DIR="$SAFE_ROOT/target/original-relinked"
RUN_TARGETS=0
LIST_GROUPS=0
declare -a RELINK_GROUPS=()

usage() {
  cat <<'EOF'
usage: relink-original-objects.sh [--group <name>]... [--stage-dir <dir>] [--cache-dir <dir>] [--out-dir <dir>] [--manifest <json>] [--run] [--list-groups]

Compiles cached upstream test and utility objects against the canonical staged
headers/configuration, then relinks them against the staged safe libraries.

--group may be repeated. Available groups are defined in
safe/scripts/original-object-groups.json and include at least:
  smoke, decompress, compress, turbojpeg, compat, cli, all
--manifest overrides the original-object group manifest consumed by this helper.
--run executes each relinked binary after linking.
--list-groups prints the available group names and exits.
EOF
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

while (($#)); do
  case "$1" in
    --group)
      RELINK_GROUPS+=("${2:?missing value for --group}")
      shift 2
      ;;
    --stage-dir)
      STAGE_DIR="${2:?missing value for --stage-dir}"
      shift 2
      ;;
    --cache-dir)
      CACHE_DIR="${2:?missing value for --cache-dir}"
      shift 2
      ;;
    --out-dir)
      OUT_DIR="${2:?missing value for --out-dir}"
      shift 2
      ;;
    --manifest)
      MANIFEST="${2:?missing value for --manifest}"
      shift 2
      ;;
    --run)
      RUN_TARGETS=1
      shift
      ;;
    --list-groups)
      LIST_GROUPS=1
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

if ((${#RELINK_GROUPS[@]} == 0)); then
  RELINK_GROUPS=("smoke")
fi

[[ -f "$MANIFEST" ]] || die "missing manifest: $MANIFEST"
command -v jq >/dev/null 2>&1 || die "jq is required"

if ((LIST_GROUPS)); then
  jq -r '.groups | keys[]' "$MANIFEST"
  exit 0
fi

if [[ ! -e "$STAGE_DIR/usr/lib" ]]; then
  DEB_HOST_MULTIARCH="${DEB_HOST_MULTIARCH:-}" \
    bash "$SAFE_ROOT/scripts/stage-install.sh" --stage-dir "$STAGE_DIR" --with-java=0
fi

multiarch() {
  if [[ -n "${DEB_HOST_MULTIARCH:-}" ]]; then
    printf '%s\n' "$DEB_HOST_MULTIARCH"
  elif command -v dpkg-architecture >/dev/null 2>&1; then
    dpkg-architecture -qDEB_HOST_MULTIARCH
  else
    gcc -print-multiarch
  fi
}

MULTIARCH="$(multiarch)"
LIB_DIR="$STAGE_DIR/usr/lib/$MULTIARCH"
INC_DIR="$STAGE_DIR/usr/include"
INC_MULTIARCH_DIR="$INC_DIR/$MULTIARCH"

[[ -d "$LIB_DIR" ]] || die "missing staged library directory: $LIB_DIR"

mapfile -t TARGETS < <(
  {
    for group in "${RELINK_GROUPS[@]}"; do
      jq -e --arg group "$group" '.groups[$group]' "$MANIFEST" >/dev/null \
        || die "unknown relink group: $group"
      jq -r --arg group "$group" '.groups[$group][]' "$MANIFEST"
    done
  } | awk '!seen[$0]++'
)

((${#TARGETS[@]})) || die "no targets selected"

mkdir -p "$CACHE_DIR/$MULTIARCH" "$OUT_DIR/$MULTIARCH"

compile_source() {
  local target="$1"
  local source_rel="$2"
  local source="$ROOT/original/$source_rel"
  local obj_dir="$CACHE_DIR/$MULTIARCH/$target"
  local obj_name="${source_rel//\//_}"
  local obj="$obj_dir/${obj_name%.c}.o"
  shift 2
  local -a extra_cflags=("$@")

  mkdir -p "$obj_dir"
  if [[ ! -f "$obj" || "$source" -nt "$obj" ]]; then
    gcc -c -O2 -fPIC \
      -I"$INC_MULTIARCH_DIR" \
      -I"$INC_DIR" \
      -I"$ROOT/original" \
      "${extra_cflags[@]}" \
      -o "$obj" \
      "$source"
  fi
  printf '%s\n' "$obj"
}

for target in "${TARGETS[@]}"; do
  mapfile -t sources < <(jq -r --arg target "$target" '.targets[$target].sources[]' "$MANIFEST")
  mapfile -t cflags < <(jq -r --arg target "$target" '.targets[$target].cflags[]?' "$MANIFEST")
  mapfile -t libs < <(jq -r --arg target "$target" '.targets[$target].libs[]' "$MANIFEST")
  mapfile -t ldflags < <(jq -r --arg target "$target" '.targets[$target].ldflags[]?' "$MANIFEST")

  [[ ${#sources[@]} -gt 0 ]] || die "target $target has no sources"
  [[ ${#libs[@]} -gt 0 ]] || die "target $target has no libraries"

  objects=()
  for source_rel in "${sources[@]}"; do
    objects+=("$(compile_source "$target" "$source_rel" "${cflags[@]}")")
  done

  output="$OUT_DIR/$MULTIARCH/$target"
  gcc -o "$output" \
    "${objects[@]}" \
    -L"$LIB_DIR" \
    -Wl,-rpath,"$LIB_DIR" \
    "${ldflags[@]}" \
    $(printf '%s ' "${libs[@]/#/-l}")

  if ((RUN_TARGETS)); then
    LD_LIBRARY_PATH="$LIB_DIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" "$output"
  fi
done

printf 'relinked %d target(s) under %s\n' "${#TARGETS[@]}" "$OUT_DIR/$MULTIARCH"
