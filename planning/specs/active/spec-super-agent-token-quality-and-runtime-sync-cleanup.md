# Super Agent Token Quality And Runtime Sync Cleanup

Generated: 2026-03-22

## Objective

Protect output quality while pursuing token savings through safer output-side controls, investigate runtime-sync duplication or garbage under the mirrored `.github` surface that may be causing duplicated Copilot slash commands such as `/super-agent`, and immediately stop local Codex session/runtime blowups plus VS Code Copilot workspace bloat that are consuming excessive disk and likely inflating token use.

## Normalized Request Summary

The user does not want token economy to reduce quality. They also suspect the runtime synchronization flow is leaving garbage or duplicated surfaces under `.github`, based on duplicated command entries visible in VS Code. The risky input/context compaction should stay reverted, and any future optimization should focus on safer output-side economy plus better local RAG/CAG usage. In parallel, local Codex sessions have grown into multi-gigabyte JSONL files dominated by compacted history, encrypted reasoning, and repeated worker/subagent context, while VS Code `workspaceStorage` now contains multi-gigabyte Copilot workspace indexes and huge chat session payloads. The runtime now needs safer defaults and stronger cleanup to prevent both from recurring.

## Design Summary

This follow-up must treat token economy as a quality-preserving optimization, not a blind context-cutting exercise. The reverted input/context compaction should stay reverted unless later evidence proves it harmless. The safer path is output-side economy: shorter default responses, less repeated status text, less duplicated logs, and stronger use of local RAG/CAG so retrieval quality stays high without over-explaining the same state repeatedly. That should be backed by a local incremental index/cache so the system can reuse repository knowledge without rereading everything on each turn.

The urgent runtime slice is separate and mechanical: it does not trim execution context. Instead, it applies safer local Codex defaults that directly reduce runaway session amplification:
- keep the default reasoning effort at a safer repository-owned level
- keep local multi-agent mode enabled, but rely on the repository-owned Super Agent guidance to call subagents strategically instead of for trivial work
- enforce session cleanup primarily by stale age (`LastWriteTime`) so active conversations keep their context by default
- leave oversized-file thresholds and total session-storage budgets available only as explicit override paths for emergency cleanup
- wire these controls into install and runtime hooks so users do not need to remember manual cleanup

The same urgent slice must also harden the VS Code global user runtime:
- lower the managed global chat request budget from repository-owned runaway values to a safer cap
- stop restoring the last chat panel session by default
- stop keeping empty-state chat history by default
- prune stale `workspaceStorage` directories, old Copilot session/transcript files, old `History` files, old `settings.json.*.bak` and `mcp.json.*.bak` files, and oversized `GitHub.copilot-chat/local-index*.db` files
- throttle hook-driven VS Code cleanup so post-commit does not regress into a slow path again

In parallel, the runtime-sync flow still needs a focused audit to identify whether mirrored `.github` content, duplicate adapters, or duplicate agent-surface registration is causing repeated slash-command entries in VS Code.

The instruction and prompt layer can safely enforce the first slice immediately: concise default responses, delta-focused stage summaries, and an explicit prohibition on trimming required execution context by default purely for token savings.

The next safe continuity slice can also ship immediately: the VS Code `SessionStart` and `SubagentStart` bootstrap hooks should inject a short continuity summary derived from the current active plan/spec artifacts. That gives the controller and workers a bounded restart point after context compaction or a new session, without relying on replaying oversized chat history.

## Key Decisions

