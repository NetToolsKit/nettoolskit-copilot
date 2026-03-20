---
applyTo: "**/*.{cs,csproj,sln,ps1,rs,toml,ts,tsx,js,jsx,vue,yml,yaml,json,jsonc,md,sql}"
priority: high
---

# Sub-Agent Planning Workflow

## Purpose
- Standardize how the repository-owned Super Agent lifecycle plans, routes, executes, reviews, and closes out non-trivial work with a deterministic sub-agent chain.
- Keep planning artifacts versioned under `planning/` as first-class operational assets without mixing them into stable product documentation.

## Planning Workspace Contract
- Use `planning/README.md` as the planning workspace guide.
- Active plans live in `planning/active/`.
- Finished plans move to `planning/completed/` only when the work is genuinely complete.
- Use `planning/specs/README.md` as the versioned specification guide for brainstorming/spec artifacts.
- Active specs live in `planning/specs/active/`.
- Finished specs move to `planning/specs/completed/` with the related workstream when applicable.
- Plan files should use stable slugged names such as `plan-<scope>.md`.
- Reuse and update an existing active plan when the request continues the same workstream instead of creating a duplicate file.

## When Planning Is Mandatory
Create or update an active planning document when any of these are true:
- the task is non-trivial or spans multiple files
- the task crosses more than one technical domain
- the task needs staged validation or rollout control
- the task requires sub-agent delegation or specialist routing
- the user explicitly asks for planning, roadmap, phases, or execution slices

## Mandatory Sub-Agent Chain
For non-trivial work, prefer this execution chain unless the user explicitly asks for a lighter flow:
1. `super-agent` contract
   - normalize the request and decide whether the task is change-bearing
   - ensure planning registration happens before execution
2. `brainstorm-spec-architect` agent
   - create or update a versioned spec in `planning/specs/active/` when non-trivial work needs design direction locked before planning
   - capture decisions, alternatives, risks, acceptance criteria, and planning readiness
3. `planner` agent
   - create or update the active plan in `planning/active/`
   - consume the current active spec when one exists
   - define scope, ordered tasks, validations, risks, and closeout requirements
4. `context-token-optimizer` agent
   - reduce token load by selecting the minimal context pack
   - recommend the correct specialist path
5. worktree isolation decision
   - for risky, long-running, or broad-scope work, prefer `instructions/worktree-isolation.instructions.md`
   - use the repository-owned helper `scripts/runtime/new-super-agent-worktree.ps1` when isolation is warranted
6. `specialist` agent
   - perform the domain implementation using the routed context only
   - keep execution aligned with `instructions/tdd-verification.instructions.md`
7. `tester` agent
   - mandatory when code, runtime behavior, or validation scripts changed
8. `reviewer` agent
   - mandatory final risk-focused code review
9. `release-closeout` agent
   - update relevant README files when needed
   - produce suggested commit message
   - update CHANGELOG with entry-ready content when the change belongs in version history
10. planning update
   - update execution state, validation status, blockers, and closeout notes
   - move the plan to `planning/completed/` and the spec to `planning/specs/completed/` only when materially complete

## Specialist Selection Rule
- Use the smallest specialist capable of executing the task correctly.
- Prefer repository specialists such as `.NET`, `frontend`, `Rust`, `platform`, `security`, `observability`, or `privacy` over the generic engineer when the domain is clear.
- Use the generic engineer only when the task is mixed or no narrower specialist fits.

## Output Contract
Every active plan should define:
1. objective and scope summary
2. normalized request or intake summary
3. spec path or explicit statement that a separate spec was not required
4. ordered tasks with dependencies when needed
5. per-task target paths, explicit commands, expected checkpoints, and commit checkpoint suggestion
6. validation checklist with explicit verification evidence expectations
7. risks and fallback path
8. selected specialist or specialist candidates
9. closeout expectations for README, commit message, and changelog
10. whether isolated worktree execution is recommended

## Closeout Rules
- If the change affects stable documentation, update the relevant README in the same workstream.
- Always return a suggested commit message when the work reaches a stable checkpoint.
- If the change should be retained in release history, update the changelog with entry-ready content following `instructions/feedback-changelog.instructions.md`.
- Do not move a plan to `planning/completed/` until implementation, validation, review, and closeout are all materially complete.