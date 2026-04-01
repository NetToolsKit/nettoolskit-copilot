# Agentic Surface Boundary Separation Plan

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 11:06
- Objective: separate MCP, A2A, RAG, and CAG into explicit repository boundaries so the agentic stack is easier to maintain, test, and extend without accidental coupling.
- Normalized Request: create a planning workstream to improve the current agentic-code boundaries and keep each technology in the right layer.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-agentic-surface-boundary-separation.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Current Slice: local context assembly and token-economy policy have been extracted from `processor.rs` into dedicated execution boundaries.
- Inputs:
  - `README.md`
  - `.github/instructions/readme.instructions.md`
  - `.github/instructions/nettoolskit-rules.instructions.md`
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/orchestrator/src/execution/ai_session.rs`
  - `crates/core/src/ai_context.rs`
  - `crates/core/src/local-context/*`
  - `crates/commands/runtime/src/sync/bootstrap.rs`
  - `crates/commands/runtime/src/sync/provider_surfaces.rs`
  - `crates/commands/runtime/src/sync/mcp_config.rs`
  - `crates/commands/runtime/src/sync/mcp_runtime_artifacts.rs`

---

## Scope Summary

This workstream coordinates four related boundary cleanups:

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| A1 | Boundary map and coupling audit | processor + runtime sync + local context modules | 🔴 Immediate | none |
| A2 | Extract CAG composition from orchestrator | processor + ai_session + ai_context | 🔴 Immediate | A1 |
| A3 | Tighten MCP/runtime projection boundaries | bootstrap + provider surfaces + MCP sync modules | 🟠 High | A1 |
| A4 | Reserve and document A2A as a separate surface | README + planning + instructions | 🟡 Medium | A1 |

This workstream focuses on code separation and maintenance clarity. It does not implement a full A2A protocol.

---

## Ordered Tasks

### [2026-03-31 00:00] Task A1: Freeze Agentic Boundary Map

- Inventory the current code paths that mix retrieval, prompt shaping, session replay, and runtime projection.
- Capture the coupling matrix for:
  - MCP
  - RAG
  - CAG
  - A2A
- Record the exact entry points that currently own multiple concerns.
- Commit checkpoint:
  - `docs(planning): freeze agentic boundary map`

### [2026-03-31 00:00] Task A2: Extract CAG Composition From Orchestrator

- Move context assembly and prompt policy composition behind a dedicated boundary so `processor.rs` stops owning the full assembly stack.
- Keep `crates/core/src/ai_context.rs` focused on context collection and rendering helpers.
- Keep `crates/orchestrator/src/execution/ai_session.rs` focused on session persistence and compression policy.
- Add or expand tests that prove the extraction does not change AI request behavior.
- Completed slice: `crates/orchestrator/src/execution/ai_request_context.rs` now owns local context assembly and session replay injection, and `crates/orchestrator/src/execution/ai_token_economy.rs` now owns prompt compaction/token policy.
- Target paths:
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/core/src/ai_context.rs`
  - `crates/orchestrator/src/execution/ai_session.rs`
  - `crates/orchestrator/tests/execution/*`
- Commit checkpoint:
  - `refactor(ai): separate context assembly from orchestration`

### [2026-03-31 00:00] Task A3: Tighten MCP Runtime Projection Boundaries

- Keep MCP config application and provider surface rendering in the runtime sync layer.
- Remove any accidental dependence on RAG/CAG helpers from MCP-related projection code.
- Ensure bootstrap remains an orchestrator for runtime assets, not a catch-all for unrelated context logic.
- Add tests that prove MCP projection stays isolated from local recall and token-policy surfaces.
- Target paths:
  - `crates/commands/runtime/src/sync/bootstrap.rs`
  - `crates/commands/runtime/src/sync/provider_surfaces.rs`
  - `crates/commands/runtime/src/sync/mcp_config.rs`
  - `crates/commands/runtime/src/sync/mcp_runtime_artifacts.rs`
  - `crates/commands/runtime/tests/sync/*`
- Commit checkpoint:
  - `refactor(runtime): isolate MCP runtime projection boundaries`

### [2026-03-31 00:00] Task A4: Reserve A2A As a Separate Future Surface

- Document A2A as a planned interoperability boundary rather than an implied part of MCP or RAG/CAG.
- Keep A2A out of the current local context and runtime projection modules unless a dedicated design lands.
- Update the README and instruction guidance so future work does not drift A2A concerns into unrelated boundaries.
- Target paths:
  - `README.md`
  - `.github/instructions/readme.instructions.md`
  - `.github/instructions/nettoolskit-rules.instructions.md`
  - `planning/active/plan-agentic-surface-boundary-separation.md`
  - `planning/specs/active/spec-agentic-surface-boundary-separation.md`
- Commit checkpoint:
  - `docs(agentic): reserve A2A as a separate boundary`

---

## Validation Checklist

- `cargo test -p nettoolskit-core --quiet`
- `cargo test -p nettoolskit-orchestrator --test test_suite ai_usage --quiet`
- `cargo test -p nettoolskit-runtime --test test_suite sync --quiet`
- `cargo test -p nettoolskit-cli --test runtime_commands_tests --quiet`
- `cargo test -p nettoolskit-cli --test validation_commands_tests --quiet`
- `git diff --check`

---

## Risks And Mitigations

- Splitting the orchestration stack too aggressively can create thin wrappers with duplicated glue.
- Moving context assembly out of `processor.rs` can break request construction if tests do not cover the same intent paths.
- MCP projection changes can ripple into bootstrap and provider-surface renderers if the boundary matrix is incomplete.
- A2A can be over-designed before there is a real protocol consumer.
- Mitigation: keep slices narrow, preserve current behavior with tests, and treat A2A as reserved until there is a concrete interoperability requirement.

---

## Specialist And Closeout

- Recommended specialist: `dev-rust-engineer`
- Tester: required
- Reviewer: required
- Release closeout: required
- README update: required for the boundary map and A2A status
- Changelog: required once implementation lands
- Suggested commit message style:
  - `refactor(ai): separate agentic surface boundaries`
  - `docs(planning): record agentic boundary separation roadmap`