# Phase 18: MCP Runtime Wrapper Retirement

Generated: 2026-03-29 09:05

## Status

- LastUpdated: 2026-03-29 09:23
- Objective: retire the local `sync-codex-mcp-config.ps1` and `render-vscode-mcp-template.ps1` wrappers after the native `ntk runtime` MCP command surfaces are exposed, the live bootstrap/docs/tests are repointed, and the tracked `render-mcp-runtime-artifacts.ps1` renderer is explicitly retained until the provider-surface catalog no longer requires a path-backed renderer.
- Normalized Request: continue the aggressive PowerShell-to-Rust retirement flow, keep planning updated, and keep committing each stable phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-18.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-17.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/runtime/sync-codex-mcp-config.ps1`
  - `scripts/runtime/render-vscode-mcp-template.ps1`
  - `scripts/runtime/render-mcp-runtime-artifacts.ps1`
  - `crates/commands/runtime/src/sync/mcp_config.rs`
  - `crates/commands/runtime/src/sync/mcp_runtime_artifacts.rs`
  - `crates/cli/src/runtime_commands.rs`

## Scope Summary

1. `scripts/runtime/sync-codex-mcp-config.ps1`
2. `scripts/runtime/render-vscode-mcp-template.ps1`
3. native `ntk runtime sync-codex-mcp-config`
4. native `ntk runtime render-vscode-mcp-template`
5. native `ntk runtime render-mcp-runtime-artifacts` as the canonical tracked-artifact renderer while the path-backed provider-surface catalog still pins `render-mcp-runtime-artifacts.ps1`

This phase is complete only if:

- the native MCP command surfaces exist in `ntk runtime`
- bootstrap, Codex/Claude docs, template comments, and parity tests stop requiring the two retired wrapper paths
- `render-mcp-runtime-artifacts.ps1` is not deleted prematurely and is documented as the temporary retained renderer until the catalog model changes
- the two safe wrappers are deleted in the same slice

## Ordered Tasks

### Task 1: Freeze The Native MCP Command Boundary

Status: `[x]` Completed

- Expose `render-vscode-mcp-template`, `render-mcp-runtime-artifacts`, and `sync-codex-mcp-config` through `ntk runtime`.
- Add runtime crate and CLI tests that prove both rendering and Codex config application through the native boundary.

### Task 2: Repoint Live Consumers To The Native Boundary

Status: `[x]` Completed

- Repoint `scripts/runtime/bootstrap.ps1` to `ntk runtime sync-codex-mcp-config`.
- Repoint authored docs, projected Codex compatibility wrappers, MCP template comments, and runtime parity tests to the native boundary.
- Keep `render-mcp-runtime-artifacts.ps1` explicitly retained because `.github/governance/provider-surface-projection.catalog.json` still requires a path-backed renderer through `scripts/common/provider-surface-catalog.ps1`.

### Task 3: Delete The Two Safe Wrappers

Status: `[x]` Completed

- Delete `scripts/runtime/sync-codex-mcp-config.ps1`.
- Delete `scripts/runtime/render-vscode-mcp-template.ps1`.

### Task 4: Rebaseline Phase Evidence And Archive

Status: `[x]` Completed

- Update `planning/completed/script-retirement-safety-matrix.md`.
- Update `planning/completed/rust-script-parity-ledger.md`.
- Update the continuity plan/spec to reflect the new 100-script baseline and Phase 18 completion.
- Move the phase plan/spec to completed only after validations prove the phase result.

## Validation Checklist

- [x] `cargo test -p nettoolskit-runtime --quiet`
- [x] `cargo test -p nettoolskit-cli --test test_suite runtime_commands_tests --quiet`
- [x] `pwsh -NoProfile -File .\scripts\tests\runtime\mcp-config-sync.tests.ps1`
- [x] `pwsh -NoProfile -File .\scripts\tests\runtime\runtime-scripts.tests.ps1`
- [x] `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- [x] `git diff --check`

## Risks And Fallbacks

- `render-mcp-runtime-artifacts.ps1` cannot be deleted in this slice because `.github/governance/provider-surface-projection.catalog.json` still points to a script path and `scripts/common/provider-surface-catalog.ps1` executes that path directly.
- The parity tests must prove the native command surfaces without silently shrinking MCP coverage down to `apply-vscode-templates` only.
- Bootstrap remains a PowerShell compatibility wrapper and must stay copy-pasteable for operators after it starts dispatching the native MCP sync command.

## Closeout Expectations

- Commit implementation and planning closeout separately.
- Archive the phase only after the two wrappers are deleted, the tracked MCP renderer remains intentionally retained, and the validation evidence proves the native path is canonical.

## Executed Result

- `ntk runtime` now exposes the native MCP command surfaces for VS Code template rendering, tracked-artifact rendering, and Codex config application.
- `scripts/runtime/bootstrap.ps1`, the projected Codex compatibility wrappers, MCP docs, and runtime parity tests no longer require `scripts/runtime/sync-codex-mcp-config.ps1` or `scripts/runtime/render-vscode-mcp-template.ps1`.
- The two local wrappers were deleted after the native command tests, runtime parity scripts, and vulnerability audit all passed in the same slice.
- `scripts/runtime/render-mcp-runtime-artifacts.ps1` remains intentionally retained as a temporary compatibility leaf until the provider-surface projection catalog no longer requires a path-backed renderer.
- The live local `scripts/**/*.ps1` estate dropped from `102` to `100`.