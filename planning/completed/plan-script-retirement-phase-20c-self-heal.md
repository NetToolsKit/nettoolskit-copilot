# Phase 20c: Runtime Self-Heal Wrapper Retirement

Generated: 2026-03-29 09:24

## Status

- LastUpdated: 2026-03-29 09:24
- Objective: retire the local `scripts/runtime/self-heal.ps1` wrapper after the native `ntk runtime self-heal` boundary, docs, policy inventory, and retained parity fixtures all point to the managed Rust contract.
- Normalized Request: continue the aggressive PowerShell-to-Rust retirement flow, keep planning updated, and keep committing each stable phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-20c-self-heal.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-19.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/runtime/self-heal.ps1`
  - `crates/commands/runtime/src/diagnostics/self_heal.rs`
  - `crates/cli/src/runtime_commands.rs`
  - `crates/cli/tests/runtime_commands_tests.rs`

## Scope Summary

1. native `ntk runtime self-heal`
2. `scripts/runtime/self-heal.ps1`
3. `scripts/tests/runtime/runtime-scripts.tests.ps1`
4. `scripts/README.md`
5. `.github/policies/instruction-system.policy.json`
6. `crates/orchestrator/tests/execution/pipeline_parity/support/validation_baseline.rs`

This phase is complete only if:

- the native `self-heal` CLI surface is covered by deterministic tests
- the runtime parity harness stops treating the deleted `.ps1` path as canonical
- docs and policy inventory point to the native Rust contract instead of the wrapper path
- the local wrapper is deleted in the same slice

## Ordered Tasks

### Task 1: Freeze The Native Self-Heal Command Boundary

Status: `[x]` Completed

- Exposed `ntk runtime self-heal` as the managed operator-facing runtime remediation entrypoint.
- Added CLI coverage proving the native command writes the expected report and log outputs.

### Task 2: Repoint Live Consumers To The Native Boundary

Status: `[x]` Completed

- Repointed the runtime parity harness from script-parameter inspection to native CLI help coverage.
- Replaced direct wrapper evidence in:
  - `scripts/README.md`
  - `.github/policies/instruction-system.policy.json`
  - `crates/orchestrator/tests/execution/pipeline_parity/support/validation_baseline.rs`

### Task 3: Delete The Local Wrapper

Status: `[x]` Completed

- Deleted `scripts/runtime/self-heal.ps1`.

### Task 4: Rebaseline Phase Evidence And Archive

Status: `[x]` Completed

- Updated `planning/completed/script-retirement-safety-matrix.md`.
- Updated `planning/completed/rust-script-parity-ledger.md`.
- Updated the active continuity plan/spec to inherit the post-self-heal 99-script baseline.

## Validation Checklist

- [x] `cargo test -p nettoolskit-cli --test runtime_commands_tests --quiet`
- [x] `cargo test -p nettoolskit-orchestrator --test test_suite --no-run`
- [x] `pwsh -NoProfile -File .\scripts\tests\runtime\runtime-scripts.tests.ps1`
- [x] `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- [x] `git diff --check`

## Risks And Fallbacks

- The runtime parity harness is intentionally retained, so replacing the wrapper path with native CLI evidence must preserve the operator-facing contract rather than silently dropping coverage.
- The wrapper could not be deleted until policy inventory stopped requiring the local `.ps1` path.

## Closeout Expectations

- Keep the native command-surface commit and the wrapper-retirement/planning closeout separated.
- Archive the phase only after the wrapper is gone and the evidence bundle reflects the new 99-script baseline.

## Executed Result

- `ntk runtime self-heal` is now the canonical operator-facing remediation entrypoint.
- The local wrapper `scripts/runtime/self-heal.ps1` was removed after same-slice consumer repoints across docs, policy inventory, and retained runtime parity coverage.
- The live local `scripts/**/*.ps1` estate dropped from `100` to `99`.