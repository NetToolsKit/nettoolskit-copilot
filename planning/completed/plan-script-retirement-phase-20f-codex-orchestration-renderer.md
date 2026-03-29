# Phase 20f: Codex Orchestration Renderer Retirement

Generated: 2026-03-29

## Status

- LastUpdated: 2026-03-29 13:35
- Objective: retire the local `scripts/runtime/render-codex-orchestration-surfaces.ps1` wrapper now that the provider-surface catalog supports native renderer dispatch.
- Normalized Request: continue the aggressive script-retirement flow, keep planning updated, and commit each stable phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-20f-codex-orchestration-renderer.md`
- Inputs:
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

## Scope Summary

1. `codex-orchestration-surfaces` renderer catalog entry
2. runtime parity coverage for the native renderer path
3. Codex orchestration README surfaces
4. `scripts/runtime/render-codex-orchestration-surfaces.ps1`

This phase is complete only if:

- the `codex-orchestration-surfaces` renderer dispatches natively through `ntk runtime render-provider-surfaces`
- runtime parity coverage stops requiring the deleted wrapper
- the authored and projected Codex orchestration READMEs stop advertising the deleted wrapper path
- the local wrapper is deleted in the same slice
- continuity planning and retirement evidence reflect the post-cutover 96-script baseline

## Ordered Tasks

### Task 1: Move The Catalog Entry To The Native Renderer Contract

Status: `[x]` Completed

- Update `.github/governance/provider-surface-projection.catalog.json` so `codex-orchestration-surfaces` dispatches natively.

### Task 2: Repoint Live Consumers

Status: `[x]` Completed

- Update runtime parity coverage in `scripts/tests/runtime/runtime-scripts.tests.ps1`.
- Update:
  - `.codex/orchestration/README.md`
  - `definitions/providers/codex/orchestration/README.md`

### Task 3: Delete The Local Wrapper

Status: `[x]` Completed

- Delete `scripts/runtime/render-codex-orchestration-surfaces.ps1`.

### Task 4: Rebaseline Continuity And Archive

Status: `[x]` Completed

- Update `planning/completed/script-retirement-safety-matrix.md`.
- Update `planning/completed/rust-script-parity-ledger.md`.
- Update the active continuity plan/spec to inherit the post-cutover 96-script baseline.
- Move this plan/spec to `planning/completed/` and `planning/specs/completed/` after validation passes.

## Validation Checklist

- [x] `cargo test -p nettoolskit-runtime --test test_suite sync::provider_surfaces_tests --quiet`
- [x] `cargo test -p nettoolskit-cli --test runtime_commands_tests --quiet`
- [x] `pwsh -NoProfile -File .\scripts\tests\runtime\runtime-scripts.tests.ps1`
- [x] `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- [x] `git diff --check`

## Risks And Fallbacks

- The Codex orchestration READMEs are operator-facing and must stop advertising the deleted wrapper in the same slice.
- This slice depends on the mixed native/script-backed provider-surface catalog support delivered in Phase 20e; if that foundation regresses, this phase must stop.

## Closeout Expectations

- Keep the phase-opening planning commit, the wrapper-retirement code/docs commit, and the planning closeout commit separate.
- Archive the phase only after the wrapper is deleted and the continuity baseline reflects the new 96-script live estate.

## Executed Result

- The `codex-orchestration-surfaces` catalog entry now dispatches natively through `ntk runtime render-provider-surfaces`.
- Runtime parity coverage and both authored/projected Codex orchestration README surfaces now point to the native renderer contract.
- `scripts/runtime/render-codex-orchestration-surfaces.ps1` was removed in the same slice.
- The live local `scripts/**/*.ps1` estate dropped from `97` to `96`, and the continuity backlog now reflects `63` `retain until consumer migration completes` leaves.