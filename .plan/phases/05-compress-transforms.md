# Compression, Coefficient Transcoding, and jpegtran Transform Core

## Phase Name
Compression, Coefficient Transcoding, and jpegtran Transform Core

## Implement Phase ID
`impl_compress_transforms`

## Preexisting Inputs
- `original/jcapimin.c`
- `original/jcapistd.c`
- `original/jcinit.c`
- `original/jcmaster.c`
- `original/jcparam.c`
- `original/jcmainct.c`
- `original/jcprepct.c`
- `original/jccolor.c`
- `original/jccolext.c`
- `original/jcsample.c`
- `original/jcdctmgr.c`
- `original/jfdctint.c`
- `original/jfdctfst.c`
- `original/jfdctflt.c`
- `original/jchuff.c`
- `original/jcphuff.c`
- `original/jcarith.c`
- `original/jcmarker.c`
- `original/jctrans.c`
- `original/transupp.c`
- `safe/crates/jpeg-core/src/ported/compress/`
- `safe/crates/jpeg-core/src/ported/transform/transupp.rs`
- `safe/crates/libjpeg-abi/src/lib.rs`
- `safe/crates/libjpeg-abi/src/common_exports.rs`
- `safe/crates/libjpeg-abi/src/decompress_exports.rs`
- `safe/tests/upstream_matrix.rs`
- `safe/scripts/relink-original-objects.sh`
- `safe/scripts/original-object-groups.json`

## New Outputs
- full Rust compression pipeline
- coefficient-transcode and lossless transform core used by `jpegtran` and TurboJPEG transforms
- completed encode/transcode rows in `safe/tests/upstream_matrix.rs`

## File Changes
- Modify `safe/crates/jpeg-core/src/ported/compress/{jcapimin,jcapistd,jcinit,jcmaster,jcparam,jcmainct,jcprepct,jccolor,jccolext,jcsample,jcdctmgr,jfdctint,jfdctfst,jfdctflt,jchuff,jcphuff,jcarith,jcmarker,jctrans,jccoefct}.rs`
- Modify `safe/crates/jpeg-core/src/ported/transform/transupp.rs`
- Modify `safe/crates/libjpeg-abi/src/{lib,common_exports,decompress_exports}.rs`
- Modify `safe/tests/upstream_matrix.rs`

## Implementation Details
- Port the compression lifecycle, parameter helpers, preprocessing/downsampling, forward DCTs, entropy encoders, progressive/arithmetic output, marker emission, raw-data input, and table-writing helpers.
- Port coefficient-transcode and transform support from `jctrans.c` and `transupp.c`, including crop/flip/rotate/transverse/perfect checks and marker-copy behavior.
- Preserve upstream quantization defaults, scan scripts, restart intervals, and output bytes well enough that the upstream MD5 matrix still matches.
- By the end of this phase, the core codec should no longer depend on a temporary reference backend for encode/decode internals.

## Verification Phases

### `check_compress_matrix`
- Phase ID: `check_compress_matrix`
- Type: `check`
- Bounce Target: `impl_compress_transforms`
- Purpose: verify the upstream encode/transcode MD5 matrix, including arithmetic/progressive output and lossless transform cases.
- Commands:

```bash
cargo test --manifest-path safe/Cargo.toml -p safe --test upstream_matrix -- encode-transcode
```

### `check_compress_relink`
- Phase ID: `check_compress_relink`
- Type: `check`
- Bounce Target: `impl_compress_transforms`
- Purpose: verify compression/transcode original objects still link and run against staged safe libraries.
- Commands:

```bash
bash safe/scripts/relink-original-objects.sh --group compress
```

### `check_compress_software_tester`
- Phase ID: `check_compress_software_tester`
- Type: `check`
- Bounce Target: `impl_compress_transforms`
- Purpose: software-tester review of encode defaults, transform behavior, and MD5-matrix completeness.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo test --manifest-path safe/Cargo.toml -p safe --test upstream_matrix -- encode-transcode
bash safe/scripts/relink-original-objects.sh --group compress
```

### `check_compress_senior_tester`
- Phase ID: `check_compress_senior_tester`
- Type: `check`
- Bounce Target: `impl_compress_transforms`
- Purpose: senior-tester review of quantization defaults, scan scripts, marker emission, and whether any temporary C bridge still remains in the core codec path.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo test --manifest-path safe/Cargo.toml -p safe --test upstream_matrix -- encode-transcode
bash safe/scripts/relink-original-objects.sh --group compress
```

## Success Criteria
- The Rust compressor, coefficient-transcode path, and lossless transform core are complete enough to satisfy the upstream encode/transcode MD5 matrix.
- The core codec no longer depends on a temporary reference backend for encode/decode internals.
- All four verifier phases pass with `impl_compress_transforms` as their only bounce target.

## Git Commit Requirement
The implementer must commit all work for `impl_compress_transforms` to git before yielding. Do not yield with unstaged or uncommitted changes.
