# Runtime Operational Diagnostics And Observability Plan

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-04-05 22:40
- Objective: unify operator-facing diagnostics and observability concepts across runtime, MCP, AI, task execution, and service surfaces.
- Normalized Request: create a detailed workstream for runtime diagnostics and observability because the repository needs stronger operational visibility across its growing agent runtime.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/completed/spec-runtime-operational-diagnostics-and-observability.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Current Slice: D1, D4, and D5 are now materially implemented through the canonical runtime-diagnostics taxonomy manifest, the operator playbook, and README/instruction updates; together with the existing `ntk ai doctor` proof for D2-D3, the workstream is materially complete and ready for archive.
- Related Workstreams:
  - `planning/active/plan-sdk-control-plane-schemas-and-runtime-introspection.md`
  - `planning/active/plan-mcp-transport-auth-and-session-resilience.md`
  - `planning/active/plan-task-output-spill-to-disk-and-retention-control.md`
  - `planning/active/plan-token-economy-optimization.md`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| D1 | Diagnostics taxonomy and ownership | health/degraded/runtime status map | 🔴 Immediate | none |
| D2 | Normalized runtime health surfaces | runtime + MCP + task + AI status | 🔴 Immediate | D1 |
| D3 | Operator inspection commands and reports | CLI and future service adapters | 🟠 High | D1, D2 |
| D4 | Degraded-state runbooks and telemetry | observability + troubleshooting | 🟠 High | D2, D3 |
| D5 | Docs and governance | README + operator guidance | 🟡 Medium | D1-D4 |

---

## Ordered Tasks

### [2026-03-31 00:00] Task D1: Freeze The Diagnostics Taxonomy

- Define the operator-visible classes:
  - healthy
  - degraded
  - blocked
  - misconfigured
  - recovering
- Define the subsystems that must surface diagnostics:
  - MCP
  - AI/token economy
  - local recall
  - task execution
  - service/runtime state
- Implemented slice:
  - `definitions/templates/manifests/runtime-diagnostics.taxonomy.json` now freezes the canonical health states, subsystem ownership, inspection surfaces, and degraded-state evidence requirements in one authored manifest.
- Commit checkpoint:
  - `docs(planning): freeze runtime diagnostics taxonomy`

### [2026-03-31 00:00] Task D2: Define Normalized Runtime Health Surfaces

- Define normalized runtime inspection objects for each subsystem.
- Keep raw traces and logs behind diagnostics rather than making them the contract.
- Align diagnostics with typed control schemas where possible.
- Implemented slice:
  - `AiDoctorResult`, `AiDoctorStatus`, and `AiDoctorModelSelection` provide a normalized health surface for AI profile, provider chain, timeout, auth readiness, and fallback state.
- Commit checkpoint:
  - `docs(planning): define normalized runtime health surfaces`

### [2026-03-31 00:00] Task D3: Design Operator Inspection Commands And Reports

- Define which CLI/service surfaces expose diagnostics and in what shape.
- Keep diagnostic outputs machine-readable where appropriate.
- Define concise and detailed inspection modes.
- Implemented slice:
  - `ntk ai doctor` exposes concise text output.
  - `ntk ai doctor --json-output` exposes machine-readable inspection data.
  - `ntk ai doctor --report-path` emits a Markdown operator report without mutating runtime state.
- Commit checkpoint:
  - `docs(planning): define operator inspection surfaces`

### [2026-03-31 00:00] Task D4: Add Degraded-State Runbook And Telemetry Requirements

- Define what must be logged, counted, or surfaced when the runtime degrades.
- Connect diagnostics to remediation guidance and troubleshooting flows.
- Keep observability and validation related but distinct.
- Implemented slice:
  - `docs/operations/runtime-diagnostics-observability-playbook.md` now explains the canonical health states, subsystem map, operator workflow, and degraded-state telemetry expectations.
  - `docs/samples/manifests/runtime-diagnostics.taxonomy.sample.json` now provides a human-facing sample while authored truth remains under `definitions/templates/manifests/`.
- Commit checkpoint:
  - `docs(planning): define degraded-state diagnostics and telemetry requirements`

### [2026-03-31 00:00] Task D5: Document The Operational Diagnostics Model

- Update README and runtime/operator docs.
- Add governance rules so new runtime features expose diagnostics consistently.
- Implemented slice:
  - `README.md`, `docs/README.md`, and `definitions/templates/*` now point operators and contributors to the canonical runtime diagnostics taxonomy and playbook.
  - `definitions/instructions/operations/ntk-operations-observability-sre.instructions.md` and `definitions/instructions/governance/ntk-governance-repository-readme-overrides.instructions.md` now require doctor/report surfaces to align with the shared taxonomy and runbook model.
- Commit checkpoint:
  - `docs(runtime): document operational diagnostics model`

---

## Validation Checklist

- `git diff --check`

---

## Specialist And Closeout

- Recommended specialist: `obs-sre-observability-engineer`
- Reviewer: required
- README update: required
- Suggested commit message style:
  - `docs(planning): add runtime diagnostics and observability roadmap`