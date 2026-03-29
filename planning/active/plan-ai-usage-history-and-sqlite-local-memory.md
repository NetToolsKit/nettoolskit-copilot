# AI Usage History and SQLite Local Memory Plan

Generated: 2026-03-29

## Status

- LastUpdated: 2026-03-29 17:59
- Objective: plan the implementation of persisted weekly AI usage history and the migration from the current JSON-backed local context index to a SQLite-backed local RAG/CAG memory system.
- Normalized Request: create a planning workstream for weekly limit-consumption history and create a planning workstream for a local SQLite-based RAG/CAG system similar in spirit to `context-mode`, while keeping the repository operating model and current local-context behavior intact.
- Active Branch: `feature/ai-usage-history-ledger`
- Spec Path: `planning/specs/active/spec-ai-usage-history-and-sqlite-local-memory.md`
- Inputs:
  - `crates/orchestrator/src/execution/ai.rs`
  - `crates/orchestrator/src/execution/ai_session.rs`
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/core/src/local-context/catalog.rs`
  - `crates/core/src/local-context/document.rs`
  - `crates/core/src/local-context/search.rs`
  - `crates/commands/runtime/src/continuity/local_context.rs`
  - `crates/commands/runtime/src/diagnostics/enterprise_trends.rs`
  - `planning/active/plan-repository-consolidation-continuity.md`
  - `planning/specs/active/spec-repository-consolidation-continuity.md`

---

## Scope Summary

This plan coordinates two linked workstreams:

| ID | Workstream | Target | Priority | Dependency |
|---|---|---|---|---|
| U1 | Weekly AI usage history | `crates/orchestrator`, `crates/cli`, local app-data store | 🔴 Immediate | none |
| U2 | SQLite local RAG/CAG memory | `crates/core`, `crates/commands/runtime`, repo-local `.temp/` store | 🔴 Immediate | U1 can proceed in parallel |

The workstreams are linked by continuity and usage telemetry, but they intentionally keep different persistence scopes.

---

## Ordered Tasks

### Workstream U1 — Weekly AI Usage History

Status: `[x]` Complete

#### Task U1.1: Freeze Current AI Usage Capture Baseline

Status: `[x]` Complete

- Audit current usage capture in:
  - `crates/orchestrator/src/execution/ai.rs`
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/orchestrator/src/execution/ai_session.rs`
- Confirm which paths already expose:
  - actual provider usage
  - estimated token usage
  - estimated cost
  - session identifiers
  - intent, model, and provider identity
- Output:
  - implementation note in this plan
  - exact insertion points for ledger writes
- Implementation note:
  - cache-hit persistence is now attached to the existing cache-hit success branch beside `record_ai_usage_estimates(...)` and `persist_ai_session_exchange(...)`
  - provider-success persistence is now attached to the existing `Ok(routed)` success branch after output validation and cache insert
  - the current `AiProvider::stream()` contract still discards `AiResponse.usage`, so this first checkpoint records estimated usage only while keeping actual-usage columns ready for a follow-up stream contract upgrade
- Checkpoint commit:
  - `docs(planning): freeze weekly ai usage capture baseline`

#### Task U1.2: Define SQLite Usage Ledger Contract

Status: `[x]` Complete

- Create the canonical schema contract for the user-local store:
  - `usage_events`
  - optional `weekly_budget_profiles`
  - optional `weekly_usage_snapshots`
- Minimum event columns:
  - `event_id`
  - `timestamp_utc`
  - `iso_year`
  - `iso_week`
  - `provider`
  - `model`
  - `intent`
  - `repo_root`
  - `session_id`
  - `input_tokens_estimated`
  - `output_tokens_estimated`
  - `input_tokens_actual`
  - `output_tokens_actual`
  - `estimated_cost_usd`
  - `actual_cost_usd` when derivable
  - `status`
- Decide retention and pruning policy.
- Delivered in this checkpoint:
  - `usage_events` SQLite schema under the user-local data directory
  - explicit columns for estimated and actual token/cost fields
  - deterministic ISO-week indexing and repo-root filtering
  - optional environment-configured weekly budget burn calculation
- Checkpoint commit:
  - `docs(planning): define weekly ai usage ledger contract`

#### Task U1.3: Add Native Recording Boundary

Status: `[x]` Complete

- Implement a native usage-recorder boundary in orchestrator code after provider completion and after cache-hit execution where relevant.
- Record both:
  - actual usage when available
  - estimated usage fallback when not available
- Guarantee idempotency for a single command execution.
- Add tests for:
  - provider success with actual usage
  - provider success with estimated-only usage
  - repeated writes across same ISO week
