# Repository Consolidation and Continuity Plan

Generated: 2026-03-29

## Status

- LastUpdated: 2026-04-05 17:35
- Objective: preserve the final continuity and handoff record for repository consolidation after the local planning sequence, audit-only script-retirement sweeps, and external Rust-directive handoff all completed.
- Normalized Request: create a detailed and complete plan for all gaps and pending workstreams identified in the repository consolidation analysis conducted on 2026-03-29, then split the remaining open work into smaller category-specific planning tracks.
- Active Branch: `docs/planning-gap-workstreams` (planning closeout)
- Spec Path: `planning/specs/completed/spec-repository-consolidation-continuity.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Dependency: `planning/completed/plan-script-retirement-phase-18.md`, `planning/completed/plan-script-retirement-phase-19.md`, `planning/completed/plan-script-retirement-phase-20c-self-heal.md`, `planning/completed/plan-script-retirement-phase-20d-provider-surface-dispatcher.md`, `planning/completed/plan-script-retirement-phase-20e-catalog-native-renderer-dispatch.md`, and `planning/completed/plan-script-retirement-phase-20f-codex-orchestration-renderer.md` are now complete; Workstream W5 now continues from the closed 96-script baseline.
- Inputs:
  - `planning/completed/plan-script-retirement-phase-17.md`
  - `planning/completed/plan-script-retirement-phase-18.md`
  - `planning/completed/plan-script-retirement-phase-19.md`
  - `planning/completed/plan-script-retirement-phase-20c-self-heal.md`
  - `planning/specs/completed/spec-repository-consolidation-continuity.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`
  - `definitions/providers/github/root/instruction-routing.catalog.yml`
  - `.github/workflows/ci.yml`
  - `crates/cli/src/main.rs`
  - `crates/cli/src/runtime_commands.rs`
  - `crates/cli/src/validation_commands.rs`
  - `crates/cli/README.md`
  - `README.md`
  - `scripts/tests/runtime/*.ps1` (23 files)
  - `C:\Users\tguis\copilot-instructions\planning\active\plan-rust-runtime-engine-foundation-phase-8.md`

---

## Closeout Summary

- W1-W5 are complete in `nettoolskit-copilot`, including the audit-only Phase 19-22 script-retirement sweeps and the post-Phase-22 retained-estate proof.
- W6.1 is complete in `copilot-instructions`, and the remaining W6.2-W6.5 work is explicitly implementation work in that external repository rather than additional planning discovery here.
- The local continuity umbrella is therefore archived in this repository: future changes belong either to concrete implementation workstreams under the remaining active plans or to the external `copilot-instructions` repo.

## Scope Summary

This plan coordinates six workstreams:

| ID | Workstream | Repo | Priority | Dependency |
|---|---|---|---|---|
| W1 | Phase 17 completion reference | `nettoolskit-copilot` | 🔴 Immediate | — |
| W2 | Repository operating model alignment | `nettoolskit-copilot` | 🔴 Immediate | — |
| W3 | CI PowerShell parity test coverage | `nettoolskit-copilot` | 🟡 High | — |
| W4 | CLI surface documentation | `nettoolskit-copilot` | 🟡 High | — |
| W5 | Post-Phase-18 domain consumer migration (Phases 19–22) | `nettoolskit-copilot` | 🟠 Planned | W1 complete |
| W6 | `copilot-instructions` Phase 8 Rust directives | `copilot-instructions` | 🟡 High | — |

This plan does not replace the tactical phase plans; Phases 17 and 18 are now archived and this plan inherits their closed baseline for W5.

Operational hardening checkpoint:
- The managed pre-commit EOF hygiene path now needs bounded batching when forwarding large staged-file sets to `ntk runtime trim-trailing-blank-lines`, because Windows process invocation fails once hundreds of `--literal-path` arguments exceed command-line limits during commit-time hook execution.
- The provider-root global-core layer (`definitions/providers/github/root/AGENTS.md` and `definitions/providers/github/root/copilot-instructions.md`) now fits the architecture budget again by keeping only bootstrap, precedence, and lifecycle summaries in the root files and pushing detailed policy to canonical `definitions/instructions/*` guidance.

---

## Ordered Tasks

---

### Workstream W1 — Phase 17/18 Reference (archived sibling plans)

Status: `[x]` Completed (Phases 17 and 18 archived and consumer-sweep baseline unlocked)

This workstream is tracked in `planning/completed/plan-script-retirement-phase-17.md`
and `planning/completed/plan-script-retirement-phase-18.md`.
Together they targeted the runtime diagnostics and MCP wrapper slices and reduced
the live script estate from 104 to 100.

**This plan's only dependency on W1:**
- W5 now starts from the archived Phase 20f Codex orchestration renderer result with the safety matrix reflecting 96 scripts.

**Checkpoint: Phases 17 and 18 Complete**
- `planning/completed/plan-script-retirement-phase-17.md` archived with executed result
- `planning/completed/plan-script-retirement-phase-18.md` archived with executed result
- `planning/completed/plan-script-retirement-phase-19.md` archived with audit-only blocker evidence
- `planning/completed/plan-script-retirement-phase-20c-self-heal.md` archived with executed result
- `planning/completed/script-retirement-safety-matrix.md` reflects live estate of 96
- `planning/completed/rust-script-parity-ledger.md` records `doctor`, `healthcheck`, `sync-codex-mcp-config`, `render-vscode-mcp-template`, and `self-heal` as `retired locally`

---

### Workstream W2 — Repository Operating Model Alignment

Status: `[x]` Completed

**Problem being fixed:**
`ntk-core-repository-operating-model.instructions.md` describes a .NET/Clean Architecture monorepo with
`src/`, `modules/`, `samples/src/Rent.Service.*`, `dotnet build`, and `dotnet test`. The actual
workspace is a Rust multi-crate layout. Every AI agent consuming this file receives wrong build
commands, wrong topology, and wrong test filters.

#### Task W2.1: Verify Authoritative Source Location

Status: `[x]` Completed

- Confirm the canonical authored source for the repository operating model under the shallow taxonomy.
- The authoritative source is `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`.
- Provider/runtime projections consume that canonical file through `definitions/providers/*` and generated `.github/*` surfaces.
- Commands:
  - `Test-Path definitions\\instructions\\governance\\ntk-governance-repository-operating-model.instructions.md`
- Target paths confirmed post-check:
  - `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md` (authoritative source)
  - `definitions/providers/github/root/AGENTS.md` and `.github/instructions/governance/ntk-governance-repository-operating-model.instructions.md` (consumer/projection surfaces)
- Checkpoint: source of truth identified.

#### Task W2.2: Rewrite Repository Topology Section

Status: `[x]` Completed

- Replace the current .NET topology description with the correct Rust workspace topology.
- Target: the `## Repository Topology` block.
- New content must include:
  - Purpose statement: Rust workspace that unifies the NetToolsKit CLI product with the copilot instruction runtime.
  - Main layout:
    - `crates/` multi-crate Rust workspace
      - `cli` — `ntk` binary entry point, interactive mode, runtime/validation CLI routing
      - `core` — shared domain types, configuration, error taxonomy
      - `ui` — terminal UI primitives, color/unicode detection, rendering
      - `otel` — OpenTelemetry-inspired observability, timing, metrics
      - `orchestrator` — high-level command orchestration, ChatOps, AI pipeline
      - `commands/help` — help discovery and manifest listing
      - `commands/manifest` — manifest parsing, validation, application
      - `commands/runtime` — runtime bootstrap, diagnostics, maintenance, context index
      - `commands/templating` — Handlebars template rendering
      - `commands/validation` — native `validate-all` orchestration and 29 check surfaces
      - `task-worker` — background task execution worker
    - `scripts/` PowerShell compatibility wrappers and retained operator surfaces (104 scripts at W2.2 time, decreasing per retirement plan)
    - `definitions/` authoritative provider and shared instruction assets (projected into `.github/`, `.codex/`, `.claude/`, `.vscode/`)
    - `deployments/` Docker Compose profiles and service configuration
    - `benchmarks/` Criterion benchmarks for CLI-critical paths
    - `docs/` architecture, operations, and UI guidelines
    - `planning/` versioned planning workspace with `active/` and `completed/`
    - `planning/specs/` versioned design workspace with `active/` and `completed/`
    - `templates/` project scaffolding templates (dotnet/ etc.)
    - `.github/` projected and native GitHub surfaces: instructions, prompts, workflows, governance, hooks
- Commit checkpoint for this task:
  - `docs(instructions): update repository topology to Rust workspace layout`

#### Task W2.3: Rewrite Build, Test, and Run Section

Status: `[x]` Completed

- Replace the current `dotnet build` / `dotnet test` block with Rust workspace commands.
- Target: the `## Build, Test, and Run` block.
- New content must include:
  - Build: `cargo build --workspace`
  - Test all: `cargo test --workspace`
  - Test unit only: `cargo test --workspace -- --lib`
  - Lint: `cargo clippy --workspace -- -D warnings`
  - Format: `cargo fmt --all -- --check`
  - Validation (native): `ntk validation all --repo-root . --warning-only false`
  - Runtime diagnostics: `ntk runtime healthcheck --repo-root .`
  - Context index: `ntk runtime update-local-context-index --repo-root .`
  - PowerShell runtime sync: `pwsh -NoProfile -File scripts/runtime/bootstrap.ps1`
  - Vulnerability audit: `pwsh -NoProfile -File scripts/security/Invoke-RustPackageVulnerabilityAudit.ps1 -ProjectPath . -FailOnSeverities Critical,High`
  - Artifact locations: `.build/cargo-target`, `.build/coverage`, `.deployment/local/`
- Remove all references to `dotnet build`, `dotnet test`, `dotnet run`, `dotnet pack`, NetToolsKit.sln, net8.0, net9.0.
- Commit checkpoint for this task:
  - `docs(instructions): replace dotnet build/test commands with cargo workspace commands`

#### Task W2.4: Update Style, UI/API, and Testing Sections

Status: `[x]` Completed

- Target: `## Style and Artifact Hygiene`, `## UI, API, and Database Conventions`, `## Testing Expectations`.
- Changes:
  - Style/artifacts: verify EOF policy, `.build/` and `.deployment/` references, and naming rules are correct for Rust (they may already be correct). Remove any C# style entries (`PascalCase for types` etc.) that don't apply to Rust.
  - Add Rust-specific style entries: `snake_case` for types, functions, modules; `SCREAMING_SNAKE_CASE` for constants; `CamelCase` for types/enums matching Rust idioms.
  - Testing: replace `{Project}.Tests` / `{TypeName}Tests.cs` / `[Trait("Category","Unit")]` with Rust test conventions: `#[cfg(test)]` modules, `#[test]` and `#[tokio::test]`, integration tests under `crates/*/tests/`, test file naming as `<module>_tests.rs`.
  - Keep the pt-BR chat restriction and EN code/database restriction.
  - Keep EOF policy.
- Commit checkpoint:
  - `docs(instructions): align style and test conventions with Rust workspace`

#### Task W2.5: Update Domain Instruction Map Section

Status: `[x]` Completed

- Target: `## Domain Instruction Map`.
- The current map includes Vue/Quasar, ORM/EF Core, Docker/K8s, .NET-specific entries.
- New correct map for this workspace:
  - Development (Rust):
    - `instructions/architecture/backend/ntk-backend-rust-code-organization.instructions.md`
    - `instructions/architecture/backend/ntk-backend-rust-testing.instructions.md`
    - `instructions/architecture/backend/ntk-backend-architecture-core.instructions.md`
    - `instructions/data/ntk-data-orm.instructions.md`
    - `instructions/security/ntk-security-api-high-performance.instructions.md`
  - Infrastructure:
    - `instructions/operations/containers/ntk-runtime-docker.instructions.md`
    - `instructions/operations/devops/ntk-runtime-ci-cd-devops.instructions.md`
    - `instructions/operations/devops/ntk-runtime-workflow-generation.instructions.md`
    - `instructions/operations/reliability/ntk-runtime-observability-sre.instructions.md`
    - `instructions/operations/reliability/ntk-runtime-platform-reliability-resilience.instructions.md`
    - `instructions/operations/automation/ntk-runtime-powershell-script-creation.instructions.md`
    - `instructions/operations/automation/ntk-runtime-powershell-execution.instructions.md`
  - Security:
    - `instructions/security/ntk-security-vulnerabilities.instructions.md`
    - `instructions/security/ntk-security-api-high-performance.instructions.md`
    - `instructions/security/ntk-security-data-privacy-compliance.instructions.md`
  - Testing:
    - `instructions/architecture/backend/ntk-backend-rust-testing.instructions.md`
    - `instructions/architecture/backend/ntk-backend-integration-testing.instructions.md`
    - `instructions/architecture/frontend/ntk-frontend-e2e-testing.instructions.md`
    - `instructions/process/delivery/ntk-process-tdd-verification.instructions.md`
    - `instructions/operations/quality/ntk-runtime-static-analysis-sonarqube.instructions.md`
  - Documentation and process: keep as-is (shared lifecycle files apply generically).
  - Remove: `dotnet-csharp`, `orm`, `database`, `database-configuration-operations`, `vue-quasar`, `vue-quasar-architecture`, `frontend`, `ui-ux`, `microservices-performance`.
