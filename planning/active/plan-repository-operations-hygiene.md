# Repository Operations Hygiene Plan

Generated: 2026-03-26 16:20

## Status

- LastUpdated: 2026-03-26 22:06
- Objective: keep repository hygiene, policy enforcement, and parity guardrails ready for the full PowerShell-to-Rust script transcription program.
- Normalized Request: align the operations hygiene plan with the repository-wide decision to transcribe every tracked PowerShell script into Rust, using `.temp/arquitetura_enterprise_llm.md` only as architectural source input while preserving prior hygiene obligations that still matter to migration safety.
- Active Branch: `feature/rust-script-transcription-planning`
- Spec Path: `planning/specs/active/spec-repository-unification-and-rust-migration.md`
- Supporting Architecture Spec: `planning/specs/active/spec-enterprise-rust-runtime-transcription-architecture.md`
- Ownership Matrix: `planning/active/rust-script-transcription-ownership-matrix.md`
- Parity Ledger: `planning/active/rust-script-parity-ledger.md`
- Worktree Isolation: not recommended for this planning-only update; a dedicated branch is active in the current checkout.

## Scope Summary

This plan complements the main migration roadmap. It does not own feature delivery slices; it owns the hygiene gates that must remain stable while the Rust transcription expands across the repository.

Current hygiene priorities for the migration:

- keep the Cargo workspace and dependency baseline healthy before adding more Rust surface area
- define parity evidence and audit visibility for all `147` PowerShell scripts
- harden CI and validation expectations so Rust-backed replacements become the default quality gate
- preserve wrapper safety, hook integrity, and artifact hygiene during the transition

## Current Rust Hygiene Snapshot

- [2026-03-26 16:48] `cargo check --workspace` passed.
- [2026-03-26 16:48] `cargo test --workspace` passed.
- [2026-03-26 16:48] `cargo fmt --all -- --check` failed across many existing files, so repository-wide formatting is still a blocking hygiene item.
- [2026-03-26 17:11] The external Rust test-contract gap for `crates/commands` and `crates/task-worker` is now closed, but repository-wide formatting debt still blocks a fully green hygiene baseline.
- [2026-03-26 18:47] `crates/commands/runtime` now carries an executable Rust replacement for the local context index flow with dedicated command tests, so CI hardening has a concrete runtime migration target.
- [2026-03-26 18:59] `crates/commands/runtime` now carries an executable Rust replacement for `export-planning-summary`, extending the real migrated runtime surface beyond the local context index path.
- [2026-03-26 19:06] `crates/core` now owns the runtime install-profile and execution-context helpers, reducing shared PowerShell coupling for the remaining runtime sync/doctor/health flows.
- [2026-03-26 19:55] `crates/commands/runtime` now carries an executable Rust replacement for audit-only `doctor` drift checks, so hygiene hardening can validate runtime alignment without depending on PowerShell for diagnosis.
- [2026-03-26 20:05] `crates/commands/runtime` now carries an executable Rust replacement for `healthcheck` orchestration/report generation, so hygiene evidence can be emitted from Rust even before the validation suite itself migrates.
- [2026-03-26 20:14] `crates/commands/runtime` now carries an executable Rust replacement for `bootstrap` sync and mirror hygiene, which also removes the remaining bootstrap delegation from `healthcheck -SyncRuntime`.
- [2026-03-26 20:33] `crates/commands/runtime` now carries an executable Rust replacement for `self-heal`, so repository recovery evidence and repair orchestration no longer depend on the legacy PowerShell wrapper.
- [2026-03-26 20:47] `crates/commands/runtime` now carries an executable Rust replacement for `apply-vscode-templates`, removing the last PowerShell bridge still embedded in the `self-heal` path.
- [2026-03-26 20:53] `crates/commands/runtime` no longer keeps all migrated surfaces flat in one folder; the crate now uses responsibility-based submodules with mirrored tests, which reduces future maintenance risk as Wave 1 grows.
- [2026-03-26 21:11] `crates/commands/runtime` now carries an executable Rust replacement for `doctor -SyncOnDrift`, so runtime drift remediation no longer depends on the legacy PowerShell wrapper when repair is requested.
- [2026-03-26 21:32] `crates/commands/runtime` now carries an executable Rust replacement for bootstrap-owned provider surface rendering too, so `bootstrap` no longer depends on `render-provider-surfaces.ps1` in the runtime sync path.
- [2026-03-26 21:39] `crates/commands/runtime` now carries an executable Rust replacement for bootstrap-owned MCP config application too, so `bootstrap` no longer depends on `sync-codex-mcp-config.ps1` to rewrite Codex `config.toml`.
- [2026-03-26 22:06] `crates/commands/validation` now carries an executable Rust replacement for `validate-all` orchestration, so `healthcheck` no longer depends on the top-level PowerShell validation wrapper even though Wave 2 still delegates the individual validation checks.
- [2026-03-26 16:48] Large files in `orchestrator` and `cli` are already past the comfort threshold for safe broad migration work and should be treated as hygiene risk, not as default extension points.

