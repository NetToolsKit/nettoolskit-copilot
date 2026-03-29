# Spec: AI Usage History and SQLite Local Memory

Generated: 2026-03-29

## Status

- LastUpdated: 2026-03-29 20:41
- Objective: define the design intent, boundaries, and rollout strategy for two related capabilities: persisted weekly AI usage history and a SQLite-backed local RAG/CAG memory system that supersedes the current JSON-only local context index.
- Planning Readiness: ready-for-plan
- Related Plan: `planning/active/plan-ai-usage-history-and-sqlite-local-memory.md`
- Source Inputs:
  - `crates/orchestrator/src/execution/ai.rs`
  - `crates/orchestrator/src/execution/ai_session.rs`
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/core/src/local-context/catalog.rs`
  - `crates/core/src/local-context/document.rs`
  - `crates/core/src/local-context/search.rs`
  - `crates/commands/runtime/src/continuity/local_context.rs`
  - `crates/commands/runtime/src/diagnostics/enterprise_trends.rs`
  - `.github/instructions/super-agent.instructions.md`
  - `.github/instructions/repository-operating-model.instructions.md`
  - `planning/active/plan-repository-consolidation-continuity.md`
  - User reference: `https://github.com/mksglu/context-mode`

---

## Problem Statement

The repository already captures several pieces of local continuity and AI usage state, but they remain fragmented and are not sufficient for weekly quota history or SQLite-backed local memory:

1. **Weekly AI usage history is not persisted as an operator-facing ledger.**
   - `AiUsage` already captures input/output token counts per response.
   - `processor.rs` already estimates token budgets, cost, and emits metrics.
   - `ai_session.rs` already persists local session snapshots.
   - However, there is no persisted usage ledger that aggregates usage by provider/model/session/repository/day/week, no weekly quota projection, and no CLI surface to inspect weekly burn.

2. **Local RAG/CAG exists, but only as a JSON-backed deterministic lexical index.**
   - `local-context` is already implemented and usable.
   - The current persistence format is `index.json`, not SQLite.
   - Retrieval is lexical and deterministic; there is no FTS-backed event memory, no session continuity ledger, and no unified memory store for plan/task/error/user-decision recall.

3. **The current split creates duplicated continuity concerns.**
   - Repository-local context lives under `.temp/context-index/`.
   - AI session history lives under local app data.
   - Validation history uses JSONL ledgers.
   - None of these stores provide a coherent query layer for “what happened this week?” or “what was the last relevant context for this repo/session?”.

4. **The repository now needs a local memory model closer to `context-mode`, but adapted to NTK boundaries.**
   - The target is not a clone of `context-mode`.
   - The target is a repository-appropriate design that keeps deterministic, local-first behavior, preserves existing planning/continuity rules, and uses SQLite/FTS where it materially improves recall and history queries.

---

## Desired Outcome

- `ntk` persists AI usage events in a structured local ledger that supports daily and weekly aggregation.
- Operators can inspect weekly usage history, token burn, estimated cost, and remaining configured weekly budget through a native CLI command.
- The repository-owned local context memory moves from JSON-only storage to a SQLite-backed store with FTS-based retrieval.
- The memory system can retrieve not only indexed repository chunks but also bounded local operational events such as AI session checkpoints, planning summaries, task transitions, and selected runtime diagnostics.
- Existing JSON-based local-context commands continue to work during migration via dual-write or compatibility read mode.
- The final design keeps repository-local retrieval bounded and deterministic, instead of becoming an opaque remote knowledge system.

---

## Design Decisions

### 1. Separate Store Scope by Responsibility

Two local SQLite stores are required because the ownership boundary is different:

1. **Repository-local memory store**
   - Path: `.temp/context-memory/context.db`
   - Scope: repository chunks, planning summaries, selected operational events, bounded continuity data for this repo only
   - Purpose: local RAG/CAG retrieval for repo work

2. **User-local AI usage ledger store**
   - Path: `AppConfig::default_data_dir()/ai-usage/usage.db`
   - Scope: provider/model/session/repo usage events across local runs
   - Purpose: weekly limit history, burn-rate reporting, operator budget visibility
   - Current checkpoint:
     - `ntk ai usage weekly`
     - `ntk ai usage summary`
     - local budget config document at `AppConfig::default_data_dir()/ai-usage/budgets.toml`
     - explicit overrides via `NTK_AI_USAGE_DB_PATH`, `NTK_AI_USAGE_BUDGET_CONFIG_PATH`, and `NTK_AI_WEEKLY_BUDGET_PROFILE`