- Commit checkpoint:
  - `docs(instructions): align domain instruction map with Rust workspace scope`

#### Task W2.6: Re-render Projection and Validate

Status: `[x]` Completed

- Trigger a re-render so provider/runtime consumer surfaces reflect the canonical definitions roots:
  - `ntk runtime bootstrap --repo-root . --mirror` or the equivalent `scripts/runtime/render-github-instruction-surfaces.ps1` invocation
- Validate:
  - `& .\.build\target\debug\ntk.exe validation instructions --repo-root . --warning-only false`
  - `& .\.build\target\debug\ntk.exe validation planning-structure --repo-root . --warning-only false`
  - `git diff --check`
- Commit checkpoint (after re-render):
  - `docs(instructions): re-render github instruction surfaces with updated operating model`

**Checkpoint: W2 Operating Model Alignment Complete**
- authoritative source confirmed at `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`
- projected GitHub copy now derives from the canonical `definitions/*` roots and lands under `.github/instructions/governance/ntk-governance-repository-operating-model.instructions.md`
- repository topology now describes the Rust multi-crate workspace instead of the old .NET monorepo layout
- build/test/run guidance now points to `cargo build`, `cargo test`, `cargo clippy`, `cargo fmt`, native `ntk validation`, runtime continuity commands, and the Rust vulnerability audit
- style, testing, workflow patterns, and domain instruction map now align to the actual Rust workspace and retained PowerShell compatibility model
- validation evidence:
  - `cargo run -p nettoolskit-cli -- validation instructions --repo-root . --warning-only false` ✅
  - `cargo run -p nettoolskit-cli -- validation planning-structure --repo-root . --warning-only false` ✅
  - `git diff --check` ✅

