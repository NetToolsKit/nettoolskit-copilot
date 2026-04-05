# Build Target Cleanup And Artifact Pruning Spec

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-04-06 01:55
- Objective: define the policy for keeping Cargo build output bounded and pruning stale artifacts safely on a Windows developer workstation.
- Normalized Request: plan a cleanup system for cargo targets and generated build state so the repository does not accumulate multi-gigabyte artifact directories.
- Active Branch: `docs/planning-gap-workstreams`
- Planning Path: `planning/completed/plan-build-target-cleanup-and-artifact-pruning.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`
- Current Slice: the workstream now exposes a native cleanup CLI, measured byte inventory, reclaimed-byte reporting, safe terminal artifact discovery, and repository guidance for scoped versus destructive cleanup; the design intent is materially complete.

---

## Problem Statement

The repository already redirects build output to `.build/target`, but that alone does not guarantee bounded growth. A cleanup policy needs to exist so the target directory stays usable and does not balloon into an operational hazard.

---

## Design Intent

- Keep build output out of the source tree.
- Prune stale artifacts safely without harming active iteration.
- Treat cleanup as a deterministic maintenance action, not a destructive surprise.

---

## Options Considered

1. Keep the current target redirection and do nothing else.
   - Rejected: it does not address uncontrolled growth.
2. Delete the whole target tree regularly.
   - Rejected: too destructive for normal development.
3. Define scoped cleanup, thresholds, and opt-in automation.
   - Preferred: safer and more maintainable.

---

## Proposed Boundaries

- `.cargo/config.toml` owns the target-dir redirection.
- `.gitignore` owns the non-versioned artifact exclusions.
- Scripts or Rust helpers own scoped cleanup actions.
- The repo operating model documents the policy and the operator expectations.

---

## Acceptance Criteria

- `.build/target` stays bounded by a documented policy.
- Cleanup can run safely on Windows.
- The policy distinguishes full cleanup from scoped cleanup.
- New build artifacts do not spill into source folders.
- Progress [2026-04-06 00:45]:
  - The repository now exposes `ntk runtime clean-build-artifacts` as the native cleanup entry point over the existing Rust maintenance boundary.
  - CLI coverage now proves dry-run and forced-removal behavior on Windows-friendly temp repositories.
- Progress [2026-04-06 01:35]:
  - The cleanup boundary now reports measured artifact-byte totals and reclaimed bytes so the operator can prove cleanup footprint before deletion.
  - Artifact discovery now treats managed roots as terminal cleanup boundaries, preventing nested double-counting inside `.build`, `.deployment`, `bin`, and `obj`.
  - Artifact discovery now treats managed roots as terminal cleanup boundaries, preventing nested double-counting inside `.build`, `.deployment`, `bin`, and `obj`.

---

## Planning Readiness

- The spec is planning-ready once the baseline footprint and prune thresholds are defined in the active plan.
- Implementation should happen in a separate branch because cleanup touches build and developer workflow behavior.