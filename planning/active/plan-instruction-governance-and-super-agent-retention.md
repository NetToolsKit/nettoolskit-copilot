# Instruction Governance And Super Agent Retention Plan

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-04-03 23:26
- Objective: keep repository instructions, `super-agent` behavior, and the external `copilot-instructions` reference aligned without losing canonical guidance or routing fidelity.
- Normalized Request: create a planning workstream for instruction organization and retention so the repository keeps the shared instruction system intact while avoiding drift from `C:\Users\tguis\copilot-instructions`.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-instruction-governance-and-super-agent-retention.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Dependency: `planning/active/plan-instruction-rules-board-and-surface-layout.md`
- Inputs:
  - `definitions/shared/instructions/core/ntk-core-repository-operating-model.instructions.md`
  - `.github/instructions/core/ntk-core-repository-operating-model.instructions.md`
  - `.github/instructions/core/ntk-core-super-agent.instructions.md`
  - `.github/instruction-routing.catalog.yml`
  - `C:\Users\tguis\copilot-instructions\`
  - `planning/completed/plan-instruction-parity-and-script-retirement.md`

---

## Scope Summary

This plan coordinates four governance slices:

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| G1 | Instruction parity audit | repo vs external reference | 🔴 Immediate | none |
| G2 | Super-agent retention | local and projected instruction surfaces | ✅ Done | G1 |
| G3 | Routing and precedence clarity | catalog + operating model docs | ✅ Done | G1 |
| G4 | Drift monitoring and sync rules | instructions projections and planning docs | ✅ Done | G2, G3 |
| G5 | Domain consolidation | semantic instruction domains with reduced repetition | 🟡 In progress | G2, G3, G4 |

The `ntk` CLI prefix is already in place and is not a new implementation gap.

---

## Ordered Tasks

### [2026-03-30 07:31] Task G1: Baseline Instruction Parity

- Compare the repository-owned instruction surfaces with the external `copilot-instructions` baseline.
- Record the intentionally diverged areas so they are not “fixed” by accident.
- Keep the `super-agent` operating model explicitly canonical.
- Commit checkpoint:
  - `docs(planning): freeze instruction governance parity baseline`

### [2026-03-30 07:31] Task G2: Preserve The Super Agent Surface

- Keep the `super-agent` workflow visible in the repo-owned instruction set.
- Make sure instruction projections do not overwrite the canonical guidance with a stale copy.
- Confirm the repo still routes work through the same lifecycle contract.
- Status:
  - complete; `super-agent` remains in the `core/` lane and the rules board now keeps it globally visible with semantic taxonomy
- Commit checkpoint:
  - `docs(instructions): reinforce super agent canonical routing`

### [2026-03-30 07:31] Task G3: Clarify Routing And Precedence

- Ensure the routing catalog and operating model remain consistent with the actual workspace.
- Keep `ntk` and instruction-file guidance aligned for operators and agents.
- Document how the repository keeps command surfaces and instruction surfaces in sync.
- Status:
  - complete; precedence and semantic-folder routing are now documented in `AGENTS.md`, instruction READMEs, and governance metadata
- Commit checkpoint:
  - `docs(instructions): clarify routing and precedence for repository guidance`

### [2026-03-30 07:31] Task G4: Define Drift Monitoring And Sync Rules

- Add the ongoing rule for syncing from the external baseline without losing repo-owned changes.
- Document how to detect and review drift before it becomes a branch or PR problem.
- Keep the guidance about `ntk` prefix usage and the `super-agent` lifecycle intact.
- Status:
  - complete; the instruction ownership manifest and repository operating model now define canonical root, projected root, provider roots, and drift rules
- Commit checkpoint:
  - `docs(planning): define instruction drift monitoring and sync policy`

### [2026-04-03 17:10] Task G5: Consolidate Semantic Instruction Domains

- Reduce repeated guidance inside semantic instruction folders without collapsing distinct responsibilities.
- Keep canonical authority in `definitions/shared/instructions/` and project identical copies into `.github/instructions/`.
- Clarify the separation between:
  - architecture invariants
  - platform/runtime behavior
  - language/framework specifics
- Start with `architecture/backend/`, then continue with frontend, agentic, runtime-ops, data, and security slices.
- Status:
  - backend slice complete; `ntk-backend-architecture-core`, `ntk-backend-architecture-platform`, and `ntk-backend-dotnet-csharp` now have narrower scopes and less repeated policy
  - frontend slice complete; `ntk-frontend-architecture-core`, `ntk-frontend-vue-quasar-architecture`, `ntk-frontend-vue-quasar`, and `ntk-frontend-ui-ux` now separate architecture, framework structure, implementation, and design-system guidance
  - agentic slice complete; `ntk-agentic-surfaces` now owns MCP/A2A/RAG/CAG boundaries while `ntk-agentic-context-economy-checkpoint` keeps only the checkpoint/compression protocol
  - runtime-ops PowerShell slice complete; `ntk-runtime-powershell-execution` now owns runtime invocation safety while `ntk-runtime-powershell-script-creation` owns authoring/template rules
  - runtime-ops workflow slice complete; `ntk-runtime-ci-cd-devops` now owns general pipeline and DevOps platform guidance while `ntk-runtime-workflow-generation` owns GitHub Actions authoring requirements
  - runtime-ops reliability slice complete; `ntk-runtime-observability-sre` now owns telemetry, SLO, dashboards, alerts, and incident operations while `ntk-runtime-platform-reliability-resilience` owns resilience patterns, capacity, chaos, and disaster readiness
  - runtime-ops microservice slice complete; `ntk-runtime-microservices-performance` now owns service boundaries, service contracts, caching, and application-level throughput guidance while Docker, Kubernetes, observability, and resilience details stay in their specialized instruction files
  - runtime-ops container slice complete; `ntk-runtime-docker` now owns image construction, container runtime, and Docker Compose policy while `ntk-runtime-k8s` owns cluster manifests, rollout, networking, storage, and autoscaling policy
  - runtime-ops static-analysis slice complete; `ntk-runtime-static-analysis-sonarqube` now owns SonarQube/static-analysis configuration, quality profiles, exclusions, and report import policy while CI/workflow execution stays in the CI/CD and workflow-generation instructions
  - data slice complete; `ntk-data-database` now owns schema and query design while `ntk-data-database-configuration-operations` owns connection/failover/backup operations and `ntk-data-orm` owns ORM/repository mapping conventions
  - taxonomy split complete; the former `data-security/` lane is now represented by separate `data/` and `security/` folders across canonical, projected, and provider-consumer surfaces
  - security supply-chain slice complete; `ntk-security-cicd-supply-chain-hardening` now owns trusted workflow boundaries, immutable action pinning, OIDC, runner isolation, SBOM, and provenance policy while CI/CD and workflow-generation instructions keep their narrower operational scopes
- Commit checkpoint:
  - `docs(instructions): narrow backend instruction responsibilities`

---

## Validation Checklist

- `git diff --check`
- instruction parity audit against `C:\Users\tguis\copilot-instructions`
- repository instruction validation command(s)
- planning structure validation

---

## Risks And Mitigations

- Overwriting repo-owned instruction surfaces with external copies would erase local policy.
- Routing drift can cause agents to load the wrong instructions and make wrong decisions.
- Mitigation: keep a named parity baseline and only sync through explicit review.

---

## Specialist And Closeout

- Recommended specialist: `docs-release-engineer`
- Tester: required if projections or commands change
- Reviewer: required
- Release closeout: required
- README update: required if the operator guidance changes
- Changelog: required if the instruction behavior changes materially
- Suggested commit message style:
  - `docs(instructions): align instruction governance with external baseline`
  - `docs(planning): record instruction governance roadmap`