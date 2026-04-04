# Remove the Temporary C Bridge, Minimize Unsafe, Recover Performance, and Run the Full Compatibility Sweep

## Phase Name
Remove the Temporary C Bridge, Minimize Unsafe, Recover Performance, and Run the Full Compatibility Sweep

## Implement Phase ID
`impl_final_cleanup`

## Preexisting Inputs
- `safe/Cargo.toml`
- `safe/build.rs`
- `safe/src/lib.rs`
- `safe/README.md`
- `safe/crates/ffi-types/src/lib.rs`
- `safe/crates/jpeg-core/src/`
- `safe/crates/libjpeg-abi/src/`
- `safe/crates/libturbojpeg-abi/src/`
- `safe/crates/jpeg-tools/src/`
- `safe/include/install-manifest.txt`
- `safe/link/libjpeg.map`
- `safe/link/turbojpeg-mapfile`
- `safe/link/turbojpeg-mapfile.jni`
- `safe/pkgconfig/libjpeg.pc.in`
- `safe/pkgconfig/libturbojpeg.pc.in`
- `safe/cmake/libjpeg-turboConfig.cmake.in`
- `safe/cmake/libjpeg-turboConfigVersion.cmake.in`
- `safe/cmake/libjpeg-turboTargets.cmake.in`
- `safe/debian/`
- `safe/java/`
- `safe/scripts/stage-install.sh`
- `safe/scripts/check-symbols.sh`
- `safe/scripts/relink-original-objects.sh`
- `safe/scripts/original-object-groups.json`
- `safe/scripts/run-compat-fixtures.sh`
- `safe/scripts/run-debian-autopkgtests.sh`
- `safe/scripts/run-dependent-subset.sh`
- `safe/scripts/run-dependent-regressions.sh`
- `safe/scripts/run-progs-smoke.sh`
- `safe/scripts/run-java-tests.sh`
- `safe/scripts/audit-unsafe.sh`
- `safe/scripts/run-bench-smoke.sh`
- `safe/tests/layout_ctest.rs`
- `safe/tests/compat_smoke.rs`
- `safe/tests/upstream_matrix.rs`
- `safe/tests/cve_regressions.rs`
- `safe/tests/turbojpeg_suite.rs`
- `safe/tests/dependent_regressions.rs`
- `safe/tests/fixtures/dependents/`
- `safe/target/dependent-matrix/summary.json`
- `safe/target/dependent-matrix-fixed/summary.json`
- `original/debian/libjpeg-turbo8.symbols`
- `original/debian/libturbojpeg.symbols`
- `original/CMakeLists.txt`
- `original/testimages/`
- `original/jcstest.c`
- `original/strtest.c`
- `original/tjunittest.c`
- `original/tjbenchtest.in`
- `original/tjexampletest.in`
- `original/croptest.in`
- `original/java/`
- `relevant_cves.json`
- `dependents.json`
- `test-original.sh`

## New Outputs
- final Rust-first drop-in package with no remaining build-time dependency on `original/*.c` except any documented minimal boundary shim
- `safe/target/dependent-matrix-final/summary.json`
- passing safety and performance audit tooling plus full compatibility evidence

## File Changes
- Modify `safe/build.rs` and `safe/Cargo.toml` as needed to remove temporary bridge logic
- Modify the affected paths under `safe/crates/{jpeg-core,libjpeg-abi,libturbojpeg-abi,jpeg-tools}/`
- Modify `safe/scripts/{audit-unsafe,run-bench-smoke,stage-install,relink-original-objects,run-dependent-regressions}.sh` as needed
- Modify `safe/README.md` or equivalent package notes if the remaining unavoidable unsafe boundary needs documentation

## Implementation Details
- Remove every remaining temporary compilation dependency on `original/*.c` from the safe build.
- If a tiny C or `unsafe` boundary remains for `longjmp`, JNI, or raw FFI, isolate it to the narrowest possible files and document precisely why it cannot be made safe Rust.
- Eliminate the current linker-plugin warning flood around opaque pointers so final verification is clean and readable.
- Recover performance in the main hot paths without weakening ABI or runtime compatibility.

## Verification Phases

### `check_full_compatibility`
- Phase ID: `check_full_compatibility`
- Type: `check`
- Bounce Target: `impl_final_cleanup`
- Purpose: run the final staged build, self-tests, Debian package build, symbol diff, original-object relink, dependent regressions, and full dependent harness.
- Commands:

