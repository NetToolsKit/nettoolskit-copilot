# Spec: Phase 14 Validation Wrapper Retirement Readiness

Generated: 2026-03-28 23:59

## Status

- LastUpdated: 2026-03-28 23:59
- Objective: define the design intent and acceptance criteria for retiring the remaining instruction-entry validation wrappers.
- Planning Readiness: ready-for-implementation
- Related Plan: `planning/active/plan-script-retirement-phase-14.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/instruction_graph/instructions.rs`
  - `crates/commands/validation/src/governance/routing_coverage.rs`
  - `crates/commands/validation/src/orchestration/validate_all.rs`

## Problem Statement

The repository still carries `validate-instructions.ps1` as a compatibility entrypoint even though the Rust implementation already exists in `crates/commands/validation`. The only remaining direct dependency on `test-routing-selection.ps1` is inside that wrapper, so the routing golden helper also remains local even though routing coverage is already owned natively in Rust. Because the CLI boundary for `validate-instructions` is missing, `validate-all.ps1`, stage orchestration, templates, runbooks, and guidance still treat the local PowerShell wrapper as canonical.

## Desired Outcome

- `ntk validation instructions` becomes the authoritative executable contract for instruction validation.
- `validate-all.ps1`, `validate-stage.ps1`, policies, runbooks, templates, and provider guidance stop requiring `validate-instructions.ps1`.
- `test-routing-selection.ps1` is retired once `validate-instructions.ps1` no longer references it and `routing-coverage` remains the native parity owner.

## Design Decision

Use the existing validation crate implementations as the source of truth, add the missing CLI boundary for `instructions`, repoint the remaining PowerShell and documentation consumers in the same slice, then delete `validate-instructions.ps1`. Once that cutover is complete, remove `test-routing-selection.ps1` because its only remaining purpose was wrapper-local routing golden coverage that now lives in the native `routing-coverage` surface.

## Alternatives Considered

1. Keep both wrappers indefinitely
   - Rejected because it preserves duplicate operational surfaces after native Rust ownership already exists.
2. Delete `validate-instructions.ps1` without adding a CLI contract
   - Rejected because `validate-all.ps1`, `validate-stage.ps1`, templates, and runbooks still encode the wrapper path directly.
3. Add `ntk validation instructions`, repoint consumers, delete `validate-instructions.ps1`, then delete `test-routing-selection.ps1`
   - Selected because it preserves operational continuity while removing the remaining low-fanout instruction-entry compatibility debt safely.

## Risks

- `validate-stage.ps1` could silently keep the deleted wrapper path if only `validate-all.ps1` is updated.
- Template/governance baselines could keep requiring `validate-instructions.ps1` as evidence after the local wrapper is removed.
- `test-routing-selection.ps1` could be deleted too early if any non-obvious local consumer still references it outside `validate-instructions.ps1`.

## Acceptance Criteria

- Native CLI coverage exists for `validation instructions` and is covered by focused command tests.
- `validate-all.ps1` and `validate-stage.ps1` invoke the instruction validation through the native command surface rather than the deleted wrapper.
- Policies, runbooks, templates, and provider guidance no longer require `validate-instructions.ps1`.
- `test-routing-selection.ps1` has no remaining live local consumer when it is deleted.
- The completed safety matrix and parity ledger record the phase-14 result and the inventory reduction from `109` to `107` overall scripts and from `5` to `3` scripts under `scripts/validation/`.