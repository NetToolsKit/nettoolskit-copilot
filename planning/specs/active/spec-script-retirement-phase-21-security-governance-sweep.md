# Spec: Phase 21 Security And Governance Consumer Sweep

Generated: 2026-04-05

## Status

- LastUpdated: 2026-04-05 16:30
- Objective: define the safe execution model for auditing `scripts/security/*.ps1` and `scripts/governance/*.ps1` before any domain deletion is allowed.
- Planning Readiness: ready-for-execution-planning
- Related Plan: `planning/active/plan-script-retirement-phase-21-security-governance-sweep.md`
- Source Inputs:
  - `planning/completed/plan-script-retirement-phase-20-runtime-consumer-sweep.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `definitions/providers/github/governance/shared-script-checksums.manifest.json`
  - `scripts/security/*.ps1`
  - `scripts/governance/*.ps1`

## Problem Statement

The runtime domain is now fully audited, but the next eight leaves in `scripts/security/*.ps1` and `scripts/governance/*.ps1` still sit in `retain until consumer migration completes`. These scripts have native Rust ownership at the domain level, yet they are also tied to explicit security and governance evidence surfaces, especially the shared checksum manifest for the security domain.

Without a dedicated Phase 21 execution model, this domain would either be deleted unsafely or remain indefinitely blocked behind generic backlog wording. The plan needs to make checksum-governance explicit so a delete cannot silently desynchronize the policy baseline.

## Desired Outcome

- Every security/governance leaf has concrete local-consumer evidence.
- Security deletions, if any, update the shared checksum manifest in the same slice.
- Governance deletions, if any, update the affected policy/release baselines in the same slice.
- The continuity workstream can treat Phase 21 as explicit evidence whether the result is deletion or audit-only retention.

## Design Decision

Run Phase 21 as a combined security/governance consumer sweep because both domains are small, adjacent, and governed by shared policy artifacts. Keep the checksum manifest as a first-class acceptance rule for the security leaves instead of treating it as post-hoc cleanup.

## Alternatives Considered

1. Split security and governance into separate phases immediately
   - Rejected because the current scope is only eight leaves and they already share governance/policy baselines.

2. Delete security leaves first and update the checksum manifest later
   - Rejected because it risks leaving the repository in a false-compliant state between commits.

3. Skip the checksum manifest and trust native validation alone
   - Rejected because the manifest is still an authored policy contract and must stay accurate until that policy lane is migrated.

## Risks

- The checksum manifest may block every security leaf, yielding another audit-only phase.
- Security/governance policy baselines may still encode script paths even when the native owner exists.
- The vulnerability-audit launcher may be intentionally retained as an operator-facing compatibility surface longer than the rest of the domain.

## Acceptance Criteria

- A dedicated active plan exists for the eight security/governance leaves.
- The plan makes checksum-manifest synchronization mandatory for any security deletion.
- The plan explicitly allows an audit-only closeout if blockers remain real.
- The continuity workstream points to Phase 21 as the next domain after the closed Phase 20 runtime sweep.