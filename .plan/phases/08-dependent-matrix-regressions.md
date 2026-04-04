# Dependent-Application Docker Matrix and Regression Capture

## Phase Name
Dependent-Application Docker Matrix and Regression Capture

## Implement Phase ID
`impl_dependent_matrix_regressions`

## Preexisting Inputs
- `dependents.json`
- `test-original.sh`
- `safe/debian/`
- `safe/java/`
- `safe/scripts/run-dependent-subset.sh`
- `safe/scripts/run-progs-smoke.sh`
- `safe/scripts/run-java-tests.sh`
- `safe/stage/usr/`
- `safe/stage/usr/share/java/turbojpeg.jar`
- `safe/debian/libjpeg-turbo8/`
- `safe/debian/libjpeg-turbo8-dev/`
- `safe/debian/libturbojpeg/`
- `safe/debian/libturbojpeg0-dev/`
- `safe/debian/libjpeg-turbo-progs/`
- `safe/debian/libturbojpeg-java/`

## New Outputs
- `safe/target/dependent-matrix/summary.json`
- `safe/target/dependent-matrix/` per-row logs and artifact bundles referenced by `summary.json`
- committed `safe/scripts/run-dependent-regressions.sh`
- committed `safe/tests/dependent_regressions.rs`
- committed `safe/tests/fixtures/dependents/`
- reporting hooks in `test-original.sh` and any committed case metadata needed to map failures to reproducible rows

## File Changes
- Modify `test-original.sh` to add structured reporting such as `--report-dir`
- Modify `safe/scripts/run-dependent-subset.sh` if shared reporting or fixture helpers are needed
- Create or modify `safe/scripts/run-dependent-regressions.sh`
- Create or modify `safe/tests/dependent_regressions.rs`
- Create or modify `safe/tests/fixtures/dependents/**`
- Optionally create or modify committed case metadata adjacent to the dependent regression fixtures

## Implementation Details
- Reuse the existing app inventory from `dependents.json`; do not create a second list of "twelve apps."
- Consume the package and harness artifacts produced by Phase 7 in place, including the staged `/usr` tree, the Java jar, the package payload trees, and the switched root harness. Do not substitute those dependencies with a fresh source-file rediscovery pass.
- Run the full compile/runtime matrix against safe packages using the existing container flow in `test-original.sh`.
- Extend the harness only enough to emit a machine-readable report mapping each dependent row to pass/fail status, relevant logs, and the invoked command or fixture.
- `--report-dir` must write `summary.json` with top-level `runtime` and `compile` arrays. Each emitted row must include at least the authoritative dependent name or source package, `status` (`pass`, `fail`, or `skipped`), the executed command or fixture identifier, and a path to the captured log or artifact bundle.
- When `--report-dir` is supplied, `test-original.sh` must finish writing `summary.json` and any per-row logs before returning. It must still exit non-zero if any selected row fails, so later fix and final phases can rerun the same command without `|| true`.
- For every failing row, add a committed reproducer:
  - preferably a narrow root-package test in `safe/tests/dependent_regressions.rs` when the failure can be reproduced directly against the staged library or staged tools
  - otherwise an app-driven reproducer in `safe/tests/fixtures/dependents/**` driven by `safe/scripts/run-dependent-regressions.sh`
- This phase must not fix production code. Its commit should contain only harness, test, fixture, metadata, or reporting changes needed to preserve the bug as a reproducible artifact.

## Verification Phases

### `check_dependent_matrix_capture`
- Phase ID: `check_dependent_matrix_capture`
- Type: `check`
- Bounce Target: `impl_dependent_matrix_regressions`
- Purpose: verify that the existing Ubuntu 24.04 Docker/app matrix was run through `test-original.sh`, that it exercised the authoritative dependent inventory, and that it emitted structured failure artifacts.
- Commands:

```bash
./test-original.sh --checks all --report-dir safe/target/dependent-matrix || true
test -f safe/target/dependent-matrix/summary.json
jq -e '(.runtime | length) >= 12 and (.compile | length) >= 8' safe/target/dependent-matrix/summary.json
```

### `check_dependent_regressions_reproduce`
- Phase ID: `check_dependent_regressions_reproduce`
- Type: `check`
- Bounce Target: `impl_dependent_matrix_regressions`
- Purpose: verify that every failing matrix row has been turned into a committed reproducer before any production fix is attempted.
- Commands:

```bash
bash safe/scripts/run-dependent-regressions.sh --mode reproduce
```

### `check_dependent_capture_software_tester`
- Phase ID: `check_dependent_capture_software_tester`
- Type: `check`
- Bounce Target: `impl_dependent_matrix_regressions`
- Purpose: software-tester review that the regression-capture commit contains only harness/tests/fixtures/reporting changes and that each reproducer is actionable.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
./test-original.sh --checks all --report-dir safe/target/dependent-matrix || true
bash safe/scripts/run-dependent-regressions.sh --mode reproduce
```

### `check_dependent_capture_senior_tester`
- Phase ID: `check_dependent_capture_senior_tester`
- Type: `check`
- Bounce Target: `impl_dependent_matrix_regressions`
- Purpose: senior-tester review that no production fixes leaked into the regression-capture commit and that the committed reproducers cover every reported failure instead of only the easiest ones.
- Commands:

```bash
git show --stat --name-only --format=fuller HEAD
./test-original.sh --checks all --report-dir safe/target/dependent-matrix || true
test -f safe/target/dependent-matrix/summary.json
bash safe/scripts/run-dependent-regressions.sh --mode reproduce
```

## Success Criteria
- The authoritative dependent matrix runs through the existing root harness and emits `safe/target/dependent-matrix/summary.json` plus per-row logs or artifacts.
- Every failing matrix row is preserved as a committed reproducer before any production fix phase begins.
- The capture commit contains no production-code changes.
- All four verifier phases pass with `impl_dependent_matrix_regressions` as their only bounce target.

## Git Commit Requirement
The implementer must commit all work for `impl_dependent_matrix_regressions` to git before yielding. Do not yield with unstaged or uncommitted changes.
