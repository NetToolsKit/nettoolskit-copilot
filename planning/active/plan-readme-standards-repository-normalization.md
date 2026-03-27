# Plan: Repository README Standards Normalization

## Objective

Normalize every `README.md` in the repository to the repository-owned standard, codify the real README classes in instructions and baseline governance, and close the validation gap so the full README estate stays enforceable.

## Scope

- root README: `1`
- crate/package READMEs: `13`
- workspace/reference READMEs: `13`
- total tracked README files: `27`

## Source Of Truth

- `.github/instructions/nettoolskit-rules.instructions.md`
- `.github/instructions/readme.instructions.md`
- `.github/templates/readme-template.md`
- `.github/governance/readme-standards.baseline.json`
- `planning/specs/active/spec-readme-standards-repository-normalization.md`

## Decisions Locked

- Treat `crates/*` and `crates/commands/*` as the effective package README scope for this repository.
- Introduce an explicit workspace/reference README class for `planning/**`, `scripts/README.md`, and `definitions/**`.
- Expand the README validation baseline to the full repository README estate.
- Rewrite README files in grouped slices so commits stay reviewable.

## Execution Slices

### Slice 1: Governance And Instruction Alignment

Status: `[ ]` Pending

- Update repository README instructions to match the real topology and README classes.
- Expand `readme-standards.baseline.json` to `27` tracked README files.
- Record the README classes directly in the active plan/spec.

### Slice 2: Root And Umbrella READMEs

Status: `[ ]` Pending

- `README.md`
- `planning/README.md`
- `planning/specs/README.md`
- `scripts/README.md`
- `definitions/README.md`

### Slice 3: Crate And Command Package READMEs

Status: `[ ]` Pending

- `crates/*/README.md`
- `crates/commands/*/README.md`

### Slice 4: Definitions And Provider Reference READMEs

Status: `[ ]` Pending

- `definitions/shared/**/README.md`
- `definitions/providers/**/README.md`

## Validation

- `cargo test -p nettoolskit-validation`
- `git diff --check`
- targeted spot checks for `Contents`, `License`, and section order across each README class

## Parallelization Plan

- Worker A: root and umbrella README files
- Worker B: `crates/*` and `crates/commands/*` README files
- Worker C: `definitions/shared/**` README files
- Worker D: `definitions/providers/**` README files
- Main rollout: governance/instruction alignment, planning, baseline coordination, final integration, validation, and commits

## Completion Criteria

- The README class rules are explicit in repository instructions and planning.
- The validation baseline tracks all `27` README files.
- All README files are rewritten to the correct class standard.
- The workstream is committed in reviewable phases with validation evidence.