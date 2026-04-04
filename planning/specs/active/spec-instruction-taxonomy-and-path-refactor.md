# Instruction Taxonomy And Path Refactor Spec

Generated: 2026-04-03 00:00

## Status

- LastUpdated: 2026-04-04 11:05
- Objective: refactor the repository definition system into a shallow root taxonomy centered on `definitions/`, separating `instructions/`, `templates/`, `agents/`, `skills/`, `hooks/`, and `providers/` while preserving stable naming, manifest samples, and migration safety.
- Normalized Request: reorganize the definition system so it stays predictable across projects, uses shallow canonical roots under `definitions/`, separates repository instructions from agents, skills, hooks, and provider projections, and keeps documentation samples distinct from canonical templates.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-instruction-taxonomy-and-path-refactor.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already separates shared instruction sources under `definitions/shared/instructions/` from projected GitHub-facing copies under `.github/instructions/`, but the current legacy layout still mixes multiple concerns under transitional roots and keeps canonical assets split between `definitions/shared/`, root `templates/`, and documentation example folders. Several instruction files overlap in responsibility, and there is already real drift between some shared and projected copies.

---

## Design Intent

- Organize the canonical definition surface under six stable roots inside `definitions/`:
  - `instructions/`
  - `templates/`
  - `agents/`
  - `skills/`
  - `hooks/`
  - `providers/`
- Keep each root shallow, with no more than one semantic folder below the root unless a later spec explicitly reopens that decision.
- Use stable repository-owned `ntk-*` prefixes for instruction filenames.
- Move canonical authorship to `definitions/` and treat `definitions/shared/` plus root `templates/` as legacy compatibility sources during the migration.
- Limit `instructions/` to five first-level categories:
  - `governance`
  - `development`
  - `operations`
  - `security`
  - `data`
- Limit `templates/` to:
  - `codegen`
  - `docs`
  - `manifests`
  - `prompts`
  - `workflows`
- Carry narrower ownership inside those folders through file names such as:
  - `ntk-governance-*`
  - `ntk-development-*`
  - `ntk-operations-*`
  - `ntk-security-*`
  - `ntk-data-*`
- Keep documentation samples in `docs/samples/manifests/` instead of mixing them with canonical manifest templates.
- Preserve routing, prompts, skills, governance manifests, README references, and provider projections through path updates.

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

- `definitions/instructions/governance/`
  - repository invariants, workflow guidance, README/changelog policy, and collaboration rules live here as `ntk-governance-*` files.
- `definitions/instructions/development/`
  - architecture, backend, frontend, persistence, and testing guidance live here as `ntk-development-*` files.
- `definitions/instructions/operations/`
  - DevOps, platform, reliability, workspace, and local runtime operations live here as `ntk-operations-*` files.
- `definitions/instructions/security/`
  - application hardening, supply-chain trust, and secret-handling guidance live here as `ntk-security-*` files.
- `definitions/instructions/data/`
  - database and privacy guidance live here as `ntk-data-*` files.
- `definitions/templates/`
  - `codegen/`
  - `docs/`
  - `manifests/`
  - `prompts/`
  - `workflows/`
- `definitions/agents/`
  - `super-agent/`
  - `planner/`
  - `reviewer/`
  - `implementer/`
- `definitions/skills/`
  - `dev-backend/`
  - `dev-frontend/`
  - `dev-rust/`
  - `test/`
  - `security/`
  - `docs/`
- `definitions/hooks/`
  - `session-start/`
  - `pre-tool-use/`
  - `subagent-start/`
  - `stop/`
- `definitions/providers/`
  - `github/`
  - `codex/`
  - `claude/`
  - `vscode/`
- `docs/samples/manifests/`
  - human-readable manifest examples rendered or curated for operators and contributors

This structure is the target contract for the next refactor. Instructions must stay within the five semantic instruction categories; templates stay separate from instructions; agents, skills, and hooks must not be forced back into the instruction tree; and documentation samples stay outside canonical template storage.

---

## Acceptance Criteria

- Instructions are grouped into concern-based folders with stable `ntk-*` prefixed filenames.
- Shared and projected instruction copies are aligned for all renamed files.
- Routing catalog, prompts, skills, manifests, plans, and README references point to the new paths.
- README policy and repo override files no longer drift between shared and projected copies.
- The most overlapping backend/frontend instruction surfaces have sharper ownership after the refactor.
- The canonical definition tree under `definitions/` is limited to `instructions/`, `templates/`, `agents/`, `skills/`, `hooks/`, and `providers/`.
- The instruction tree is limited to the five semantic categories `governance`, `development`, `operations`, `security`, and `data`.
- `agents/`, `skills/`, and `hooks/` become distinct canonical roots instead of being nested under instructions.
- `templates/` is grouped by artifact type and not mixed into instructions.
- `docs/samples/manifests/` exists as the human sample surface for manifests.
- The taxonomy stays shallow and avoids reintroducing another semantic folder layer below those first-level category and role folders.
- `super-agent` no longer competes with repository invariants inside the instruction tree; it belongs to the `agents/` root.

---

## Planning Readiness

- Ready for planning immediately.
- The first slice should freeze the target taxonomy and rename map before moving files.
- Implementation should update references in the same slice as file moves to avoid dangling paths.