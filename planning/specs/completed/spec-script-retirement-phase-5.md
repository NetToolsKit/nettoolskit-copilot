# Script Retirement Phase 5

Generated: 2026-03-28 19:39

## Objective

Define the safe cutover path for the next three validation-owned local leaves:

1. `scripts/validation/validate-routing-coverage.ps1`
2. `scripts/validation/validate-architecture-boundaries.ps1`
3. `scripts/validation/validate-audit-ledger.ps1`

by replacing their live compatibility orchestration with native `ntk validation` entrypoints.

## Context

Phase 4 retired the runtime continuity/template leaves and reduced the live local PowerShell estate to `135`. Phase 5 then completed the validation consumer cutover and reduced the live local PowerShell estate to `132`.

The three low-fanout validation leaves already had native Rust implementations in `crates/commands/validation`; the work in this phase was opening the executable boundary and clearing policy/inventory blockers rather than implementing new validation logic.

## Outcome

1. `scripts/validation/validate-routing-coverage.ps1` was retired locally after `validate-all.ps1` moved to `ntk validation routing-coverage`.
2. `scripts/validation/validate-architecture-boundaries.ps1` was retired locally after `validate-all.ps1` moved to `ntk validation architecture-boundaries`.
3. `scripts/validation/validate-audit-ledger.ps1` was retired locally after `validate-all.ps1` and release guidance moved to `ntk validation audit-ledger`.
4. Governance and inventory surfaces stopped requiring the deleted validation wrapper paths.
5. The remaining validation retirement backlog is now concentrated in the broader wrapper hubs and policy-heavy checks rather than low-fanout leaves.

## Design Decisions

1. Introduce a narrow `ntk validation` boundary instead of embedding more Rust-specific knowledge inside PowerShell wrappers.
2. Keep `validate-all.ps1` as a compatibility orchestrator, but let it dispatch eligible checks through the native executable contract.
3. Remove deleted leaf paths from validation inventory and governance evidence in the same slice so policy files do not become the new blocker.
4. Record retired validation leaves as `retired locally` in the parity ledger rather than implicitly folding them into the remaining validation domain row.

## Non-Goals

- retiring the full validation domain in one patch
- changing validation profile ids or suite semantics
- removing `validate-all.ps1` in this slice

## Acceptance Criteria

1. `ntk validation` exposes `routing-coverage`, `architecture-boundaries`, and `audit-ledger`.
2. `validate-all.ps1` can execute the three checks without their local `.ps1` wrappers.
3. Inventory/policy/docs no longer require the three deleted script paths.
4. The three leaves are deleted locally.
5. The retirement matrix and parity ledger are updated to the final executed state.
6. Phase 5 is ready to archive because no in-scope leaf remains blocked by a local path requirement.

## Planning Readiness

- ready-for-implementation