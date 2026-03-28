# Plan: Rust Migration Closeout And Cutover

Generated: 2026-03-28 08:28

## Status

- LastUpdated: 2026-03-28 08:28
- Objective: consolidate the remaining backlog after README normalization and Waves 1-3 so the repository can reach a clean, evidence-backed Rust-default cutover.
- Active Branch: `feature/native-validation-policy`
- Inputs:
  - `planning/active/plan-readme-standards-repository-normalization.md`
  - `planning/active/plan-repository-operations-hygiene.md`
  - `planning/active/plan-repository-unification-and-rust-migration.md`
  - `planning/active/rust-script-parity-ledger.md`
  - `planning/active/rust-script-transcription-ownership-matrix.md`
- Review Conclusion: README normalization is complete, the migration implementation waves are effectively complete through Wave 3, and the remaining work is now closeout-oriented: hygiene restoration, parity evidence rebasing, wrapper/default cutover, and final documentation/release alignment.

## Review Summary

### Closed Workstreams

- `plan-readme-standards-repository-normalization.md`: complete; no remaining delivery backlog beyond keeping the validator green.
- `plan-repository-unification-and-rust-migration.md`: Tasks 1-6 are complete; Task 7 is implementation-complete in practice and now needs status normalization; Task 8 remains the primary open delivery item.
- `rust-script-transcription-ownership-matrix.md`: still valid as the canonical inventory/owner map; only metadata and closeout linkage are stale.

### Open Workstreams

- `plan-repository-operations-hygiene.md`: still owns unresolved hygiene gates (`cargo fmt`, `clippy`, CI/wrapper governance, artifact closeout).
- `rust-script-parity-ledger.md`: evidence policy remains valid, but multiple domain rows still show stale pre-closeout statuses such as `implementation pending`, `boundary crate created`, or `planning locked`.

## Backlog Size Assessment

- This is not another full `147`-script planning cycle.
- The architecture, ownership, and most implementation waves are already in place.
- The remaining backlog is moderate but still material:
  - one planning/evidence rebaseline
  - one hygiene restoration block
  - one domain cutover-readiness block
  - one staged wrapper/default cutover block

## Remaining Open Backlog

1. Planning drift:
   - the parity ledger understates current implementation progress
   - the ownership matrix still carries stale branch metadata
   - the old active plans still describe open work in separate places
2. Hygiene debt:
   - `cargo fmt --all -- --check` is still a recorded failing gate
   - `cargo clippy --workspace --all-targets -- -D warnings` is still an unclosed requirement in planning
   - CI/workflow governance has not been rebaselined around the new Rust-owned defaults
3. Cutover readiness debt:
   - wrapper end state is not normalized in one artifact
   - operator smoke evidence is not summarized by domain in one artifact
   - docs and release-facing cutover guidance are not yet closed out
4. Artifact and recovery closeout:
   - maintenance/recovery flows still need final evidence and operator-facing closeout criteria

## Ordered Tasks

### Task 1: Rebaseline The Active Planning State Around Closeout

Status: `[~]` In Progress

- Refresh the parity ledger so every domain row reflects the current real state:
  - `parity proven`
  - `cutover ready`
  - `wrapper retained intentionally`
  - `evidence gap remains`
- [2026-03-28 08:28] Closeout review completed: README normalization is closed, the main migration plan is materially complete through Wave 3, and the remaining open backlog is now owned by this closeout plan ✓ [2026-03-28 08:28]
- [2026-03-28 08:28] Baseline verification captured the active technical blockers for closeout:
  - `cargo fmt --all -- --check` still fails broadly across the repository
  - `cargo test --workspace` is blocked by the `run_test_closeout` parity path in `nettoolskit-orchestrator`
  - `cargo clippy --workspace --all-targets -- -D warnings` is currently blocked in `nettoolskit-validation` and `nettoolskit-orchestrator`
  - Rust vulnerability audit is currently passing ✓ [2026-03-28 08:28]
- Refresh metadata drift in the ownership matrix and active plan references.
- Mark historical wave plans as implementation records and this plan as the owner of the open backlog.
- Target paths:
  - `planning/active/plan-rust-migration-closeout-and-cutover.md`
  - `planning/active/plan-repository-operations-hygiene.md`
  - `planning/active/plan-repository-unification-and-rust-migration.md`
  - `planning/active/rust-script-parity-ledger.md`
  - `planning/active/rust-script-transcription-ownership-matrix.md`
