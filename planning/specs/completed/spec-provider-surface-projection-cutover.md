# Spec: Provider Surface Projection Cutover

Generated: 2026-04-05 12:20

## Status

- LastUpdated: 2026-04-05 10:38
- Objective: define the remaining design intent for cutting generated/runtime provider surfaces over to canonical `definitions/*` roots after the authored migration finished.
- Planning Readiness: completed-implemented
- Related Plan: `planning/completed/plan-provider-surface-projection-cutover.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Source Inputs:
  - `planning/completed/plan-instruction-taxonomy-and-path-refactor.md`
  - `planning/completed/plan-instruction-rules-board-and-surface-layout.md`
  - `planning/completed/plan-instruction-governance-and-super-agent-retention.md`
  - `definitions/providers/github/governance/provider-surface-projection.catalog.json`
  - `crates/commands/runtime/src/sync/provider_surfaces.rs`
  - `scripts/runtime/render-github-instruction-surfaces.ps1`

---

## Problem Statement

Canonical authored roots now live under `definitions/*`, but the generated/runtime folders still carry residual references to legacy lane names and older authored-source assumptions. This leaves warning noise in validation output and keeps `.github/.codex/.claude` closer to hand-edited mirrors than to pure generated outputs.

---

## Design Intent

- Keep `definitions/*` as the only authored source of truth.
- Treat `.github`, `.codex`, and `.claude` as generated/runtime consumer surfaces.
- Keep provider-owned authored overlays under `definitions/providers/*`.
- Remove legacy path assumptions from renderers, catalogs, and validation expectations in a controlled sequence.
- Preserve compatibility mirrors only while real consumers still need them.

---

## Acceptance Criteria

- Generated/runtime renderers consume canonical `definitions/*` roots by default.
- Validation warnings caused only by removed legacy lane names are eliminated or reduced to documented compatibility exceptions.
- `.github/.codex/.claude` are treated as downstream outputs, not authored source trees.
- Compatibility mirrors remain only where validator/runtime evidence still requires them.

Completion Summary:

- Runtime renderers and provider consumers now resolve canonical definitions, templates, governance catalogs, and projected GitHub surfaces from `definitions/*`.
- Generated GitHub templates now project under nested category roots, and projected prompts/chatmodes were normalized to the new relative-link model.
- Validation/test scaffolds now author and resolve governance/template evidence from canonical roots first, leaving legacy mirrors as compatibility-only output surfaces.

---

## Planning Readiness

- Implementation and cutover completed.
- Residual warning-only findings are budget related (`AGENTS.md` and `copilot-instructions.md` size ceilings), not generated-surface drift.