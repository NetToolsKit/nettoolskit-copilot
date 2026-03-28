# NetToolsKit CLI - Enterprise Progress Tracker

> **Created:** July/2025
> **Baseline:** Deep analysis report + terminal resize hardening sessions
> **Binary:** `ntk` | **Crates:** 11 | **LOC:** ~7,300
> **Status:** Phases 0 through 10 delivered; optimization backlog remains open

---

## Legend

- `[x]` Completed
- `[-]` In progress
- `[ ]` Pending
- `[~]` Partial / workaround applied
- `[!]` Blocked

---

## Phase 0 - Critical Fixes (P0)

> **Goal:** Remove undefined behavior and establish non-negotiable quality/security gates.
> **Estimated timeline:** 1 sprint (5-7 business days)

### 0.1 Undefined Behavior Removal

| # | Task | File | Status |
|---|------|------|--------|
| 0.1.1 | Replace unsafe cast in `TemplateEngine` with safe shared synchronization | `templating/src/rendering/engine.rs` | `[x]` |
| 0.1.2 | Add multi-thread stress test for template registration | `templating/tests/` | `[x]` |
| 0.1.3 | Validate `#![forbid(unsafe_code)]` across all crates | workspace-wide | `[x]` |

### 0.2 Dependency Alignment

| # | Task | File | Status |
|---|------|------|--------|
| 0.2.1 | Upgrade `handlebars` to `6.2` in workspace | root `Cargo.toml` | `[x]` |
| 0.2.2 | Align `strum`/`strum_macros` and `thiserror` versions | root `Cargo.toml` | `[x]` |
| 0.2.3 | Move multiple crates to `{ workspace = true }` dependency references | multiple crates | `[x]` |
| 0.2.4 | Run full build and test validation | workspace-wide | `[x]` |

### 0.3 Timer Double-Record Fix

| # | Task | File | Status |
|---|------|------|--------|
| 0.3.1 | Add `stopped: bool` guard in timer lifecycle | `otel/src/telemetry.rs` | `[x]` |
| 0.3.2 | Protect `Drop` path against duplicate record when `stop()` already executed | `otel/src/telemetry.rs` | `[x]` |
| 0.3.3 | Add tests for stop/drop/idempotency paths | `otel/tests/telemetry_tests.rs` | `[x]` |

---

## Phase 1 - Terminal Runtime Stability (P1)

> **Goal:** Robust terminal behavior with clean resize handling, correct exit behavior, and deterministic rendering.
> **Estimated timeline:** 1 sprint (5-7 business days)
> **Dependency:** Phase 0 completed

### 1.1 Resize Debounce and Coalescing

| # | Task | File | Status |
|---|------|------|--------|
| 1.1.1 | Implement trailing-edge resize debounce with shared pending state | `ui/src/interaction/terminal.rs` | `[x]` |
| 1.1.2 | Make `handle_resize()` mark state only | `ui/src/interaction/terminal.rs` | `[x]` |
| 1.1.3 | Process delayed resize in poll timeout branch | `ui/src/interaction/terminal.rs` + `cli/src/input.rs` | `[x]` |

### 1.2 Atomic Reconfigure Rendering

| # | Task | File | Status |
|---|------|------|--------|
| 1.2.1 | Add `RECONFIGURING` guard to avoid concurrent footer redraw | `ui/src/interaction/terminal.rs` | `[x]` |
| 1.2.2 | Hide/show cursor safely around reconfigure | `ui/src/interaction/terminal.rs` | `[x]` |
| 1.2.3 | Preserve lock consistency while updating runtime metrics + footer | `ui/src/interaction/terminal.rs` | `[x]` |

### 1.3 Exit and History Preservation

| # | Task | File | Status |
|---|------|------|--------|
| 1.3.1 | Keep terminal history on `/quit` and `Ctrl+C` (no destructive clear) | `cli/src/lib.rs` | `[x]` |
| 1.3.2 | Ensure cursor restoration and final terminal state cleanup | `ui/src/interaction/terminal.rs` | `[x]` |
| 1.3.3 | Keep cursor visible/blinking at prompt after reconfigure | UI + CLI integration | `[x]` |

### 1.4 Width, Narrow-Terminal, and UX Guideline

| # | Task | File | Status |
|---|------|------|--------|
| 1.4.1 | Remove critical fixed width usages (`with_width(89)`) | UI + manifest flows | `[x]` |
| 1.4.2 | Add narrow-terminal fallback behavior (<80 columns) | UI rendering/palette | `[x]` |
| 1.4.3 | Publish official TUI UX guideline | `docs/ui/tui-ux-guidelines.md` | `[x]` |

---

## Phase 2 - Feature Completeness (P2)

> **Goal:** Replace all placeholders with real implementations and contract-tested behavior.
> **Estimated timeline:** 2 sprints (10-14 business days)
> **Dependency:** Phase 1 completed

### 2.1 Manifest Runtime Flows

| # | Task | File | Status |
|---|------|------|--------|
| 2.1.1 | Implement real schema/semantic validation in `manifest check` | `manifest/src/handlers/check.rs` | `[x]` |
| 2.1.2 | Implement real render preview (`manifest render`) | manifest execution/rendering | `[x]` |
| 2.1.3 | Integrate interactive `manifest apply` in orchestrator | orchestrator + manifest UI | `[x]` |
| 2.1.4 | Cover success/error E2E for list/check/render/apply | CLI + manifest tests | `[x]` |

### 2.2 Translate and Text Routing

| # | Task | File | Status |
|---|------|------|--------|
| 2.2.1 | Replace `/translate` placeholder with real handler routing | orchestrator translate flow | `[x]` |
| 2.2.2 | Add argument parser and validations (`--from`, `--to`, `path`) | orchestrator + translate | `[x]` |
| 2.2.3 | Route free text into real command intents | `orchestrator/src/execution/processor.rs` | `[x]` |
| 2.2.4 | Add unit/integration tests for free-text routing and translate paths | orchestrator tests | `[x]` |

### 2.3 Product Integrity Enhancements

| # | Task | File | Status |
|---|------|------|--------|
| 2.3.1 | Expand translate language support and behavior checks | translate handlers/tests | `[x]` |
| 2.3.2 | Enforce deterministic command behavior in non-interactive mode | orchestrator/CLI | `[x]` |
| 2.3.3 | Keep beta limits explicit for unsupported target classes | docs + interactive messaging | `[x]` |

---

## Phase 3 - Security and Supply Chain (P1-P2)

> **Goal:** Harden dependency and release supply-chain security.
> **Estimated timeline:** 1 sprint (5-7 business days)

| # | Task | File | Status |
|---|------|------|--------|
| 3.1 | Remediate `RUSTSEC-2026-0007` (`bytes >= 1.11.1`) | lockfile/dependencies | `[x]` |
| 3.2 | Address dependency maintenance alerts (`fxhash`, `paste`, `lru`) | workspace dependency policy | `[x]` |
| 3.3 | Add `cargo-deny` policy and CI checks | `deny.toml`, CI | `[x]` |
| 3.4 | Generate signed SBOMs (CycloneDX + SPDX) in release pipeline | `release.yml` | `[x]` |

---

## Phase 4 - Observability and Operations (P2-P3)

> **Goal:** Production-grade telemetry and operational readiness.
> **Estimated timeline:** 1 sprint (5-7 business days)

| # | Task | File | Status |
|---|------|------|--------|
| 4.1 | Add correlation IDs per session/command | CLI + orchestrator + otel | `[x]` |
| 4.2 | Build runtime metrics catalog (latency, error, cancellation, text routing) | orchestrator + otel | `[x]` |
| 4.3 | Enable optional OTLP export for traces and metrics with safe shutdown | `otel/src/tracing_setup.rs`, `telemetry.rs` | `[x]` |
| 4.4 | Publish incident/troubleshooting playbook | `docs/operations/incident-response-playbook.md` | `[x]` |

---

## Phase 5 - CI/CD and Release Engineering (P3)

> **Goal:** Deterministic, secure, and verifiable release lifecycle.
> **Estimated timeline:** 1 sprint (5-7 business days)

| # | Task | File | Status |
|---|------|------|--------|
| 5.1 | Enforce full CI gates (`fmt`, `clippy`, `build`, `test`, `deny`, audit) | CI workflows | `[x]` |
| 5.2 | Implement deterministic release notes from exact changelog section | `release.yml` | `[x]` |
| 5.3 | Add release smoke tests on Linux/Windows/macOS | `release.yml` | `[x]` |
| 5.4 | Add keyless `cosign` artifact signing (`.sig`, `.pem`) | `release.yml` | `[x]` |
| 5.5 | Add compatibility/support policy enforcement gates | `COMPATIBILITY.md`, release gates | `[x]` |

---

## Phase 6 - Configuration and Runtime Behavior (P3)

> **Goal:** User-facing configuration and graceful runtime adaptation.

| # | Task | File | Status |
|---|------|------|--------|
| 6.1 | Implement user config schema, loader precedence, and `/config` command | `core/src/config.rs`, orchestrator | `[x]` |
| 6.2 | Add capability detection and fallback (color/unicode/plain text) | UI + core | `[x]` |
| 6.3 | Ensure terminal behavior remains stable under constraints | CLI + UI runtime | `[x]` |

---

## Phase 7 - Documentation and Product Polish (P3)

> **Goal:** Enterprise-grade documentation and release references.