## Ordered Tasks

### Task 1: Rebaseline Repository Hygiene Around Full Script Transcription

Status: `[x]` Completed

- [2026-03-26 16:20] Updated the hygiene plan so it supports the full PowerShell-to-Rust transcription program instead of unrelated follow-up backlog ✓ [2026-03-26 16:20]
- [2026-03-26 16:48] Recorded the current Rust hygiene baseline so formatting debt and test-structure gaps are explicit before migration expansion ✓ [2026-03-26 16:48]
- Target paths:
  - `planning/active/plan-repository-operations-hygiene.md`
  - `planning/active/plan-repository-unification-and-rust-migration.md`
  - `planning/specs/active/spec-repository-unification-and-rust-migration.md`
- Commands:
  - `git status --short --branch`
  - `rg --files scripts -g *.ps1`
- Checkpoints:
  - hygiene planning clearly supports the migration plan rather than competing with it
  - migration guardrails are explicit and versioned
- Commit checkpoint:
  - `docs(planning): align repository hygiene with rust transcription scope`

### Task 2: Remove Blocking Dependency And Toolchain Debt Before Expansion

Status: `[ ]` Pending

- [2026-03-26 16:20] Clear the dependency, formatting, and test-contract debt that would make broad Rust expansion noisy or unsafe
- [2026-03-26 17:05] Added the missing external test entry surfaces for `crates/commands` and `crates/task-worker`; formatting debt remains open, but the most immediate test-contract gap is now reduced ✓ [2026-03-26 17:05]
- Target paths:
  - `Cargo.toml`
  - `Cargo.lock`
  - `crates/*/Cargo.toml`
  - `crates/commands/`
  - `crates/task-worker/`
  - `.github/workflows/`
- Commands:
  - `cargo audit`
  - `pwsh -File C:\Users\tguis\.github\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -ProjectPath . -FailOnSeverities Critical,High`
  - `cargo fmt --all -- --check`
  - `cargo test --workspace`
- Checkpoints:
  - no accepted dependency exception blocks the next Rust migration waves
  - `cargo fmt --check` is restored as a reliable workspace gate
  - `crates/commands` and `crates/task-worker` are aligned with the repository Rust test contract
  - CI prerequisites for new crates and commands remain explicit
  - migration work does not hide supply-chain debt behind feature pressure
- Commit checkpoint:
  - `chore(deps): clear rust migration blocking debt`

### Task 3: Define The Parity Ledger And Coverage Policy For All Scripts

Status: `[x]` Completed

