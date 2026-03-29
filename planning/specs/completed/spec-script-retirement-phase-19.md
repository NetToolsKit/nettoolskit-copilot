# Spec: Phase 19 Common Helper Consumer Audit

Generated: 2026-03-29 09:24

## Status

- LastUpdated: 2026-03-29 09:24
- Objective: define the safe execution rules for auditing `scripts/common/*.ps1` before any shared-helper deletion is allowed.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-19.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/common/*.ps1`
  - `scripts/runtime/*.ps1`
  - `scripts/orchestration/**/*.ps1`

## Problem Statement

The shared-helper domain already has native Rust ownership across `crates/core` and related runtime/validation crates, but shared helpers remain dot-sourced by multiple PowerShell domains. A domain-level consumer sweep is required before any delete is even considered. Without that evidence, the common domain stays permanently ambiguous and blocks the downstream runtime sweeps from having a trustworthy baseline.

## Desired Outcome

- Every `scripts/common/*.ps1` file is classified with concrete local-consumer evidence.
- No common helper is deleted unless zero non-self consumers are proven.
- The continuity workstream can treat the common domain as audited, even if the result is zero deletions.

## Design Decision

Run Phase 19 as an audit-first domain sweep and accept an audit-only closeout when blockers are real. Shared helpers are too foundational to force into a delete-only outcome. The design prefers explicit blocker evidence over unstable partial deletions.

## Alternatives Considered

1. Skip the common-domain sweep and move straight to runtime/security/orchestration deletions
   - Rejected because the shared-helper domain would remain an unverified blind spot.
2. Delete low-fanout helpers opportunistically without full consumer proof
   - Rejected because several seemingly low-fanout helpers still anchor critical runtime flows or parity fixtures.

## Risks

- `common-bootstrap.ps1` fanout is high enough that a mistaken delete would cascade across multiple domains.
- Several helpers are blocked by retained runtime compatibility surfaces, so the audit must distinguish temporary tactical blockers from actual deletion readiness.

## Acceptance Criteria

- All 15 common helpers have recorded consumer evidence.
- Zero unsafe deletions are attempted.
- The continuity plan can reference Phase 19 as completed evidence.

## Executed Result

- The consumer sweep confirmed that all 15 common helpers still have live local consumers.
- No helper met the zero-consumer delete rule, so the phase closed as audit-only.
- The common domain remains in `retain until consumer migration completes`, but the blocker baseline is now explicit and reusable.