---

### Workstream W3 — CI PowerShell Parity Test Coverage

Status: `[x]` Completed

**Problem being fixed:**
23 `scripts/tests/runtime/*.ps1` files are marked as `compatibility wrapper retained intentionally`
and described as the canonical parity harness in the safety matrix. `ci.yml` has zero PowerShell
test invocations; these tests only run on developer machines.

#### Task W3.1: Audit Parity Test Scope and Execution Model

Status: `[x]` Completed

- Review each of the 23 parity test files to determine:
  - Whether the test uses Pester syntax (`Describe`, `It`, `Should`) or is a standalone script.
  - Whether any test requires local file system access that would not be available in CI.
  - Whether any test depends on `ntk` being built and in PATH.
- Audit result:
  - the retained parity harness is a standalone PowerShell script suite, not a real Pester suite
  - CI coverage should therefore use the native `ntk validation runtime-script-tests` surface instead of installing/running Pester
  - the native validator now normalizes Windows extended-length paths before invoking PowerShell test files, which prevents `\\?\` path forwarding from breaking shared bootstrap resolution
- Target paths: `scripts/tests/runtime/*.ps1`
- Commands:
  - `rg "Describe|Context|It |Should " scripts/tests/runtime/ --count`
  - `rg "Invoke-Pester|Install-Module.*Pester" scripts/tests/runtime/`
- Checkpoint: parity test inventory complete; native runtime-script-tests gate selected.

#### Task W3.2: Add Windows Native Runtime Parity Job to `ci.yml`

Status: `[x]` Completed

- Target: `.github/workflows/ci.yml`
- Add a new job `pwsh-runtime-parity` using `windows-latest` runner.
- Job structure:
  - Checkout
  - Set up Rust toolchain (stable)
  - Cache Rust build artifacts via `Swatinem/rust-cache@v2`
  - Build the workspace: `cargo build --workspace`
  - Run parity tests through the native validation boundary:
    - `cargo run -q -p nettoolskit-cli -- validation runtime-script-tests --repo-root . --warning-only false`
- The job runs on `push` and `pull_request` targeting `main` and `develop` (same triggers as existing jobs).
- Commands to validate locally before merging:
  - `cargo run -q -p nettoolskit-cli -- validation runtime-script-tests --repo-root . --warning-only false`
- Commit checkpoint:
  - `ci(workflows): add windows runtime parity validation job for retained PowerShell tests`

#### Task W3.3: Address Test Failures and Stabilize

Status: `[x]` Completed

- Fixed the two remaining failing parity harness paths:
  - `runtime-scripts.tests.ps1` now scaffolds canonical `definitions/providers/github/{governance,policies}` fixture roots required by `render-github-instruction-surfaces.ps1`
  - `agent-orchestration-engine.tests.ps1` now isolates `validate-stage.ps1` from the live projected `.github` state by overriding the managed runtime binary with a fake success-only `ntk` shim for the orchestration smoke path
- The native runtime validator now executes all 23 retained parity scripts successfully on Windows.
- Commit checkpoint (if fixes needed):
  - `test(runtime): stabilize PowerShell parity tests for CI execution`

#### Task W3.4: Record Coverage Model and Evidence

Status: `[x]` Completed

- Updated `scripts/README.md` so the runtime/operator surface now documents `ntk validation runtime-script-tests --repo-root . --warning-only false` as the supported parity harness command.
- Validation evidence:
  - `cargo test -p nettoolskit-validation runtime_script_tests_tests --quiet` ✅
  - `cargo run -q -p nettoolskit-cli -- validation runtime-script-tests --repo-root . --warning-only false` ✅
- Commit checkpoint:
  - `docs(runtime): document native PowerShell parity coverage model`

**Checkpoint: W3 PowerShell Parity Coverage Complete**
- the retained 23-script runtime parity harness now runs through the native `ntk validation runtime-script-tests` surface instead of an implied Pester dependency
- Windows path normalization in the validator prevents `\\?\`-prefixed test script invocations from breaking shared bootstrap discovery
- the remaining two smoke regressions were fixed by updating the canonical GitHub fixture roots in `runtime-scripts.tests.ps1` and isolating `validate-stage.ps1` with a fake managed runtime binary in `agent-orchestration-engine.tests.ps1`
- `.github/workflows/ci.yml` now has a dedicated `pwsh-runtime-parity` Windows gate
- validation evidence:
  - `cargo test -p nettoolskit-validation runtime_script_tests_tests --quiet` ✅
  - `cargo run -q -p nettoolskit-cli -- validation runtime-script-tests --repo-root . --warning-only false` ✅

---

### Workstream W4 — CLI Surface Documentation

Status: `[x]` Completed

**Problem being fixed:**
The `ntk` binary exposes 50+ subcommands across 5 groups (`manifest`, `runtime`, `validation`,
`service`, `completions`) plus global args. None of these are documented beyond 3 bullet points
in `crates/cli/README.md` and a features list in the root `README.md`.

#### Task W4.1: Compile Full Command Reference

Status: `[x]` Completed

- Extract all command groups and subcommands from:
  - `crates/cli/src/main.rs` — top-level `Commands` enum
  - `crates/cli/src/runtime_commands.rs` — `RuntimeCommand` enum (12 variants)
  - `crates/cli/src/validation_commands.rs` — `ValidationCommand` enum (29 variants)
- Build the reference table with: `ntk <group> <subcommand>` form, one-line description.
- Commands:
  - `rg "/// " crates/cli/src/main.rs crates/cli/src/runtime_commands.rs crates/cli/src/validation_commands.rs`
- Checkpoint: command inventory compiled from live `--help` output and aligned with the current Clap command surfaces in `crates/cli/src/main.rs`, `crates/cli/src/runtime_commands.rs`, and `crates/cli/src/validation_commands.rs`.

#### Task W4.2: Update Root `README.md` — Command Reference Section

Status: `[x]` Completed

- Target: `README.md`
- Add a `### Command Reference` subsection inside `## Features` or as its own `## Command Reference` section (after `## Features`).
- Content structure:

  ```
  ## Command Reference

  Run `ntk --help` to see all available commands and flags.

  | Group | Command | Description |
  |---|---|---|
  | manifest | ntk manifest list | Discover available manifests in the workspace |
  | manifest | ntk manifest check | Validate manifest structure and references |
  | manifest | ntk manifest render | Preview generated file output (dry-run) |
  | manifest | ntk manifest apply | Apply manifest and generate or update files |
  | runtime | ntk runtime pre-tool-use | Pre-tool-use AI editor hook (EOF hygiene, safety checks) |
  | runtime | ntk runtime healthcheck | Full runtime, provider surface, and drift health report |
  | runtime | ntk runtime export-enterprise-trends | Export consolidated validation/security trends dashboard |
  | runtime | ntk runtime bootstrap | Sync runtime surfaces (github/codex/claude/vscode profiles) |
  | runtime | ntk runtime doctor | Provider surface drift diagnosis |
  | runtime | ntk runtime update-local-context-index | Build or rebuild the repository context index |
  | runtime | ntk runtime query-local-context-index | Query the context index for targeted continuity retrieval |
  | runtime | ntk runtime export-planning-summary | Print active plan context for handoff and execution reviews |
  | runtime | ntk runtime apply-vscode-templates | Apply tracked VS Code workspace templates |
  | runtime | ntk runtime trim-trailing-blank-lines | Remove trailing blank lines from changed files |
  | runtime | ntk runtime pre-commit-eof-hygiene | Run EOF hygiene on git-staged files |
  | runtime | ntk runtime setup-git-hooks | Install repository git hooks locally |
  | runtime | ntk runtime setup-global-git-aliases | Register global git aliases (e.g. git trim-eof) |
  | validation | ntk validation all | Run all validation checks with profile selection |
  | validation | ntk validation instructions | Validate instruction file coverage and metadata |
  | validation | ntk validation planning-structure | Validate planning active/completed/specs layout |
  | validation | ntk validation readme-standards | Validate README files against the baseline |
  | validation | ntk validation policy | Validate agent and instruction policy baselines |
  | validation | ntk validation security-baseline | Validate security baseline governance artifacts |
  | validation | ntk validation agent-orchestration | Validate orchestrator pipeline contracts |
  | validation | ntk validation agent-permissions | Validate agent permission matrix |
  | validation | ntk validation agent-skill-alignment | Validate skill alignment against governance catalog |
  | validation | ntk validation agent-hooks | Validate VS Code hook definitions and targets |
  | validation | ntk validation audit-ledger | Validate audit ledger integrity |
  | validation | ntk validation architecture-boundaries | Validate crate dependency boundaries |
  | validation | ntk validation authoritative-source-policy | Validate official source map entries |
  | validation | ntk validation compatibility-lifecycle-policy | Validate COMPATIBILITY.md lifecycle table |
  | validation | ntk validation dotnet-standards | Validate .NET compatibility wrapper standards |
  | validation | ntk validation instruction-architecture | Validate instruction file graph and ownership |
  | validation | ntk validation instruction-metadata | Validate instruction metadata and applyTo patterns |
  | validation | ntk validation powershell-standards | Validate PowerShell script header and style |
  | validation | ntk validation release-governance | Validate release governance evidence artifacts |
  | validation | ntk validation release-provenance | Validate release provenance and traceability chain |
  | validation | ntk validation routing-coverage | Validate routing catalog coverage completeness |
  | validation | ntk validation runtime-script-tests | Validate runtime parity test inventory |
  | validation | ntk validation security-baseline | Validate security baseline governance |
  | validation | ntk validation shared-script-checksums | Validate shared script checksum manifest |
  | validation | ntk validation shell-hooks | Validate shell hook dispatch contracts |
  | validation | ntk validation supply-chain | Validate supply-chain dependency baseline |
  | validation | ntk validation template-standards | Validate template structure and metadata |
  | validation | ntk validation warning-baseline | Validate warning baseline against current counts |
  | validation | ntk validation workspace-efficiency | Validate VS Code workspace efficiency settings |
  | service | ntk service | Start background HTTP service (health, ready, task/submit endpoints) |
  | completions | ntk completions <shell> | Generate shell completions (bash, zsh, fish, powershell) |
  ```

- Also add a `### Global Flags` subsection:
  ```
  ### Global Flags

  | Flag | Type | Description |
  |---|---|---|
  | --log-level | string | Set logging level: off, error, warn, info, debug, trace |
  | --config | string | Path to configuration file |
  | --verbose / -v | bool | Enable verbose output |
  ```

- Outcome:
  - `README.md` now exposes a dedicated `## Command Reference` section after `## Features`.
  - The section documents the live top-level surface, manifest commands, AI usage commands, runtime commands, validation commands, and global flags.
  - The contents index now links directly to the command reference section.

#### Task W4.3: Update `crates/cli/README.md` — Expand Features and Add Subcommand Table

Status: `[x]` Completed

- Target: `crates/cli/README.md`
- Replace the current 3-feature bullet list under `## Features` with a grouped table by command surface.
- Add a `## Quick Start` section showing the most common invocations:
  - `ntk manifest list`
  - `ntk validation all --repo-root .`
  - `ntk runtime healthcheck --repo-root .`
  - `ntk service --port 8080`
  - `ntk completions powershell | Out-String | Invoke-Expression`
- Outcome:
  - `crates/cli/README.md` now documents the `ntk` binary as a developer orchestrator entry point instead of a minimal library wrapper.
  - The feature section is grouped by command surface.
  - `## Quick Start` now covers the most common operator invocations.
  - `## Command Surfaces` summarizes the top-level groups plus representative manifest, AI, runtime, and validation commands.
  - `## Service Mode` now documents the real HTTP routes:
    - `GET /`
    - `GET /health`
    - `GET /ready`
    - `POST /task/submit`
    - `POST /chatops/telegram/webhook`
    - `POST /chatops/discord/interactions`
  - The service-mode section now points to the real environment variables:
    - `NTK_SERVICE_AUTH_TOKEN`
    - `NTK_CHATOPS_TELEGRAM_WEBHOOK_SECRET_TOKEN`
    - `NTK_CHATOPS_DISCORD_INTERACTIONS_PUBLIC_KEY`
    - replay-protection and timeout controls
  - Added a small Mermaid architecture section and preserved the embedded API contract for `interactive_mode`.

#### Task W4.4: Validate All README Changes

Status: `[x]` Completed

- Run: `& .\.build\target\debug\ntk.exe validation readme-standards --repo-root . --warning-only false`
- Run: `& .\.build\target\debug\ntk.exe validation planning-structure --repo-root . --warning-only false`
- Run: `git diff --check`
- Validation evidence:
  - `cargo run -q -p nettoolskit-cli -- validation readme-standards --repo-root . --warning-only false` ✅
  - `cargo run -q -p nettoolskit-cli -- validation planning-structure --repo-root . --warning-only false` ✅
  - `git diff --check` ✅

**Checkpoint: W4 CLI Surface Documentation Complete**
- `README.md` now exposes a durable command reference for the live `ntk` surface instead of a feature-only summary.
- `crates/cli/README.md` now serves as the operator-facing reference for interactive use, service mode, shell completions, validation, and runtime maintenance.
- Command tables were derived from the current Clap surfaces instead of stale documentation.

---

### Workstream W5 — Post-Phase-20c Domain Consumer Migration (Phases 19–22)

Status: `[ ]` Pending

**Pre-condition:** satisfied. `plan-script-retirement-phase-17.md`, `plan-script-retirement-phase-18.md`, `plan-script-retirement-phase-19.md`, and `plan-script-retirement-phase-20c-self-heal.md` are now in `planning/completed/`, so future consumer sweeps can start from a stable script estate.

**Script estate after Phase 20f Codex orchestration renderer retirement:** 96 total (33 retained by policy + 63 `retain until consumer migration`).

The 63 `retain until` scripts are distributed:

| Domain | Count | Safety Matrix Status |
|---|---:|---|
| `scripts/common/*.ps1` | 15 | `retain until consumer migration completes` |
| `scripts/runtime/*.ps1` excl. hooks and retired Phases 17, 18, 20c, 20d, 20e, and 20f leaves | 30 | `retain until consumer migration completes` |
| `scripts/security/*.ps1` | 6 | `retain until consumer migration completes` |
| `scripts/governance/*.ps1` | 2 | `retain until consumer migration completes` |
| `scripts/orchestration/**/*.ps1` | 10 | `retain until consumer migration completes` |

This workstream creates four sub-phase plans (Phase 19–22). Each sub-phase plan follows the same
    pattern as Phase 17 and Phase 18: it is created as an active plan, executed, then archived to completed.

The completed tactical slice `plan-script-retirement-phase-20c-self-heal.md` is the precedent for a runtime leaf cutover that already had native CLI parity and same-slice consumer proof. It does not replace the broader Phase 20 runtime consumer sweep.

#### Task W5.1: Create Phase 19 Plan — `scripts/common/*.ps1` (15)

Status: `[x]` Completed

- Created and executed:
  - `planning/completed/plan-script-retirement-phase-19.md`
  - `planning/specs/completed/spec-script-retirement-phase-19.md`
- The phase was intentionally audit-only because every shared helper still had live local consumers.
- The consumer sweep established the blocker profile that future domain sweeps must clear before any common-helper deletion is attempted.

#### Task W5.2: Execute Phase 19 Consumer Sweep

Status: `[x]` Completed

- Executed the common-domain consumer sweep and confirmed no zero-consumer deletions were safe in this phase.
- Representative blockers recorded in the archived phase artifacts:
  - `common-bootstrap.ps1` is still the high-fanout shared bootstrap loader across runtime, security, orchestration, and tests.
  - `provider-surface-catalog.ps1` now supports mixed native/script-backed renderer dispatch, but the common-domain helper still remains blocked from deletion by the remaining path-backed renderer leaves, especially `render-mcp-runtime-artifacts.ps1`.
  - `mcp-runtime-catalog.ps1` remains blocked by `sync-vscode-global-mcp.ps1` and `render-mcp-runtime-artifacts.ps1`.
  - `runtime-execution-context.ps1`, `runtime-install-profiles.ps1`, and `runtime-operation-support.ps1` remain blocked by `bootstrap.ps1`, `install.ps1`, and retained runtime parity flows.
- Updated the continuity workstream so the runtime-domain tactical leaf cutovers can proceed without falsely implying the common domain is deletion-ready.

#### Task W5.3: Create Phase 20 Plan — `scripts/runtime/*.ps1` Excluding Hooks (30)

Status: `[x]` Completed

- This is the largest single domain: 30 scripts after Phases 17, 18, the tactical Phase 20c self-heal slice, the tactical Phase 20d provider-surface dispatcher slice, the tactical Phase 20e catalog-native renderer slice, and the tactical Phase 20f Codex orchestration slice remove `doctor.ps1`, `healthcheck.ps1`, `sync-codex-mcp-config.ps1`, `render-vscode-mcp-template.ps1`, `self-heal.ps1`, `render-provider-surfaces.ps1`, `render-codex-compatibility-surfaces.ps1`, and `render-codex-orchestration-surfaces.ps1`.
- Created:
  - `planning/completed/plan-script-retirement-phase-20-runtime-consumer-sweep.md`
  - `planning/specs/completed/spec-script-retirement-phase-20-runtime-consumer-sweep.md`
- The active phase plan freezes the 30 remaining runtime leaves into:
  - Slice A: projection, profile, sync, and workspace runtime surfaces
  - Slice B: orchestration runtime entrypoints and replay helpers
  - Slice C: bootstrap, install, and cleanup surfaces
- Phase 20 now uses internal slice names inside one runtime-domain plan instead of opening new `phase-20a/20b/20c` plan files, which avoids collision with the already-completed tactical `Phase 20c self-heal` slice.
- Commit checkpoint:
  - `docs(planning): open Phase 20 runtime consumer sweep with internal slice boundaries`

#### Task W5.4: Execute Phase 20 Consumer Sweep (Sub-slices A, B, C)

Status: `[x]` Completed (all sub-slices audit-only; zero deletions)

- Run consumer sweeps for each sub-slice.
- For each zero-consumer script: delete it.
- For each retained: document the retaining consumer.
- Slice A result recorded:
  - zero-consumer leaves: none
  - deleted leaves: none
  - blocking consumers were confirmed in the provider-surface projection catalog, provider README/operator docs, `install.ps1`, runtime parity harness tests, and the shell-hook validation fixture
  - the runtime domain therefore stays at `30` retained leaves for now
- Slice B result recorded:
  - zero-consumer leaves: none
  - deleted leaves: none
  - blocking consumers were confirmed in orchestration policies, Codex orchestration README surfaces, orchestrator parity tests, validation fixtures, retained runtime parity tests, `validate-stage.ps1`, and the `run/resume/replay` runtime chain
  - the runtime domain still stays at `30` retained leaves
- Slice C result recorded:
  - zero-consumer leaves: none
  - deleted leaves: none
  - blocking consumers were confirmed in bootstrap/install fanout, Codex and Claude runtime-sync surfaces, shared checksum governance, git hooks, stage scripts, retained runtime parity tests, super-agent housekeeping flows, and repository operating-model guidance
  - the runtime domain still stays at `30` retained leaves
- After each sub-slice:
  - Update safety matrix and parity ledger.
  - Run the Phase 20 validation checklist.
- Archive `plan-script-retirement-phase-20-runtime-consumer-sweep.md` and `spec-script-retirement-phase-20-runtime-consumer-sweep.md` to completed only when all sub-slices are done. ✅
- Closeout:
  - Phase 20 is now archived as an audit-only runtime-domain sweep with explicit retained-blocker evidence for all three slices
- Validation checklist:
  - `cargo test -p nettoolskit-runtime --quiet`
  - `cargo test -p nettoolskit-cli --test test_suite runtime_commands_tests --quiet`
  - `& .\.build\target\debug\ntk.exe validation all --repo-root . --warning-only false`
  - `git diff --check`
- Commit checkpoint (one per sub-slice):
  - `docs(runtime-retirement): Phase 20 Slice A — record audit-only consumer proof for projection, profile, and sync scripts`
  - `docs(runtime-retirement): Phase 20 Slice B — record audit-only consumer proof for orchestration runtime scripts`
  - `docs(runtime-retirement): Phase 20 Slice C — record audit-only consumer proof for bootstrap, install, and cleanup scripts`

#### Task W5.5: Create and Execute Phase 21 — `scripts/security/*.ps1` + `scripts/governance/*.ps1` (8)

Status: `[x]` Completed (audit-only; zero deletions)

- Created:
  - `planning/completed/plan-script-retirement-phase-21-security-governance-sweep.md`
  - `planning/specs/completed/spec-script-retirement-phase-21-security-governance-sweep.md`
- Special constraint for `scripts/security/*.ps1` (6): `.github/governance/shared-script-checksums.manifest.json` explicitly tracks this domain. The Phase 21 plan must include a task to update the checksums manifest before deleting any security scripts.
- Consumer sweep commands:
  - `Get-ChildItem scripts\security, scripts\governance -Filter "*.ps1" | ForEach-Object { rg $_.Name .github\governance, scripts --type ps1 --count }`
- After sweep:
  - No deletions were safe, so `definitions/providers/github/governance/shared-script-checksums.manifest.json` stayed unchanged.
  - The blocker graph is now explicit for both the security and governance domains.
- Validation checklist:
  - `cargo test -p nettoolskit-validation --quiet`
  - `& .\.build\target\debug\ntk.exe validation all --repo-root . --warning-only false`
  - `git diff --check`
- Commit checkpoint:
  - `docs(runtime-retirement): Phase 21 — record audit-only consumer proof for security and governance scripts`

#### Task W5.6: Create and Execute Phase 22 — `scripts/orchestration/**/*.ps1` (10)

Status: `[x]` Completed (audit-only; zero deletions)

- Create `planning/specs/completed/spec-script-retirement-phase-22-orchestration-sweep.md` and `planning/completed/plan-script-retirement-phase-22-orchestration-sweep.md`.
- The 10 orchestration scripts are staged-execution wrappers (`intake-stage.ps1`, `plan-stage.ps1`, `spec-stage.ps1`, `implement-stage.ps1`, `review-stage.ps1`, `validate-stage.ps1`, `closeout-stage.ps1`, `route-stage.ps1`, `invoke-codex-dispatch.ps1`, `invoke-task-worker.ps1`).
- Consumer sweep commands:
  - `Get-ChildItem scripts\orchestration -Filter "*.ps1" -Recurse | ForEach-Object { rg $_.Name scripts, definitions, .github --count }`
- The orchestration scripts are likely consumed by each other — the sweep must map the full dependency graph before deleting any leaf.
- After sweep:
  - No deletions were safe, so `crates/orchestrator` docs and skill surfaces did not need same-slice re-pointing.
  - The blocker graph is now explicit for the entire orchestration domain.
- Validation checklist:
  - `cargo test -p nettoolskit-orchestrator --quiet` (known unrelated baseline failure in ChatOps tool-scope allowlist test)
  - `cargo test -p nettoolskit-cli --test test_suite --quiet`
  - `& .\.build\target\debug\ntk.exe validation all --repo-root . --warning-only false`
  - `pwsh -NoProfile -File scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
  - `git diff --check`
