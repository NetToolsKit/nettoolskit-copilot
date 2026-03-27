# Spec: Repository README Standards Normalization

## Context

The repository currently has `27` `README.md` files spread across root, `crates/`, `crates/commands/`, `planning/`, `scripts/`, and `definitions/`.
The authoritative README guidance already exists, but it is incomplete for the repository's real topology:

- `.github/instructions/nettoolskit-rules.instructions.md`
- `.github/instructions/readme.instructions.md`
- `.github/templates/readme-template.md`
- `.github/governance/readme-standards.baseline.json`

The highest-priority repository override still talks about package READMEs under `src/*`, while the repository actually exposes package-like surfaces under `crates/*` and `crates/commands/*`.
The executable baseline only tracks a small subset of README files, so most of the repository can drift away from the intended standard without validation catching it.

## Problem

README structure and coverage are inconsistent across the repository:

- the root README carries sections that do not match the current repository-specific override
- crate/package READMEs are not consistently normalized to the package pattern
- workspace/reference READMEs under `planning/`, `scripts/`, and `definitions/` do not have an explicit repository-owned class definition
- the validation baseline does not cover the full README estate

## Decision

Normalize the full README estate around three repository-owned classes:

1. Root README
   - `README.md`
   - follows the existing root override, with the repository topology corrected from `src/*` to `crates/*`

2. Package/Crate README
   - all README files under `crates/*` and `crates/commands/*`
   - treated as the real equivalent of the current `src/*` package rule
   - must follow the package override order and coverage expectations

3. Workspace/Reference README
   - `planning/README.md`
   - `planning/specs/README.md`
   - `scripts/README.md`
   - all README files under `definitions/**`
   - follows the generic README baseline with a repository-specific narrowed section set for non-package reference surfaces

## README Class Rules

### Root README

- Sections must stay in this order:
  - `Introduction`
  - `Features`
  - `Contents`
  - `Build and Tests`
  - `Contributing`
  - `Dependencies`
  - `References`
  - `License`
- Must not include:
  - `Installation`
  - `Quick Start`
  - `Usage Examples`
  - `API Reference`
- Must not include code examples.
- `References` must list all crate/package README files under `crates/*` and `crates/commands/*`.

### Package/Crate README

- Applies to:
  - `crates/*/README.md`
  - `crates/commands/*/README.md`
- Sections must stay in this order:
  - `Introduction`
  - `Features`
  - `Contents`
  - `Installation`
  - `Quick Start`
  - `Usage Examples`
  - `API Reference`
  - `References`
  - `License`
- Must omit:
  - `Build and Tests`
  - `Contributing`
  - `Dependencies`
- `Usage Examples` should use numbered subsections when more than one scenario exists.
- `API Reference` should be grouped by logical areas when the public surface is broad.
- Enum tables and `Data Shapes` tables should be included when the underlying public surface makes them relevant.
- Coverage target remains `>= 70%` of the intended consumer-facing surface.

### Workspace/Reference README

- Applies to:
  - `planning/**/README.md`
  - `scripts/README.md`
  - `definitions/**/README.md`
- Sections must stay in this order:
  - `Introduction`
  - `Features`
  - `Contents`
  - `References`
  - `License`
- Additional sections are allowed only when the surface is operational and materially benefits from them:
  - `Installation`
  - `Quick Start`
  - `Usage Examples`
  - `Build and Tests`
- These READMEs must not pretend to be package-install guides when they are actually indexes or reference surfaces.

## Validation Direction

- Expand `.github/governance/readme-standards.baseline.json` from the current narrow scope to all `27` tracked README files.
- Keep `validate-readme-standards` as the executable gate.
- Align the repository-specific README instruction override with the real `crates/*` topology and the new workspace/reference class.

## Non-Goals

- No behavior or API changes.
- No invented package examples.
- No conversion of README work into a generic prose rewrite disconnected from current code and folder purpose.

## Risks

- Crate READMEs may overclaim APIs if rewritten too aggressively.
- Reference/workspace READMEs may drift into package-style sections that do not match their actual purpose.
- The root README can become noisy if every crate reference is added without clear grouping.

## Acceptance Criteria

- A versioned plan exists for the full README normalization workstream.
- The repository README instructions explicitly cover `crates/*` and workspace/reference READMEs.
- The README standards baseline covers all `27` README files.
- Every repository README is rewritten in English and matches its assigned class.
- Root, crate/package, and workspace/reference READMEs use consistent section order and house style.
- `cargo test -p nettoolskit-validation` passes after the baseline/rule changes.
- `git diff --check` passes.