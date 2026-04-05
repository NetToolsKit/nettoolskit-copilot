# Instruction Taxonomy And Path Refactor Plan

Generated: 2026-04-03 00:00

## Status

- LastUpdated: 2026-04-05 00:20
- Objective: refactor the repository definition system into a shallow, predictable layout rooted in `definitions/instructions/`, `definitions/templates/`, `definitions/agents/`, `definitions/skills/`, `definitions/hooks/`, and `definitions/providers/`, with stable file naming, preserved documents, and safe migration from legacy roots.
- Normalized Request: reorganize the repository definition system while the workspace is still evolving so `definitions/` becomes the canonical root, `instructions/` keeps only five primary categories, templates are grouped by artifact type, docs gain stable manifest samples, and no existing document is lost during the migration.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-instruction-taxonomy-and-path-refactor.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Related Workstreams:
  - `planning/active/plan-instruction-rules-board-and-surface-layout.md`
  - `planning/active/plan-instruction-governance-and-super-agent-retention.md`
  - `planning/active/plan-repository-consolidation-continuity.md`
- Inputs:
  - `definitions/*`
  - `definitions/shared/*`
  - `definitions/providers/*`
  - `docs/manifest/*`
  - `planning/README.md`
  - `planning/specs/README.md`
  - `crates/commands/validation/*`
  - `scripts/validation/*`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| I1 | Freeze taxonomy and rename map | definition roots + file names | 🔴 Immediate | none |
| I2 | Establish canonical `definitions/` roots | `definitions/instructions`, `templates`, `agents`, `skills`, `hooks`, `providers` | 🔴 Immediate | I1 |
| I3 | Reproject generated surfaces and unify drift | `.github`, `.codex`, `.claude` copies + overrides | 🟡 Deferred | I15 |
| I4 | Update canonical consumers and validation paths | definitions-aware routing, fixtures, prompts, skills, manifests, plans | 🔴 Immediate | I2 |
| I5 | Tighten backend/frontend ownership | reduce overlap in the highest-conflict files | ✅ Done | I3 |
| I6 | Add taxonomy docs and validation references | README/governance/closeout | 🟡 Medium | I4, I5 |
| I7 | Split generic operations lane | replace `runtime-ops/` with narrower `operations/*` subdomains | ✅ Done | I4, I6 |
| I8 | Split process lane into workflow subdomains | replace flat `process/` paths with `planning`, `collaboration`, and `delivery` | ✅ Done | I4, I6 |
| I9 | Separate agent-controller lane | move `super-agent` from `core/` into dedicated `agents/` surfaces | ✅ Done | I4, I6, I8 |
| I10 | Freeze root taxonomy and shallow depth rule | move from the transitional lane split to `instructions/`, `agents/`, `skills/`, and `hooks/` roots with shallow paths and explicit authority | 🔴 Immediate | I9 |
| I11 | Re-map semantic lanes to five primary instruction categories | collapse current instruction lanes into `governance`, `development`, `operations`, `security`, and `data`, with specialization encoded in file names | 🔴 Immediate | I10 |
| I12 | Add canonical template and sample lanes | create `definitions/templates/*` and `docs/samples/manifests/` without deleting legacy sources | 🟠 High | I2, I10, I11 |
| I13 | Scaffold shallow domain instruction copies | copy existing non-governance instruction content into `development`, `operations`, `security`, and `data` lanes with stable new file names while keeping legacy roots intact | 🔴 Immediate | I10, I11 |
| I14 | Simplify instruction lane docs and pause generated-surface expansion | remove per-lane instruction READMEs, freeze generated-surface scaffolds, and avoid further `.github/.codex/.claude` churn until validators are definitions-aware | ✅ Done | I13 |
| I15 | Repoint validation and audit code to canonical definitions | `crates/commands/validation/*`, `scripts/validation/*`, fixtures, and audit helpers must treat `definitions/` as primary and generated surfaces as deferred outputs | 🔴 Immediate | I10, I11, I12, I13, I14 |

---

## Ordered Tasks

### [2026-04-03 00:00] Task I1: Freeze Taxonomy And Rename Map

- Define the target folder taxonomy inside:
  - `definitions/instructions/`
  - `definitions/templates/`
  - `definitions/agents/`
  - `definitions/skills/`
  - `definitions/hooks/`
  - `definitions/providers/`
  - `.github/instructions/`
- Define the stable rename map for every instruction file.
- Keep `.instructions.md` suffix and add `ntk-*` prefix to the renamed files.
- Commit checkpoint:
  - `docs(planning): freeze instruction taxonomy and rename map`