| # | Task | File | Status |
|---|------|------|--------|
| 7.1.1 | Update README with status, references, and quality badges | `README.md` | `[x]` |
| 7.1.2 | Update CONTRIBUTING with setup and workflow guidance | `CONTRIBUTING.md` | `[x]` |
| 7.1.3 | Keep `cargo doc` warning-free | workspace | `[x]` |
| 7.1.4 | Expand public API documentation comments | workspace | `[x]` |
| 7.1.5 | Consolidate architecture decisions into `CHANGELOG.md` | `CHANGELOG.md` | `[x]` |
| 7.2.1 | Adopt Keep a Changelog format | `CHANGELOG.md` | `[x]` |
| 7.2.2 | Keep session changes recorded in `Unreleased` | `CHANGELOG.md` | `[x]` |

---

## Phase 8 - AI Assistant Integration (P2-P3)

> **Goal:** Integrate an opt-in AI assistant in the CLI (Codex/OpenClaw style) with safe execution controls and local-only persistence.
> **Estimated timeline:** 2 sprints (10-14 business days)
> **Dependency:** Phases 2, 4, and 6 completed

| # | Task | File | Status |
|---|------|------|--------|
| 8.1 | Define provider abstraction (`AiProvider`, request/response, streaming chunks) with deterministic mock provider | orchestrator + core | `[x]` |
| 8.1.1 | Implement OpenAI-compatible provider adapter (endpoint + API key + timeout + response parsing) | orchestrator + core | `[x]` |
| 8.2 | Add `/ai` command family (`ask`, `plan`, `explain`, `apply --dry-run`) and command palette entry points | CLI + orchestrator + UI | `[x]` |
| 8.3 | Build workspace context collector with explicit allowlist, size budget, and secret redaction | core + orchestrator | `[x]` |
| 8.4 | Add approval gateway for command execution and file writes (no implicit side effects) | CLI + orchestrator | `[x]` |
| 8.5 | Implement local-only AI session persistence, resume picker integration, and retention policy controls | `orchestrator/src/execution/ai_session.rs` + `cli/src/lib.rs` + `core/src/config.rs` | `[x]` |
| 8.6 | Add operational controls (timeouts, retry/backoff, rate limiting, token/cost counters, provider health metrics) | orchestrator + otel | `[x]` |
| 8.7 | Add unit/integration/E2E tests for AI routing, tool safety, and fallback behavior | workspace tests | `[x]` |

---

## Phase 9 - Dual Runtime Operations (P3-P4)

> **Goal:** Support two official runtime modes: local-first CLI mode and background service mode (Docker) acting as an AI/task manager.
> **Estimated timeline:** 2-3 sprints (10-21 business days)
> **Dependency:** Phases 6 and 8 completed

| # | Task | File | Status |
|---|------|------|--------|
| 9.1 | Define dual runtime contracts (`RuntimeMode`, task intent/status model, shared orchestration boundaries) | orchestrator + core + docs | `[x]` |
| 9.2 | Add CLI service-control command family (`/task submit`, `/task list`, `/task watch`, `/task cancel`) with local fallback behavior | CLI + orchestrator | `[x]` |
| 9.3 | Implement background worker runtime with queued task lifecycle and bounded concurrency/retry/cancel policies | new runtime crate/module + orchestrator | `[x]` |
| 9.4 | Integrate AI task manager flows in service mode (plan/explain/apply-dry-run orchestration with existing safety controls) | orchestrator + AI modules | `[x]` |
| 9.5 | Add Docker packaging + compose local profile + service health checks + runbook | `deployments/` + docs + CI smoke | `[x]` |
| 9.6 | Add dual-mode validation/release gates (runtime-mode integration tests + container smoke checks) | CI workflows + workspace tests | `[x]` |
| 9.7 | Add autonomous ChatOps agent model (Telegram/Discord command ingress + notifier + VPS profile) | runtime + adapters + docs | `[x]` |
| 9.8 | Add OpenClaw-inspired security/performance/token-economy hardening profile for service-mode agent runtime | orchestrator + core + otel + docs | `[x]` |

---

## Phase 10 - Commercial Platform Hardening (P0-P2)

> **Goal:** Close the remaining gaps between a strong local-first engineering agent and a commercially defensible developer platform.
> **Estimated timeline:** 2-3 sprints (10-21 business days)
> **Dependency:** Phases 5, 8, and 9 completed

| # | Task | File | Status |
|---|------|------|--------|
| 10.1 | Harden service control plane with loopback-by-default bind, bearer auth for mutating API paths, and fail-closed non-loopback startup policy | `crates/cli/src/main.rs` + ops docs + CI/docker assets | `[x]` |
| 10.2 | Separate readiness from liveness with dependency-aware checks (worker, audit store, replay backend, ChatOps runtime state) | `crates/cli/src/main.rs` + orchestrator + runbooks | `[x]` |
| 10.3 | Enforce critical-file coverage budgets for public entrypoints (`cli/main.rs`, `manifest/ui/menu.rs`, `otel/tracing_setup.rs`, `processor.rs`) | CI + tests + coverage policy | `[x]` |
| 10.4 | Add commercial OSS governance baseline (`LICENSE`, `SECURITY.md`, `CODEOWNERS`, disclosure policy) | repo root + `.github/` | `[x]` |
| 10.5 | Replace ad-hoc service HTTP transport with framework-grade stack (`axum`/`hyper`) including middleware for auth, request IDs, limits, and tracing | service runtime | `[x]` |
| 10.6 | Publish formal control-plane/session/operator model aligned with Codex/OpenClaw reference capabilities | `planning/active/` + docs | `[x]` |

---

## Continuous Backlog (Post-Enterprise)

