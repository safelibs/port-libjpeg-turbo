#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
IMAGE_TAG="${LIBJPEG_TURBO_ORIGINAL_TEST_IMAGE:-libjpeg-turbo-original-test:ubuntu24.04}"
CHECKS="all"
ONLY=""

usage() {
  cat <<'EOF'
usage: test-original.sh [--checks runtime|compile|all] [--only <runtime-package-or-source-package>]

Builds the safe Debian packages from ./safe inside an Ubuntu 24.04 Docker
container, installs them into the container, and then exercises the direct
dependent software listed in dependents.json.

--checks defaults to all.
runtime runs the runtime-dependent package smoke tests.
compile runs the build-time matrix by building package-native targets from each
source package in build_time_dependents[] and checking that the resulting
artifacts resolve libjpeg/libturbojpeg from the installed safe Debian packages.
all runs compile checks first and runtime checks second.

--only filters by exact runtime package name from runtime_dependents[].name or
by exact source package name from build_time_dependents[].source_package.
EOF
}

while (($#)); do
  case "$1" in
    --checks)
      CHECKS="${2:?missing value for --checks}"
      shift 2
      ;;
    --only)
      ONLY="${2:?missing value for --only}"
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
    printf 'unsupported checks mode: %s\n' "$CHECKS" >&2
    usage >&2
    exit 1
    ;;
esac

command -v docker >/dev/null 2>&1 || {
  echo "docker is required to run $0" >&2
  exit 1
}

[[ -d "$ROOT/original" ]] || {
  echo "missing original source tree" >&2
  exit 1
}

[[ -f "$ROOT/dependents.json" ]] || {
  echo "missing dependents.json" >&2
  exit 1
}

