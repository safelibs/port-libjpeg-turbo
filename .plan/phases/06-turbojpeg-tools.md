# TurboJPEG API, Helper Image I/O, and CLI Programs

## Phase Name
TurboJPEG API, Helper Image I/O, and CLI Programs

## Implement Phase ID
`impl_turbojpeg_tools`

## Preexisting Inputs
- `original/turbojpeg.h`
- `original/turbojpeg.c`
- `original/jdatasrc-tj.c`
- `original/jdatadst-tj.c`
- `original/tjutil.c`
- `original/tjutil.h`
- `original/tjbench.c`
- `original/tjexample.c`
- `original/cjpeg.c`
- `original/djpeg.c`
- `original/jpegtran.c`
- `original/rdjpgcom.c`
- `original/wrjpgcom.c`
- `original/cjpeg.1`
- `original/djpeg.1`
- `original/jpegtran.1`
- `original/rdjpgcom.1`
- `original/wrjpgcom.1`
- `original/rdbmp.c`
- `original/rdppm.c`
- `original/rdgif.c`
- `original/rdtarga.c`
- `original/wrbmp.c`
- `original/wrppm.c`
- `original/wrgif.c`
- `original/wrtarga.c`
- `original/cdjpeg.c`
- `original/cdjpeg.h`
- `original/rdswitch.c`
- `original/rdcolmap.c`
- `original/tjunittest.c`
- `original/tjbenchtest.in`
- `original/tjexampletest.in`
- `original/debian/libjpeg-turbo-progs.install`
- `original/debian/extra/`
- `original/debian/libturbojpeg.symbols`
- `safe/crates/jpeg-core/src/ported/turbojpeg/`
- `safe/crates/libturbojpeg-abi/src/lib.rs`
- `safe/crates/libturbojpeg-abi/src/generated/`
- `safe/crates/jpeg-tools/src/cdjpeg.rs`
- `safe/crates/jpeg-tools/src/rdswitch.rs`
- `safe/crates/jpeg-tools/src/rdcolmap.rs`
- `safe/crates/jpeg-tools/src/lib.rs`
- `safe/crates/jpeg-tools/src/image_io/`
- `safe/crates/jpeg-tools/src/bin/`
- `safe/crates/jpeg-tools/src/generated/`
- `safe/debian/`
- `safe/debian/extra/`
- `safe/scripts/stage-install.sh`
- `safe/scripts/check-symbols.sh`
- `safe/scripts/relink-original-objects.sh`
- `safe/scripts/run-progs-smoke.sh`
- `safe/scripts/run-dependent-subset.sh`
- `safe/scripts/original-object-groups.json`
- `safe/tests/turbojpeg_suite.rs`
- `dependents.json`

## New Outputs
- Rust TurboJPEG API implementation with the non-JNI symbol surface
- Rust helper-image I/O and CLI-tool behavior for the `libjpeg-turbo-progs` package
- staged tool/manpage contract verified entirely from `safe/`

## File Changes
- Modify `safe/crates/jpeg-core/src/ported/turbojpeg/{turbojpeg,jdatasrc_tj,jdatadst_tj,tjutil}.rs`
- Modify `safe/crates/libturbojpeg-abi/src/{lib,generated/**}`
- Modify `safe/crates/jpeg-tools/src/{cdjpeg,rdswitch,rdcolmap,lib}.rs`
- Modify `safe/crates/jpeg-tools/src/image_io/{rdbmp,rdppm,rdgif,rdtarga,wrbmp,wrppm,wrgif,wrtarga}.rs`
- Modify `safe/crates/jpeg-tools/src/bin/{cjpeg,djpeg,jpegtran,rdjpgcom,wrjpgcom,tjbench,tjexample,jpegexiforient}.rs`
- Modify `safe/crates/jpeg-tools/src/generated/**` as needed
- Modify `safe/debian/{cjpeg.1,djpeg.1,jpegtran.1,rdjpgcom.1,wrjpgcom.1,tjbench.1.in}`
- Modify `safe/debian/extra/{exifautotran,jpegexiforient.1,exifautotran.1}`
- Modify `safe/scripts/{stage-install,run-progs-smoke}.sh`
- Modify `safe/tests/turbojpeg_suite.rs`

