# Script Retirement Safety Matrix

Generated: 2026-03-28 18:05

## Status

- LastUpdated: 2026-03-28 17:18
- Objective: record the live deletion-readiness state for the local `scripts/**/*.ps1` estate after the completed Rust migration bundle.
- Baseline Inventory: `147` PowerShell files from `scripts/**/*.ps1`
- Live Inventory After Executed Slice: `141`
- Current Classification Totals:
  - `retired in this workstream`: `6`
  - `retain wrapper intentionally`: `33`
  - `retain until consumer migration completes`: `108`
- Decision Rule:
  - `remove-now candidate` means Rust parity exists and no blocking local consumer remains after same-slice doc cleanup.
  - `retain wrapper intentionally` means the script stays by policy even when Rust owns the underlying behavior.
  - `retain until consumer migration completes` means Rust parity exists, but local runtime, validation, doc, or test consumers still encode the `.ps1` path.

## Executed Retirement Slice

| Scope | Count | Rust Owner | Local Consumer Evidence | Same-Slice Update | Result |
| --- | ---: | --- | --- | --- | --- |
| `scripts/doc/validate-xml-documentation.ps1` | 1 | `crates/commands/validation` | No non-self local `.ps1` consumer remained; local references were already Rust-native through `validate-all` and validation profiles. | None | retired |
| `scripts/maintenance/fix-version-ranges.ps1` | 1 | `crates/commands/runtime` | No non-self local `.ps1` consumer remained; runtime ownership was already native in `crates/commands/runtime/src/maintenance/fix_version_ranges.rs`. | None | retired |
| `scripts/maintenance/fix-region-spacing.ps1` | 1 | `crates/commands/runtime` | No non-self local `.ps1` consumer remained; runtime ownership was already native in `crates/commands/runtime/src/maintenance/fix_region_spacing.rs`. | None | retired |
| `scripts/maintenance/clean-build-artifacts.ps1` | 1 | `crates/commands/runtime` | Only blocker was the artifact-layout guidance that named the wrapper directly. | Updated `.github/instructions/artifact-layout.instructions.md` and `definitions/shared/instructions/artifact-layout.instructions.md`. | retired |
| `scripts/tests/check-test-naming.ps1` | 1 | `crates/commands/validation` | The only live blockers were `crates/commands/validation/src/contracts.rs` and `crates/commands/validation/tests/contracts_tests.rs`; both were cleared in the same slice. | Removed the legacy validation surface contract entries and deleted the wrapper. | retired |
| `scripts/tests/refactor_tests_to_aaa.ps1` | 1 | `crates/commands/validation` | The only live blockers were `crates/commands/validation/src/contracts.rs` and `crates/commands/validation/tests/contracts_tests.rs`; both were cleared in the same slice. | Removed the legacy validation surface contract entries and deleted the wrapper. | retired |

## Retained Wrappers By Policy

| Scope | Count | Retention Basis | Policy Evidence |
| --- | ---: | --- | --- |
| `scripts/runtime/hooks/common.ps1`, `scripts/runtime/hooks/session-start.ps1`, `scripts/runtime/hooks/subagent-start.ps1` | 3 | shell-owned startup and workspace bootstrap glue | `planning/completed/rust-script-cutover-default-map.md` marks `scripts/runtime/hooks` as `legacy integration wrapper retained intentionally` with only `pre-tool-use` migrated natively |
| `scripts/maintenance/generate-http-from-openapi.ps1` | 1 | external OpenAPI toolchain wrapper remains approved | `planning/completed/rust-script-cutover-default-map.md` marks this as the single retained maintenance exception |
| `scripts/deploy/deploy-backend-to-vps.ps1` | 1 | operational SSH/SCP executor retained intentionally | `planning/completed/rust-script-cutover-default-map.md` marks `scripts/deploy` as `compatibility wrapper retained intentionally` |
| `scripts/git-hooks/*.ps1` | 3 | Git integration entrypoints remain compatibility surfaces | `planning/completed/rust-script-cutover-default-map.md` marks `scripts/git-hooks` as `compatibility wrapper retained intentionally` |
| `scripts/tests/apply-aaa-pattern.ps1`, `scripts/tests/run-coverage.ps1` | 2 | explicit non-runtime test wrapper exceptions | `planning/completed/rust-script-cutover-default-map.md` marks these as `legacy integration wrapper retained intentionally` |
| `scripts/tests/runtime/*.ps1` | 23 | runtime parity harness remains canonical while wrapper launchers stay for operator compatibility | `planning/completed/rust-script-cutover-default-map.md` marks `scripts/tests/runtime` as `compatibility wrapper retained intentionally` |

