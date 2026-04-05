# Planning Workspace

> Versioned planning artifacts for repository workstreams.

---

## Introduction

`planning/` stores the active and completed plans that guide non-trivial repository work. It keeps execution planning versioned, discoverable, and separate from temporary runtime state.

This folder is part of the repository operating model, not disposable scratch state. Active plans define the current execution surface, and completed plans preserve the operational history that future workstreams depend on.

---

## Features

- ✅ Active plans live in `planning/active/`
- ✅ Completed plans preserve workstream history in `planning/completed/`
- ✅ Active specs live in `planning/specs/active/`
- ✅ Completed specs preserve design history in `planning/specs/completed/`
- ✅ Planning artifacts stay separate from temporary build and runtime outputs
- ✅ Active and completed folders are created on demand instead of being kept alive with placeholders
- ✅ Stable plan slugs and plan reuse prevent duplicate workstreams for the same scope

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Active Workstreams](#active-workstreams)
- [Structure](#structure)
- [Rules](#rules)
- [References](#references)
- [License](#license)

---

## Active Workstreams

- [planning/active/plan-spec-driven-development-operating-model.md](planning/active/plan-spec-driven-development-operating-model.md)
- [planning/active/plan-agent-runtime-comparison-and-improvement-matrix.md](planning/active/plan-agent-runtime-comparison-and-improvement-matrix.md)
- [planning/active/plan-provider-surface-projection-cutover.md](planning/active/plan-provider-surface-projection-cutover.md)
- [planning/active/plan-agentic-surface-boundary-separation.md](planning/active/plan-agentic-surface-boundary-separation.md)
- [planning/active/plan-cli-fast-path-and-startup-decomposition.md](planning/active/plan-cli-fast-path-and-startup-decomposition.md)
- [planning/active/plan-mcp-transport-auth-and-session-resilience.md](planning/active/plan-mcp-transport-auth-and-session-resilience.md)
- [planning/active/plan-memory-file-layering-and-operational-recall.md](planning/active/plan-memory-file-layering-and-operational-recall.md)
- [planning/active/plan-plugin-skill-extensibility-and-runtime-governance.md](planning/active/plan-plugin-skill-extensibility-and-runtime-governance.md)
- [planning/active/plan-context-selection-reranking-and-budgeted-recall.md](planning/active/plan-context-selection-reranking-and-budgeted-recall.md)
- [planning/active/plan-task-output-spill-to-disk-and-retention-control.md](planning/active/plan-task-output-spill-to-disk-and-retention-control.md)
- [planning/active/plan-sdk-control-plane-schemas-and-runtime-introspection.md](planning/active/plan-sdk-control-plane-schemas-and-runtime-introspection.md)
- [planning/active/plan-multi-agent-runtime-lineage-and-a2a-readiness.md](planning/active/plan-multi-agent-runtime-lineage-and-a2a-readiness.md)
- [planning/active/plan-runtime-operational-diagnostics-and-observability.md](planning/active/plan-runtime-operational-diagnostics-and-observability.md)
- [planning/active/plan-free-llm-provider-test-matrix.md](planning/active/plan-free-llm-provider-test-matrix.md)
- [planning/active/plan-development-agent-orchestrator-experience.md](planning/active/plan-development-agent-orchestrator-experience.md)
- [planning/active/plan-repository-consolidation-continuity.md](planning/active/plan-repository-consolidation-continuity.md)
- [planning/active/plan-token-economy-optimization.md](planning/active/plan-token-economy-optimization.md)
- [planning/active/plan-rag-cag-sqlite-evolution.md](planning/active/plan-rag-cag-sqlite-evolution.md)
- [planning/active/plan-build-target-cleanup-and-artifact-pruning.md](planning/active/plan-build-target-cleanup-and-artifact-pruning.md)
- [planning/active/plan-script-retirement-tail-cutover.md](planning/active/plan-script-retirement-tail-cutover.md)

---

## Structure

- `planning/active/` holds the current execution workstreams.
- `planning/completed/` preserves closed workstreams and operational history.
- `planning/specs/active/` holds active design intent that must exist before non-trivial implementation planning.
- `planning/specs/completed/` preserves design history after workstreams close.

Use `planning/` for versioned workstream control, not for transient scratch output.

---

## Rules

- Create or update active plans in `planning/active/`.
- Move a plan to `planning/completed/` only after implementation, validation, review, and closeout are materially complete.
- Create or update versioned specs in `planning/specs/active/` when non-trivial work needs design decisions locked before planning.
- Move specs to `planning/specs/completed/` together with their workstream when the associated plan is materially complete.
- Reuse an existing active plan for the same workstream instead of creating duplicates.
- Use stable slugged names such as `plan-<scope>.md` when creating new plans.
- Keep transient run outputs, smoke artifacts, and execution scratch data in `.temp/`, not under `planning/`.

---

## References

- [planning/completed/plan-instruction-parity-and-script-retirement.md](planning/completed/plan-instruction-parity-and-script-retirement.md)
- [planning/completed/plan-instruction-governance-and-super-agent-retention.md](planning/completed/plan-instruction-governance-and-super-agent-retention.md)
- [planning/completed/plan-instruction-rules-board-and-surface-layout.md](planning/completed/plan-instruction-rules-board-and-surface-layout.md)
- [planning/completed/plan-instruction-taxonomy-and-path-refactor.md](planning/completed/plan-instruction-taxonomy-and-path-refactor.md)
- [planning/completed/plan-script-retirement-phase-2.md](planning/completed/plan-script-retirement-phase-2.md)
- [planning/completed/plan-script-retirement-phase-3.md](planning/completed/plan-script-retirement-phase-3.md)
- [planning/completed/plan-script-retirement-phase-4.md](planning/completed/plan-script-retirement-phase-4.md)
- [planning/completed/plan-script-retirement-phase-5.md](planning/completed/plan-script-retirement-phase-5.md)
- [planning/completed/plan-script-retirement-phase-6.md](planning/completed/plan-script-retirement-phase-6.md)
- [planning/completed/plan-script-retirement-phase-7.md](planning/completed/plan-script-retirement-phase-7.md)
- [planning/completed/plan-script-retirement-phase-8.md](planning/completed/plan-script-retirement-phase-8.md)
- [planning/completed/plan-script-retirement-phase-9.md](planning/completed/plan-script-retirement-phase-9.md)
- [planning/completed/plan-script-retirement-phase-10.md](planning/completed/plan-script-retirement-phase-10.md)
- [planning/completed/plan-script-retirement-phase-11.md](planning/completed/plan-script-retirement-phase-11.md)
- [planning/completed/plan-script-retirement-phase-12.md](planning/completed/plan-script-retirement-phase-12.md)
- [planning/completed/plan-script-retirement-phase-13.md](planning/completed/plan-script-retirement-phase-13.md)
- [planning/completed/plan-script-retirement-phase-14.md](planning/completed/plan-script-retirement-phase-14.md)
- [planning/completed/plan-script-retirement-phase-15.md](planning/completed/plan-script-retirement-phase-15.md)
- [planning/completed/plan-script-retirement-phase-16.md](planning/completed/plan-script-retirement-phase-16.md)
- [planning/completed/plan-script-retirement-phase-17.md](planning/completed/plan-script-retirement-phase-17.md)
- [planning/completed/plan-script-retirement-phase-18.md](planning/completed/plan-script-retirement-phase-18.md)
- [planning/completed/plan-script-retirement-phase-19.md](planning/completed/plan-script-retirement-phase-19.md)
- [planning/completed/plan-script-retirement-phase-20c-self-heal.md](planning/completed/plan-script-retirement-phase-20c-self-heal.md)
- [planning/completed/plan-script-retirement-phase-20d-provider-surface-dispatcher.md](planning/completed/plan-script-retirement-phase-20d-provider-surface-dispatcher.md)
- [planning/completed/plan-script-retirement-phase-20e-catalog-native-renderer-dispatch.md](planning/completed/plan-script-retirement-phase-20e-catalog-native-renderer-dispatch.md)
- [planning/completed/plan-script-retirement-phase-20f-codex-orchestration-renderer.md](planning/completed/plan-script-retirement-phase-20f-codex-orchestration-renderer.md)
- [planning/completed/script-retirement-safety-matrix.md](planning/completed/script-retirement-safety-matrix.md)
- [planning/completed/plan-readme-standards-repository-normalization.md](planning/completed/plan-readme-standards-repository-normalization.md)
- [planning/completed/plan-repository-operations-hygiene.md](planning/completed/plan-repository-operations-hygiene.md)
- [planning/completed/plan-repository-unification-and-rust-migration.md](planning/completed/plan-repository-unification-and-rust-migration.md)
- [planning/completed/plan-rust-migration-closeout-and-cutover.md](planning/completed/plan-rust-migration-closeout-and-cutover.md)
- [planning/completed/plan-ai-usage-history-and-sqlite-local-memory.md](planning/completed/plan-ai-usage-history-and-sqlite-local-memory.md)
- [planning/completed/rust-script-cutover-default-map.md](planning/completed/rust-script-cutover-default-map.md)
- [planning/completed/rust-script-parity-ledger.md](planning/completed/rust-script-parity-ledger.md)
- [planning/completed/rust-script-transcription-ownership-matrix.md](planning/completed/rust-script-transcription-ownership-matrix.md)
- [planning/specs/README.md](specs/README.md)
- [planning/specs/completed/spec-instruction-parity-and-script-retirement-readiness.md](planning/specs/completed/spec-instruction-parity-and-script-retirement-readiness.md)
- [planning/specs/completed/spec-script-retirement-phase-2.md](planning/specs/completed/spec-script-retirement-phase-2.md)
- [planning/specs/completed/spec-script-retirement-phase-3.md](planning/specs/completed/spec-script-retirement-phase-3.md)
- [planning/specs/completed/spec-script-retirement-phase-4.md](planning/specs/completed/spec-script-retirement-phase-4.md)
- [planning/specs/completed/spec-script-retirement-phase-5.md](planning/specs/completed/spec-script-retirement-phase-5.md)
- [planning/specs/completed/spec-script-retirement-phase-6.md](planning/specs/completed/spec-script-retirement-phase-6.md)
- [planning/specs/completed/spec-script-retirement-phase-7.md](planning/specs/completed/spec-script-retirement-phase-7.md)
- [planning/specs/completed/spec-script-retirement-phase-8.md](planning/specs/completed/spec-script-retirement-phase-8.md)
- [planning/specs/completed/spec-script-retirement-phase-9.md](planning/specs/completed/spec-script-retirement-phase-9.md)
- [planning/specs/completed/spec-script-retirement-phase-10.md](planning/specs/completed/spec-script-retirement-phase-10.md)
- [planning/specs/completed/spec-script-retirement-phase-11.md](planning/specs/completed/spec-script-retirement-phase-11.md)
- [planning/specs/completed/spec-script-retirement-phase-12.md](planning/specs/completed/spec-script-retirement-phase-12.md)
- [planning/specs/completed/spec-script-retirement-phase-15.md](planning/specs/completed/spec-script-retirement-phase-15.md)
- [planning/specs/completed/spec-script-retirement-phase-14.md](planning/specs/completed/spec-script-retirement-phase-14.md)
- [planning/specs/completed/spec-script-retirement-phase-20c-self-heal.md](planning/specs/completed/spec-script-retirement-phase-20c-self-heal.md)
- [planning/specs/completed/spec-script-retirement-phase-20d-provider-surface-dispatcher.md](planning/specs/completed/spec-script-retirement-phase-20d-provider-surface-dispatcher.md)
- [planning/specs/completed/spec-script-retirement-phase-20e-catalog-native-renderer-dispatch.md](planning/specs/completed/spec-script-retirement-phase-20e-catalog-native-renderer-dispatch.md)
- [planning/specs/completed/spec-script-retirement-phase-20f-codex-orchestration-renderer.md](planning/specs/completed/spec-script-retirement-phase-20f-codex-orchestration-renderer.md)
- [planning/specs/completed/spec-enterprise-rust-runtime-transcription-architecture.md](planning/specs/completed/spec-enterprise-rust-runtime-transcription-architecture.md)
- [planning/specs/completed/spec-readme-standards-repository-normalization.md](planning/specs/completed/spec-readme-standards-repository-normalization.md)
- [planning/specs/completed/spec-repository-unification-and-rust-migration.md](planning/specs/completed/spec-repository-unification-and-rust-migration.md)
- [ntk-agents-super-agent.instructions.md](../definitions/agents/super-agent/ntk-agents-super-agent.instructions.md)
- [ntk-process-brainstorm-spec-workflow.instructions.md](../definitions/instructions/governance/ntk-governance-brainstorm-spec-workflow.instructions.md)
- [ntk-process-subagent-planning-workflow.instructions.md](../definitions/instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md)
- [ntk-core-repository-operating-model.instructions.md](../definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---