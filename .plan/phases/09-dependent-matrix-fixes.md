# Dependent-Application Compatibility Fixes and Reviewed Regression Closure

## Phase Name
Dependent-Application Compatibility Fixes and Reviewed Regression Closure

## Implement Phase ID
`impl_dependent_matrix_fixes`

## Preexisting Inputs
- `safe/Cargo.toml`
- `safe/target/dependent-matrix/summary.json`
- `safe/target/dependent-matrix/`
- `safe/scripts/run-dependent-regressions.sh`
- `safe/tests/dependent_regressions.rs`
- `safe/tests/fixtures/dependents/`
- `safe/crates/jpeg-core/src/`
- `safe/crates/libjpeg-abi/src/`
- `safe/crates/libturbojpeg-abi/src/`
- `safe/crates/jpeg-tools/src/`
- `safe/scripts/stage-install.sh`
- `safe/debian/`
- `dependents.json`
- `test-original.sh`

## New Outputs
- committed production fixes for each recorded dependent compatibility issue
- passing committed dependent regression suite
- `safe/target/dependent-matrix-fixed/summary.json`
- `safe/target/dependent-matrix-fixed/` logs and artifact bundles from the passing full Ubuntu 24.04 matrix rerun

## File Changes
- Modify the production files implicated by the recorded failures, including the relevant paths under:
  - `safe/crates/jpeg-core/src/**`
  - `safe/crates/libjpeg-abi/src/**`
  - `safe/crates/libturbojpeg-abi/src/**`
  - `safe/crates/jpeg-tools/src/**`
  - `safe/scripts/stage-install.sh`
  - `safe/debian/**`
- Modify `safe/tests/dependent_regressions.rs` and `safe/tests/fixtures/dependents/**` only to tighten or clarify the reproducer after the fix, not to remove coverage
- Modify `safe/scripts/run-dependent-regressions.sh` only if the reproducer runner itself needs to be hardened

## Implementation Details
- Start from the committed failure inventory produced in Phase 8; do not rerun the matrix and invent a new bug list.
- Fix the recorded issues in the narrowest correct production layer: core codec, ABI surface, packaged tools, staged install, or Debian packaging.
- Keep the regression suite from Phase 8 intact and make it pass; do not delete or mark cases ignored to "fix" the matrix.
- Use the structured report from Phase 8 to ensure each recorded failure is explicitly closed and traceable.

## Verification Phases

### `check_dependent_regressions_pass`
- Phase ID: `check_dependent_regressions_pass`
- Type: `check`
- Bounce Target: `impl_dependent_matrix_fixes`
- Purpose: verify that every committed dependent reproducer from Phase 8 now passes against the fixed implementation.
- Commands:

```bash
cargo test --manifest-path safe/Cargo.toml -p safe --test dependent_regressions
bash safe/scripts/run-dependent-regressions.sh --mode verify
```

### `check_dependent_matrix_full_harness`
- Phase ID: `check_dependent_matrix_full_harness`
- Type: `check`
- Bounce Target: `impl_dependent_matrix_fixes`
- Purpose: verify that the same Docker/app matrix now passes end to end through the existing root harness.
- Commands:

```bash
./test-original.sh --checks all --report-dir safe/target/dependent-matrix-fixed
jq -e 'all(.runtime[]; .status == "pass") and all(.compile[]; .status == "pass")' safe/target/dependent-matrix-fixed/summary.json
```

### `check_dependent_fix_software_tester`
- Phase ID: `check_dependent_fix_software_tester`
- Type: `check`
- Bounce Target: `impl_dependent_matrix_fixes`
- Purpose: software-tester review that the production fix actually closes the recorded regressions and does not silently drop or weaken them.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
cargo test --manifest-path safe/Cargo.toml -p safe --test dependent_regressions
bash safe/scripts/run-dependent-regressions.sh --mode verify
./test-original.sh --checks all --report-dir safe/target/dependent-matrix-fixed
```

### `check_dependent_fix_senior_tester`
- Phase ID: `check_dependent_fix_senior_tester`
- Type: `check`
- Bounce Target: `impl_dependent_matrix_fixes`
- Purpose: senior-tester review of root cause coverage, package/runtime blast radius, and whether the dependent fixes preserve the source/link/runtime compatibility contract instead of only masking symptoms.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
bash safe/scripts/run-dependent-regressions.sh --mode verify
./test-original.sh --checks all --report-dir safe/target/dependent-matrix-fixed
jq -e 'all(.runtime[]; .status == "pass") and all(.compile[]; .status == "pass")' safe/target/dependent-matrix-fixed/summary.json
```

## Success Criteria
- Every committed dependent reproducer from Phase 8 passes without deleting or weakening coverage.
- The same Docker/app matrix recorded in Phase 8 now passes end to end through the existing root harness.
- All four verifier phases pass with `impl_dependent_matrix_fixes` as their only bounce target.

## Git Commit Requirement
The implementer must commit all work for `impl_dependent_matrix_fixes` to git before yielding. Do not yield with unstaged or uncommitted changes.
