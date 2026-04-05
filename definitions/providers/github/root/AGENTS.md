# Copilot Agents and Context Policy

## Agents
- `Workspace`: code-first agent for repository changes, file edits, and validation work.
- `GitHub`: PR, issue, workflow, and repository artifact agent.
- `Profiler`: performance and benchmark agent for hot-path analysis.
- `VS`: IDE helper for build/debug/editor issues.

## Mandatory Context Bootstrap
- Always load `AGENTS.md` first, then `copilot-instructions.md`.
- Treat these two files as the thin global-core layer only; push detailed policy to canonical instructions under `instructions/`.
- Canonical governance references for this provider surface:
  - `instructions/governance/ntk-governance-repository-operating-model.instructions.md`
  - `instructions/governance/ntk-governance-authoritative-sources.instructions.md`
  - `governance/authoritative-source-map.json`

## Workspace Modes
- `workspace-adapter`:
  - active when the target workspace exposes local `.github/AGENTS.md` and `.github/copilot-instructions.md`
  - use workspace-owned routing when `.github/instruction-routing.catalog.yml` and `.github/prompts/route-instructions.prompt.md` exist
  - use versioned planning under `planning/` when the workspace exposes `planning/README.md` and `planning/specs/README.md`
- `global-runtime`:
  - active when the target workspace does not expose the local adapter above
  - load the mirrored runtime baseline from `%USERPROFILE%\\.github`
  - do not assume the runtime repository routing catalog or `instructions/governance/ntk-governance-repository-operating-model.instructions.md` applies to the target workspace
  - use `.build/super-agent/planning/` and `.build/super-agent/specs/` for transient orchestration continuity

## Routing and Instruction Selection
1. Load `AGENTS.md` and `copilot-instructions.md`.
2. In `workspace-adapter` mode, route first with:
   - `.github/instruction-routing.catalog.yml`
   - `.github/prompts/route-instructions.prompt.md`
3. Execute with the returned minimal context pack plus the mandatory global-core files.
4. In `global-runtime` mode, build a minimal local context pack manually from the target repo.

Precedence:
- user prompt
- `AGENTS.md` + `copilot-instructions.md`
- the most specific matching instruction under `instructions/`
- the safest minimal interpretation when ambiguity remains

## Super Agent Baseline
- Change-bearing work must follow `agents/super-agent/ntk-agents-super-agent.instructions.md`.
- Non-trivial work also follows:
  - `instructions/governance/ntk-governance-brainstorm-spec-workflow.instructions.md`
  - `instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md`
  - `instructions/governance/ntk-governance-worktree-isolation.instructions.md`
  - `instructions/governance/ntk-governance-tdd-verification.instructions.md`
  - `instructions/governance/ntk-governance-workflow-optimization.instructions.md`
  - `instructions/governance/ntk-governance-feedback-changelog.instructions.md`
- Repository-specific topology, commands, style, release process, and domain mapping live in `instructions/governance/ntk-governance-repository-operating-model.instructions.md`.

## Reporting and Output Economy
- In every substantive terminal-facing completion, include a final `Agents used:` line.
- Keep default user-facing outputs concise: outcome, affected area, validation, and blockers/risks when they exist.
- Do not restate large route packs, plan bodies, or validation logs when file references or short deltas are sufficient.
- Use the active plan/spec and local context index as the main continuity anchors instead of replaying large chat history.

## Authoritative Sources and .github Authoring
- Resolve project behavior from repository context first.
- Resolve external platform and SDK behavior from `instructions/governance/ntk-governance-authoritative-sources.instructions.md` and `governance/authoritative-source-map.json`.
- For `.github` authoring, also load `instructions/governance/ntk-governance-copilot-instruction-creation.instructions.md` when relevant.

## Rules Board Summary
- `agents/`: controller lifecycle and orchestration rules
- `governance/`: repository invariants, planning, README, PR, changelog, and instruction-authoring rules
- `development/`: backend, frontend, persistence, testing, and agentic development rules
- `operations/`: DevOps, platform, reliability, and workspace automation rules
- `security/`: application, supply-chain, and secrets/security-hardening rules
- `data/`: database and privacy/data-governance rules