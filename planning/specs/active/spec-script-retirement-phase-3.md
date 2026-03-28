# Script Retirement Phase 3

Generated: 2026-03-28 17:46

## Objective

Define the safe cutover path for the next two blocked local leaves:

1. `scripts/runtime/hooks/pre-tool-use.ps1`
2. `scripts/maintenance/trim-trailing-blank-lines.ps1`

by replacing their live consumer contracts with Rust-native invocation boundaries.

## Context

Phase 2 retired the validation-owned test automation wrappers and reduced the live local PowerShell estate to `141`. The backlog is smaller now, but the next two leaves are not metadata-only deletions:

- `pre-tool-use.ps1` still sits in the hook bootstrap/projection chain and in validation checks that encode the legacy path
- `trim-trailing-blank-lines.ps1` still sits in git-hook installation, global alias setup, and runtime parity tests

This means the next phase is a consumer-decoupling design problem before it becomes a deletion problem.

## Current Slice Summary

- live local script inventory: `141`
- blocked leaves in scope: `2`
- native Rust owners already exist:
  - `crates/commands/runtime/src/hooks/pre_tool_use.rs`
  - Rust-owned EOF hygiene and hook setup surfaces in `crates/commands/runtime/src/hooks/`
- upstream comparison result:
  - `C:\Users\tguis\copilot-instructions` still carries both PowerShell files
  - the authoritative guidance lives in instruction docs, hook bootstrap JSON, and runtime/operator documentation, not in the local leaves themselves

## Design Decisions

1. Treat both leaves as consumer-decoupling work, not as simple dead-wrapper cleanup.
2. Keep provider/bootstrap wrappers minimal and move canonical behavior ownership to Rust wherever feasible.
3. Preserve hook/operator compatibility only through explicit retained wrapper decisions, not through accidental path coupling.
4. Require the safety matrix to choose one of only two outcomes by the end of the phase:
   - `retired locally`
   - `retain wrapper intentionally`

## Non-Goals

- deleting additional unrelated domains in the same phase
- byte-for-byte synchronization with `C:\Users\tguis\copilot-instructions`
- changing retained-wrapper policy for unrelated deploy or runtime test harness scripts

## Risks

- there may be no acceptable zero-PowerShell launch path for the provider hook bootstrap without adding a new CLI/runtime entrypoint
- the git alias surface may still need a PowerShell launch shim even when the underlying trim behavior is Rust-native
- parity tests may currently be validating the wrong contract by asserting legacy file paths instead of behavior

## Alternatives Considered

### Alternative 1: Delete Both Leaves Immediately

Rejected. Live consumers still hardcode their paths.

### Alternative 2: Leave Both Leaves Blocked And Move On

Rejected. They are now the smallest concrete blocked leaves and the right next decoupling target.

### Alternative 3: Reclassify Both As Retained Wrappers Without Refactoring Consumers

Rejected for now. Retention may be the final outcome, but only after the consumer chain is fully understood and the Rust-native boundary is evaluated against the live hook and alias flows.

## Acceptance Criteria

1. Every live blocker for both leaves is recorded explicitly.
2. The canonical Rust-facing invocation path is defined for `PreToolUse` and EOF trim hygiene.
3. Consumers and tests are updated to the new contract.
4. Each leaf ends the phase either deleted or explicitly reclassified as an intentional retained wrapper.
5. The retirement matrix and parity ledger are updated to the final state.

## Planning Readiness

- ready-for-implementation

## Recommended Specialist Focus

- `dev-rust-engineer`
- `test-engineer`
- `docs-release-engineer`