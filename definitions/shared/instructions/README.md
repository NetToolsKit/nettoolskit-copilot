# Instruction Taxonomy

`definitions/shared/instructions/` is the canonical source for repository instruction content.

## Rules Board

The canonical tree is organized as a semantic rules board.

- `core/`: mandatory repository-wide control, authority, artifact, and super-agent rules.
- `process/`: planning, verification, PR, worktree, and workflow execution rules.
- `architecture/backend/`: backend platform, language, and architecture rules.
- `architecture/frontend/`: frontend stack, UX, and component architecture rules.
- `architecture/agentic/`: context economy and agentic-surface rules.
- `runtime-ops/`: runtime, CI/CD, workflow generation, automation, observability, resilience, and infrastructure execution rules.
- `data/`: schema design, query policy, database operations, and ORM/database rules.
- `security/`: API security, privacy/compliance, vulnerability, and hardening rules.
- `docs/`: README, instruction-authoring, and prompt-template rules.

## Precedence

The shared source is designed for projection into runtime surfaces with this authority model:

1. direct user request
2. global runtime entry files such as `AGENTS.md` and `copilot-instructions.md`
3. `core/`
4. the narrowest matching domain folder under `process/`, `architecture/`, `runtime-ops/`, `data/`, `security/`, or `docs/`
5. prompts, templates, snippets, and other projected helpers as non-authoritative consumers

## Structure

The instruction tree is grouped by semantic domain, not numeric prefixes. Directory order is not part of the contract; agents should route by path and file purpose instead of lexical ordering.

- `core/`: repository-wide operating model, authoritative sources, artifact layout, and super-agent control.
- `process/`: planning, verification, worktree isolation, PR, and workflow execution rules.
- `architecture/backend/`: backend platform, clean architecture, .NET, and Rust organization guidance.
- `architecture/frontend/`: frontend architecture, Vue/Quasar, and UI/UX guidance.
- `architecture/agentic/`: agentic-surface boundaries plus context economy and checkpoint guidance.
- `runtime-ops/`: CI/CD, workflow generation, Docker, Kubernetes, PowerShell, resilience, observability, and execution rules.

Within `runtime-ops/`, keep general pipeline and DevOps platform guidance in
`ntk-runtime-ci-cd-devops.instructions.md` and keep GitHub Actions-specific
authoring rules in `ntk-runtime-workflow-generation.instructions.md`.
Keep telemetry, SLO, dashboards, alerts, and incident operations in
`ntk-runtime-observability-sre.instructions.md`, and keep resilience patterns,
capacity, chaos testing, and disaster readiness in
`ntk-runtime-platform-reliability-resilience.instructions.md`.
Keep service-boundary, service-contract, caching, and application-level
throughput guidance in `ntk-runtime-microservices-performance.instructions.md`,
and keep container or cluster implementation details in the Docker and
Kubernetes instructions.
Keep image construction, container runtime, and Docker Compose guidance in
`ntk-runtime-docker.instructions.md`, and keep Kubernetes manifests, cluster
rollout, networking, storage, and autoscaling guidance in
`ntk-runtime-k8s.instructions.md`.
Keep SonarQube/static-analysis configuration, quality profiles, exclusions, and
report import policy in
`ntk-runtime-static-analysis-sonarqube.instructions.md`, and keep CI/workflow
execution wiring in the CI/CD and workflow-generation instructions.
- `data/`: database schema, query design, database operations, and ORM guidance.
- Within `data/`, keep schema/query design in
  `ntk-data-database.instructions.md`, keep connection/failover/backup and DB
  operations in `ntk-data-database-configuration-operations.instructions.md`,
  and keep ORM/repository mapping rules in `ntk-data-orm.instructions.md`.
- `security/`: API security, privacy/compliance, and vulnerability guidance.
- `docs/`: README, instruction authoring, and prompt-template guidance.

## Naming

Instruction files use stable `ntk-*` prefixes so the domain stays visible even when a file is referenced outside its folder.

Examples:

- `ntk-core-repository-operating-model.instructions.md`
- `ntk-backend-architecture-platform.instructions.md`
- `ntk-frontend-vue-quasar.instructions.md`
- `ntk-agentic-surfaces.instructions.md`
- `ntk-runtime-powershell-execution.instructions.md`
- `ntk-docs-readme.instructions.md`

## Projection

`.github/instructions/` is a projected runtime surface. Shared definitions stay authoritative; projected copies must preserve the same folder taxonomy and file names.