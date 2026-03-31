# Planning Specs Workspace

> Versioned design intent for repository workstreams.

---

## Introduction

`planning/specs/` stores the active and completed specifications that define design intent before implementation planning begins.

This workspace separates design intent from execution planning so non-trivial work does not jump from intake directly into implementation without an explicit architectural checkpoint.

---

## Features

- ✅ Active specs capture design intent before execution planning
- ✅ Completed specs preserve architectural and workflow decisions
- ✅ Spec files stay versioned and reusable across related workstreams
- ✅ Spec artifacts remain separate from task execution plans
- ✅ Spec reuse prevents duplicate design work for the same workstream
- ✅ Focused specs replace open-ended roadmap documents in the active workspace

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Active Specs](#active-specs)
- [Structure](#structure)
- [Rules](#rules)
- [References](#references)
- [License](#license)

---

`planning/specs/` stores the active and completed specifications that define design intent before implementation planning begins.

---

## Active Specs

- [planning/specs/active/spec-spec-driven-development-operating-model.md](planning/specs/active/spec-spec-driven-development-operating-model.md)
- [planning/specs/active/spec-instruction-rules-board-and-surface-layout.md](planning/specs/active/spec-instruction-rules-board-and-surface-layout.md)
- [planning/specs/active/spec-agentic-surface-boundary-separation.md](planning/specs/active/spec-agentic-surface-boundary-separation.md)
- [planning/specs/active/spec-repository-consolidation-continuity.md](planning/specs/active/spec-repository-consolidation-continuity.md)
- [planning/specs/active/spec-token-economy-optimization.md](planning/specs/active/spec-token-economy-optimization.md)
- [planning/specs/active/spec-rag-cag-sqlite-evolution.md](planning/specs/active/spec-rag-cag-sqlite-evolution.md)
- [planning/specs/active/spec-build-target-cleanup-and-artifact-pruning.md](planning/specs/active/spec-build-target-cleanup-and-artifact-pruning.md)
- [planning/specs/active/spec-instruction-governance-and-super-agent-retention.md](planning/specs/active/spec-instruction-governance-and-super-agent-retention.md)
- [planning/specs/active/spec-script-retirement-tail-cutover.md](planning/specs/active/spec-script-retirement-tail-cutover.md)

---

## Rules

- Create or update specs in `planning/specs/active/` for non-trivial feature, behavior, workflow, or architecture work.
- Reuse an existing active spec for the same workstream instead of creating duplicates.
- Move a spec to `planning/specs/completed/` only when the workstream is materially finished and the active plan is also ready to close.
- Do not keep umbrella roadmaps or strategic backlogs open indefinitely in `planning/specs/active/`.
- Use stable slugged names such as `spec-<scope>.md`.
- Keep the spec focused on intent, decisions, alternatives, risks, and acceptance criteria instead of turning it into the task execution plan.

---

## References

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
- [planning/specs/completed/spec-script-retirement-phase-13.md](planning/specs/completed/spec-script-retirement-phase-13.md)
- [planning/specs/completed/spec-script-retirement-phase-15.md](planning/specs/completed/spec-script-retirement-phase-15.md)
- [planning/specs/completed/spec-script-retirement-phase-14.md](planning/specs/completed/spec-script-retirement-phase-14.md)
- [planning/specs/completed/spec-script-retirement-phase-16.md](planning/specs/completed/spec-script-retirement-phase-16.md)
- [planning/specs/completed/spec-script-retirement-phase-17.md](planning/specs/completed/spec-script-retirement-phase-17.md)
- [planning/specs/completed/spec-script-retirement-phase-18.md](planning/specs/completed/spec-script-retirement-phase-18.md)
- [planning/specs/completed/spec-script-retirement-phase-19.md](planning/specs/completed/spec-script-retirement-phase-19.md)
- [planning/specs/completed/spec-script-retirement-phase-20c-self-heal.md](planning/specs/completed/spec-script-retirement-phase-20c-self-heal.md)
- [planning/specs/completed/spec-script-retirement-phase-20d-provider-surface-dispatcher.md](planning/specs/completed/spec-script-retirement-phase-20d-provider-surface-dispatcher.md)
- [planning/specs/completed/spec-script-retirement-phase-20e-catalog-native-renderer-dispatch.md](planning/specs/completed/spec-script-retirement-phase-20e-catalog-native-renderer-dispatch.md)
- [planning/specs/completed/spec-script-retirement-phase-20f-codex-orchestration-renderer.md](planning/specs/completed/spec-script-retirement-phase-20f-codex-orchestration-renderer.md)
- [planning/specs/completed/spec-enterprise-rust-runtime-transcription-architecture.md](planning/specs/completed/spec-enterprise-rust-runtime-transcription-architecture.md)
- [planning/specs/completed/spec-ai-usage-history-and-sqlite-local-memory.md](planning/specs/completed/spec-ai-usage-history-and-sqlite-local-memory.md)
- [planning/specs/completed/spec-readme-standards-repository-normalization.md](planning/specs/completed/spec-readme-standards-repository-normalization.md)
- [planning/specs/completed/spec-repository-unification-and-rust-migration.md](planning/specs/completed/spec-repository-unification-and-rust-migration.md)
- [super-agent.instructions.md](../../.github/instructions/super-agent.instructions.md)
- [brainstorm-spec-workflow.instructions.md](../../.github/instructions/brainstorm-spec-workflow.instructions.md)
- [subagent-planning-workflow.instructions.md](../../.github/instructions/subagent-planning-workflow.instructions.md)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---