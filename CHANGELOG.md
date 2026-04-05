# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added an embedded free-provider harness catalog plus orchestrator validation helpers so every family in the free-provider matrix now shares the same prompt fixture, output contract, latency/error-path expectations, and deterministic offline contract checks before any live smoke test is enabled.
- Added an embedded free-provider matrix catalog under `definitions/templates/manifests/free-llm-provider-matrix.catalog.json` plus an orchestrator `ai_provider_matrix` module so free-tier families, quota hints, compatibility tags, and operator caveats are versioned once and reused by runtime diagnostics/reporting.
- Added canonical agent and skill model-routing policy manifests under `definitions/agents/*` and `definitions/skills/*`, plus a shared orchestrator `ai_model_routing` module, so the development runtime can resolve lane-aware profile/model defaults without hiding them inside provider adapters.
- Added normalized AI provider adapter descriptors for transport, auth, streaming, usage, and fallback-output capabilities so provider-family contracts are inspectable without leaking vendor-specific transport details into orchestration code.
- Added strategy-aware AI provider routing under `crates/orchestrator/src/execution/ai_routing.rs`, with explicit `latency`, `balanced`, and `cost` strategies, scored provider ordering, and shared timeout/provider-chain resolution for both the runtime pipeline and `ntk ai doctor`.
- Added `docs/operations/ai-development-operator-playbook.md` as the stable human-facing runbook for AI profile selection, `ntk ai doctor`, JSON/Markdown diagnostics, local-vs-remote guidance, and degraded-state recovery.
- Added `ntk ai doctor` with JSON output and optional Markdown report generation so operators can inspect active AI profile, provider chain, timeout, auth readiness, and fallback state without executing a request.
- Added built-in AI provider profiles (`balanced`, `coding`, `cheap`, `latency`, `local`) with canonical orchestrator exports, `NTK_AI_PROFILE` resolution, and new `ntk ai profiles list/show` operator surfaces.
- Added an active `Phase 22 orchestration consumer sweep` plan/spec pair so the last 10 `scripts/orchestration/**/*.ps1` wrappers are tracked as their own final blocker-audit domain before the post-Phase-22 retention audit.
- Added an active `Phase 21 security and governance consumer sweep` plan/spec pair so the next eight `scripts/security/*.ps1` and `scripts/governance/*.ps1` leaves are tracked with an explicit checksum-manifest rule before any delete is allowed.
- Added an active `Phase 20 runtime consumer sweep` plan/spec pair so the remaining 30 `scripts/runtime/*.ps1` leaves are now tracked under one dedicated runtime-domain workstream with internal Slice A/B/C boundaries instead of staying implicit under the umbrella continuity backlog.
- Added canonical GitHub provider policy assets under `definitions/providers/github/policies/` so instruction-system and release/security governance can be projected from authored definitions instead of hand-maintained `.github/policies/*` copies.
- Added a focused follow-up workstream, `plan-provider-surface-projection-cutover`, so the remaining generated/runtime `.github/.codex/.claude` cutover is tracked separately from the completed canonical taxonomy migration.
- Added a mirrored canonical GitHub governance catalog set under `definitions/providers/github/governance/`, covering the current `.github/governance/*` authored assets so runtime and validation code can cut over without losing compatibility.
- Added canonical template copies under `definitions/templates/{docs,codegen}` for the shared docs/codegen assets plus the mirrored root `.NET` scaffold tree, so authoring can continue under `definitions/templates/*` while legacy roots remain compatibility-only.
- Added canonical GitHub governance definitions under `definitions/providers/github/governance/` for `instruction-ownership.manifest.json` and `authoritative-source-map.json`, so validation and audit commands can resolve authored policy without starting from `.github/governance/`.
- Added an instruction-taxonomy migration checkpoint that defers `.github/.codex/.claude` projection cutover until canonical `definitions/` assets and validation/audit code are definitions-aware.
- Added the first canonical template copies under `definitions/templates/codegen/` and `definitions/templates/docs/` so provider-facing prompts can stop depending on `.github/templates/`.
- Added `definitions/templates/manifests/runtime-diagnostics.taxonomy.json` plus a matching docs sample and operator playbook so health states, subsystem ownership, degraded-state evidence, and future doctor/report expectations are versioned once for runtime diagnostics.

