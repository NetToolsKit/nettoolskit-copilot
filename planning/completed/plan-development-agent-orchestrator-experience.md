# Development Agent Orchestrator Experience Plan

Generated: 2026-04-04 00:00

## Status

- LastUpdated: 2026-04-05 14:05
- Objective: evolve `ntk` into a stronger development-focused AI agent orchestrator with explicit provider profiles, runtime diagnostics, smart routing, normalized provider adapters, operator playbook coverage, and agent-to-model routing.
- Normalized Request: create a detailed application plan for the strongest orchestrator concepts we want to bring into the repository so the system becomes better for AI-assisted software development workflows.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/completed/spec-development-agent-orchestrator-experience.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Current Slice: D1 through D7 are materially complete. The workstream now lives in `planning/completed/` after provider profiles, runtime doctor/reporting, smart routing, normalized adapter contracts, the operator playbook, agent/skill model routing, and governance integration were all delivered and validated.
- Inputs:
  - `planning/completed/plan-free-llm-provider-test-matrix.md`
  - `planning/active/plan-token-economy-optimization.md`
  - `planning/active/plan-runtime-operational-diagnostics-and-observability.md`
  - `planning/active/plan-multi-agent-runtime-lineage-and-a2a-readiness.md`
  - `planning/active/plan-mcp-transport-auth-and-session-resilience.md`
  - `planning/active/plan-agent-runtime-comparison-and-improvement-matrix.md`

---

## Scope Summary

This workstream coordinates the development-operator experience for `ntk` as an AI agent orchestrator. It does not replace the narrower provider, token, runtime, or multi-agent plans; it binds them into a coherent development-focused runtime model.

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| D1 | Provider profiles and presets | stable local profile model for model/provider selection | 🔴 Immediate | existing provider matrix |
| D2 | Runtime doctor and report surfaces | fast diagnostics and operator troubleshooting | 🔴 Immediate | runtime diagnostics |
| D3 | Smart provider routing and fallback scoring | latency/cost/reliability-aware routing | 🔴 Immediate | token economy, provider matrix |
| D4 | Normalized provider adapter contracts | common envelopes for chat, stream, usage, and errors | 🟠 High | provider matrix, MCP boundaries |
| D5 | Development operator playbook | practical bootstrap, troubleshooting, and recovery guidance | 🟠 High | D1, D2 |
| D6 | Agent-to-model routing policy | per-agent and per-skill default model selection | 🟠 High | multi-agent runtime, token economy |
| D7 | Control-plane integration and closeout | wire this umbrella into README, plans, and runtime governance | 🟡 Medium | D1-D6 |

---

## Ordered Tasks

### [2026-04-04 00:00] Task D1: Define Provider Profiles And Presets

- Define a profile model for development usage such as `coding`, `latency`, `balanced`, `cheap`, and `local`.
- Keep profile persistence separate from provider adapter code.
- Make profile selection readable by CLI, orchestrator, and future provider tests without duplicating configuration rules.
- Reuse the free-provider matrix as the source of provider-mode classification instead of re-documenting providers here.
- Completed slice:
  - `crates/orchestrator/src/execution/ai_profiles.rs` now owns the built-in profile catalog and `NTK_AI_PROFILE` resolution.
  - `crates/orchestrator/src/execution/processor.rs` now overlays provider-chain, timeout, and model-selection defaults from the selected profile while keeping explicit env vars authoritative.
  - `crates/cli/src/ai_commands.rs` now exposes `ntk ai profiles list` and `ntk ai profiles show [profile]`.
