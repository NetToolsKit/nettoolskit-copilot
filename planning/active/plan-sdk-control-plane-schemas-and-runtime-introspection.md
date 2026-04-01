# SDK Control Plane Schemas And Runtime Introspection Plan

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: expose typed control-plane schemas and runtime introspection surfaces for operational, CLI, and future SDK consumers.
- Normalized Request: create a detailed workstream for typed runtime control contracts and introspection APIs without coupling callers to internal structs.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-sdk-control-plane-schemas-and-runtime-introspection.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Related Workstreams:
  - `planning/active/plan-agentic-surface-boundary-separation.md`
  - `planning/active/plan-mcp-transport-auth-and-session-resilience.md`
  - `planning/active/plan-token-economy-optimization.md`
- Inputs:
  - `crates/cli/src/*`
  - `crates/orchestrator/src/execution/*`
  - `crates/commands/runtime/src/*`
  - `README.md`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| S1 | Introspection contract inventory | CLI/runtime/model/permission surfaces | 🔴 Immediate | none |
| S2 | Typed control schemas | shared schema boundary | 🔴 Immediate | S1 |
| S3 | Runtime inspection services | context/model/permission/MCP status | 🟠 High | S2 |
| S4 | CLI and service adapters | renderers over typed control data | 🟠 High | S3 |
| S5 | Docs and versioning guidance | README + governance | 🟡 Medium | S2, S4 |

---

## Ordered Tasks

### [2026-03-31 00:00] Task S1: Freeze Introspection Surface Inventory

- Inventory what runtime data should be inspectable:
  - context usage
  - model state
  - permission mode
  - MCP status
  - session/runtime health
- Define which outputs are for machines and which are for humans.
- Commit checkpoint:
  - `docs(planning): freeze control-plane introspection inventory`

### [2026-03-31 00:00] Task S2: Define Typed Control Schemas

- Create a dedicated contract set for runtime/control requests and responses.
- Keep schema evolution separate from CLI formatting.
- Version the contract and add test coverage for serialization and compatibility.
- Commit checkpoint:
  - `feat(runtime): add typed control schemas`

### [2026-03-31 00:00] Task S3: Add Runtime Inspection Services

- Build services that gather normalized introspection state.
- Keep collectors separate from renderers and commands.
- Ensure status gathering is bounded and safe in degraded states.
- Commit checkpoint:
  - `feat(runtime): add runtime introspection services`

### [2026-03-31 00:00] Task S4: Adapt CLI And Service Surfaces

- Make CLI output a renderer over typed status objects.
- Prepare service/SDK callers to reuse the same schemas.
- Avoid parallel schema definitions in multiple surfaces.
- Commit checkpoint:
  - `refactor(cli): render runtime inspection from typed control schemas`

### [2026-03-31 00:00] Task S5: Document Control Contracts

- Update README and instructions with the control-plane inspection model.
- Document versioning expectations for schemas and adapters.
- Record how these schemas align with future service/SDK integration.
- Commit checkpoint:
  - `docs(runtime): document control schemas and introspection model`

---

## Validation Checklist

- `cargo test -p nettoolskit-cli --quiet`
- `cargo test -p nettoolskit-runtime --quiet`
- `cargo test -p nettoolskit-orchestrator --quiet`
- `git diff --check`

---

## Risks And Mitigations

- Schema growth can overfit internal runtime state.
- CLI and service outputs can drift if they bypass typed contracts.
- Runtime status collection can become expensive if not bounded.
- Mitigation: start with a minimal contract set and keep renderers thin.

---

## Specialist And Closeout

- Recommended specialist: `dev-rust-engineer`
- Tester: required
- Reviewer: required
- README update: required
- Suggested commit message style:
  - `feat(runtime): add typed control schemas and introspection`
  - `docs(planning): record control-plane schema roadmap`