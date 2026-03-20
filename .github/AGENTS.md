# Copilot Agents and Context Policy

# Agents
- Workspace: code-first agent for this repo. Reads/edits files, runs searches, and proposes patches.
- GitHub: PR/issue-centric agent. Summarizes, reviews, and changes GitHub artifacts.
- Profiler: performance agent. Benchmarks, profiles, and optimizes hot paths.
- VS: IDE helper. Settings, build/debug help, MSBuild/solution issues.

# Mandatory Context Files
- Always include BOTH of these files first when selecting context for Copilot Chat:
  1. AGENTS.md (this file)
  2. copilot-instructions.md

# Enterprise-First Default
- All tasks must be designed and implemented with real-world enterprise standards by default.
- Target the highest feasible quality level by default across planning, implementation, validation, and documentation.
- This includes architecture consistency, security, testing, observability, maintainability, documentation, and operational safety.
- Exception: only relax this standard when the user explicitly states the task is a `POC`, `spike`, or `informal test`.
- In non-enterprise exceptions, keep minimum safety (no secrets exposure, no destructive commands without explicit approval).

Default workflow for all tasks: Static RAGs Routing (Route → Execute)
- Route first (pick minimal context):
  - `instruction-routing.catalog.yml` (single source of truth for routes)
  - `prompts/route-instructions.prompt.md` (route-only prompt that outputs a JSON context pack)
- Execute next: use ONLY the files returned by the Context Pack.

If context budget is tight, drop other files before these two.

# How to Use in Chat
- Prefer the Workspace agent for code changes in this repo.
- Switch to GitHub for PR/issue workflows.
- Switch to Profiler for performance/benchmark tasks.
- Switch to VS for IDE or build tooling questions.

# Instruction Entry Point (Decision Flow)
## Static RAGs Routing
Baseline workflow for anything in this repo.

1) Always load these first (in order)
Use the **Mandatory Context Files** list above.

2) Then select additional instruction files based on what you are changing
- If editing `.github/**`: include `instructions/pr.instructions.md` and `instructions/prompt-templates.instructions.md` when relevant.
- If editing `instructions/**`: include `instructions/copilot-instruction-creation.instructions.md`.
- If editing code: select the domain instruction file(s) under `instructions/` that match the language/folder (e.g., `instructions/dotnet-csharp.instructions.md`, `backend.instructions.md`, `database.instructions.md`, etc).

3) Precedence rules when instructions conflict
- Follow the user prompt first.
- Then follow `AGENTS.md` + `copilot-instructions.md`.
- Then prefer the most specific instruction file by scope/path (narrower `applyTo` wins over broader).
- If two instructions are equally specific and conflict, pick the safer/minimal option and call out the ambiguity.

# Auditing & Transparency
- When listing applied instructions in a response or PR body, reference both copilot-instructions.md and AGENTS.md when they influence the change.

# Context Preservation & Execution Patterns

## Session Continuity
- Review recent changes and current state at session start
- Preserve established patterns, boundaries, and previous technical choices unless explicitly changing direction

## Execution Flow for Development Tasks
1. Super Agent intake: normalize the request, identify constraints and risk, and decide whether the work is change-bearing
2. Planning Registration: create or update the active plan for any change-bearing task
3. Spec Registration: create or update a versioned spec under `planning/specs/active/` when non-trivial work needs design direction before planning
4. Specialist Routing: identify the smallest correct specialist set and whether safe delegation is possible
5. Implementation: follow established templates and patterns, maintain standards
6. Validation: execute relevant checks, verify compilation, run tests, and confirm architectural compliance
7. Review: perform mandatory final risk-focused review for repository changes
8. Closeout: prepare commit message guidance and changelog/README follow-up as required
9. Planning Update: update plan/spec status and move to completed only when materially finished

## Sub-Agent Planning Chain
- For non-trivial work, use the repository planning pattern under `planning/`.
- Create or update an active plan in `planning/active/` before implementation.
- Create or update an active spec in `planning/specs/active/` before planning when the work is non-trivial and design direction must be locked first.
- Preferred fixed lifecycle for non-trivial change-bearing work:
  1. `Super Agent` intake and request normalization
  2. planning registration
  3. brainstorming/spec registration when required
  4. specialist identification
  5. execution, with multiple subagents only when write-scope conflicts are controlled
  6. tester when code/runtime changed
  7. reviewer
  8. release-closeout
  9. planning update
- Follow `instructions/subagent-planning-workflow.instructions.md` for planning structure, specialist routing, and closeout expectations.
- Follow `instructions/worktree-isolation.instructions.md` when the workstream should move into an isolated git worktree.
- Follow `instructions/tdd-verification.instructions.md` for code-bearing work that needs explicit verification evidence.
- Follow `instructions/super-agent.instructions.md` for the mandatory lifecycle contract.
- Follow `instructions/brainstorm-spec-workflow.instructions.md` when non-trivial work needs design direction before planning.

### For Multi-Task Requests
- Apply Task-Based Execution Methodology (see below)
- Break complex requests into numbered, sequential tasks
- Validate each task completion before proceeding

## Quality Gates
Before generating code: Context loaded, patterns identified, approach validated
During implementation: Templates followed, naming conventions applied, boundaries respected
After changes: Code compiles, tests pass, architecture maintained, documentation updated

## Command Usage Patterns
- Use semantic_search for finding related patterns and implementations
- Use grep_search for locating specific patterns across files
- Use file_search for finding files by naming patterns
- Always use create_file following templates and replace_string_in_file for targeted changes

## Common Pitfalls to Avoid
- Context loss: Forgetting architectural decisions from earlier in session
- Pattern deviation: Creating new patterns instead of following established ones
- Layer violations: Breaking Clean Architecture dependency rules
- Standard drift: Not maintaining consistent standards

# Task-Based Execution Methodology

## Multi-Task Request Structure
- Break complex requests into numbered, sequential tasks
- Each task should have clear scope, dependencies, and success criteria
- Use format: "Tarefas: 1- [task], 2- [task], 3- [task]"
- Validate completion of each task before proceeding to next

## Task Execution Pattern
1. Task Analysis: Review all tasks, identify dependencies and execution order
2. Task Planning: Confirm approach and tools needed for each task
3. Sequential Execution: Complete tasks in order, validate each step
4. Progress Tracking: Report completion status and any blockers
5. Final Validation: Ensure all tasks completed successfully

## Task Documentation
- Document task completion in session context
- Reference specific files/changes made per task
- Note any deviations from original task specification
- Provide rollback information if tasks need to be undone

# Repository Operating Model
- Repo-specific topology, commands, style, release process, and domain instruction map live in:
  - `copilot-instructions.md`
  - `instructions/repository-operating-model.instructions.md`
- Mandatory repo-wide instructions are:
  - `instructions/super-agent.instructions.md`
  - `instructions/brainstorm-spec-workflow.instructions.md`
  - `instructions/repository-operating-model.instructions.md`
  - `instructions/authoritative-sources.instructions.md`
  - `instructions/subagent-planning-workflow.instructions.md`
  - `instructions/worktree-isolation.instructions.md`
  - `instructions/tdd-verification.instructions.md`
  - `instructions/workflow-optimization.instructions.md`
  - `instructions/powershell-execution.instructions.md`
  - `instructions/feedback-changelog.instructions.md`
- Resolve project-specific uncertainty from repository context first; resolve external technology behavior from the official domains defined in `.github/governance/authoritative-source-map.json`.
- For `.github` authoring, include `instructions/copilot-instruction-creation.instructions.md`.