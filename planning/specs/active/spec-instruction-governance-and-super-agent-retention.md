# Instruction Governance And Super Agent Retention Spec

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-04-03 23:48
- Objective: define the design intent for keeping repository instructions authoritative while preserving the `super-agent` lifecycle and avoiding drift from the external `copilot-instructions` baseline.
- Normalized Request: plan how to preserve and sync the repository instruction system without losing the shared guidance that already exists in `C:\Users\tguis\copilot-instructions`.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-instruction-governance-and-super-agent-retention.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Related Workstream: `planning/active/plan-instruction-rules-board-and-surface-layout.md`

---

## Problem Statement

The repository already has a rich instruction and routing system, but it must stay synchronized with the external reference while preserving local ownership of the `super-agent` lifecycle and repo-operating-model rules.

---

## Design Intent

- Preserve repository-owned instruction files as the source of truth for this workspace.
- Keep the external `copilot-instructions` repository as a reference baseline, not as a live write target.
- Make routing and precedence rules explicit so the `ntk` prefix and instruction surfaces stay stable.
- Keep semantic folder taxonomy and stable `ntk-*` filenames as part of the governance contract.
- Reduce repeated policy across semantic domains so backend, frontend, agentic, runtime-ops, data, and security instructions each keep a clear responsibility boundary.

---

## Options Considered

1. Mirror the external instruction repo wholesale.
   - Rejected: too easy to overwrite repo-specific behavior.
2. Keep everything local with no baseline sync.
   - Rejected: drift becomes invisible and hard to correct.
3. Maintain explicit parity checks and deliberate local ownership.
   - Preferred: preserves the current system and prevents accidental loss.

---

## Proposed Boundaries

- `AGENTS.md` and `copilot-instructions.md` remain the mandatory context entry points.
- `ntk-core-super-agent.instructions.md` remains the workflow controller contract.
- `ntk-core-repository-operating-model.instructions.md` remains the repo-local source of truth for workspace behavior.
- Semantic domain folders remain stable, but each file inside them must keep a narrow responsibility and avoid restating adjacent instruction files without need.

---

## Acceptance Criteria

- The repo still exposes the `super-agent` lifecycle.
- The instruction routing catalog stays aligned with the actual workspace.
- Repo-owned instruction changes can be distinguished from baseline reference drift.
- `ntk` surfaces remain documented and canonical.
- Canonical source, projected runtime surface, and provider consumers are explicitly documented.
- Repeated backend guidance is reduced by separating architecture core, platform/runtime behavior, and stack-specific implementation rules.
- Repeated frontend guidance is reduced by separating frontend architecture, Vue/Quasar structure, Vue/Quasar implementation, and UI/UX system guidance.
- Repeated agentic guidance is reduced by separating architectural surface ownership from the checkpoint/compression protocol.
- Repeated runtime PowerShell guidance is reduced by separating execution safety from script-authoring/template rules.
- Repeated runtime workflow guidance is reduced by separating general CI/CD and DevOps platform guidance from GitHub Actions-specific workflow authoring.
- Repeated runtime reliability guidance is reduced by separating observability and incident operations from resilience patterns and disaster readiness.
- Repeated runtime microservice guidance is reduced by separating service-boundary and application-level performance policy from Docker, Kubernetes, observability, and resilience guidance.
- Repeated runtime container guidance is reduced by separating Docker image/container policy from Kubernetes cluster-manifest and rollout policy.
- Repeated runtime static-analysis guidance is reduced by separating SonarQube/static-analysis configuration from CI/workflow execution policy.
- Repeated data/database guidance is reduced by separating schema/query design from connection/failover/backup operations and ORM mapping policy.
- Repeated CI/CD security guidance is reduced by separating trusted workflow, supply-chain, runner, and provenance policy from general CI/CD architecture and GitHub Actions authoring rules.
- Repeated testing guidance is reduced by separating cross-cutting TDD/verification workflow from Rust crate testing, backend integration/API testing, and frontend/browser E2E testing.

---

## Planning Readiness

- The spec is implementation-ready for governance/documentation slices because precedence, projection, and drift rules are now being applied incrementally.
- Follow-up parity audit against the external baseline can remain staged because instruction projections affect both docs and operator workflows.
- The next consolidation slices should follow the same pattern used for backend:
  - preserve semantic taxonomy
  - narrow each instruction file to one responsibility
  - update the canonical shared source first
  - keep `.github/instructions/` synchronized with the canonical copy