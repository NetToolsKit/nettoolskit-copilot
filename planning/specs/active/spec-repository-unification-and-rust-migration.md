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
- [2026-03-26 20:47] The runtime VS Code workspace apply path is now live in Rust too: `crates/commands/runtime` owns `apply-vscode-templates`, and `self-heal` no longer delegates that repair step to PowerShell.
- [2026-03-26 20:53] The runtime crate has now been restructured by capability as well: `sync`, `diagnostics`, and `continuity` submodules replace the previous flat root layout, and the mirrored external test tree follows the same grouping.
- [2026-03-26 21:11] The runtime doctor remediation path is now live in Rust too: `crates/commands/runtime` owns `doctor -SyncOnDrift` by chaining the Rust `bootstrap` implementation and re-auditing drift after the repair attempt.
- [2026-03-26 21:32] The bootstrap provider render path is now live in Rust too: `crates/commands/runtime` reads the projection catalog, selects bootstrap renderers by condition, and renders the managed GitHub, VS Code, Codex, and Claude surfaces without shelling out to `render-provider-surfaces.ps1`.
- [2026-03-26 21:39] The bootstrap MCP apply path is now live in Rust too: `crates/commands/runtime` reads the canonical MCP runtime catalog, projects the Codex server subset, rewrites `mcp_servers` in `config.toml`, and creates timestamped backups without shelling out to `sync-codex-mcp-config.ps1`.
- [2026-03-26 22:06] The first Wave 2 validation boundary is now live in Rust too: `crates/commands/validation` owns `validate-all` profile selection, delegated validation sequencing, JSON report generation, and hash-chained ledger repair/write, and `healthcheck` now uses that Rust surface instead of the PowerShell wrapper.
- [2026-03-26 22:24] The first individual Wave 2 validation checks are now live in Rust too: `crates/commands/validation` owns `validate-planning-structure` and `validate-audit-ledger`, and `validate-all` dispatches those checks natively while other validation checks remain explicitly delegated.
- [2026-03-26 22:44] The documentation/authoring validation slice is now live in Rust too: `crates/commands/validation/documentation` owns `validate-readme-standards` and `validate-instruction-metadata`, and `validate-all` preserves native warning status so repository authoring drift remains visible at the suite layer.
- [2026-03-26 23:06] The routing/template governance validation slice is now live in Rust too: `crates/commands/validation/governance` owns `validate-routing-coverage` and `validate-template-standards`, so static route-fixture parity and template contract drift now run through Rust-owned validation paths.
- [2026-03-27 08:07] The workspace/runtime hygiene validation slice is now live in Rust too: `crates/commands/validation/workspace` owns `validate-workspace-efficiency`, and `validate-all` dispatches it natively while preserving the validation crate's capability-based module layout.
- [2026-03-27 08:22] The first instruction-graph policy slice is now live in Rust too: `crates/commands/validation/instruction_graph` owns `validate-authoritative-source-policy`, and `validate-all` dispatches it natively while keeping instruction-system policy separate from generic governance checks.
- [2026-03-27 09:00] The instruction ownership and routing-discipline slice is now live in Rust too: `crates/commands/validation/instruction_graph` owns `validate-instruction-architecture`, and `validate-all` dispatches it natively with direct coverage for manifest shape, ownership overlap, routing hard-cap enforcement, and canonical skill references.
- [2026-03-27 10:12] The next workspace/runtime hygiene slice is now live in Rust too: `crates/commands/validation/operational_hygiene` owns `validate-warning-baseline`, and `validate-all` dispatches it natively while preserving warning-threshold governance, analyzer replay, and report emission semantics.
- [2026-03-27 10:28] The runtime smoke-execution slice is now live in Rust too: `crates/commands/validation/operational_hygiene` owns `validate-runtime-script-tests`, and `validate-all` dispatches it natively while preserving PowerShell test harness execution semantics from the legacy validator.
- [2026-03-27 10:46] The final workspace/runtime hygiene slice is now live in Rust too: `crates/commands/validation/operational_hygiene` owns `validate-shell-hooks`, and `validate-all` dispatches it natively while preserving shell syntax, semantic guard, and optional shellcheck semantics from the legacy validator.
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
21. [2026-03-26 20:47] `apply-vscode-templates` should remain a narrow workspace-surface command in `crates/commands/runtime`; it does not belong in `core`, and `self-heal` should call it directly rather than shelling out.
22. [2026-03-26 20:53] As Wave 1 expands, `crates/commands/runtime` should group modules by responsibility instead of keeping every command file at `src/` root; the external test tree must mirror that folder structure exactly.
23. [2026-03-26 21:11] `doctor` should call Rust `bootstrap` directly for `-SyncOnDrift` instead of shelling out to the PowerShell wrapper, and the result contract should expose whether remediation was attempted and whether it cleared drift.
24. [2026-03-26 21:32] The `bootstrap` consumer of `render-provider-surfaces` should move under Rust ownership before the direct maintenance form of that command; bootstrap is the critical path, and the remaining MCP generation path can stay separate until its dedicated slice lands.
25. [2026-03-26 21:39] The `bootstrap` consumer of Codex MCP config apply should also move under Rust ownership before any broader MCP maintenance command cutover; the critical bootstrap path only needs catalog-driven Codex projection and backup semantics.
26. [2026-03-26 22:06] `validate-all` orchestration should move under Rust ownership before the individual validation scripts, so runtime health flows can bind to a stable validation boundary while Wave 2 continues migrating each policy check independently.
27. [2026-03-26 22:24] Once `validate-all` is Rust-owned, Wave 2 should cut over individual validation checks in small native slices, starting with repository-structure and evidence-chain validators that have low external dependency risk.
28. [2026-03-26 22:44] Native validation slices must preserve warning semantics end-to-end in `validate-all`; the orchestration layer should not treat Rust-side warnings as passes just because the exit code is zero.
29. [2026-03-26 23:06] After the repository-structure, documentation, and governance slices land, the remaining Wave 2 sequence should prioritize workspace-efficiency and instruction-graph validation before the higher-coupling agent/security/release checks.
30. [2026-03-27 08:07] Once `validate-workspace-efficiency` lands, the next Wave 2 slices should stay capability-grouped as well: instruction-graph checks should form their own authoring/policy boundary instead of expanding `governance/` or the crate root ad hoc.
31. [2026-03-27 09:00] With `instruction_graph/` now owning both `validate-authoritative-source-policy` and `validate-instruction-architecture`, the next slice in that boundary should be `validate-instructions`, so the entire instruction system converges inside one cohesive Rust module tree before the plan marks that block complete.
32. [2026-03-27 09:31] Once `validate-instructions` lands, the instruction-system validation block is complete and Wave 2 should move to the remaining hygiene cluster (`validate-warning-baseline`, `validate-runtime-script-tests`, `validate-shell-hooks`) before expanding back into the higher-coupling agent and release-policy checks.
33. [2026-03-27 10:12] After `validate-warning-baseline` lands, the remaining hygiene cluster should stay in the same capability family: `validate-runtime-script-tests` and `validate-shell-hooks` should join `operational_hygiene/` so runtime smoke checks and hook validation converge before the plan marks that hygiene block complete.
34. [2026-03-27 10:28] After `validate-runtime-script-tests` lands, only `validate-shell-hooks` remains in the hygiene cluster, so closing that final slice should be treated as the explicit completion point for the entire workspace/runtime hygiene validation block.
35. [2026-03-27 10:46] With `validate-shell-hooks` now live, the full workspace/runtime hygiene validation block is complete; the next Wave 2 execution should move to agent-orchestration and release/policy surfaces while keeping them grouped in similarly cohesive Rust module families.
36. [2026-03-27 11:03] Once `validate-agent-hooks` lands, the rest of the agent-policy work should stay inside `agent_orchestration/`; `validate-agent-permissions`, `validate-agent-skill-alignment`, and `validate-agent-orchestration` should close that whole block before the plan expands into release/security policy checks.
37. [2026-03-27 11:24] Once `validate-agent-permissions` lands, the remaining agent-policy execution should stay on the same typed fixtures and shared models; `validate-agent-skill-alignment` should reuse the manifest/pipeline/eval graph next, and `validate-agent-orchestration` should close the block as the final structural integrity sweep.
38. [2026-03-27 11:41] Once `validate-agent-skill-alignment` lands, only `validate-agent-orchestration` remains in the agent block; that last slice should be treated as the explicit completion point for all agent-policy and orchestration validation work in Wave 2.
39. [2026-03-27 12:02] With `validate-agent-orchestration` now live, the full agent-policy and orchestration validation block is complete; the next Wave 2 execution should move entirely to release, supply-chain, architecture-boundary, and standards-policy surfaces.

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
- Updated: `2026-03-26 20:47` — implemented the Rust-backed `apply-vscode-templates` workspace slice in `crates/commands/runtime` and removed that bridge from the `self-heal` flow.
- Updated: `2026-03-26 20:53` — reorganized the runtime crate and mirrored tests into `sync`, `diagnostics`, and `continuity` submodules so Wave 1 growth stays aligned with the repository Rust organization rules.
- Updated: `2026-03-26 21:11` — implemented the Rust-backed `doctor -SyncOnDrift` remediation slice and removed the remaining PowerShell wrapper dependency from runtime drift repair.
- Updated: `2026-03-26 21:32` — implemented the Rust-backed bootstrap provider render slice and removed the PowerShell dispatcher dependency from the bootstrap projection path.
- Updated: `2026-03-26 21:39` — implemented the Rust-backed bootstrap MCP apply slice and removed the PowerShell Codex config rewrite dependency from the bootstrap path.
- Updated: `2026-03-26 22:06` — implemented the Rust-backed `validate-all` orchestration slice in `crates/commands/validation` and switched `healthcheck` to that Rust validation boundary.
- Updated: `2026-03-26 22:24` — implemented the first native per-check Wave 2 slice in `crates/commands/validation` for `validate-planning-structure` and `validate-audit-ledger`, and routed both through `validate-all`.
- Updated: `2026-03-26 22:44` — implemented the documentation/authoring Wave 2 slice in `crates/commands/validation/documentation` for `validate-readme-standards` and `validate-instruction-metadata`, and fixed `validate-all` to preserve native warning status.
- Updated: `2026-03-26 23:06` — implemented the routing/template governance Wave 2 slice in `crates/commands/validation/governance` for `validate-routing-coverage` and `validate-template-standards`.
- Updated: `2026-03-27 08:07` — implemented the workspace-efficiency Wave 2 slice in `crates/commands/validation/workspace`, routed it through `validate-all`, and kept validation growth aligned to capability-specific submodules.
- Updated: `2026-03-27 09:00` — implemented the authoritative-source-policy and instruction-architecture Wave 2 instruction-graph slices in `crates/commands/validation/instruction_graph`, routed both through `validate-all`, and reduced the remaining instruction-system backlog to `validate-instructions`.
- Updated: `2026-03-27 09:31` — implemented the `validate-instructions` Wave 2 instruction-graph slice in `crates/commands/validation/instruction_graph`, routed it through `validate-all`, and closed the full instruction-system validation block before moving to the remaining hygiene backlog.
- Updated: `2026-03-27 10:12` — implemented the `validate-warning-baseline` Wave 2 hygiene slice in `crates/commands/validation/operational_hygiene`, routed it through `validate-all`, and reduced the remaining hygiene backlog to runtime script execution parity plus shell-hook validation.
- Updated: `2026-03-27 10:28` — implemented the `validate-runtime-script-tests` Wave 2 hygiene slice in `crates/commands/validation/operational_hygiene`, routed it through `validate-all`, and reduced the remaining hygiene backlog to the final shell-hook validation slice.
- Updated: `2026-03-27 10:46` — implemented the `validate-shell-hooks` Wave 2 hygiene slice in `crates/commands/validation/operational_hygiene`, routed it through `validate-all`, and closed the full workspace/runtime hygiene validation block.

## Recommended Specialist Focus

- `dev-rust-engineer` for implementation waves and target crate boundaries
- `plan-active-work-planner` for active migration sequencing and checkpoints
- `test-engineer` for parity harness and replacement test coverage
- `docs-release-engineer` for operator-path documentation and cutover messaging