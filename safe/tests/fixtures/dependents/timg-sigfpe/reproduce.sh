#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../../../../../ && pwd)"

mkdir -p "$ROOT/safe/target/dependent-regressions"
REPORT_DIR="$(mktemp -d "$ROOT/safe/target/dependent-regressions/timg-sigfpe.XXXXXX")"
SUMMARY="$REPORT_DIR/summary.json"

"$ROOT/test-original.sh" --checks all --only timg --report-dir "$REPORT_DIR"

[[ -f "$SUMMARY" ]] || {
  printf 'missing summary: %s\n' "$SUMMARY" >&2
  exit 1
}

jq -e '
  all(.compile[]; if .source_package == "timg" then .status == "pass" else .status == "skipped" end)
  and
  all(.runtime[]; if .name == "timg" then .status == "pass" else .status == "skipped" end)
' "$SUMMARY" >/dev/null

for log_path in \
  "$REPORT_DIR/compile/timg-source/row.log" \
  "$REPORT_DIR/runtime/timg-runtime/row.log"
do
  [[ -f "$log_path" ]] || {
    printf 'missing log: %s\n' "$log_path" >&2
    exit 1
  }

  if grep -E 'SIGFPE|Arithmetic Exception|signal 8' "$log_path" >/dev/null; then
    printf 'unexpected SIGFPE marker in %s\n' "$log_path" >&2
    cat "$log_path" >&2
    exit 1
  fi
done
