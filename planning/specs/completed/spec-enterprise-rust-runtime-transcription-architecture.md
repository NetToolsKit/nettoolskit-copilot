# Enterprise Rust Runtime Transcription Architecture

Generated: 2026-03-26 16:20

## Objective

Preserve the architectural intent captured in `.temp/arquitetura_enterprise_llm.md` as a versioned planning artifact and adapt it to the `nettoolskit-copilot` migration goal: transcribe the full PowerShell execution estate into a deterministic Rust runtime without losing enterprise-grade orchestration, structural intelligence, validation, and operator safety.

## Source Material

- Temporary reference: `.temp/arquitetura_enterprise_llm.md`
- Primary workstream: `planning/specs/completed/spec-repository-unification-and-rust-migration.md`
- Completed execution plan: `planning/completed/plan-repository-unification-and-rust-migration.md`

## Problem Statement

The temporary architecture note correctly argues that enterprise-grade code automation should not depend on raw LLM generation alone. However, that direction must be translated into the actual repository target: a Rust-owned execution plane that replaces PowerShell script logic while keeping static repository surfaces authoritative and preserving validation-first delivery.

Without a versioned architecture artifact, the reasoning in `.temp/arquitetura_enterprise_llm.md` can be lost when temporary files are deleted, and the migration may drift back into file-by-file rewrites without a stable enterprise execution model.

## Architecture Summary

The target architecture is a layered enterprise automation model:

1. `Super Agent` remains the planning and governance control plane.
2. Rust becomes the deterministic execution plane for runtime, validation, orchestration, security, and maintenance flows.
3. Static repository surfaces remain the source of truth for prompts, templates, definitions, policies, and planning.
4. Structural intelligence, retrieval, templates, and validation support Rust execution rather than replacing it.
5. PowerShell becomes a temporary wrapper layer only until parity is proven.

## Target Layer Model

```text
Operator / VS Code / CI / Git Hooks
            |
            v
Super Agent Control Plane
  - intake
  - spec
  - plan
  - routing
  - validation / review / closeout
            |
            v
Compatibility Wrapper Layer
  - existing scripts/**/*.ps1 entrypoints
  - thin shims only while parity is pending
            |
            v
Rust Command Surface
  - crates/cli
  - crates/commands/*
            |
            v
Rust Execution Services
  - crates/core
  - crates/orchestrator
  - focused supporting modules when justified
            |
            v
Authoritative Static Surfaces
  - definitions/
  - .github/
  - .codex/ .claude/ .vscode/
  - planning/
            |
            v
Validation And Parity Gates
  - cargo test
  - cargo clippy
  - parity checks
  - CI policy enforcement
```

## Enterprise Capability Layers

### 1. Structural Intelligence

Purpose:

- understand repository structure deterministically
- map script responsibilities to Rust owners
- support safe refactors and parity analysis

Examples:

- AST and parser-backed inspection where applicable
- manifest and catalog parsing
- deterministic file-graph analysis

### 2. Contextual Retrieval

Purpose:

- reopen only the minimum planning, definitions, and repository context required for a migration slice
- avoid reasoning from stale temporary notes or raw chat history

Repository meaning:

- use `planning/`, `definitions/`, `.github/`, and local catalogs as authoritative context
- use `.temp` only as disposable source input, never as the long-term source of truth

### 3. Deterministic Generation

Purpose:

- render runtime artifacts, sync outputs, templates, manifests, and workspace surfaces from Rust without relying on ad hoc script duplication

Repository meaning:

- render and sync flows should move into stable Rust commands
- generated outputs must still respect authoritative static sources

### 4. Validation And Policy Enforcement

Purpose:

- preserve enterprise quality during migration
- prevent a "ported but unverified" runtime estate

Repository meaning:

- validation, security, governance, and parity scripts are first-class migration scope
- Rust is not complete until the validation model also has a Rust-backed owner

### 5. LLM Reasoning

Purpose:

- assist planning, ambiguity resolution, architecture reasoning, and operator guidance

Constraint:

- LLM output is not the final execution substrate
- deterministic runtime behavior must live in Rust, not in generated chat responses

## Repository Boundary Decisions

1. `crates/core` owns shared runtime support, path logic, catalogs, manifests, and filesystem services.
2. `crates/commands/*` own stable domain verbs such as render, sync, validate, security, maintenance, governance, and deploy.
3. `crates/orchestrator` owns staged execution, replay, resume, and dispatch behavior.
4. `crates/cli` owns user-facing invocation, discovery, and wrapper delegation.
5. `definitions/`, `.github/`, `.codex/`, `.claude/`, `.vscode/`, and `planning/` remain static authority surfaces and must not be collapsed into generated runtime state.

## Migration Implications

- `147` tracked PowerShell scripts are in scope.
- Shared helper logic must move early because it is a dependency multiplier across runtime, validation, hooks, and governance.
- Validation and test harnesses must be transcribed too; they are not long-term exceptions.
- Wrapper removal is a cutover step, not a first-port step.
- README and CHANGELOG updates are part of the architecture because operator-path clarity is part of enterprise runtime safety.

## Current Rust Baseline Assessment

