# Development Agent Orchestrator Experience Spec

Generated: 2026-04-04 00:00

## Status

- LastUpdated: 2026-04-05 11:42
- Objective: define the target experience and architectural boundaries required for `ntk` to operate as a development-focused AI agent orchestrator with strong provider ergonomics, diagnostics, routing, and operator guidance.
- Normalized Request: create a design specification for applying stronger development-agent orchestrator patterns to the repository without collapsing existing focused workstreams.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-development-agent-orchestrator-experience.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already has strong workstreams for provider testing, token economy, runtime diagnostics, multi-agent behavior, and agentic boundaries, but they do not yet present a cohesive development-operator experience. Without an umbrella design, the system risks growing the technical pieces without delivering a predictable orchestrator model for day-to-day AI-assisted development.

---

## Design Intent

- Treat `ntk` as a development-oriented AI agent orchestrator, not only as a generic AI command surface.
- Add a stable development experience around:
  - provider profiles and presets
  - runtime doctor and operator reports
  - smart provider routing and fallback strategy
  - normalized provider adapter contracts
  - development operator playbook guidance
  - agent-to-model routing policy
- Keep existing workstreams as the owners of detailed implementation rather than duplicating their scopes.
- Preserve clear boundaries between:
  - provider configuration
  - provider transport adapters
  - request routing
  - token economy
  - operator diagnostics
  - multi-agent lineage and inherited settings

---

## Options Considered

1. Extend the current AI runtime incrementally without an umbrella orchestrator design.
   - Rejected: improvements would remain fragmented across multiple workstreams and produce inconsistent operator experience.
2. Replace existing workstreams with one large orchestrator mega-plan.
   - Rejected: this would erase already useful boundaries and create a planning monolith.
3. Add a dedicated umbrella spec/plan that coordinates the development-orchestrator experience while reusing existing owner plans.
   - Preferred: preserves focused workstreams while making the target runtime model explicit.

---

## Architecture Direction

The target repository model should separate these layers cleanly:

- `profiles`: user-facing presets and persisted selection state
- `doctor`: diagnostics, health, and operator reporting
- `router`: provider scoring, strategy, and fallback control
- `adapters`: provider-specific transport/auth/stream envelopes
- `agent policy`: skill or agent defaults for model/profile selection
- `playbook`: operator guidance and troubleshooting

This separation is required so the development experience becomes stronger without violating SOLID boundaries already being improved elsewhere.

---

## Acceptance Criteria

- A dedicated umbrella plan/spec exists for the development-agent orchestrator experience.
- The orchestrator experience is decomposed into provider profiles, doctor/report surfaces, smart routing, normalized adapters, operator playbook, and agent-to-model routing.
- The workstream explicitly reuses existing plans for provider matrix, token economy, runtime diagnostics, MCP/runtime resilience, and multi-agent lineage instead of duplicating them.
- The planning indexes and changelog are updated so this becomes a first-class active workstream.
- No implementation is required for this slice.

---

## Risks

- The umbrella plan can become redundant if it starts duplicating narrower plans.
- Smart routing and agent/model policy can create hidden behavior if observability is weak.
- Provider ergonomics can drift into ad-hoc configuration if profiles and adapters are not separated.

---

## Planning Readiness

- Ready for planning immediately.
- The first execution slice should keep this workstream planning-only and integrate it into the active planning indexes.
- Implementation should start with the fastest operator-value slices:
  - provider profiles
  - doctor/report
  - routing strategy/status
- First implementation proof now exists for provider profiles:
  - built-in `balanced`, `coding`, `cheap`, `latency`, and `local` presets
  - optional `NTK_AI_PROFILE` selection with env override precedence preserved
  - `ntk ai profiles list/show` operator inspection surfaces
- Second implementation proof now exists for diagnostics:
  - `ntk ai doctor` read-only runtime inspection
  - machine-readable JSON output
  - optional Markdown report generation for operator troubleshooting
- Third implementation proof now exists for operator guidance:
  - `docs/operations/ai-development-operator-playbook.md`
  - profile-selection and diagnostics workflow documented outside the root README