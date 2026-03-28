# Repository Operations Hygiene Plan

Generated: 2026-03-26 16:20

## Status

- LastUpdated: 2026-03-28 10:00
- Objective: keep repository hygiene, policy enforcement, and cutover guardrails green while the repository moves from migration implementation into Rust-default closeout.
- Normalized Request: align the operations hygiene plan with the repository-wide decision to transcribe every tracked PowerShell script into Rust, using `.temp/arquitetura_enterprise_llm.md` only as architectural source input while preserving prior hygiene obligations that still matter to migration safety.
- Active Branch: `feature/native-validation-policy`
- Spec Path: `planning/specs/active/spec-repository-unification-and-rust-migration.md`
- Supporting Architecture Spec: `planning/specs/active/spec-enterprise-rust-runtime-transcription-architecture.md`
- Ownership Matrix: `planning/active/rust-script-transcription-ownership-matrix.md`
- Parity Ledger: `planning/active/rust-script-parity-ledger.md`
- Remaining Open Backlog: `planning/active/plan-rust-migration-closeout-and-cutover.md`
- Historical Role: hygiene record for the migration waves; this artifact now owns only the remaining non-functional closeout gates.
- Worktree Isolation: not recommended for this planning-only update; a dedicated branch is active in the current checkout.

## Scope Summary

This plan complements the main migration roadmap. It does not own feature delivery slices; it owns the hygiene gates that must remain stable while the Rust transcription expands across the repository.

Current hygiene priorities for the migration:

- keep the Cargo workspace and dependency baseline healthy before adding more Rust surface area
- define parity evidence and audit visibility for all `147` PowerShell scripts
- harden CI and validation expectations so Rust-backed replacements become the default quality gate
- preserve wrapper safety, hook integrity, and artifact hygiene during the transition
- rebaseline the remaining hygiene work around closeout rather than around earlier migration waves

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
- [2026-03-26 22:24] `crates/commands/validation` now carries executable Rust replacements for `validate-planning-structure` and `validate-audit-ledger`, so the first individual Wave 2 checks also emit hygiene evidence without PowerShell wrappers.
- [2026-03-26 22:44] `crates/commands/validation` now carries executable Rust replacements for `validate-readme-standards` and `validate-instruction-metadata`, and `validate-all` now preserves native warning outcomes so documentation/authorship drift cannot be hidden by the orchestration layer.
- [2026-03-26 23:06] `crates/commands/validation` now carries executable Rust replacements for `validate-routing-coverage` and `validate-template-standards`, so routing-fixture drift and shared template contract drift now emit hygiene evidence without PowerShell wrappers too.
- [2026-03-27 08:07] `crates/commands/validation` now carries an executable Rust replacement for `validate-workspace-efficiency` under a dedicated `workspace` module, so workspace/runtime hygiene evidence and VS Code workspace policy drift no longer rely on the PowerShell validator.
- [2026-03-27 08:22] `crates/commands/validation` now carries an executable Rust replacement for `validate-authoritative-source-policy` under a dedicated `instruction_graph` module, so centralized official-doc policy drift no longer relies on the PowerShell validator either.
- [2026-03-27 09:00] `crates/commands/validation` now carries an executable Rust replacement for `validate-instruction-architecture` under the same `instruction_graph` module, so instruction ownership, deterministic routing budget, and canonical skill-reference drift no longer rely on the PowerShell validator either.
- [2026-03-28 09:32] Operator-facing docs and release workflows now describe PowerShell entrypoints as compatibility surfaces and validate the Rust-owned release-governance/provenance path by default.
- [2026-03-28 09:35] The native orchestrator parity harness is now stable for the staged `run-test` closeout path plus `resume` / `evaluate-agent-pipeline`; the remaining non-functional gap is fixture cleanup, not missing parity coverage.
- [2026-03-28 09:40] The remaining closeout clippy blockers were cleared in `runtime`, and follow-up closeout fixes already brought the full workspace back under `-D warnings`.
- [2026-03-28 09:58] Runtime PowerShell hook helpers, maintenance trim logic, and VS Code hook normalization now honor mixed `.editorconfig` `insert_final_newline` rules instead of assuming a single repository-wide default.
- [2026-03-28 10:00] `cargo fmt --all -- --check` passed after persisting the workspace Rust EOF/format baseline.
- [2026-03-28 10:00] `cargo clippy --workspace --all-targets -- -D warnings` passed.
- [2026-03-28 10:00] `cargo test --workspace` passed.
- [2026-03-28 10:00] `Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High` passed.
- [2026-03-28 10:00] Full-workspace parity runs still project temporary approval/readme/workflow artifacts into the repo during some fixture paths; cleanup is deterministic, but fixture isolation remains the main artifact-hygiene follow-up.
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

