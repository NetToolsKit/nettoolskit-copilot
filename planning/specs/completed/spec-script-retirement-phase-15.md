# Spec: Phase 15 Native Validate-All Cutover Readiness

Generated: 2026-03-29 00:40

## Status

- LastUpdated: 2026-03-29 07:35
- Objective: define the design intent and safe cutover conditions for replacing the local `validate-all` PowerShell wrapper with the native `ntk validation all` contract.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-15.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/orchestration/validate_all.rs`
  - `crates/cli/src/validation_commands.rs`
  - `scripts/runtime/healthcheck.ps1`

## Problem Statement

After Phase 14, the validation folder still contains only three PowerShell files: `validate-all.ps1`, `export-audit-report.ps1`, and `export-enterprise-trends.ps1`. The native Rust orchestration already exists for `validate-all`, but runtime scripts, orchestration stages, runbooks, policies, and projected skill surfaces still encode the wrapper path directly. That leaves the native contract underused and keeps the local wrapper in the critical path.

## Desired Outcome

- `ntk validation all` becomes the canonical validation orchestration contract for runtime, orchestration, docs, runbooks, and governance evidence.
- The repository stops treating `scripts/validation/validate-all.ps1` as the authoritative proof surface.
- The wrapper is deleted in the same phase if no hard compatibility blocker remains after the repoint.

## Design Decision

Use the existing Rust `validate-all` implementation and CLI surface as the authoritative contract, repoint the remaining repository consumers to the native command, then delete the local wrapper if the remaining operational semantics are already covered by Rust and no compatibility-only blocker survives.

## Alternatives Considered

1. Keep the wrapper indefinitely
   - Rejected because the Rust orchestration already owns the behavior and the remaining references are mostly consumer inertia, not missing implementation.
2. Delete the wrapper immediately without repointing consumers
   - Rejected because runtime scripts, stage fallbacks, runbooks, release provenance, and projected skills still encode the wrapper path explicitly.
3. Repoint the consumer chain first, then delete the wrapper only if no hard blocker remains
   - Selected because it preserves operator continuity while letting the repository prove whether the wrapper is still necessary.

## Risks

- `scripts/runtime/healthcheck.ps1` currently shells into the wrapper and needs a native invocation path before deletion.
- `release-provenance` and instruction policy baselines still lock the wrapper path as evidence and must be repointed atomically.
- One shell-hook regression test currently encodes a PowerShell-specific `validate-all.ps1` example and needs a semantically equivalent replacement when the wrapper disappears.

## Acceptance Criteria

- Runtime and orchestration entrypoints no longer rely on `scripts/validation/validate-all.ps1`.
- Authored docs, runbooks, policies, and skill surfaces no longer present the wrapper as the canonical validation command.
- Release provenance uses `validateAllCommand` instead of `validateAllPath`.
- If the wrapper is deleted, the safety matrix and parity ledger record the inventory reduction and the validation folder narrows from `3` to `2` scripts.
- If the wrapper is retained, the blocker is explicit, narrow, and recorded as an intentional retention rather than open ambiguity.

## Executed Result

- The native `ntk validation all` contract was confirmed as the authoritative repository validation entrypoint.
- Runtime/orchestration consumers, authored docs, runbooks, governance baselines, and projected skill surfaces were repointed away from `scripts/validation/validate-all.ps1`.
- The local wrapper was deleted safely in this phase after live references were cleared from active repository consumers.
- The validation folder narrowed from `3` scripts to the `2` remaining reporting leaves.