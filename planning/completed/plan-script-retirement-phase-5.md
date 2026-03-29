# Plan: Script Retirement Phase 5

Generated: 2026-03-28 19:39

## Status

- LastUpdated: 2026-03-28 19:39
- Objective: retire the next low-fanout validation PowerShell leaves by introducing native `ntk validation` entrypoints and then deleting the local wrappers safely.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior while keeping the planning ledger current and slice-oriented.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-5.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-4.md`
  - `planning/specs/completed/spec-script-retirement-phase-4.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- Current Slice Snapshot:
  - local `scripts/**/*.ps1` inventory is `132`
  - the three in-scope validation leaves are retired locally
  - `validate-all.ps1` now dispatches the in-scope checks through `ntk validation`
  - inventory and governance surfaces no longer require the deleted local wrapper paths

## Scope Summary

1. `scripts/validation/validate-routing-coverage.ps1`
2. `scripts/validation/validate-architecture-boundaries.ps1`
3. `scripts/validation/validate-audit-ledger.ps1`

This phase is complete only if:

- `ntk validation` exposes a native executable contract for each in-scope check
- `validate-all.ps1` stops requiring the local leaf path for those checks
- validation inventory and policy surfaces stop treating the deleted `.ps1` files as required local evidence
- the three leaves can be deleted safely and the retirement matrix/parity ledger reflect the executed state

## Ordered Tasks

### Task 1: Open The Native Validation CLI Boundary

Status: `[x]` Completed

- Add `ntk validation` entrypoints for:
  - `routing-coverage`
  - `architecture-boundaries`
  - `audit-ledger`
- Add CLI tests that prove each subcommand against a minimal repository fixture.
- ✓ [2026-03-28 19:39] Added `crates/cli/src/validation_commands.rs` plus CLI tests that cover the three native validation entrypoints.

### Task 2: Repoint Compatibility Orchestration

Status: `[x]` Completed

- Teach `scripts/validation/validate-all.ps1` to execute the three checks through the native `ntk validation` boundary instead of direct script paths.
- Preserve warning-only behavior for checks that support warning demotion.
- ✓ [2026-03-28 19:39] `validate-all.ps1` now executes `routing-coverage`, `architecture-boundaries`, and `audit-ledger` through native `ntk validation` subcommands, while preserving warning-only demotion for the checks that support it.

### Task 3: Clear Inventory And Policy Blockers

Status: `[x]` Completed

- Remove the three deleted wrappers from local required-file inventories and policy files.
- Update any release/operator documentation that still names the deleted wrapper path directly.
- ✓ [2026-03-28 19:39] Cleared the deleted wrapper paths from validation inventory and governance evidence, and updated release guidance to use `ntk validation audit-ledger --warning-only false`.

### Task 4: Retire The Validation Leaves

Status: `[x]` Completed

- Delete the three local `.ps1` leaves once the live consumer chain is clear.
- Update:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- ✓ [2026-03-28 19:39] Deleted `validate-routing-coverage.ps1`, `validate-architecture-boundaries.ps1`, and `validate-audit-ledger.ps1`, then rebaselined the retirement matrix and parity ledger to the new `132` script inventory.

### Task 5: Validate And Close The Slice

Status: `[x]` Completed

- Run the relevant CLI, validation crate, PowerShell wrapper, planning, and security validations.
- Archive the phase plan/spec when the slice is stable.
- ✓ [2026-03-28 19:39] Verified the slice with CLI tests, the full validation crate, targeted `validate-all` phase profile, `validate-instructions`, and supporting planning/security gates before archiving the phase.

## Validation Checklist

- `cargo test -p nettoolskit-cli validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- targeted `validate-all.ps1` phase profile for:
  - `validate-routing-coverage`
  - `validate-architecture-boundaries`
  - `validate-audit-ledger`
- `pwsh -File .\scripts\validation\validate-instructions.ps1`
- `pwsh -File .\scripts\validation\validate-planning-structure.ps1`
- `pwsh -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- `git diff --check`

## Outcome

- `scripts/validation/validate-routing-coverage.ps1`: retired locally
- `scripts/validation/validate-architecture-boundaries.ps1`: retired locally
- `scripts/validation/validate-audit-ledger.ps1`: retired locally
- The live local `scripts/**/*.ps1` estate is reduced from `135` to `132`.
- Remaining validation retirement now shifts from low-fanout leaves to the more central wrapper surfaces still coupled to suite orchestration and policy.