| # | Task | Status |
|---|------|--------|
| B1 | Publish signed artifact verification runbook | `[x]` |
| B2 | Add manual release verification workflow (`workflow_dispatch`) | `[x]` |
| B3 | Add formal minor support expiration + EOL table | `[x]` |
| B4 | Expand manual verification to signed SBOM checks | `[x]` |
| B5 | Add semantic EOL lifecycle gate in release pipeline | `[x]` |
| B64 | Materialize typed control-plane contracts in `nettoolskit-core` for future ingress adoption | `[x]` |
| B65 | Adopt typed control-plane metadata in service HTTP ingress for `/task/submit` | `[x]` |
| B66 | Adopt typed control-plane metadata in ChatOps `submit` ingress and persist normalized audit fields | `[x]` |
| B67 | Adopt typed control-plane metadata in local CLI `/task submit` and persist normalized task audit fields | `[x]` |
| B68 | Extend typed control-plane attribution to ChatOps `help/list/watch/cancel` without regressing existing command handlers | `[x]` |
| B6 | Align lifecycle validator flow to shared external script pattern | `[x]` |
| B7 | Remove legacy aliases and move to canonical modern APIs | `[x]` |
| B8 | Optimize manifest discovery with heavy-directory pruning and deterministic deduplication | `[x]` |
| B9 | Deliver enhanced CLI input (rustyline history/autocomplete) and coverage graph export pipeline | `[x]` |
| B10 | Add multiline input editing support in CLI (`rustyline` continuation validator) | `[x]` |
| B11 | Add interactive manifest file picker with fuzzy/regex filtering and manual path fallback | `[x]` |
| B12 | Add interactive status bar with mode indicators, notifications queue, and runtime usage summary | `[x]` |
| B13 | Add interactive history viewer with pagination and search/filter, integrated through local `/history` command | `[x]` |
| B14 | Add interactive input syntax highlighting with command flags plus Rust/C#/JS/TS lexical support | `[x]` |
| B15 | Upgrade syntax highlighting with tree-sitter parsing and cached fast-path for interactive input performance | `[x]` |
| B16 | Implement baseline translation execution for `clojure` and `typescript` targets (remove `not yet implemented` failure path) | `[x]` |
| B17 | Implement Markdown renderer for terminal output and integrate `/help` with structured Markdown content | `[x]` |
| B18 | Add clipboard integration shortcuts (`Ctrl/Alt+V`, `Alt+C`) with status notifications and error-safe tests | `[x]` |
| B19 | Add configurable interactive attention bell on command failure/interruption (`attention_bell`) with `/config` integration | `[x]` |
| B20 | Add cross-platform desktop attention notifications with configurable runtime toggle (`attention_desktop_notification`) | `[x]` |
| B21 | Reintroduce async command aliases (`/new-async`, `/render-async`, `/apply-async`) with structured progress updates on top of `AsyncCommandExecutor` | `[x]` |
| B22 | Enforce cooperative Ctrl+C cancellation for async manifest aliases and propagate interruption state from CLI loops | `[x]` |
| B23 | Introduce runtime command cache in orchestrator with LRU ordering, TTL by command type, and memory-bounded eviction | `[x]` |
| B24 | Add dedicated Criterion benchmarks for runtime cache hit/miss/insert/eviction paths and capture baseline timings | `[x]` |
| B25 | Add predictive interactive input hints for slash-command prefixes in `rustyline` with deterministic candidate ranking | `[x]` |
| B26 | Extend runtime configuration to control predictive hints (`predictive_input`) through config file/env and `/config` command | `[x]` |
| B27 | Add plugin foundation in orchestrator (registry + before/after command hooks + runtime plugin metrics) | `[x]` |
| B28 | Add bounded interactive error recovery (input retry budget + panic-safe command/text task execution) to keep sessions alive under transient failures | `[x]` |
| B29 | Implement rich shared CLI state foundation (`CliState`, typed history entries, Arc/RwLock sharing, JSON round-trip) for session persistence roadmap | `[x]` |
| B30 | Implement local-only session persistence (JSON snapshot save/load/prune, auto-resume latest, and history seeding in interactive loops) | `[x]` |
| B31 | Add startup local session resume picker (menu-based selection when multiple snapshots exist) with safe fallback to latest snapshot | `[x]` |
| B32 | Implement frame scheduler runtime (coalesced frame requests, 60 FPS limiter, async poll timeout adaptation, and event-loop wiring) | `[x]` |
| B33 | Deliver Markdown fenced code block highlighting with language-aware lexical styling and ANSI-safe test normalization | `[x]` |
| B34 | Publish Phase 8 AI assistant integration plan with acceptance criteria, risks, and rollout slices | `[x]` |
| B35 | Deliver Phase 8.1 provider abstraction spike (`AiProvider` + mock provider) with deterministic tests | `[x]` |
| B36 | Implement OpenAI-compatible provider adapter with configurable endpoint/API key and deterministic transport fallback tests | `[x]` |
| B37 | Integrate `/ai` command surface (`ask`, `plan`, `explain`, `apply --dry-run`) into CLI/orchestrator with streaming output | `[x]` |
| B38 | Implement workspace context collector for AI prompts with explicit allowlist, byte budgets, and secret redaction | `[x]` |
| B39 | Add approval gateway + local audit trail for AI side-effect intents (`command_execution`, `file_write`) with explicit non-dry-run confirmation | `[x]` |
| B40 | Deliver local AI session persistence + `/ai resume` picker integration + retention controls (`ai_session_retention`) with deterministic tests | `[x]` |
| B41 | Publish formal Phase 9 dual-runtime technical plan (`CLI` + `service`) with acceptance criteria and rollout slices | `[x]` |
| B42 | Define local-first Docker operational baseline for background AI/task manager mode (compose profile + runbook) | `[x]` |
| B43 | Define autonomous ChatOps agent scope for Telegram/Discord command ingress and delivery notifications | `[x]` |
| B44 | Deliver ChatOps execution foundation (`ingress/notifier` contracts, parser/auth policy, local audit store, deterministic mock adapters) | `[x]` |
| B45 | Deliver ChatOps runtime wiring (`ntk service` polling loop + Telegram/Discord async adapters + env-driven startup policy) | `[x]` |
| B46 | Deliver policy-gated repository workflow intent (`repo-workflow`) for `clone -> branch -> execute -> commit/push/PR` in task runtime | `[x]` |
| B47 | Deliver scoped ChatOps authorization + per-user/per-channel rate limiting budgets for remote agent commands | `[x]` |
| B48 | Deliver ChatOps VPS smoke profile in CI/release verification (deterministic mock ingress + packaged service health smoke) | `[x]` |
| B49 | Deliver service automation policy profile (`strict/balanced/open`) with allowed-action and budget gates for service-mode task admission | `[x]` |
| B50 | Extract background worker runtime to dedicated crate (`nettoolskit-task-worker`) and integrate orchestrator via callback hooks | `[x]` |
| B51 | Deliver Telegram webhook ingress mode for ChatOps (`/chatops/telegram/webhook`) with local queue ingestion and service endpoint tests | `[x]` |
| B52 | Deliver Discord interaction ingress mode for ChatOps (`/chatops/discord/interactions`) with local queue ingestion and service endpoint tests | `[x]` |
| B53 | Deliver burst-aware ChatOps throttling strategy (`token_bucket` + optional burst budgets) while preserving fixed-window default | `[x]` |
| B54 | Deliver ingress hardening for ChatOps endpoints (Telegram secret token + Discord request signature validation + replay protection window) | `[x]` |
| B55 | Deliver reverse-proxy reference profiles (Nginx/Caddy) for signed ChatOps ingress on VPS with endpoint/path hardening guidance | `[x]` |
| B56 | Deliver adaptive ChatOps rate-limit auto-tuning profile based on observed ingress traffic (`NTK_CHATOPS_RATE_LIMIT_AUTOTUNE_PROFILE`) | `[x]` |
| B57 | Deliver replay-cache backend options (`memory`/`file`) with shared file-backed mode for multi-process service replicas | `[x]` |
| B58 | Add provider routing and deterministic fallback chain with latency/timeout budgets for service-mode AI runtime | `[x]` |
| B59 | Add token-economy policy (token/cost caps, prompt compaction tiers, cache-first reuse) for `/ai` and `/task submit ai-*` flows | `[x]` |
| B60 | Add secure tool-execution scope model (allowlisted tools, intent-level permissions, deny-by-default) with local audit proofs | `[x]` |
| B61 | Add adaptive model-selection policy (cheap model for classify/route, premium model for reasoning) with cost guardrails | `[x]` |
| B62 | Add compressed AI session persistence mode (delta/summary storage) to reduce local storage and replay token overhead | `[x]` |
| B63 | Add service-agent SLO pack (`p95`, success ratio, tokens/task, cost/task) with CI budget regression checks | `[x]` |

---

## Progress Summary

| Phase | Status |
|------|--------|
| Phase 0 | `[x]` Completed |
| Phase 1 | `[x]` Completed |
| Phase 2 | `[x]` Completed |
| Phase 3 | `[x]` Completed |
| Phase 4 | `[x]` Completed |
| Phase 5 | `[x]` Completed |
| Phase 6 | `[x]` Completed |
| Phase 7 | `[x]` Completed |
| Phase 8 | `[x]` Completed |
| Phase 9 | `[x]` Completed |
| Phase 10 | `[x]` Completed |
| Continuous Backlog | `[x]` Delivered to current scope |

Overall completion estimate: **Phases 0-10 completed; current work is follow-up optimization and platform hygiene**.

## Active Follow-Up Queue (2026-03-20)

| # | Task | Status |
|---|------|--------|
| F1 | Modernize dependency chain to remove allowed `cargo audit` warnings (`rustls-pemfile` via `reqwest 0.11`, `windows 0.24.0` via `winrt-notification`) | `[ ]` |
| F2 | Propagate typed control-plane metadata into outbound Telegram/Discord notifications, not only inbound audit trails | `[ ]` |
| F3 | Reuse the real interactive CLI session identifier for local `/task submit` instead of a request-derived fallback session ID | `[ ]` |
| F4 | Keep canonical planning in `planning/active` and keep `.temp` disposable-only | `[x]` |
| F5 | Centralize Cargo build output, coverage artifacts, and local deployment/runtime artifacts under `.build/` and `.deployment/` | `[x]` |

---

## Work Sessions (Condensed Log)

### Session 1 - 2026-02-27: Deep Analysis + Resize Foundation
- Performed full codebase analysis and identified terminal resize root causes.
- Delivered first-pass resize hardening and runtime state cleanup.

### Session 2 - 2026-02-27: Terminal Exit and Cursor Behavior
- Preserved terminal history on exit (`/quit`, `Ctrl+C`).
- Ensured cursor state restoration and prompt visibility.

### Session 3 - 2026-02-28: Security Remediation
- Applied dependency remediations and validated `cargo audit` baseline.
- Documented post-fix security report artifacts.

### Session 4 - 2026-02-28: Parallel Stability and CI Gate
- Stress-tested templating concurrency locally.
- Added dedicated CI stress gate to prevent concurrency regressions.

### Session 5 - 2026-02-28: Width Hardcode Removal
- Removed critical fixed-width layout behavior from key UI/manifest flows.
- Added responsive width clamping and safer truncation behavior.

### Session 6 - 2026-03-01: Narrow-Terminal Fallback
- Standardized compact behavior for terminals below 80 columns.
- Reduced visual metadata and ensured readability under constraints.

### Session 7 - 2026-03-01: Repeated Resize Test Suite
- Added repeated shrink/grow resize tests and burst/debounce validation.
- Closed Phase 1 runtime stability acceptance criteria.

### Session 8 - 2026-03-01: TUI UX Guideline Publication
- Published centralized TUI UX engineering guideline.
- Linked implementation checklists and error/fallback states.

### Session 9 - 2026-03-01: Manifest Real Execution (List/Check/Render)
- Replaced non-interactive manifest placeholders with real execution paths.
- Added deterministic path handling and expanded E2E coverage.

### Session 10 - 2026-03-01: Translate Real Routing
- Replaced translate placeholder with real orchestration flow.
- Added parser/validation and coverage for success/failure paths.

### Session 11 - 2026-03-01: Free-Text Intent Routing
- Implemented free-text to command routing heuristics.
- Added inference and integration tests.

### Session 12 - 2026-03-01: Interactive Manifest Apply Integration
- Routed `/manifest apply` (without path) to real interactive apply flow.
- Preserved deterministic explicit-path behavior.

### Session 13 - 2026-03-01: Release Smoke Test
- Added cross-platform post-release smoke validation and checksum checks.

### Session 14 - 2026-03-01: Deterministic Release Notes and Tag Gate
- Added strict semver/tag-version validation.
- Enforced exact changelog-section extraction.

### Session 15 - 2026-03-01: Keyless Artifact Signing
- Added `cosign` keyless signing and published signature artifacts.

### Session 16 - 2026-03-01: Compatibility and Support Matrix
- Published compatibility/support policy and added release enforcement gate.

### Session 17 - 2026-03-01: Artifact Verification Runbook
- Published release artifact verification operational guide.

### Session 18 - 2026-03-01: Manual Release Verification Workflow
- Added `workflow_dispatch` verification workflow for published tags.

### Session 19 - 2026-03-01: EOL Table and Policy Window
- Added official support lifecycle and EOL table semantics.