```bash
cargo build --manifest-path safe/Cargo.toml --workspace --release
bash safe/scripts/stage-install.sh --with-java=1
cargo test --manifest-path safe/Cargo.toml --workspace --release
cargo test --manifest-path safe/Cargo.toml -p safe --test layout_ctest --release
cargo test --manifest-path safe/Cargo.toml -p safe --test compat_smoke --release
cargo test --manifest-path safe/Cargo.toml -p safe --test upstream_matrix --release
cargo test --manifest-path safe/Cargo.toml -p safe --test cve_regressions --release
cargo test --manifest-path safe/Cargo.toml -p safe --test turbojpeg_suite --release
cargo test --manifest-path safe/Cargo.toml -p safe --test dependent_regressions --release
(cd safe && dpkg-buildpackage -us -uc -b)
LIBJPEG_DEB="$(find . -maxdepth 1 -type f -name 'libjpeg-turbo8_*_*.deb' -print -quit)"
TURBOJPEG_DEB="$(find . -maxdepth 1 -type f -name 'libturbojpeg_*_*.deb' -print -quit)"
PROGS_DEB="$(find . -maxdepth 1 -type f -name 'libjpeg-turbo-progs_*_*.deb' -print -quit)"
test -n "$LIBJPEG_DEB"
test -n "$TURBOJPEG_DEB"
test -n "$PROGS_DEB"
TMPROOT="$(mktemp -d)"
trap 'rm -rf "$TMPROOT"' EXIT
dpkg-deb -x "$LIBJPEG_DEB" "$TMPROOT"
dpkg-deb -x "$TURBOJPEG_DEB" "$TMPROOT"
dpkg-deb -x "$PROGS_DEB" "$TMPROOT"
bash safe/scripts/check-symbols.sh original/debian/libjpeg-turbo8.symbols "$(find safe/stage/usr/lib -name 'libjpeg.so.8' -print -quit)"
bash safe/scripts/check-symbols.sh original/debian/libturbojpeg.symbols "$(find safe/stage/usr/lib -name 'libturbojpeg.so.0' -print -quit)"
bash safe/scripts/relink-original-objects.sh --group all
bash safe/scripts/run-debian-autopkgtests.sh
bash safe/scripts/run-progs-smoke.sh
bash safe/scripts/run-progs-smoke.sh --usr-root "$TMPROOT/usr"
bash safe/scripts/run-java-tests.sh
bash safe/scripts/run-dependent-regressions.sh --mode verify
./test-original.sh --checks all --report-dir safe/target/dependent-matrix-final
jq -e 'all(.runtime[]; .status == "pass") and all(.compile[]; .status == "pass")' safe/target/dependent-matrix-final/summary.json
```

### `check_safety_and_perf`
- Phase ID: `check_safety_and_perf`
- Type: `check`
- Bounce Target: `impl_final_cleanup`
- Purpose: verify that only justified boundary `unsafe` remains and that the hot paths have regained acceptable performance characteristics.
- Commands:

```bash
cargo clippy --manifest-path safe/Cargo.toml --workspace --all-targets --all-features -- -D warnings
bash safe/scripts/audit-unsafe.sh
bash safe/scripts/run-bench-smoke.sh
```

### `check_final_software_tester`
- Phase ID: `check_final_software_tester`
- Type: `check`
- Bounce Target: `impl_final_cleanup`
- Purpose: software-tester review of the final compatibility evidence bundle, including dependent regressions and package and runtime replacement behavior.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo test --manifest-path safe/Cargo.toml -p safe --test dependent_regressions --release
bash safe/scripts/run-dependent-regressions.sh --mode verify
./test-original.sh --checks all --report-dir safe/target/dependent-matrix-final
```

### `check_final_senior_tester`
- Phase ID: `check_final_senior_tester`
- Type: `check`
- Bounce Target: `impl_final_cleanup`
- Purpose: senior-tester review that all earlier contracts were actually honored: no remaining bridge dependency, no unresolved regression ownership, and no skipped packaging or runtime checks.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
bash safe/scripts/stage-install.sh --with-java=1
bash safe/scripts/check-symbols.sh original/debian/libjpeg-turbo8.symbols "$(find safe/stage/usr/lib -name 'libjpeg.so.8' -print -quit)"
bash safe/scripts/check-symbols.sh original/debian/libturbojpeg.symbols "$(find safe/stage/usr/lib -name 'libturbojpeg.so.0' -print -quit)"
bash safe/scripts/audit-unsafe.sh
./test-original.sh --checks all --report-dir safe/target/dependent-matrix-final
jq -e 'all(.runtime[]; .status == "pass") and all(.compile[]; .status == "pass")' safe/target/dependent-matrix-final/summary.json
```

## Success Criteria
- Temporary bridge logic is removed or reduced to a precisely documented minimal boundary, and remaining `unsafe` is justified and isolated.
- Final verification covers staging, packages, symbols, upstream tests, dependent regressions, the full dependent matrix, and safety and performance audits.
- All four verifier phases pass with `impl_final_cleanup` as their only bounce target.

## Git Commit Requirement
The implementer must commit all work for `impl_final_cleanup` to git before yielding. Do not yield with unstaged or uncommitted changes.
