# Repository Unification And Rust Script Transcription Plan

Generated: 2026-03-26 16:20

## Status

- LastUpdated: 2026-03-28 10:00
- Objective: convert the unified repository migration plan into a full `scripts/**/*.ps1` to Rust transcription roadmap while preserving current operator compatibility.
- Normalized Request: resume planning on a dedicated branch, keep work isolated, use `.temp/arquitetura_enterprise_llm.md` as the architectural source input, and make the migration scope cover all existing PowerShell scripts.
- Active Branch: `feature/native-validation-policy`
- Spec Path: `planning/specs/active/spec-repository-unification-and-rust-migration.md`
- Supporting Architecture Spec: `planning/specs/active/spec-enterprise-rust-runtime-transcription-architecture.md`
- Ownership Matrix: `planning/active/rust-script-transcription-ownership-matrix.md`
- Parity Ledger: `planning/active/rust-script-parity-ledger.md`
- Remaining Open Backlog: `planning/active/plan-rust-migration-closeout-and-cutover.md`
- Historical Role: implementation record for the completed migration waves; the remaining open delivery backlog is now owned by the closeout plan.
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

## Current Rust Baseline

- [2026-03-26 16:48] `cargo check --workspace` passed.
- [2026-03-26 16:48] `cargo test --workspace` passed.
- [2026-03-26 16:48] `cargo fmt --all -- --check` failed across existing files, so the formatting/EOF baseline is not yet a dependable gate.
- [2026-03-26 16:48] Reusable architectural anchors already exist:
  - `crates/core` for shared helper, filesystem, path, config, and runtime primitives
  - `crates/orchestrator` for staged orchestration and policy-aware execution
  - `crates/cli` for operator-facing verbs and compatibility entrypoints
  - `crates/commands/help`, `crates/commands/manifest`, and `crates/commands/templating` as the best current examples of modular command crates with mirrored tests
