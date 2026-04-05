---
applyTo: "**/*.{cs,csproj,sln,ps1,rs,toml,ts,tsx,js,jsx,vue,yml,yaml,json,jsonc,md,sql}"
priority: high
---

# Agent Model Routing

Use this instruction when the repository changes how agents, skills, provider profiles,
or model tiers are selected for AI-assisted development work.

## Purpose

- keep agent and skill defaults explicit instead of hidden in prompts
- preserve operator override precedence for profiles and model-selection env vars
- make agent-to-model routing inspectable through stable runtime surfaces
- keep agent routing separate from provider transport, MCP projection, and A2A concerns

## Canonical Sources

- `definitions/agents/*/model-routing.policy.json`
- `definitions/skills/*/model-routing.policy.json`
- `crates/orchestrator/src/execution/`

## Rules

- Agent and skill routing defaults must be versioned under `definitions/`, not authored directly in projected runtime folders.
- `NTK_AI_PROFILE` remains the highest-priority profile selection control for operators.
- `NTK_AI_ACTIVE_AGENT` and `NTK_AI_ACTIVE_SKILL` may provide default profile/model hints only when explicit profile/model env overrides are absent.
- Skill-level routing may override agent-level routing when both are active because the skill is the narrower workload boundary.
- Routing policy must stay inspectable through operator commands or diagnostics; do not hide lane-derived defaults inside provider adapters.
- Agent-to-model routing complements token economy; it must not bypass request budget, cost guardrails, or prompt-compaction policy.
- Internal agent routing must not be documented as A2A. It is an internal orchestration concern unless a real interoperability protocol surface exists.

## Documentation Expectations

- Operator-facing docs should explain the precedence chain:
  - explicit model-selection env vars
  - explicit `NTK_AI_PROFILE`
  - active skill routing defaults
  - active agent routing defaults
  - built-in profile defaults
- README summaries should stay concise and link to the operator playbook for detail.