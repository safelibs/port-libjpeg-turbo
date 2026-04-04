#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
STAGE_USR="$SAFE_ROOT/stage/usr"
TJEXAMPLE_BIN="$SAFE_ROOT/target/release/tjexample"

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -e "$path" ]] || die "missing required path: $path"
}

require_exec() {
  local path="$1"
  [[ -x "$path" ]] || die "missing required executable: $path"
}

compare_bmp_payload() {
  local left="$1"
  local right="$2"

  python3 - "$left" "$right" <<'PY'
from pathlib import Path
import sys

left = Path(sys.argv[1]).read_bytes()
right = Path(sys.argv[2]).read_bytes()
header = 54

if len(left) < header or len(right) < header:
    raise SystemExit("BMP payload compare requires 54-byte headers")
if left[header:] != right[header:]:
    raise SystemExit("BMP payloads differ")
PY
}

seed_exif_orientation() {
  local src="$1"
  local dst="$2"

  python3 - "$src" "$dst" <<'PY'
from pathlib import Path
import sys

src = Path(sys.argv[1]).read_bytes()
dst = Path(sys.argv[2])

if len(src) < 2 or src[:2] != b"\xff\xd8":
    raise SystemExit("expected a JPEG SOI marker")

app1 = bytes.fromhex(
    "ffe10022"
    "457869660000"
    "49492a0008000000"
    "0100"
    "120103000100000001000000"
    "00000000"
)

dst.write_bytes(src[:2] + app1 + src[2:])
PY
}

multiarch() {
  if command -v dpkg-architecture >/dev/null 2>&1; then
    dpkg-architecture -qDEB_HOST_MULTIARCH
  else
    gcc -print-multiarch
  fi
}

[[ -d "$STAGE_USR" ]] || die "missing staged tree under $STAGE_USR"

MULTIARCH="$(multiarch)"
LIBDIR="$STAGE_USR/lib/$MULTIARCH"
BINDIR="$STAGE_USR/bin"
MANDIR="$STAGE_USR/share/man/man1"

[[ -d "$LIBDIR" ]] || die "missing staged library directory: $LIBDIR"

for tool in cjpeg djpeg jpegtran rdjpgcom wrjpgcom tjbench jpegexiforient exifautotran; do
  require_exec "$BINDIR/$tool"
done
require_exec "$TJEXAMPLE_BIN"

for page in cjpeg.1 djpeg.1 jpegtran.1 rdjpgcom.1 wrjpgcom.1 tjbench.1 jpegexiforient.1 exifautotran.1; do
  require_file "$MANDIR/$page"
done

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

export PATH="$BINDIR${PATH:+:$PATH}"
export LD_LIBRARY_PATH="$LIBDIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

cp "$ROOT/original/testimages/testorig.ppm" "$TMPDIR/testorig.ppm"
seed_exif_orientation "$ROOT/original/testimages/testorig.jpg" "$TMPDIR/testorig.jpg"
cp "$ROOT/original/testimages/vgl_6548_0026a.bmp" "$TMPDIR/tjexample_input.bmp"

"$BINDIR/cjpeg" -outfile "$TMPDIR/base.jpg" "$TMPDIR/testorig.ppm" >/dev/null 2>&1
printf 'phase-8 progs smoke comment\n' >"$TMPDIR/comment.txt"
"$BINDIR/wrjpgcom" -replace -cfile "$TMPDIR/comment.txt" "$TMPDIR/base.jpg" >"$TMPDIR/commented.jpg"
comment_output="$("$BINDIR/rdjpgcom" "$TMPDIR/commented.jpg")"
case "$comment_output" in
  *"phase-8 progs smoke comment"*)
    ;;
  *)
    die "rdjpgcom did not round-trip the staged comment"
    ;;
esac

"$BINDIR/jpegexiforient" -6 "$TMPDIR/testorig.jpg" >/dev/null
[[ "$("$BINDIR/jpegexiforient" -n "$TMPDIR/testorig.jpg")" == "6" ]] \
  || die "jpegexiforient did not persist orientation 6"
"$BINDIR/exifautotran" "$TMPDIR/testorig.jpg" >/dev/null 2>&1
[[ "$("$BINDIR/jpegexiforient" -n "$TMPDIR/testorig.jpg")" == "1" ]] \
  || die "exifautotran did not normalize the Exif orientation"
"$BINDIR/djpeg" -ppm -outfile "$TMPDIR/final.ppm" "$TMPDIR/testorig.jpg" >/dev/null 2>&1
[[ -s "$TMPDIR/final.ppm" ]] || die "final.ppm was not produced"

"$BINDIR/cjpeg" -quality 95 -dct fast -sample 2x2 \
  -outfile "$TMPDIR/ref_tjexample.jpg" "$TMPDIR/tjexample_input.bmp" >/dev/null 2>&1
"$BINDIR/djpeg" -rgb -bmp -outfile "$TMPDIR/ref_tjexample.bmp" \
  "$TMPDIR/ref_tjexample.jpg" >/dev/null 2>&1
"$BINDIR/jpegtran" -crop 70x60+16+16 -rotate 90 -trim \
  -outfile "$TMPDIR/ref_tjexample_rot90.jpg" "$TMPDIR/ref_tjexample.jpg" >/dev/null 2>&1

"$TJEXAMPLE_BIN" "$TMPDIR/tjexample_input.bmp" "$TMPDIR/tjexample.jpg" \
  -q 95 -subsamp 420 -fastdct >/dev/null 2>&1
cmp -s "$TMPDIR/ref_tjexample.jpg" "$TMPDIR/tjexample.jpg" \
  || die "Rust tjexample JPEG output differed from staged cjpeg reference"

"$TJEXAMPLE_BIN" "$TMPDIR/ref_tjexample.jpg" "$TMPDIR/tjexample.bmp" >/dev/null 2>&1
compare_bmp_payload "$TMPDIR/ref_tjexample.bmp" "$TMPDIR/tjexample.bmp" \
  || die "Rust tjexample BMP output differed from staged djpeg reference"

"$TJEXAMPLE_BIN" "$TMPDIR/ref_tjexample.jpg" "$TMPDIR/tjexample_rot90.jpg" \
  -rot90 -crop 70x60+16+16 >/dev/null 2>&1
cmp -s "$TMPDIR/ref_tjexample_rot90.jpg" "$TMPDIR/tjexample_rot90.jpg" \
  || die "Rust tjexample transform output differed from staged jpegtran reference"

(
  cd "$TMPDIR"
  "$BINDIR/tjbench" "$TMPDIR/testorig.ppm" 95 \
    -rgb -quiet -benchtime 0.01 -warmup 0 >/dev/null 2>&1
)

printf 'staged progs smoke passed\n'
