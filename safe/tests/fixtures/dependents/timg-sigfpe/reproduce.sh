#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../../../../../ && pwd)"
REPORT_DIR="$ROOT/safe/target/dependent-regressions/timg-sigfpe"
SUMMARY="$REPORT_DIR/summary.json"

mkdir -p "$ROOT/safe/target/dependent-regressions"
rm -rf "$REPORT_DIR"

set +e
"$ROOT/test-original.sh" --checks all --only timg --report-dir "$REPORT_DIR"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
  printf 'expected ./test-original.sh --checks all --only timg to fail\n' >&2
  exit 1
fi

[[ -f "$SUMMARY" ]] || {
  printf 'missing summary: %s\n' "$SUMMARY" >&2
  exit 1
}

jq -e '
  [.compile[] | select(.status == "fail") | .source_package] == ["timg"]
  and
  [.runtime[] | select(.status == "fail") | .name] == ["timg"]
' "$SUMMARY" >/dev/null

for log_path in \
  "$REPORT_DIR/compile/timg-source/row.log" \
  "$REPORT_DIR/runtime/timg-runtime/row.log"
do
  [[ -f "$log_path" ]] || {
    printf 'missing log: %s\n' "$log_path" >&2
    exit 1
  }

  grep -E 'SIGFPE|Arithmetic Exception|signal 8' "$log_path" >/dev/null || {
    printf 'missing SIGFPE marker in %s\n' "$log_path" >&2
    cat "$log_path" >&2
    exit 1
  }
done
