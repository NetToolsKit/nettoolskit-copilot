---
name: super-agent
description: Use when starting any workspace work that may change files, runtime assets, planning state, docs, settings, or governance. Normalize the request into the Super Agent lifecycle: intake, planning registration, specialist routing, execution, testing, review, closeout, and planning-state updates before implementation begins.
---

# Super Agent

## Load minimal context first

1. If the workspace provides local `.github/AGENTS.md` and `.github/copilot-instructions.md`, load them first.
2. Otherwise load the mirrored runtime `~/.github/AGENTS.md` and `~/.github/copilot-instructions.md`.
3. Load `.github/instructions/super-agent.instructions.md` when the workspace provides it, otherwise use the mirrored runtime copy under `~/.github/instructions/`.
4. Load `.github/instructions/subagent-planning-workflow.instructions.md` when the workspace provides it, otherwise use the mirrored runtime copy.
5. Only load `.github/instruction-routing.catalog.yml` and `.github/prompts/route-instructions.prompt.md` when the workspace actually provides them.
6. Only load `.github/instructions/repository-operating-model.instructions.md` when the workspace actually provides a local repo adapter and repo-specific operating model.
7. Reuse the shared `$plan-active-work-planner`, `$context-token-optimizer`, and `$release-closeout-engineer` skills as downstream stages.

## Responsibilities

- normalize the request before planning or implementation
- act as the first controller for any change-bearing workspace request, even when not explicitly named
- decide whether the task is change-bearing, non-trivial, or safe to keep lightweight
- create or update the active plan for any change-bearing workstream under `planning/` when available, otherwise under `.build/super-agent/`
- identify the smallest correct specialist chain
- keep execution sequential by default and allow multiple subagents only when write-scope conflicts are controlled
- require tester, reviewer, and closeout before claiming workspace work is complete
- always prepare a commit message suggestion and update planning state after execution

## Required lifecycle

1. Super Agent intake
2. planning registration
3. specialist identification
4. execution
5. testing
6. code review
7. closeout
8. planning update

## Invocation rule

- In Codex, this skill should be the first repository-owned controller for change-bearing work whenever skill discovery can match it.
- In Copilot, the same lifecycle is enforced through `instructions/super-agent.instructions.md` even though Copilot does not execute skills directly.
- In workspaces without a local adapter, stay in `global-runtime` mode: do not assume the `copilot-instructions` routing catalog or repository operating model applies to the target repo.

## Output contract

1. normalized request summary
2. plan decision and active plan path
3. selected specialist chain
4. execution mode (`sequential` or `parallel-safe`)
5. validation obligations
6. closeout obligations
7. planning update instructions