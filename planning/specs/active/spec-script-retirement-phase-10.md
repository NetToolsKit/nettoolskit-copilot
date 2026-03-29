# Spec: Phase 10 Validation Wrapper Retirement Readiness

Generated: 2026-03-28 22:35

## Status

- LastUpdated: 2026-03-28 22:35
- Objective: define the design intent and acceptance criteria for retiring the remaining low-fanout validation wrappers for compatibility lifecycle policy and dotnet standards.
- Planning Readiness: ready-for-implementation
- Related Plan: `planning/active/plan-script-retirement-phase-10.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/completed/plan-script-retirement-phase-9.md`
  - `crates/commands/validation/src/policy/compatibility_lifecycle_policy.rs`
  - `crates/commands/validation/src/standards/dotnet_standards.rs`
  - `scripts/tests/runtime/compatibility-lifecycle.tests.ps1`

## Problem Statement

The repository still carries two local validation wrappers as compatibility launch surfaces:

1. `scripts/validation/validate-compatibility-lifecycle-policy.ps1`
2. `scripts/validation/validate-dotnet-standards.ps1`

The Rust implementations already exist and `validate-all` already understands both checks semantically, but the executable CLI boundary is still missing and a few local consumers still treat the `.ps1` paths as canonical. This prevents safe deletion even though the real validation behavior is already owned natively in Rust.

## Desired Outcome

- `ntk validation compatibility-lifecycle-policy` becomes the authoritative executable contract for COMPATIBILITY lifecycle governance.
- `ntk validation dotnet-standards` becomes the authoritative executable contract for .NET template governance.
- `validate-all.ps1`, policy evidence, and runtime tests stop requiring the local wrapper paths.
- The two wrapper leaves are deleted only after the remaining consumer chain is repointed.

## Design Decision

Use the existing validation crate implementations as the source of truth and close the gap at the CLI boundary first. Then repoint the residual PowerShell and policy consumers in the same slice before deleting the local wrappers.

This keeps behavior stable, avoids duplicate business logic, and limits the phase to one coherent low-fanout cutover.

## Alternatives Considered

1. Keep both wrappers indefinitely
   - Rejected because it preserves duplicate operational surfaces after native Rust ownership already exists.
2. Delete the wrappers immediately and rely on `validate-all`
   - Rejected because `validate-all` still encodes the wrapper paths and external operator/test surfaces would break.
3. Add the native CLI boundaries, repoint the remaining local consumers, then delete the wrappers
   - Selected because it preserves operational continuity while removing the compatibility debt safely.

## Risks

- The compatibility runtime test can silently become stale if it is not moved off the deleted wrapper path in the same slice.
- The instruction-system policy can keep failing after deletion if it still names the dotnet wrapper path instead of the native Rust owner.
- A partial cutover could make the Rust implementation look retired-ready while the suite orchestration still shells into deleted files.

## Acceptance Criteria

- Native validation commands exist for `compatibility-lifecycle-policy` and `dotnet-standards` and are covered by focused CLI tests.
- `validate-all.ps1` invokes both checks through the native command surface rather than the deleted wrappers.
- The compatibility runtime test no longer hardcodes `scripts/validation/validate-compatibility-lifecycle-policy.ps1`.
- Policy or inventory surfaces no longer require the deleted wrapper paths.
- The two wrapper files are deleted only after the above conditions are met.
- The completed safety matrix and parity ledger record the phase-10 result and the inventory reduction.

## Planning Readiness

- The scope is narrow and coherent enough to execute in one retirement phase.
- The Rust owners are already present and tested at the validation crate layer.
- No further architecture decision remains open for this slice.