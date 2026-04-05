# Free LLM Provider Test Matrix Spec

Generated: 2026-03-31 17:37

## Status

- LastUpdated: 2026-04-05 12:20
- Objective: define the design intent for a free-provider test matrix that covers the providers shown in `.docs/llm-free.png` while keeping provider-specific concerns separate from the main AI orchestration path.
- Normalized Request: create a planning workstream for using all listed free providers in tests in a controlled, maintainable, SOLID-aligned way.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-free-llm-provider-test-matrix.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Current Slice: the repository now exposes a generic AI provider trait, an OpenAI-compatible provider implementation, persisted AI usage reporting, a built-in provider-profile catalog, and a strategy-aware routing layer for `openai-compatible` and `local/mock` development presets, but the full free-provider matrix and live harness are still pending.

---

## Problem Statement

The repository already has an AI provider abstraction and usage reporting, but the free providers listed in the image differ in transport, stability, and operator expectations. Without a declared matrix, provider-specific assumptions can leak into orchestration, tests, and documentation, making the system harder to maintain and reason about.

The gap is not merely “add more providers”. The gap is to classify provider families, keep the runtime abstraction thin, and ensure the test matrix can exercise each free provider without coupling the production AI path to one vendor.

---

## Design Intent

- Keep provider families classified by mode instead of pretending they are interchangeable.
- Keep gateway-style providers, native APIs, and orchestrator-style surfaces separate in the design.
- Keep production provider selection independent from test-only provider evaluation.
- Keep OpenCode.ai treated as an orchestration/control-plane surface, not as a model family.
- Use the existing `AiProvider` contract as the lowest common denominator, but do not force every provider into an identical behavior model.
- Make usage reporting capable of identifying provider family, model, and fallback decisions.
- Keep test harness logic outside the main orchestration path so the runtime remains focused on execution, not evaluation.

---

## Provider Classification

The initial classification should be explicit and versioned:

- `OpenRouter` - gateway / aggregator
- `Groq` - API / fast inference
- `Google AI Studio` - native API
- `Together AI` - API / gateway-style provider
- `Hugging Face (Inference API)` - hosted inference API
- `NVIDIA (NIM preview)` - infra / preview runtime
- `OpenCode.ai` - orchestrator / proxy control plane

This classification can evolve, but the repository should store the mode and assumptions in one place instead of rediscovering them in every test.

---

## Options Considered

1. Hardcode provider-specific smoke tests directly into `processor.rs`.
   - Rejected: this would mix evaluation with runtime orchestration and increase coupling.
2. Add ad hoc scripts for each provider family.
   - Rejected: that would duplicate logic and make maintenance harder.
3. Create a provider matrix with explicit mode classification and a reusable harness.
   - Preferred: this keeps provider differences visible while preserving a single testing strategy.

---

## Proposed Boundaries

- `crates/orchestrator/src/execution/ai.rs` remains the provider abstraction and adapter home.
- `crates/orchestrator/src/execution/processor.rs` continues to orchestrate runtime execution, but should not own provider-family policy.
- `crates/cli/src/ai_commands.rs` remains the reporting surface for AI usage summaries and future provider-matrix reporting.
- Provider-family evaluation lives in dedicated tests and fixtures, not in the production request path.
- Documentation in `README.md` and `.github/instructions/*` should describe the matrix and mode classification, but not duplicate runtime logic.

---

## Acceptance Criteria

- Each free provider family is classified by transport mode and support tier.
- The repository has a reusable test matrix concept that can run provider smoke checks without changing runtime defaults.
- Production provider selection remains separate from evaluation-only provider testing.
- Usage reporting can identify provider family/model and fallback behavior.
- The README and instruction surfaces clearly explain where MCP, RAG, CAG, A2A, and the free-provider matrix fit.

---

## Planning Readiness

- The current provider abstraction and usage ledger are sufficient to begin design work.
- The next planning step should map the provider-mode matrix against the current `AiProvider` trait and `NTK_AI_PROVIDER_CHAIN` behavior.
- Implementation should start with the classification matrix and harness design before any vendor-specific adapter work.
- Initial implementation proof now exists for provider-mode boundaries through the built-in profile catalog and `ntk ai profiles` inspection surfaces, which can be reused when the wider free-provider matrix lands.