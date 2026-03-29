# Spec: Phase 10 Validation Wrapper Retirement Readiness

Generated: 2026-03-28 22:35

## Status

- LastUpdated: 2026-03-28 22:13
- Objective: define the design intent and acceptance criteria for retiring the remaining low-fanout validation wrappers for compatibility lifecycle policy and dotnet standards.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-10.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/completed/plan-script-retirement-phase-9.md`
  - `crates/commands/validation/src/policy/compatibility_lifecycle_policy.rs`
  - `crates/commands/validation/src/standards/dotnet_standards.rs`
  - `scripts/tests/runtime/compatibility-lifecycle.tests.ps1`

## Problem Statement

The repository still carried two local validation wrappers as compatibility launch surfaces:

1. `scripts/validation/validate-compatibility-lifecycle-policy.ps1`
2. `scripts/validation/validate-dotnet-standards.ps1`

The Rust implementations already existed and `validate-all` already understood both checks semantically, but the executable CLI boundary was missing and a few local consumers still treated the `.ps1` paths as canonical. That prevented safe deletion even though the real validation behavior was already owned natively in Rust.

## Desired Outcome

- `ntk validation compatibility-lifecycle-policy` becomes the authoritative executable contract for COMPATIBILITY lifecycle governance.
- `ntk validation dotnet-standards` becomes the authoritative executable contract for .NET template governance.
- `validate-all.ps1`, policy evidence, and runtime tests stop requiring the local wrapper paths.
- The two wrapper leaves are deleted only after the remaining consumer chain is repointed.

## Design Decision

Use the existing validation crate implementations as the source of truth and close the gap at the CLI boundary first. Then repoint the residual PowerShell and policy consumers in the same slice before deleting the local wrappers.

## Alternatives Considered

1. Keep both wrappers indefinitely
   - Rejected because it preserves duplicate operational surfaces after native Rust ownership already exists.
2. Delete the wrappers immediately and rely on `validate-all`
   - Rejected because `validate-all` still encoded the wrapper paths and external operator/test surfaces would break.
3. Add the native CLI boundaries, repoint the remaining local consumers, then delete the wrappers
   - Selected because it preserved operational continuity while removing the compatibility debt safely.

## Risks

- The compatibility runtime test could silently become stale if it was not moved off the deleted wrapper path in the same slice.
- The instruction-system policy could keep failing after deletion if it still named the dotnet wrapper path instead of the native Rust owner.
- A partial cutover could make the Rust implementation look retired-ready while the suite orchestration still shelled into deleted files.

## Acceptance Criteria

- Native validation commands exist for `compatibility-lifecycle-policy` and `dotnet-standards` and are covered by focused CLI tests.
- `validate-all.ps1` invokes both checks through the native command surface rather than the deleted wrappers.
- The compatibility runtime test no longer hardcodes `scripts/validation/validate-compatibility-lifecycle-policy.ps1`.
- Policy or inventory surfaces no longer require the deleted wrapper paths.
- The two wrapper files are deleted only after the above conditions are met.
- The completed safety matrix and parity ledger record the phase-10 result and the inventory reduction.

## Executed Result

Both validation wrappers were retired after the native CLI contract, the `validate-all` consumer chain, the remaining policy evidence, and the compatibility runtime parity test were updated in the same slice. Focused `validate-all` warning-only proof passed with both checks routed through the native executable contract, and enforcing mode exposed only pre-existing repository lifecycle-table debt rather than a cutover regression.