### [2026-04-03 00:00] Task I2: Move Canonical Shared Instructions

- Move canonical instruction sources into `definitions/instructions/` from the legacy `definitions/shared/instructions/` tree.
- Initialize the other canonical roots in `definitions/` at the same time so future slices can migrate templates, agents, skills, and hooks into place without inventing new roots later.
- Update canonical copies first and keep legacy compatibility copies until routing and provider consumers are updated.
- Keep semantics stable during the move.
- Commit checkpoint:
  - `refactor(instructions): move canonical shared instruction sources`

### [2026-04-03 00:00] Task I3: Reproject And Unify Generated Copies

- Reproject `.github/`, `.codex/`, and `.claude/` only after canonical `definitions/` roots and validators are aligned.
- Eliminate shared/projected drift in:
  - README baseline
  - repo-specific README overrides
  - any renamed instruction that still differs across the mirror
- Status:
  - deferred; generated/runtime surfaces are explicitly last so canonical instruction authorship and validator contracts can stabilize first
- Commit checkpoint:
  - `refactor(instructions): align projected instruction surfaces`

### [2026-04-03 00:00] Task I4: Update All Consumers

- Update references in:
  - `definitions/providers/*`
  - governance manifests and policies
  - planning indexes and active plans/specs
- Update validators, fixtures, and audit helpers before generated/runtime surfaces.
- Ensure there are no dangling legacy paths in canonical `definitions/` consumers.
- Status:
  - in progress; `validate-instructions` now prioritizes canonical `definitions/` assets and codex skill definitions
  - in progress; GitHub provider prompt/chatmode markdown references now target canonical `definitions/instructions/*`, `definitions/templates/*`, and `definitions/providers/github/root/*`
  - in progress; canonical provider roots, orchestration prompts, and skill packs now resolve the shallow `definitions/instructions/{governance,development,operations,security,data}` taxonomy instead of the legacy `core/process/architecture/runtime-ops` path graph
  - in progress; validation fixtures, active planning indexes, and VS Code provider snippets now point to canonical `definitions/` roots and the shallow taxonomy instead of legacy lane paths
  - in progress; routing-coverage and validate-instructions now default to the canonical provider catalog at `definitions/providers/github/root/instruction-routing.catalog.yml` and resolve `instructions/*` references against `definitions/`
  - in progress; Rust validation tests and CLI validation command fixtures now scaffold canonical `definitions/` trees in temp repos so native checks pass without authored dependence on `.github/*`
  - in progress; provider-authored consumer surfaces under `definitions/providers/{claude,codex,github}` now resolve canonical `definitions/instructions/*`, `definitions/agents/*`, `definitions/templates/*`, and `definitions/providers/github/root/*` paths instead of linking back to projected `.github/*` authoring paths
  - in progress; runtime-facing provider docs, sync skills, and orchestration pipeline metadata now reference canonical governance catalogs under `definitions/providers/github/governance/*` instead of authored `.github/governance/*` paths
  - in progress; agent-orchestration validators and their shared fixtures now resolve the permission matrix plus runtime/model routing catalogs from `definitions/providers/github/governance/*` first while still writing `.github/governance/*` mirrors for transition coverage
  - in progress; shared governance baseline validators and their fixture layers now resolve `validation-profiles`, architecture, README, template, and workspace baselines from `definitions/providers/github/governance/*`, while template fixtures also materialize canonical authored files under `definitions/templates/*`
- Commit checkpoint:
  - `refactor(instructions): update instruction routing and consumers`

### [2026-04-03 00:00] Task I5: Reduce Ownership Overlap

- Narrow backend instruction ownership so architecture, stack-specific conventions, and testing do not repeat excessively.
- Narrow frontend instruction ownership so frontend baseline, Vue/Quasar implementation, Vue/Quasar architecture, and UI/UX each have a distinct role.
- Keep business/commercial guidance out of repo-wide technical instructions.
- Commit checkpoint:
  - `refactor(instructions): narrow backend and frontend ownership`

### [2026-04-03 00:00] Task I6: Document The Taxonomy

- Update instruction-system docs, relevant READMEs, and planning references.
- Record the canonical authority rule:
  - `definitions/instructions` is source of truth
  - provider/runtime projections are downstream consumer surfaces
- Status:
  - complete; semantic taxonomy now exposes `data/` and `security/` as separate lanes instead of the previous combined `data-security/` folder
- Commit checkpoint:
  - `docs(instructions): document instruction taxonomy and authority model`

