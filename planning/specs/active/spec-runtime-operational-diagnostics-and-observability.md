# Runtime Operational Diagnostics And Observability Spec

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-04-05 22:40
- Objective: define a coherent runtime diagnostics and observability model for operator-facing health, degradation, inspection, and troubleshooting.
- Normalized Request: open a dedicated workstream for operational diagnostics because current runtime visibility exists but is not yet unified enough across CLI, service, MCP, AI, and task execution surfaces.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-runtime-operational-diagnostics-and-observability.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already has health, runtime, validation, and telemetry concepts, but operator-facing diagnostics are still split across multiple paths. Without a unified design, degraded states become harder to inspect, runtime behavior becomes harder to tune, and the eventual service/operator surface is harder to support.

---

## Design Intent

- Define a coherent diagnostics model for runtime health and degraded states.
- Unify visibility across MCP, token economy, local recall, task execution, and service/runtime inspection.
- Distinguish operational diagnostics from validation and from internal debug traces.
- Support both CLI inspection and future service/operator consumers.

---

## Options Considered

1. Keep diagnostics fragmented across commands and logs.
   - Rejected: hard for operators and hard to evolve.
2. Push everything into raw observability/telemetry backends.
   - Rejected: operators also need local typed diagnostics and health surfaces.
3. Define a unified diagnostics and observability model with explicit runtime inspection boundaries.
   - Preferred: aligns with control schemas and improves supportability.

---

## Acceptance Criteria

- The repository has a documented runtime diagnostics model.
- Diagnostics cover health, degraded states, task output pressure, MCP state, and context/token surfaces.
- CLI and future service/operator consumers share the same normalized inspection concepts.
- Validation and diagnostics remain related but distinct concerns.

---

## Planning Readiness

- Ready for planning now.
- The first slice should freeze the diagnostics taxonomy and ownership map.
- Initial implementation proof now exists for the AI subsystem:
  - `ntk ai doctor` exposes a normalized read-only health surface
  - JSON output is available for machine consumers
  - Markdown report generation is available for operator troubleshooting
- The authored diagnostics taxonomy, degraded-state runbook, README guidance, and observability instructions now provide a canonical cross-subsystem model for future MCP, recall, task, and service doctor/report surfaces.