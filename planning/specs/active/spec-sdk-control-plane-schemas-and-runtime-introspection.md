# SDK Control Plane Schemas And Runtime Introspection Spec

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-04-06 10:25
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
- Progress [2026-04-05 23:55]:
  - The first shared control-schema boundary now exists in `nettoolskit-core` for `ai_doctor` and `runtime_doctor`.
  - CLI JSON inspection for both doctor surfaces now goes through shared control schemas instead of crate-local structs.
  - The versioning and schema-kind catalog now lives under `definitions/templates/manifests/control-plane-introspection.catalog.json` with matching architecture guidance in `docs/architecture/control-plane-introspection-model.md`.
- Progress [2026-04-06 02:10]:
  - The shared control-plane boundary now also covers `runtime_healthcheck`, so typed runtime inspection is no longer limited to doctor surfaces.
  - `ntk runtime healthcheck --json-output` now emits the shared schema while preserving the persisted operator report/log workflow.
  - The canonical control-plane catalog and samples now register `runtime_healthcheck` as a first-class schema kind and entry point.
- Progress [2026-04-06 02:35]:
  - The shared control-plane boundary now also covers `runtime_self_heal`, so machine-readable runtime recovery state is aligned with the same typed family as doctor and healthcheck.
  - `ntk runtime self-heal --json-output` now emits the shared schema while preserving the persisted repair report/log workflow.
  - The canonical control-plane catalog and samples now register `runtime_self_heal` as a first-class schema kind and entry point.
- Progress [2026-04-06 09:55]:
  - The shared control-plane boundary now also covers `local_context_query` and `local_memory_query`, so repository recall no longer relies on ad-hoc JSON when consumed programmatically.
  - `ntk runtime query-local-context-index --json-output` and `ntk runtime query-local-memory --json-output` now emit the shared schemas while preserving the existing human-readable operator flow.
  - The canonical control-plane catalog and samples now register both local recall surfaces as first-class schema kinds and entry points.
- Progress [2026-04-06 10:25]:
  - The shared control-plane boundary now also covers `ai_provider_profiles` and `ai_provider_profile`, so AI preset inventory no longer relies on direct orchestrator struct serialization when consumed programmatically.
  - `ntk ai profiles list --json-output` and `ntk ai profiles show --json-output` now emit the shared schemas while preserving the existing human-readable operator flow.
  - The canonical control-plane catalog and samples now register both AI profile surfaces as first-class schema kinds and entry points.

---

## Planning Readiness

- Ready for planning now.
- The first slice should inventory existing inspection-capable surfaces and normalize a small contract set.
- Validation should prove schema stability and renderer independence.