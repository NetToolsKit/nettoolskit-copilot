# Spec: Phase 13 Validation Wrapper Retirement Readiness

Generated: 2026-03-28 23:33

## Status

- LastUpdated: 2026-03-28 23:46
- Objective: define the design intent and acceptance criteria for retiring the remaining hook and runtime-test validation wrappers.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-13.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/agent_orchestration/agent_hooks.rs`
  - `crates/commands/validation/src/operational_hygiene/shell_hooks.rs`
  - `crates/commands/validation/src/operational_hygiene/runtime_script_tests.rs`

## Problem Statement

The repository still carries three low-fanout validation wrappers for agent hooks, shell hooks, and runtime script tests. The Rust implementations already exist and `validate-all` already models the checks semantically, but the executable CLI boundary is still missing. Because of that gap, `validate-all`, release/security evidence, and the runtime parity harness still treat the `.ps1` files as canonical launch surfaces.

## Desired Outcome

- `ntk validation` becomes the authoritative executable contract for:
  - `agent-hooks`
  - `shell-hooks`
  - `runtime-script-tests`
- `validate-all.ps1`, release/security evidence, and runtime parity fixtures stop requiring the three local wrapper paths.
- The three wrapper leaves are deleted only after the remaining consumer chain is repointed.

## Design Decision

Use the existing validation crate implementations as the source of truth and close the gap at the CLI boundary first. Then repoint the residual PowerShell, orchestration, runtime-test, and guidance consumers in the same slice before deleting the local wrappers.

## Alternatives Considered

1. Keep the wrappers indefinitely
   - Rejected because it preserves duplicate operational surfaces after native Rust ownership already exists.
2. Delete the wrappers immediately and rely on `validate-all`
   - Rejected because `validate-all`, release/security baselines, and parity fixtures still encode the wrapper paths directly.
3. Add the native CLI boundaries, repoint the remaining local consumers, then delete the wrappers
   - Selected because it preserves operational continuity while removing compatibility debt safely.

## Risks

- The parity harness could drift silently if the fake runner fixtures are not repointed from `validate-runtime-script-tests.ps1` in the same slice.
- Governance baselines could continue failing after deletion if they still require the wrapper paths as evidence.
- `validate-all.ps1` would become inconsistent with the Rust orchestration contract if the three checks are not cut over together.

## Acceptance Criteria

- Native validation commands exist for all three checks and are covered by focused CLI tests.
- `validate-all.ps1` invokes the three checks through the native command surface rather than the deleted wrappers.
- Release/security baselines and runtime parity fixtures no longer require the deleted wrapper paths.
- The three wrapper files are deleted only after the above conditions are met.
- The completed safety matrix and parity ledger record the phase-13 result and the inventory reduction from `112` to `109` overall scripts and from `8` to `5` scripts under `scripts/validation/`.

## Executed Result

All three validation wrappers were retired after the native CLI contract, the `validate-all` consumer chain, the runtime parity fixtures, the release/security evidence baselines, and the authored Codex guidance were updated in the same slice. The focused phase-13 `validate-all` proof passed in warning-only mode with all three checks routed through the native executable contract. The remaining failing repository-wide validations are pre-existing governance and runtime harness debt rather than regressions introduced by this phase.