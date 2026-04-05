# Memory File Layering And Operational Recall Spec

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-04-05 23:45
- Objective: define a layered local-memory model that distinguishes concise always-loaded memory, topic files, and operational logs without collapsing them into one generic context store.
- Normalized Request: create a workstream for file-based operational memory layering that complements the repository's RAG/CAG evolution and improves persistent recall discipline.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-memory-file-layering-and-operational-recall.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already has local context, planning, and instruction systems, but it does not yet define a layered operational memory model that clearly separates always-loaded operator memory, topic-specific memory, and append-only operational notes. Without that separation, future local-memory work risks mixing permanent memory, transient plans, and execution artifacts.

---

## Design Intent

- Introduce a concise always-loaded memory entrypoint.
- Keep topic memory in separate files instead of inflating the entrypoint.
- Keep append-only operational notes distinct from curated memory.
- Define how memory interacts with plans, tasks, and local context indexes.
- Ensure memory layering complements SQLite recall rather than competing with it.

---

## Options Considered

1. Store all persistent local memory only in SQLite.
   - Rejected: operational editing and manual curation become less transparent.
2. Keep all memory in one markdown file.
   - Rejected: the file becomes noisy and less useful as a stable always-loaded source.
3. Use a layered memory model with a concise entrypoint plus topic and log files.
   - Preferred: balances operational readability, curation, and machine retrieval.

---

## Proposed Boundaries

- A concise memory entrypoint is always loadable.
- Topic memory files provide curated detail.
- Operational notes remain append-only and can later be distilled.
- Plans and tasks stay under `planning/` rather than becoming memory files.
- SQLite/local-context indexing can read from these files but does not own their editorial model.

---

## Acceptance Criteria

- The repository has a documented layered memory model.
- Memory entrypoint size/shape rules are explicit.
- Topic memory and operational logs are separated.
- Plans are not repurposed as local memory.
- Future RAG/CAG work can index memory files without redefining their editorial role.

---

## Planning Readiness

- Ready for planning now.
- Initial implementation should freeze the editorial and directory model before wiring retrieval.
- Validation should cover both file shape rules and the relationship to the local-context index.
- The repository now has a canonical operational-memory layering catalog plus README and architecture guidance that separate curated file memory from planning artifacts and from SQLite-backed recall ownership.