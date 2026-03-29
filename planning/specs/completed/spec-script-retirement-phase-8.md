# Script Retirement Phase 8

Generated: 2026-03-28 21:06

## Objective

Define the safe cutover path for the next three low-fanout validation-owned local wrappers:

1. `scripts/validation/validate-policy.ps1`
2. `scripts/validation/validate-agent-skill-alignment.ps1`
3. `scripts/validation/validate-agent-permissions.ps1`

by exposing the missing native `ntk validation` entrypoints and then deleting the local wrappers only after compatibility, governance, and orchestration surfaces stop requiring their paths.

## Context

Phase 7 retired the local `security-baseline` and `supply-chain` wrappers, reducing the live local PowerShell estate to `127`.

The three Phase 8 checks already exist as native Rust implementations inside `crates/commands/validation`, but they still depend on local PowerShell wrappers as the executable boundary. Compared with `validate-agent-orchestration`, `validate-release-governance`, and `validate-release-provenance`, this bundle has lower live fanout and fewer user-facing docs to reconcile.

## Outcome

1. `ntk validation` exposes native executable contracts for:
   - `policy`
   - `agent-skill-alignment`
   - `agent-permissions`
2. `scripts/validation/validate-all.ps1` dispatches the three checks through the native executable boundary.
3. Inventory, policy, and orchestration surfaces stop requiring the local wrapper paths directly.
4. The three local wrappers are deleted from `scripts/validation/`.
5. The retirement matrix and parity ledger capture the final executed state, reducing the live local script estate from `127` to `124`.

## Design Decisions

1. Keep the bundle narrow and coherent around repository policy and agent alignment/permissions, instead of mixing in higher-fanout release or orchestration wrappers.
2. Use `ntk validation` as the only new executable boundary; do not add more compatibility PowerShell leaf wrappers.
3. Replace removed wrapper evidence with native Rust implementation files and CLI/orchestration entrypoints when required-evidence baselines would otherwise lose coverage.
4. Defer `validate-agent-orchestration`, `validate-release-governance`, and `validate-release-provenance` to later phases because they still have broader orchestration and documentation fanout.

## Non-Goals

- retiring `validate-all.ps1`
- changing validation profile ids or orchestration order
- broad release or pipeline cutover in the same slice
- retirement of `validate-agent-orchestration.ps1`

## Acceptance Criteria

1. `ntk validation policy` executes successfully with the current repository policy contract.
2. `ntk validation agent-skill-alignment` executes successfully against deterministic fixtures.
3. `ntk validation agent-permissions` executes successfully in both warning-only and enforcing flows.
4. `validate-all.ps1` no longer resolves any of the three deleted local wrapper paths.
5. The deleted wrapper paths are removed from policy, inventory, and orchestration surfaces that would otherwise block retirement.
6. The three `.ps1` files are deleted locally and the slice passes its scoped validation checklist.

## Executed Result

The executable boundary, compatibility routing, inventory cleanup, and wrapper deletion were completed in this phase. Focused `validate-all` proof passed in warning-only mode and exposed only pre-existing repository policy debt in enforcing mode.

## Risks

1. `validate-agent-permissions` has warning-only semantics, so the CLI and suite wiring must preserve enforcement behavior precisely.
2. `validate-policy` reads local git config and `.githooks`, so fixtures and repository-root assumptions can make CLI regressions non-obvious.
3. `validate-stage.ps1` and runtime orchestration may still assume local `.ps1` leaves, even when `validate-all.ps1` is already native-routed.

## Planning Readiness

- executed-and-completed