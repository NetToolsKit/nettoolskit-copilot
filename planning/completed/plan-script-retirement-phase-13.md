# Phase 13: Validation Wrapper Retirement - Hook And Runtime Test Leaves

Generated: 2026-03-28 23:33

## Status

- LastUpdated: 2026-03-28 23:46
- Objective: retire the remaining low-fanout hook and runtime-test validation wrappers by exposing native `ntk validation` command surfaces, repointing live consumers, and deleting the local PowerShell leaves.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keep planning current, and commit each stable retirement phase with detailed messages.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-13.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-12.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `crates/commands/validation/src/agent_orchestration/agent_hooks.rs`
  - `crates/commands/validation/src/operational_hygiene/shell_hooks.rs`
  - `crates/commands/validation/src/operational_hygiene/runtime_script_tests.rs`
- Current Slice Snapshot:
  - `scripts/**/*.ps1` inventory fell from `112` to `109`
  - `scripts/validation/*.ps1` inventory fell from `8` to `5`
  - native Rust owners and executable CLI boundaries now exist for all three target checks
  - the remaining consumer chain no longer requires the three local wrapper paths

## Scope Summary

1. `scripts/validation/validate-agent-hooks.ps1`
2. `scripts/validation/validate-shell-hooks.ps1`
3. `scripts/validation/validate-runtime-script-tests.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for all three checks
- `validate-all.ps1` no longer shells into the three local wrappers
- governance baselines, runtime parity harnesses, and authored guidance stop treating the deleted `.ps1` files as canonical
- the three wrappers are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Freeze The Phase-13 Contract

Status: `[x]` Completed

- Lock the phase-13 design intent in the spec and confirm the acceptance criteria.
- Confirm the native Rust owners and exact CLI contracts for:
  - `agent-hooks`
  - `shell-hooks`
  - `runtime-script-tests`

### Task 2: Add The Missing Native CLI Boundaries

Status: `[x]` Completed

- Extend `ntk validation` with executable native contracts for the three checks.
- Keep CLI arguments aligned with the existing behavior:
  - `agent-hooks`: repo root, warning-only
  - `shell-hooks`: repo root, warning-only, enable-shellcheck
  - `runtime-script-tests`: repo root, warning-only
- Add or update focused CLI tests for each new subcommand.

### Task 3: Repoint Consumers And Delete The Three Leaves

Status: `[x]` Completed

- Repoint live consumers that still encode the wrapper paths:
  - `scripts/validation/validate-all.ps1`
  - `scripts/orchestration/stages/validate-stage.ps1`
  - `scripts/tests/runtime/agent-orchestration-engine.tests.ps1`
  - `crates/orchestrator/tests/execution/pipeline_parity/support/fake_codex_runner.rs`
  - `definitions/providers/codex/scripts/README.md`
  - `.github/governance/release-governance.md`
  - `.github/governance/security-baseline.json`
  - `.github/governance/release-provenance.baseline.json`
  - `.github/policies/instruction-system.policy.json`
- Delete the three wrappers only after the consumer chain is cut over.

### Task 4: Rebaseline Planning Evidence And Archive The Phase

Status: `[x]` Completed

- Updated `planning/completed/script-retirement-safety-matrix.md`.
- Updated `planning/completed/rust-script-parity-ledger.md`.
- Moved the plan/spec to completed after the validations and inventory updates landed.

## Execution Outcome

- Added native `ntk validation` executable boundaries for:
  - `agent-hooks`
  - `shell-hooks`
  - `runtime-script-tests`
- Repointed:
  - `scripts/validation/validate-all.ps1`
  - `scripts/orchestration/stages/validate-stage.ps1`
  - `scripts/tests/runtime/agent-orchestration-engine.tests.ps1`
  - `crates/orchestrator/tests/execution/pipeline_parity/support/fake_codex_runner.rs`
  - `definitions/providers/codex/scripts/README.md`
  - `.codex/scripts/README.md`
  - `.github/governance/release-governance.md`
  - `.github/governance/security-baseline.json`
  - `.github/governance/release-provenance.baseline.json`
  - `.github/policies/instruction-system.policy.json`
  - `crates/commands/validation/tests/support/security_fixtures.rs`
  - `crates/commands/validation/tests/security/security_baseline_tests.rs`
  - `crates/cli/tests/validation_commands_tests.rs`
- Deleted:
  - `scripts/validation/validate-agent-hooks.ps1`
  - `scripts/validation/validate-shell-hooks.ps1`
  - `scripts/validation/validate-runtime-script-tests.ps1`
- Re-rendered the Codex compatibility surfaces after the authored provider guidance changed.
- Confirmed the local `scripts/**/*.ps1` estate fell from `112` to `109`.
- Confirmed the local `scripts/validation/*.ps1` estate fell from `8` to `5`.
- Focused `validate-all` proof for this phase passed in warning-only mode with all three checks routed through the native executable contract.
- Full repository enforcing validations remain blocked by pre-existing governance and runtime-test debt rather than by the phase-13 cutover itself.

## Validation Checklist

- `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- `pwsh -NoProfile -File .\scripts\validation\validate-instructions.ps1`
- `pwsh -NoProfile -File .\scripts\tests\runtime\agent-orchestration-engine.tests.ps1`
- `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- focused `validate-all.ps1` proof for the three phase-13 checks
- `git diff --check`

## Risks And Fallbacks

- `validate-instructions.ps1` is still red for pre-existing repository policy debt such as missing `CODEOWNERS`, issue templates, workflow files, and managed `.githooks/*` surfaces; that debt is independent of the three wrapper deletions in this phase.
- `agent-orchestration-engine.tests.ps1` still exposes broader runtime harness debt because parts of the retained runtime test suite expect `shared-scripts/common/common-bootstrap.ps1`; Phase 13 only moved the local validation launch surface to the native executable contract.
- The runtime parity harness remains a compatibility launch surface by policy; this phase only changed the internal executor contract from local PowerShell wrappers to the native `ntk validation` binary.

## Closeout Expectations

- This plan is now archived because the wrapper deletions and focused validations are materially complete.