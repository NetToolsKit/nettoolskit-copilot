# RAG/CAG SQLite Evolution Spec

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-03-30 07:31
- Objective: define the architecture for evolving the repository-local recall layer into a richer SQLite-backed memory system while keeping the current JSON baseline available during the transition.
- Normalized Request: plan a local SQLite RAG/CAG system similar in spirit to `context-mode`, but shaped for this repository's deterministic operator and agent workflows.
- Active Branch: `main` (planning only; implementation branches TBD)
- Planning Path: `planning/active/plan-rag-cag-sqlite-evolution.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already has a local recall index and SQLite support, but the current behavior is still mostly a compatibility baseline. It is good enough for deterministic recall, yet it does not fully model session memory, richer query ranking, or a clean migration path from JSON-only artifacts.

---

## Design Intent

- Keep memory local to the repository and user profile boundaries.
- Preserve deterministic recall and stable tie-breaking.
- Favor incremental cutover over a hard switch.
- Keep the JSON path as a compatibility fallback until SQLite proves stable.

---

## Options Considered

1. Keep JSON-only memory and improve search heuristics.
   - Rejected: it does not scale well as a memory system.
2. Replace JSON immediately with SQLite.
   - Rejected: risky without dual-write and validation evidence.
3. Dual-write into SQLite while preserving JSON compatibility.
   - Preferred: safest path for measured migration.

---

## Proposed Boundaries

- `crates/core/src/local-context/sqlite.rs` owns schema/bootstrap/query primitives.
- `crates/core/src/local-context/search.rs` owns ranking and retrieval semantics.
- `crates/commands/runtime/src/continuity/local_context.rs` owns CLI-facing continuity behavior.
- Orchestrator/session code owns write-time ingestion and replay/retention policy.

---

## Acceptance Criteria

- SQLite can serve the default recall path without changing the user-facing contract.
- JSON remains available as a fallback until the migration is explicitly closed.
- Retrieval is deterministic across repeated runs.
- Pruning and retention behavior are explicit and tested.
- Operator commands remain English and `ntk`-native.

---

## Planning Readiness

- The spec is planning-ready once the retention and query targets are written into the active plan.
- A separate implementation branch should be used because the memory stack touches core, runtime, and orchestrator code.