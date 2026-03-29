# Script Retirement Phase 7

Generated: 2026-03-28 20:49

## Objective

Define the safe cutover path for the next two validation-owned local wrappers:

1. `scripts/validation/validate-security-baseline.ps1`
2. `scripts/validation/validate-supply-chain.ps1`

by exposing the missing native `ntk validation` entrypoints and then deleting the local wrappers only after compatibility and governance surfaces stop requiring their paths.

## Context

Phase 6 retired the validation wrappers for PowerShell standards, shared script checksums, and warning baseline, reducing the live local PowerShell estate to `129`.

The two Phase 7 checks already exist as native Rust implementations inside `crates/commands/validation`, and the native validation orchestration already treats them as Rust-owned surfaces. The remaining gap is the shell compatibility boundary: `validate-all.ps1`, validation inventories, release evidence, and some provider-authored skill examples still encode the local wrapper paths directly.

Phase 7 completed that executable-boundary cleanup and reduced the live local PowerShell estate to `127`.

## Outcome

1. `ntk validation` exposes native executable contracts for:
   - `security-baseline`
   - `supply-chain`
2. `scripts/validation/validate-all.ps1` dispatches the two checks through the native executable boundary.
3. Inventory, policy, and release guidance stop requiring the local wrapper paths directly.
4. The two local wrappers are deleted from `scripts/validation/`.
5. The retirement matrix and parity ledger capture the final executed state, reducing the live local script estate from `129` to `127`.

## Design Decisions

1. Use `ntk validation` as the only new executable boundary; do not add more compatibility PowerShell leaf wrappers.
2. Keep `validate-all.ps1` as the shell orchestration surface for now, but let it route eligible checks to the native runtime binary.
3. Remove local path requirements from policy, inventory, release guidance, and provider-authored skill examples in the same slice so deletion safety is proven by the executed phase, not deferred.
4. Record each deleted validation wrapper explicitly as `retired locally` in the parity ledger instead of treating the whole validation domain as implicitly cut over.
5. When legacy wrapper paths are removed from required-evidence surfaces, replace them with the native Rust implementation files and CLI/orchestration entrypoints that now own the behavior.

## Non-Goals

- retiring `validate-all.ps1`
- changing validation profile ids or orchestration order
- relaxing the current `security-baseline` or `supply-chain` policy expectations
- broad domain-level validation retirement beyond the two wrappers in scope

## Acceptance Criteria

1. `ntk validation security-baseline` executes successfully with the existing compatibility semantics.
2. `ntk validation supply-chain` executes successfully with the existing compatibility semantics.
3. `validate-all.ps1` no longer resolves either deleted local wrapper path.
4. The deleted wrapper paths are removed from policy, inventory, release guidance, and provider-authored skill examples that would otherwise block retirement, and the native Rust implementation files replace them as required evidence.
5. The two `.ps1` files are deleted locally and the slice passes its scoped validation checklist.
6. Focused `validate-all` proof exists for both warning-only and enforcing mode, even if enforcing still surfaces pre-existing repository debt unrelated to the wrapper cutover.
7. Post-review regression coverage proves that `security-baseline` scans beyond an allowlisted first match and that `supply-chain` fails when required license evidence has no configured path.

## Risks

1. `validate-all.ps1` must preserve warning-demotion semantics exactly; a CLI flag mismatch would silently change enforcement behavior.
2. The provider-authored skill surfaces can drift unless they are re-rendered after updating the definitions-backed skill examples.
3. The enforcing run can still fail on pre-existing repository hygiene debt, so the phase closeout must distinguish routing proof from baseline cleanup debt.

## Planning Readiness

- completed