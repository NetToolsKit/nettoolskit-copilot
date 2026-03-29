# Phase 11: Validation Wrapper Retirement - Instruction Graph Command Leaves

Generated: 2026-03-28 22:34

## Status

- LastUpdated: 2026-03-28 22:48
- Objective: retire the remaining low-fanout instruction-graph validation wrappers for `validate-authoritative-source-policy` and `validate-instruction-architecture` by exposing native `ntk validation` command surfaces, repointing live PowerShell and governance consumers, and deleting the local PowerShell leaves.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keep planning current, use subagents aggressively when safe, and commit each stable retirement phase.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-11.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-10.md`
  - `planning/specs/completed/spec-script-retirement-phase-10.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/instruction_graph/authoritative_source_policy.rs`
  - `crates/commands/validation/src/instruction_graph/instruction_architecture.rs`
  - `scripts/tests/runtime/authoritative-source-policy.tests.ps1`
  - `scripts/tests/runtime/instruction-architecture.tests.ps1`
- Current Slice Snapshot:
  - `scripts/**/*.ps1` inventory fell from `119` to `117`
  - `scripts/validation/*.ps1` inventory fell from `15` to `13`
  - native Rust owners and executable CLI boundaries now exist for both target checks
  - the remaining consumer chain no longer requires the two local wrapper paths

## Scope Summary

1. `scripts/validation/validate-authoritative-source-policy.ps1`
2. `scripts/validation/validate-instruction-architecture.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for both checks
- `validate-all.ps1` no longer shells into the two local wrappers
- `validate-instructions.ps1` and governance evidence stop treating the deleted `.ps1` files as canonical
- the two runtime parity scripts stop invoking the deleted validation wrappers directly
- both wrappers are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Freeze The Phase-11 Native Validation Contract

Status: `[x]` Completed

- Register the phase-11 design intent in the spec and lock the cutover acceptance criteria.
- Confirm the native Rust owners and the exact CLI contract for:
  - `authoritative-source-policy`
  - `instruction-architecture`

### Task 2: Add The Missing Native CLI Boundaries

Status: `[x]` Completed

- Extend `ntk validation` with executable native contracts for:
  - `authoritative-source-policy`
  - `instruction-architecture`
- Keep CLI arguments aligned with the current behavior:
  - `authoritative-source-policy`: repo root, source map path, instruction path, AGENTS path, global instructions path, routing catalog path, instruction search root, warning-only
  - `instruction-architecture`: repo root, manifest path, AGENTS path, global instructions path, routing catalog path, route prompt path, prompt root, template root, skill root, warning-only
- Add or update focused CLI tests for both subcommands.

### Task 3: Retire The Two Wrapper Leaves And Repoint Their Consumers

Status: `[x]` Completed

- Repointed the remaining local consumers that still encoded wrapper paths:
  - `scripts/validation/validate-all.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/tests/runtime/authoritative-source-policy.tests.ps1`
  - `scripts/tests/runtime/instruction-architecture.tests.ps1`
  - `.github/policies/instruction-system.policy.json`
  - `.github/governance/release-provenance.baseline.json`
- Deleted:
  - `scripts/validation/validate-authoritative-source-policy.ps1`
  - `scripts/validation/validate-instruction-architecture.ps1`

### Task 4: Rebaseline Planning Evidence And Archive The Phase

Status: `[x]` Completed

## Execution Outcome

- Added native `ntk validation` executable boundaries for:
  - `authoritative-source-policy`
  - `instruction-architecture`
- Repointed:
  - `scripts/validation/validate-all.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/tests/runtime/authoritative-source-policy.tests.ps1`
  - `scripts/tests/runtime/instruction-architecture.tests.ps1`
  - `.github/policies/instruction-system.policy.json`
  - `.github/governance/release-provenance.baseline.json`
- Deleted:
  - `scripts/validation/validate-authoritative-source-policy.ps1`
  - `scripts/validation/validate-instruction-architecture.ps1`
- Confirmed the local `scripts/**/*.ps1` estate fell from `119` to `117`.
- Confirmed the local `scripts/validation/*.ps1` estate fell from `15` to `13`.
- Focused `validate-all` proof for this phase passed in both modes with both checks routed through the native executable contract.

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

- The native CLI boundary intentionally does not preserve the shell-only `DetailedOutput` switch; operational parity is held at the result and exit-code level instead of wrapper-specific formatting.
- The remaining instruction-governance backlog is broader than this slice and should continue as future domain-level retirement phases, not by restoring the deleted wrappers.
- The runtime parity scripts remain intentional compatibility launch surfaces; only their internal executor changed in this slice.

## Closeout Expectations

- This plan is now archived because the wrapper deletions and focused validations are materially complete.