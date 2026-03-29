# Phase 14: Validation Wrapper Retirement - Instruction Entry Leaves

Generated: 2026-03-28 23:59

## Status

- LastUpdated: 2026-03-28 23:59
- Objective: retire the remaining instruction-entry validation wrappers by promoting the native CLI contract for instruction validation, repointing the remaining consumers, and deleting the local PowerShell leaves.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keep planning current, and commit each stable retirement phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/active/spec-script-retirement-phase-14.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-13.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/instruction_graph/instructions.rs`
  - `crates/commands/validation/src/governance/routing_coverage.rs`
  - `crates/commands/validation/src/orchestration/validate_all.rs`
- Current Slice Snapshot:
  - `scripts/**/*.ps1` inventory should fall from `109` to `107`
  - `scripts/validation/*.ps1` inventory should fall from `5` to `3`
  - native Rust owners already exist for `validate-instructions` and the routing golden coverage that makes `test-routing-selection` redundant
  - the remaining gap is executable CLI ownership plus local consumer cutover

## Scope Summary

1. `scripts/validation/validate-instructions.ps1`
2. `scripts/validation/test-routing-selection.ps1`

This phase is complete only if:

- `ntk validation` exposes an executable native contract for `instructions`
- `validate-all.ps1` and `validate-stage.ps1` stop requiring `validate-instructions.ps1`
- authored templates, runbooks, policies, and provider guidance stop treating `validate-instructions.ps1` as canonical
- `test-routing-selection.ps1` has no remaining live local consumer after `validate-instructions.ps1` is removed
- the two wrapper leaves are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Freeze The Phase-14 Contract

Status: `[ ]` Pending

- Lock the phase-14 design intent in the spec and confirm the acceptance criteria.
- Confirm the native Rust owners and exact CLI contracts for:
  - `instructions`
  - `routing-coverage`

### Task 2: Add The Missing Native CLI Boundary

Status: `[ ]` Pending

- Extend `ntk validation` with an executable native `instructions` contract.
- Keep CLI arguments aligned with the existing behavior:
  - `instructions`: repo root, warning-only
- Add or update focused CLI tests for:
  - `validation instructions`
  - `validation all` consuming the native instruction contract

### Task 3: Repoint Consumers And Delete The Two Leaves

Status: `[ ]` Pending

- Repoint live consumers that still encode `validate-instructions.ps1`:
  - `scripts/validation/validate-all.ps1`
  - `scripts/orchestration/stages/validate-stage.ps1`
  - `.github/governance/release-governance.md`
  - `.github/runbooks/README.md`
  - `.github/policies/instruction-system.policy.json`
  - `.github/governance/template-standards.baseline.json`
  - `.github/templates/github-change-checklist-template.md`
  - `definitions/shared/templates/github-change-checklist-template.md`
  - `.github/instructions/copilot-instruction-creation.instructions.md`
  - `definitions/shared/instructions/copilot-instruction-creation.instructions.md`
  - `definitions/providers/vscode/workspace/README.md`
- Delete `test-routing-selection.ps1` only after confirming `validate-instructions.ps1` was its last live local consumer and `routing-coverage` remains the native parity owner.

### Task 4: Rebaseline Planning Evidence And Archive The Phase

Status: `[ ]` Pending

- Update `planning/completed/script-retirement-safety-matrix.md`.
- Update `planning/completed/rust-script-parity-ledger.md`.
- Move the plan/spec to completed once validations and inventory updates materially land.

## Validation Checklist

- `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- `pwsh -NoProfile -File .\scripts\validation\validate-all.ps1 -ValidationProfile dev`
- `pwsh -NoProfile -File .\scripts\orchestration\stages\validate-stage.ps1 -RepoRoot . -WarningOnly`
- `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- focused `validate-all.ps1` proof for the phase-14 scope
- `git diff --check`