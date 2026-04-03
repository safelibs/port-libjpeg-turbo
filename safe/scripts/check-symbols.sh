#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SKIP_REGEX=""

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

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

while (($#)); do
  case "$1" in
    --skip-regex)
      SKIP_REGEX="${2:?missing value for --skip-regex}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    --*)
      printf 'unknown option: %s\n' "$1" >&2
      usage >&2
      exit 1
      ;;
    *)
      break
      ;;
  esac
done

[[ $# -eq 2 ]] || {
  usage >&2
  exit 1
}

SYMBOLS_FILE="$1"
LIBRARY="$2"

[[ -f "$SYMBOLS_FILE" ]] || die "missing symbols file: $SYMBOLS_FILE"
[[ -f "$LIBRARY" || -L "$LIBRARY" ]] || die "missing library: $LIBRARY"

HOST_ARCH="${DEB_HOST_ARCH:-$(dpkg --print-architecture 2>/dev/null || uname -m)}"
declare -A EXPORTED=()
declare -A MANIFEST_TOKENS=()

while IFS= read -r token; do
  [[ -n "$token" ]] && EXPORTED["$token"]=1
done < <(
  objdump -T "$LIBRARY" |
    awk '
      /^[0-9a-fA-F]/ {
        section=$4
        version=$(NF - 1)
        symbol=$NF
        if (section != "*UND*" &&
            version ~ /^[A-Za-z0-9_.-]+$/ &&
            symbol ~ /^[A-Za-z0-9_.$@-]+$/) {
          print symbol "@" version
        }
      }
    '
)

qualifier_matches() {
  local qualifier="$1"
  local arch_list arch

  [[ -z "$qualifier" ]] && return 0
  case "$qualifier" in
    arch=*)
      arch_list="${qualifier#arch=}"
      for arch in $arch_list; do
        [[ "$arch" == "$HOST_ARCH" ]] && return 0
      done
      return 1
      ;;
    *)
      return 0
      ;;
  esac
}

manifest_entries=0

while IFS= read -r raw_line; do
  line="${raw_line//$'\r'/}"
  [[ -z "${line//[[:space:]]/}" ]] && continue
  [[ "$line" =~ ^[[:space:]]*# ]] && continue
  [[ "$line" =~ ^[[:space:]]*\* ]] && continue

  if [[ "$line" =~ ^[^[:space:]] ]]; then
    continue
  fi

  line="${line#"${line%%[![:space:]]*}"}"
  qualifier=""

  if [[ "$line" == \(* ]]; then
    qualifier="${line%%)*}"
    qualifier="${qualifier#(}"
    line="${line#*)}"
  fi

  token="${line%%[[:space:]]*}"
  [[ -z "$token" ]] && continue
  [[ "$token" != *@* ]] && continue
  qualifier_matches "$qualifier" || continue

  symbol_name="${token%@*}"
  if [[ -n "$SKIP_REGEX" && "$symbol_name" =~ $SKIP_REGEX ]]; then
    continue
  fi

  MANIFEST_TOKENS["$token"]=1
  manifest_entries=$((manifest_entries + 1))
done <"$SYMBOLS_FILE"

unexpected=()
matched=0

for token in "${!EXPORTED[@]}"; do
  symbol_name="${token%@*}"
  if [[ -n "$SKIP_REGEX" && "$symbol_name" =~ $SKIP_REGEX ]]; then
    continue
  fi

  if [[ -n "${MANIFEST_TOKENS[$token]:-}" ]]; then
    matched=$((matched + 1))
    continue
  fi

  unexpected+=("$token")
done

if ((${#unexpected[@]})); then
  printf 'found %d exported symbol(s) in %s that are not declared in %s:\n' "${#unexpected[@]}" "$LIBRARY" "$SYMBOLS_FILE" >&2
  printf '  %s\n' "${unexpected[@]}" >&2
  exit 1
fi

if ((matched == 0)); then
  die "no exported symbols from $LIBRARY matched $SYMBOLS_FILE"
fi

printf 'validated %d exported symbol(s) from %s against %d manifest entries in %s\n' "$matched" "$LIBRARY" "$manifest_entries" "$SYMBOLS_FILE"
