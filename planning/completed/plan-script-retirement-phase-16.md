# Phase 16: Validation Reporting Wrapper Retirement

Generated: 2026-03-29 08:00

## Status

- LastUpdated: 2026-03-29 08:01
- Objective: replace the remaining validation reporting wrappers with native runtime reporting commands, then delete the local `.ps1` leaves after the live consumer chain is repointed in the same slice.
- Normalized Request: continue the aggressive PowerShell-to-Rust retirement flow, keep planning updated, and commit each stable phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-16.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-15.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/validation/export-audit-report.ps1`
  - `scripts/validation/export-enterprise-trends.ps1`
  - `crates/commands/runtime/src/diagnostics/healthcheck.rs`

## Scope Summary

1. `scripts/validation/export-audit-report.ps1`
2. `scripts/validation/export-enterprise-trends.ps1`
3. native `ntk runtime` reporting entrypoints plus consumer repoints in docs, runbooks, and governance evidence

This phase is complete only if:

- native reporting commands own the audit-report and enterprise-trends behavior without reintroducing a PowerShell wrapper dependency
- runbooks, policy baselines, and release evidence stop treating the validation-folder scripts as required operator entrypoints
- both reporting wrappers are either deleted safely in this phase or retained with a narrow, explicit blocker record

## Ordered Tasks

### Task 1: Lock The Reporting Surface Design

Status: `[x]` Completed

- Confirm the reporting pair should migrate into `crates/commands/runtime/src/diagnostics/` and the `ntk runtime` CLI surface instead of the validation crate, to avoid a runtime/validation dependency cycle.
- Freeze the native command names, arguments, outputs, and result ownership in the spec before implementation begins.

### Task 2: Implement Native Reporting Commands

Status: `[x]` Completed

- Reused the native `ntk runtime healthcheck` surface as the canonical audit-report export command.
- Added the native `ntk runtime export-enterprise-trends` reporting command in `crates/commands/runtime/src/diagnostics/enterprise_trends.rs`.
- Exposed both reporting entrypoints through `ntk runtime`.
- Add runtime crate tests and CLI tests that prove deterministic output for both commands.

### Task 3: Repoint Repository Consumers And Delete The Wrappers

Status: `[x]` Completed

- Repoint runbooks, governance baselines, and policy evidence to the new native command or native Rust owner files.
- Delete `scripts/validation/export-audit-report.ps1` and `scripts/validation/export-enterprise-trends.ps1` if all live consumers are cleared in the same slice.

### Task 4: Rebaseline Phase Evidence And Archive

Status: `[x]` Completed

- Update `planning/completed/script-retirement-safety-matrix.md`.
- Update `planning/completed/rust-script-parity-ledger.md`.
- Move the plan/spec to completed only after the validations prove the phase result.

## Validation Checklist

- `cargo test -p nettoolskit-runtime --quiet`
- `cargo test -p nettoolskit-cli --test test_suite runtime_commands_tests --quiet`
- targeted native `export-audit-report` / `export-enterprise-trends` proof through `ntk runtime`
- `& .\\.build\\target\\debug\\ntk.exe validation planning-structure --repo-root . --warning-only false`
- `& .\\.build\\target\\debug\\ntk.exe validation instructions --repo-root . --warning-only false`
- `pwsh -NoProfile -File .\\scripts\\security\\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- `git diff --check`

## Execution Outcome

- `ntk runtime healthcheck` is now the canonical audit-report export path for release and rollback runbooks plus governance guidance.
- `ntk runtime export-enterprise-trends` now owns the enterprise trend export behavior natively.
- The live validation-folder reporting wrappers were deleted:
  - `scripts/validation/export-audit-report.ps1`
  - `scripts/validation/export-enterprise-trends.ps1`
- Repository evidence and policy baselines now point at the runtime diagnostics Rust owners instead of the deleted wrapper paths.
- Script inventory moved from `106` to `104`.
- Validation-folder `.ps1` inventory moved from `2` to `0`.

## Risks And Fallbacks

- `export-audit-report` depends on runtime healthcheck behavior, so the native implementation must stay in the runtime crate or a higher layer that does not create a crate cycle.
- Governance and instruction policy files currently pin the reporting scripts as required evidence, so deletion without same-slice rebaseline would create false failures.
- The reporting artifacts are operator-facing; the native commands must preserve output paths and warning-only semantics closely enough to avoid breaking release workflows.

## Closeout Expectations

- Implementation and planning closeout remain committed separately.
- This phase is ready to archive because the reporting leaves were deleted safely and the same-slice consumer repoints plus validation evidence completed.