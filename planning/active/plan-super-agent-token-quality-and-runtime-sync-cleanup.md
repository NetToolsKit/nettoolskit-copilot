# Super Agent Token Quality And Runtime Sync Cleanup Plan

Generated: 2026-03-22 00:00
LastUpdated: 2026-03-23 00:00

## Status

- State: active
- Spec: `planning/specs/active/spec-super-agent-token-quality-and-runtime-sync-cleanup.md`
- Current safe slice implemented: instruction/prompt-level output economy, quality-first routing guidance, session-start continuity summaries, throttled housekeeping with planning export, runtime overflow fixes, context boundary monitoring, dating policy, progress logging, Stop hook for planning export, hardcoded-path removal
- Current urgent slice completed: tasks 1a–1g below
- Remaining deferred scope: local incremental RAG/CAG index plus mirrored `.github` runtime-sync duplication audit (tasks 4–6)

## Objective And Scope

Prepare the deferred follow-up that will keep quality-first behavior, avoid risky input/context trimming, and instead target safer output-side token economy plus an audit of the mirrored `.github` runtime-sync flow for duplicate or garbage command surfaces in VS Code. In the immediate slice, stop Codex local-session disk and token blowups and prune VS Code Copilot workspace trash by applying safer runtime defaults and stale-session cleanup that preserves active context.

## Normalized Request Summary

The user does not want token-economy behavior to reduce quality. They want risky input/context compaction undone and the next optimization wave to focus on safer output-side economy, including shorter default responses, less duplication, and better local RAG/CAG usage. They also suspect the `.github` runtime sync may be leaving duplicate or garbage surfaces, visible as repeated `/super-agent` command entries in VS Code. In parallel, Codex local sessions are consuming extreme disk and likely amplifying token use through long reasoning chains and unbounded local session retention, while VS Code `Code/User/workspaceStorage` is accumulating multi-gigabyte Copilot state such as `GitHub.copilot-chat/local-index*.db` and huge `chatSessions/*.jsonl` files. The immediate safe fix is to enforce saner runtime defaults plus stale-session cleanup for both local runtimes without disabling strategic multi-agent use.

## Ordered Tasks

1. Apply safe Codex and VS Code runtime defaults and cleanup controls ✓ [2026-03-23 00:00]
   - [2026-03-22 00:00] Task 1 (runtime hygiene catalog, codex/vscode hygiene scripts, set-codex-runtime-preferences, install, hooks) — completed in prior session ✓ [2026-03-22 00:00]
   - [2026-03-23 00:00] 1a. Fix Int32 overflow in `clean-codex-runtime.ps1` — `[int]` → `[long]` for `MaxFileSizeBytes`, `MaxSessionFileSizeMB`, `MaxSessionStorageGB`, and `Resolve-OptionalNumericHygieneSetting` ✓ [2026-03-23 00:00]
   - [2026-03-23 00:00] 1b. Add `export-planning-summary.ps1` — standalone context handoff export script, auto-detects repo root, writes to `.temp/context-handoff-<ts>.md` or prints with `-PrintOnly` ✓ [2026-03-23 00:00]
   - [2026-03-23 00:00] 1c. Add `-ExportPlanningSummary` param to `clean-codex-runtime.ps1` and `clean-vscode-user-runtime.ps1` — exports handoff before applying cleanup ✓ [2026-03-23 00:00]
   - [2026-03-23 00:00] 1d. Fix empty-array bugs in `clean-vscode-user-runtime.ps1` — `Test-HasPlannedAncestor` call guarded when `$staleWorkspaceDirectoryPaths.Count -eq 0`; same in `Compress-RemovalPlan` ✓ [2026-03-23 00:00]
   - [2026-03-23 00:00] 1e. Fix hanging at `[7/7]` in `clean-vscode-user-runtime.ps1` — removed recursive `Get-PathStat` pre-scan loop; replaced `Invoke-RemovalPlan` directory size with `[long] 0`; added 7-step progress markers ✓ [2026-03-23 00:00]
   - [2026-03-23 00:00] 1f. Add Stop hook to `.claude/settings.json` — auto-exports planning summary when active plans exist at session close ✓ [2026-03-23 00:00]
   - [2026-03-23 00:00] 1g. Remove all hardcoded `\Users\tguis` paths — `architecture-boundaries.baseline.json`, `CLAUDE.md`, `install.ps1` examples, `.claude/settings.json` Bash permission ✓ [2026-03-23 00:00]
   - [2026-03-23 00:00] 1h. Update `runtime-scripts.tests.ps1` — cover new params (`ExportPlanningSummary`, `RepoRoot`) in both cleanup scripts plus `export-planning-summary.ps1` functional test ✓ [2026-03-23 00:00]

2. Define safe output-economy rules and context boundary monitoring ✓ [2026-03-23 00:00]
   - [2026-03-22 00:00] Output economy rules, quality-first routing — completed in prior session ✓ [2026-03-22 00:00]
   - [2026-03-23 00:00] 2a. Add `## Context Boundary Monitoring` to `super-agent.instructions.md` — planning-first continuity, handoff export usage, cleanup commands, per-runtime session-end semantics ✓ [2026-03-23 00:00]
   - [2026-03-23 00:00] 2b. Add `## Dating Policy (Mandatory)` to `subagent-planning-workflow.instructions.md` — `[YYYY-MM-DD HH:mm]` on all tasks, `Generated`/`LastUpdated` fields, example format ✓ [2026-03-23 00:00]
   - [2026-03-23 00:00] 2c. Add `## Dating Policy (Mandatory)` to `brainstorm-spec-workflow.instructions.md` — `Generated`, key-decision timestamps, planning-readiness `Updated` line ✓ [2026-03-23 00:00]
   - [2026-03-23 00:00] 2d. Add `## Iterative In-Session Directive Exception` to `subagent-planning-workflow.instructions.md` — clarifies when retroactive plan updates replace upfront spec/plan ✓ [2026-03-23 00:00]
   - [2026-03-23 00:00] 2e. Update `scripts/README.md` — add `export-planning-summary.ps1` to directory tree and API reference table ✓ [2026-03-23 00:00]

