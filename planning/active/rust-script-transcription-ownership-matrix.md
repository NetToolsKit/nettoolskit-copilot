# Rust Script Transcription Ownership Matrix

Generated: 2026-03-26 17:05

## Status

- LastUpdated: 2026-03-26 17:05
- Objective: freeze the canonical PowerShell inventory, Rust owner boundary, target surface, and migration wave for every tracked script under `scripts/**/*.ps1`.
- Source Plan: `planning/active/plan-repository-unification-and-rust-migration.md`
- Source Spec: `planning/specs/active/spec-repository-unification-and-rust-migration.md`
- Active Branch: `feature/native-validation-policy`
- Remaining Open Backlog: `planning/active/plan-rust-migration-closeout-and-cutover.md`

## Owner Summary

| Rust Owner | Script Count | Notes |
| --- | ---: | --- |
| `crates/core` | 15 | shared helper and deterministic support primitives |
| `crates/commands/runtime + crates/cli` | 42 | operator-facing runtime commands and compatibility entrypoints |
| `crates/commands/runtime + crates/orchestrator` | 4 | runtime hook contracts bound to orchestration lifecycle |
| `crates/commands/runtime` | 8 | maintenance and git hook operator flows |
| `crates/commands/validation` | 41 | validation, security, governance, doc, and deploy-preflight surfaces |
| `crates/orchestrator` | 10 | staged control-plane execution and dispatch |
| `crate test suites + root parity harness` | 27 | parity and migration-proof test replacement surface |
| Total | 147 | full script estate locked for migration |

## Canonical Matrix

| Scope | Count | Rust Owner | Wave | Target Surface |
| --- | ---: | --- | --- | --- |
| `scripts/common/*.ps1` | 15 | `crates/core` | `Wave 1` | core support modules |
| `scripts/runtime/*.ps1` excluding `scripts/runtime/hooks/*.ps1` | 42 | `crates/commands/runtime + crates/cli` | `Wave 1` | runtime command contracts |
| `scripts/runtime/hooks/*.ps1` | 4 | `crates/commands/runtime + crates/orchestrator` | `Wave 3` | runtime hook contracts |
| `scripts/maintenance/*.ps1` | 5 | `crates/commands/runtime` | `Wave 2` | runtime maintenance contracts |
| `scripts/validation/*.ps1` | 31 | `crates/commands/validation` | `Wave 2` | validation command contracts |
| `scripts/security/*.ps1` | 6 | `crates/commands/validation` | `Wave 2` | validation security contracts |
| `scripts/governance/*.ps1` | 2 | `crates/commands/validation` | `Wave 2` | validation governance contracts |
| `scripts/doc/*.ps1` | 1 | `crates/commands/validation` | `Wave 2` | validation documentation contracts |
| `scripts/deploy/*.ps1` | 1 | `crates/commands/validation` | `Wave 2` | validation deploy preflight contracts |
| `scripts/orchestration/**/*.ps1` | 10 | `crates/orchestrator` | `Wave 3` | orchestrator stage and engine contracts |
| `scripts/git-hooks/*.ps1` | 3 | `crates/commands/runtime` | `Wave 3` | runtime git hook install and check contracts |
| `scripts/tests/*.ps1` excluding `scripts/tests/runtime/*.ps1` | 4 | `crate test suites + root parity harness` | `Wave 3` | root parity harness |
| `scripts/tests/runtime/*.ps1` | 23 | `crate test suites + root parity harness` | `Wave 3` | runtime parity harness |

## Locked Inventory

### `scripts/common/*.ps1`

- Rust owner: `crates/core`
- Wave: `Wave 1`
- Included scripts: `agent-runtime-hardening`, `codex-runtime-hygiene`, `common-bootstrap`, `console-style`, `git-hook-eof-settings`, `local-context-index`, `mcp-runtime-catalog`, `provider-surface-catalog`, `repository-paths`, `runtime-execution-context`, `runtime-install-profiles`, `runtime-operation-support`, `runtime-paths`, `validation-logging`, `vscode-runtime-hygiene`

### `scripts/runtime/*.ps1` excluding hooks

- Rust owner: `crates/commands/runtime + crates/cli`
- Wave: `Wave 1`
- Included scripts: `apply-vscode-templates`, `bootstrap`, `clean-codex-runtime`, `clean-vscode-user-runtime`, `doctor`, `evaluate-agent-pipeline`, `export-planning-summary`, `healthcheck`, `install`, `invoke-super-agent-brainstorm`, `invoke-super-agent-execute`, `invoke-super-agent-housekeeping`, `invoke-super-agent-parallel-dispatch`, `invoke-super-agent-plan`, `new-super-agent-worktree`, `query-local-context-index`, `render-claude-runtime-surfaces`, `render-codex-compatibility-surfaces`, `render-codex-orchestration-surfaces`, `render-github-instruction-surfaces`, `render-mcp-runtime-artifacts`, `render-provider-skill-surfaces`, `render-provider-surfaces`, `render-vscode-mcp-template`, `render-vscode-profile-surfaces`, `render-vscode-workspace-surfaces`, `replay-agent-run`, `resume-agent-pipeline`, `run-agent-pipeline`, `self-heal`, `set-codex-runtime-preferences`, `setup-vscode-profiles`, `sync-claude-settings`, `sync-claude-skills`, `sync-codex-mcp-config`, `sync-vscode-global-mcp`, `sync-vscode-global-settings`, `sync-vscode-global-snippets`, `sync-workspace-settings`, `update-copilot-chat-titles`, `update-local-context-index`, `validate-vscode-global-alignment`

