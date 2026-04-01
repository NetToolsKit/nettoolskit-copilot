# Script Retirement Tail Cutover Plan

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-03-30 07:31
- Objective: finish the remaining `scripts/*.ps1` retirement work by converting the open leaves to Rust where parity already exists and retaining only the wrappers that are explicitly intentional.
- Normalized Request: create a planning workstream for the remaining script retirement tail so the repository can eventually delete the last `.ps1` leaves safely.
- Active Branch: `main` (planning only; implementation branches TBD)
- Spec Path: `planning/specs/active/spec-script-retirement-tail-cutover.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/completed/rust-script-transcription-ownership-matrix.md`
  - `scripts/`
  - `crates/commands/runtime/`
  - `crates/commands/validation/`
  - `crates/orchestrator/tests/execution/pipeline_parity/`

---

## Scope Summary

This plan coordinates four cutover slices:

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| S1 | Remaining runtime wrappers | `scripts/runtime/*.ps1` | 🔴 Immediate | previous runtime retirement phases |
| S2 | Remaining validation wrappers | `scripts/validation/*.ps1` | 🔴 Immediate | previous validation retirement phases |
| S3 | Remaining maintenance/parity wrappers | `scripts/maintenance` and `scripts/tests/runtime` | 🟠 High | S1, S2 |
| S4 | Final retention audit and branch closeout | safety matrix + parity ledger | 🟠 High | S1, S2, S3 |

---

## Ordered Tasks

### [2026-03-30 07:31] Task S1: Categorize The Remaining Runtime Tail

- Refresh the remaining runtime script inventory.
- Separate delete-now candidates from intentionally retained compatibility wrappers.
- Record the current tail count and the expected retention floor.
- Commit checkpoint:
  - `docs(planning): baseline remaining runtime script tail`

### [2026-03-30 07:31] Task S2: Retire The Remaining Validation Tail

- Identify the last validation wrappers that can move to native Rust surfaces.
- Keep the cutover atomic per slice so the validation chain stays reviewable.
- Update the parity ledger when a wrapper is removed.
- Commit checkpoint:
  - `refactor(validation): retire next script-tail validation wrapper slice`

### [2026-03-30 07:31] Task S3: Finish Maintenance And Parity Retainers

- Reassess the maintenance scripts and parity harness wrappers.
- Retain only the leaves that are still required as explicit compatibility/operator entrypoints.
- Keep the test harnesses in sync with any wrapper deletion.
- Commit checkpoint:
  - `refactor(runtime): retire remaining maintenance and parity wrappers`

### [2026-03-30 07:31] Task S4: Close The Final Retention Audit

- Confirm the live estate matches the retention matrix.
- Rebaseline the safety matrix and parity ledger after each removal slice.
- Close the workstream only when the remaining `.ps1` estate is intentional.
- Commit checkpoint:
  - `docs(planning): close script retirement tail cutover`

---

## Validation Checklist

- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `pwsh -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- `git diff --check`
- targeted PowerShell parity tests for any deleted wrapper slice

---

## Risks And Mitigations

- Retiring wrappers too quickly can break operator entrypoints or CI parity tests.
- The remaining tail includes intentional compatibility surfaces that must not be deleted by accident.
- Mitigation: keep the safety matrix authoritative and only delete leaves that have a Rust-owned replacement plus test coverage.

---

## Specialist And Closeout

- Recommended specialist: `dev-rust-engineer`
- Tester: required
- Reviewer: required
- Release closeout: required
- README update: required if wrapper behavior or operator guidance changes
- Changelog: required once a deletion slice lands
- Suggested commit message style:
  - `refactor(runtime): retire remaining script tail slice`
  - `docs(planning): record script retirement tail roadmap`