# Copilot Instructions

Structured AI agent guidelines for software development projects. Focuses on repeatable engineering workflows (planning, implementation, testing, docs, and reviews) using hierarchical instruction files, domain-specific conventions, and reusable prompt templates. Includes examples for .NET, Rust, frontend stacks, and DevOps, but the core goal is consistent, high-quality software delivery across technologies.

## Features

- ✅ **Hierarchical Instruction Structure:** Solution-level → Global → Domain-specific guidelines
- ✅ **Multi-Stack Coverage:** .NET/C#, Rust, Vue.js/Quasar, Docker, Kubernetes, databases
- ✅ **Architecture Patterns:** Clean Architecture, CQRS, DDD, microservices
- ✅ **Convention Standardization:** Code style, test patterns, commits, file organization
- ✅ **Authoritative Source Policy:** Repository context first, then official docs by stack
- ✅ **VS Code Session Bootstrap Hooks:** repository-owned `SessionStart`, `PreToolUse`, and `SubagentStart` hooks for Copilot and Codex sessions inside VS Code
- ✅ **Configurable Startup Controller Selector:** repository-owned hook selector with repo default plus local and environment overrides for the startup controller injected by VS Code hooks
- ✅ **Native Copilot Super Agent Surface:** repository-owned `.github/skills/super-agent` and `.github/agents/super-agent.agent.md` for GitHub Copilot-native discovery
- ✅ **Origin-Level EOF Guardrail:** repository-owned `PreToolUse` hook strips terminal newlines from supported AI edit payloads before VS Code writes tracked files
- ✅ **Tool Integration:** Git, CLI tools, CI/CD pipelines, static analysis
- ✅ **Custom Chat Modes:** Architecture review, instruction generation
- ✅ **Prompt Templates:** POML-based templates with CoT, SoT, ToT patterns
- ✅ **Multi-Agent Contracts:** Versioned orchestration manifests, schemas, and runtime artifacts
- ✅ **Versioned Planning Workspace:** Active/completed plans under `planning/` plus active/completed specs under `planning/specs/`
- ✅ **Mandatory Non-Trivial Flow:** super-agent -> brainstorm-spec -> planner -> context-token-optimizer -> specialist -> tester -> reviewer -> release-closeout
- ✅ **Approval Gate For Sensitive Execution:** sensitive implementation and closeout agents require explicit approval metadata before the runner dispatches file-mutating or release-mutating work
- ✅ **Worker-Ready Planning:** planner work items now carry target paths, explicit commands, expected checkpoints, and commit checkpoint suggestions
- ✅ **Task-Level Review Loop:** each implementation slice can pass through task spec review and task quality review before completion
- ✅ **Safe Parallel Dispatch:** dependency-aware batching blocks overlapping write-sets before parallel worker fan-out
- ✅ **Worktree Isolation Helpers:** repository-owned worktree creation flow for risky or long-running workstreams
- ✅ **Workflow Entry Commands:** thin PowerShell entrypoints for brainstorm, plan, execute, and parallel dispatch flows
- ✅ **Canonical Artifact Layout:** non-versioned generated outputs standardized under `.build/` and `.deployment/`
- ✅ **TDD and Verification Contracts:** repository-owned workflow rules for test-first implementation and verification-before-completion
- ✅ **Closeout Documentation Automation:** release closeout can rewrite repository README files and prepend CHANGELOG entries when the workstream is ready for commit
- ✅ **Guardrailed Multi-Agent Runner:** Deterministic pipeline execution with handoffs, budgets, allowed-path enforcement, and optional live `codex-exec` dispatch
- ✅ **Run-State Diagnostics:** Persisted `.temp/runs/<traceId>/run-state.json` snapshots for orchestration auditing and recovery analysis
- ✅ **Unified Validation Suite:** Single `validate-all` command for hooks/CI governance checks
- ✅ **Security Baseline Validation:** Local enforcement for sensitive files and secret-like content patterns
- ✅ **Release Provenance Validation:** Local traceability checks for release evidence and validation coverage
- ✅ **Validation Profiles:** `dev`, `release`, and `enforced` profiles with warning-only policy
- ✅ **Immutable Audit Trail:** Hash-chained validation ledger under `.temp/audit/`
- ✅ **Agent Permission Matrix + Supply Chain:** Governance checks for agent permissions and dependency risk baseline
- ✅ **Enterprise Dev Container:** Standardized toolchain for .NET, Rust, Node, PowerShell, and GitHub CLI
- ✅ **Dependency Automation:** Dependabot updates and PR dependency severity policy in warning-only observability mode
- ✅ **Security Observability Pipelines:** SBOM, provenance attestation, CodeQL, and Scorecard workflows without blocking merges
- ✅ **Trends Dashboard Export:** Consolidated metrics for validation warnings, vulnerability posture, and execution performance

