# Spec: Phase 20e Catalog Native Renderer Dispatch

Generated: 2026-03-29

## Status

- LastUpdated: 2026-03-29 13:05
- Objective: define the safe cutover conditions for retiring `scripts/runtime/render-codex-compatibility-surfaces.ps1` by teaching the shared provider-surface catalog to dispatch native runtime renderers.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-20e-catalog-native-renderer-dispatch.md`
- Source Inputs:
  - `planning/completed/plan-repository-consolidation-continuity.md`
  - `planning/specs/completed/spec-repository-consolidation-continuity.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `.github/governance/provider-surface-projection.catalog.json`
  - `.github/schemas/provider-surface-projection.catalog.schema.json`
  - `scripts/common/provider-surface-catalog.ps1`
  - `scripts/runtime/render-codex-compatibility-surfaces.ps1`
  - `scripts/tests/runtime/runtime-scripts.tests.ps1`
  - `crates/commands/runtime/src/sync/provider_surfaces.rs`
  - `crates/cli/src/runtime_commands.rs`

## Problem Statement

The native `ntk runtime render-provider-surfaces` boundary already owns the Codex compatibility renderer behavior, but the repository still keeps the local `render-codex-compatibility-surfaces.ps1` wrapper because the shared provider-surface projection catalog is still script-path-only. That makes the leaf impossible to retire cleanly without either leaving stale governance metadata behind or teaching the shared helper to dispatch native renderers.

## Desired Outcome

- The provider-surface projection catalog can express native runtime renderer ownership for selected leaves.
- The shared PowerShell catalog helper remains backward compatible for script-backed renderers while dispatching native provider-surface renderers through `ntk runtime render-provider-surfaces`.
- Runtime parity coverage stops requiring `scripts/runtime/render-codex-compatibility-surfaces.ps1`.
- The local Codex compatibility renderer wrapper is deleted in the same slice.

## Design Decision

Adopt a mixed catalog model. Script-backed renderers keep `scriptPath`, but native leaves can opt into a new native-renderer contract that dispatches `ntk runtime render-provider-surfaces` using the renderer id already recorded in the catalog. This keeps the catalog authoritative, preserves the shared PowerShell helper as a compatibility surface, and avoids inventing one-off wrapper exceptions for renderers that already have stable Rust ownership.

## Alternatives Considered

1. Keep deleting provider-surface leaves only after the entire common catalog helper is retired
   - Rejected because it would stall multiple low-fanout renderer retirements behind one larger common-domain sweep.
2. Delete the local wrapper and leave the catalog pointing at a dead `scriptPath`
   - Rejected because it would leave governance metadata stale and make manual/shared helper execution fail.
3. Replace the shared PowerShell helper entirely in this phase
   - Rejected because the remaining renderer leaves are still intentionally mixed between script-backed and native-backed execution.

## Risks

- The mixed model must not break the remaining script-backed renderer leaves or reorder the bootstrap/direct selection semantics.
- The native renderer contract is narrower than the deleted wrapper and no longer preserves the old custom output-path override parameters.
- The continuity backlog and safety matrix must be rebaselined in the same slice so the runtime-domain count stays numerically correct.

## Acceptance Criteria

- `.github/schemas/provider-surface-projection.catalog.schema.json` allows a renderer to be described through native runtime metadata without `scriptPath`.
- `scripts/common/provider-surface-catalog.ps1` can dispatch both script-backed and native provider-surface renderers.
- `.github/governance/provider-surface-projection.catalog.json` records `codex-compatibility-surfaces` as a native renderer.
- Runtime parity tests and operator smoke checks prove the native Codex compatibility renderer path.
- `scripts/runtime/render-codex-compatibility-surfaces.ps1` is deleted.
- The safety matrix, parity ledger, and continuity plan/spec reflect a 97-script live estate and 64 `retain until consumer migration completes`.

## Executed Result

- The catalog schema now allows native provider-surface renderer metadata without a local `scriptPath`.
- The shared PowerShell catalog helper now dispatches native renderer leaves through `ntk runtime render-provider-surfaces` while preserving the remaining script-backed renderers.
- The `codex-compatibility-surfaces` renderer moved onto the native dispatcher contract, runtime parity coverage was repointed, and `scripts/runtime/render-codex-compatibility-surfaces.ps1` was deleted.
- The continuity plan/spec, safety matrix, and parity ledger now inherit the `98 -> 97` live-estate rebaseline and the `65 -> 64` consumer-migration backlog change.