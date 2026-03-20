# NetToolsKit CLI - Enterprise Roadmap (2026-02-27)

## Objective
Turn the CLI into a robust enterprise-grade product: stable runtime behavior, secure supply chain, real observability, CI/CD with hard gates, and predictable releases.

## Current Diagnosis (Baseline)
- Modular workspace architecture with `10` crates (good separation of concerns).
- Broad test base (`~467` identified tests) with CI quality gates.
- Terminal UX improved with resize fixes, but some flows were previously width-constrained.
- CI/CD is in place (`.github/workflows/ci.yml` and `release.yml`) with fmt/clippy/build/test/audit.
- Dependency security baseline has no Critical/High Rust advisories.
- Observability supports traces and metrics with optional OTLP export via environment variables.
- Release pipeline includes cross-platform artifacts, checksums, keyless signing, and post-release smoke tests.

## Deliveries Completed Today
- [x] Fixed resize layout reflow in `crates/ui/src/interaction/terminal.rs`.
- [x] Header/logo now re-render on resize to prevent visual corruption.
- [x] Cursor is clamped to visible terminal bounds after resize.
- [x] Initial header became responsive in `crates/cli/src/display.rs` (box/logo/tips by width).
- [x] Added unit tests for width rules and layout metrics.
- [x] Validated with `cargo build -p nettoolskit-cli -p nettoolskit-ui` and `cargo test -p nettoolskit-ui -p nettoolskit-cli`.
- [x] Remediated `cargo audit` vulnerabilities/transitives via lock/dependency updates (`bytes`, `inquire`) and removal of unused `ratatui`.
- [x] Recorded post-fix audit in disposable local audit output.
- [x] Validated templating concurrency stress locally (5 iterations, `--test-threads=16`, no failures).
- [x] Strengthened CI with `Templating Parallel Stress` job (10 iterations) to prevent concurrency regressions.
- [x] Updated README with dynamic `build`, `test`, and `security` badges.
- [x] Removed critical fixed-width behavior in box/menu flows: `manifest` no longer forces `with_width(89)` and `BoxConfig`/`EnumMenuConfig` are responsive.
- [x] Box rendering now clamps to current terminal width and avoids footer truncation underflow.
- [x] Standardized narrow-terminal fallback: compact instructions/prompt and reduced visual metadata below `80` columns.
- [x] Added repeated resize suite (shrink/grow) in `terminal.rs` with burst validation, debounce cycles, and invalid-dimension recovery.
- [x] Published official TUI UX guideline in `docs/ui/tui-ux-guidelines.md` (layout, resize, error states, fallback, engineering checklist).
- [x] Replaced non-interactive `manifest list/check/render` orchestrator placeholders with real execution.
- [x] `manifest check` and `manifest render` now accept explicit CLI path for deterministic execution.
- [x] Strengthened `manifest` E2E coverage with real success/error cases (validation and dry-run render).
- [x] Replaced `/translate` placeholder in orchestrator with real execution plus argument parser (`--from`, `--to`, `path`) and routing to `nettoolskit_translate::handle_translate`.
- [x] Strengthened orchestrator tests for `/translate` (missing-args error + success with temp file).
- [x] Explicitly documented beta scope for `clojure/typescript` in interactive translate flow.
- [x] `process_text` now routes free text to real commands (`help`, `manifest`, `translate`, `config`) using alias/intent heuristics.
- [x] Added free-text routing tests (inference unit tests + integration translate without slash).
- [x] `/manifest apply` without path now opens interactive apply flow (real prompt) instead of immediate error, while keeping real execution when a path is provided.
- [x] Exposed interactive apply flow as public API in `manifest` crate and integrated it in orchestrator.
- [x] Release workflow gained post-release smoke test: downloads draft release artifacts, validates checksums, and executes `--version/--help` on Linux/Windows/macOS.
- [x] Release workflow now validates semver tag format, validates crate version vs tag, and extracts release notes from the exact `CHANGELOG.md` section (deterministic failure when missing).
- [x] Added automated release artifact signing with keyless `cosign` (OIDC), publishing `.sig` and `.pem` for binaries/checksums.
- [x] Published official compatibility matrix and support policy in `COMPATIBILITY.md` and integrated it into release pipeline (required validation + package inclusion).
- [x] Published release artifact verification playbook (`docs/operations/release-artifact-verification.md`) with checksum + keyless cosign flow.
- [x] Defined formal minor support expiration policy with official EOL table in `COMPATIBILITY.md`.
- [x] Strengthened manual release verification workflow to validate signed SBOMs (CycloneDX/SPDX) and minimum metadata headers.
- [x] Added semantic support lifecycle/EOL gate in release pipeline (date order and status coherence).
- [x] Consolidated lifecycle/EOL semantic gate in release workflow using PowerShell (no repository-local versioned script).
- [x] Aligned release workflow to shared pattern: clone `copilot-instructions` via git and use shared validator when available (inline local fallback otherwise).
- [x] Removed legacy aliases in `orchestrator` (`Command`, `get_command`, `definitions` module), migrating CLI to `MainAction/get_main_action`.
- [x] Removed legacy `nettoolskit_manifest::parser` alias and migrated consumers to canonical crate-root `ManifestParser` API.
- [x] Optimized `help` manifest discovery using heavy-directory pruning (`.git`, `target`, `node_modules`, `.idea`, `.vscode`) plus deterministic deduplication.

