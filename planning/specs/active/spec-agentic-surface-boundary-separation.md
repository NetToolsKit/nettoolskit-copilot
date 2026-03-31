# Agentic Surface Boundary Separation Spec

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: define the design intent for separating MCP, A2A, RAG, and CAG responsibilities into explicit repository boundaries that are easier to maintain and reason about.
- Normalized Request: open a planning workstream to improve the current agentic-code boundaries so the repository keeps MCP, A2A, RAG, and CAG distinct and SOLID-aligned.
- Active Branch: `docs/planning-gap-workstreams` (planning only; implementation branches TBD)
- Planning Path: `planning/active/plan-agentic-surface-boundary-separation.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already documents MCP, A2A, RAG, and CAG as distinct concepts, but the current code structure still composes several of them in shared execution flows. That makes maintenance harder, broadens the impact of future changes, and forces unrelated concerns to move together.

---

## Design Intent

- Keep MCP as the tool/projection boundary for runtime and editor surfaces.
- Keep RAG as deterministic local recall and indexing.
- Keep CAG as prompt shaping, compaction, and token-budget policy.
- Reserve A2A as a separate interoperability boundary without coupling it to local recall or MCP projection.
- Reduce cross-cutting logic inside orchestrator and runtime bootstrap paths.
- Improve code clarity by aligning boundaries with single responsibility and dependency inversion.

---

## Options Considered

1. Leave the current composition points in `processor.rs` and `bootstrap.rs`.
   - Rejected: responsibilities remain mixed and future changes stay harder to isolate.
2. Only document the boundaries in README files.
   - Rejected: documentation alone does not reduce coupling in the implementation.
3. Introduce explicit service boundaries and keep each agentic technology focused on one responsibility.
   - Preferred: this supports SOLID and keeps the repository easier to evolve.

---

## Proposed Boundaries

- `MCP` remains in `crates/commands/runtime/src/sync/*` and related runtime projections.
- `RAG` remains in `crates/core/src/local-context/*` and owns deterministic retrieval/indexing.
- `CAG` is isolated from raw retrieval and should own context assembly, compaction, and budget policy.
- `A2A` remains a reserved future boundary and should not leak into current runtime or retrieval internals.

---

## Acceptance Criteria

- The repository has an explicit boundary map for MCP, A2A, RAG, and CAG.
- Cross-cutting AI context assembly is reduced in the orchestrator pipeline.
- Runtime bootstrap stays focused on runtime projection and does not absorb unrelated recall or compaction logic.
- Local context retrieval remains deterministic and testable without prompt policy concerns.
- A2A stays documented as a future interoperability surface instead of becoming an implicit dependency.

---

## Planning Readiness

- The design is ready for planning once the current coupling points in `processor.rs`, `ai_context.rs`, `ai_session.rs`, `bootstrap.rs`, and `provider_surfaces.rs` are mapped into a small boundary matrix.
- The implementation work should proceed in small slices so each boundary change can be validated independently.
- The next execution plan should start with the smallest safe extraction that removes the most cross-cutting logic from `processor.rs`.