- [2026-03-26 16:20] Create the canonical evidence model that proves each PowerShell script is covered by Rust parity before cutover
- [2026-03-26 17:18] Captured the versioned parity ledger in `planning/active/rust-script-parity-ledger.md` and locked the required evidence model for every script domain ✓ [2026-03-26 17:18]
- Target paths:
  - `planning/active/plan-repository-unification-and-rust-migration.md`
  - `planning/active/rust-script-parity-ledger.md`
  - `scripts/tests/`
  - `tests/`
  - `.github/workflows/`
- Commands:
  - `rg --files scripts -g *.ps1`
  - `cargo test`
  - `git diff --check`
- Checkpoints:
  - every script domain has a parity evidence expectation
  - PowerShell-heavy runtime tests have a Rust-native successor plan
  - cutover cannot happen without recorded parity status
- Commit checkpoint:
  - `docs(quality): define parity ledger for rust script migration`

### Task 4: Harden CI, Validation, And Wrapper Governance For Rust Cutover

Status: `[ ]` Pending

- [2026-03-26 16:20] Make validation and CI gates prefer Rust-backed execution as migration waves land, without removing fallback safety too early
- [2026-03-26 17:11] The new `runtime` and `validation` command crates now exist and already carry contract tests, so CI hardening can target concrete Rust surfaces instead of future placeholders ✓ [2026-03-26 17:11]
- [2026-03-26 18:47] The first executable runtime replacement (`update/query-local-context-index`) now runs from Rust, so CI hardening can validate both contract and behavior on a real migrated surface ✓ [2026-03-26 18:47]
- [2026-03-26 19:55] The runtime hygiene diagnosis path (`doctor`) now runs from Rust for audit-only flows, while bootstrap-driven remediation remains explicitly pending the `bootstrap` port ✓ [2026-03-26 19:55]
- [2026-03-26 20:05] The runtime health evidence path (`healthcheck`) now runs from Rust for report/log orchestration, while `validate-all` remains a temporary delegated PowerShell step until Wave 2 ✓ [2026-03-26 20:05]
- [2026-03-26 20:14] The runtime sync path (`bootstrap`) now runs from Rust for repository-managed asset projection, while provider render dispatch and MCP config apply remain explicit delegated substeps ✓ [2026-03-26 20:14]
- [2026-03-26 20:33] The runtime repair path (`self-heal`) now runs from Rust for bootstrap-plus-healthcheck orchestration and persisted evidence, while optional VS Code template application remains an explicit delegated PowerShell step ✓ [2026-03-26 20:33]
- [2026-03-26 20:47] The VS Code workspace template apply path now runs from Rust too, so `self-heal` no longer delegates any repair step to PowerShell in the Wave 1 runtime surface ✓ [2026-03-26 20:47]
- [2026-03-26 20:53] The runtime crate/test tree is now grouped by capability (`sync`, `diagnostics`, `continuity`) instead of accumulating new flat files at the root, preserving the repository Rust organization rule as the migration expands ✓ [2026-03-26 20:53]
- [2026-03-26 21:11] The runtime doctor remediation path (`doctor -SyncOnDrift`) now runs from Rust too, reusing the Rust bootstrap implementation and preserving explicit failure propagation when remediation cannot complete ✓ [2026-03-26 21:11]
- [2026-03-26 21:32] The bootstrap provider render path now runs from Rust too, so runtime sync no longer shells out to the PowerShell dispatcher before projecting repository-managed assets ✓ [2026-03-26 21:32]
- [2026-03-26 21:39] The bootstrap MCP apply path now runs from Rust too, so runtime sync no longer shells out to the PowerShell Codex config rewrite helper ✓ [2026-03-26 21:39]
- [2026-03-26 22:06] The top-level validation evidence path now runs through Rust too: `healthcheck` calls the Rust `validate-all` orchestrator, which owns profile selection, report emission, and ledger repair/write while keeping per-check legacy execution explicit until the rest of Wave 2 lands ✓ [2026-03-26 22:06]
- Target paths:
  - `.github/workflows/ci.yml`
  - `.github/workflows/release.yml`
  - `scripts/validation/`
  - `scripts/runtime/`
  - `scripts/git-hooks/`
