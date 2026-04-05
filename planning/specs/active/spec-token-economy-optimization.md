# Token Economy Optimization Spec

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-04-05 14:05
- Objective: define the design intent for lowering AI token burn across routing, compaction, reuse, and output budgeting without breaking the existing `ntk` AI surfaces.
- Normalized Request: plan a token-economy workstream that uses the existing usage ledger and runtime policies to reduce spend.
- Active Branch: `main` (planning only; implementation branches TBD)
- Planning Path: `planning/active/plan-token-economy-optimization.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already measures AI usage and enforces some token budgets, but the current policy is still conservative and not tightly optimized around intent. Low-risk prompts can often be served with smaller outputs, smaller models, or more aggressive compaction.

---

## Design Intent

- Keep the AI usage ledger as the measurement source.
- Reduce spend first through deterministic policy, not heuristics hidden inside prompt text.
- Preserve correctness for planning, orchestration, and complex reasoning requests.
- Make token economy visible enough that regressions can be measured per command surface.

---

## Options Considered

1. Keep current budgets and only report usage.
   - Rejected: this measures waste but does not reduce it.
2. Apply global token caps everywhere.
   - Rejected: too blunt and likely to damage complex tasks.
3. Route by intent, compact prompts by risk, and reuse cache aggressively.
   - Preferred: gives measurable savings while preserving correctness by command class.

---

## Proposed Boundaries

- `processor.rs` owns policy decisions.
- `ai_session.rs` owns replay compression and session persistence shaping.
- `ai_commands.rs` only exposes reporting and user-facing budget controls.
- The weekly usage ledger remains the feedback loop.

---

## Acceptance Criteria

- Simple requests prefer cheaper models by default.
- Repeatable prompts reuse cached results when safe.
- Low-risk session replay is smaller than the current baseline.
- Budget warnings are visible from usage reports.
- Tests cover at least one command path for each policy change.

---

## Planning Readiness

- The design is ready once the current token-policy defaults are captured in the active plan.
- The next implementation branch should be isolated if the policy changes touch multiple AI flows at once.
- A first routing-alignment proof now exists outside this plan: provider-order strategy is explicit and inspectable, so this workstream can stay focused on token/cost policy and compaction rather than route ordering.
- A second policy-alignment proof now exists outside this plan: canonical agent/skill model-routing defaults can bias profile and model selection, but the final request still flows through the same token-economy guardrails and explicit `NTK_AI_MODEL_SELECTION_*` overrides.