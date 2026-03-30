# Instruction Governance And Super Agent Retention Spec

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-03-30 07:31
- Objective: define the design intent for keeping repository instructions authoritative while preserving the `super-agent` lifecycle and avoiding drift from the external `copilot-instructions` baseline.
- Normalized Request: plan how to preserve and sync the repository instruction system without losing the shared guidance that already exists in `C:\Users\tguis\copilot-instructions`.
- Active Branch: `main` (planning only; implementation branches TBD)
- Planning Path: `planning/active/plan-instruction-governance-and-super-agent-retention.md`

---

## Problem Statement

The repository already has a rich instruction and routing system, but it must stay synchronized with the external reference while preserving local ownership of the `super-agent` lifecycle and repo-operating-model rules.

---

## Design Intent

- Preserve repository-owned instruction files as the source of truth for this workspace.
- Keep the external `copilot-instructions` repository as a reference baseline, not as a live write target.
- Make routing and precedence rules explicit so the `ntk` prefix and instruction surfaces stay stable.

---

## Options Considered

1. Mirror the external instruction repo wholesale.
   - Rejected: too easy to overwrite repo-specific behavior.
2. Keep everything local with no baseline sync.
   - Rejected: drift becomes invisible and hard to correct.
3. Maintain explicit parity checks and deliberate local ownership.
   - Preferred: preserves the current system and prevents accidental loss.

---

## Proposed Boundaries

- `AGENTS.md` and `copilot-instructions.md` remain the mandatory context entry points.
- `super-agent.instructions.md` remains the workflow controller contract.
- `repository-operating-model.instructions.md` remains the repo-local source of truth for workspace behavior.

---

## Acceptance Criteria

- The repo still exposes the `super-agent` lifecycle.
- The instruction routing catalog stays aligned with the actual workspace.
- Repo-owned instruction changes can be distinguished from baseline reference drift.
- `ntk` surfaces remain documented and canonical.

---

## Planning Readiness

- The spec is planning-ready once the parity baseline and sync rules are written into the active plan.
- Implementation should be staged because instruction projections affect both docs and operator workflows.