- [2026-03-26 18:47] `crates/core` now owns the first Rust-backed shared-helper foundations for repository paths, runtime location catalogs, and local context index build/search.
- [2026-03-26 18:47] `crates/commands/runtime` now executes Rust-backed `update/query-local-context-index` flows with dedicated external tests, making the first Wave 1 runtime replacement real rather than contract-only.
- [2026-03-26 18:59] `crates/commands/runtime` now executes a Rust-backed `export-planning-summary` flow that consumes the active planning surface and optional local context index references.
- [2026-03-26 19:06] `crates/core` now owns the runtime install-profile catalog and shared runtime execution context used by `bootstrap`, `doctor`, `healthcheck`, `install`, and `self-heal`.
- [2026-03-26 19:55] `crates/commands/runtime` now executes a Rust-backed `doctor` flow for GitHub/Codex runtime drift detection, strict-extra handling, and repo-managed Codex skill duplicate auditing; `SyncOnDrift` remains pending until `bootstrap` is ported.
- [2026-03-26 20:05] `crates/commands/runtime` now executes a Rust-backed `healthcheck` flow that owns report/log generation and Rust `doctor` integration while delegating `validate-all` and optional `bootstrap` through explicit PowerShell bridge steps until those surfaces migrate.
- [2026-03-26 20:14] `crates/commands/runtime` now executes a Rust-backed `bootstrap` flow for GitHub/Codex runtime sync, mirror cleanup, duplicate-skill removal, and delegated MCP/provider render hooks; `healthcheck -SyncRuntime` now uses this Rust bootstrap path.
- [2026-03-26 20:33] `crates/commands/runtime` now executes a Rust-backed `self-heal` flow that owns repair/report orchestration, Rust `bootstrap`, and Rust `healthcheck` chaining while delegating optional `apply-vscode-templates` as an explicit bridge step.
- [2026-03-26 20:47] `crates/commands/runtime` now executes a Rust-backed `apply-vscode-templates` flow for workspace template projection, and `self-heal` now uses that Rust path instead of a PowerShell bridge.
- [2026-03-26 20:53] `crates/commands/runtime` is now organized into submodules by responsibility: `sync`, `diagnostics`, and `continuity`, with tests mirrored to the same folder structure instead of keeping every runtime surface flat at the crate root.
- [2026-03-26 21:11] `crates/commands/runtime` now executes Rust-backed `doctor -SyncOnDrift` remediation too, reusing the Rust `bootstrap` path for runtime repair and re-auditing the mappings after sync.
- [2026-03-26 21:32] `crates/commands/runtime` now executes Rust-backed provider surface rendering for the `bootstrap` consumer too, so bootstrap no longer shells out to `render-provider-surfaces.ps1` before synchronizing runtime assets.
- [2026-03-26 21:39] `crates/commands/runtime` now executes Rust-backed MCP config application inside `bootstrap`, so the bootstrap path no longer shells out to `sync-codex-mcp-config.ps1` to rewrite Codex `config.toml`.
- [2026-03-26 22:06] `crates/commands/validation` now executes Rust-backed `validate-all` orchestration for profile selection, delegated validation sequencing, JSON report generation, and hash-chained ledger repair/write; `crates/commands/runtime` `healthcheck` now calls this Rust validation surface instead of shelling out to `validate-all.ps1`.
- [2026-03-26 22:24] `crates/commands/validation` now executes Rust-backed `validate-planning-structure` and `validate-audit-ledger` checks too, and `validate-all` dispatches those checks natively while the remaining Wave 2 checks stay explicitly delegated.
- [2026-03-26 22:44] `crates/commands/validation` now executes Rust-backed `validate-readme-standards` and `validate-instruction-metadata` too, under a dedicated `documentation` module; `validate-all` now preserves native warning status instead of flattening Rust-side warnings into silent passes.
- [2026-03-26 23:06] `crates/commands/validation` now executes Rust-backed `validate-routing-coverage` and `validate-template-standards` under a dedicated `governance` module, so route-fixture parity and shared template contracts now run natively in Rust too.
- [2026-03-27 08:07] `crates/commands/validation` now executes Rust-backed `validate-workspace-efficiency` under a dedicated `workspace` module, with direct external coverage for template workspaces, duplicate folders, redundant local settings, forbidden settings, heuristics, and native dispatch through `validate-all`.
- [2026-03-27 08:22] `crates/commands/validation` now executes Rust-backed `validate-authoritative-source-policy` under a dedicated `instruction_graph` module, with direct external coverage for source-map contract enforcement, required global references, duplicated official-doc domains, and native dispatch through `validate-all`.
- [2026-03-27 09:00] `crates/commands/validation` now executes Rust-backed `validate-instruction-architecture` under `instruction_graph`, with direct external coverage for manifest shape, required global references, deterministic routing hard-cap enforcement, ownership-marker warnings, skill canonical references, and native dispatch through `validate-all`.
- [2026-03-27 12:50] `crates/commands/validation/security` now owns both `validate-security-baseline` and `validate-shared-script-checksums`, so the low-risk security policy block runs natively in Rust with direct manifest-drift, checksum, and secret-baseline coverage.
- [2026-03-27 12:51] `crates/commands/validation/policy` now also owns `validate-compatibility-lifecycle-policy`, so COMPATIBILITY.md support-window enforcement runs natively in Rust beside the rest of the repository policy surfaces.
- [2026-03-27 13:21] `crates/commands/validation/standards` now owns `validate-dotnet-standards`, so .NET template governance runs natively in Rust with direct required-template, placeholder, XML summary, and whitespace hygiene coverage through `validate-all`.
- [2026-03-27 13:28] `crates/commands/validation/architecture` now owns `validate-architecture-boundaries`, so architecture baseline enforcement runs natively in Rust with direct wildcard matching, required-pattern, forbidden-pattern, allowed-exception, and severity-aware coverage through `validate-all`.
- [2026-03-27 13:29] `crates/commands/validation/security` now also owns `validate-supply-chain`, so dependency inventory, blocked/sensitive package policy checks, SBOM export, and optional license-evidence enforcement now run natively in Rust through `validate-all`.
- [2026-03-27 18:46] `crates/orchestrator` now owns typed public pipeline contract models plus manifest parsing/validation for stage order, handoff references, and completion criteria, so the remaining Wave 3 orchestration work can build on a stable Rust manifest boundary instead of ad hoc JSON/script parsing.
- [2026-03-27 18:58] `crates/commands/validation/agent_orchestration` now consumes the typed `crates/orchestrator` pipeline contract too, eliminating the duplicate local manifest model and aligning permission/skill/orchestration validation on the same Rust-owned stage, dispatch, and runtime metadata boundary.
- [2026-03-28 09:32] The staged `run-test` closeout parity path, `resume-agent-pipeline`, and `evaluate-agent-pipeline` are now stable enough to keep the full workspace test suite green again.
- [2026-03-28 09:32] Repository/runtime/validation docs and release workflows now frame PowerShell as the compatibility layer over Rust-owned command surfaces.
- [2026-03-28 09:58] Mixed `.editorconfig` EOF rules are now honored consistently across runtime Rust hooks, PowerShell maintenance scripts, and VS Code hook normalization.
- [2026-03-28 10:00] The workspace closeout baseline is green again: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and the Rust vulnerability audit all pass.
- [2026-03-26 16:48] Immediate migration blockers in the Rust layout:
  - oversized files should not become default landing zones for ported scripts:
    - `crates/orchestrator/src/execution/processor.rs` (`8151` lines)
    - `crates/orchestrator/src/execution/chatops_runtime.rs` (`2380` lines)
    - `crates/orchestrator/src/execution/chatops.rs` (`1618` lines)
    - `crates/cli/src/main.rs` (`2950` lines)
    - `crates/cli/src/lib.rs` (`2024` lines)
- [2026-03-26 17:11] The earlier external test-surface gap for `crates/commands` and `crates/task-worker` has been closed; the remaining structural risk is now concentrated in formatting debt and oversized control-plane files.

## Target Rust Ownership Model

