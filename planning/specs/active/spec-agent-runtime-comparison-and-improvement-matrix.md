# Agent Runtime Comparison And Improvement Matrix Spec

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: define a comparison matrix between the repository's current agent runtime capabilities and a stronger target architecture across ten high-value topics, then map each topic to an owned improvement workstream.
- Normalized Request: compare the repository topic-by-topic against a richer agent runtime reference model and create explicit improvement planning for every relevant gap.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-agent-runtime-comparison-and-improvement-matrix.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository now has multiple active workstreams related to startup, MCP, RAG/CAG, token economy, runtime control, and multi-agent behavior. Without a comparison matrix, it is difficult to see which topics are already covered, which are only partially addressed, and which still need dedicated workstreams.

---

## Design Intent

- Compare the repository against ten concrete runtime topics:
  - CLI fast-path startup
  - MCP robustness
  - memory layering
  - context reranking
  - token and cost tracking
  - task output spill-to-disk
  - typed control schemas
  - internal multi-agent runtime
  - plugin and skill extensibility
  - operational diagnostics
- Classify each topic as strong, partial, or weak in the current repository state.
- Map every topic to a single owner workstream so follow-up work stays explicit.
- Avoid duplicating workstreams that already exist.

---

## Options Considered

1. Track each topic only informally in chat or commit history.
   - Rejected: too easy to lose the relationship between topics and workstreams.
2. Open independent plans with no comparison layer.
   - Rejected: this makes prioritization and overlap harder to reason about.
3. Keep topic-specific plans but add a matrix plan/spec that maps current state to owned workstreams.
   - Preferred: preserves focused plans while making the overall architecture easier to manage.

---

## Acceptance Criteria

- All ten topics are compared explicitly against the current repository state.
- Each topic has a state classification and an owned improvement workstream.
- Missing areas receive new dedicated workstreams instead of being folded into unrelated plans.
- The matrix is kept in active planning until the repository reaches the desired baseline.

---

## Planning Readiness

- Ready for planning immediately.
- The first plan slice should freeze the ten-topic matrix and classify the repository state.
- Subsequent slices should only open new workstreams where there is no existing owner.