# Script Retirement Tail Cutover Plan

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-04-05 17:20
- Objective: close the planning umbrella for the remaining `scripts/*.ps1` retirement tail after the Phase 19-22 consumer sweeps and the post-Phase-22 retention audit fully categorized the retained estate.
- Normalized Request: create a planning workstream for the remaining script retirement tail so the repository can eventually delete the last `.ps1` leaves safely.
- Active Branch: `docs/planning-gap-workstreams` (planning closeout)
- Spec Path: `planning/specs/completed/spec-script-retirement-tail-cutover.md`
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

## Closeout Summary

- Phase 19 completed as an audit-only common-domain consumer sweep.
- Phase 20 completed as an audit-only runtime-domain consumer sweep with explicit Slice A/B/C blocker evidence.
- Phase 21 completed as an audit-only security/governance consumer sweep with checksum-governed security retention.
- Phase 22 completed as an audit-only orchestration consumer sweep with authored pipeline, policy, and parity blockers recorded.
- The post-Phase-22 retention audit closed the planning sequence by proving that the remaining `96` live PowerShell scripts are fully explained by the intentional-retain floor plus the five audited blocked domains.
- Further progress now belongs to blocker-reduction implementation workstreams, not to another generic script-tail planning umbrella.

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

- Closeout: completed by the Phase 19-22 consumer-sweep archive set plus the post-Phase-22 retention audit.
- Refresh the remaining runtime script inventory.
- Separate delete-now candidates from intentionally retained compatibility wrappers.
- Record the current tail count and the expected retention floor.
- Commit checkpoint:
  - `docs(planning): baseline remaining runtime script tail`

### [2026-03-30 07:31] Task S2: Retire The Remaining Validation Tail

- Closeout: completed by the earlier validation retirement phases and the final retention audit evidence model.
- Identify the last validation wrappers that can move to native Rust surfaces.
- Keep the cutover atomic per slice so the validation chain stays reviewable.
- Update the parity ledger when a wrapper is removed.
- Commit checkpoint:
  - `refactor(validation): retire next script-tail validation wrapper slice`

### [2026-03-30 07:31] Task S3: Finish Maintenance And Parity Retainers

- Closeout: completed by the audit-only consumer sweeps, which proved the retained maintenance/parity wrappers are still blocked intentionally.
- Reassess the maintenance scripts and parity harness wrappers.
- Retain only the leaves that are still required as explicit compatibility/operator entrypoints.
- Keep the test harnesses in sync with any wrapper deletion.
- Commit checkpoint:
  - `refactor(runtime): retire remaining maintenance and parity wrappers`

### [2026-03-30 07:31] Task S4: Close The Final Retention Audit

- Closeout: completed by the recorded `96`-script retained-estate baseline and grouped blocked-domain accounting.
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