### Session 20 - 2026-03-01: SBOM Verification Hardening
- Expanded manual verification flow to signed SBOM validation and metadata checks.

### Session 21 - 2026-03-01: EOL Semantic Gate
- Enforced semantic lifecycle validations in release pipeline.

### Session 22 - 2026-03-01: Shared Script Pattern Alignment
- Aligned lifecycle validation with shared external script pattern and safe fallback.

### Session 23 - 2026-03-02: Legacy Alias Cleanup
- Removed legacy orchestrator aliases (`Command`, `get_command`, `definitions`).
- Migrated CLI to canonical `MainAction/get_main_action` API.

### Session 24 - 2026-03-02: Manifest API Canonicalization
- Removed legacy `nettoolskit_manifest::parser` alias.
- Updated consumers to canonical crate-root `ManifestParser` API.

### Session 25 - 2026-03-02: Discovery Performance Cleanup
- Optimized manifest discovery by pruning heavy directories and deterministic deduplication.

### Session 26 - 2026-03-02: Enhanced Input + Coverage Insights
- Integrated `rustyline` in CLI loop with persistent history and command auto-complete, preserving `/` palette flow and fallback path.
- Upgraded CI coverage job to publish HTML graph artifacts (`coverage-report`), JSON summary, and enforce minimum line/function thresholds.

