# Instruction Rules Board And Surface Layout Spec

Generated: 2026-03-30 08:59

## Status

- LastUpdated: 2026-04-05 12:05
- Objective: define the design intent for a repo-native rules board that makes repository instructions, agents, skills, and hooks easier to discover, route, and keep in sync.
- Normalized Request: compare the architecture in the reference image against the current repository structure and capture the useful improvements as a repo-native rules board with explicit precedence and shallow shared roots.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/completed/plan-instruction-rules-board-and-surface-layout.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already has the real ingredients of the reference architecture: agent bootstrap files, routing catalogs, instruction surfaces, skills, planning docs, and repo-owned projections. What it does not yet have is a single, explicit board that tells humans and agents how those pieces are grouped and which surfaces are authoritative.

---

## Design Summary

- Keep the current repo-owned sources of truth unchanged.
- Define the board as a governance and discoverability layer, not as a second instruction tree.
- Group surfaces under four shallow roots so the loading path is obvious and token-efficient:
  - `instructions/`
  - `agents/`
  - `skills/`
  - `hooks/`
- Preserve `super-agent` under the dedicated `agents/` root and keep repository operating-model guidance canonical.
- Keep stable `ntk-*` file names; do not use numeric directory prefixes or another deep semantic folder layer under `instructions/`.

---

## Key Decisions

- [2026-03-30 08:59] Do not copy the reference architecture literally; translate it into repo-native paths and responsibilities.
- [2026-03-30 08:59] Keep the board informational and governance-oriented so it cannot drift from the canonical instruction tree.
- [2026-03-30 08:59] Use the board to improve discoverability and token efficiency, not to create another source of truth.
- [2026-03-30 08:59] Keep the repository-owned instruction surfaces canonical and treat the external `copilot-instructions` repo as a parity reference.
- [2026-04-03 15:40] The board must document precedence explicitly in README and governance surfaces, not rely on folder order.
- [2026-04-04 10:30] The board must use shallow shared roots; `instructions/` is limited to five first-level categories and narrower specialization should move into file names instead of deeper folder trees.

---

## Alternatives Considered

1. Leave the current instruction tree as-is.
   - Rejected: it is already functional, but the grouping is not explicit enough for fast routing and low-context loading.
2. Copy the image architecture exactly.
   - Rejected: that would create a parallel layout with names that do not match the repo's actual ownership model.
3. Create a lightweight rules board that maps the existing repo surfaces into clear categories.
   - Preferred: captures the useful structure without duplicating the tree.

---

## Assumptions And Constraints

- The board may update documentation, governance metadata, and projection comments, but must not create a competing instruction tree.
- The repo already has enough surfaces to map into the board model.
- The board must not compete with `AGENTS.md`, `copilot-instructions.md`, or `ntk-agents-super-agent.instructions.md`.
- The external `copilot-instructions` repository remains a comparison baseline, not a live sync target.

---

## Risks

- If the board becomes a second instruction source, agents may route to stale guidance.
- If the categories are too granular, context loading becomes more expensive, not less.
- If the precedence rules are not explicit, repository-owned guidance can be overwritten by accident.

---

## Acceptance Criteria

- The repo has a board spec and plan that classify instruction surfaces by function.
- The board clearly distinguishes `instructions`, `agents`, `skills`, and `hooks`.
- The current repo layout is mapped to the board without copying the reference image verbatim.
- README and governance surfaces make the semantic taxonomy and precedence explicit.
- The taxonomy explicitly documents that numeric directory prefixes are not part of the contract.
- `instructions/` is explicitly limited to the five first-level categories `governance`, `development`, `operations`, `security`, and `data`.
- The board documents that narrower specialization should prefer file names over deeper nested instruction folders.

---

## Planning Readiness

- Completed: semantic taxonomy, board precedence, discoverability rules, and shallow-root guidance are now implemented at the governance/documentation layer.
- The remaining generated-surface cutover later closed under `planning/completed/plan-provider-surface-projection-cutover.md`.

---

## Recommended Specialist Focus

- Primary: `docs-release-engineer`
- Supporting: `plan-active-work-planner`