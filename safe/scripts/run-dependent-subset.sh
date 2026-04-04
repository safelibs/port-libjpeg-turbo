#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/../.. && pwd)"
SAFE_ROOT="$ROOT/safe"
STAGE_INSTALL_SCRIPT="$SAFE_ROOT/scripts/stage-install.sh"
DEPENDENTS_MANIFEST="$ROOT/dependents.json"
CHECKS="all"
declare -a ONLY_FILTERS=()
IMAGE_TAG="${LIBJPEG_TURBO_SAFE_TEST_IMAGE:-libjpeg-turbo-safe-test:ubuntu24.04}"

usage() {
  cat <<'EOF'
usage: run-dependent-subset.sh [--checks runtime|compile|all] [--only <runtime-package-or-source-package>]... [--image-tag <docker-tag>]

Stages the current safe/ bootstrap inside an Ubuntu 24.04 container or
temporary prefix, then runs the selected runtime-dependent smoke checks against
the staged safe libraries. Compile mode still prints the selected source subset
for follow-on build verification.

--checks defaults to all.
--only may be repeated. Each value matches either:
  - runtime_dependents[].name
  - build_time_dependents[].source_package
--image-tag overrides the Docker image name used for the isolated dependency
subset harness build.
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
    --image-tag)
      IMAGE_TAG="${2:?missing value for --image-tag}"
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
command -v jq >/dev/null 2>&1 || die "jq is required"
[[ -f "$DEPENDENTS_MANIFEST" ]] || die "missing dependents.json"
[[ -f "$STAGE_INSTALL_SCRIPT" ]] || die "missing bootstrap stage installer"

ONLY_SERIALIZED="$(printf '%s\n' "${ONLY_FILTERS[@]:-}" | paste -sd: -)"

