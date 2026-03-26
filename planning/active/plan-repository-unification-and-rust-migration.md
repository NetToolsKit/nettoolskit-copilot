# Repository Unification And Rust Migration Plan

## Scope Summary

This active plan establishes `nettoolskit-copilot` as the unified successor workspace for the current Rust-first `nettoolskit-cli` repository while using `C:\Users\tguis\copilot-instructions` as the external legacy automation/runtime reference during the migration.

Completed in the current workstream:

- the new local repository was created at `C:\Users\tguis\Documents\Trabalho\Pessoal\Desenvolvimento\Projetos\nettoolskit-copilot`
- `nettoolskit-cli` remains the root lineage and default remote baseline
- `copilot-instructions` remains available through the preserved source remote and the external reference path at `C:\Users\tguis\copilot-instructions`
- the organized AI/runtime folders were brought into the new repository root without embedding `legacy/copilot-instructions/` in the active worktree:
  - `definitions/`
  - `scripts/`
  - `.codex/`
  - `.claude/`
  - `.vscode/`
  - AI/runtime-specific `.github/` surfaces such as `agents/`, `chatmodes/`, `instructions/`, `prompts/`, `schemas/`, and related governance/hooks assets

Planned follow-up in this workstream:

- classify the imported PowerShell/runtime assets now hydrated into the new repository root into explicit migration buckets
- define the Rust engine boundaries that will replace the legacy runtime scripts in phases
- migrate script families behind stable wrappers and parity validation instead of a one-shot rewrite
- keep the external `copilot-instructions` repository available as the migration reference until the Rust cutover is proven and explicitly approved

## Ordered Tasks

### Task 1: Create The Unified Repository Baseline

Status: `[x]` Completed

- Target paths:
  - `.git/`
  - `definitions/`
  - `scripts/`
  - `.codex/`
  - `.claude/`
  - `.vscode/`
  - `.github/`
  - `C:\Users\tguis\copilot-instructions`
  - `planning/active/plan-repository-unification-and-rust-migration.md`
- Commands:
  - `git status --short --branch`
  - `git remote -v`
  - `git log --oneline --graph --decorate -10`
- Checkpoints:
  - `nettoolskit-copilot` exists as a standalone git repository
  - the root repository history still follows `nettoolskit-cli`
  - the legacy `copilot-instructions` source remains reachable through the dedicated source remote and external working repository
  - the organized AI/runtime root folders now exist directly in `nettoolskit-copilot` without relying on an embedded `legacy/` subtree
- Commit checkpoint:
  - `chore(repo): create unified nettoolskit-copilot baseline with preserved history`

### Task 2: Classify Legacy Runtime Assets For Migration

Status: `[x]` Completed

- Target paths:
  - `scripts/`
  - `definitions/`
  - `.github/`
  - `.codex/`
  - `.claude/`
  - `.vscode/`
  - `C:\Users\tguis\copilot-instructions`
- Commands:
  - `rg --files scripts`
  - `rg -n "render-|sync-|install|bootstrap|validate|index" scripts`
  - `git diff --stat`
- Checkpoints:
  - each legacy script family is assigned to a migration bucket such as render, sync, runtime bootstrap, validation, MCP, or local context index
  - provider/runtime surfaces that must stay projected are explicitly identified
  - scripts that should remain PowerShell wrappers during early Rust phases are called out
- Commit checkpoint:
  - `docs(planning): classify legacy runtime assets for phased rust migration`

Current classification snapshot:

- legacy script inventory:
  - `runtime/`: `46` files
  - `validation/`: `32` files
  - `tests/`: `27` files
  - `common/`: `15` files
  - `orchestration/`: `10` files
- high-confidence Rust migration buckets:
  - `render`: provider surface renderers and MCP artifact renderers
  - `sync`: runtime sync entrypoints for VS Code, Claude, Codex, and workspace settings
  - `index`: local context index update/query and planning-summary continuity helpers
  - `agent-runtime`: pipeline/resume/replay helpers and structured orchestration entrypoints
