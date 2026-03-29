# Spec: Phase 20c Runtime Self-Heal Wrapper Retirement

Generated: 2026-03-29 09:24

## Status

- LastUpdated: 2026-03-29 09:24
- Objective: define the cutover conditions for deleting `scripts/runtime/self-heal.ps1` once the native `ntk runtime self-heal` contract, docs, policy inventory, and parity evidence all converge on Rust ownership.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-20c-self-heal.md`
- Source Inputs:
  - `planning/completed/plan-script-retirement-phase-19.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/runtime/self-heal.ps1`
  - `crates/commands/runtime/src/diagnostics/self_heal.rs`
  - `crates/cli/src/runtime_commands.rs`

## Problem Statement

The runtime crate already owns the self-heal behavior natively, but the local wrapper still remained in the repository because docs, policy inventory, and retained runtime parity coverage were still encoded around `scripts/runtime/self-heal.ps1`. That created a false signal that the PowerShell wrapper was still canonical even after native Rust ownership existed.

## Desired Outcome

- `ntk runtime self-heal` becomes the canonical operator-facing self-heal entrypoint.
- No non-planning consumer in this repository still requires `scripts/runtime/self-heal.ps1`.
- The local wrapper is deleted in the same slice that repoints docs, policy evidence, and parity coverage.

## Design Decision

Treat `self-heal` as a tactical Phase 20c runtime leaf cutover. The broader Phase 20 runtime-domain sweep remains open, but `self-heal` already had native CLI parity and a small enough consumer surface to justify a same-slice delete. The retained parity harness moves from direct script inspection to native CLI help and execution evidence instead of being removed.

## Alternatives Considered

1. Keep `self-heal.ps1` until the entire runtime domain sweep closes
   - Rejected because the wrapper already had a stable native replacement and only a small set of same-slice consumers remained.
2. Delete the wrapper without updating policy and parity evidence
   - Rejected because it would leave stale inventory and misleading regression coverage behind.

## Risks

- Removing the wrapper too early would break retained runtime parity coverage.
- Policy inventory must point at the native Rust owner before the `.ps1` path disappears.

## Acceptance Criteria

- Native CLI test coverage exists for `ntk runtime self-heal`.
- Retained runtime parity coverage proves the native flag surface.
- Docs and policy inventory no longer require `scripts/runtime/self-heal.ps1`.
- The local wrapper is deleted and the safety matrix reflects a 99-script live estate.

## Executed Result

- The native `ntk runtime self-heal` boundary is now the canonical contract.
- Runtime parity, docs, and policy evidence were repointed to the native contract.
- `scripts/runtime/self-heal.ps1` was deleted and the retirement evidence moved into the completed phase artifacts.