### Session 27 - 2026-03-02: Multiline Input Support
- Added multiline input support in CLI through `rustyline` validation, allowing explicit continuation with trailing `\`.
- Added normalization + tests to keep command/text routing stable when multiline submissions are finalized.

### Session 28 - 2026-03-02: Manifest File Picker (Phase 4.2)
- Added `FilePicker` component in `nettoolskit-ui` with fuzzy filtering, regex mode (`re:`), literal mode (`lit:`), and keyboard navigation.
- Integrated picker into `manifest check`, `manifest render`, and `manifest apply`, preserving manual path entry fallback when picker is cancelled.
- Added unit coverage for filter/scoring behavior and manifest path parsing.

### Session 29 - 2026-03-02: Interactive Status Bar (Phase 4.3)
- Added `StatusBar` component in `nettoolskit-ui` with explicit modes (`READY`, `MENU`, `COMMAND`, `TEXT`, `SHUTDOWN`) and bounded notification queue.
- Integrated status bar rendering into both interactive input paths (`rustyline` and legacy raw-mode loop).
- Added command outcome tracking (success/error/interrupted counters + last command latency) and runtime usage segment (terminal dimensions + uptime).

### Session 30 - 2026-03-02: Interactive History Viewer (Phase 4.4)
- Added `HistoryViewer` widget in `nettoolskit-ui` with paging, index-based entry rendering, and case-insensitive filtering support.
- Added local `/history` command handling in CLI interactive loops (rustyline + legacy), without depending on orchestrator command routing.
- Added bounded session history tracking with deterministic capacity policy and test coverage for helper behavior.

### Session 31 - 2026-03-03: Interactive Input Syntax Highlighting (Phase 5.1 Start)
- Added ANSI syntax highlighting to the `rustyline` input helper for command lines and code-like text.
- Added language detection + lexical keyword highlighting for Rust, C#, JavaScript, and TypeScript with string/comment styling.
- Added tests for language detection and highlighting output plus quality/security validations (`fmt`, `clippy`, `test`, `cargo audit` script).

### Session 32 - 2026-03-03: Tree-Sitter Integration + Highlight Performance (Phase 5.1 Complete)
- Integrated `tree-sitter` parsers for Rust, C#, JavaScript, and TypeScript token extraction in interactive highlight flow.
- Added safe fallback to lexical highlighter and bounded fast-path (`MAX_HIGHLIGHT_LINE_BYTES`) to avoid expensive parsing on large inputs.
- Added thread-local highlight result cache for repeated lines and parser reuse to keep interactive typing latency low.

### Session 33 - 2026-03-03: Translate Expansion for Clojure/TypeScript
- Implemented real `/translate` execution paths for `clojure` and `typescript` targets using shared translation pipeline.
- Added target-specific placeholder convention conversion for both languages and kept deterministic output extension handling (`.clj`, `.ts`).
- Updated translate test suite expectations and added integration coverage for generated output files and conversion behavior.

### Session 34 - 2026-03-03: Markdown Rendering in UI + Help Integration
- Added a lightweight Markdown renderer in `nettoolskit-ui` using `pulldown-cmark` with support for headings, lists, links, inline/fenced code, and task markers.
- Integrated `/help` output in orchestrator to render structured Markdown instead of line-by-line manual formatting.
- Added dedicated renderer tests and validated full quality gates (`fmt`, `test`, `clippy`) for affected crates.

### Session 35 - 2026-03-03: Clipboard Integration in Interactive Input
- Added clipboard API in `nettoolskit-ui` (`arboard` backend) and exported copy/paste helpers with deterministic error handling.
- Integrated shortcuts in CLI input flows (`rustyline` + legacy): paste via `Ctrl+V`/`Alt+V`, copy via `Alt+C`/`Ctrl+Shift+C`, with footer status notifications.
- Added normalization and safety tests for clipboard input handling and validated gates (`cargo fmt`, `cargo test -p nettoolskit-ui -p nettoolskit-cli`, `cargo clippy ... -D warnings`, Rust vulnerability audit script).

### Session 36 - 2026-03-03: Configurable Attention Bell for Interactive Failures
- Extended `AppConfig` general settings with `attention_bell` (default `false`) plus environment override `NTK_ATTENTION_BELL`.
- Added `/config` support for `attention_bell` (`set`/`unset`/`show`) and kept runtime behavior consistent with existing key parsing.
- Integrated terminal bell emission (`BEL`) in interactive command outcome handling for `Error` and `Interrupted` states when enabled, without introducing external runtime dependencies.
- Added/updated tests and validated quality/security gates (`cargo fmt --all`, `cargo test -p nettoolskit-core -p nettoolskit-ui -p nettoolskit-orchestrator -p nettoolskit-cli`, `cargo clippy ... -D warnings`, Rust vulnerability audit script).

### Session 37 - 2026-03-03: Cross-Platform Desktop Attention Notifications
- Added `emit_desktop_attention_notification` in `nettoolskit-ui` with platform-specific backends (Windows toast via `winrt-notification`, macOS `osascript`, Linux `notify-send`) and deterministic validation/error paths.
- Extended runtime config with `attention_desktop_notification` (default `false`), including env override `NTK_ATTENTION_DESKTOP_NOTIFICATION` and `/config show|set|unset` integration.
- Updated interactive failure/interruption handling to emit desktop attention notifications (plus existing bell path when enabled), respecting `attention_unfocused_only` focus gating.

### Session 38 - 2026-03-03: Phase 2.4 Async Command Alias Delivery
- Added async aliases for manifest workflows: top-level `/new-async`, `/render-async`, `/apply-async`, plus `/manifest render-async` and `/manifest apply-async`.
- Integrated alias execution through `AsyncCommandExecutor::spawn_with_progress` with standardized progress payload formatting (message + percent + steps) streamed to footer logs.
- Added parser/helper test coverage for async alias detection, positional argument offsets, and progress string formatting behavior.

### Session 39 - 2026-03-03: Async Alias Ctrl+C Cancellation Hardening
- Wired CLI command execution to pass interruption state (`AtomicBool`) into orchestrator command processing.
- Added cooperative cancellation checks in async alias progress loop, aborting executor tasks on Ctrl+C and returning `ExitStatus::Interrupted`.
- Added async tests for interrupted/non-interrupted alias execution paths and updated phase planning focus to caching work (Phase 2.5).

### Session 40 - 2026-03-03: Phase 2.5 Cache Foundation (LRU + TTL + Memory Bound)
- Added dedicated command-result cache module in orchestrator with deterministic LRU recency handling and bounded footprint control.
- Implemented TTL partitioning per command type (`Help`, `ManifestList`) and enforced eviction for expired entries and memory pressure.
- Integrated cache hits/misses for `/help` and `/manifest list` command flows with runtime cache gauges and full test/quality gate validation.

### Session 41 - 2026-03-03: Phase 2.5 Performance Benchmark Completion
- Added Criterion benchmark suite for runtime command cache operations (`command_cache_insert_help`, `command_cache_get_help_hit`, `command_cache_get_manifest_miss`, `command_cache_eviction_pressure`).
- Captured baseline timing envelope: ~155-270ns for hit/miss/insert paths and ~18.7-19.5ms for eviction-pressure batch flow.
- Completed remaining Phase 2.5 benchmark requirement and advanced implementation focus to Phase 2.6.

### Session 42 - 2026-03-03: Phase 2.6 Predictive Input Delivery
- Implemented `rustyline` hinter support for slash-command predictive hints with stable shortest-candidate ranking and no-op behavior for complete/non-command input.
- Added unit coverage for predictive hint suffix generation and guard paths (`non-command`, `already complete`, `unknown`, `trailing-space`).
- Preserved existing command palette behavior and completion flow while improving proactive guidance in command typing.

### Session 43 - 2026-03-03: Phase 2.6 Configuration + Plugin Foundation
- Extended configuration surface with `predictive_input` (default `true`), environment override support (`NTK_PREDICTIVE_INPUT`), and interactive option plumbing into `RustylineInput`.
- Added `/config` coverage for `predictive_input` key management (`show`, `set`, `unset`) with default restoration and effective-config display updates.
- Implemented plugin foundation in orchestrator with deterministic registry (`register/list/enable`), safe before/after command hooks, and non-blocking failure handling.
- Integrated command hook execution into processor flow and published runtime plugin gauges/error counters for observability.

### Session 44 - 2026-03-03: Phase 2.6 Error Recovery Completion
- Added bounded recovery strategy for interactive input backends (`rustyline` and legacy): transient read failures now trigger warning notifications, footer diagnostics, and short backoff retries instead of immediate session abort.
- Enforced deterministic retry budget (`3` consecutive failures) with explicit exhaustion path to avoid infinite degraded loops.
- Added panic-safe task execution wrapper for command/text processing using `tokio::spawn` join handling; panics are converted into `ExitStatus::Error` with session continuity.
- Added unit coverage for recovery budget behavior and panic-to-error conversion to protect against regressions.

### Session 45 - 2026-03-03: Phase 3.1 Rich State Management Completion
- Added new `cli::state` module with serializable `CliState` model containing session metadata, effective config snapshot, and typed bounded history entries.
- Introduced `HistoryEntry` trait + `HistoryEntryKind` to formalize command/text history contract.
- Added shared runtime state handle (`SharedCliState = Arc<RwLock<CliState>`) and integrated state recording into interactive loops (rustyline and legacy paths).
- Added JSON serialize/deserialize API (`to_json`, `to_json_pretty`, `from_json`) and unit tests for bounded history, blank-entry guard, shared access, and round-trip integrity.

### Session 46 - 2026-03-03: Phase 3.2 Local Session Persistence Delivery
- Implemented local snapshot storage in `cli::state` with JSON save/load/list/prune APIs rooted in `AppConfig::default_data_dir()` (`.../ntk/sessions`), including resilient sorting by last activity and bounded retention support.
- Integrated startup auto-resume of the latest local snapshot in interactive mode, preserving typed history and refreshing runtime config/session capacity for the current process.
- Integrated exit-time persistence for interactive sessions (success/error/interrupted paths) with non-fatal save/prune handling and local history seeding so `/history` reflects resumed entries.
- Added unit coverage for snapshot round-trip, latest-session loading, pruning behavior, runtime resume initialization, and persistence pipeline hooks.

### Session 47 - 2026-03-03: Phase 3.2 Session Picker Completion
- Added startup local session picker flow for interactive mode when multiple local snapshots are available.
- Reused `CommandPalette` with dedicated resume entries and metadata-rich descriptions (history count + timestamps), preserving local-only persistence policy.
- Added safe fallback logic: cancel/selection failures fall back to latest snapshot load path without blocking CLI startup.
- Added tests for picker option formatting and selection pipeline behavior (`selected snapshot`, `single snapshot skip`) plus full gate validation (`fmt`, `clippy`, `test`, vulnerability audit).

### Session 48 - 2026-03-03: Phase 3.3 Frame Scheduler Completion
- Added frame scheduler runtime in `nettoolskit-ui` terminal interaction layer with coalesced frame requests and 60 FPS target budget (`request`, `consume`, `poll timeout` APIs).
- Integrated scheduler with CLI rendering path so status bar redraw requests are coalesced and rate-limited instead of rendering every loop iteration.
- Integrated scheduler timeout adaptation in legacy async input loop (`read_line`) and tied resize events to frame requests for event-loop friendly wakeups.
- Added deterministic scheduler unit tests (coalescing, rate limit, timeout behavior) and validated full quality/security gates (`fmt`, `clippy`, `test`, vulnerability audit).

### Session 49 - 2026-03-03: Phase 5.2 Markdown Code Block Highlighting Completion
- Extended Markdown renderer to parse fenced code block language hints and apply lightweight language-aware lexical highlighting (Rust, C#, JavaScript, TypeScript, JSON, TOML, Bash, PowerShell).
- Added token-level coloring for keywords, strings, numbers, and comments while preserving plain-text fallback when terminal color is unavailable.
- Added focused renderer unit tests for language parsing, ANSI-highlight behavior, and comment parsing edge cases.
- Hardened integration tests with ANSI normalization helper to assert semantic output content independently from terminal styling codes.

### Session 50 - 2026-03-03: Phase 8 AI Assistant Integration Planning
- Added a dedicated Phase 8 plan for a Codex/OpenClaw-style AI assistant integrated into CLI workflows.
- Defined strict safety constraints (explicit approvals for side effects) and local-only persistence requirements for AI sessions.
- Published next actionable backlog item (B35) to implement provider abstraction with deterministic tests.
- Re-ran validation gates after planning updates: `cargo test -p nettoolskit-cli -p nettoolskit-orchestrator -p nettoolskit-ui` and `cargo clippy -p nettoolskit-cli -p nettoolskit-orchestrator -p nettoolskit-ui --all-targets -- -D warnings`.

### Session 51 - 2026-03-03: Phase 8.1 AI Provider Abstraction + Deterministic Mock Delivery
- Added `execution::ai` module in `nettoolskit-orchestrator` with provider contract (`AiProvider`), typed request/response/chunk models, and explicit error taxonomy (`AiProviderError`).
- Implemented `MockAiProvider` with deterministic scripted outcomes (`Complete`, `Stream`, `Error`) plus optional fixed delay for reproducible timeout/latency behavior.
- Added focused async tests for scripted order, error handling, invalid request/response validation, streaming chunks, and deterministic delay.
- Exported AI primitives through `execution` and crate root public API to support upcoming `/ai` command integration.
- Left OpenAI-compatible transport adapter as the next explicit item (`8.1.1` / `B36`) to keep this slice deterministic-first for test reliability.
- Validation executed: Rust security audit script (`Invoke-RustPackageVulnerabilityAudit.ps1`), `cargo clippy -p nettoolskit-orchestrator --all-targets -- -D warnings`, and `cargo test -p nettoolskit-orchestrator`.

### Session 52 - 2026-03-03: Phase 8.1.1 OpenAI-Compatible Adapter Delivery
- Added `OpenAiCompatibleProviderConfig` and `OpenAiCompatibleProvider` to `execution::ai` with endpoint/API key/default model/timeout controls.
- Implemented HTTP request/response handling for OpenAI-compatible chat completions schema, including typed payload parsing and usage extraction.
- Added deterministic fallback behavior for transport/unavailability/timeout errors (`fallback_output_text`) while keeping malformed success payloads as explicit `InvalidResponse`.
- Added adapter-focused tests using local `tokio::net::TcpListener` server for success parsing, timeout errors, transport fallback, service-unavailable fallback, and malformed payload handling.
- Validation executed: `cargo fmt --all`, `cargo clippy -p nettoolskit-orchestrator --all-targets -- -D warnings`, `cargo test -p nettoolskit-orchestrator`, and Rust vulnerability audit script (`Invoke-RustPackageVulnerabilityAudit.ps1`).

### Session 53 - 2026-03-04: Phase 8.2 `/ai` Command Surface Integration
- Integrated `MainAction::Ai` and wired `/ai` command routing in orchestrator with supported intents (`ask`, `plan`, `explain`, `apply --dry-run`) and usage/help flows.
- Added provider bootstrapping from environment (`mock` default, `openai` adapter path), streaming output handling, and safety guard to enforce `--dry-run` for `apply`.
- Updated free-text routing aliases (`ai`, `assistant`, `copilot`) to resolve consistently to `/ai ...` commands and preserved prompt tail for practical execution.
- Extended CLI interactive completions/hints for `/ai` family and added unit/integration coverage for routing and safety paths.
- Validation executed: Rust vulnerability audit script (`Invoke-RustPackageVulnerabilityAudit.ps1`), `cargo fmt --all`, `cargo clippy -p nettoolskit-orchestrator -p nettoolskit-cli --all-targets -- -D warnings`, and `cargo test -p nettoolskit-orchestrator -p nettoolskit-cli`.

### Session 54 - 2026-03-04: Phase 8.3 Workspace Context Collector Delivery
- Added `nettoolskit_core::ai_context` module with deterministic workspace context collection from explicit allowlisted paths.
- Implemented strict path normalization and workspace-boundary enforcement, plus byte budgets (`max_files`, `max_file_bytes`, `max_total_bytes`) with UTF-8-safe truncation.
- Implemented secret redaction for key/value lines and bearer tokens before context is attached to AI requests.
- Integrated context bundle injection into `/ai` request flow in orchestrator as an additional system message with local-only defaults and environment overrides (`NTK_AI_CONTEXT_*`).
- Added focused tests for redaction, allowlist/budget behavior, path parsing helpers, and markdown-style context message rendering.
- Validation executed: Rust vulnerability audit script (`Invoke-RustPackageVulnerabilityAudit.ps1`), `cargo fmt --all`, `cargo clippy -p nettoolskit-core -p nettoolskit-orchestrator -p nettoolskit-cli --all-targets -- -D warnings`, and `cargo test -p nettoolskit-core -p nettoolskit-orchestrator -p nettoolskit-cli`.

### Session 55 - 2026-03-04: Phase 8.4 Approval Gateway + Local Audit Trail Delivery
- Added `execution::approval` module in orchestrator with typed approval contracts for mutating intents (`command_execution`, `file_write`), explicit-approval enforcement, and deterministic dry-run auto-approval.
- Added local JSONL audit trail writer for approval decisions with a workspace-local default path and optional override (`NTK_AI_APPROVAL_AUDIT_PATH`).
- Integrated approval gateway into `/ai apply`: non-dry-run requests are blocked unless explicit write approval (`--approve-write`) is provided, and denial reasons are surfaced without crashing session.
- Updated `/ai` help surface and CLI completion catalog to include explicit write-approval path.
- Added unit/integration coverage for approval decisions, audit persistence, and `/ai apply` deny/approve flows.
- Validation executed: Rust vulnerability audit script (`Invoke-RustPackageVulnerabilityAudit.ps1`), `cargo fmt --all`, `cargo clippy -p nettoolskit-core -p nettoolskit-orchestrator -p nettoolskit-cli --all-targets -- -D warnings`, and `cargo test -p nettoolskit-orchestrator -p nettoolskit-cli`.

### Session 56 - 2026-03-04: Phase 8.5 Local AI Session Persistence + Resume Picker Delivery
- Added `execution::ai_session` module with local-only JSON persistence under app data (`.../ntk/ai-sessions`), bounded exchange history, listing/loading/pruning APIs, and process-local active session id management.
- Integrated `/ai` runtime with persisted conversation context injection, active session continuity, post-response snapshot persistence, and retention pruning controlled by config.
- Added `/ai resume <session-id>` orchestrator subcommand and interactive `/ai resume` picker in CLI input loop to switch active AI session without leaving interactive mode.
- Extended config surface with `general.ai_session_retention` plus env override (`NTK_AI_SESSION_RETENTION`) and `/config show|set|unset|reset` handling including immediate prune application.
- Added deterministic test coverage for new persistence module, picker helpers, `/ai resume` flows, and retention parsing/config paths; updated `/ai` command completion/help to include `resume`.
- Validation executed: Rust vulnerability audit script (`Invoke-RustPackageVulnerabilityAudit.ps1`), `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace`.

### Session 57 - 2026-03-04: Phase 8.6 Operational Controls Delivery
- Added bounded provider resilience controls for `/ai` requests: timeout wrapping, retriable error detection, exponential retry/backoff, and process-local rate limiting with environment-based policy overrides.
- Added operational telemetry for AI flows: request/attempt latency, success/error/timeout/retry counters, rate-limit counters/gauges, approval ratio metrics, provider health gauges, and token/cost estimate metrics.
- Added user-facing fallback guidance for transient provider failures (`timeout`, `unavailable`, `transport`) and expanded `/ai` help text with operational tuning variables.
- Added deterministic unit/integration coverage for retry policy, backoff bounds, rate-limit evaluation, approval metric ratio updates, token/cost metric updates, fallback messaging, and transient retry success flow.
- Validation executed: `cargo fmt --all`, `cargo test -p nettoolskit-orchestrator --lib`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace`.

