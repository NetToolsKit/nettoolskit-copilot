# Planning Workspace

Versioned planning artifacts for non-trivial work live here. This folder is part of the repository operating model, not disposable temporary state.

## Purpose

- Keep active plans visible and searchable.
- Preserve completed plans as operational history.
- Support the mandatory sub-agent chain for non-trivial work.
- Separate stable planning assets from transient runtime artifacts under `.temp/`.

## Structure

```text
planning/
├─ README.md
├─ active/
└─ completed/
```

## Rules

- Create or update active plans in `planning/active/`.
- Move a plan to `planning/completed/` only after implementation, validation, review, and closeout are materially complete.
- Reuse an existing active plan for the same workstream instead of creating duplicates.
- Use stable slugged names such as `plan-<scope>.md` when creating new plans.
- Keep transient run outputs, smoke artifacts, and execution scratch data in `.temp/`, not here.

## References

- `.github/instructions/master-orchestrator.instructions.md`
- `.github/instructions/subagent-planning-workflow.instructions.md`
- `.github/instructions/repository-operating-model.instructions.md`