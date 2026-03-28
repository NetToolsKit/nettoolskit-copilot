# Rust Script Parity Ledger

Generated: 2026-03-27 20:01

## Status

- LastUpdated: 2026-03-28 16:10
- Objective: define the parity evidence model and the current closeout status that every PowerShell migration domain must satisfy before wrapper cutover.
- Source Plan: `planning/active/plan-repository-unification-and-rust-migration.md`
- Supporting Matrix: `planning/active/rust-script-transcription-ownership-matrix.md`
- Cutover Map: `planning/active/rust-script-cutover-default-map.md`
- Active Branch: `feature/native-validation-policy`
- Remaining Open Backlog: explicit retained wrapper policy only; no remaining execution backlog
- Live parity harness: `approval-approved-test`, the staged `run-test` closeout success path, `evaluate-agent-pipeline`, and `resume-agent-pipeline` are now covered by the native orchestrator harness in `crates/orchestrator/tests/execution/pipeline_parity`
- Parity cleanup note: tracked repository artifacts now restore through Git-backed recovery, and projected POML assets are restored explicitly after parity runs so the worktree returns to a clean baseline deterministically.
- Current closeout stance: the explicit cutover map now records which parity-proven domains are Rust-default now, intentionally wrapper-retained, or retained as legacy integration wrappers; the parity ledger still avoids promoting a domain to `cutover ready` by implication alone.
- Closeout board semantics:
  - `parity proven`: native Rust behavior and deterministic evidence exist; the cutover map then decides whether the domain is Rust-default now or wrapper retained intentionally
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
| `scripts/common/*.ps1` | 15 | `crates/core` | unit tests for path/catalog/runtime helpers plus fixture-based parity assertions | `parity proven` | Shared helper foundations are implemented and covered, and the cutover map now marks this domain Rust-default now. |
| `scripts/runtime/*.ps1` excluding hooks | 42 | `crates/commands/runtime + crates/cli` | command-contract tests, CLI smoke checks, wrapper delegation proof | `parity proven` | Wave 1 runtime surface is implemented in Rust, docs present PowerShell as a compatibility layer, and the cutover map now marks this domain Rust-default now. |
| `scripts/runtime/hooks/*.ps1` | 4 | `crates/commands/runtime + crates/orchestrator` | hook contract tests, orchestration integration tests, local hook dispatch smoke | `evidence gap remains` | `pre-tool-use` now has a native Rust hook boundary with dedicated tests, while `common`, `session-start`, and `subagent-start` are now recorded in the cutover map as explicit retained startup wrappers for the VS Code/Codex hook contract. |
| `scripts/maintenance/*.ps1` | 5 | `crates/commands/runtime` | command tests plus filesystem fixture validation for mutation-heavy flows | `evidence gap remains` | `clean-build-artifacts`, `trim-trailing-blank-lines`, `fix-region-spacing`, and `fix-version-ranges` now run natively with direct tests; `generate-http-from-openapi` remains an explicit retained generator wrapper around the external OpenAPI reader toolchain. |
| `scripts/validation/*.ps1` | 31 | `crates/commands/validation` | policy tests, fixture validation, severity-preservation assertions | `parity proven` | Wave 2 validation orchestration and per-check native surfaces are recorded as complete, and the cutover map now marks this domain Rust-default now. |
| `scripts/security/*.ps1` | 6 | `crates/commands/validation` | security gate tests, tool invocation contracts, failure-path assertions | `parity proven` | Native security policy coverage is recorded in the validation crate, and the cutover map now marks this domain Rust-default now. |
| `scripts/governance/*.ps1` | 2 | `crates/commands/validation` | governance contract tests plus safe no-op / denied-path assertions | `parity proven` | Governance checks are implemented natively through the validation surface, and the cutover map now marks this domain Rust-default now. |
| `scripts/doc/*.ps1` | 1 | `crates/commands/validation` | documentation validation tests against fixture docs | `parity proven` | `validate-xml-documentation` now exists natively in `crates/commands/validation/documentation`, is routed through `validate-all`, and the cutover map now marks this domain Rust-default now. |
| `scripts/deploy/*.ps1` | 1 | `crates/commands/validation` | deploy preflight tests and protected execution-path assertions | `parity proven` | `validate-deploy-preflight` now exists natively in `crates/commands/validation/deploy`, runs through `validate-all`, and the PowerShell deploy executor is intentionally retained as a compatibility wrapper. |
| `scripts/orchestration/**/*.ps1` | 10 | `crates/orchestrator` | staged execution tests, resume/replay assertions, dispatch integration checks | `parity proven` | Wave 3 control-plane parity is implemented and recorded, and the cutover map now marks this domain Rust-default now. |
| `scripts/git-hooks/*.ps1` | 3 | `crates/commands/runtime` | git hook install/check tests plus local hook bootstrap smoke | `parity proven` | Native git hook install and EOF hygiene coverage exists, but the hook surface remains a compatibility wrapper intentionally retained in the cutover map. |
| `scripts/tests/check-test-naming.ps1` | 1 | `crates/commands/validation` | validation-fixture coverage plus `validate-all` profile routing | `parity proven` | `validate-test-naming` now exists natively in `crates/commands/validation/operational_hygiene`, is tracked in the validation surface contracts, and runs through repository validation profiles. |
| `scripts/tests/refactor_tests_to_aaa.ps1` | 1 | `crates/commands/validation` | direct command tests plus validation-surface contract coverage | `parity proven` | `refactor_tests_to_aaa` now exists natively in `crates/commands/validation/operational_hygiene`, has dedicated Rust tests, and is locked in the validation surface catalog. |
| `scripts/tests/*.ps1` excluding `check-test-naming.ps1`, `refactor_tests_to_aaa.ps1`, and runtime subfolder | 2 | `crate test suites + root parity harness` | Rust-native replacements for coverage/test-shape automation | `evidence gap remains` | The remaining two scripts, `apply-aaa-pattern` and `run-coverage`, are now explicit retained wrapper exceptions in the cutover map rather than unresolved migration slices. |
| `scripts/tests/runtime/*.ps1` | 23 | `crate test suites + root parity harness` | root integration harness plus owning-crate assertions for each replaced runtime test | `parity proven` | The native parity harness covers `approval-approved`, staged `run-test` closeout, `evaluate-agent-pipeline`, and `resume-agent-pipeline`; the wrapper surface is retained intentionally as a compatibility launch surface, and parity cleanup is now stable. |

## Acceptance Gate

- `Ready for wrapper cutover`: Rust implementation exists, deterministic tests pass, operator smoke checks pass, docs are updated, and the domain row is updated from `implementation pending` or `planning locked` to `parity proven`.
- `Not ready for wrapper cutover`: any missing Rust test surface, missing smoke evidence, missing docs update, or unresolved policy regression.