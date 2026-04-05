# Memory File Layering And Operational Recall Plan

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-04-05 23:45
- Objective: add a layered operational-memory model that distinguishes concise memory entrypoints, topic memory, and append-only operational logs while staying aligned with SQLite-based recall work.
- Normalized Request: create a detailed workstream for layered local memory and operational recall discipline in the repository.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-memory-file-layering-and-operational-recall.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Related Workstreams:
  - `planning/active/plan-rag-cag-sqlite-evolution.md`
  - `planning/active/plan-token-economy-optimization.md`
  - `planning/active/plan-agentic-surface-boundary-separation.md`
- Current Slice: R1 through R5 are now materially implemented through the canonical operational-memory layering catalog, architecture/sample docs, README coverage, and governance rules that keep file memory separate from planning and RAG/CAG boundaries; the workstream is ready for archive.
- Inputs:
  - `crates/core/src/local-context/*`
  - `planning/*`
  - `.github/instructions/*`
  - `README.md`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| R1 | Memory editorial model | entrypoint/topic/log rules | 🔴 Immediate | none |
| R2 | Memory directory and indexing boundary | local-context + file ownership | 🔴 Immediate | R1 |
| R3 | Operational note distillation flow | append-only log -> curated memory | 🟠 High | R1 |
| R4 | Retrieval alignment | memory files + SQLite recall | 🟠 High | R2 |
| R5 | Docs and instructions | README + repository guidance | 🟡 Medium | R1, R2 |

---

## Ordered Tasks

### [2026-03-31 00:00] Task R1: Freeze The Layered Memory Model

- Define the always-loaded memory entrypoint and its size expectations.
- Define topic memory files and when they should be created.
- Define append-only operational notes and what does not belong there.
- Record how memory differs from plans, tasks, and instructions.
- Implemented slice:
  - `definitions/templates/manifests/operational-memory-layering.catalog.json` now freezes the layered editorial model for entrypoint memory, topic memory, operational notes, distillation, and retrieval alignment.
- Commit checkpoint:
  - `docs(planning): freeze layered memory model`

### [2026-03-31 00:00] Task R2: Define Directory And Indexing Boundaries

- Choose the repository-local or user-local storage model for memory files.
- Define how local-context indexing discovers and classifies memory files.
- Keep editorial ownership separate from indexing ownership.
- Implemented slice:
  - the operational-memory catalog now records directory and ownership boundaries that keep file memory authored separately from `planning/` and from indexing ownership.
- Commit checkpoint:
  - `docs(planning): define memory directory and indexing boundary`

### [2026-03-31 00:00] Task R3: Add Operational Note Distillation Design

- Define how append-only notes can later be distilled into concise memory files.
- Keep the distillation flow explicit and reviewable.
- Avoid turning plans or task transcripts into raw memory by default.
- Implemented slice:
  - the operational-memory catalog and architecture doc now define the reviewable distillation flow from operational notes to topic memory to concise entrypoint memory.
- Commit checkpoint:
  - `docs(planning): define operational note distillation flow`

### [2026-03-31 00:00] Task R4: Align Memory Files With Recall

- Define how memory files participate in recall ranking and context packaging.
- Ensure memory entrypoints do not flood the always-loaded context budget.
- Connect this model to the SQLite RAG/CAG roadmap without merging the concerns.
- Implemented slice:
  - the canonical catalog, architecture doc, and agentic-surface instruction now define how file memory complements RAG/CAG without becoming the retrieval store or collapsing into planning.
- Commit checkpoint:
  - `docs(planning): align memory layering with repository recall`

### [2026-03-31 00:00] Task R5: Document The Memory Model

- Update README and instructions with the layered memory rules.
- Clarify what belongs in memory, plans, tasks, and instructions.
- Add examples that show the expected layout and operator workflow.
- Implemented slice:
  - `README.md`, `definitions/README.md`, `docs/README.md`, and `docs/samples/manifests/operational-memory-layering.catalog.sample.json` now document the operational-memory model and show the expected layer boundaries.
- Commit checkpoint:
  - `docs(memory): document layered operational recall model`

---

## Validation Checklist

- `cargo test -p nettoolskit-core --quiet`
- `cargo test -p nettoolskit-orchestrator --quiet`
- `git diff --check`

---

## Risks And Mitigations

- Memory files can become a second planning system if the editorial boundary is unclear.
- Append-only notes can become noisy if there is no distillation contract.
- Local indexing can over-prioritize memory files and inflate token usage.
- Mitigation: define editorial roles first and tie retrieval only after the roles are stable.

---

## Specialist And Closeout

- Recommended specialist: `docs-release-engineer`
- Implementation specialist: `dev-rust-engineer`
- Tester: required once indexing work starts
- README update: required
- Suggested commit message style:
  - `docs(planning): define layered memory and operational recall model`
  - `docs(memory): add repository local-memory guidance`