- Commit checkpoint:
  - `docs(runtime-retirement): Phase 22 — record audit-only consumer proof for orchestration stage scripts`

#### Task W5.7: Post-Phase-22 Retention Audit

Status: `[x]` Completed

- After Phases 19–22, the live estate remains `96`, not near the `retain wrapper intentionally` floor of `33`.
- Audit result:
  - `33` scripts are still covered by the explicit `retain wrapper intentionally` table.
  - `63` scripts remain blocked under the explicit audited-domain rows for:
    - `scripts/common/*.ps1` (`15`)
    - `scripts/runtime/*.ps1` excluding hooks (`30`)
    - `scripts/security/*.ps1` (`6`)
    - `scripts/governance/*.ps1` (`2`)
    - `scripts/orchestration/**/*.ps1` (`10`)
  - the `96 - 33 = 63` gap exactly matches the blocked-domain total, so no extra drift bucket exists outside the safety matrix.
  - grouped domain rows, not per-file rows, are the intended explicit accounting model for the remaining live estate.
- Update `script-retirement-safety-matrix.md` and `rust-script-parity-ledger.md` to close this workstream.
- Commit checkpoint:
  - `docs(planning): close the post-Phase-22 retention audit and finish the scripted consumer-migration planning sequence`

**Checkpoint: W5 Consumer Migration Sequence Complete**
- Phases 19, 20, 21, and 22 are now all archived with explicit blocker evidence.
- No further domain-level consumer-sweep planning remains open inside this workstream.
- Future retirement progress now depends on blocker-reduction workstreams, not on additional discovery sweeps.

