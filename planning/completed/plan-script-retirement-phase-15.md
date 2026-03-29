# Phase 15: Validation Wrapper Retirement - Native Validate-All Cutover

Generated: 2026-03-29 00:40

## Status

- LastUpdated: 2026-03-29 07:35
- Objective: repoint the remaining repository consumers of `scripts/validation/validate-all.ps1` to the native `ntk validation all` contract, then delete the local wrapper if no runtime or policy blocker remains.
- Normalized Request: continue the aggressive PowerShell-to-Rust retirement flow, keep planning updated, and commit each stable phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-15.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-14.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/orchestration/validate_all.rs`
  - `crates/cli/src/validation_commands.rs`
  - `scripts/runtime/healthcheck.ps1`
  - `scripts/orchestration/stages/validate-stage.ps1`

## Scope Summary

1. `scripts/validation/validate-all.ps1`
2. native `ntk validation all` consumer repoints in runtime, orchestration, docs, runbooks, policies, and projected skill surfaces

This phase is complete only if:

- the native `ntk validation all` command becomes the canonical consumer-facing contract for repository validation orchestration
- runtime/orchestration wrappers no longer depend on the local `validate-all.ps1` path
- release/policy baselines and authored docs stop treating the wrapper path as canonical evidence
- the wrapper is either deleted safely in this phase or explicitly retained with a reduced, documented blocker set

## Ordered Tasks

### Task 1: Freeze The Phase-15 Contract

Status: `[x]` Completed

- Confirmed that the current Rust `validate-all` surface already covers the compatibility wrapper responsibilities that still matter operationally for this repository.
- Locked the deletion decision in the spec and executed the broad consumer repoint onto `ntk validation all`.

### Task 2: Repoint Remaining Native Consumers

Status: `[x]` Completed

- Repointed runtime/orchestration scripts and fallbacks:
  - `scripts/runtime/healthcheck.ps1`
  - `scripts/orchestration/stages/validate-stage.ps1`
  - `scripts/orchestration/stages/plan-stage.ps1`
  - `scripts/orchestration/stages/implement-stage.ps1`
- Repointed authored docs, runbooks, policies, and projected skill/readme surfaces to `ntk validation all`.

### Task 3: Remove Or Retain The Wrapper Deliberately

Status: `[x]` Completed

- All hard blockers cleared for the local wrapper, so `scripts/validation/validate-all.ps1` was deleted in this phase.
- The remaining validation folder is now reduced to the reporting pair `export-audit-report.ps1` and `export-enterprise-trends.ps1`.

### Task 4: Rebaseline Phase Evidence And Archive

Status: `[x]` Completed

- Updated `planning/completed/script-retirement-safety-matrix.md`.
- Updated `planning/completed/rust-script-parity-ledger.md`.
- This plan/spec are ready to move to completed because the validations proved the phase result.

## Execution Outcome

- `ntk validation all` is now the canonical validation orchestration contract for runtime, orchestration, docs, runbooks, governance evidence, and projected skill surfaces.
- `scripts/runtime/healthcheck.ps1` now dispatches validation through the native `rust:nettoolskit-validation::validate-all` surface instead of shelling into the deleted local wrapper.
- No live reference to `scripts/validation/validate-all.ps1` remains outside `planning/**`.
- The local `scripts/**/*.ps1` estate dropped from `107` to `106`, and `scripts/validation/*.ps1` dropped from `3` to `2`.

## Validation Checklist

- [x] `cargo test -p nettoolskit-validation --quiet`
- [x] `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
- [x] focused `validate-all` / `healthcheck` proof for the native `validate-all` cutover
- [x] `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- [x] `git diff --check`

## Risks And Fallbacks

- `export-audit-report.ps1` and `export-enterprise-trends.ps1` remain in the validation folder, so this phase still avoids conflating `validate-all` cutover with reporting-script retirement.
- Runtime healthcheck still reports warning-only drift in the user runtime, but that warning is unrelated to the `validate-all` cutover itself.

## Closeout Expectations

- Commit implementation and planning closeout separately.
- Archive the phase now that the wrapper decision is explicit and the remaining validation backlog is reduced to the reporting leaves.