# Instruction Taxonomy

`definitions/shared/instructions/` is the canonical source for repository instruction content.

## Structure

The instruction tree is grouped by semantic domain, not numeric prefixes. Directory order is not part of the contract; agents should route by path and file purpose instead of lexical ordering.

- `core/`: repository-wide operating model, authoritative sources, artifact layout, and super-agent control.
- `process/`: planning, verification, worktree isolation, PR, and workflow execution rules.
- `architecture/backend/`: backend platform, clean architecture, .NET, and Rust organization guidance.
- `architecture/frontend/`: frontend architecture, Vue/Quasar, and UI/UX guidance.
- `architecture/agentic/`: context economy and agentic-surface guidance.
- `runtime-ops/`: CI/CD, Docker, Kubernetes, PowerShell, resilience, observability, and workflow generation.
- `data-security/`: database, ORM, privacy/compliance, API security, and vulnerability guidance.
- `docs/`: README, instruction authoring, and prompt-template guidance.

## Naming

Instruction files use stable `ntk-*` prefixes so the domain stays visible even when a file is referenced outside its folder.

Examples:

- `ntk-core-repository-operating-model.instructions.md`
- `ntk-backend-architecture-platform.instructions.md`
- `ntk-frontend-vue-quasar.instructions.md`
- `ntk-runtime-powershell-execution.instructions.md`
- `ntk-docs-readme.instructions.md`

## Projection

`.github/instructions/` is a projected runtime surface. Shared definitions stay authoritative; projected copies must preserve the same folder taxonomy and file names.