| Script Domain | Target Rust Owner | Planned End State |
| --- | --- | --- |
| `scripts/common` | `crates/core` | Shared primitives only; no operator verb logic should remain duplicated elsewhere. |
| `scripts/runtime`, `scripts/maintenance`, `scripts/git-hooks`, `scripts/runtime/hooks` | `crates/commands/runtime` plus `crates/cli` | Rust owns runtime verbs, bootstrap, render/sync, doctor, clean, recovery, local index, and hook install/check; PowerShell becomes wrapper-only. |
| `scripts/validation`, `scripts/security`, `scripts/governance`, `scripts/doc`, `scripts/deploy` | `crates/commands/validation` | Rust owns standards validation, policy/security gates, governance checks, doc validation, and deploy preflight. |
| `scripts/orchestration` | `crates/orchestrator` | Orchestrator remains the staged control plane and event dispatch layer. |
| `scripts/tests` and `tests/runtime` | Owning crate test suites plus root integration harnesses | Parity evidence moves into Rust test surfaces and stops depending on PowerShell harnesses. |
| worker/retry runtime helpers | `crates/task-worker` | Keep background execution support narrow and reusable. |
| top-level command exports | `crates/commands` | Aggregates migration subcrates once brought into test-contract compliance. |

## Ordered Tasks

### Task 1: Rebaseline The Migration Scope Around Full Script Transcription

Status: `[x]` Completed

- [2026-03-26 16:20] Rewrote the active spec and this plan around the full `147`-script migration scope ✓ [2026-03-26 16:20]
- [2026-03-26 16:20] Captured `.temp/arquitetura_enterprise_llm.md` into a versioned architecture spec under `planning/specs/active/` and folded the relevant direction into the migration artifacts ✓ [2026-03-26 16:20]
- [2026-03-26 16:48] Captured the current Rust baseline and target ownership model so migration tasks align to the real workspace rather than an abstract future state ✓ [2026-03-26 16:48]
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
  - the active artifacts describe the current Rust baseline honestly before implementation starts
- Commit checkpoint:
  - `docs(planning): rebaseline rust migration around full script transcription`

### Task 2: Freeze The Canonical Inventory And Rust Ownership Map

Status: `[x]` Completed

- [2026-03-26 16:20] Build a canonical script-to-domain, script-to-owner, and script-to-wave matrix for all tracked `.ps1` files
- [2026-03-26 17:05] Captured the canonical matrix in `planning/active/rust-script-transcription-ownership-matrix.md` and locked every tracked PowerShell script to a Rust owner boundary and wave ✓ [2026-03-26 17:05]
- Target paths:
  - `planning/active/plan-repository-unification-and-rust-migration.md`
  - `planning/active/plan-repository-operations-hygiene.md`
  - `planning/active/rust-script-transcription-ownership-matrix.md`
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

### Task 3: Harden The Existing Rust Workspace Before Expansion

Status: `[x]` Completed

- [2026-03-26 16:48] Bring the current workspace into migration-ready shape before broad script transcription starts
- [2026-03-26 17:05] Added external `tests/test_suite.rs` and `tests/error_tests.rs` surfaces for `crates/commands` and `crates/task-worker`, and removed the inline `task-worker` test module to align that crate with the repository Rust testing pattern ✓ [2026-03-26 17:05]
- [2026-03-26 17:11] Validated the hardened baseline with `cargo test --workspace` after integrating the new migration boundary crates, keeping the workspace green at the architecture checkpoint ✓ [2026-03-26 17:11]
- Target paths:
  - `Cargo.toml`
  - `crates/commands/`
  - `crates/task-worker/`
  - `crates/orchestrator/`
  - `crates/cli/`
- Commands:
  - `cargo check --workspace`
  - `cargo test --workspace`
  - `cargo metadata --format-version 1 --no-deps`
  - `cargo fmt --all -- --check`
- Checkpoints:
  - `crates/commands` and `crates/task-worker` have an explicit test-structure remediation path
  - new migration logic no longer needs to land in the oversized orchestrator or CLI files by default
  - the workspace is ready to add migration-focused command crates without compounding current layout debt
- Commit checkpoint:
  - `refactor(rust): harden workspace baseline for script migration`

### Task 4: Define Rust Workspace Boundaries And Command Contracts

Status: `[x]` Completed

- [2026-03-26 16:48] Lock the target Rust command surfaces for runtime, validation, orchestration, maintenance, hooks, governance, deploy, and test parity flows
- [2026-03-26 17:11] Created `crates/commands/runtime` and `crates/commands/validation`, integrated both into the workspace and command aggregator, and backed the new boundaries with contract and error test suites ✓ [2026-03-26 17:11]
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
  - `cargo check --workspace`
  - `cargo test --workspace`
- Checkpoints:
  - `crates/commands/runtime` and `crates/commands/validation` are locked as the first migration-focused command boundaries
  - compatibility wrappers map to stable Rust commands and argument contracts
  - shared helper behavior has a clear home in Rust before family-level implementation starts
- Commit checkpoint:
  - `docs(rust): define target command contracts for full script transcription`

