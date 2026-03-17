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

If context budget is tight, prefer dropping any other files before these. These two documents coordinate global rules and agent usage and must be loaded to avoid inconsistent answers.

# How to Use in Chat
- Prefer the Workspace agent for code changes in this repo.
- Switch to GitHub for PR/issue workflows.
- Switch to Profiler for performance/benchmark tasks.
- Switch to VS for IDE or build tooling questions.

# Instruction Entry Point (Decision Flow)
Use this section as the quick “what do I load / follow first?” guide.

## Static RAGs Routing
Baseline workflow for anything in this repo.

1) Always load these first (in order)
Follow the **Mandatory Context Files** list above.

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
- Load previous context at session start: review recent changes and current state
- Maintain architectural patterns and decisions from earlier work
- Preserve Clean Architecture boundaries and established abstractions
- Respect previous technical choices unless explicitly changing approach

## Execution Flow for Development Tasks
1. Task Analysis: Load mandatory context files, identify domain, analyze scope
2. Planning: Create execution plan for non-trivial tasks
3. Implementation: Follow established templates and patterns, maintain standards
4. Validation: Execute relevant checks, verify compilation, run tests, and confirm architectural compliance

## Sub-Agent Planning Chain
- For non-trivial work, use the repository planning pattern under `.temp/planning/`.
- Create or update an active plan in `.temp/planning/plans-active/` before implementation.
- Preferred chain for non-trivial work:
  1. planner
  2. context-token-optimizer
  3. specialist
  4. tester when code/runtime changed
  5. reviewer
  6. release-closeout
- Follow `instructions/subagent-planning-workflow.instructions.md` for planning structure, specialist routing, and closeout expectations.

### For Multi-Task Requests
- Apply Task-Based Execution Methodology (see below)
- Break complex requests into numbered, sequential tasks
- Validate each task completion before proceeding
- Maintain session continuity across task boundaries

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
- Standard drift: Not maintaining consistent coding standards

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

## Benefits of Task-Based Approach
- Improved clarity and reduced ambiguity in complex requests
- Better progress tracking and session continuity
- Easier debugging when tasks fail or need modification
- Enhanced collaboration between human and AI agents
- Systematic approach aligned with Clean Architecture principles

# Repository Operating Model
- Repo-specific topology, commands, style, release process, and domain instruction map live in:
  - `copilot-instructions.md`
  - `instructions/repository-operating-model.instructions.md`
- Mandatory repo-wide instructions are:
  - `instructions/repository-operating-model.instructions.md`
  - `instructions/authoritative-sources.instructions.md`
  - `instructions/subagent-planning-workflow.instructions.md`
  - `instructions/workflow-optimization.instructions.md`
  - `instructions/powershell-execution.instructions.md`
  - `instructions/feedback-changelog.instructions.md`
- Resolve project-specific uncertainty from repository context first; resolve external technology behavior from the official domains defined in `.github/governance/authoritative-source-map.json`.
- For `.github` authoring, include `instructions/copilot-instruction-creation.instructions.md`.