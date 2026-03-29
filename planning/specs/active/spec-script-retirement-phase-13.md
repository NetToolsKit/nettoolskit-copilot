# Spec: Phase 13 Validation Wrapper Retirement Readiness

Generated: 2026-03-28 23:32

## Status

- LastUpdated: 2026-03-28 23:32
- Objective: define the design intent and acceptance criteria for retiring the remaining low-fanout operational validation wrappers.
- Planning Readiness: ready-for-implementation
- Related Plan: `planning/active/plan-script-retirement-phase-13.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/agent_orchestration/agent_hooks.rs`
  - `crates/commands/validation/src/operational_hygiene/shell_hooks.rs`
  - `crates/commands/validation/src/operational_hygiene/runtime_script_tests.rs`

## Problem Statement

The repository still carries three low-fanout operational validation wrappers in `scripts/validation/` for agent hooks, shell hooks, and runtime script tests. The Rust implementations already exist and `validate-all` already models the checks semantically, but the executable CLI boundary is still missing. Because of that gap, the compatibility orchestrator, validation inventories, runtime parity fixtures, and authored guidance still treat the three `.ps1` files as canonical launch surfaces.

## Desired Outcome

- `ntk validation` becomes the authoritative executable contract for:
  - `agent-hooks`
  - `shell-hooks`
  - `runtime-script-tests`
- `validate-all.ps1`, `validate-instructions.ps1`, runtime parity fixtures, validation fixtures, and authored guidance stop requiring the three local wrapper paths.
- The three wrapper leaves are deleted only after the remaining consumer chain is repointed.

## Design Decision

Use the existing validation crate implementations as the source of truth and close the gap at the CLI boundary first. Then repoint the residual PowerShell, validation-fixture, runtime-test, and guidance consumers in the same slice before deleting the local wrappers.

## Alternatives Considered

1. Keep the wrappers indefinitely
   - Rejected because it preserves duplicate operational surfaces after native Rust ownership already exists.
2. Delete the wrappers immediately and rely on `validate-all`
   - Rejected because `validate-all`, `validate-instructions`, runtime parity fixtures, and validation support fixtures still encode the wrapper paths directly.
3. Add the native CLI boundaries, repoint the remaining local consumers, then delete the wrappers
   - Selected because it preserves operational continuity while removing compatibility debt safely.

## Risks

- Runtime parity coverage could silently drift if `agent-orchestration-engine.tests.ps1` and the fake Codex runner fixture are not moved off the deleted wrapper paths in the same slice.
- Security and release fixtures could keep validating deleted wrapper paths if their required-evidence sets are not repointed to the native Rust owners.
- `validate-all.ps1` must preserve warning-demotion semantics exactly for the three checks when the runner changes from script to native.

## Acceptance Criteria

- Native validation commands exist for all three checks and are covered by focused CLI tests.
- `validate-all.ps1` invokes the three checks through the native command surface rather than the deleted wrappers.
- `validate-instructions.ps1`, validation fixtures, runtime parity fixtures, and authored guidance no longer require the deleted wrapper paths.
- The three wrapper files are deleted only after the above conditions are met.
- The completed safety matrix and parity ledger record the phase-13 result and the inventory reduction from `112` to `109` overall scripts, from `8` to `5` validation-folder PowerShell files, and from `5` to `2` `validate-*.ps1` leaves.