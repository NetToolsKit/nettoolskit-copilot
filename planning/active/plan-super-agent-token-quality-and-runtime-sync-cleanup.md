# Super Agent Token Quality And Runtime Sync Cleanup Plan

Generated: 2026-03-22

## Status

- State: active
- Spec: `planning/specs/active/spec-super-agent-token-quality-and-runtime-sync-cleanup.md`
- Current safe slice implemented: instruction/prompt-level output economy and quality-first routing guidance
- Current urgent slice in progress: Codex plus VS Code runtime bloat controls for safer hygiene defaults, strategic multi-agent use, and stale-session cleanup of local session/workspace artifacts
- Remaining deferred scope: local incremental RAG/CAG index plus mirrored `.github` runtime-sync duplication audit

## Objective And Scope

Prepare the deferred follow-up that will keep quality-first behavior, avoid risky input/context trimming, and instead target safer output-side token economy plus an audit of the mirrored `.github` runtime-sync flow for duplicate or garbage command surfaces in VS Code. In the immediate slice, stop Codex local-session disk and token blowups and prune VS Code Copilot workspace trash by applying safer runtime defaults and stale-session cleanup that preserves active context.

## Normalized Request Summary

The user does not want token-economy behavior to reduce quality. They want risky input/context compaction undone and the next optimization wave to focus on safer output-side economy, including shorter default responses, less duplication, and better local RAG/CAG usage. They also suspect the `.github` runtime sync may be leaving duplicate or garbage surfaces, visible as repeated `/super-agent` command entries in VS Code. In parallel, Codex local sessions are consuming extreme disk and likely amplifying token use through long reasoning chains and unbounded local session retention, while VS Code `Code/User/workspaceStorage` is accumulating multi-gigabyte Copilot state such as `GitHub.copilot-chat/local-index*.db` and huge `chatSessions/*.jsonl` files. The immediate safe fix is to enforce saner runtime defaults plus stale-session cleanup for both local runtimes without disabling strategic multi-agent use.

## Ordered Tasks

1. Apply safe Codex and VS Code runtime defaults and cleanup controls
   - Target paths:
     - `.github/governance/codex-runtime-hygiene.catalog.json`
     - `.github/governance/vscode-runtime-hygiene.catalog.json`
     - `scripts/common/codex-runtime-hygiene.ps1`
     - `scripts/common/vscode-runtime-hygiene.ps1`
     - `scripts/runtime/set-codex-runtime-preferences.ps1`
     - `scripts/runtime/clean-codex-runtime.ps1`
     - `scripts/runtime/clean-vscode-user-runtime.ps1`
     - `.vscode/settings.tamplate.jsonc`
     - `.github/governance/workspace-efficiency.baseline.json`
     - `scripts/runtime/install.ps1`
     - `.githooks/post-commit`
     - `.githooks/post-merge`
   - Commands:
     - `pwsh -NoLogo -NoProfile -File scripts/tests/runtime/runtime-scripts.tests.ps1 -RepoRoot .`
     - `pwsh -NoLogo -NoProfile -File scripts/tests/runtime/install-runtime.tests.ps1 -RepoRoot .`
   - Checkpoints:
     - Codex config defaults keep `multi_agent` enabled while still applying repository-owned runtime hygiene defaults
     - session cleanup defaults to LastWriteTime retention for conversations older than 30 days without update
     - oversized-file and storage-budget pruning remain available only through explicit override paths
     - VS Code global settings stop biasing Copilot toward runaway session restore/history/request budgets
     - VS Code cleanup prunes stale workspaceStorage, old History, old backup files, and oversized Copilot local indexes safely
     - install applies the safe runtime preferences deterministically
     - post-commit/post-merge cleanup uses catalog-driven defaults instead of hardcoded 30-day retention and throttles the heavier VS Code cleanup path

2. Define safe output-economy rules
   - Target paths:
     - `.github/AGENTS.md`
     - `.github/copilot-instructions.md`
     - `README.md`
     - `scripts/README.md`
     - `scripts/common/repository-paths.ps1`
     - `scripts/common/runtime-operation-support.ps1`
     - `scripts/common/validation-logging.ps1`
   - Commands:
     - `pwsh -NoLogo -NoProfile -File scripts/validation/validate-readme-standards.ps1 -RepoRoot .`
   - Checkpoints:
     - default responses stay concise without hiding errors or summaries
     - verbose/detailed modes remain the place for full diagnostics
     - duplication between step logs, summaries, and closeout text is reduced safely

3. Define local RAG/CAG-first response guidance
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