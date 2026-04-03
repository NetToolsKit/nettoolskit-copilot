# Build Target Cleanup And Artifact Pruning Plan

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-03-30 07:31
- Objective: prevent `.build/target` and related transient artifacts from growing without bound and consuming excessive disk on the developer workstation.
- Normalized Request: create a planning workstream for cargo target cleanup and generated-artifact pruning so the repository stops accumulating multi-gigabyte build state.
- Active Branch: `main` (planning only; implementation branches TBD)
- Spec Path: `planning/specs/active/spec-build-target-cleanup-and-artifact-pruning.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Inputs:
  - `.cargo/config.toml`
  - `.gitignore`
  - `instructions/core/ntk-core-artifact-layout.instructions.md`
  - `instructions/architecture/backend/ntk-backend-rust-code-organization.instructions.md`
  - `planning/completed/enterprise-progress-tracker.md`

---

## Scope Summary

This plan coordinates four cleanup slices:

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| B1 | Baseline and budget measurement | build output footprint | 🔴 Immediate | none |
| B2 | Safe pruning policy | `.build/target` and transient caches | 🔴 Immediate | B1 |
| B3 | Automation and guardrails | scripts/CI/docs | 🟠 High | B1, B2 |
| B4 | User guidance and drift prevention | repo docs and planning | 🟠 High | B2, B3 |

---

## Ordered Tasks

### [2026-03-30 07:31] Task B1: Measure The Current Build Footprint

- Record the current size and growth pattern of `.build/target`.
- Check whether any build output still spills into `target/` or other uncontrolled folders.
- Use the measurement as the acceptance baseline for the cleanup policy.
- Commit checkpoint:
  - `docs/planning: freeze cargo target cleanup baseline`

### [2026-03-30 07:31] Task B2: Define Safe Prune Policy

- Decide what can be pruned automatically and what should remain cached.
- Keep worktree safety in mind for parallel branches and local development.
- Ensure cleanup does not destroy useful incremental state unnecessarily.
- Commit checkpoint:
  - `docs/planning: define cargo target pruning policy`

### [2026-03-30 07:31] Task B3: Add Cleanup Automation

- Add or update scripts/commands that can prune stale build artifacts deterministically.
- Prefer repository-owned automation over ad hoc manual deletion.
- Validate the automation on Windows.
- Commit checkpoint:
  - `feat(build): add deterministic cargo target cleanup automation`

### [2026-03-30 07:31] Task B4: Document The Ongoing Hygiene Rule

- Document the cleanup policy in the repo operating model and/or README.
- Clarify when to use scoped cleanup versus full cleanup.
- Explain how to avoid reintroducing uncontrolled `target/` growth.
- Commit checkpoint:
  - `docs(build): document cargo target hygiene and prune policy`

---

## Validation Checklist

- `cargo build --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `git diff --check`
- size check for `.build/target` before and after prune

---

## Risks And Mitigations

- Over-pruning can slow down iterative Rust work.
- Cross-worktree cleanup can accidentally remove artifacts still needed by another active branch.
- Mitigation: keep cleanup scoped, measurable, and opt-in until the policy is proven.

---

## Specialist And Closeout

- Recommended specialist: `ops-devops-platform-engineer`
- Tester: required
- Reviewer: required
- Release closeout: required
- README update: required for any visible cleanup policy change
- Changelog: required once implementation lands
- Suggested commit message style:
  - `fix(build): add cargo target cleanup guardrails`
  - `docs(planning): record build artifact pruning roadmap`
