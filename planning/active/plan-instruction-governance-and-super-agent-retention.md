# Instruction Governance And Super Agent Retention Plan

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-04-04 22:20
- Objective: keep repository instructions, agent surfaces, skill surfaces, hook surfaces, and the external `copilot-instructions` reference aligned without losing canonical guidance, routing fidelity, or the `super-agent` lifecycle.
- Normalized Request: create a planning workstream for instruction organization and retention so the repository keeps the shared control-surface system intact while avoiding drift from `C:\Users\tguis\copilot-instructions`.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-instruction-governance-and-super-agent-retention.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Dependency: `planning/active/plan-instruction-rules-board-and-surface-layout.md`
- Inputs:
  - `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`
  - `definitions/agents/super-agent/ntk-agents-super-agent.instructions.md`
  - `definitions/providers/github/root/instruction-routing.catalog.yml`
  - `C:\Users\tguis\copilot-instructions\`
  - `planning/completed/plan-instruction-parity-and-script-retirement.md`

---

## Scope Summary

This plan coordinates four governance slices:

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| G1 | Instruction parity audit | repo vs external reference | 🔴 Immediate | none |
| G2 | Super-agent retention | local and projected instruction surfaces | ✅ Done | G1 |
| G3 | Routing and precedence clarity | catalog + operating model docs | ✅ Done | G1 |
| G4 | Drift monitoring and sync rules | instructions projections and planning docs | ✅ Done | G2, G3 |
| G5 | Domain consolidation and root normalization | shallow shared roots with reduced repetition and explicit surface ownership | 🟡 In progress | G2, G3, G4 |

The `ntk` CLI prefix is already in place and is not a new implementation gap.

---

## Ordered Tasks

### [2026-03-30 07:31] Task G1: Baseline Instruction Parity

- Compare the repository-owned instruction surfaces with the external `copilot-instructions` baseline.
- Record the intentionally diverged areas so they are not “fixed” by accident.
- Keep the `super-agent` operating model explicitly canonical.
- Commit checkpoint:
  - `docs(planning): freeze instruction governance parity baseline`

### [2026-03-30 07:31] Task G2: Preserve The Super Agent Surface

- Keep the `super-agent` workflow visible in the repo-owned instruction set.
- Make sure instruction projections do not overwrite the canonical guidance with a stale copy.
- Confirm the repo still routes work through the same lifecycle contract.
- Status:
  - complete; `super-agent` now lives in the dedicated `agents/` lane and the rules board keeps it globally visible with semantic taxonomy
- Commit checkpoint:
  - `docs(instructions): reinforce super agent canonical routing`

### [2026-03-30 07:31] Task G3: Clarify Routing And Precedence

- Ensure the routing catalog and operating model remain consistent with the actual workspace.
- Keep `ntk` and instruction-file guidance aligned for operators and agents.
- Document how the repository keeps command surfaces and instruction surfaces in sync.
- Status:
  - complete; precedence and semantic-folder routing are now documented in `AGENTS.md`, instruction READMEs, and governance metadata
- Commit checkpoint:
  - `docs(instructions): clarify routing and precedence for repository guidance`

### [2026-03-30 07:31] Task G4: Define Drift Monitoring And Sync Rules

- Add the ongoing rule for syncing from the external baseline without losing repo-owned changes.
- Document how to detect and review drift before it becomes a branch or PR problem.
- Keep the guidance about `ntk` prefix usage and the `super-agent` lifecycle intact.
- Status:
  - complete; the instruction ownership manifest and repository operating model now define canonical root, projected root, provider roots, and drift rules
- Commit checkpoint:
  - `docs(planning): define instruction drift monitoring and sync policy`

### [2026-04-03 17:10] Task G5: Consolidate Semantic Instruction Domains

- Reduce repeated guidance inside semantic instruction folders without collapsing distinct responsibilities.
- Keep canonical authority in `definitions/` and project consumer-facing copies from those canonical surfaces.
- Normalize the final shared-root contract as:
  - `instructions/`
  - `agents/`
  - `skills/`
  - `hooks/`
- Limit `instructions/` to:
  - `governance/`
  - `development/`
  - `operations/`
  - `security/`
  - `data/`
- Carry specialization primarily in file names instead of deep nested instruction lanes.
- Status:
  - responsibility narrowing is complete across backend, frontend, agentic, operations, data, security, and testing content; those slices are now treated as transitional inputs to the final shallow root taxonomy
  - `super-agent` is already treated as a dedicated agent surface and must stay outside the final `instructions/` tree
  - final normalization is still pending: collapse transitional instruction lanes into the five first-level instruction categories and reserve dedicated shared roots for `agents/`, `skills/`, and `hooks/`
- Commit checkpoint:
  - `docs(instructions): narrow backend instruction responsibilities`

---

## Validation Checklist

- `git diff --check`
- instruction parity audit against `C:\Users\tguis\copilot-instructions`
- repository instruction validation command(s)
- planning structure validation

---

## Risks And Mitigations

- Overwriting repo-owned instruction surfaces with external copies would erase local policy.
- Routing drift can cause agents to load the wrong instructions and make wrong decisions.
- Mitigation: keep a named parity baseline and only sync through explicit review.

---

## Specialist And Closeout

- Recommended specialist: `docs-release-engineer`
- Tester: required if projections or commands change
- Reviewer: required
- Release closeout: required
- README update: required if the operator guidance changes
- Changelog: required if the instruction behavior changes materially
- Suggested commit message style:
  - `docs(instructions): align instruction governance with external baseline`
  - `docs(planning): record instruction governance roadmap`