### Session 58 - 2026-03-04: Phase 8.7 AI Validation and Release Gating Completion
- Added dedicated AI E2E integration scenarios in orchestrator test suite for `/ai plan`, `/ai apply --dry-run`, safety blocking of `/ai apply` without flags, and free-text alias routing into AI flow.
- Added an explicit `AI Gate` job in CI to enforce AI-focused test slices (`e2e_ai_*`, `process_ai_command_*`, retry/rate-limit resilience tests) as a first-class release quality gate.
- Re-ran full workspace quality gates after AI test/gating updates to ensure no regressions in non-AI flows.
- Validation executed: `cargo fmt --all`, `cargo test -p nettoolskit-orchestrator --test test_suite e2e_ai_`, `cargo test -p nettoolskit-orchestrator --lib process_ai_command_`, `cargo test -p nettoolskit-orchestrator --lib request_ai_stream_with_retry_`, `cargo test -p nettoolskit-orchestrator --lib evaluate_ai_rate_limit_`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace`.

### Session 59 - 2026-03-04: Phase 9 Planning Kickoff (Dual Runtime CLI + Background Services)
- Added a dedicated Phase 9 planning track for a dual runtime model: local-first CLI mode and Dockerized background service mode acting as AI/task manager.
- Defined planned deliverables for shared runtime contracts, CLI service-control surface, queued worker lifecycle, AI orchestration in service mode, Docker operations, and dual-mode CI gates.
- Created dedicated planning document `task-phase-9.0-dual-runtime-cli-and-background-services.md` and linked it to roadmap/index tracking files.
- Marked new backlog entries for Phase 9 planning and local Docker operational baseline.

### Session 60 - 2026-03-04: Phase 9.0.1 Runtime Contracts Delivery
- Added shared runtime contracts in `nettoolskit-core` (`RuntimeMode`, `TaskIntentKind`, `TaskIntent`, `TaskExecutionStatus`, `TaskAuditEvent`) and deterministic runtime-mode resolution helper.
- Integrated runtime mode into `AppConfig` (`general.runtime_mode`) with environment override (`NTK_RUNTIME_MODE`) and expanded config test coverage.
- Extended `/config` handling in orchestrator to support `runtime_mode` in `show`, `set`, and `unset` flows with input validation.
- Validation executed: shared Rust vulnerability audit script, `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace`.

### Session 61 - 2026-03-04: Phase 9.0.6 Dual Runtime Validation Gate Completion
- Added `Dual Runtime Gate` CI job in `.github/workflows/ci.yml` to enforce runtime contract tests, task service orchestration slices, service endpoint tests, and Docker compose smoke checks.
- Added service endpoint tests in `crates/cli/src/main.rs` for `GET /health`, invalid JSON handling on `POST /task/submit`, and accepted submit responses.
- Marked dual-mode validation/release gate item as completed in Phase 9 tracker and aligned Phase 9.0 planning acceptance criteria.
- Validation executed: `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace`, `docker compose -f deployments/docker-compose.local.yml up --build -d`, health/task smoke checks, and `docker compose ... down`.

### Session 62 - 2026-03-04: Phase 9.0.7 ChatOps Foundation Slice
- Added new `execution::chatops` module in orchestrator with platform-neutral contracts for Telegram/Discord adapters (`ChatOpsIngress`, `ChatOpsNotifier`, command envelope, notifications, and severity model).
- Implemented ChatOps command parser and authorization policy (deny-by-default, explicit user/channel allowlists), then routed authorized commands to existing `/task` processing for local-first execution safety.
- Added local JSONL audit persistence (`ChatOpsLocalAuditStore`) for received/rejected/executed/notified lifecycle events and deterministic mock adapters (`MockChatOpsIngress`, `RecordingChatOpsNotifier`) for testability.
- Added ChatOps operational runbook for VPS profile with secure defaults and local persistence requirements (`docs/operations/chatops-agent-vps-profile.md`).
- Validation executed: `cargo fmt --all`, `cargo clippy -p nettoolskit-orchestrator --all-targets -- -D warnings`, and `cargo test -p nettoolskit-orchestrator`.

### Session 63 - 2026-03-04: Phase 9.0.7 ChatOps Runtime Wiring Slice
- Added `execution::chatops_runtime` module with environment-driven config loader and runtime builder (`build_chatops_runtime_from_env`) to wire ChatOps background processing in service mode.
- Implemented asynchronous Telegram and Discord adapters for ingress polling and outbound notification dispatch, including local offset/last-seen tracking.
- Wired `ntk service` startup to spawn optional ChatOps polling loop when `NTK_CHATOPS_ENABLED=true`, with periodic tick summaries in tracing logs.
- Added runtime-focused tests for env parsing and startup gating, and updated operational runbook/changelog to reflect implemented polling-mode behavior.
- Validation executed: vulnerability audit script, `cargo fmt --all`, `cargo clippy -p nettoolskit-orchestrator -p nettoolskit-cli --all-targets -- -D warnings`, and `cargo test -p nettoolskit-orchestrator -p nettoolskit-cli`.

### Session 64 - 2026-03-04: Phase 9.0.7 Repository Workflow Policy-Gated Slice
- Added `execution::repo_workflow` module with request parser (JSON + key-value), explicit policy model, and deny-by-default gates for repository hosts, command prefixes, push, and PR creation.
- Integrated new `/task submit repo-workflow <payload>` intent into task runtime using explicit policy/env controls and dry-run-safe default behavior.
- Added deterministic unit/integration coverage for payload parsing, policy-denied paths, dry-run planning, and task-level repo-workflow submission.
- Updated ChatOps and service-mode operational runbooks with the repository workflow environment contract and secure-default guidance.

### Session 65 - 2026-03-04: Phase 9.0.7 Scoped Authorization + Rate-Limit Hardening
- Extended ChatOps authorization policy with command-scope allowlist (`NTK_CHATOPS_ALLOWED_COMMANDS`) to bound remote operators to explicit task intents.
- Added per-user/per-channel rate-limit controls in ChatOps runtime (`NTK_CHATOPS_RATE_LIMIT_PER_USER`, `NTK_CHATOPS_RATE_LIMIT_PER_CHANNEL`, `NTK_CHATOPS_RATE_LIMIT_WINDOW_SECONDS`) with warning notifications and local audit entries on throttle.
- Added deterministic unit/integration coverage for scope denial and rate-limit exhaustion/window reset behavior.
- Updated ChatOps VPS runbook and planning artifacts to reflect policy-bounded remote execution as completed for Phase 9.0.7.

### Session 66 - 2026-03-04: Phase 9.0.7 ChatOps VPS Smoke Profile Completion
- Added deterministic ChatOps VPS smoke integration test with local Telegram-compatible mock server validating ingress polling, command routing, notification dispatch, and local audit persistence.
- Extended CI `Dual Runtime Gate` with explicit ChatOps smoke slice (`chatops_vps_smoke_profile_*`) to prevent regressions in remote-agent runtime flow.
- Extended release verification workflow to launch packaged `ntk service` binaries and validate `/health` response in service mode on Linux, Windows, and macOS.
- Updated operational runbooks and planning tracker to mark end-to-end ChatOps VPS smoke coverage as delivered.

### Session 67 - 2026-03-04: Phase 9.0.4 Service Automation Policy Profile Completion
- Added service automation policy profiles (`strict`, `balanced`, `open`) with environment-driven overrides for allowed intents and queue-admission budgets in `runtime_mode=service`.
- Added service submit guards for intent allowlist, payload byte budget, in-flight task budget, and submit-rate budget window before task queue admission.
- Added deterministic unit/integration tests for profile defaults/overrides, budget exhaustion behavior, and service submit rejections for disallowed intent and oversized payload.
- Updated service/chatops runbooks, local service env template, and changelog to document `NTK_SERVICE_*` policy contract.

### Session 68 - 2026-03-04: Phase 9.0.3 Worker Runtime Extraction to Dedicated Crate
- Added new shared crate `nettoolskit-task-worker` with reusable queue runtime, bounded concurrency, retry backoff, submit error contract, and callback hooks.
- Migrated orchestrator background task worker wiring from in-file dispatcher logic to `TaskWorkerRuntime`, keeping existing task lifecycle/audit semantics through callback integration.
- Preserved existing service-mode `/task submit` behavior and queue admission errors while reducing worker/runtime coupling in `processor.rs`.
- Added dedicated crate unit tests for retry-delay bounds, retry-to-success behavior, and pre-attempt cancellation handling.

### Session 69 - 2026-03-04: Phase 9.0.7 Telegram Webhook Ingress Mode
- Added Telegram webhook ingress queue mode in ChatOps runtime (`NTK_CHATOPS_TELEGRAM_WEBHOOK_ENABLED`) as an alternative to polling.
- Added runtime enqueue API for raw Telegram webhook payload ingestion and wired service endpoint `POST /chatops/telegram/webhook`.
- Preserved local-first behavior by keeping webhook queue in-process and auditable through existing ChatOps execution/audit pipeline.
- Added deterministic tests for webhook payload parsing, queue draining, runtime mode gating, and service endpoint valid/invalid/disabled flows.

### Session 70 - 2026-03-05: Phase 9.0.7 Discord Interaction Ingress Mode
- Added Discord interaction ingress queue mode in ChatOps runtime (`NTK_CHATOPS_DISCORD_INTERACTIONS_ENABLED`) as an alternative to Discord channel polling.
- Split Discord notifier transport from polling ingress so interaction mode can run without requiring `NTK_CHATOPS_DISCORD_CHANNELS`.
- Added runtime enqueue API for raw Discord interaction payload ingestion and wired service endpoint `POST /chatops/discord/interactions`.
- Added deterministic tests for interaction payload parsing (`ping` and command), queue draining, runtime mode gating, and service endpoint valid/invalid/disabled flows.

### Session 71 - 2026-03-05: Phase 9.0.7 Burst-Aware Throttling Strategy
- Added ChatOps rate-limit strategy selector (`NTK_CHATOPS_RATE_LIMIT_STRATEGY`) with `fixed_window` (default) and `token_bucket` options.
- Added optional burst budgets for token-bucket mode (`NTK_CHATOPS_RATE_LIMIT_BURST_PER_USER`, `NTK_CHATOPS_RATE_LIMIT_BURST_PER_CHANNEL`) to absorb short traffic spikes.
- Preserved existing fixed-window behavior for backward compatibility and deterministic operations.
- Added deterministic tests for strategy parsing, token-bucket burst + refill behavior, and env-driven configuration parsing.

### Session 72 - 2026-03-05: Phase 9.0.7 Ingress Signature + Replay Hardening
- Added optional Telegram webhook secret-token validation (`NTK_CHATOPS_TELEGRAM_WEBHOOK_SECRET_TOKEN`) for `POST /chatops/telegram/webhook`.
- Added optional Discord Ed25519 request-signature validation (`NTK_CHATOPS_DISCORD_INTERACTIONS_PUBLIC_KEY`) for `POST /chatops/discord/interactions`.
- Added bounded replay-protection guard (`NTK_CHATOPS_INGRESS_REPLAY_WINDOW_SECONDS`, `NTK_CHATOPS_INGRESS_REPLAY_MAX_ENTRIES`) shared by webhook/interaction ingress paths.
- Added deterministic service endpoint tests for invalid token/signature rejection and replay rejection while preserving valid request acceptance.

### Session 73 - 2026-03-05: Phase 9.0.7 Reverse Proxy Reference Profiles
- Added Nginx and Caddy reference profiles for ChatOps ingress exposure (`deployments/reverse-proxy/...`) with TLS and endpoint allowlist defaults.
- Preserved required security headers through proxy (`X-Telegram-Bot-Api-Secret-Token`, `X-Signature-Ed25519`, `X-Signature-Timestamp`) to keep service-side validation effective.
- Published dedicated reverse-proxy runbook (`docs/operations/chatops-reverse-proxy-profiles.md`) and linked it from service/chatops operational docs.

### Session 74 - 2026-03-05: Phase 9.0.7 Adaptive Rate-Limit Auto-Tuning
- Added ChatOps auto-tuning profile env contract (`NTK_CHATOPS_RATE_LIMIT_AUTOTUNE_PROFILE`) with `conservative`, `balanced`, `aggressive`, and `disabled` options.
- Implemented traffic-observation window logic in ChatOps rate limiter to auto-switch between `fixed_window` and `token_bucket` under sustained high/low ingress load.
- Added deterministic runtime tests for profile parsing, high-traffic switch to token-bucket, and low-traffic switch back to fixed-window behavior.

### Session 75 - 2026-03-05: Phase 9.0.7 Multi-Process Replay Cache Backend
- Added replay backend contract for ingress security (`NTK_CHATOPS_INGRESS_REPLAY_BACKEND=memory|file`) with optional explicit file path (`NTK_CHATOPS_INGRESS_REPLAY_FILE_PATH`).
- Implemented file-backed replay cache mode with lock file coordination to share replay detection across independent service states/processes on the same persisted volume.
- Added deterministic service endpoint tests validating shared replay detection across independent runtime states and `503` behavior when file backend storage is unavailable.

### Session 76 - 2026-03-05: Coverage Expansion + OpenClaw Alignment Planning
- Increased deep-test coverage on low-coverage hotspots by extending `repo_workflow` and `tracing_setup` unit tests with environment, parser, policy, and execution-path scenarios (including local git workflow execution and push path validation).
- Coverage delta after full workspace run: line coverage moved from `72.54%` to `74.98%`; key module improvements: `repo_workflow.rs` to `90.62%` lines, `tracing_setup.rs` to `72.24%`, `file_picker.rs` to `69.94%`, `prompt.rs` to `93.48%`, and `manifest/ui/menu.rs` to `18.64%`.
- Added explicit OpenClaw-inspired backlog track focused on security, performance, and token economy for service-mode agent runtime (provider fallback budgets, token/cost policy, tool permission scopes, adaptive model routing, compression, and SLO gates).

### Session 77 - 2026-03-05: Phase 9.8/B58 Provider Routing + Deterministic Fallback Delivery
- Implemented deterministic AI provider routing chain with `primary -> secondary` execution order and bounded depth (2 providers max), using `NTK_AI_PROVIDER_CHAIN` and `NTK_AI_FALLBACK_PROVIDER`.
- Added per-provider timeout budgets for routed execution attempts (`NTK_AI_PROVIDER_PRIMARY_TIMEOUT_MS`, `NTK_AI_PROVIDER_SECONDARY_TIMEOUT_MS`) and wired routed attempts into `/ai` runtime flow.
- Added transient-error failover behavior (timeout/unavailable/transport) with deterministic non-failover for non-retriable errors, plus route-aware telemetry (`runtime_ai_provider_failovers_total`, last failover gauge, per-provider request/error counters).
- Extended `/ai` help and runtime log output to expose active provider route and operational routing controls.
- Added deterministic unit/integration coverage for provider-chain parsing, route construction, timeout budget parsing, transient failover success, and fail-closed behavior on invalid request errors.
- Stabilized orchestrator task submission test isolation under parallel workspace runs by adding env-lock/cleanup to `process_task_command_submit_ai_plan_succeeds`.
- Validation executed: shared Rust vulnerability audit script, `cargo fmt --all`, `cargo test -p nettoolskit-orchestrator --lib`, `cargo test -p nettoolskit-cli --bin ntk service_mode_`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`.

