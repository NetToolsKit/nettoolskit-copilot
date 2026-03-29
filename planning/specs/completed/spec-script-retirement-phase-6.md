# Script Retirement Phase 6

Generated: 2026-03-28 20:19

## Objective

Define the safe cutover path for the next three validation-owned local wrappers:

1. `scripts/validation/validate-powershell-standards.ps1`
2. `scripts/validation/validate-shared-script-checksums.ps1`
3. `scripts/validation/validate-warning-baseline.ps1`

by exposing the missing native `ntk validation` entrypoints and then deleting the local wrappers only after compatibility and governance surfaces stop requiring their paths.

## Context

Phase 5 retired the low-fanout validation wrappers for routing coverage, architecture boundaries, and audit ledger, reducing the live local PowerShell estate to `132`.

The three Phase 6 checks already exist as native Rust implementations inside `crates/commands/validation`, and the native `validate-all` orchestration already treats them as Rust-owned surfaces. The remaining gap is the shell compatibility boundary: `validate-all.ps1`, validation inventories, and governance evidence still encode the local wrapper paths directly.

Phase 6 completed that executable-boundary cleanup and reduced the live local PowerShell estate to `129`.

## Outcome

1. `ntk validation` exposes native executable contracts for:
   - `powershell-standards`
   - `shared-script-checksums`
   - `warning-baseline`
2. `scripts/validation/validate-all.ps1` dispatches the three checks through the native executable boundary while preserving current suite-level options.
3. Inventory, policy, and release guidance stop requiring the local wrapper paths directly.
4. The three local wrappers are deleted from `scripts/validation/`.
5. The retirement matrix and parity ledger capture the final executed state, reducing the live local script estate from `132` to `129`.

## Design Decisions

1. Use `ntk validation` as the only new executable boundary; do not add more compatibility PowerShell leaf wrappers.
2. Keep `validate-all.ps1` as the shell orchestration surface for now, but let it route eligible checks to the native runtime binary.
3. Remove local path requirements from policy and inventory in the same slice so deletion safety is proven by the executed phase, not deferred.
4. Record each deleted validation wrapper explicitly as `retired locally` in the parity ledger instead of treating the whole validation domain as implicitly cut over.

## Non-Goals

- retiring `validate-all.ps1`
- changing validation profile ids or orchestration order
- removing the shared checksum manifest or warning baseline artifacts themselves
- broad domain-level validation retirement beyond the three wrappers in scope

## Acceptance Criteria

1. `ntk validation powershell-standards` executes successfully with the existing suite options (`include-all-scripts`, `strict`, `skip-script-analyzer`, `warning-only`).
2. `ntk validation shared-script-checksums` executes successfully with `manifest-path`, `warning-only`, and `detailed-output`.
3. `ntk validation warning-baseline` executes successfully with `baseline-path`, `analyzer-report-path`, `report-path`, and `warning-only`.
4. `validate-all.ps1` no longer resolves any of the three local wrapper paths.
5. The deleted wrapper paths are removed from policy, inventory, and release evidence that would otherwise block retirement.
6. The three `.ps1` files are deleted locally and the slice passes its scoped validation checklist.

## Risks

1. `validate-all.ps1` must preserve warning-demotion semantics exactly; a CLI flag mismatch would silently change enforcement behavior.
2. The policy surfaces may still treat deleted wrapper paths as required evidence if any list is missed.
3. `validate-warning-baseline` has report-writing side effects, so the CLI contract must preserve deterministic testability through explicit report-path support.

## Planning Readiness

- ready-for-implementation