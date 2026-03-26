# Repository Unification And Full Rust Script Transcription

Generated: 2026-03-26 16:20

## Objective

Establish `nettoolskit-copilot` as the unified long-term workspace that preserves the `nettoolskit-cli` Rust lineage, keeps `C:\Users\tguis\copilot-instructions` as the external legacy reference, and transcribes the complete tracked PowerShell script portfolio into Rust-backed runtime capabilities without breaking current operator workflows.

## Problem Statement

The current repository already contains the unified runtime surfaces, but its executable automation model still depends on a large PowerShell estate spread across `scripts/`. As of `2026-03-26`, the workspace contains `147` tracked `.ps1` scripts that cover runtime bootstrap, rendering, synchronization, validation, security, orchestration, hooks, maintenance, governance, deploy helpers, and PowerShell-based test harnesses.

Partial migration by script family is no longer enough for the desired end state. Shared helpers in `scripts/common/`, parity-critical validation logic in `scripts/validation/`, and operational wrappers in `scripts/runtime/` create cross-cutting dependencies that keep the control plane split between Rust and PowerShell unless the full portfolio is accounted for up front.

## Scope Inventory Snapshot

| Domain | Script Count | Notes |
| --- | ---: | --- |
| `scripts/runtime` | 46 | bootstrap, render, sync, housekeeping, recovery, runtime wrappers |
| `scripts/validation` | 31 | policy, architecture, parity, release, and standards validation |
| `scripts/tests` | 27 | PowerShell-based runtime and policy test harnesses |
| `scripts/common` | 15 | shared helpers, catalogs, paths, bootstrap support |
| `scripts/orchestration` | 10 | staged Super Agent execution and task workers |
| `scripts/security` | 6 | supply-chain and pre-build audit gates |
| `scripts/maintenance` | 5 | cleanup and repository maintenance utilities |
| `scripts/git-hooks` | 3 | git hook bootstrap and EOF hygiene |
| `scripts/governance` | 2 | governance automation and protection helpers |
| `scripts/deploy` | 1 | deployment helper |
| `scripts/doc` | 1 | documentation validation helper |
| Total | 147 | Full migration scope locked for planning |

## Design Summary

- Keep `nettoolskit-cli` history as the repository base and preserve the unified workspace already created in `nettoolskit-copilot`.
- Keep `C:\Users\tguis\copilot-instructions` as an external reference repository during migration instead of embedding a `legacy/` working tree inside the active repository.
- Treat every tracked `scripts/**/*.ps1` file as in-scope for Rust transcription, including validation, tests, security, governance, deploy, and hook surfaces.
- Use an inventory-driven migration model: every script must have a Rust owner, a compatibility story, a parity strategy, and a cutover rule before implementation starts.
- Keep `definitions/`, `.github/`, `.codex/`, `.claude/`, `.vscode/`, and `planning/` as the authoritative non-executable surfaces; Rust replaces execution logic, not those source-of-truth assets.
- Keep PowerShell entrypoints available as compatibility wrappers until the Rust implementation reaches proven parity and the default cutover is explicitly approved.
- Align Rust execution with the existing workspace topology first: `crates/core`, `crates/commands/*`, `crates/orchestrator`, and `crates/cli`; add new crates only when current boundaries cannot safely absorb a capability.
- Treat `.temp/arquitetura_enterprise_llm.md` as temporary source input and preserve its durable architectural direction in `planning/specs/active/spec-enterprise-rust-runtime-transcription-architecture.md`, while this spec remains the canonical migration design artifact.

## Current Rust Readiness Snapshot

- [2026-03-26 16:48] The current Rust workspace is execution-capable today: `cargo check --workspace` passed and `cargo test --workspace` passed.
- [2026-03-26 16:48] The formatting baseline is not yet release-clean for migration scale: `cargo fmt --all -- --check` failed across many existing files, so formatting debt must be treated as a real hygiene item.
- [2026-03-26 16:48] The strongest reuse candidates for the migration are:
  - `crates/core` for shared helpers currently implemented in `scripts/common`
  - `crates/orchestrator` for staged orchestration and control-plane behavior
  - `crates/cli` for user-facing verbs and compatibility aliases
  - `crates/commands/help`, `crates/commands/manifest`, and `crates/commands/templating` as the best examples of modular command crates with mirrored tests
