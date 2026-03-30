# Repository Consolidation and Continuity Plan

Generated: 2026-03-29

## Status

- LastUpdated: 2026-03-30 07:31
- Objective: keep the repository consolidation baseline and point the remaining open gaps into focused category plans so token economy, SQLite memory, build-target hygiene, instruction governance, and the remaining script tail can each move independently.
- Normalized Request: create a detailed and complete plan for all gaps and pending workstreams identified in the repository consolidation analysis conducted on 2026-03-29, then split the remaining open work into smaller category-specific planning tracks.
- Active Branch: `main` (planning only; follow-on implementation branches TBD)
- Spec Path: `planning/specs/active/spec-repository-consolidation-continuity.md`
- Dependency: `planning/completed/plan-script-retirement-phase-18.md`, `planning/completed/plan-script-retirement-phase-19.md`, `planning/completed/plan-script-retirement-phase-20c-self-heal.md`, `planning/completed/plan-script-retirement-phase-20d-provider-surface-dispatcher.md`, `planning/completed/plan-script-retirement-phase-20e-catalog-native-renderer-dispatch.md`, and `planning/completed/plan-script-retirement-phase-20f-codex-orchestration-renderer.md` are now complete; Workstream W5 now continues from the closed 96-script baseline.
- Inputs:
  - `planning/completed/plan-script-retirement-phase-17.md`
  - `planning/completed/plan-script-retirement-phase-18.md`
  - `planning/completed/plan-script-retirement-phase-19.md`
  - `planning/completed/plan-script-retirement-phase-20c-self-heal.md`
  - `planning/specs/active/spec-repository-consolidation-continuity.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `.github/instructions/repository-operating-model.instructions.md`
  - `definitions/shared/instructions/repository-operating-model.instructions.md`
  - `.github/workflows/ci.yml`
  - `crates/cli/src/main.rs`
  - `crates/cli/src/runtime_commands.rs`
  - `crates/cli/src/validation_commands.rs`
  - `crates/cli/README.md`
  - `README.md`
  - `scripts/tests/runtime/*.ps1` (23 files)
  - `C:\Users\tguis\copilot-instructions\planning\active\plan-rust-runtime-engine-foundation-phase-8.md`

---

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
`repository-operating-model.instructions.md` describes a .NET/Clean Architecture monorepo with
`src/`, `modules/`, `samples/src/Rent.Service.*`, `dotnet build`, and `dotnet test`. The actual
workspace is a Rust multi-crate layout. Every AI agent consuming this file receives wrong build
commands, wrong topology, and wrong test filters.

#### Task W2.1: Verify Authoritative Source Location

Status: `[x]` Completed

- Confirm whether `definitions/shared/instructions/repository-operating-model.instructions.md` exists.
- If it exists, the authoritative source is `definitions/shared/instructions/` and the `.github/instructions/` copy is a projection.
- If it does not exist, the authoritative source is `.github/instructions/` directly.
- Commands:
  - `Test-Path definitions\shared\instructions\repository-operating-model.instructions.md`
- Target paths confirmed post-check:
  - `definitions/shared/instructions/repository-operating-model.instructions.md` (if it exists — authoritative source)
  - `.github/instructions/repository-operating-model.instructions.md` (projection or direct if no definitions source)
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
    - `instructions/rust-code-organization.instructions.md`
    - `instructions/rust-testing.instructions.md`
    - `instructions/clean-architecture-code.instructions.md`
    - `instructions/backend.instructions.md`
    - `instructions/api-high-performance-security.instructions.md`
  - Infrastructure:
    - `instructions/docker.instructions.md`
    - `instructions/ci-cd-devops.instructions.md`
    - `instructions/workflow-generation.instructions.md`
    - `instructions/observability-sre.instructions.md`
    - `instructions/platform-reliability-resilience.instructions.md`
    - `instructions/powershell-script-creation.instructions.md`
    - `instructions/powershell-execution.instructions.md`
  - Security:
    - `instructions/security-vulnerabilities.instructions.md`
    - `instructions/api-high-performance-security.instructions.md`
    - `instructions/data-privacy-compliance.instructions.md`
  - Testing:
    - `instructions/rust-testing.instructions.md`
    - `instructions/tdd-verification.instructions.md`
    - `instructions/e2e-testing.instructions.md`
    - `instructions/static-analysis-sonarqube.instructions.md`
  - Documentation and process: keep as-is (shared lifecycle files apply generically).
  - Remove: `dotnet-csharp`, `orm`, `database`, `database-configuration-operations`, `vue-quasar`, `vue-quasar-architecture`, `frontend`, `ui-ux`, `microservices-performance`.
- Commit checkpoint:
  - `docs(instructions): align domain instruction map with Rust workspace scope`

#### Task W2.6: Re-render Projection and Validate

Status: `[x]` Completed

- If `definitions/shared/instructions/` was the authoritative source, trigger a re-render so `.github/instructions/` reflects the new content:
  - `ntk runtime bootstrap --repo-root . --mirror` or the equivalent `scripts/runtime/render-github-instruction-surfaces.ps1` invocation
- Validate:
  - `& .\.build\target\debug\ntk.exe validation instructions --repo-root . --warning-only false`
  - `& .\.build\target\debug\ntk.exe validation planning-structure --repo-root . --warning-only false`
  - `git diff --check`
- Commit checkpoint (after re-render):
  - `docs(instructions): re-render github instruction surfaces with updated operating model`

**Checkpoint: W2 Operating Model Alignment Complete**
- authoritative source confirmed at `definitions/shared/instructions/repository-operating-model.instructions.md`
- projected GitHub copy re-rendered into `.github/instructions/repository-operating-model.instructions.md`
- repository topology now describes the Rust multi-crate workspace instead of the old .NET monorepo layout
- build/test/run guidance now points to `cargo build`, `cargo test`, `cargo clippy`, `cargo fmt`, native `ntk validation`, runtime continuity commands, and the Rust vulnerability audit
- style, testing, workflow patterns, and domain instruction map now align to the actual Rust workspace and retained PowerShell compatibility model
- validation evidence:
  - `cargo run -p nettoolskit-cli -- validation instructions --repo-root . --warning-only false` ✅
  - `cargo run -p nettoolskit-cli -- validation planning-structure --repo-root . --warning-only false` ✅
  - `git diff --check` ✅

---

### Workstream W3 — CI PowerShell Parity Test Coverage

Status: `[ ]` Pending

**Problem being fixed:**
23 `scripts/tests/runtime/*.ps1` files are marked as `compatibility wrapper retained intentionally`
and described as the canonical parity harness in the safety matrix. `ci.yml` has zero PowerShell
test invocations; these tests only run on developer machines.

#### Task W3.1: Audit Parity Test Scope and Pester Compatibility

Status: `[ ]` Pending

- Review each of the 23 parity test files to determine:
  - Whether the test uses Pester syntax (`Describe`, `It`, `Should`) or is a standalone script.
  - Whether any test requires local file system access that would not be available in CI.
  - Whether any test depends on `ntk` being built and in PATH.
- Confirm Pester version required (check existing invocation patterns in the scripts).
- Target paths: `scripts/tests/runtime/*.ps1`
- Commands:
  - `rg "Describe|Context|It |Should " scripts/tests/runtime/ --count`
  - `rg "Invoke-Pester|Install-Module.*Pester" scripts/tests/runtime/`
- Checkpoint: parity test inventory complete; Pester dependency confirmed.

#### Task W3.2: Add `pwsh-parity` Job to `ci.yml`

Status: `[ ]` Pending

- Target: `.github/workflows/ci.yml`
- Add a new job `pwsh-parity` using `windows-latest` runner.
- Job structure:
  - Checkout
  - Set up Rust toolchain (stable) — needed if any test invokes the `ntk` binary
  - Cache Rust build artifacts via `Swatinem/rust-cache@v2`
  - Build the workspace: `cargo build --workspace`
  - Install Pester: `Install-Module -Name Pester -Force -Scope CurrentUser -SkipPublisherCheck`
  - Run parity tests: `Invoke-Pester -Path scripts/tests/runtime/ -EnableExit -Output Detailed`
  - Upload test output artifact on failure (`actions/upload-artifact@v4`)
- The job runs on `push` and `pull_request` targeting `main` and `develop` (same triggers as existing jobs).
- Use `continue-on-error: false` after a first stable run; accept `continue-on-error: true` for the initial merge to confirm baseline stability.
- Commands to validate locally before merging:
  - `Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned`
  - `Install-Module -Name Pester -Force -Scope CurrentUser -SkipPublisherCheck`
  - `Invoke-Pester -Path scripts/tests/runtime/ -EnableExit -Output Detailed`
- Commit checkpoint:
  - `ci(workflows): add pwsh-parity job to CI for PowerShell runtime parity tests`

#### Task W3.3: Address Test Failures and Stabilize

Status: `[ ]` Pending

- If Task W3.2 reveals test failures, fix root causes in the parity test files or in the underlying scripts.
- For tests that cannot run in CI due to environment-specific resources (absolute paths, registry access, VS Code CLI presence), either:
  - Parameterize the test to accept a skip condition when the resource is absent (`-Skip` in Pester), or
  - Move the test into a `scripts/tests/runtime/local/` folder and exclude it from the CI invocation pattern.
- Commit checkpoint (if fixes needed):
  - `test(runtime): stabilize PowerShell parity tests for CI execution`

#### Task W3.4: Document Parity Test Coverage Model (Optional Fallback to Option B)

Status: `[ ]` Pending (only if Option B was selected instead of W3.2)

- If adding CI coverage was rejected, create `scripts/tests/runtime/README.md` with:
  - Explicit statement that these tests are local-only.
  - Rationale for local-only coverage.
  - How to run them: `Invoke-Pester -Path scripts/tests/runtime/ -EnableExit -Output Detailed`.
  - Known limitations: environment-specific resources, VS Code presence required, etc.
- Commit checkpoint:
  - `docs(tests): document parity test local-only coverage model`

---

### Workstream W4 — CLI Surface Documentation

Status: `[ ]` Pending

**Problem being fixed:**
The `ntk` binary exposes 50+ subcommands across 5 groups (`manifest`, `runtime`, `validation`,
`service`, `completions`) plus global args. None of these are documented beyond 3 bullet points
in `crates/cli/README.md` and a features list in the root `README.md`.

#### Task W4.1: Compile Full Command Reference

Status: `[ ]` Pending

- Extract all command groups and subcommands from:
  - `crates/cli/src/main.rs` — top-level `Commands` enum
  - `crates/cli/src/runtime_commands.rs` — `RuntimeCommand` enum (12 variants)
  - `crates/cli/src/validation_commands.rs` — `ValidationCommand` enum (29 variants)
- Build the reference table with: `ntk <group> <subcommand>` form, one-line description.
- Commands:
  - `rg "/// " crates/cli/src/main.rs crates/cli/src/runtime_commands.rs crates/cli/src/validation_commands.rs`
- Checkpoint: command table in working notes, ready to insert into docs.

#### Task W4.2: Update Root `README.md` — Command Reference Section

Status: `[ ]` Pending

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

- Commands to validate after edit:
  - `& .\.build\target\debug\ntk.exe validation readme-standards --repo-root .`
- Commit checkpoint:
  - `docs(readme): add full command reference and global flags section`

#### Task W4.3: Update `crates/cli/README.md` — Expand Features and Add Subcommand Table

Status: `[ ]` Pending

- Target: `crates/cli/README.md`
- Replace the current 3-feature bullet list under `## Features` with a grouped table by command surface.
- Add a `## Quick Start` section showing the most common invocations:
  - `ntk manifest list`
  - `ntk validation all --repo-root .`
  - `ntk runtime healthcheck --repo-root .`
  - `ntk service --port 8080`
  - `ntk completions powershell | Out-String | Invoke-Expression`
- Add a `## Service Mode` section covering:
  - `POST /task/submit` — authenticated task submission with `NTK_SERVICE_AUTH_TOKEN`
  - `GET /health` — runtime health JSON
  - `GET /ready` — readiness check
  - ChatOps ingress: Telegram webhook (`/ntk/telegram`) and Discord interactions (`/ntk/discord`)
  - Environment variables: `NTK_SERVICE_AUTH_TOKEN`, `NTK_CHATOPS_TELEGRAM_WEBHOOK_SECRET_TOKEN_ENV`, `NTK_CHATOPS_DISCORD_INTERACTIONS_PUBLIC_KEY_ENV`
- Commit checkpoint:
  - `docs(cli): expand README with full subcommand groups, service mode, and quick start`

#### Task W4.4: Validate All README Changes

Status: `[ ]` Pending

- Run: `& .\.build\target\debug\ntk.exe validation readme-standards --repo-root . --warning-only false`
- Run: `& .\.build\target\debug\ntk.exe validation planning-structure --repo-root . --warning-only false`
- Run: `git diff --check`
- Checkpoint: all validation checks pass; no trailing whitespace or merge conflicts.

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

Status: `[ ]` Pending

- This is the largest single domain: 30 scripts after Phases 17, 18, the tactical Phase 20c self-heal slice, the tactical Phase 20d provider-surface dispatcher slice, the tactical Phase 20e catalog-native renderer slice, and the tactical Phase 20f Codex orchestration slice remove `doctor.ps1`, `healthcheck.ps1`, `sync-codex-mcp-config.ps1`, `render-vscode-mcp-template.ps1`, `self-heal.ps1`, `render-provider-surfaces.ps1`, `render-codex-compatibility-surfaces.ps1`, and `render-codex-orchestration-surfaces.ps1`.
- Create `planning/specs/active/spec-script-retirement-phase-20.md` with:
    - Problem: 30 runtime scripts have confirmed Rust owner in `crates/commands/runtime + crates/cli` but no zero-consumer proof.
  - Group the remaining scripts into sub-slices by functional surface to enable incremental deletion:
    - Sub-slice A: sync/render scripts (`render-*.ps1`, `sync-*.ps1`, `setup-*.ps1`) — likely consumed only by CI/docs
    - Sub-slice B: invoke/pipeline scripts (`invoke-*.ps1`, `run-*.ps1`, `replay-*.ps1`, `resume-*.ps1`, `evaluate-*.ps1`) — likely consumed by orchestration and CI
    - Sub-slice C: install/bootstrap/clean scripts (`bootstrap.ps1`, `install.ps1`, `clean-*.ps1`, `set-*.ps1`, plus the broader runtime leaves not already removed by Phases 17, 18, and the tactical 20c self-heal slice)
    - Decision: each sub-slice gets its own Phase 20a/20b/20c consumer sweep so the safety matrix stays accurate.
- This task creates `planning/specs/active/spec-script-retirement-phase-20.md` and `planning/active/plan-script-retirement-phase-20.md`.
- Commit checkpoint:
  - `docs(planning): register Phase 20 plan for scripts/runtime consumer sweep`

#### Task W5.4: Execute Phase 20 Consumer Sweep (Sub-slices A, B, C)

Status: `[ ]` Pending (blocked on Phase 20 plan creation)

- Run consumer sweeps for each sub-slice.
- For each zero-consumer script: delete it.
- For each retained: document the retaining consumer.
- After each sub-slice:
  - Update safety matrix and parity ledger.
  - Run the Phase 20 validation checklist.
- Archive Phase 20 plan/spec to completed only when all sub-slices are done.
- Validation checklist:
  - `cargo test -p nettoolskit-runtime --quiet`
  - `cargo test -p nettoolskit-cli --test test_suite runtime_commands_tests --quiet`
  - `& .\.build\target\debug\ntk.exe validation all --repo-root . --warning-only false`
  - `git diff --check`
- Commit checkpoint (one per sub-slice):
  - `chore(scripts): Phase 20a — retire confirmed-zero-consumer scripts/runtime/render-* scripts`
  - `chore(scripts): Phase 20b — retire confirmed-zero-consumer scripts/runtime/invoke-* and pipeline scripts`
  - `chore(scripts): Phase 20c — retire confirmed-zero-consumer scripts/runtime/bootstrap and clean scripts`

#### Task W5.5: Create and Execute Phase 21 — `scripts/security/*.ps1` + `scripts/governance/*.ps1` (8)

Status: `[ ]` Pending (blocked on Phase 20 complete)

- Create `planning/specs/active/spec-script-retirement-phase-21.md` and `planning/active/plan-script-retirement-phase-21.md`.
- Special constraint for `scripts/security/*.ps1` (6): `.github/governance/shared-script-checksums.manifest.json` explicitly tracks this domain. The Phase 21 plan must include a task to update the checksums manifest before deleting any security scripts.
- Consumer sweep commands:
  - `Get-ChildItem scripts\security, scripts\governance -Filter "*.ps1" | ForEach-Object { rg $_.Name .github\governance, scripts --type ps1 --count }`
- After sweep:
  - Update `.github/governance/shared-script-checksums.manifest.json` to remove deleted security script entries.
  - Re-validate with `ntk validation supply-chain --repo-root . --warning-only false`.
- Validation checklist:
  - `cargo test -p nettoolskit-validation --quiet`
  - `& .\.build\target\debug\ntk.exe validation all --repo-root . --warning-only false`
  - `git diff --check`
- Commit checkpoint:
  - `chore(scripts): Phase 21 — retire confirmed-zero-consumer scripts/security and scripts/governance scripts`

#### Task W5.6: Create and Execute Phase 22 — `scripts/orchestration/**/*.ps1` (10)

Status: `[ ]` Pending (blocked on Phase 21 complete)

- Create `planning/specs/active/spec-script-retirement-phase-22.md` and `planning/active/plan-script-retirement-phase-22.md`.
- The 10 orchestration scripts are staged-execution wrappers (`intake-stage.ps1`, `plan-stage.ps1`, `spec-stage.ps1`, `implement-stage.ps1`, `review-stage.ps1`, `validate-stage.ps1`, `closeout-stage.ps1`, `route-stage.ps1`, `invoke-codex-dispatch.ps1`, `invoke-task-worker.ps1`).
- Consumer sweep commands:
  - `Get-ChildItem scripts\orchestration -Filter "*.ps1" -Recurse | ForEach-Object { rg $_.Name scripts, definitions, .github --count }`
- The orchestration scripts are likely consumed by each other — the sweep must map the full dependency graph before deleting any leaf.
- After sweep:
  - Update the `crates/orchestrator` documentation to reflect that orchestration is now fully native.
  - Update skill surfaces and authored runbooks that reference the stage scripts.
- Validation checklist:
  - `cargo test -p nettoolskit-orchestrator --quiet`
  - `cargo test -p nettoolskit-cli --test test_suite --quiet`
  - `& .\.build\target\debug\ntk.exe validation all --repo-root . --warning-only false`
  - `pwsh -NoProfile -File scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
  - `git diff --check`
