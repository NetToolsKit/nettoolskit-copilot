# Phase 9: Validation Wrapper Retirement - Agent Orchestration and Release Governance

Generated: 2026-03-28 21:41

## Status

- LastUpdated: 2026-03-28 21:41
- Objective: retire the remaining low-fanout validation wrappers by exposing native `ntk validation` executable boundaries for `agent-orchestration`, `release-governance`, and `release-provenance`, then delete the local PowerShell leaves after the documentation and orchestration consumers stop requiring their paths.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keep planning current, and commit each stable retirement phase.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/active/spec-script-retirement-phase-9.md`
- Inputs:
  - `planning/completed/plan-script-retirement-phase-8.md`
  - `planning/specs/completed/spec-script-retirement-phase-8.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `definitions/providers/codex/orchestration/README.md`
  - `definitions/providers/codex/skills/privacy-compliance-engineer/SKILL.md`
  - `definitions/providers/claude/skills/privacy-compliance-engineer/SKILL.md`
  - `definitions/providers/claude/skills/sec-security-vulnerability-engineer/SKILL.md`
- Current Slice Snapshot:
  - `scripts/**/*.ps1` inventory is `124`
  - native Rust validation owners already exist for the three target checks
  - local wrapper paths still appear in planning documentation and provider-authored examples

## Scope Summary

1. `scripts/validation/validate-agent-orchestration.ps1`
2. `scripts/validation/validate-release-governance.ps1`
3. `scripts/validation/validate-release-provenance.ps1`

This phase is complete only if:

- `ntk validation` exposes executable native contracts for each in-scope check
- `validate-all.ps1`, `validate-stage.ps1`, `validate-instructions.ps1`, and `run-agent-pipeline.ps1` no longer require the local wrapper paths for those checks
- policy, inventory, and provider-authored documentation stop treating the deleted `.ps1` files as required local evidence
- the three wrappers are deleted safely and the retirement matrix/parity ledger record the executed result

## Ordered Tasks

### Task 1: Freeze The Phase-9 Native Validation Contract

Status: `[2026-03-28 21:41]`

- Register the phase-9 design intent in the spec and lock the cutover acceptance criteria.
- Confirm the native Rust owners and the exact CLI contract for each of:
  - `agent-orchestration`
  - `release-governance`
  - `release-provenance`
- Target paths:
  - `planning/specs/active/spec-script-retirement-phase-9.md`
  - `planning/active/plan-script-retirement-phase-9.md`
- Validation checkpoints:
  - `rg -n "validate-agent-orchestration|validate-release-governance|validate-release-provenance" crates/cli/src/validation_commands.rs crates/commands/validation/src`
  - `git diff --check`
- Commit checkpoint suggestion:
  - `docs(planning): open script retirement phase 9`

### Task 2: Repoint Docs And Provider Examples To Native Commands

Status: `[2026-03-28 21:41]`

- Update provider-authored orchestration and security docs to reference `ntk validation` instead of retired wrapper paths.
- Keep the docs aligned with the native validation contract while leaving execution details for the implementation phase.
- Target paths:
  - `definitions/providers/codex/orchestration/README.md`
  - `definitions/providers/codex/skills/privacy-compliance-engineer/SKILL.md`
  - `definitions/providers/claude/skills/privacy-compliance-engineer/SKILL.md`
  - `definitions/providers/claude/skills/sec-security-vulnerability-engineer/SKILL.md`
- Validation checkpoints:
  - `rg -n "validate-agent-orchestration.ps1|validate-release-governance.ps1|validate-release-provenance.ps1" definitions/providers`
  - `git diff --check`
- Commit checkpoint suggestion:
  - `docs(provider): align release and orchestration examples with ntk validation`

### Task 3: Retire The Remaining Validation Wrappers

Status: `[2026-03-28 21:41]`

- Repoint the live consumers that still hardcode wrapper paths:
  - `scripts/validation/validate-all.ps1`
  - `scripts/orchestration/stages/validate-stage.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/runtime/run-agent-pipeline.ps1`
- Delete:
  - `scripts/validation/validate-agent-orchestration.ps1`
  - `scripts/validation/validate-release-governance.ps1`
  - `scripts/validation/validate-release-provenance.ps1`
- Target paths:
  - `crates/cli/src/validation_commands.rs`
  - `crates/cli/tests/validation_commands_tests.rs`
  - `crates/commands/validation/src/agent_orchestration/orchestration_integrity.rs`
  - `crates/commands/validation/src/release/release_governance.rs`
  - `crates/commands/validation/src/release/release_provenance.rs`
  - `scripts/validation/validate-all.ps1`
  - `scripts/orchestration/stages/validate-stage.ps1`
  - `scripts/validation/validate-instructions.ps1`
  - `scripts/runtime/run-agent-pipeline.ps1`
- Validation checkpoints:
  - `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
  - `cargo test -p nettoolskit-validation --quiet`
  - focused `validate-all.ps1` profiles for the three checks
  - `pwsh -File .\scripts\validation\validate-instructions.ps1`
  - `git diff --check`
- Commit checkpoint suggestion:
  - `refactor(validation): retire agent orchestration and release governance wrappers`

### Task 4: Rebaseline Planning Evidence And Archive The Phase

Status: `[2026-03-28 21:41]`

- Update the completed matrix and parity ledger with the executed phase-9 result.
- Archive the phase-9 plan and spec only after the implementation, validation, and closeout evidence is complete.
- Target paths:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/README.md`
  - `planning/specs/README.md`
- Validation checkpoints:
  - `git diff --check`
  - final smoke runs from the implementation phase
- Commit checkpoint suggestion:
  - `docs(planning): close script retirement phase 9`

## Validation Checklist

- `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- targeted crate/CLI tests for:
  - `agent-orchestration`
  - `release-governance`
  - `release-provenance`
- `pwsh -File .\scripts\validation\validate-instructions.ps1`
- `pwsh -File .\scripts\validation\validate-planning-structure.ps1`
- `pwsh -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- targeted `validate-all.ps1` phase profile for:
  - `validate-agent-orchestration`
  - `validate-release-governance`
  - `validate-release-provenance`
- `git diff --check`

## Risks And Fallbacks

- The live orchestration scripts still hardcode wrapper paths, so the implementation slice must update those consumers before deletion.
- Provider docs and skill examples can drift independently from code; if they are not updated in the same slice, the retirement may look complete while still advertising retired wrappers.
- Release governance files may still name the wrappers as required evidence; if that surfaces during implementation, keep the wrappers until the documentation evidence is converted to native Rust paths.
- If the native CLI contract changes while the phase is active, update the spec first and then revise the plan so task ordering stays deterministic.

## Recommended Specialist

- Primary: `dev-rust-engineer`
- Secondary: `test-engineer`
- Documentation and closeout: `docs-release-engineer`
- Review: `review-code-engineer`

## Closeout Expectations

- Update `planning/README.md` and `planning/specs/README.md` when the active phase is created and again when it is archived.
- Use an English commit message aligned to the retirement slice, for example: `refactor(validation): retire agent orchestration and release governance wrappers`.
- Do not add a changelog entry for this planning artifact alone; add release notes only when the implementation phase lands user-visible behavior.
- Archive this plan and its spec only after the wrappers are actually removed and the replacement references have been validated.

## Worktree Guidance

- Isolated worktree execution is recommended for the implementation phase because this slice touches validation, orchestration, and provider documentation simultaneously.