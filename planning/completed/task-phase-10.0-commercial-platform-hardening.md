# Phase 10.0 - Commercial Platform Hardening

## Objective

Close the remaining gaps between the current local-first engineering agent runtime and a commercially defensible developer platform aligned with the product direction inspired by Codex/OpenClaw.

## Scope

This phase focuses on control-plane hardening, operational correctness, quality policy, and repository governance. It is not a feature-expansion phase.

## Baseline Findings

1. The mutable service API surface (`POST /task/submit`) was not protected by first-party authentication.
2. `GET /health` and `GET /ready` currently represent the same shallow readiness signal.
3. Coverage gates pass globally, but critical public entrypoints still have weak or zero direct coverage.
4. Governance artifacts expected from commercial OSS operation are still missing (`LICENSE`, `SECURITY.md`, `CODEOWNERS`).
5. The service HTTP surface is still implemented on ad-hoc `TcpListener`/`TcpStream` parsing rather than middleware-capable transport.

## Delivery Slices

### Slice 10.1 - Service API Security Baseline

- Bind `ntk service` to loopback by default.
- Require explicit bearer auth for mutable service API routes.
- Fail startup when binding to non-loopback host without configured service auth.
- Keep health/readiness unauthenticated for local liveness and release smoke checks.
- Update Docker local profile, CI smoke, and runbooks to match the new contract.

Acceptance:

- `POST /task/submit` returns `401` when auth is configured and header is missing/invalid.
- Non-loopback bind without `NTK_SERVICE_AUTH_TOKEN` fails before listener startup.
- Local Docker smoke remains deterministic with configured local token.

### Slice 10.2 - Real Readiness

- Separate `GET /ready` from `GET /health`.
- Add readiness checks for worker runtime admission path, replay backend initialization, and required local persistence paths.
- Expose degraded vs ready states clearly in JSON payload.

Acceptance:

- `GET /health` proves liveness only.
- `GET /ready` fails when critical service dependencies are unavailable.

### Slice 10.3 - Critical Coverage Budgets

- Enforce no-`0%` policy for public entrypoint files.
- Add file-level minimum budgets for:
  - `crates/cli/src/main.rs`
  - `crates/commands/manifest/src/ui/menu.rs`
  - `crates/otel/src/tracing_setup.rs`
  - `crates/orchestrator/src/execution/processor.rs`
- Expand direct tests before raising global thresholds.

Acceptance:

- Coverage gate fails when any critical file drops below configured budget.

### Slice 10.4 - Governance Baseline

- Add repository `LICENSE`.
- Add `SECURITY.md` with disclosure/response process.
- Add `CODEOWNERS` for review accountability.
- Align README references with actual repository artifacts.

Acceptance:

- Release packaging and repository root contain valid governance artifacts.

### Slice 10.5 - Service Transport Modernization

- Replace manual HTTP parsing with framework-grade transport (`axum`/`hyper`).
- Add middleware for auth, request ID, body limits, tracing, and timeout budgets.
- Preserve existing service endpoints and tests during migration.

Acceptance:

- Service runtime keeps current endpoints while gaining standard middleware and simpler hardening.

### Slice 10.6 - Formal Control Plane Specification

- Publish a formal control-plane/session/operator model under repository docs.
- Separate current implementation truth from target platform evolution.
- Define the transport-neutral envelope future gateway/control-UI work must reuse.

Acceptance:

- Repository has one architecture document that explains operator identity, session boundaries, task attribution, and local-first persistence without overstating unimplemented features.

## Validation Checklist

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test -p nettoolskit-cli --bin ntk service_mode_`
- `cargo test --workspace`
- `docker compose -f deployments/docker-compose.local.yml up --build -d`
- `curl /health`
- `curl /ready`
- `curl /task/submit` with and without bearer token

## Risks

- Service auth can break local smoke flows if Docker/CI/docs are not updated together.
- HTTP transport migration can destabilize ChatOps ingress if endpoint behavior changes unintentionally.
- Coverage gates can create large follow-up work if raised before targeted test slices are added.

## Current Status

- Slice 10.1 completed.
- Slice 10.2 completed.
- Slice 10.3 completed with CI-enforced critical-file coverage budgets sourced from `cargo llvm-cov` JSON reports.
- Slice 10.3 is now validated end-to-end: `cargo test --workspace --all-targets` passes locally again on Windows MSVC, `cargo llvm-cov --workspace --all-targets --json --output-path .build/coverage/report.json` generates successfully, and the critical-file gate passes with totals of `76.52%` lines and `76.82%` functions.
- Critical-file coverage snapshot after closure: `cli/main.rs` (`79.34%` lines / `72.69%` functions), `manifest/ui/menu.rs` (`18.64%` / `40.00%`), `otel/tracing_setup.rs` (`72.24%` / `79.31%`), and `orchestrator/processor.rs` (`74.51%` / `82.34%`).
- Slice 10.4 completed with root/license governance artifacts, private disclosure policy, and CODEOWNERS review routing.
- Slice 10.5 completed: service mode now runs on an `axum`/`hyper` stack with middleware-based request IDs, body limits, bearer-auth enforcement, timeout handling, and direct transport tests for `x-request-id`, `413`, and `408` behavior.
- Residual supply-chain follow-up remains after 10.5 validation: `cargo audit` still reports allowed warnings for `rustls-pemfile 1.0.4` (via `reqwest 0.11`) and `windows 0.24.0` (via `winrt-notification`), so dependency modernization is still recommended outside the critical/high gate.
- Slice 10.6 completed with a formal architecture spec at `docs/architecture/control-plane-session-operator-model.md`, plus README cross-links and an explicit target `ControlEnvelope`/`OperatorContext` direction for future gateway work.
- Post-10.6 implementation follow-up completed in `nettoolskit-core`: typed `OperatorContext`, `SessionContext`, `ControlPolicyContext`, and `ControlEnvelope` contracts are now exported and validated, ready for future adoption by HTTP and ChatOps ingress layers.
- First ingress adoption completed for service HTTP: `/task/submit` now derives typed request/operator/session metadata from headers, echoes `x-correlation-id`, returns `task_id` plus control-plane metadata in accepted responses, and persists the admitted envelope into task registry/audit events.
- Second ingress adoption completed for ChatOps task submission: remote `submit` intents now derive typed request/operator/session metadata, reuse `process_control_envelope`, and persist normalized control-plane fields into local ChatOps audit trails.
- Third ingress adoption completed for local CLI task submission: `/task submit` now derives a typed local control envelope, routes through `process_control_envelope`, and persists normalized control metadata into task registry and audit events.
- ChatOps non-submit commands (`help`, `list`, `watch`, `cancel`) now derive typed control-plane metadata as well, so remote management actions carry normalized request/operator/session/correlation attribution in local audit trails even when execution stays on the existing command handlers.
- Canonical planning for this phase now lives under `planning/active/`, and `.temp` has been reduced to disposable-only scratch usage.

## Post-Phase Follow-Up

- Modernize the dependency chain to remove the remaining allowed `cargo audit` warnings.
- Propagate typed control-plane metadata into outbound Telegram/Discord notifications.
- Reuse the real interactive CLI session identifier for local `/task submit` flows.
