# Instruction Taxonomy

`.github/instructions/` is the projected runtime surface used by local agents and provider integrations.

## Rules Board

The instruction tree acts as a semantic rules board for the runtime.

- `core/`: mandatory repository-wide control, authority, artifact, and super-agent rules loaded first.
- `process/`: planning, verification, PR, worktree, and workflow execution rules.
- `architecture/backend/`: backend platform, language, architecture, Rust crate testing, and backend integration/API testing rules.
- `architecture/frontend/`: frontend stack, UX, component architecture, and browser/E2E testing rules.
- `architecture/agentic/`: agentic-surface rules plus context economy and checkpoint protocol.
- `operations/devops/`: CI/CD platform policy, delivery controls, and workflow governance.
- `operations/automation/`: PowerShell execution, script authoring, and workspace automation.
- `operations/containers/`: Docker image/runtime rules and Kubernetes workload orchestration.
- `operations/reliability/`: observability, SRE, resilience, and service/runtime performance.
- `operations/quality/`: static analysis, SonarQube, and quality-gate policy.
- `data/`: schema design, query policy, database operations, and ORM/database rules.
- `security/`: API security, privacy/compliance, vulnerability, hardening, and CI/CD supply-chain rules.
- `docs/`: README, instruction-authoring, and prompt-template rules.

## Precedence

When multiple surfaces apply, use this order:

1. user prompt
2. `.github/AGENTS.md`
3. `.github/copilot-instructions.md`
4. `core/`
5. the narrowest matching domain folder under `process/`, `architecture/`, `operations/`, `data/`, `security/`, or `docs/`
6. prompts, templates, snippets, and projected surfaces as non-authoritative helpers

## Structure

This folder mirrors the semantic taxonomy from `definitions/shared/instructions/`.

- `core/`: repository-wide control and authority rules.
- `process/`: planning, verification, worktree, and delivery workflow rules.
- `architecture/backend/`: backend architecture, stack-specific rules, Rust crate testing, and backend integration/API testing.
- `architecture/frontend/`: frontend architecture, stack-specific rules, and browser/E2E testing.
- `architecture/agentic/`: context economy and agentic-surface rules.
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
- `data/`: database schema, query design, database operations, and ORM rules.
- `security/`: API security, privacy/compliance, dependency hardening, and vulnerability rules.
- Within `data/`, keep schema/query design in `ntk-data-database.instructions.md`, keep connection/failover/backup and DB operations in `ntk-data-database-configuration-operations.instructions.md`, and keep ORM/repository mapping rules in `ntk-data-orm.instructions.md`.
- `docs/`: README and instruction-authoring rules.

The taxonomy intentionally avoids numeric directory prefixes. Agents should select instructions by domain and route metadata, not by folder sort order.

## Naming

All instruction files keep `ntk-*` prefixes so references remain stable and self-describing across prompts, catalogs, skills, and generated surfaces.