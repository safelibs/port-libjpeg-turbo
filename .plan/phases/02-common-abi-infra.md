# Common ABI Infrastructure: Layouts, Error Paths, Memory Manager, and Callback Boundary

## Phase Name
Common ABI Infrastructure: Layouts, Error Paths, Memory Manager, and Callback Boundary

## Implement Phase ID
`impl_common_abi_infra`

## Preexisting Inputs
- `original/jpeglib.h`
- `original/jmorecfg.h`
- `original/jpegint.h`
- `original/jerror.h`
- `original/jmemsys.h`
- `original/jcomapi.c`
- `original/jerror.c`
- `original/jutils.c`
- `original/jmemmgr.c`
- `original/jmemnobs.c`
- `original/jdatasrc.c`
- `original/jdatadst.c`
- `original/jcicc.c`
- `original/jdicc.c`
- `original/jcstest.c`
- `original/strtest.c`
- `safe/crates/ffi-types/src/lib.rs`
- `safe/crates/jpeg-core/src/common/`
- `safe/crates/libjpeg-abi/src/common_exports.rs`
- `safe/crates/libjpeg-abi/src/lib.rs`
- `safe/tests/layout_ctest.rs`
- `safe/tests/compat_smoke.rs`
- `safe/scripts/run-compat-fixtures.sh`
- `safe/scripts/run-debian-autopkgtests.sh`
- `safe/scripts/relink-original-objects.sh`
- `safe/scripts/original-object-groups.json`
- `safe/c_shim/error_bridge.c`

## New Outputs
- exact ABI/layout parity for public and installed internal structs
- stable `error_exit` / `longjmp` boundary
- working pool allocator, source/destination managers, and ICC helpers
- ported `jcstest` / `strtest` coverage that remains invokable from the root package

## File Changes
- Modify `safe/crates/ffi-types/src/lib.rs`
- Modify `safe/crates/jpeg-core/src/common/{error,memory,utils,registry,source_dest,icc}.rs`
- Modify `safe/crates/libjpeg-abi/src/{common_exports,lib}.rs`
- Modify `safe/tests/{layout_ctest,compat_smoke}.rs`
- Modify `safe/scripts/{run-compat-fixtures,run-debian-autopkgtests}.sh`
- Modify `safe/c_shim/error_bridge.c` only if the C boundary remains necessary

## Implementation Details
- Match the layouts of every public struct applications can allocate or inspect, plus the installed internal controller structs from `original/jpegint.h`.
- Lock down `jpeg_std_error`, create/destroy helpers, stdio/memory source-destination managers, ICC profile helpers, and the no-backing-store memory manager semantics.
- Preserve `jpeg_std_message_table`, `msg_code`, and formatting behavior so downstream diagnostics and wrappers stay stable.
- Keep the original-fixture cross-checks alive; the Rust tests are not a substitute for compiling the upstream C fixtures against safe headers/libraries.

## Verification Phases

### `check_common_layouts`
- Phase ID: `check_common_layouts`
- Type: `check`
- Bounce Target: `impl_common_abi_infra`
- Purpose: verify exact size, alignment, offset, constant, and callback-table parity for the public and installed internal ABI.
- Commands:

```bash
cargo test --manifest-path safe/Cargo.toml -p safe --test layout_ctest
```

### `check_common_runtime_boundary`
- Phase ID: `check_common_runtime_boundary`
- Type: `check`
- Bounce Target: `impl_common_abi_infra`
- Purpose: verify lifecycle, memory-manager, source/destination-manager, ICC, and `setjmp`/`longjmp` compatibility across Rust and original-C fixtures.
- Commands:

```bash
cargo test --manifest-path safe/Cargo.toml -p safe --test compat_smoke
bash safe/scripts/run-compat-fixtures.sh jcstest strtest
bash safe/scripts/run-debian-autopkgtests.sh
bash safe/scripts/relink-original-objects.sh --group smoke
```

### `check_common_software_tester`
- Phase ID: `check_common_software_tester`
- Type: `check`
- Bounce Target: `impl_common_abi_infra`
- Purpose: software-tester review of ABI layout coverage, fixture parity, and callback-boundary robustness.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo test --manifest-path safe/Cargo.toml -p safe --test layout_ctest
cargo test --manifest-path safe/Cargo.toml -p safe --test compat_smoke
bash safe/scripts/run-compat-fixtures.sh jcstest strtest
```

### `check_common_senior_tester`
- Phase ID: `check_common_senior_tester`
- Type: `check`
- Bounce Target: `impl_common_abi_infra`
- Purpose: senior-tester review of the highest-risk ABI boundary (`error_exit`/`longjmp`, open structs, memory pools) and whether coverage is sufficient before codec internals are touched.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo test --manifest-path safe/Cargo.toml -p safe --test layout_ctest
bash safe/scripts/run-debian-autopkgtests.sh
bash safe/scripts/relink-original-objects.sh --group smoke
```

## Success Criteria
- Public and installed-internal ABI layouts, callback tables, and exported helpers match the upstream headers closely enough for open-struct callers and fixture parity.
- Error handling, memory management, ICC helpers, and source or destination manager behavior remain compatible across Rust tests, original fixtures, and Debian autopkgtests.
- All four verifier phases pass with `impl_common_abi_infra` as their only bounce target.

## Git Commit Requirement
The implementer must commit all work for `impl_common_abi_infra` to git before yielding. Do not yield with unstaged or uncommitted changes.