- Commit checkpoint:
  - `chore(scripts): Phase 22 — retire confirmed-zero-consumer scripts/orchestration stage scripts`

#### Task W5.7: Post-Phase-22 Retention Audit

Status: `[ ]` Pending (blocked on Phase 22 complete)

- After Phases 19–22, the live estate should be at or near the `retain wrapper intentionally` floor of 33 scripts.
- Print the remaining live estate: `(Get-ChildItem scripts -Filter "*.ps1" -Recurse).Count`
- Compare to the expected floor.
- For each script still present:
  - Confirm it has an explicit entry in the `retain wrapper intentionally` table of `script-retirement-safety-matrix.md`.
  - If any script is not in that table, it is either a missed deletion candidate or a new addition that escaped the safety matrix — investigate and resolve.
- Update `script-retirement-safety-matrix.md` and `rust-script-parity-ledger.md` to close this workstream.
- Commit checkpoint:
  - `docs(planning): Phase 22 retention audit — confirm floor and close consumer migration workstream`

---

### Workstream W6 — `copilot-instructions` Phase 8 Rust Directives

Status: `[ ]` Pending

**Problem being fixed:**
`copilot-instructions` Phase 8 spec is planning-ready but awaiting user-provided Rust directives.
No `Cargo.toml`, no crates, no Rust files have been created. This migration would bring the
instruction runtime into the same Rust-native model that `nettoolskit-copilot` already operates with.

