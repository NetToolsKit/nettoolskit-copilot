---
name: master-orchestrator
description: Use when starting any repository work that may change files, runtime assets, planning state, docs, settings, or governance. Normalize the request into the repository-owned lifecycle: master intake, planning registration, specialist routing, execution, testing, review, closeout, and planning-state updates before implementation begins.
---

# Master Orchestrator

## Load minimal context first

1. Load `.github/AGENTS.md`.
2. Load `.github/copilot-instructions.md`.
3. Load `.github/instruction-routing.catalog.yml`.
4. Load `.github/instructions/master-orchestrator.instructions.md`.
5. Load `.github/instructions/repository-operating-model.instructions.md`.
6. Load `.github/instructions/subagent-planning-workflow.instructions.md`.
7. Reuse the shared `$plan-active-work-planner`, `$context-token-optimizer`, and `$release-closeout-engineer` skills as downstream stages.

## Responsibilities

- normalize the request before planning or implementation
- act as the first controller for any change-bearing repository request, even when not explicitly named
- decide whether the task is change-bearing, non-trivial, or safe to keep lightweight
- create or update the active plan for any change-bearing workstream
- identify the smallest correct specialist chain
- keep execution sequential by default and allow multiple subagents only when write-scope conflicts are controlled
- require tester, reviewer, and closeout before claiming repository work is complete
- always prepare a commit message suggestion and update planning state after execution

## Required lifecycle

1. MASTER intake
2. planning registration
3. specialist identification
4. execution
5. testing
6. code review
7. closeout
8. planning update

## Invocation rule

- In Codex, this skill should be the first repository-owned controller for change-bearing work whenever skill discovery can match it.
- In Copilot, the same lifecycle is enforced through `instructions/master-orchestrator.instructions.md` even though Copilot does not execute skills directly.

## Output contract

1. normalized request summary
2. plan decision and active plan path
3. selected specialist chain
4. execution mode (`sequential` or `parallel-safe`)
5. validation obligations
6. closeout obligations
7. planning update instructions