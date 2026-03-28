# Plan: Instruction Parity And Script Retirement

Generated: 2026-03-28 15:42

## Status

- LastUpdated: 2026-03-28 15:42
- Objective: reconcile high-risk instruction drift against `C:\Users\tguis\copilot-instructions` and determine the exact local `scripts/**/*.ps1` retirement scope that is safe after the completed Rust migration.
- Normalized Request: compare the repository instruction system against `C:\Users\tguis\copilot-instructions`, verify whether the local `scripts/` tree is fully covered by Rust, and plan the safe path to retire local PowerShell scripts without losing repository instructions or runtime contracts.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/active/spec-instruction-parity-and-script-retirement-readiness.md`
- Inputs:
  - `C:\Users\tguis\copilot-instructions`
  - `planning/completed/plan-repository-unification-and-rust-migration.md`
  - `planning/completed/plan-repository-operations-hygiene.md`
  - `planning/completed/plan-rust-migration-closeout-and-cutover.md`
  - `planning/completed/rust-script-cutover-default-map.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/completed/rust-script-transcription-ownership-matrix.md`
- Current Audit Snapshot:
  - structural instruction parity is intact across the requested authority surfaces
  - content drift exists in `14` of `201` compared instruction/runtime-authority files
  - the local `147`-script estate is still not safe to delete as a whole because many `.ps1` paths remain live consumers in code, docs, tests, and governance surfaces
- Worktree Isolation: not required for the planning slice; a dedicated semantic branch is already active

## Scope Summary

This workstream does not reopen the completed Rust migration. It plans the next closeout problem on top of it:

1. prove which instruction drifts are acceptable local specialization versus accidental divergence from `C:\Users\tguis\copilot-instructions`
2. prove which local scripts are:
   - removable now
   - intentionally retained wrappers
   - blocked by live consumers that still need migration

The expected outcome is not “delete `scripts/` immediately”. The expected outcome is a deterministic deletion-readiness program.

## Ordered Tasks

### Task 1: Freeze The Instruction Parity Baseline

Status: `[ ]` Pending

- [2026-03-28 15:42] Record the current comparison baseline between this repository and `C:\Users\tguis\copilot-instructions`.
- Capture structural parity evidence for:
  - `.github/AGENTS.md`
  - `.github/copilot-instructions.md`
  - `.github/instructions/**`
  - `.github/prompts/**`
  - `.github/governance/**`
  - `.codex/**`
  - `.claude/**`
  - `.vscode/**`
  - `planning/README.md`
  - `planning/specs/README.md`
- Capture the exact `content-diff` file list and classify each one as:
  - canonical drift to reconcile
  - accepted repository-specific divergence
  - drift still requiring deeper review
- Target paths:
  - `planning/specs/active/spec-instruction-parity-and-script-retirement-readiness.md`
  - `planning/active/plan-instruction-parity-and-script-retirement.md`
  - `.github/`
  - `.codex/`
  - `.claude/`
  - `.vscode/`
- Commands:
  - hash comparison between `C:\Users\tguis\copilot-instructions` and the local repo for the requested authority surfaces
  - `Compare-Object` over relative file inventories
  - `git diff --check`
- Checkpoints:
  - no missing local instruction surfaces remain undiscovered
  - the drift list is explicit and reviewable
  - the plan distinguishes absence from content specialization
- Commit checkpoint:
  - `docs(planning): freeze instruction parity audit baseline`

### Task 2: Build The Script Retirement Safety Matrix

Status: `[ ]` Pending

- [2026-03-28 15:42] Convert the completed migration bundle into a deletion-readiness classification for the local `scripts/` tree.
- Record one explicit state for every relevant script leaf or tight domain:
  - `remove candidate`
  - `retain wrapper intentionally`
  - `retain until consumer migration completes`
- Use the completed ownership matrix, parity ledger, and cutover map as inputs, but do not trust them alone; cross-check with the real `scripts/` tree.
- Target paths:
  - `planning/active/plan-instruction-parity-and-script-retirement.md`
  - optional supporting artifact: `planning/active/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-transcription-ownership-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/completed/rust-script-cutover-default-map.md`
  - `scripts/`
- Commands:
  - `rg --files scripts -g *.ps1`
  - targeted `rg -n` consumer scans for `scripts/**/*.ps1`
  - `git diff --check`
- Checkpoints:
  - the plan no longer talks about `scripts/` as one deletable block
  - every retained exception is explicit
  - every Rust-default domain is separated from true deletion readiness
- Commit checkpoint:
  - `docs(planning): classify script retirement safety`

### Task 3: Audit Live Script Consumers And Deletion Blockers

Status: `[ ]` Pending

- [2026-03-28 15:42] Map every live local consumer of `.ps1` paths and group them by blocker class:
  - runtime code
  - validation code
  - orchestrator code
  - tests
  - docs/readmes
  - governance/policy manifests
  - workflows
- Require exact file/line evidence for any script marked “not deletable yet”.
- Highlight the strongest current blockers:
  - retained wrapper domains by policy
  - validation/orchestration contracts that still encode `.ps1` script paths
  - docs and operator guidance that still advertise PowerShell entrypoints as local commands
- Target paths:
  - `crates/`
  - `.github/`
  - `README.md`
  - `scripts/README.md`
  - `planning/active/plan-instruction-parity-and-script-retirement.md`
- Commands:
  - `rg -n "scripts[/\\\\].+\\.ps1" .`
  - targeted scans excluding `planning/completed/**`
  - `git diff --check`
- Checkpoints:
  - every deletion blocker is tied to a concrete consumer
  - the next execution phase can migrate consumers instead of guessing
- Commit checkpoint:
  - `docs(planning): freeze script deletion blockers`

### Task 4: Reconcile High-Risk Instruction Drift

Status: `[ ]` Pending

- Reconcile the high-risk drift files from Task 1.
- For each drifted file, decide one outcome:
  - sync from `C:\Users\tguis\copilot-instructions`
  - keep local version and document the intentional divergence
  - merge both versions into one canonical local artifact
- Prioritize:
  - `.github/governance/readme-standards.baseline.json`
  - `.github/governance/validation-profiles.json`
  - `.codex/*/README.md`
  - `.vscode/README.md`
  - `.vscode/profiles/README.md`
  - `planning/README.md`
  - `planning/specs/README.md`
- Target paths:
  - the drifted files identified in Task 1
  - `planning/active/plan-instruction-parity-and-script-retirement.md`
- Commands:
  - targeted file diff against `C:\Users\tguis\copilot-instructions`
  - `git diff --check`
  - relevant validator/test commands when governance files change
- Checkpoints:
  - the repository no longer depends on the external repo to recover missing instruction intent
  - intentional local specialization is documented and bounded
- Commit checkpoint:
  - `docs(runtime): reconcile instruction parity drift`

### Task 5: Execute Staged Script Retirement

Status: `[ ]` Pending

- Retire only the script leaves or domains that pass both gates:
  - Rust/default ownership and parity are already proven
  - no local consumer still depends on the `.ps1` path
- Keep the following domains out of the first deletion wave unless policy changes:
  - `scripts/runtime/hooks`
  - `scripts/deploy`
  - `scripts/git-hooks`
  - `scripts/tests/runtime`
  - `scripts/tests/apply-aaa-pattern.ps1`
  - `scripts/tests/run-coverage.ps1`
  - `scripts/maintenance/generate-http-from-openapi.ps1`
- Update docs, governance baselines, validation fixtures, and any hardcoded path contracts in the same slice as each deletion.
- Target paths:
  - `scripts/`
  - `crates/`
  - `.github/`
  - `README.md`
  - `scripts/README.md`
  - `planning/active/plan-instruction-parity-and-script-retirement.md`
- Commands:
  - domain-specific tests and smoke checks
  - `cargo test`
  - relevant validation commands
  - `git diff --check`
- Checkpoints:
  - no deleted script path remains referenced locally
  - retained-wrapper policy stays explicit for every kept script
  - `scripts/` shrinkage is reviewable and reversible by domain
- Commit checkpoint:
  - `refactor(runtime): retire rust-covered powershell wrappers`

## Validation Checklist

- structural inventory comparison against `C:\Users\tguis\copilot-instructions`
- hash comparison for the requested authority surfaces
- `rg --files scripts -g *.ps1`
- targeted `rg -n` consumer scans for `.ps1` references
- `git diff --check`
- domain-specific cargo/test/validation commands once execution begins

## Recommended Specialist

- Primary: `plan-active-work-planner`
- Discovery:
  - `docs-release-engineer`
  - `dev-rust-engineer`
  - `test-engineer`

## Risks And Fallback

- Risk: deleting wrappers based only on planning labels will break live runtime contracts.
- Risk: some content drift versus `C:\Users\tguis\copilot-instructions` is intentional local specialization and should not be blindly overwritten.
- Fallback: if any deletion wave still leaves ambiguous `.ps1` consumers, downgrade that target from `remove candidate` to `retain until consumer migration completes`.

## Closeout Expectations

- Update README/runtime/governance artifacts whenever operator-visible command paths change.
- Keep commit messages in English and slice-oriented.
- Do not delete local scripts in bulk.
- Do not treat the external repo as retireable until the high-risk drift list is reconciled or explicitly accepted.