This avoids mixing repo-operational continuity with cross-repo account usage telemetry.

### 2. SQLite for Persistence, FTS5 for Retrieval

The local memory design should use SQLite as the persistence engine and FTS5 as the retrieval engine where supported.

For repository-local memory:
- canonical row tables hold normalized entities (`documents`, `chunks`, `events`, `sessions`, `artifacts`)
- FTS tables mirror searchable text fields for BM25-style ranking
- retrieval stays bounded by repo id, time window, source type, and top-k

For usage history:
- normalized event rows track each AI interaction
- summary views or query helpers aggregate by ISO week and provider/model
- no FTS is required for weekly usage unless diagnostic search proves useful later

### 3. Keep the Existing JSON Index as a Compatibility Layer During Migration

The current local-context JSON model already has working CLI surfaces and tests.

Migration strategy:
- Phase 1: introduce SQLite writer + reader alongside current JSON persistence
- Phase 2: dual-write JSON + SQLite while query compares both implementations in tests
- Phase 3: make SQLite the default query path, keep JSON export only for debugging/fallback
- Phase 4: retire JSON as the primary runtime store when parity is proven

This preserves current continuity behavior while avoiding a risky flag-day rewrite.

Current implementation checkpoint:
- the repository-local SQLite store now boots under `.temp/context-memory/context.db`
- schema bootstrap provisions `schema_metadata`, `documents`, `files`, `chunks`, `chunk_fts`, `events`, `sessions`, and `artifacts`
- `build_local_context_index(...)` now dual-writes the JSON compatibility document and the SQLite snapshot in one pass
- the first SQLite slice mirrored catalog/document/file/chunk state before continuity ingestion; that additive continuity phase is now live
- build/test evidence now proves path resolution, schema initialization, idempotent bootstrap, repeated rebuilds, and JSON/SQLite count parity
- native command surfaces now include `ntk runtime update-local-memory` and `ntk runtime query-local-memory`
- the SQLite query path is now live with FTS-backed recall plus deterministic `path_prefix`, `heading_contains`, and `exclude_paths` filters while the legacy JSON query remains available for compatibility
- planning-summary exports now persist bounded `planning-summary` events into the repository-local memory store
- orchestrator AI session checkpoints now persist bounded `sessions` rows plus `ai-session-checkpoint` events into the repository-local memory store
- selected queued/failure/cancelled task audit transitions now persist bounded `runtime-task-audit` events using resolved repository-root detection

### 4. Weekly Usage History Must Record Both Actual and Estimated Values

`processor.rs` already computes estimates before provider execution, while `AiResponse.usage` captures actual response usage when available.

The usage ledger must persist:
- `input_tokens_estimated`
- `output_tokens_estimated`
- `input_tokens_actual` when provider returns usage
- `output_tokens_actual` when provider returns usage
- `estimated_cost_usd`
- `provider`, `model`, `intent`, `repo_root`, `session_id`, `timestamp_utc`

This allows the weekly report to distinguish:
- exact usage when the provider reports it
- estimated usage when only local approximation is available

Current implementation checkpoint:
- the first delivered slice persists the full schema for estimated/actual fields
- the active `AiProvider::stream()` pipeline still drops `AiResponse.usage`, so the orchestrator currently records estimated usage for cache-hit and provider-success paths
- a later follow-up should upgrade the stream contract or completion handoff so actual provider usage can populate the existing nullable columns without changing the ledger schema

### 5. Weekly Budgets Are Configured Locally, Not Assumed from a Provider API

The repository should not assume it can read weekly quota limits from external providers.

Instead:
- operators configure optional weekly budgets locally by provider/model/profile
- reports compare observed usage against configured weekly budget
- the CLI clearly marks the result as `configured budget tracking`, not authoritative provider billing
- current implementation checkpoint uses a versioned local TOML document with named profiles and an optional default profile

This avoids false precision and still gives a useful burn-rate view.

### 6. Local Memory Scope Must Stay Explicitly Bounded

