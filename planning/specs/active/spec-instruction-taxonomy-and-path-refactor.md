# Instruction Taxonomy And Path Refactor Spec

Generated: 2026-04-03 00:00

## Status

- LastUpdated: 2026-04-04 10:30
- Objective: refactor the repository instruction system into a shallow root taxonomy that separates `instructions/`, `agents/`, `skills/`, and `hooks/`, keeps `instructions/` limited to five first-level categories, and carries specialization mainly through stable file names instead of deep folder nesting.
- Normalized Request: reorganize the instruction system so it stays predictable across projects, uses four shallow shared roots, and separates repository instructions from agent, skill, and hook surfaces.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-instruction-taxonomy-and-path-refactor.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already separates shared instruction sources under `definitions/shared/instructions/` from projected GitHub-facing copies under `.github/instructions/`, but the current flat layout still mixes architecture, process, docs, runtime, security, and stack-specific guidance in one directory. Several instruction files overlap in responsibility, and there is already real drift between some shared and projected copies.

---

## Design Intent

- Organize the shared control surface under four stable roots:
  - `instructions/`
  - `agents/`
  - `skills/`
  - `hooks/`
- Keep each root shallow, with no more than one semantic folder below the root unless a later spec explicitly reopens that decision.
- Use stable repository-owned `ntk-*` prefixes for instruction filenames.
- Keep `definitions/shared/instructions/` as the canonical editable source and `.github/instructions/` as the projected mirror.
- Limit `instructions/` to five first-level categories:
  - `governance`
  - `development`
  - `operations`
  - `security`
  - `data`
- Carry narrower ownership inside those folders through file names such as:
  - `ntk-governance-*`
  - `ntk-development-*`
  - `ntk-operations-*`
  - `ntk-security-*`
  - `ntk-data-*`
- Preserve routing, prompts, skills, governance manifests, and README references through path updates.

---

## Options Considered

1. Keep the flat instruction layout and only adjust wording.
   - Rejected: taxonomy remains unclear and drift risk stays high.
2. Add folders but keep old names.
   - Rejected: ownership improves only partially and naming remains inconsistent.
3. Refactor to grouped folders with stable `ntk-*` names and authority rules.
   - Preferred: strongest long-term maintainability and clearest path semantics.

---

## Proposed Taxonomy

- `instructions/governance/`
  - repository invariants, workflow guidance, README/changelog policy, and collaboration rules live here as `ntk-governance-*` files.
- `instructions/development/`
  - architecture, backend, frontend, persistence, and testing guidance live here as `ntk-development-*` files.
- `instructions/operations/`
  - DevOps, platform, reliability, workspace, and local runtime operations live here as `ntk-operations-*` files.
- `instructions/security/`
  - application hardening, supply-chain trust, and secret-handling guidance live here as `ntk-security-*` files.
- `instructions/data/`
  - database and privacy guidance live here as `ntk-data-*` files.
- `agents/`
  - `super-agent/`
  - `planner/`
  - `reviewer/`
  - `implementer/`
- `skills/`
  - `dev-backend/`
  - `dev-frontend/`
  - `dev-rust/`
  - `test/`
  - `security/`
  - `docs/`
- `hooks/`
  - `session-start/`
  - `pre-tool-use/`
  - `subagent-start/`
  - `stop/`

This structure is the target contract for the next refactor. Instructions must stay within the five semantic instruction categories; agents, skills, and hooks must not be forced back into the instruction tree; specialization should prefer file names over deeper nested folders.

---

## Acceptance Criteria

- Instructions are grouped into concern-based folders with stable `ntk-*` prefixed filenames.
- Shared and projected instruction copies are aligned for all renamed files.
- Routing catalog, prompts, skills, manifests, plans, and README references point to the new paths.
- README policy and repo override files no longer drift between shared and projected copies.
- The most overlapping backend/frontend instruction surfaces have sharper ownership after the refactor.
- The instruction tree is limited to the five semantic categories `governance`, `development`, `operations`, `security`, and `data`.
- `agents/`, `skills/`, and `hooks/` become distinct shared roots instead of being nested under `instructions/`.
- The taxonomy stays shallow and avoids reintroducing another semantic folder layer below those first-level category and role folders.
- `super-agent` no longer competes with repository invariants inside the instruction tree; it belongs to the `agents/` root.

---

## Planning Readiness

- Ready for planning immediately.
- The first slice should freeze the target taxonomy and rename map before moving files.
- Implementation should update references in the same slice as file moves to avoid dangling paths.