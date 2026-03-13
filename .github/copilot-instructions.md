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

## How to use
- Start with AGENTS.md for solution-specific details (stack, folders, commands).
- Use this file for global rules and technology mappings.
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
- instructions/authoritative-sources.instructions.md
- instructions/workflow-optimization.instructions.md
- instructions/powershell-execution.instructions.md
- instructions/feedback-changelog.instructions.md

## Only for .github Changes
- instructions/copilot-instruction-creation.instructions.md

# Domain Instructions

## Development
- C#/.NET: instructions/dotnet-csharp.instructions.md (e.g., namespaces, sealed classes, XML docs).
- Architecture and backend: instructions/clean-architecture-code.instructions.md; instructions/backend.instructions.md (e.g., CQRS, mediator patterns).
- Frontend and UI/UX: instructions/frontend.instructions.md; instructions/vue-quasar.instructions.md; instructions/vue-quasar-architecture.instructions.md; instructions/ui-ux.instructions.md (e.g., i18n pt-BR, responsive design, feature-first Clean Architecture).

## Data and Infrastructure
- Data/ORM/Databases: instructions/orm.instructions.md; instructions/database.instructions.md; instructions/database-configuration-operations.instructions.md (e.g., EF Core, migrations, connection/pooling/failover operations).
- Privacy and data protection: instructions/data-privacy-compliance.instructions.md (e.g., PII handling, minimization, retention/deletion, compliance controls).
- Microservices and performance: instructions/microservices-performance.instructions.md; instructions/platform-reliability-resilience.instructions.md (e.g., async patterns, caching, resilience, chaos, DR readiness).
- Infrastructure and DevOps: instructions/docker.instructions.md; instructions/k8s.instructions.md; instructions/ci-cd-devops.instructions.md; instructions/workflow-generation.instructions.md; instructions/static-analysis-sonarqube.instructions.md (e.g., pipelines, security scans).
- Developer workspace and VS Code: instructions/vscode-workspace-efficiency.instructions.md (e.g., efficient `.code-workspace` design, Git/watcher throttling, shared AI context layout).
- Observability and SRE: instructions/observability-sre.instructions.md (e.g., SLI/SLO, telemetry quality, alerting, runbooks, incident readiness).
- Security and vulnerabilities: instructions/security-vulnerabilities.instructions.md; instructions/api-high-performance-security.instructions.md (e.g., OWASP/NIST-aligned controls and high-performance secure API patterns with rate limiting and abuse protection).
- Dependency vulnerability automation scripts (shared runtime): ~/.codex/shared-scripts/security/Invoke-PreBuildSecurityGate.ps1; ~/.codex/shared-scripts/security/Invoke-VulnerabilityAudit.ps1; ~/.codex/shared-scripts/security/Invoke-FrontendPackageVulnerabilityAudit.ps1; ~/.codex/shared-scripts/security/Invoke-RustPackageVulnerabilityAudit.ps1.
- For GitHub Actions in external repositories, consume shared scripts from pinned refs in `https://github.com/ThiagoGuislotti/copilot-instructions` instead of copying scripts into target repositories.
- Validate remote script integrity using `.github/governance/shared-script-checksums.manifest.json`.
- PowerShell script authoring: instructions/powershell-script-creation.instructions.md (e.g., script skeleton, root detection, mutation safety, exit codes).

## Testing and Documentation
- Rust organization and testing: instructions/rust-code-organization.instructions.md (e.g., mirror src/ structure, no inline tests, test_suite.rs entry point); instructions/rust-testing.instructions.md (e.g., error_tests.rs mandatory, coverage requirements, templates).
- E2E testing: instructions/e2e-testing.instructions.md (e.g., Playwright, test categories).
- Documentation and processes: instructions/readme.instructions.md; instructions/pr.instructions.md; instructions/prompt-templates.instructions.md; instructions/effort-estimation-ucp.instructions.md (e.g., README creation with template, PR guidelines, changelog versioning).

# Transparency

## Pragmatic use
- List applied instructions only when there are relevant actions (plans, command executions, patches/file changes).
- Use a short preamble to indicate key instructions before tool/command calls; omit in purely informational answers.
- For auditing, consolidate the full list of instructions in PR/commit body or CHANGELOG.md.
- When requested, include an Applied instructions section with the actually used set.
- After finishing a logically complete item, return a suggested commit message in English using semantic commit prefixes such as `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`, `perf:`, `build:`, or `ci:`.
- When the current state is stable and ready for persistence, explicitly tell the user that the work is ready to commit.
- For large tasks, surface stable intermediate commit checkpoints as soon as they are reached.

# Security
- No secrets in repo; use User Secrets/Azure Key Vault; typed options via IOptions.

# Changelog
- Single source: root CHANGELOG.md for .github and project changes
- Process: instructions/feedback-changelog.instructions.md
- Mandatory versioning: every CHANGELOG entry must include semantic version [X.Y.Z] and date YYYY-MM-DD; no [Unreleased] accumulation; immediate versioning on changes.
- Release tagging for rollback: after commit, create and push matching tag `copilot-vX.Y.Z` (for example: `git tag -a copilot-v1.1.3 -m "copilot instructions 1.1.3"` and `git push origin copilot-v1.1.3`).

# STYLE (EOF and whitespace)
- Do not leave a trailing blank line at the end of files.
- For files under `instructions/*.md` and Copilot/Codex instruction outputs: do NOT include a final newline.
- For the rest of the repository, follow `.editorconfig` exactly: current repository policy is `insert_final_newline = false`, including Rust/TOML/lock files unless a future file-specific rule explicitly opts in.
- Never add an extra empty line at EOF just because a file was edited; always avoid trailing whitespace.