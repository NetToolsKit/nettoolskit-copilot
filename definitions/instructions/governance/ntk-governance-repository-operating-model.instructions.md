---
applyTo: "**/*.{cs,csproj,sln,ps1,rs,toml,ts,tsx,js,jsx,vue,yml,yaml,json,jsonc,md,sql}"
priority: high
---

# Repository Operating Model

## Purpose
- Centralize repository-specific operational rules so `AGENTS.md` and `copilot-instructions.md` stay focused on global behavior, routing, and precedence.
- Keep repo topology, build/test/run commands, style, release process, and domain instruction references in one canonical location.

## Scope and References
- Repo-wide; subfolder `AGENTS.md` may specialize. Direct prompts override.
- Core global files remain:
  - `AGENTS.md`
  - `copilot-instructions.md`
- Cross-cutting policies remain centralized:
  - `instructions/governance/ntk-governance-authoritative-sources.instructions.md`
  - `instructions/governance/ntk-governance-artifact-layout.instructions.md`
  - `governance/authoritative-source-map.json`
- Planning lifecycle rules are centralized in `instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md` and `planning/README.md`.
- Brainstorm/spec rules are centralized in `instructions/governance/ntk-governance-brainstorm-spec-workflow.instructions.md` and `planning/specs/README.md`.
- Super Agent lifecycle rules are centralized in `agents/super-agent/ntk-agents-super-agent.instructions.md`.
- Worktree isolation rules are centralized in `instructions/governance/ntk-governance-worktree-isolation.instructions.md`.
- TDD and verification rules are centralized in `instructions/governance/ntk-governance-tdd-verification.instructions.md`.
- For GitHub Actions in external repositories, consume pinned shared scripts from `https://github.com/ThiagoGuislotti/copilot-instructions` instead of copying scripts into target repositories.
- Validate remote script integrity using `definitions/providers/github/governance/shared-script-checksums.manifest.json`.

## Instruction Source Of Truth And Projection
- `definitions/instructions/` is the canonical instruction content for repo-owned guidance.
- `.github/instructions/` is the projected runtime surface consumed by local agents, editors, and validation gates.
- `definitions/providers/github/root/`, `definitions/providers/vscode/workspace/`, `definitions/providers/codex/`, and `definitions/providers/claude/` are provider-specific consumers of the canonical taxonomy.
- Keep semantic folder names and stable `ntk-*` filenames aligned across canonical and projected surfaces.
- Do not use numeric directory prefixes to imply precedence or execution order; precedence is explicit in `AGENTS.md`, `copilot-instructions.md`, and the instruction ownership manifest.
- When drift is found:
  - review canonical shared content first
  - update projected `.github/instructions/` to match canonical intent
  - update provider surfaces only after canonical and projected paths are stable
  - treat `C:\Users\tguis\copilot-instructions` as a comparison baseline, never as a blind overwrite source for this repository

## Repository Topology
- Rust workspace that unifies the `ntk` CLI product, repository-managed AI runtime surfaces, validation gates, orchestration flows, and compatibility wrapper retirement program.
- Main layout:
  - `crates/` multi-crate Rust workspace
    - `cli/` — `ntk` binary entry point, top-level command routing, and CLI UX
    - `core/` — shared domain types, configuration, path resolution, local context, and common utilities
    - `ui/` — terminal UI primitives, color/unicode detection, and rendering helpers
    - `otel/` — local observability, metrics, timers, and telemetry helpers
    - `orchestrator/` — AI orchestration, ChatOps, service flows, repo workflow handling, and usage history
    - `commands/help/` — help discovery and manifest listing
    - `commands/manifest/` — manifest parsing, validation, and apply/render helpers
    - `commands/runtime/` — runtime bootstrap, drift diagnosis, self-heal, continuity, and maintenance surfaces
    - `commands/templating/` — Handlebars template rendering
    - `commands/validation/` — native validation orchestration and repository policy checks
    - `task-worker/` — background task execution worker for orchestrated flows
  - `scripts/` compatibility wrappers, retained operator surfaces, parity harnesses, and migration holdovers
  - `definitions/` authoritative shared and provider-specific assets projected into `.github/`, `.codex/`, `.claude/`, and `.vscode/`
  - `deployments/` Docker Compose profiles, service configuration, and deployment-oriented assets
  - `benchmarks/` Criterion benchmarks and performance fixtures
  - `docs/` architecture, operational, and UX guidance
  - `templates/` scaffolding templates and reference project surfaces
  - `.github/` projected and native GitHub surfaces: instructions, prompts, workflows, governance, hooks, and repository metadata
  - `.codex/` repo-local Codex runtime assets and skills
  - `.claude/` repo-local Claude runtime assets
  - `planning/` versioned planning workspace with `active/` and `completed/`
  - `planning/specs/` versioned design/spec workspace with `active/` and `completed/`