- Target paths:
  - `crates/orchestrator/src/execution/ai.rs`
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/cli/src/ai_commands.rs`
  - `planning/completed/plan-free-llm-provider-test-matrix.md`
- Commit checkpoint:
  - `feat(ai): define development provider profiles`

### [2026-04-04 00:00] Task D2: Add Runtime Doctor And Report Surfaces

- Define `ntk ai doctor` and `ntk ai doctor --json` as explicit operator surfaces.
- Validate configuration, env resolution, provider reachability, auth readiness, profile resolution, and fallback readiness.
- Add a report mode for operator troubleshooting that stays read-only and safe for local use.
- Keep diagnostics independent from request execution so failures in one path do not hide failures in the other.
- Completed slice:
  - `crates/orchestrator/src/execution/ai_doctor.rs` now owns read-only AI runtime diagnostics and Markdown report rendering.
  - `crates/cli/src/ai_commands.rs` now exposes `ntk ai doctor`, `ntk ai doctor --json-output`, and `ntk ai doctor --report-path`.
  - tests cover local/mock, remote readiness, provider-chain override, JSON output, and Markdown report generation.
- Target paths:
  - `crates/cli/src/ai_commands.rs`
  - `crates/orchestrator/src/execution/`
  - `planning/active/plan-runtime-operational-diagnostics-and-observability.md`
- Commit checkpoint:
  - `feat(ai): add runtime doctor surfaces`

### [2026-04-04 00:00] Task D3: Define Smart Routing And Fallback Scoring

- Add a routing model that scores candidate providers using latency, cost, quota/failure state, and policy fit.
- Keep routing strategy configurable with clear operator modes such as `latency`, `balanced`, and `cost`.
- Ensure fallback remains explicit and observable rather than hidden behind silent retries.
- Reuse the token-economy workstream for budget policy instead of redefining cost controls here.
- Completed slice:
  - `crates/orchestrator/src/execution/ai_routing.rs` now owns provider-chain resolution, routing-strategy selection, timeout budgets, and scored provider ordering.
  - `crates/orchestrator/src/execution/processor.rs` now consumes the scored routing plan before building provider routes, and logs attempt scores/rationales during failover execution.
  - `crates/orchestrator/src/execution/ai_doctor.rs` and `crates/cli/src/ai_commands.rs` now expose the resolved strategy, ordered provider chain, and scored candidates for operator inspection.
- Target paths:
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/orchestrator/src/execution/ai_usage.rs`
  - `planning/active/plan-token-economy-optimization.md`
  - `planning/completed/plan-free-llm-provider-test-matrix.md`
- Commit checkpoint:
  - `feat(ai): add smart provider routing strategy`

### [2026-04-04 00:00] Task D4: Define Normalized Provider Adapter Contracts

- Keep provider-specific transport, auth, streaming, usage, and error semantics behind adapters.
- Define a normalized contract for:
  - chat requests/responses
  - streaming chunks
  - usage and cost metadata
  - typed error mapping
- Ensure this boundary does not leak gateway-native quirks into the orchestrator core.
- Completed slice:
  - `crates/orchestrator/src/execution/ai.rs` now exposes normalized adapter descriptors for transport, auth, streaming, usage-reporting, and fallback-output capabilities.
  - `crates/orchestrator/src/execution/ai_doctor.rs` and `crates/cli/src/ai_commands.rs` now surface those adapter contracts to operators alongside provider routing.
  - `crates/orchestrator/tests/execution/ai_adapter_contract_tests.rs` now locks the adapter contract for `mock` and `openai-compatible`.
- Target paths:
  - `crates/orchestrator/src/execution/ai.rs`
  - `crates/orchestrator/src/execution/mod.rs`
  - `planning/active/plan-mcp-transport-auth-and-session-resilience.md`
- Commit checkpoint:
  - `refactor(ai): normalize provider adapter contracts`

### [2026-04-04 00:00] Task D5: Create Development Operator Playbook

- Create a dedicated playbook for local AI development operations instead of overloading the root README.
- Cover:
  - bootstrap
  - profile selection
  - doctor/report usage
  - fallback troubleshooting
  - degraded provider behavior
  - local vs remote provider guidance