- Delivered in this checkpoint:
  - local SQLite usage recorder in `crates/orchestrator/src/execution/ai_usage.rs`
  - persistence on cache-hit and provider-success branches in `processor.rs`
  - idempotent `INSERT OR IGNORE` event writes keyed by deterministic event ids
  - tests for repeated writes, repo-root filtering, partial week validation, and budget projection
- Checkpoint commit:
  - `feat(orchestrator): persist local ai usage events`

#### Task U1.4: Add Weekly Reporting CLI Surface

Status: `[x]` Complete

- Add CLI command surfaces for:
  - `ntk ai usage weekly`
  - `ntk ai usage summary`
- Minimum outputs:
  - weekly total input/output tokens
  - weekly estimated/actual cost
  - top providers/models
  - budget burn percent when weekly budgets are configured
  - “remaining” budget estimate
- Support both text and JSON output.
- Delivered in this checkpoint:
  - `ntk ai usage weekly`
  - `ntk ai usage summary`
  - human-readable and JSON output modes for both commands
  - weekly and multi-week provider/model breakdown
  - current-week budget burn surfaced inside the summary command
  - CLI overrides for budget config path and budget profile selection
- Checkpoint commit:
  - `feat(cli): add weekly ai usage reporting commands`

#### Task U1.5: Add Budget Config and Validation

Status: `[x]` Complete

- Decide config path and schema for optional weekly budget definitions.
- Add validation for invalid budgets and malformed week/profile settings.
- Add docs to the CLI README and runtime/operator docs.
- Delivered in this checkpoint:
  - local budget config document support at `AppConfig::default_data_dir()/ai-usage/budgets.toml`
  - explicit path override via `NTK_AI_USAGE_BUDGET_CONFIG_PATH` and CLI `--budget-config-path`
  - named profile selection via `NTK_AI_WEEKLY_BUDGET_PROFILE` and CLI `--budget-profile`
  - config validation for version, empty profiles, missing defaults, and non-positive cost budgets
  - CLI/operator documentation for `weekly` and `summary` usage plus budget config examples
- Checkpoint commit:
  - `docs(runtime): document configured weekly ai usage budgets`

---

### Workstream U2 — SQLite Local RAG/CAG Memory

Status: `[~]` In Progress

#### Task U2.1: Freeze Current Local-Context Baseline

Status: `[x]` Complete

- Audit the current JSON-backed path in:
  - `crates/core/src/local-context/catalog.rs`
  - `crates/core/src/local-context/document.rs`
  - `crates/core/src/local-context/search.rs`
  - `crates/commands/runtime/src/continuity/local_context.rs`
- Record:
  - current catalog semantics
  - chunk model
  - persisted `index.json` format
  - lexical ranking expectations
  - existing CLI and test coverage
- Delivered in this checkpoint:
  - the JSON baseline remains authoritative for chunk inclusion through `local-context-index.catalog.json`
  - persisted compatibility output remains `index.json` under `.temp/context-index/`
  - lexical retrieval expectations remain path/heading/text deterministic scoring with path/id tie-breaking
  - existing `build_local_context_index(...)` and JSON search tests were frozen as the parity baseline for the SQLite migration
- Checkpoint commit:
  - `docs(planning): freeze local-context json baseline`

#### Task U2.2: Define SQLite Memory Schema and Query Rules

Status: `[x]` Complete

- Create the schema contract for `.temp/context-memory/context.db`.
- Minimum tables:
  - `documents`
  - `chunks`
  - `chunk_fts`
  - `events`
  - `sessions`
  - optional `artifacts`
- Minimum query filters:
  - `repo_root`
  - source kind
  - path exclusion
  - time window
  - top-k
- Define ranking policy:
  - FTS/BM25 primary
  - deterministic tie-breakers by path/id
- Delivered in this checkpoint:
  - repository-local store path fixed at `.temp/context-memory/context.db`
  - schema bootstrap now provisions `documents`, `files`, `chunks`, `chunk_fts`, `events`, `sessions`, `artifacts`, and `schema_metadata`
  - compatibility snapshot mirroring currently writes catalog/document/file/chunk state while keeping `events`, `sessions`, and `artifacts` reserved for later ingestion phases
  - schema versioning is recorded in `schema_metadata` and covered by idempotent initialization tests
- Checkpoint commit:
  - `docs(planning): define sqlite local memory schema`

#### Task U2.3: Implement Dual-Write Memory Builder

Status: `[x]` Complete

