# Task Output Spill To Disk And Retention Control Plan

Generated: 2026-03-31 00:00

## Status

- LastUpdated: 2026-03-31 00:00
- Objective: add bounded task-output buffering, spill-to-disk, progress inspection, and retention cleanup to protect runtime memory and disk usage.
- Normalized Request: create a detailed workstream for handling large command/task output safely and predictably in the repository runtime.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-task-output-spill-to-disk-and-retention-control.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Related Workstreams:
  - `planning/completed/plan-build-target-cleanup-and-artifact-pruning.md`
  - `planning/completed/plan-repository-consolidation-continuity.md`
- Inputs:
  - `crates/orchestrator/src/execution/*`
  - `crates/commands/runtime/src/*`
  - `.build/*`
  - `README.md`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| O1 | Output lifecycle matrix | small/medium/large output classes | 🔴 Immediate | none |
| O2 | Hybrid buffer/storage boundary | memory + spill storage services | 🔴 Immediate | O1 |
| O3 | Tail/progress inspection surface | runtime diagnostics/CLI | 🟠 High | O2 |
| O4 | Retention and pruning policy | output cleanup and caps | 🟠 High | O1, O2 |
| O5 | Operator docs and telemetry | status/reporting | 🟡 Medium | O3, O4 |

---

## Ordered Tasks

### [2026-03-31 00:00] Task O1: Freeze Output Lifecycle And Size Classes

- Define output classes by size and duration.
- Define memory cap, spill threshold, file cap, and truncation semantics.
- Define retention classes and cleanup triggers.
- Commit checkpoint:
  - `docs(planning): freeze task output lifecycle matrix`

### [2026-03-31 00:00] Task O2: Add Hybrid Buffer And Spill Boundaries

- Design the output service boundary for:
  - in-memory buffering
  - spill-to-disk storage
  - explicit flush/finalize behavior
- Keep orchestration code from owning file append logic directly.
- Commit checkpoint:
  - `refactor(runtime): add hybrid task output boundary`

### [2026-03-31 00:00] Task O3: Add Tail And Progress Inspection

- Design bounded APIs for:
  - tail
  - recent lines
  - byte counts
  - truncation flags
- Expose these through runtime diagnostics or CLI inspection as appropriate.
- Commit checkpoint:
  - `feat(runtime): add bounded task output inspection`

### [2026-03-31 00:00] Task O4: Add Retention And Pruning Control

- Define output cleanup scheduling and explicit prune commands.
- Ensure runtime output does not become another uncontrolled disk sink.
- Align task-output cleanup with build-target cleanup rather than mixing the concerns.
- Commit checkpoint:
  - `feat(runtime): add task output retention and pruning`

### [2026-03-31 00:00] Task O5: Document Runtime Output Handling

- Update README and operator guidance with output caps and retention behavior.
- Add telemetry/diagnostics expectations for large-output scenarios.
- Document how operators recover from truncated or capped outputs.
- Commit checkpoint:
  - `docs(runtime): document task output spill and retention model`

---

## Validation Checklist

- `cargo test -p nettoolskit-orchestrator --quiet`
- `cargo test -p nettoolskit-runtime --quiet`
- `cargo test -p nettoolskit-cli --quiet`
- `git diff --check`

---

## Risks And Mitigations

- Output spill can create data-loss confusion if truncation is not explicit.
- Retention can remove useful diagnostics if classes are too aggressive.
- Tail/progress APIs can accidentally read large files inefficiently.
- Mitigation: freeze lifecycle classes first and require explicit truncation and cap metadata.

---

## Specialist And Closeout

- Recommended specialist: `dev-rust-engineer`
- Tester: required
- Reviewer: required
- README update: required
- Suggested commit message style:
  - `feat(runtime): add task output spill and retention boundaries`
  - `docs(planning): record task output spill roadmap`