- Keep the root README concise and link out to the playbook.
- Completed slice:
  - `docs/operations/ai-development-operator-playbook.md` now provides the stable human-facing runbook for profile selection, doctor usage, Markdown reports, local-vs-remote guidance, and degraded-state recovery.
  - `README.md`, `docs/README.md`, `crates/cli/README.md`, and `crates/orchestrator/README.md` now link to the playbook instead of overloading their local AI sections with operator detail.
- Target paths:
  - `docs/operations/`
  - `README.md`
  - `definitions/instructions/governance/`
- Commit checkpoint:
  - `docs(ai): add development operator playbook`

### [2026-04-04 00:00] Task D6: Define Agent-To-Model Routing Policy

- Allow different agents or skills to declare preferred model tiers or provider profiles.
- Keep the routing policy explicit, inspectable, and overrideable by operators.
- Ensure model routing complements rather than bypasses token-economy and provider-profile policy.
- Tie lineage and inherited settings into the multi-agent runtime plan instead of duplicating that work here.
- Completed slice:
  - `definitions/agents/*/model-routing.policy.json` and `definitions/skills/*/model-routing.policy.json` now define canonical lane defaults for profile/model selection.
  - `crates/orchestrator/src/execution/ai_model_routing.rs` now resolves lane defaults, explicit env activation, and profile precedence without leaking lane policy into provider adapters.
  - `crates/orchestrator/src/execution/processor.rs`, `crates/orchestrator/src/execution/ai_doctor.rs`, and `crates/cli/src/ai_commands.rs` now expose lane-aware model routing through execution, diagnostics, and `ntk ai model-routing list/show`.
- Target paths:
  - `crates/orchestrator/src/execution/`
  - `definitions/instructions/development/`
  - `planning/active/plan-multi-agent-runtime-lineage-and-a2a-readiness.md`
- Commit checkpoint:
  - `feat(agentic): add agent model routing policy`

### [2026-04-04 00:00] Task D7: Integrate The Orchestrator Experience Into Governance

- Update active planning indexes, runtime comparison matrix, and README summaries so the orchestrator experience is visible as a first-class workstream.
- Keep this plan as the umbrella and continue implementing details in the narrower owner plans where appropriate.
- Close this plan only after profiles, diagnostics, routing, adapters, playbook, and agent-routing are materially implemented or formally delegated.
- Completed slice:
  - README, crate READMEs, and the AI operator playbook now document `ntk ai doctor`, `ntk ai model-routing`, and lane-aware profile/model precedence.
  - Related active plans now record the delivered agent/skill routing proof where it affects provider-matrix, multi-agent, and token-economy workstreams.
  - The umbrella plan/spec have been moved to `planning/completed/` and `planning/specs/completed/` because the orchestrator experience is now a delivered baseline rather than an open design umbrella.
- Commit checkpoint:
  - `docs(planning): integrate development orchestrator umbrella`

---

## Validation Checklist

- `git diff --check`
- planning-only slices must keep `planning/README.md` and `planning/specs/README.md` synchronized
- implementation slices must additionally reuse validation from the owning narrower plans

---

## Risks And Mitigations

- Provider-profile UX can sprawl if it duplicates runtime/provider configuration.
- Smart routing can hide operator intent if scores are not observable.
- Diagnostics can become noisy if they are coupled to mutating execution paths.
- Agent-to-model routing can undermine cost controls if it bypasses token-economy policy.
- Mitigation: keep this plan as an umbrella, keep provider adapters narrow, make routing/status inspectable, and reuse existing owner plans for implementation details.

---

## Specialist And Closeout

- Recommended specialist: `plan-active-work-planner`
- Implementation specialists:
  - `dev-rust-engineer`
  - `docs-release-engineer`
  - `obs-sre-observability-engineer`
- Tester: required once implementation starts
- Reviewer: required
- Release closeout: required when implementation lands
- README update: required
- Changelog: required
- Suggested commit message style:
  - `docs(planning): open development agent orchestrator experience`
  - `docs(planning): integrate development agent orchestrator umbrella`