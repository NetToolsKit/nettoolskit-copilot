---
applyTo: "**/*.{cs,csproj,sln,ps1,rs,toml,ts,tsx,js,jsx,vue,yml,yaml,json,jsonc,md,sql}"
priority: high
---

# Sub-Agent Planning Workflow

## Purpose
- Standardize how non-trivial work is planned, routed, executed, reviewed, and closed out with a deterministic sub-agent chain.
- Keep planning artifacts versioned under `planning/` as first-class operational assets without mixing them into stable product documentation.

## Planning Workspace Contract
- Use `planning/README.md` as the planning workspace guide.
- Active plans live in `planning/active/`.
- Finished plans move to `planning/completed/` only when the work is genuinely complete.
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
1. `planner` agent
   - create or update the active plan in `planning/active/`
   - define scope, ordered tasks, validations, risks, and closeout requirements
2. `context-token-optimizer` agent
   - reduce token load by selecting the minimal context pack
   - recommend the correct specialist path
3. `specialist` agent
   - perform the domain implementation using the routed context only
4. `tester` agent
   - mandatory when code, runtime behavior, or validation scripts changed
5. `reviewer` agent
   - mandatory final risk-focused code review
6. `release-closeout` agent
   - adjust README files when needed
   - produce suggested commit message
   - produce changelog summary or entry-ready content when the change belongs in version history

## Specialist Selection Rule
- Use the smallest specialist capable of executing the task correctly.
- Prefer repository specialists such as `.NET`, `frontend`, `Rust`, `platform`, `security`, `observability`, or `privacy` over the generic engineer when the domain is clear.
- Use the generic engineer only when the task is mixed or no narrower specialist fits.

## Output Contract
Every active plan should define:
1. objective and scope summary
2. ordered tasks with dependencies when needed
3. validation checklist
4. risks and fallback path
5. selected specialist or specialist candidates
6. closeout expectations for README, commit message, and changelog

## Closeout Rules
- If the change affects stable documentation, update the relevant README in the same workstream.
- Always return a suggested commit message when the work reaches a stable checkpoint.
- If the change should be retained in release history, produce changelog-ready summary content following `instructions/feedback-changelog.instructions.md`.
- Do not move a plan to `planning/completed/` until implementation, validation, review, and closeout are all materially complete.