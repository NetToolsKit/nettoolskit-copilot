# Spec: Phase 14 Validation Wrapper Retirement Readiness

Generated: 2026-03-28 23:52

## Status

- LastUpdated: 2026-03-29 00:08
- Objective: define the design intent and acceptance criteria for retiring the remaining instruction coverage wrapper and the redundant routing-selection PowerShell leaf.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-14.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/instruction_graph/instructions.rs`
  - `crates/commands/validation/src/governance/routing_coverage.rs`
  - `crates/commands/validation/src/orchestration/validate_all.rs`

## Problem Statement

The repository still carries the `scripts/validation/validate-instructions.ps1` wrapper and the redundant `scripts/validation/test-routing-selection.ps1` PowerShell leaf even though the Rust implementation already exists in `crates/commands/validation`. Because the CLI and integrated routing-coverage boundaries were incomplete, `validate-all`, stage orchestration, authored checklists, runbook guidance, and template governance still treated the `.ps1` files as canonical.

## Desired Outcome

- `ntk validation instructions` becomes the authoritative executable contract for instruction coverage validation.
- `ntk validation all` exists as the native orchestration boundary for future wrapper retirement without forcing the compatibility wrapper to remain the only executable contract.
- `validate-all.ps1`, `validate-stage.ps1`, authored checklists, runbooks, and instruction governance stop requiring the local instruction wrapper path.
- The `validate-instructions` and `test-routing-selection` leaves are deleted only after the remaining consumer chain is repointed.

## Design Decision

Use the existing Rust instruction-validation and routing-coverage implementations as the source of truth, close the CLI gap for `instructions` and `all`, then repoint the residual PowerShell, authored documentation, template, and governance consumers in the same slice before deleting the local leaves.

## Alternatives Considered

1. Keep the wrapper and routing-selection script indefinitely
   - Rejected because it preserves duplicate operational surface after native Rust ownership already exists.
2. Delete the leaves immediately and rely on existing crate code only
   - Rejected because `validate-all`, `validate-stage`, and multiple authored/governance surfaces still encoded the wrapper path directly, and the route-selection leaf had not yet been folded into the native instruction-validation result.
3. Add the native CLI boundaries, repoint the remaining local consumers, then delete the leaves
   - Selected because it preserves operational continuity while removing compatibility debt safely.

## Risks

- The authored change checklist and template standards baseline could drift if both the shared template and the projected `.github/templates` copy are not updated in the same slice.
- The instruction system policy could continue failing after deletion if it still requires the wrapper path as evidence.
- `validate-stage.ps1` could keep a hidden direct dependency on the wrapper if only `validate-all.ps1` is repointed.
- The route-selection leaf could be deleted unsafely if the integrated Rust instruction-validation flow does not preserve the golden routing fixture checks.

## Acceptance Criteria

- Native validation commands exist for `instructions` and `all`, and the new boundaries are covered by focused CLI tests.
- `validate-all.ps1` invokes the instruction coverage check through the native command surface rather than the deleted wrapper.
- `validate-stage.ps1`, authored docs/templates, and governance/policy baselines no longer require the deleted instruction wrapper path.
- The route-selection coverage is executed inside the Rust instruction-validation flow before the PowerShell test leaf is deleted.
- The two PowerShell leaves are deleted only after the above conditions are met.
- The completed safety matrix and parity ledger record the phase-14 result and the inventory reduction from `109` to `107` overall scripts and from `5` to `3` validation wrappers.

## Executed Result

The native `ntk validation instructions` and `ntk validation all` boundaries are now executable, the instruction coverage consumer chain no longer requires `scripts/validation/validate-instructions.ps1`, the routing golden coverage is integrated into the Rust instruction-validation result, and both local PowerShell leaves were removed. The focused phase-14 `validate-all` proof passed in warning-only mode with the instruction coverage check routed through the native executable contract, while the compatibility `validate-all.ps1` wrapper remains intentionally retained for future top-level cutover work.