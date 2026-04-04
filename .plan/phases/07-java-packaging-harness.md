# Java/JNI Compatibility, Debian Package Build, and Root Harness Switch-Over

## Phase Name
Java/JNI Compatibility, Debian Package Build, and Root Harness Switch-Over

## Implement Phase ID
`impl_java_packaging_harness`

## Preexisting Inputs
- `original/java/`
- `original/turbojpeg-jni.c`
- `original/turbojpeg-mapfile.jni`
- `original/debian/`
- `original/debian/libturbojpeg.symbols`
- `safe/java/`
- `safe/scripts/stage-install.sh`
- `safe/scripts/check-symbols.sh`
- `safe/scripts/run-progs-smoke.sh`
- `safe/scripts/run-java-tests.sh`
- `safe/crates/libturbojpeg-abi/build.rs`
- `safe/crates/libturbojpeg-abi/src/lib.rs`
- `safe/crates/libturbojpeg-abi/src/generated/`
- `safe/link/turbojpeg-mapfile.jni`
- `safe/debian/`
- `test-original.sh`
- `dependents.json`

## New Outputs
- safe Debian packages for `libjpeg-turbo8`, `libjpeg-turbo8-dev`, `libturbojpeg`, `libturbojpeg0-dev`, `libjpeg-turbo-progs`, and `libturbojpeg-java`
- Java/JNI-compatible `libturbojpeg.so.0` and `turbojpeg.jar`
- shared progs smoke coverage that can validate both `safe/stage/usr` and an extracted Debian package tree
- repo-level harness switched fully to the safe package flow

## File Changes
- Modify `safe/java/**`, including committed `TJLoader-*.java.in` templates and rendered `TJLoader.java`
- Modify `safe/scripts/{run-java-tests,run-progs-smoke,stage-install}.sh`
- Modify `safe/crates/libturbojpeg-abi/{build.rs,src/lib.rs,src/generated/**}`
- Modify `safe/link/turbojpeg-mapfile.jni`
- Modify `safe/debian/{control,rules,*.install,*.docs,*.examples,*.lintian-overrides,*.symbols,libjpeg-turbo-only.symbols,copyright,tests/*}`
- Modify `test-original.sh`

## Implementation Details
- Preserve the upstream Java package layout and loader-template flow instead of inventing a new Java API.
- Export the JNI entry points from `libturbojpeg.so.0` using the canonical JNI symbol names already reflected in `original/debian/libturbojpeg.symbols`.
- Keep the Debian package build self-contained under `safe/`; neither `dpkg-buildpackage` nor the rewritten `test-original.sh` may read packaging/manpage/wrapper assets from `../original`.
- Extend the existing `safe/scripts/run-progs-smoke.sh` runner with an explicit alternate-root mode such as `--usr-root <path>` so the same harness validates both `safe/stage/usr` and the extracted `libjpeg-turbo-progs`/`libjpeg-turbo8`/`libturbojpeg` Debian payload. Do not add a second package-progs smoke script.
- Keep `dependents.json` authoritative; the harness changes only the install path and reporting flow, not the application inventory.

## Verification Phases

### `check_java_and_packages`
- Phase ID: `check_java_and_packages`
- Type: `check`
- Bounce Target: `impl_java_packaging_harness`
- Purpose: verify the full `libturbojpeg.symbols` manifest including JNI exports, the Java bindings, and the Debian package build for the full package set.
- Commands:

```bash
bash safe/scripts/stage-install.sh --with-java=1
bash safe/scripts/check-symbols.sh original/debian/libturbojpeg.symbols "$(find safe/stage/usr/lib -name 'libturbojpeg.so.0' -print -quit)"
(cd safe && dpkg-buildpackage -us -uc -b)
bash safe/scripts/run-java-tests.sh
```

### `check_debian_progs_package_contract`
- Phase ID: `check_debian_progs_package_contract`
- Type: `check`
- Bounce Target: `impl_java_packaging_harness`
- Purpose: verify that the built `libjpeg-turbo-progs` Debian package contains the full drop-in binary/manpage payload and that the extracted package-installed tools behave compatibly under the shared progs smoke runner.
- Commands:

