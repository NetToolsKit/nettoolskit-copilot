# Phase 22: Orchestration Consumer Sweep

Generated: 2026-04-05

## Status

- LastUpdated: 2026-04-05 17:20
- Objective: execute the final domain-level consumer sweep for `scripts/orchestration/**/*.ps1`, prove whether any orchestration wrapper is safe to retire, and close the scripted-consumer-migration backlog with explicit evidence.
- Normalized Request: continue the script-retirement planning flow after the closed Phase 21 sweep, keep planning updated, and commit each stable phase separately.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-script-retirement-phase-22-orchestration-sweep.md`
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

Status: `[ ]` Pending

- Lock the 10-script orchestration working set above.
- Reuse deterministic search commands for every candidate:
  - `rg -n "<script-name>" definitions crates planning scripts docs`
  - `rg -n "default.pipeline.json|agent-orchestration.policy.json|agent-skill-permissions.matrix.json" definitions crates planning scripts docs`

### Task 2: Execute The Orchestration Consumer Sweep

Status: `[ ]` Pending

- Expected blocker classes:
  - Codex orchestration pipeline definitions and README surfaces
  - agent-orchestration policy baselines and permission matrices
  - orchestrator parity tests and validation fixtures
  - stage-to-stage orchestration chaining
- Deliverables:
  - exact zero-consumer list for deletable orchestration leaves
  - retained-blocker list for non-deletable orchestration leaves
  - same-slice doc/policy/test re-points for any deleted leaf
- Commit checkpoint:
  - `docs(runtime-retirement): record Phase 22 orchestration-domain consumer proof for engine and stage wrappers`

### Task 3: Rebaseline And Close Out Phase 22

Status: `[ ]` Pending

- After the sweep:
  - update `planning/completed/script-retirement-safety-matrix.md`
  - update `planning/completed/rust-script-parity-ledger.md`
  - update `planning/active/plan-repository-consolidation-continuity.md`
  - update `crates/orchestrator` docs only if any wrapper is actually retired
- Closeout checkpoint:
  - move this plan/spec to `planning/completed/` and `planning/specs/completed/`

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