### [2026-04-04 00:18] Task I7: Split Generic Operations Lane

- Replace the generic `runtime-ops/` bucket with narrower semantic lanes:
  - `operations/devops/`
  - `operations/automation/`
  - `operations/containers/`
  - `operations/reliability/`
  - `operations/quality/`
- Keep the existing `ntk-runtime-*` file names stable during the folder move to minimize rename churn outside path updates.
- Update canonical shared paths first, projected `.github` paths second, then routing, providers, README indexes, prompts, skills, VS Code assets, plans, and validation fixtures in the same slice.
- Status:
  - complete as a transitional slice; canonical shared files and projected `.github` files now live under narrower `operations/*` subdomains that will later collapse into the final shallow `instructions/operations/` category
  - complete; routing catalogs, provider prompts/chatmodes/skills, VS Code settings/snippets, validation fixtures, and active plans now point at the semantic operations subdomains
- Commit checkpoint:
  - `refactor(instructions): split operations taxonomy into semantic subdomains`

### [2026-04-04 00:32] Task I8: Split Flat Process Lane

- Replace the flat `process/` lane with narrower workflow subdomains:
  - `process/planning/`
  - `process/collaboration/`
  - `process/delivery/`
- Keep the existing `ntk-process-*` file names stable during the folder move to minimize rename churn outside path updates.
- Move backend and frontend test-specific guidance out of `process/`; keep only cross-cutting workflow verification in `process/delivery/`.
- Update canonical shared paths first, projected `.github` paths second, then routing, skills, prompts, governance, validation fixtures, and active plans in the same slice.
- Status:
  - complete as a transitional slice; canonical shared files and projected `.github` files now live under `process/planning`, `process/collaboration`, and `process/delivery`, which will later collapse into the final shallow `instructions/governance/` category
  - complete; routing catalogs, provider skills/prompts, governance manifests, validation fixtures, and active planning references now point at the narrower process workflow lanes
  - complete; dedicated `process/README.md` files now document the three workflow lanes and keep platform concerns under `operations/`
- Commit checkpoint:
  - `refactor(instructions): split process taxonomy into workflow subdomains`

### [2026-04-04 02:20] Task I9: Separate Agent-Control From Core

- Move `super-agent` out of `core/` into a dedicated `agents/` lane.
- Keep `core/` reserved for repository invariants:
  - repository operating model
  - authoritative sources
  - artifact layout
- Update canonical shared paths first, projected `.github` paths second, then routing, provider consumers, governance manifests, validation fixtures, and instruction-architecture tests in the same slice.
- Status:
  - complete; `ntk-agents-super-agent.instructions.md` now lives under `agents/` in canonical and projected trees
  - complete; rules-board docs, governance manifests, provider surfaces, and validation fixtures now treat `agents/` as a dedicated semantic lane instead of overloading `core/`
- Commit checkpoint:
  - `refactor(instructions): separate agent-controller guidance from core invariants`

### [2026-04-04 10:05] Task I10: Freeze Root Taxonomy And Depth Rule

- Adopt the canonical root contract under `definitions/`:
  - `instructions/`
  - `templates/`
  - `agents/`
  - `skills/`
  - `hooks/`
  - `providers/`
- Mirror the same conceptual layout for projected and provider-facing consumers where applicable.
- Keep the taxonomy shallow:
  - `instructions/` may contain only the five first-level semantic categories
  - `templates/` may contain only artifact-type categories
  - `agents/`, `skills/`, and `hooks/` may contain only role/lifecycle folders plus files
  - avoid adding another nested lane below those category/role folders unless a later spec explicitly reopens that decision
  - no numeric prefixes
  - no mixed concerns such as `runtime-ops/` or `process/` surviving the final layout
- Status:
  - planning only; no file moves executed in this slice
- Commit checkpoint:
  - `docs(planning): freeze shared instruction root taxonomy`

### [2026-04-04 10:05] Task I11: Re-map Instruction Lanes To Five Primary Categories

- Freeze the final instruction categories as:
  - `instructions/governance/`
  - `instructions/development/`
  - `instructions/operations/`
  - `instructions/security/`
  - `instructions/data/`
- Keep specialization inside those folders at the file-name level, not as another folder taxonomy:
  - `ntk-governance-*`
  - `ntk-development-*`
  - `ntk-operations-*`
  - `ntk-security-*`
  - `ntk-data-*`
- Move agent-controller lifecycle out of `instructions/` entirely and into:
  - `agents/super-agent/`
  - `agents/planner/`
  - `agents/reviewer/`
  - `agents/implementer/`
