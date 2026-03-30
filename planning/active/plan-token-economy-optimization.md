# Token Economy Optimization Plan

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-03-30 07:31
- Objective: reduce AI token, latency, and cost waste by tightening model routing, prompt compaction, session replay, and cache reuse across the `ntk` AI surfaces.
- Normalized Request: create a planning workstream for token economy so the repository can actively reduce token burn instead of only measuring it.
- Active Branch: `main` (planning only; implementation branches TBD)
- Spec Path: `planning/specs/active/spec-token-economy-optimization.md`
- Inputs:
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/orchestrator/src/execution/ai.rs`
  - `crates/orchestrator/src/execution/ai_session.rs`
  - `crates/orchestrator/src/execution/ai_usage.rs`
  - `crates/cli/src/ai_commands.rs`
  - `planning/completed/plan-ai-usage-history-and-sqlite-local-memory.md`
  - `planning/completed/enterprise-progress-tracker.md`

---

## Scope Summary

This plan coordinates four optimization slices:

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| T1 | Spend visibility and budget guardrails | orchestrator + CLI + local usage ledger | 🔴 Immediate | existing weekly usage ledger |
| T2 | Prompt compaction and context trimming | orchestrator + AI session replay | 🔴 Immediate | none |
| T3 | Adaptive model routing by intent | orchestrator + CLI policy surfaces | 🟠 High | T1 |
| T4 | Cache-first reuse and output caps | processor + tests | 🟠 High | T2 |

This workstream does not replace the usage-history ledger. It consumes that ledger to reduce future token burn.

---

## Ordered Tasks

### [2026-03-30 07:31] Task T1: Freeze Token Economy Baseline

- Audit current policy points in `crates/orchestrator/src/execution/processor.rs` and `crates/orchestrator/src/execution/ai_session.rs`.
- Capture the current defaults for:
  - request token caps
  - session token caps
  - cache-first reuse
  - prompt compaction tiers
  - weekly burn visibility from the AI usage ledger
- Record the baseline in this plan before changing policy behavior.
- Commit checkpoint:
  - `docs(planning): freeze token economy optimization baseline`

### [2026-03-30 07:31] Task T2: Tighten Prompt Compaction and Replay

- Define intent-specific compaction rules for low-risk AI requests.
- Reduce replay size for resumed sessions and long-running conversations.
- Add or expand tests for compaction tiers, truncation boundaries, and replay stability.
- Target paths:
  - `crates/orchestrator/src/execution/ai_session.rs`
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/orchestrator/tests/execution/*`
- Commit checkpoint:
  - `feat(ai): tighten prompt compaction and session replay`

### [2026-03-30 07:31] Task T3: Route Cheap Requests to Smaller Models

- Formalize intent-based routing for cheap, intermediate, and reasoning paths.
- Keep deterministic fallbacks when higher-capability models are unavailable.
- Validate that simple queries do not pay the reasoning-model cost by default.
- Target paths:
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/cli/src/ai_commands.rs`
  - `crates/orchestrator/tests/execution/ai_usage_tests.rs`
- Commit checkpoint:
  - `feat(ai): add intent-aware model routing policy`

### [2026-03-30 07:31] Task T4: Enforce Cache-First and Output Budgets

- Keep cache-first reuse enabled for repeatable prompts and low-entropy flows.
- Tighten `max_output_tokens` defaults per command surface where the current baseline is too wide.
- Validate budget rejection and reuse behavior with deterministic tests.
- Target paths:
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/cli/tests/ai_commands_tests.rs`
  - `crates/orchestrator/tests/execution/ai_usage_tests.rs`
- Commit checkpoint:
  - `fix(ai): enforce cache-first reuse and output budgets`

---

## Validation Checklist

- `cargo test -p nettoolskit-orchestrator --test test_suite ai_usage --quiet`
- `cargo test -p nettoolskit-cli --test ai_commands_tests --quiet`
- `cargo test -p nettoolskit-cli --quiet`
- `cargo clippy -p nettoolskit-orchestrator --all-targets -- -D warnings`
- `git diff --check`

---

## Risks And Mitigations

- Over-aggressive compaction can hide context needed for correctness.
- Smaller model routing can regress quality on ambiguous tasks if intent classification is too broad.
- Cache-first reuse must stay deterministic to avoid stale or wrong answers.
- Mitigation: keep the existing weekly usage ledger as the measurement baseline and only tighten one policy surface per slice.

---

## Specialist And Closeout

- Recommended specialist: `dev-rust-engineer`
- Tester: required
- Reviewer: required
- Release closeout: required
- README update: likely needed for user-visible policy changes
- Changelog: required once implementation lands
- Suggested commit message style:
  - `feat(ai): tighten token economy policy and routing`
  - `docs(planning): record token economy optimization roadmap`