- Commands:
  - `git status --short --branch`
  - `rg -n "^### Task|^Status:" planning/active/*.md`
  - `git diff --check`
- Commit checkpoint:
  - `docs(planning): rebaseline rust migration closeout state`

### Task 2: Restore A Fully Green Rust Hygiene Baseline

Status: `[ ]` Pending

- Close the remaining workspace hygiene gates before wrapper retirement:
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - security audit gate
- Resolve any residual test drift in `runtime`, `validation`, `core`, `commands`, and `task-worker` that prevents a trustworthy closeout baseline.
- Rebaseline CI/workflow expectations so Rust-owned commands are the default validated path.
- Target paths:
  - `Cargo.toml`
  - `Cargo.lock`
  - `crates/`
  - `.github/workflows/`
  - `planning/active/plan-repository-operations-hygiene.md`
- Commands:
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `pwsh -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -ProjectPath . -FailOnSeverities Critical,High`
- Commit checkpoint:
  - `chore(rust): restore closeout hygiene gates`

### Task 3: Normalize Domain-Level Parity And Cutover Readiness

Status: `[ ]` Pending

- For each owned domain, record one explicit end state:
  - Rust parity proven and ready for default cutover
  - Rust parity proven but wrapper intentionally retained
  - evidence gap still open
- Ensure the parity ledger explicitly covers:
  - `scripts/common`
  - `scripts/runtime`
  - `scripts/runtime/hooks`
  - `scripts/maintenance`
  - `scripts/validation`
  - `scripts/security`
  - `scripts/governance`
  - `scripts/doc`
  - `scripts/deploy`
  - `scripts/orchestration`
  - `scripts/git-hooks`
  - `scripts/tests`
- Require at least one operator smoke path per cutover-ready domain.
- Target paths:
  - `planning/active/rust-script-parity-ledger.md`
  - `crates/commands/runtime/`
  - `crates/commands/validation/`
  - `crates/orchestrator/`
  - `crates/cli/`
- Commands:
  - `cargo test --workspace`
  - targeted command smoke checks per domain
  - `git diff --check`
- Commit checkpoint:
  - `test(rust): close parity evidence gaps for cutover`

### Task 4: Prepare The Rust-Default Cutover Package

Status: `[ ]` Pending

- Define the wrapper end-state map for every PowerShell entrypoint:
  - Rust-default wrapper
  - compatibility wrapper kept intentionally
  - legacy script retained temporarily with explicit reason
  - retired
- Update operator-facing docs and release-facing guidance for the new defaults.
- Rebaseline CI and validation so Rust-owned entrypoints are the canonical path.
- Target paths:
  - `scripts/`
  - `README.md`
  - `CHANGELOG.md`
  - `.github/workflows/`
  - `docs/`
- Commands:
  - `cargo test --workspace`
  - final domain smoke suite
  - `git diff --check`
- Commit checkpoint:
  - `docs(runtime): prepare rust default cutover package`

### Task 5: Execute Staged Wrapper And Default Cutover

Status: `[ ]` Pending

- Switch default operator flows to Rust for domains that are explicitly marked `cutover ready`.
- Preserve only approved fallback wrappers, with the reason recorded in planning.
- Archive or downgrade the old active wave plans once the closeout state is green and unambiguous.
- Target paths:
  - `scripts/`
  - `crates/cli/`
  - `.github/workflows/`
  - `planning/active/`
- Commands:
  - `cargo test --workspace`
  - final wrapper smoke suite
  - `git diff --check`
- Commit checkpoint:
  - `refactor(runtime): cut over rust-backed defaults`

## Exit Criteria

- one active plan owns the remaining open migration backlog
- the parity ledger matches real implementation status by domain
- the workspace is green on `fmt`, `clippy`, tests, and security audit
- wrapper/default end state is explicit for the full PowerShell estate
- operator-facing docs describe the Rust-default path and fallback story

## Recommended Specialist

- Primary: `plan-active-work-planner`
- Delivery:
  - `dev-rust-engineer`
  - `test-engineer`
  - `ops-devops-platform-engineer`
  - `docs-release-engineer`

## Closeout Expectations

- Do not open new wave plans for already-owned script domains unless scope genuinely expands.
- Keep commit messages in English and checkpoint-oriented.
- Preserve fallback wrappers until the parity ledger marks the owning domain as `cutover ready`.
- Archive or clearly demote completed historical plans once the closeout plan becomes the single open backlog owner.