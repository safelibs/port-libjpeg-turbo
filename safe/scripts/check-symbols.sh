#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

usage() {
  cat <<'EOF'
usage: check-symbols.sh [--skip-regex <extended-regex>] <debian-symbols-file> <shared-library>

Validates that the selected shared library exports the Debian symbol surface
described by a .symbols file. The parser understands:
  - the package header line
  - versioned symbol tokens such as symbol@VERSION
  - optional minimum-version fields after the symbol token
  - architecture qualifiers such as (arch=amd64 arm64)

--skip-regex filters symbol names before validation. Earlier phases use this to
defer the TurboJPEG JNI exports with:
  --skip-regex '^Java_org_libjpegturbo_turbojpeg_'
EOF
}

if (($# == 0)); then
  usage >&2
  exit 1
fi

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  usage
  exit 0
fi

skip_args=()
while (($#)); do
  case "$1" in
    --skip-regex)
      (($# >= 2)) || die "missing value for --skip-regex"
      skip_args=("$1" "$2")
      shift 2
      ;;
    --skip-regex=*)
      skip_args=("--skip-regex" "${1#*=}")
      shift
      ;;
    --)
      shift
      break
      ;;
    -*)
      die "unknown option: $1"
      ;;
    *)
      break
      ;;
  esac
done

(($# == 2)) || die "expected <debian-symbols-file> and <shared-library>"
[[ -f "$1" ]] || die "missing symbols file: $1"
[[ -e "$2" ]] || die "missing shared library: $2"

exec python3 "$ROOT/safe/scripts/debian_symbols.py" check "${skip_args[@]}" "$1" "$2"
