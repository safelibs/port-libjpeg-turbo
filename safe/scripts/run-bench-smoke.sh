#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
REFERENCE_BUILD_DIR="$SAFE_ROOT/target/perf-reference"
HOST_LD_LIBRARY_PATH="${LD_LIBRARY_PATH-}"
TJBENCH_BENCHTIME="${LIBJPEG_TURBO_TJBENCH_BENCHTIME:-0.05}"
TJBENCH_MIN_RATIO="${LIBJPEG_TURBO_TJBENCH_MIN_RATIO:-0.18}"
TJBENCH_TILE_MIN_RATIO="${LIBJPEG_TURBO_TJBENCH_TILE_MIN_RATIO:-0.18}"
CLI_ITERATIONS="${LIBJPEG_TURBO_BENCH_ITERATIONS:-30}"
CJPEG_MAX_SLOWDOWN="${LIBJPEG_TURBO_CJPEG_MAX_SLOWDOWN:-1.75}"
DJPEG_MAX_SLOWDOWN="${LIBJPEG_TURBO_DJPEG_MAX_SLOWDOWN:-1.75}"

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "missing required command: $1"
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || die "missing required file: $path"
}

require_exec() {
  local path="$1"
  [[ -x "$path" ]] || die "missing required executable: $path"
}

multiarch() {
  if command -v dpkg-architecture >/dev/null 2>&1; then
    dpkg-architecture -qDEB_HOST_MULTIARCH
  elif command -v gcc >/dev/null 2>&1; then
    gcc -print-multiarch
  else
    printf '%s-linux-gnu\n' "$(uname -m)"
  fi
}

job_count() {
  if command -v getconf >/dev/null 2>&1; then
    getconf _NPROCESSORS_ONLN
  elif command -v nproc >/dev/null 2>&1; then
    nproc
  else
    printf '1\n'
  fi
}

ld_path_for() {
  local libdir="$1"

  if [[ -n "$HOST_LD_LIBRARY_PATH" ]]; then
    printf '%s:%s\n' "$libdir" "$HOST_LD_LIBRARY_PATH"
  else
    printf '%s\n' "$libdir"
  fi
}

run_logged_command() {
  local label="$1"
  local logfile="$2"
  shift 2

  if ! "$@" >"$logfile" 2>&1; then
    tail -n 200 "$logfile" >&2 || true
    die "$label failed"
  fi
}

run_logged_with_libdir() {
  local label="$1"
  local logfile="$2"
  local libdir="$3"
  shift 3

  if ! env LD_LIBRARY_PATH="$(ld_path_for "$libdir")" "$@" >"$logfile" 2>&1; then
    tail -n 200 "$logfile" >&2 || true
    die "$label failed"
  fi
}

time_iterations() {
  local iterations="$1"
  local libdir="$2"
  shift 2
  local start_ns end_ns

  start_ns="$(date +%s%N)"
  for ((i = 0; i < iterations; i++)); do
    env LD_LIBRARY_PATH="$(ld_path_for "$libdir")" "$@" >/dev/null 2>&1
  done
  end_ns="$(date +%s%N)"
  printf '%s\n' "$(( (end_ns - start_ns) / 1000000 ))"
}

check_max_slowdown() {
  local label="$1"
  local current_ms="$2"
  local reference_ms="$3"
  local max_ratio="$4"
  local slowdown

  slowdown="$(
    awk -v current="$current_ms" -v reference="$reference_ms" '
      BEGIN {
        if (reference <= 0) {
          exit 1
        }
        printf "%.4f", current / reference
      }
    '
  )" || die "unable to compute slowdown for $label"

  printf '%-18s %8s x (%4sms vs %4sms)\n' \
    "$label" "$slowdown" "$current_ms" "$reference_ms"

  awk -v slowdown="$slowdown" -v limit="$max_ratio" '
    BEGIN {
      exit !(slowdown + 0 <= limit + 0)
    }
  ' || die "$label exceeded maximum slowdown ${max_ratio}x"
}

extract_tjbench_samples() {
  awk '
    /^>>>>>/ {
      section = $0
      gsub(/^>>>>>[[:space:]]+/, "", section)
      gsub(/[[:space:]]+<<<<<$/, "", section)
      next
    }
    /^Compress[[:space:]]+-->/ {
      phase = "compress"
      next
    }
    /^Decompress[[:space:]]+-->/ {
      phase = "decompress"
      next
    }
    /Throughput:/ {
      if (section != "" && phase != "") {
        printf "%s\t%s\t%s\n", section, phase, $2
      }
    }
  ' "$1"
}

