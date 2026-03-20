# Phase 9.0: Dual Runtime Model (CLI + Background Services)

**Date**: 2026-03-04
**Status**: [x] Completed
**Priority**: High
**Track**: Post-Enterprise Expansion

## Objective

Add two official operation modes for NetToolsKit:
- **Mode A - CLI Runtime**: interactive/local-first workflow for direct engineering execution.
- **Mode B - Background Services Runtime (Docker)**: long-running AI/task manager for queued automation, orchestration, and controlled asynchronous execution.

## Scope

- Define runtime boundaries between `ntk` CLI and background service container(s).
- Keep a single domain/orchestrator core reused by both modes.
- Add task queue + worker lifecycle for background execution.
- Add AI-task manager capabilities in service mode (plan, explain, apply-dry-run orchestration pipelines).
- Add Docker packaging, local compose profile, and operational runbook.

## Non-Goals (Phase 9.0)

- Multi-node distributed scheduling in this phase.
- Cloud-only persistence.
- Unbounded autonomous execution without explicit policy controls.

## Constraints

- **Local-first persistence remains mandatory** by default.
- Service mode must be runnable locally through Docker without mandatory external cloud dependencies.
- No unsafe side effects without explicit approval/policy gating.
- Preserve existing quality gates (`fmt`, `clippy -D warnings`, tests, audit/deny checks).

## Work Breakdown

### 9.0.1 - Runtime Contracts and Architecture

- Status: Completed (2026-03-04)
- [x] Define shared contracts for command/task intents, execution status, and audit events.
- [x] Define `RuntimeMode` (`cli`, `service`) with deterministic config/env selection.
- [x] Publish architecture decision in planning docs.

Acceptance:
- [x] CLI and service runtimes consume the same orchestrator contracts.
- [x] Runtime mode selection is deterministic and test covered.

### 9.0.2 - CLI Runtime Path (Mode A)

- Status: Completed (2026-03-04)
- [x] Keep current CLI interactive and non-interactive flows as first-class mode.
- [x] Add explicit command surface for service interaction (`/task submit`, `/task list`, `/task watch`, `/task cancel`).
- [x] Preserve local fallback when service endpoint is unavailable.

Acceptance:
- [x] CLI remains fully functional standalone.
- [x] Service integration commands degrade gracefully.

### 9.0.3 - Background Services Runtime (Mode B)

- Status: Completed (2026-03-04)
- [x] Add background worker runtime module for queued tasks (embedded worker in orchestrator).
- [x] Add task state machine (`queued`, `running`, `succeeded`, `failed`, `cancelled`).
- [x] Add bounded concurrency, retry policy, and cancellation support.
- [x] Extract worker runtime to dedicated crate (optional refactor).

Acceptance:
- [x] Background service executes queued tasks deterministically.
- [x] Task lifecycle is queryable from CLI/API with consistent status transitions.

### 9.0.4 - AI Task Manager in Service Mode

- Status: Completed (2026-03-04)
- [x] Add AI task orchestration handlers (plan/explain/apply-dry-run jobs) through `/task submit` intent routing in service mode.
- [x] Reuse existing AI safety controls (approval, rate limit, retries, observability) by delegating execution to existing `/ai` command pipeline.
- [x] Add policy profile for service automation limits (budgets, allowed actions).

Acceptance:
- [x] AI tasks can be submitted and tracked without bypassing safety controls.
- [x] Dry-run semantics are preserved for mutating intents.

### 9.0.5 - Docker Packaging and Local Operations

- Status: Completed (2026-03-04)
- [x] Add Dockerfile for service runtime and compose profile for local stack.
- [x] Add environment template and startup health checks.
- [x] Publish runbook for local boot, smoke checks, and troubleshooting.

Acceptance:
- [x] `docker compose up` starts service mode locally with documented defaults.
- [x] Health checks and logs provide actionable diagnostics.

### 9.0.6 - Validation and Release Gating

- Status: Completed (2026-03-04)
- [x] Add unit/integration/E2E coverage for dual runtime paths.
- [x] Add CI gate for service-mode tests and container smoke validation.
- [x] Keep existing CLI gates unchanged and green.

Acceptance:
- [x] Dual-mode paths pass deterministic CI validation.
- [x] No warnings, no unsafe code, no regressions in existing phases.

### 9.0.7 - Autonomous ChatOps Agent (Telegram/Discord) on VPS