---

### Workstream W6 — `copilot-instructions` Phase 8 Rust Directives

Status: `[x]` Completed locally / external implementation handed off

**Problem being fixed:**
`copilot-instructions` Phase 8 was blocked on explicit Rust directives. Those directives are now
versioned in the external repo, but no `Cargo.toml`, no crates, and no Rust files have been created yet.
The remaining W6 work is implementation in the external repository, not planning discovery.

This workstream executes inside the `copilot-instructions` repository. All file paths below are relative to that repo root.

#### Task W6.1: Provide Phase 8 Rust Directives

Status: `[x]` Completed

- Updated `planning/active/plan-rust-runtime-engine-foundation-phase-8.md` and `planning/specs/active/spec-rust-runtime-engine-foundation-phase-8.md` in `C:\Users\tguis\copilot-instructions`.
- Directives to provide:
  - **Rust edition**: 2021
  - **MSRV**: 1.85.0 (matches `nettoolskit-copilot`)
  - **Deny list**: align with `nettoolskit-copilot/deny.toml` (auditable versions, license allowlist)
  - **Workspace layout**:
    - `Cargo.toml` at repo root (workspace manifest)
    - `src/` reserved for future crate modules
    - First crate: `src/runtime/` → crate name `ntk-runtime-engine`
  - **`ntk-runtime-engine` initial scope**: export parity contracts for:
    - bootstrap surface (wrapping `scripts/runtime/bootstrap.ps1` behavior)
    - renderer dispatch (wrapping `scripts/runtime/render-*.ps1` behavior)
    - These are parity contracts only — no script deletions until smoke tests pass
  - **Cargo config**: set `target-dir = "../.build/cargo-target"` via `.cargo/config.toml`
  - **clippy flags**: `-D warnings` in `RUSTFLAGS` or CI config
