#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
CASES_ROOT="$ROOT/safe/tests/fixtures/dependents"
SUMMARY_PATH="$ROOT/safe/target/dependent-matrix/summary.json"
MODE="reproduce"

usage() {
  cat <<'EOF'
usage: run-dependent-regressions.sh [--mode reproduce] [--summary <path>]

Run the committed dependent-regression reproducers that correspond to the
failing rows captured in a dependent matrix summary. The default summary path is
safe/target/dependent-matrix/summary.json.
EOF
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

while (($#)); do
  case "$1" in
    --mode)
      MODE="${2:?missing value for --mode}"
      shift 2
      ;;
    --summary)
      SUMMARY_PATH="${2:?missing value for --summary}"
      shift 2
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

[[ "$MODE" == "reproduce" ]] || die "unsupported mode: $MODE"
command -v jq >/dev/null 2>&1 || die "jq is required"
[[ -d "$CASES_ROOT" ]] || die "missing case metadata root: $CASES_ROOT"
[[ -f "$SUMMARY_PATH" ]] || die "missing dependent matrix summary: $SUMMARY_PATH"

mapfile -t CASE_FILES < <(find "$CASES_ROOT" -type f -name case.json | sort)
((${#CASE_FILES[@]} > 0)) || die "no dependent regression cases found under $CASES_ROOT"

declare -A FAIL_ROWS=()
declare -A COVERED_ROWS=()
declare -A SELECTED_CASES=()

while IFS= read -r row_key; do
  [[ -n "$row_key" ]] || continue
  FAIL_ROWS["$row_key"]=1
done < <(
  jq -r '
    (.compile[] | select(.status == "fail") | "compile:" + .source_package),
    (.runtime[] | select(.status == "fail") | "runtime:" + .name)
  ' "$SUMMARY_PATH"
)

((${#FAIL_ROWS[@]} > 0)) || die "summary does not contain any failing dependent rows"

for case_file in "${CASE_FILES[@]}"; do
  case_id="$(jq -r '.id' "$case_file")"
  [[ -n "$case_id" && "$case_id" != "null" ]] || die "case metadata is missing .id: $case_file"

  matched=0
  while IFS= read -r row_key; do
    [[ -n "$row_key" ]] || continue
    if [[ -n "${FAIL_ROWS[$row_key]:-}" ]]; then
      COVERED_ROWS["$row_key"]="$case_id"
      matched=1
    fi
  done < <(
    jq -r '
      .rows[]
      | if .kind == "compile" then
          "compile:" + .source_package
        elif .kind == "runtime" then
          "runtime:" + .name
        else
          empty
        end
    ' "$case_file"
  )

  if [[ "$matched" -eq 1 ]]; then
    SELECTED_CASES["$case_file"]=1
  fi
done

missing_rows=()
for row_key in "${!FAIL_ROWS[@]}"; do
  if [[ -z "${COVERED_ROWS[$row_key]:-}" ]]; then
    missing_rows+=("$row_key")
  fi
done

if ((${#missing_rows[@]} > 0)); then
  printf 'error: missing reproducer coverage for failing rows:\n' >&2
  printf '  %s\n' "${missing_rows[@]}" >&2
  exit 1
fi

for case_file in "${CASE_FILES[@]}"; do
  [[ -n "${SELECTED_CASES[$case_file]:-}" ]] || continue

  case_dir="$(cd -- "$(dirname -- "$case_file")" && pwd)"
  case_id="$(jq -r '.id' "$case_file")"
  runner_type="$(jq -r '.runner.type' "$case_file")"

  printf '\n==> %s\n' "$case_id"

  case "$runner_type" in
    cargo-test)
      test_name="$(jq -r '.runner.test' "$case_file")"
      [[ -n "$test_name" && "$test_name" != "null" ]] || die "missing cargo test selector for $case_id"
      cargo test --manifest-path "$ROOT/safe/Cargo.toml" --test dependent_regressions -- "$test_name"
      ;;
    script)
      script_rel="$(jq -r '.runner.script' "$case_file")"
      [[ -n "$script_rel" && "$script_rel" != "null" ]] || die "missing script runner path for $case_id"
      bash "$case_dir/$script_rel"
      ;;
    *)
      die "unsupported runner type for $case_id: $runner_type"
      ;;
  esac
done
