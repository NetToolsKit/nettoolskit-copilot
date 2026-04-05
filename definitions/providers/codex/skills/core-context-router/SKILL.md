---
name: core-context-router
description: Route tasks using this repository's static routing catalog and load only the minimal context pack.
---

# Repo Context Router

Use this skill when the task should follow the repo routing model before execution.

## Always Load First

1. `definitions/providers/github/root/AGENTS.md`
2. `definitions/providers/github/root/copilot-instructions.md`
3. `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`

## Route Then Execute

1. Route with:
   - `definitions/providers/github/root/instruction-routing.catalog.yml`
   - `definitions/providers/github/prompts/route-instructions.prompt.md`
2. Build a minimal context pack.
3. Execute using only the files selected by the context pack.

## Rules

- Keep context minimal (2-5 domain files whenever possible).
- Prefer the most specific instruction by scope/path on conflicts.
- If route is ambiguous, ask up to 3 short clarifying questions.