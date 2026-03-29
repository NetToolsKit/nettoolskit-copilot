# Spec: Phase 18 MCP Runtime Wrapper Retirement

Generated: 2026-03-29 09:05

## Status

- LastUpdated: 2026-03-29 09:23
- Objective: define the cutover intent and safe deletion conditions for replacing the local MCP runtime wrappers with native `ntk runtime` command surfaces without deleting the path-backed tracked-artifact renderer prematurely.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-18.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/runtime/sync-codex-mcp-config.ps1`
  - `scripts/runtime/render-vscode-mcp-template.ps1`
  - `scripts/runtime/render-mcp-runtime-artifacts.ps1`
  - `.github/governance/provider-surface-projection.catalog.json`
  - `scripts/common/provider-surface-catalog.ps1`
  - `crates/commands/runtime/src/sync/mcp_config.rs`
  - `crates/commands/runtime/src/sync/mcp_runtime_artifacts.rs`
  - `crates/cli/src/runtime_commands.rs`

## Problem Statement

The runtime crate already owns MCP catalog rendering and Codex config application, but the local repository still carries `scripts/runtime/sync-codex-mcp-config.ps1` and `scripts/runtime/render-vscode-mcp-template.ps1`. Those wrappers are still referenced by bootstrap, MCP docs, projected Codex compatibility wrappers, and runtime parity tests. At the same time, `scripts/runtime/render-mcp-runtime-artifacts.ps1` cannot be removed yet because the provider-surface projection catalog still pins a concrete `scriptPath` that is executed by the shared PowerShell catalog helper.

## Desired Outcome

- `ntk runtime sync-codex-mcp-config` becomes the canonical Codex MCP config application path.
- `ntk runtime render-vscode-mcp-template` becomes the canonical VS Code MCP projection path.
- `ntk runtime render-mcp-runtime-artifacts` becomes the canonical tracked-artifact renderer while the local `render-mcp-runtime-artifacts.ps1` compatibility leaf remains intentionally retained for the current catalog model.
- The local wrappers `sync-codex-mcp-config.ps1` and `render-vscode-mcp-template.ps1` are deleted in the same slice.

## Design Decision

Retire only the two safe local wrappers and keep the tracked-artifact renderer for one more phase. The native `ntk runtime` command group will own all three MCP behaviors. Bootstrap, projected Codex helper wrappers, runtime parity tests, and MCP docs are repointed to the native commands. The remaining `render-mcp-runtime-artifacts.ps1` leaf is treated as an explicit temporary compatibility wrapper until the provider-surface projection catalog no longer requires a path-backed renderer.

## Alternatives Considered

1. Retire all three MCP wrappers in one phase
   - Rejected because `.github/governance/provider-surface-projection.catalog.json` still pins `scripts/runtime/render-mcp-runtime-artifacts.ps1`, and `scripts/common/provider-surface-catalog.ps1` executes that path directly.
2. Keep all three wrappers as intentional compatibility surfaces
   - Rejected because `sync-codex-mcp-config` and `render-vscode-mcp-template` no longer own shell-only behavior; their implementation already lives natively in Rust.
3. Defer the whole MCP cutover until the provider-surface catalog is redesigned
   - Rejected because two of the three wrappers can be safely retired now without waiting for the catalog refactor.

## Risks

- Bootstrap still runs as a PowerShell compatibility surface, so the native MCP sync command must remain operator-usable from shell invocation.
- The runtime parity tests must keep proving MCP rendering/application semantics after the wrapper paths disappear.
- If docs/projected wrappers are updated without the native command implementation landing in the same slice, the user-visible instructions will break.

## Acceptance Criteria

- `ntk runtime render-vscode-mcp-template`, `ntk runtime render-mcp-runtime-artifacts`, and `ntk runtime sync-codex-mcp-config` exist and have deterministic test coverage.
- `scripts/runtime/bootstrap.ps1`, `.codex`/`definitions` MCP docs, projected Codex helper wrappers, and runtime parity tests no longer require `scripts/runtime/sync-codex-mcp-config.ps1` or `scripts/runtime/render-vscode-mcp-template.ps1`.
- `scripts/runtime/render-mcp-runtime-artifacts.ps1` remains present and is documented as a temporary retained renderer, not an accidental survivor.
- The safety matrix records the inventory reduction from `102` to `100`.

## Executed Result

- `crates/commands/runtime/src/sync/mcp_config.rs`, `mcp_runtime_artifacts.rs`, and the CLI runtime command surface now provide the native MCP execution boundary.
- `scripts/runtime/bootstrap.ps1` dispatches Codex MCP application through the managed `ntk` binary instead of the deleted local sync wrapper.
- The runtime parity tests now prove the native `render-vscode-mcp-template`, `render-mcp-runtime-artifacts`, and `sync-codex-mcp-config` contracts directly.
- The local wrappers `scripts/runtime/sync-codex-mcp-config.ps1` and `scripts/runtime/render-vscode-mcp-template.ps1` were deleted after same-slice consumer repoints and validation evidence passed.
- `scripts/runtime/render-mcp-runtime-artifacts.ps1` remains intentionally retained until the provider-surface projection catalog stops requiring a script-backed renderer path.