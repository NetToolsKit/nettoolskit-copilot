---
name: plan-task-planner
description: Plan and sequence complex implementation work into deterministic, testable steps for this repository. Use when the user asks for planning, roadmap breakdown, execution strategy, effort estimation, or multi-step delivery coordination across domains.
---

# Task Planner

## Load minimal context first

1. Load `definitions/providers/github/root/AGENTS.md`, `definitions/providers/github/root/copilot-instructions.md`, and `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`.
2. Route with `definitions/providers/github/root/instruction-routing.catalog.yml` and `definitions/providers/github/prompts/route-instructions.prompt.md`.
3. Load only planning/process files plus the domain packs needed by the plan.

## Planning instruction pack

- `definitions/instructions/governance/ntk-governance-workflow-optimization.instructions.md`
- `definitions/instructions/governance/ntk-governance-effort-estimation-ucp.instructions.md`
- `definitions/instructions/governance/ntk-governance-pr.instructions.md` (when output is PR-oriented)
- `definitions/instructions/governance/ntk-governance-feedback-changelog.instructions.md` (when release/change log impact exists)

## Planning workflow

1. Define objective, constraints, assumptions, and acceptance criteria.
2. Break work into small ordered tasks with dependencies.
3. Add exact target files or the narrowest safe path scope for each task.
4. Add explicit runnable commands and expected fail/pass checkpoints per task.
5. Add validation per task (build, tests, smoke checks).
6. Identify risks and fallback path for each critical task.
7. Add stable commit checkpoint suggestions for meaningful delivery slices.
8. Produce an execution order that can be run incrementally.

## Output contract

1. Scope summary.
2. Ordered tasks.
3. Per-task target paths, commands, checkpoints, and commit checkpoints.
4. Validation checklist.
5. Risk list and mitigation.
6. Delivery slices (POC, incremental, final).