## Planning Workspace
- Use `agents/super-agent/ntk-agents-super-agent.instructions.md` for the mandatory intake-to-closeout lifecycle on change-bearing work.
- Use `instructions/governance/ntk-governance-brainstorm-spec-workflow.instructions.md` when non-trivial work needs design direction locked before execution planning.
- Use `instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md` for the planning and sub-agent workflow on non-trivial work.
- Use `instructions/governance/ntk-governance-worktree-isolation.instructions.md` when the workstream should move into an isolated git worktree.
- Use `instructions/governance/ntk-governance-tdd-verification.instructions.md` for code-bearing work that needs explicit verification checkpoints.
- Active plans live in `planning/active/`.
- Completed plans move to `planning/completed/` only after implementation, validation, review, and release closeout are materially complete.
- Active specs live in `planning/specs/active/`.
- Completed specs move to `planning/specs/completed/` with the related workstream when applicable.

## Build, Test, and Run
- Build workspace:
  - `cargo build --workspace`
- Run tests:
  - `cargo test --workspace`
  - targeted crate tests: `cargo test -p <crate-name>`
- Lint and format:
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo fmt --all -- --check`
- Validation:
  - `ntk validation all --repo-root . --warning-only false`
  - targeted: `ntk validation instructions --repo-root . --warning-only false`
  - targeted: `ntk validation planning-structure --repo-root . --warning-only false`
- Runtime diagnostics and continuity:
  - `ntk runtime healthcheck --repo-root .`
  - `ntk runtime update-local-context-index --repo-root .`
  - `ntk runtime query-local-context-index --repo-root . --query-text "super agent clarification" --json-output`
  - `ntk runtime update-local-memory --repo-root .`
- Runtime sync and compatibility wrappers:
  - `pwsh -NoProfile -File scripts/runtime/bootstrap.ps1`
- Vulnerability checks:
  - `pwsh -NoProfile -File scripts/security/Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- Runtime sync:
  - mirrors `.github` and `scripts` into `~/.github`
  - syncs `.codex/skills` into `~/.agents/skills` as the single visible Codex/VS Code starter/controller surface
  - removes legacy starter/controller skill duplicates from both `~/.github/skills` and `~/.copilot/skills` so the shared `super-agent` starter stays canonical
  - syncs remaining `.codex` runtime assets into `~/.codex`
- Local RAG/CAG continuity:
  - `ntk runtime update-local-context-index --repo-root .`
  - `ntk runtime query-local-context-index --repo-root . --query-text "super agent clarification" --json-output`
  - `ntk runtime query-local-context-index --repo-root . --query-text "compatibility fallback" --use-json-index --json-output`
  - prefer the SQLite-backed default recall path; use `--use-json-index` only for explicit compatibility/debug fallback
- Non-versioned artifact layout:
  - `.build/` for transient build and generated outputs
  - `.deployment/` for publish, package, release, and deployment-ready outputs
  - `.build/cargo-target/` for Cargo target output
  - `.build/coverage/` for coverage and transient verification artifacts
  - `.deployment/local/` for local deployment-ready or packaged outputs
  - do not invent new top-level artifact folders when these two cover the need

## Style and Artifact Hygiene
- Crate, module, and folder boundaries should remain aligned; do not create catch-all files that bypass the workspace organization model.
- Rust naming:
  - `UpperCamelCase` for structs, enums, traits, and type aliases
  - `snake_case` for modules, files, functions, locals, and parameters
  - `SCREAMING_SNAKE_CASE` for constants and environment-variable identifiers
- Prefer small focused modules over very large mixed-responsibility files.
- Keep imports deterministic and minimal.
- Use UTF-8 without BOM unless a file-specific format requires otherwise.
- Public Rust APIs should include concise rustdoc comments when the contract is not obvious from the type or function name.
- Avoid inline comments unless the user explicitly asks for them or the logic is genuinely non-obvious.
- EOF and whitespace:
  - never leave trailing blank lines at EOF
  - follow `.editorconfig`
  - repository policy currently uses `insert_final_newline = false`
  - preserve the exact terminal EOF state during edits; when a file currently has no terminal newline, keep it that way after AI-generated changes
  - do not append a terminal newline unless a narrower file-specific rule explicitly requires it
  - do not add trailing whitespace
- Generated output hygiene:
  - keep non-versioned build or generated output in `.build/`
  - keep publish, release, and deployable output in `.deployment/`
  - do not scatter generated artifacts under source folders when the artifacts are not source of truth

## UI, API, and Database Conventions
- Chat remains pt-BR, but technical assets stay in English unless the repository explicitly requires localized user-facing text.
- CLI messages, docs, JSON payloads, and repository-authored instruction assets remain in English unless a surface is explicitly user-localized.
- HTTP/service surfaces should use stable nouns, standard status codes, and `application/problem+json`-style error payloads when exposed externally.
- SQLite schemas, table names, column names, and persisted JSON keys remain in English.

