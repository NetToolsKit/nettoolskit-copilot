# SDK Control Plane Schemas And Runtime Introspection Spec

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: define typed control-plane schemas and runtime introspection surfaces for models, permissions, MCP status, and context usage.
- Normalized Request: create a workstream for stronger SDK/control schemas and runtime inspection so tool callers and operators can reason about the system through stable contracts.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-sdk-control-plane-schemas-and-runtime-introspection.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already exposes command and runtime surfaces, but several control-plane concepts are still primarily internal. Typed request/response schemas and inspection endpoints can improve tooling, SDK integration, diagnostics, and future interoperability without binding the system to a single frontend.

---

## Design Intent

- Define typed control schemas for runtime/session operations.
- Expose context-usage, permission-mode, model-state, and MCP-status introspection through stable contracts.
- Keep schema ownership explicit and reusable across CLI, service, and future SDK consumers.
- Avoid making raw internal state the public control contract.

---

## Options Considered

1. Keep runtime introspection as ad-hoc command output.
   - Rejected: hard to reuse and harder to validate.
2. Expose internal structs directly.
   - Rejected: over-couples control consumers to internal runtime evolution.
3. Create typed control schemas and explicit runtime introspection surfaces.
   - Preferred: improves reuse, diagnostics, and future interoperability.

---

## Proposed Boundaries

- Control schemas live in a dedicated boundary.
- Runtime inspection services produce normalized status objects.
- CLI and future SDK/service surfaces consume these contracts.
- Schema evolution is versioned and tested.
- Control-plane contracts remain separate from user-facing rendering.

---

## Acceptance Criteria

- The repository has stable typed schemas for key runtime/control operations.
- MCP status, context usage, permission mode, and model state can be inspected programmatically.
- CLI rendering becomes an adapter over typed introspection, not the source of truth.
- Future SDK or service consumers can reuse the same contracts.

---

## Planning Readiness

- Ready for planning now.
- The first slice should inventory existing inspection-capable surfaces and normalize a small contract set.
- Validation should prove schema stability and renderer independence.