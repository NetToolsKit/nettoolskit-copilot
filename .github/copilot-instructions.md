# Global Instructions

Language:
- chat and user-facing terminal responses: pt-BR
- code, commits, docs, UI structure, and database artifacts: EN

## Always-On Policies
- Default quality bar is enterprise-grade engineering.
- Only relax to prototype mode when the user explicitly asks for `POC`, `spike`, or `informal test`.
- Preserve exact EOF state; the repository default is `insert_final_newline = false`.
- Keep this file global and thin; domain and repository detail belongs in canonical instructions under `instructions/`.

## Workspace Bootstrap
- Always load `AGENTS.md` first, then this file.
- In `workspace-adapter` mode, use the workspace-owned `.github/` surfaces first.
- In `global-runtime` mode, use `%USERPROFILE%\\.github` and do not assume the runtime repository routing catalog or `instructions/governance/ntk-governance-repository-operating-model.instructions.md` applies to the target workspace.
- When no local planning surface exists, use `.build/super-agent/planning/` and `.build/super-agent/specs/`.

## Hierarchy and Canonical References
- User prompt wins first.
- Then apply `AGENTS.md` and this file.
- Then apply the most specific matching instruction under `instructions/`.
- Canonical global references for this provider surface:
  - `instructions/governance/ntk-governance-repository-operating-model.instructions.md`
  - `instructions/governance/ntk-governance-authoritative-sources.instructions.md`
  - `instructions/governance/ntk-governance-artifact-layout.instructions.md`
  - `governance/authoritative-source-map.json`
- The Super Agent lifecycle always comes from `agents/super-agent/ntk-agents-super-agent.instructions.md`.

## Routing and Execution
- Preferred `workspace-adapter` flow is `Route -> Execute`.
- Route with:
  - `.github/instruction-routing.catalog.yml`
  - `.github/prompts/route-instructions.prompt.md`
- Execute using only the mandatory global-core files plus the routed context pack.
- Do not duplicate large domain maps here; the routing catalog and repository operating model own that detail.

## Change-Bearing Workflow
- For change-bearing work, follow the Super Agent lifecycle:
  1. intake and clarification when ambiguity materially affects scope or safety
  2. spec registration for non-trivial design-bearing work
  3. planning registration
  4. specialist selection
  5. execution
  6. validation
  7. review
  8. closeout
  9. planning update
- The detailed workflow contract lives in:
  - `agents/super-agent/ntk-agents-super-agent.instructions.md`
  - `instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md`
  - `instructions/governance/ntk-governance-brainstorm-spec-workflow.instructions.md`
  - `instructions/governance/ntk-governance-worktree-isolation.instructions.md`
  - `instructions/governance/ntk-governance-tdd-verification.instructions.md`

## Validation and Output Discipline
- Every non-trivial task must keep a scope-specific validation checklist.
- Final reporting should default to: outcome, affected area, validation status, and blockers/risks when they exist.
- Keep outputs concise, but do not hide failures, skipped checks, or validation gaps.
- Include a final `Agents used:` line in substantive completions.
- When the state is stable, provide a semantic commit message suggestion and say the work is ready to commit.

## Authoritative Sources Policy
- Use repository context first for project-specific behavior.
- Use `instructions/governance/ntk-governance-authoritative-sources.instructions.md` for external platform, framework, SDK, API, CLI, or tool behavior.
- Use `governance/authoritative-source-map.json` as the canonical official-domain registry.

## Mandatory Baseline
- `AGENTS.md`
- `agents/super-agent/ntk-agents-super-agent.instructions.md`
- `instructions/governance/ntk-governance-artifact-layout.instructions.md`
- `instructions/governance/ntk-governance-authoritative-sources.instructions.md`
- `instructions/governance/ntk-governance-feedback-changelog.instructions.md`
- `instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md`
- `instructions/governance/ntk-governance-workflow-optimization.instructions.md`
- `instructions/operations/ntk-operations-powershell-execution.instructions.md`
- `instructions/governance/ntk-governance-repository-operating-model.instructions.md` only when the workspace exposes a local adapter

## Repository and Transparency
- Repository-specific topology, commands, style, and release rules live in `instructions/governance/ntk-governance-repository-operating-model.instructions.md`.
- Use `instructions/governance/ntk-governance-copilot-instruction-creation.instructions.md` when editing `.github` instruction surfaces.
- Consolidate detailed auditing in plans, PRs, and `CHANGELOG.md` instead of bloating the global-core files.