- keep as compatibility wrappers in early phases:
  - `install.ps1`
  - git hook entrypoints
  - hygiene/doctor/self-heal flows
  - broad validation aggregators such as `validate-all.ps1`
- first Rust slice candidate:
  - `render` plus `sync` helpers, because they are deterministic file-graph transforms with lower operational risk than install/bootstrap cutover

### Task 3: Define The Rust Engine Foundation Boundaries

Status: `[ ]` Pending

- Target paths:
  - `crates/`
  - `tests/`
  - `Cargo.toml`
  - `definitions/`
  - `scripts/runtime/`
  - `C:\Users\tguis\copilot-instructions`
- Commands:
  - `cargo metadata --format-version 1 --no-deps`
  - `cargo fmt --all -- --check`
  - `cargo test`
- Checkpoints:
  - the future Rust engine boundaries are explicit for workspace crates such as `cli`, `render`, `sync`, `runtime`, and `index`
  - `definitions/` remains the authority for non-executable assets and projected surfaces
  - PowerShell wrappers remain valid until Rust parity is proven
- Commit checkpoint:
  - `docs(rust): define engine boundaries for unified runtime migration`

### Task 4: Port Script Families In Compatible Slices

Status: `[ ]` Pending

- Target paths:
  - `crates/render/`
  - `crates/sync/`
  - `crates/runtime/`
  - `crates/index/`
  - `scripts/runtime/`
- Commands:
  - `cargo build`
  - `cargo test`
  - targeted parity checks against legacy PowerShell entrypoints
- Checkpoints:
  - each Rust slice lands behind a stable wrapper or equivalent operator entrypoint
  - Rust implementations can be validated against the preserved legacy behavior
  - no legacy PowerShell surface is removed before parity is demonstrated
- Commit checkpoint:
  - `feat(rust): port first unified runtime slice behind compatible wrappers`

### Task 5: Cut Over And Retire Legacy Surfaces Safely

Status: `[ ]` Pending

- Target paths:
  - `scripts/runtime/`
  - `README.md`
  - `CHANGELOG.md`
  - `C:\Users\tguis\copilot-instructions`
- Commands:
  - `cargo test`
  - `git diff --check`
  - final parity validation commands for the affected slice
- Checkpoints:
  - wrapper defaults prefer the Rust implementation only after parity is proven
  - the external legacy repository remains available until the user explicitly approves retirement work
  - documentation reflects the new operator path and fallback story
- Commit checkpoint:
  - `refactor(runtime): cut over unified runtime slice to rust implementation`

## Validation Checklist

- `git status --short --branch`
- `git remote -v`
- `git log --oneline --graph --decorate -10`
- `cargo metadata --format-version 1 --no-deps`
- `cargo fmt --all -- --check`
- `cargo build`
- `cargo test`
- targeted parity validation between legacy PowerShell entrypoints and the new Rust-backed replacements

## Recommended Specialist

- Primary: `dev-rust-engineer`
- Planning/design support:
  - `brainstorm-spec-architect`
  - `plan-active-work-planner`
- Follow-up support:
  - `docs-release-engineer`
  - `test-engineer`

## Closeout Expectations

- Preserve the current `nettoolskit-cli` repository history throughout the migration and keep the external `copilot-instructions` repository available as provenance/reference.
- Do not modify or delete `C:\Users\tguis\copilot-instructions` early.
- Keep commit messages in English and provide them in fenced code blocks when requested.
- Update `README.md` and `CHANGELOG.md` only when the migrated slice changes operator-visible behavior.
- Keep PowerShell wrappers or equivalent compatibility shims until the Rust path is validated and approved for cutover.

## Delivery Slices

- Slice A: unified repository baseline and planning registration
- Slice B: legacy asset classification and Rust boundary definition
- Slice C: first compatible Rust-backed runtime slices
- Slice D: gradual cutover and legacy retirement