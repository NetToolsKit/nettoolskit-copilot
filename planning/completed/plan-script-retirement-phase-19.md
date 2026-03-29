# Phase 19: Common Helper Consumer Audit

Generated: 2026-03-29 09:24

## Status

- LastUpdated: 2026-03-29 09:24
- Objective: execute the first domain-level consumer sweep for `scripts/common/*.ps1`, prove whether any shared helpers are safe to retire, and record blocker evidence without forcing unsafe deletions.
- Normalized Request: continue the aggressive PowerShell-to-Rust retirement flow, keep planning updated, and keep committing each stable phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-19.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-18.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/common/*.ps1`
  - `scripts/runtime/*.ps1`
  - `scripts/security/*.ps1`
  - `scripts/orchestration/**/*.ps1`

## Scope Summary

1. `scripts/common/agent-runtime-hardening.ps1`
2. `scripts/common/codex-runtime-hygiene.ps1`
3. `scripts/common/common-bootstrap.ps1`
4. `scripts/common/console-style.ps1`
5. `scripts/common/git-hook-eof-settings.ps1`
6. `scripts/common/local-context-index.ps1`
7. `scripts/common/mcp-runtime-catalog.ps1`
8. `scripts/common/provider-surface-catalog.ps1`
9. `scripts/common/repository-paths.ps1`
10. `scripts/common/runtime-execution-context.ps1`
11. `scripts/common/runtime-install-profiles.ps1`
12. `scripts/common/runtime-operation-support.ps1`
13. `scripts/common/runtime-paths.ps1`
14. `scripts/common/validation-logging.ps1`
15. `scripts/common/vscode-runtime-hygiene.ps1`

This phase is complete only if:

- every shared helper is classified with concrete local-consumer evidence
- zero-consumer deletions happen only when no non-self blocker remains
- the continuity workstream can move to the remaining runtime/security/governance/orchestration sweeps with an explicit common-domain baseline

## Ordered Tasks

### Task 1: Freeze The Common-Domain Consumer Inventory

Status: `[x]` Completed

- Enumerated all 15 `scripts/common/*.ps1` leaves and captured their non-self local fanout.

### Task 2: Execute The Shared-Helper Consumer Sweep

Status: `[x]` Completed

- Verified that every shared helper still has live local consumers.
- Representative blocker evidence:
  - `common-bootstrap.ps1` remains the high-fanout loader across runtime, security, orchestration, and parity tests.
  - `provider-surface-catalog.ps1` remains blocked by `render-provider-surfaces.ps1`.
  - `mcp-runtime-catalog.ps1` remains blocked by `sync-vscode-global-mcp.ps1` and `render-mcp-runtime-artifacts.ps1`.
  - `runtime-execution-context.ps1`, `runtime-install-profiles.ps1`, and `runtime-operation-support.ps1` remain blocked by `bootstrap.ps1`, `install.ps1`, retained parity harnesses, and related runtime flows.

### Task 3: Record The Audit-Only Result

Status: `[x]` Completed

- No shared helper met the zero-consumer deletion rule in this phase.
- The domain remains in `retain until consumer migration completes`.
- The active continuity workstream now treats Phase 19 as closed evidence instead of pending work.

## Validation Checklist

- [x] targeted `rg` consumer sweep across `scripts/`, `.github/`, `definitions/`, and authored docs
- [x] no deletions were attempted without zero-consumer proof
- [x] `git diff --check` for the planning closeout bundle

## Risks And Fallbacks

- Shared helpers are foundational and dot-sourced widely; a false delete here would break multiple runtime domains at once.
- The correct fallback for a blocked shared helper is explicit retention evidence, not optimistic deletion.

## Closeout Expectations

- Archive the phase as completed even with zero deletions because the consumer evidence is now explicit and reusable.
- Leave the runtime-domain tactical sweeps free to continue without implying the common domain is deletion-ready.

## Executed Result

- Phase 19 closed as an audit-only phase with zero deletions.
- All 15 `scripts/common/*.ps1` leaves remain in `retain until consumer migration completes`.
- The continuity workstream now carries forward a proven common-domain blocker baseline instead of a speculative pending phase.