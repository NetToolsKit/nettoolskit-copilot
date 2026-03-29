# Phase 13: Validation Wrapper Retirement - Operational Hygiene Leaves

Generated: 2026-03-28 23:32

## Status

- LastUpdated: 2026-03-28 23:32
- Objective: retire the remaining low-fanout operational validation wrappers by exposing native `ntk validation` command surfaces, repointing live consumers, and deleting the local PowerShell leaves.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keep planning current, and commit each stable retirement phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/active/spec-script-retirement-phase-13.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-12.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/agent_orchestration/agent_hooks.rs`
  - `crates/commands/validation/src/operational_hygiene/shell_hooks.rs`
  - `crates/commands/validation/src/operational_hygiene/runtime_script_tests.rs`
- Current Slice Snapshot:
  - `scripts/**/*.ps1` inventory should fall from `112` to `109`
  - `scripts/validation/*.ps1` inventory should fall from `8` to `5`
  - `scripts/validation/validate-*.ps1` inventory should fall from `5` to `2`
  - native Rust owners already exist for all three target checks
  - the remaining gap is executable CLI ownership plus local consumer cutover

## Scope Summary

1. `scripts/validation/validate-agent-hooks.ps1`
2. `scripts/validation/validate-shell-hooks.ps1`
3. `scripts/validation/validate-runtime-script-tests.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for all three checks
- `validate-all.ps1` no longer shells into the three local wrappers
- `validate-instructions.ps1`, runtime parity fixtures, validation fixtures, and authored guidance stop treating the deleted `.ps1` files as canonical
- the three wrappers are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Freeze The Phase-13 Contract

Status: `[ ]` Pending

- Lock the phase-13 design intent in the spec and confirm the acceptance criteria.
- Confirm the native Rust owners and exact CLI contracts for:
  - `agent-hooks`
  - `shell-hooks`
  - `runtime-script-tests`

### Task 2: Add The Missing Native CLI Boundaries

Status: `[ ]` Pending

- Extend `ntk validation` with executable native contracts for the three checks.
- Keep CLI arguments aligned with the existing behavior:
  - `agent-hooks`: repo root, hooks config path, common script path, warning-only
  - `shell-hooks`: repo root, hooks directory, shellcheck toggle, warning-only
  - `runtime-script-tests`: repo root, tests root, warning-only
- Add or update focused CLI tests for each new subcommand.

### Task 3: Repoint Consumers And Delete The Three Leaves

Status: `[ ]` Pending

- Repoint live consumers that still encode the wrapper paths:
  - `scripts/validation/validate-all.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `crates/commands/validation/tests/support/security_fixtures.rs`
  - `crates/orchestrator/tests/execution/pipeline_parity/support/fake_codex_runner.rs`
  - `scripts/tests/runtime/agent-orchestration-engine.tests.ps1`
  - `definitions/providers/codex/scripts/README.md`
- Delete the three wrappers only after the consumer chain is cut over.

### Task 4: Rebaseline Planning Evidence And Archive The Phase

Status: `[ ]` Pending

- Update `planning/completed/script-retirement-safety-matrix.md`.
- Update `planning/completed/rust-script-parity-ledger.md`.
- Move the plan/spec to completed once the validations and inventory updates are materially done.

## Validation Checklist

- `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- `pwsh -NoProfile -File .\scripts\validation\validate-instructions.ps1`
- `pwsh -NoProfile -File .\scripts\tests\runtime\agent-orchestration-engine.tests.ps1`
- `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- focused `validate-all.ps1` proof for the three phase-13 checks
- `git diff --check`