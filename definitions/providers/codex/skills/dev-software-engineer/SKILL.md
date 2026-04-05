---
name: dev-software-engineer
description: Base implementation skill for this repository across .NET/C#, backend APIs, database/ORM, frontend Vue/Quasar, and Rust with Clean Architecture rules. Use when the user asks to build features, fix bugs, refactor modules, or improve code-level performance, and use specialized skills as inheritance layers when available.
---

# Dev Software Engineer

## Base skill contract

1. Use this skill as the default implementation baseline across stacks.
2. Specialized skills can inherit this workflow by loading this file first and then applying deltas.
3. Current inheritance mapping: `dev-dotnet-backend-engineer` extends this skill for .NET backend-only execution.

## Load minimal context first

1. Load `definitions/providers/github/root/AGENTS.md`, `definitions/providers/github/root/copilot-instructions.md`, and `definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md`.
2. Route with `definitions/providers/github/root/instruction-routing.catalog.yml` and `definitions/providers/github/prompts/route-instructions.prompt.md`.
3. Keep only mandatory files plus the selected domain pack.

## Select domain instruction pack

- .NET and backend:
  - `definitions/instructions/development/ntk-development-backend-dotnet-csharp.instructions.md`
  - `definitions/instructions/development/ntk-development-backend-architecture-core.instructions.md`
  - `definitions/instructions/development/ntk-development-persistence-orm.instructions.md`
- Database and ORM:
  - `definitions/instructions/data/ntk-data-database.instructions.md`
  - `definitions/instructions/development/ntk-development-persistence-orm.instructions.md`
- Frontend Vue/Quasar:
  - `definitions/instructions/development/ntk-development-frontend-architecture-core.instructions.md`
  - `definitions/instructions/development/ntk-development-frontend-vue-quasar.instructions.md`
  - `definitions/instructions/development/ntk-development-frontend-vue-quasar-architecture.instructions.md`
  - `definitions/instructions/development/ntk-development-frontend-ui-ux.instructions.md`
- Rust:
  - `definitions/instructions/development/ntk-development-backend-rust-code-organization.instructions.md`
  - `definitions/instructions/development/ntk-development-backend-rust-testing.instructions.md`
- Performance/microservices (when applicable):
  - `definitions/instructions/operations/ntk-operations-microservices-performance.instructions.md`

## Execution workflow

1. Define scope, constraints, and impacted modules.
2. Implement the smallest safe change that satisfies the request.
3. Preserve layer boundaries and dependency direction.
4. Add or update tests for changed behavior.
5. Run targeted validation commands and dependency vulnerability audit for each impacted stack before build/package.

## Prompt accelerators

- `definitions/providers/github/prompts/create-dotnet-class.prompt.md`
- `definitions/providers/github/prompts/create-api-endpoint.prompt.md`
- `definitions/providers/github/prompts/create-ef-migration.prompt.md`
- `definitions/providers/github/prompts/create-vue-component.prompt.md`
- `definitions/providers/github/prompts/create-rust-module.prompt.md`
- `definitions/providers/github/prompts/refactor-to-clean-architecture.prompt.md`

## Validation examples

```powershell
$SecurityScriptsRoot = Join-Path $env:USERPROFILE '.codex\shared-scripts\security'
pwsh -File (Join-Path $SecurityScriptsRoot 'Invoke-PreBuildSecurityGate.ps1') -RepoRoot $PWD -FailOnSeverities Critical,High
dotnet build
dotnet test --filter "Category=Unit"
pwsh -File (Join-Path $SecurityScriptsRoot 'Invoke-VulnerabilityAudit.ps1') -RepoRoot $PWD -FailOnSeverities Critical,High
npm run build
pwsh -File (Join-Path $SecurityScriptsRoot 'Invoke-FrontendPackageVulnerabilityAudit.ps1') -RepoRoot $PWD -ProjectPath src/WebApp -FailOnSeverities Critical,High
cargo test
pwsh -File (Join-Path $SecurityScriptsRoot 'Invoke-RustPackageVulnerabilityAudit.ps1') -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High
```