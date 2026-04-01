# MCP Transport Auth And Session Resilience Plan

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: harden MCP transport, auth, session, and retry behavior through explicit boundaries that stay separate from runtime bootstrap and other agentic surfaces.
- Normalized Request: create a detailed MCP workstream that improves transport resilience, auth/session lifecycle handling, and diagnostics in the runtime stack.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-mcp-transport-auth-and-session-resilience.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Inputs:
  - `crates/commands/runtime/src/sync/mcp_config.rs`
  - `crates/commands/runtime/src/sync/mcp_runtime_artifacts.rs`
  - `crates/commands/runtime/src/sync/bootstrap.rs`
  - `crates/commands/runtime/src/error.rs`
  - `README.md`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| M1 | MCP lifecycle matrix | runtime sync + live client ownership | 🔴 Immediate | none |
| M2 | Transport adapter boundaries | stdio/http/sse/ws style separation | 🔴 Immediate | M1 |
| M3 | Auth and session lifecycle services | credentials + refresh + expiry + retry | 🟠 High | M1 |
| M4 | MCP diagnostics and status reporting | health/runtime command surfaces | 🟠 High | M2, M3 |
| M5 | Projection/runtime sync cleanup | bootstrap/provider surfaces alignment | 🟡 Medium | M2, M3 |

---

## Ordered Tasks

### [2026-03-31 00:00] Task M1: Freeze MCP Lifecycle Matrix

- Inventory MCP server types, connection modes, and current failure classes.
- Classify configuration/projection steps separately from connection/session steps.
- Define the retryable and terminal error categories.
- Commit checkpoint:
  - `docs(planning): freeze mcp lifecycle matrix`

### [2026-03-31 00:00] Task M2: Introduce MCP Transport Adapters

- Create explicit transport boundaries for the MCP connection modes we support.
- Prevent transport mechanics from leaking into projection or auth code.
- Add tests that validate adapter behavior independently of runtime bootstrap.
- Commit checkpoint:
  - `refactor(runtime): add mcp transport adapter boundaries`

### [2026-03-31 00:00] Task M3: Extract Auth And Session Lifecycle Services

- Move credentials, refresh, expiry, and retry policy into dedicated services.
- Record session expiry as a first-class runtime state, not a generic failure.
- Ensure retry behavior is observable and bounded.
- Commit checkpoint:
  - `refactor(runtime): isolate mcp auth and session lifecycle`

### [2026-03-31 00:00] Task M4: Add MCP Diagnostics And Runtime Inspection

- Expose MCP status, connection mode, and failure class through runtime diagnostics.
- Add smoke coverage for status visibility and degraded states.
- Ensure errors are actionable and not only transport-specific traces.
- Commit checkpoint:
  - `feat(runtime): add mcp diagnostics and status reporting`

### [2026-03-31 00:00] Task M5: Align Projection And Bootstrap Boundaries

- Keep runtime sync responsible for static projection and config application.
- Keep live MCP connection logic outside generic bootstrap flows.
- Update README and instruction surfaces to describe the MCP ownership model.
- Commit checkpoint:
  - `docs(runtime): document mcp transport and lifecycle boundaries`

---

## Validation Checklist

- `cargo test -p nettoolskit-runtime --quiet`
- `cargo test -p nettoolskit-runtime --test test_suite sync --quiet`
- `cargo test -p nettoolskit-cli --test test_suite runtime_commands_tests --quiet`
- `git diff --check`

---

## Risks And Mitigations

- Introducing transport adapters too early can create abstractions without stable use cases.
- Auth/session separation can break projection flows if lifecycle ownership is not mapped first.
- Diagnostics can become noisy if statuses are not normalized.
- Mitigation: start with lifecycle inventory, then move code only after the state machine is frozen.

---

## Specialist And Closeout

- Recommended specialist: `dev-rust-engineer`
- Tester: required
- Reviewer: required
- README update: required
- Suggested commit message style:
  - `refactor(runtime): isolate mcp lifecycle boundaries`
  - `docs(planning): record mcp resilience roadmap`