### Session 78 - 2026-03-05: Phase 9.8/B59 Token-Economy Policy Delivery
- Implemented AI token-economy policy enforcement across `/ai` and `/task submit ai-*` with request/session token caps, cost caps, prompt compaction tiers, and cache-first response reuse controls.
- Extended command cache model with dedicated AI response partition/TTL and wired cache-first hit/miss/insert telemetry in AI command flow.
- Added deterministic test coverage for token policy env overrides, prompt compaction behavior, budget acceptance/rejection paths, cache-first reuse on repeated prompts, and `/task submit ai-plan` rejection when token budget is exceeded.
- Fixed strict lint breakages introduced by cache TTL expansion (benchmark initializer alignment + test helper utilization under `-D warnings`).
- Validation executed: shared Rust vulnerability audit script, `cargo fmt --all`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`.

### Session 79 - 2026-03-05: Phase 9.8/B60 Secure Tool Scope Gateway
- Implemented secure tool-scope policy for task flows with service-mode deny-by-default behavior, global tool allowlist, and per-intent tool scopes (`NTK_TOOL_SCOPE_*`).
- Added layered enforcement for both task admission (`/task submit`) and task execution (local worker path) to prevent bypasses across runtime branches.
- Added local JSONL audit trail for tool-scope decisions with a workspace-local default path and optional override (`NTK_TOOL_SCOPE_AUDIT_PATH`).
- Added deterministic test coverage for tool-scope parsing, wildcard/alias support, repo-workflow scope expansion (`execute/push/pr`), allow/deny decisions, audit file persistence, and service-mode submit behavior with/without explicit allowlist.
- Validation executed: shared Rust vulnerability audit script, `cargo fmt --all`, `cargo test -p nettoolskit-orchestrator --lib`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`.

