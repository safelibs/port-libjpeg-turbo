#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
JAVA_ROOT="$SAFE_ROOT/java"
TEST_ROOT="$SAFE_ROOT/target/java-tests"
WORK_ROOT="$TEST_ROOT/work"
CLASS_ROOT="$TEST_ROOT/classes"
SOURCE_ROOT="$TEST_ROOT/source"
USR_ROOT="$SAFE_ROOT/stage/usr"
CUSTOM_USR_ROOT=0

usage() {
  cat <<'EOF'
usage: run-java-tests.sh [--usr-root <path>]

Run the committed Java compatibility suite against a staged or extracted
package-installed /usr tree.  The default target is safe/stage/usr.
EOF
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
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

render_template() {
  local template="$1"
  local output="$2"
  shift 2

  python3 - "$template" "$output" "$@" <<'PY'
import pathlib
import sys

template_path = pathlib.Path(sys.argv[1])
output_path = pathlib.Path(sys.argv[2])
content = template_path.read_text(encoding="utf-8")

for arg in sys.argv[3:]:
    key, value = arg.split("=", 1)
    content = content.replace(key, value)

output_path.parent.mkdir(parents=True, exist_ok=True)
output_path.write_text(content, encoding="utf-8")
PY
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || die "missing required file: $path"
}

require_command() {
  local name="$1"
  command -v "$name" >/dev/null 2>&1 || die "missing required command: $name"
}

have_java_compiler() {
  command -v javac >/dev/null 2>&1 || \
    java --module jdk.compiler/com.sun.tools.javac.Main -version >/dev/null 2>&1
}

run_javac() {
  if command -v javac >/dev/null 2>&1; then
    javac "$@"
  else
    java --module jdk.compiler/com.sun.tools.javac.Main "$@"
  fi
}

require_command java
require_command python3
have_java_compiler || die "missing Java compiler support (javac or jdk.compiler module)"

while (($#)); do
  case "$1" in
    --usr-root)
      USR_ROOT="${2:?missing value for --usr-root}"
      CUSTOM_USR_ROOT=1
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      die "unknown option: $1"
      ;;
  esac
done

MULTIARCH="$(multiarch)"
STAGE_LIBDIR="$USR_ROOT/lib/$MULTIARCH"
STAGE_BINDIR="$USR_ROOT/bin"
STAGE_JAR="$USR_ROOT/share/java/turbojpeg.jar"
JAVA_BIN="$(command -v java)"

if [[ "$CUSTOM_USR_ROOT" -eq 0 && ( ! -f "$STAGE_JAR" || ! -f "$STAGE_LIBDIR/libturbojpeg.so.0" ) ]]; then
  bash "$SAFE_ROOT/scripts/stage-install.sh" --with-java=1
fi

require_file "$STAGE_JAR"
require_file "$STAGE_LIBDIR/libturbojpeg.so.0"

rm -rf "$TEST_ROOT"
mkdir -p "$WORK_ROOT/java" "$CLASS_ROOT" "$SOURCE_ROOT"
cp -a "$JAVA_ROOT/." "$SOURCE_ROOT/"

require_file "$SOURCE_ROOT/TJUnitTest.java"
require_file "$SOURCE_ROOT/tjbenchtest.java.in"
require_file "$SOURCE_ROOT/tjexampletest.java.in"

render_template \
  "$SOURCE_ROOT/org/libjpegturbo/turbojpeg/TJLoader-unix.java.in" \
  "$SOURCE_ROOT/org/libjpegturbo/turbojpeg/TJLoader.java" \
  "@CMAKE_INSTALL_FULL_LIBDIR@=$STAGE_LIBDIR" \
  "@CMAKE_INSTALL_DEFAULT_PREFIX@=$USR_ROOT"

ln -sf "$STAGE_JAR" "$WORK_ROOT/java/turbojpeg.jar"
for tool in cjpeg djpeg jpegtran; do
  ln -sf "$STAGE_BINDIR/$tool" "$WORK_ROOT/$tool"
done
for lib in libturbojpeg.so libturbojpeg.so.0 libturbojpeg.so.0.2.0; do
  if [[ -e "$STAGE_LIBDIR/$lib" ]]; then
    ln -sf "$STAGE_LIBDIR/$lib" "$WORK_ROOT/$lib"
  fi
done
if [[ ! -e "$WORK_ROOT/libturbojpeg.so" && -e "$WORK_ROOT/libturbojpeg.so.0" ]]; then
  ln -sf libturbojpeg.so.0 "$WORK_ROOT/libturbojpeg.so"
fi

render_template \
  "$SOURCE_ROOT/tjbenchtest.java.in" \
  "$WORK_ROOT/tjbenchtest.java" \
  "@CMAKE_CURRENT_SOURCE_DIR@=$ROOT/original" \
  "@CMAKE_CURRENT_BINARY_DIR@=$WORK_ROOT" \
  "@Java_JAVA_EXECUTABLE@=$JAVA_BIN"
render_template \
  "$SOURCE_ROOT/tjexampletest.java.in" \
  "$WORK_ROOT/tjexampletest.java" \
  "@CMAKE_CURRENT_SOURCE_DIR@=$ROOT/original" \
  "@CMAKE_CURRENT_BINARY_DIR@=$WORK_ROOT" \
  "@Java_JAVA_EXECUTABLE@=$JAVA_BIN"
chmod +x "$WORK_ROOT/tjbenchtest.java" "$WORK_ROOT/tjexampletest.java"

export LD_LIBRARY_PATH="$STAGE_LIBDIR${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

run_javac -encoding UTF-8 -cp "$STAGE_JAR" -d "$CLASS_ROOT" "$SOURCE_ROOT/TJUnitTest.java"

run_tjunittest() {
  java -cp "$CLASS_ROOT:$STAGE_JAR" -Djava.library.path="$WORK_ROOT" TJUnitTest "$@"
}

pushd "$WORK_ROOT" >/dev/null

run_tjunittest
run_tjunittest -yuv
run_tjunittest -yuv -noyuvpad
run_tjunittest -bi
run_tjunittest -bi -yuv
run_tjunittest -bi -yuv -noyuvpad

bash "$WORK_ROOT/tjbenchtest.java"
bash "$WORK_ROOT/tjbenchtest.java" -yuv
bash "$WORK_ROOT/tjbenchtest.java" -progressive
bash "$WORK_ROOT/tjbenchtest.java" -progressive -yuv
bash "$WORK_ROOT/tjexampletest.java"

popd >/dev/null
