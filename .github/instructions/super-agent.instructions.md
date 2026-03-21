---
applyTo: "**/*.{cs,csproj,sln,ps1,rs,toml,ts,tsx,js,jsx,vue,yml,yaml,json,jsonc,md,sql}"
priority: high
---

# Super Agent Lifecycle

## Purpose
- Normalize all change-bearing work through one Super Agent lifecycle before implementation starts.
- Ensure planning, routing, specialist selection, testing, review, closeout, and planning-state updates happen in a fixed order.
- Prevent silent skipping of skills, instructions, or validation stages.
- Provide a workspace-safe bootstrap controller that can operate in both repository-adapter mode and global-runtime mode without making external skills or unrelated repositories the source of truth.

## Scope
- Apply this lifecycle to any task that changes code, scripts, instructions, docs, runtime assets, workspace settings, pipelines, or repository governance.
- Purely informational answers with no intended file or runtime change may stay lightweight, but must still respect global routing and context rules.
- When the workspace exposes local `.github` adapter files, use the workspace-owned operating model and planning surfaces.
- When the workspace does not expose those files, stay in global-runtime mode and use `.build/super-agent/` for transient planning/spec artifacts.

## Hard Rule
- Do not jump directly from user request to implementation.
- The Super Agent flow owns intake, normalization, planning registration, specialist selection, execution strategy, validation, closeout, and planning-state updates.
- In Codex, prefer the repo-owned `super-agent` skill as the bootstrap controller whenever skill discovery can activate it.
- In Copilot, enforce the same lifecycle through this instruction and the mandatory routing flow because Copilot does not execute local skills directly.
- Do not assume `instruction-routing.catalog.yml`, `planning/`, or `instructions/repository-operating-model.instructions.md` belong to an arbitrary client repo unless that repo actually provides them.

## Required Lifecycle
1. `Super Agent` intake
   - normalize the request
   - identify goals, constraints, risks, and whether the task is trivial or change-bearing
   - break the request into explicit work items when needed
2. planning registration
   - create or update the active planning artifact under `planning/active/` when the workspace provides `planning/README.md`
   - otherwise create or update the transient planning artifact under `.build/super-agent/planning/active/`
   - reuse the existing active plan for the same workstream instead of creating duplicates
   - define expected generated outputs up front and keep non-versioned artifacts under `.build/` or `.deployment/`
3. spec registration when required
   - create or update the active spec under `planning/specs/active/` when the workspace provides `planning/specs/README.md`
   - otherwise create or update the transient spec under `.build/super-agent/specs/active/`
   - skip a separate spec only when the work is trivial and no design direction needs to be locked before planning
4. specialist identification
   - when the workspace provides `.github/instruction-routing.catalog.yml` and `.github/prompts/route-instructions.prompt.md`, route through that local catalog
   - otherwise build the minimal local context pack manually from the target repo structure and select the smallest correct specialist set
5. execution
   - execute sequentially by default
   - divide into multiple subagents only when work items are independent and write-scope conflicts are controlled
6. testing
   - mandatory when code, runtime behavior, scripts, validation logic, or generated artifacts changed
7. code review
   - mandatory final risk-focused review for any repository change
8. closeout
   - always produce a commit message suggestion
   - commit only when the user explicitly allows it or the active workflow policy authorizes it
   - produce changelog-ready summary only when the target workspace actually tracks release history that way
   - update README or stable docs only when the target workspace uses them as source of truth for the changed area
9. planning update
   - update plan status, validation status, blockers, and completion notes
   - move the plan to `planning/completed/` or `.build/super-agent/planning/completed/` only when the workstream is materially finished
   - move the spec to `planning/specs/completed/` or `.build/super-agent/specs/completed/` with the same completion standard

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