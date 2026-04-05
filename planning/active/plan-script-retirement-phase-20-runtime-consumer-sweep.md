# Phase 20: Runtime Consumer Sweep

Generated: 2026-04-05

## Status

- LastUpdated: 2026-04-05 12:18
- Objective: prove the remaining local consumer graph for the 30 retained `scripts/runtime/*.ps1` leaves, then retire only the zero-consumer subsets without reopening the already-closed tactical runtime slices.
- Normalized Request: continue the script-retirement program with a dedicated Phase 20 plan for the remaining runtime-domain scripts, keep planning updated, and commit each stable phase separately.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-script-retirement-phase-20-runtime-consumer-sweep.md`
- Inputs:
  - `planning/active/plan-repository-consolidation-continuity.md`
  - `planning/specs/active/spec-repository-consolidation-continuity.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/runtime/`
  - `scripts/tests/runtime/*.ps1`
  - `definitions/providers/github/governance/provider-surface-projection.catalog.json`
  - `definitions/providers/github/policies/*.json`
  - `crates/commands/runtime/`
  - `crates/commands/validation/`
  - `crates/orchestrator/`

## Scope Summary

1. Freeze the current 30-script runtime-domain inventory.
2. Split the domain into execution slices that do not collide with the already-completed tactical `Phase 20c self-heal` slice.
3. Collect exact local-consumer proof before any deletion.
4. Retire only the confirmed zero-consumer leaves in each slice.
5. Rebaseline the safety matrix, parity ledger, and umbrella continuity plan after every retirement commit.

Current runtime-domain inventory under this phase:

- Slice A — projection, profile, sync, and workspace runtime surfaces:
  - `render-claude-runtime-surfaces.ps1`
  - `render-github-instruction-surfaces.ps1`
  - `render-mcp-runtime-artifacts.ps1`
  - `render-provider-skill-surfaces.ps1`
  - `render-vscode-profile-surfaces.ps1`
  - `render-vscode-workspace-surfaces.ps1`
  - `set-codex-runtime-preferences.ps1`
  - `setup-vscode-profiles.ps1`
  - `sync-claude-settings.ps1`
  - `sync-claude-skills.ps1`
  - `sync-vscode-global-mcp.ps1`
  - `sync-vscode-global-settings.ps1`
  - `sync-vscode-global-snippets.ps1`
  - `sync-workspace-settings.ps1`
  - `update-copilot-chat-titles.ps1`
  - `validate-vscode-global-alignment.ps1`

- Slice B — orchestration runtime entrypoints and replay helpers:
  - `evaluate-agent-pipeline.ps1`
  - `invoke-super-agent-brainstorm.ps1`
  - `invoke-super-agent-execute.ps1`
  - `invoke-super-agent-housekeeping.ps1`
  - `invoke-super-agent-parallel-dispatch.ps1`
  - `invoke-super-agent-plan.ps1`
  - `new-super-agent-worktree.ps1`
  - `replay-agent-run.ps1`
  - `resume-agent-pipeline.ps1`
  - `run-agent-pipeline.ps1`

- Slice C — bootstrap, install, and cleanup surfaces:
  - `bootstrap.ps1`
  - `clean-codex-runtime.ps1`
  - `clean-vscode-user-runtime.ps1`
  - `install.ps1`

This phase is complete only if:

- each slice has explicit zero-consumer proof before any deletion is attempted
- no slice reuses the tactical `20c/20d/20e/20f` naming already archived
- partial retirement is allowed, documented, and rebaselined instead of forcing an all-or-nothing deletion batch
- the safety matrix and parity ledger record every retained blocker that survives the sweep

## Ordered Tasks

### Task 1: Freeze Runtime Slice Boundaries And Search Patterns

Status: `[ ]` Pending

- Lock the 30-script inventory above into a deterministic Phase 20 working set.
- Build the per-slice consumer-search checklist:
  - authored definitions
  - provider projections and governance baselines
  - runtime parity tests
  - CLI/validation/orchestrator fixtures
  - README and operator guidance
- Confirm the search commands that will be reused for each slice:
  - `rg -n "scripts/runtime/<script-name>" .`
  - `rg -n "ntk runtime|ntk validation" definitions docs planning scripts crates`
- Checkpoint:
  - inventory and slice boundaries locked

### Task 2: Slice A Consumer Sweep — Projection, Profile, Sync, And Workspace Surfaces

Status: `[ ]` Pending

- Target paths:
  - `scripts/runtime/render-*.ps1`
  - `scripts/runtime/sync-*.ps1`
  - `scripts/runtime/setup-vscode-profiles.ps1`
  - `scripts/runtime/set-codex-runtime-preferences.ps1`
  - `scripts/runtime/update-copilot-chat-titles.ps1`
  - `scripts/runtime/validate-vscode-global-alignment.ps1`
- Expected blocker classes:
  - `definitions/providers/*/README.md`
  - `provider-surface-projection.catalog.json`
  - `.vscode` sync runtime tests
  - generated-surface instructions that still advertise script paths
- Deliverables:
  - exact zero-consumer list for deletable Slice A leaves
  - retained-blocker list for non-deletable Slice A leaves
  - same-slice doc/test re-points for every deleted leaf
- Commit checkpoint:
  - `chore(runtime-retirement): execute Phase 20 Slice A consumer sweep for projection, profile, and sync surfaces`

### Task 3: Slice B Consumer Sweep — Orchestration Runtime Entry Points

Status: `[ ]` Pending

- Target paths:
  - `scripts/runtime/evaluate-agent-pipeline.ps1`
  - `scripts/runtime/invoke-super-agent-*.ps1`
  - `scripts/runtime/new-super-agent-worktree.ps1`
  - `scripts/runtime/replay-agent-run.ps1`
  - `scripts/runtime/resume-agent-pipeline.ps1`
  - `scripts/runtime/run-agent-pipeline.ps1`
- Expected blocker classes:
  - `crates/orchestrator/tests/execution/pipeline_parity`
  - validation fixtures and policy baselines
  - provider-authored orchestration READMEs and prompts
  - super-agent instructions and runtime operator guidance
- Deliverables:
  - exact zero-consumer list for deletable Slice B leaves
  - retained-blocker list for non-deletable Slice B leaves
  - same-slice parity/test/doc re-points for every deleted leaf
- Commit checkpoint:
  - `chore(runtime-retirement): execute Phase 20 Slice B consumer sweep for orchestration entrypoints and replay helpers`

### Task 4: Slice C Consumer Sweep — Bootstrap, Install, And Cleanup Surfaces

Status: `[ ]` Pending

- Target paths:
  - `scripts/runtime/bootstrap.ps1`
  - `scripts/runtime/install.ps1`
  - `scripts/runtime/clean-codex-runtime.ps1`
  - `scripts/runtime/clean-vscode-user-runtime.ps1`
- Expected blocker classes:
  - operating-model instructions
  - git hook/runtime path helpers
  - provider bootstrap docs
  - security/release governance baselines
  - retained operator workflows that still invoke the local compatibility path
- Deliverables:
  - exact zero-consumer list for deletable Slice C leaves
  - retained-blocker list for non-deletable Slice C leaves
  - same-slice doc/policy/runtime re-points for every deleted leaf
- Commit checkpoint:
  - `chore(runtime-retirement): execute Phase 20 Slice C consumer sweep for bootstrap, install, and cleanup surfaces`

### Task 5: Rebaseline And Close Out Phase 20

Status: `[ ]` Pending

- After every executed slice:
  - update `planning/completed/script-retirement-safety-matrix.md`
  - update `planning/completed/rust-script-parity-ledger.md`
  - update `planning/active/plan-repository-consolidation-continuity.md`
  - update `planning/specs/active/spec-repository-consolidation-continuity.md` when phase-level design intent changes
- Archive the Phase 20 plan/spec only when all three slices are either executed or explicitly closed as blocked with recorded evidence.
- Closeout checkpoint:
  - move this plan/spec to `planning/completed/` and `planning/specs/completed/`

## Validation Checklist

- [ ] `rg -n "scripts/runtime/<script-name>" .` for every candidate leaf
- [ ] `cargo run -q -p nettoolskit-cli -- validation runtime-script-tests --repo-root . --warning-only false`
- [ ] `cargo run -q -p nettoolskit-cli -- validation policy --repo-root .`
- [ ] `cargo run -q -p nettoolskit-cli -- validation agent-orchestration --repo-root .`
- [ ] `cargo run -q -p nettoolskit-cli -- validation instructions --repo-root . --warning-only false`
- [ ] `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- [ ] `git diff --check`

## Risks And Fallbacks

- `render-mcp-runtime-artifacts.ps1` may remain blocked by the provider-surface projection catalog even if other render/sync leaves become deletion-ready.
- `run-agent-pipeline.ps1`, `resume-agent-pipeline.ps1`, and `replay-agent-run.ps1` may remain blocked by validation fixtures and orchestrator parity tests even if Rust ownership is already proven.
- `bootstrap.ps1` and `install.ps1` are high-fanout compatibility entrypoints; if consumer proof remains broad, keep them retained and document the blocker graph instead of forcing deletion.
- If a slice produces only blockers and zero deletions, commit the evidence-only result and keep the remaining leaves classified as `retain until consumer migration completes`.

## Closeout Expectations

- Keep the phase-opening planning commit separate from each slice-execution commit.
- Keep each slice commit focused on one family only.
- Use detailed commit messages that state the consumer-proof result, the deleted or retained leaves, and the validations run.