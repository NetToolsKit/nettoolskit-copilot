# Phase 12: Validation Wrapper Retirement - Documentation and Structure Leaves

Generated: 2026-03-28 23:05

## Status

- LastUpdated: 2026-03-28 23:20
- Objective: retire the remaining documentation and planning-structure validation wrappers by exposing native `ntk validation` command surfaces, repointing live consumers, and deleting the local PowerShell leaves.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keep planning current, and commit each stable retirement phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-12.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-11.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/structure/planning_structure.rs`
  - `crates/commands/validation/src/documentation/readme_standards.rs`
  - `crates/commands/validation/src/governance/template_standards.rs`
  - `crates/commands/validation/src/workspace/workspace_efficiency.rs`
  - `crates/commands/validation/src/documentation/instruction_metadata.rs`
- Current Slice Snapshot:
  - `scripts/**/*.ps1` inventory fell from `117` to `112`
  - `scripts/validation/*.ps1` inventory fell from `13` to `8`
  - native Rust owners and executable CLI boundaries now exist for all five target checks
  - the remaining consumer chain no longer requires the five local wrapper paths

## Scope Summary

1. `scripts/validation/validate-planning-structure.ps1`
2. `scripts/validation/validate-readme-standards.ps1`
3. `scripts/validation/validate-template-standards.ps1`
4. `scripts/validation/validate-workspace-efficiency.ps1`
5. `scripts/validation/validate-instruction-metadata.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for all five checks
- `validate-all.ps1` no longer shells into the five local wrappers
- `validate-instructions.ps1`, stage orchestration, runtime parity tests, and authored guidance stop treating the deleted `.ps1` files as canonical
- the five wrappers are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Freeze The Phase-12 Contract

Status: `[x]` Completed

- Lock the phase-12 design intent in the spec and confirm the acceptance criteria.
- Confirm the native Rust owners and exact CLI contracts for:
  - `planning-structure`
  - `readme-standards`
  - `template-standards`
  - `workspace-efficiency`
  - `instruction-metadata`

### Task 2: Add The Missing Native CLI Boundaries

Status: `[x]` Completed

- Extend `ntk validation` with executable native contracts for the five checks.
- Keep CLI arguments aligned with the existing behavior:
  - `planning-structure`: repo root, warning-only
  - `readme-standards`: repo root, baseline path, warning-only
  - `template-standards`: repo root, baseline path, template directory, warning-only
  - `workspace-efficiency`: repo root, baseline path, settings template path, workspace search root, warning-only
  - `instruction-metadata`: repo root, warning-only
- Add or update focused CLI tests for each new subcommand.

### Task 3: Repoint Consumers And Delete The Five Leaves

Status: `[x]` Completed

- Repoint live consumers that still encode the wrapper paths:
  - `scripts/validation/validate-all.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/orchestration/stages/validate-stage.ps1`
  - `scripts/tests/runtime/planning-structure.tests.ps1`
  - `scripts/tests/runtime/template-standards.tests.ps1`
  - `scripts/tests/runtime/workspace-efficiency.tests.ps1`
  - `definitions/shared/templates/github-change-checklist-template.md`
- Delete the five wrappers only after the consumer chain is cut over.

### Task 4: Rebaseline Planning Evidence And Archive The Phase

Status: `[x]` Completed

- Updated `planning/completed/script-retirement-safety-matrix.md`.
- Updated `planning/completed/rust-script-parity-ledger.md`.
- Moved the plan/spec to completed after validations and inventory updates landed.

## Execution Outcome

- Added native `ntk validation` executable boundaries for:
  - `planning-structure`
  - `readme-standards`
  - `template-standards`
  - `workspace-efficiency`
  - `instruction-metadata`
- Repointed:
  - `scripts/validation/validate-all.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/orchestration/stages/validate-stage.ps1`
  - `scripts/tests/runtime/planning-structure.tests.ps1`
  - `scripts/tests/runtime/template-standards.tests.ps1`
  - `scripts/tests/runtime/workspace-efficiency.tests.ps1`
  - `README.md`
  - `scripts/README.md`
  - `definitions/shared/templates/github-change-checklist-template.md`
  - `.github/templates/github-change-checklist-template.md`
  - `.github/policies/instruction-system.policy.json`
  - `.github/governance/template-standards.baseline.json`
  - `.github/governance/security-baseline.json`
  - `.github/governance/release-provenance.baseline.json`
- Deleted:
  - `scripts/validation/validate-planning-structure.ps1`
  - `scripts/validation/validate-readme-standards.ps1`
  - `scripts/validation/validate-template-standards.ps1`
  - `scripts/validation/validate-workspace-efficiency.ps1`
  - `scripts/validation/validate-instruction-metadata.ps1`
- Re-rendered the GitHub instruction surfaces after the shared checklist template changed.
- Confirmed the local `scripts/**/*.ps1` estate fell from `117` to `112`.
- Confirmed the local `scripts/validation/*.ps1` estate fell from `13` to `8`.
- Focused `validate-all` proof for this phase passed in warning-only and enforcing modes with all five checks routed through the native executable contract.

## Validation Checklist

- `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- `pwsh -NoProfile -File .\scripts\validation\validate-instructions.ps1`
- `pwsh -NoProfile -File .\scripts\tests\runtime\planning-structure.tests.ps1`
- `pwsh -NoProfile -File .\scripts\tests\runtime\template-standards.tests.ps1`
- `pwsh -NoProfile -File .\scripts\tests\runtime\workspace-efficiency.tests.ps1`
- `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- focused `validate-all.ps1` proof for the five phase-12 checks
- `git diff --check`

## Risks And Fallbacks

- The runtime parity harness remains a compatibility launch surface by policy; this phase only changed the internal executor contract from local PowerShell wrappers to the native `ntk validation` binary.
- Template-standards governance had to be updated in the same slice because the authored checklist now validates through `ntk validation instruction-metadata`; leaving the baseline stale would have produced false-positive drift.
- The next validation consumer sweep should stay domain-focused instead of reopening these five leaves.

## Closeout Expectations

- This plan is now archived because the wrapper deletions and focused validations are materially complete.