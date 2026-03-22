# Super Agent Token Quality And Runtime Sync Cleanup

Generated: 2026-03-22

## Objective

Protect output quality while pursuing token savings through safer output-side controls, and investigate runtime-sync duplication or garbage under the mirrored `.github` surface that may be causing duplicated Copilot slash commands such as `/super-agent`.

## Normalized Request Summary

The user does not want token economy to reduce quality. They also suspect the runtime synchronization flow is leaving garbage or duplicated surfaces under `.github`, based on duplicated command entries visible in VS Code. The risky input/context compaction should stay reverted, and any future optimization should focus on safer output-side economy plus better local RAG/CAG usage.

## Design Summary

This follow-up must treat token economy as a quality-preserving optimization, not a blind context-cutting exercise. The reverted input/context compaction should stay reverted unless later evidence proves it harmless. The safer path is output-side economy: shorter default responses, less repeated status text, less duplicated logs, and stronger use of local RAG/CAG so retrieval quality stays high without over-explaining the same state repeatedly. That should be backed by a local incremental index/cache so the system can reuse repository knowledge without rereading everything on each turn. In parallel, the runtime-sync flow needs a focused audit to identify whether mirrored `.github` content, duplicate adapters, or duplicate agent-surface registration is causing repeated slash-command entries in VS Code.

The instruction and prompt layer can safely enforce the first slice immediately: concise default responses, delta-focused stage summaries, and an explicit prohibition on trimming required execution context by default purely for token savings.

## Key Decisions

1. Quality remains the primary objective; token economy is acceptable only when it preserves or improves task quality.
2. Input/context compaction is rolled back for now and must not return by default without explicit evidence.
3. The next safe optimization target is output economy: shorter default responses, less duplicated wording, and less repeated orchestration/log narration.
4. Local RAG/CAG usage should be preferred over larger repeated textual restatement when the answer can rely on retrieved local context safely.
5. The local retrieval layer should eventually use an incremental index/cache with add/update/delete invalidation instead of full rereads.
6. The duplicated `/super-agent` entries are treated as a runtime-sync hygiene defect until proven otherwise.
7. Sync cleanup should prefer one canonical runtime authority per surface and should avoid duplicated adapter/agent registration between repo-local and mirrored runtime folders.

## Alternatives Considered

1. Keep tightening context caps immediately.
   - Rejected because the user explicitly does not want quality risk.
2. Ignore the duplicate command entries as a VS Code quirk.
   - Rejected because the screenshot indicates a plausible repo-owned sync/config duplication that should be audited.
3. Optimize only by switching to cheaper models immediately.
   - Rejected because that still requires quality proof and does not address duplicated output or orchestration verbosity.

## Assumptions And Constraints

- The risky input/context token-economy implementation is rolled back and should remain rolled back unless explicitly revisited.
- A future local index/cache should be deterministic, incremental, and delete-safe.
- The duplicate slash-command behavior is likely related to mirrored `.github` runtime sync, local adapter duplication, or overlapping agent/skill registration.
- The follow-up should remain repo-owned and deterministic.
- The workstream is intentionally deferred until the user decides to resume it.

## Risks

- Over-aggressive input/context optimization can reduce task quality or omit needed domain context.
- Output-side economy can still regress operator clarity if summaries become too short or hide failures.
- A stale local index/cache can return outdated context unless invalidation on add/update/delete is reliable.
- Runtime sync may be projecting duplicate command/agent surfaces into VS Code and confusing discovery.
- If the duplicated command source is not isolated precisely, cleanup could remove a valid runtime surface accidentally.

## Acceptance Criteria

1. A follow-up plan exists that explicitly treats quality preservation as a hard requirement for any future token-economy change.
2. The future implementation focuses first on output-side economy: shorter defaults, less duplication, and better local RAG/CAG usage.
3. The future implementation defines a local incremental index/cache with add/update/delete invalidation for safer retrieval reuse.
4. The future runtime-sync audit isolates the exact source of duplicated slash commands or `.github` garbage.
5. The future cleanup defines one canonical registration path per mirrored surface and removes duplicated registration safely.
6. README and runtime docs are updated when that cleanup eventually lands.

## Planning Readiness Statement

Planning is ready. The workstream is clear enough to sequence, but execution is intentionally deferred for later.

## Recommended Specialist Focus

- Primary: `ops-devops-platform-engineer`
- Secondary: `docs-release-engineer`
- Review focus: `review-code-engineer`