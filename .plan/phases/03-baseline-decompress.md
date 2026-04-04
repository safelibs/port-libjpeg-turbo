# Baseline Decompression Path and Baseline Decode Test Port

## Phase Name
Baseline Decompression Path and Baseline Decode Test Port

## Implement Phase ID
`impl_baseline_decompress`

## Preexisting Inputs
- `original/jdapimin.c`
- `original/jdapistd.c`
- `original/jdinput.c`
- `original/jdmarker.c`
- `original/jdhuff.c`
- `original/jdcoefct.c`
- `original/jdmainct.c`
- `original/jdmaster.c`
- `original/jdmerge.c`
- `original/jdsample.c`
- `original/jdcolor.c`
- `original/jdcolext.c`
- `original/jddctmgr.c`
- `original/jidctint.c`
- `original/jidctfst.c`
- `original/jidctflt.c`
- `original/jidctred.c`
- `original/CMakeLists.txt`
- `original/testimages/`
- `safe/crates/jpeg-core/src/ported/decompress/`
- `safe/crates/jpeg-core/src/ported/decompress/generated/`
- `safe/crates/libjpeg-abi/src/lib.rs`
- `safe/crates/libjpeg-abi/src/decompress_exports.rs`
- `safe/tests/upstream_matrix.rs`
- `safe/scripts/run-dependent-subset.sh`
- `dependents.json`

## New Outputs
- working baseline/sequential decompressor in Rust
- baseline rows in `safe/tests/upstream_matrix.rs`
- staged decode behavior good enough for early runtime dependents

## File Changes
- Modify `safe/crates/jpeg-core/src/ported/decompress/{jdapimin,jdapistd,jdinput,jdmarker,jdhuff,jdcoefct,jdmainct,jdmaster,jdmerge,jdsample,jdcolor,jdcolext,jddctmgr,jidctint,jidctfst,jidctflt,jidctred}.rs`
- Modify `safe/crates/jpeg-core/src/ported/decompress/generated/*_translated.rs` as needed
- Modify `safe/crates/libjpeg-abi/src/{lib,decompress_exports}.rs`
- Modify `safe/tests/upstream_matrix.rs`

## Implementation Details
- Port the baseline header-read to scanline-output flow, including marker parsing, sequential Huffman entropy decode, coefficient buffering, IDCT selection, range limiting, upsampling, and color conversion.
- Preserve observable output fields and decoder state transitions exactly enough for downstream callers and tests to rely on them.
- Keep the acceptance standard tied to the upstream MD5 outputs from `original/CMakeLists.txt`, not visual similarity or ad hoc new fixtures.

## Verification Phases

### `check_baseline_decode_matrix`
- Phase ID: `check_baseline_decode_matrix`
- Type: `check`
- Bounce Target: `impl_baseline_decompress`
- Purpose: verify sequential Huffman decode, marker processing, baseline IDCT selection, upsampling, and scanline output against the upstream MD5 matrix.
- Commands:

```bash
cargo test --manifest-path safe/Cargo.toml -p safe --test upstream_matrix -- baseline-decode
```

### `check_baseline_decode_dependents`
- Phase ID: `check_baseline_decode_dependents`
- Type: `check`
- Bounce Target: `impl_baseline_decompress`
- Purpose: verify a decode-heavy runtime subset against the staged safe libraries before the full dependent-harness sweep.
- Commands:

```bash
bash safe/scripts/run-dependent-subset.sh --checks runtime --only eog --only openjdk-17-jre-headless --only python3-pil --only tracker-extract --only libcamera-tools
```

### `check_baseline_software_tester`
- Phase ID: `check_baseline_software_tester`
- Type: `check`
- Bounce Target: `impl_baseline_decompress`
- Purpose: software-tester review of baseline-decode coverage, MD5 expectations, and staging/runtime behavior.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo test --manifest-path safe/Cargo.toml -p safe --test upstream_matrix -- baseline-decode
bash safe/scripts/run-dependent-subset.sh --checks runtime --only eog --only python3-pil
```

### `check_baseline_senior_tester`
- Phase ID: `check_baseline_senior_tester`
- Type: `check`
- Bounce Target: `impl_baseline_decompress`
- Purpose: senior-tester review of decoder state transitions, exposed helper symbols, and remaining bridge assumptions before advanced decode features land.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo test --manifest-path safe/Cargo.toml -p safe --test upstream_matrix -- baseline-decode
bash safe/scripts/run-dependent-subset.sh --checks runtime --only openjdk-17-jre-headless --only tracker-extract
```

## Success Criteria
- The baseline decode path is implemented in Rust with MD5-checked coverage in `safe/tests/upstream_matrix.rs`.
- Early decode-heavy runtime dependents succeed against the staged safe libraries.
- All four verifier phases pass with `impl_baseline_decompress` as their only bounce target.

## Git Commit Requirement
The implementer must commit all work for `impl_baseline_decompress` to git before yielding. Do not yield with unstaged or uncommitted changes.
