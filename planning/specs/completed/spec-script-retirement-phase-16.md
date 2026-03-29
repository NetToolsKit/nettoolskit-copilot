# Spec: Phase 16 Validation Reporting Wrapper Retirement

Generated: 2026-03-29 08:00

## Status

- LastUpdated: 2026-03-29 08:01
- Objective: define the design intent and safe cutover conditions for replacing the remaining validation-folder reporting scripts with native runtime reporting commands.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-16.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/validation/export-audit-report.ps1`
  - `scripts/validation/export-enterprise-trends.ps1`
  - `crates/commands/runtime/src/diagnostics/healthcheck.rs`

## Problem Statement

After Phase 15, the validation folder narrowed to two reporting scripts: `export-audit-report.ps1` and `export-enterprise-trends.ps1`. They are no longer low-fanout validation wrappers; they are operator-facing reporting entrypoints that depend on runtime healthcheck outputs, validation ledgers, and release evidence. Keeping them in PowerShell leaves the last validation-folder leaves alive even though the repository already owns the underlying behavior and artifact formats natively enough to port them.

## Desired Outcome

- Native `ntk runtime` reporting commands become the canonical audit-report and enterprise-trends entrypoints.
- Governance baselines and authored runbooks stop treating the validation-folder scripts as required evidence.
- The validation folder no longer contains reporting scripts after the phase closes.

## Design Decision

Implement the reporting pair in `crates/commands/runtime/src/diagnostics/` and expose them through `ntk runtime`, because `export-audit-report` depends on runtime healthcheck and moving that behavior into the validation crate would create a runtime/validation dependency cycle.

## Alternatives Considered

1. Keep the scripts as intentional exceptions
   - Rejected because they are repository-owned reporting logic, not external tool wrappers or shell-only launch surfaces.
2. Implement the reporting pair inside `crates/commands/validation`
   - Rejected because runtime already depends on validation; adding the reverse dependency would create a crate cycle.
3. Implement the reporting pair inside `crates/commands/runtime` and expose them through `ntk runtime`
   - Selected because runtime already owns healthcheck and can consume validation outputs without introducing a dependency cycle.

## Risks

- The native audit-report exporter must preserve warning-only semantics and path defaults closely enough to avoid breaking release and rollback runbooks.
- The enterprise-trends exporter depends on generated artifacts that may be absent locally, so the native command needs the same graceful warning behavior as the PowerShell version.
- `crates/cli/src/runtime_commands.rs` already has parallel formatting-only edits in the worktree, so the implementation must integrate without trampling those changes.

## Acceptance Criteria

- `ntk runtime healthcheck` is the canonical audit-report export path and produces a structured JSON report from native runtime healthcheck plus repository metadata.
- `ntk runtime export-enterprise-trends` exists and produces the JSON plus Markdown trend outputs with warning-only handling for missing inputs.
- Runbooks, release provenance evidence, and instruction policy baselines no longer require the two validation-folder scripts.
- If both wrappers are deleted, the safety matrix and parity ledger record the inventory reduction from `106` to `104` total scripts and from `2` to `0` validation-folder scripts.
- If one or both wrappers remain, the blocker is explicit, narrow, and recorded as intentional retention rather than open ambiguity.

## Execution Result

- The selected design was executed in `crates/commands/runtime/src/diagnostics/healthcheck.rs`, `crates/commands/runtime/src/diagnostics/enterprise_trends.rs`, and `crates/cli/src/runtime_commands.rs`.
- Operator-facing consumers now call `ntk runtime healthcheck` for audit-report export and `ntk runtime export-enterprise-trends` for trend export.
- The validation-folder reporting wrappers were removed locally after governance, runbook, and policy evidence stopped requiring them.