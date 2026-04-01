# MCP Transport Auth And Session Resilience Spec

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: define a resilient MCP client/runtime design with explicit transport, auth, session, and retry boundaries.
- Normalized Request: open a detailed workstream to harden MCP behavior in runtime projection and client execution without coupling it to unrelated agentic surfaces.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-mcp-transport-auth-and-session-resilience.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already treats MCP as a first-class runtime/tooling surface, but the current implementation can still improve in transport abstraction, auth handling, session lifecycle management, and operational visibility. Without dedicated design intent, MCP logic risks remaining partly embedded in broader runtime bootstrap flows.

---

## Design Intent

- Separate MCP transport concerns from auth/session concerns.
- Support multiple MCP transport modes behind stable repository-owned abstractions.
- Treat expired sessions, auth failures, and retriable connection issues as explicit lifecycle cases.
- Improve visibility of MCP connection state, server state, and retry behavior.
- Keep MCP isolated from RAG/CAG and future A2A surfaces.

---

## Options Considered

1. Keep MCP inside runtime bootstrap and extend it incrementally.
   - Rejected: bootstrap remains too broad and MCP lifecycle logic stays harder to test.
2. Push all complexity into a single MCP client module.
   - Rejected: transport, auth, and session management should evolve independently.
3. Create explicit MCP transport/auth/session/status boundaries under runtime sync and client services.
   - Preferred: aligns with SOLID and improves observability and testing.

---

## Proposed Boundaries

- Transport adapters own protocol mechanics only.
- Auth/session services own credentials, refresh, expiry, and retry policy.
- Runtime sync owns projection/config application, not connection lifecycle internals.
- MCP status and diagnostics are exposed through dedicated inspection APIs or commands.
- Future A2A work must not reuse MCP lifecycle modules as a shortcut.

---

## Acceptance Criteria

- MCP transport types are explicit and testable.
- Auth and session expiry handling are not buried in bootstrap code.
- Runtime projection and live connection handling are separated.
- Diagnostics can report MCP server state, transport mode, and failure class.
- README and instructions describe MCP as a dedicated runtime/tool surface.

---

## Planning Readiness

- Ready for planning once current MCP projection and client ownership are mapped.
- The first implementation slice should freeze the lifecycle matrix before moving code.
- Validation should include both config/projection tests and connection-state tests.