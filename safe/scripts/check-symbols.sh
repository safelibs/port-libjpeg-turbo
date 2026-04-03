#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"

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

exec python3 "$ROOT/safe/scripts/debian_symbols.py" check "$@"
