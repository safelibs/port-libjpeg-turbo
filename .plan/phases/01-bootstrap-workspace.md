# Bootstrap Rust Workspace, Installed Metadata, and Compatibility Harness Scaffolding

## Phase Name
Bootstrap Rust Workspace, Installed Metadata, and Compatibility Harness Scaffolding

## Implement Phase ID
`impl_bootstrap_workspace`

## Preexisting Inputs
- `safe/Cargo.toml`
- `safe/src/lib.rs`
- `safe/build.rs`
- `safe/.cargo/config.toml`
- `safe/include/install-manifest.txt`
- `safe/link/libjpeg.map`
- `safe/link/turbojpeg-mapfile`
- `safe/link/turbojpeg-mapfile.jni`
- `safe/pkgconfig/libjpeg.pc.in`
- `safe/pkgconfig/libturbojpeg.pc.in`
- `safe/cmake/libjpeg-turboConfig.cmake.in`
- `safe/cmake/libjpeg-turboConfigVersion.cmake.in`
- `safe/cmake/libjpeg-turboTargets.cmake.in`
- `safe/scripts/stage-install.sh`
- `safe/scripts/check-symbols.sh`
- `safe/scripts/relink-original-objects.sh`
- `safe/scripts/run-dependent-subset.sh`
- `safe/scripts/original-object-groups.json`
- `safe/debian/`
- `safe/debian/extra/`
- `original/debian/`
- `original/debian/extra/`
- `original/sharedlib/CMakeLists.txt`
- `original/libjpeg.map.in`
- `original/turbojpeg-mapfile`
- `original/turbojpeg-mapfile.jni`

## New Outputs
- corrected staged install tree under `safe/stage/usr/`
- working pkg-config and CMake metadata under the staged multiarch libdir
- helper-script interfaces stabilized for later phases
- Debian-compatible install manifests and staged manpage and wrapper sources kept under `safe/debian/`

## File Changes
- Modify `safe/Cargo.toml`, `safe/build.rs`, `safe/src/lib.rs`, and `safe/.cargo/config.toml`
- Modify `safe/include/install-manifest.txt`
- Modify `safe/link/{libjpeg.map,turbojpeg-mapfile,turbojpeg-mapfile.jni}`
- Modify `safe/pkgconfig/{libjpeg.pc.in,libturbojpeg.pc.in}`
- Modify `safe/cmake/{libjpeg-turboConfig.cmake.in,libjpeg-turboConfigVersion.cmake.in,libjpeg-turboTargets.cmake.in}`
- Modify `safe/scripts/{stage-install,check-symbols,relink-original-objects,run-dependent-subset}.sh`
- Modify `safe/scripts/original-object-groups.json`
- Modify `safe/debian/{control,rules,*.install,*.symbols,*.docs,*.examples,*.lintian-overrides,tjbench.1.in,cjpeg.1,djpeg.1,jpegtran.1,rdjpgcom.1,wrjpgcom.1}`
- Modify `safe/debian/extra/{Makefile,exifautotran,jpegexiforient.1,exifautotran.1}`

## Implementation Details
- Keep `safe/Cargo.toml` as both the workspace manifest and a real root package; do not move the repo-wide tests out of `safe/tests/`.
- Ensure the staged install tree matches Debian expectations from `original/debian/*.install`, including multiarch `jconfig.h`, installed internal headers (`jpegint.h`, `jmemsys.h`, `jsimd.h`), static and shared libraries, pkg-config files, and CMake package metadata.
- Keep the staged tool and manpage assets self-contained under `safe/`; later package builds must not read manpages or wrapper scripts from `../original`.
- Preserve symbol and version-script wiring against `original/debian/libjpeg-turbo8.symbols` and the non-JNI subset of `original/debian/libturbojpeg.symbols`.
- Stabilize helper-script CLIs now, because later phases will consume them directly rather than inventing replacements.

## Verification Phases

