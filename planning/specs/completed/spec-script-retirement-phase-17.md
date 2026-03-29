# Spec: Phase 17 Runtime Diagnostics Wrapper Retirement

Generated: 2026-03-29 08:01

## Status

- LastUpdated: 2026-03-29 08:26
- Objective: define the cutover intent and safe deletion conditions for replacing the local runtime diagnostics wrappers with native `ntk runtime` diagnostics commands.
- Planning Readiness: executed-and-completed
- Related Plan: `planning/completed/plan-script-retirement-phase-17.md`
- Source Inputs:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `scripts/runtime/doctor.ps1`
  - `scripts/runtime/healthcheck.ps1`
  - `crates/commands/runtime/src/diagnostics/doctor.rs`
  - `crates/commands/runtime/src/diagnostics/healthcheck.rs`

## Problem Statement

After Phase 16, the validation folder is fully retired locally, but two repository-owned runtime diagnostics wrappers still remain in `scripts/runtime/`: `doctor.ps1` and `healthcheck.ps1`. Their native behavior already exists in the runtime crate, yet the local wrapper paths are still treated as canonical by shell-owned runtime scripts, runbooks, README surfaces, skill docs, and retained parity fixtures.

## Desired Outcome

- `ntk runtime doctor` becomes the canonical drift-diagnosis entrypoint.
- `ntk runtime healthcheck` remains the canonical end-to-end runtime and validation health entrypoint.
- Local runtime consumers and authored/operator-facing surfaces stop requiring `scripts/runtime/doctor.ps1` and `scripts/runtime/healthcheck.ps1`.
- Both wrappers are deleted locally in the same slice.

## Design Decision

Expose `doctor` through `crates/cli/src/runtime_commands.rs` and route both diagnostics commands through the existing `nettoolskit-runtime` boundary. Repoint the remaining shell-owned consumers to the native binary instead of introducing another compatibility wrapper.

## Alternatives Considered

1. Keep `doctor.ps1` and `healthcheck.ps1` as intentional compatibility wrappers
   - Rejected because the underlying behavior is already repository-owned in Rust and the wrappers are no longer shell-only glue.
2. Retire `doctor.ps1` first and keep `healthcheck.ps1` for a later phase
   - Rejected because the remaining healthcheck wrapper would still preserve the old consumer chain and leave the diagnostics surface split unnecessarily.
3. Retire both diagnostics wrappers in one phase
   - Selected because `healthcheck` already exists natively and `doctor` only needs a CLI surface plus live-consumer repoints.

## Risks

- `install.ps1` currently models install steps as script invocations, so binary-backed diagnostics execution must be introduced without regressing existing steps.
- Some retained parity fixtures are still shell-based and may need contract-level updates rather than simple string replacements.
- Runbook and skill examples must remain copy-pasteable for operators after the cutover.

## Acceptance Criteria

- `ntk runtime doctor` exists and exposes the native drift-diagnosis flow.
- `ntk runtime healthcheck` remains the canonical runtime health surface and the deleted wrapper path is no longer required.
- `install.ps1`, `self-heal.ps1`, and `validate-stage.ps1` stop requiring the deleted diagnostics wrapper paths.
- Runbooks, README surfaces, authored skills, and policy evidence no longer require `scripts/runtime/doctor.ps1` or `scripts/runtime/healthcheck.ps1`.
- The safety matrix and parity ledger record the inventory reduction from `104` to `102` total scripts.

## Executed Result

- `crates/cli/src/runtime_commands.rs` now exposes `ntk runtime doctor` directly over the native runtime crate boundary.
- `scripts/runtime/install.ps1`, `scripts/runtime/self-heal.ps1`, and `scripts/orchestration/stages/validate-stage.ps1` dispatch diagnostics through the managed `ntk` binary instead of the deleted local wrapper paths.
- The compatibility wrappers `scripts/runtime/doctor.ps1` and `scripts/runtime/healthcheck.ps1` were removed after runtime tests, CLI tests, and native operator-path smoke checks passed.
- The live PowerShell estate decreased from `104` to `102`, and the remaining runtime domain stayed classified as `parity proven` rather than being broadened into an implicit full-domain cutover.