- Keep reusable engineering playbooks under:
  - `skills/dev-backend/`
  - `skills/dev-frontend/`
  - `skills/dev-rust/`
  - `skills/test/`
  - `skills/security/`
  - `skills/docs/`
- Keep lifecycle automation under:
  - `hooks/session-start/`
  - `hooks/pre-tool-use/`
  - `hooks/subagent-start/`
  - `hooks/stop/`
- Status:
  - planning only; rename and move map still pending execution
- Commit checkpoint:
  - `docs(planning): freeze shallow instruction category model`

### [2026-04-04 11:05] Task I12: Add Canonical Template And Sample Lanes

- Freeze the final template categories under `definitions/templates/` as:
  - `codegen/`
  - `docs/`
  - `manifests/`
  - `prompts/`
  - `workflows/`
- Keep `docs/` focused on human-readable technical documentation and add:
  - `docs/samples/manifests/`
- Treat `docs/samples/manifests/` as the human-facing sample surface while `definitions/templates/manifests/` becomes the canonical authored manifest-template surface.
- Keep `templates/` at the repository root and `definitions/shared/templates/` as legacy compatibility roots until template consumers and crates finish migrating.
- Status:
  - first scaffolding slice ready; canonical root folders and manifest sample lane can be created without deleting legacy documents
  - in progress; the first canonical template copies now exist under `definitions/templates/codegen/` and `definitions/templates/docs/` for the provider prompt surfaces already migrated off `.github/templates/`
  - in progress; migrated shared docs/codegen templates and the root `.NET` scaffold tree now exist under `definitions/templates/{docs,codegen}` while legacy `definitions/shared/templates/` and root `templates/` remain as compatibility surfaces
  - in progress; canonical governance/development/operations instructions and VS Code provider snippets now reference `definitions/templates/*` instead of authored `.github/templates/*` paths
- Commit checkpoint:
  - `docs(repo): scaffold canonical definition roots and manifest samples`

### [2026-04-04 12:08] Task I13: Scaffold Shallow Domain Instruction Copies

- Copy the current canonical instruction content into the new shallow instruction lanes:
  - `definitions/instructions/development/`
  - `definitions/instructions/operations/`
  - `definitions/instructions/security/`
  - `definitions/instructions/data/`
- Keep legacy copies under `definitions/shared/instructions/` intact during this phase.
- Encode specialization in the new file names instead of creating deeper subfolders.
- Normalize the first pass of remapping as follows:
  - backend, frontend, agentic, and ORM guidance move under `development/`
  - DevOps, platform, reliability, quality, and automation guidance move under `operations/`
  - vulnerability, API, and supply-chain guidance remain under `security/`
  - database, database-operations, and privacy/data-compliance guidance move under `data/`
- Status:
  - in progress; first shallow copies created for all four lanes without deleting legacy sources
  - ORM now enters the shallow model as `ntk-development-persistence-orm.instructions.md`
  - privacy/data-compliance now enters the shallow model as `ntk-data-privacy-compliance.instructions.md`
- Commit checkpoint:
  - `docs(instructions): scaffold shallow domain instruction copies`

### [2026-04-04 12:32] Task I14: Simplify Instruction Lane Docs And Pause Generated-Surface Expansion

- Remove redundant per-folder `README.md` files from instruction category folders.
- Keep instruction taxonomy discovery in the root READMEs instead of multiplying lane-local docs.
- Freeze the current generated-surface scaffolds and avoid further `.github/.codex/.claude` migration work until validator and audit code treats `definitions/` as primary.
- Status:
  - complete; shallow generated-surface scaffolds exist as transitional output only
  - complete; redundant instruction-lane `README.md` files are removed in favor of root-level taxonomy docs
- Commit checkpoint:
  - `docs(instructions): scaffold shallow projected instruction copies`

### [2026-04-04 14:25] Task I15: Repoint Validation And Audit Code To Canonical Definitions

- Refactor validation and audit code so canonical `definitions/` assets are the primary contract:
  - `definitions/instructions/*`
  - `definitions/agents/*`
  - `definitions/providers/*`
- Treat `.github/`, `.codex/`, and `.claude/` as generated/runtime output surfaces and defer strict enforcement of those copies until the canonical move is complete.
- Start with:
  - `crates/commands/validation/src/instruction_graph/instructions.rs`
  - `crates/commands/validation/tests/support/instruction_graph_fixtures.rs`
  - `crates/commands/validation/tests/instruction_graph/instructions_tests.rs`
  - `scripts/validation/fixtures/routing-golden-tests.json`
