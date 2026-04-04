# Plan: Repository README Standards Normalization

## Objective

Normalize every `README.md` in the repository to the repository-owned standard, codify the real README classes in instructions and baseline governance, and close the validation gap so the full README estate stays enforceable.

## Scope

- root README: `1`
- crate/package READMEs: `13`
- workspace/reference READMEs: `13`
- total tracked README files: `27`

## Source Of Truth

- `.github/instructions/docs/ntk-docs-repository-readme-overrides.instructions.md`
- `.github/instructions/docs/ntk-docs-readme.instructions.md`
- `.github/templates/readme-template.md`
- `.github/governance/readme-standards.baseline.json`
- `planning/specs/completed/spec-readme-standards-repository-normalization.md`

## Decisions Locked

- Treat `crates/*` and `crates/commands/*` as the effective package README scope for this repository.
- Introduce an explicit workspace/reference README class for `planning/**`, `scripts/README.md`, and `definitions/**`.
- Expand the README validation baseline to the full repository README estate.
- Rewrite README files in grouped slices so commits stay reviewable.

## Execution Slices

### Slice 1: Governance And Instruction Alignment

Status: `[x]` Completed

Commit:
- `d5dbfef` `docs(readme): align repository rules with full readme estate`

- Update repository README instructions to match the real topology and README classes.
- Expand `readme-standards.baseline.json` to `27` tracked README files.
- Record the README classes directly in the active plan/spec.

### Slice 2: Root And Umbrella READMEs

Status: `[x]` Completed

Commit:
- `974ebd3` `docs(readme): normalize workspace and reference indexes`

- `README.md`
- `planning/README.md`
- `planning/specs/README.md`
- `scripts/README.md`
- `definitions/README.md`

### Slice 3: Crate And Command Package READMEs

Status: `[x]` Completed

Commits:
- `7522ab5` `docs(commands): normalize command surface readmes`
- `dc15132` `docs(commands): normalize runtime templating validation readmes`
- `43ff5ba` `docs(readme): normalize core package cluster`

- `crates/*/README.md`
- `crates/commands/*/README.md`

### Slice 4: Definitions And Provider Reference READMEs

Status: `[x]` Completed

Commits:
- `41f113f` `docs(readme): normalize shared definitions cluster`
- `70012d2` `docs(readme): normalize codex provider reference cluster`

- `definitions/shared/**/README.md`
- `definitions/providers/**/README.md`

## Validation

- `pwsh -File .\scripts\validation\validate-readme-standards.ps1`: passed
- `git diff --check`: passed
- targeted spot checks for `Contents`, `License`, and section order across each README class: completed
- `cargo test -p nettoolskit-validation`: failed due concurrent, non-README changes already present in `crates/commands/validation`

## Parallelization Plan

- Worker A: root and umbrella README files
- Worker B: `crates/*` and `crates/commands/*` README files
- Worker C: `definitions/shared/**` README files
- Worker D: `definitions/providers/**` README files
- Main rollout: governance/instruction alignment, planning, baseline coordination, final integration, validation, and commits

## Completion Criteria

- `[x]` The README class rules are explicit in repository instructions and planning.
- `[x]` The validation baseline tracks all `27` README files.
- `[x]` All README files are rewritten to the correct class standard.
- `[x]` The workstream is committed in reviewable phases with validation evidence.

## Outcome

The README normalization workstream is complete for the full repository estate.
All `27` tracked README files now conform to the repository-owned class model and pass the executable README validator.
The only remaining validation noise in this branch is unrelated concurrent Rust test drift inside `crates/commands/validation`, outside the scope of the README rewrite.