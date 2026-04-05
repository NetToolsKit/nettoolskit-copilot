# Free LLM Provider Test Matrix Plan

Generated: 2026-03-31 17:37

## Status

- LastUpdated: 2026-04-05 12:20
- Objective: compare, classify, and prepare the free AI providers shown in `.docs/llm-free.png` for deterministic test coverage without coupling the runtime to a single vendor surface.
- Normalized Request: create a planning workstream for evaluating all listed free providers and using them in repository tests through explicit, SOLID-aligned boundaries.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-free-llm-provider-test-matrix.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Current Slice: F2 now has both a built-in provider-profile catalog and a strategy-aware routing plan that classifies and orders the currently supported `openai-compatible` and `local/mock` modes without coupling production routing to a single vendor.
- Inputs:
  - `.docs/llm-free.png`
  - `crates/orchestrator/src/execution/ai.rs`
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/orchestrator/src/execution/ai_usage.rs`
  - `crates/cli/src/ai_commands.rs`
  - `planning/active/plan-token-economy-optimization.md`
  - `planning/active/plan-agentic-surface-boundary-separation.md`

---

## Scope Summary

This workstream formalizes a free-provider evaluation matrix and the support boundaries needed to test all listed providers in a controlled way.

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| F1 | Inventory and classify free providers | `.docs/llm-free.png` + provider abstraction + current AI routing | 🔴 Immediate | none |
| F2 | Define provider mode boundaries | `AiProvider` layer + provider profiles + transport adapters | 🔴 Immediate | F1 |
| F3 | Design the free-provider test harness | orchestrator/CLI tests + smoke matrix + evaluation prompts | 🟠 High | F1 |
| F4 | Define per-provider usage, quota, and fallback reporting | usage ledger + AI reporting commands | 🟠 High | F2 |
| F5 | Document the provider matrix and operator guidance | README + instructions + planning docs | 🟡 Medium | F1 |

Provider families covered by the matrix:

- OpenRouter
- Groq
- Google AI Studio
- Together AI
- Hugging Face (Inference API)
- NVIDIA (NIM preview)
- OpenCode.ai

This workstream treats OpenCode.ai as an orchestration/control-plane surface, not as a model family.

---

## Ordered Tasks

### [2026-03-31 17:37] Task F1: Freeze Free-Provider Capability Matrix

- Inventory the provider families shown in `.docs/llm-free.png`.
- Classify each provider by integration mode:
  - gateway / OpenAI-compatible
  - native API
  - orchestrator
- Record which providers are stable, which are experimental, and which should be treated as best-effort only.
- Capture the current repository touchpoints that already support AI provider routing and usage reporting.
- Commit checkpoint:
  - `docs(planning): freeze free provider capability matrix`

### [2026-03-31 17:37] Task F2: Define Provider Mode Boundaries

- Normalize provider configuration so each provider family has a declared transport shape and auth boundary.
- Keep vendor-specific endpoint/auth details behind adapters instead of leaking them into orchestration or tests.
- Map the provider modes to the current `AiProvider` abstraction and the existing `OpenAiCompatibleProvider` checkpoint.
- Keep production provider selection separate from evaluation-only provider selection.
- Implemented slice:
  - `crates/orchestrator/src/execution/ai_profiles.rs` now declares stable provider-mode metadata for built-in development presets.
  - `crates/orchestrator/src/execution/processor.rs` consumes those presets only as optional defaults layered underneath explicit env overrides.
  - `crates/cli/src/ai_commands.rs` exposes the preset catalog for operator inspection before live-provider harness work lands.
  - `crates/orchestrator/src/execution/ai_routing.rs` now declares explicit routing strategy and scored fallback order for the currently supported provider modes.
- Target paths:
  - `crates/orchestrator/src/execution/ai.rs`
  - `crates/orchestrator/src/execution/processor.rs`
  - `crates/orchestrator/src/execution/mod.rs`
- Commit checkpoint:
  - `refactor(ai): define free provider mode boundaries`

### [2026-03-31 17:37] Task F3: Design the Free-Provider Test Harness

- Define a matrix of smoke, quality, latency, streaming, and error-path tests that can run per provider family.
- Separate deterministic offline assertions from live-network smoke tests.
- Ensure the harness can reuse the same prompt set and expected output structure across providers.
- Keep provider-specific prompts and result normalization outside the main orchestration path.
- Target paths:
  - `crates/orchestrator/tests/execution/*`
  - `crates/cli/tests/ai_commands_tests.rs`
  - future provider matrix fixtures under `planning/` or `tests/`
- Commit checkpoint:
  - `test(ai): add provider matrix harness`

### [2026-03-31 17:37] Task F4: Define Usage, Quota, and Fallback Reporting

- Extend the existing AI usage reporting model so provider family and model identity are visible in reports.
- Define how free-tier quotas, hidden rate limits, and fallback behavior will be surfaced to operators.
- Keep usage reporting separate from provider selection so the reporting path remains read-only.
- Tie the matrix into the weekly/summary usage commands rather than creating a one-off reporting surface.
- Target paths:
  - `crates/orchestrator/src/execution/ai_usage.rs`
  - `crates/cli/src/ai_commands.rs`
  - `crates/orchestrator/tests/execution/ai_usage_tests.rs`
- Commit checkpoint:
  - `feat(ai): report free provider usage and fallback status`

### [2026-03-31 17:37] Task F5: Document the Matrix and Operator Guidance

- Add a concise provider matrix section to the repository README and instruction surfaces.
- Record the provider modes, operator caveats, and the distinction between gateway, native API, and orchestrator surfaces.
- Keep the README summary short and defer detail to the planning/spec docs.
- Target paths:
  - `README.md`
  - `.github/instructions/docs/ntk-docs-readme.instructions.md`
  - `.github/instructions/docs/ntk-docs-repository-readme-overrides.instructions.md`
  - `planning/active/plan-free-llm-provider-test-matrix.md`
  - `planning/specs/active/spec-free-llm-provider-test-matrix.md`
- Commit checkpoint:
  - `docs(agentic): document free llm provider matrix`

---

## Validation Checklist

- `git diff --check`
- `cargo test -p nettoolskit-orchestrator --quiet`
- `cargo test -p nettoolskit-cli --test ai_commands_tests --quiet`
- `cargo test -p nettoolskit-orchestrator --test test_suite ai_usage --quiet`
- future provider smoke checks for each enabled free provider family

---

## Risks And Mitigations

- Free tiers are unstable and can change limits without notice.
- Provider-specific prompt or streaming behavior can drift across vendors.
- Some providers are gateways, some are native APIs, and some are orchestrators; mixing them in one abstraction can hide important differences.
- Live-network smoke tests can be flaky if they are not separated from deterministic offline checks.
- Mitigation: classify provider modes explicitly, keep adapters thin, and make the evaluation harness read-only with explicit provider selection.

---

## Specialist And Closeout

- Recommended specialist: `dev-rust-engineer`
- Tester: required
- Reviewer: required
- Release closeout: required when implementation lands
- README update: required for the provider matrix and mode classification
- Changelog: required once implementation lands
- Suggested commit message style:
  - `docs(planning): open free llm provider test matrix`
  - `docs(planning): record free provider capability classification`