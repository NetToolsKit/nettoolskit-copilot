# Repository Unification And Rust Script Transcription Plan

Generated: 2026-03-26 16:20

## Status

- LastUpdated: 2026-03-26 16:20
- Objective: convert the unified repository migration plan into a full `scripts/**/*.ps1` to Rust transcription roadmap while preserving current operator compatibility.
- Normalized Request: resume planning on a dedicated branch, keep work isolated, use `.temp/arquitetura_enterprise_llm.md` as the architectural source input, and make the migration scope cover all existing PowerShell scripts.
- Active Branch: `feature/rust-script-transcription-planning`
- Spec Path: `planning/specs/active/spec-repository-unification-and-rust-migration.md`
- Supporting Architecture Spec: `planning/specs/active/spec-enterprise-rust-runtime-transcription-architecture.md`
- Worktree Isolation: not recommended for this planning-only checkpoint; a dedicated branch is active in the current checkout.

## Scope Summary

This active plan now treats the full PowerShell estate as the migration target. The repository has already been unified structurally; the next planning baseline must cover the complete `147`-script portfolio rather than a small subset of runtime families.

The migration remains compatibility-first:

- `nettoolskit-copilot` stays the implementation workspace
- `C:\Users\tguis\copilot-instructions` stays available as the legacy reference
- PowerShell entrypoints remain available until the Rust path reaches validated parity
- `definitions/`, `.github/`, `.codex/`, `.claude/`, `.vscode/`, and `planning/` remain the static authority surfaces

## Script Inventory Snapshot

| Domain | Script Count | Planned Migration Wave |
| --- | ---: | --- |
| `scripts/common` | 15 | Wave 1 foundation |
| `scripts/runtime` | 46 | Wave 1 foundation |
| `scripts/validation` | 31 | Wave 2 quality and policy |
| `scripts/security` | 6 | Wave 2 quality and policy |
| `scripts/governance` | 2 | Wave 2 quality and policy |
| `scripts/maintenance` | 5 | Wave 2 quality and policy |
| `scripts/doc` | 1 | Wave 2 quality and policy |
| `scripts/deploy` | 1 | Wave 2 quality and policy |
| `scripts/orchestration` | 10 | Wave 3 control plane |
| `scripts/git-hooks` | 3 | Wave 3 control plane |
| `scripts/tests` | 27 | Wave 3 control plane and parity |
| Total | 147 | Full migration scope |

## Ordered Tasks

### Task 1: Rebaseline The Migration Scope Around Full Script Transcription

Status: `[x]` Completed

- [2026-03-26 16:20] Rewrote the active spec and this plan around the full `147`-script migration scope ✓ [2026-03-26 16:20]
- [2026-03-26 16:20] Captured `.temp/arquitetura_enterprise_llm.md` into a versioned architecture spec under `planning/specs/active/` and folded the relevant direction into the migration artifacts ✓ [2026-03-26 16:20]
- Target paths:
  - `planning/specs/active/spec-repository-unification-and-rust-migration.md`
  - `planning/specs/active/spec-enterprise-rust-runtime-transcription-architecture.md`
  - `planning/active/plan-repository-unification-and-rust-migration.md`
  - `.temp/arquitetura_enterprise_llm.md` (source input)
- Commands:
  - `git status --short --branch`
  - `rg --files scripts -g *.ps1`
  - `Get-Date -Format 'yyyy-MM-dd HH:mm'`
- Checkpoints:
  - the active spec locks the new full-scope migration target
  - the active plan references the complete script inventory and migration waves
  - the supporting `.temp` note is treated as source input and the versioned architecture spec carries the durable architectural output
- Commit checkpoint:
  - `docs(planning): rebaseline rust migration around full script transcription`

### Task 2: Freeze The Canonical Inventory And Rust Ownership Map

Status: `[ ]` Pending

- [2026-03-26 16:20] Build a canonical script-to-domain, script-to-owner, and script-to-wave matrix for all tracked `.ps1` files
- Target paths:
  - `planning/active/plan-repository-unification-and-rust-migration.md`
  - `planning/active/plan-repository-operations-hygiene.md`
  - `scripts/`
  - `Cargo.toml`
- Commands:
  - `rg --files scripts -g *.ps1`
  - `rg -n "common-bootstrap|render-|sync-|validate-|invoke-|hook" scripts`
  - `cargo metadata --format-version 1 --no-deps`
- Checkpoints:
  - every tracked PowerShell script belongs to a named capability domain
  - every script has a Rust owner boundary aligned to the workspace
  - no script remains classified as "misc" or "later"
- Commit checkpoint:
  - `docs(planning): freeze rust ownership map for all scripts`

### Task 3: Define Rust Workspace Boundaries And Command Contracts

Status: `[ ]` Pending

- [2026-03-26 16:20] Lock the target Rust command surfaces for runtime, render, sync, validate, security, orchestration, maintenance, hooks, governance, deploy, and test parity flows
- Target paths:
  - `Cargo.toml`
  - `crates/core/`
  - `crates/commands/`
  - `crates/orchestrator/`
  - `crates/cli/`
  - `scripts/common/`
  - `scripts/runtime/`
- Commands:
  - `cargo metadata --format-version 1 --no-deps`
  - `cargo fmt --all -- --check`
  - `cargo test`
