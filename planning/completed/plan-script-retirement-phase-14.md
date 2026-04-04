# Phase 14: Validation Wrapper Retirement - Instruction Coverage and Routing Leaves

Generated: 2026-03-28 23:52

## Status

- LastUpdated: 2026-03-29 00:08
- Objective: retire the remaining instruction coverage wrapper plus the redundant routing-selection PowerShell leaf by exposing native `ntk validation` command surfaces, repointing the live consumers, and deleting the local PowerShell leaves.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keep planning current, and commit each stable retirement phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-14.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-13.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/instruction_graph/instructions.rs`
  - `crates/commands/validation/src/governance/routing_coverage.rs`
  - `crates/commands/validation/src/orchestration/validate_all.rs`
- Current Slice Snapshot:
  - `scripts/**/*.ps1` inventory fell from `109` to `107`
  - `scripts/validation/*.ps1` inventory fell from `5` to `3`
  - native Rust ownership and executable CLI boundaries now exist for `validate-instructions`
  - routing golden coverage is now absorbed directly into the Rust instruction validation flow
  - the remaining validation folder is reduced to orchestration/reporting leaves only

## Scope Summary

1. `scripts/validation/validate-instructions.ps1`
2. `scripts/validation/test-routing-selection.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for `instructions` and `all`
- `validate-all.ps1` and `validate-stage.ps1` no longer shell into the local `validate-instructions` wrapper
- authored guidance, templates, and instruction governance stop treating the deleted `.ps1` file as canonical
- the two local leaves are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Freeze The Phase-14 Contract

Status: `[x]` Completed

- Locked the phase-14 design intent in the spec and confirmed the acceptance criteria.
- Confirmed the native Rust owners and exact CLI contracts for:
  - `instructions`
  - `all`

### Task 2: Add The Missing Native CLI Boundaries

Status: `[x]` Completed

- Extended `ntk validation` with executable native contracts for:
  - `instructions`
  - `all`
- Kept CLI arguments aligned with the existing behavior:
  - `instructions`: repo root, warning-only
  - `all`: repo root, validation profile, warning-only, ledger/output options, PowerShell standards switches
- Added or updated focused CLI tests for the new subcommands.

### Task 3: Repoint Consumers And Delete The Leaves

Status: `[x]` Completed

- Repointed live consumers that still encoded the wrapper path:
  - `scripts/validation/validate-all.ps1`
  - `scripts/orchestration/stages/validate-stage.ps1`
  - `definitions/providers/vscode/workspace/README.md`
  - `definitions/shared/templates/github-change-checklist-template.md`
  - `.github/templates/github-change-checklist-template.md`
  - `definitions/shared/instructions/docs/ntk-docs-copilot-instruction-creation.instructions.md`
  - `.github/instructions/docs/ntk-docs-copilot-instruction-creation.instructions.md`
  - `.github/runbooks/README.md`
  - `.github/governance/release-governance.md`
  - `.github/policies/instruction-system.policy.json`
  - `.github/governance/template-standards.baseline.json`
- Deleted:
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/validation/test-routing-selection.ps1`

### Task 4: Rebaseline Planning Evidence And Archive The Phase

Status: `[x]` Completed

- Updated `planning/completed/script-retirement-safety-matrix.md`.
- Updated `planning/completed/rust-script-parity-ledger.md`.
- Moved the plan/spec to completed once the validations and inventory updates were materially done.

## Execution Outcome

- Added native `ntk validation` executable boundaries for:
  - `instructions`
  - `all`
- Integrated routing golden coverage directly into the Rust `validate-instructions` flow so the local `test-routing-selection.ps1` leaf is no longer required.
- Repointed:
  - `scripts/validation/validate-all.ps1`
  - `scripts/orchestration/stages/validate-stage.ps1`
  - `definitions/providers/vscode/workspace/README.md`
  - `definitions/shared/templates/github-change-checklist-template.md`
  - `.github/templates/github-change-checklist-template.md`
  - `definitions/shared/instructions/docs/ntk-docs-copilot-instruction-creation.instructions.md`
  - `.github/instructions/docs/ntk-docs-copilot-instruction-creation.instructions.md`
  - `.github/runbooks/README.md`
  - `.github/governance/release-governance.md`
  - `.github/policies/instruction-system.policy.json`
  - `.github/governance/template-standards.baseline.json`
- Updated validation crate fixtures and release-provenance support surfaces so native `validate-all` contracts can be reasoned about without reading the deleted wrapper path as canonical proof.
- Deleted:
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/validation/test-routing-selection.ps1`
- Confirmed the local `scripts/**/*.ps1` estate fell from `109` to `107`.
- Confirmed the local `scripts/validation/*.ps1` estate fell from `5` to `3`.
- Focused `validate-all` proof for this phase passed in warning-only mode with the instruction coverage check routed through the native executable contract.

## Validation Checklist

- `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- focused `validate-all.ps1` proof for the `validate-instructions` check
- `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- `git diff --check`

## Risks And Fallbacks

- `validate-all.ps1` remains in the repository after this phase because several authored workflows and release surfaces still intentionally depend on the compatibility wrapper while the native `ntk validation all` boundary is being prepared for a future cutover.
- The runtime/orchestration PowerShell estate still contains broader wrapper-retirement debt outside this phase; this slice only removes the instruction coverage and routing-selection leaves.
- Repository-wide enforcing validation still reflects pre-existing governance debt where the repo is intentionally not yet green on missing community/governance assets.

## Closeout Expectations

- This plan is now archived because the instruction coverage and routing-selection leaf deletions are materially complete.