docker build -t "$IMAGE_TAG" - <<'DOCKERFILE'
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_HOME=/opt/cargo
ENV RUSTUP_HOME=/opt/rustup
ENV PATH=/opt/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN sed 's/^Types: deb$/Types: deb-src/' /etc/apt/sources.list.d/ubuntu.sources \
      > /etc/apt/sources.list.d/ubuntu-src.sources \
 && apt-get update \
 && apt-get install -y --no-install-recommends \
      build-essential \
      ca-certificates \
      cargo \
      cmake \
      curl \
      debhelper \
      dbus-x11 \
      dcm2niix \
      dpkg-dev \
      eog \
      file \
      gimp \
      git \
      gphoto2 \
      help2man \
      jq \
      krita \
      libcamera-tools \
      libcamera-dev \
      libreoffice-core \
      libreoffice-draw \
      libopencv-dev \
      libsdl2-dev \
      libvips-dev \
      libvips-tools \
      libwebkit2gtk-4.1-dev \
      meson \
      nasm \
      ninja-build \
      openjdk-17-jdk \
      pkg-config \
      python3 \
      python3-pil \
      python3-pydicom \
      timg \
      tracker \
      tracker-extract \
      xauth \
      xdotool \
      xpra \
      xvfb \
 && rm -rf /var/lib/apt/lists/* \
 && curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain 1.85.1 \
 && chmod -R a+rX /opt/cargo /opt/rustup
DOCKERFILE

docker run --rm -i \
  -e "LIBJPEG_TURBO_TEST_CHECKS=$CHECKS" \
  -e "LIBJPEG_TURBO_TEST_ONLY=$ONLY" \
  -v "$ROOT":/work:ro \
  "$IMAGE_TAG" \
  bash -s <<'CONTAINER_SCRIPT'
set -euo pipefail

export LANG=C.UTF-8
export LC_ALL=C.UTF-8

ROOT=/work
CHECKS="${LIBJPEG_TURBO_TEST_CHECKS:-all}"
ONLY_FILTER="${LIBJPEG_TURBO_TEST_ONLY:-}"
MULTIARCH="$(gcc -print-multiarch)"
HOME=/tmp/libjpeg-home
FIXTURE_DIR=/tmp/libjpeg-fixtures
TEST_ROOT=/tmp/libjpeg-dependent-tests
SAFE_SRC_COPY=/tmp/libjpeg-safe-src
DEPENDENT_SOURCE_ROOT=/tmp/libjpeg-dependent-sources
APT_UPDATED=0

mkdir -p "$HOME" "$TEST_ROOT" "$DEPENDENT_SOURCE_ROOT"

declare -A COMPLETED_CHECKS=()
declare -A SOURCE_CACHE=()
declare -A BUILD_DEPS_READY=()

log_step() {
  printf '\n==> %s\n' "$1"
}

die() {
  echo "error: $*" >&2
  exit 1
}

require_nonempty_file() {
  local path="$1"

  if [[ ! -s "$path" ]]; then
    printf 'expected non-empty file: %s\n' "$path" >&2
    exit 1
  fi
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
    /usr/lib/*|/usr/lib/"$MULTIARCH"/*|/lib/*|/lib/"$MULTIARCH"/*)
      ;;
    *)
      printf 'expected %s to resolve %s from a system libdir, got %s\n' "$target" "$soname" "$resolved" >&2
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
    resolved="$(ldd "$candidate" 2>/dev/null | awk -v soname="$soname" '$1 == soname { print $3; exit }')"

    case "$resolved" in
      /usr/lib/*|/usr/lib/"$MULTIARCH"/*|/lib/*|/lib/"$MULTIARCH"/*)
        return 0
        ;;
    esac
  done < <(find "$root" -type f -name "$name_pattern" -print0 2>/dev/null)

  die "expected $description to resolve $soname from a system libdir"
}

find_first_file() {
  local root="$1"
  local name_pattern="$2"

  find "$root" -type f -name "$name_pattern" -print -quit 2>/dev/null
}

find_first_elf_shared_object() {
  local root="$1"
  shift
  local name_pattern candidate

  for name_pattern in "$@"; do
    while IFS= read -r -d '' candidate; do
      if file -b "$candidate" | grep -F 'ELF ' >/dev/null 2>&1; then
        printf '%s\n' "$candidate"
        return 0
      fi
    done < <(find "$root" -type f -name "$name_pattern" -print0 2>/dev/null)
  done

  return 1
}

run_check() {
  local key="$1"
  local label="$2"
  local fn="$3"

  if [[ -n "${COMPLETED_CHECKS[$key]:-}" ]]; then
    log_step "$label (already covered)"
    return 0
  fi

  log_step "$label"
  "$fn"
  COMPLETED_CHECKS[$key]=1
}

run_selected_runtime() {
  local runtime_name="$1"
  local key="$2"
  local label="$3"
  local fn="$4"

  if [[ -n "$ONLY_FILTER" && "$ONLY_FILTER" != "$runtime_name" ]]; then
    return 0
  fi

  run_check "$key" "$label" "$fn"
}

run_selected_compile() {
  local source_package="$1"
  local key="$2"
  local label="$3"
  local fn="$4"

  if [[ -n "$ONLY_FILTER" && "$ONLY_FILTER" != "$source_package" ]]; then
    return 0
  fi

  run_check "$key" "$label" "$fn"
}

ensure_apt_updated() {
  if [[ "$APT_UPDATED" -eq 0 ]]; then
    apt-get update >/tmp/libjpeg-apt-update.log 2>&1 || {
      cat /tmp/libjpeg-apt-update.log >&2
      exit 1
    }
    APT_UPDATED=1
  fi
}

ensure_package_build_deps() {
  local source_package="$1"

  if [[ -n "${BUILD_DEPS_READY[$source_package]:-}" ]]; then
    return 0
  fi

  ensure_apt_updated
  apt-get build-dep -y "$source_package" >"/tmp/${source_package}-build-deps.log" 2>&1 || {
    cat "/tmp/${source_package}-build-deps.log" >&2
    exit 1
  }

  BUILD_DEPS_READY[$source_package]=1
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

assert_dependents_inventory() {
  local expected_runtime expected_build actual_runtime actual_build

  expected_runtime=$'dcm2niix\neog\ngimp\ngphoto2\nkrita\nlibcamera-tools\nlibopencv-imgcodecs406t64\nlibreoffice-core\nlibvips42t64\nlibwebkit2gtk-4.1-0\nopenjdk-17-jre-headless\npython3-pil\ntimg\ntracker-extract\nxpra'
  actual_runtime="$(jq -r '.runtime_dependents[].name' "$ROOT/dependents.json")"

  if [[ "$actual_runtime" != "$expected_runtime" ]]; then
    echo "runtime_dependents in dependents.json do not match the expected matrix" >&2
    diff -u <(printf '%s\n' "$expected_runtime") <(printf '%s\n' "$actual_runtime") >&2 || true
    exit 1
  fi

  expected_build=$'dcm2niix\nkrita\nlibreoffice\nopencv\ntimg\nvips\nwebkit2gtk\nxpra'
  actual_build="$(jq -r '.build_time_dependents[].source_package' "$ROOT/dependents.json")"

  if [[ "$actual_build" != "$expected_build" ]]; then
    echo "build_time_dependents in dependents.json do not match the expected matrix" >&2
    diff -u <(printf '%s\n' "$expected_build") <(printf '%s\n' "$actual_build") >&2 || true
    exit 1
  fi

  if [[ -n "$ONLY_FILTER" ]]; then
    jq -e --arg filter "$ONLY_FILTER" '
      (.runtime_dependents[] | select(.name == $filter)) ,
      (.build_time_dependents[] | select(.source_package == $filter))
    ' "$ROOT/dependents.json" >/dev/null || die "--only did not match any runtime package or source package: $ONLY_FILTER"
  fi
}

build_safe_packages() {
  local -a debs

  rm -rf "$SAFE_SRC_COPY"
  mkdir -p "$SAFE_SRC_COPY"
  cp -a "$ROOT/safe" "$SAFE_SRC_COPY/safe"
  cp -a "$ROOT/original" "$SAFE_SRC_COPY/original"

  (
    cd "$SAFE_SRC_COPY/safe"
    dpkg-buildpackage -us -uc -b >/tmp/libjpeg-safe-build.log 2>&1
  ) || {
    cat /tmp/libjpeg-safe-build.log >&2
    exit 1
  }

  mapfile -t debs < <(find "$SAFE_SRC_COPY" -maxdepth 1 -type f -name '*.deb' | sort)
  ((${#debs[@]} > 0)) || die "dpkg-buildpackage did not produce any .deb files"

  dpkg -i "${debs[@]}" >/tmp/libjpeg-safe-install.log 2>&1 || {
    cat /tmp/libjpeg-safe-install.log >&2
    exit 1
  }

  export LIBJPEG_TURBO_BACKEND_LIB="$SAFE_SRC_COPY/safe/target/upstream-bootstrap/libturbojpeg.so.0"
  [[ -f "$LIBJPEG_TURBO_BACKEND_LIB" ]] || die "missing TurboJPEG backend library: $LIBJPEG_TURBO_BACKEND_LIB"

  ldconfig

  assert_uses_local_soname /usr/bin/dcm2niix libturbojpeg.so.0
  assert_uses_local_soname /usr/lib/jvm/java-17-openjdk-amd64/lib/libjavajpeg.so libjpeg.so.8
}

prepare_fixtures() {
  rm -rf "$FIXTURE_DIR"
  mkdir -p "$FIXTURE_DIR/dcm" "$FIXTURE_DIR/camera/store_00010001/DCIM/100CANON"

  cp "$ROOT/original/testimages/testorig.jpg" "$FIXTURE_DIR/input.jpg"
  cp "$ROOT/original/testimages/testorig.jpg" "$FIXTURE_DIR/camera/store_00010001/DCIM/100CANON/IMG_0001.JPG"

  FIXTURE_DIR="$FIXTURE_DIR" python3 - <<'PY'
import io
import os
from PIL import Image
from PIL import ImageDraw
from pydicom.dataset import FileDataset, FileMetaDataset
from pydicom.encaps import encapsulate
from pydicom.uid import JPEGBaseline8Bit, SecondaryCaptureImageStorage, generate_uid

fixture_dir = os.environ["FIXTURE_DIR"]
input_jpg = os.path.join(fixture_dir, "input.jpg")
input_png = os.path.join(fixture_dir, "input.png")
dicom_dir = os.path.join(fixture_dir, "dcm")
dicom_path = os.path.join(dicom_dir, "input.dcm")


def make_quadrant_jpeg(path, size):
    width, height = size
    image = Image.new("RGB", size)
    draw = ImageDraw.Draw(image)
    mid_x = width // 2
    mid_y = height // 2
    draw.rectangle((0, 0, mid_x - 1, mid_y - 1), fill=(250, 20, 20))
    draw.rectangle((mid_x, 0, width - 1, mid_y - 1), fill=(20, 250, 20))
    draw.rectangle((0, mid_y, mid_x - 1, height - 1), fill=(20, 20, 250))
    draw.rectangle((mid_x, mid_y, width - 1, height - 1), fill=(250, 250, 20))
    image.save(path, format="JPEG", quality=95, subsampling=0)


rgb = Image.open(input_jpg).convert("RGB")
rgb.save(input_png)
make_quadrant_jpeg(os.path.join(fixture_dir, "eog-pattern.jpg"), (1024, 768))
make_quadrant_jpeg(os.path.join(fixture_dir, "mjpeg-pattern.jpg"), (128, 128))

jpeg_buffer = io.BytesIO()
rgb.save(jpeg_buffer, format="JPEG")
jpeg_bytes = jpeg_buffer.getvalue()

meta = FileMetaDataset()
meta.MediaStorageSOPClassUID = SecondaryCaptureImageStorage
meta.MediaStorageSOPInstanceUID = generate_uid()
meta.TransferSyntaxUID = JPEGBaseline8Bit
meta.ImplementationClassUID = generate_uid()

dataset = FileDataset(dicom_path, {}, file_meta=meta, preamble=b"\0" * 128)
dataset.is_little_endian = True
dataset.is_implicit_VR = False
dataset.PatientName = "Test^JPEG"
dataset.PatientID = "12345"
dataset.StudyInstanceUID = generate_uid()
dataset.SeriesInstanceUID = generate_uid()
dataset.SOPClassUID = meta.MediaStorageSOPClassUID
dataset.SOPInstanceUID = meta.MediaStorageSOPInstanceUID
dataset.Modality = "OT"
dataset.Manufacturer = "PYDICOM"
dataset.StudyDate = "20260402"
dataset.SeriesNumber = "1"
dataset.InstanceNumber = "1"
dataset.Rows = rgb.height
dataset.Columns = rgb.width
dataset.SamplesPerPixel = 3
dataset.PhotometricInterpretation = "YBR_FULL_422"
dataset.PlanarConfiguration = 0
dataset.BitsAllocated = 8
dataset.BitsStored = 8
dataset.HighBit = 7
dataset.PixelRepresentation = 0
dataset.LossyImageCompression = "01"
dataset.LossyImageCompressionMethod = "ISO_10918_1"
dataset.PixelData = encapsulate([jpeg_bytes])
dataset["PixelData"].is_undefined_length = True
dataset.save_as(dicom_path, write_like_original=False)
PY

  cat >"$FIXTURE_DIR/webkit.html" <<EOF
<!doctype html>
<html>
<body>
<img id="target" src="file://$FIXTURE_DIR/input.jpg" onload="document.title='loaded-'+this.naturalWidth+'x'+this.naturalHeight" onerror="document.title='error'">
</body>
</html>
EOF
}

check_dcm2niix_source_build() {
  local source_dir build_dir binary dir nifti_file json_file

  ensure_package_build_deps dcm2niix
  source_dir="$(fetch_ubuntu_source dcm2niix)"
  build_dir="$TEST_ROOT/build-dcm2niix"
  rm -rf "$build_dir"

  cmake -S "$source_dir/console" -B "$build_dir" \
    -DCMAKE_BUILD_TYPE=Release \
    -DUSE_TURBOJPEG=ON \
    -DUSE_OPENJPEG=ON \
    -DBATCH_VERSION=ON \
    >/tmp/dcm2niix-source-config.log 2>&1 || {
      cat /tmp/dcm2niix-source-config.log >&2
      exit 1
    }

  cmake --build "$build_dir" -j"$(nproc)" >/tmp/dcm2niix-source-build.log 2>&1 || {
    cat /tmp/dcm2niix-source-build.log >&2
    exit 1
  }

  binary="$(find "$build_dir" -maxdepth 2 -type f -name dcm2niix -print -quit)"
  [[ -n "$binary" ]] || die "unable to locate source-built dcm2niix binary"
  assert_uses_local_soname "$binary" libturbojpeg.so.0

  dir="$(reset_test_dir dcm2niix-source)"
  mkdir -p "$dir/out"
  "$binary" -f '%p_%s' -o "$dir/out" "$FIXTURE_DIR/dcm" >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }

  nifti_file="$(find "$dir/out" -type f \( -name '*.nii' -o -name '*.nii.gz' \) -print -quit)"
  json_file="$(find "$dir/out" -type f -name '*.json' -print -quit)"
  [[ -n "$nifti_file" ]] || die "source-built dcm2niix did not produce NIfTI output"
  [[ -n "$json_file" ]] || die "source-built dcm2niix did not produce sidecar JSON output"
  require_nonempty_file "$nifti_file"
  require_nonempty_file "$json_file"
}

check_timg_source_build() {
  local source_dir build_dir binary dir

  ensure_package_build_deps timg
  source_dir="$(fetch_ubuntu_source timg)"
  build_dir="$TEST_ROOT/build-timg"
  rm -rf "$build_dir"

  cmake -S "$source_dir" -B "$build_dir" -DCMAKE_BUILD_TYPE=Release >/tmp/timg-source-config.log 2>&1 || {
    cat /tmp/timg-source-config.log >&2
    exit 1
  }

  cmake --build "$build_dir" -j"$(nproc)" >/tmp/timg-source-build.log 2>&1 || {
    cat /tmp/timg-source-build.log >&2
    exit 1
  }

  binary="$(find "$build_dir" -type f -name timg -print -quit)"
  [[ -n "$binary" ]] || die "unable to locate source-built timg binary"
  assert_uses_local_soname "$binary" libturbojpeg.so.0

  dir="$(reset_test_dir timg-source)"
  TERM=xterm "$binary" -g 40x15 "$FIXTURE_DIR/input.jpg" >"$dir/render.txt" 2>&1 || {
    cat "$dir/render.txt" >&2
    exit 1
  }
  require_nonempty_file "$dir/render.txt"
}

check_krita_source_build() {
  local source_dir build_dir import_module export_module

  ensure_package_build_deps krita
  source_dir="$(fetch_ubuntu_source krita)"
  build_dir="$TEST_ROOT/build-krita"
  rm -rf "$build_dir"

  cmake -S "$source_dir" -B "$build_dir" -GNinja \
    -DCMAKE_BUILD_TYPE=Release \
    -DFOUNDATION_BUILD=OFF \
    -DBUILD_TESTING=OFF \
    -DENABLE_UPDATERS=OFF \
    -DKRITA_ENABLE_PCH=OFF \
    >/tmp/krita-source-config.log 2>&1 || {
      cat /tmp/krita-source-config.log >&2
      exit 1
    }

  cmake --build "$build_dir" -j"$(nproc)" --target kritajpegimport kritajpegexport >/tmp/krita-source-build.log 2>&1 || {
    cat /tmp/krita-source-build.log >&2
    exit 1
  }

  import_module="$(find "$build_dir" -type f -name 'kritajpegimport.so' -print -quit)"
  export_module="$(find "$build_dir" -type f -name 'kritajpegexport.so' -print -quit)"
  [[ -n "$import_module" ]] || die "unable to locate source-built kritajpegimport module"
  [[ -n "$export_module" ]] || die "unable to locate source-built kritajpegexport module"

  assert_uses_local_soname "$import_module" libjpeg.so.8
  assert_uses_local_soname "$export_module" libjpeg.so.8
}

check_libreoffice_source_build() {
  local source_dir multiarch vcl_lib dir

  ensure_package_build_deps libreoffice
  source_dir="$(fetch_ubuntu_source libreoffice)"
  multiarch="$(gcc -print-multiarch)"
  dir="$(reset_test_dir libreoffice-source)"

  (
    cd "$source_dir"
    rm -f autogen.lastrun config.status
    export SAL_USE_VCLPLUGIN=headless

    ./autogen.sh \
      --disable-cups \
      --disable-dbus \
      --disable-dconf \
      --disable-epm \
      --disable-evolution2 \
      --disable-ext-nlpsolver \
      --disable-ext-wiki-publisher \
      --disable-firebird-sdbc \
      --disable-gio \
      --disable-gstreamer-1-0 \
      --disable-gtk3 \
      --disable-gui \
      --disable-kf5 \
      --disable-libcmis \
      --disable-lto \
      --disable-odk \
      --disable-online-update \
      --disable-poppler \
      --disable-postgresql-sdbc \
      --disable-report-builder \
      --disable-scripting-beanshell \
      --disable-scripting-javascript \
      --disable-sdremote \
      --disable-sdremote-bluetooth \
      --disable-skia \
      --enable-cairo-rgba \
      --enable-extension-integration \
      --enable-mergelibs \
      --enable-python=system \
      --enable-release-build \
      --with-external-dict-dir=/usr/share/hunspell \
      --with-external-hyph-dir=/usr/share/hyphen \
      --with-external-thes-dir=/usr/share/mythes \
      --without-fonts \
      --with-galleries=no \
      --with-lang=en-US \
      --with-linker-hash-style=both \
      --with-system-dicts \
      --with-system-jpeg \
      --with-theme=colibre \
      --without-branding \
      --without-help \
      --without-java \
      --without-junit \
      --without-package-format \
      --without-system-cairo \
      --without-system-jars \
      --without-system-libpng \
      --without-system-libxml \
      --without-system-openssl \
      --without-system-postgresql \
      >/tmp/libreoffice-source-config.log 2>&1 || {
        tail -n 200 /tmp/libreoffice-source-config.log >&2
        exit 1
      }

    grep -F 'checking which libjpeg to use... external' /tmp/libreoffice-source-config.log >/dev/null || {
      cat /tmp/libreoffice-source-config.log >&2
      exit 1
    }

    make -j"$(nproc)" CppunitTest_vcl_jpeg_read_write_test >/tmp/libreoffice-source-build.log 2>&1 || {
      tail -n 200 /tmp/libreoffice-source-build.log >&2
      exit 1
    }
  )

  vcl_lib="$(find_first_elf_shared_object "$source_dir" 'libvcllo.so.*' 'libmergedlo.so.*' 'libvcllo.so' 'libmergedlo.so')"
  [[ -n "$vcl_lib" ]] || die "unable to locate source-built LibreOffice VCL library"
  assert_uses_local_soname "$vcl_lib" libjpeg.so.8
}

check_opencv_source_build() {
  local source_dir build_dir lib dir

  ensure_package_build_deps opencv
  source_dir="$(fetch_ubuntu_source opencv)"
  build_dir="$TEST_ROOT/build-opencv"
  rm -rf "$build_dir"

  cmake -S "$source_dir" -B "$build_dir" -GNinja \
    -DCMAKE_BUILD_TYPE=Release \
    -DBUILD_LIST=core,imgproc,imgcodecs \
    -DBUILD_SHARED_LIBS=ON \
    -DBUILD_TESTS=OFF \
    -DBUILD_PERF_TESTS=OFF \
    -DBUILD_EXAMPLES=OFF \
    -DBUILD_opencv_apps=OFF \
    -DBUILD_opencv_java=OFF \
    -DBUILD_opencv_python3=OFF \
    -DBUILD_PROTOBUF=OFF \
    -DWITH_PROTOBUF=OFF \
    -DWITH_QUIRC=OFF \
    -DWITH_1394=OFF \
    -DWITH_VTK=OFF \
    -DWITH_JPEG=ON \
    -DBUILD_JPEG=OFF \
    -DWITH_PNG=OFF \
    -DWITH_TIFF=OFF \
    -DWITH_WEBP=OFF \
    -DWITH_OPENEXR=OFF \
    -DWITH_OPENJPEG=OFF \
    -DWITH_JASPER=OFF \
    -DWITH_GDAL=OFF \
    -DWITH_GTK=OFF \
    -DWITH_QT=OFF \
    -DWITH_OPENCL=OFF \
    -DWITH_IPP=OFF \
    -DWITH_TBB=OFF \
    -DWITH_ITT=OFF \
    -DWITH_ADE=OFF \
    -DWITH_GSTREAMER=OFF \
    -DWITH_V4L=OFF \
    -DWITH_GPHOTO2=OFF \
    -DWITH_FFMPEG=OFF \
    >/tmp/opencv-source-config.log 2>&1 || {
      cat /tmp/opencv-source-config.log >&2
      exit 1
    }

  cmake --build "$build_dir" -j"$(nproc)" --target opencv_imgcodecs >/tmp/opencv-source-build.log 2>&1 || {
    cat /tmp/opencv-source-build.log >&2
    exit 1
  }

  lib="$(find "$build_dir/lib" -type f -name 'libopencv_imgcodecs.so*' -print -quit)"
  [[ -n "$lib" ]] || die "unable to locate source-built libopencv_imgcodecs"
  assert_uses_local_soname "$lib" libjpeg.so.8

  dir="$(reset_test_dir opencv-source)"
  cat >"$dir/opencv_smoke.cpp" <<'EOF'
#include <opencv2/imgcodecs.hpp>
#include <iostream>

int main() {
  cv::Mat input = cv::imread("INPUT_JPG", cv::IMREAD_COLOR);
  if (input.empty()) {
    std::cerr << "imread failed\n";
    return 1;
  }
  if (!cv::imwrite("OUTPUT_JPG", input)) {
    std::cerr << "imwrite failed\n";
    return 1;
  }
  std::cout << input.cols << "x" << input.rows << "\n";
  return 0;
}
EOF
  sed -i "s|INPUT_JPG|$FIXTURE_DIR/input.jpg|g; s|OUTPUT_JPG|$dir/opencv-out.jpg|g" "$dir/opencv_smoke.cpp"

  c++ -std=c++17 "$dir/opencv_smoke.cpp" \
    -I"$source_dir/modules/core/include" \
    -I"$source_dir/modules/imgproc/include" \
    -I"$source_dir/modules/imgcodecs/include" \
    -I"$build_dir" \
    -L"$build_dir/lib" \
    -Wl,-rpath,"$build_dir/lib" \
    -lopencv_imgcodecs -lopencv_imgproc -lopencv_core \
    -ldl -lm -lpthread -lrt \
    -o "$dir/opencv-smoke" \
    >/tmp/opencv-source-consumer.log 2>&1 || {
      cat /tmp/opencv-source-consumer.log >&2
      exit 1
    }

  "$dir/opencv-smoke" >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }

  require_nonempty_file "$dir/opencv-out.jpg"
}

check_vips_source_build() {
  local source_dir build_dir lib dir

  ensure_package_build_deps vips
  source_dir="$(fetch_ubuntu_source vips)"
  build_dir="$TEST_ROOT/build-vips"
  rm -rf "$build_dir"

  meson setup "$build_dir" "$source_dir" \
    -Djpeg=enabled \
    -Djpeg-xl=disabled \
    -Dopenjpeg=disabled \
    -Ddeprecated=false \
    -Dexamples=false \
    -Dgtk_doc=false \
    -Ddoxygen=false \
    -Dintrospection=disabled \
    >/tmp/vips-source-config.log 2>&1 || {
      cat /tmp/vips-source-config.log >&2
      exit 1
    }

  ninja -C "$build_dir" tools/vips >/tmp/vips-source-build.log 2>&1 || {
    cat /tmp/vips-source-build.log >&2
    exit 1
  }

  lib="$(find "$build_dir/libvips" -maxdepth 1 -type f -name 'libvips.so*' ! -name '*.symbols' -print -quit)"
  [[ -n "$lib" ]] || die "unable to locate source-built libvips shared library"
  assert_uses_local_soname "$lib" libjpeg.so.8

  dir="$(reset_test_dir vips-source)"
  "$build_dir/tools/vips" copy "$FIXTURE_DIR/input.jpg" "$dir/roundtrip.jpg" >/tmp/vips-source-run.log 2>&1 || {
    cat /tmp/vips-source-run.log >&2
    exit 1
  }

  require_nonempty_file "$dir/roundtrip.jpg"
}

check_webkit_source_build() {
  local source_dir build_dir webkit_lib
  local -a cmake_args

  ensure_package_build_deps webkit2gtk
  source_dir="$(fetch_ubuntu_source webkit2gtk)"
  build_dir="$TEST_ROOT/build-webkit2gtk"
  rm -rf "$build_dir"

  cmake_args=(
    -GNinja
    -DPORT=GTK
    -DCMAKE_BUILD_TYPE=Release
    -DCMAKE_BUILD_WITH_INSTALL_RPATH=ON
    -DUSE_LIBBACKTRACE=OFF
    -DENABLE_MINIBROWSER=ON
    -DENABLE_DOCUMENTATION=OFF
    -DENABLE_INTROSPECTION=OFF
    -DENABLE_API_TESTS=OFF
    -DENABLE_LAYOUT_TESTS=OFF
    -DENABLE_BUBBLEWRAP_SANDBOX=OFF
    -DENABLE_GAMEPAD=OFF
    -DENABLE_MEMORY_SAMPLER=OFF
    -DENABLE_RESOURCE_USAGE=OFF
    -DENABLE_SPEECH_SYNTHESIS=OFF
    -DENABLE_WEBDRIVER=OFF
    -DUSE_GTK4=OFF
    -DUSE_JPEGXL=OFF
    -DUSE_GBM=OFF
    -DUSE_LIBDRM=OFF
    -DUSE_SOUP2=OFF
    -DUSE_SYSTEM_SYSPROF_CAPTURE=OFF
    -DUSE_SYSPROF_CAPTURE=OFF
  )

  if command -v clang >/dev/null 2>&1 && command -v clang++ >/dev/null 2>&1; then
    cmake_args+=(-DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++)
  fi

  cmake -S "$source_dir" -B "$build_dir" "${cmake_args[@]}" >/tmp/webkit-source-config.log 2>&1 || {
    cat /tmp/webkit-source-config.log >&2
    exit 1
  }

  cmake --build "$build_dir" -j"$(nproc)" --target WebKit >/tmp/webkit-source-build.log 2>&1 || {
    cat /tmp/webkit-source-build.log >&2
    exit 1
  }

  webkit_lib="$(find_first_elf_shared_object "$build_dir" 'libwebkit2gtk-4.1.so.*' 'libwebkit2gtk-4.1.so')"
  [[ -n "$webkit_lib" ]] || die "unable to locate source-built libwebkit2gtk shared library"
  assert_uses_local_soname "$webkit_lib" libjpeg.so.8
}

check_xpra_source_build() {
  local source_dir encoder dir

  ensure_package_build_deps xpra
  source_dir="$(fetch_ubuntu_source xpra)"
  dir="$(reset_test_dir xpra-source)"

  (
    cd "$source_dir"
    python3 setup.py build_ext --inplace \
      --with-verbose \
      --with-Xdummy \
      --without-Xdummy_wrapper \
      --with-html5 \
      --without-minify \
      --without-html5_gzip \
      --without-strict \
      >/tmp/xpra-source-build.log 2>&1
  ) || {
    tail -n 200 /tmp/xpra-source-build.log >&2
    exit 1
  }

  encoder="$(find "$source_dir/xpra/codecs/jpeg" -type f -name 'encoder*.so' -print -quit)"
  [[ -n "$encoder" ]] || die "unable to locate source-built Xpra JPEG encoder extension"
  assert_uses_local_soname "$encoder" libturbojpeg.so.0

  FIXTURE_DIR="$FIXTURE_DIR" PYTHONPATH="$source_dir" python3 - <<'PY' >"$dir/run.log" 2>&1
from PIL import Image
from xpra.codecs.image_wrapper import ImageWrapper
from xpra.codecs.jpeg import encoder, decoder
import os

fixture_dir = os.environ["FIXTURE_DIR"]
image = Image.open(os.path.join(fixture_dir, "input.jpg")).convert("RGBA")
raw = image.tobytes("raw", "RGBA")
wrapper = ImageWrapper(0, 0, image.width, image.height, raw, "RGBX", 24, image.width * 4)
encoding, compressed, options, width, height, _, _ = encoder.encode(wrapper, quality=80, speed=50, options={})
decoded = decoder.decompress_to_rgb("RGBX", compressed.data, width, height, {})

print(encoding)
print(len(compressed.data))
print(decoded.get_width(), decoded.get_height(), decoded.get_pixel_format())
PY

  require_contains "$dir/run.log" 'jpeg'
  require_contains "$dir/run.log" 'RGBX'
}

check_dcm2niix_runtime() {
  local dir nifti_file json_file

  dir="$(reset_test_dir dcm2niix-runtime)"
  mkdir -p "$dir/out"
  assert_uses_local_soname /usr/bin/dcm2niix libturbojpeg.so.0

  /usr/bin/dcm2niix -f '%p_%s' -o "$dir/out" "$FIXTURE_DIR/dcm" >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }

  nifti_file="$(find "$dir/out" -type f \( -name '*.nii' -o -name '*.nii.gz' \) -print -quit)"
  json_file="$(find "$dir/out" -type f -name '*.json' -print -quit)"
  [[ -n "$nifti_file" ]] || die "dcm2niix did not produce NIfTI output"
  [[ -n "$json_file" ]] || die "dcm2niix did not produce sidecar JSON output"
  require_nonempty_file "$nifti_file"
  require_nonempty_file "$json_file"
}

check_eog_runtime() {
  local dir

  dir="$(reset_test_dir eog-runtime)"
  export XDG_RUNTIME_DIR="$dir/xdg"
  mkdir -p "$XDG_RUNTIME_DIR"

  set +e
  timeout 60 dbus-run-session -- xvfb-run -a --server-args="-screen 0 1024x768x24" \
    bash -s -- "$FIXTURE_DIR/eog-pattern.jpg" "$dir/eog.log" "$dir/render-probe.log" "$dir/window-id.txt" <<'EOF'
set -euo pipefail
image="$1"
log_path="$2"
probe_path="$3"
window_path="$4"

eog --fullscreen "$image" >"$log_path" 2>&1 &
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
}

check_gimp_runtime() {
  local dir plugin

  dir="$(reset_test_dir gimp-runtime)"
  plugin="$(find /usr/lib -path '*gimp/2.0/plug-ins/file-jpeg/file-jpeg' -print -quit)"
  [[ -n "$plugin" ]] || die "unable to locate GIMP JPEG plugin"
  assert_uses_local_soname "$plugin" libjpeg.so.8

  timeout 120 gimp-console-2.10 -i -d -f \
    -b "(let* ((image (car (gimp-file-load RUN-NONINTERACTIVE \"$FIXTURE_DIR/input.jpg\" \"$FIXTURE_DIR/input.jpg\"))) (drawable (car (gimp-image-get-active-layer image)))) (gimp-file-save RUN-NONINTERACTIVE image drawable \"$dir/gimp-out.jpg\" \"$dir/gimp-out.jpg\") (gimp-image-delete image))" \
    -b "(gimp-quit 0)" \
    >"$dir/run.log" 2>&1 || {
      cat "$dir/run.log" >&2
      exit 1
    }

  require_nonempty_file "$dir/gimp-out.jpg"
}

check_gphoto2_runtime() {
  local dir

  dir="$(reset_test_dir gphoto2-runtime)"
  assert_any_file_under_uses_local_soname /usr/lib/x86_64-linux-gnu/libgphoto2 '*.so*' libjpeg.so.8 'libgphoto2 camera modules'

  gphoto2 --camera 'Directory Browse' --port "disk:$FIXTURE_DIR/camera" --list-files >"$dir/list.log" 2>&1 || {
    cat "$dir/list.log" >&2
    exit 1
  }
  require_contains "$dir/list.log" 'IMG_0001.JPG'

  GPHOTO_LOGFILE="$dir/driver.log" gphoto2 \
    --camera 'Directory Browse' \
    --port "disk:$FIXTURE_DIR/camera" \
    --get-file 1 \
    --filename "$dir/downloaded.jpg" \
    >"$dir/get.log" 2>&1 || {
      cat "$dir/get.log" >&2
      exit 1
    }

  require_nonempty_file "$dir/downloaded.jpg"
}

check_krita_runtime() {
  local dir

  dir="$(reset_test_dir krita-runtime)"
  export XDG_RUNTIME_DIR="$dir/xdg"
  mkdir -p "$XDG_RUNTIME_DIR"

  timeout 180 xvfb-run -a --server-args="-screen 0 1024x768x24" \
    krita --nosplash --export --export-filename "$dir/from-jpg.png" "$FIXTURE_DIR/input.jpg" \
    >"$dir/from-jpg.log" 2>&1 || {
      cat "$dir/from-jpg.log" >&2
      exit 1
    }

  timeout 180 xvfb-run -a --server-args="-screen 0 1024x768x24" \
    krita --nosplash --export --export-filename "$dir/from-png.jpg" "$FIXTURE_DIR/input.png" \
    >"$dir/from-png.log" 2>&1 || {
      cat "$dir/from-png.log" >&2
      exit 1
    }

  require_nonempty_file "$dir/from-jpg.png"
  require_nonempty_file "$dir/from-png.jpg"
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

  # Containers generally have no camera hardware. Exercise cam's own MJPEG
  # processing path directly by building a tiny probe against the source files
  # that implement `cam --sdl` JPEG frame handling.
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
    $(pkg-config --cflags libcamera-base sdl2) \
    "$dir/libcamera_mjpg_probe.cpp" \
    "$source_dir/src/apps/cam/sdl_texture.cpp" \
    "$source_dir/src/apps/cam/sdl_texture_mjpg.cpp" \
    -L"/usr/lib/$MULTIARCH" \
    -Wl,-rpath,"/usr/lib/$MULTIARCH" \
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

check_opencv_consumer() {
  local dir

  dir="$(reset_test_dir opencv-consumer)"
  assert_uses_local_soname /usr/lib/x86_64-linux-gnu/libopencv_imgcodecs.so libjpeg.so.8

  cat >"$dir/opencv_smoke.cpp" <<'EOF'
#include <opencv2/imgcodecs.hpp>
#include <iostream>

int main() {
  cv::Mat input = cv::imread("INPUT_JPG", cv::IMREAD_COLOR);
  if (input.empty()) {
    std::cerr << "imread failed\n";
    return 1;
  }
  if (!cv::imwrite("OUTPUT_JPG", input)) {
    std::cerr << "imwrite failed\n";
    return 1;
  }
  std::cout << input.cols << "x" << input.rows << "\n";
  return 0;
}
EOF
  sed -i "s|INPUT_JPG|$FIXTURE_DIR/input.jpg|g; s|OUTPUT_JPG|$dir/opencv-out.jpg|g" "$dir/opencv_smoke.cpp"

  c++ -std=c++17 "$dir/opencv_smoke.cpp" -o "$dir/opencv-smoke" $(pkg-config --cflags --libs opencv4) >/tmp/opencv-compile.log 2>&1 || {
    cat /tmp/opencv-compile.log >&2
    exit 1
  }

  "$dir/opencv-smoke" >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }

  require_nonempty_file "$dir/opencv-out.jpg"
}

check_libreoffice_runtime() {
  local dir

  dir="$(reset_test_dir libreoffice-runtime)"
  mkdir -p "$dir/profile" "$dir/out"
  assert_any_file_under_uses_local_soname /usr/lib/libreoffice/program '*.so*' libjpeg.so.8 'LibreOffice program libraries'

  timeout 180 libreoffice --headless \
    "-env:UserInstallation=file://$dir/profile" \
    --convert-to pdf \
    --outdir "$dir/out" \
    "$FIXTURE_DIR/input.jpg" \
    >"$dir/run.log" 2>&1 || {
      cat "$dir/run.log" >&2
      exit 1
    }

  require_nonempty_file "$dir/out/input.pdf"
}

check_vips_consumer() {
  local dir

  dir="$(reset_test_dir vips-consumer)"
  assert_uses_local_soname /usr/lib/x86_64-linux-gnu/libvips.so libjpeg.so.8

  vips copy "$FIXTURE_DIR/input.jpg" "$dir/roundtrip.png" >/tmp/vips-cli.log 2>&1 || {
    cat /tmp/vips-cli.log >&2
    exit 1
  }
  vips copy "$dir/roundtrip.png" "$dir/roundtrip.jpg" >/tmp/vips-cli2.log 2>&1 || {
    cat /tmp/vips-cli2.log >&2
    exit 1
  }

  cat >"$dir/vips_smoke.c" <<'EOF'
#include <vips/vips.h>
#include <stdio.h>

int main(void) {
  if (VIPS_INIT("vips-smoke")) {
    return 1;
  }

  VipsImage *image = vips_image_new_from_file("INPUT_JPG", NULL);
  if (!image) {
    vips_error_exit(NULL);
  }

  if (vips_image_write_to_file(image, "OUTPUT_JPG", NULL)) {
    g_object_unref(image);
    vips_error_exit(NULL);
  }

  printf("%dx%d\n", vips_image_get_width(image), vips_image_get_height(image));
  g_object_unref(image);
  vips_shutdown();
  return 0;
}
EOF
  sed -i "s|INPUT_JPG|$FIXTURE_DIR/input.jpg|g; s|OUTPUT_JPG|$dir/vips-out.jpg|g" "$dir/vips_smoke.c"

  cc "$dir/vips_smoke.c" -o "$dir/vips-smoke" $(pkg-config --cflags --libs vips) >/tmp/vips-compile.log 2>&1 || {
    cat /tmp/vips-compile.log >&2
    exit 1
  }

  "$dir/vips-smoke" >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }

  require_nonempty_file "$dir/roundtrip.jpg"
  require_nonempty_file "$dir/vips-out.jpg"
}

check_webkit_consumer() {
  local dir

  dir="$(reset_test_dir webkit-consumer)"
  assert_uses_local_soname /usr/lib/x86_64-linux-gnu/libwebkit2gtk-4.1.so.0 libjpeg.so.8

  cat >"$dir/webkit_smoke.c" <<'EOF'
#include <gtk/gtk.h>
#include <webkit2/webkit2.h>

static gboolean check_title(gpointer data) {
  WebKitWebView *view = WEBKIT_WEB_VIEW(data);
  const gchar *title = webkit_web_view_get_title(view);

  if (!title) {
    return G_SOURCE_CONTINUE;
  }

  if (g_str_has_prefix(title, "loaded-")) {
    g_print("title=%s\n", title);
    gtk_main_quit();
    return G_SOURCE_REMOVE;
  }

  if (g_strcmp0(title, "error") == 0) {
    g_printerr("image load failed\n");
    gtk_main_quit();
    return G_SOURCE_REMOVE;
  }

  return G_SOURCE_CONTINUE;
}

int main(void) {
  gtk_init(NULL, NULL);

  GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  GtkWidget *view = webkit_web_view_new();
  gtk_container_add(GTK_CONTAINER(window), view);
  gtk_widget_show_all(window);

  webkit_web_view_load_uri(WEBKIT_WEB_VIEW(view), "WEBKIT_HTML");
  g_timeout_add(100, check_title, view);
  g_timeout_add_seconds(20, (GSourceFunc)gtk_main_quit, NULL);
  gtk_main();

  const gchar *title = webkit_web_view_get_title(WEBKIT_WEB_VIEW(view));
  return (title && g_str_has_prefix(title, "loaded-")) ? 0 : 1;
}
EOF
  sed -i "s|WEBKIT_HTML|file://$FIXTURE_DIR/webkit.html|g" "$dir/webkit_smoke.c"

  cc "$dir/webkit_smoke.c" -o "$dir/webkit-smoke" $(pkg-config --cflags --libs webkit2gtk-4.1) >/tmp/webkit-compile.log 2>&1 || {
    cat /tmp/webkit-compile.log >&2
    exit 1
  }

  timeout 60 xvfb-run -a --server-args="-screen 0 1024x768x24" "$dir/webkit-smoke" >"$dir/run.log" 2>&1 || {
    cat "$dir/run.log" >&2
    exit 1
  }

  require_contains "$dir/run.log" 'title=loaded-'
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

  javac "$dir/JpegSmoke.java" >/tmp/java-compile.log 2>&1 || {
    cat /tmp/java-compile.log >&2
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

check_timg_runtime() {
  local dir

  dir="$(reset_test_dir timg-runtime)"
  assert_uses_local_soname /usr/bin/timg libturbojpeg.so.0

  TERM=xterm /usr/bin/timg -g 40x15 "$FIXTURE_DIR/input.jpg" >"$dir/render.txt" 2>&1 || {
    cat "$dir/render.txt" >&2
    exit 1
  }

  require_nonempty_file "$dir/render.txt"
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

check_xpra_jpeg_codec() {
  local dir

  dir="$(reset_test_dir xpra-jpeg-codec)"
  assert_any_file_under_uses_local_soname /usr/lib/python3/dist-packages/xpra '*.so*' libturbojpeg.so.0 'Xpra JPEG codec modules'

  FIXTURE_DIR="$FIXTURE_DIR" python3 - <<'PY' >"$dir/run.log" 2>&1
from PIL import Image
from xpra.codecs.image_wrapper import ImageWrapper
from xpra.codecs.jpeg import encoder, decoder
import os

fixture_dir = os.environ["FIXTURE_DIR"]
image = Image.open(os.path.join(fixture_dir, "input.jpg")).convert("RGBA")
raw = image.tobytes("raw", "RGBA")
wrapper = ImageWrapper(0, 0, image.width, image.height, raw, "RGBX", 24, image.width * 4)
encoding, compressed, options, width, height, _, _ = encoder.encode(wrapper, quality=80, speed=50, options={})
decoded = decoder.decompress_to_rgb("RGBX", compressed.data, width, height, {})

print(encoding)
print(len(compressed.data))
print(decoded.get_width(), decoded.get_height(), decoded.get_pixel_format())
PY

  require_contains "$dir/run.log" 'jpeg'
  require_contains "$dir/run.log" 'RGBX'
}

run_compile_checks() {
  run_selected_compile dcm2niix dcm2niix-source 'dcm2niix source build' check_dcm2niix_source_build
  run_selected_compile timg timg-source 'timg source build' check_timg_source_build
  run_selected_compile opencv opencv-source 'opencv source build' check_opencv_source_build
  run_selected_compile vips vips-source 'vips source build' check_vips_source_build
  run_selected_compile xpra xpra-source 'xpra source build' check_xpra_source_build
  run_selected_compile krita krita-source 'krita source build' check_krita_source_build
  run_selected_compile libreoffice libreoffice-source 'libreoffice source build' check_libreoffice_source_build
  run_selected_compile webkit2gtk webkit-source 'webkit2gtk source build' check_webkit_source_build
}

run_runtime_checks() {
  run_selected_runtime dcm2niix dcm2niix-runtime 'dcm2niix runtime smoke' check_dcm2niix_runtime
  run_selected_runtime eog eog-runtime 'eog runtime smoke' check_eog_runtime
  run_selected_runtime gimp gimp-runtime 'gimp runtime smoke' check_gimp_runtime
  run_selected_runtime gphoto2 gphoto2-runtime 'gphoto2 runtime smoke' check_gphoto2_runtime
  run_selected_runtime krita krita-runtime 'krita runtime smoke' check_krita_runtime
  run_selected_runtime libcamera-tools libcamera-tools-runtime 'libcamera-tools runtime smoke' check_libcamera_tools_runtime
  run_selected_runtime libopencv-imgcodecs406t64 opencv-consumer 'libopencv-imgcodecs406t64 runtime smoke' check_opencv_consumer
  run_selected_runtime libreoffice-core libreoffice-runtime 'libreoffice-core runtime smoke' check_libreoffice_runtime
  run_selected_runtime libvips42t64 vips-consumer 'libvips42t64 runtime smoke' check_vips_consumer
  run_selected_runtime libwebkit2gtk-4.1-0 webkit-consumer 'libwebkit2gtk-4.1-0 runtime smoke' check_webkit_consumer
  run_selected_runtime openjdk-17-jre-headless openjdk-runtime 'openjdk-17-jre-headless runtime smoke' check_openjdk_runtime
  run_selected_runtime python3-pil pillow-runtime 'python3-pil runtime smoke' check_pillow_runtime
  run_selected_runtime timg timg-runtime 'timg runtime smoke' check_timg_runtime
  run_selected_runtime tracker-extract tracker-extract-runtime 'tracker-extract runtime smoke' check_tracker_extract_runtime
  run_selected_runtime xpra xpra-jpeg-codec 'xpra runtime smoke' check_xpra_jpeg_codec
}

assert_dependents_inventory
log_step 'Building and installing safe Debian packages'
build_safe_packages
log_step 'Preparing JPEG, PNG, HTML, DICOM, and pseudo-camera fixtures'
prepare_fixtures

case "$CHECKS" in
  compile)
    run_compile_checks
    ;;
  runtime)
    run_runtime_checks
    ;;
  all)
    run_compile_checks
    run_runtime_checks
    ;;
esac

log_step 'All requested dependent checks passed'
CONTAINER_SCRIPT