### Task 5: Implement Wave 1 Foundation For Shared Helpers And Runtime Surfaces

Status: `[x]` Completed

- [2026-03-26 16:20] Transcribe shared helper logic plus runtime/render/sync/index flows behind stable wrappers
- [2026-03-26 18:47] Ported repository path resolution, runtime location catalog resolution, and local context index build/search primitives into `crates/core` ✓ [2026-03-26 18:47]
- [2026-03-26 18:47] Implemented Rust-backed `update_local_context_index` and `query_local_context_index` commands in `crates/commands/runtime` with external test coverage ✓ [2026-03-26 18:47]
- [2026-03-26 18:59] Implemented Rust-backed `export_planning_summary` in `crates/commands/runtime` with coverage for workspace planning, `.build/super-agent` fallback, and persisted output ✓ [2026-03-26 18:59]
- [2026-03-26 19:06] Ported `runtime-install-profiles` and `runtime-execution-context` shared helpers into `crates/core`, giving the remaining Wave 1 runtime scripts a typed Rust foundation ✓ [2026-03-26 19:06]
- [2026-03-26 19:55] Implemented Rust-backed `doctor` in `crates/commands/runtime` with coverage for missing runtime files, strict extras, clean-with-extras semantics, duplicate Codex skill detection, and disabled-profile no-op behavior ✓ [2026-03-26 19:55]
- [2026-03-26 20:05] Implemented Rust-backed `healthcheck` in `crates/commands/runtime` with coverage for passed runs, runtime-drift warning conversion, hard-fail mode, JSON report generation, and log persistence ✓ [2026-03-26 20:05]
- [2026-03-26 20:14] Implemented Rust-backed `bootstrap` in `crates/commands/runtime` with coverage for GitHub sync, Codex sync, mirror cleanup, delegated MCP apply, and `healthcheck -SyncRuntime` integration ✓ [2026-03-26 20:14]
- [2026-03-26 20:33] Implemented Rust-backed `self-heal` in `crates/commands/runtime` with coverage for passed runs, optional VS Code template application, structured JSON/log output, and step-failure propagation while reusing the Rust `bootstrap` and `healthcheck` paths ✓ [2026-03-26 20:33]
- [2026-03-26 20:47] Implemented Rust-backed `apply-vscode-templates` in `crates/commands/runtime` with coverage for initial projection, skip-without-force semantics, overwrite behavior, and `self-heal` integration ✓ [2026-03-26 20:47]
- [2026-03-26 20:53] Reorganized `crates/commands/runtime` into `sync`, `diagnostics`, and `continuity` submodules and mirrored the external test tree to the same layout, reducing root-level sprawl without changing command contracts ✓ [2026-03-26 20:53]
- [2026-03-26 21:11] Implemented Rust-backed `doctor -SyncOnDrift` remediation in `crates/commands/runtime`, with coverage for successful runtime repair and explicit remediation failure propagation ✓ [2026-03-26 21:11]
- [2026-03-26 21:32] Implemented Rust-backed provider surface rendering for the `bootstrap` consumer in `crates/commands/runtime`, with coverage for GitHub, VS Code, Codex, and Claude projection gating during bootstrap-driven sync flows ✓ [2026-03-26 21:32]
- [2026-03-26 21:39] Implemented Rust-backed MCP config application for `bootstrap` in `crates/commands/runtime`, with coverage for catalog-driven Codex server projection, TOML section replacement, and timestamped backup creation ✓ [2026-03-26 21:39]
- [2026-03-26 22:06] Switched `healthcheck` to the Rust-backed `validate-all` orchestration path, removing the remaining direct dependency on `scripts/validation/validate-all.ps1` from the runtime health flow ✓ [2026-03-26 22:06]
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
  - the local context index flow no longer depends on PowerShell business logic
  - planning handoff export no longer depends on PowerShell business logic
  - bootstrap/doctor/healthcheck/install/self-heal now share a Rust-owned execution context foundation
  - `doctor` diagnosis and `-SyncOnDrift` remediation now run from Rust and reuse the Rust `bootstrap` repair path
  - `healthcheck` orchestration/reporting and top-level validation dispatch now run from Rust, and `-SyncRuntime` now uses Rust `bootstrap`
  - bootstrap-owned provider surface rendering and MCP config application now run from Rust
  - `self-heal` orchestration/reporting now runs from Rust and reuses the Rust `bootstrap` plus `healthcheck` execution path
  - `apply-vscode-templates` now runs from Rust and no longer needs a PowerShell bridge inside `self-heal`
  - the runtime health path no longer depends on `validate-all.ps1`; only the individual Wave 2 validation subchecks remain delegated
- Commit checkpoint:
  - `feat(rust): implement shared helper and runtime transcription wave`

### Task 6: Implement Wave 2 Quality, Policy, And Support Surfaces

Status: `[x]` Completed

