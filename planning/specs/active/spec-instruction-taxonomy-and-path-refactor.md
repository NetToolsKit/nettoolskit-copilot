# Instruction Taxonomy And Path Refactor Spec

Generated: 2026-04-03 00:00

## Status

- LastUpdated: 2026-04-03 01:05
- Objective: refactor the repository instruction system into a clearer folder taxonomy with stable `ntk-*` naming, explicit authority rules, and reduced duplication/divergence across projected and shared instruction surfaces.
- Normalized Request: reorganize the instruction system so instructions are grouped by concern, use stable prefixed names, and reduce repetition and drift while the repository is still in development.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/active/plan-instruction-taxonomy-and-path-refactor.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository already separates shared instruction sources under `definitions/shared/instructions/` from projected GitHub-facing copies under `.github/instructions/`, but the current flat layout still mixes architecture, process, docs, runtime, security, and stack-specific guidance in one directory. Several instruction files overlap in responsibility, and there is already real drift between some shared and projected copies.

---

## Design Intent

- Organize instructions by concern using subfolders instead of one flat directory.
- Use stable repository-owned `ntk-*` prefixes for instruction filenames.
- Keep `definitions/shared/instructions/` as the canonical editable source and `.github/instructions/` as the projected mirror.
- Reduce content overlap by sharpening file ownership:
  - backend architecture
  - backend stack-specific
  - frontend architecture
  - frontend stack-specific
  - agentic/runtime architecture
  - process/workflow
  - docs/readme
  - security/privacy
  - infrastructure/operations
- Preserve routing, prompts, skills, governance manifests, and README references through path updates.

---

## Options Considered

1. Keep the flat instruction layout and only adjust wording.
   - Rejected: taxonomy remains unclear and drift risk stays high.
2. Add folders but keep old names.
   - Rejected: ownership improves only partially and naming remains inconsistent.
3. Refactor to grouped folders with stable `ntk-*` names and authority rules.
   - Preferred: strongest long-term maintainability and clearest path semantics.

---

## Proposed Taxonomy

- `instructions/core/`
  - repository operating model
  - authoritative sources
  - artifact layout
  - super-agent lifecycle
- `instructions/process/`
  - planning/spec workflows
  - worktree isolation
  - TDD/verification
  - PR/changelog feedback
- `instructions/architecture/backend/`
  - backend architecture baseline
  - backend stack-specific guidance
- `instructions/architecture/frontend/`
  - frontend architecture baseline
  - Vue/Quasar stack guidance
  - UI/UX rules
- `instructions/architecture/agentic/`
  - agentic surface model
  - context economy / RAG/CAG
- `instructions/operations/devops/`
  - CI/CD platform policy
  - workflow generation
- `instructions/operations/automation/`
  - PowerShell execution/creation
  - runtime/editor efficiency
- `instructions/operations/containers/`
  - Docker
  - Kubernetes
- `instructions/operations/reliability/`
  - observability/SRE
  - resilience
  - microservices/runtime performance
- `instructions/operations/quality/`
  - static analysis and quality gates
- `instructions/data/`
- `instructions/security/`
- Keep data and security as separate semantic lanes so database/ORM guidance does not drift into vulnerability, privacy, or API-security policy.
  - database/ORM
  - privacy/compliance
  - security vulnerabilities
  - API performance/security
- `instructions/docs/`
  - README
  - repo-specific README overrides
  - prompt template authoring

The exact numbering may adjust, but the architecture/process/operations/docs separation must remain explicit and generic buckets must keep shrinking over time.

---

## Acceptance Criteria

- Instructions are grouped into concern-based folders with stable `ntk-*` prefixed filenames.
- Shared and projected instruction copies are aligned for all renamed files.
- Routing catalog, prompts, skills, manifests, plans, and README references point to the new paths.
- README policy and repo override files no longer drift between shared and projected copies.
- The most overlapping backend/frontend instruction surfaces have sharper ownership after the refactor.
- The generic `runtime-ops/` lane is replaced by narrower `operations/*` subfolders so DevOps, automation, containers, reliability, and quality guidance are not mixed in one bucket.

---

## Planning Readiness

- Ready for planning immediately.
- The first slice should freeze the target taxonomy and rename map before moving files.
- Implementation should update references in the same slice as file moves to avoid dangling paths.