## Testing Expectations
- Unit tests use `#[test]`; async tests use `#[tokio::test]` when Tokio is already part of the crate boundary.
- Integration tests live under `crates/*/tests/`.
- Test files use `<module>_tests.rs`.
- Assert behavior across CLI routing, runtime flows, validation contracts, orchestrator boundaries, and compatibility-cutover paths.
- Keep `scripts/tests/runtime/*.ps1` as the PowerShell parity harness while those wrappers remain intentionally retained.
- Ensure relevant tests pass locally whenever practical.

## Commits, PRs, and Transparency
- Commits must be in English, imperative, and no longer than 72 characters.
- Use semantic prefixes such as:
  - `feat:`
  - `fix:`
  - `docs:`
  - `refactor:`
  - `test:`
  - `chore:`
  - `perf:`
  - `build:`
  - `ci:`
- When a logically complete item is finished, always return a suggested commit message to the user.
- When the current changes are stable and ready to persist, explicitly state that the work is ready to commit.
- For large tasks, surface stable intermediate commit checkpoints.
- PR structure:
  - Context
  - Changes
  - Rationale
  - Risks
  - Testing
  - Docs
  - Breaking Changes
  - Migration
- List applied instruction paths and deviations when relevant.
- Require green validation and no secrets.

## Security and Changelog
- No secrets in repo; use environment variables, local config overrides, or platform secret stores outside the repository.
- Root `CHANGELOG.md` is the single source of truth for `.github` and project changes.
- Every changelog entry must include semantic version `[X.Y.Z]` and `YYYY-MM-DD`.

## Agent Workflow and Patterns
- Copilot: small edits, refactors, tests, docs.
- Codex: deterministic generation, infra/pipeline YAML, runtime/governance automation, and multi-step repository tasks.
- For non-trivial tasks:
  - create a short plan
  - use a short preamble before tool calls
  - validate crate boundaries, command routing, docs, tests, and EOF policy when relevant
  - follow super-agent -> brainstorm-spec -> planner -> specialist -> tester -> reviewer -> release-closeout
  - use `context-token-optimizer` only when the task is multi-domain or the context pack has obvious redundancy; do not trim required working context purely for token savings
  - prefer isolated worktrees for risky or long-running workstreams
  - treat verification evidence as mandatory before completion claims
  - keep plan artifacts in `planning/active/` and spec artifacts in `planning/specs/active/` until the work is genuinely complete
- Patterns:
  - multi-crate Rust workspace with explicit ownership by crate and module
  - retain PowerShell wrappers only when compatibility or operator entrypoint requirements still justify them
  - keep repo-local memory and user-local usage history as separate persistence scopes
  - use stack-specific audit scripts under `scripts/security/` before build/package

## Domain Instruction Map
- Development:
  - `instructions/development/ntk-development-backend-architecture-core.instructions.md`
  - `instructions/development/ntk-development-backend-rust-code-organization.instructions.md`
  - `instructions/development/ntk-development-backend-rust-testing.instructions.md`
  - `instructions/development/ntk-development-persistence-orm.instructions.md`
  - `instructions/security/ntk-security-api-high-performance.instructions.md`
- Infrastructure:
  - `instructions/operations/ntk-operations-docker.instructions.md`
  - `instructions/operations/ntk-operations-ci-cd-devops.instructions.md`
  - `instructions/operations/ntk-operations-workflow-generation.instructions.md`
  - `instructions/operations/ntk-operations-observability-sre.instructions.md`
  - `instructions/operations/ntk-operations-platform-reliability-resilience.instructions.md`
  - `instructions/operations/ntk-operations-powershell-script-creation.instructions.md`
  - `instructions/operations/ntk-operations-powershell-execution.instructions.md`
- Security:
  - `instructions/security/ntk-security-cicd-supply-chain-hardening.instructions.md`
  - `instructions/security/ntk-security-vulnerabilities.instructions.md`
  - `instructions/security/ntk-security-api-high-performance.instructions.md`
  - `instructions/data/ntk-data-privacy-compliance.instructions.md`
- Testing:
  - `instructions/development/ntk-development-backend-integration-testing.instructions.md`
  - `instructions/development/ntk-development-backend-rust-code-organization.instructions.md`
  - `instructions/development/ntk-development-backend-rust-testing.instructions.md`
  - `instructions/development/ntk-development-frontend-e2e-testing.instructions.md`
  - `instructions/operations/ntk-operations-static-analysis-sonarqube.instructions.md`
  - `instructions/governance/ntk-governance-tdd-verification.instructions.md`
- Documentation and process:
  - `instructions/governance/ntk-governance-readme.instructions.md`
  - `instructions/governance/ntk-governance-prompt-templates.instructions.md`
  - `instructions/governance/ntk-governance-effort-estimation-ucp.instructions.md`
  - `instructions/governance/ntk-governance-brainstorm-spec-workflow.instructions.md`
  - `instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md`
  - `agents/super-agent/ntk-agents-super-agent.instructions.md`
  - `instructions/governance/ntk-governance-worktree-isolation.instructions.md`
  - `instructions/governance/ntk-governance-tdd-verification.instructions.md`
  - `instructions/governance/ntk-governance-pr.instructions.md`