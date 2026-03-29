# Spec: Phase 11 Validation Wrapper Retirement Readiness

Generated: 2026-03-28 22:32

## Status

- LastUpdated: 2026-03-28 22:32
- Objective: define the cutover design and acceptance criteria for retiring the remaining instruction-graph validation wrappers for authoritative source policy and instruction architecture.
- Planning Readiness: ready-for-implementation
- Related Plan: `planning/active/plan-script-retirement-phase-11.md`
- Source Inputs:
  - `planning/completed/plan-script-retirement-phase-10.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/instruction_graph/authoritative_source_policy.rs`
  - `crates/commands/validation/src/instruction_graph/instruction_architecture.rs`
  - `scripts/tests/runtime/authoritative-source-policy.tests.ps1`
  - `scripts/tests/runtime/instruction-architecture.tests.ps1`

## Problem Statement

The repository still carries two local PowerShell wrappers under `scripts/validation/` for checks that are already implemented natively in Rust:

1. `validate-authoritative-source-policy`
2. `validate-instruction-architecture`

The Rust implementations already participate in `validate-all` semantically, but the operator-facing `ntk validation` surface is still missing for this pair and the residual consumer chain still encodes the deleted-wrapper paths as canonical evidence. That keeps the wrappers alive even though behavior ownership already moved to Rust.

## Desired Outcome

- `ntk validation authoritative-source-policy` becomes the authoritative executable contract for authoritative-source governance.
- `ntk validation instruction-architecture` becomes the authoritative executable contract for instruction ownership and boundary governance.
- `validate-all.ps1`, `validate-instructions.ps1`, runtime parity tests, and governance evidence stop treating the `.ps1` wrappers as canonical.
- Both wrapper leaves are deleted only after the native executable boundary and residual consumers are aligned in the same slice.

## Design Decision

Use the existing validation crate implementations as the source of truth and close the gap at the CLI boundary first. Then repoint the remaining PowerShell tests, validation orchestration, and governance evidence in the same slice before deleting the wrappers.

## Alternatives Considered

1. Keep both wrappers as stable compatibility shims
   - Rejected because it preserves duplicate executable surfaces after Rust ownership already exists.
2. Delete the wrappers immediately and rely on internal `validate-all` routing
   - Rejected because the operator-facing CLI, runtime tests, and evidence files still encode the wrapper paths.
3. Add native CLI surfaces, repoint all residual consumers, then delete the wrappers
   - Selected because it removes compatibility debt without breaking validation workflows or audit evidence.

## Risks

- The runtime parity tests could stay stale if they continue to shell into deleted wrapper paths.
- Governance evidence could keep failing after deletion if `instruction-system.policy.json` or `release-provenance.baseline.json` still require the wrapper files.
- A partial cutover could make the native validation layer look complete while `validate-instructions.ps1` still treats the old wrappers as canonical.

## Acceptance Criteria

- Native validation commands exist for `authoritative-source-policy` and `instruction-architecture` and are covered by focused CLI tests.
- `validate-all.ps1` routes both checks through the native executable surface instead of the deleted wrapper paths.
- `validate-instructions.ps1` no longer requires the two wrapper files as canonical validation evidence.
- Runtime parity tests stop invoking `scripts/validation/validate-authoritative-source-policy.ps1` and `scripts/validation/validate-instruction-architecture.ps1` directly.
- Governance evidence no longer requires the deleted wrapper paths.
- Both wrapper files are deleted only after the above conditions are met.
- The completed safety matrix and parity ledger record the phase-11 cutover and the script inventory reduction.