# Phase 20e: Catalog Native Renderer Dispatch and Codex Compatibility Retirement

Generated: 2026-03-29

## Status

- LastUpdated: 2026-03-29 13:05
- Objective: teach the shared provider-surface projection catalog to dispatch native `ntk runtime render-provider-surfaces` renderer leaves, then retire the local `scripts/runtime/render-codex-compatibility-surfaces.ps1` wrapper in the same slice.
- Normalized Request: continue the aggressive script-retirement flow, keep planning updated, and commit each stable phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-20e-catalog-native-renderer-dispatch.md`
- Inputs:
  - `planning/active/plan-repository-consolidation-continuity.md`
  - `planning/specs/active/spec-repository-consolidation-continuity.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `.github/governance/provider-surface-projection.catalog.json`
  - `.github/schemas/provider-surface-projection.catalog.schema.json`
  - `scripts/common/provider-surface-catalog.ps1`
  - `scripts/runtime/render-codex-compatibility-surfaces.ps1`
  - `scripts/tests/runtime/runtime-scripts.tests.ps1`
  - `crates/commands/runtime/src/sync/provider_surfaces.rs`
  - `crates/cli/src/runtime_commands.rs`

## Scope Summary

1. provider-surface projection catalog schema
2. shared `provider-surface-catalog.ps1` dispatcher helper
3. `codex-compatibility-surfaces` renderer catalog entry
4. runtime parity coverage for the native renderer path
5. `scripts/runtime/render-codex-compatibility-surfaces.ps1`

This phase is complete only if:

- the projection catalog can describe a native provider-surface renderer without a `.ps1` leaf
- the shared PowerShell catalog helper can dispatch the native runtime command without losing `direct` and `bootstrap` semantics
- runtime parity coverage stops requiring the deleted local wrapper
- the local Codex compatibility renderer wrapper is deleted in the same slice
- continuity planning and retirement evidence reflect the post-cutover 97-script baseline

## Ordered Tasks

### Task 1: Add Native Renderer Metadata To The Catalog Contract

Status: `[x]` Completed

- Extend `.github/schemas/provider-surface-projection.catalog.schema.json` so a renderer can be described by either:
  - `scriptPath`
  - or native runtime renderer metadata for `ntk runtime render-provider-surfaces`
- Keep the contract backward compatible for the remaining script-backed renderers.

### Task 2: Teach The Shared Catalog Helper To Dispatch Native Renderers

Status: `[x]` Completed

- Update `scripts/common/provider-surface-catalog.ps1` so it can dispatch:
  - script-backed renderers through `scriptPath`
  - native provider-surface renderers through `ntk runtime render-provider-surfaces`
- Preserve current `direct` / `bootstrap` selection semantics and renderer ordering.

### Task 3: Retire The Codex Compatibility Wrapper

Status: `[x]` Completed

- Move the `codex-compatibility-surfaces` renderer entry in `.github/governance/provider-surface-projection.catalog.json` onto the native dispatcher contract.
- Repoint runtime parity coverage away from the deleted wrapper.
- Delete `scripts/runtime/render-codex-compatibility-surfaces.ps1`.

### Task 4: Rebaseline Continuity And Archive

Status: `[x]` Completed

- Update `planning/completed/script-retirement-safety-matrix.md`.
- Update `planning/completed/rust-script-parity-ledger.md`.
- Update the active continuity plan/spec to inherit the post-cutover 97-script baseline.
- Move this plan/spec to `planning/completed/` and `planning/specs/completed/` after validation passes.

## Validation Checklist

- [x] `cargo test -p nettoolskit-runtime --test test_suite sync::provider_surfaces_tests --quiet`
- [x] `cargo test -p nettoolskit-cli --test runtime_commands_tests --quiet`
- [x] `pwsh -NoProfile -File .\scripts\tests\runtime\runtime-scripts.tests.ps1`
- [x] `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- [x] `git diff --check`

## Risks And Fallbacks

- The shared provider-surface catalog helper still documents script-backed execution, so changing it without a backward-compatible mixed model would break remaining renderer leaves.
- The native `render-provider-surfaces` contract is narrower than the deleted wrapper and does not preserve the old ad hoc override parameters; the cutover is only acceptable because the live local consumers now depend on the canonical repo-root projection behavior.

## Closeout Expectations

- Keep the phase-opening planning commit, the native catalog/helper plus wrapper-retirement code commit, and the planning closeout commit separate.
- Archive the phase only after the deleted wrapper is removed and the continuity baseline reflects the new 97-script live estate.

## Executed Result

- The provider-surface projection catalog now supports mixed script-backed and native runtime renderer dispatch.
- The shared `provider-surface-catalog.ps1` helper can invoke `ntk runtime render-provider-surfaces` for native leaves while preserving the remaining script-backed renderers.
- The `codex-compatibility-surfaces` catalog entry now dispatches natively, and `scripts/runtime/render-codex-compatibility-surfaces.ps1` was removed in the same slice.
- The live local `scripts/**/*.ps1` estate dropped from `98` to `97`, and the continuity backlog now reflects `64` `retain until consumer migration completes` leaves.