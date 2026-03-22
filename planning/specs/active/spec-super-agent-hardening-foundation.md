# Super Agent Hardening Foundation

Generated: 2026-03-22

## Objective

Harden the repository-owned Super Agent runtime with a shared foundation for contextual policy enforcement, structured tracing, checkpoint-aware resume, eval execution, and model routing without replacing the current Codex/Copilot-based orchestration architecture.

## Normalized Request Summary

The user wants the repository to absorb the highest-value operational capabilities seen in popular agent systems on GitHub and start implementing them now. The improvements must preserve the current repo-owned Super Agent, avoid intrusive platform rewrites, and favor reusable shared runtime contracts over duplicated logic.

## Design Summary

The implementation will extend the current orchestration runner instead of introducing an external agent framework. The foundation will add:

1. a versioned policy catalog for contextual stage/tool-call guardrails
2. a structured trace artifact for stage timing, decisions, and dispatch metadata
3. a checkpoint artifact for deterministic resume points
4. a lightweight resume entrypoint that continues from the last safe checkpoint
5. a local eval runner that measures pipeline behavior against versioned fixtures
6. a versioned model-routing catalog that resolves the effective model per stage/agent

The first implementation slice will land these capabilities as repository-owned runtime artifacts and shared helpers. Sandbox isolation and full A2A interoperability are intentionally deferred because they require broader runtime boundaries than the current PowerShell-first orchestration foundation safely provides in one slice.

## Key Decisions

1. Keep `scripts/runtime/run-agent-pipeline.ps1` as the authoritative runner and extend it through shared helpers instead of adding a second orchestration engine.
2. Version policy, routing, trace, and checkpoint contracts under `.github/governance/`, `.github/schemas/`, and `.codex/orchestration/templates/`.
3. Treat trace, policy, and checkpoint state as first-class run artifacts under `.temp/runs/<traceId>/`.
4. Keep the first policy engine deterministic and local: rule evaluation is based on stage metadata, planned commands, approval state, changed paths, and dispatch records already available inside the runner.
5. Start model routing as a repository-owned catalog that selects an effective model per stage/agent while preserving current defaults.
6. Add resume and replay as explicit entrypoints instead of hiding retry semantics inside hooks.

## Alternatives Considered

1. Replace the runtime with LangGraph, OpenHands, CrewAI, or another external framework.
   - Rejected because it would discard repository-owned planning, validation, and governance value.
2. Add only documentation and roadmap items for future phases.
   - Rejected because it would not materially improve safety or observability.
3. Build sandbox isolation first.
   - Rejected because trace, policy, and checkpoint artifacts are the safer dependency foundation for later sandboxing.

## Assumptions And Constraints

- The repository-owned Super Agent pipeline remains the source of truth.
- Existing validations must remain green.
- The runtime must stay PowerShell-first and compatible with the current Codex/Copilot setup.
- Shared helpers should reduce duplication rather than create another layer of ad-hoc script-local functions.
- Resume must only continue from completed safe stages; it must not guess partial stage recovery.

## Risks

- New hardening artifacts can add noise if they are too verbose or not versioned cleanly.
- Policy rules that are too coarse can block normal work and create workflow friction.
- Model routing can drift from agent contracts if validations are not tightened.
- Resume logic can become unsafe if checkpoints are not explicit about stage success boundaries.

## Acceptance Criteria

1. A versioned hardening foundation spec exists in `planning/specs/active/`.
2. The runner writes structured trace, policy, and checkpoint artifacts for each run.
3. A versioned policy catalog is enforced before and after sensitive stage execution.
4. A versioned model-routing catalog resolves effective stage/agent model selection and records it in artifacts.
5. A resume entrypoint can continue a run from the last safe completed stage.
6. A replay/eval entrypoint can summarize run artifacts and execute versioned eval fixtures.
7. Runtime schemas, templates, tests, and docs are updated and validation remains green.

## Planning Readiness Statement

Planning is ready. The scope is broad but cohesive because all requested improvements depend on one shared runtime foundation. Execution should start with shared contracts and runner integration before adding entrypoints and eval coverage.

## Recommended Specialist Focus

- Primary: `ops-devops-platform-engineer`
- Secondary: `sec-api-performance-security-engineer`
- Follow-on review: `obs-sre-observability-engineer`, `review-code-engineer`, `release-closeout-engineer`