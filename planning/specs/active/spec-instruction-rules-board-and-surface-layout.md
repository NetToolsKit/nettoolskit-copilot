# Instruction Rules Board And Surface Layout Spec

Generated: 2026-03-30 08:59

## Status

- LastUpdated: 2026-03-30 08:59
- Objective: define the design intent for a repo-native rules board that makes instruction configuration, rules, commands, and skills easier to discover, route, and keep in sync.
- Normalized Request: compare the architecture in the reference image against the current repository structure and capture the useful improvements as a planning-only workstream.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-instruction-rules-board-and-surface-layout.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already has the real ingredients of the reference architecture: agent bootstrap files, routing catalogs, instruction surfaces, skills, planning docs, and repo-owned projections. What it does not yet have is a single, explicit board that tells humans and agents how those pieces are grouped and which surfaces are authoritative.

---

## Design Summary

- Keep the current repo-owned sources of truth unchanged.
- Define the board as a governance and discoverability layer, not as a second instruction tree.
- Group surfaces by responsibility so the loading path is obvious and token-efficient.
- Preserve `super-agent` and the repo operating model as the highest-precedence contracts.

---

## Key Decisions

- [2026-03-30 08:59] Do not copy the reference architecture literally; translate it into repo-native paths and responsibilities.
- [2026-03-30 08:59] Keep the board informational and governance-oriented so it cannot drift from the canonical instruction tree.
- [2026-03-30 08:59] Use the board to improve discoverability and token efficiency, not to create another source of truth.
- [2026-03-30 08:59] Keep the repository-owned instruction surfaces canonical and treat the external `copilot-instructions` repo as a parity reference.

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

- No instruction content is being changed by this spec.
- The repo already has enough surfaces to map into the board model.
- The board must not compete with `AGENTS.md`, `copilot-instructions.md`, or `ntk-core-super-agent.instructions.md`.
- The external `copilot-instructions` repository remains a comparison baseline, not a live sync target.

---

## Risks

- If the board becomes a second instruction source, agents may route to stale guidance.
- If the categories are too granular, context loading becomes more expensive, not less.
- If the precedence rules are not explicit, repository-owned guidance can be overwritten by accident.

---

## Acceptance Criteria

- The repo has a planning-only board spec and plan that classify instruction surfaces by function.
- The board clearly distinguishes configuration, rules, commands, and skills.
- The current repo layout is mapped to the board without copying the reference image verbatim.
- No instruction files or code files are changed as part of this planning-only workstream.

---

## Planning Readiness

- Updated: 2026-03-30 08:59 — ready for planning-only alignment; implementation should wait until the board model is accepted and placed under the SDD baseline.

---

## Recommended Specialist Focus

- Primary: `docs-release-engineer`
- Supporting: `plan-active-work-planner`