## Retain Until Consumer Migration Completes

| Scope | Count | Blocking Reason | Blocking Evidence |
| --- | ---: | --- | --- |
| `scripts/common/*.ps1` | 15 | completed Rust ownership exists, but this audit has not yet proven zero local consumers for the full domain | requires follow-up consumer sweep before deletion |
| `scripts/runtime/*.ps1` excluding hooks | 42 | completed Rust ownership exists, but this audit has not yet proven zero local consumers for the full domain | requires follow-up consumer sweep before deletion |
| `scripts/validation/*.ps1` | 31 | completed Rust ownership exists, but this audit has not yet proven zero local consumers for the full domain | requires follow-up consumer sweep before deletion |
| `scripts/security/*.ps1` | 6 | shared-script governance still tracks this domain as a pinned script surface | `.github/governance/shared-script-checksums.manifest.json` includes `scripts/security` |
| `scripts/governance/*.ps1` | 2 | completed Rust ownership exists, but this audit has not yet proven zero local consumers for the full domain | requires follow-up consumer sweep before deletion |
| `scripts/orchestration/**/*.ps1` | 10 | completed Rust ownership exists, but this audit has not yet proven zero local consumers for the full domain | requires follow-up consumer sweep before deletion |
| `scripts/runtime/hooks/pre-tool-use.ps1` | 1 | hook contract and validation fixtures still depend on the script name/path | `.github/hooks/super-agent.bootstrap.json`, `definitions/providers/github/hooks/super-agent.bootstrap.json`, `crates/commands/validation/src/agent_orchestration/agent_hooks.rs`, `crates/commands/validation/tests/support/agent_orchestration_fixtures.rs`, `crates/commands/validation/tests/agent_orchestration/agent_hooks_tests.rs` |
| `scripts/maintenance/trim-trailing-blank-lines.ps1` | 1 | runtime hook setup and parity tests still depend on the script path | `scripts/git-hooks/*.ps1`, `crates/commands/runtime/src/hooks/setup_git_hooks.rs`, `crates/commands/runtime/src/hooks/setup_global_git_aliases.rs`, `crates/commands/runtime/tests/hooks/*.rs`, `scripts/tests/runtime/*trim-trailing-blank-lines*` |
## Current Immediate Queue

- No additional leaf is promoted to `remove-now candidate` yet.
- The next consumer sweep is explicitly narrowed to:
  - `scripts/runtime/hooks/pre-tool-use.ps1`
  - `scripts/maintenance/trim-trailing-blank-lines.ps1`

## Notes

- The completed migration bundle proves Rust ownership for much more than the first four leaves, but this matrix intentionally separates parity evidence from deletion safety.
- `114` previously broad `Rust-default now` leaves were narrowed to `4` immediate deletion candidates for the first patch because this audit requires concrete local-consumer proof before removal.
- The first execution slice retired those `4` leaves and reduced the live local `scripts/**/*.ps1` estate from `147` to `143`.
- The second execution slice retired the validation-owned test automation wrappers `check-test-naming` and `refactor_tests_to_aaa`, reducing the live local estate from `143` to `141`.
- No domain should move from `retain until consumer migration completes` to `remove-now candidate` without the same kind of exact local consumer evidence used above.
- The remaining backlog is intentionally left for future domain-level consumer-migration workstreams rather than being forced into this audit closeout.