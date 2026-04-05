# Spec: Phase 20 Runtime Consumer Sweep

Generated: 2026-04-05

## Status

- LastUpdated: 2026-04-05 12:18
- Objective: define the execution model for the remaining 30 `scripts/runtime/*.ps1` leaves after the tactical 20c/20d/20e/20f runtime slices retired isolated native-ready wrappers.
- Planning Readiness: ready-for-execution-planning
- Related Plan: `planning/active/plan-script-retirement-phase-20-runtime-consumer-sweep.md`
- Source Inputs:
  - `planning/active/plan-repository-consolidation-continuity.md`
  - `planning/specs/active/spec-repository-consolidation-continuity.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/runtime/`
  - `scripts/tests/runtime/*.ps1`
  - `definitions/providers/github/governance/provider-surface-projection.catalog.json`

## Problem Statement

The runtime domain still contains 30 local PowerShell compatibility leaves even after the tactical 20c self-heal, 20d provider-surface dispatcher, 20e catalog-native renderer, and 20f Codex orchestration renderer cutovers. Rust ownership is already proven at the domain level, but deletion safety is still blocked by local docs, tests, fixtures, policies, provider catalogs, and compatibility entrypoints that continue to encode `.ps1` paths.

Without a dedicated Phase 20 execution model, this remaining runtime domain will stay stuck as one oversized backlog item. The retirement program now needs a stable plan that groups the remaining leaves into coherent consumer-sweep slices and allows partial retirement without reopening the already-closed tactical runtime work.

## Desired Outcome

- The remaining 30 runtime leaves are grouped into coherent execution slices with deterministic search boundaries.
- Each slice proves exact local consumers before any deletion is attempted.
- Partial deletion is allowed when only part of a slice reaches zero-consumer readiness.
- The plan naming no longer collides with the already-completed tactical Phase 20c self-heal slice.
- Every executed slice rebaselines the safety matrix and parity ledger immediately.

## Design Decision

Use one active Phase 20 plan with three internal slices instead of opening multiple new `phase-20a/20b/20c` plan files. The internal slice names are:

1. Slice A — projection, profile, sync, and workspace runtime surfaces
2. Slice B — orchestration runtime entrypoints and replay helpers
3. Slice C — bootstrap, install, and cleanup surfaces

This keeps the runtime-domain sweep cohesive while avoiding name collisions with the already-archived tactical `Phase 20c self-heal` workstream.

## Alternatives Considered

1. Open three new plan files named `phase-20a`, `phase-20b`, and `phase-20c`
   - Rejected because `Phase 20c` already exists as a completed tactical self-heal slice, and reusing that label would make planning history ambiguous.

2. Keep the whole runtime domain under the umbrella continuity plan only
   - Rejected because the remaining 30 leaves are too large and too heterogeneous for one checklist without a dedicated runtime-domain plan.

3. Force a single deletion batch for all 30 scripts
   - Rejected because the current blocker profile is uneven; some leaves are likely deletion-ready much earlier than bootstrap/install or orchestration entrypoints.

## Risks

- Projection and sync leaves may still be blocked by provider catalogs, authored/provider READMEs, and runtime parity fixtures.
- Orchestration entrypoints may still be blocked by orchestrator parity tests, validation fixtures, and policy baselines.
- Bootstrap/install/cleanup leaves are likely the highest-fanout compatibility surfaces and may remain intentionally retained after the first sweep.
- If the plan does not allow partial retirement, the runtime domain could stall behind one or two high-fanout blockers.

## Acceptance Criteria

- A dedicated active plan exists for the remaining runtime-domain sweep.
- The plan freezes the 30-leaf inventory and groups it into three internal slices.
- The plan explicitly allows partial retirement and evidence-only slice outcomes.
- The umbrella continuity plan points to this active plan as the Phase 20 execution surface.
- `planning/README.md` and `planning/specs/README.md` list the new active plan and spec.