### Session 80 - 2026-03-05: Phase 9.8/B61 Adaptive Model Selection with Cost Guardrails
- Implemented adaptive AI model-selection policy with explicit intent-tier routing (`cheap` / `reasoning`) and env-driven model resolution for `/ai` and `/task submit ai-*` flows.
- Added tier-specific cost guardrails and optional auto-downgrade from `reasoning` to `cheap` when reasoning-tier cap is exceeded (`NTK_AI_MODEL_SELECTION_*`).
- Updated AI cache-key routing signature to include selected model tier/model label to prevent stale cross-tier cache collisions.
- Extended runtime telemetry for model selection decisions (`runtime_ai_model_selection_*`) including selected tier, estimated tier-adjusted cost, guardrail fallback, and guardrail rejection counters.
- Added deterministic tests for intent-tier parsing, policy env overrides, guardrail rejection, fallback-to-cheap behavior, process-level guardrail enforcement, and task-submit guardrail validation.
- Stabilized env-sensitive `/ai apply` tests for parallel workspace execution by enforcing shared env lock + deterministic cleanup.
- Validation executed: shared Rust vulnerability audit script, `cargo fmt --all`, `cargo test -p nettoolskit-orchestrator --lib`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`.

### Session 81 - 2026-03-05: Phase 9.8/B62 Compressed AI Session Persistence
- Implemented configurable AI session compression modes for local persistence (`off`, `delta`, `summary`) to reduce storage footprint and replay token overhead.
- Added deterministic environment controls for compression behavior (`NTK_AI_SESSION_COMPRESSION_MODE`, `NTK_AI_SESSION_COMPRESSION_MAX_CHARS`, `NTK_AI_SESSION_DELTA_MIN_SHARED_PREFIX_CHARS`) and exposed them in `/ai` and `/task` operational help.
- Extended persisted AI session schema with backward-compatible metadata (`response_storage_mode`, `original_response_chars`) and legacy snapshot normalization on load.
- Added deterministic tests for compression-mode parsing, summary compaction, delta compaction, and backward compatibility when loading legacy snapshots.
- Validation executed: shared Rust vulnerability audit script, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace`.

### Session 82 - 2026-03-05: Phase 9.8/B63 Service-Agent SLO Pack + CI Regression Gate
- Added AI service-agent SLO policy profile with deterministic env thresholds (`NTK_AI_SLO_MAX_P95_LATENCY_MS`, `NTK_AI_SLO_MIN_SUCCESS_RATE_PCT`, `NTK_AI_SLO_MAX_TOKENS_PER_TASK`, `NTK_AI_SLO_MAX_COST_USD_PER_TASK`).
- Added runtime SLO indicator gauges for agent mode: `runtime_ai_request_latency_p95_ms`, `runtime_ai_tokens_per_task`, `runtime_ai_cost_per_task_usd`, and compliance/budget gauges (`runtime_ai_slo_*`).
- Added deterministic SLO budget evaluation logic and integrated it into AI request-rate gauge refresh for continuous runtime compliance snapshots.
- Added CI `Agent SLO Gate` with targeted regression checks for percentile math and SLO policy/compliance tests.
- Validation executed: shared Rust vulnerability audit script, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace`.

### Session 83 - 2026-03-07: Commercial Gap Analysis + Service Control Plane Hardening Kickoff
- Compared the current workspace against `openai/codex` and `openclaw/openclaw` reference expectations, then recorded a new Phase 10 for commercial control-plane hardening.
- Identified highest-priority gaps: unauthenticated mutable service API surface, health/readiness parity, weak coverage budgets on critical entrypoints, missing OSS governance files, and ad-hoc HTTP transport.
- Started Phase 10.1 by switching `ntk service` to loopback-by-default semantics, adding bearer-token protection for `/task/submit`, and wiring fail-closed startup policy for non-loopback binds without `NTK_SERVICE_AUTH_TOKEN`.
- Updated local Docker/CI/runbook assets to keep service-mode smoke flows aligned with the new auth contract.
- Validation pending full compile/test in this shell because local Windows toolchain still lacks `cl.exe` for transitive native builds (`tree-sitter` path).

### Session 84 - 2026-03-08: Phase 10.3 Coverage Gate Closure + Windows MSVC Validation Recovery
- Restored the local Windows MSVC/SDK validation path so workspace-native crates (`tree-sitter`) compile again in the active shell.
- Ran `cargo test --workspace --all-targets` successfully after removing an unnecessary `Debug` derive from the service runtime bootstrap state in `crates/cli/src/main.rs`.
- Generated `.build/coverage/report.json` with `cargo llvm-cov --workspace --all-targets --json --output-path .build/coverage/report.json` and passed the PowerShell critical-file coverage gate.
- Recorded current workspace coverage totals at `76.47%` lines, `77.20%` functions, and `69.73%` regions.
- Verified critical-file results above policy thresholds: `cli/main.rs` (`75.88%` lines / `71.36%` functions), `manifest/ui/menu.rs` (`33.92%` / `57.14%`), `otel/tracing_setup.rs` (`72.24%` / `79.31%`), and `orchestrator/processor.rs` (`74.85%` / `82.78%`).

### Session 85 - 2026-03-08: Reapplied Missing Phase 10 Hardening After Incorrect Commit
- Reapplied the missing service hardening package after confirming the previous `HEAD` only captured translate-removal scope and omitted the latest enterprise/runtime changes.
- Restored loopback-by-default service binding, bearer-token enforcement for `POST /task/submit`, explicit non-loopback fail-closed startup validation, and real `/ready` dependency checks in `crates/cli/src/main.rs`.
- Restored governance/process assets to the tracked repository state: `LICENSE`, `SECURITY.md`, `.github/CODEOWNERS`, and `.github/scripts/Invoke-RustCriticalFileCoverageGate.ps1`.
- Updated CI, Docker, and operational runbooks to use the service auth contract and to validate both liveness and readiness.
- Revalidated the reapplied package with Rust vulnerability audit, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, service endpoint tests, full `cargo llvm-cov` JSON export, and critical-file coverage gate pass.
- Current post-reapply coverage totals: `76.52%` lines, `76.82%` functions, and `69.59%` regions; critical-file snapshot: `cli/main.rs` (`77.06%` / `67.98%`), `manifest/ui/menu.rs` (`18.64%` / `40.00%`), `otel/tracing_setup.rs` (`72.24%` / `79.31%`), and `orchestrator/processor.rs` (`74.83%` / `82.56%`).

### Session 86 - 2026-03-08: Service HTTP Control-Envelope Adoption
- Wired the shared control-plane contracts into service HTTP admission for `POST /task/submit` by deriving `request_id`, optional `correlation_id`, operator identity, session identity, and transport metadata from middleware plus request headers.
- Wired the shared control-plane contracts into ChatOps `submit` admission by deriving typed operator/session metadata from Telegram/Discord envelopes, routing `submit` through `process_control_envelope`, and persisting normalized request/task metadata into ChatOps audit entries.
- Wired the shared control-plane contracts into local CLI `/task submit` so local task admission now emits typed request/operator/session metadata and persists the envelope into task registry/audit events.
- Updated service middleware to preserve and echo both `x-request-id` and `x-correlation-id`, so upstream callers can keep deterministic trace context without losing the generated service request ID.
- Extended accepted task-submit responses with `task_id` plus envelope metadata (`request_id`, `correlation_id`, `operator_id`, `operator_kind`, `session_id`, `transport`) and persisted the same admitted envelope into task registry/audit events so service ingress now has an end-to-end typed trace path.
- Added direct router tests covering generated metadata defaults, supplied operator/session/correlation overrides, and correlation-header propagation on service responses.
- Revalidated the slice with Rust vulnerability audit, `cargo fmt --all`, and targeted service endpoint tests; `cargo clippy -p nettoolskit-cli --bin ntk --all-features -- -D warnings` still needs a rerun with a longer shell timeout.

### Session 87 - 2026-03-20: Planning Canonicalization + Local Build Hygiene
- Moved canonical planning documents from temporary local planning storage into tracked root-level `planning/active` and completed historical plans into `planning/completed`.
- Removed `.temp` from any source-of-truth role and centralized local build/runtime artifacts under hidden root folders.
- Updated service-mode Docker/runbook assets and AI context defaults to point to the new permanent planning location and `.deployment/local/service-data`.
- Added workspace Cargo configuration to redirect build outputs into `.build/target`, preventing repo-local `target/` growth in the visible solution root.

---

## Notes

- This document is the **single source of truth** for enterprise progress tracking.
- Each work session should be appended with scope, outputs, and validation evidence.
- Update item statuses immediately after completion.
- Phases are sequential by default, except where explicitly marked as parallel-safe.