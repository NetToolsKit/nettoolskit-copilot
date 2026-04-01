# CLI Fast-Path And Startup Decomposition Spec

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: define a startup architecture where `ntk` can serve lightweight commands, daemon modes, and runtime sub-surfaces through explicit fast paths without loading the full interactive stack.
- Normalized Request: open a detailed planning workstream for modular CLI startup and fast-path decomposition so the repository gains better latency, clearer boundaries, and lower token/runtime overhead.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-cli-fast-path-and-startup-decomposition.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The current repository already exposes multiple `ntk` command surfaces, but startup ownership is still broad enough that lightweight runtime, validation, MCP, and AI-related commands risk paying for initialization that belongs to richer interactive paths. That increases latency, broadens dependency surfaces, and makes the CLI harder to evolve safely.

---

## Design Intent

- Introduce explicit startup classes for:
  - lightweight fast paths
  - service/daemon entrypoints
  - runtime sync and projection entrypoints
  - interactive command execution
- Keep heavy interactive dependencies out of lightweight `ntk` commands whenever possible.
- Make startup ownership explicit enough that tests can verify which surfaces require full initialization and which do not.
- Reduce incidental coupling between CLI parsing, bootstrapping, runtime sync, and agentic orchestration.

---

## Options Considered

1. Keep a single startup path and rely on conditional branches inside one entrypoint.
   - Rejected: this keeps initialization broad and makes future fast paths harder to reason about.
2. Split the binary into multiple standalone executables.
   - Rejected: that weakens discoverability and conflicts with the `ntk` single-entry UX.
3. Keep one binary but decompose startup into explicit fast-path and full-runtime boundaries.
   - Preferred: preserves UX while reducing cold-start cost and clarifying responsibilities.

---

## Proposed Boundaries

- `crates/cli` owns argument parsing and startup routing only.
- Lightweight fast paths should avoid loading interactive, orchestration, or runtime-sync modules unless required.
- `crates/orchestrator` should be loaded only for paths that need AI/task execution.
- Runtime projection and sync should be reachable through dedicated startup boundaries rather than implicit fallthrough.
- Service/daemon modes should have their own startup contracts and health probes.

---

## Acceptance Criteria

- The repository has a defined startup boundary map for `ntk`.
- Lightweight commands have a documented and testable fast-path route.
- Runtime, service, and interactive entrypoints can be reasoned about independently.
- Startup tests can prove that selected commands bypass heavyweight initialization.
- README and CLI docs reflect the decomposed startup model.

---

## Planning Readiness

- Ready for planning once the current `ntk` startup routes and module loads are mapped.
- Implementation should start with routing and ownership extraction before any performance tuning.
- Each slice should keep behavior stable and prove startup intent through tests or instrumentation.