## Phase 0 - Quality Gates and Foundation (Highest Priority)
Suggested timeline: 2-3 days

- [x] Create CI pipeline (Windows + Linux) with `fmt`, `clippy -D warnings`, `build`, `test`, `audit`.
- [x] Define merge-blocking severity policy (Critical/High blocks merge).
- [x] Pin `rust-toolchain.toml` with Rust version and MSRV.
- [x] Stabilize suite for parallel execution (local stress + dedicated CI concurrency gate for templating).
- [x] Publish build/test/security badges in README.

Acceptance criteria:
- Every PR is automatically gated; merge is blocked on failures.

Validation:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo build --workspace`
- `cargo test --workspace`
- `cargo audit`

## Phase 1 - Runtime and Terminal UX (Stability)
Suggested timeline: 1 week

- [x] Fix resize layout corruption (delivered).
- [x] Remove remaining fixed widths (`with_width(89)` in critical menus/boxes).
- [x] Standardize fallback for narrow terminals (<80 columns).
- [x] Create test suite for repeated resize behavior (shrink/grow).
- [x] Define TUI UX guideline (header, dynamic area, footer, error states).

Acceptance criteria:
- No visual corruption after repeated resize in long interactive sessions.

## Phase 2 - Product Functionality (Feature Completeness)
Suggested timeline: 2-3 weeks

- [x] Finish `manifest check` (real schema/reference/rule validation).
- [x] Finish `manifest render` (artifact preview and diffs).
- [x] Integrate interactive `manifest apply` with real execution handler.
- [x] Complete `translate` beyond .NET (or clearly mark beta limits).
- [x] Implement real free-text processing in orchestrator (formerly placeholder).

Acceptance criteria:
- Core flows are no longer placeholders and include success + error tests.

## Phase 3 - Security and Supply Chain
Suggested timeline: 1 week

- [x] Fix `RUSTSEC-2026-0007` (`bytes` -> `>=1.11.1`).
- [x] Address maintenance/unsound dependency alerts (`fxhash`, `paste`, `lru`).
- [x] Add `cargo-deny` and license policy.
- [x] Generate SBOMs (CycloneDX/SPDX) per release.

Acceptance criteria:
- `cargo audit` has no Critical/High issues and no open vulnerability without documented mitigation.

## Phase 4 - Observability and Operations
Suggested timeline: 1-2 weeks

- [x] Export traces/metrics to OTLP (endpoint configurable by environment variables).
- [x] Define business/runtime metrics (latency by command, error rate, cancellation rate).
- [x] Introduce correlation ID per execution/command.
- [x] Create incident and troubleshooting playbook.

Acceptance criteria:
- Real-environment execution with centralized telemetry and reproducible diagnostics.

## Phase 5 - Release Engineering and Distribution
Suggested timeline: 1 week

- [x] Versioned release pipeline with automated changelog extraction.
- [x] Cross-platform binaries (win/linux/macos) with checksum and signature.
- [x] Post-release smoke test flow.
- [x] Compatibility matrix and support policy.

Acceptance criteria:
- Repeatable release process in one pipeline, with trustworthy artifacts.

## Phase 6 - AI-Assisted Automation (Next Enterprise Expansion)
Suggested timeline: 2 sprints

- [x] Introduce provider abstraction for AI backends (OpenAI-compatible + deterministic mock/local provider).
- [x] Add `/ai` CLI surface (`ask`, `plan`, `explain`, `apply --dry-run`) with streaming responses.
- [x] Add workspace context collection with redaction and token/size budget.
- [x] Enforce explicit approval for all side effects (command execution/file writes).
- [x] Add local-only AI conversation persistence and retention controls.
- [x] Add tests + telemetry budgets for AI workflows.

Acceptance criteria:
- AI assistant is useful for engineering tasks without compromising safety, determinism, or local data control.

## Phase 7 - Dual Runtime Operations (CLI + Background Services/Docker)
Suggested timeline: 2-3 sprints

- [x] Define dual runtime contracts and mode selection (`cli` / `service`) with shared orchestration boundaries.
- [x] Add CLI service-control workflows (`/task submit`, `/task list`, `/task watch`, `/task cancel`).
- [x] Implement embedded background worker runtime for queued task lifecycle with bounded concurrency/retry/cancel policies.
- [x] Implement Dockerized background runtime for queued task orchestration and AI-task management.
- [x] Reuse AI safety controls (approval, rate limiting, retry budgets, observability) in service mode.
- [x] Add Docker local profile (`compose`) with health checks and operational runbook.
- [x] Add dual-mode CI gates (runtime integration + container smoke).

Acceptance criteria:
- Product supports both local interactive execution and background managed execution with deterministic lifecycle and local-first defaults.

## Phase 10 - Commercial Control Plane Hardening
Suggested timeline: 2-3 sprints

- [x] Harden service control plane with loopback-by-default bind, explicit bearer auth for mutating API paths, and fail-closed startup for non-loopback binds without auth.
- [x] Separate liveness from readiness with dependency-aware checks for worker runtime, replay backend, audit persistence, and ChatOps readiness.
- [x] Raise quality bar with critical-file coverage budgets and remove `0%` public entrypoints from release posture.
- [x] Add governance baseline required for commercial OSS operation (`LICENSE`, `SECURITY.md`, `CODEOWNERS`, disclosure flow).
- [x] Replace ad-hoc HTTP transport with framework-grade service stack (`axum`/`hyper`) including auth, tracing, request IDs, and body/timeout middleware.
- [x] Publish formal control-plane/operator/session specification aligned with Codex/OpenClaw-inspired product direction.

Acceptance criteria:
- Service mode is safe-by-default when exposed beyond localhost, and commercial governance/validation gaps are explicitly closed.

## Continuous Backlog (Post-Enterprise Expansion)

- [x] Publish operational guide to verify signed release artifacts (`checksum + cosign keyless`).
- [x] Add manual `workflow_dispatch` workflow to validate an already-published release/tag.
- [x] Define minor support expiration policy (official time window) and EOL table.
- [x] Expand manual release validation to include signed SBOM and metadata sanity checks (`CycloneDX` and `SPDX`).
- [x] Automate semantic validation of EOL table (`EOL = maintenance + 1 day`, date order, and status coherence by reference date).
- [x] Materialize typed control-plane contracts in `nettoolskit-core` so future HTTP/ChatOps/gateway ingress can converge on one envelope model.
- [x] Adopt typed control-plane metadata in service HTTP ingress so `/task/submit` can derive request/operator/session context from headers and return stable envelope metadata.
- [x] Adopt typed control-plane metadata in ChatOps `submit` ingress so Telegram/Discord remote task admission reuses the same envelope model and persists normalized audit metadata.
- [x] Adopt typed control-plane metadata in local CLI `/task submit` so local task admission follows the same envelope model and persists normalized audit metadata.
- [x] Extend typed control-plane attribution to ChatOps `help/list/watch/cancel` so remote management actions keep normalized request/operator/session metadata without changing the existing management handlers.
- [x] Publish formal technical spec for dual runtime architecture (CLI + background service).
- [x] Deliver local Docker baseline for AI/task manager runtime with deterministic smoke checks.
- [x] Deliver autonomous ChatOps agent profile (Telegram/Discord command ingress + notifications + VPS hardening profile).
- [ ] Modernize dependency chain to remove the remaining allowed `cargo audit` warnings (`rustls-pemfile` via `reqwest 0.11`, `windows 0.24.0` via `winrt-notification`).
- [ ] Propagate typed control-plane metadata into outbound Telegram/Discord notifications, not only inbound audit trails.
- [ ] Reuse the real interactive CLI session identifier for local `/task submit` flows.
- [x] Move canonical planning out of `.temp` into tracked root-level `planning/active`.
- [x] Redirect Cargo build outputs outside the repository root to prevent local `target/` bloat.

## Risks and Mitigations
- Risk: feature scope expansion before quality gates are stabilized.
- Mitigation: block new features until Phase 0 is complete.

- Risk: placeholder-related technical debt causing functional regressions.
- Mitigation: phase command rollout with contract tests and incremental rollout.

- Risk: vulnerabilities in transitive dependency chains.
- Mitigation: automated PR auditing + weekly dependency update window.

## Overall Progress
- [x] Resize/layout baseline stabilized.
- [x] Phase 0 completed
- [x] Phase 1 completed
- [x] Phase 2 completed
- [x] Phase 3 completed
- [x] Phase 4 completed
- [x] Phase 5 completed
- [x] Phase 6 (AI-Assisted Automation) completed
- [x] Phase 7 (Dual Runtime Operations) completed
- [x] Phase 10 (Commercial Control Plane Hardening) completed

Estimated current status: enterprise roadmap delivered; remaining work is focused on dependency modernization, notification attribution, and continued operational cleanup.
