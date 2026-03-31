# Agent Runtime Comparison And Improvement Matrix Plan

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: maintain a topic-by-topic improvement matrix that compares the repository against a stronger agent runtime model and links every gap to an owned workstream.
- Normalized Request: compare the repository one-by-one across the ten most relevant runtime topics and create explicit planning coverage for the gaps.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-agent-runtime-comparison-and-improvement-matrix.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Topic Matrix

| Topic | Current Repository State | Rating | Improvement Workstream |
|---|---|---|---|
| CLI fast-path startup | `ntk` is modular but startup decomposition is not yet explicit enough | Partial | `plan-cli-fast-path-and-startup-decomposition.md` |
| MCP robustness | runtime projection exists but transport/auth/session lifecycle still needs hardening | Partial | `plan-mcp-transport-auth-and-session-resilience.md` |
| Memory layering | local context exists, but layered operational memory is not fully defined | Partial | `plan-memory-file-layering-and-operational-recall.md` |
| Context reranking | deterministic recall exists, but cheap second-pass selection is still planned | Weak | `plan-context-selection-reranking-and-budgeted-recall.md` |
| Token and cost tracking | weekly usage ledger and token policy exist, but richer tuning is still needed | Partial | `plan-token-economy-optimization.md` |
| Task output spill-to-disk | large-output lifecycle is not yet formalized | Weak | `plan-task-output-spill-to-disk-and-retention-control.md` |
| Typed control schemas | runtime inspection is still more internal than contract-driven | Weak | `plan-sdk-control-plane-schemas-and-runtime-introspection.md` |
| Internal multi-agent runtime | delegation exists, but lineage/inheritance/cleanup need a stronger contract | Partial | `plan-multi-agent-runtime-lineage-and-a2a-readiness.md` |
| Plugin and skill extensibility | skills exist, but extension contracts and governance are still underspecified | Partial | `plan-plugin-skill-extensibility-and-runtime-governance.md` |
| Operational diagnostics | diagnostics exist, but runtime/operator visibility is not yet unified | Partial | `plan-runtime-operational-diagnostics-and-observability.md` |

---

## Ordered Tasks

### [2026-03-31 00:00] Task X1: Freeze The Ten-Topic Comparison Matrix

- Record the repository state across all ten topics.
- Classify each topic as strong, partial, or weak.
- Confirm that every topic has an owned workstream.
- Commit checkpoint:
  - `docs(planning): freeze agent runtime comparison matrix`

### [2026-03-31 00:00] Task X2: Open Missing Owner Workstreams

- Create dedicated plans/specs for topics that do not yet have a proper owner.
- Avoid merging unrelated gaps into existing plans just to reduce file count.
- Commit checkpoint:
  - `docs(planning): open missing runtime improvement workstreams`

### [2026-03-31 00:00] Task X3: Keep The Matrix In Sync

- Update the matrix whenever a topic changes classification or ownership.
- Move matrix references to completed planning only when the underlying workstreams are materially closed.
- Commit checkpoint:
  - `docs(planning): update runtime comparison matrix progress`

---

## Validation Checklist

- `git diff --check`

---

## Specialist And Closeout

- Recommended specialist: `plan-active-work-planner`
- Reviewer: required
- README update: not required for this planning-only slice
- Suggested commit message style:
  - `docs(planning): add agent runtime comparison matrix`