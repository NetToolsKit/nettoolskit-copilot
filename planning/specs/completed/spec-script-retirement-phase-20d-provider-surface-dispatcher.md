# Spec: Phase 20d Provider Surface Dispatcher Retirement

Generated: 2026-03-29

## Status

- LastUpdated: 2026-03-29 12:20
- Objective: define the cutover conditions for deleting `scripts/runtime/render-provider-surfaces.ps1` once the native `ntk runtime render-provider-surfaces` contract, runtime parity evidence, and continuity planning all converge on Rust ownership.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-20d-provider-surface-dispatcher.md`
- Source Inputs:
  - `planning/active/plan-repository-consolidation-continuity.md`
  - `planning/specs/active/spec-repository-consolidation-continuity.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/runtime/render-provider-surfaces.ps1`
  - `crates/commands/runtime/src/sync/provider_surfaces.rs`
  - `crates/cli/src/runtime_commands.rs`

## Problem Statement

The runtime crate already owns the provider-surface rendering logic, but the repository still carries a local PowerShell dispatcher that remains encoded in runtime parity coverage, bootstrap compatibility glue, and reference docs. That leaves the runtime-domain consumer sweep stalled on a wrapper that no longer needs to remain canonical once a native operator-facing dispatcher exists.

## Desired Outcome

- `ntk runtime render-provider-surfaces` becomes the canonical operator-facing dispatcher.
- The native command preserves the current wrapper semantics for consumer selection, renderer filtering, bootstrap gating, and summary-only inspection.
- No non-planning consumer in this repository still requires `scripts/runtime/render-provider-surfaces.ps1`.
- The local dispatcher wrapper is deleted in the same slice that repoints parity, bootstrap, and docs.

## Design Decision

Treat `render-provider-surfaces` as a tactical Phase 20d runtime leaf cutover. The broader Phase 20 runtime-domain sweep remains open, but this dispatcher is the smallest remaining runtime blocker with a stable Rust implementation already present in `crates/commands/runtime`. The native boundary must stay narrower than `bootstrap`, so the new command dispatches only renderer selection and render execution without syncing runtime assets.

## Alternatives Considered

1. Keep the dispatcher wrapper until the entire runtime domain sweep closes
   - Rejected because the wrapper is now a pure coordination layer with small same-slice consumer fanout.
2. Reuse `ntk runtime bootstrap` as the public dispatcher contract
   - Rejected because `bootstrap` also syncs runtime assets and optionally applies MCP config, which would change operator semantics relative to the wrapper being retired.
3. Delete the wrapper without same-slice parity and planning updates
   - Rejected because it would leave stale parity evidence and a false continuity baseline behind.

## Risks

- The direct consumer path still includes `mcp-runtime-artifacts`; the native dispatcher must support that renderer instead of silently dropping it.
- The retained runtime parity harness must move from script-parameter inspection to native CLI coverage without losing operator-path proof.
- The active continuity plan/spec must be rebaselined in the same slice so the open retirement backlog stays numerically correct.

## Acceptance Criteria

- Native runtime and CLI tests cover direct, bootstrap-gated, filtered, and summary-only dispatcher behavior.
- `scripts/runtime/bootstrap.ps1` dispatches through `ntk runtime render-provider-surfaces`.
- Runtime parity coverage and reference docs stop requiring `scripts/runtime/render-provider-surfaces.ps1`.
- The local wrapper is deleted.
- The safety matrix, parity ledger, and continuity plan/spec reflect a 98-script live estate and 65 `retain until consumer migration completes`.

## Executed Result

- The native dispatcher landed in `crates/commands/runtime` and `crates/cli` with deterministic runtime and CLI coverage.
- Same-slice consumer repoints were completed in bootstrap, runtime parity tests, authored docs, and orchestrator parity support.
- `scripts/runtime/render-provider-surfaces.ps1` was deleted after the native dispatcher became canonical.
- The continuity plan/spec, safety matrix, and parity ledger now inherit the `99 -> 98` live-estate rebaseline and the `66 -> 65` consumer-migration backlog change.