# CLI Fast-Path And Startup Decomposition Plan

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: decompose `ntk` startup into explicit fast-path, service, runtime, and interactive boundaries to reduce startup overhead and improve maintainability.
- Normalized Request: create a detailed workstream for modular CLI startup and fast-path routing without changing the single-binary `ntk` operating model.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-cli-fast-path-and-startup-decomposition.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Inputs:
  - `crates/cli/src/main.rs`
  - `crates/cli/src/runtime_commands.rs`
  - `crates/cli/src/validation_commands.rs`
  - `crates/cli/src/ai_commands.rs`
  - `crates/orchestrator/src/execution/*`
  - `README.md`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| E1 | Startup map and load audit | `crates/cli/src/*` | 🔴 Immediate | none |
| E2 | Fast-path routing boundary | lightweight `ntk` command surfaces | 🔴 Immediate | E1 |
| E3 | Service/daemon entrypoint isolation | service/runtime startup | 🟠 High | E1 |
| E4 | Interactive stack boundary cleanup | full orchestration/UI startup | 🟠 High | E2 |
| E5 | Startup documentation and validation | README + tests + instrumentation | 🟡 Medium | E2, E3, E4 |

---

## Ordered Tasks

### [2026-03-31 00:00] Task E1: Freeze Startup Boundary Map

- Inventory the current `ntk` command families and the modules they load.
- Identify the commands that should remain lightweight:
  - version/help/info
  - runtime projection and diagnostics
  - validation entrypoints
  - ledger/reporting commands
- Record the current coupling points between CLI parsing and orchestrator/runtime layers.
- Commit checkpoint:
  - `docs(planning): freeze cli startup boundary map`

### [2026-03-31 00:00] Task E2: Introduce Explicit Fast-Path Routing

- Extract a startup router in `crates/cli` that can dispatch lightweight surfaces without loading the full interactive stack.
- Define an ownership contract for fast-path modules:
  - no interactive rendering
  - no orchestration bootstrap
  - no MCP/runtime sync load unless directly required
- Add tests that prove selected commands stay on the lightweight path.
- Commit checkpoint:
  - `refactor(cli): add explicit fast-path startup routing`

### [2026-03-31 00:00] Task E3: Isolate Service And Daemon Modes

- Move service/daemon-style startup responsibilities behind dedicated boundaries.
- Keep health checks, pid/session ownership, and local service data setup isolated from user-facing CLI routes.
- Ensure service startup does not accidentally inherit interactive-only state.
- Commit checkpoint:
  - `refactor(cli): isolate service startup boundaries`

### [2026-03-31 00:00] Task E4: Reduce Interactive Startup Coupling

- Keep interactive runtime wiring behind a dedicated bootstrap boundary.
- Prevent AI/task orchestration initialization from leaking into non-interactive commands.
- Align startup code with existing `orchestrator` and `runtime` crate ownership.
- Commit checkpoint:
  - `refactor(cli): narrow interactive startup ownership`

### [2026-03-31 00:00] Task E5: Validate And Document Startup Surfaces

- Add startup-path tests or instrumentation-based assertions.
- Update README and CLI docs with the startup architecture and expected fast paths.
- Record the supported command families and their intended startup class.
- Commit checkpoint:
  - `docs(cli): document fast-path and startup model`

---

## Validation Checklist

- `cargo test -p nettoolskit-cli --quiet`
- `cargo test -p nettoolskit-cli --test test_suite runtime_commands_tests --quiet`
- `cargo test -p nettoolskit-cli --test test_suite validation_commands_tests --quiet`
- `cargo test -p nettoolskit-cli --test test_suite ai_commands_tests --quiet`
- `git diff --check`

---

## Risks And Mitigations

- Startup decomposition can accidentally change command behavior if routing is incomplete.
- Lightweight-path assertions can become brittle if they depend on unstable internals.
- Service and interactive entrypoints can drift if contracts are not written down.
- Mitigation: freeze the route map first, keep slices small, and validate with command-family tests.

---

## Specialist And Closeout

- Recommended specialist: `dev-rust-engineer`
- Tester: required
- Reviewer: required
- README update: required
- Changelog: required when implementation lands
- Suggested commit message style:
  - `refactor(cli): add fast-path startup routing`
  - `docs(planning): record cli startup decomposition roadmap`