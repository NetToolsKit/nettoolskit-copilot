# Script Retirement Phase 4

Generated: 2026-03-28 20:05

## Objective

Define the safe cutover path for the next four runtime-owned local leaves:

1. `scripts/runtime/update-local-context-index.ps1`
2. `scripts/runtime/query-local-context-index.ps1`
3. `scripts/runtime/export-planning-summary.ps1`
4. `scripts/runtime/apply-vscode-templates.ps1`

by replacing their live consumer contracts with native `ntk runtime` entrypoints.

## Context

Phase 3 retired the hook and EOF leaves and reduced the live local PowerShell estate to `139`. The next smallest concrete runtime leaves already have native Rust business logic, but their canonical shell contracts are still encoded in local consumers, tests, and authored guidance.

- `update-local-context-index.ps1` and `query-local-context-index.ps1` already map to `crates/commands/runtime/src/continuity/local_context.rs`
- `export-planning-summary.ps1` already maps to `crates/commands/runtime/src/continuity/planning_summary.rs`
- `apply-vscode-templates.ps1` already maps to `crates/commands/runtime/src/sync/apply_vscode_templates.rs`

This makes Phase 4 a consumer-cutover and executable-boundary problem, not a missing-implementation problem.

## Current Slice Summary

- live local script inventory: `139`
- blocked leaves in scope: `4`
- native Rust owners already exist:
  - `crates/commands/runtime/src/continuity/local_context.rs`
  - `crates/commands/runtime/src/continuity/planning_summary.rs`
  - `crates/commands/runtime/src/sync/apply_vscode_templates.rs`
- high-value live consumers:
  - `scripts/runtime/invoke-super-agent-housekeeping.ps1`
  - `scripts/runtime/self-heal.ps1`
  - `definitions/shared/instructions/repository-operating-model.instructions.md`
  - `definitions/shared/instructions/super-agent.instructions.md`
  - `definitions/providers/vscode/workspace/README.md`
  - `scripts/tests/runtime/runtime-scripts.tests.ps1`
  - `scripts/tests/runtime/vscode-agent-hooks.tests.ps1`

## Design Decisions

1. Retire this slice through a native executable contract first, not by pointing more wrappers at other wrappers.
2. Treat `ntk runtime ...` as the canonical operator-facing executable surface when Rust already owns the behavior.
3. Update authored guidance and parity tests in the same phase as the consumer cutover so documentation does not become the new blocker.
4. Keep retained PowerShell launchers only when a shell-hosted compatibility contract is still required after the native cutover.

## Non-Goals

- retiring runtime diagnostics wrappers in the same phase
- retiring validation shell wrappers in the same phase
- forcing byte-for-byte convergence with `C:\Users\tguis\copilot-instructions` when this repository already has a safer native executable contract

## Risks

- the continuity commands may depend on output formatting that current PowerShell wrappers provide to operators or tests
- planning-summary export must still honor the fallback from `planning/active` to `.build/super-agent/planning/active`
- VS Code template application may still be referenced as a direct shell action in docs and operator routines even when native ownership already exists

## Alternatives Considered

### Alternative 1: Keep The PowerShell Leaves And Only Document Rust Ownership

Rejected. These four leaves already have native business logic, so leaving them untouched just preserves path coupling.

### Alternative 2: Retire The Entire Validation Domain First

Rejected for this phase. Validation has a wider authored and policy surface, while the runtime continuity/template leaves are smaller and more deterministic.

### Alternative 3: Retire Only `apply-vscode-templates.ps1`

Rejected. The continuity trio and template application all depend on the same kind of executable-cutover work, so batching them keeps the CLI and test changes coherent.

## Acceptance Criteria

1. Each in-scope behavior has a native `ntk runtime` entrypoint.
2. Live consumers and docs stop treating the `.ps1` files as the canonical executable contract.
3. Runtime parity tests validate the native entrypoint instead of the leaf file.
4. Each in-scope leaf ends the phase either deleted or explicitly reclassified as an intentional retained wrapper.
5. The retirement matrix and parity ledger are updated to the final state.
6. Phase 4 is ready to archive when no in-scope leaf remains in ambiguous blocked state.

## Planning Readiness

- ready-for-implementation

## Recommended Specialist Focus

- `dev-rust-engineer`
- `test-engineer`
- `docs-release-engineer`