---
name: plan-task-planner
description: Plan and sequence complex implementation work into deterministic, testable steps. Use when the user asks for planning, roadmap breakdown, execution strategy, effort estimation, or multi-step delivery coordination across domains.
---

# Task Planner

## Load context first

1. `definitions/providers/github/root/AGENTS.md`
2. `definitions/providers/github/root/copilot-instructions.md`
3. `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`

## Planning instruction pack

- `definitions/instructions/governance/ntk-governance-workflow-optimization.instructions.md`
- `definitions/instructions/governance/ntk-governance-effort-estimation-ucp.instructions.md`
- `definitions/instructions/governance/ntk-governance-pr.instructions.md` (when output is PR-oriented)
- `definitions/instructions/governance/ntk-governance-feedback-changelog.instructions.md` (when release/change log impact exists)

## Claude-native execution

Run as a `Plan` agent. Use `EnterPlanMode` before planning. Part of the Super Agent pipeline after spec registration.

## Planning workflow

1. Define objective, constraints, assumptions, and acceptance criteria.
2. Break work into small ordered tasks with dependencies.
3. Add exact target files or the narrowest safe path scope per task.
4. Add explicit runnable commands and expected checkpoints per task.
5. Add validation per task (build, tests, smoke checks).
6. Identify risks and fallback path for each critical task.
7. Add stable commit checkpoint suggestions for meaningful delivery slices.
8. Produce an execution order that can be run incrementally.

## Output contract

1. Scope summary
2. Ordered tasks with target paths, commands, and checkpoints
3. Validation checklist
4. Risk list and mitigation
5. Delivery slices (incremental → final)