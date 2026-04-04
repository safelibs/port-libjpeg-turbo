#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
STAGE_USR="$SAFE_ROOT/stage/usr"

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
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

run_timed() {
  local label="$1"
  shift
  local log="$TMPDIR/${label}.log"
  local start end elapsed_ms

  start="$(date +%s%N)"
  "$@" >"$log" 2>&1
  end="$(date +%s%N)"
  elapsed_ms="$(( (end - start) / 1000000 ))"
  printf '%-18s %6d ms\n' "$label" "$elapsed_ms"
}

if [[ ! -d "$STAGE_USR" ]]; then
  bash "$SAFE_ROOT/scripts/stage-install.sh"
fi

MULTIARCH="$(multiarch)"
LIBDIR="$STAGE_USR/lib/$MULTIARCH"
BINDIR="$STAGE_USR/bin"

require_exec "$BINDIR/cjpeg"
require_exec "$BINDIR/djpeg"
require_exec "$BINDIR/tjbench"
require_file "$ROOT/original/testimages/testorig.ppm"
require_file "$ROOT/original/testimages/testorig.jpg"

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

export LD_LIBRARY_PATH="$LIBDIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

cp "$ROOT/original/testimages/testorig.ppm" "$TMPDIR/testorig.ppm"
cp "$ROOT/original/testimages/testorig.ppm" "$TMPDIR/testorig-tile.ppm"

run_timed \
  cjpeg_rgb \
  "$BINDIR/cjpeg" -quality 95 -outfile "$TMPDIR/testorig-q95.jpg" \
  "$ROOT/original/testimages/testorig.ppm"
require_file "$TMPDIR/testorig-q95.jpg"

run_timed \
  djpeg_ppm \
  "$BINDIR/djpeg" -ppm -outfile "$TMPDIR/testorig.ppm" \
  "$ROOT/original/testimages/testorig.jpg"
require_file "$TMPDIR/testorig.ppm"

run_timed \
  tjbench_rgb \
  "$BINDIR/tjbench" "$TMPDIR/testorig.ppm" 95 \
  -rgb -benchtime 0.02 -warmup 0
grep -Eq 'Frame rate|Throughput' "$TMPDIR/tjbench_rgb.log" \
  || die "tjbench output did not contain benchmark data"

run_timed \
  tjbench_tile \
  "$BINDIR/tjbench" "$TMPDIR/testorig-tile.ppm" 95 \
  -rgb -tile -benchtime 0.02 -warmup 0
grep -Eq 'Frame rate|Throughput' "$TMPDIR/tjbench_tile.log" \
  || die "tjbench tile output did not contain benchmark data"

printf 'run-bench-smoke: ok\n'