- Checkpoints:
  - current workspace crates are reused where possible before introducing new crates
  - compatibility wrappers map to stable Rust commands and argument contracts
  - shared helper behavior has a clear home in Rust before family-level implementation starts
- Commit checkpoint:
  - `docs(rust): define target command contracts for full script transcription`

### Task 4: Implement Wave 1 Foundation For Shared Helpers And Runtime Surfaces

Status: `[ ]` Pending

- [2026-03-26 16:20] Transcribe shared helper logic plus runtime/render/sync/index flows behind stable wrappers
- Target paths:
  - `scripts/common/`
  - `scripts/runtime/`
  - `crates/core/`
  - `crates/commands/`
  - `crates/cli/`
- Commands:
  - `cargo build`
  - `cargo test`
  - targeted parity checks for `render-*`, `sync-*`, `bootstrap`, `doctor`, `clean-*`, and `update/query-local-context-index`
- Checkpoints:
  - shared path, catalog, template, and runtime state logic executes from Rust
  - runtime wrappers can delegate to Rust without changing operator-visible command names
  - parity evidence exists for the first foundation wave
- Commit checkpoint:
  - `feat(rust): implement shared helper and runtime transcription wave`

### Task 5: Implement Wave 2 Quality, Policy, And Support Surfaces

Status: `[ ]` Pending

- [2026-03-26 16:20] Transcribe validation, security, governance, maintenance, deploy, and documentation helper flows into Rust-native commands
- Target paths:
  - `scripts/validation/`
  - `scripts/security/`
  - `scripts/governance/`
  - `scripts/maintenance/`
  - `scripts/deploy/`
  - `scripts/doc/`
  - `crates/commands/`
  - `crates/core/`
- Commands:
  - `cargo fmt --all -- --check`
  - `cargo test`
  - targeted parity checks for `validate-*`, `Invoke-*`, `generate-http-from-openapi`, `set-branch-protection`, and documentation validation flows
- Checkpoints:
  - policy and audit workflows no longer depend on PowerShell-only business logic
  - security gates retain or improve current severity handling
  - maintenance and deploy helpers remain deterministic and operator-safe
- Commit checkpoint:
  - `feat(rust): implement quality and policy transcription wave`

### Task 6: Implement Wave 3 Control Plane, Hooks, And Parity Harness

Status: `[ ]` Pending

- [2026-03-26 16:20] Transcribe orchestration stages, git hooks, and PowerShell-based runtime tests into Rust-backed control-plane capabilities and parity coverage
- Target paths:
  - `scripts/orchestration/`
  - `scripts/runtime/hooks/`
  - `scripts/git-hooks/`
  - `scripts/tests/`
  - `crates/orchestrator/`
  - `crates/cli/`
  - `tests/`
- Commands:
  - `cargo test`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - targeted parity checks for orchestration stages, hook dispatch, and runtime policy tests
- Checkpoints:
  - staged Super Agent execution survives without PowerShell engine dependence
  - git hook setup and entrypoints have Rust-backed ownership
  - parity coverage replaces the current PowerShell-heavy test harness
- Commit checkpoint:
  - `feat(rust): implement control-plane and parity transcription wave`

### Task 7: Cut Over Defaults And Retire Legacy PowerShell Execution Safely

Status: `[ ]` Pending

- [2026-03-26 16:20] Switch default operator flows to Rust only after all migration waves reach parity and documentation is updated
- Target paths:
  - `scripts/`
  - `README.md`
  - `CHANGELOG.md`
  - `.github/workflows/`
  - `docs/`
- Commands:
  - `cargo test`
  - `git diff --check`
  - final parity command set for every migrated domain
- Checkpoints:
  - every previously tracked PowerShell script has a Rust-backed replacement or approved wrapper end state
  - operator-visible docs and release notes describe the default cutover and fallback story
  - legacy PowerShell execution is retired only after explicit approval
- Commit checkpoint:
  - `refactor(runtime): cut over full script estate to rust-backed execution`

## Validation Checklist

- `git status --short --branch`
- `rg --files scripts -g *.ps1`
- `cargo metadata --format-version 1 --no-deps`
- `cargo fmt --all -- --check`
- `cargo build`
- `cargo test`
- `cargo clippy --workspace --all-targets -- -D warnings`
- targeted parity validation for each migration wave
- `git diff --check`

## Recommended Specialist

- Primary: `dev-rust-engineer`
- Mandatory follow-up: `test-engineer`
- Supporting specialists:
  - `docs-release-engineer`
  - `ops-devops-platform-engineer`

## Closeout Expectations

- Update `README.md` when operator-visible commands or migration guidance change.
- Update `CHANGELOG.md` when a migration wave changes default behavior or release-relevant workflows.
- Keep commit messages in English and surface stable checkpoint commits between waves.
- Preserve PowerShell wrappers until parity is proven and explicit cutover approval is given.

## Delivery Slices

- Slice A: planning and architecture rebaseline for the full `147`-script scope
- Slice B: Wave 1 foundation for `scripts/common` plus `scripts/runtime` (`61` scripts)
- Slice C: Wave 2 quality and policy domains for `scripts/validation`, `scripts/security`, `scripts/governance`, `scripts/maintenance`, `scripts/doc`, and `scripts/deploy` (`46` scripts)
- Slice D: Wave 3 control plane for `scripts/orchestration`, `scripts/git-hooks`, and `scripts/tests` (`40` scripts)