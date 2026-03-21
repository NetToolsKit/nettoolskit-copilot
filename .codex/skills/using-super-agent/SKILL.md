---
name: using-super-agent
description: Use when starting any workspace conversation that may lead to changes. Load the Super Agent lifecycle first so planning, routing, validation, review, closeout, and planning-state updates happen in the correct order.
---

# Using Super Agent

## Purpose

Use this starter skill when you want the workflow controller to be obvious in the picker and at session start.

## Invocation rule

- Use this at the beginning of repository work that may change files, runtime assets, settings, docs, planning state, or governance.
- If the target workspace has no local `.github` adapter, stay in `global-runtime` mode and use the mirrored runtime baseline under `~/.github`.
- After loading this skill, immediately invoke `$super-agent`.
- Do not replace the Super Agent lifecycle with external starter skills when the runtime already provides `Super Agent`.

## Required handoff

1. Load workspace `.github/AGENTS.md` and `.github/copilot-instructions.md` when they exist; otherwise load the mirrored runtime copies under `~/.github`.
2. Load workspace `.github/instructions/repository-operating-model.instructions.md` only when the target repo actually provides it.
3. Invoke `$super-agent`.
4. Let `Super Agent` decide whether the workspace is in `workspace-adapter` mode or `global-runtime` mode.
5. Let `Super Agent` decide planning roots, specialist routing, execution mode, validation, review, closeout, and planning update.

## Output contract

1. repository context loaded
2. super-agent invoked
3. lifecycle controller active