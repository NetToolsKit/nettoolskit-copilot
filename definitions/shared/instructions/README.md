# Instruction Taxonomy

`definitions/shared/instructions/` is the canonical source for repository instruction content.

## Rules Board

The canonical tree is organized as a semantic rules board.

- `agents/`: mandatory agent-controller lifecycle and orchestration rules.
- `core/`: mandatory repository-wide control, authority, artifact, and operating-model rules.
- `process/planning/`: specification, estimation, planning, and workflow optimization rules.
- `process/collaboration/`: PR collaboration and isolated-worktree coordination rules.
- `process/delivery/`: verification, quality evidence, and changelog/closeout rules.
- `architecture/backend/`: backend platform, language, architecture, Rust crate testing, and backend integration/API testing rules.
- `architecture/frontend/`: frontend stack, UX, component architecture, and browser/E2E testing rules.
- `architecture/agentic/`: context economy and agentic-surface rules.
- `operations/devops/`: CI/CD platform policy, delivery controls, and workflow governance.
- `operations/automation/`: PowerShell execution, script authoring, and workspace automation.
- `operations/containers/`: Docker image/runtime rules and Kubernetes workload orchestration.
- `operations/reliability/`: observability, SRE, resilience, and service/runtime performance.
- `operations/quality/`: static analysis, SonarQube, and quality-gate policy.
- `data/`: schema design, query policy, database operations, and ORM/database rules.
- `security/`: API security, privacy/compliance, vulnerability, hardening, and CI/CD supply-chain rules.
- `docs/`: README, instruction-authoring, and prompt-template rules.

## Precedence

The shared source is designed for projection into runtime surfaces with this authority model:

1. direct user request
2. global runtime entry files such as `AGENTS.md` and `copilot-instructions.md`
3. `agents/`
4. `core/`
5. the narrowest matching domain folder under `process/`, `architecture/`, `operations/`, `data/`, `security/`, or `docs/`
6. prompts, templates, snippets, and other projected helpers as non-authoritative consumers

## Structure

The instruction tree is grouped by semantic domain, not numeric prefixes. Directory order is not part of the contract; agents should route by path and file purpose instead of lexical ordering.

- `agents/`: agent-controller lifecycle and orchestration rules.
- `core/`: repository-wide operating model, authoritative sources, artifact layout, and other repository invariants.
- `process/`: semantic human-workflow lanes for planning, collaboration, and delivery.
- `process/planning/`: brainstorming/spec workflow, effort estimation, active-planning flow, and workflow optimization.
- `process/collaboration/`: PR authoring/review coordination and worktree isolation.
- `process/delivery/`: TDD/verification evidence and changelog/release-history closeout.
- `architecture/backend/`: backend platform, clean architecture, .NET, Rust organization, Rust crate testing, and backend integration/API testing guidance.
- `architecture/frontend/`: frontend architecture, Vue/Quasar, UI/UX, and browser/E2E guidance.
- `architecture/agentic/`: agentic-surface boundaries plus context economy and checkpoint guidance.
- `operations/`: semantic operational lanes for DevOps, automation, containers, reliability, and quality.
- `operations/devops/`: platform-level CI/CD policy and workflow authoring.
- `operations/automation/`: PowerShell runtime safety, script creation, and workspace efficiency.
- `operations/containers/`: Docker and Kubernetes implementation surfaces.
- `operations/reliability/`: observability, resilience, and distributed runtime performance.
- `operations/quality/`: SonarQube/static-analysis configuration and quality gates.
- Within `operations/devops/`, keep general pipeline and DevOps platform guidance in `ntk-runtime-ci-cd-devops.instructions.md` and keep GitHub Actions-specific authoring rules in `ntk-runtime-workflow-generation.instructions.md`.
- Within `operations/automation/`, keep invocation safety in `ntk-runtime-powershell-execution.instructions.md`, keep script-authoring rules in `ntk-runtime-powershell-script-creation.instructions.md`, and keep editor automation/workspace composition in `ntk-runtime-vscode-workspace-efficiency.instructions.md`.
- Within `operations/containers/`, keep image construction, container runtime, and Docker Compose guidance in `ntk-runtime-docker.instructions.md`, and keep Kubernetes manifests, cluster rollout, networking, storage, and autoscaling guidance in `ntk-runtime-k8s.instructions.md`.
- Within `operations/reliability/`, keep telemetry, SLO, dashboards, alerts, and incident operations in `ntk-runtime-observability-sre.instructions.md`, keep resilience patterns, capacity, chaos testing, and disaster readiness in `ntk-runtime-platform-reliability-resilience.instructions.md`, and keep service-boundary, service-contract, caching, and application-level throughput guidance in `ntk-runtime-microservices-performance.instructions.md`.
- Within `operations/quality/`, keep SonarQube/static-analysis configuration, quality profiles, exclusions, and report import policy in `ntk-runtime-static-analysis-sonarqube.instructions.md`.
- `data/`: database schema, query design, database operations, and ORM guidance.
- Within `data/`, keep schema/query design in `ntk-data-database.instructions.md`, keep connection/failover/backup and DB operations in `ntk-data-database-configuration-operations.instructions.md`, and keep ORM/repository mapping rules in `ntk-data-orm.instructions.md`.
- `security/`: API security, privacy/compliance, vulnerability, and CI/CD supply-chain guidance.
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