docker build -t "$IMAGE_TAG" - <<'DOCKERFILE' >/dev/null
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN sed 's/^Types: deb$/Types: deb-src/' /etc/apt/sources.list.d/ubuntu.sources \
      > /etc/apt/sources.list.d/ubuntu-src.sources \
 && apt-get update \
 && apt-get install -y --no-install-recommends \
      build-essential \
      ca-certificates \
      cmake \
      curl \
      default-jdk \
      jq \
      nasm \
      pkg-config \
      python3 \
 && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --profile minimal --default-toolchain stable \
 && cargo --version \
 && rustc --version \
 && rm -rf /var/lib/apt/lists/*
DOCKERFILE

docker run --rm -i \
  -e "LIBJPEG_TURBO_TEST_CHECKS=$CHECKS" \
  -e "LIBJPEG_TURBO_TEST_ONLY=$ONLY_SERIALIZED" \
  -v "$ROOT":/work:ro \
  "$IMAGE_TAG" \
  bash -s <<'CONTAINER_SCRIPT'
set -euo pipefail

export LANG=C.UTF-8
export LC_ALL=C.UTF-8

ROOT=/work
SAFE_ROOT=/work/safe
CHECKS="${LIBJPEG_TURBO_TEST_CHECKS:-all}"
ONLY_FILTERS="${LIBJPEG_TURBO_TEST_ONLY:-}"
TMP_ROOT=/tmp/libjpeg-safe-dependent-subset
WORK_ROOT="$TMP_ROOT/work"
STAGE_ROOT="$WORK_ROOT/safe/stage"
TEST_ROOT="$TMP_ROOT/runtime-checks"
FIXTURE_DIR="$TMP_ROOT/fixtures"
DEPENDENT_SOURCE_ROOT="$TMP_ROOT/dependent-sources"
MULTIARCH="$(gcc -print-multiarch)"
APT_UPDATED=0

declare -A SOURCE_CACHE=()

rm -rf "$TMP_ROOT"
mkdir -p "$WORK_ROOT" "$TEST_ROOT" "$DEPENDENT_SOURCE_ROOT"
cp -a "$ROOT/." "$WORK_ROOT/"

log_step() {
  printf '\n==> %s\n' "$1"
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

require_nonempty_file() {
  local path="$1"

  [[ -s "$path" ]] || die "expected non-empty file: $path"
}

require_contains() {
  local path="$1"
  local needle="$2"

  if ! grep -F -- "$needle" "$path" >/dev/null 2>&1; then
    printf 'missing expected text in %s: %s\n' "$path" "$needle" >&2
    printf -- '--- %s ---\n' "$path" >&2
    cat "$path" >&2
    exit 1
  fi
}

reset_test_dir() {
  local name="$1"
  local dir="$TEST_ROOT/$name"

  rm -rf "$dir"
  mkdir -p "$dir"
  printf '%s\n' "$dir"
}

assert_uses_local_soname() {
  local target="$1"
  local soname="$2"
  local resolved

  resolved="$(ldd "$target" | awk -v soname="$soname" '$1 == soname { print $3; exit }')"
  [[ -n "$resolved" ]] || die "ldd did not report $soname for $target"

  case "$resolved" in
    "$STAGE_ROOT"/usr/lib/*)
      ;;
    *)
      printf 'expected %s to resolve %s from %s, got %s\n' "$target" "$soname" "$STAGE_ROOT/usr/lib" "$resolved" >&2
      ldd "$target" >&2
      exit 1
      ;;
  esac
}

assert_any_file_under_uses_local_soname() {
  local root="$1"
  local name_pattern="$2"
  local soname="$3"
  local description="$4"
  local candidate resolved

  while IFS= read -r -d '' candidate; do
    resolved="$(
      ldd "$candidate" 2>/dev/null \
        | awk -v soname="$soname" '$1 == soname { print $3; exit }' \
        || true
    )"
    case "$resolved" in
      "$STAGE_ROOT"/usr/lib/*)
        return 0
        ;;
    esac
  done < <(find "$root" -type f -name "$name_pattern" -print0 2>/dev/null)

  die "expected $description to resolve $soname from $STAGE_ROOT/usr/lib"
}

assert_package_uses_local_soname() {
  local package="$1"
  local soname="$2"
  local description="$3"
  local candidate resolved

  while IFS= read -r candidate; do
    [[ -f "$candidate" ]] || continue
    resolved="$(
      ldd "$candidate" 2>/dev/null \
        | awk -v soname="$soname" '$1 == soname { print $3; exit }' \
        || true
    )"
    case "$resolved" in
      "$STAGE_ROOT"/usr/lib/*)
        return 0
        ;;
    esac
  done < <(dpkg -L "$package" 2>/dev/null)

  die "expected $description to resolve $soname from $STAGE_ROOT/usr/lib"
}

ensure_apt_updated() {
  if [[ "$APT_UPDATED" -eq 0 ]]; then
    apt-get update >/tmp/libjpeg-safe-apt-update.log 2>&1 || {
      cat /tmp/libjpeg-safe-apt-update.log >&2
      exit 1
    }
    APT_UPDATED=1
  fi
}

fetch_ubuntu_source() {
  local source_package="$1"
  local checkout_root source_dir

  if [[ -n "${SOURCE_CACHE[$source_package]:-}" ]]; then
    printf '%s\n' "${SOURCE_CACHE[$source_package]}"
    return 0
  fi

  ensure_apt_updated
  checkout_root="$DEPENDENT_SOURCE_ROOT/$source_package"
  rm -rf "$checkout_root"
  mkdir -p "$checkout_root"

  (
    cd "$checkout_root"
    apt-get source "$source_package" >"$checkout_root/source.log" 2>&1
  ) || {
    cat "$checkout_root/source.log" >&2
    exit 1
  }

  source_dir="$(find "$checkout_root" -mindepth 1 -maxdepth 1 -type d -name "${source_package}-*" -print -quit)"
  [[ -n "$source_dir" ]] || die "unable to locate extracted source for $source_package"

  SOURCE_CACHE[$source_package]="$source_dir"
  printf '%s\n' "$source_dir"
}

configure_stage_runtime() {
  printf '%s\n%s\n' "$STAGE_ROOT/usr/lib" "$STAGE_ROOT/usr/lib/$MULTIARCH" > /etc/ld.so.conf.d/zz-libjpeg-safe-stage.conf
  ldconfig

  export LD_LIBRARY_PATH="$STAGE_ROOT/usr/lib:$STAGE_ROOT/usr/lib/$MULTIARCH${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  export LIBRARY_PATH="$STAGE_ROOT/usr/lib:$STAGE_ROOT/usr/lib/$MULTIARCH${LIBRARY_PATH:+:$LIBRARY_PATH}"
  export PKG_CONFIG_PATH="$STAGE_ROOT/usr/lib/pkgconfig:$STAGE_ROOT/usr/lib/$MULTIARCH/pkgconfig:$STAGE_ROOT/usr/share/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
}

list_selected_runtime() {
  jq -r --arg filters "$ONLY_FILTERS" '
    ($filters | split(":") | map(select(length > 0))) as $filter_values
    | .runtime_dependents[]
    | . as $dep
    | select(($filter_values | length) == 0 or ($filter_values | index($dep.name)))
    | $dep.name
  ' "$WORK_ROOT/dependents.json"
}

validate_filters() {
  local filter
  IFS=: read -r -a filters <<< "$ONLY_FILTERS"
  for filter in "${filters[@]}"; do
    [[ -n "$filter" ]] || continue
    jq -e --arg filter "$filter" '
      any(.runtime_dependents[]; .name == $filter) or
      any(.build_time_dependents[]; .source_package == $filter)
    ' "$WORK_ROOT/dependents.json" >/dev/null || die "--only did not match any runtime package or source package: $filter"
  done
}

install_runtime_packages() {
  local -a selected_runtime=()
  local -a packages=(
    python3
  )
  local runtime_name

  mapfile -t selected_runtime < <(list_selected_runtime)
  for runtime_name in "${selected_runtime[@]}"; do
    case "$runtime_name" in
      dcm2niix)
        packages+=(dcm2niix)
        ;;
      eog)
        packages+=(eog dbus-x11 python3-pil xauth xdotool xvfb)
        ;;
      krita)
        packages+=(krita xauth xvfb)
        ;;
      libcamera-tools)
        packages+=(libcamera-dev libcamera-tools libsdl2-dev)
        ;;
      openjdk-17-jre-headless)
        packages+=(openjdk-17-jdk-headless openjdk-17-jre-headless)
        ;;
      python3-pil)
        packages+=(python3-pil)
        ;;
      timg)
        packages+=(timg)
        ;;
      tracker-extract)
        packages+=(tracker tracker-extract)
        ;;
      xpra)
        packages+=(xpra)
        ;;
      *)
        die "runtime smoke is not implemented for $runtime_name in this harness"
        ;;
    esac
  done

  if ((${#selected_runtime[@]} == 0)); then
    return 0
  fi

  ensure_apt_updated
  mapfile -t packages < <(printf '%s\n' "${packages[@]}" | awk 'NF && !seen[$0]++')
  apt-get install -y --no-install-recommends "${packages[@]}" >/tmp/libjpeg-safe-runtime-install.log 2>&1 || {
    cat /tmp/libjpeg-safe-runtime-install.log >&2
    exit 1
  }
}

prepare_runtime_fixtures() {
  local -a selected_runtime=()
  local need_quadrant_fixtures=0

  rm -rf "$FIXTURE_DIR"
  mkdir -p "$FIXTURE_DIR"
  cp "$WORK_ROOT/original/testimages/testorig.jpg" "$FIXTURE_DIR/input.jpg"

  mapfile -t selected_runtime < <(list_selected_runtime)
  for runtime_name in "${selected_runtime[@]}"; do
    case "$runtime_name" in
      eog|libcamera-tools)
        need_quadrant_fixtures=1
        ;;
    esac
  done

  if [[ "$need_quadrant_fixtures" -eq 0 ]]; then
    return 0
  fi

  FIXTURE_DIR="$FIXTURE_DIR" python3 - <<'PY'
import os

fixture_dir = os.environ["FIXTURE_DIR"]


def make_quadrant_ppm(path, size):
    width, height = size
    mid_x = width // 2
    mid_y = height // 2
    header = f"P6\n{width} {height}\n255\n".encode("ascii")

    with open(path, "wb") as handle:
        handle.write(header)
        for y in range(height):
            row = bytearray()
            for x in range(width):
                if x < mid_x and y < mid_y:
                    pixel = (250, 20, 20)
                elif x >= mid_x and y < mid_y:
                    pixel = (20, 250, 20)
                elif x < mid_x and y >= mid_y:
                    pixel = (20, 20, 250)
                else:
                    pixel = (250, 250, 20)
                row.extend(pixel)
            handle.write(row)


make_quadrant_ppm(os.path.join(fixture_dir, "eog-pattern.ppm"), (1024, 768))
make_quadrant_ppm(os.path.join(fixture_dir, "mjpeg-pattern.ppm"), (128, 128))
PY

  "$STAGE_ROOT/usr/bin/cjpeg" -quality 95 -sample 1x1 "$FIXTURE_DIR/eog-pattern.ppm" > "$FIXTURE_DIR/eog-pattern.jpg"
  "$STAGE_ROOT/usr/bin/cjpeg" -quality 95 -sample 1x1 "$FIXTURE_DIR/mjpeg-pattern.ppm" > "$FIXTURE_DIR/mjpeg-pattern.jpg"
  rm -f "$FIXTURE_DIR/eog-pattern.ppm" "$FIXTURE_DIR/mjpeg-pattern.ppm"
}

check_eog_runtime() {
  local dir status

  dir="$(reset_test_dir eog-runtime)"
  export XDG_RUNTIME_DIR="$dir/xdg"
  mkdir -p "$XDG_RUNTIME_DIR"

  set +e
  timeout 60 dbus-run-session -- xvfb-run -a --server-args="-screen 0 1024x768x24" \
    bash -s -- "$FIXTURE_DIR/eog-pattern.jpg" "$dir/eog.log" "$dir/render-probe.log" "$dir/window-id.txt" "$STAGE_ROOT/usr/lib/$MULTIARCH/libjpeg.so.8" <<'EOF'
set -euo pipefail
image="$1"
log_path="$2"
probe_path="$3"
window_path="$4"
stage_libjpeg="$5"

LD_DEBUG=libs eog --fullscreen "$image" >"$log_path" 2>&1 &
pid=$!

cleanup() {
  kill "$pid" || true
  wait "$pid" || true
}

trap cleanup EXIT

for _ in $(seq 1 20); do
  if xdotool search --onlyvisible --class Eog >"$window_path" 2>/dev/null; then
    break
  fi
  sleep 1
done

[[ -s "$window_path" ]] || {
  cat "$log_path" >&2
  exit 1
}

python3 - "$probe_path" <<'PY'
import os
import sys
import time
from PIL import ImageGrab

probe_path = sys.argv[1]
expected = [
    ("top-left", (256, 192), (250, 20, 20)),
    ("top-right", (768, 192), (20, 250, 20)),
    ("bottom-left", (256, 576), (20, 20, 250)),
    ("bottom-right", (768, 576), (250, 250, 20)),
]

with open(probe_path, "w", encoding="utf-8") as probe_log:
    for attempt in range(20):
        time.sleep(1)
        try:
            image = ImageGrab.grab(xdisplay=os.environ["DISPLAY"])
        except Exception as exc:
            print(f"attempt {attempt}: ImageGrab failed: {type(exc).__name__}: {exc}", file=probe_log)
            probe_log.flush()
            continue

        ok = True
        samples = []
        for label, coord, want in expected:
            got = image.getpixel(coord)
            samples.append(f"{label}={got}")
            if any(abs(got_channel - want_channel) > 90 for got_channel, want_channel in zip(got, want)):
                ok = False

        print(f"attempt {attempt}: {' '.join(samples)}", file=probe_log)
        probe_log.flush()
        if ok:
            sys.exit(0)

sys.exit(1)
PY
EOF
  status=$?
  set -e

  if [[ "$status" -ne 0 ]]; then
    cat "$dir/render-probe.log" >&2 2>/dev/null || true
    cat "$dir/eog.log" >&2 2>/dev/null || true
    exit 1
  fi

  require_nonempty_file "$dir/window-id.txt"
  require_nonempty_file "$dir/render-probe.log"
  require_contains "$dir/eog.log" "$STAGE_ROOT/usr/lib/$MULTIARCH/libjpeg.so.8"
}

check_dcm2niix_runtime() {
  local dir status

  dir="$(reset_test_dir dcm2niix-runtime)"
  assert_package_uses_local_soname dcm2niix libturbojpeg.so.0 'dcm2niix package files'

  set +e
  dcm2niix -h >"$dir/help.log" 2>&1
  status=$?
  set -e

  [[ "$status" -eq 0 ]] || {
    cat "$dir/help.log" >&2
    die "dcm2niix -h failed with status $status"
  }

  require_contains "$dir/help.log" 'dcm2niix'
}

check_libcamera_tools_runtime() {
  local dir status source_dir

  dir="$(reset_test_dir libcamera-tools-runtime)"
  assert_uses_local_soname /usr/bin/cam libjpeg.so.8

  set +e
  cam -l >"$dir/list.log" 2>&1
  status=$?
  set -e

  [[ "$status" -eq 0 ]] || {
    cat "$dir/list.log" >&2
    die "cam -l failed with status $status"
  }

  require_contains "$dir/list.log" 'Available cameras:'
  source_dir="$(fetch_ubuntu_source libcamera)"

  cat >"$dir/libcamera_mjpg_probe.cpp" <<'EOF'
#include <SDL2/SDL.h>
#include <libcamera/base/span.h>

#include <fstream>
#include <iostream>
#include <iterator>
#include <vector>

#include "sdl_texture_mjpg.h"

namespace {

unsigned char pixel_at(SDL_Surface *surface, int x, int y, int channel)
{
  auto *row = static_cast<unsigned char *>(surface->pixels) + y * surface->pitch;
  return row[x * 3 + channel];
}

bool within_tolerance(int got, int want)
{
  return got >= want - 90 && got <= want + 90;
}

}  // namespace

int main(int argc, char **argv)
{
  if (argc != 2) {
    std::cerr << "usage: libcamera-mjpg-probe <jpeg>\n";
    return 1;
  }

  std::ifstream input(argv[1], std::ios::binary);
  std::vector<unsigned char> bytes((std::istreambuf_iterator<char>(input)),
                                   std::istreambuf_iterator<char>());
  if (bytes.empty()) {
    std::cerr << "empty jpeg input\n";
    return 1;
  }

  if (SDL_Init(SDL_INIT_VIDEO) != 0) {
    std::cerr << "SDL_Init failed: " << SDL_GetError() << "\n";
    return 1;
  }

  SDL_Surface *surface = SDL_CreateRGBSurfaceWithFormat(0, 128, 128, 24, SDL_PIXELFORMAT_RGB24);
  if (!surface) {
    std::cerr << "SDL_CreateRGBSurfaceWithFormat failed: " << SDL_GetError() << "\n";
    SDL_Quit();
    return 1;
  }

  SDL_Renderer *renderer = SDL_CreateSoftwareRenderer(surface);
  if (!renderer) {
    std::cerr << "SDL_CreateSoftwareRenderer failed: " << SDL_GetError() << "\n";
    SDL_FreeSurface(surface);
    SDL_Quit();
    return 1;
  }

  SDL_Rect rect{0, 0, 128, 128};
  SDLTextureMJPG texture(rect);
  if (texture.create(renderer) != 0) {
    SDL_DestroyRenderer(renderer);
    SDL_FreeSurface(surface);
    SDL_Quit();
    return 1;
  }

  std::vector<libcamera::Span<const uint8_t>> planes;
  planes.emplace_back(bytes.data(), bytes.size());
  texture.update(planes);

  SDL_RenderClear(renderer);
  SDL_RenderCopy(renderer, texture.get(), nullptr, nullptr);
  SDL_RenderPresent(renderer);

  struct Sample {
    const char *label;
    int x;
    int y;
    int r;
    int g;
    int b;
  };

  const Sample samples[] = {
      { "top-left", 32, 32, 250, 20, 20 },
      { "top-right", 96, 32, 20, 250, 20 },
      { "bottom-left", 32, 96, 20, 20, 250 },
      { "bottom-right", 96, 96, 250, 250, 20 },
  };

  for (const Sample &sample : samples) {
    int r = pixel_at(surface, sample.x, sample.y, 0);
    int g = pixel_at(surface, sample.x, sample.y, 1);
    int b = pixel_at(surface, sample.x, sample.y, 2);
    std::cout << sample.label << "=" << r << "," << g << "," << b << "\n";
    if (!within_tolerance(r, sample.r) ||
        !within_tolerance(g, sample.g) ||
        !within_tolerance(b, sample.b)) {
      SDL_DestroyRenderer(renderer);
      SDL_FreeSurface(surface);
      SDL_Quit();
      return 1;
    }
  }

  SDL_DestroyRenderer(renderer);
  SDL_FreeSurface(surface);
  SDL_Quit();
  return 0;
}
EOF

  c++ -std=c++17 \
    -I"$source_dir/src/apps/cam" \
    -I"$source_dir/src/apps/common" \
    -I"$STAGE_ROOT/usr/include" \
    -I"$STAGE_ROOT/usr/include/$MULTIARCH" \
    $(pkg-config --cflags libcamera-base sdl2) \
    "$dir/libcamera_mjpg_probe.cpp" \
    "$source_dir/src/apps/cam/sdl_texture.cpp" \
    "$source_dir/src/apps/cam/sdl_texture_mjpg.cpp" \
    -L"$STAGE_ROOT/usr/lib/$MULTIARCH" \
    -Wl,-rpath,"$STAGE_ROOT/usr/lib/$MULTIARCH" \
    $(pkg-config --libs sdl2) \
    -ljpeg \
    -o "$dir/libcamera-mjpg-probe" \
    >"$dir/build.log" 2>&1 || {
      cat "$dir/build.log" >&2
      exit 1
    }

  assert_uses_local_soname "$dir/libcamera-mjpg-probe" libjpeg.so.8

  SDL_VIDEODRIVER=dummy "$dir/libcamera-mjpg-probe" "$FIXTURE_DIR/mjpeg-pattern.jpg" >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }

  require_contains "$dir/run.log" 'top-left='
  require_contains "$dir/run.log" 'top-right='
  require_contains "$dir/run.log" 'bottom-left='
  require_contains "$dir/run.log" 'bottom-right='
}

check_openjdk_runtime() {
  local dir

  dir="$(reset_test_dir openjdk-runtime)"
  assert_uses_local_soname /usr/lib/jvm/java-17-openjdk-amd64/lib/libjavajpeg.so libjpeg.so.8

  cat >"$dir/JpegSmoke.java" <<'EOF'
import javax.imageio.ImageIO;
import java.awt.image.BufferedImage;
import java.io.File;

public class JpegSmoke {
  public static void main(String[] args) throws Exception {
    BufferedImage image = ImageIO.read(new File("INPUT_JPG"));
    if (image == null) {
      throw new RuntimeException("ImageIO.read returned null");
    }
    if (!ImageIO.write(image, "jpeg", new File("OUTPUT_JPG"))) {
      throw new RuntimeException("ImageIO.write failed");
    }
    System.out.println(image.getWidth() + "x" + image.getHeight());
  }
}
EOF
  sed -i "s|INPUT_JPG|$FIXTURE_DIR/input.jpg|g; s|OUTPUT_JPG|$dir/java-out.jpg|g" "$dir/JpegSmoke.java"

  javac "$dir/JpegSmoke.java" >"$dir/compile.log" 2>&1 || {
    cat "$dir/compile.log" >&2
    exit 1
  }

  java -cp "$dir" JpegSmoke >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }

  require_nonempty_file "$dir/java-out.jpg"
}

check_pillow_runtime() {
  local dir

  dir="$(reset_test_dir pillow-runtime)"
  assert_any_file_under_uses_local_soname /usr/lib/python3/dist-packages/PIL '_imaging*.so' libjpeg.so.8 'Pillow imaging extension'

  FIXTURE_DIR="$FIXTURE_DIR" OUTPUT_JPG="$dir/pillow-out.jpg" python3 - <<'PY'
import os
from PIL import Image

input_jpg = os.path.join(os.environ["FIXTURE_DIR"], "input.jpg")
output_jpg = os.environ["OUTPUT_JPG"]

image = Image.open(input_jpg)
image.transpose(Image.Transpose.FLIP_LEFT_RIGHT).save(output_jpg, format="JPEG")
print(image.size)
PY

  require_nonempty_file "$dir/pillow-out.jpg"
}

check_krita_runtime() {
  local dir status

  dir="$(reset_test_dir krita-runtime)"
  assert_package_uses_local_soname krita libjpeg.so.8 'krita package files'
  assert_package_uses_local_soname krita libturbojpeg.so.0 'krita package files'

  set +e
  timeout 60 env QT_QPA_PLATFORM=offscreen krita --help >"$dir/help.log" 2>&1
  status=$?
  if [[ "$status" -ne 0 ]]; then
    timeout 60 xvfb-run -a krita --help >"$dir/help.log" 2>&1
    status=$?
  fi
  set -e

  [[ "$status" -eq 0 ]] || {
    cat "$dir/help.log" >&2
    die "krita --help failed with status $status"
  }

  require_contains "$dir/help.log" 'krita'
}

check_timg_runtime() {
  local dir status

  dir="$(reset_test_dir timg-runtime)"
  assert_package_uses_local_soname timg libturbojpeg.so.0 'timg package files'

  set +e
  TERM=xterm-256color PAGER=cat timg --help >"$dir/help.log" 2>&1
  status=$?
  set -e

  [[ "$status" -eq 0 ]] || {
    cat "$dir/help.log" >&2
    die "timg --help failed with status $status"
  }

  require_contains "$dir/help.log" 'timg'
}

check_tracker_extract_runtime() {
  local dir extractor

  dir="$(reset_test_dir tracker-extract-runtime)"
  extractor=/usr/lib/x86_64-linux-gnu/tracker-miners-3.0/extract-modules/libextract-jpeg.so
  [[ -f "$extractor" ]] || die "unable to locate Tracker JPEG extractor module"
  assert_uses_local_soname "$extractor" libjpeg.so.8

  tracker3 extract "$FIXTURE_DIR/input.jpg" >"$dir/extract.log" 2>&1 || {
    cat "$dir/extract.log" >&2
    exit 1
  }

  require_contains "$dir/extract.log" 'nfo:width 227'
  require_contains "$dir/extract.log" 'nfo:height 149'
}

check_xpra_runtime() {
  local dir

  dir="$(reset_test_dir xpra-runtime)"
  assert_package_uses_local_soname xpra libturbojpeg.so.0 'xpra package files'

  python3 - <<'PY' >"$dir/version.log" 2>&1
import xpra
print(xpra.__version__)
PY

  require_nonempty_file "$dir/version.log"
}

run_runtime_checks() {
  local -a selected_runtime=()
  local runtime_name

  mapfile -t selected_runtime < <(list_selected_runtime)
  if ((${#selected_runtime[@]} == 0)); then
    return 0
  fi

  install_runtime_packages
  prepare_runtime_fixtures

  for runtime_name in "${selected_runtime[@]}"; do
    case "$runtime_name" in
      dcm2niix)
        log_step 'dcm2niix runtime smoke'
        check_dcm2niix_runtime
        ;;
      eog)
        log_step 'eog runtime smoke'
        check_eog_runtime
        ;;
      krita)
        log_step 'krita runtime smoke'
        check_krita_runtime
        ;;
      libcamera-tools)
        log_step 'libcamera-tools runtime smoke'
        check_libcamera_tools_runtime
        ;;
      openjdk-17-jre-headless)
        log_step 'openjdk-17-jre-headless runtime smoke'
        check_openjdk_runtime
        ;;
      python3-pil)
        log_step 'python3-pil runtime smoke'
        check_pillow_runtime
        ;;
      timg)
        log_step 'timg runtime smoke'
        check_timg_runtime
        ;;
      tracker-extract)
        log_step 'tracker-extract runtime smoke'
        check_tracker_extract_runtime
        ;;
      xpra)
        log_step 'xpra runtime smoke'
        check_xpra_runtime
        ;;
      *)
        die "runtime smoke is not implemented for $runtime_name in this harness"
        ;;
    esac
  done
}

cd "$WORK_ROOT/safe"
cargo build --manifest-path Cargo.toml --workspace --release >/dev/null
DEB_HOST_MULTIARCH="$MULTIARCH" LIBJPEG_TURBO_STAGE_ROOT="$STAGE_ROOT" \
  bash scripts/stage-install.sh --clean --stage-dir "$STAGE_ROOT" >/dev/null
configure_stage_runtime
validate_filters

printf 'staged safe bootstrap under %s\n' "$STAGE_ROOT/usr"

case "$CHECKS" in
  runtime|all)
    printf 'runtime subset:\n'
    jq -r --arg filters "$ONLY_FILTERS" '
      ($filters | split(":") | map(select(length > 0))) as $filter_values
      | .runtime_dependents[]
      | . as $dep
      | select(($filter_values | length) == 0 or ($filter_values | index($dep.name)))
      | "  " + $dep.name + " - " + $dep.summary
    ' "$WORK_ROOT/dependents.json"
    run_runtime_checks
    ;;
esac

case "$CHECKS" in
  compile|all)
    printf 'compile subset:\n'
    jq -r --arg filters "$ONLY_FILTERS" '
      ($filters | split(":") | map(select(length > 0))) as $filter_values
      | .build_time_dependents[]
      | . as $dep
      | select(($filter_values | length) == 0 or ($filter_values | index($dep.source_package)))
      | "  " + $dep.source_package + " - " + ($dep.binary_examples | join(", "))
    ' "$WORK_ROOT/dependents.json"
    ;;
esac
CONTAINER_SCRIPT
