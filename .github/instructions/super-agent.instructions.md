---
applyTo: "**/*.{cs,csproj,sln,ps1,rs,toml,ts,tsx,js,jsx,vue,yml,yaml,json,jsonc,md,sql}"
priority: high
---

# Super Agent Lifecycle

## Purpose
- Normalize all change-bearing work through one repository-owned lifecycle before implementation starts.
- Ensure planning, routing, specialist selection, testing, review, closeout, and planning-state updates happen in a fixed order.
- Prevent silent skipping of skills, instructions, or validation stages.
- Provide the repository-owned equivalent of a bootstrap controller without making external skills or repositories the source of truth.

## Scope
- Apply this lifecycle to any task that changes code, scripts, instructions, docs, runtime assets, workspace settings, pipelines, or repository governance.
- Purely informational answers with no intended file or runtime change may stay lightweight, but must still respect global routing and context rules.

## Hard Rule
- Do not jump directly from user request to implementation.
- The Super Agent flow owns intake, normalization, planning registration, specialist selection, execution strategy, validation, closeout, and planning-state updates.
- In Codex, prefer the repo-owned `super-agent` skill as the bootstrap controller whenever skill discovery can activate it.
- In Copilot, enforce the same lifecycle through this instruction and the mandatory routing flow because Copilot does not execute local skills directly.

## Required Lifecycle
1. `Super Agent` intake
   - normalize the request
   - identify goals, constraints, risks, and whether the task is trivial or change-bearing
   - break the request into explicit work items when needed
2. planning registration
   - create or update the active planning artifact under `planning/active/` for any change-bearing task
   - reuse the existing active plan for the same workstream instead of creating duplicates
   - define expected generated outputs up front and keep non-versioned artifacts under `.build/` or `.deployment/`
3. specialist identification
   - route through the repository routing catalog
   - select the smallest correct specialist set
4. execution
   - execute sequentially by default
   - divide into multiple subagents only when work items are independent and write-scope conflicts are controlled
5. testing
   - mandatory when code, runtime behavior, scripts, validation logic, or generated artifacts changed
6. code review
   - mandatory final risk-focused review for any repository change
7. closeout
   - always produce a commit message suggestion
   - commit only when the user explicitly allows it or the active workflow policy authorizes it
   - produce changelog-ready summary when release history should record the change
8. planning update
   - update plan status, validation status, blockers, and completion notes
   - move the plan to `planning/completed/` only when the workstream is materially finished

## Delegation Rules
- Use the planner first for non-trivial work.
- Use the context optimizer before specialist execution when the task spans multiple domains or risks context bloat.
- Use the tester whenever repository state changed beyond pure discussion.
- Use the reviewer before claiming completion.
- Use release closeout for commit-message and changelog output.

## Parallelization Rules
- Parallel subagents are allowed only when:
  - tasks are independent
  - file ownership/write-set boundaries are explicit
  - no shared-state race is expected
- If those conditions are not met, keep execution sequential.

## Commit Rules
- The lifecycle must always prepare a commit message when the state is stable.
- The lifecycle must never auto-commit unless the user explicitly opted in to commit execution.

## Preservation Rules
- This lifecycle augments the existing repository governance model; it does not replace it.
- Never drop existing mandatory instructions, baselines, or validations to satisfy the lifecycle.
- User instructions still take precedence over automation details.