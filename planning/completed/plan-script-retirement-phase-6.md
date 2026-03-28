# Plan: Script Retirement Phase 6

Generated: 2026-03-28 20:19

## Status

- LastUpdated: 2026-03-28 20:30
- Objective: retire the next validation-owned local wrappers by exposing the missing native `ntk validation` boundaries and clearing the remaining compatibility-policy blockers in the same slice.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior while keeping versioned planning current and committing each stable retirement phase.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-6.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-5.md`
  - `planning/specs/completed/spec-script-retirement-phase-5.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- Current Slice Snapshot:
  - local `scripts/**/*.ps1` inventory is `129`
  - the in-scope validation behaviors are exposed through `ntk validation`
  - the three local wrapper leaves are retired and no live non-self local path reference remains

## Scope Summary

1. `scripts/validation/validate-powershell-standards.ps1`
2. `scripts/validation/validate-shared-script-checksums.ps1`
3. `scripts/validation/validate-warning-baseline.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for each in-scope check
- `validate-all.ps1` no longer requires the local wrapper paths for those checks
- policy, inventory, and release guidance stop treating the deleted `.ps1` files as required local evidence
- the three wrappers are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Open The Missing Native Validation CLI Boundary

Status: `[x]` Completed

- Add `ntk validation` entrypoints for:
  - `powershell-standards`
  - `shared-script-checksums`
  - `warning-baseline`
- Add CLI tests that prove each subcommand against deterministic repository fixtures.
- ✓ [2026-03-28 20:30] Added the three native `ntk validation` entrypoints and CLI tests covering deterministic fixtures for each command surface.

### Task 2: Repoint Compatibility Orchestration

Status: `[x]` Completed

- Teach `scripts/validation/validate-all.ps1` to execute the three in-scope checks through the native `ntk validation` boundary.
- Preserve `warning-only` behavior and the compatibility switches that still belong to suite orchestration:
  - `IncludeAllScripts`
  - `Strict`
  - `SkipScriptAnalyzer`
  - `DetailedOutput`
- ✓ [2026-03-28 20:30] `validate-all.ps1` now dispatches `powershell-standards`, `shared-script-checksums`, and `warning-baseline` through `ntk validation` while preserving suite-owned compatibility flags and warning-only demotion.

### Task 3: Clear Inventory, Policy, And Release Blockers

Status: `[x]` Completed

- Remove the deleted wrapper paths from:
  - `scripts/validation/validate-instructions.ps1`
  - `.github/governance/security-baseline.json`
  - `.github/governance/release-provenance.baseline.json`
  - `.github/policies/instruction-system.policy.json`
  - `.github/governance/release-governance.md`
- Keep check ids stable; only remove the local wrapper path as required evidence.
- ✓ [2026-03-28 20:30] Cleared the deleted wrapper paths from validation inventory, security/release provenance baselines, instruction policy, and release guidance without changing check ids.

### Task 4: Retire The Validation Leaves

Status: `[x]` Completed

- Delete the three local `.ps1` wrappers once the live consumer chain is clear.
- Rebaseline:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- ✓ [2026-03-28 20:30] Deleted the three local validation wrappers and rebaselined the retirement matrix/parity ledger to the new `129` script inventory.

### Task 5: Validate And Close The Slice

Status: `[x]` Completed

- Run the relevant CLI, validation crate, compatibility wrapper, planning, and security validations.
- Move the phase plan/spec to `completed` only when the slice is stable and the worktree returns clean.
- ✓ [2026-03-28 20:30] Verified the slice with CLI tests, the full validation crate, `validate-instructions`, targeted `validate-all` phase profiles in enforcing and warning-only modes, planning validation, and the Rust vulnerability audit before archiving the phase.

## Validation Checklist

- `cargo test -p nettoolskit-cli validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- targeted `validate-all.ps1` phase profile for:
  - `validate-powershell-standards`
  - `validate-shared-script-checksums`
  - `validate-warning-baseline`
- `pwsh -File .\scripts\validation\validate-instructions.ps1`
- `pwsh -File .\scripts\validation\validate-planning-structure.ps1`
- `pwsh -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- `git diff --check`

## Checkpoints

- Commit checkpoint 1: `feat(validation): add native standards and checksum validation commands`
- Commit checkpoint 2: `refactor(validation): retire standards, checksum, and warning wrappers`
- Closeout checkpoint: `docs(planning): close script retirement phase 6`

## Outcome Target

- `scripts/validation/validate-powershell-standards.ps1`: retired locally
- `scripts/validation/validate-shared-script-checksums.ps1`: retired locally
- `scripts/validation/validate-warning-baseline.ps1`: retired locally
- The live local `scripts/**/*.ps1` estate is reduced from `132` to `129`.