- Status: Completed (2026-03-05)
- [x] Add platform-neutral ChatOps contracts (`ingress`, `notifier`, parser, auth policy, local audit store) and deterministic mock adapters for testability.
- [x] Add VPS operational profile runbook with secure defaults and local-persistence requirements.
- [x] Add ChatOps ingress adapters for Telegram and Discord command channels.
- [x] Add outbound notifier adapters for Telegram and Discord task status updates.
- [x] Add Telegram webhook ingress mode as an alternative to polling, wired to service endpoint `/chatops/telegram/webhook`.
- [x] Add Discord interaction ingress mode as an alternative to channel polling, wired to service endpoint `/chatops/discord/interactions`.
- [x] Add optional ingress security controls for internet-exposed endpoints (Telegram secret token + Discord request signature validation + bounded replay protection).
- [x] Add reverse-proxy reference profiles for signed ChatOps ingress on VPS (Nginx + Caddy).
- [x] Add repository execution workflow (`clone -> branch -> execute -> commit/push/PR`) with policy gating.
- [x] Add scoped command authorization and rate limits for remote agent commands.
- [x] Add burst-aware throttling strategy for ChatOps ingress (`fixed_window` and `token_bucket` with optional burst budgets).
- [x] Add adaptive auto-tuning profile for ChatOps throttling strategy based on observed ingress traffic.
- [x] Add replay-cache backend options (`memory` + `file`) with shared file-mode support for multi-process service replicas.
- [x] Add VPS deployment profile with secure defaults (firewall/reverse proxy/secret management).
- [x] Add end-to-end VPS smoke profile for ChatOps flow in CI/release verification.

Acceptance:
- [x] Local ChatOps execution foundation is available with deterministic tests and auditable local persistence.
- [x] User can submit approved tasks through Telegram/Discord and receive deterministic execution status (polling mode + Telegram webhook mode + Discord interaction mode via env configuration).
- [x] Internet-exposed webhook/interaction endpoints can be protected with signature/origin validation and replay detection.
- [x] Reference proxy profiles are available for secure VPS exposure with required signature/token header preservation.
- [x] Agent execution remains policy-bounded with local-first persistence and auditable trails.

### 9.0.8 - OpenClaw-Inspired Hardening (Security + Performance + Token Economy)

- Status: Completed (2026-03-05)
- Source baseline: `https://github.com/openclaw/openclaw` (multi-provider agent runtime, local/cloud/hybrid execution, ChatOps integrations, usage tracking/session pruning, secure-default operations).
- [x] Add provider-routing policy with deterministic fallback chain (`primary -> secondary`) and per-provider latency/timeout budgets.
- [x] Add token-economy policy engine (hard token budget per request/session, prompt compaction, context truncation tiers, cache-first response reuse).
- [x] Add secure tool-execution gateway for service mode (allowlisted tools, per-intent permission scopes, secret redaction at ingress/egress).
- [x] Add adaptive model-selection heuristics (cheap model for classify/route, stronger model for plan/explain) with measurable cost caps.
- [x] Add response-stream persistence compression (store deltas/summaries instead of full raw streams when configured) to reduce storage + replay token cost.
- [x] Add production SLO bundle for agent mode (`p95 latency`, `task success`, `tokens/task`, `cost/task`) with CI regression budget checks.

Acceptance:
- [x] Service-mode AI flows respect configured token/cost budgets and fail closed when limits are exceeded.
- [x] Security controls enforce deny-by-default tool permissions with auditable decisions.
- [x] Performance telemetry exposes SLO budget indicators (`p95`, success ratio, tokens/task, cost/task) with deterministic CI regression checks.

## Validation Checklist

- `cargo fmt --all`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`
- Runtime mode tests:
  - `cargo test -p nettoolskit-core runtime_mode_`
  - `cargo test -p nettoolskit-orchestrator --test test_suite test_process_task_`
  - `cargo test -p nettoolskit-cli --bin ntk service_mode_`
- Container smoke:
  - `docker compose -f deployments/docker-compose.local.yml up --build -d`
  - service health endpoint check + queued task smoke run

## Risks and Mitigations

- Risk: duplicated logic between CLI and service runtimes.
  - Mitigation: shared orchestrator contracts and execution pipeline.
- Risk: background mode introduces hidden side effects.
  - Mitigation: explicit policy/approval gates and dry-run defaults.
- Risk: Docker operational complexity for developers.
  - Mitigation: local compose profile + documented runbook + health checks.
- Risk: AI workload saturation.
  - Mitigation: bounded queue, concurrency caps, retry budget, rate limits, and telemetry alerts.

## Delivery Slices

- Slice A (POC): runtime contracts + minimal service worker + CLI submit/list.
- Slice B (Incremental): AI task manager flow + policy controls + Docker local profile.
- Slice C (Production): full E2E gates, observability dashboards, and operational runbook.

## Exit Criteria

- Both modes (`cli` and `service`) are officially documented and validated.
- Service mode runs locally with Docker and exposes deterministic task lifecycle.
- Planning trackers and changelog reflect dual-runtime delivery status.
