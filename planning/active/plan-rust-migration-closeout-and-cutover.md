# Plan: Rust Migration Closeout And Cutover

Generated: 2026-03-28 08:28

## Status

- LastUpdated: 2026-03-28 10:23
- Objective: consolidate the remaining backlog after README normalization and Waves 1-3 so the repository can reach a clean, evidence-backed Rust-default cutover.
- Active Branch: `feature/native-validation-policy`
- Inputs:
  - `planning/active/plan-readme-standards-repository-normalization.md`
  - `planning/active/plan-repository-operations-hygiene.md`
  - `planning/active/plan-repository-unification-and-rust-migration.md`
  - `planning/active/rust-script-parity-ledger.md`
  - `planning/active/rust-script-transcription-ownership-matrix.md`
  - `planning/active/rust-script-cutover-default-map.md`
- Review Conclusion: README normalization is complete, the migration implementation waves are effectively complete through Wave 3, and the closeout package is now materially advanced: workspace hygiene gates are green again, parity harness coverage is stable, and operator-facing docs/workflows already frame PowerShell as the compatibility surface. The remaining work is now the blocked-domain tail plus parity fixture isolation.

## Review Summary

### Closed Workstreams

- `plan-readme-standards-repository-normalization.md`: complete; no remaining delivery backlog beyond keeping the validator green.
- `plan-repository-unification-and-rust-migration.md`: Tasks 1-6 are complete; Task 7 is implementation-complete in practice and now needs status normalization; Task 8 is narrowed to the blocked-domain tail and fixture isolation.
- `rust-script-transcription-ownership-matrix.md`: still valid as the canonical inventory/owner map; only metadata and closeout linkage are stale.

### Open Workstreams

- `plan-repository-operations-hygiene.md`: now records a green `fmt` / `clippy` / `test` / audit baseline plus the remaining artifact-isolation follow-up for parity fixtures.
- `rust-script-parity-ledger.md`: evidence policy remains valid, but the domain notes still need to reflect the stabilized parity harness, mixed EOF-policy handling, and the current distinction between `parity proven` and `cutover ready`.
- `rust-script-cutover-default-map.md`: new closeout artifact that records the final per-domain default decision and isolates the remaining blocked slices from the domains that are already Rust-default.

## Backlog Size Assessment

- This is not another full `147`-script planning cycle.
- The architecture, ownership, and most implementation waves are already in place.
- The remaining backlog is now small-to-moderate and concentrated in closeout:
  - one residual blocked-domain group that still needs native replacement evidence
  - one residual artifact-isolation follow-up for parity fixtures

## Remaining Open Backlog

1. Blocked-domain tail:
   - `scripts/runtime/hooks`
   - `scripts/maintenance`
   - `scripts/doc`
   - `scripts/deploy`
   - `scripts/tests` excluding runtime
2. Artifact and recovery closeout:
   - parity suites still generate temporary repository artifacts during full-workspace runs and require deterministic cleanup before the next commit

## Ordered Tasks

### Task 1: Rebaseline The Active Planning State Around Closeout

Status: `[x]` Completed

- Refresh the parity ledger so every domain row reflects the current real state:
  - `parity proven`
  - `cutover ready`
  - `wrapper retained intentionally`
  - `evidence gap remains`
- [2026-03-28 08:28] Closeout review completed: README normalization is closed, the main migration plan is materially complete through Wave 3, and the remaining open backlog is now owned by this closeout plan ✓ [2026-03-28 08:28]
- [2026-03-28 10:00] Rebaselined the closeout state around the current branch reality:
  - `cargo fmt --all -- --check` now passes after persisting the workspace Rust EOF/format baseline
  - `cargo clippy --workspace --all-targets -- -D warnings` now passes after the runtime/validation closeout fixes
  - `cargo test --workspace` now passes after parity-harness stabilization and mixed EOF-policy alignment
  - Rust vulnerability audit remains green
  - runtime/docs/workflows now already describe PowerShell as the compatibility layer rather than the primary implementation path ✓ [2026-03-28 10:00]
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

Status: `[x]` Completed

- [2026-03-28 10:00] Restored the closeout hygiene baseline:
  - `cargo fmt --all -- --check` passed
  - `cargo clippy --workspace --all-targets -- -D warnings` passed
  - `cargo test --workspace` passed
  - `Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High` passed ✓ [2026-03-28 10:00]
- [2026-03-28 10:00] Mixed `.editorconfig` EOF rules are now honored consistently across:
  - Rust runtime pre-commit hygiene
  - PowerShell trim/maintenance flows
  - VS Code hook normalization and related runtime tests ✓ [2026-03-28 10:00]
- [2026-03-28 10:00] CI/release governance already validates the Rust-owned release-governance and provenance paths, so the remaining closeout work is no longer blocked by baseline hygiene gates ✓ [2026-03-28 10:00]
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

Status: `[x]` Completed

- For each owned domain, record one explicit end state:
  - Rust parity proven and ready for default cutover
  - Rust parity proven but wrapper intentionally retained
  - evidence gap still open
- [2026-03-28 10:00] Rebased the domain ledger around the current closeout truth:
  - no domain is overstated as `cutover ready`
  - implemented domains stay recorded as `parity proven`
  - maintenance, runtime hook, doc, deploy, and non-runtime test automation remain explicitly tracked as evidence gaps until their end-state decision is recorded ✓ [2026-03-28 10:00]
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

Status: `[x]` Completed

- Define the wrapper end-state map for every PowerShell entrypoint:
  - Rust-default wrapper
  - compatibility wrapper kept intentionally
  - legacy script retained temporarily with explicit reason
  - retired
- [2026-03-28 09:32] Updated repository/runtime/validation docs so PowerShell entrypoints are described as compatibility wrappers over Rust-owned command surfaces ✓ [2026-03-28 09:32]
- [2026-03-28 09:32] Rebaselined CI and release workflows so Rust-owned release-governance and provenance checks are the canonical validated path ✓ [2026-03-28 09:32]
- [2026-03-28 10:23] Recorded the final per-domain default map in `planning/active/rust-script-cutover-default-map.md`, explicitly separating Rust-default domains, compatibility wrappers retained intentionally, and still-blocked domains ✓ [2026-03-28 10:23]
- Remaining work:
  - keep the blocked-domain backlog explicit in Task 5
  - preserve compatibility-wrapper reasoning where intentionally retained
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

- Switch default operator flows to Rust for domains that are explicitly marked `Rust-default now` in the cutover map, while preserving approved wrappers for intentionally retained domains.
- Preserve only approved fallback wrappers, with the reason recorded in planning.
- Archive or downgrade the old active wave plans once the closeout state is green and unambiguous.
- [2026-03-28 10:23] The remaining backlog is now limited to the still-blocked domains and parity fixture isolation; the domains already marked Rust-default now or compatibility-wrapper-retained are not part of the remaining execution backlog.
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