This workstream executes inside the `copilot-instructions` repository. All file paths below are relative to that repo root.

#### Task W6.1: Provide Phase 8 Rust Directives

Status: `[ ]` Pending

- Update `planning/active/plan-rust-runtime-engine-foundation-phase-8.md` with explicit implementation directives.
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
- Commit checkpoint:
  - `docs(planning): provide Phase 8 Rust directives for ntk-runtime-engine`

#### Task W6.2: Scaffold Cargo Workspace

Status: `[ ]` Pending (blocked on W6.1)

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

Status: `[ ]` Pending (blocked on W6.2)

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

Status: `[ ]` Pending (blocked on W6.3)

- In `src/runtime/src/renderer.rs`, define the renderer dispatch parity surface:
  - `pub enum RendererTarget { GithubInstructions, CodexCompatibility, ClaudeRuntime, VscodeProfiles, ProviderSurfaces, McpArtifacts }`
  - `pub const RENDERER_SCRIPTS: &[(&str, RendererTarget)]` mapping each `scripts/runtime/render-*.ps1` to its target
  - Contract-first: no logic, just enumeration and tests.
- Add `src/runtime/tests/renderer_parity_tests.rs`.
- Commit checkpoint:
  - `feat(runtime-engine): renderer dispatch parity contract with target enumeration`

