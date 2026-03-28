# Script Retirement Phase 3

Generated: 2026-03-28 18:41

## Objective

Define the safe cutover path for the next two blocked local leaves:

1. `scripts/runtime/hooks/pre-tool-use.ps1`
2. `scripts/maintenance/trim-trailing-blank-lines.ps1`

by replacing their live consumer contracts with Rust-native invocation boundaries.

## Context

Phase 2 retired the validation-owned test automation wrappers and reduced the live local PowerShell estate to `141`. Phase 3 then completed the hook/EOF consumer decoupling and reduced the live local PowerShell estate to `139`.

- `pre-tool-use.ps1` no longer owns behavior locally; wrapper/bootstrap surfaces now dispatch through `ntk runtime pre-tool-use`
- `trim-trailing-blank-lines.ps1` no longer owns behavior locally; git-hook installation, global alias setup, and runtime parity tests now dispatch through `ntk runtime trim-trailing-blank-lines`

This means the phase is no longer a design-only problem. The consumer cutover completed and both local leaves were retired safely.

## Current Slice Summary

- live local script inventory: `139`
- blocked leaves in scope: `0`
- native Rust owners already exist:
  - `crates/commands/runtime/src/hooks/pre_tool_use.rs`
  - Rust-owned EOF hygiene and hook setup surfaces in `crates/commands/runtime/src/hooks/`
- upstream comparison result:
  - `C:\Users\tguis\copilot-instructions` still carries both PowerShell files
  - the authoritative guidance lives in instruction docs, hook bootstrap JSON, and runtime/operator documentation, not in the local leaves themselves

## Outcome

1. `scripts/runtime/hooks/pre-tool-use.ps1` was retired locally after `.github/hooks/scripts/pre-tool-use.ps1` and the provider-authored wrapper moved to the native `ntk runtime pre-tool-use` contract.
2. `scripts/maintenance/trim-trailing-blank-lines.ps1` was retired locally after `git trim-eof`, managed pre-commit EOF hygiene, and bootstrap runtime projection moved to the native `ntk runtime trim-trailing-blank-lines` contract.
3. The remaining hook wrappers are explicit compatibility launch surfaces, not accidental owners of the retired behavior.

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
6. Phase 3 is ready to archive because both in-scope leaves are now `retired locally`.

## Planning Readiness

- ready-for-implementation

## Recommended Specialist Focus

- `dev-rust-engineer`
- `test-engineer`
- `docs-release-engineer`