```bash
(cd safe && dpkg-buildpackage -us -uc -b)
LIBJPEG_DEB="$(find . -maxdepth 1 -type f -name 'libjpeg-turbo8_*_*.deb' -print -quit)"
TURBOJPEG_DEB="$(find . -maxdepth 1 -type f -name 'libturbojpeg_*_*.deb' -print -quit)"
PROGS_DEB="$(find . -maxdepth 1 -type f -name 'libjpeg-turbo-progs_*_*.deb' -print -quit)"
test -n "$LIBJPEG_DEB"
test -n "$TURBOJPEG_DEB"
test -n "$PROGS_DEB"
dpkg-deb -c "$PROGS_DEB" | grep -E '/usr/bin/(cjpeg|djpeg|jpegtran|rdjpgcom|wrjpgcom|tjbench|jpegexiforient|exifautotran)$'
dpkg-deb -c "$PROGS_DEB" | grep -E '/usr/share/man/man1/(cjpeg|djpeg|jpegtran|rdjpgcom|wrjpgcom|tjbench|jpegexiforient|exifautotran)\\.1(\\.gz)?$'
TMPROOT="$(mktemp -d)"
trap 'rm -rf "$TMPROOT"' EXIT
dpkg-deb -x "$LIBJPEG_DEB" "$TMPROOT"
dpkg-deb -x "$TURBOJPEG_DEB" "$TMPROOT"
dpkg-deb -x "$PROGS_DEB" "$TMPROOT"
bash safe/scripts/run-progs-smoke.sh --usr-root "$TMPROOT/usr"
```

### `check_root_harness_subset`
- Phase ID: `check_root_harness_subset`
- Type: `check`
- Bounce Target: `impl_java_packaging_harness`
- Purpose: verify that the repo-level Ubuntu 24.04 harness now installs safe Debian packages and can drive representative compile/runtime checks unchanged.
- Commands:

```bash
./test-original.sh --checks compile --only dcm2niix
./test-original.sh --checks compile --only vips
./test-original.sh --checks runtime --only dcm2niix
./test-original.sh --checks runtime --only python3-pil
```

### `check_java_packaging_software_tester`
- Phase ID: `check_java_packaging_software_tester`
- Type: `check`
- Bounce Target: `impl_java_packaging_harness`
- Purpose: software-tester review of Java loader behavior, staged/package parity, and harness switch-over safety.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
bash safe/scripts/stage-install.sh --with-java=1
bash safe/scripts/run-java-tests.sh
(cd safe && dpkg-buildpackage -us -uc -b)
./test-original.sh --checks runtime --only python3-pil
```

### `check_java_packaging_senior_tester`
- Phase ID: `check_java_packaging_senior_tester`
- Type: `check`
- Bounce Target: `impl_java_packaging_harness`
- Purpose: senior-tester review of JNI symbol ownership, Debian package self-containment, and whether the root harness now consumes only safe artifacts.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
bash safe/scripts/stage-install.sh --with-java=1
bash safe/scripts/check-symbols.sh original/debian/libturbojpeg.symbols "$(find safe/stage/usr/lib -name 'libturbojpeg.so.0' -print -quit)"
(cd safe && dpkg-buildpackage -us -uc -b)
./test-original.sh --checks compile --only dcm2niix
./test-original.sh --checks runtime --only dcm2niix
```

## Success Criteria
- The safe Debian package set builds from `safe/` alone, including JNI-compatible `libturbojpeg.so.0`, `turbojpeg.jar`, and the `libjpeg-turbo-progs` package.
- The shared progs smoke runner validates both the staged install tree and an extracted Debian package root, and `test-original.sh` now exercises the safe package flow without inventing a new dependent inventory.
- All five verifier phases pass with `impl_java_packaging_harness` as their only bounce target.

## Git Commit Requirement
The implementer must commit all work for `impl_java_packaging_harness` to git before yielding. Do not yield with unstaged or uncommitted changes.
