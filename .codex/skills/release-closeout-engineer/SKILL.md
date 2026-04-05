---
name: release-closeout-engineer
description: Close out a completed workstream by updating README artifacts when needed, producing a commit message, and applying changelog-ready content when the work is ready for commit. Use after review when a stable checkpoint is ready.
---

# Release Closeout Engineer

## Load minimal context first

1. Load `definitions/providers/github/root/AGENTS.md`.
2. Load `definitions/providers/github/root/copilot-instructions.md`.
3. Load `definitions/providers/github/root/instruction-routing.catalog.yml`.
4. Load `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`.
5. Load `definitions/agents/super-agent/ntk-agents-super-agent.instructions.md`.
6. Load `definitions/instructions/governance/ntk-governance-feedback-changelog.instructions.md`.
7. Load `definitions/instructions/governance/ntk-governance-readme.instructions.md` when README files are in scope.
8. Reuse the shared `$docs-release-engineer` skill for documentation and changelog behavior.

## Responsibilities

- determine whether README updates are required for the completed workstream and produce the final file content when they are
- prepare a commit message suggestion in English using repository commit conventions
- prepare and apply changelog-ready content when the change belongs in version history
- keep closeout artifacts concise, accurate, and aligned with the implemented scope

## Output contract

1. closeout summary
2. README actions or confirmation that no README update is needed
3. structured README updates when files must be rewritten
4. suggested commit message
5. changelog summary plus structured changelog update payload
6. ready-to-commit flag