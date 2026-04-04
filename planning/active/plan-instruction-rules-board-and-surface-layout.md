# Instruction Rules Board And Surface Layout Plan

Generated: 2026-03-30 08:59

## Status

- LastUpdated: 2026-04-04 02:20
- Objective: evaluate and define a repo-native rules board that groups instruction surfaces by responsibility so agents can load config, rules, commands, and skills with less ambiguity and less context waste.
- Normalized Request: compare the board-style agent architecture against the current repository instruction layout and plan the improvements that would make it more discoverable, token-efficient, and SDD-friendly.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-instruction-rules-board-and-surface-layout.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Dependencies:
  - `planning/active/plan-spec-driven-development-operating-model.md`
  - `planning/active/plan-instruction-governance-and-super-agent-retention.md`
  - `.github/AGENTS.md`
  - `.github/copilot-instructions.md`
  - `.github/instruction-routing.catalog.yml`
  - `.github/instructions/agents/ntk-agents-super-agent.instructions.md`
  - `.github/instructions/core/ntk-core-repository-operating-model.instructions.md`
  - `.codex/skills/README.md`
  - `definitions/README.md`

---

## Scope Summary

This plan coordinates four layout slices:

| ID | Slice | Priority | Dependency |
|---|---|---|---|
| B1 | Inventory current instruction surfaces | ✅ Done | none |
| B2 | Define rules board categories and precedence | ✅ Done | B1 |
| B3 | Map current repo folders to the board model | ✅ Done | B1, B2 |
| B4 | Define update and drift rules for the board | 🟠 Active | B2, B3 |

The board is implemented as documentation and governance. It does not create a second instruction source of truth.

---

## Ordered Tasks

### [2026-03-30 08:59] Task B1: Inventory Current Instruction Surfaces

- Capture the current authoritative surfaces:
  - `.github/AGENTS.md`
  - `.github/copilot-instructions.md`
  - `.github/instruction-routing.catalog.yml`
  - `.github/instructions/**/*.instructions.md`
  - `.codex/skills/*/SKILL.md`
  - `definitions/shared/instructions/**/*.instructions.md`
- Identify which surfaces are canonical, projected, or local overrides.
- Status:
  - complete; inventory now reflects the semantic folder taxonomy under `core/`, `process/`, `architecture/`, `operations/`, `data/`, `security/`, and `docs/`
- Commit checkpoint:
  - `docs(planning): baseline instruction surface inventory`

### [2026-03-30 08:59] Task B2: Define Board Categories And Precedence

- Group surfaces into the same conceptual lanes shown in the reference image:
  - AI agent configuration
  - core agent functions
  - mandatory rules
  - standard structure
  - skills
- Define which surfaces are mandatory, which are standard, and which are optional or projection-only.
- Status:
  - complete; categories and precedence are now documented in `.github/instructions/README.md`, `definitions/shared/instructions/README.md`, and `.github/AGENTS.md`
- Commit checkpoint:
  - `docs(instructions): define rules board categories and precedence`

### [2026-03-30 08:59] Task B3: Map Repo Paths To The Board Model

- Map the existing repo layout into the board model without copying the image verbatim.
- Keep the existing repo-owned source of truth model intact.
- Document where the board should point for each category.
- Status:
  - complete; the board now maps semantic instruction folders and the governance manifest tracks domain surfaces by nested taxonomy
- Commit checkpoint:
  - `docs(instructions): map repository instruction surfaces to rules board`

### [2026-03-30 08:59] Task B4: Define Drift And Update Rules

- Define how the board gets updated when instructions or skills change.
- Keep the external `copilot-instructions` baseline as a reference comparison, not a live write target.
- Add rules for avoiding duplicate or stale instruction surfaces.
- Active follow-up:
  - finish normalizing remaining references, labels, and helper surfaces that still mention legacy flat instruction names
  - keep `super-agent` under the dedicated `agents/` lane so `core/` remains reserved for repository invariants
- Commit checkpoint:
  - `docs(planning): define rules board update and drift policy`

---

## Validation Checklist

- `git diff --check`
- `& .\.build\target\debug\ntk.exe validation instructions --repo-root . --warning-only false`
- `& .\.build\target\debug\ntk.exe validation planning-structure --repo-root . --warning-only false`

---

## Risks And Mitigations

- A board that duplicates the actual instructions tree would create more drift, not less.
  - Mitigation: make the board an index and governance model, not a parallel source of truth.
- Too many categories could increase token usage instead of reducing it.
  - Mitigation: keep the board shallow and aligned with existing repo surfaces.
- Changes to the board could accidentally override repo-owned instruction precedence.
  - Mitigation: keep the repository operating model and super-agent rules canonical.

---

## Specialist And Closeout

- Recommended specialist: `docs-release-engineer`
- Tester: required for validation commands
- Reviewer: required
- Release closeout: required
- README update: required if the board becomes a user-facing navigation surface
- Changelog: not required until implementation lands
- Suggested commit message style:
  - `docs(instructions): define repository rules board model`
  - `docs(planning): record instruction surface layout roadmap`