### Changed
- Changed the root README, the AI development operator playbook, docs manifest samples, and the canonical README/agentic-surface instructions so the free-provider matrix is documented as its own architecture/reporting boundary instead of being conflated with MCP, A2A, RAG, or CAG.
- Changed the root README, docs tree, template manifests, and observability/README instructions so runtime diagnostics now have a canonical taxonomy manifest, a dedicated operator playbook, and an explicit architecture boundary separate from validation and raw logs.
- Changed AI weekly/summary usage reporting so `ntk ai usage weekly|summary` now surfaces a best-effort runtime route snapshot, compatible free-provider families with quota/fallback guidance, and matrix-aware provider classifications when persisted provider ids match a known family alias.
- Closed the `development-agent-orchestrator-experience` umbrella as a completed workstream after profiles, runtime doctor/reporting, smart routing, normalized adapters, the AI operator playbook, and canonical agent/skill model-routing all landed with validated CLI and orchestrator surfaces.
- Closed the `free-provider-test-matrix` workstream as a completed artifact after the canonical provider-family catalog, route-aware usage reporting, documentation/sample surfaces, and deterministic harness coverage all landed with validated CLI/orchestrator coverage.
- Closed the `runtime-operational-diagnostics-and-observability` workstream after the canonical diagnostics taxonomy manifest, operator playbook, README architecture coverage, and observability governance rules landed on top of the existing `ntk ai doctor` proof.
- Changed the AI development-orchestrator umbrella, the free-provider matrix, the multi-agent lineage plan, and token-economy notes to record that canonical agent-to-model routing is now materially implemented and inspectable.
- Changed `ntk ai doctor`, `ntk ai model-routing`, the AI operator playbook, and root/crate READMEs so active agent/skill lanes and their derived profile/model defaults are visible to operators instead of remaining implicit runtime state.
- Changed the development-orchestrator and free-provider-matrix workstreams to record that normalized provider adapter contracts are now materially implemented and surfaced through `ntk ai doctor`.
- Changed the AI development-orchestrator umbrella, free-provider matrix, and token-economy workstreams to record that smart provider routing and observable fallback scoring are now materially implemented.
- Changed `ntk ai doctor`, the AI operator playbook, and root/crate READMEs so routing strategy, ordered candidates, and per-provider scoring are visible to operators instead of staying implicit inside provider failover logic.
- Changed the development-orchestrator umbrella to record that the operator playbook slice is now implemented, with root/crate/documentation READMEs linking to the dedicated playbook instead of duplicating troubleshooting guidance inline.
- Normalized archived planning references so historical phase plans/specs and earlier completed workstreams now point at the completed `repository-consolidation-continuity` umbrella instead of stale `planning/active/*` paths.
- Archived the local `repository-consolidation-continuity` umbrella after the continuity planning sequence, the audit-only Phase 19-22 script-retirement sweeps, the retained-estate proof, and the external W6 Rust-directive handoff left no remaining planning-discovery work open in this repository.
- Closed the generic `script-retirement-tail-cutover` umbrella as a completed planning artifact after the Phase 19-22 consumer sweeps and the post-Phase-22 retention audit proved that the remaining PowerShell estate is fully categorized and now belongs to blocker-reduction implementation work instead of further generic discovery planning.
- Updated the repository-consolidation umbrella/spec to reflect that `copilot-instructions` Phase 8 is no longer blocked on directive discovery: the Rust directive baseline is now versioned externally, and the remaining W6 work is implementation in the other repository.
- Closed the post-Phase-22 retention audit by recording that the live `scripts/**/*.ps1` estate remains `96`, and that the full `63`-script gap above the `retain wrapper intentionally` floor of `33` is completely explained by the five audited blocked domains rather than any unclassified wrapper drift.
- Closed the Phase 22 orchestration consumer sweep as an audit-only workstream after confirming that all engine and stage wrappers still remain pinned by authored pipeline definitions, orchestration policy baselines, orchestrator and validation fixtures, retained runtime tests, and stage chaining.
- Closed the Phase 21 security/governance consumer sweep as an audit-only workstream after confirming that all six security wrappers still remain pinned by the checksum manifest plus skill/doc/runtime/test consumers and both governance wrappers still remain pinned by authored release and branch-protection baselines.
- Closed the Phase 20 runtime consumer sweep as an audit-only workstream after Slice C confirmed that `bootstrap.ps1`, `install.ps1`, `clean-codex-runtime.ps1`, and `clean-vscode-user-runtime.ps1` still have broad authored/runtime consumer fanout, so the runtime-domain backlog stays at `30` retained leaves with an explicit blocker graph instead of unsafe deletions.
- Recorded the Phase 20 Slice B runtime consumer sweep as a second audit-only checkpoint: no orchestration or replay leaf was deletion-ready because authored consumers still remain in orchestration policies, Codex orchestration README surfaces, orchestrator parity tests, validation fixtures, retained runtime parity tests, and the `run/resume/replay` chain itself.
- Recorded the Phase 20 Slice A runtime consumer sweep as an audit-only checkpoint: no `scripts/runtime` projection/profile/sync/workspace leaf was deletion-ready yet because authored consumers still remain in the provider-surface projection catalog, provider README/operator guidance, `install.ps1`, runtime parity tests, and the shell-hook validation fixture.
- Added Windows path normalization to the native `runtime-script-tests` validator so retained PowerShell smoke tests execute correctly even when the repository root is resolved through `\\?\` extended-length paths during CI or commit-time automation.
- Added a dedicated Windows CI gate, `pwsh-runtime-parity`, that runs `ntk validation runtime-script-tests --repo-root . --warning-only false` instead of relying on an implied local-only parity harness or a non-existent Pester suite.
- Stabilized the last two retained runtime parity smoke tests by fixing canonical GitHub provider fixture roots in `scripts/tests/runtime/runtime-scripts.tests.ps1` and by isolating `validate-stage.ps1` from live projected `.github` state in `scripts/tests/runtime/agent-orchestration-engine.tests.ps1` through a fake managed runtime binary override.
- Expanded `scripts/README.md` so the native PowerShell parity command is documented alongside the other supported runtime and validation entrypoints.
- Expanded the root `README.md` and `crates/cli/README.md` so the live `ntk` command surface, runtime/validation groups, shell completions, service mode endpoints, and operator quick-start flows are documented from the current Clap command inventory instead of stale summary bullets.
- Reduced `definitions/providers/github/root/AGENTS.md` and `definitions/providers/github/root/copilot-instructions.md` to thin bootstrap documents so instruction-architecture passes without global-core budget warnings while detailed policy stays in canonical governance and domain instructions.
- Completed the generated provider-surface cutover: `.github`, `.codex`, and `.claude` are now regenerated from canonical `definitions/*` roots for governance, policies, prompts, chatmodes, templates, orchestration prompts, MCP artifacts, and skill/runtime mirrors.
- Changed the GitHub projection pipeline to emit nested `.github/templates/{codegen,docs,manifests,prompts,workflows}` outputs, canonical governance/policy mirrors, and normalized projected relative links so provider consumers stop depending on legacy authored `.github/*` paths.
- Changed planning closeout so `plan-instruction-taxonomy-and-path-refactor` is now a completed authored-root migration workstream, with the remaining generated/runtime projection work narrowed into `plan-provider-surface-projection-cutover`.
- Changed legacy compatibility docs and the repository consolidation umbrella/spec to stop treating `definitions/shared/instructions/*` and `.github/instructions/*` as authored sources; canonical instruction authority is now documented consistently under `definitions/instructions/*`, with `definitions/shared/*` retained only for compatibility and shared prompt assets.
- Changed the authored PowerShell GitHub surface renderer and its smoke harness to project `.github/instructions` from `definitions/instructions` and `.github/templates` from `definitions/templates`, while keeping shared POML prompts under `definitions/shared/prompts/poml` until that lane is migrated.
- Changed the native provider-surface renderer and its Rust/CLI test scaffolds to source GitHub instruction and template projections from `definitions/{instructions,templates}` directly, while leaving only shared POML prompt assets on the legacy `definitions/shared/*` path for now.
- Changed runtime/bootstrap/doctor/healthcheck/self-heal, hook, MCP, and local-context Rust test scaffolds to author governance catalogs under `definitions/providers/github/governance/*` first, while mirroring `.github/governance/*` only for migration-time compatibility.
- Changed canonical security and release baselines plus their Rust/CLI fixtures to require authored evidence from `definitions/providers/github/*` first, while preserving `.github/governance/*` mirrors only as temporary migration compatibility.
- Changed agent-orchestration validation defaults, fixtures, and CLI test scaffolds to resolve the permission matrix and runtime/model routing catalogs from `definitions/providers/github/governance/*` first, while still materializing `.github/governance/*` mirrors for migration-time compatibility.
- Changed shared governance baseline validators (`validate-all`, architecture boundaries, README standards, template standards, and workspace efficiency) plus their Rust/CLI fixtures to prefer `definitions/providers/github/governance/*`, with canonical template files under `definitions/templates/*` and legacy `.github/*` mirrors retained only for transition coverage.
- Added the first shallow projected instruction copies under `.github/instructions/{governance,development,operations,data}` while preserving the legacy projected taxonomy during migration.
- Scaffolded shallow `definitions/instructions/{development,operations,security,data}` copies for backend, frontend, agentic, persistence, runtime, security, database, and privacy guidance while preserving the legacy instruction tree during migration.
- Added the first canonical-root reorganization slice for `definitions/`, introducing scaffolded `instructions/`, `templates/`, `agents/`, `skills/`, and `hooks/` roots plus `docs/samples/manifests/` as the stable human-facing manifest sample lane.
- Added planning baseline for a shallow shared control-surface taxonomy rooted in `instructions/`, `agents/`, `skills/`, and `hooks/`, with `instructions/` limited to five first-level categories and narrower specialization carried by file names.
- Added batched staged-file dispatch to the managed pre-commit EOF hygiene hook so large Windows commits do not fail when hundreds of explicit file paths are staged at once.
- Added a dedicated `agents/` instruction lane so `ntk-agents-super-agent.instructions.md` no longer shares the `core/` lane with repository invariants.
- Added planning workstream for a development-focused AI agent orchestrator covering provider profiles, runtime doctor/report surfaces, smart routing, normalized provider adapters, operator playbook guidance, and agent-to-model routing.
- Added dedicated CI/CD supply-chain hardening instruction covering trusted workflow boundaries, immutable action pinning, OIDC, runner isolation, SBOM, and provenance policy for GitHub Actions.
- Decision log centralized in `CHANGELOG.md` as the single source of truth for architecture/engineering decisions.
- Added `COMPATIBILITY.md` as the official compatibility matrix and support policy for release artifacts.
- Added release verification runbook (`docs/operations/release-artifact-verification.md`) for checksum and keyless cosign validation of published artifacts.
- Added manual release verification workflow (`.github/workflows/release-verify.yml`) for tag-based validation of published assets.
- Added formal support lifecycle and EOL table in `COMPATIBILITY.md` with dated maintenance windows.
- Added SBOM verification coverage (signature + metadata sanity) to the manual release verification workflow.
- Added deterministic PowerShell-based compatibility lifecycle validation in the release workflow to enforce EOL policy semantics.
- Added shared-script pattern to release validation: workflow clones `copilot-instructions` and uses shared lifecycle validator when available (inline fallback retained).
- Added `rustyline`-based CLI input path with persisted history and command auto-complete (with fallback to the legacy input loop if initialization fails).
- Added multiline input support in CLI (`rustyline` validator + explicit trailing `\` continuation marker).
- Added interactive `FilePicker` component with fuzzy filtering, regex mode (`re:`), literal mode (`lit:`), and keyboard navigation for manifest file selection.
- Added interactive `StatusBar` component with mode indicator, bounded notifications queue, command outcome counters, and runtime usage summary.
- Added interactive `HistoryViewer` component with pagination, indexed entry rendering, and case-insensitive filtering.
- Added interactive input syntax highlighting in `rustyline` for commands/flags plus lexical styles for Rust, C#, JavaScript, and TypeScript lines.
- Added `tree-sitter` parser integration for Rust, C#, JavaScript, and TypeScript token-aware interactive highlighting.
- Added cross-platform desktop attention notifications in interactive mode with configurable runtime toggle (`attention_desktop_notification`).
- Added async manifest aliases (`/new-async`, `/render-async`, `/apply-async`) with progress streaming in command execution.
- Added orchestrator runtime command cache module with LRU ordering, per-command TTL, and memory-budget eviction controls.
- Added dedicated Criterion benchmark target (`command_cache`) covering runtime cache insert/hit/miss/eviction paths.
- Added predictive slash-command hints in interactive `rustyline` input for faster command completion guidance.
- Added runtime configuration support for predictive input hints (`predictive_input`) with file/env and `/config` command integration.
- Added orchestrator plugin foundation with in-process registry and safe before/after command hook pipeline.
- Added bounded interactive error-recovery flow for input backends with retry budget and backoff before failing session startup/loop.
- Added panic-safe async task wrapper in CLI interactive runtime to recover from command/text task panics without crashing the full session.
- Added rich CLI state module (`cli::state`) with serializable `CliState`, typed history entries, and shared `Arc<RwLock<_>>` handle for session-scoped state coordination.
- Added local-only interactive session persistence with JSON snapshots (save/load/list/prune) under the OS app data directory and latest-session auto-resume support.
- Added startup local session resume picker (when multiple local snapshots exist), built on `CommandPalette`.
- Added terminal frame scheduler runtime with coalesced frame requests, 60 FPS rate limiting, and async poll-timeout adaptation helpers.
- Added language-aware fenced code block highlighting in Markdown renderer (Rust, C#, JavaScript, TypeScript, JSON, TOML, Bash, PowerShell).
- Added dedicated AI E2E integration tests in orchestrator for `/ai plan`, `/ai apply --dry-run`, safety blocking of mutating apply without approval, and free-text alias routing to AI flows.
- Added explicit CI `AI Gate` job to enforce AI-specific E2E/safety/resilience test slices.
- Added shared runtime contracts in `nettoolskit-core` for dual-mode execution planning (`RuntimeMode`, `TaskIntentKind`, `TaskIntent`, `TaskExecutionStatus`, `TaskAuditEvent`).
- Added embedded background worker runtime in orchestrator for service-mode task execution with bounded queue, concurrency limits, retry backoff, cancellation, and task audit trail.
- Added `ntk service` subcommand with HTTP endpoints (`GET /health`, `GET /ready`, `POST /task/submit`) for local background-service operation.
- Added local Docker service baseline assets: `deployments/Dockerfile.service`, `deployments/docker-compose.local.yml`, and `deployments/service.local.env.example`.
- Added local service-mode operations runbook: `docs/operations/service-mode-local-runbook.md`.
- Added CI `Dual Runtime Gate` job validating runtime-mode contracts, service orchestration tests, service endpoint tests, and Docker compose smoke checks.
- Added ChatOps orchestration foundation in orchestrator (`execution::chatops`) with Telegram/Discord-neutral ingress/notifier contracts, authorization policy, local JSONL audit store, and deterministic mock adapters.
- Added ChatOps VPS operations profile runbook: `docs/operations/chatops-agent-vps-profile.md`.
- Added ChatOps runtime module (`execution::chatops_runtime`) with environment-driven policy/config loader and asynchronous Telegram/Discord adapters for polling and notification dispatch.
- Added Telegram webhook ingress queue mode for ChatOps (`NTK_CHATOPS_TELEGRAM_WEBHOOK_ENABLED`) with service endpoint ingestion (`POST /chatops/telegram/webhook`).
- Added Discord interaction ingress queue mode for ChatOps (`NTK_CHATOPS_DISCORD_INTERACTIONS_ENABLED`) with service endpoint ingestion (`POST /chatops/discord/interactions`).
- Added optional ChatOps ingress security controls for internet exposure: Telegram webhook secret-token validation (`NTK_CHATOPS_TELEGRAM_WEBHOOK_SECRET_TOKEN`), Discord interaction signature validation (`NTK_CHATOPS_DISCORD_INTERACTIONS_PUBLIC_KEY`), and bounded replay protection (`NTK_CHATOPS_INGRESS_REPLAY_WINDOW_SECONDS`, `NTK_CHATOPS_INGRESS_REPLAY_MAX_ENTRIES`).
- Added ChatOps rate-limit strategy controls with `NTK_CHATOPS_RATE_LIMIT_STRATEGY` (`fixed_window`/`token_bucket`) and optional token-bucket burst budgets (`NTK_CHATOPS_RATE_LIMIT_BURST_PER_USER`, `NTK_CHATOPS_RATE_LIMIT_BURST_PER_CHANNEL`).
- Added ChatOps adaptive throttling profile (`NTK_CHATOPS_RATE_LIMIT_AUTOTUNE_PROFILE`) with ingress-driven strategy switching (`conservative`/`balanced`/`aggressive`/`disabled`).
- Added ChatOps replay-cache backend selection (`NTK_CHATOPS_INGRESS_REPLAY_BACKEND=memory|file`) with optional shared file path (`NTK_CHATOPS_INGRESS_REPLAY_FILE_PATH`) for multi-process replay detection.
- Added reverse-proxy reference profiles for ChatOps ingress hardening on VPS:
  - `deployments/reverse-proxy/nginx/ntk-chatops.conf.example`
  - `deployments/reverse-proxy/caddy/Caddyfile.example`
  - `docs/operations/chatops-reverse-proxy-profiles.md`
- Added repository workflow module (`execution::repo_workflow`) with explicit policy gates (host allowlist, command allowlist, push/PR switches), JSON/key-value payload parsing, and deterministic dry-run planning.
- Added scoped ChatOps command authorization (`NTK_CHATOPS_ALLOWED_COMMANDS`) and per-user/per-channel rate-limit controls with auditable throttle notifications.
- Added deterministic ChatOps VPS smoke profile coverage in CI using a local Telegram-compatible mock server (`chatops_vps_smoke_profile_*`).
- Added service automation policy profiles (`strict`/`balanced`/`open`) with explicit allowed-intent controls and queue-admission budgets (`NTK_SERVICE_*`) for `runtime_mode=service`.
- Added deterministic AI provider routing chain for `/ai` (`primary -> secondary`, max depth 2) with configurable route controls (`NTK_AI_PROVIDER_CHAIN`, `NTK_AI_FALLBACK_PROVIDER`) and per-provider timeout budgets (`NTK_AI_PROVIDER_PRIMARY_TIMEOUT_MS`, `NTK_AI_PROVIDER_SECONDARY_TIMEOUT_MS`).
- Added AI token-economy policy for `/ai` and `/task submit ai-*` with request/session token caps, per-request cost cap, prompt compaction tiers, and cache-first response reuse controls (`NTK_AI_TOKEN_BUDGET_*`, `NTK_AI_COST_BUDGET_USD_PER_REQUEST`, `NTK_AI_PROMPT_COMPACTION_TIER`, `NTK_AI_CACHE_FIRST_ENABLED`).
- Added secure tool-scope gateway for `/task submit` and worker execution with service-mode deny-by-default policy, global tool allowlist, intent-level tool scopes, and local JSONL audit proofs (`NTK_TOOL_SCOPE_*`).

### Changed
- Changed the development-orchestrator and runtime-diagnostics workstreams to record that provider-profile inspection and the first AI-specific health/report surface are now materially implemented.
- Changed AI provider/model policy resolution so the orchestrator can layer profile defaults for provider chain, timeout budgets, and model-selection tiers under explicit env overrides without changing existing env precedence.
- Changed validation baseline resolution so `readme-standards`, `workspace-efficiency`, `warning-baseline`, release validation, security baselines, and `validate-all` now prefer canonical governance assets under `definitions/providers/github/governance/*` before falling back to `.github/governance/*`.
- Changed core local-context and runtime-install-profile catalog resolution so repository services prefer canonical governance catalogs under `definitions/providers/github/governance/*` while keeping legacy `.github/governance/*` compatibility for temp repos and older fixtures.
- Changed `validate-instructions` to treat canonical governance JSON assets under `definitions/providers/github/governance/*` as required authored inputs, while keeping `.github/governance/*` only as compatibility or generated-surface mirrors and aligning shared CLI/test fixtures to materialize both paths during migration.
- Added adaptive AI model-selection policy with tiered routing (`cheap` for lightweight intents, `reasoning` for deeper intents) plus tier-specific cost guardrails and optional fallback-to-cheap controls for `/ai` and `/task submit ai-*` (`NTK_AI_MODEL_SELECTION_*`).
- Added compressed local AI session persistence modes (`off`, `delta`, `summary`) with backward-compatible snapshot metadata and operational env controls (`NTK_AI_SESSION_COMPRESSION_*`) to reduce replay/storage overhead.
- Added service-agent SLO bundle for AI runtime with p95 latency, success ratio, tokens/task, and cost/task indicators plus environment-configurable SLO thresholds (`NTK_AI_SLO_*`).
- Added CI `Agent SLO Gate` to run deterministic regression checks for percentile calculation and AI SLO budget compliance paths.
- Added dedicated worker runtime crate (`nettoolskit-task-worker`) and migrated orchestrator queue/dispatch/retry execution to callback-based integration.
- Added service API bearer-token contract for mutable HTTP task submission (`NTK_SERVICE_AUTH_TOKEN`) with explicit `401` rejection on missing or invalid tokens.
- Added real service readiness semantics for `GET /ready`, with dependency checks for task admission, local persistence, replay backend state, ChatOps audit store, and ChatOps startup health.
- Added a CI-enforced critical-file coverage gate using `cargo llvm-cov` JSON output and PowerShell policy checks for public entrypoints.
- Added validated full-workspace coverage evidence for the critical-file policy: `76.65%` lines, `77.10%` functions, and all tracked entrypoints above enforced budgets.
- Added governance baseline artifacts: root `LICENSE`, `SECURITY.md`, and `.github/CODEOWNERS`, plus README/CONTRIBUTING references for commercial OSS operation.
- Added framework-grade service transport on `axum`/`hyper` with request-ID propagation, explicit body-limit handling (`413`), and configurable HTTP timeout middleware (`NTK_SERVICE_HTTP_TIMEOUT_MS`).
- Added formal control-plane/session/operator architecture specification for the dual-runtime platform direction, including current-state contracts and future gateway/operator envelope targets.
- Added typed control-plane contracts in `nettoolskit-core` (`OperatorContext`, `SessionContext`, `ControlPolicyContext`, `ControlEnvelope`) with normalized transport/operator metadata and serialization-safe tests.
- Added first end-to-end service adoption of the shared control-plane model: `/task/submit` now derives request/session/operator metadata from HTTP headers, returns `task_id` plus envelope metadata in accepted responses, and persists the admitted envelope into downstream task registry/audit events.
- Added first ChatOps adoption of the shared control-plane model: remote `submit` intents now derive typed request/operator/session metadata, flow through `process_control_envelope`, and persist normalized metadata (`request_id`, `correlation_id`, `operator_id`, `session_id`, `transport`, `task_id`) into ChatOps audit records.
- Added local CLI adoption of the shared control-plane model for `/task submit`, so local task admission now persists normalized request/operator/session metadata into task registry and audit events before execution.
- Expanded ChatOps control-plane attribution to non-submit commands (`help`, `list`, `watch`, `cancel`), so remote management actions now derive typed request/operator/session/correlation metadata even when execution reuses the existing command handlers.

### Decisions
- **DEC-0001 (Accepted, 2026-02-28): Modular workspace boundaries**
  - Keep a modular Cargo workspace with clear crate responsibilities:
    - `core` (shared models/utilities)
    - `ui` (terminal rendering/interaction)
    - `otel` (telemetry/tracing setup)
    - `orchestrator` (execution flow)
    - `commands/*` (domain command implementations)
    - `cli` (binary entrypoint + interactive loop)
  - Enforce dependency direction from higher-level crates to lower-level crates only.
- **DEC-0002 (Accepted, 2026-02-28): Terminal rendering without alternate screen**
  - Keep rendering in the main terminal buffer (no alternate screen).
  - Preserve output/history on `/quit` and `Ctrl+C`.
  - Use resize debounce and explicit clear/reflow ordering for stability.
  - Keep cursor explicitly visible/blinking in prompt states.
- **DEC-0003 (Accepted, 2026-02-28): Quality gates and lint policy**
  - Hard gates:
    - `cargo fmt --all -- --check`
    - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
    - `cargo test --workspace --all-targets`
    - dependency security audit (`cargo audit` / `cargo-deny` in CI)
  - Lint policy:
    - `clippy::all` as blocking baseline
    - `pedantic`, `nursery`, and `cargo` as advisory by default
- **DEC-0004 (Accepted, 2026-03-01): Hybrid observability model with optional OTLP traces**
  - Keep custom in-process metrics API for fast/local CLI usage.
  - Add optional OpenTelemetry trace export via OTLP, enabled only when endpoint env vars are set.
  - Support OTLP gRPC and HTTP/protobuf protocols with configurable timeout.
- **DEC-0005 (Accepted, 2026-03-01): Correlation ID at session and command boundaries**
  - Add lightweight correlation IDs for interactive session, non-interactive execution, and command dispatch spans.
  - Keep format process-local and dependency-light (`prefix + timestamp + sequence`) for CLI performance.
- **DEC-0006 (Accepted, 2026-03-01): Runtime metrics taxonomy for command operations**
  - Standardize counters, gauges, and timing names for command-level observability.
  - Track command latency, success/error/cancellation rates, and non-command text input volume.
- **DEC-0007 (Accepted, 2026-03-01): Incident response playbook as operational baseline**
  - Establish a single operational runbook for severity classification, triage, mitigation, and post-incident review.
  - Include scenario-specific troubleshooting for terminal resize/layout, command error/cancellation spikes, and OTLP export failures.
- **DEC-0008 (Accepted, 2026-03-01): OTLP metrics export and explicit telemetry shutdown**
  - Mirror in-process runtime metrics to OTLP when metrics endpoint env vars are configured.
  - Keep in-process metrics as the source API while enabling centralized metric pipelines.
  - Trigger explicit telemetry shutdown before process exit to flush traces/metrics in short-lived CLI runs.
- **DEC-0009 (Accepted, 2026-03-01): Pin Rust toolchain to MSRV 1.85.0**
  - Adopt `rust-toolchain.toml` with `1.85.0` to stabilize local and CI behavior.
  - Align MSRV policy with current dependency graph requirements (lockfile v4 and edition2024 dependencies).
- **DEC-0010 (Accepted, 2026-03-01): Release must publish dual-format SBOM assets**
  - Generate SBOM for every tagged release in both CycloneDX and SPDX JSON formats.
  - Publish SBOM files as release assets for supply-chain transparency and auditability.
- Historical ADR files from `docs/adr/` were retired and consolidated into this section.

### Changed
- Runtime catalog readers now prefer canonical governance mirrors under `definitions/providers/github/governance/*` for MCP runtime, provider-surface projection, and git-hook EOF settings, while retaining `.github/governance/*` fallback compatibility for transitional temp repos and generated surfaces.
- Canonical provider runtime docs, sync skills, and orchestration pipeline metadata now point at `definitions/providers/github/governance/*` instead of authored `.github/governance/*` paths.
- Canonical governance, development, and operations instructions now reference `definitions/templates/*` instead of authored `.github/templates/*` paths, and VS Code provider snippets now point at the same canonical template roots.
- `validate-template-standards` and `validate-dotnet-standards` now prefer canonical template content under `definitions/templates/*` and the canonical template baseline at `definitions/providers/github/governance/template-standards.baseline.json`, while retaining legacy `.github/templates/*` fallback compatibility during migration.
- Provider-authored consumer surfaces in `definitions/providers/{claude,codex,github}` now resolve canonical `definitions/instructions/*`, `definitions/templates/*`, `definitions/agents/*`, and `definitions/providers/github/root/*` paths instead of authored `.github/*` references, keeping generated runtime surfaces deferred until the final projection cutover.
- Canonical support assets now follow the shallow `definitions/` taxonomy: routing golden tests, active planning index references, and VS Code provider snippets point to `definitions/instructions/{governance,development,operations,security,data}` and `definitions/agents/super-agent/` instead of the legacy `core/process/architecture/runtime-ops` layout.
- Canonical `definitions/providers/{github,codex,claude}` references now target the shallow `definitions/instructions/{governance,development,operations,security,data}` taxonomy and `definitions/agents/super-agent/`, removing the remaining provider-side dependency on the transitional `core/process/architecture/runtime-ops` path graph.
- `validate-instruction-architecture` and `validate-authoritative-source-policy` now default to canonical `definitions/` assets (`definitions/providers/github/{governance,root,prompts}`, `definitions/templates/`, `definitions/instructions/`, and `definitions/providers/codex/skills`) while keeping compatibility regexes for transitional `core/` references during migration.
- Routing coverage, validate-instructions, and their Rust/CLI fixtures now resolve `instructions/*` through the canonical provider catalog at `definitions/providers/github/root/instruction-routing.catalog.yml`, with temp repos scaffolding `definitions/` as the authored input surface and `.github` treated as projection-only compatibility.
- Repointed `definitions/providers/github/{chatmodes,prompts}` markdown references to canonical `definitions/instructions/*`, `definitions/templates/*`, and `definitions/providers/github/root/*`, which cleared the current `validation instructions` warning set for canonical surfaces.
- Instruction taxonomy documentation now keeps lane discovery in root READMEs instead of adding `README.md` files to every instruction category folder.
- Instruction governance taxonomy now keeps cross-cutting TDD and verification workflow in `process/` while moving Rust crate testing into backend, backend integration/API testing into backend, and browser E2E automation into frontend.
- Instruction governance taxonomy now narrows `runtime-ops` guidance by separating general CI/CD and DevOps platform policy from GitHub Actions-specific workflow authoring.
- Instruction governance taxonomy now narrows `runtime-ops` guidance by separating observability and incident operations from resilience and disaster-readiness policy.
- Instruction governance taxonomy now narrows `runtime-ops` guidance by separating microservice boundary and application-performance policy from Docker, Kubernetes, observability, and resilience guidance.
- Instruction governance taxonomy now narrows `runtime-ops` guidance by separating Docker image/container policy from Kubernetes cluster-manifest and rollout policy.
- Instruction governance taxonomy now narrows `runtime-ops` guidance by separating SonarQube/static-analysis configuration from CI and workflow execution policy.
- Instruction governance taxonomy now replaces the generic `runtime-ops/` lane with semantic `operations/devops`, `operations/automation`, `operations/containers`, `operations/reliability`, and `operations/quality` subdomains.
- Instruction governance taxonomy now replaces the flat `process/` lane with semantic `process/planning`, `process/collaboration`, and `process/delivery` subdomains.
- Instruction governance taxonomy now separates `data/` and `security/` into independent rules-board lanes, keeping database/ORM policy distinct from vulnerability, privacy, and API-security policy.
- Workspace lint policy adjusted to keep CI gate strict on `clippy::all` with `-D warnings`.
- `CHANGELOG.md` aligned to Keep a Changelog structure with an explicit `Unreleased` section.
- `crates/otel` migrated to a hybrid model: optional OTLP trace export plus existing in-process metrics.
- OTLP dependencies were added to workspace/crate manifests (`tracing-opentelemetry`, `opentelemetry`, `opentelemetry_sdk`, `opentelemetry-otlp`).
- Hardened orchestrator task submission test isolation by applying env lock + deterministic env cleanup in the default `ai-plan` submit test path.
- Correlation IDs were introduced and attached to tracing spans at session/execution/command boundaries.
- Runtime/business metrics were defined in orchestrator with stable names for latency, error rate, and cancellation rate.
- Incident response and troubleshooting playbook was added under `docs/operations/` and linked from project README.
- OpenTelemetry support now includes optional OTLP metrics export (`OTEL_EXPORTER_OTLP_METRICS_*` / `NTK_OTLP_METRICS_*`) in addition to trace export.
- OTLP env resolution now supports signal-specific overrides for traces and metrics with shared fallbacks.
- Rust toolchain is now pinned via `rust-toolchain.toml` and CI MSRV check moved to `1.85.0`.
- Release pipeline now generates and publishes SBOM assets in CycloneDX and SPDX formats.
- Release pipeline now validates compatibility/support documentation and ships `COMPATIBILITY.md` inside packaged artifacts.
- Release pipeline now enforces presence of support lifecycle/EOL section and EOL table header in `COMPATIBILITY.md`.
- CI coverage job now exports `lcov`, JSON summary, and HTML report artifacts, and enforces minimum line/function coverage thresholds.
- Manifest interactive commands (`check`, `render`, `apply`) now try picker-based manifest selection first, with manual path input fallback on cancel.
- Interactive CLI loops (`rustyline` and legacy raw-mode fallback) now render a live status bar above the prompt and update status by command outcome.
- Interactive CLI now handles local `/history` command to open session history viewer without delegating to orchestrator command routing.
- Interactive `rustyline` helper now applies lightweight ANSI-based highlighting with language detection and keyword/string/comment styling.
- Interactive syntax highlighting now uses parser reuse + thread-local cache and a bounded large-line fast-path for lower input latency.
- Interactive command outcome signaling now supports optional desktop notifications (Windows toast / macOS `osascript` / Linux `notify-send`) and respects focus-based gating when enabled.
- Orchestrator command routing now recognizes async manifest aliases (top-level and `/manifest *-async` forms) and emits standardized progress messages with percent/step context.
- Interactive CLI loops now route interruption state into orchestrator command execution, enabling runtime-aware command cancellation checks.
- `/help` and `/manifest list` command paths now use bounded runtime cache lookups with cache hit/miss metrics and stale-entry pruning.
- Interactive input startup now wires `predictive_input` from resolved config into `RustylineInput`, allowing runtime enable/disable without code changes.
- Command processor now executes plugin before/after hooks with non-blocking error isolation and plugin observability gauges.
- Interactive loops now apply deterministic recovery policy for `rustyline` and legacy read failures (`3` consecutive failures max) with warning notifications and footer diagnostics.
- Interactive loops now mirror command/text history into shared typed state (`CliState`) while preserving existing history viewer behavior.
- Interactive runtime now seeds in-memory history from resumed local state and persists snapshots on shutdown paths (including interrupted/error exits), with bounded local snapshot retention.
- Interactive startup flow now prompts for local snapshot selection only when multiple session snapshots are available, with fallback to latest snapshot on cancel/error.
- Interactive status bar rendering now goes through frame scheduling (coalesced/rate-limited), and legacy async input polling now uses scheduler-aware timeouts for smoother frame cadence.
- Markdown rendering now applies token-level ANSI styling for fenced code blocks (keywords/strings/numbers/comments) while preserving non-color fallback output.
- Enterprise roadmap Phase 8 (AI Assistant Integration) is now fully delivered, including operational controls and AI-specific release gating.
- Configuration now supports deterministic runtime mode selection (`general.runtime_mode`) with environment override (`NTK_RUNTIME_MODE`), and `/config` supports showing/updating runtime mode.
- `/task submit` now uses runtime-aware execution: immediate local execution in `cli` mode, and asynchronous queued background-worker dispatch in `service` mode.
- `/task submit` now supports `repo-workflow` intent for policy-gated repository automation (`clone -> branch -> execute -> commit -> optional push/PR`) with dry-run default behavior.
- `/task list` and `/task watch` now include retry-attempt metadata and recent audit event history for task lifecycle transparency.
- Non-interactive CLI now supports a long-running service runtime profile via `ntk service --host <host> --port <port>`.
- Service runtime now optionally starts a background ChatOps polling loop when `NTK_CHATOPS_ENABLED=true`, routing remote commands through existing `/task` safety path.
- Service runtime now keeps shared ChatOps runtime state for HTTP handlers and can enqueue Telegram webhook updates directly into local ChatOps ingress queue.
- Service runtime now supports Discord interaction HTTP ingress (`type=1` ping and `type=2` command) with local queue enqueue and deferred interaction acknowledgements.
- Service runtime now validates configured Telegram/Discord ingress security headers/signatures before queue admission and rejects replayed webhook/interaction payloads within configurable window.
- ChatOps ingress throttling now supports burst-aware token-bucket mode while preserving fixed-window as backward-compatible default.
- ChatOps ingress throttling now optionally auto-switches strategy under sustained traffic changes when auto-tuning profile is enabled.
- Service ingress replay protection now supports process-local memory backend and shared file backend for horizontally scaled local/VPS replicas.
- Service/chatops runbooks now include concrete Nginx/Caddy reverse-proxy references for secure internet exposure of ingress endpoints.
- Manual release verification now starts packaged `ntk service` binaries and validates `/health` in `service` runtime mode on Linux, Windows, and macOS.

### Fixed
- Terminal resize stability improvements to avoid duplicated/overlapped UI content on rapid terminal/font-size changes.
- Interactive terminal behavior now preserves visible shell output/history on `/quit` and `Ctrl+C` (no alternate screen wipe).
- Cursor visibility/blinking handling improved in interactive prompt flow.
- Environment-variable race flake fixed in feature-detection tests by synchronizing tests that mutate `NTK_USE_*`.
- OpenTelemetry subscriber layering/type mismatch fixed in `otel` tracing setup (paths with/without OTLP now compile and initialize correctly).
- Non-interactive CLI now calls telemetry shutdown before `process::exit`, preventing loss of buffered OTLP data.
- Async manifest aliases now honor `Ctrl+C` cancellation by aborting in-flight async executor tasks and returning `Interrupted` status.
- Interactive runtime now avoids immediate session termination on transient input backend failures and recovers command/text panics as controlled `Error` outcomes.

### Security
- Dependency hardening and audit cleanups (`cargo audit` baseline cleaned for current lockfile updates).

### Testing
- Quality validation passes in workspace:
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace --all-targets`
  - `cargo doc --workspace --no-deps`
- Additional validation for OTLP migration:
  - `cargo clippy -p nettoolskit-otel --all-targets --all-features -- -D warnings`
  - `cargo test -p nettoolskit-otel --all-targets`
- Added release gate validation for compatibility lifecycle semantics in GitHub Actions.
- Added coverage sweep validation with `cargo llvm-cov` and report exports (line coverage baseline around `68.2%`, functions around `72.5%`).
- Added AI gate validation slices in CI: `e2e_ai_*`, `process_ai_command_*`, and retry/rate-limit resilience tests.
- Added dual-runtime service-mode task tests validating queue submission behavior and worker retry-delay policy semantics.
- Added service CLI tests covering `service --help` command surface and HTTP helper parsing/response routines.
- Added service endpoint handler tests for `GET /health`, invalid JSON rejection on `POST /task/submit`, and accepted task submission responses.
- Added adaptive model-selection tests covering intent-tier routing, env override parsing, tier cost guardrail rejection, fallback-to-cheap behavior, and task-submit guardrail enforcement.
- Added ChatOps unit/integration tests covering remote command parsing, allowlist authorization, local audit persistence, inbox processing, and command execution through `/task` routing.
- Added ChatOps runtime tests for environment configuration parsing and startup gating (disabled mode, missing token validation, list parsing).
- Added ChatOps runtime tests for Telegram webhook payload parsing, queue drain order, and runtime webhook-mode enqueue gating.
- Added service endpoint tests for `POST /chatops/telegram/webhook` covering valid payload enqueue, invalid payload rejection, and disabled-mode conflict response.
- Added ChatOps runtime tests for Discord interaction payload parsing (`ping` + command), queue draining, and runtime interaction-mode enqueue gating.
- Added service endpoint tests for `POST /chatops/discord/interactions` covering ping handshake, command acknowledgement, invalid payload rejection, and disabled-mode conflict response.
- Added service endpoint tests for ingress hardening paths: Telegram secret-token rejection, Discord signature rejection, and replay rejection for duplicate webhook/interaction payloads.
- Added ChatOps rate-limit tests for strategy parsing and token-bucket burst/refill behavior.
- Added ChatOps rate-limit auto-tuning tests for profile parsing and deterministic strategy switching under high/low ingress windows.
- Added service endpoint tests for file-backed replay cache behavior across independent runtime states and backend-unavailable (`503`) path coverage.
- Added ChatOps VPS smoke test slice covering Telegram polling ingress, notification dispatch, and local audit persistence under CI dual-runtime gate.

## [1.0.0] - 2025-01-04

### Changed - Major Architecture Refactoring ✅

#### Created Orchestrator Layer
- **New Crate**: `crates/orchestrator/` for command orchestration
  - Centralized command dispatch and routing
  - Async execution with progress tracking
  - Command models (MainAction, ExitStatus)
  - Clean separation from UI and command implementations

#### CLI Layer Cleanup
- **CLI Now UI-Only**: `crates/cli/` simplified to terminal interface
  - Removed: execution/, models/, handlers/ (moved to orchestrator)
  - Kept: display.rs, events.rs, input.rs (UI concerns only)
  - Dependencies reduced: removed strum, walkdir, inquire, regex, futures, handlebars
  - Clear responsibility: user interaction and display

#### Command Structure Reorganization
- **Removed**: `crates/commands/management/` (deprecated, replaced by orchestrator)
- **Help Command**: Moved from `cli/src/handlers/help.rs` to `crates/commands/help/`
  - Created dedicated `nettoolskit-help` crate
  - Structure matches other commands (manifest)
- **Commands Crate**: Now pure aggregator of command implementations
  - Simplified to re-export help and manifest
  - No orchestration logic

#### Architecture Benefits
- **Clear Separation of Concerns**:
  - CLI: User interface (input, display, events)
  - Orchestrator: Command routing and execution
  - Commands: Business logic implementations
- **Reduced Coupling**: Each layer has minimal dependencies
- **Easier Testing**: Isolated test suites per layer
- **Better Scalability**: Easy to add new commands without touching CLI

#### Testing
- **8 tests passing** across new structure:
  - Orchestrator: 4 tests (execution, progress tracking)
  - Help: 2 tests (discovery handlers)
  - Manifest: 2 tests (apply handlers)
- Clean workspace build with proper dependency graph

## [0.3.0] - 2025-11-10

### Added - Phase 3: Templating Engine Refactoring ✅

#### Architecture
- **Commands Dispatcher**: Implemented thin orchestrator pattern at `crates/commands/`
  - Feature aggregation: Re-exports sub-features as modules
  - Command routing: Unified entry point for CLI commands
  - Error handling: `CommandError` and `Result` types
  - Async support: `AsyncCommandExecutor` with progress tracking

- **Templating Feature**: Relocated to `crates/commands/templating/` (sub-crate)
  - **Strategy Pattern**: Language-specific rendering strategies (6 languages)
  - **Factory Pattern**: `LanguageStrategyFactory` with dynamic strategy creation
  - **Async APIs**: Non-blocking template operations with `tokio`
  - **Caching**: Template caching with 2000x performance improvement
  - **Parallelism**: Batch rendering with 10x speedup via `rayon`
  - **Languages Supported**: DotNet, Java, Go, Python, Rust, Clojure

#### Features
- Template caching with TTL (Time-To-Live) configuration
- Parallel batch rendering for multiple templates
- Path normalization for cross-platform compatibility
- Filename-based template discovery
- TODO marker insertion and detection in generated code

#### Testing
- **33 tests total** (100% passing):
  - 27 integration tests across 6 test files
  - 6 documentation tests
- Test categories:
  - `strategy_tests.rs`: Language strategy validation (6 tests)
  - `factory_tests.rs`: Factory pattern and detection (6 tests)
  - `engine_tests.rs`: Template rendering engine (5 tests)
  - `resolver_tests.rs`: Path resolution and caching (7 tests)
  - `batch_tests.rs`: Parallel batch operations (3 tests)
  - Doc tests: API usage examples (6 tests)

#### Performance
- **Caching**: 2000x speedup for repeated template access
- **Parallelism**: 10x speedup for batch operations
- **Async**: Non-blocking I/O for file operations

#### Documentation
- Created `crates/commands/README.md`: Commands architecture guide
- Updated workspace `README.md`: Architecture overview with 10 crates
- Inline documentation: Comprehensive rustdoc comments
- Usage examples: Doc tests demonstrating API usage

### Changed

#### Structure
- **Relocated**: `crates/templating/` → `crates/commands/templating/`
  - Aligns with architecture plan: features as sub-crates of commands
  - Commands acts as thin dispatcher for features
  - Proper separation of concerns (SOLID principles)

#### Workspace
- Updated `Cargo.toml` workspace members:
  - Removed: `"crates/templating"`, `"crates/commands-old"`
  - Added: `"crates/commands"`, `"crates/commands/templating"`
- Fixed all dependency paths in referencing crates
- Re-exported templating through commands public API

#### API
- **Public API**: `nettoolskit_commands::templating` module
- **Error Types**: Unified `CommandError` across all features
- **Async Executor**: `AsyncCommandExecutor` for long-running operations

### Technical Details

#### Workspace Configuration
```toml
[workspace]
members = [
    "crates/cli",
    "crates/core",
    "crates/commands",
    "crates/commands/templating",  # Feature sub-crate
    "crates/ui",
    "crates/otel",
    "crates/ollama",
    "crates/shared/async-utils",
    "crates/shared/file-search",
    "crates/shared/utils",
]
```

#### Commands Public API
```rust
// Re-export templating feature
pub use nettoolskit_templating as templating;

// Access via commands
use nettoolskit_commands::templating::{
    TemplateEngine,
    LanguageStrategyFactory,
    BatchRenderer,
};
```

#### Test Results
```bash
cargo test -p nettoolskit-templating
# Running 33 tests
# test result: ok. 33 passed; 0 failed
```

### Migration Notes

#### For Contributors
1. **New Location**: Templating code now at `crates/commands/templating/`
2. **Import Path**: Use `nettoolskit_commands::templating` instead of `nettoolskit_templating`
3. **Tests**: Run `cargo test -p nettoolskit-templating` for feature tests
4. **Workspace**: Build with `cargo check --workspace` to verify all references

#### For Maintainers
- Phase 3 objectives: ✅ Complete
- Architecture alignment: ✅ Confirmed
- Test coverage: ✅ 100% (33/33 tests passing)
- Performance: ✅ Validated (2000x cache, 10x parallel)
- Documentation: ✅ Updated

---

## [0.4.0] - 2025-11-12

### Added - Phase 4: Manifest Feature ✅

#### Architecture
- **Manifest Feature**: Implemented at `crates/commands/manifest/` (sub-crate)
  - **Domain Models**: Complete manifest structure in `models.rs` (469 lines)
  - **Parser**: YAML parsing and validation in `parser.rs`
  - **Executor**: Orchestration engine in `executor.rs`
  - **Task Generation**: Layer-based task builders (domain, application, api)
  - **File Operations**: Collision detection and file management
  - **Rendering Integration**: Delegates to templating crate

#### Features
- **YAML-based Configuration**: Define entire code generation workflows declaratively
- **DDD Support**: First-class support for bounded contexts, aggregates, entities, value objects, domain events
- **Multiple Apply Modes**: Feature, Artifact, and Layer-based generation
- **Template Orchestration**: 55 templates mapped to 7 ArtifactKind types
- **Dry-Run Mode**: Preview changes before applying
- **Collision Detection**: Configurable policies (fail/overwrite)
- **Async Execution**: Non-blocking operations with progress tracking

#### Artifact Kinds
- **ValueObject**: DDD Value Objects
- **Entity**: DDD Entities
- **DomainEvent**: Domain Events
- **RepositoryInterface**: Repository Interfaces
- **EnumType**: Enumerations
- **UseCaseCommand**: CQRS Commands/Queries
- **Endpoint**: API Controllers/Endpoints

#### Templates
- **55 Templates** in `templates/dotnet/`:
  - Domain layer: entity.hbs, value-object.hbs, domain-event.hbs, enum.hbs
  - Application layer: command.hbs, command-handler.hbs, query.hbs, query-handler.hbs
  - API layer: controller.hbs, endpoint.hbs
  - Infrastructure layer: repository-interface.hbs, repository-impl.hbs

#### Testing
- **87 tests total** (100% passing):
  - 11 async tests: Async execution, timeouts, concurrency
  - 17 error tests: All error types, propagation, display
  - 8 executor tests: Configuration, execution, dry-run
  - 10 file tests: Create, update, collision handling
  - 7 integration tests: End-to-end workflows with real templates
  - 15 model tests: Domain models, serialization
  - 10 parser tests: YAML parsing, validation
  - 8 task tests: Task generation, filtering

#### Documentation
- Created `crates/commands/manifest/README.md`: Comprehensive feature guide (1000+ lines)
- Created test fixtures in workspace `tests/fixtures/`:
  - `ntk-manifest.yml`: Complete DDD example (285 lines)
  - `ntk-manifest-minimal.yml`: Minimal test manifest (50 lines)
  - `ntk-manifest-domain.yml`: Domain-focused example
  - `templates/`: Copy of workspace templates for integration tests
- Inline documentation: Comprehensive rustdoc comments

### Changed

#### Integration
- **Commands Dispatcher**: Added manifest re-export in `crates/commands/src/lib.rs`
  - Public API: `nettoolskit_commands::manifest`
  - Re-exports: `ManifestExecutor`, `ExecutionConfig`, `ManifestDocument`, `ApplyModeKind`

#### Testing
- **Integration Test**: Enabled `test_integration_full_workflow_with_templates`
  - Removed `#[ignore]` attribute
  - Uses test fixtures from workspace `tests/fixtures/` (with templates nearby)
  - Validates full workflow with dry-run mode
  - Now passing (was 1 ignored, now 7 passing)

### Technical Details

#### Manifest Structure
```yaml
apiVersion: ntk/v1
kind: solution

meta:
  name: my-project

solution:
  root: ./src
  slnFile: MyProject.sln

conventions:
  namespaceRoot: MyProject
  targetFramework: net9.0
  policy:
    collision: fail

templates:
  mapping:
    - artifact: entity
      template: templates/dotnet/src/domain/Entities/entity.hbs
      dst: Domain/Entities/{name}.cs

contexts:
  - name: Orders
    aggregates:
      - name: Order
        entities:
          - name: OrderItem

apply:
  mode: feature
  feature:
    context: Orders
```

#### Commands Public API
```rust
// Re-export manifest feature
pub use nettoolskit_manifest as manifest;

// Access via commands
use nettoolskit_commands::manifest::{
    ManifestExecutor,
    ExecutionConfig,
    ManifestDocument,
};
```

#### Test Results
```bash
cargo test -p nettoolskit-manifest
# Running 87 tests
# test result: ok. 87 passed; 0 failed
```

#### Apply Modes

**Feature Mode** - Generate all artifacts for a bounded context:
```yaml
apply:
  mode: feature
  feature:
    context: Orders
    include: [entity, value-object, usecase-command]
```

**Artifact Mode** - Generate specific artifacts by type:
```yaml
apply:
  mode: artifact
  artifact:
    kind: entity
    context: Orders
    name: OrderItem
```

**Layer Mode** - Generate by architectural layer:
```yaml
apply:
  mode: layer
  layer:
    include: [domain, application]
```

### Migration Notes

#### For Contributors
1. **New Location**: Manifest code now at `crates/commands/manifest/`
2. **Import Path**: Use `nettoolskit_commands::manifest` instead of direct dependency
3. **Tests**: Run `cargo test -p nettoolskit-manifest` for feature tests
4. **Test Fixtures**: Located in workspace `tests/fixtures/` with templates copy

#### For Maintainers
- Phase 4 objectives: ✅ Complete
- Architecture alignment: ✅ Confirmed (Clean Architecture with ports/adapters)
- Test coverage: ✅ 100% (87/87 tests passing)
- Template coverage: ✅ 55 templates for 7 artifact kinds
- Documentation: ✅ Complete (README.md + examples)

### Performance
- **Async Operations**: Non-blocking I/O for file operations
- **Template Caching**: Shared with templating crate (2000x speedup)
- **Task Generation**: Efficient filtering and aggregation

---

## [0.2.0] - 2025-11-08

### Added
- Interactive TUI with command palette
- Async command execution with progress tracking
- File search utilities with glob pattern support
- OpenTelemetry integration for observability

### Changed
- Refactored UI components using ratatui
- Improved error handling with thiserror

---

## [0.1.0] - 2025-11-01

### Added
- Initial CLI implementation
- Basic template rendering
- Command processing infrastructure
- Core types and utilities

---

[Unreleased]: https://github.com/ThiagoGuislotti/NetToolsKit/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/ThiagoGuislotti/NetToolsKit/releases/tag/v1.0.0
[0.4.0]: https://github.com/ThiagoGuislotti/NetToolsKit/releases/tag/v0.4.0
[0.3.0]: https://github.com/ThiagoGuislotti/NetToolsKit/releases/tag/v0.3.0
[0.2.0]: https://github.com/ThiagoGuislotti/NetToolsKit/releases/tag/v0.2.0
[0.1.0]: https://github.com/ThiagoGuislotti/NetToolsKit/releases/tag/v0.1.0