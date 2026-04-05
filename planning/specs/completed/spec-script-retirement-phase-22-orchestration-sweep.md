# Spec: Phase 22 Orchestration Consumer Sweep

Generated: 2026-04-05

## Status

- LastUpdated: 2026-04-05 17:45
- Objective: define the safe execution model for auditing `scripts/orchestration/**/*.ps1` before any orchestration-wrapper deletion is allowed.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-22-orchestration-sweep.md`
- Source Inputs:
  - `planning/completed/plan-script-retirement-phase-21-security-governance-sweep.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/orchestration/**/*.ps1`
  - `definitions/providers/codex/orchestration/**/*`
  - `crates/orchestrator/tests/execution/pipeline_parity/**/*`

## Problem Statement

The final `retain until consumer migration completes` domain is `scripts/orchestration/**/*.ps1`. Native parity is already proven in the orchestrator and validation crates, but the local wrappers still appear in authored orchestration pipelines, policy baselines, validation fixtures, runtime tests, and stage-to-stage chaining.

Without a dedicated Phase 22 execution model, the consumer-migration backlog cannot close cleanly. The repository would keep one unverified orchestration bucket open, and the final retention audit would have no stable evidence surface to reference.

## Desired Outcome

- Every orchestration wrapper has concrete local-consumer evidence.
- No orchestration wrapper is deleted without zero non-self consumer proof.
- The continuity workstream can close the consumer-migration sequence with explicit orchestration evidence, whether the result is deletion or audit-only retention.

## Design Decision

Run Phase 22 as the final orchestration-domain consumer sweep and allow an audit-only closeout if every wrapper remains pinned. This keeps the final retention audit evidence-first instead of forcing an artificial delete target.

## Alternatives Considered

1. Skip a dedicated Phase 22 and go straight to the final retention audit
   - Rejected because the last unresolved domain would remain undocumented.

2. Delete low-fanout orchestration wrappers opportunistically
   - Rejected because the stage scripts and engine wrappers are mutually entangled through pipelines and validation fixtures.

3. Fold orchestration evidence into Phase 20 or Phase 21
   - Rejected because runtime and security/governance already closed as their own audit domains and the orchestration domain deserves a standalone blocker graph.

## Risks

- The default Codex orchestration pipeline may pin almost every stage wrapper.
- Validation fixtures and parity harnesses may continue to require the local script names even when native parity exists.
- `validate-stage.ps1` may be blocked by historical closeout evidence as well as live fixtures.

## Acceptance Criteria

- A dedicated active plan exists for the 10 orchestration wrappers.
- The plan explicitly allows an audit-only closeout if blockers remain real.
- The continuity workstream points to Phase 22 as the final domain before the post-Phase-22 retention audit.

## Executed Result

- The consumer sweep confirmed that all 10 orchestration wrappers still have live local consumers in authored pipeline definitions, policy baselines, orchestrator/validation fixtures, retained runtime tests, or stage chaining.
- No orchestration leaf met the zero-consumer delete rule, so the phase closed as audit-only.