#### Task W6.5: Verify Phase 8 Validates and Update Evidence

Status: `[ ]` Pending (blocked on W6.4)

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
| R1 | Editing `definitions/shared/instructions/` triggers a cascade re-render that breaks unrelated surfaces | Run `ntk validation all` in warning-only mode first; verify diff before committing |
| R2 | `pwsh-parity` CI job fails on first run due to environment differences | Use `continue-on-error: true` for the initial merge; switch to `false` after baseline is stable |
| R3 | Phase 19 common-script consumer sweep finds 12+ blockers | Accept a partial Phase 19 that deletes only zero-consumer scripts; document retained ones explicitly |
| R4 | Phase 20 is too large for one PR review | Split into 20a/20b/20c sub-phases; each sub-phase has its own commit checkpoint |
| R5 | Phase 21 security script deletion requires updating `shared-script-checksums.manifest.json` in a separate PR | Create the manifest update as a prerequisite commit before the deletion PR |
| R6 | `copilot-instructions` Phase 8 scaffold breaks the existing PowerShell validation in that repo | Add `cargo build` after workspace creation; if `validate-planning-structure.ps1` fails, debug before proceeding |
| R7 | `definitions/shared/instructions/repository-operating-model.instructions.md` does not exist (no `definitions/` source) | Apply the fix directly to `.github/instructions/` and skip W2.6 re-render step |

---

## Closeout Expectations

- Each workstream closes with its own commit and its own stable intermediate checkpoint.
- W2 closes when `ntk validation instructions` passes with the Rust topology; commit message: `docs(instructions): align repository operating model with Rust workspace — closeout`.
- W3 closes when `pwsh-parity` CI job passes consistently or a documented local-only decision record exists.
- W4 closes when `ntk validation readme-standards` passes and all 50+ subcommands are documented.
- W5 phases each close when their safety matrix and parity ledger are updated and the phase plan/spec is archived.
- W6 closes when Phase 8 first implementation slice passes `cargo build` and `cargo test` in `copilot-instructions`.
- This plan itself moves to `planning/completed/` only when W2, W3, W4, and W6 are all closed. W5 phases have their own plans and close independently.