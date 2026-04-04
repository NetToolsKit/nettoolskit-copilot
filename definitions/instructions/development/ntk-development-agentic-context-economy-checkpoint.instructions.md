---
applyTo: "**/*.{cs,csproj,sln,ps1,rs,toml,ts,tsx,js,jsx,vue,yml,yaml,json,jsonc,md,sql}"
priority: high
---

# Context Economy and Checkpoint Protocol

## Purpose
This file owns only the in-session context economy and checkpoint protocol.

- automatic context compression
- checkpoint formatting
- continuity block structure
- command vocabulary for checkpoint and progress operations

It does not define MCP, A2A, RAG, or CAG boundaries by itself. Use
`ntk-agentic-surfaces.instructions.md` for the architecture-level separation of
those concepts.

## Core Principle
Treat conversation history as temporary and disposable.
Maintain a compact, structured operational memory that is continuously updated.
Continuity depends on the active plan artifact and this checkpoint model — not on raw history.

## Operating Modes

### EXECUTION
Respond to and execute tasks normally.

### CONTINUOUS COMPRESSION
Silently consolidate context when any compression trigger fires (see below).
Never announce compression unless the user asks for a checkpoint.

### STRUCTURED CHECKPOINT
When resuming or transitioning phases, use the compressed state as the primary continuity source.
Show the CHECKPOINT block only when triggered, never proactively.

## Compression Triggers
Auto-compress when at least one condition applies:
- a task completes
- a planning phase ends
- a technical decision is taken and closed
- the topic shifts within the same project
- a refactor or architecture decision is finalized
- the response is long with many transitory details
- a new project phase begins
- context irrelevant to the next step has accumulated

## What Compression Preserves vs. Discards

**Preserve:**
- active state (what is in progress)
- confirmed decisions (architecture, patterns, constraints)
- pending items and open questions
- next concrete step

**Discard (compact or drop):**
- resolved discussion threads
- rejected or abandoned alternatives
- already-delivered explanations not needed again
- duplication of context already in the active plan

## Internal State Model (six blocks)
Maintain this structure as internal working memory:

1. **Current state** — what is being done now; affected area; immediate technical goal
2. **In progress** — active tasks, subtasks in progress, current blockers
3. **Completed** — what was implemented, validated, or decided; no longer needed in active context unless it affects next steps
4. **Decisions** — defined patterns, chosen architecture, accepted constraints, established conventions
5. **Pending items** — fragile points, open questions, future validations needed
6. **Next step** — exact next item, suggested order, dependencies

## CHECKPOINT Format
Use this exact format when showing a checkpoint:

```
CHECKPOINT
Current state: ...
In progress: ...
Completed: ...
Decisions: ...
Pending items: ...
Next step: ...
```

## When to Show the CHECKPOINT
Show the CHECKPOINT only when:
- the user explicitly requests it
- a phase is ending and safe handoff requires it
- continuity after a context reset would be ambiguous without it

Never show CHECKPOINT proactively in the middle of active execution.

## Recognized User Commands
Execute immediately when received. English is the canonical form; PT-BR aliases are listed in `.github/COMMANDS.md`.

| Command | Behavior |
|---|---|
| `checkpoint` | Output the full CHECKPOINT block |
| `compress context` | Apply compression immediately; confirm silently |
| `update plan` | Update the active plan artifact (`planning/active/`) with current state |
| `show status` | Output Current state block only |
| `show progress` | Output Completed + Next step blocks |
| `resume from summary` | Drop raw history; resume from last CHECKPOINT |

## Priority Order
1. Continuity of the work
2. Token economy
3. Preservation of important decisions
4. Technical clarity
5. Historical detail

A detail that does not influence the next execution step must be compressed or dropped from active memory.

## Relationship to Planning Artifacts
This protocol complements the planning-first continuity model.

- The active plan under `planning/active/` remains the authoritative durable resume point.
- The CHECKPOINT is the in-session working memory tool.

## Relationship to Other Instructions
- `ntk-agents-super-agent.instructions.md` owns lifecycle, planning-first continuity, and session-boundary handling.
- `ntk-process-workflow-optimization.instructions.md` owns output-discipline and token-economy guidance.
- `ntk-agentic-surfaces.instructions.md` owns the architectural separation of MCP, A2A, RAG, and CAG.

Do not duplicate those responsibilities here.