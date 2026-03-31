# Context Selection Reranking And Budgeted Recall Spec

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: define a second-pass context selection model that can cheaply rerank retrieved artifacts before they are added to the primary prompt budget.
- Normalized Request: create a workstream for budget-aware context reranking so the repository gains better recall quality and lower token waste without abandoning deterministic retrieval.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-context-selection-reranking-and-budgeted-recall.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

Deterministic retrieval is necessary but not sufficient for efficient context packing. A lexical or index-driven first pass can still return too many candidates, and pushing them directly into the main prompt wastes tokens. The repository needs a design for cheap reranking, selection, and budget-aware admission before the primary generation path runs.

---

## Design Intent

- Preserve deterministic first-pass retrieval.
- Add an optional cheap second-pass selector/reranker.
- Make selection budget-aware and observable.
- Keep context selection separate from the main generation path.
- Support both file-based memory and SQLite/local-context candidates.

---

## Options Considered

1. Keep only lexical/index-based recall and static caps.
   - Rejected: relevant candidates can still be noisy and token-heavy.
2. Move entirely to semantic/vector recall.
   - Rejected: deterministic repo recall should remain available and testable.
3. Keep deterministic retrieval but add a cheap selection/reranking phase.
   - Preferred: better token efficiency without losing control.

---

## Proposed Boundaries

- First pass: deterministic retrieval/indexing.
- Second pass: candidate selection/reranking.
- Third pass: budgeted admission to the final prompt/context pack.
- Token economy records what was selected, dropped, and why.
- Main generation/orchestration should consume the final context pack, not own selection policy.

---

## Acceptance Criteria

- The repository has an explicit design for two-pass recall.
- Budgeted admission rules are defined and observable.
- Context selection can work across local context, memory files, and future external artifacts.
- Token economy and retrieval stay linked but not conflated.
- Orchestrator code no longer owns the full selection stack implicitly.

---

## Planning Readiness

- Ready for planning now.
- The first slice should define candidate classes, ranking signals, and decision logging.
- Implementation should be staged behind metrics and fallback-safe behavior.