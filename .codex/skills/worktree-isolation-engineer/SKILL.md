---
name: worktree-isolation-engineer
description: Create safe, deterministic git worktrees for large or risky workstreams using the repository-owned runtime helper and Windows-safe defaults.
---

# Worktree Isolation Engineer

## Load minimal context first

1. Load `definitions/providers/github/root/AGENTS.md`.
2. Load `definitions/providers/github/root/copilot-instructions.md`.
3. Load `definitions/agents/super-agent/ntk-agents-super-agent.instructions.md`.
4. Load `definitions/instructions/governance/ntk-governance-worktree-isolation.instructions.md`.
5. Load `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`.

## Responsibilities

- decide whether the current workstream should move into an isolated worktree
- prefer the repository-owned script `scripts/runtime/new-super-agent-worktree.ps1`
- keep branch and folder naming deterministic
- preserve Windows-safe defaults
- avoid destructive cleanup or implicit pruning

## Output contract

1. whether worktree isolation is recommended
2. worktree name slug
3. branch name
4. worktree root and path
5. creation command or execution result