Status: `[x]` Completed

- [2026-03-26 16:20] Clear the dependency, formatting, and test-contract debt that would make broad Rust expansion noisy or unsafe
- [2026-03-26 17:05] Added the missing external test entry surfaces for `crates/commands` and `crates/task-worker`; formatting debt remains open, but the most immediate test-contract gap is now reduced ✓ [2026-03-26 17:05]
- [2026-03-28 10:00] Restored the full closeout hygiene baseline: workspace `fmt`, workspace `clippy`, workspace tests, and the Rust vulnerability audit all pass again ✓ [2026-03-28 10:00]
- [2026-03-28 10:00] Closed the mixed EOF-policy drift between Rust runtime helpers, PowerShell maintenance scripts, and VS Code hook normalization, so hygiene behavior now matches the repository `.editorconfig` contract per file type ✓ [2026-03-28 10:00]
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

Status: `[x]` Completed

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
- [2026-03-26 22:24] The first individual validation checks now run through Rust too: `validate-planning-structure` and `validate-audit-ledger` execute natively under `crates/commands/validation`, and `validate-all` routes them without shelling out to their legacy scripts ✓ [2026-03-26 22:24]
- [2026-03-26 22:44] The first documentation/authoring validation checks now run through Rust too: `validate-readme-standards` and `validate-instruction-metadata` execute natively under `crates/commands/validation/documentation`, and `validate-all` now preserves native warning status instead of collapsing it into a pass ✓ [2026-03-26 22:44]
- [2026-03-26 23:06] The first routing/template governance checks now run through Rust too: `validate-routing-coverage` and `validate-template-standards` execute natively under `crates/commands/validation/governance`, and `validate-all` now surfaces their outcomes without the PowerShell bridge ✓ [2026-03-26 23:06]
- [2026-03-27 08:07] The workspace-efficiency hygiene check now runs through Rust too: `validate-workspace-efficiency` executes natively under `crates/commands/validation/workspace`, and `validate-all` routes it without the PowerShell bridge while keeping the validation crate grouped by responsibility ✓ [2026-03-27 08:07]
- [2026-03-27 08:22] The authoritative-source policy check now runs through Rust too: `validate-authoritative-source-policy` executes natively under `crates/commands/validation/instruction_graph`, and `validate-all` routes it without the PowerShell bridge while keeping instruction-system policy checks out of the crate root ✓ [2026-03-27 08:22]
- [2026-03-27 09:00] The instruction-architecture ownership check now runs through Rust too: `validate-instruction-architecture` executes natively under `crates/commands/validation/instruction_graph`, and `validate-all` routes it without the PowerShell bridge while keeping instruction-system ownership/routing rules grouped with the rest of the instruction graph ✓ [2026-03-27 09:00]
- [2026-03-27 09:31] The top-level instruction asset sweep now runs through Rust too: `validate-instructions` executes natively under `crates/commands/validation/instruction_graph`, and `validate-all` routes it without the PowerShell bridge while keeping required instruction assets, catalog references, authoring links, and skill metadata checks inside the same capability boundary ✓ [2026-03-27 09:31]
- [2026-03-27 10:12] The analyzer warning baseline check now runs through Rust too: `validate-warning-baseline` executes natively under `crates/commands/validation/operational_hygiene`, and `validate-all` routes it without the PowerShell bridge while keeping warning-threshold governance, report emission, and analyzer replay logic out of the crate root ✓ [2026-03-27 10:12]
- [2026-03-27 10:28] The runtime script smoke suite now runs through Rust too: `validate-runtime-script-tests` executes natively under `crates/commands/validation/operational_hygiene`, and `validate-all` routes it without the PowerShell bridge while keeping PowerShell test execution parity inside the same hygiene boundary ✓ [2026-03-27 10:28]
- [2026-03-27 10:46] The shell hook validation suite now runs through Rust too: `validate-shell-hooks` executes natively under `crates/commands/validation/operational_hygiene`, and `validate-all` routes it without the PowerShell bridge while keeping syntax, semantic guard, and optional shellcheck logic inside the same hygiene boundary ✓ [2026-03-27 10:46]
- [2026-03-27 11:03] The Super Agent hook contract now runs through Rust too: `validate-agent-hooks` executes natively under `crates/commands/validation/agent_orchestration`, and `validate-all` routes it without the PowerShell bridge while keeping bootstrap/selector manifests, required hook scripts, and hook-helper contract markers inside a dedicated agent boundary ✓ [2026-03-27 11:03]
- [2026-03-27 11:24] The agent permission matrix contract now runs through Rust too: `validate-agent-permissions` executes natively under `crates/commands/validation/agent_orchestration`, and `validate-all` routes it without the PowerShell bridge while keeping matrix/manifest/pipeline alignment, budget contracts, and stage permission rules inside the same dedicated agent boundary ✓ [2026-03-27 11:24]
- [2026-03-27 11:41] The agent skill contract now runs through Rust too: `validate-agent-skill-alignment` executes natively under `crates/commands/validation/agent_orchestration`, and `validate-all` routes it without the PowerShell bridge while keeping skill folder/frontmatter integrity, mandatory instruction references, eval links, and pipeline role discipline inside the same dedicated agent boundary ✓ [2026-03-27 11:41]
- [2026-03-27 12:02] The orchestration integrity contract now runs through Rust too: `validate-agent-orchestration` executes natively under `crates/commands/validation/agent_orchestration`, and `validate-all` routes it without the PowerShell bridge while keeping required orchestration assets, pipeline/handoff/run-artifact integrity, runtime catalog references, and eval-order warnings inside the same dedicated agent boundary ✓ [2026-03-27 12:02]
- [2026-03-28 09:32] Release/workflow governance now treats the Rust-owned validation path as canonical, and operator-facing runtime docs position PowerShell as compatibility wrappers rather than primary business logic ✓ [2026-03-28 09:32]
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
  - the first Wave 2 per-check validation surfaces now emit evidence through Rust-owned execution paths too
  - documentation/authorship drift checks now emit Rust-native warning/failure status without losing signal in the top-level orchestrator
  - routing and template governance drift now emit Rust-native warning/failure status without relying on legacy scripts
  - workspace-efficiency drift now emits Rust-native warning/failure status without relying on the legacy validator, and the validation crate remains organized by capability boundaries as Wave 2 grows
  - authoritative-source policy drift now emits Rust-native warning/failure status without relying on the legacy validator, and the validation crate keeps instruction-system policy isolated under `instruction_graph/`
  - instruction-architecture drift now emits Rust-native warning/failure status without relying on the legacy validator
  - the full instruction-graph hygiene block now runs through Rust-owned checks inside `instruction_graph/`, so the remaining Wave 2 hygiene backlog is limited to workspace/runtime, agent, and release-policy slices
  - warning-baseline drift now emits Rust-native warning/failure status without relying on the legacy validator, and the remaining hygiene backlog is reduced to runtime script execution parity plus shell-hook validation
  - runtime script smoke execution now emits Rust-native warning/failure status without relying on the legacy validator, and only the shell-hook check remains before the hygiene block can be marked complete
  - shell-hook drift now emits Rust-native warning/failure status without relying on the legacy validator, and the full workspace/runtime hygiene validation block is now complete
  - agent-hook drift now emits Rust-native warning/failure status without relying on the legacy validator, and the remaining agent backlog is reduced to orchestration, skill-alignment, and permissions slices
  - agent-permission drift now emits Rust-native warning/failure status without relying on the legacy validator, and the remaining agent backlog is reduced to orchestration plus skill-alignment slices
  - agent-skill drift now emits Rust-native warning/failure status without relying on the legacy validator, and the remaining agent backlog is reduced to the final orchestration integrity sweep
  - orchestration-asset drift now emits Rust-native warning/failure status without relying on the legacy validator, and the full agent policy plus orchestration validation block is now complete
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
- [2026-03-28 11:40] The main artifact/recovery closeout blocker is now the parity closeout test's Windows file-lock behavior during temporary validation baseline projection; runtime repair flows themselves are no longer the dominant gap.
- [2026-03-28 10:00] The main remaining hygiene follow-up is fixture isolation: full-workspace parity suites still touch projected approval/readme/workflow artifacts inside the real repository before cleanup, which is recoverable but still noisier than the desired closeout baseline.
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