## Implementation Details
- Preserve the versioned `tj*` export history from the TurboJPEG mapfile and current ABI crate.
- Port the `cdjpeg` helper layer and image readers/writers in Rust instead of reimplementing tool frontends ad hoc.
- Preserve the packaged `libjpeg-turbo-progs` payload: `cjpeg`, `djpeg`, `jpegtran`, `rdjpgcom`, `wrjpgcom`, `tjbench`, `jpegexiforient`, `exifautotran`, and their manpages.
- Encode the explicit non-memory CVE surfaces around malformed BMP, GIF, and Targa helper I/O.
- Keep `safe/debian/extra/exifautotran` and `safe/debian/extra/*.1` as the authoritative packaged sources rather than generating wrapper copies elsewhere.

## Verification Phases

### `check_turbojpeg_suite`
- Phase ID: `check_turbojpeg_suite`
- Type: `check`
- Bounce Target: `impl_turbojpeg_tools`
- Purpose: verify the ported TurboJPEG/unit/tool suite, the complete non-JNI `libturbojpeg` export set, and TurboJPEG-oriented relink coverage.
- Commands:

```bash
cargo test --manifest-path safe/Cargo.toml -p safe --test turbojpeg_suite
bash safe/scripts/stage-install.sh
bash safe/scripts/check-symbols.sh --skip-regex '^Java_org_libjpegturbo_turbojpeg_' original/debian/libturbojpeg.symbols "$(find safe/stage/usr/lib -name 'libturbojpeg.so.0' -print -quit)"
bash safe/scripts/relink-original-objects.sh --group turbojpeg
```

### `check_cli_progs_contract`
- Phase ID: `check_cli_progs_contract`
- Type: `check`
- Bounce Target: `impl_turbojpeg_tools`
- Purpose: verify the staged `libjpeg-turbo-progs` payload end to end, including binaries, manpages, comment round-trips, and Exif orientation behavior.
- Commands:

```bash
bash safe/scripts/stage-install.sh
bash safe/scripts/run-progs-smoke.sh
```

### `check_turbojpeg_dependents`
- Phase ID: `check_turbojpeg_dependents`
- Type: `check`
- Bounce Target: `impl_turbojpeg_tools`
- Purpose: verify direct `libturbojpeg` consumers and tool-facing runtime dependents.
- Commands:

```bash
bash safe/scripts/run-dependent-subset.sh --checks runtime --only dcm2niix --only timg --only xpra --only krita
```

### `check_turbojpeg_software_tester`
- Phase ID: `check_turbojpeg_software_tester`
- Type: `check`
- Bounce Target: `impl_turbojpeg_tools`
- Purpose: software-tester review of tool behavior, helper image I/O edge cases, and TurboJPEG API coverage.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo test --manifest-path safe/Cargo.toml -p safe --test turbojpeg_suite
bash safe/scripts/stage-install.sh
bash safe/scripts/run-progs-smoke.sh
```

### `check_turbojpeg_senior_tester`
- Phase ID: `check_turbojpeg_senior_tester`
- Type: `check`
- Bounce Target: `impl_turbojpeg_tools`
- Purpose: senior-tester review of compatibility exports, CLI option parity, and non-memory CVE surfaces in helper image loaders/writers.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
bash safe/scripts/stage-install.sh
bash safe/scripts/check-symbols.sh --skip-regex '^Java_org_libjpegturbo_turbojpeg_' original/debian/libturbojpeg.symbols "$(find safe/stage/usr/lib -name 'libturbojpeg.so.0' -print -quit)"
bash safe/scripts/run-dependent-subset.sh --checks runtime --only dcm2niix --only timg --only xpra --only krita
```

## Success Criteria
- TurboJPEG, helper image I/O, and packaged CLI program behavior are implemented in Rust with the non-JNI export surface preserved.
- The staged `libjpeg-turbo-progs` contract is validated entirely from committed assets under `safe/`.
- All five verifier phases pass with `impl_turbojpeg_tools` as their only bounce target.

## Git Commit Requirement
The implementer must commit all work for `impl_turbojpeg_tools` to git before yielding. Do not yield with unstaged or uncommitted changes.
