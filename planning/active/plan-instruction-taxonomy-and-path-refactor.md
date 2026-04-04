# Instruction Taxonomy And Path Refactor Plan

Generated: 2026-04-03 00:00

## Status

- LastUpdated: 2026-04-04 10:30
- Objective: refactor the repository instruction system into a shallow, predictable layout rooted in `instructions/`, `agents/`, `skills/`, and `hooks/`, with `instructions/` limited to five first-level categories and specialization carried by file names instead of deeper folder nesting.
- Normalized Request: reorganize the instruction system while the repository is still evolving so the top-level layout stays predictable across projects, `instructions/` keeps only five primary categories, and agent/skill/hook surfaces stop being mixed into the instruction tree.
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
| I8 | Split process lane into workflow subdomains | replace flat `process/` paths with `planning`, `collaboration`, and `delivery` | ✅ Done | I4, I6 |
| I9 | Separate agent-controller lane | move `super-agent` from `core/` into dedicated `agents/` surfaces | ✅ Done | I4, I6, I8 |
| I10 | Freeze root taxonomy and shallow depth rule | move from the transitional lane split to `instructions/`, `agents/`, `skills/`, and `hooks/` roots with shallow paths and explicit authority | 🔴 Immediate | I9 |
| I11 | Re-map semantic lanes to five primary instruction categories | collapse current instruction lanes into `governance`, `development`, `operations`, `security`, and `data`, with specialization encoded in file names | 🔴 Immediate | I10 |

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
  - complete as a transitional slice; canonical shared files and projected `.github` files now live under narrower `operations/*` subdomains that will later collapse into the final shallow `instructions/operations/` category
  - complete; routing catalogs, provider prompts/chatmodes/skills, VS Code settings/snippets, validation fixtures, and active plans now point at the semantic operations subdomains
- Commit checkpoint:
  - `refactor(instructions): split operations taxonomy into semantic subdomains`

### [2026-04-04 00:32] Task I8: Split Flat Process Lane

- Replace the flat `process/` lane with narrower workflow subdomains:
  - `process/planning/`
  - `process/collaboration/`
  - `process/delivery/`
- Keep the existing `ntk-process-*` file names stable during the folder move to minimize rename churn outside path updates.
- Move backend and frontend test-specific guidance out of `process/`; keep only cross-cutting workflow verification in `process/delivery/`.
- Update canonical shared paths first, projected `.github` paths second, then routing, skills, prompts, governance, validation fixtures, and active plans in the same slice.
- Status:
  - complete as a transitional slice; canonical shared files and projected `.github` files now live under `process/planning`, `process/collaboration`, and `process/delivery`, which will later collapse into the final shallow `instructions/governance/` category
  - complete; routing catalogs, provider skills/prompts, governance manifests, validation fixtures, and active planning references now point at the narrower process workflow lanes
  - complete; dedicated `process/README.md` files now document the three workflow lanes and keep platform concerns under `operations/`
- Commit checkpoint:
  - `refactor(instructions): split process taxonomy into workflow subdomains`

### [2026-04-04 02:20] Task I9: Separate Agent-Control From Core

- Move `super-agent` out of `core/` into a dedicated `agents/` lane.
- Keep `core/` reserved for repository invariants:
  - repository operating model
  - authoritative sources
  - artifact layout
- Update canonical shared paths first, projected `.github` paths second, then routing, provider consumers, governance manifests, validation fixtures, and instruction-architecture tests in the same slice.
- Status:
  - complete; `ntk-agents-super-agent.instructions.md` now lives under `agents/` in canonical and projected trees
  - complete; rules-board docs, governance manifests, provider surfaces, and validation fixtures now treat `agents/` as a dedicated semantic lane instead of overloading `core/`
- Commit checkpoint:
  - `refactor(instructions): separate agent-controller guidance from core invariants`

### [2026-04-04 10:05] Task I10: Freeze Root Taxonomy And Depth Rule

- Adopt the shared-root contract under `definitions/shared/`:
  - `instructions/`
  - `agents/`
  - `skills/`
  - `hooks/`
- Mirror the same conceptual layout for projected and provider-facing consumers where applicable.
- Keep the taxonomy shallow:
  - `instructions/` may contain only the five first-level semantic categories
  - `agents/`, `skills/`, and `hooks/` may contain only role/lifecycle folders plus files
  - avoid adding another nested lane below those category/role folders unless a later spec explicitly reopens that decision
  - no numeric prefixes
  - no mixed concerns such as `runtime-ops/` or `process/` surviving the final layout
- Status:
  - planning only; no file moves executed in this slice
- Commit checkpoint:
  - `docs(planning): freeze shared instruction root taxonomy`

### [2026-04-04 10:05] Task I11: Re-map Instruction Lanes To Five Primary Categories

- Freeze the final instruction categories as:
  - `instructions/governance/`
  - `instructions/development/`
  - `instructions/operations/`
  - `instructions/security/`
  - `instructions/data/`
- Keep specialization inside those folders at the file-name level, not as another folder taxonomy:
  - `ntk-governance-*`
  - `ntk-development-*`
  - `ntk-operations-*`
  - `ntk-security-*`
  - `ntk-data-*`
- Move agent-controller lifecycle out of `instructions/` entirely and into:
  - `agents/super-agent/`
  - `agents/planner/`
  - `agents/reviewer/`
  - `agents/implementer/`
- Keep reusable engineering playbooks under:
  - `skills/dev-backend/`
  - `skills/dev-frontend/`
  - `skills/dev-rust/`
  - `skills/test/`
  - `skills/security/`
  - `skills/docs/`
- Keep lifecycle automation under:
  - `hooks/session-start/`
  - `hooks/pre-tool-use/`
  - `hooks/subagent-start/`
  - `hooks/stop/`
- Status:
  - planning only; rename and move map still pending execution
- Commit checkpoint:
  - `docs(planning): freeze shallow instruction category model`

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
- Changing root taxonomy affects not only instruction paths but also skill, prompt, hook, and provider projection paths.
- Shared/projected drift can reappear if the authority rule stays implicit.
- Over-consolidating backend/frontend files can remove useful stack-specific guidance.
- Mitigation: freeze the root taxonomy and shallow-depth rule first, update consumers in the same refactor, and keep instructions vs agents vs skills vs hooks explicit.

---

## Specialist And Closeout

- Recommended specialist: `docs-release-engineer`
- Tester: required
- Reviewer: required
- README update: required
- Suggested commit message style:
  - `refactor(instructions): reorganize instruction taxonomy`
  - `docs(planning): record instruction taxonomy refactor roadmap`