---

## Table of Contents

- [Installation](#installation)
- [Contribution Workflow](#contribution-workflow)
- [Integration Matrix](#integration-matrix)
- [Architecture Model](#architecture-model)
- [Dev Container](#dev-container)
- [Parameterization & Privacy](#parameterization--privacy)
- [Git Hooks](#git-hooks)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
- [Chat Modes](#chat-modes)
- [Prompt Templates](#prompt-templates)
- [API Reference](#api-reference)
- [Dependencies](#dependencies)
- [References](#references)

---

## Installation

### Using in Existing Projects

Copy relevant files to your project (`.github/` for instructions and repo root for routing assets):

```bash
# Copy core instruction files
cp .github/AGENTS.md /path/to/your/project/.github/
cp .github/copilot-instructions.md /path/to/your/project/.github/

# Copy domain-specific instructions as needed
cp -r .github/instructions/ /path/to/your/project/.github/

# Optional: Copy chat modes, prompts, and routing schema
cp -r .github/chatmodes/ /path/to/your/project/.github/
cp -r .github/prompts/ /path/to/your/project/.github/
cp -r .github/schemas/ /path/to/your/project/.github/
```

### Repository Setup

```bash
git clone https://github.com/ThiagoGuislotti/copilot-instructions.git
cd copilot-instructions
```

### One-Step Local Onboarding

```powershell
$RepoRoot = '<REPO_ROOT>'
pwsh -File (Join-Path $RepoRoot 'scripts/runtime/install.ps1') -RuntimeProfile all -CreateSettingsBackup -ApplyMcpConfig -BackupMcpConfig
```

You do not need to be inside the repository directory when running the installer. Point `pwsh -File` at the versioned script path and it will detect the repository root from the script location. Use `-RepoRoot` only when you need to override that default.

The installer is intentionally non-intrusive by default:

- `install.ps1` now defaults to runtime profile `none`
- `none` plans/applies nothing until you opt in with `-RuntimeProfile`
- the supported profiles are defined in `.github/governance/runtime-install-profiles.json`

Use one of these explicit profiles:

| Profile | What it enables |
| --- | --- |
| `none` | Default. No runtime projection, no VS Code global changes, no Git integration changes. |
| `github` | Only the GitHub/Copilot runtime surface: `%USERPROFILE%\\.github`, `%USERPROFILE%\\.github\\scripts`, and `%USERPROFILE%\\.copilot\\skills`. |
| `codex` | Only the Codex runtime surface: `%USERPROFILE%\\.agents\\skills` plus `%USERPROFILE%\\.codex\\shared-*`. |
| `all` | Everything above plus global VS Code settings/snippets, local Git hooks, global Git aliases, and installer healthcheck. |

Examples:

```powershell
# explicit no-op preview (default behavior)
pwsh -File (Join-Path $RepoRoot 'scripts/runtime/install.ps1') -PreviewOnly

# enable only the GitHub/Copilot runtime surface
pwsh -File (Join-Path $RepoRoot 'scripts/runtime/install.ps1') -RuntimeProfile github

# enable only the Codex runtime surface
pwsh -File (Join-Path $RepoRoot 'scripts/runtime/install.ps1') -RuntimeProfile codex -ApplyMcpConfig -BackupMcpConfig

# enable everything
pwsh -File (Join-Path $RepoRoot 'scripts/runtime/install.ps1') -RuntimeProfile all -CreateSettingsBackup -ApplyMcpConfig -BackupMcpConfig
```

### Cross-Platform Prerequisites

- PowerShell 7+ (`pwsh`) installed on Windows, Linux, or macOS.
- Git installed and available in `PATH`.
- Optional on Linux/macOS: ensure `chmod` is available (used by hook setup).

## Contribution Workflow

Use the repository-managed community flow instead of ad-hoc issue and PR descriptions.

- Read [CONTRIBUTING.md](./CONTRIBUTING.md) before changing versioned runtime assets.
- Use `.github/PULL_REQUEST_TEMPLATE.md` for pull requests.
- Use `.github/ISSUE_TEMPLATE/*` for bugs, new skill requests, runtime sync problems, and validation gaps.
- Keep repository changes in `<REPO_ROOT>`; use sync scripts to propagate runtime state afterward.

## Integration Matrix

| Integration target | Versioned source of truth | Runtime target |
| --- | --- | --- |
| Copilot instructions, prompts, and VS Code agent hooks | `.github/` | `%USERPROFILE%\\.github` |
| Codex runtime skills, MCP, orchestration, shared scripts | `.codex/` + `scripts/common` + `scripts/security` + `scripts/maintenance` | `%USERPROFILE%\\.codex` |
| Picker-visible local skills for VS Code/Codex | `.codex/skills/` | `%USERPROFILE%\\.agents\\skills` |
| VS Code global settings | `.vscode/settings.tamplate.jsonc` | `%APPDATA%\\Code\\User\\settings.json` |
| VS Code global snippets | `.vscode/snippets/*.tamplate.code-snippets` | `%APPDATA%\\Code\\User\\snippets\\*.code-snippets` |
| VS Code workspaces | `.vscode/base.code-workspace` + `.github/governance/workspace-efficiency.baseline.json` | `.code-workspace` files refreshed by script |

## Architecture Model

The repository uses an explicit layered instruction architecture so context stays predictable and no single file silently becomes the owner of unrelated policy.

### Layers

- `Global core`: `.github/AGENTS.md` and `.github/copilot-instructions.md` stay short and define universal behavior only.
- `Repository operating model`: `.github/instructions/repository-operating-model.instructions.md` owns repository topology, build/test/run, style, release, and domain map details.
- `Planning workspace`: `.github/instructions/subagent-planning-workflow.instructions.md`, `.github/instructions/brainstorm-spec-workflow.instructions.md`, `planning/README.md`, and `planning/specs/README.md` define active/completed plan and spec handling for non-trivial work.
- `Cross-cutting policy`: `.github/instructions/authoritative-sources.instructions.md`, `.github/governance/*`, and `.github/policies/*` own rules that apply across domains.
- `Cross-cutting policy`: `.github/instructions/artifact-layout.instructions.md` owns the canonical non-versioned build and deployment artifact layout.
- `Domain instructions`: `.github/instructions/*.instructions.md` own stack-specific technical behavior.
- `Prompts`: `.github/prompts/*` are execution helpers and must not become normative policy owners.
- `Templates`: `.github/templates/*`, `.vscode/*.tamplate.jsonc`, `.vscode/snippets/*.tamplate.code-snippets`, and `.codex/mcp/*.template.*` define concrete artifact shapes only.
- `Codex skills`: `.codex/skills/*/SKILL.md` specialize execution and must reference canonical repo instructions instead of duplicating policy.
- `Runtime projection`: `scripts/runtime/*` renders the versioned source of truth into `%USERPROFILE%\\.github`, `%USERPROFILE%\\.codex`, and the VS Code global profile.

### Architecture Contracts

- Keep the route-first model deterministic: `instruction-routing.catalog.yml` is the only routing source of truth.
- Keep global context stable: avoid regrowing `AGENTS.md` and `copilot-instructions.md` with domain detail that belongs elsewhere.
- Keep policy centralized: if a rule can be defined once in governance or a shared instruction, do not duplicate it in domain files, prompts, templates, or skills.
- Keep runtime non-authoritative: local runtime folders are projections of the repository, never the source of truth.
- Keep non-trivial work on the mandatory planning chain and keep active plans in `planning/active/` plus active specs in `planning/specs/active/` until the work is materially complete.

### Planning Workspace

The repository uses versioned planning artifacts to keep non-trivial work auditable without polluting stable docs.

- `planning/README.md` explains the planning contract.
- `planning/active/` stores the current active plan files.
- `planning/specs/README.md` explains the brainstorming/spec contract.
- `planning/specs/active/` stores the current active spec files when design direction must be versioned before planning.
- `planning/completed/` stores closed plans after implementation, validation, review, and closeout.
- `instructions/super-agent.instructions.md` defines the mandatory intake-to-closeout lifecycle for change-bearing work.
- `instructions/brainstorm-spec-workflow.instructions.md` defines when a separate spec is required before planning.
- `instructions/subagent-planning-workflow.instructions.md` defines the mandatory super-agent -> brainstorm-spec -> planner -> context-token-optimizer -> specialist -> tester -> reviewer -> release-closeout flow.
- `instructions/worktree-isolation.instructions.md` defines when isolated worktrees should be created for risky or multi-slice execution.
- `instructions/tdd-verification.instructions.md` defines the default test-first and verification-before-completion workflow contract.

## Dev Container

Use `.devcontainer/devcontainer.json` to run this repository with a standardized enterprise toolchain.

### Includes

- .NET SDK (`8.0`)
- Rust toolchain
- Node.js LTS
- PowerShell (`pwsh`)
- GitHub CLI

### Usage

1. Open the repository in VS Code.
2. Run `Dev Containers: Reopen in Container`.
3. Wait for `postCreateCommand` to execute runtime bootstrap + instruction validation.

### Repository Layout

```text
copilot-instructions/
├─ .github/   # shared Copilot + Codex instructions, prompts, chatmodes, schemas
├─ .codex/    # shared Codex assets (skills/mcp/scripts/orchestration)
├─ scripts/   # bootstrap + automation scripts
├─ README.md
└─ .gitignore
```

### Bootstrap Local Folders

```powershell
pwsh -File ./scripts/runtime/bootstrap.ps1

# one-step local onboarding wrapper
$RepoRoot = '<REPO_ROOT>'
pwsh -File (Join-Path $RepoRoot 'scripts/runtime/install.ps1') -RuntimeProfile all -CreateSettingsBackup -ApplyMcpConfig -BackupMcpConfig

# optional: apply shared MCP servers separately when you do not want the full install wrapper
pwsh -File ./scripts/runtime/bootstrap.ps1 -ApplyMcpConfig -BackupConfig

# enterprise healthcheck (instructions + policy + agent orchestration + release governance + runtime doctor)
pwsh -File ./scripts/runtime/healthcheck.ps1 -StrictExtras
```

`bootstrap.ps1` remains the direct runtime sync entrypoint and defaults to runtime profile `all` when called on its own. Use `-RuntimeProfile github` or `-RuntimeProfile codex` when you want the projection to stay scoped to one runtime surface.

With profile `all`, bootstrap syncs versioned `.github/` and `.codex/` assets into your local runtime paths (`~/.github` and `~/.codex`), projects the single visible repository-owned starter/controller into `~/.agents/skills`, projects native Copilot skills into `~/.copilot/skills`, mirrors shared helper scripts from `scripts/common`, `scripts/security`, and `scripts/maintenance` into `~/.codex/shared-scripts`, removes stale duplicate repo-managed skill folders from `~/.codex/skills`, and applies MCP servers into `~/.codex/config.toml` when `-ApplyMcpConfig` is included.

The synced `.github/` runtime also carries VS Code hook configuration under `~/.github/hooks`, and the global settings template loads hooks from that path so Copilot and Codex sessions in VS Code receive the repository-owned bootstrap automatically.

The startup controller injected by those hooks is selected from `.github/hooks/super-agent.selector.json`. The repository default remains `Super Agent`, and you can override it without changing tracked files by either:

- creating `~/.github/hooks/super-agent.selector.local.json`
- setting `COPILOT_SUPER_AGENT_SKILL` and optionally `COPILOT_SUPER_AGENT_NAME`

The repository-owned `PreToolUse` hook also normalizes supported AI edit payloads (`createFile`, `insertEdit`, `replaceString`, and `multiReplaceString`) so files created or edited through VS Code agents preserve the repository EOF policy instead of gaining a terminal newline.

To apply active VS Code workspace files from templates:

```powershell
pwsh -File ./scripts/runtime/apply-vscode-templates.ps1 -Force

# render the versioned global VS Code settings template into Code/User/settings.json
pwsh -File ./scripts/runtime/sync-vscode-global-settings.ps1 -CreateBackup

# synchronize versioned snippets into Code/User/snippets
pwsh -File ./scripts/runtime/sync-vscode-global-snippets.ps1
```

---

## Parameterization & Privacy

To avoid exposing machine-specific information (for example `<HOME_PATH>/...`) in docs, logs, screenshots, or commits, prefer parameterized paths and environment variables.

### Rules

- Use relative repo paths in documentation and examples (`./scripts/...`) whenever possible.
- Use environment variables for runtime locations: `$env:USERPROFILE`, `$HOME`, `$env:REPO_ROOT`.
- Use placeholders in shared docs: `<REPO_ROOT>`, `<GITHUB_RUNTIME_PATH>`, `<CODEX_RUNTIME_PATH>`.
- Do not hardcode personal absolute paths in tracked files, prompts, or snippets.
- For chat runtime storage examples on Windows, keep paths parameterized:
  - `"%USERPROFILE%\\.codex\\session_index.jsonl"`
  - `"%APPDATA%\\Code\\User\\workspaceStorage\\<workspace-id>\\chatSessions\\*.json"`
  - `"%APPDATA%\\Code\\User\\workspaceStorage\\<workspace-id>\\chatSessions\\*.jsonl"`
  - `"%APPDATA%\\Code\\User\\globalStorage\\emptyWindowChatSessions\\*.json"`
  - `"%APPDATA%\\Code\\User\\globalStorage\\emptyWindowChatSessions\\*.jsonl"`

### PowerShell Example (Safe Defaults)

```powershell
$RepoRoot = $env:REPO_ROOT
if ([string]::IsNullOrWhiteSpace($RepoRoot)) {
    $RepoRoot = (Get-Location).Path
}

function Resolve-UserHome {
    if (-not [string]::IsNullOrWhiteSpace($env:USERPROFILE)) {
        return $env:USERPROFILE
    }

    if (-not [string]::IsNullOrWhiteSpace($HOME)) {
        return $HOME
    }

    $folder = [Environment]::GetFolderPath([Environment+SpecialFolder]::UserProfile)
    if (-not [string]::IsNullOrWhiteSpace($folder)) {
        return $folder
    }

    throw 'Could not resolve user home path. Set USERPROFILE or HOME.'
}

$UserHome = Resolve-UserHome
$GithubRuntimePath = Join-Path $UserHome '.github'
$CodexRuntimePath = Join-Path $UserHome '.codex'

pwsh -File (Join-Path $RepoRoot 'scripts/runtime/bootstrap.ps1') `
    -RepoRoot $RepoRoot `
    -TargetGithubPath $GithubRuntimePath `
    -TargetCodexPath $CodexRuntimePath `
    -Mirror
```

### Bash Example (Linux/macOS)

```bash
export REPO_ROOT="${REPO_ROOT:-$(pwd)}"
export HOME_DIR="${HOME:-$USERPROFILE}"

pwsh -File "$REPO_ROOT/scripts/runtime/bootstrap.ps1" \
  -RepoRoot "$REPO_ROOT" \
  -TargetGithubPath "$HOME_DIR/.github" \
  -TargetCodexPath "$HOME_DIR/.codex" \
  -Mirror
```

### Optional Local Override (Not Committed)

Set local environment variables in your shell profile and keep them out of versioned files:

```powershell
$env:REPO_ROOT = (Get-Location).Path
```

---

## Git Hooks

Local hooks are managed by `core.hooksPath=.githooks`.

### Setup

```powershell
pwsh -File ./scripts/git-hooks/setup-git-hooks.ps1
```

### Global Manual Alias

```powershell
pwsh -File ./scripts/git-hooks/setup-global-git-aliases.ps1
```

This installs a manual global alias:

```powershell
git trim-eof
```

Use it in any Git repository before `git add` when you want to trim only the files currently reported by `git status`.

### pre-commit

- Runs `scripts/validation/validate-all.ps1`
- Uses `-ValidationProfile dev -WarningOnly true` (best effort)
- Never blocks commit; failures are reported as warnings

### post-commit

- Runs `scripts/runtime/bootstrap.ps1 -Mirror` (best effort) to sync and clean drift in `~/.github` and managed `~/.codex` folders
- If `.codex/mcp/servers.manifest.json` changed in `HEAD`, can optionally apply MCP config
- Runs `scripts/runtime/validate-vscode-global-alignment.ps1` (best effort) to verify repository `.vscode` templates/snippets are contained in global VS Code User files
- Runs `scripts/runtime/clean-codex-runtime.ps1 -LogRetentionDays 30 -IncludeSessions -SessionRetentionDays 30 -Apply` (best effort) to clean runtime garbage and stale session/history files by **LastWriteTime** (last update)

### post-merge

- Runs `scripts/validation/validate-all.ps1 -ValidationProfile release -WarningOnly true` (best effort)
- Runs `scripts/runtime/clean-codex-runtime.ps1 -LogRetentionDays 30 -IncludeSessions -SessionRetentionDays 30 -Apply` (best effort)
- Does not run runtime sync

### post-checkout

- Runs `scripts/validation/validate-all.ps1 -ValidationProfile dev -WarningOnly true` (best effort)
- Does not run runtime sync

Environment variables:
- `CODEX_SKIP_POST_COMMIT_SYNC=1`: skip runtime sync
- `CODEX_APPLY_MCP_ON_POST_COMMIT=1`: enable MCP apply when manifest changed
- `CODEX_BACKUP_MCP_CONFIG=1|0`: backup control before MCP apply (`1` default)
- `CODEX_POST_COMMIT_MIRROR=0|1`: controls mirror cleanup on post-commit sync (`1` default)
- `CODEX_SKIP_VSCODE_GLOBAL_CHECK=1`: skips `.vscode` containment check against global VS Code User files in `post-commit`
- `CODEX_VSCODE_GLOBAL_USER_PATH=<path>`: overrides global VS Code User folder used by `post-commit` check
- `CODEX_SKIP_VSCODE_SNIPPET_CHECK=1`: skips snippet containment checks in `post-commit`
- `CODEX_SKIP_RUNTIME_CLEANUP=1`: skip automatic runtime cleanup in `post-commit` and `post-merge`
- `CODEX_LOG_RETENTION_DAYS=<n>`: log retention window in days for automatic runtime cleanup (`30` default, by `LastWriteTime`)
- `CODEX_INCLUDE_SESSIONS_CLEANUP=0|1`: enable cleanup for old session/history files (`1` default)
- `CODEX_SESSION_RETENTION_DAYS=<n>`: retention window for session/history cleanup (`30` default, by `LastWriteTime`)

### Enterprise Ops

```powershell
# run end-to-end checks and generate .temp/healthcheck-report.json + logs
pwsh -File ./scripts/runtime/healthcheck.ps1 -StrictExtras

# run full validation suite (instructions, policies, orchestration, docs standards)
pwsh -File ./scripts/validation/validate-all.ps1

# run full suite with release profile
pwsh -File ./scripts/validation/validate-all.ps1 -ValidationProfile release

# validate routing catalog coverage against golden fixtures
pwsh -File ./scripts/validation/validate-routing-coverage.ps1

# validate alignment between agents, skills, pipeline and evals
pwsh -File ./scripts/validation/validate-agent-skill-alignment.ps1

# validate agent permission matrix
pwsh -File ./scripts/validation/validate-agent-permissions.ps1

# validate release governance contracts (CHANGELOG + CODEOWNERS + baseline)
pwsh -File ./scripts/validation/validate-release-governance.ps1

# validate security baseline (sensitive paths + secret-like patterns)
pwsh -File ./scripts/validation/validate-security-baseline.ps1

# validate supply-chain baseline and generate SBOM artifact
pwsh -File ./scripts/validation/validate-supply-chain.ps1

# validate analyzer warning baseline
pwsh -File ./scripts/validation/validate-warning-baseline.ps1

# validate release provenance baseline (checks + evidence + git traceability)
pwsh -File ./scripts/validation/validate-release-provenance.ps1

# validate audit ledger hash chain
pwsh -File ./scripts/validation/validate-audit-ledger.ps1

# validate multi-agent contracts and orchestration integrity
pwsh -File ./scripts/validation/validate-agent-orchestration.ps1

# execute default multi-agent pipeline and generate run artifacts
pwsh -File ./scripts/runtime/run-agent-pipeline.ps1 -RequestText "Implement and validate request"

# execute the same pipeline with live sequential planner/executor/reviewer dispatch
pwsh -File ./scripts/runtime/run-agent-pipeline.ps1 -RequestText "Implement and validate request" -ExecutionBackend codex-exec

# execute the same pipeline with explicit approval for sensitive agents
pwsh -File ./scripts/runtime/run-agent-pipeline.ps1 -RequestText "Implement and validate request" -ExecutionBackend codex-exec -ApprovedAgentIds specialist,release-engineer -ApprovedBy "thiago.guislotti" -ApprovalJustification "Approved implementation and closeout for this run"

# create an isolated Super Agent worktree before risky or parallel work
pwsh -File ./scripts/runtime/new-super-agent-worktree.ps1 -WorktreeName "feature-slice"

# stop after the brainstorm/spec stage
pwsh -File ./scripts/runtime/invoke-super-agent-brainstorm.ps1 -RequestText "Design the workstream"

# stop after planning
pwsh -File ./scripts/runtime/invoke-super-agent-plan.ps1 -RequestText "Write the execution plan"

# run the full lifecycle
pwsh -File ./scripts/runtime/invoke-super-agent-execute.ps1 -RequestText "Implement the approved plan"

# run the full lifecycle with safe parallel batching
pwsh -File ./scripts/runtime/invoke-super-agent-parallel-dispatch.ps1 -RequestText "Execute independent work items"

# run the full lifecycle with explicit approval for sensitive stages
pwsh -File ./scripts/runtime/invoke-super-agent-execute.ps1 -RequestText "Implement the approved plan" -ApprovedAgentIds specialist,release-engineer -ApprovedBy "thiago.guislotti" -ApprovalJustification "Approved full lifecycle execution"

# validate branch protection drift against baseline (no mutation)
pwsh -File ./scripts/governance/set-branch-protection.ps1

# apply branch protection baseline (opt-in remote mutation)
pwsh -File ./scripts/governance/set-branch-protection.ps1 -Apply

# repair runtime and validate final state
pwsh -File ./scripts/runtime/self-heal.ps1 -Mirror -StrictExtras

# clean local Codex runtime garbage and prune sessions by retention window
pwsh -File ./scripts/runtime/clean-codex-runtime.ps1 -IncludeSessions -SessionRetentionDays 30 -LogRetentionDays 30 -Apply

# export consolidated audit report with git metadata and policy inventory
pwsh -File ./scripts/validation/export-audit-report.ps1 -ValidationProfile release -StrictExtras

# export enterprise trends dashboard artifacts
pwsh -File ./scripts/validation/export-enterprise-trends.ps1 -MaxEntries 30

# validate/install audit prerequisites and run security gate
pwsh -File ./scripts/security/Install-SecurityAuditPrerequisites.ps1 -FrontendPackageManager auto
pwsh -File ./scripts/security/Invoke-PreBuildSecurityGate.ps1 -InstallMissingPrerequisites -FailOnSeverities Critical,High
```

Audit logs are generated under `.temp/logs/`.
Validation ledger and SBOM artifacts are generated under `.temp/audit/`.

Security and governance observability workflows:
- `.github/workflows/dependency-risk-observability.yml`
- `.github/workflows/enterprise-trends-dashboard.yml`
- `.github/workflows/sbom-attestation-observability.yml`
- `.github/workflows/security-static-observability.yml`

`validate-agent-system.yml` owns push-time audit/healthcheck coverage; `enterprise-trends-dashboard.yml` is reserved for scheduled or manually-triggered trend exports to avoid duplicate `validate-all` runs on every push.

---

## Quick Start

### Recommended (Most Important): Static RAGs Routing

Use a routing step to select a minimal “context pack” before doing any work.

1. **Copy the core files (required):**
   ```bash
   cp .github/AGENTS.md .github/copilot-instructions.md /your/project/.github/
   ```

2. **Copy the routing assets (recommended):**
   ```bash
   cp .github/instruction-routing.catalog.yml /your/project/.github/
   cp .github/prompts/route-instructions.prompt.md /your/project/.github/prompts/
   cp -r .github/schemas/ /your/project/.github/
   ```

3. **Route first, then execute:**
   - Run the route-only prompt `.github/prompts/route-instructions.prompt.md`.
   - Load ONLY the files from the returned Context Pack (mandatory + selected).
   - Execute the task using that minimal context.

### Basic Setup (3 Steps)

1. **Copy core files:**
   ```bash
   cp .github/AGENTS.md .github/copilot-instructions.md /your/project/.github/
   ```

2. **Adapt `.github/AGENTS.md`** for your project structure

3. **Select relevant instructions:**
   ```bash
   # .NET project
   cp .github/instructions/{dotnet-csharp,clean-architecture-code,backend}.instructions.md /your/project/.github/instructions/

   # Rust project
   cp .github/instructions/rust-testing.instructions.md /your/project/.github/instructions/

   # Frontend project
   cp .github/instructions/{frontend,vue-quasar,ui-ux}.instructions.md /your/project/.github/instructions/

   # DevOps
   cp .github/instructions/{docker,k8s,ci-cd-devops}.instructions.md /your/project/.github/instructions/
   ```

### First AI Interaction

```text
# In GitHub Copilot Chat (reference loaded instruction files)
"Refactor following dotnet-csharp.instructions.md conventions"
"Generate Rust tests using rust-testing.instructions.md patterns"
"Review architecture compliance per clean-architecture-code.instructions.md"
```

---

## Usage Examples

### Code Refactoring (.NET)

```text
"Refactor to C# 12 with sealed classes and file-scoped namespaces per dotnet-csharp.instructions.md"
```

### Test Generation (Rust)

```text
"Generate async tests for this module following rust-testing.instructions.md patterns"
```

### Architecture Review

```text
@clean-architecture-review "Analyze this service for SOLID violations"
```

### Component Generation (Vue)

```text
"Create Quasar component with Composition API per vue-quasar.instructions.md"
```

### Using Prompt Templates

```text
# Reference .github/prompts/generate-unit-tests.prompt.md
"Generate xUnit tests for OrderService with AAA pattern and mocking"
```

---

## Chat Modes

### clean-architecture-review.chatmode.md

Specialized mode for reviewing code against Clean Architecture principles.

**Capabilities:**
- SOLID principles validation
- Dependency rule enforcement
- Layer boundary verification
- Code smell detection

**Usage:**
```text
@clean-architecture-review "Analyze this repository structure and identify violations"
```

### instruction-writer.chatmode.md

Specialized mode for creating new instruction files following meta-conventions.

**Capabilities:**
- Instruction file scaffolding
- Consistency with existing instructions
- Best practice enforcement
- Automatic formatting

**Usage:**
```text
@instruction-writer "Create instruction file for gRPC service development"
```

---

## Prompt Templates

### Standard Templates (Markdown-based)

Located in `.github/prompts/`:
- **create-dotnet-class.prompt.md** - Generate Clean Architecture compliant classes
- **create-powershell-script.prompt.md** - Generate `scripts/*` PowerShell automation with safe defaults
- **generate-changelog.prompt.md** - Create semantic versioning CHANGELOG entries
- **generate-unit-tests.prompt.md** - Generate comprehensive xUnit/NUnit tests

### POML Templates (XML-based)

Located in `.github/prompts/poml/templates/`:
- **changelog-entry.poml** - Structured CHANGELOG generator with versioning
- **unit-test-generator.poml** - AAA pattern test generator with mocking

**Learn more:** [POML Guide](./.github/prompts/poml/prompt-engineering-poml.md)

---

## API Reference

### Core Files

| File | Purpose | When to Use |
|------|---------|-------------|
| **.github/AGENTS.md** | Agent policies, workflow patterns, context selection rules | Always load FIRST in Copilot sessions |
| **.github/copilot-instructions.md** | Global rules, domain mapping, repository structure | Always load SECOND in Copilot sessions |

### Instruction Files

| Domain | File | Description |
|--------|------|-------------|
| **.NET/C#** | `dotnet-csharp.instructions.md` | .NET 8+, naming, conventions |
| **Rust** | `rust-testing.instructions.md` | Test patterns, async, error handling |
| **Architecture** | `clean-architecture-code.instructions.md` | Clean Architecture, CQRS, DDD |
| **Backend** | `backend.instructions.md` | REST APIs, validation, error handling |
| **Frontend** | `frontend.instructions.md`, `vue-quasar.instructions.md` | SPA, Vue 3, Quasar, state management |
| **Data** | `orm.instructions.md`, `database.instructions.md` | EF Core, SQL, schema design |
| **DevOps** | `docker.instructions.md`, `k8s.instructions.md`, `ci-cd-devops.instructions.md` | Containers, orchestration, pipelines |
| **Security** | `security-vulnerabilities.instructions.md` | OWASP/NIST-aligned controls for API, frontend, backend, and database |
| **Testing** | `e2e-testing.instructions.md` | E2E strategies, test frameworks |
| **Quality** | `static-analysis-sonarqube.instructions.md` | Code quality, static analysis |
| **Documentation** | `readme.instructions.md`, `pr.instructions.md` | READMEs, PR guidelines |
| **Workflow** | `workflow-optimization.instructions.md` | Development efficiency |
| **Super Agent Workflow** | `super-agent.instructions.md`, `brainstorm-spec-workflow.instructions.md`, `subagent-planning-workflow.instructions.md`, `worktree-isolation.instructions.md`, `tdd-verification.instructions.md` | Intake, specs, planning, worktree isolation, TDD, verification, and closeout |

### Context Selection Rule (Hard Requirement)

**Always load FIRST in any Copilot Chat session:**
1. `.github/AGENTS.md`
2. `.github/copilot-instructions.md`

This ensures consistent agent behavior and proper context hierarchy.

### Static RAGs Routing

If you want a RAGs-style routing step (selecting a minimal “context pack” before execution), use:
- `.github/instruction-routing.catalog.yml` (single source of truth for routes)
- `.github/prompts/route-instructions.prompt.md` (route-only prompt that outputs a JSON context pack)

---

## Dependencies

### Runtime Dependencies
None. This is a documentation and policy repository.

### Development Dependencies
- **GitHub Copilot** (or compatible AI coding assistant)
- **VS Code** with Copilot Chat extension
- **Git** for version control

### Optional Dependencies
- **POML CLI** (`npm install -g @microsoft/poml-cli`) for POML template rendering
- **Language SDKs** (.NET SDK, Rust toolchain, Node.js) depending on your project stack

---

## References

### Official Documentation

- [GitHub Copilot Documentation](https://docs.github.com/en/copilot) - Complete Copilot reference
- [VS Code Copilot Tips](https://code.visualstudio.com/docs/copilot/copilot-tips-and-tricks) - Best practices and shortcuts
- [VS Code Prompt Crafting](https://code.visualstudio.com/docs/copilot/chat/prompt-crafting) - Effective prompt engineering
- [Custom Instructions Guide](https://docs.github.com/en/copilot/how-tos/configure-custom-instructions/add-repository-instructions) - Repository-level instructions

### Best Practices & Articles

- [Microsoft DevBlogs: 5 Copilot Chat Prompts .NET Devs Should Steal](https://devblogs.microsoft.com/dotnet/5-copilot-chat-prompts-dotnet-devs-should-steal-today/) - Practical .NET prompts
- [Dev.to: Supercharge VSCode Copilot](https://dev.to/pwd9000/supercharge-vscode-github-copilot-using-instructions-and-prompt-files-2p5e) - Advanced instruction techniques
- [GitHub Copilot Troubleshooting](https://docs.github.com/copilot/troubleshooting-github-copilot/troubleshooting-common-issues-with-github-copilot) - Common issues and solutions

### Standards & Specifications

- [Microsoft POML](https://github.com/microsoft/poml) - Prompt Orchestration Markup Language
- [Keep a Changelog](https://keepachangelog.com/) - Changelog format standard
- [Semantic Versioning](https://semver.org/) - Version numbering convention
- [Conventional Commits](https://www.conventionalcommits.org/) - Commit message convention

### Internal Documentation

- [CHANGELOG](./CHANGELOG.md) - Version history
- [COMPATIBILITY](./COMPATIBILITY.md) - Support lifecycle and EOL policy
- [CONTRIBUTING](./CONTRIBUTING.md) - Contribution workflow and validation guidance

---

## License

This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.

---