check_min_throughput_ratio() {
  local label="$1"
  local current_log="$2"
  local reference_log="$3"
  local min_ratio="$4"
  local i worst_ratio worst_label worst_current worst_reference ratio
  local -a current_samples reference_samples

  mapfile -t current_samples < <(extract_tjbench_samples "$current_log")
  mapfile -t reference_samples < <(extract_tjbench_samples "$reference_log")

  [[ ${#current_samples[@]} -gt 0 ]] || die "$label did not report any throughput samples"
  [[ ${#current_samples[@]} -eq ${#reference_samples[@]} ]] \
    || die "$label reported a mismatched number of samples"

  worst_ratio="9999"
  for ((i = 0; i < ${#current_samples[@]}; i++)); do
    local current_section current_phase current_value
    local reference_section reference_phase reference_value

    IFS=$'\t' read -r current_section current_phase current_value <<<"${current_samples[$i]}"
    IFS=$'\t' read -r reference_section reference_phase reference_value <<<"${reference_samples[$i]}"

    [[ "$current_section" == "$reference_section" ]] \
      || die "$label sample mismatch at index $i: '$current_section' vs '$reference_section'"
    [[ "$current_phase" == "$reference_phase" ]] \
      || die "$label phase mismatch at index $i: '$current_phase' vs '$reference_phase'"

    ratio="$(
      awk -v current="$current_value" -v reference="$reference_value" '
        BEGIN {
          if (reference <= 0) {
            exit 1
          }
          printf "%.4f", current / reference
        }
      '
    )" || die "unable to compute throughput ratio for $label"

    if awk -v lhs="$ratio" -v rhs="$worst_ratio" 'BEGIN { exit !(lhs + 0 < rhs + 0) }'; then
      worst_ratio="$ratio"
      worst_label="$current_section / $current_phase"
      worst_current="$current_value"
      worst_reference="$reference_value"
    fi
  done

  printf '%-18s %8s x (%s)\n' "$label" "$worst_ratio" "$worst_label"
  printf '  current/reference throughput: %s / %s Megapixels/sec\n' \
    "$worst_current" "$worst_reference"

  awk -v ratio="$worst_ratio" -v minimum="$min_ratio" '
    BEGIN {
      exit !(ratio + 0 >= minimum + 0)
    }
  ' || die "$label fell below minimum throughput ratio $min_ratio"
}

build_reference_tools() {
  require_cmd cmake

  if [[ -x "$REFERENCE_BUILD_DIR/cjpeg" && -x "$REFERENCE_BUILD_DIR/djpeg" \
    && -x "$REFERENCE_BUILD_DIR/tjbench" ]]; then
    return 0
  fi

  mkdir -p "$REFERENCE_BUILD_DIR"
  run_logged_command \
    "cmake configure" \
    "$WORK_ROOT/reference-configure.log" \
    cmake -S "$ROOT/original" -B "$REFERENCE_BUILD_DIR" \
      -DCMAKE_BUILD_TYPE=Release \
      -DENABLE_SHARED=1 \
      -DWITH_JAVA=0
  run_logged_command \
    "cmake build" \
    "$WORK_ROOT/reference-build.log" \
    cmake --build "$REFERENCE_BUILD_DIR" \
      --target cjpeg djpeg tjbench \
      -j"$(job_count)"
}

require_file "$ROOT/original/testimages/testorig.ppm"
require_file "$ROOT/original/testimages/testorig.jpg"

WORK_ROOT="$(mktemp -d "$SAFE_ROOT/target/bench-smoke.XXXXXX")"
trap 'rm -rf "$WORK_ROOT"' EXIT

STAGE_DIR="$WORK_ROOT/stage"
CURRENT_INPUT_DIR="$WORK_ROOT/current"
REFERENCE_INPUT_DIR="$WORK_ROOT/reference"
mkdir -p "$CURRENT_INPUT_DIR" "$REFERENCE_INPUT_DIR"

run_logged_command \
  "stage install" \
  "$WORK_ROOT/stage-install.log" \
  bash "$SAFE_ROOT/scripts/stage-install.sh" \
    --stage-dir "$STAGE_DIR" \
    --with-java 0

if rg -q 'Opaque pointers are only supported|-plugin has failed to create LTO module' \
  "$WORK_ROOT/stage-install.log"; then
  tail -n 200 "$WORK_ROOT/stage-install.log" >&2 || true
  die "stage install emitted linker-plugin warnings; archives were not sanitized correctly"
fi

build_reference_tools

MULTIARCH="$(multiarch)"
CURRENT_BINDIR="$STAGE_DIR/usr/bin"
CURRENT_LIBDIR="$STAGE_DIR/usr/lib/$MULTIARCH"
REFERENCE_BINDIR="$REFERENCE_BUILD_DIR"
REFERENCE_LIBDIR="$REFERENCE_BUILD_DIR"

require_exec "$CURRENT_BINDIR/cjpeg"
require_exec "$CURRENT_BINDIR/djpeg"
require_exec "$CURRENT_BINDIR/tjbench"
require_exec "$REFERENCE_BINDIR/cjpeg"
require_exec "$REFERENCE_BINDIR/djpeg"
require_exec "$REFERENCE_BINDIR/tjbench"

cp "$ROOT/original/testimages/testorig.ppm" "$CURRENT_INPUT_DIR/testorig.ppm"
cp "$ROOT/original/testimages/testorig.jpg" "$CURRENT_INPUT_DIR/testorig.jpg"
cp "$ROOT/original/testimages/testorig.ppm" "$REFERENCE_INPUT_DIR/testorig.ppm"
cp "$ROOT/original/testimages/testorig.jpg" "$REFERENCE_INPUT_DIR/testorig.jpg"

printf 'Benchmark thresholds\n'
printf '  tjbench        >= %sx original throughput\n' "$TJBENCH_MIN_RATIO"
printf '  tjbench -tile  >= %sx original throughput\n' "$TJBENCH_TILE_MIN_RATIO"
printf '  cjpeg          <= %sx original elapsed time\n' "$CJPEG_MAX_SLOWDOWN"
printf '  djpeg          <= %sx original elapsed time\n' "$DJPEG_MAX_SLOWDOWN"

CURRENT_CJPEG_MS="$(
  time_iterations \
    "$CLI_ITERATIONS" \
    "$CURRENT_LIBDIR" \
    "$CURRENT_BINDIR/cjpeg" -quality 95 -memdst \
    "$CURRENT_INPUT_DIR/testorig.ppm"
)"
REFERENCE_CJPEG_MS="$(
  time_iterations \
    "$CLI_ITERATIONS" \
    "$REFERENCE_LIBDIR" \
    "$REFERENCE_BINDIR/cjpeg" -quality 95 -memdst \
    "$REFERENCE_INPUT_DIR/testorig.ppm"
)"
check_max_slowdown \
  "cjpeg loop" \
  "$CURRENT_CJPEG_MS" \
  "$REFERENCE_CJPEG_MS" \
  "$CJPEG_MAX_SLOWDOWN"

CURRENT_DJPEG_MS="$(
  time_iterations \
    "$CLI_ITERATIONS" \
    "$CURRENT_LIBDIR" \
    "$CURRENT_BINDIR/djpeg" -ppm \
    "$CURRENT_INPUT_DIR/testorig.jpg"
)"
REFERENCE_DJPEG_MS="$(
  time_iterations \
    "$CLI_ITERATIONS" \
    "$REFERENCE_LIBDIR" \
    "$REFERENCE_BINDIR/djpeg" -ppm \
    "$REFERENCE_INPUT_DIR/testorig.jpg"
)"
check_max_slowdown \
  "djpeg loop" \
  "$CURRENT_DJPEG_MS" \
  "$REFERENCE_DJPEG_MS" \
  "$DJPEG_MAX_SLOWDOWN"

run_logged_with_libdir \
  "tjbench" \
  "$WORK_ROOT/tjbench-current.log" \
  "$CURRENT_LIBDIR" \
  "$CURRENT_BINDIR/tjbench" \
  "$CURRENT_INPUT_DIR/testorig.ppm" 95 \
  -rgb -benchtime "$TJBENCH_BENCHTIME" -warmup 0
run_logged_with_libdir \
  "reference tjbench" \
  "$WORK_ROOT/tjbench-reference.log" \
  "$REFERENCE_LIBDIR" \
  "$REFERENCE_BINDIR/tjbench" \
  "$REFERENCE_INPUT_DIR/testorig.ppm" 95 \
  -rgb -benchtime "$TJBENCH_BENCHTIME" -warmup 0
grep -q 'Throughput:' "$WORK_ROOT/tjbench-current.log" \
  || die "tjbench output did not contain throughput samples"
grep -q 'Throughput:' "$WORK_ROOT/tjbench-reference.log" \
  || die "reference tjbench output did not contain throughput samples"
check_min_throughput_ratio \
  "tjbench" \
  "$WORK_ROOT/tjbench-current.log" \
  "$WORK_ROOT/tjbench-reference.log" \
  "$TJBENCH_MIN_RATIO"

run_logged_with_libdir \
  "tjbench -tile" \
  "$WORK_ROOT/tjbench-tile-current.log" \
  "$CURRENT_LIBDIR" \
  "$CURRENT_BINDIR/tjbench" \
  "$CURRENT_INPUT_DIR/testorig.ppm" 95 \
  -rgb -tile -benchtime "$TJBENCH_BENCHTIME" -warmup 0
run_logged_with_libdir \
  "reference tjbench -tile" \
  "$WORK_ROOT/tjbench-tile-reference.log" \
  "$REFERENCE_LIBDIR" \
  "$REFERENCE_BINDIR/tjbench" \
  "$REFERENCE_INPUT_DIR/testorig.ppm" 95 \
  -rgb -tile -benchtime "$TJBENCH_BENCHTIME" -warmup 0
grep -q 'Throughput:' "$WORK_ROOT/tjbench-tile-current.log" \
  || die "tjbench -tile output did not contain throughput samples"
grep -q 'Throughput:' "$WORK_ROOT/tjbench-tile-reference.log" \
  || die "reference tjbench -tile output did not contain throughput samples"
check_min_throughput_ratio \
  "tjbench -tile" \
  "$WORK_ROOT/tjbench-tile-current.log" \
  "$WORK_ROOT/tjbench-tile-reference.log" \
  "$TJBENCH_TILE_MIN_RATIO"

printf 'run-bench-smoke: ok\n'
