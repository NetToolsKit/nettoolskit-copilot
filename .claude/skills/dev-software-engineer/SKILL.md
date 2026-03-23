---
name: dev-software-engineer
description: Base implementation skill. Use for code changes, bug fixes, refactors, and script work across .NET/C#, backend APIs, Vue/Quasar frontend, Rust, and PowerShell. Specialized skills extend this as a base.
---

# Dev Software Engineer

## Load context first

1. `.github/AGENTS.md`
2. `.github/copilot-instructions.md`
3. `.github/instructions/repository-operating-model.instructions.md`
4. Domain instruction pack from `.github/instruction-routing.catalog.yml`

## Domain instruction pack

Select based on target area:

- .NET/backend: `dotnet-csharp.instructions.md`, `clean-architecture-code.instructions.md`, `backend.instructions.md`
- Database: `database.instructions.md`, `orm.instructions.md`
- Frontend: `frontend.instructions.md`, `vue-quasar.instructions.md`, `vue-quasar-architecture.instructions.md`
- Rust: `rust-code-organization.instructions.md`, `rust-testing.instructions.md`
- PowerShell/scripts: `powershell-execution.instructions.md`

## Claude-native execution

- Run as a `general-purpose` agent within the Super Agent pipeline.
- Use worktree isolation for risky or large-scope changes (see `.github/instructions/worktree-isolation.instructions.md`).

## Execution workflow

1. Define scope, constraints, and impacted modules.
2. Implement the smallest safe change that satisfies the request.
3. Preserve layer boundaries and dependency direction.
4. Add or update tests for changed behavior.
5. Run targeted validation before claiming completion.

## Prompt accelerators

- `.github/prompts/create-dotnet-class.prompt.md`
- `.github/prompts/create-api-endpoint.prompt.md`
- `.github/prompts/create-vue-component.prompt.md`
- `.github/prompts/create-rust-module.prompt.md`
- `.github/prompts/create-powershell-script.prompt.md`
- `.github/prompts/refactor-to-clean-architecture.prompt.md`