- [2026-03-26 16:20] Transcribe validation, security, governance, maintenance, deploy, and documentation helper flows into Rust-native commands
- [2026-03-26 22:06] Implemented Rust-backed `validate-all` orchestration in `crates/commands/validation`, with profile selection, delegated check execution, JSON report generation, hash-chained ledger repair/write, and targeted external tests ✓ [2026-03-26 22:06]
- [2026-03-26 22:24] Implemented Rust-backed `validate-planning-structure` and `validate-audit-ledger` in `crates/commands/validation`, with direct external coverage and native dispatch through `validate-all` for the first per-check Wave 2 cutover ✓ [2026-03-26 22:24]
- [2026-03-26 22:44] Implemented Rust-backed `validate-readme-standards` and `validate-instruction-metadata` in `crates/commands/validation/documentation`, with direct external coverage and native dispatch through `validate-all` for the first documentation/authoring cutover slice ✓ [2026-03-26 22:44]
- [2026-03-26 23:06] Implemented Rust-backed `validate-routing-coverage` and `validate-template-standards` in `crates/commands/validation/governance`, with direct external coverage and native dispatch through `validate-all` for the first routing/template governance cutover slice ✓ [2026-03-26 23:06]
- [2026-03-27 08:07] Implemented Rust-backed `validate-workspace-efficiency` in `crates/commands/validation/workspace`, with direct external coverage and native dispatch through `validate-all` for the workspace/runtime hygiene slice ✓ [2026-03-27 08:07]
- [2026-03-27 08:22] Implemented Rust-backed `validate-authoritative-source-policy` in `crates/commands/validation/instruction_graph`, with direct external coverage and native dispatch through `validate-all` for the first instruction-graph policy slice ✓ [2026-03-27 08:22]
- [2026-03-27 09:00] Implemented Rust-backed `validate-instruction-architecture` in `crates/commands/validation/instruction_graph`, with direct external coverage and native dispatch through `validate-all` for the instruction ownership, routing-discipline, and canonical-skill-reference slice ✓ [2026-03-27 09:00]
- [2026-03-27 09:31] Implemented Rust-backed `validate-instructions` in `crates/commands/validation/instruction_graph`, with direct external coverage for required instruction assets, routing catalog references, JSON/JSONC contracts, markdown links, skill metadata, workspace-template parity, snippet references, and native dispatch through `validate-all` ✓ [2026-03-27 09:31]
- [2026-03-27 10:12] Implemented Rust-backed `validate-warning-baseline` in `crates/commands/validation/operational_hygiene`, with direct external coverage for threshold enforcement, unknown-rule warnings, analyzer-report replay, report emission, and native dispatch through `validate-all` ✓ [2026-03-27 10:12]
- [2026-03-27 10:28] Implemented Rust-backed `validate-runtime-script-tests` in `crates/commands/validation/operational_hygiene`, with direct external coverage for runtime test discovery, PowerShell execution parity, failing-test propagation, warning-only conversion, and native dispatch through `validate-all` ✓ [2026-03-27 10:28]
- [2026-03-27 10:46] Implemented Rust-backed `validate-shell-hooks` in `crates/commands/validation/operational_hygiene`, with direct external coverage for required-hook discovery, shell syntax checks, semantic guard enforcement, optional shellcheck warnings, injected tool paths, and native dispatch through `validate-all` ✓ [2026-03-27 10:46]
- [2026-03-27 11:03] Implemented Rust-backed `validate-agent-hooks` in `crates/commands/validation/agent_orchestration`, with direct external coverage for Super Agent bootstrap/selector manifests, required hook scripts, hook helper contract markers, warning-only conversion, and native dispatch through `validate-all` ✓ [2026-03-27 11:03]
- [2026-03-27 11:24] Implemented Rust-backed `validate-agent-permissions` in `crates/commands/validation/agent_orchestration`, with direct external coverage for matrix/manifest/pipeline alignment, blocked command prefixes, allowed path globs, budget contracts, stage-script permission rules, warning-only conversion, and native dispatch through `validate-all` ✓ [2026-03-27 11:24]
- [2026-03-27 11:41] Implemented Rust-backed `validate-agent-skill-alignment` in `crates/commands/validation/agent_orchestration`, with direct external coverage for manifest/eval/pipeline integrity, skill folder contracts, SKILL frontmatter, mandatory instruction references, pipeline stage role alignment, and native dispatch through `validate-all` ✓ [2026-03-27 11:41]
- [2026-03-27 12:02] Implemented Rust-backed `validate-agent-orchestration` in `crates/commands/validation/agent_orchestration`, with direct external coverage for required orchestration assets, manifest/pipeline/handoff/run-artifact integrity, eval order drift warnings, runtime catalog references, and native dispatch through `validate-all` ✓ [2026-03-27 12:02]
- [2026-03-27 12:37] Implemented Rust-backed `validate-policy` in `crates/commands/validation/policy`, with direct external coverage for repository policy files, required files/directories, forbidden files, git hook requirements, unknown-key warnings, invalid JSON failures, and native dispatch through `validate-all` ✓ [2026-03-27 12:37]
- [2026-03-27] Implemented Rust-backed `validate-security-baseline` in `crates/commands/validation/security`, with direct external coverage for required files/directories, forbidden path globs, secret-like content patterns, allowlisted content regexes, and native dispatch through `validate-all` ✓
- [2026-03-27 12:50] Implemented Rust-backed `validate-shared-script-checksums` in `crates/commands/validation/security`, with direct external coverage for manifest shape, missing manifest/source drift, SHA-256 mismatches, warning-only conversion, and native dispatch through `validate-all` ✓ [2026-03-27 12:50]
- [2026-03-27 12:51] Implemented Rust-backed `validate-compatibility-lifecycle-policy` in `crates/commands/validation/policy`, with direct external coverage for support-window section discovery, reference-date parsing, markdown lifecycle table ordering, EOL-plus-one enforcement, status alignment, hard-fail missing-file semantics, warning-only conversion, and native dispatch through `validate-all` ✓ [2026-03-27 12:51]
- [2026-03-27 13:21] Implemented Rust-backed `validate-dotnet-standards` in `crates/commands/validation/standards`, with direct external coverage for required .NET template presence, regex placeholder enforcement, XML summary warnings, whitespace hygiene, and native dispatch through `validate-all` ✓ [2026-03-27 13:21]
- [2026-03-27 13:28] Implemented Rust-backed `validate-architecture-boundaries` in `crates/commands/validation/architecture`, with direct external coverage for baseline loading, wildcard file resolution, severity-aware required/forbidden pattern enforcement, unmatched-pattern warnings, and native dispatch through `validate-all` ✓ [2026-03-27 13:28]
- [2026-03-27 13:29] Implemented Rust-backed `validate-supply-chain` in `crates/commands/validation/security`, with direct external coverage for manifest discovery, blocked dependency enforcement, invalid package warnings, required license evidence, SBOM export, and native dispatch through `validate-all` ✓ [2026-03-27 13:29]
- [2026-03-27 13:43] Implemented Rust-backed `validate-release-governance` in `crates/commands/validation/release`, with direct external coverage for required governance files, changelog semantic/date ordering, CODEOWNERS rule quality, branch-protection baseline structure, warning-only conversion, and native dispatch through `validate-all` ✓ [2026-03-27 13:43]
- [2026-03-27 13:51] Implemented Rust-backed `validate-release-provenance` in `crates/commands/validation/release`, with direct external coverage for provenance baseline loading, `validate-all` coverage checks, required evidence files, git branch/HEAD/worktree traceability, audit-report enforcement, profile-driven `RequireAuditReport`, warning-only conversion, and native dispatch through `validate-all` ✓ [2026-03-27 13:51]
- [2026-03-27 14:25] Implemented Rust-backed `validate-powershell-standards` in `crates/commands/validation/standards`, with direct external coverage for script discovery, comment-based help, parameter help coverage, strict style escalation, optional PSScriptAnalyzer replay, warning-only conversion, and native dispatch through `validate-all` ✓ [2026-03-27 14:25]
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
  - validation suite orchestration, reporting, and ledger evidence no longer depend on PowerShell-only business logic
  - `healthcheck` consumes the Rust validation boundary instead of shelling out to the legacy wrapper
  - `validate-planning-structure` and `validate-audit-ledger` no longer depend on PowerShell business logic
  - `validate-all` now mixes native and delegated execution explicitly instead of treating every Wave 2 check as a script shell-out
  - `validate-readme-standards` and `validate-instruction-metadata` no longer depend on PowerShell business logic
  - native validation warnings now remain visible to `validate-all` instead of being flattened into passed checks
  - `validate-routing-coverage` and `validate-template-standards` no longer depend on PowerShell business logic
  - `validate-workspace-efficiency` no longer depends on PowerShell business logic, and the validation crate now keeps workspace-specific rules in a dedicated `workspace/` boundary instead of expanding the root module flatly
  - `validate-authoritative-source-policy` no longer depends on PowerShell business logic, and the validation crate now keeps instruction-system policy checks in a dedicated `instruction_graph/` boundary instead of mixing them into `governance/` or the crate root
  - `validate-instruction-architecture` no longer depends on PowerShell business logic, and the instruction ownership plus routing-discipline slice now lives beside the authoritative-source policy checks inside `instruction_graph/`
  - `validate-instructions` no longer depends on PowerShell business logic, and the entire instruction-system validation block now converges inside `crates/commands/validation/instruction_graph/`
  - `validate-warning-baseline` no longer depends on PowerShell business logic, and the remaining hygiene validators now have a dedicated `operational_hygiene/` landing zone instead of spilling into `workspace/` or the crate root
  - `validate-runtime-script-tests` no longer depends on the legacy validation wrapper, and runtime smoke execution now lives beside warning-baseline governance inside `crates/commands/validation/operational_hygiene/`
  - `validate-shell-hooks` no longer depends on the legacy validation wrapper, and the full workspace/runtime hygiene validation block now converges inside `crates/commands/validation/operational_hygiene/`
  - the workspace/runtime hygiene validation block is now complete
  - `validate-agent-hooks` no longer depends on PowerShell business logic, and the agent-orchestration backlog now has a dedicated `agent_orchestration/` landing zone instead of expanding `operational_hygiene/` or the crate root
  - `validate-agent-permissions` no longer depends on PowerShell business logic, and the remaining agent-orchestration backlog is now limited to skill-alignment plus orchestration integrity
  - `validate-agent-skill-alignment` no longer depends on PowerShell business logic, and the agent-orchestration backlog is now reduced to the final structural orchestration sweep
  - `validate-agent-orchestration` no longer depends on PowerShell business logic, and the full agent policy plus orchestration validation block now converges inside `crates/commands/validation/agent_orchestration/`
  - the agent policy and orchestration validation block is now complete
  - `validate-policy` no longer depends on PowerShell business logic, and repository policy contract enforcement now lives in a dedicated `policy/` capability boundary with native dispatch through `validate-all`
  - `validate-security-baseline` no longer depends on PowerShell business logic, and security baseline enforcement now lives in a dedicated `security/` capability boundary with native dispatch through `validate-all`
  - `validate-shared-script-checksums` no longer depends on PowerShell business logic, and checksum manifest enforcement now lives beside the rest of the security policy surfaces inside `crates/commands/validation/security/`
