---
name: dev-dotnet-backend-engineer
description: Extend dev-software-engineer with .NET/C# backend specialization for Clean Architecture, CQRS, EF Core, and production-ready API practices. Use when tasks involve ASP.NET endpoints, handlers, services, dependency injection, MediatR, migrations, or backend performance/resilience.
---

# Dev Dotnet Backend Engineer

## Inherit from base skill

1. Load `.claude/skills/dev-software-engineer/SKILL.md` as the base contract.
2. Restrict domain scope to .NET backend, database, and ORM concerns only.

## Domain instruction delta

- `definitions/instructions/development/ntk-development-backend-dotnet-csharp.instructions.md`
- `definitions/instructions/development/ntk-development-backend-architecture-core.instructions.md`
- `definitions/instructions/development/ntk-development-persistence-orm.instructions.md`
- `definitions/instructions/data/ntk-data-database.instructions.md`
- `definitions/instructions/development/ntk-development-persistence-orm.instructions.md`
- `definitions/instructions/operations/ntk-operations-microservices-performance.instructions.md` (when applicable)

## Claude-native execution

Run as a `general-purpose` agent within the Super Agent pipeline.

## Execution workflow delta

1. Confirm layer boundaries and domain model ownership.
2. Implement minimal backend change preserving contracts.
3. Apply validation, error model, and observability patterns.
4. Add/update unit and integration tests for changed behavior.
5. Run targeted validation before claiming completion.

## Prompt accelerators

- `definitions/providers/github/prompts/create-dotnet-class.prompt.md`
- `definitions/providers/github/prompts/create-api-endpoint.prompt.md`
- `definitions/providers/github/prompts/create-ef-migration.prompt.md`
- `definitions/providers/github/prompts/refactor-to-clean-architecture.prompt.md`

## Validation examples

```powershell
dotnet build
dotnet test --filter "Category=Unit"
dotnet test --filter "Category=Integration"
dotnet pack -c Release
```