- [2026-03-26 18:47] The first executable Wave 1 replacement is now live in Rust: `crates/core` owns repository/runtime path and local context index foundations, and `crates/commands/runtime` owns `update/query-local-context-index`.
- [2026-03-26 18:59] The planning continuity surface has started migrating too: `crates/commands/runtime` now owns a Rust-backed `export-planning-summary` flow that reuses the local context index instead of depending on PowerShell orchestration.
- [2026-03-26 19:06] The remaining Wave 1 runtime commands now have a typed shared foundation in Rust: `crates/core` owns the runtime install-profile catalog and execution-context contract previously centralized in PowerShell helpers.
- [2026-03-26 19:55] The runtime drift diagnosis path is now live in Rust too: `crates/commands/runtime` owns audit-only `doctor` behavior for GitHub/Codex mappings, strict extras, and duplicate skill detection, while `SyncOnDrift` remains pending the `bootstrap` port.
- [2026-03-26 20:05] The runtime health orchestration path is now partially live in Rust: `crates/commands/runtime` owns `healthcheck` report/log orchestration and Rust `doctor` integration, while `validate-all` and optional bootstrap execution remain temporary PowerShell bridge calls.
- [2026-03-26 20:14] The runtime sync path is now mostly live in Rust: `crates/commands/runtime` owns `bootstrap` file projection, mirror cleanup, and duplicate-skill hygiene, while provider render dispatch and MCP config apply remain temporary delegated PowerShell substeps.
- [2026-03-26 20:33] The runtime repair path is now mostly live in Rust too: `crates/commands/runtime` owns `self-heal` sequencing, persisted evidence, and Rust bootstrap-plus-healthcheck chaining, while optional VS Code template application remains a temporary delegated PowerShell substep.
- [2026-03-26 16:48] Immediate structural gaps that should be closed before broad transcription:
  - new migration code should not be added directly into the already oversized `processor.rs`, `chatops*.rs`, `cli/main.rs`, or `cli/lib.rs` files
- [2026-03-26 17:11] The missing external test surfaces for `crates/commands` and `crates/task-worker` have now been implemented, so the next structural pressure points are command-family implementation and oversized control-plane files.

## Target Rust Ownership Model

| Capability Domain | Target Owner |
| --- | --- |
| `scripts/common` | `crates/core` |
| `scripts/runtime`, `scripts/maintenance`, `scripts/git-hooks`, `scripts/runtime/hooks` | `crates/commands/runtime` plus `crates/cli` |
| `scripts/validation`, `scripts/security`, `scripts/governance`, `scripts/doc`, `scripts/deploy` | `crates/commands/validation` |
| `scripts/orchestration` | `crates/orchestrator` |
| `scripts/tests` and `tests/runtime` | Rust test suites under the owning crates plus root integration harnesses |
| background worker and retry runtime | `crates/task-worker` |
| command export hub | `crates/commands` |

The canonical script-to-owner lock is tracked in `planning/active/rust-script-transcription-ownership-matrix.md`.
The parity evidence policy is tracked in `planning/active/rust-script-parity-ledger.md`.

## Key Decisions

1. `nettoolskit-copilot` remains the canonical workspace for the migration.
2. `C:\Users\tguis\copilot-instructions` remains available as the external provenance and parity reference during the migration.
3. The full `147`-script PowerShell portfolio is in scope; no script family is silently excluded.
4. Migration waves are organized by capability domains and dependency layers, not just by ad hoc file-by-file rewrites.
5. Shared helper logic in `scripts/common/` must be ported early because runtime, validation, hooks, and governance depend on it.
6. Validation and test scripts are part of the productized runtime estate and must also receive Rust-native replacements or owners.
7. PowerShell wrappers remain available until parity is validated; removing a wrapper before parity is not allowed.
8. The Rust target should expose stable command contracts for render, sync, bootstrap, validate, security, orchestration, maintenance, hooks, governance, and deploy flows.
9. LLM-driven reasoning remains a planning and orchestration aid only; deterministic execution and repository automation must move into Rust.
10. [2026-03-26 16:48] The migration should introduce `crates/commands/runtime` and `crates/commands/validation` as the first new command boundaries instead of overloading `cli` or `orchestrator`.
11. [2026-03-26 16:48] `crates/commands` and `crates/task-worker` must be brought into the repository Rust testing contract before they become expansion anchors.
12. [2026-03-26 16:48] Oversized orchestrator and CLI files are considered migration risk surfaces and should be reduced by extraction, not used as default landing zones for ported script logic.
13. [2026-03-26 17:11] The first architecture checkpoint is implemented in code: `crates/commands/runtime` and `crates/commands/validation` now exist in the workspace, and the missing external test surfaces for `crates/commands` and `crates/task-worker` are no longer deferred.
14. [2026-03-26 18:47] Wave 1 should start with reusable helper and index flows in `crates/core` and `crates/commands/runtime`, using the local context index path as the first executable compatibility target.
15. [2026-03-26 18:59] Wave 1 planning continuity helpers should follow immediately after index migration so context handoff/export stops depending on the legacy PowerShell runtime.
16. [2026-03-26 19:06] Before porting `bootstrap`, `doctor`, and `healthcheck` end-to-end, the shared runtime profile and execution-context helpers must live in `crates/core` so those commands do not re-encode path/profile resolution independently.
17. [2026-03-26 19:55] `doctor` should land in two steps: first the audit-only Rust path, then `SyncOnDrift` after `bootstrap` has a Rust owner, so diagnosis can cut over before remediation.
18. [2026-03-26 20:05] `healthcheck` should also land in stages: Rust should own orchestration/reporting as soon as possible, but validation and remediation steps may stay delegated until their target crates are ready.
19. [2026-03-26 20:14] `bootstrap` can land before render/apply helpers fully migrate as long as the delegated substeps stay explicit and the main sync logic plus mirror hygiene move under Rust ownership first.
20. [2026-03-26 20:33] `self-heal` can land before `apply-vscode-templates` fully migrates as long as repair sequencing, persisted evidence, and the bootstrap-plus-healthcheck chain move under Rust ownership first.

