# Multi-Agent Runtime Lineage And A2A Readiness Plan

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-04-05 14:05
- Objective: strengthen the repository's internal multi-agent orchestration model while keeping future A2A work explicitly separate and planned.
- Normalized Request: create a detailed workstream for lineage, delegation, permission inheritance, mailbox/bridge coordination, and A2A readiness in the agent runtime.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-multi-agent-runtime-lineage-and-a2a-readiness.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Related Workstreams:
  - `planning/active/plan-agentic-surface-boundary-separation.md`
  - `planning/active/plan-sdk-control-plane-schemas-and-runtime-introspection.md`
  - `planning/active/plan-token-economy-optimization.md`
- Current Slice: A22 now has a first implementation proof because canonical agent and skill lanes can declare inherited profile/model defaults under `definitions/agents/*` and `definitions/skills/*`, and the orchestrator consumes those defaults without conflating internal delegation with A2A.
- Inputs:
  - `crates/orchestrator/src/execution/*`
  - `README.md`
  - `.github/instructions/*`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| A21 | Internal delegation contract | lineage + roles + lifecycle | 🔴 Immediate | none |
| A22 | Permission and setting inheritance | model/permission/runtime propagation | 🔴 Immediate | A21 |
| A23 | Mailbox and approval bridge design | leader-worker coordination | 🟠 High | A21, A22 |
| A24 | Cleanup and retention model | spawned agents and artifacts | 🟠 High | A21 |
| A25 | A2A readiness boundary | future interoperability prep only | 🟡 Medium | A21, A23 |

---

## Ordered Tasks

### [2026-03-31 00:00] Task A21: Freeze Internal Multi-Agent Runtime Contract

- Define leader, worker, and delegated-role semantics.
- Define lifecycle states:
  - spawned
  - active
  - blocked
  - idle
  - completed
  - aborted
- Define lineage metadata and ownership expectations.
- Commit checkpoint:
  - `docs(planning): freeze multi-agent runtime contract`

### [2026-03-31 00:00] Task A22: Define Safe Inheritance Rules

- Define which settings propagate from leader to worker:
  - model
  - permission mode
  - token budget
  - plugin/runtime settings
  - local context scope
- Define which settings must not propagate automatically.
- Commit checkpoint:
  - `docs(planning): define multi-agent inheritance rules`

### [2026-03-31 00:00] Task A23: Design Mailbox And Approval Bridges

- Define asynchronous messaging and approval/permission escalation flows.
- Clarify when a mailbox is enough and when a direct bridge is needed.
- Keep the internal coordination model protocol-agnostic.
- Commit checkpoint:
  - `docs(planning): define mailbox and approval bridge model`

### [2026-03-31 00:00] Task A24: Define Cleanup And Retention

- Define cleanup rules for child sessions, artifacts, and temporary state.
- Define retention for useful lineage/audit metadata.
- Align spawned-agent cleanup with disk and runtime cleanup workstreams.
- Commit checkpoint:
  - `docs(planning): define spawned-agent cleanup and retention`

### [2026-03-31 00:00] Task A25: Reserve A2A Readiness Without Conflation

- Document what internal multi-agent runtime can expose later to an A2A layer.
- Keep A2A out of current implementation slices unless a concrete protocol target exists.
- Update README and instructions so future work does not conflate swarm/runtime with A2A.
- Commit checkpoint:
  - `docs(agentic): reserve a2a readiness boundary`

---

## Validation Checklist

- `cargo test -p nettoolskit-orchestrator --quiet`
- `cargo test -p nettoolskit-cli --quiet`
- `git diff --check`

---

## Risks And Mitigations

- Multi-agent runtime can become over-designed before its lifecycle is frozen.
- Permission inheritance can widen execution scope if not constrained carefully.
- A2A can be accidentally back-filled into internal runtime assumptions.
- Mitigation: freeze the local runtime contract first and keep A2A as a design-reserved edge.

---

## Specialist And Closeout

- Recommended specialist: `dev-rust-engineer`
- Tester: required once implementation begins
- Reviewer: required
- README update: required
- Suggested commit message style:
  - `docs(planning): define multi-agent runtime and a2a readiness roadmap`
  - `docs(agentic): clarify internal delegation and future a2a boundary`