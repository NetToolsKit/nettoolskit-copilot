# Claude Code — Workspace Adapter

## Workspace Mode

`workspace-adapter` — local `.github/AGENTS.md`, `.github/copilot-instructions.md`, and `planning/` are present and authoritative.

## Authoritative Instructions (load in order)

1. `.github/AGENTS.md` — agent contracts, workspace modes, quality gates
2. `.github/copilot-instructions.md` — language policy, routing, output economy, EOF policy
3. `.github/instructions/super-agent.instructions.md` — mandatory lifecycle for change-bearing work

For domain-specific work, select instruction files from `.github/instructions/` via `.github/instruction-routing.catalog.yml`.

## Language Policy

- Chat responses: pt-BR
- Code, commits, docs, file names, database: English

## EOF Policy

Preserve exact EOF state of edited files. Repository default (`insert_final_newline = false`): no trailing newline, no trailing blank lines.

## Super Agent Lifecycle

For any change-bearing task, follow the 9-step lifecycle from `.github/instructions/super-agent.instructions.md`:

1. Intake — normalize request, identify risks, classify as trivial or change-bearing
2. Spec — create/update `planning/specs/active/` for non-trivial work (use `Plan` agent)
3. Plan — create/update `planning/active/` (use `Plan` agent)
4. Route — assemble minimal context pack (use `Explore` agent)
5. Execute — implement with smallest correct specialist set (use `general-purpose` agent)
6. Test — mandatory when code/scripts/runtime changed
7. Review — mandatory final risk-focused review
8. Closeout — commit message suggestion, changelog, README updates
9. Planning update — move to `completed/` only when materially finished

## Claude-Native Agent Type Mapping

| Pipeline Role | Claude Agent Type | When |
|---|---|---|
| brainstormer (spec) | `Plan` | Non-trivial design-bearing work |
| planner (active plan) | `Plan` | Any change-bearing workstream |
| router (context pack) | `Explore` | Multi-domain or context-heavy tasks |
| specialist (implementation) | `general-purpose` | Code, scripts, docs changes |
| tester | `general-purpose` | After any state change |
| reviewer | `general-purpose` | Before claiming completion |

Use `EnterPlanMode` before spawning Plan agents for spec and planning stages.

## Claude Skills

Skill adapters live in `.claude/skills/`. Invoke with the Skill tool or reference by name.

Available: `super-agent`, `brainstorm-spec-architect`, `plan-active-work-planner`, `context-token-optimizer`, `dev-software-engineer`, `review-code-engineer`

## Memory Policy

Persist to the project memory directory (`~/.claude/projects/<project-slug>/memory/`, auto-resolved from working directory):

- **project**: active plan/spec paths, current workstream state, key decisions
- **feedback**: user corrections and confirmed approaches specific to this repo
- **user**: user role, preferences, and collaboration style

Do not save: code patterns (read from source), git history (use `git log`), or ephemeral session state.

## Transparency

Surface Super Agent activation banner once at the start of the first substantive reply in change-bearing sessions. Include `Agents used:` line in every terminal completion.

## Validation Commands

```powershell
pwsh -NoLogo -NoProfile -File scripts/tests/runtime/runtime-scripts.tests.ps1 -RepoRoot .
pwsh -NoLogo -NoProfile -File scripts/validation/validate-instructions.ps1 -RepoRoot .
pwsh -NoLogo -NoProfile -File scripts/validation/validate-all.ps1 -RepoRoot . -Profile dev
```