## Constraints

- Preserve repository history and current remote topology.
- Avoid breaking the current Cargo workspace and existing Rust application flows.
- Do not modify or delete `C:\Users\tguis\copilot-instructions` during early migration phases.
- Maintain current operator-visible PowerShell entrypoints until approved cutover.
- Keep migration artifacts and planning under the repository-owned Super Agent workflow.
- Do not narrow the scope back to selected script families without explicit user approval.

## Alternatives Considered

### Alternative 1: Continue Migrating Only `render` And `sync` Families First

Rejected. The user has now set the planning target to full script transcription, and the shared helper plus validation dependencies make a narrow-family scope insufficient as the canonical plan.

### Alternative 2: Rewrite Everything Into One Monolithic Rust Runtime Crate

Rejected. This would collapse unrelated concerns into one unstable boundary and create long-term maintenance friction.

### Alternative 3: Replace PowerShell Entry Points Immediately With Rust Binaries

Rejected. This would raise operational risk before parity, wrapper stability, and operator-path documentation are proven.

### Alternative 4: Leave Validation And Test Scripts In PowerShell Indefinitely

Rejected. Validation and test harnesses are part of the executable control plane and must not become permanent exceptions to the Rust target.

## Risks

- Shared PowerShell helpers contain implicit coupling that may not be obvious from top-level script names alone.
- Validation, security, and hook scripts interact with OS and toolchain behavior that may need platform-specific Rust abstractions.
- The test estate is itself PowerShell-heavy, so parity coverage must evolve at the same time as the runtime implementation.
- A broad rewrite without explicit capability ownership could blur boundaries across `crates/core`, `crates/commands/*`, `crates/orchestrator`, and `crates/cli`.

## Acceptance Criteria

1. The unified repository remains the execution home and preserves the current history model.
2. The complete `147`-script PowerShell inventory is accounted for in active planning by domain, owner boundary, and migration wave.
3. The active plan sequences implementation waves that eventually cover all runtime, validation, test, helper, security, governance, hook, deploy, and maintenance scripts.
4. PowerShell wrappers remain available until Rust parity is demonstrated for the corresponding operator flow.
5. Static authorities such as `definitions/`, `.github/`, `.codex/`, `.claude/`, `.vscode/`, and `planning/` remain source-of-truth assets instead of being collapsed into generated Rust state.
6. The supporting architecture note at `.temp/arquitetura_enterprise_llm.md` is preserved as a dedicated versioned architecture spec and reflected in the versioned migration artifacts.
7. Cutover planning includes README and CHANGELOG obligations before operator-visible defaults change.
8. The current Rust baseline is explicitly assessed in the migration artifacts, including what already compiles/tests cleanly and what does not.
9. The migration artifacts define a concrete Rust ownership target for every script domain before implementation waves begin.

## Planning Readiness

- `ready-for-plan`
- Updated: `2026-03-26 16:48` — added the validated Rust baseline snapshot and concrete target ownership model.
- Updated: `2026-03-26 17:11` — implemented the first migration boundary crates and aligned the baseline test surfaces with the Rust testing contract.
- Updated: `2026-03-26 18:47` — implemented the first executable Wave 1 replacement around the local context index flow and promoted its shared foundations into `crates/core`.
- Updated: `2026-03-26 18:59` — implemented the Rust-backed planning summary export flow as the next executable Wave 1 compatibility target.
- Updated: `2026-03-26 19:06` — implemented the runtime install-profile and execution-context foundations in `crates/core` for the remaining Wave 1 runtime commands.
- Updated: `2026-03-26 19:55` — implemented the audit-only Rust `doctor` path in `crates/commands/runtime` and kept bootstrap-driven remediation explicitly deferred to the `bootstrap` migration slice.
- Updated: `2026-03-26 20:05` — implemented the Rust-backed `healthcheck` orchestration/report slice in `crates/commands/runtime` and kept validation/bootstrap delegation explicit until their owning crates take over.
- Updated: `2026-03-26 20:14` — implemented the Rust-backed `bootstrap` sync/mirror slice in `crates/commands/runtime` and switched `healthcheck -SyncRuntime` to the Rust bootstrap path.
- Updated: `2026-03-26 20:33` — implemented the Rust-backed `self-heal` repair/report slice in `crates/commands/runtime` and kept optional VS Code template application as an explicit delegated bridge.

## Recommended Specialist Focus

- `dev-rust-engineer` for implementation waves and target crate boundaries
- `plan-active-work-planner` for active migration sequencing and checkpoints
- `test-engineer` for parity harness and replacement test coverage
- `docs-release-engineer` for operator-path documentation and cutover messaging