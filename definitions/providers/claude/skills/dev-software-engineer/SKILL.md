---
name: dev-software-engineer
description: Base implementation skill. Use for code changes, bug fixes, refactors, and script work across .NET/C#, backend APIs, database/ORM, Vue/Quasar frontend, Rust, PowerShell, Docker, K8s, CI/CD, security, observability, and SRE domains. Specialized skills extend this as a base.
---

# Dev Software Engineer

## Load context first

1. `.github/AGENTS.md`
2. `.github/copilot-instructions.md`
3. `.github/instructions/governance/ntk-governance-repository-operating-model.instructions.md`
4. Domain instruction pack selected via `.github/instruction-routing.catalog.yml`

## Domain instruction packs

Select based on target area. Load only the pack(s) relevant to the task.

### .NET / C# / Backend
- `.github/instructions/development/ntk-development-backend-dotnet-csharp.instructions.md`
- `.github/instructions/development/ntk-development-backend-architecture-core.instructions.md`
- `.github/instructions/development/ntk-development-persistence-orm.instructions.md`
- `.github/instructions/security/ntk-security-api-high-performance.instructions.md`
- `.github/instructions/operations/ntk-operations-microservices-performance.instructions.md`
- `.github/instructions/governance/ntk-governance-repository-readme-overrides.instructions.md`

### Database / ORM
- `.github/instructions/data/ntk-data-database.instructions.md`
- `.github/instructions/development/ntk-development-persistence-orm.instructions.md`
- `.github/instructions/data/ntk-data-database-configuration-operations.instructions.md`

### Frontend / Vue / Quasar
- `.github/instructions/development/ntk-development-frontend-architecture-core.instructions.md`
- `.github/instructions/development/ntk-development-frontend-vue-quasar.instructions.md`
- `.github/instructions/development/ntk-development-frontend-vue-quasar-architecture.instructions.md`
- `.github/instructions/development/ntk-development-frontend-ui-ux.instructions.md`

### Rust
- `.github/instructions/development/ntk-development-backend-rust-code-organization.instructions.md`
- `.github/instructions/development/ntk-development-backend-rust-testing.instructions.md`

### PowerShell / Scripts
- `.github/instructions/operations/ntk-operations-powershell-execution.instructions.md`
- `.github/instructions/operations/ntk-operations-powershell-script-creation.instructions.md`

### Infrastructure / DevOps / CI-CD
- `.github/instructions/operations/ntk-operations-ci-cd-devops.instructions.md`
- `.github/instructions/operations/ntk-operations-docker.instructions.md`
- `.github/instructions/operations/ntk-operations-k8s.instructions.md`
- `.github/instructions/operations/ntk-operations-workflow-generation.instructions.md`

### Security / Compliance / Privacy
- `.github/instructions/security/ntk-security-vulnerabilities.instructions.md`
- `.github/instructions/data/ntk-data-privacy-compliance.instructions.md`

### Observability / SRE / Reliability
- `.github/instructions/operations/ntk-operations-observability-sre.instructions.md`
- `.github/instructions/operations/ntk-operations-platform-reliability-resilience.instructions.md`

### Testing
- `.github/instructions/governance/ntk-governance-tdd-verification.instructions.md`
- `.github/instructions/development/ntk-development-backend-integration-testing.instructions.md`
- `.github/instructions/development/ntk-development-frontend-e2e-testing.instructions.md`
- `.github/instructions/operations/ntk-operations-static-analysis-sonarqube.instructions.md`

### Documentation / README / Changelog
- `.github/instructions/governance/ntk-governance-readme.instructions.md`
- `.github/instructions/governance/ntk-governance-feedback-changelog.instructions.md`
- `.github/instructions/governance/ntk-governance-effort-estimation-ucp.instructions.md`

### VS Code / Workspace
- `.github/instructions/operations/ntk-operations-vscode-workspace-efficiency.instructions.md`
- `.github/instructions/governance/ntk-governance-workflow-optimization.instructions.md`

### Orchestration / Instructions Authoring
- `.github/agents/super-agent/ntk-agents-super-agent.instructions.md`
- `.github/instructions/governance/ntk-governance-brainstorm-spec-workflow.instructions.md`
- `.github/instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md`
- `.github/instructions/governance/ntk-governance-worktree-isolation.instructions.md`
- `.github/instructions/governance/ntk-governance-artifact-layout.instructions.md`
- `.github/instructions/governance/ntk-governance-authoritative-sources.instructions.md`
- `.github/instructions/governance/ntk-governance-copilot-instruction-creation.instructions.md`
- `.github/instructions/governance/ntk-governance-prompt-templates.instructions.md`

### Pull Requests
- `.github/instructions/governance/ntk-governance-pr.instructions.md`

## Claude-native execution

- Run as a `general-purpose` agent within the Super Agent pipeline.
- Use worktree isolation for risky or large-scope changes (`.github/instructions/governance/ntk-governance-worktree-isolation.instructions.md`).

## Execution workflow

1. Define scope, constraints, and impacted modules.
2. Implement the smallest safe change that satisfies the request.
3. Preserve layer boundaries and dependency direction.
4. Add or update tests for changed behavior.
5. Run targeted validation before claiming completion.

## Prompt accelerators

- `.github/prompts/create-dotnet-class.prompt.md`
- `.github/prompts/create-api-endpoint.prompt.md`
- `.github/prompts/create-ef-migration.prompt.md`
- `.github/prompts/create-vue-component.prompt.md`
- `.github/prompts/create-rust-module.prompt.md`
- `.github/prompts/create-docker-setup.prompt.md`
- `.github/prompts/create-powershell-script.prompt.md`
- `.github/prompts/generate-unit-tests.prompt.md`
- `.github/prompts/refactor-to-clean-architecture.prompt.md`
- `.github/prompts/generate-changelog.prompt.md`
- `.github/prompts/generate-pr-description.prompt.md`