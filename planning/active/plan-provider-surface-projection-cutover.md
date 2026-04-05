# Provider Surface Projection Cutover Plan

Generated: 2026-04-05 12:20

## Status

- LastUpdated: 2026-04-05 12:20
- Objective: finish the generated/runtime projection cutover so `.github`, `.codex`, and `.claude` consume the canonical `definitions/*` tree without reopening authored-source drift.
- Normalized Request: close the remaining projection lane after the canonical instruction taxonomy migration by keeping `definitions/*` authoritative and treating generated provider/runtime folders as downstream outputs only.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-provider-surface-projection-cutover.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Dependencies:
  - `planning/completed/plan-instruction-taxonomy-and-path-refactor.md`
  - `planning/completed/plan-instruction-rules-board-and-surface-layout.md`
  - `planning/completed/plan-instruction-governance-and-super-agent-retention.md`
  - `definitions/providers/github/governance/provider-surface-projection.catalog.json`
  - `scripts/runtime/render-github-instruction-surfaces.ps1`
  - `crates/commands/runtime/src/sync/provider_surfaces.rs`

---

## Scope Summary

| ID | Slice | Target | Priority | Dependency |
|---|---|---|---|---|
| P1 | Projection inventory and drift freeze | `.github/.codex/.claude` authored vs generated paths | 🔴 Immediate | none |
| P2 | Renderer and catalog normalization | runtime renderers + provider projection catalog | 🔴 Immediate | P1 |
| P3 | Generated surface rebuild and validation | projected folders + validation warnings | 🟠 High | P2 |
| P4 | Compatibility mirror reduction | legacy path references and deferred fallback removal | 🟡 Medium | P3 |

---

## Ordered Tasks

### [2026-04-05 12:20] Task P1: Freeze Projection Drift Inventory

- Inventory the remaining generated-surface paths that still assume legacy authored roots or legacy lane names.
- Focus on:
  - `.github/`
  - `.codex/`
  - `.claude/`
  - VS Code mirrored runtime assets under `%USERPROFILE%\\.github`
- Record which paths are true provider-owned authored overlays versus generated outputs that must be regenerated only.
- Commit checkpoint:
  - `docs(planning): inventory remaining provider-surface cutover`

### [2026-04-05 12:20] Task P2: Normalize Renderer And Catalog Contracts

- Finish canonical-first projection rules in:
  - `definitions/providers/github/governance/provider-surface-projection.catalog.json`
  - `crates/commands/runtime/src/sync/provider_surfaces.rs`
  - `scripts/runtime/render-github-instruction-surfaces.ps1`
  - other provider/runtime render helpers when they still assume legacy roots
- Keep `definitions/shared/prompts/poml/` only where prompt assets are still intentionally shared.
- Commit checkpoint:
  - `refactor(runtime): normalize provider projection contracts`

### [2026-04-05 12:20] Task P3: Rebuild Generated Surfaces And Shrink Validation Warnings

- Re-render generated/runtime surfaces from canonical roots.
- Target the warnings that still mention:
  - missing generated `.github/instructions/core/*`
  - VS Code references to removed legacy lane paths
  - other generated/runtime assets that should now resolve through canonical shallow taxonomy
- Validate with:
  - `ntk validation instructions --warning-only true`
  - `ntk validation planning-structure --warning-only true`
  - any provider/runtime smoke harness needed for the touched renderer path
- Commit checkpoint:
  - `refactor(runtime): rebuild generated provider surfaces from canonical roots`

### [2026-04-05 12:20] Task P4: Reduce Compatibility Mirrors Safely

- After projection cutover is stable, reduce compatibility-only references that no longer have active consumers.
- Do not delete compatibility surfaces until validator, renderer, and runtime smoke evidence proves the cutover is stable.
- Update README/changelog/planning if the compatibility contract changes materially.
- Commit checkpoint:
  - `refactor(definitions): reduce projection compatibility mirrors`

---

## Validation Checklist

- `cargo run -q -p nettoolskit-cli -- validation instructions --warning-only true`
- `cargo run -q -p nettoolskit-cli -- validation planning-structure --warning-only true`
- renderer-specific smoke tests when runtime scripts or projection code changes
- `git diff --check`

---

## Risks And Mitigations

- Generated folders can drift if humans keep editing them directly.
  - Mitigation: keep canonical authorship in `definitions/*` and regenerate outputs.
- Projection cutover can break VS Code/runtime references that still assume removed legacy paths.
  - Mitigation: inventory and normalize runtime consumers before deleting compatibility references.
- Compatibility mirrors can be removed too early.
  - Mitigation: require validator + smoke evidence before reducing fallback paths.

---

## Specialist And Closeout

- Recommended specialist: `docs-release-engineer`
- Tester: required
- Reviewer: required
- Release closeout: required
- README update: required when runtime/provider operator guidance changes
- Changelog: required
- Suggested commit message style:
  - `docs(planning): open provider surface projection cutover`