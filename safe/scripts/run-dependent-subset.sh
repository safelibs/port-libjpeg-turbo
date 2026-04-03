#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
CHECKS="all"
declare -a ONLY_FILTERS=()
IMAGE_TAG="${LIBJPEG_TURBO_SAFE_TEST_IMAGE:-libjpeg-turbo-safe-test:ubuntu24.04}"

usage() {
  cat <<'EOF'
usage: run-dependent-subset.sh [--checks runtime|compile|all] [--only <runtime-package-or-source-package>]...

Stages the current safe/ bootstrap inside an Ubuntu 24.04 container or
temporary prefix, then prepares the selected direct dependent subsets declared
in dependents.json for later runtime and compile checks.

--checks defaults to all.
--only may be repeated. Each value matches either:
  - runtime_dependents[].name
  - build_time_dependents[].source_package
EOF
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

while (($#)); do
  case "$1" in
    --checks)
      CHECKS="${2:?missing value for --checks}"
      shift 2
      ;;
    --only)
      ONLY_FILTERS+=("${2:?missing value for --only}")
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

case "$CHECKS" in
  runtime|compile|all)
    ;;
  *)
    die "unsupported checks mode: $CHECKS"
    ;;
esac

command -v docker >/dev/null 2>&1 || die "docker is required"
[[ -f "$ROOT/dependents.json" ]] || die "missing dependents.json"
[[ -f "$SAFE_ROOT/scripts/stage-install.sh" ]] || die "missing bootstrap stage installer"

ONLY_SERIALIZED="$(printf '%s\n' "${ONLY_FILTERS[@]:-}" | paste -sd: -)"

docker build -t "$IMAGE_TAG" - <<'DOCKERFILE' >/dev/null
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN sed 's/^Types: deb$/Types: deb-src/' /etc/apt/sources.list.d/ubuntu.sources \
      > /etc/apt/sources.list.d/ubuntu-src.sources \
 && apt-get update \
 && apt-get install -y --no-install-recommends \
      build-essential \
      cargo \
      ca-certificates \
      cmake \
      curl \
      default-jdk \
      jq \
      nasm \
      pkg-config \
      python3 \
      rustc \
 && rm -rf /var/lib/apt/lists/*
DOCKERFILE

docker run --rm -i \
  -e "LIBJPEG_TURBO_TEST_CHECKS=$CHECKS" \
  -e "LIBJPEG_TURBO_TEST_ONLY=$ONLY_SERIALIZED" \
  -v "$ROOT":/work:ro \
  "$IMAGE_TAG" \
  bash -s <<'CONTAINER_SCRIPT'
set -euo pipefail

ROOT=/work
SAFE_ROOT=/work/safe
CHECKS="${LIBJPEG_TURBO_TEST_CHECKS:-all}"
ONLY_FILTERS="${LIBJPEG_TURBO_TEST_ONLY:-}"
TMP_ROOT=/tmp/libjpeg-safe-dependent-subset
WORK_ROOT="$TMP_ROOT/work"
STAGE_ROOT="$WORK_ROOT/safe/stage"

rm -rf "$TMP_ROOT"
mkdir -p "$WORK_ROOT"
cp -a "$ROOT/." "$WORK_ROOT/"

cd "$WORK_ROOT/safe"
cargo build --manifest-path Cargo.toml --workspace --release >/dev/null
bash scripts/stage-install.sh --stage-dir "$STAGE_ROOT" >/dev/null

select_runtime='.runtime_dependents[].name'
select_compile='.build_time_dependents[].source_package'
if [[ -n "$ONLY_FILTERS" ]]; then
  runtime_filter='select(. as $name | ($filters | index($name)))'
  compile_filter='select(. as $name | ($filters | index($name)))'
else
  runtime_filter='.'
  compile_filter='.'
fi

printf 'staged safe bootstrap under %s\n' "$STAGE_ROOT/usr"

case "$CHECKS" in
  runtime|all)
    printf 'runtime subset:\n'
    jq -r --arg filters "$ONLY_FILTERS" '
      ($filters | split(":") | map(select(length > 0))) as $filter_values
      | .runtime_dependents[]
      | select(($filter_values | length) == 0 or ($filter_values | index(.name)))
      | "  " + .name + " - " + .summary
    ' "$WORK_ROOT/dependents.json"
    ;;
esac

case "$CHECKS" in
  compile|all)
    printf 'compile subset:\n'
    jq -r --arg filters "$ONLY_FILTERS" '
      ($filters | split(":") | map(select(length > 0))) as $filter_values
      | .build_time_dependents[]
      | select(($filter_values | length) == 0 or ($filter_values | index(.source_package)))
      | "  " + .source_package + " - " + (.binary_examples | join(", "))
    ' "$WORK_ROOT/dependents.json"
    ;;
esac
CONTAINER_SCRIPT

