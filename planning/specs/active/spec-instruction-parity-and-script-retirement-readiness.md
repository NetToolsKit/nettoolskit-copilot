# Instruction Parity And Script Retirement Readiness

Generated: 2026-03-28 15:42

## Objective

Define the safe path to:

1. verify that `nettoolskit-copilot` has not lost repository-owned instruction surfaces compared with `C:\Users\tguis\copilot-instructions`
2. determine whether any local `scripts/**/*.ps1` files can be retired without breaking runtime, validation, orchestration, hook, or documentation flows

## Context

The repository has already closed the Rust migration planning bundle and archived the historical wave plans under `planning/completed/`. The next operator concern is not whether Rust ownership exists on paper, but whether:

- the local repository still preserves the required instruction system
- the local `scripts/` tree is truly removable, partially removable, or still required as a compatibility/runtime surface

The audit target is split across two independent but related questions:

1. instruction parity against `C:\Users\tguis\copilot-instructions`
2. script retirement readiness inside `C:\Users\tguis\Documents\Trabalho\Pessoal\Desenvolvimento\Projetos\nettoolskit-copilot`

## Current Audit Summary

### Instruction Surface Parity

- Structural parity is currently intact across the requested authority surfaces.
- The local repository and `C:\Users\tguis\copilot-instructions` expose matching file counts and matching relative file paths for:
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
- Content parity is not fully aligned: `14` of `201` compared files currently differ by hash.
- The highest-risk drift is concentrated in governance baselines and operator-facing README/index surfaces rather than in the core instruction files.

### Script Retirement Readiness

- The repository still contains the full `147`-script PowerShell estate under `scripts/`.
- The completed migration bundle already distinguishes:
  - `Rust-default now` domains
  - `compatibility wrapper retained intentionally` domains
  - `legacy integration wrapper retained intentionally` domains
- That planning evidence is not enough to delete `scripts/` yet.
- The repository still has many live consumers of local `.ps1` paths across:
  - runtime code
  - validation code
  - orchestrator code
  - tests
  - docs
  - policy/governance manifests
  - PowerShell wrappers themselves
- The first consumer-backed safety matrix now narrows immediate retirement to `4` leaves:
  - `scripts/doc/validate-xml-documentation.ps1`
  - `scripts/maintenance/fix-version-ranges.ps1`
  - `scripts/maintenance/fix-region-spacing.ps1`
  - `scripts/maintenance/clean-build-artifacts.ps1` after same-slice instruction cleanup
- The rest of the estate is now split explicitly between:
  - `33` intentionally retained wrappers by policy
  - `110` leaves or tight domains that still require consumer migration evidence before removal

## Key Findings To Preserve

1. No requested instruction directories are missing locally; the parity problem is content drift, not absent assets.
2. `.github/AGENTS.md`, `.github/copilot-instructions.md`, and `.github/instructions/**` do not currently show the same high-risk drift pattern as governance baselines and README/index files.
3. The following content drifts need explicit reconciliation before the external repo can stop being treated as a reference:
   - `.github/governance/readme-standards.baseline.json`
   - `.github/governance/validation-profiles.json`
   - `.codex/mcp/README.md`
   - `.codex/orchestration/README.md`
   - `.codex/scripts/README.md`
   - `.codex/skills/README.md`
   - `.vscode/README.md`
   - `.vscode/profiles/README.md`
   - `planning/README.md`
   - `planning/specs/README.md`
   - the remaining prompt-template POML files reported in the hash audit
4. Deleting the whole local `scripts/` tree is unsafe in the current state.
5. The retained-wrapper domains are explicit non-deletion targets until the operating model itself changes:
   - `scripts/runtime/hooks`
   - `scripts/maintenance` for `generate-http-from-openapi.ps1`
   - `scripts/deploy`
   - `scripts/git-hooks`
   - `scripts/tests/runtime`
   - `scripts/tests/apply-aaa-pattern.ps1`
   - `scripts/tests/run-coverage.ps1`
6. Even the domains documented as `Rust-default now` still require consumer migration before local wrapper deletion is safe.
7. `Rust-default now` and `remove-now candidate` are not equivalent states; deletion requires zero blocking local consumers, not just parity evidence.

## Design Decisions

1. Treat `C:\Users\tguis\copilot-instructions` as the temporary upstream comparison source until every high-risk content drift is either synchronized or explicitly accepted as local specialization.
2. Do not plan deletion at the `scripts/` root level. Plan retirement per script leaf or tightly bounded domain.
3. Split script retirement into three classes:
   - `remove candidate`
   - `retain wrapper intentionally`
   - `retain until consumer migration completes`
4. Require both kinds of evidence before deletion:
   - Rust coverage/parity evidence from the completed migration bundle
   - zero remaining local runtime/policy/doc/test consumers of the deleted `.ps1` path
5. Keep the completed migration bundle authoritative for Rust ownership and parity; the new workstream should only decide retirement readiness and instruction reconciliation.

## Non-Goals

- deleting scripts immediately in this planning slice
- rewriting the external repository
- re-opening the completed migration wave plans as active implementation records
- treating every content drift against `C:\Users\tguis\copilot-instructions` as automatically incorrect without classifying intentional repository-specific overrides

## Risks

- governance manifests and validation profiles may still encode direct PowerShell-path contracts even where Rust ownership exists
- runtime and orchestrator code may continue to require wrapper scripts for operator compatibility or external process dispatch
- deleting wrappers too early can silently break hook/bootstrap/validation flows that still use `.ps1` entrypoints as part of the runtime contract
- some README and planning drift may be intentional local specialization, so blind synchronization from the external repository could regress this repository's completed-state documentation

## Alternatives Considered

### Alternative 1: Delete `scripts/` based only on the completed cutover map

Rejected. The cutover map captures ownership and policy state, but it does not prove that local code, tests, docs, and governance manifests stopped consuming script paths.

### Alternative 2: Re-sync every differing instruction file from `C:\Users\tguis\copilot-instructions`

Rejected. Several drifts may be repository-specific and correct for this workspace, especially archived planning indexes and README baselines that already evolved with the Rust migration.

### Alternative 3: Keep all scripts indefinitely and stop auditing retirement readiness

Rejected. That would leave the repository carrying unnecessary compatibility debt and would prevent a clean post-migration operating model.

## Acceptance Criteria

1. A versioned plan exists for instruction parity reconciliation and script retirement readiness.
2. The workstream records the exact high-risk content drifts versus `C:\Users\tguis\copilot-instructions`.
3. The workstream records a deletion-safety classification for the local PowerShell estate.
4. Every retained-wrapper script is explicitly justified instead of being left as generic migration debt.
5. Every deletion candidate is blocked from removal until local consumers are migrated or removed.
6. The resulting plan can drive safe execution without reopening the completed migration design bundle.
7. The workstream can distinguish immediate retirement candidates from parity-proven-but-still-blocked domains without deleting `scripts/` in bulk.

## Planning Readiness

- `ready-for-plan`
- Updated: `2026-03-28 15:42` — structural instruction parity is intact, content drift is bounded, and the retirement problem is now clearly reducible to consumer migration plus retained-wrapper policy.

## Recommended Specialist Focus

- `plan-active-work-planner`
- `docs-release-engineer`
- `dev-rust-engineer`
- `test-engineer`