# Scripts

> Automation scripts for bootstrap, deployment, docs validation, maintenance, and tests.

---

## Introduction

This folder centralizes operational scripts used by this repository. It includes bootstrap sync for shared `.github/.codex` assets and utility scripts for maintenance and tests.

---

## Features

- ✅ Bootstrap local runtime from versioned shared assets
- ✅ Utility scripts for deployment, docs, maintenance, and tests
- ✅ MCP apply mode integrated in bootstrap flow
- ✅ Policy-as-code validation for instruction/runtime governance
- ✅ Multi-agent contract and orchestration validation
- ✅ Multi-agent pipeline runner with guardrails, deterministic stage handoffs, and optional live `codex-exec` dispatch
- ✅ Task-loop implementation with per-task spec review and code-quality review
- ✅ Safe parallel batching for dependency-independent work items with write-set conflict blocking
- ✅ Super Agent worktree isolation helper and thin lifecycle entry commands
- ✅ Single picker-visible `super-agent` controller projected into `%USERPROFILE%\\.agents\\skills`
- ✅ Native Copilot skill projection from `.github/skills` into `%USERPROFILE%\\.copilot\\skills`
- ✅ Repository-owned Copilot agent profile under `.github/agents/super-agent.agent.md`
- ✅ Configurable VS Code startup-controller selector with repository default plus local and environment overrides
- ✅ Repository-owned PreToolUse EOF normalization for supported VS Code AI edit tools
- ✅ Repository-owned TDD and verification workflow contracts for execution stages
- ✅ Persisted orchestration run state for replay diagnostics and auditability
- ✅ Release governance checks (CODEOWNERS, changelog contracts, branch-protection baseline)
- ✅ Security baseline checks (sensitive file patterns and secret-like content scanning)
- ✅ Shared script checksum manifest governance for external workflow integrity
- ✅ Release provenance checks (validation coverage, evidence files, git traceability, optional audit proof)
- ✅ Validation profiles (`dev`, `release`, `enforced`) with warning-only execution policy
- ✅ Hash-chained validation ledger for immutable local audit trail
- ✅ Agent/skill permission matrix checks and supply-chain baseline checks
- ✅ Unified validation suite runner (`validate-all`) for local hooks and CI
- ✅ Healthcheck and self-heal flows with JSON reports and execution logs

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
- [API Reference](#api-reference)
- [Build and Tests](#build-and-tests)
- [Contributing](#contributing)
- [Dependencies](#dependencies)
- [References](#references)

---

## Installation

No package installation is required. Scripts run with PowerShell 7+.

---

## Quick Start

```powershell
# run the full recommended local onboarding flow
$RepoRoot = '<REPO_ROOT>'
pwsh -File (Join-Path $RepoRoot 'scripts/runtime/install.ps1') -CreateSettingsBackup -ApplyMcpConfig -BackupMcpConfig

# Sync shared assets
pwsh -File .\scripts\runtime\bootstrap.ps1

# Sync and apply MCP servers into ~/.codex/config.toml
pwsh -File .\scripts\runtime\bootstrap.ps1 -ApplyMcpConfig -BackupConfig

# Run full enterprise healthcheck
pwsh -File .\scripts\runtime\healthcheck.ps1 -StrictExtras

# Execute default multi-agent pipeline (writes artifacts to .temp/runs)
pwsh -File .\scripts\runtime\run-agent-pipeline.ps1 -RequestText "Validate enterprise multi-agent flow"

# Execute the same pipeline through the live Codex backend
pwsh -File .\scripts\runtime\run-agent-pipeline.ps1 -RequestText "Validate enterprise multi-agent flow" -ExecutionBackend codex-exec

# Approve sensitive agents explicitly for a live run
pwsh -File .\scripts\runtime\run-agent-pipeline.ps1 -RequestText "Validate enterprise multi-agent flow" -ExecutionBackend codex-exec -ApprovedAgentIds specialist,release-engineer -ApprovedBy "thiago.guislotti" -ApprovalJustification "Approved sensitive pipeline stages"

# Create an isolated worktree before risky or multi-slice execution
pwsh -File .\scripts\runtime\new-super-agent-worktree.ps1 -WorktreeName "feature-slice"

# Stop after brainstorm/spec
pwsh -File .\scripts\runtime\invoke-super-agent-brainstorm.ps1 -RequestText "Design the workstream"

# Stop after planning
pwsh -File .\scripts\runtime\invoke-super-agent-plan.ps1 -RequestText "Write the execution plan"

# Execute the full lifecycle
pwsh -File .\scripts\runtime\invoke-super-agent-execute.ps1 -RequestText "Implement the approved plan"

# Execute the full lifecycle and allow safe parallel worker fan-out
pwsh -File .\scripts\runtime\invoke-super-agent-parallel-dispatch.ps1 -RequestText "Implement independent work items"

# Execute the full lifecycle with explicit approval for sensitive stages
pwsh -File .\scripts\runtime\invoke-super-agent-execute.ps1 -RequestText "Implement the approved plan" -ApprovedAgentIds specialist,release-engineer -ApprovedBy "thiago.guislotti" -ApprovalJustification "Approved implementation and closeout"

# Enable local Git hooks (pre-commit + post-commit sync)
pwsh -File .\scripts\git-hooks\setup-git-hooks.ps1
```

The installer does not require the current shell to be in the repository root. If `pwsh -File` points to the versioned `install.ps1` path, the script resolves the repository root from its own location. Use `-RepoRoot` only when you need to override that auto-detection.

The VS Code hook bootstrap selects its startup controller from `.github/hooks/super-agent.selector.json`. Keep the repository default in version control and override locally only through `~/.github/hooks/super-agent.selector.local.json` or the environment variables `COPILOT_SUPER_AGENT_SKILL` and `COPILOT_SUPER_AGENT_NAME`.

---

## Usage Examples

### Example 1: Bootstrap Shared Runtime Assets

```powershell
pwsh -File .\scripts\runtime\bootstrap.ps1
```

### Example 2: Mirror Sync (Cleanup Included)

```powershell
pwsh -File .\scripts\runtime\bootstrap.ps1 -Mirror
```

### Example 3: Apply MCP Servers With Backup

```powershell
pwsh -File .\scripts\runtime\bootstrap.ps1 -ApplyMcpConfig -BackupConfig
```

---

## API Reference

### Structure

```text
scripts/
├── runtime/
│   ├── bootstrap.ps1
│   ├── doctor.ps1
│   ├── install.ps1
│   ├── apply-vscode-templates.ps1
│   ├── new-super-agent-worktree.ps1
│   ├── sync-vscode-global-snippets.ps1
│   ├── sync-workspace-settings.ps1
│   ├── invoke-super-agent-brainstorm.ps1
│   ├── invoke-super-agent-plan.ps1
│   ├── invoke-super-agent-execute.ps1
│   ├── invoke-super-agent-parallel-dispatch.ps1
│   ├── healthcheck.ps1
│   ├── self-heal.ps1
│   ├── run-agent-pipeline.ps1
│   └── clean-codex-runtime.ps1
├── orchestration/
│   ├── engine/
│   │   ├── invoke-codex-dispatch.ps1
│   │   └── invoke-task-worker.ps1
│   └── stages/
│       ├── intake-stage.ps1
│       ├── spec-stage.ps1
│       ├── plan-stage.ps1
│       ├── route-stage.ps1
│       ├── implement-stage.ps1
│       ├── validate-stage.ps1
│       ├── review-stage.ps1
│       └── closeout-stage.ps1
├── governance/
│   ├── set-branch-protection.ps1
│   └── update-shared-script-checksums-manifest.ps1
├── git-hooks/
│   └── setup-git-hooks.ps1
├── validation/
│   ├── validate-instructions.ps1
│   ├── validate-agent-orchestration.ps1
│   ├── validate-policy.ps1
│   ├── validate-release-governance.ps1
│   ├── validate-routing-coverage.ps1
│   ├── validate-agent-skill-alignment.ps1
│   └── export-audit-report.ps1
├── deploy/
├── doc/
├── maintenance/
├── security/
├── tests/
└── README.md
```

### Bootstrap Contract

`runtime/bootstrap.ps1` syncs:
- `.github/` -> `~/.github`
- `.codex/skills/` -> `~/.agents/skills` as the canonical picker-visible/runtime skill target
- `.github/skills/` -> `~/.copilot/skills` for native GitHub Copilot skill discovery
- stale repo-managed duplicates are removed from `~/.codex/skills` while unmanaged/system skill folders are preserved
- `.codex/mcp/` -> `~/.codex/shared-mcp`
- `.codex/scripts/` (root MCP tools) + `scripts/common/` + `scripts/security/` -> `~/.codex/shared-scripts`
- `.codex/orchestration/` -> `~/.codex/shared-orchestration`

MCP apply mode updates only `[mcp_servers.*]` sections in `~/.codex/config.toml`, preserving the rest.

Runtime-sensitive files such as `~/.codex/auth.json`, `~/.codex/sessions/`, and `~/.codex/log/` are not managed.

### Utility Scripts

| Script | Purpose | Quick example |
|--------|---------|---------------|
| `deploy/deploy-backend-to-vps.ps1` | Interactive Docker deployment pipeline for VPS hosts. | `& .\scripts\deploy\deploy-backend-to-vps.ps1 @params` |
| `doc/validate-xml-documentation.ps1` | Audits `<summary>` XML documentation across C# projects. | `pwsh -File scripts/doc/validate-xml-documentation.ps1 -ProjectPath src/Api` |
| `validation/validate-instructions.ps1` | Validates instruction assets (routing catalog paths, markdown links, and JSON files used by prompts/skills/snippets). | `pwsh -File scripts/validation/validate-instructions.ps1` |
| `validation/validate-agent-orchestration.ps1` | Validates multi-agent contracts (`.codex/orchestration/*`) against schemas and cross-file integrity rules. | `pwsh -File scripts/validation/validate-agent-orchestration.ps1` |
| `validation/validate-planning-structure.ps1` | Validates the versioned planning workspace under `planning/`, including required folders, README, and legacy `.temp/planning` drift detection. | `pwsh -File scripts/validation/validate-planning-structure.ps1 -RepoRoot .` |
| `validation/validate-policy.ps1` | Validates policy contracts declared in `.github/policies/*.json` (required files/directories/hooks). | `pwsh -File scripts/validation/validate-policy.ps1` |
| `validation/validate-release-governance.ps1` | Validates release-governance baseline (`CHANGELOG`, `CODEOWNERS`, branch-protection baseline, governance docs). | `pwsh -File scripts/validation/validate-release-governance.ps1` |
| `validation/validate-readme-standards.ps1` | Validates README structure/formatting using `.github/governance/readme-standards.baseline.json`. | `pwsh -File scripts/validation/validate-readme-standards.ps1` |
| `validation/validate-authoritative-source-policy.ps1` | Validates the centralized official-doc policy, the stack-to-domain map in `.github/governance/authoritative-source-map.json`, and required references from global instruction files and routing. | `pwsh -File scripts/validation/validate-authoritative-source-policy.ps1` |
| `validation/validate-instruction-architecture.ps1` | Validates ownership boundaries between the global core, repository operating model, domain instructions, prompts, templates, skills, and runtime/orchestration assets, including global-core size budgets and deterministic routing constraints. | `pwsh -File scripts/validation/validate-instruction-architecture.ps1` |
| `validation/validate-template-standards.ps1` | Validates shared templates against `.github/governance/template-standards.baseline.json`, including required/forbidden patterns and referenced script/doc paths. | `pwsh -File scripts/validation/validate-template-standards.ps1` |
| `validation/validate-workspace-efficiency.ps1` | Validates `.code-workspace` files against `.github/governance/workspace-efficiency.baseline.json` using the effective combination of global template plus local workspace overrides. Covers redundant settings, Git throttling, watcher/search inheritance, and multi-folder heuristics for Codex/Copilot usage. | `pwsh -File scripts/validation/validate-workspace-efficiency.ps1 -WorkspaceSearchRoot .\workspaces` |
| `validation/validate-compatibility-lifecycle-policy.ps1` | Validates `COMPATIBILITY.md` Support Lifecycle/EOL table semantics (reference date, ordering, EOL + 1 day, status). | `pwsh -File scripts/validation/validate-compatibility-lifecycle-policy.ps1` |
| `validation/validate-powershell-standards.ps1` | Validates `scripts/**/*.ps1` for script help coverage, per-parameter `.PARAMETER` entries, function description comments, approved verbs, and tracked line-ending normalization. | `pwsh -File scripts/validation/validate-powershell-standards.ps1` |
| `validation/validate-shell-hooks.ps1` | Validates `.githooks/*` shell syntax with `sh -n` and optional `shellcheck`. | `pwsh -File scripts/validation/validate-shell-hooks.ps1` |
| `validation/validate-agent-hooks.ps1` | Validates repository-owned VS Code hook JSON and required bootstrap scripts under `.github/hooks/`. | `pwsh -File scripts/validation/validate-agent-hooks.ps1 -WarningOnly:$false` |
| `validation/validate-runtime-script-tests.ps1` | Runs runtime test scripts under `scripts/tests/runtime` without external test frameworks and replays child test diagnostics only when verbose mode is enabled or a test fails. | `pwsh -File scripts/validation/validate-runtime-script-tests.ps1` |
| `validation/validate-dotnet-standards.ps1` | Validates .NET template standards under `.github/templates/*.cs`. | `pwsh -File scripts/validation/validate-dotnet-standards.ps1` |
| `validation/validate-architecture-boundaries.ps1` | Validates architecture boundaries from `.github/governance/architecture-boundaries.baseline.json`. | `pwsh -File scripts/validation/validate-architecture-boundaries.ps1` |
| `validation/validate-instruction-metadata.ps1` | Validates frontmatter metadata for `.github/instructions`, `.github/prompts`, and `.github/chatmodes`. | `pwsh -File scripts/validation/validate-instruction-metadata.ps1` |
| `validation/validate-routing-coverage.ps1` | Validates route coverage between `instruction-routing.catalog.yml` and golden fixtures. | `pwsh -File scripts/validation/validate-routing-coverage.ps1` |
| `validation/validate-agent-skill-alignment.ps1` | Validates consistency among agent manifest, skills, pipeline stages, and eval requiredAgents. | `pwsh -File scripts/validation/validate-agent-skill-alignment.ps1` |
| `validation/validate-agent-permissions.ps1` | Validates alignment between agent contracts and `.github/governance/agent-skill-permissions.matrix.json`. | `pwsh -File scripts/validation/validate-agent-permissions.ps1` |
| `validation/validate-security-baseline.ps1` | Validates `.github/governance/security-baseline.json` (required governance paths, forbidden sensitive files, and secret-like pattern scanning). Runs in warning-only mode by default. | `pwsh -File scripts/validation/validate-security-baseline.ps1` |
| `validation/validate-shared-script-checksums.ps1` | Validates `.github/governance/shared-script-checksums.manifest.json` against current files under configured script roots. | `pwsh -File scripts/validation/validate-shared-script-checksums.ps1` |
| `validation/validate-warning-baseline.ps1` | Validates PSScriptAnalyzer warning volume against `.github/governance/warning-baseline.json`. | `pwsh -File scripts/validation/validate-warning-baseline.ps1` |
| `validation/validate-supply-chain.ps1` | Validates dependency manifests against `.github/governance/supply-chain.baseline.json` and exports local SBOM artifact. | `pwsh -File scripts/validation/validate-supply-chain.ps1` |
| `validation/validate-release-provenance.ps1` | Validates release provenance baseline (`requiredValidationChecks`, `requiredEvidenceFiles`, changelog recency, git traceability, optional audit report). Runs in warning-only mode by default. | `pwsh -File scripts/validation/validate-release-provenance.ps1` |
| `validation/validate-audit-ledger.ps1` | Validates `.temp/audit/validation-ledger.jsonl` hash-chain integrity. | `pwsh -File scripts/validation/validate-audit-ledger.ps1` |
| `validation/validate-all.ps1` | Runs full profile-based suite, warning-only by default, appends hash-chained ledger evidence, and emits performance report at `.temp/audit/validate-all.latest.json`. | `pwsh -File scripts/validation/validate-all.ps1 -ValidationProfile dev` |
| `validation/export-audit-report.ps1` | Runs health baseline and exports consolidated JSON audit report with git metadata and policy inventory. | `pwsh -File scripts/validation/export-audit-report.ps1` |
| `validation/export-enterprise-trends.ps1` | Exports trends dashboard artifacts (`warnings`, `vulnerabilities`, and validation performance) from ledger + latest reports. | `pwsh -File scripts/validation/export-enterprise-trends.ps1` |
| `validation/test-routing-selection.ps1` | Runs deterministic golden tests for static routing behavior based on catalog + fixtures. | `pwsh -File scripts/validation/test-routing-selection.ps1` |
| `governance/set-branch-protection.ps1` | Validates or applies branch protection from `.github/governance/branch-protection.baseline.json` using GitHub CLI. | `pwsh -File scripts/governance/set-branch-protection.ps1 -Apply` |
| `governance/update-shared-script-checksums-manifest.ps1` | Regenerates `.github/governance/shared-script-checksums.manifest.json` with deterministic SHA256 entries for shared script roots. | `pwsh -File scripts/governance/update-shared-script-checksums-manifest.ps1` |
| `git-hooks/setup-git-hooks.ps1` | Configures local Git hooks path (`core.hooksPath=.githooks`) and enables `pre-commit` validation + `post-commit` sync. | `pwsh -File scripts/git-hooks/setup-git-hooks.ps1` |
| `common/common-bootstrap.ps1` | Shared helper-loader bootstrap that resolves and imports `console-style`, `repository-paths`, `runtime-paths`, and `validation-logging` from repository and mirrored runtime layouts. | `. ./scripts/common/common-bootstrap.ps1 -CallerScriptRoot $PSScriptRoot -Helpers @('console-style','repository-paths')` |
| `common/repository-paths.ps1` | Shared repository helper for repository/git/solution root discovery, repo-relative and full-path conversion, parent directory handling, verbose diagnostics, and structured execution logging reused across runtime, security, orchestration, and runtime test scripts. | `. ./scripts/common/repository-paths.ps1` |
| `common/validation-logging.ps1` | Shared validation log helper for warning/failure registration, verbose output, and compact validation summaries reused across the validation script family. | `. ./scripts/common/validation-logging.ps1` |
| `runtime/doctor.ps1` | Diagnoses drift between repository-managed runtime assets and local `~/.github`/`~/.codex` copies. | `pwsh -File scripts/runtime/doctor.ps1` |
| `runtime/install.ps1` | Runs the recommended onboarding flow by orchestrating bootstrap, optional MCP apply, VS Code sync, Git hook setup, and healthcheck. Supports preview mode and parameterized runtime paths. Can be invoked from any directory when `pwsh -File` points to the versioned script path. Warning/error lines now carry runtime issue IDs and the script always closes with a deduplicated issue summary plus severity counts. | `$RepoRoot = '<REPO_ROOT>'; pwsh -File (Join-Path $RepoRoot 'scripts/runtime/install.ps1') -CreateSettingsBackup -ApplyMcpConfig -BackupMcpConfig` |
| `runtime/apply-vscode-templates.ps1` | Applies `.vscode/*.tamplate.jsonc` into active `.vscode/settings.json` and `.vscode/mcp.json` files. | `pwsh -File scripts/runtime/apply-vscode-templates.ps1 -Force` |
| `runtime/sync-vscode-global-settings.ps1` | Renders `.vscode/settings.tamplate.jsonc` into the global VS Code user profile `settings.json`, replacing runtime placeholders such as `%USERPROFILE%` and optionally creating a backup first. | `pwsh -File scripts/runtime/sync-vscode-global-settings.ps1 -CreateBackup` |
| `runtime/sync-vscode-global-snippets.ps1` | Synchronizes versioned `.vscode/snippets/*.tamplate.code-snippets` files into the global VS Code user profile under `Code/User/snippets`, removing `.tamplate` from target names. | `pwsh -File scripts/runtime/sync-vscode-global-snippets.ps1` |
| `runtime/sync-workspace-settings.ps1` | Generates or refreshes `.code-workspace` files from `.vscode/base.code-workspace` plus the approved local override block derived from `.github/governance/workspace-efficiency.baseline.json`. Preserves folders, removes settings already covered by the global template, and merges workspace-specific extension recommendations with the shared base. | `pwsh -File scripts/runtime/sync-workspace-settings.ps1 -WorkspacePath .\workspaces\api.code-workspace -FolderPath src\Api` |
| `runtime/update-copilot-chat-titles.ps1` | Normalizes persisted GitHub Copilot chat titles to `<project-prefix> - <task summary>` using `%APPDATA%\\Code\\User\\workspaceStorage\\<workspace-id>\\chatSessions\\*.json*`, with optional backups before writing. | `pwsh -File scripts/runtime/update-copilot-chat-titles.ps1 -Apply -CreateBackup` |
| `runtime/healthcheck.ps1` | Runs `validate-all` (profile-aware) plus `runtime-doctor`, emits report/log, and defaults to warning-only mode. Runtime warning/error lines use issue IDs and the final output/report include a deduplicated issue summary with severity counts. | `pwsh -File scripts/runtime/healthcheck.ps1 -ValidationProfile release` |
| `runtime/self-heal.ps1` | Runs controlled repair flow (bootstrap + optional templates) and validates final state via healthcheck. Runtime warning/error lines use issue IDs and the final output/report include a deduplicated issue summary with severity counts. | `pwsh -File scripts/runtime/self-heal.ps1 -Mirror -StrictExtras` |
| `runtime/run-agent-pipeline.ps1` | Executes default multi-agent pipeline with blocked-command, allowed-path, budget, and approval guardrails; supports `script-only` and live `codex-exec` backends; writes `run-artifact.json`, `run-state.json`, `approval-record.json`, and stage artifacts under `.temp/runs/<traceId>/`. Sensitive stages require `-ApprovedStageIds` or `-ApprovedAgentIds` plus `-ApprovedBy` and `-ApprovalJustification`. | `pwsh -File scripts/runtime/run-agent-pipeline.ps1 -RequestText "Implement and validate change" -ExecutionBackend codex-exec -ApprovedAgentIds specialist,release-engineer -ApprovedBy "thiago.guislotti" -ApprovalJustification "Approved sensitive pipeline stages"` |
| `runtime/new-super-agent-worktree.ps1` | Creates deterministic git worktrees for isolated Super Agent work, with Windows-safe branch/worktree naming and preview support. | `pwsh -File scripts/runtime/new-super-agent-worktree.ps1 -WorktreeName "feature-slice"` |
| `runtime/invoke-super-agent-brainstorm.ps1` | Runs the Super Agent lifecycle through intake and spec only, producing normalized request and active spec artifacts. | `pwsh -File scripts/runtime/invoke-super-agent-brainstorm.ps1 -RequestText "Design the workstream"` |
| `runtime/invoke-super-agent-plan.ps1` | Runs the Super Agent lifecycle through planning and stops after a worker-ready plan is written. | `pwsh -File scripts/runtime/invoke-super-agent-plan.ps1 -RequestText "Write the execution plan"` |
| `runtime/invoke-super-agent-execute.ps1` | Runs the full Super Agent lifecycle end-to-end with the configured execution backend and forwards explicit approval parameters to sensitive stages. | `pwsh -File scripts/runtime/invoke-super-agent-execute.ps1 -RequestText "Implement the approved plan" -ApprovedAgentIds specialist,release-engineer -ApprovedBy "thiago.guislotti" -ApprovalJustification "Approved full lifecycle execution"` |
| `runtime/invoke-super-agent-parallel-dispatch.ps1` | Runs the full Super Agent lifecycle and is intended for safe parallel worker dispatch once dependency-independent batches are present; forwards explicit approval parameters when sensitive stages are allowed to run. | `pwsh -File scripts/runtime/invoke-super-agent-parallel-dispatch.ps1 -RequestText "Implement independent work items" -ApprovedAgentIds specialist,release-engineer -ApprovedBy "thiago.guislotti" -ApprovalJustification "Approved full lifecycle execution"` |
| `orchestration/engine/invoke-codex-dispatch.ps1` | Renders a stage prompt, invokes the local Codex CLI, captures JSON output, and persists dispatch logs/records for live planner, executor, and reviewer stages. | `pwsh -File scripts/orchestration/engine/invoke-codex-dispatch.ps1 -RepoRoot . -TraceId trace-001 -StageId plan -AgentId planner -PromptPath .temp\\planner.md -ResponseSchemaPath .github\\schemas\\agent.stage-plan-result.schema.json -ResultPath .temp\\planner-result.json -DispatchRecordPath .temp\\planner-dispatch.json` |
| `orchestration/engine/invoke-task-worker.ps1` | Executes a single worker-ready task through implementer -> task spec review -> task quality review, with retry and allowed-path enforcement. | `pwsh -File scripts/orchestration/engine/invoke-task-worker.ps1 -RepoRoot . -TraceId trace-001 -TaskRecordPath .temp\\runs\\trace-001\\artifacts\\task-001.json -RunDirectory .temp\\runs\\trace-001 -ExecutionBackend codex-exec` |
| `runtime/clean-codex-runtime.ps1` | Cleans local Codex runtime garbage (`tmp`, `vendor_imports`) and prunes `log`/`sessions` files older than retention using `LastWriteTime` (default 30 days). | `pwsh -File scripts/runtime/clean-codex-runtime.ps1 -IncludeSessions -SessionRetentionDays 30 -LogRetentionDays 30 -Apply` |
| `orchestration/stages/*.ps1` | Stage executors (`plan`, `implement`, `validate`, `review`) consumed by `run-agent-pipeline.ps1`. | `pwsh -File scripts/runtime/run-agent-pipeline.ps1 -RequestText "Smoke run"` |
| `maintenance/clean-build-artifacts.ps1` | Deletes `.build`, `.deployment`, `bin`, and `obj` directories. Supports dry-run and prompts for confirmation. | `pwsh -File scripts/maintenance/clean-build-artifacts.ps1 -DryRun` |

## VS Code Agent Hooks

- Repository-owned VS Code agent hooks live under `.github/hooks/`.
- Runtime sync mirrors them to `%USERPROFILE%\\.github\\hooks`.
- The global VS Code settings template loads hooks from both:
  - `.github/hooks`
  - `~/.github/hooks`
- Current bootstrap events:
  - `SessionStart`
  - `PreToolUse`
  - `SubagentStart`
- Startup controller selection:
  - versioned default: `.github/hooks/super-agent.selector.json`
  - optional local override: `~/.github/hooks/super-agent.selector.local.json`
  - optional environment override: `COPILOT_SUPER_AGENT_SKILL`, `COPILOT_SUPER_AGENT_NAME`
- EOF normalization:
  - `PreToolUse` normalizes supported edit/create tool payloads before disk writes
  - current supported tools: `createFile`, `insertEdit`, `replaceString`, `multiReplaceString`
  - `applyPatch` still relies on model compliance with the EOF policy because its patch grammar is not safely rewritten by the hook
| `maintenance/generate-http-from-openapi.ps1` | Generates a REST Client .http file from OpenAPI (default) or Swagger JSON. | `pwsh -File scripts/maintenance/generate-http-from-openapi.ps1 -Source http://localhost:5000` |
| `maintenance/fix-version-ranges.ps1` | Normalises PackageReference versions into `[current, limit)` ranges. | `pwsh -File scripts/maintenance/fix-version-ranges.ps1 -Verbose` |
| `maintenance/trim-trailing-blank-lines.ps1` | Removes trailing spaces and blank lines at EOF while respecting the repository EOF policy: files end without final newline unless a future explicit rule says otherwise. Supports `-GitChangedOnly` to limit trimming to files currently reported by `git status`. | `pwsh -File scripts/maintenance/trim-trailing-blank-lines.ps1 -GitChangedOnly` |
| `security/Invoke-VulnerabilityAudit.ps1` | Audits .NET backend package vulnerabilities via `dotnet list package --vulnerable --include-transitive`. | `pwsh -File scripts/security/Invoke-VulnerabilityAudit.ps1 -FailOnSeverities Critical,High` |
| `security/Invoke-FrontendPackageVulnerabilityAudit.ps1` | Audits frontend dependencies using npm/pnpm/yarn and applies severity quality gate. | `pwsh -File scripts/security/Invoke-FrontendPackageVulnerabilityAudit.ps1 -ProjectPath src/WebApp -FailOnSeverities Critical,High` |
| `security/Invoke-RustPackageVulnerabilityAudit.ps1` | Audits Rust dependencies via `cargo audit --json` with severity quality gate. | `pwsh -File scripts/security/Invoke-RustPackageVulnerabilityAudit.ps1 -ProjectPath . -FailOnSeverities Critical,High` |
| `security/Install-SecurityAuditPrerequisites.ps1` | Validates and auto-installs audit prerequisites (`dotnet`, `cargo`, `cargo-audit`, npm/pnpm/yarn) with optional system package manager support. | `pwsh -File scripts/security/Install-SecurityAuditPrerequisites.ps1 -FrontendPackageManager auto` |
| `security/Invoke-PreBuildSecurityGate.ps1` | Runs unified pre-build vulnerability gate across .NET, frontend, and Rust using stack scripts and consolidated summary artifacts; optionally runs prerequisite setup first. | `pwsh -File scripts/security/Invoke-PreBuildSecurityGate.ps1 -InstallMissingPrerequisites -FailOnSeverities Critical,High` |
| `security/Invoke-CiPreBuildSecuritySnapshot.ps1` | Runs the warning-only pre-build security gate with stack-aware skip detection for CI workflows, so the inline GitHub Actions logic stays minimal and deterministic. | `pwsh -File scripts/security/Invoke-CiPreBuildSecuritySnapshot.ps1 -RepoRoot . -WarningOnly:$true -AllowMissingCargoAudit` |
| `tests/apply-aaa-pattern.ps1` | Applies AAA comments to frontend TypeScript/Vue test files. | `pwsh -File scripts/tests/apply-aaa-pattern.ps1` |
| `tests/check-test-naming.ps1` | Validates required underscore segments in test names. | `pwsh -File scripts/tests/check-test-naming.ps1 Projects "OpenApi.Readers.UnitTests"` |
| `tests/refactor_tests_to_aaa.ps1` | Refactors Rust test files to follow AAA pattern with comments and formatting. | `pwsh -File scripts/tests/refactor_tests_to_aaa.ps1 -TestFile tests/unit/config_tests.rs` |
| `tests/run-coverage.ps1` | Runs tests with coverage and generates HTML/Cobertura reports. | `pwsh -File scripts/tests/run-coverage.ps1 -ProjectsDir tests` |

---

## Build and Tests

```powershell
# lint-like execution test for bootstrap (safe dry usage)
pwsh -File .\scripts\runtime\bootstrap.ps1 -TargetGithubPath .\.temp\github -TargetCodexPath .\.temp\codex

# preview the recommended onboarding flow without mutating runtime files
pwsh -File .\scripts\runtime\install.ps1 -PreviewOnly

# all runtime warning/error logs now include IDs such as WRN001 / ERR001
# and finish with a deduplicated issue summary plus severity counts
pwsh -File .\scripts\runtime\install.ps1 -CreateSettingsBackup -ApplyMcpConfig

# diagnose runtime drift
pwsh -File .\scripts\runtime\doctor.ps1

# apply VS Code active files from versioned templates
pwsh -File .\scripts\runtime\apply-vscode-templates.ps1 -Force

# render the versioned settings template into the global VS Code user profile
pwsh -File .\scripts\runtime\sync-vscode-global-settings.ps1 -CreateBackup

# synchronize canonical VS Code snippets into the global user profile
pwsh -File .\scripts\runtime\sync-vscode-global-snippets.ps1

# generate or refresh a workspace from the shared base and settings baseline
pwsh -File .\scripts\runtime\sync-workspace-settings.ps1 -WorkspacePath .\workspaces\api.code-workspace -FolderPath src\Api

# normalize persisted Copilot chat titles with the current project/workspace prefix
pwsh -File .\scripts\runtime\update-copilot-chat-titles.ps1 -Apply -CreateBackup

# run enterprise healthcheck with strict runtime guarantees
pwsh -File .\scripts\runtime\healthcheck.ps1 -StrictExtras

# auto-repair runtime and validate final state
pwsh -File .\scripts\runtime\self-heal.ps1 -Mirror -StrictExtras

# execute default multi-agent pipeline and produce run artifact
pwsh -File .\scripts\runtime\run-agent-pipeline.ps1 -RequestText "Run pipeline smoke test"

# execute default multi-agent pipeline with live planner/executor/reviewer dispatch
pwsh -File .\scripts\runtime\run-agent-pipeline.ps1 -RequestText "Run pipeline smoke test" -ExecutionBackend codex-exec

# create an isolated worktree for Super Agent execution
pwsh -File .\scripts\runtime\new-super-agent-worktree.ps1 -WorktreeName "feature-slice" -PreviewOnly

# run Super Agent through brainstorm/spec only
pwsh -File .\scripts\runtime\invoke-super-agent-brainstorm.ps1 -RequestText "Design the workstream" -PreviewOnly

# run Super Agent through planning only
pwsh -File .\scripts\runtime\invoke-super-agent-plan.ps1 -RequestText "Write the execution plan" -PreviewOnly

# run the full Super Agent lifecycle
pwsh -File .\scripts\runtime\invoke-super-agent-execute.ps1 -RequestText "Implement the approved plan" -PreviewOnly

# run the full lifecycle with the safe parallel-dispatch entrypoint
pwsh -File .\scripts\runtime\invoke-super-agent-parallel-dispatch.ps1 -RequestText "Implement independent work items" -PreviewOnly

# trim EOF only for files currently changed in git
pwsh -File .\scripts\maintenance\trim-trailing-blank-lines.ps1 -GitChangedOnly

# preview/runtime cleanup (no deletion)
pwsh -File .\scripts\runtime\clean-codex-runtime.ps1 -IncludeSessions -SessionRetentionDays 30 -LogRetentionDays 30

# apply runtime cleanup and session retention
pwsh -File .\scripts\runtime\clean-codex-runtime.ps1 -IncludeSessions -SessionRetentionDays 30 -LogRetentionDays 30 -Apply

# validate instruction assets and static routing references
pwsh -File .\scripts\validation\validate-instructions.ps1

# run full validation suite (hooks baseline)
pwsh -File .\scripts\validation\validate-all.ps1

# validate policy contracts
pwsh -File .\scripts\validation\validate-policy.ps1

# validate multi-agent contracts and pipeline integrity
pwsh -File .\scripts\validation\validate-agent-orchestration.ps1

# run the native Super Agent worktree tests
pwsh -File .\scripts\tests\runtime\super-agent-worktree.tests.ps1 -RepoRoot .

# run the native Super Agent entrypoint tests
pwsh -File .\scripts\tests\runtime\super-agent-entrypoints.tests.ps1 -RepoRoot .

# validate routing catalog coverage against golden fixtures
pwsh -File .\scripts\validation\validate-routing-coverage.ps1

# validate centralized official-doc source routing policy
pwsh -File .\scripts\validation\validate-authoritative-source-policy.ps1

# validate instruction ownership boundaries and global-core architecture
pwsh -File .\scripts\validation\validate-instruction-architecture.ps1

# validate shared templates against the template baseline
pwsh -File .\scripts\validation\validate-template-standards.ps1

# validate VS Code .code-workspace files for efficient Codex/Copilot usage
pwsh -File .\scripts\validation\validate-workspace-efficiency.ps1 -WorkspaceSearchRoot .\workspaces

# validate COMPATIBILITY lifecycle policy
pwsh -File .\scripts\validation\validate-compatibility-lifecycle-policy.ps1

# validate agent, skill, pipeline, and eval alignment
pwsh -File .\scripts\validation\validate-agent-skill-alignment.ps1

# validate release governance contracts
pwsh -File .\scripts\validation\validate-release-governance.ps1

# validate security baseline (paths + secret-like content patterns)
pwsh -File .\scripts\validation\validate-security-baseline.ps1

# validate shared script checksum manifest integrity
pwsh -File .\scripts\validation\validate-shared-script-checksums.ps1

# validate release provenance baseline and evidence traceability
pwsh -File .\scripts\validation\validate-release-provenance.ps1

# validate agent permission matrix
pwsh -File .\scripts\validation\validate-agent-permissions.ps1

# validate warning baseline (PSScriptAnalyzer volume)
pwsh -File .\scripts\validation\validate-warning-baseline.ps1

# validate supply-chain baseline and export SBOM
pwsh -File .\scripts\validation\validate-supply-chain.ps1

# validate immutable validation ledger chain
pwsh -File .\scripts\validation\validate-audit-ledger.ps1

# run suite with release profile
pwsh -File .\scripts\validation\validate-all.ps1 -ValidationProfile release

# validate branch protection drift from baseline
pwsh -File .\scripts\governance\set-branch-protection.ps1

# regenerate shared script checksum manifest
pwsh -File .\scripts\governance\update-shared-script-checksums-manifest.ps1

# export consolidated audit report (JSON + logs)
pwsh -File .\scripts\validation\export-audit-report.ps1 -ValidationProfile release -StrictExtras

# export enterprise trends dashboard artifacts
pwsh -File .\scripts\validation\export-enterprise-trends.ps1 -MaxEntries 30

# validate deterministic route selection
pwsh -File .\scripts\validation\test-routing-selection.ps1

# audit backend .NET dependencies before build/package
pwsh -File .\scripts\security\Invoke-VulnerabilityAudit.ps1 -FailOnSeverities Critical,High

# audit frontend dependencies (auto npm/pnpm/yarn)
pwsh -File .\scripts\security\Invoke-FrontendPackageVulnerabilityAudit.ps1 -ProjectPath src/WebApp -FailOnSeverities Critical,High

# audit Rust dependencies
pwsh -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -ProjectPath . -FailOnSeverities Critical,High

# validate/install security audit prerequisites
pwsh -File .\scripts\security\Install-SecurityAuditPrerequisites.ps1 -FrontendPackageManager auto

# run single pre-build security gate for all stacks
pwsh -File .\scripts\security\Invoke-PreBuildSecurityGate.ps1 -InstallMissingPrerequisites -FailOnSeverities Critical,High

# same gate with system-level installation attempts (winget/choco/brew/apt/etc)
pwsh -File .\scripts\security\Invoke-PreBuildSecurityGate.ps1 -InstallMissingPrerequisites -AllowSystemPrerequisiteInstall -FailOnSeverities Critical,High

# run same gate from shared runtime in any repository (Windows/Linux/macOS)
$HomePath = if (-not [string]::IsNullOrWhiteSpace($env:USERPROFILE)) { $env:USERPROFILE } else { $HOME }
$SecurityScriptsRoot = Join-Path $HomePath '.codex/shared-scripts/security'
pwsh -File (Join-Path $SecurityScriptsRoot 'Invoke-PreBuildSecurityGate.ps1') -RepoRoot $PWD -InstallMissingPrerequisites -FailOnSeverities Critical,High

# install local Git hooks (validation + sync)
pwsh -File .\scripts\git-hooks\setup-git-hooks.ps1

# inspect help for script contracts
Get-Help .\scripts\runtime\bootstrap.ps1 -Full
```

After setup, hooks behavior is:
- `pre-commit`: runs `validate-all -ValidationProfile dev -WarningOnly true` (best effort, warning-only)
- `post-commit`: runs `scripts/runtime/bootstrap.ps1 -Mirror` to sync and clean drift in `~/.github` and managed `~/.codex` folders (best effort)
- `post-commit`: runs `scripts/runtime/clean-codex-runtime.ps1 -LogRetentionDays 30 -IncludeSessions -SessionRetentionDays 30 -Apply` (best effort)
- `post-merge`: runs `validate-all -ValidationProfile release -WarningOnly true` (best effort, warning-only)
- `post-merge`: runs `scripts/runtime/clean-codex-runtime.ps1 -LogRetentionDays 30 -IncludeSessions -SessionRetentionDays 30 -Apply` (best effort)
- `post-checkout`: runs `validate-all -ValidationProfile dev -WarningOnly true` (best effort, warning-only)
- `post-commit` optional MCP apply on manifest changes: set `CODEX_APPLY_MCP_ON_POST_COMMIT=1`
- `post-commit` MCP backup control: `CODEX_BACKUP_MCP_CONFIG=1|0` (`1` default)
- `post-commit` mirror control: `CODEX_POST_COMMIT_MIRROR=0|1` (`1` default)
- `post-commit` and `post-merge` cleanup bypass: `CODEX_SKIP_RUNTIME_CLEANUP=1`
- `post-commit` and `post-merge` log retention: `CODEX_LOG_RETENTION_DAYS=<n>` (`30` default, by `LastWriteTime`)
- `post-commit` and `post-merge` session cleanup toggle: `CODEX_INCLUDE_SESSIONS_CLEANUP=0|1` (`1` default)
- `post-commit` and `post-merge` session retention: `CODEX_SESSION_RETENTION_DAYS=<n>` (`30` default, by `LastWriteTime`)

To skip sync for a single shell session:
```powershell
$env:CODEX_SKIP_POST_COMMIT_SYNC = "1"
```

Full hook contracts are documented in `README.md` (`Git Hooks` section).

Health and audit logs are written under `.temp/logs/` by healthcheck and audit scripts.
Validation ledger and SBOM artifacts are written under `.temp/audit/`.

---

## Contributing

- Keep scripts idempotent and safe by default.
- Document new script parameters in this README.
- Prefer explicit paths and robust error handling.

---

## Dependencies

- Runtime: PowerShell 7+, `robocopy` (Windows).
- Tooling-specific: may require `npx`, `dotnet`, or other CLIs depending on each script.

---

## References

- `.codex/scripts/README.md`
- `.github/AGENTS.md`
- `.github/copilot-instructions.md`
- `README.md`