- Commands:
  - `cargo fmt --all -- --check`
  - `cargo test`
  - `cargo clippy --workspace --all-targets -- -D warnings`
- Checkpoints:
  - CI can validate Rust-backed replacements per wave
  - wrappers and hook entrypoints remain deterministic during partial migration
  - policy scripts do not drift away from the real Rust execution path
  - top-level runtime health and top-level validation orchestration now share Rust-owned execution boundaries
- Commit checkpoint:
  - `ci(rust): harden migration validation and wrapper governance`

### Task 5: Preserve Artifact Hygiene And Operator Recovery Paths During Transition

Status: `[~]` In Progress

- [2026-03-26 16:20] Keep `.build/`, `.deployment/`, local runtime state, and recovery helpers stable while the execution engine changes underneath
- [2026-03-26 20:33] Migrated `self-heal` orchestration into `crates/commands/runtime`, preserving JSON/log evidence and repair sequencing while keeping the optional VS Code template bridge explicit ✓ [2026-03-26 20:33]
- [2026-03-26 20:47] Migrated `apply-vscode-templates` into `crates/commands/runtime`, removing the last self-heal-internal bridge and keeping workspace template projection under typed Rust ownership ✓ [2026-03-26 20:47]
- [2026-03-26 21:11] Migrated `doctor -SyncOnDrift` remediation into `crates/commands/runtime`, so direct runtime repair and audit re-checks no longer need the PowerShell wrapper path ✓ [2026-03-26 21:11]
- [2026-03-26 21:32] Migrated the bootstrap provider render dispatcher into `crates/commands/runtime`, so repository projection and runtime sync now share the same Rust-owned execution path for the bootstrap consumer ✓ [2026-03-26 21:32]
- [2026-03-26 21:39] Migrated the bootstrap MCP config rewrite into `crates/commands/runtime`, so runtime sync now owns both the catalog-driven Codex config projection and backup behavior without the PowerShell helper ✓ [2026-03-26 21:39]
- Target paths:
  - `.build/`
  - `.deployment/`
  - `scripts/runtime/`
  - `scripts/maintenance/`
  - `docs/`
- Commands:
  - `git status --short`
  - `cargo test`
  - targeted local smoke tests for bootstrap, doctor, clean, and self-heal flows
- Checkpoints:
  - generated artifacts remain centralized and predictable
  - operator recovery flows stay documented and usable
  - migration work does not reintroduce ad hoc temporary-state sprawl
- Commit checkpoint:
  - `chore(ops): preserve artifact hygiene during rust cutover`

## Deferred Backlog Preserved From Prior Hygiene Scope

- `Propagate Typed Control-Plane Metadata To Outbound Notifications`
- `Reuse Real Interactive CLI Session Identity For Local Task Submit`

These items remain valid backlog, but they are intentionally deferred while the repository planning focus is the full PowerShell-to-Rust transcription program.

## Validation Checklist

- `git status --short --branch`
- `rg --files scripts -g *.ps1`
- `cargo audit`
- `cargo fmt --all -- --check`
- `cargo test`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `git diff --check`

## Recommended Specialist

- Primary: `ops-devops-platform-engineer`
- Secondary:
  - `sec-security-vulnerability-engineer`
  - `test-engineer`
  - `docs-release-engineer`

## Closeout Expectations

- Keep `README.md` aligned when operator-visible runtime, recovery, or validation flows change.
- Update `CHANGELOG.md` only when hygiene work affects release-visible behavior or migration defaults.
- Keep commit messages in English and checkpoint-oriented.
- Do not remove fallback wrappers or recovery paths without explicit parity evidence.

## Delivery Slices

- Slice A: planning and hygiene rebaseline
- Slice B: dependency and toolchain cleanup
- Slice C: parity ledger and CI hardening
- Slice D: artifact hygiene and recovery-path protection during cutover