- `validate-compatibility-lifecycle-policy` no longer depends on PowerShell business logic, and COMPATIBILITY.md lifecycle governance now lives beside the rest of the repository policy surfaces inside `crates/commands/validation/policy/`
- `validate-dotnet-standards` no longer depends on PowerShell business logic, and .NET template governance now lives in a dedicated `standards/` capability boundary with native dispatch through `validate-all`
- `validate-architecture-boundaries` no longer depends on PowerShell business logic, and architecture baseline enforcement now lives in a dedicated `architecture/` capability boundary with native dispatch through `validate-all`
- `validate-supply-chain` no longer depends on PowerShell business logic, and dependency inventory plus SBOM/license evidence enforcement now live beside the rest of the security policy surfaces inside `crates/commands/validation/security/`
- `validate-release-governance` no longer depends on PowerShell business logic, and release-governance contracts now live in a dedicated `release/` capability boundary with native dispatch through `validate-all`
- `validate-release-provenance` no longer depends on PowerShell business logic, and the full release evidence/traceability slice now lives beside `validate-release-governance` inside `crates/commands/validation/release/`
- `validate-powershell-standards` no longer depends on PowerShell business logic, and PowerShell script quality rules now live beside the rest of the language standards surfaces inside `crates/commands/validation/standards/`
- the full Wave 2 quality, policy, and support transcription block is now complete
- security gates retain or improve current severity handling
- maintenance and deploy helpers remain deterministic and operator-safe
- Next recommended slice moves to Task 7 with the runtime/git hook foundation before the broader orchestration stage migration
- Commit checkpoint:
  - `feat(rust): implement quality and policy transcription wave`