### `check_bootstrap_stage_tree`
- Phase ID: `check_bootstrap_stage_tree`
- Type: `check`
- Bounce Target: `impl_bootstrap_workspace`
- Purpose: verify that the existing Rust workspace builds, stages the expected Debian-compatible install tree, and produces the helper artifacts needed by later phases.
- Commands:

```bash
cargo build --manifest-path safe/Cargo.toml --workspace --release
bash safe/scripts/stage-install.sh
readelf -d "$(find safe/stage/usr/lib -name 'libjpeg.so.8' -print -quit)" | grep -F 'SONAME'
readelf -d "$(find safe/stage/usr/lib -name 'libturbojpeg.so.0' -print -quit)" | grep -F 'SONAME'
find safe/stage/usr/lib -name 'libjpeg.a' -print -quit | grep -q .
find safe/stage/usr/lib -name 'libturbojpeg.a' -print -quit | grep -q .
find safe/stage/usr/lib -path '*/pkgconfig/libjpeg.pc' -print -quit | grep -q .
find safe/stage/usr/lib -path '*/pkgconfig/libturbojpeg.pc' -print -quit | grep -q .
find safe/stage/usr/lib -path '*/cmake/libjpeg-turbo/libjpeg-turboConfig.cmake' -print -quit | grep -q .
find safe/stage/usr/include -path '*/jconfig.h' -print -quit | grep -q .
bash safe/scripts/relink-original-objects.sh --help
bash safe/scripts/run-dependent-subset.sh --help
```

### `check_bootstrap_symbol_plumbing`
- Phase ID: `check_bootstrap_symbol_plumbing`
- Type: `check`
- Bounce Target: `impl_bootstrap_workspace`
- Purpose: verify that the staged libraries are already wired to the canonical Debian symbol oracles, with the JNI subset explicitly deferred.
- Commands:

```bash
bash safe/scripts/stage-install.sh
bash safe/scripts/check-symbols.sh original/debian/libjpeg-turbo8.symbols "$(find safe/stage/usr/lib -name 'libjpeg.so.8' -print -quit)"
bash safe/scripts/check-symbols.sh --skip-regex '^Java_org_libjpegturbo_turbojpeg_' original/debian/libturbojpeg.symbols "$(find safe/stage/usr/lib -name 'libturbojpeg.so.0' -print -quit)"
```

### `check_bootstrap_software_tester`
- Phase ID: `check_bootstrap_software_tester`
- Type: `check`
- Bounce Target: `impl_bootstrap_workspace`
- Purpose: software-tester review of staged install coverage, helper-script ergonomics, and package-layout completeness before deeper porting starts.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo build --manifest-path safe/Cargo.toml --workspace --release
bash safe/scripts/stage-install.sh
bash safe/scripts/check-symbols.sh original/debian/libjpeg-turbo8.symbols "$(find safe/stage/usr/lib -name 'libjpeg.so.8' -print -quit)"
```

### `check_bootstrap_senior_tester`
- Phase ID: `check_bootstrap_senior_tester`
- Type: `check`
- Bounce Target: `impl_bootstrap_workspace`
- Purpose: senior-tester review of bootstrap assumptions, ownership boundaries, and whether the produced artifacts are sufficient for the remaining linear workflow.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
bash safe/scripts/stage-install.sh
bash safe/scripts/check-symbols.sh --skip-regex '^Java_org_libjpegturbo_turbojpeg_' original/debian/libturbojpeg.symbols "$(find safe/stage/usr/lib -name 'libturbojpeg.so.0' -print -quit)"
```

## Success Criteria
- The workspace builds and stages a Debian-compatible install tree with the expected headers, static and shared libraries, pkg-config metadata, and CMake metadata.
- Symbol plumbing and helper-script interfaces are stable enough for later phases to consume the staged artifacts directly.
- All four verifier phases pass with `impl_bootstrap_workspace` as their only bounce target.

## Git Commit Requirement
The implementer must commit all work for `impl_bootstrap_workspace` to git before yielding. Do not yield with unstaged or uncommitted changes.