- Extend the current local-context update flow so one command can:
  - keep writing `index.json`
  - also write the SQLite repository-local store
- Keep the catalog as the single inclusion authority.
- Preserve current chunking behavior in the first SQLite slice.
- Add migration-safe tests for:
  - schema initialization
  - rebuild
  - incremental update
  - JSON/SQLite count parity
- Delivered in this checkpoint:
  - `build_local_context_index(...)` now dual-writes `index.json` plus the SQLite snapshot in one pass
  - the SQLite writer clears and repopulates the current document/files/chunks/FTS snapshot without touching future continuity tables
  - build reports now surface `memory_root` and `memory_db_path` for downstream runtime/doctor use
  - tests now cover path resolution, schema bootstrap, idempotent initialization, repeated rebuilds, and JSON/SQLite chunk-count parity
- Checkpoint commit:
  - `feat(core): add sqlite local memory dual-write builder`

#### Task U2.4: Implement SQLite Query Boundary

Status: `[ ]` Pending

- Add a native query path for the SQLite memory store.
- New runtime command surfaces:
  - `ntk runtime update-local-memory`
  - `ntk runtime query-local-memory`
- Keep `query-local-context-index` alive until parity is proven.
- Add tests comparing:
  - hit ordering
  - excluded paths
  - top-k behavior
  - heading/path filters
- Checkpoint commit:
  - `feat(runtime): add sqlite local memory query commands`

#### Task U2.5: Add Continuity Event Ingestion

Status: `[ ]` Pending

- Add bounded event ingestion into the SQLite memory store for:
  - planning summary references
  - AI session checkpoint summaries
  - selected runtime task transitions/failures
- Do not ingest raw large outputs.
- Add pruning/TTL policy for events.
- Checkpoint commit:
  - `feat(runtime): add bounded local continuity event memory`

#### Task U2.6: Cut Over the Default Retrieval Path

Status: `[ ]` Pending

- Make SQLite the default local-memory retrieval path only after parity evidence passes.
- Keep JSON export/debug fallback behind an explicit path or flag.
- Update docs and operator guidance.
- Checkpoint commit:
  - `refactor(runtime): make sqlite local memory the default recall store`

---

## Shared Validation Plan

Status: `[ ]` Pending

- `cargo test -p nettoolskit-core`
- `cargo test -p nettoolskit-runtime`
- `cargo test -p nettoolskit-orchestrator`
- `cargo test -p nettoolskit-cli`
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `pwsh -File .\\scripts\\security\\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- targeted parity tests comparing JSON and SQLite local-memory retrieval
- Checkpoint validation executed for the SQLite local-memory foundation slice:
  - `cargo fmt --all -- --check` ✅
  - `cargo check -p nettoolskit-core` ✅
  - `cargo test -p nettoolskit-core --test test_suite local_context::sqlite_tests --quiet` ✅
  - `cargo test -p nettoolskit-core --test test_suite local_context::document_tests --quiet` ✅
- Checkpoint validation executed for this slice:
  - `cargo fmt --all -- --check` ✅
  - `cargo test -p nettoolskit-orchestrator --test test_suite ai_usage --quiet` ✅
  - `cargo test -p nettoolskit-cli --test test_suite ai_usage --quiet` ✅
  - `cargo clippy -p nettoolskit-orchestrator --all-targets -- -D warnings` ✅
  - `cargo check -p nettoolskit-cli` ✅
  - `pwsh -NoProfile -File .\\scripts\\security\\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High` ✅
  - `cargo test -p nettoolskit-orchestrator --quiet` ⚠️ blocked by pre-existing unrelated failure in `execution::chatops::tests::execute_chatops_envelope_submit_records_control_plane_metadata`
  - `cargo clippy -p nettoolskit-cli --all-targets -- -D warnings` ⚠️ blocked by a pre-existing `nettoolskit-validation` warning outside this slice
  - `cargo test -p nettoolskit-orchestrator --test test_suite ai_usage --quiet` ✅
  - `cargo test -p nettoolskit-cli --test test_suite ai_usage --quiet` ✅
  - `cargo check -p nettoolskit-orchestrator` ✅
  - `cargo check -p nettoolskit-cli` ✅

---

## Closeout Conditions

- The weekly usage ledger exists and can answer weekly burn questions locally.
- The AI usage CLI exposes both `weekly` and `summary` reporting with local budget profiles.
- The SQLite local-memory store exists and supports bounded repo-local recall.
- Planning and operator docs explain the split between repo-local memory and user-local usage history.
- The active plan and spec move to `completed/` only after implementation, validation, and documentation are materially finished.