- External planning checkpoint:
  - branch: `feature/context-mode`
  - commit: `bd5502c`
- Commit checkpoint:
  - `docs(planning): provide Phase 8 Rust directives for ntk-runtime-engine`

#### Task W6.2: Scaffold Cargo Workspace

Status: `[~]` External implementation pending (`copilot-instructions`)

- Create `Cargo.toml` at `copilot-instructions` root:
  ```toml
  [workspace]
  members = ["src/runtime"]
  resolver = "2"
  ```
- Create `.cargo/config.toml`:
  ```toml
  [build]
  target-dir = ".build/cargo-target"
  ```
- Create `rust-toolchain.toml`:
  ```toml
  [toolchain]
  channel = "1.85.0"
  ```
- Create `deny.toml` aligned with the policy in `nettoolskit-copilot/deny.toml`.
- Create `rustfmt.toml` with the same settings as `nettoolskit-copilot/rustfmt.toml`.
- Update `.gitignore` to include `.build/` and `Cargo.lock` (or track lock if binary).
- Commands:
  - `cargo init --lib src/runtime --name ntk-runtime-engine --edition 2021`
  - `cargo build --workspace`
  - `cargo clippy --workspace -- -D warnings`
  - `cargo test --workspace`
- Checkpoint: `cargo build --workspace` passes with zero warnings.
- Commit checkpoint:
  - `chore(cargo): scaffold Cargo workspace and ntk-runtime-engine crate`

