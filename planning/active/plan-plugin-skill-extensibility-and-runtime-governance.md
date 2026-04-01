# Plugin Skill Extensibility And Runtime Governance Plan

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: define and strengthen the repository's extension model for skills, prompts, runtime projections, and future plugins.
- Normalized Request: create a detailed workstream for extensibility and runtime governance because the repository needs stronger ownership and validation around its extension surfaces.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-plugin-skill-extensibility-and-runtime-governance.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Related Workstreams:
  - `planning/active/plan-instruction-rules-board-and-surface-layout.md`
  - `planning/active/plan-instruction-governance-and-super-agent-retention.md`
  - `planning/active/plan-mcp-transport-auth-and-session-resilience.md`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| P1 | Extension taxonomy freeze | skills/prompts/projections/plugins | 🔴 Immediate | none |
| P2 | Discovery and manifest contracts | extension metadata and routing | 🔴 Immediate | P1 |
| P3 | Validation and governance rules | policy/tests/ownership | 🟠 High | P1, P2 |
| P4 | Runtime loading boundaries | prevent extension leakage into core | 🟠 High | P2, P3 |
| P5 | Docs and operator model | README + instructions | 🟡 Medium | P1-P4 |

---

## Ordered Tasks

### [2026-03-31 00:00] Task P1: Freeze The Extension Taxonomy

- Classify current extension surfaces by role and ownership.
- Define what counts as:
  - skill
  - prompt
  - runtime projection
  - plugin
- Record what must remain repository-owned and what may be extension-owned.
- Commit checkpoint:
  - `docs(planning): freeze extension taxonomy`

### [2026-03-31 00:00] Task P2: Define Discovery And Manifest Contracts

- Define discovery metadata and manifest expectations for extension surfaces.
- Align routing, loading, and documentation with explicit contracts.
- Prevent hidden extension points from accumulating in ad-hoc directories.
- Commit checkpoint:
  - `docs(planning): define extension discovery and manifest contracts`

### [2026-03-31 00:00] Task P3: Add Validation And Governance Rules

- Define validation for extension metadata, ownership, and structure.
- Define policy gates for unsafe or ambiguous extension behavior.
- Connect governance to existing README/instruction validation work.
- Commit checkpoint:
  - `docs(planning): define extension governance rules`

### [2026-03-31 00:00] Task P4: Define Runtime Loading Boundaries

- Ensure extension loading is explicit and bounded.
- Prevent skills or plugins from bypassing runtime, MCP, or instruction governance.
- Align runtime loading with the broader boundary-separation workstream.
- Commit checkpoint:
  - `docs(planning): define extension runtime loading boundaries`

### [2026-03-31 00:00] Task P5: Document The Extension Model

- Update README and instruction guidance with the extension taxonomy and loading rules.
- Add examples of correct extension layout and ownership.
- Commit checkpoint:
  - `docs(runtime): document plugin and skill extensibility model`

---

## Validation Checklist

- `git diff --check`

---

## Specialist And Closeout

- Recommended specialist: `docs-release-engineer`
- Reviewer: required
- README update: required
- Suggested commit message style:
  - `docs(planning): add plugin and skill extensibility roadmap`