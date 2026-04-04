# Advanced Decompression, Partial Decode, and CVE Hardening

## Phase Name
Advanced Decompression, Partial Decode, and CVE Hardening

## Implement Phase ID
`impl_advanced_decompress`

## Preexisting Inputs
- `original/jdphuff.c`
- `original/jdarith.c`
- `original/jdpostct.c`
- `original/jquant1.c`
- `original/jquant2.c`
- `original/jdtrans.c`
- `original/jdicc.c`
- `original/jdapistd.c`
- `original/libjpeg.txt`
- `original/ChangeLog.md`
- `relevant_cves.json`
- `safe/crates/jpeg-core/src/ported/decompress/`
- `safe/crates/jpeg-core/src/ported/decompress/generated/`
- `safe/crates/jpeg-core/src/common/error.rs`
- `safe/crates/jpeg-core/src/common/icc.rs`
- `safe/crates/jpeg-core/src/common/registry.rs`
- `safe/crates/libjpeg-abi/src/lib.rs`
- `safe/crates/libjpeg-abi/src/decompress_exports.rs`
- `safe/tests/upstream_matrix.rs`
- `safe/tests/cve_regressions.rs`
- `safe/scripts/relink-original-objects.sh`
- `safe/scripts/original-object-groups.json`

## New Outputs
- full decode-side feature coverage, including progressive/arithmetic paths
- stable scan-limit and warning/fatal plumbing for later tool/TurboJPEG phases
- committed malformed-input regressions

## File Changes
- Modify `safe/crates/jpeg-core/src/ported/decompress/{jdphuff,jdarith,jdpostct,jquant1,jquant2,jdtrans,jdapistd}.rs`
- Modify `safe/crates/jpeg-core/src/ported/decompress/generated/*_translated.rs` as needed
- Modify `safe/crates/jpeg-core/src/common/{icc,registry,error}.rs`
- Modify `safe/crates/libjpeg-abi/src/{lib,decompress_exports}.rs`
- Modify `safe/tests/{upstream_matrix,cve_regressions}.rs`

## Implementation Details
- Port progressive Huffman decode, arithmetic decode, post-processing controllers, one-pass/two-pass quantizers, buffered-image mode, coefficient access, and marker save/processor APIs.
- Preserve `jpeg_crop_scanline()` and `jpeg_skip_scanlines()` semantics from upstream state-machine expectations.
- Encode the explicit CVE surfaces from `relevant_cves.json`, including scan skipping under quantization/merged-upsampling states and progressive scan/resource abuse.
- Keep decoder-side scan counting reusable, because later phases must expose the same policy through `TJFLAG_LIMITSCANS`, `tjbench -limitscans`, and `djpeg` / `jpegtran` switches.

## Verification Phases

### `check_advanced_decode_matrix`
- Phase ID: `check_advanced_decode_matrix`
- Type: `check`
- Bounce Target: `impl_advanced_decompress`
- Purpose: verify progressive/arithmetic decode, color quantization, buffered-image mode, coefficient APIs, and crop/skip behavior against the remaining upstream decode matrix.
- Commands:

```bash
cargo test --manifest-path safe/Cargo.toml -p safe --test upstream_matrix -- advanced-decode
cargo test --manifest-path safe/Cargo.toml -p safe --test upstream_matrix -- croptest
```

### `check_decode_cve_and_relink`
- Phase ID: `check_decode_cve_and_relink`
- Type: `check`
- Bounce Target: `impl_advanced_decompress`
- Purpose: verify CVE-derived regressions and decode-oriented original-object link compatibility.
- Commands:

```bash
cargo test --manifest-path safe/Cargo.toml -p safe --test cve_regressions
bash safe/scripts/relink-original-objects.sh --group decompress
```

### `check_advanced_software_tester`
- Phase ID: `check_advanced_software_tester`
- Type: `check`
- Bounce Target: `impl_advanced_decompress`
- Purpose: software-tester review of malformed-input handling, warning semantics, and crop/skip compatibility.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo test --manifest-path safe/Cargo.toml -p safe --test upstream_matrix -- advanced-decode
cargo test --manifest-path safe/Cargo.toml -p safe --test cve_regressions
```

### `check_advanced_senior_tester`
- Phase ID: `check_advanced_senior_tester`
- Type: `check`
- Bounce Target: `impl_advanced_decompress`
- Purpose: senior-tester review of scan-limit plumbing, coefficient-mode behavior, and whether the decoder now covers the security-sensitive edge cases explicitly called out by `relevant_cves.json`.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo test --manifest-path safe/Cargo.toml -p safe --test upstream_matrix -- croptest
bash safe/scripts/relink-original-objects.sh --group decompress
```

## Success Criteria
- Progressive, arithmetic, buffered-image, coefficient, crop, skip, and quantization decode paths are implemented with explicit regression coverage.
- Decoder-side CVE regressions are committed and scan-limit behavior is reusable by later tool and TurboJPEG phases.
- All four verifier phases pass with `impl_advanced_decompress` as their only bounce target.

## Git Commit Requirement
The implementer must commit all work for `impl_advanced_decompress` to git before yielding. Do not yield with unstaged or uncommitted changes.