#### Task W6.3: Implement Bootstrap Parity Contract

Status: `[~]` External implementation pending (`copilot-instructions`)

- In `src/runtime/src/lib.rs`, define the bootstrap parity surface:
  - `pub struct RuntimeBootstrapParity { pub legacy_script: &'static str, pub native_ready: bool }`
  - `pub const BOOTSTRAP_SCRIPTS: &[RuntimeBootstrapParity]` listing each `scripts/runtime/*.ps1` surface
  - This is a contract-first implementation: no logic, just the contract and tests proving the contract compiles.
- Add `src/runtime/tests/bootstrap_parity_tests.rs` with:
  - Test confirming the script count matches expected
  - Test confirming `bootstrap.ps1` is listed as a known surface
- Commit checkpoint:
  - `feat(runtime-engine): bootstrap parity contract with surface inventory`

#### Task W6.4: Implement Renderer Dispatch Parity Contract

Status: `[~]` External implementation pending (`copilot-instructions`)

- In `src/runtime/src/renderer.rs`, define the renderer dispatch parity surface:
  - `pub enum RendererTarget { GithubInstructions, CodexCompatibility, ClaudeRuntime, VscodeProfiles, ProviderSurfaces, McpArtifacts }`
  - `pub const RENDERER_SCRIPTS: &[(&str, RendererTarget)]` mapping each `scripts/runtime/render-*.ps1` to its target
  - Contract-first: no logic, just enumeration and tests.
