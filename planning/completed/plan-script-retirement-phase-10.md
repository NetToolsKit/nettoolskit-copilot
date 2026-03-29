# Phase 10: Validation Wrapper Retirement - Compatibility Lifecycle and Dotnet Standards

Generated: 2026-03-28 22:35

## Status

- LastUpdated: 2026-03-28 22:13
- Objective: retire the low-fanout validation wrappers for `validate-compatibility-lifecycle-policy` and `validate-dotnet-standards` by exposing native `ntk validation` executable boundaries, repointing the remaining validation and policy consumers, and deleting the local PowerShell leaves.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keep planning current, and commit each stable retirement phase.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-10.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-9.md`
  - `planning/specs/completed/spec-script-retirement-phase-9.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/policy/compatibility_lifecycle_policy.rs`
  - `crates/commands/validation/src/standards/dotnet_standards.rs`
  - `scripts/tests/runtime/compatibility-lifecycle.tests.ps1`
- Current Slice Snapshot:
  - `scripts/**/*.ps1` inventory is `119`
  - native Rust owners and executable CLI boundaries now exist for both target checks
  - the remaining consumer chain no longer requires the two local wrapper paths

## Scope Summary

1. `scripts/validation/validate-compatibility-lifecycle-policy.ps1`
2. `scripts/validation/validate-dotnet-standards.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for both checks
- `validate-all.ps1` no longer requires the two local wrapper paths
- policy and validation inventory evidence stop treating the deleted `.ps1` files as canonical
- the compatibility runtime test no longer shells into the deleted validation wrapper
- both wrappers are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Freeze The Phase-10 Native Validation Contract

Status: `[x]` Completed

- Register the phase-10 design intent in the spec and lock the cutover acceptance criteria.
- Confirm the native Rust owners and the exact CLI contract for:
  - `compatibility-lifecycle-policy`
  - `dotnet-standards`

### Task 2: Add The Missing Native CLI Boundaries

Status: `[x]` Completed

- Extend `ntk validation` with executable native contracts for:
  - `compatibility-lifecycle-policy`
  - `dotnet-standards`
- Keep CLI arguments aligned with the current behavior:
  - `compatibility-lifecycle-policy`: repo root, compatibility path, warning-only, detailed output
  - `dotnet-standards`: repo root, template directory
- Add or update focused CLI tests for both subcommands.

### Task 3: Retire The Two Wrapper Leaves And Repoint Their Consumers

Status: `[x]` Completed

- Repoint the remaining local consumers that still encode wrapper paths:
  - `scripts/validation/validate-all.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/tests/runtime/compatibility-lifecycle.tests.ps1`
  - `.github/policies/instruction-system.policy.json`
- Delete:
  - `scripts/validation/validate-compatibility-lifecycle-policy.ps1`
  - `scripts/validation/validate-dotnet-standards.ps1`

### Task 4: Rebaseline Planning Evidence And Archive The Phase

Status: `[x]` Completed

## Execution Outcome

- Added native `ntk validation` executable boundaries for:
  - `compatibility-lifecycle-policy`
  - `dotnet-standards`
- Repointed:
  - `scripts/validation/validate-all.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `.github/policies/instruction-system.policy.json`
  - `scripts/tests/runtime/compatibility-lifecycle.tests.ps1`
- Deleted:
  - `scripts/validation/validate-compatibility-lifecycle-policy.ps1`
  - `scripts/validation/validate-dotnet-standards.ps1`
- Confirmed the local `scripts/**/*.ps1` estate fell from `121` to `119`.
- Focused `validate-all` proof for this phase produced:
  - enforcing: executed the native checks correctly and failed only on pre-existing `COMPATIBILITY.md` policy debt in the repository (`N/A` mixed with dates in one lifecycle row)
  - warning-only: passed with both checks routed through the native executable contract

## Validation Checklist

- `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- `pwsh -File .\scripts\validation\validate-instructions.ps1`
- `pwsh -File .\scripts\tests\runtime\compatibility-lifecycle.tests.ps1`
- `pwsh -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- focused `validate-all.ps1` phase-10 profile for:
  - `validate-compatibility-lifecycle-policy`
  - `validate-dotnet-standards`
- `git diff --check`

## Risks And Fallbacks

- The enforcing validation proof still surfaces real repository debt in `COMPATIBILITY.md`; that debt must be addressed in a separate content-fix slice, not by restoring the retired wrapper.
- The next retirement phase should keep the same pattern of replacing direct wrapper evidence in policy and inventory surfaces before deletion.
- Runtime parity tests remain a live blocker type; whenever a wrapper is still called directly from `scripts/tests/runtime`, the parity harness must move in the same slice or the retirement will be incomplete.

## Closeout Expectations

- This plan is now archived because the wrapper deletions and focused validations are materially complete.