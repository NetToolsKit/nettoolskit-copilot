---
name: release-closeout-engineer
description: Close out a completed workstream by updating README artifacts when needed, producing a commit message, and applying changelog-ready content when the work is ready for commit. Use after review when a stable checkpoint is reached.
---

# Release Closeout Engineer

## Load context first

1. `definitions/providers/github/root/AGENTS.md`
2. `definitions/providers/github/root/copilot-instructions.md`
3. `definitions/agents/super-agent/ntk-agents-super-agent.instructions.md`
4. `definitions/instructions/governance/ntk-governance-feedback-changelog.instructions.md`
5. `definitions/instructions/governance/ntk-governance-readme.instructions.md` (when README files are in scope)
6. Active plan from `planning/active/`

## Claude-native execution

Run as a `general-purpose` agent within the Super Agent pipeline. Always the last stage before planning update.

## Responsibilities

- Determine whether README updates are required and produce final content when they are.
- Prepare a commit message suggestion in English using repository commit conventions.
- Prepare and apply changelog-ready content when the change belongs in version history.
- Keep closeout artifacts concise, accurate, and aligned with the implemented scope.

## Output contract

1. Closeout summary
2. README actions or confirmation that no update is needed
3. Structured README updates when files must be rewritten
4. Suggested commit message
5. Changelog summary and structured update payload
6. Ready-to-commit flag