- Follow with the remaining audit surfaces:
  - `instruction_architecture.rs`
  - `authoritative_source_policy.rs`
  - routing coverage and documentation metadata validators
- Status:
  - in progress; the first slice repoints `validate-instructions` to canonical definitions and codex skill definitions before generated surfaces
  - in progress; `validate-instruction-architecture` and `validate-authoritative-source-policy` now default to `definitions/providers/github/{governance,root,prompts}` plus `definitions/instructions/`, with canonical fixture coverage and compatibility regexes for transitional `core/` references
  - in progress; canonical GitHub governance copies now exist under `definitions/providers/github/governance/` for `instruction-ownership.manifest.json` and `authoritative-source-map.json`
  - in progress; validator-backed canonical references now cover provider skill packs, codex orchestration prompts, and GitHub root governance assets against the shallow taxonomy without reintroducing generated-surface dependencies
  - in progress; routing golden tests, planning indexes, and provider-authored VS Code snippets now validate or reference the same canonical shallow paths consumed by the canonical routing catalog
  - in progress; canonical provider consumers now pass the validation stack after replacing remaining `.github/*` authored references in codex/claude skill packs, codex orchestration prompts, GitHub root command docs, and provider runtime settings with `definitions/*` paths
  - in progress; `validate-template-standards` and `validate-dotnet-standards` now prefer canonical `definitions/templates/*` and `definitions/providers/github/governance/template-standards.baseline.json`, while runtime test fixtures keep legacy template copies only as temporary compatibility
  - in progress; runtime catalog readers now prefer canonical governance mirrors under `definitions/providers/github/governance/*` for MCP runtime, provider-surface projection, and git-hook EOF settings, while falling back to `.github/governance/*` only when a temp repo has not been scaffolded canonically yet
  - in progress; shared validation path helpers now resolve governance defaults to `definitions/providers/github/governance/*` before `.github/governance/*`, so README/workspace/warning/release/security validators and `validate-all` no longer start from generated governance surfaces
  - in progress; core local-context and runtime-install-profile readers now prefer canonical governance catalogs under `definitions/providers/github/governance/*`, with explicit regression tests proving canonical preference and legacy fallback
  - in progress; `validate-instructions` now requires canonical governance assets under `definitions/providers/github/governance/*`, accepts either canonical or legacy JSON labels for known-contract checks, and prefers canonical governance documents when both mirrors exist
  - in progress; shared validation fixtures and CLI validation command fixtures now materialize authored governance JSON into both `definitions/providers/github/governance/*` and `.github/governance/*` so canonical-first validation can coexist with deferred generated-surface coverage
  - in progress; `validate-agent-orchestration` and `validate-agent-permissions` now resolve agent governance catalogs from `definitions/providers/github/governance/*`, and their orchestration fixtures/CLI command scaffolds materialize canonical plus legacy governance mirrors so the canonical contract is enforced before projected-surface cutover
  - in progress; `validate-all`, `validate-architecture-boundaries`, `validate-readme-standards`, `validate-template-standards`, and `validate-workspace-efficiency` now exercise canonical governance defaults in both Rust fixtures and CLI command scaffolds, with canonical template examples under `definitions/templates/*`
- Commit checkpoint:
  - `refactor(validation): prioritize canonical definitions in validate-instructions`

---

## Validation Checklist

- `cargo test -p nettoolskit-validation --quiet`
- `cargo test -p nettoolskit-validation planning_structure --quiet`
- `pwsh -File .\\scripts\\validation\\validate-instructions.ps1`
- `pwsh -File .\\scripts\\validation\\validate-planning-structure.ps1`
- `git diff --check`

---

## Risks And Mitigations

- Renaming instructions can break routing, prompts, and skills if path updates are incomplete.
- Changing root taxonomy affects not only instruction paths but also skill, prompt, hook, and provider projection paths.
- Shared/projected drift can reappear if the authority rule stays implicit.
- Over-consolidating backend/frontend files can remove useful stack-specific guidance.
- Mitigation: freeze the root taxonomy and shallow-depth rule first, update consumers in the same refactor, and keep instructions vs agents vs skills vs hooks explicit.

---

## Specialist And Closeout

- Recommended specialist: `docs-release-engineer`
- Tester: required
- Reviewer: required
- README update: required
- Suggested commit message style:
  - `refactor(instructions): reorganize instruction taxonomy`
  - `docs(planning): record instruction taxonomy refactor roadmap`