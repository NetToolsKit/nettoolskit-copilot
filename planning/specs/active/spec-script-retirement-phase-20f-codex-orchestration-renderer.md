# Spec: Phase 20f Codex Orchestration Renderer Retirement

Generated: 2026-03-29

## Status

- LastUpdated: 2026-03-29 13:20
- Objective: define the safe cutover conditions for deleting `scripts/runtime/render-codex-orchestration-surfaces.ps1` after the provider-surface catalog gained native renderer support.
- Planning Readiness: ready-for-implementation
- Related Plan: `planning/active/plan-script-retirement-phase-20f-codex-orchestration-renderer.md`
- Source Inputs:
  - `planning/active/plan-repository-consolidation-continuity.md`
  - `planning/specs/active/spec-repository-consolidation-continuity.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `.github/governance/provider-surface-projection.catalog.json`
  - `scripts/runtime/render-codex-orchestration-surfaces.ps1`
  - `scripts/tests/runtime/runtime-scripts.tests.ps1`
  - `.codex/orchestration/README.md`
  - `definitions/providers/codex/orchestration/README.md`
  - `crates/commands/runtime/src/sync/provider_surfaces.rs`
  - `crates/cli/src/runtime_commands.rs`

## Problem Statement

The native `ntk runtime render-provider-surfaces` boundary already owns the Codex orchestration renderer behavior, but the local repository still keeps `render-codex-orchestration-surfaces.ps1` as a compatibility wrapper. After Phase 20e, the provider-surface catalog can dispatch native leaves directly, so the remaining blockers are limited to runtime parity coverage and the Codex orchestration README surfaces that still advertise the deleted wrapper path.

## Desired Outcome

- The `codex-orchestration-surfaces` renderer entry dispatches natively through `ntk runtime render-provider-surfaces`.
- Runtime parity coverage stops requiring `scripts/runtime/render-codex-orchestration-surfaces.ps1`.
- The authored and projected Codex orchestration READMEs advertise the native renderer contract instead of the deleted wrapper.
- The local wrapper is deleted in the same slice.

## Design Decision

Treat the Codex orchestration renderer as the first pure provider-surface leaf that benefits from the mixed native catalog model added in Phase 20e. This phase changes only the catalog entry, parity coverage, and the two user-facing README surfaces before deleting the local wrapper.

## Alternatives Considered

1. Keep the wrapper until every remaining provider-surface renderer leaf is ready
   - Rejected because the native catalog contract already makes this leaf independently deletable.
2. Delete the wrapper without updating the authored and projected README surfaces
   - Rejected because it would leave operator guidance pointing at a dead script path.

## Risks

- The native renderer contract must still emit the Codex orchestration README, prompts, and template outputs expected by the parity harness.
- The authored and projected READMEs must remain aligned so the next provider-surface re-render does not reintroduce the deleted wrapper path.

## Acceptance Criteria

- `.github/governance/provider-surface-projection.catalog.json` records `codex-orchestration-surfaces` as a native renderer.
- Runtime parity tests prove the native Codex orchestration renderer path.
- `.codex/orchestration/README.md` and `definitions/providers/codex/orchestration/README.md` advertise the native renderer contract.
- `scripts/runtime/render-codex-orchestration-surfaces.ps1` is deleted.
- The safety matrix, parity ledger, and continuity plan/spec reflect a 96-script live estate and 63 `retain until consumer migration completes`.