- [2026-03-26 16:48] The workspace already has a viable Rust execution base with `11` Cargo members: `cli`, `core`, `ui`, `otel`, `orchestrator`, `task-worker`, `commands`, `commands/help`, `commands/manifest`, `commands/templating`, and `benchmarks`.
- [2026-03-26 16:48] Baseline validation is partially healthy: `cargo check --workspace` passed, `cargo test --workspace` passed, and `cargo fmt --all -- --check` failed across many existing files because the repository formatting/EOF baseline is not currently green.
- [2026-03-26 16:48] The best migration anchors already exist:
  - `crates/core` for shared runtime contracts, path logic, config, file search, async utilities, and deterministic filesystem helpers
  - `crates/orchestrator` for staged execution, policy-aware processing, chatops/service runtime, and control-plane dispatch
  - `crates/cli` for operator-facing invocation and compatibility aliases
  - `crates/commands/help`, `crates/commands/manifest`, and `crates/commands/templating` as the strongest current examples of command-focused Rust subcrates with mirrored test structure
- [2026-03-26 16:48] Structural debt is already visible and should not absorb new script-port logic directly:
  - `crates/orchestrator/src/execution/processor.rs` is `8151` lines
  - `crates/orchestrator/src/execution/chatops_runtime.rs` is `2380` lines
  - `crates/orchestrator/src/execution/chatops.rs` is `1618` lines
  - `crates/cli/src/main.rs` is `2950` lines
  - `crates/cli/src/lib.rs` is `2024` lines
- [2026-03-26 16:48] Test-structure debt also exists:
  - top-level `crates/commands` does not yet provide `tests/test_suite.rs` or `tests/error_tests.rs`
  - `crates/task-worker` does not yet provide external mirrored tests
- [2026-03-26 16:48] Architectural conclusion: the repository does not need a reset or reclone to host the migration, but it does need boundary hardening before broad script transcription starts.

## Target Rust Ownership Model

| Capability Family | Target Owner | Architectural Direction |
| --- | --- | --- |
| `scripts/common` | `crates/core` | Shared path, env, process, manifest, catalog, runtime state, and filesystem primitives stay centralized here. |
| `scripts/runtime`, `scripts/maintenance`, `scripts/git-hooks`, and `scripts/runtime/hooks` | `crates/commands/runtime` plus `crates/cli` | A dedicated runtime command crate should own bootstrap, render, sync, doctor, clean, self-heal, indexing, and hook install/check flows; `cli` stays a thin entry layer. |
| `scripts/validation`, `scripts/security`, `scripts/governance`, `scripts/doc`, and `scripts/deploy` | `crates/commands/validation` | A dedicated validation command crate should own policy, security, governance, documentation, and deploy-preflight flows; split into smaller subcrates only if dependency or size pressure justifies it later. |
| `scripts/orchestration` | `crates/orchestrator` | Staged execution, dispatch, replay/resume, service runtime, and policy-aware control-plane behavior remain orchestrator concerns. |
| `scripts/tests` and `tests/runtime` | Mirrored Rust test suites under owning crates plus root integration harnesses | PowerShell parity tests should become crate-owned Rust tests instead of staying as a separate scripting runtime. |
| Background queue and retry runtime | `crates/task-worker` | Keep this crate narrow and reusable; add the missing mirrored test surface before expanding responsibilities. |
| Command discovery and re-export surface | `crates/commands` | Keep the top-level command aggregator, but bring it into compliance with the Rust testing contract before it becomes the public export hub for new migration subcrates. |

The existing `help`, `manifest`, and `templating` command crates should be treated as the reference implementation style for new migration-focused subcrates.

## Migration Readiness Gates

1. Add the missing `tests/test_suite.rs` and `tests/error_tests.rs` surfaces for `crates/commands` and mirrored external tests for `crates/task-worker`.
2. Create the dedicated runtime and validation command boundaries before large-scale script ports land in the workspace.
3. Avoid placing new migration logic into the already oversized `processor.rs`, `chatops*.rs`, `cli/main.rs`, or `cli/lib.rs` files; extract modules instead.
4. Keep PowerShell entrypoints as thin wrappers only; do not allow new business logic to stay in PowerShell after a Rust owner exists.
5. Bring the formatting baseline back to green so future migration waves can use `cargo fmt --check` as a reliable gate.

## Non-Goals

- keeping a permanent mixed PowerShell and Rust execution core
- treating `.temp/` as the long-term architecture source
- replacing deterministic execution logic with LLM-only reasoning
- collapsing the workspace into a single monolithic runtime crate without clear capability boundaries

## Risks

- structural helper coupling may be deeper than current script names suggest
- Windows-specific operational behavior in hooks, runtime cleanup, and toolchain calls may need careful Rust abstractions
- parity evidence may lag behind implementation unless tests and validation are migrated alongside runtime code

## Acceptance Criteria

1. The architecture captured in `.temp/arquitetura_enterprise_llm.md` is preserved in a versioned planning artifact.
2. The versioned architecture aligns with the repository's Rust migration target rather than remaining generic.
3. The completed migration records can reference this artifact without depending on `.temp`.
4. The architecture explicitly keeps LLM reasoning as a planning aid and Rust as the execution substrate.
5. The architecture keeps static repository surfaces authoritative and PowerShell wrappers temporary.
6. The architecture documents the current Rust baseline honestly, including compile/test health, formatting debt, and test-structure gaps.
7. The architecture defines a target ownership model that explains where full-script transcription should land before implementation starts.

## Completion Status

- `completed-and-archived`
- Updated: `2026-03-28 17:35` — the architecture intent is preserved as historical design input for the completed Rust migration bundle.