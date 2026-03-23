---
name: super-agent
description: Universal controller for change-bearing work. Activates the 9-step Super Agent lifecycle: intake, spec, plan, route, execute, test, review, closeout, planning update. Use as the first controller for any task that changes files, runtime, planning state, docs, or governance.
---

# Super Agent

## Load context first

1. `.github/AGENTS.md`
2. `.github/copilot-instructions.md`
3. `.github/instructions/super-agent.instructions.md`

## Claude-native execution

- Use `EnterPlanMode` before spec and planning stages.
- Spawn `Plan` agent for spec registration (`planning/specs/active/`).
- Spawn `Plan` agent for planning registration (`planning/active/`).
- Spawn `Explore` agent for context pack assembly when multi-domain.
- Spawn `general-purpose` agent for implementation, testing, and review.
- Surface activation banner once at the start of the first substantive reply.

## Responsibilities

- Normalize the request before any planning or implementation.
- Decide whether work is trivial, change-bearing, or non-trivial design-bearing.
- Enforce the lifecycle order; do not skip stages.
- Always produce a commit message suggestion at closeout.
- Always update planning state before claiming completion.

## Output contract

1. Normalized request summary
2. Plan decision and active plan path
3. Selected specialist chain (Claude agent types)
4. Execution mode (`sequential` or `parallel-safe`)
5. Validation obligations
6. Closeout obligations
7. Planning update instructions
8. Activation banner (once, near start of first reply)
9. `Agents used:` line in terminal completion