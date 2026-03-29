# Phase 10: Validation Wrapper Retirement - Compatibility Lifecycle and Dotnet Standards

Generated: 2026-03-28 22:35

## Status

- LastUpdated: 2026-03-28 22:35
- Objective: retire the low-fanout validation wrappers for `validate-compatibility-lifecycle-policy` and `validate-dotnet-standards` by exposing native `ntk validation` executable boundaries, repointing the remaining validation and policy consumers, and deleting the local PowerShell leaves.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keep planning current, and commit each stable retirement phase.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/active/spec-script-retirement-phase-10.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-9.md`
  - `planning/specs/completed/spec-script-retirement-phase-9.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/policy/compatibility_lifecycle_policy.rs`
  - `crates/commands/validation/src/standards/dotnet_standards.rs`
  - `scripts/tests/runtime/compatibility-lifecycle.tests.ps1`
- Current Slice Snapshot:
  - `scripts/**/*.ps1` inventory is `121`
  - native Rust owners already exist for both target checks
  - `validate-all.ps1` still treats both checks as local script launch surfaces

## Scope Summary

1. `scripts/validation/validate-compatibility-lifecycle-policy.ps1`
2. `scripts/validation/validate-dotnet-standards.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for both checks
- `validate-all.ps1` no longer requires the two local wrapper paths
- policy and validation inventory evidence stop treating the deleted `.ps1` files as canonical
- the compatibility runtime test no longer shells into the deleted validation wrapper
- both wrappers are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Freeze The Phase-10 Native Validation Contract

Status: `[ ]` Pending

- Register the phase-10 design intent in the spec and lock the cutover acceptance criteria.
- Confirm the native Rust owners and the exact CLI contract for:
  - `compatibility-lifecycle-policy`
  - `dotnet-standards`
- Target paths:
  - `planning/specs/active/spec-script-retirement-phase-10.md`
  - `planning/active/plan-script-retirement-phase-10.md`
  - `planning/README.md`
  - `planning/specs/README.md`
- Validation checkpoints:
  - `rg -n "compatibility_lifecycle_policy|dotnet_standards" crates/commands/validation/src crates/cli/src/validation_commands.rs`
  - `git diff --check`
- Commit checkpoint suggestion:
  - `docs(planning): open script retirement phase 10`

### Task 2: Add The Missing Native CLI Boundaries

Status: `[ ]` Pending

- Extend `ntk validation` with executable native contracts for:
  - `compatibility-lifecycle-policy`
  - `dotnet-standards`
- Keep CLI arguments aligned with the current behavior:
  - `compatibility-lifecycle-policy`: repo root, compatibility path, warning-only, detailed output
  - `dotnet-standards`: repo root, template directory
- Add or update focused CLI tests for both subcommands.
- Target paths:
  - `crates/cli/src/validation_commands.rs`
  - `crates/cli/tests/validation_commands_tests.rs`
- Validation checkpoints:
  - `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
  - `git diff --check`
- Commit checkpoint suggestion:
  - `feat(validation): add native compatibility and dotnet command surfaces`

### Task 3: Retire The Two Wrapper Leaves And Repoint Their Consumers

Status: `[ ]` Pending

- Repoint the remaining local consumers that still encode wrapper paths:
  - `scripts/validation/validate-all.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/tests/runtime/compatibility-lifecycle.tests.ps1`
  - `.github/policies/instruction-system.policy.json`
  - validation surface contracts and tests
- Delete:
  - `scripts/validation/validate-compatibility-lifecycle-policy.ps1`
  - `scripts/validation/validate-dotnet-standards.ps1`
- Target paths:
  - `crates/commands/validation/src/contracts.rs`
  - `crates/commands/validation/tests/contracts_tests.rs`
  - `scripts/validation/validate-all.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/tests/runtime/compatibility-lifecycle.tests.ps1`
  - `.github/policies/instruction-system.policy.json`
- Validation checkpoints:
  - `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
  - `cargo test -p nettoolskit-validation --quiet`
  - `pwsh -File .\scripts\validation\validate-instructions.ps1`
  - focused `validate-all.ps1` profile for:
    - `validate-compatibility-lifecycle-policy`
    - `validate-dotnet-standards`
  - `git diff --check`
- Commit checkpoint suggestion:
  - `refactor(validation): retire compatibility and dotnet wrappers`

### Task 4: Rebaseline Planning Evidence And Archive The Phase

Status: `[ ]` Pending

- Update the completed matrix and parity ledger with the executed phase-10 result.
- Archive the phase-10 plan and spec only after implementation and focused validation are complete.
- Target paths:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/README.md`
  - `planning/specs/README.md`
  - `planning/completed/plan-script-retirement-phase-10.md`
  - `planning/specs/completed/spec-script-retirement-phase-10.md`
- Validation checkpoints:
  - `git diff --check`
  - final smoke runs from the implementation phase
- Commit checkpoint suggestion:
  - `docs(planning): close script retirement phase 10`

## Validation Checklist

- `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- `pwsh -File .\scripts\validation\validate-instructions.ps1`
- `pwsh -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- focused `validate-all.ps1` phase-10 profile for:
  - `validate-compatibility-lifecycle-policy`
  - `validate-dotnet-standards`
- `git diff --check`

## Risks And Fallbacks

- `validate-all.ps1` still owns the live suite orchestration, so the wrapper deletion must not happen before native CLI dispatch exists.
- The compatibility runtime test still shells directly into the local wrapper and must move in the same slice or it will become a false negative after deletion.
- `instruction-system.policy.json` still encodes the legacy dotnet wrapper path as required evidence; if that is not converted in the same slice, policy validation will keep treating the wrapper as canonical.
- If the CLI contract needs to diverge from the wrapper behavior, update the spec first and then revise this plan before deleting the wrappers.

## Recommended Specialist

- Primary: `dev-rust-engineer`
- Secondary: `test-engineer`
- Documentation and closeout: `docs-release-engineer`
- Review: `review-code-engineer`

## Closeout Expectations

- Update `planning/README.md` and `planning/specs/README.md` when the active phase is created and again when it is archived.
- Use an English commit message aligned to the retirement slice, for example: `refactor(validation): retire compatibility and dotnet wrappers`.
- Do not add a changelog entry for this planning artifact alone; add release notes only when the implementation phase lands user-visible behavior.
- Move the phase to completed only after the wrapper deletions and focused validations are materially complete.

## Worktree Guidance

- Isolated worktree execution is optional for this phase because the slice is narrow and confined to CLI, validation, policy, and planning surfaces.