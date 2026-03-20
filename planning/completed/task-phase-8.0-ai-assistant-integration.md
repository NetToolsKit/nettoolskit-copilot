# Phase 8.0: AI Assistant Integration (Codex/OpenClaw Style)

**Date**: 2026-03-03
**Status**: [x] Completed (2026-03-04)
**Priority**: High
**Track**: Enterprise Expansion

## Objective

Integrate an opt-in AI assistant into the NetToolsKit CLI to support planning, explanation, command drafting, and safe automation workflows while preserving terminal stability and deterministic behavior.

## Scope

- Add AI command surface in CLI (`/ai ...`) with streaming responses.
- Support provider abstraction for OpenAI-compatible endpoints and deterministic local mock providers.
- Add workspace-aware context packing for prompts.
- Add strict approval gates for side-effecting actions.
- Persist AI session state locally only.
- Add tests, telemetry, and rollout guardrails.

## Non-Goals (Phase 8.0)

- Autonomous background execution without explicit user confirmation.
- Cloud-only persistence for prompt/response history.
- Full agentic orchestration across external systems.

## Constraints

- Local persistence only for AI conversations, settings, and approvals.
- No command execution or file mutation without explicit per-action confirmation.
- Prompt context must enforce allowlist, size budget, and secret redaction.
- Feature must degrade gracefully when provider is unavailable.
- Preserve current enterprise quality gates (`fmt`, `clippy -D warnings`, tests, audit/deny checks).

## Work Breakdown

### 8.0.1 - Provider Abstraction Layer

- Define contracts: `AiProvider`, `AiRequest`, `AiResponse`, `AiChunk`, `AiToolCall`. ✅
- Implement OpenAI-compatible adapter and deterministic `MockAiProvider`. ✅
- Add provider configuration in `AppConfig` with environment overrides.

Acceptance:
- Provider can be swapped without CLI/orchestrator call-site changes.
- Unit tests cover success, timeout, malformed response, and retry decisions.

### 8.0.2 - CLI Command Surface + UX

- Status: Completed (2026-03-04)
- [x] Add `/ai ask`, `/ai plan`, `/ai explain`, `/ai apply --dry-run`.
- [x] Integrate command palette discovery and slash completion for `/ai` subcommands.
- [x] Render streaming output using current terminal stability/runtime guards.

Acceptance:
- [x] `/ai` commands are discoverable and responsive in interactive mode.
- [x] Streaming output remains stable across resize and Ctrl+C interruption.

### 8.0.3 - Workspace Context Pipeline

- Status: Completed (2026-03-04)
- [x] Build a context collector with allowlisted sources.
- [x] Add redaction rules for secrets/tokens and deterministic truncation budget.
- [x] Add context rendering pipeline before send.

Acceptance:
- [x] Sensitive patterns are redacted by default.
- [x] Context bundle remains inside configured token/byte budget.

### 8.0.4 - Safety and Approval Gateway

- Status: Completed (2026-03-04)
- [x] Introduce explicit approval flow for tool calls and file writes.
- [x] Enforce dry-run default for mutating actions.
- [x] Record local audit trail entries for approved/denied actions.

Acceptance:
- [x] Mutating operation never executes silently.
- [x] Rejections are observable and do not crash session.

### 8.0.5 - Local Session Persistence

- Status: Completed (2026-03-04)
- [x] Persist AI sessions in local app data (`.../ntk/ai-sessions`) only.
- [x] Reuse/extend session picker for AI conversation resume (`/ai resume` in interactive mode).
- [x] Add retention and purge controls through `/config` (`ai_session_retention`).

Acceptance:
- [x] AI history survives restarts locally.
- [x] Retention/purge rules are deterministic and test covered.

### 8.0.6 - Observability, Budgets, and Resilience

- Status: Completed (2026-03-04)
- [x] Add metrics: request latency, timeout count, retries, token/cost estimate, approval ratio.
- [x] Add provider timeout + bounded retry/backoff policy.
- [x] Add rate limiting and provider health operational gauges.
- [x] Add fallback messaging when provider unavailable.

Acceptance:
- [x] Operational metrics are emitted without breaking current telemetry pipeline.
- [x] Provider outage path remains functional and user-guided.

### 8.0.7 - Validation and Release Gating

- Status: Completed (2026-03-04)
- [x] Add unit/integration tests for routing, approvals, persistence, and fallbacks.
- [x] Add E2E scenario for `/ai plan` and `/ai apply --dry-run`.
- [x] Gate release on AI-feature test suite when feature is enabled.

Acceptance:
- [x] AI feature path passes deterministic tests in CI across platforms.
- [x] No new warnings, no unsafe code, and no regression in existing phases.

## Validation Checklist

- `cargo fmt --all`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`
- `cargo test -p nettoolskit-cli -p nettoolskit-orchestrator -p nettoolskit-ui`
- Security checks: existing `cargo audit`/`cargo deny` workflows

## Risks and Mitigations

- Risk: prompt context leaks sensitive data.
  - Mitigation: allowlist collection + redaction + preview-before-send.
- Risk: model output triggers unsafe actions.
  - Mitigation: explicit approvals + dry-run-by-default + audit trail.
- Risk: latency degrades CLI UX.
  - Mitigation: streaming responses + timeout/retry budgets + cancellation support.
- Risk: provider lock-in.
  - Mitigation: provider abstraction and deterministic mock provider.

## Delivery Slices

- Slice A (POC): provider abstraction + `/ai ask` with mock provider only.
- Slice B (Incremental): real provider adapter + context collector + approvals.
- Slice C (Enterprise): persistence, telemetry budgets, E2E gates, and documentation.

## Exit Criteria

- All acceptance criteria above validated.
- Phase 8 tasks migrated from planned to completed in enterprise tracker.
- Changelog updated with AI integration capabilities and safety model.