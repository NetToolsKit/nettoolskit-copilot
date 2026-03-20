# Global Instructions

Language: pt-BR for chat; EN for code/commits/docs/UI/database; pt-BR i18n output.

# Enterprise-First Default
- Default quality bar is real-world enterprise standard for all tasks.
- Target the highest feasible quality level by default in planning, implementation, validation, and documentation.
- Plan and execute with high rigor in security, reliability, observability, testing, documentation, and maintainability.
- Only downgrade to lightweight/prototype mode when the user explicitly labels the request as `POC`, `spike`, or `informal test`.
- Even in POC/informal mode, keep baseline safety controls (no secrets exposure, no unsafe destructive actions).

# Language Policy
- Chat/Conversation: pt-BR (Portuguese) - all responses to user in chat
- Code/Commits/Docs: EN (English) - all technical content
- UI: EN (English) keys/structure; pt-BR translations via i18n for end users
- Database: EN (English) - schema, table names, column names

# Hierarchy and Scope
- Global rules live here and are always applied.
- The MASTER lifecycle lives in `instructions/master-orchestrator.instructions.md` and is always applied for change-bearing work.
- Repository-specific operating rules live in `instructions/repository-operating-model.instructions.md`.
- Domain instruction files extend these rules; do not duplicate globals.
- Prefer the most specific domain rule when conflicts occur.
- Map and reference new instruction files here.

# Context Selection

## Hard rule
- Always load `AGENTS.md` first, then this file.
- If the workspace is available, do not proceed unless both files are loaded.

# Static RAGs Routing
Preferred default workflow: **Route → Execute** (always route first to generate a minimal Context Pack).

Use static routing when you want consistent instruction selection without running any external service.

Flow (two-stage):
1) Route: Use instruction-routing.catalog.yml + prompts/route-instructions.prompt.md to produce a Context Pack (mandatory + minimal domain files).
2) Execute: Perform the actual task using ONLY the Context Pack files as context.

Rules:
- Always include mandatory context (AGENTS.md + this file) and mandatory instruction files.
- Prefer 2–5 domain instruction files per task.
- If ambiguous, ask up to 3 clarifying questions before executing.

## Decision Quickstart (Instruction Hierarchy)

Follow this order of operations on every task:

1) Read the user request and identify the target area
- `.github/**` (policies, prompts, instructions)
- Code workspace (C#, Rust, TS/JS, etc.)
- Build/CI/CD/infra (pipelines, Docker, Kubernetes)

2) Apply instructions in this precedence order
- User prompt (explicit constraints)
- `AGENTS.md` + this file
- Domain instruction files under `instructions/` (pick by language/folder)
- Any additional, file-scoped instructions (e.g., `instructions/copilot-instruction-creation.instructions.md` when editing `instructions/*`)

3) Resolve conflicts
- More specific scope wins (narrower `applyTo` beats broader)
- Prefer safer/minimal changes when ambiguous, and ask 1–3 clarifying questions if needed

# Workflow

## Master Orchestrator Lifecycle
- Treat `instructions/master-orchestrator.instructions.md` as the mandatory controller contract for change-bearing work.
- Default lifecycle:
  1. MASTER intake
  2. planning registration
  3. specialist identification
  4. execution
  5. testing
  6. code review
  7. closeout
  8. planning update
- Do not skip directly from request to implementation when files, runtime assets, or repository state are expected to change.

## How to use
- Start with AGENTS.md for solution-specific details (stack, folders, commands).
- Use this file for global rules, precedence, and always-applied policies.
- Use `instructions/repository-operating-model.instructions.md` for repo topology, build/test/run commands, style, release, and domain instruction mapping.
- Follow domain-specific files in instructions/*.md for technical details.

# Authoritative Sources Policy
- Use repository context first for project-specific behavior, architecture, scripts, templates, and conventions.
- For external platform, framework, SDK, API, CLI, or tool behavior, follow `instructions/authoritative-sources.instructions.md`.
- Use `.github/governance/authoritative-source-map.json` as the single source of truth for stack-specific official documentation domains.
- Do not duplicate official documentation domain lists across domain instruction files.

# Validation Checklist Policy
- Every non-trivial task must define a concrete validation checklist before or during implementation.
- The checklist must be scope-specific and cover only the relevant checks for the task (for example: build, tests, docs, security, migrations, runtime behavior, links, formatting).
- Final task reporting must include checklist status using `passed`, `pending`, or `blocked`.
- If a validation item cannot be executed, keep it in the checklist and state why it remained pending or blocked.

# Chat Session Naming and Runtime Paths
- Start each new Copilot or Codex chat by normalizing the session title to `<project-prefix> - <task summary>` as soon as the client or runtime allows it.
- The project prefix must come from the active workspace or repository name; do not omit it and do not duplicate it when the title is already prefixed.
- Prefer workspace-scoped Copilot sessions over empty-window sessions for project work so the title stays attached to the correct project scope.
- When scripting or documenting chat runtime storage, never hardcode personal absolute paths in tracked files. Use parameterized paths such as:
  - `"%USERPROFILE%\\.codex\\session_index.jsonl"`
  - `"%APPDATA%\\Code\\User\\workspaceStorage\\<workspace-id>\\chatSessions\\*.json"`
  - `"%APPDATA%\\Code\\User\\workspaceStorage\\<workspace-id>\\chatSessions\\*.jsonl"`
  - `"%APPDATA%\\Code\\User\\globalStorage\\emptyWindowChatSessions\\*.json"`
  - `"%APPDATA%\\Code\\User\\globalStorage\\emptyWindowChatSessions\\*.jsonl"`
- If Copilot session titles need bulk normalization, use `scripts/runtime/update-copilot-chat-titles.ps1` instead of editing unrelated session payload fields by hand.

# Mandatory Instructions

## Always Applied
- AGENTS.md (agents and context policy)
- instructions/master-orchestrator.instructions.md
- instructions/repository-operating-model.instructions.md
- instructions/subagent-planning-workflow.instructions.md
- instructions/authoritative-sources.instructions.md
- instructions/workflow-optimization.instructions.md
- instructions/powershell-execution.instructions.md
- instructions/feedback-changelog.instructions.md

## Only for .github Changes
- instructions/copilot-instruction-creation.instructions.md

# Repository and Domain Rules
- Repo topology, build/test/run commands, style, security/changelog process, and the full domain instruction map live in `instructions/repository-operating-model.instructions.md`.
- Change-bearing work must start with `instructions/master-orchestrator.instructions.md` before planning and implementation.
- Non-trivial tasks must also follow `instructions/subagent-planning-workflow.instructions.md` and the versioned planning workspace under `planning/`.
- Use domain instructions from that map according to the active route and file scope.

# Transparency

## Pragmatic use
- List applied instructions only when there are relevant actions (plans, command executions, patches/file changes).
- Use a short preamble to indicate key instructions before tool/command calls; omit in purely informational answers.
- For auditing, consolidate the full list of instructions in PR/commit body or CHANGELOG.md.
- When requested, include an Applied instructions section with the actually used set.
- After finishing a logically complete item, return a suggested commit message in English using semantic commit prefixes such as `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`, `perf:`, `build:`, or `ci:`.
- When the current state is stable and ready for persistence, explicitly tell the user that the work is ready to commit.
- For large tasks, surface stable intermediate commit checkpoints as soon as they are reached.

# Repository Style, Security, and Release
- Follow `instructions/repository-operating-model.instructions.md` for style, EOF policy, security handling, commit/PR expectations, and changelog rules.