The new SQLite memory system must not become an unbounded dump of every tool result.

Allowed memory classes in the first implementation:
- local-context document chunks already admitted by the repository catalog
- planning summaries and active-plan/spec references
- AI session checkpoints and compressed exchanges
- selected runtime events: task id, intent, status transition, failure summary, chosen file refs

Current implementation checkpoint:
- planning-summary persistence stores bounded JSON metadata with active titles and suggested references instead of full markdown handoff bodies
- AI session checkpoints store compressed summaries of the most recent exchanges instead of raw full prompt/response transcripts
- runtime task audit persistence is intentionally limited to the initial queued submission and failure/cancelled transitions, not the full in-memory task audit stream

Rejected for first release:
- raw large tool outputs
- full terminal transcripts
- entire web fetch payloads
- uncontrolled binary/blob content

### 7. Command Surfaces Stay Native and Explicit

New CLI surfaces should be introduced rather than overloading current behavior invisibly.

Planned command families:
- `ntk ai usage weekly`
- `ntk ai usage summary`
- `ntk runtime update-local-memory`
- `ntk runtime query-local-memory`
- optional `ntk runtime memory-doctor`

Current implementation checkpoint:
- `weekly` and `summary` are now native CLI/reporting surfaces
- both commands support JSON/text output
- both commands support explicit budget config path and budget profile selection
- summary reports expose current-week budget burn plus multi-week provider rollups

The current `update-local-context-index` and `query-local-context-index` remain supported during migration.

---

## Alternatives Considered

1. **Keep JSON for local memory and add only a weekly usage JSONL ledger**
   - Rejected: it continues the fragmentation problem and does not deliver the SQLite-backed local memory requested.

2. **Use one single SQLite database for everything under the repo `.temp/` directory**
   - Rejected: weekly usage history is account/user-local and spans sessions beyond a single repository.

3. **Store weekly usage only in OpenTelemetry metrics**
   - Rejected: metrics are insufficient as an operator-facing local history source and are not durable enough for local inspection.

4. **Replace the current local-context system in one cutover**
   - Rejected: too risky given existing tests and continuity flows already depend on the JSON-backed model.

5. **Adopt vector embeddings immediately**
   - Deferred: the first SQLite-based memory release should land with deterministic lexical/FTS retrieval first. Embeddings can be added later if the bounded local-first model proves insufficient.

---

## Risks

| # | Risk | Severity | Mitigation |
|---|---|---|---|
| R1 | SQLite/FTS5 availability differs across Windows environments | Medium | add doctor checks and fallback behavior; keep JSON compatibility during migration |
| R2 | Weekly usage numbers are mistaken for provider-billing truth | High | label reports as configured-budget tracking and distinguish estimated vs actual usage |
| R3 | Dual-write mode causes drift between JSON and SQLite memory stores | Medium | add parity tests comparing hit sets and index counts during migration |
| R4 | Memory ingestion grows without bounds and harms local performance | High | cap source classes, retain bounded chunking, add pruning and TTL policy |
| R5 | Session/event capture stores too much sensitive content locally | High | redact payloads, store summaries instead of raw outputs, keep retention configurable |
| R6 | The new memory model bypasses current planning-first continuity rules | Medium | keep plan/spec summaries as primary resume anchors and use memory as additive recall only |

---

## Acceptance Criteria

### Weekly Usage History
- A persistent local usage ledger exists and survives process restarts.
- A native CLI command reports weekly usage aggregated by provider/model and marks configured budget burn clearly.
- A native CLI summary command reports a bounded recent week window and current-week budget burn.
- The report distinguishes actual usage from estimated usage.
- Tests cover aggregation across multiple days and ISO-week boundaries.

### SQLite Local RAG/CAG
- A SQLite-backed repository-local memory store exists under `.temp/`.
- It can ingest the current local-context document set and query it through a native command.
- FTS-backed retrieval returns bounded top-k results with deterministic filters.
- Current local-context commands remain usable during migration.
- Tests prove parity or acceptable compatibility against the existing JSON-based recall path.

### Operational Safety
- Retention/pruning rules exist for both stores.
- Sensitive content handling is explicit and tested.
- A doctor/check command can verify the local SQLite environment and schema readiness.