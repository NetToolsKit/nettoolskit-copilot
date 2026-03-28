# Rust Script Parity Ledger

Generated: 2026-03-27 20:01

## Status

- LastUpdated: 2026-03-28 11:45
- Objective: define the parity evidence model and the current closeout status that every PowerShell migration domain must satisfy before wrapper cutover.
- Source Plan: `planning/active/plan-repository-unification-and-rust-migration.md`
- Supporting Matrix: `planning/active/rust-script-transcription-ownership-matrix.md`
- Active Branch: `feature/native-validation-policy`
- Remaining Open Backlog: `planning/active/plan-rust-migration-closeout-and-cutover.md`
- Live parity harness: `approval-approved-test`, the staged `run-test` closeout success path, `evaluate-agent-pipeline`, and `resume-agent-pipeline` are now covered by the native orchestrator harness in `crates/orchestrator/tests/execution/pipeline_parity`
- Closeout board semantics:
  - `parity proven`: native Rust behavior and deterministic evidence exist, but wrapper/default cutover may still be pending
  - `cutover ready`: parity is proven and the domain is ready to become Rust-default
  - `wrapper retained intentionally`: parity exists, but the legacy wrapper remains part of the approved operating model
  - `evidence gap remains`: the owner boundary exists, but the recorded closeout evidence is still incomplete

## Evidence Policy

1. A migration slice is not parity-complete until the owning Rust crate has deterministic tests for the migrated behavior.
2. Wrapper cutover is not allowed with compile-only evidence; each migrated domain also needs at least one operator-path smoke check.
3. Validation-owned domains must preserve or improve current policy severity behavior before the PowerShell logic can be retired.
4. Test-owned domains must replace the PowerShell harness with Rust-native assertions in the owning crates or a root integration harness.
5. Docs and CHANGELOG updates are part of parity for user-visible or operator-visible command changes.

## Domain Ledger

| Legacy Scope | Count | Rust Owner | Required Evidence | Current Status | Closeout Notes |
| --- | ---: | --- | --- | --- |
| `scripts/common/*.ps1` | 15 | `crates/core` | unit tests for path/catalog/runtime helpers plus fixture-based parity assertions | `parity proven` | Shared helper foundations are implemented and covered, but wrapper/default cutover remains pending in Task 8. |
| `scripts/runtime/*.ps1` excluding hooks | 42 | `crates/commands/runtime + crates/cli` | command-contract tests, CLI smoke checks, wrapper delegation proof | `parity proven` | Wave 1 runtime surface is implemented in Rust; wrapper/default promotion still depends on the closeout package. |
| `scripts/runtime/hooks/*.ps1` | 4 | `crates/commands/runtime + crates/orchestrator` | hook contract tests, orchestration integration tests, local hook dispatch smoke | `evidence gap remains` | Hook-adjacent validation and orchestration coverage exist, but the runtime hook scripts themselves are not yet recorded as cutover-ready. |
| `scripts/maintenance/*.ps1` | 5 | `crates/commands/runtime` | command tests plus filesystem fixture validation for mutation-heavy flows | `evidence gap remains` | Maintenance ownership is locked, but the ledger still lacks explicit native replacement evidence for the full mutation-heavy cluster. |
| `scripts/validation/*.ps1` | 31 | `crates/commands/validation` | policy tests, fixture validation, severity-preservation assertions | `parity proven` | Wave 2 validation orchestration and per-check native surfaces are recorded as complete. |
| `scripts/security/*.ps1` | 6 | `crates/commands/validation` | security gate tests, tool invocation contracts, failure-path assertions | `parity proven` | Native security policy coverage is recorded in the validation crate; cutover/default ownership is still pending. |
| `scripts/governance/*.ps1` | 2 | `crates/commands/validation` | governance contract tests plus safe no-op / denied-path assertions | `parity proven` | Governance checks are implemented natively through the validation surface. |
| `scripts/doc/*.ps1` | 1 | `crates/commands/validation` | documentation validation tests against fixture docs | `evidence gap remains` | Documentation validation is part of Wave 2, but the closeout ledger does not yet record an explicit native replacement for the doc-only script domain. |
| `scripts/deploy/*.ps1` | 1 | `crates/commands/validation` | deploy preflight tests and protected execution-path assertions | `evidence gap remains` | Deploy-preflight ownership is locked, but explicit closeout evidence for the deploy domain is still missing. |
| `scripts/orchestration/**/*.ps1` | 10 | `crates/orchestrator` | staged execution tests, resume/replay assertions, dispatch integration checks | `parity proven` | Wave 3 control-plane parity is implemented and recorded; wrapper/default cutover remains a Task 8 decision. |
| `scripts/git-hooks/*.ps1` | 3 | `crates/commands/runtime` | git hook install/check tests plus local hook bootstrap smoke | `parity proven` | Native git hook install and EOF hygiene coverage exists; wrapper/default promotion remains pending. |
| `scripts/tests/*.ps1` excluding runtime subfolder | 4 | `crate test suites + root parity harness` | Rust-native replacements for coverage/test-shape automation | `evidence gap remains` | The runtime parity harness is real, but the non-runtime test automation scripts still lack explicit closeout evidence. |
| `scripts/tests/runtime/*.ps1` | 23 | `crate test suites + root parity harness` | root integration harness plus owning-crate assertions for each replaced runtime test | `parity proven` | The native parity harness covers `approval-approved`, staged `run-test` closeout, `evaluate-agent-pipeline`, and `resume-agent-pipeline`. |

## Acceptance Gate

- `Ready for wrapper cutover`: Rust implementation exists, deterministic tests pass, operator smoke checks pass, docs are updated, and the domain row is updated from `implementation pending` or `planning locked` to `parity proven`.
- `Not ready for wrapper cutover`: any missing Rust test surface, missing smoke evidence, missing docs update, or unresolved policy regression.