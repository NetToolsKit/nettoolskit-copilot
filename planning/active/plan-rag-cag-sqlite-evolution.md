# RAG/CAG SQLite Evolution Plan

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-03-30 07:31
- Objective: evolve the current local-context and AI continuity layer into a richer SQLite-backed RAG/CAG memory system that keeps the repository-local recall deterministic and searchable.
- Normalized Request: create a planning workstream for the local SQLite memory system so it can grow beyond the current JSON baseline and support better session recall.
- Active Branch: `main` (planning only; implementation branches TBD)
- Spec Path: `planning/specs/active/spec-rag-cag-sqlite-evolution.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Inputs:
  - `crates/core/src/local-context/catalog.rs`
  - `crates/core/src/local-context/document.rs`
  - `crates/core/src/local-context/search.rs`
  - `crates/core/src/local-context/sqlite.rs`
  - `crates/commands/runtime/src/continuity/local_context.rs`
  - `crates/commands/runtime/src/diagnostics/enterprise_trends.rs`
  - `planning/completed/plan-ai-usage-history-and-sqlite-local-memory.md`
  - `planning/completed/spec-ai-usage-history-and-sqlite-local-memory.md`

---

## Scope Summary

This plan coordinates four memory-system slices:

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| R1 | SQLite schema growth and compatibility | core local-context store | 🔴 Immediate | existing local-context SQLite baseline |
| R2 | Query quality and retrieval ranking | core search + runtime continuity | 🔴 Immediate | R1 |
| R3 | Dual-write / cutover policy from JSON to SQLite | runtime continuity + CLI surfaces | 🟠 High | R1, R2 |
| R4 | Memory pruning, retention, and session recovery | core + orchestrator | 🟠 High | R1 |

---

## Ordered Tasks

### [2026-03-30 07:31] Task R1: Capture The Current Local Memory Baseline

- Audit the existing JSON and SQLite local-context paths.
- Record the current index, file, document, and chunk semantics.
- Capture the compatibility behavior that must remain stable during cutover.
- Commit checkpoint:
  - `docs(planning): freeze sqlite local memory baseline`

### [2026-03-30 07:31] Task R2: Define Query Quality And Ranking Targets

- Decide how far the SQLite path should go beyond the current lexical/default search.
- Define the ranking and tie-break behavior for memory recall.
- Add tests around deterministic retrieval and repo-root filtering.
- Commit checkpoint:
  - `docs(planning): define sqlite memory retrieval contract`

### [2026-03-30 07:31] Task R3: Plan Dual-Write And Cutover

- Define a migration path that keeps JSON compatibility while SQLite grows.
- Keep the current memory path usable until SQLite proves stable in practice.
- Document the cutover point for any future JSON removal.
- Commit checkpoint:
  - `docs(planning): plan json to sqlite local memory cutover`

### [2026-03-30 07:31] Task R4: Add Retention And Pruning Rules

- Define pruning policy for stale memory entries and expired session state.
- Ensure memory cleanup does not remove active context too aggressively.
- Validate size and retention behavior with deterministic tests.
- Commit checkpoint:
  - `docs(planning): define local memory retention and pruning policy`

---

## Validation Checklist

- `cargo test -p nettoolskit-core --test local_context --quiet`
- `cargo test -p nettoolskit-runtime --test test_suite continuity --quiet`
- `cargo test -p nettoolskit-cli --test runtime_commands_tests --quiet`
- `cargo clippy -p nettoolskit-core --all-targets -- -D warnings`
- `git diff --check`

---

## Risks And Mitigations

- Retrieval quality may regress if the SQLite path changes semantics too quickly.
- Dual-write cutovers can create state drift if pruning is too aggressive.
- Session recovery can leak stale memory into new work unless the retention rules are strict.
- Mitigation: keep JSON compatibility until SQLite retrieval and pruning are validated on real flows.

---

## Specialist And Closeout

- Recommended specialist: `dev-rust-engineer`
- Tester: required
- Reviewer: required
- Release closeout: required
- README update: likely needed for local-memory behavior and operator commands
- Changelog: required once implementation lands
- Suggested commit message style:
  - `feat(memory): evolve local context to sqlite-backed recall`
  - `docs(planning): record sqlite rag/cag evolution roadmap`