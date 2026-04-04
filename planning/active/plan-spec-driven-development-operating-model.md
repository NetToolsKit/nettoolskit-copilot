# Spec-Driven Development Operating Model Plan

Generated: 2026-03-30 08:59

## Status

- LastUpdated: 2026-03-30 08:59
- Objective: establish the spec-first operating model that every remaining planning workstream must follow so the repository can preserve quality, token efficiency, instruction governance, and branch discipline without starting implementation.
- Normalized Request: adjust the remaining planning backlog to run under a Spec-Driven Development baseline aligned with the current repository instruction system, while keeping this work planning-only for now.
- Active Branch: `docs/planning-gap-workstreams` (planning only; implementation branches TBD)
- Spec Path: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Dependencies:
  - `planning/README.md`
  - `planning/specs/README.md`
  - `definitions/shared/instructions/process/planning/ntk-process-brainstorm-spec-workflow.instructions.md`
  - `.github/instructions/agents/ntk-agents-super-agent.instructions.md`
  - `.github/instructions/core/ntk-core-repository-operating-model.instructions.md`
  - `.codex/skills/context-token-optimizer/SKILL.md`
  - `.codex/skills/core-context-router/SKILL.md`
  - `planning/active/plan-token-economy-optimization.md`
  - `planning/active/plan-rag-cag-sqlite-evolution.md`
  - `planning/active/plan-build-target-cleanup-and-artifact-pruning.md`
  - `planning/active/plan-instruction-governance-and-super-agent-retention.md`
  - `planning/active/plan-script-retirement-tail-cutover.md`

---

## Scope Summary

This plan coordinates four operating-model slices:

| ID | Slice | Priority | Dependency |
|---|---|---|---|
| S1 | Spec-first intake and planning gate | 🔴 Immediate | none |
| S2 | Token-efficient planning context packs | 🔴 Immediate | S1 |
| S3 | Instruction and routing governance | 🟠 High | S1 |
| S4 | Category-plan alignment and closeout rules | 🟠 High | S1, S2, S3 |

This workstream does not implement product behavior. It defines the planning discipline that the category workstreams must use.

---

## Ordered Tasks

### [2026-03-30 08:59] Task S1: Baseline The Current Spec-First Surface

- Inventory the current spec-first tooling and docs:
  - `planning/specs/README.md`
  - `planning/README.md`
  - `definitions/shared/instructions/process/planning/ntk-process-brainstorm-spec-workflow.instructions.md`
  - `.github/instructions/agents/ntk-agents-super-agent.instructions.md`
  - `.github/instructions/core/ntk-core-repository-operating-model.instructions.md`
  - `.codex/skills/context-token-optimizer/SKILL.md`
  - `.codex/skills/core-context-router/SKILL.md`
- Record the current planning and spec surfaces that already support SDD.
- Keep the current repo operating model as the baseline instead of replacing it.
- Commit checkpoint:
  - `docs(planning): baseline spec-driven development operating model`

### [2026-03-30 08:59] Task S2: Define Token-Efficient Planning Context Packs

- Define the smallest reliable context pack for planning work.
- Separate intake, spec design, and execution planning context so agents do not re-read the full repo unnecessarily.
- Document when to load a spec, when to load a plan, and when to route through the context-token optimizer.
- Commit checkpoint:
  - `docs(planning): define token-efficient planning context pack`

### [2026-03-30 08:59] Task S3: Clarify Instruction And Routing Governance

- Make instruction precedence explicit for the repository-owned surfaces and the external `copilot-instructions` reference.
- Keep `super-agent` canonical while preserving the repo operating model and routing catalog.
- Document how planning work should choose specialists and load only minimal context.
- Commit checkpoint:
  - `docs(instructions): define spec-driven instruction routing policy`

### [2026-03-30 08:59] Task S4: Align Category Plans Under The SDD Umbrella

- Rebase the open category plans so they reference this operating-model baseline.
- Keep token economy, RAG/CAG, build cleanup, instruction governance, and script retirement as separate planning tracks, but under one SDD governance model.
- Update planning indexes and parent pointers to make the relationship obvious.
- Commit checkpoint:
  - `docs(planning): align category workstreams under spec-driven development`

### [2026-03-30 08:59] Task S5: Define Closeout Rules For Planning-Only Work

- Define when a planning doc is ready to move from active to completed.
- Keep planning-only work clearly separated from implementation branches.
- Document the branch and commit cadence for future SDD-aligned workstreams.
- Commit checkpoint:
  - `docs(planning): finalize spec-driven development closeout rules`

---

## Validation Checklist

- `git diff --check`
- `& .\.build\target\debug\ntk.exe validation planning-structure --repo-root . --warning-only false`
- `& .\.build\target\debug\ntk.exe validation instructions --repo-root . --warning-only false`
- `& .\.build\target\debug\ntk.exe validation readme-standards --repo-root . --warning-only false`

---

## Risks And Mitigations

- Over-specifying the operating model could add more planning overhead than value.
  - Mitigation: keep the plan slice-oriented and reuse the existing planning docs instead of duplicating them.
- Token-efficiency guidance could become too strict and hide needed context.
  - Mitigation: define minimal context packs with explicit exceptions for complex work.
- Instruction drift could reappear if the external baseline and repo-owned guidance diverge.
  - Mitigation: keep the parity baseline visible and route changes through explicit review.

---

## Specialist And Closeout

- Recommended specialist: `plan-active-work-planner`
- Tester: required for validation commands and index checks
- Reviewer: required
- Release closeout: required
- README update: required
- Changelog: not required until implementation work lands
- Suggested commit message style:
  - `docs(planning): align repository planning with spec-driven development`
  - `docs(planning): record SDD operating model roadmap`