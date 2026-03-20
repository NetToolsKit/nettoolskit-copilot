# Repository Operations Hygiene Plan

## Scope Summary

This active plan tracks the current repository hygiene work after the enterprise delivery phases were closed.

Completed in the current workstream:

- canonical planning moved out of temporary local storage and normalized under `planning/`
- build, coverage, and local deployment/runtime artifacts centralized under `.build/` and `.deployment/`
- service-mode local persistence and local audit defaults moved away from `.temp`

Remaining follow-up backlog:

- remove the remaining allowed Rust dependency warnings from the supply-chain baseline
- propagate typed control-plane metadata into outbound Telegram/Discord notifications
- reuse the real interactive CLI session identifier for local `/task submit` flows

## Ordered Tasks

### Task 1: Normalize Canonical Planning Layout

Status: `[x]` Completed

- Target paths:
  - `planning/active/plan-repository-operations-hygiene.md`
  - `planning/README.md`
  - `planning/completed/`
  - `planning/specs/README.md`
- Commands:
  - `rg -n "planning/active|planning/completed|planning/specs" .`
  - `git status --short`
- Checkpoints:
  - canonical active plan exists under `planning/active/`
  - completed roadmap/tracker/phase documents are preserved under `planning/completed/`
  - planning workspace matches the shared `active/completed/specs` convention
- Commit checkpoint:
  - `docs(planning): normalize active and completed planning layout`

### Task 2: Centralize Local Build And Deployment Artifacts

Status: `[x]` Completed

- Target paths:
  - `.cargo/config.toml`
  - `.gitignore`
  - `.github/workflows/ci.yml`
  - `.github/workflows/release.yml`
  - `deployments/docker-compose.local.yml`
  - `docs/operations/service-mode-local-runbook.md`
- Commands:
  - `cargo metadata --format-version 1 --no-deps | rg -o '"target_directory":"[^"]+"'`
  - `docker compose -f deployments/docker-compose.local.yml config`
- Checkpoints:
  - Cargo target directory resolves to `.build/cargo-target`
  - coverage outputs resolve to `.build/coverage`
  - local service-mode data resolves to `.deployment/local/service-data`
- Commit checkpoint:
  - `chore(repo): centralize local build and deployment artifacts`

### Task 3: Modernize Remaining Allowed Rust Dependency Warnings

Status: `[ ]` Pending

- Target paths:
  - `Cargo.toml`
  - `Cargo.lock`
  - impacted crate manifests under `crates/*/Cargo.toml`
- Commands:
  - `cargo audit`
  - `pwsh -File C:\\Users\\tguis\\.github\\scripts\\security\\Invoke-RustPackageVulnerabilityAudit.ps1 -ProjectPath . -FailOnSeverities Critical,High`
- Checkpoints:
  - no remaining allowed warning for `rustls-pemfile` via `reqwest 0.11`
  - no remaining allowed warning for `windows 0.24.0` via `winrt-notification`
- Commit checkpoint:
  - `chore(deps): modernize remaining allowed supply-chain warnings`

### Task 4: Propagate Typed Control-Plane Metadata To Outbound Notifications

Status: `[ ]` Pending

- Target paths:
  - `crates/orchestrator/src/execution/chatops.rs`
  - `crates/orchestrator/src/execution/chatops_runtime.rs`
  - related tests under `crates/orchestrator/tests/`
- Commands:
  - `cargo test -p nettoolskit-orchestrator`
  - `cargo clippy -p nettoolskit-orchestrator --all-targets -- -D warnings`
- Checkpoints:
  - Telegram/Discord outbound notifications include normalized request/operator/session correlation metadata
  - local audit and outbound notifier payloads stay aligned
- Commit checkpoint:
  - `feat(chatops): propagate control-plane metadata to outbound notifications`

### Task 5: Reuse Real Interactive CLI Session Identity For Local Task Submit

Status: `[ ]` Pending

- Target paths:
  - `crates/cli/src/main.rs`
  - `crates/cli/src/lib.rs`
  - `crates/orchestrator/src/execution/processor.rs`
- Commands:
  - `cargo test -p nettoolskit-cli --bin ntk`
  - `cargo test -p nettoolskit-orchestrator`
- Checkpoints:
  - local `/task submit` reuses the active interactive session identity when available
  - request-derived fallback remains only for non-interactive or sessionless paths
- Commit checkpoint:
  - `feat(cli): reuse interactive session identity for local task submit`

## Validation Checklist

- `cargo metadata --format-version 1 --no-deps`
- `cargo fmt --all -- --check`
- `git diff --check`
- `docker compose -f deployments/docker-compose.local.yml config`
- targeted `cargo test` and `cargo clippy` commands for each pending task

## Recommended Specialist

- Primary: `ops-devops-platform-engineer`
- Secondary: `docs-release-engineer`
- Follow-up implementation specialists:
  - `sec-security-vulnerability-engineer` for Task 3
  - `dev-rust-engineer` for Tasks 4 and 5

## Closeout Expectations

- Update `README.md` if local developer workflow paths change again.
- Keep `planning/README.md` aligned with the current active work plan.
- Provide commit messages in English and in fenced code blocks when requested.
- Update `CHANGELOG.md` only for user-visible or release-relevant behavior changes.

## Delivery Slices

- Slice A: planning normalization and local artifact centralization
- Slice B: dependency modernization
- Slice C: outbound notification attribution
- Slice D: local interactive session identity alignment
