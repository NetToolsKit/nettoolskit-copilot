# Plan: Script Retirement Phase 7

Generated: 2026-03-28 20:49

## Status

- LastUpdated: 2026-03-28 20:59
- Objective: retire the next security-focused validation wrappers by exposing the missing native `ntk validation` executable boundaries, repointing compatibility orchestration, and deleting the local leaves only after policy and release surfaces stop requiring their paths.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior while keeping versioned planning current and committing each stable retirement phase.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-7.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-6.md`
  - `planning/specs/completed/spec-script-retirement-phase-6.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- Current Slice Snapshot:
  - local `scripts/**/*.ps1` inventory is `127`
  - the in-scope validation behaviors are exposed through `ntk validation`
  - the two local wrapper leaves are retired and no live non-planning local path reference remains

## Scope Summary

1. `scripts/validation/validate-security-baseline.ps1`
2. `scripts/validation/validate-supply-chain.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for each in-scope check
- `validate-all.ps1` no longer requires the local wrapper paths for those checks
- policy, inventory, and release guidance stop treating the deleted `.ps1` files as required local evidence
- the two wrappers are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Open The Missing Native Validation CLI Boundary

Status: `[x]` Completed

- Add `ntk validation` entrypoints for:
  - `security-baseline`
  - `supply-chain`
- Add CLI tests that prove each subcommand against deterministic repository fixtures.
- ✓ [2026-03-28 20:49] Added the two native `ntk validation` entrypoints and CLI tests covering deterministic fixtures for each command surface.

### Task 2: Repoint Compatibility Orchestration

Status: `[x]` Completed

- Teach `scripts/validation/validate-all.ps1` to execute the two in-scope checks through the native `ntk validation` boundary.
- Preserve `warning-only` behavior and the compatibility switches that still belong to suite orchestration.
- ✓ [2026-03-28 20:49] `validate-all.ps1` now dispatches `security-baseline` and `supply-chain` through `ntk validation` while preserving suite-owned warning-only behavior.

### Task 3: Clear Inventory, Policy, And Release Blockers

Status: `[x]` Completed

- Remove the deleted wrapper paths from:
  - `scripts/validation/validate-instructions.ps1`
  - `.github/governance/security-baseline.json`
  - `.github/governance/release-provenance.baseline.json`
  - `.github/policies/instruction-system.policy.json`
  - `.github/governance/release-governance.md`
  - the affected provider-authored skill examples that reference the native validation workflow
- Keep check ids stable; only remove the local wrapper path as required evidence.
- ✓ [2026-03-28 20:59] Cleared the deleted wrapper paths from validation inventory, security/release provenance baselines, instruction policy, release guidance, and provider-authored skill examples without changing check ids, and replaced the missing wrapper evidence with the native Rust implementation files and CLI/orchestration wiring paths.

### Task 4: Retire The Validation Leaves

Status: `[x]` Completed

- Delete the two local `.ps1` wrappers once the live consumer chain is clear.
- Rebaseline:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- ✓ [2026-03-28 20:49] Deleted the two local validation wrappers and rebaselined the retirement matrix/parity ledger to the new `127` script inventory.

### Task 5: Validate And Close The Slice

Status: `[x]` Completed

- Run the relevant CLI, validation crate, compatibility wrapper, planning, and security validations.
- Record both warning-only proof and enforcing proof for the focused `validate-all` phase profile.
- Move the phase plan/spec to `completed` only when the slice is stable and the worktree returns clean.
- ✓ [2026-03-28 20:59] Verified the slice with CLI tests, the full validation crate, post-review regression coverage for `security-baseline` multi-match scanning and `supply-chain` missing license-evidence paths, `validate-instructions`, planning validation, the Rust vulnerability audit, a focused warning-only `validate-all` profile that passed, and a focused enforcing `validate-all` profile that correctly failed on pre-existing `security-baseline` repository debt (`CODEOWNERS` and `.githooks/*`), proving native routing without introducing a cutover regression.

## Validation Checklist

- `cargo test -p nettoolskit-cli validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- `cargo test -p nettoolskit-validation --test test_suite security_baseline -- --nocapture`
- `cargo test -p nettoolskit-validation --test test_suite supply_chain -- --nocapture`
- `cargo test -p nettoolskit-cli --test test_suite validation_security_baseline -- --nocapture`
- `cargo test -p nettoolskit-cli --test test_suite validation_supply_chain -- --nocapture`
- `pwsh -File .\scripts\validation\validate-instructions.ps1`
- `pwsh -File .\scripts\validation\validate-planning-structure.ps1`
- `pwsh -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- targeted `validate-all.ps1` phase profile for:
  - `validate-security-baseline`
  - `validate-supply-chain`
  - warning-only mode: passed
  - enforcing mode: routed correctly and exposed pre-existing `security-baseline` hygiene debt
- `git diff --check`

## Checkpoints

- Commit checkpoint 1: `feat(validation): add native security baseline and supply chain commands`
- Commit checkpoint 2: `refactor(validation): retire security baseline and supply chain wrappers`
- Closeout checkpoint: `docs(planning): close script retirement phase 7`

## Outcome Target

- `scripts/validation/validate-security-baseline.ps1`: retired locally
- `scripts/validation/validate-supply-chain.ps1`: retired locally
- The live local `scripts/**/*.ps1` estate is reduced from `129` to `127`.