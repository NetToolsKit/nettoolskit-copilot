# Phase 17: Runtime Diagnostics Wrapper Retirement

Generated: 2026-03-29 08:01

## Status

- LastUpdated: 2026-03-29 08:26
- Objective: replace the local `doctor.ps1` and `healthcheck.ps1` compatibility wrappers with native `ntk runtime` diagnostics entrypoints, then delete the wrappers after the live consumer chain is repointed in the same slice.
- Normalized Request: continue the aggressive PowerShell-to-Rust retirement flow, keep planning updated, and keep committing each stable phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-17.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-16.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/runtime/doctor.ps1`
  - `scripts/runtime/healthcheck.ps1`
  - `crates/commands/runtime/src/diagnostics/doctor.rs`
  - `crates/commands/runtime/src/diagnostics/healthcheck.rs`

## Scope Summary

1. `scripts/runtime/doctor.ps1`
2. `scripts/runtime/healthcheck.ps1`
3. native `ntk runtime doctor` plus the existing `ntk runtime healthcheck` surface
4. live consumer repoints in runtime scripts, runbooks, policy baselines, README surfaces, and retained parity fixtures

This phase is complete only if:

- `ntk runtime doctor` and `ntk runtime healthcheck` are the canonical operator-facing diagnostics commands
- shell-owned consumers stop requiring the deleted wrapper paths
- repository docs, policy evidence, and retained parity fixtures stop treating the wrappers as canonical
- both diagnostics wrappers are deleted safely in the same slice

## Ordered Tasks

### Task 1: Freeze The Native Diagnostics Boundary

Status: `[x]` Completed

- Expose `doctor` through `ntk runtime`.
- Confirm `healthcheck` remains the canonical audit and drift summary path.

### Task 2: Repoint Live Runtime Consumers

Status: `[x]` Completed

- Repoint `install.ps1`, `self-heal.ps1`, and `validate-stage.ps1` to the native diagnostics boundary.
- Repoint runtime runbooks, authored skill surfaces, and retained parity fixtures away from the wrapper paths.

### Task 3: Delete The Local Diagnostics Wrappers

Status: `[x]` Completed

- Delete `scripts/runtime/doctor.ps1`.
- Delete `scripts/runtime/healthcheck.ps1`.

### Task 4: Rebaseline Phase Evidence And Archive

Status: `[x]` Completed

- Update `planning/completed/script-retirement-safety-matrix.md`.
- Update `planning/completed/rust-script-parity-ledger.md`.
- Move the plan/spec to completed only after the validations prove the phase result.

## Validation Checklist

- [x] `cargo test -p nettoolskit-runtime --quiet`
- [x] `cargo test -p nettoolskit-cli --test test_suite runtime_commands_tests --quiet`
- [x] targeted native `ntk runtime doctor` proof
- [x] targeted native `ntk runtime healthcheck` proof
- [x] `& .\\.build\\target\\debug\\ntk.exe validation planning-structure --repo-root . --warning-only false`
- [x] `& .\\.build\\target\\debug\\ntk.exe validation instructions --repo-root . --warning-only false`
- [x] `pwsh -NoProfile -File .\\scripts\\security\\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- [x] `git diff --check`

## Risks And Fallbacks

- `install.ps1` currently models child steps as script invocations only, so the slice may need a native-binary step contract before healthcheck can leave the local script layer safely.
- Retained runtime parity fixtures still exercise compatibility narratives, so the phase must preserve intent while replacing direct `.ps1` references with native command evidence.
- Skill and runbook surfaces are user-facing; they must stay operator-usable after the wrapper paths disappear.

## Closeout Expectations

- Commit implementation and planning closeout separately.
- Archive the phase only after both wrappers are deleted and the same-slice consumer repoints plus validation evidence are complete.

## Executed Result

- `ntk runtime doctor` and `ntk runtime healthcheck` are now the canonical operator-facing diagnostics commands.
- `install.ps1`, `self-heal.ps1`, and `validate-stage.ps1` no longer depend on `scripts/runtime/doctor.ps1` or `scripts/runtime/healthcheck.ps1`.
- The two local diagnostics wrappers were deleted after the native CLI contract, tests, and operator smoke checks proved stable.
- The live local `scripts/**/*.ps1` estate dropped from `104` to `102`.