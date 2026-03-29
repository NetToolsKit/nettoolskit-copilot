# Spec: Phase 12 Validation Wrapper Retirement Readiness

Generated: 2026-03-28 23:05

## Status

- LastUpdated: 2026-03-28 23:05
- Objective: define the design intent and acceptance criteria for retiring the remaining documentation and planning-structure validation wrappers.
- Planning Readiness: ready-for-implementation
- Related Plan: `planning/active/plan-script-retirement-phase-12.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/structure/planning_structure.rs`
  - `crates/commands/validation/src/documentation/readme_standards.rs`
  - `crates/commands/validation/src/governance/template_standards.rs`
  - `crates/commands/validation/src/workspace/workspace_efficiency.rs`
  - `crates/commands/validation/src/documentation/instruction_metadata.rs`

## Problem Statement

The repository still carries five validation wrappers in `scripts/validation/` for planning workspace structure, README standards, template standards, workspace efficiency, and instruction metadata. The Rust implementations already exist and `validate-all` already models the checks semantically, but the executable CLI boundary is still missing. Because of that gap, the local PowerShell orchestrator, runtime parity tests, stage orchestration, and authored guidance still treat the five `.ps1` files as canonical launch surfaces.

## Desired Outcome

- `ntk validation` becomes the authoritative executable contract for:
  - `planning-structure`
  - `readme-standards`
  - `template-standards`
  - `workspace-efficiency`
  - `instruction-metadata`
- `validate-all.ps1`, `validate-instructions.ps1`, runtime parity tests, and stage orchestration stop requiring the five local wrapper paths.
- The five wrapper leaves are deleted only after the remaining consumer chain is repointed.

## Design Decision

Use the existing validation crate implementations as the source of truth and close the gap at the CLI boundary first. Then repoint the residual PowerShell, orchestration, runtime-test, and guidance consumers in the same slice before deleting the local wrappers.

## Alternatives Considered

1. Keep the wrappers indefinitely
   - Rejected because it preserves duplicate operational surfaces after native Rust ownership already exists.
2. Delete the wrappers immediately and rely on `validate-all`
   - Rejected because `validate-all`, `validate-instructions`, `validate-stage`, runtime parity tests, and authored guidance still encode the wrapper paths directly.
3. Add the native CLI boundaries, repoint the remaining local consumers, then delete the wrappers
   - Selected because it preserves operational continuity while removing compatibility debt safely.

## Risks

- Runtime parity coverage for planning, template, and workspace validation could silently drift if the tests are not moved off the deleted wrapper paths in the same slice.
- `validate-stage.ps1` could keep a hidden direct dependency on `validate-planning-structure.ps1` after the wrappers are deleted.
- The instruction inventory could continue treating the `.ps1` files as canonical if `validate-instructions.ps1` and the authored checklist template are not updated in the same slice.

## Acceptance Criteria

- Native validation commands exist for all five checks and are covered by focused CLI tests.
- `validate-all.ps1` invokes the five checks through the native command surface rather than the deleted wrappers.
- `validate-instructions.ps1`, `validate-stage.ps1`, runtime parity tests, and authored checklist guidance no longer require the deleted wrapper paths.
- The five wrapper files are deleted only after the above conditions are met.
- The completed safety matrix and parity ledger record the phase-12 result and the inventory reduction from `117` to `112` overall scripts and from `13` to `8` validation wrappers.