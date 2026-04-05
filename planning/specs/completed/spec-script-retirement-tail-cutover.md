# Script Retirement Tail Cutover Spec

Generated: 2026-03-30 07:31

## Status

- LastUpdated: 2026-04-05 17:20
- Objective: record the final planning intent that governed the remaining PowerShell tail before the Phase 19-22 sweeps and retention audit closed the generic script-tail planning lane.
- Normalized Request: plan the last script-retirement slices so the repository can keep deleting `.ps1` safely without breaking operator workflows.
- Active Branch: `docs/planning-gap-workstreams` (planning closeout)
- Planning Path: `planning/completed/plan-script-retirement-tail-cutover.md`
- SDD Baseline: `planning/specs/active/spec-spec-driven-development-operating-model.md`

---

## Problem Statement

The repository has already retired a large part of the PowerShell estate. What remains is a narrower tail of compatibility, parity, and maintenance leaves that still need a decision: retire, keep intentionally, or move to a Rust-owned surface.

Closeout result:
- the Phase 19-22 domain sweeps and the post-Phase-22 retention audit now provide that decision evidence explicitly
- the remaining retained estate is no longer an uncategorized "tail"; it is a blocker-accounted implementation backlog

---

## Design Intent

- Keep only the wrappers that are still intentionally required.
- Delete wrappers only after a native Rust replacement and a parity check exist.
- Keep the safety matrix and parity ledger as the source of truth for the deletion sequence.

---

## Options Considered

1. Delete all remaining scripts immediately.
   - Rejected: too risky for operator entrypoints and parity coverage.
2. Keep all remaining scripts forever.
   - Rejected: defeats the retirement effort.
3. Retire the tail slice-by-slice and retain only explicit exceptions.
   - Preferred: safe and reviewable.

---

## Proposed Boundaries

- Runtime wrappers retire only when `ntk runtime` owns the behavior.
- Validation wrappers retire only when `ntk validation` owns the behavior.
- Maintenance/parity wrappers retire only when their Rust or test-harness replacements are stable.

---

## Acceptance Criteria

- The remaining `.ps1` estate is fully categorized.
- Every retained wrapper has an explicit reason.
- Every deleted wrapper has a Rust-owned replacement and tests.
- The safety matrix and parity ledger stay synchronized.

Closeout evidence:
- `planning/completed/plan-script-retirement-phase-19.md`
- `planning/completed/plan-script-retirement-phase-20-runtime-consumer-sweep.md`
- `planning/completed/plan-script-retirement-phase-21-security-governance-sweep.md`
- `planning/completed/plan-script-retirement-phase-22-orchestration-sweep.md`
- `planning/completed/script-retirement-safety-matrix.md`
- `planning/completed/rust-script-parity-ledger.md`

---

## Planning Readiness

- The spec is planning-ready once the remaining tail is inventoried and mapped into the active plan.
- Implementation should keep the same phase-by-phase deletion pattern already used for the earlier retirement waves.

Closeout:
- planning readiness has been consumed
- future work should open blocker-reduction implementation workstreams instead of reopening this generic tail-cutover umbrella