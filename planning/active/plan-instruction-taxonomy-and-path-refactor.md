# Instruction Taxonomy And Path Refactor Plan

Generated: 2026-04-03 00:00

## Status

- LastUpdated: 2026-04-03 01:05
- Objective: refactor the repository instruction system into grouped semantic folders with stable `ntk-*` naming, clearer authority boundaries, and reduced duplication/drift.
- Normalized Request: reorganize the instruction system while the repository is still evolving so instructions are grouped by concern, references remain valid, and repeated guidance is reduced.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-instruction-taxonomy-and-path-refactor.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Related Workstreams:
  - `planning/active/plan-instruction-rules-board-and-surface-layout.md`
  - `planning/active/plan-instruction-governance-and-super-agent-retention.md`
  - `planning/active/plan-repository-consolidation-continuity.md`
- Inputs:
  - `definitions/shared/instructions/*`
  - `.github/instructions/*`
  - `.github/instruction-routing.catalog.yml`
  - `.github/prompts/*`
  - `.codex/skills/*`
  - `planning/README.md`
  - `planning/specs/README.md`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| I1 | Freeze taxonomy and rename map | instruction folders + file names | 🔴 Immediate | none |
| I2 | Move canonical shared instructions | `definitions/shared/instructions` | 🔴 Immediate | I1 |
| I3 | Reproject `.github/instructions` and unify drift | projected copies + overrides | 🔴 Immediate | I2 |
| I4 | Update routing and consumption paths | routing catalog, prompts, skills, manifests, plans | 🟠 High | I2, I3 |
| I5 | Tighten backend/frontend ownership | reduce overlap in the highest-conflict files | ✅ Done | I3 |
| I6 | Add taxonomy docs and validation references | README/governance/closeout | 🟡 Medium | I4, I5 |
| I7 | Split generic operations lane | replace `runtime-ops/` with narrower `operations/*` subdomains | ✅ Done | I4, I6 |

---

## Ordered Tasks

### [2026-04-03 00:00] Task I1: Freeze Taxonomy And Rename Map

- Define the target folder taxonomy inside:
  - `definitions/shared/instructions/`
  - `.github/instructions/`
- Define the stable rename map for every instruction file.
- Keep `.instructions.md` suffix and add `ntk-*` prefix to the renamed files.
- Commit checkpoint:
  - `docs(planning): freeze instruction taxonomy and rename map`

### [2026-04-03 00:00] Task I2: Move Canonical Shared Instructions

- Move shared instruction sources into the new folder taxonomy.
- Update only canonical copies first.
- Keep semantics stable during the move.
- Commit checkpoint:
  - `refactor(instructions): move canonical shared instruction sources`

### [2026-04-03 00:00] Task I3: Reproject And Unify Projected Copies

- Move `.github/instructions/` to the same folder taxonomy.
- Eliminate shared/projected drift in:
  - README baseline
  - repo-specific README overrides
  - any renamed instruction that still differs across the mirror
- Commit checkpoint:
  - `refactor(instructions): align projected instruction surfaces`

### [2026-04-03 00:00] Task I4: Update All Consumers

- Update references in:
  - `.github/instruction-routing.catalog.yml`
  - `.github/prompts/*`
  - `.codex/skills/*`
  - governance manifests and policies
  - planning indexes and active plans/specs
- Ensure there are no dangling legacy paths.
- Commit checkpoint:
  - `refactor(instructions): update instruction routing and consumers`

### [2026-04-03 00:00] Task I5: Reduce Ownership Overlap

- Narrow backend instruction ownership so architecture, stack-specific conventions, and testing do not repeat excessively.
- Narrow frontend instruction ownership so frontend baseline, Vue/Quasar implementation, Vue/Quasar architecture, and UI/UX each have a distinct role.
- Keep business/commercial guidance out of repo-wide technical instructions.
- Commit checkpoint:
  - `refactor(instructions): narrow backend and frontend ownership`

### [2026-04-03 00:00] Task I6: Document The Taxonomy

- Update instruction-system docs, relevant READMEs, and planning references.
- Record the canonical authority rule:
  - `definitions/shared/instructions` is source of truth
  - `.github/instructions` is projected consumer surface
- Status:
  - complete; semantic taxonomy now exposes `data/` and `security/` as separate lanes instead of the previous combined `data-security/` folder
- Commit checkpoint:
  - `docs(instructions): document instruction taxonomy and authority model`

### [2026-04-04 00:18] Task I7: Split Generic Operations Lane

- Replace the generic `runtime-ops/` bucket with narrower semantic lanes:
  - `operations/devops/`
  - `operations/automation/`
  - `operations/containers/`
  - `operations/reliability/`
  - `operations/quality/`
- Keep the existing `ntk-runtime-*` file names stable during the folder move to minimize rename churn outside path updates.
- Update canonical shared paths first, projected `.github` paths second, then routing, providers, README indexes, prompts, skills, VS Code assets, plans, and validation fixtures in the same slice.
- Status:
  - complete; canonical shared files and projected `.github` files now live under `operations/*`
  - complete; routing catalogs, provider prompts/chatmodes/skills, VS Code settings/snippets, validation fixtures, and active plans now point at the semantic operations subdomains
- Commit checkpoint:
  - `refactor(instructions): split operations taxonomy into semantic subdomains`

---

## Validation Checklist

- `cargo test -p nettoolskit-validation --quiet`
- `cargo test -p nettoolskit-validation planning_structure --quiet`
- `pwsh -File .\\scripts\\validation\\validate-instructions.ps1`
- `pwsh -File .\\scripts\\validation\\validate-planning-structure.ps1`
- `git diff --check`

---

## Risks And Mitigations

- Renaming instructions can break routing, prompts, and skills if path updates are incomplete.
- Shared/projected drift can reappear if the authority rule stays implicit.
- Over-consolidating backend/frontend files can remove useful stack-specific guidance.
- Mitigation: freeze the rename map first, update consumers in the same refactor, and keep architecture-vs-stack ownership explicit.

---

## Specialist And Closeout

- Recommended specialist: `docs-release-engineer`
- Tester: required
- Reviewer: required
- README update: required
- Suggested commit message style:
  - `refactor(instructions): reorganize instruction taxonomy`
  - `docs(planning): record instruction taxonomy refactor roadmap`