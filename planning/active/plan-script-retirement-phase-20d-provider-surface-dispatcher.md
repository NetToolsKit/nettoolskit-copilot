# Phase 20d: Provider Surface Dispatcher Retirement

Generated: 2026-03-29

## Status

- LastUpdated: 2026-03-29 11:35
- Objective: retire the local `scripts/runtime/render-provider-surfaces.ps1` dispatcher after the native `ntk runtime render-provider-surfaces` boundary, runtime parity harness, docs, and continuity evidence all converge on the Rust-owned contract.
- Normalized Request: continue the aggressive script-retirement flow, keep planning updated, and commit each stable phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/active/spec-script-retirement-phase-20d-provider-surface-dispatcher.md`
- Inputs:
  - `planning/active/plan-repository-consolidation-continuity.md`
  - `planning/specs/active/spec-repository-consolidation-continuity.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/runtime/render-provider-surfaces.ps1`
  - `scripts/runtime/bootstrap.ps1`
  - `scripts/tests/runtime/runtime-scripts.tests.ps1`
  - `crates/commands/runtime/src/sync/provider_surfaces.rs`
  - `crates/cli/src/runtime_commands.rs`

## Scope Summary

1. native `ntk runtime render-provider-surfaces`
2. `scripts/runtime/render-provider-surfaces.ps1`
3. `scripts/runtime/bootstrap.ps1`
4. `scripts/tests/runtime/runtime-scripts.tests.ps1`
5. `definitions/README.md`
6. `crates/orchestrator/tests/execution/pipeline_parity/support/validation_baseline.rs`

This phase is complete only if:

- the native dispatcher supports `catalog-path`, `renderer-id`, `consumer-name`, bootstrap gating, and `summary-only`
- runtime and CLI tests prove both direct and bootstrap behavior
- the runtime parity harness stops requiring the deleted `.ps1` path
- the local dispatcher wrapper is deleted in the same slice
- continuity planning and retirement evidence reflect the post-dispatcher baseline

## Ordered Tasks

### Task 1: Freeze The Native Dispatcher Boundary

Status: `[ ]` Pending

- Expose `ntk runtime render-provider-surfaces` as the managed dispatcher for provider-surface rendering.
- Preserve the PowerShell wrapper semantics for:
  - default `direct` consumer selection
  - explicit `renderer-id`
  - explicit `catalog-path`
  - bootstrap gating through `enable-codex-runtime` and `enable-claude-runtime`
  - `summary-only`
- Add deterministic runtime and CLI coverage for the new command surface.

### Task 2: Repoint Live Consumers To The Native Boundary

Status: `[ ]` Pending

- Repoint `scripts/runtime/bootstrap.ps1` to the managed runtime binary.
- Move the retained runtime parity harness from script-parameter inspection to native CLI coverage for the dispatcher.
- Replace direct wrapper evidence in:
  - `definitions/README.md`
  - `crates/orchestrator/tests/execution/pipeline_parity/support/validation_baseline.rs`

### Task 3: Delete The Local Wrapper

Status: `[ ]` Pending

- Delete `scripts/runtime/render-provider-surfaces.ps1` after the same-slice consumer repoints land.

### Task 4: Rebaseline Continuity And Archive

Status: `[ ]` Pending

- Update `planning/completed/script-retirement-safety-matrix.md`.
- Update `planning/completed/rust-script-parity-ledger.md`.
- Update the active continuity plan/spec to inherit the post-dispatcher 98-script baseline.
- Move this plan/spec to `planning/completed/` and `planning/specs/completed/` after validation passes.

## Validation Checklist

- [ ] `cargo test -p nettoolskit-runtime --test test_suite sync::provider_surfaces_tests --quiet`
- [ ] `cargo test -p nettoolskit-cli --test runtime_commands_tests --quiet`
- [ ] `pwsh -NoProfile -File .\scripts\tests\runtime\runtime-scripts.tests.ps1`
- [ ] `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- [ ] `git diff --check`

## Risks And Fallbacks

- The dispatcher must remain narrower than `bootstrap`; reusing `bootstrap` directly would over-apply runtime sync and MCP config side effects.
- The provider-surface parity harness remains intentionally retained, so deleting the wrapper is only safe if the same slice preserves equivalent operator-path evidence through the native command.

## Closeout Expectations

- Keep the native command-surface commit and the wrapper-retirement/planning closeout commits separate.
- Archive the phase only after the wrapper is deleted and the continuity baseline reflects the new 98-script live estate.