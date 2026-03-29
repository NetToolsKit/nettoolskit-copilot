# Phase 11: Validation Wrapper Retirement - Instruction Graph Core

Generated: 2026-03-28 22:32

## Status

- LastUpdated: 2026-03-28 22:32
- Objective: retire the low-fanout instruction-graph validation wrappers for `validate-authoritative-source-policy` and `validate-instruction-architecture` by exposing native `ntk validation` boundaries, repointing the remaining validation, runtime-test, and governance consumers, and deleting the local PowerShell leaves.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keep planning current, and commit each stable retirement phase.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/active/spec-script-retirement-phase-11.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-10.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/instruction_graph/authoritative_source_policy.rs`
  - `crates/commands/validation/src/instruction_graph/instruction_architecture.rs`
  - `scripts/tests/runtime/authoritative-source-policy.tests.ps1`
  - `scripts/tests/runtime/instruction-architecture.tests.ps1`
- Current Slice Snapshot:
  - `scripts/**/*.ps1` inventory is `119`
  - `scripts/validation/*.ps1` inventory is `15`
  - native Rust owners already exist for both target checks
  - operator-facing CLI boundaries and residual consumer cleanup are still pending

## Scope Summary

1. `scripts/validation/validate-authoritative-source-policy.ps1`
2. `scripts/validation/validate-instruction-architecture.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for both checks
- `validate-all.ps1` no longer requires the two local wrapper paths
- validation and governance evidence stop treating the deleted `.ps1` files as canonical
- runtime parity tests no longer shell into the deleted wrapper leaves
- both wrappers are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Freeze The Phase-11 Native Validation Contract

Status: `[ ]` Pending

- Register the phase-11 design intent in the spec and lock the cutover acceptance criteria.
- Confirm the native Rust owners and the exact CLI contract for:
  - `authoritative-source-policy`
  - `instruction-architecture`

### Task 2: Add The Missing Native CLI Boundaries

Status: `[ ]` Pending

- Extend `ntk validation` with executable native contracts for:
  - `authoritative-source-policy`
  - `instruction-architecture`
- Keep CLI arguments aligned with the current behavior:
  - `authoritative-source-policy`: repo root, source map path, instruction path, agents path, global instructions path, routing catalog path, instruction search root, warning-only
  - `instruction-architecture`: repo root, manifest path, agents path, global instructions path, routing catalog path, route prompt path, prompt root, template root, skill root, warning-only
- Add or update focused CLI tests for both subcommands.

### Task 3: Retire The Two Wrapper Leaves And Repoint Their Consumers

Status: `[ ]` Pending

- Repoint the remaining local consumers that still encode wrapper paths:
  - `scripts/validation/validate-all.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/tests/runtime/authoritative-source-policy.tests.ps1`
  - `scripts/tests/runtime/instruction-architecture.tests.ps1`
  - `.github/policies/instruction-system.policy.json`
  - `.github/governance/release-provenance.baseline.json`
- Delete:
  - `scripts/validation/validate-authoritative-source-policy.ps1`
  - `scripts/validation/validate-instruction-architecture.ps1`

### Task 4: Rebaseline Planning Evidence And Archive The Phase

Status: `[ ]` Pending

- Update:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- Move this plan/spec into `planning/completed/` and `planning/specs/completed/` after validations pass.

## Validation Checklist

- `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- `pwsh -File .\scripts\validation\validate-instructions.ps1`
- `pwsh -File .\scripts\tests\runtime\authoritative-source-policy.tests.ps1`
- `pwsh -File .\scripts\tests\runtime\instruction-architecture.tests.ps1`
- `pwsh -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- focused `validate-all.ps1` phase-11 profile for:
  - `validate-authoritative-source-policy`
  - `validate-instruction-architecture`
- `git diff --check`

## Risks And Fallbacks

- `validate-instructions.ps1` is still a high-fanout consumer. If one reference is left pointing at the wrapper path, the deletion will be incomplete even when the native CLI command works.
- The runtime parity tests must move in the same slice or they will become false negatives against deleted files.
- Governance evidence files are strict. Any missed path substitution there will look like a repository regression after successful code cutover.

## Closeout Expectations

- Update the retirement matrix and parity ledger in the same slice.
- Archive the phase plan/spec only after the wrapper deletions and focused validations are materially complete.
- Commit the phase with detailed semantic messages once each stable checkpoint is proven.