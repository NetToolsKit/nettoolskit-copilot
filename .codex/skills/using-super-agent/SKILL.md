---
name: using-super-agent
description: Use when starting any repository conversation that may lead to changes. Load the repository-owned Super Agent lifecycle first so planning, routing, validation, review, closeout, and planning-state updates happen in the correct order.
---

# Using Super Agent

## Purpose

Use this starter skill when you want the repository-owned workflow controller to be obvious in the picker and at session start.

## Invocation rule

- Use this at the beginning of repository work that may change files, runtime assets, settings, docs, planning state, or governance.
- After loading this skill, immediately invoke `$super-agent`.
- Do not replace the repository lifecycle with external starter skills when the repository already provides `Super Agent`.

## Required handoff

1. Load `.github/AGENTS.md`.
2. Load `.github/copilot-instructions.md`.
3. Load `.github/instructions/repository-operating-model.instructions.md`.
4. Invoke `$super-agent`.
5. Let `Super Agent` decide planning, specialist routing, execution mode, validation, review, closeout, and planning update.

## Output contract

1. repository context loaded
2. super-agent invoked
3. lifecycle controller active