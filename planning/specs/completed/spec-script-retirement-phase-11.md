# Spec: Phase 11 Validation Wrapper Retirement Readiness

Generated: 2026-03-28 22:34

## Status

- LastUpdated: 2026-03-28 22:48
- Objective: define the design intent and acceptance criteria for retiring the remaining low-fanout instruction-graph validation wrappers for authoritative source policy and instruction architecture.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-11.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/completed/plan-script-retirement-phase-10.md`
  - `crates/commands/validation/src/instruction_graph/authoritative_source_policy.rs`
  - `crates/commands/validation/src/instruction_graph/instruction_architecture.rs`
  - `scripts/tests/runtime/authoritative-source-policy.tests.ps1`
  - `scripts/tests/runtime/instruction-architecture.tests.ps1`

## Problem Statement

The repository still carried two local validation wrappers as compatibility launch surfaces:

1. `scripts/validation/validate-authoritative-source-policy.ps1`
2. `scripts/validation/validate-instruction-architecture.ps1`

The Rust implementations already existed and `validate-all` already modeled both checks semantically, but the executable CLI boundary was still missing. Because of that gap, the live PowerShell orchestrator, policy inventory, release evidence, and runtime parity tests still treated the two `.ps1` files as canonical. That prevented safe deletion even though the real validation behavior was already owned natively in Rust.

## Desired Outcome

- `ntk validation authoritative-source-policy` becomes the authoritative executable contract for centralized authoritative-source governance.
- `ntk validation instruction-architecture` becomes the authoritative executable contract for instruction ownership and routing-boundary governance.
- `validate-all.ps1`, `validate-instructions.ps1`, runtime parity tests, and governance evidence stop requiring the local wrapper paths.
- The two wrapper leaves are deleted only after the remaining consumer chain is repointed.

## Design Decision

Use the existing validation crate implementations as the source of truth and close the gap at the CLI boundary first. Then repoint the residual PowerShell, policy, and runtime-test consumers in the same slice before deleting the local wrappers.

## Alternatives Considered

1. Keep both wrappers indefinitely
   - Rejected because it preserves duplicate operational surfaces after native Rust ownership already exists.
2. Delete the wrappers immediately and rely on `validate-all`
   - Rejected because `validate-all`, `validate-instructions`, governance baselines, and runtime parity still encoded the wrapper paths directly.
3. Add the native CLI boundaries, repoint the remaining local consumers, then delete the wrappers
   - Selected because it preserved operational continuity while removing the compatibility debt safely.

## Risks

- The runtime parity scripts could silently become stale if they were not moved off the deleted wrapper paths in the same slice.
- The instruction-system policy and release-provenance evidence could keep failing after deletion if they still named the wrapper files instead of the authoritative Rust owners.
- A partial cutover could make the Rust implementations look retired-ready while the validation shell orchestration still shelled into deleted files.

## Acceptance Criteria

- Native validation commands exist for `authoritative-source-policy` and `instruction-architecture` and are covered by focused CLI tests.
- `validate-all.ps1` invokes both checks through the native command surface rather than the deleted wrappers.
- `validate-instructions.ps1` and governance evidence no longer require the deleted wrapper paths.
- The two runtime parity scripts no longer hardcode `scripts/validation/validate-authoritative-source-policy.ps1` or `scripts/validation/validate-instruction-architecture.ps1`.
- The two wrapper files are deleted only after the above conditions are met.
- The completed safety matrix and parity ledger record the phase-11 result and the inventory reduction from `119` to `117` overall scripts and from `15` to `13` validation wrappers.

## Executed Result

Both validation wrappers were retired after the native CLI contract, the `validate-all` consumer chain, the instruction inventory, the governance evidence, and the runtime parity tests were updated in the same slice. The focused phase-11 `validate-all` proof passed in warning-only and enforcing modes with both checks routed through the native executable contract.