# Planning Workspace

Versioned planning artifacts for non-trivial work live here. This folder is part of the repository operating model, not disposable temporary state.

## Purpose

- Keep active plans visible and searchable.
- Preserve completed plans as operational history.
- Support versioned specifications before multi-step execution work.
- Separate stable planning assets from local build and deployment artifacts under `./.build/` and `./.deployment/`.

## Structure

```text
planning/
├─ README.md
├─ active/
├─ completed/
└─ specs/
   ├─ README.md
   ├─ active/
   └─ completed/
```

## Rules

- Create or update active plans in `planning/active/`.
- Move a plan to `planning/completed/` only after implementation, validation, review, and closeout are materially complete.
- Create or update versioned specs in `planning/specs/active/` when non-trivial work needs design decisions locked before planning.
- Move specs to `planning/specs/completed/` together with their workstream when the associated plan is materially complete.
- `planning/active`, `planning/completed`, `planning/specs/active`, and `planning/specs/completed` are created on demand and are not kept alive with placeholder files.
- Reuse an existing active plan for the same workstream instead of creating duplicates.
- Use stable slugged names such as `plan-<scope>.md` and `spec-<scope>.md` when creating new artifacts.

## Current Active Plan

- `planning/active/plan-repository-operations-hygiene.md`
- `planning/active/plan-repository-unification-and-rust-migration.md`
- `planning/active/plan-readme-standards-repository-normalization.md`

## Current Delivery Snapshot

As of `2026-03-20`:

- phases `0` through `10` are delivered at the roadmap level
- commercial hardening slices `10.1` through `10.6` are complete
- canonical planning no longer lives under `.temp`

## Completed Reference Plans

- `planning/completed/enterprise-progress-tracker.md`
- `planning/completed/enterprise-roadmap-2026-02-27.md`
- `planning/completed/task-phase-10.0-commercial-platform-hardening.md`