3. Define local RAG/CAG-first response guidance
   - [2026-03-22 00:00] Task created — deferred
   - Target paths:
     - `.github/AGENTS.md`
     - `.github/copilot-instructions.md`
     - `.github/instructions/repository-operating-model.instructions.md`
     - `.github/instructions/super-agent.instructions.md`
   - Commands:
     - `pwsh -NoLogo -NoProfile -File scripts/validation/validate-instruction-architecture.ps1 -RepoRoot . -WarningOnly:$false`
   - Checkpoints:
     - local repository retrieval stays preferred over repeated restatement
     - future guidance makes output shorter without trimming required working context
     - Copilot and Codex guidance stay aligned

4. Design a local incremental index for RAG/CAG
   - Target paths:
     - `scripts/runtime/**`
     - `scripts/common/**`
     - `.github/governance/**`
     - `.temp/` or future runtime cache/index location
   - Commands:
     - `pwsh -NoLogo -NoProfile -File scripts/runtime/doctor.ps1 -RepoRoot . -RuntimeProfile all -DetailedOutput`
   - Checkpoints:
     - index design covers add/update/delete invalidation
     - retrieval can reuse local code/instruction summaries without rereading everything
     - one canonical local cache/index location is defined

5. Audit mirrored `.github` runtime-sync duplication
   - Target paths:
     - `scripts/runtime/bootstrap.ps1`
     - `scripts/runtime/install.ps1`
     - `scripts/runtime/doctor.ps1`
     - `scripts/common/runtime-paths.ps1`
     - `.github/hooks/**`
     - `.github/agents/**`
     - `.github/skills/**`
   - Commands:
     - `pwsh -NoLogo -NoProfile -File scripts/runtime/doctor.ps1 -RepoRoot . -RuntimeProfile all -DetailedOutput`
     - `pwsh -NoLogo -NoProfile -File scripts/runtime/install.ps1 -RuntimeProfile all -CreateSettingsBackup -ApplyMcpConfig -BackupMcpConfig -GitHookEofMode autofix -GitHookEofScope global`
   - Checkpoints:
     - the exact source of duplicated `/super-agent` discovery is isolated
     - local vs mirrored runtime ownership is mapped clearly
     - any garbage or duplicate registration paths are identified safely

6. Define cleanup and validation strategy
   - Target paths:
     - `README.md`
     - `scripts/README.md`
     - `CHANGELOG.md`
     - `scripts/tests/runtime/**`
     - `scripts/validation/**`
   - Commands:
     - `pwsh -NoLogo -NoProfile -File scripts/validation/validate-readme-standards.ps1 -RepoRoot .`
     - `pwsh -NoLogo -NoProfile -File scripts/validation/validate-runtime-script-tests.ps1 -RepoRoot . -WarningOnly:$false`
     - `pwsh -NoLogo -NoProfile -File scripts/validation/validate-all.ps1 -RepoRoot . -ValidationProfile dev`
   - Checkpoints:
     - future cleanup has runtime tests covering command-surface duplication
     - docs explain the canonical sync authority clearly
     - install/runtime sync remains green after cleanup

## Validation Checklist

- `pwsh -NoLogo -NoProfile -File scripts/validation/validate-instruction-architecture.ps1 -RepoRoot . -WarningOnly:$false`
- `pwsh -NoLogo -NoProfile -File scripts/runtime/doctor.ps1 -RepoRoot . -RuntimeProfile all -DetailedOutput`
- `pwsh -NoLogo -NoProfile -File scripts/validation/validate-readme-standards.ps1 -RepoRoot .`
- `pwsh -NoLogo -NoProfile -File scripts/validation/validate-runtime-script-tests.ps1 -RepoRoot . -WarningOnly:$false`
- `pwsh -NoLogo -NoProfile -File scripts/validation/validate-all.ps1 -RepoRoot . -ValidationProfile dev`
- `pwsh -NoLogo -NoProfile -File scripts/runtime/install.ps1 -RuntimeProfile all -CreateSettingsBackup -ApplyMcpConfig -BackupMcpConfig -GitHookEofMode autofix -GitHookEofScope global`

## Risks And Fallbacks

- Risk: concise output becomes too terse and hides failures or operator guidance.
  - Fallback: keep full summaries mandatory and move detail behind verbose/detailed switches only.
- Risk: local RAG/CAG guidance gets interpreted as license to trim needed working context.
  - Fallback: keep retrieval/local-context guidance focused on answer construction, not on shrinking required execution context by default.
- Risk: duplicated slash commands come from both repo-local and mirrored runtime surfaces.
  - Fallback: map discovery paths first and remove only one duplicated registration path at a time.
- Risk: cleanup accidentally removes a legitimate runtime entry or a still-relevant local session.
  - Fallback: gate cleanup behind targeted runtime tests plus doctor/install validation.

## Recommended Specialists

- Implementation: `ops-devops-platform-engineer`
- Documentation: `docs-release-engineer`
- Review: `review-code-engineer`

## Closeout Expectations

- Do not implement this slice until the user explicitly resumes it.
- When resumed, keep quality preservation explicit in README/changelog language.
- Return a commit message only when the deferred cleanup and quality-safe response economy changes are materially implemented and validated.