# Enterprise Rust Runtime Transcription Architecture

## Objective

Preserve the architectural intent captured in `.temp/arquitetura_enterprise_llm.md` as a versioned planning artifact and adapt it to the `nettoolskit-copilot` migration goal: transcribe the full PowerShell execution estate into a deterministic Rust runtime without losing enterprise-grade orchestration, structural intelligence, validation, and operator safety.

## Source Material

- Temporary reference: `.temp/arquitetura_enterprise_llm.md`
- Primary workstream: `planning/specs/active/spec-repository-unification-and-rust-migration.md`
- Active execution plan: `planning/active/plan-repository-unification-and-rust-migration.md`

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
3. The active migration spec and plan can reference this artifact without depending on `.temp`.
4. The architecture explicitly keeps LLM reasoning as a planning aid and Rust as the execution substrate.
5. The architecture keeps static repository surfaces authoritative and PowerShell wrappers temporary.

## Planning Readiness

- `ready-for-plan`