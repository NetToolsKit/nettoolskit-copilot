# Context Selection Reranking And Budgeted Recall Plan

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: introduce a budget-aware context selection layer that reranks recall candidates before they enter the main prompt path.
- Normalized Request: create a detailed workstream for cheap reranking and budgeted context admission to improve recall quality and reduce token waste.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-context-selection-reranking-and-budgeted-recall.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Related Workstreams:
  - `planning/active/plan-token-economy-optimization.md`
  - `planning/active/plan-rag-cag-sqlite-evolution.md`
  - `planning/completed/plan-memory-file-layering-and-operational-recall.md`
- Inputs:
  - `crates/core/src/local-context/*`
  - `crates/orchestrator/src/execution/ai_request_context.rs`
  - `crates/orchestrator/src/execution/ai_token_economy.rs`
  - `crates/orchestrator/src/execution/processor.rs`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| C1 | Candidate class and ranking map | local-context + memory + future artifacts | 🔴 Immediate | none |
| C2 | Cheap selector boundary | reranking service / sidecar | 🔴 Immediate | C1 |
| C3 | Budget-aware admission policy | token economy integration | 🟠 High | C1, C2 |
| C4 | Decision logging and ledgering | why selected vs dropped | 🟠 High | C2, C3 |
| C5 | Docs and tuning guidance | README + instruction surfaces | 🟡 Medium | C3, C4 |

---

## Ordered Tasks

### [2026-03-31 00:00] Task C1: Freeze Candidate Classes And Ranking Signals

- Define the candidate sources:
  - local-context files
  - memory files
  - plan/spec artifacts
  - future external/tool artifacts
- Define ranking signals and their relative priority.
- Record hard exclusions and always-include classes.
- Commit checkpoint:
  - `docs(planning): freeze context candidate classes and ranking signals`

### [2026-03-31 00:00] Task C2: Introduce A Cheap Selection Boundary

- Define the reranking/selection service contract.
- Keep this boundary independent from the primary response-generation model.
- Provide deterministic fallback if the selector is unavailable.
- Commit checkpoint:
  - `refactor(ai): add cheap context selection boundary`

### [2026-03-31 00:00] Task C3: Add Budget-Aware Admission Policy

- Connect reranked candidates to context budget and compaction policy.
- Record explicit admission rules:
  - always include
  - include if budget permits
  - trim
  - defer
  - reject
- Commit checkpoint:
  - `feat(ai): add budget-aware context admission policy`

### [2026-03-31 00:00] Task C4: Add Selection And Drop Reason Logging

- Record why candidates were selected or dropped.
- Expose budget and selection breakdown in telemetry or CLI diagnostics.
- Keep logs useful for tuning and regression tests.
- Commit checkpoint:
  - `feat(ai): log context selection and drop reasons`

### [2026-03-31 00:00] Task C5: Document The Recall And Budget Model

- Update README and instructions for two-pass recall.
- Describe how deterministic recall, selection, and budget interact.
- Record tuning guidance for operators and developers.
- Commit checkpoint:
  - `docs(ai): document budgeted recall and reranking model`

---

## Validation Checklist

- `cargo test -p nettoolskit-core --quiet`
- `cargo test -p nettoolskit-orchestrator --quiet`
- `cargo test -p nettoolskit-cli --test test_suite ai_commands_tests --quiet`
- `git diff --check`

---

## Risks And Mitigations

- Reranking can make context selection opaque if it lacks decision logging.
- Budget policy can become duplicated between selection and compaction layers.
- Cheap selectors can still introduce instability if no deterministic fallback exists.
- Mitigation: define decision logging and fallback behavior in the first design slice.

---

## Specialist And Closeout

- Recommended specialist: `dev-rust-engineer`
- Tester: required
- Reviewer: required
- README update: required
- Suggested commit message style:
  - `feat(ai): add budgeted recall selection boundary`
  - `docs(planning): record context selection and reranking roadmap`