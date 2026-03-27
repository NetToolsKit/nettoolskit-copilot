# Rust Script Parity Ledger

Generated: 2026-03-27 20:01

## Status

- LastUpdated: 2026-03-27 20:01
- Objective: define the parity evidence model that every PowerShell migration slice must satisfy before wrapper cutover.
- Source Plan: `planning/active/plan-repository-unification-and-rust-migration.md`
- Supporting Matrix: `planning/active/rust-script-transcription-ownership-matrix.md`
- Active Branch: `feature/native-validation-policy`
- Live parity harness: `approval-approved-test`, the staged `run-test` closeout success path, `evaluate-agent-pipeline`, and `resume-agent-pipeline` are now covered by the native orchestrator harness in `crates/orchestrator/tests/execution/pipeline_parity`

## Evidence Policy

1. A migration slice is not parity-complete until the owning Rust crate has deterministic tests for the migrated behavior.
2. Wrapper cutover is not allowed with compile-only evidence; each migrated domain also needs at least one operator-path smoke check.
3. Validation-owned domains must preserve or improve current policy severity behavior before the PowerShell logic can be retired.
4. Test-owned domains must replace the PowerShell harness with Rust-native assertions in the owning crates or a root integration harness.
5. Docs and CHANGELOG updates are part of parity for user-visible or operator-visible command changes.

## Domain Ledger

| Legacy Scope | Count | Rust Owner | Required Evidence | Current Status |
| --- | ---: | --- | --- | --- |
| `scripts/common/*.ps1` | 15 | `crates/core` | unit tests for path/catalog/runtime helpers plus fixture-based parity assertions | `owner locked, implementation pending` |
| `scripts/runtime/*.ps1` excluding hooks | 42 | `crates/commands/runtime + crates/cli` | command-contract tests, CLI smoke checks, wrapper delegation proof | `boundary crate created` |
| `scripts/runtime/hooks/*.ps1` | 4 | `crates/commands/runtime + crates/orchestrator` | hook contract tests, orchestration integration tests, local hook dispatch smoke | `boundary crate created` |
| `scripts/maintenance/*.ps1` | 5 | `crates/commands/runtime` | command tests plus filesystem fixture validation for mutation-heavy flows | `boundary crate created` |
| `scripts/validation/*.ps1` | 31 | `crates/commands/validation` | policy tests, fixture validation, severity-preservation assertions | `boundary crate created` |
| `scripts/security/*.ps1` | 6 | `crates/commands/validation` | security gate tests, tool invocation contracts, failure-path assertions | `boundary crate created` |
| `scripts/governance/*.ps1` | 2 | `crates/commands/validation` | governance contract tests plus safe no-op / denied-path assertions | `boundary crate created` |
| `scripts/doc/*.ps1` | 1 | `crates/commands/validation` | documentation validation tests against fixture docs | `boundary crate created` |
| `scripts/deploy/*.ps1` | 1 | `crates/commands/validation` | deploy preflight tests and protected execution-path assertions | `boundary crate created` |
| `scripts/orchestration/**/*.ps1` | 10 | `crates/orchestrator` | staged execution tests, resume/replay assertions, dispatch integration checks | `wave 3 control-plane parity proven` |
| `scripts/git-hooks/*.ps1` | 3 | `crates/commands/runtime` | git hook install/check tests plus local hook bootstrap smoke | `boundary crate created` |
| `scripts/tests/*.ps1` excluding runtime subfolder | 4 | `crate test suites + root parity harness` | Rust-native replacements for coverage/test-shape automation | `parity harness planning locked` |
| `scripts/tests/runtime/*.ps1` | 23 | `crate test suites + root parity harness` | root integration harness plus owning-crate assertions for each replaced runtime test | `wave 3 parity baseline proven (approval-approved, run-test closeout, evaluate, resume)` |

## Acceptance Gate

- `Ready for wrapper cutover`: Rust implementation exists, deterministic tests pass, operator smoke checks pass, docs are updated, and the domain row is updated from `implementation pending` or `planning locked` to `parity proven`.
- `Not ready for wrapper cutover`: any missing Rust test surface, missing smoke evidence, missing docs update, or unresolved policy regression.