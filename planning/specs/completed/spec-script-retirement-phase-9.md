# Spec: Phase 9 Validation Wrapper Retirement Readiness

Generated: 2026-03-28 21:41

## Status

- LastUpdated: 2026-03-28 21:55
- Objective: define the design intent and acceptance criteria for retiring the remaining validation wrappers for agent orchestration and release governance/provenance.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-9.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/completed/rust-script-cutover-default-map.md`
  - `planning/completed/plan-script-retirement-phase-8.md`
  - `definitions/providers/codex/orchestration/README.md`
  - `definitions/providers/codex/skills/privacy-compliance-engineer/SKILL.md`
  - `definitions/providers/claude/skills/privacy-compliance-engineer/SKILL.md`
  - `definitions/providers/claude/skills/sec-security-vulnerability-engineer/SKILL.md`

## Problem Statement

The repository still carries three local validation wrappers as compatibility launch surfaces:

1. `scripts/validation/validate-agent-orchestration.ps1`
2. `scripts/validation/validate-release-governance.ps1`
3. `scripts/validation/validate-release-provenance.ps1`

The Rust implementations already exist, but local documentation and orchestration references still treat the wrapper paths as canonical in some places. This creates a retirement gap: the code is ready, but the wrapper paths are still visible in the operating model.

## Desired Outcome

- `ntk validation` is the authoritative executable contract for the three checks.
- Documentation and provider-authored examples point to the native command surface.
- Live orchestration consumers no longer require the local wrapper paths.
- The wrappers are removed only after the consumer chain and evidence surfaces are updated.

## Design Decision

Use the native Rust validation commands as the source of truth and keep any temporary compatibility behavior strictly in the PowerShell orchestration layer until the final consumer cutover is validated.

This keeps the behavior stable while allowing the local `.ps1` leaves to be deleted without losing the operational contract.

## Alternatives Considered

1. Keep the wrappers indefinitely
   - Rejected because it preserves duplicate behavior and keeps the repository tied to local shell launchers.
2. Delete the wrappers immediately
   - Rejected because several orchestration and documentation surfaces still refer to them.
3. Repoint docs and orchestration first, then delete the wrappers
   - Selected because it preserves the operating contract while removing the compatibility debt safely.

## Risks

- Orchestration scripts may still hardcode wrapper names after docs are updated.
- Provider-authored examples may drift if they are not changed in the same slice.
- Release governance evidence may continue to name the old wrappers until the baselines are refreshed.
- A partially updated retirement slice could make the repository look compliant while still exposing the wrapper paths in operator guidance.

## Acceptance Criteria

- Native validation commands exist for all three checks and are covered by tests.
- `validate-all.ps1` and the relevant orchestration consumers invoke the native commands rather than the wrappers.
- Provider docs and examples no longer present the wrappers as the recommended execution path.
- The three wrapper files are deleted only after the above conditions are met.
- The completed safety matrix and parity ledger record the phase-9 result and live inventory change.

## Executed Result

The three validation wrappers were retired after the native `ntk validation` surface, the PowerShell consumer chain, the governance evidence, and the projected provider surfaces were all updated in the same slice. Focused `validate-all` warning-only proof passed with the native routing active, and enforcing mode exposed only pre-existing repository release-baseline debt rather than cutover regressions.

## Planning Readiness

- The scope was narrow enough to execute in one retirement phase while still requiring a staged validation gate.
- No further architecture decision remains open for this slice.
- The spec is archived with the completed Phase 9 plan.