### Task 7: Implement Wave 3 Control Plane, Hooks, And Parity Harness

Status: `[x]` Completed

- [2026-03-26 16:20] Transcribe orchestration stages, git hooks, and PowerShell-based runtime tests into Rust-backed control-plane capabilities and parity coverage
- [2026-03-27 14:39] Implemented Rust-backed `setup-global-git-aliases` in `crates/commands/runtime/hooks`, with direct external coverage for isolated global Git config install/uninstall flows and runtime-managed trim-script projection ✓ [2026-03-27 14:39]
- [2026-03-27 16:59] Implemented Rust-backed `invoke-pre-commit-eof-hygiene` in `crates/commands/runtime/hooks`, with direct external coverage for local autofix/manual mode resolution, no-staged-file skip behavior, mixed-stage blocking, native trim/restage, and isolated git settings/catalog resolution ✓ [2026-03-27 16:59]
- [2026-03-27 17:12] Implemented Rust-backed `setup-git-hooks` in `crates/commands/runtime/hooks`, with direct external coverage for local install/uninstall plus global EOF-selection persistence/removal; the remaining Task 7 backlog is now orchestration-stage ownership and parity harness replacement ✓ [2026-03-27 17:12]
- [2026-03-27 17:22] Expanded Rust-backed `setup-git-hooks` to install and remove the managed global pre-commit hook path plus isolated `core.hooksPath --global` ownership, leaving Task 7 limited to orchestration-stage migration and parity harness replacement ✓ [2026-03-27 17:22]
- [2026-03-27 18:46] Implemented typed public pipeline contract models in `crates/orchestrator`, with native manifest parsing/validation and external coverage for stage order, handoff producer/consumer integrity, and completion-stage references; the next Task 7 slice is now the native parity-harness golden path for `run/resume/replay/evaluate-agent-pipeline` ✓ [2026-03-27 18:46]
- [2026-03-27 18:58] Reused the typed orchestrator pipeline contract inside `crates/commands/validation/agent_orchestration`, aligning permission, skill-alignment, and orchestration integrity checks on the same manifest parser before porting the first native parity-harness golden path ✓ [2026-03-27 18:58]
- [2026-03-27 19:25] Implemented the native `approval-approved-test` parity harness in `crates/orchestrator/tests/execution/pipeline_parity`, with deterministic fake Codex dispatch, temporary validation-green repository fixtures, replay verification, and repo-state restoration around the real PowerShell runtime; the next Task 7 slice is now the staged `run-test` closeout success path before broader `resume` and `evaluate-agent-pipeline` parity coverage ✓ [2026-03-27 19:25]
- [2026-03-27 19:42] Implemented the staged `run-test` closeout success parity harness in `crates/orchestrator/tests/execution/pipeline_parity`, covering the real PowerShell stage chain from `intake` through `closeout`, idempotent `spec/plan` replays, README/CHANGELOG mutation evidence, and plan/spec move-with-timestamp preservation; the remaining Task 7 parity backlog is now limited to `resume-agent-pipeline` and `evaluate-agent-pipeline` ✓ [2026-03-27 19:42]
- [2026-03-27 19:52] Implemented the native `evaluate-agent-pipeline` parity coverage in `crates/orchestrator/tests/execution/pipeline_parity`, verifying the repository-owned eval fixture scorecard against the real PowerShell entrypoint without Codex dispatch or repo mutation; the remaining Task 7 parity backlog is now limited to `resume-agent-pipeline` ✓ [2026-03-27 19:52]
- [2026-03-27 20:01] Implemented the native `resume-agent-pipeline` parity harness in `crates/orchestrator/tests/execution/pipeline_parity`, verifying partial-run checkpoint capture at `validate`, resumed execution from `review`, and final success metadata on the real PowerShell resume entrypoint; Task 7 is now complete and the next migration slice moves to Task 8 wrapper cutover sequencing ✓ [2026-03-27 20:01]
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
  - runtime-managed global Git aliases no longer depend on PowerShell business logic, and hook/alias migration now has a dedicated `hooks/` boundary inside `crates/commands/runtime/`
  - runtime-managed pre-commit EOF hygiene no longer depends on PowerShell business logic, and the shared local/global EOF mode resolution now lives in reusable Rust helpers for the remaining hook setup slice
  - runtime-managed `setup-git-hooks` no longer depends on PowerShell business logic for local ownership and EOF selection persistence, and the remaining Task 7 backlog is limited to orchestration-stage migration plus parity-harness replacement
  - runtime-managed `setup-git-hooks` now owns both local and managed-global `core.hooksPath` flows, including the global pre-commit wrapper installation and uninstall path
  - `crates/orchestrator` now exposes a typed Rust pipeline manifest contract for stage definitions, dispatch metadata, handoffs, and completion criteria, so the remaining orchestration-stage migration no longer needs raw JSON/script coupling
  - `crates/commands/validation/agent_orchestration` now reuses the same typed pipeline contract, so Task 7 no longer has competing Rust-side manifest models between orchestration and validation
  - the first native parity harness golden path now covers `run-agent-pipeline` plus `replay-agent-run` through the `approval-approved-test` success flow, using deterministic fake Codex dispatch and temporary validation-green fixtures instead of the legacy PowerShell-only smoke suite
  - the Wave 3 control-plane and parity-harness block is now complete: the hook-control-plane migration is native, and the `approval-approved`, staged `run-test` closeout, `evaluate-agent-pipeline`, and `resume-agent-pipeline` paths are all covered by the Rust-owned parity harness
