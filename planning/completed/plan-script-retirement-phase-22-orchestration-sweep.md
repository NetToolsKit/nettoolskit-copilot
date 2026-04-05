# Phase 22: Orchestration Consumer Sweep

Generated: 2026-04-05

## Status

- LastUpdated: 2026-04-05 17:45
- Objective: execute the final domain-level consumer sweep for `scripts/orchestration/**/*.ps1`, prove whether any orchestration wrapper is safe to retire, and close the scripted-consumer-migration backlog with explicit evidence.
- Normalized Request: continue the script-retirement planning flow after the closed Phase 21 sweep, keep planning updated, and commit each stable phase separately.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-22-orchestration-sweep.md`
- Inputs:
  - `planning/active/plan-repository-consolidation-continuity.md`
  - `planning/specs/active/spec-repository-consolidation-continuity.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/orchestration/**/*.ps1`
  - `definitions/providers/codex/orchestration/**/*`
  - `definitions/providers/github/policies/agent-orchestration.policy.json`
  - `definitions/providers/github/governance/agent-skill-permissions.matrix.json`
  - `crates/orchestrator/tests/execution/pipeline_parity/**/*`

## Scope Summary

1. `scripts/orchestration/engine/invoke-codex-dispatch.ps1`
2. `scripts/orchestration/engine/invoke-task-worker.ps1`
3. `scripts/orchestration/stages/closeout-stage.ps1`
4. `scripts/orchestration/stages/implement-stage.ps1`
5. `scripts/orchestration/stages/intake-stage.ps1`
6. `scripts/orchestration/stages/plan-stage.ps1`
7. `scripts/orchestration/stages/review-stage.ps1`
8. `scripts/orchestration/stages/route-stage.ps1`
9. `scripts/orchestration/stages/spec-stage.ps1`
10. `scripts/orchestration/stages/validate-stage.ps1`

This phase is complete only if:

- every orchestration leaf is classified with concrete local-consumer evidence
- no delete is attempted without zero non-self consumer proof
- the continuity workstream can close the post-Phase-18 consumer-migration backlog with explicit orchestration evidence

## Ordered Tasks

### Task 1: Freeze The Orchestration Inventory And Search Surface

Status: `[x]` Completed

- Lock the 10-script orchestration working set above.
- Reuse deterministic search commands for every candidate:
  - `rg -n "<script-name>" definitions crates planning scripts docs`
  - `rg -n "default.pipeline.json|agent-orchestration.policy.json|agent-skill-permissions.matrix.json" definitions crates planning scripts docs`

### Task 2: Execute The Orchestration Consumer Sweep

Status: `[x]` Completed (audit-only; zero deletions)

- Expected blocker classes:
  - Codex orchestration pipeline definitions and README surfaces
  - agent-orchestration policy baselines and permission matrices
  - orchestrator parity tests and validation fixtures
  - stage-to-stage orchestration chaining
- Deliverables:
  - exact zero-consumer list for deletable orchestration leaves
  - retained-blocker list for non-deletable orchestration leaves
  - same-slice doc/policy/test re-points for any deleted leaf
- Result:
  - zero-consumer list: none
  - deleted leaves: none
  - retained-blocker graph:
    - `definitions/providers/codex/orchestration/pipelines/default.pipeline.json` still pins every stage wrapper except `invoke-task-worker.ps1`
    - `.codex/orchestration/README.md` and `definitions/providers/codex/orchestration/README.md` still advertise the engine wrappers
    - `definitions/providers/github/policies/agent-orchestration.policy.json`, `definitions/providers/github/policies/instruction-system.policy.json`, `definitions/providers/github/governance/agent-skill-permissions.matrix.json`, and `definitions/providers/github/governance/release-provenance.baseline.json` still encode orchestration wrapper paths
    - `crates/orchestrator/tests/execution/pipeline_parity/*`, `crates/commands/validation/src/agent_orchestration/orchestration_integrity.rs`, `crates/commands/validation/tests/agent_orchestration/*`, and `crates/commands/validation/tests/support/agent_orchestration_fixtures.rs` still hardcode the engine and stage wrapper names
    - `scripts/tests/runtime/agent-orchestration-engine.tests.ps1` still hardcodes every stage wrapper
    - the stage graph itself still chains through `invoke-codex-dispatch.ps1`, `invoke-task-worker.ps1`, and `validate-stage.ps1`
- Outcome:
  - the orchestration domain closes as audit-only
  - no same-slice re-points were enough to clear the authored pipeline/policy/test fanout safely
- Commit checkpoint:
  - `docs(runtime-retirement): record Phase 22 orchestration-domain audit-only consumer proof for engine and stage wrappers`

### Task 3: Rebaseline And Close Out Phase 22

Status: `[x]` Completed

- After the sweep:
  - update `planning/completed/script-retirement-safety-matrix.md`
  - update `planning/completed/rust-script-parity-ledger.md`
  - update `planning/active/plan-repository-consolidation-continuity.md`
  - update `crates/orchestrator` docs only if any wrapper is actually retired
- Validation evidence:
  - `cargo test -p nettoolskit-orchestrator --quiet` ⚠️ existing baseline failure in `execution::chatops::tests::execute_chatops_envelope_submit_records_control_plane_metadata` caused by secure tool-scope allowlist policy for `ai.plan`, not by this orchestration sweep
  - `cargo test -p nettoolskit-cli --test test_suite --quiet` ✅
  - `cargo run -q -p nettoolskit-cli -- validation agent-orchestration --repo-root .` ✅
  - `cargo run -q -p nettoolskit-cli -- validation instructions --repo-root . --warning-only false` ✅
  - `cargo run -q -p nettoolskit-cli -- validation planning-structure --repo-root . --warning-only false` ✅
  - `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High` ✅
  - `git diff --check` ✅
- Closeout result:
  - all ten orchestration leaves remained blocked
  - no orchestration docs needed same-slice re-pointing because no wrapper was deletion-ready
- Closeout checkpoint:
  - move this plan/spec to `planning/completed/` and `planning/specs/completed/` ✅

## Executed Result

- Phase 22 closed as an audit-only phase with zero deletions.
- All ten orchestration leaves remain blocked by authored pipeline definitions, policy baselines, validation fixtures, parity tests, or stage-to-stage chaining.

## Validation Checklist

- [ ] targeted `rg` consumer sweep across `definitions/`, `crates/`, `planning/`, `scripts/`, and `docs/`
- [ ] `cargo test -p nettoolskit-orchestrator --quiet`
- [ ] `cargo test -p nettoolskit-cli --test test_suite --quiet`
- [ ] `cargo run -q -p nettoolskit-cli -- validation agent-orchestration --repo-root .`
- [ ] `cargo run -q -p nettoolskit-cli -- validation instructions --repo-root . --warning-only false`
- [ ] `cargo run -q -p nettoolskit-cli -- validation planning-structure --repo-root . --warning-only false`
- [ ] `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- [ ] `git diff --check`

## Risks And Fallbacks

- The orchestration wrappers are heavily cross-linked, so even one surviving stage usually pins several others.
- `validate-stage.ps1` is especially high-fanout because it appears in historical cutover evidence, validation fixtures, and runtime parity tests.
- If every orchestration leaf still has a live consumer, the correct result is an audit-only closeout instead of forced deletion.

## Closeout Expectations

- Phase 22 may close as audit-only if every wrapper is still pinned by live authored/runtime consumers.
- The resulting blocker graph should be explicit enough to support the final post-Phase-22 retention audit without reopening the same search work.