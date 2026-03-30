# Spec-Driven Development Operating Model Spec

Generated: 2026-03-30 08:59

## Status

- LastUpdated: 2026-03-30 08:59
- Objective: define the design intent for a spec-first operating model that keeps repository planning token-efficient, instruction-aligned, and safe for future implementation branches.
- Normalized Request: adjust the remaining planning backlog so token economy, RAG/CAG, build cleanup, instruction governance, and script retirement all run under a shared SDD baseline without implementation yet.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already has strong planning, instruction, and routing surfaces, but they are distributed across several active category plans. The missing piece is an explicit SDD umbrella that tells future workstreams when to create specs, when to create plans, how much context to load, and how to keep the repository-owned instruction system canonical.

---

## Design Summary

- Keep specs as the design gate before execution planning for non-trivial work.
- Use active plans to translate the approved spec into ordered slices, validation, and closeout.
- Keep token efficiency as a planning constraint, not an afterthought.
- Preserve `super-agent` and the repo instruction model as canonical surfaces.
- Keep the category plans independent, but governed by one SDD baseline.

---

## Key Decisions

- [2026-03-30 08:59] Use a standalone SDD umbrella spec and plan instead of overloading the repository consolidation plan.
- [2026-03-30 08:59] Treat `planning/specs/active/` as the mandatory design gate for non-trivial work before execution planning.
- [2026-03-30 08:59] Keep token economy and context routing as enablers of the SDD workflow rather than separate operating models.
- [2026-03-30 08:59] Keep instruction governance anchored to repository-owned surfaces while preserving the external `copilot-instructions` reference for parity checks.
- [2026-03-30 08:59] Keep implementation out of scope until the planning-only alignment is complete.

---

## Alternatives Considered

1. Fold SDD into the repository consolidation plan.
   - Rejected: it would bury the operating model inside a much larger backlog.
2. Keep the category plans independent and treat SDD as an informal convention.
   - Rejected: that would make the governance easy to ignore.
3. Create a standalone SDD spec and plan, then point the category plans at it.
   - Preferred: centralized intent with reusable downstream plans.

---

## Assumptions And Constraints

- No product code changes are included in this workstream.
- Existing category plans remain active and are only being aligned.
- The repository already has enough planning and instruction surface to support the SDD model.
- `main` should stay clean; the planning work stays on the planning branch until a later implementation wave exists.

---

## Risks

- If the SDD umbrella becomes too broad, it could become another large planning document that is hard to execute.
- If token-efficiency is optimized too aggressively, future agents may lose necessary context.
- If instruction precedence is not explicit, the repo can drift away from the external reference or accidentally overwrite repo-owned guidance.

---

## Acceptance Criteria

- A standalone SDD umbrella spec and plan exist in `planning/specs/active/` and `planning/active/`.
- The active category plans reference the SDD baseline instead of behaving like isolated backlogs.
- The planning indexes reflect the new SDD workstream.
- No implementation files are changed as part of this planning alignment.
- The repo still preserves `super-agent`, the local operating model, and the `ntk`-native workflow.

---

## Planning Readiness

- Updated: 2026-03-30 08:59 — ready for planning-only alignment; implementation must stay out of scope until the SDD baseline is accepted.

---

## Recommended Specialist Focus

- Primary: `plan-active-work-planner`
- Supporting: `docs-release-engineer`