- Commit checkpoint:
  - `feat(rust): implement control-plane and parity transcription wave`

### Task 8: Cut Over Defaults And Retire Legacy PowerShell Execution Safely

Status: `[~]` In Progress

- [2026-03-26 16:20] Switch default operator flows to Rust only after all migration waves reach parity and documentation is updated
- [2026-03-28 09:32] Repository/runtime/validation docs now describe the PowerShell entrypoints as compatibility wrappers over Rust-owned command surfaces ✓ [2026-03-28 09:32]
- [2026-03-28 09:32] CI/release governance now validates the Rust-owned release-governance and provenance path by default ✓ [2026-03-28 09:32]
- Remaining open work:
  - record the approved wrapper/default map per domain
  - explicitly promote any `parity proven` domain to `cutover ready` only after the closeout plan records the decision
  - finish artifact-isolation cleanup for parity fixtures before claiming a fully quiet closeout baseline
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
- `cargo check --workspace`
- `cargo fmt --all -- --check`
- `cargo build`
- `cargo test --workspace`
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
- Slice B: current Rust baseline hardening and migration-boundary lock
- Slice C: Wave 1 foundation for `scripts/common` plus `scripts/runtime` (`61` scripts)
- Slice D: Wave 2 quality and policy domains for `scripts/validation`, `scripts/security`, `scripts/governance`, `scripts/maintenance`, `scripts/doc`, and `scripts/deploy` (`46` scripts)
- Slice E: Wave 3 control plane for `scripts/orchestration`, `scripts/git-hooks`, and `scripts/tests` (`40` scripts)
