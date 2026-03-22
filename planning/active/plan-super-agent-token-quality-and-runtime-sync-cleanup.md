# Super Agent Token Quality And Runtime Sync Cleanup Plan

Generated: 2026-03-22

## Status

- State: active
- Spec: `planning/specs/active/spec-super-agent-token-quality-and-runtime-sync-cleanup.md`

## Objective And Scope

Prepare the deferred follow-up that will keep quality-first behavior, avoid risky input/context trimming, and instead target safer output-side token economy plus an audit of the mirrored `.github` runtime-sync flow for duplicate or garbage command surfaces in VS Code.

## Normalized Request Summary

The user does not want token-economy behavior to reduce quality. They want risky input/context compaction undone and the next optimization wave to focus on safer output-side economy, including shorter default responses, less duplication, and better local RAG/CAG usage. They also suspect the `.github` runtime sync may be leaving duplicate or garbage surfaces, visible as repeated `/super-agent` command entries in VS Code. This should be captured as planning for later, not executed now.

## Ordered Tasks

1. Define safe output-economy rules
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

2. Define local RAG/CAG-first response guidance
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

3. Design a local incremental index for RAG/CAG
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

4. Audit mirrored `.github` runtime-sync duplication
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

5. Define cleanup and validation strategy
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
- Risk: cleanup accidentally removes a legitimate runtime entry.
  - Fallback: gate cleanup behind targeted runtime tests plus doctor/install validation.

## Recommended Specialists

- Implementation: `ops-devops-platform-engineer`
- Documentation: `docs-release-engineer`
- Review: `review-code-engineer`

## Closeout Expectations

- Do not implement this slice until the user explicitly resumes it.
- When resumed, keep quality preservation explicit in README/changelog language.
- Return a commit message only when the deferred cleanup and quality-safe response economy changes are materially implemented and validated.