- Add `src/runtime/tests/renderer_parity_tests.rs`.
- Commit checkpoint:
  - `feat(runtime-engine): renderer dispatch parity contract with target enumeration`

#### Task W6.5: Verify Phase 8 Validates and Update Evidence

Status: `[~]` External implementation pending (`copilot-instructions`)

- Run validation to confirm the new Cargo workspace does not break the existing PowerShell validation surface:
  - `cargo build --workspace`
  - `cargo test --workspace`
  - `pwsh -NoProfile -File scripts/validation/validate-planning-structure.ps1 -RepoRoot . -WarningOnly:$false`
  - `pwsh -NoProfile -File scripts/validation/validate-readme-standards.ps1 -RepoRoot .`
- If the `scripts/validation/validate-*.ps1` pattern is still live in `copilot-instructions` at execution time, include them; otherwise route through any native validator available.
- Update `planning/active/plan-rust-runtime-engine-foundation-phase-8.md` to reflect completed tasks.
- Commit checkpoint:
  - `docs(planning): Phase 8 first implementation slice complete — bootstrap and renderer parity contracts`

---

## Validation Checklist (Cross-Workstream)

These checks apply at the end of each stable commit before PR:

- `cargo build --workspace`
- `cargo test --workspace --quiet`
- `cargo clippy --workspace -- -D warnings`
- `& .\.build\target\debug\ntk.exe validation planning-structure --repo-root . --warning-only false`
- `& .\.build\target\debug\ntk.exe validation instructions --repo-root . --warning-only false`
- `& .\.build\target\debug\ntk.exe validation readme-standards --repo-root . --warning-only false`
- `& .\.build\target\debug\ntk.exe validation all --repo-root . --warning-only false`
- `pwsh -NoProfile -File scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- `git diff --check`

For W6 (cross-repo, `copilot-instructions`):
- `cargo build --workspace` (in `copilot-instructions` repo)
- `cargo test --workspace` (in `copilot-instructions` repo)
- Legacy PowerShell validation still passes in that repo

---

## Execution Order and Parallelism

```
Phases 17-18 (W1) ─────────────────────────────── MUST COMPLETE first ─────────────────────┐
                                                                                              │
W2 (op model)     ── start immediately ──────────── parallel ────────────────────────── W2 done
W3 (CI parity)    ── start immediately ──────────── parallel ────────────────────────── W3 done
W4 (CLI docs)     ── start immediately ──────────── parallel ────────────────────────── W4 done
W6 (cross-repo)   ── start independently ────────── parallel (different repo) ─────── W6 done
                                                                                              │
W5.1 Phase 19     ──────────────────────────────────── after W1 ──────────────────────── Ph19 done
W5.3 Phase 20     ─────────────────────────────────────────────── after Ph19 ─────── Ph20 done
W5.5 Phase 21     ──────────────────────────────────────────────────────── after Ph20 ─ Ph21 done
W5.6 Phase 22     ─────────────────────────────────────────────────────────────────────── Ph22 done
```

W2, W3, W4, and W6 can be executed in any order relative to each other and do not need the tactical retirement phases to complete first.
W5 phases must be sequential and depend on Phases 17 and 18 closing.

---

## Risks And Fallbacks

| # | Risk | Fallback |
|---|---|---|
| R1 | Editing canonical `definitions/instructions/` assets triggers a provider-surface re-render that breaks unrelated generated surfaces | Run `ntk validation all` in warning-only mode first; verify diff before committing |
| R2 | `pwsh-parity` CI job fails on first run due to environment differences | Use `continue-on-error: true` for the initial merge; switch to `false` after baseline is stable |
| R3 | Phase 19 common-script consumer sweep finds 12+ blockers | Accept a partial Phase 19 that deletes only zero-consumer scripts; document retained ones explicitly |
| R4 | Phase 20 is too large for one PR review | Split into 20a/20b/20c sub-phases; each sub-phase has its own commit checkpoint |
| R5 | Phase 21 security script deletion requires updating `shared-script-checksums.manifest.json` in a separate PR | Create the manifest update as a prerequisite commit before the deletion PR |
| R6 | `copilot-instructions` Phase 8 scaffold breaks the existing PowerShell validation in that repo | Add `cargo build` after workspace creation; if `validate-planning-structure.ps1` fails, debug before proceeding |
| R7 | A legacy consumer still assumes `definitions/shared/instructions/` is authored source | Keep the compatibility mirror available, but update only the canonical `definitions/instructions/` source and re-render consumer surfaces instead of editing legacy mirrors directly |

---

## Closeout Expectations

- Each workstream closes with its own commit and its own stable intermediate checkpoint.
- W2 closes when `ntk validation instructions` passes with the Rust topology; commit message: `docs(instructions): align repository operating model with Rust workspace — closeout`.
- W3 closes when `pwsh-parity` CI job passes consistently or a documented local-only decision record exists.
- W4 closes when `ntk validation readme-standards` passes and all 50+ subcommands are documented.
- W5 phases each close when their safety matrix and parity ledger are updated and the phase plan/spec is archived.
- W6 closes when Phase 8 first implementation slice passes `cargo build` and `cargo test` in `copilot-instructions`.
- This plan is now archived because the local planning workstream is complete and the only remaining W6 activity is implementation in the external `copilot-instructions` repository.