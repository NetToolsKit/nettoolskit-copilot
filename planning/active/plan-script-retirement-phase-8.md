# Plan: Script Retirement Phase 8

Generated: 2026-03-28 21:06

## Status

- LastUpdated: 2026-03-28 21:06
- Objective: retire the next low-fanout validation wrappers by exposing native `ntk validation` executable boundaries for repository policy and agent alignment/permission checks, then deleting the local PowerShell leaves only after compatibility and governance surfaces stop requiring their paths.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keep planning current, and commit each stable retirement phase.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/active/spec-script-retirement-phase-8.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-7.md`
  - `planning/specs/completed/spec-script-retirement-phase-7.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- Current Slice Snapshot:
  - local `scripts/**/*.ps1` inventory is `127`
  - the in-scope Rust implementations already exist under `crates/commands/validation`
  - the missing executable boundary is the CLI/compatibility layer, not the core validation behavior

## Scope Summary

1. `scripts/validation/validate-policy.ps1`
2. `scripts/validation/validate-agent-skill-alignment.ps1`
3. `scripts/validation/validate-agent-permissions.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for each in-scope check
- `validate-all.ps1` no longer requires the local wrapper paths for those checks
- policy, inventory, and orchestration surfaces stop treating the deleted `.ps1` files as required local evidence
- the three wrappers are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Open The Missing Native Validation CLI Boundary

Status: `[ ]` Pending

- Add `ntk validation` entrypoints for:
  - `policy`
  - `agent-skill-alignment`
  - `agent-permissions`
- Add CLI tests that prove each subcommand against deterministic repository fixtures.

### Task 2: Repoint Compatibility Orchestration

Status: `[ ]` Pending

- Teach `scripts/validation/validate-all.ps1` to execute the three in-scope checks through the native `ntk validation` boundary.
- Preserve `warning-only` behavior and the compatibility options that still belong to suite orchestration.

### Task 3: Clear Inventory, Policy, And Orchestration Blockers

Status: `[ ]` Pending

- Remove the deleted wrapper paths from:
  - `scripts/validation/validate-instructions.ps1`
  - `.github/governance/release-provenance.baseline.json`
  - `.github/policies/instruction-system.policy.json`
  - any provider-authored docs/examples that still encode the wrapper paths
  - orchestration/runtime consumers that only need the executable contract
- Keep check ids stable; only remove the local wrapper path as required evidence.

### Task 4: Retire The Validation Leaves

Status: `[ ]` Pending

- Delete the three local `.ps1` wrappers once the live consumer chain is clear.
- Rebaseline:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`

### Task 5: Validate And Close The Slice

Status: `[ ]` Pending

- Run the relevant CLI, validation crate, compatibility wrapper, planning, and security validations.
- Record both warning-only proof and enforcing proof for the focused `validate-all` phase profile.
- Move the phase plan/spec to `completed` only when the slice is stable and the worktree returns clean.

## Validation Checklist

- `cargo test -p nettoolskit-cli validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- targeted crate/CLI tests for:
  - `policy`
  - `agent-skill-alignment`
  - `agent-permissions`
- `pwsh -File .\scripts\validation\validate-instructions.ps1`
- `pwsh -File .\scripts\validation\validate-planning-structure.ps1`
- `pwsh -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- targeted `validate-all.ps1` phase profile for:
  - `validate-policy`
  - `validate-agent-skill-alignment`
  - `validate-agent-permissions`
- `git diff --check`

## Checkpoints

- Commit checkpoint 1: `feat(validation): add native policy and agent alignment commands`
- Commit checkpoint 2: `refactor(validation): retire policy and agent alignment wrappers`
- Closeout checkpoint: `docs(planning): close script retirement phase 8`

## Outcome Target

- `scripts/validation/validate-policy.ps1`: retired locally
- `scripts/validation/validate-agent-skill-alignment.ps1`: retired locally
- `scripts/validation/validate-agent-permissions.ps1`: retired locally
- The live local `scripts/**/*.ps1` estate is reduced from `127` to `124`.