### `scripts/runtime/hooks/*.ps1`

- Rust owner: `crates/commands/runtime + crates/orchestrator`
- Wave: `Wave 3`
- Included scripts: `common`, `pre-tool-use`, `session-start`, `subagent-start`

### `scripts/maintenance/*.ps1`

- Rust owner: `crates/commands/runtime`
- Wave: `Wave 2`
- Included scripts: `clean-build-artifacts`, `fix-region-spacing`, `fix-version-ranges`, `generate-http-from-openapi`, `trim-trailing-blank-lines`

### `scripts/validation/*.ps1`

- Rust owner: `crates/commands/validation`
- Wave: `Wave 2`
- Included scripts: `export-audit-report`, `export-enterprise-trends`, `test-routing-selection`, `validate-agent-hooks`, `validate-agent-orchestration`, `validate-agent-permissions`, `validate-agent-skill-alignment`, `validate-all`, `validate-architecture-boundaries`, `validate-audit-ledger`, `validate-authoritative-source-policy`, `validate-compatibility-lifecycle-policy`, `validate-dotnet-standards`, `validate-instruction-architecture`, `validate-instruction-metadata`, `validate-instructions`, `validate-planning-structure`, `validate-policy`, `validate-powershell-standards`, `validate-readme-standards`, `validate-release-governance`, `validate-release-provenance`, `validate-routing-coverage`, `validate-runtime-script-tests`, `validate-security-baseline`, `validate-shared-script-checksums`, `validate-shell-hooks`, `validate-supply-chain`, `validate-template-standards`, `validate-warning-baseline`, `validate-workspace-efficiency`

### `scripts/security/*.ps1`

- Rust owner: `crates/commands/validation`
- Wave: `Wave 2`
- Included scripts: `Install-SecurityAuditPrerequisites`, `Invoke-CiPreBuildSecuritySnapshot`, `Invoke-FrontendPackageVulnerabilityAudit`, `Invoke-PreBuildSecurityGate`, `Invoke-RustPackageVulnerabilityAudit`, `Invoke-VulnerabilityAudit`

### `scripts/governance/*.ps1`

- Rust owner: `crates/commands/validation`
- Wave: `Wave 2`
- Included scripts: `set-branch-protection`, `update-shared-script-checksums-manifest`

### `scripts/doc/*.ps1`

- Rust owner: `crates/commands/validation`
- Wave: `Wave 2`
- Included scripts: `validate-xml-documentation`

### `scripts/deploy/*.ps1`

- Rust owner: `crates/commands/validation`
- Wave: `Wave 2`
- Included scripts: `deploy-backend-to-vps`

### `scripts/orchestration/**/*.ps1`

- Rust owner: `crates/orchestrator`
- Wave: `Wave 3`
- Included scripts: `engine/invoke-codex-dispatch`, `engine/invoke-task-worker`, `stages/closeout-stage`, `stages/implement-stage`, `stages/intake-stage`, `stages/plan-stage`, `stages/review-stage`, `stages/route-stage`, `stages/spec-stage`, `stages/validate-stage`

### `scripts/git-hooks/*.ps1`

- Rust owner: `crates/commands/runtime`
- Wave: `Wave 3`
- Included scripts: `invoke-pre-commit-eof-hygiene`, `setup-git-hooks`, `setup-global-git-aliases`

### `scripts/tests/*.ps1` excluding runtime subfolder

- Rust owner: `crate test suites + root parity harness`
- Wave: `Wave 3`
- Included scripts: `apply-aaa-pattern`, `check-test-naming`, `refactor_tests_to_aaa`, `run-coverage`

### `scripts/tests/runtime/*.ps1`

- Rust owner: `crate test suites + root parity harness`
- Wave: `Wave 3`
- Included scripts: `agent-orchestration-engine.tests`, `authoritative-source-policy.tests`, `ci-security-snapshot.tests`, `compatibility-lifecycle.tests`, `copilot-chat-title-normalization.tests`, `execution-session-logging.tests`, `git-global-aliases.tests`, `git-hook-eof-hygiene.tests`, `install-runtime.tests`, `instruction-architecture.tests`, `mcp-config-sync.tests`, `planning-structure.tests`, `runtime-location-paths.tests`, `runtime-scripts.tests`, `super-agent-entrypoints.tests`, `super-agent-worktree.tests`, `template-standards.tests`, `trim-trailing-blank-lines.tests`, `vscode-agent-hooks.tests`, `vscode-global-settings-sync.tests`, `vscode-global-snippets-sync.tests`, `workspace-efficiency.tests`, `workspace-settings-sync.tests`

## Lock Rules

1. Every tracked `scripts/**/*.ps1` file must remain covered by one row in the canonical matrix.
2. No migration task may introduce a new PowerShell script without assigning a Rust owner, target surface, and wave in this artifact.
3. Reassigning a script to a different owner boundary requires updating this matrix in the same change.
4. Wrapper removal is not allowed until the corresponding script row has recorded Rust parity evidence in the owning workstream.