1. Quality remains the primary objective; token economy is acceptable only when it preserves or improves task quality.
2. Input/context compaction is rolled back for now and must not return by default without explicit evidence.
3. The next safe optimization target is output economy: shorter default responses, less duplicated wording, and less repeated orchestration/log narration.
4. Local RAG/CAG usage should be preferred over larger repeated textual restatement when the answer can rely on retrieved local context safely.
5. The local retrieval layer should eventually use an incremental index/cache with add/update/delete invalidation instead of full rereads.
6. The duplicated `/super-agent` entries are treated as a runtime-sync hygiene defect until proven otherwise.
7. Sync cleanup should prefer one canonical runtime authority per surface and should avoid duplicated adapter/agent registration between repo-local and mirrored runtime folders.
8. Local Codex runtime defaults must bias toward predictable cost and disk growth without disabling strategic multi-agent behavior.
9. Runtime cleanup must protect active sessions by default, with stale-session retention as the normal path and oversized-file or storage-budget pruning reserved for explicit overrides.
10. VS Code user-runtime cleanup must protect recent active work by using retention windows plus grace windows and by throttling repeated scans.
11. Session bootstrap continuity should come from the active plan/spec artifacts so context compaction can recover from versioned repository state instead of large chat transcripts.

## Alternatives Considered

1. Keep tightening context caps immediately.
   - Rejected because the user explicitly does not want quality risk.
2. Ignore the duplicate command entries as a VS Code quirk.
   - Rejected because the screenshot indicates a plausible repo-owned sync/config duplication that should be audited.
3. Optimize only by switching to cheaper models immediately.
   - Rejected because that still requires quality proof and does not address duplicated output or orchestration verbosity.
4. Keep `multi_agent = true` as the global local default.
   - Accepted for the local runtime because the user wants subagents available; the remaining control point is strategic use, not blanket disablement.

## Assumptions And Constraints

- The risky input/context token-economy implementation is rolled back and should remain rolled back unless explicitly revisited.
- A future local index/cache should be deterministic, incremental, and delete-safe.
- The duplicate slash-command behavior is likely related to mirrored `.github` runtime sync, local adapter duplication, or overlapping agent/skill registration.
- The follow-up should remain repo-owned and deterministic.
- The output-economy and local-index portions remain deferred until the user decides to resume them.
- The Codex runtime blowup controls are safe to implement immediately because they do not trim required execution context.

## Risks

- Over-aggressive input/context optimization can reduce task quality or omit needed domain context.
- Output-side economy can still regress operator clarity if summaries become too short or hide failures.
- A stale local index/cache can return outdated context unless invalidation on add/update/delete is reliable.
- Runtime sync may be projecting duplicate command/agent surfaces into VS Code and confusing discovery.
- If the duplicated command source is not isolated precisely, cleanup could remove a valid runtime surface accidentally.
- If runtime cleanup thresholds are too aggressive, they could remove older large sessions the user still wanted to inspect.

## Acceptance Criteria

1. A follow-up plan exists that explicitly treats quality preservation as a hard requirement for any future token-economy change.
2. The future implementation focuses first on output-side economy: shorter defaults, less duplication, and better local RAG/CAG usage.
3. The future implementation defines a local incremental index/cache with add/update/delete invalidation for safer retrieval reuse.
4. The future runtime-sync audit isolates the exact source of duplicated slash commands or `.github` garbage.
5. The future cleanup defines one canonical registration path per mirrored surface and removes duplicated registration safely.
6. README and runtime docs are updated when that cleanup eventually lands.
7. Codex runtime install/onboarding applies safer local reasoning defaults while keeping multi-agent available by default and without trimming required execution context.
8. Codex session cleanup defaults now enforce stale-session retention after 30 days without update, with documented environment overrides for optional oversized-file and storage-budget emergency cleanup.
9. Managed global VS Code settings stop favoring runaway Copilot chat history/restore/request budgets.
10. VS Code user-runtime cleanup defaults now enforce age plus oversized-file caps with documented environment overrides and a throttled hook execution window.
11. Session bootstrap hooks inject a short continuity summary that references the latest active plan/spec artifact and tells the agent to resume from those artifacts first after context compaction.

## Planning Readiness Statement

Planning is ready. The workstream is clear enough to sequence, but execution is intentionally deferred for later.

## Recommended Specialist Focus

- Primary: `ops-devops-platform-engineer`
- Secondary: `docs-release-engineer`
- Review focus: `review-code-engineer`