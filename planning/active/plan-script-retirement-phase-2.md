# Plan: Script Retirement Phase 2

Generated: 2026-03-28 21:43

## Status

- LastUpdated: 2026-03-28 17:18
- Objective: retire the next safe local PowerShell slice by deleting `scripts/tests/check-test-naming.ps1` and `scripts/tests/refactor_tests_to_aaa.ps1` after migrating the remaining local contract references to the native Rust validation surfaces.
- Normalized Request: keep deleting local `scripts/**/*.ps1` where Rust already owns behavior, use `C:\Users\tguis\copilot-instructions` only as an upstream instruction/reference source, and preserve repository guidance while shrinking the local PowerShell estate.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/active/spec-script-retirement-phase-2.md`
- Inputs:
  - `C:\Users\tguis\copilot-instructions`
  - `planning/completed/plan-instruction-parity-and-script-retirement.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/completed/rust-script-transcription-ownership-matrix.md`
- Current Slice Snapshot:
  - local `scripts/**/*.ps1` inventory is `141`
  - Rust-native ownership already exists in `crates/commands/validation/src/operational_hygiene/test_naming.rs`
  - Rust-native ownership already exists in `crates/commands/validation/src/operational_hygiene/refactor_tests_to_aaa.rs`
  - the live code/test blockers for the two target wrappers are cleared
  - no non-archival local `.ps1` path reference remains after same-slice contract cleanup

## Scope Summary

This phase removes two narrow wrappers that are already parity-proven in Rust:

1. `scripts/tests/check-test-naming.ps1`
2. `scripts/tests/refactor_tests_to_aaa.ps1`

The slice is complete only if:

- the legacy script paths stop being encoded in local validation contracts
- the two PowerShell files are deleted
- the retirement matrix and parity artifacts reflect the new inventory
- no local references to those deleted `.ps1` paths remain outside archived history

## Ordered Tasks

### Task 1: Rebaseline The Live Removal Slice

Status: `[x]` Completed

- Confirm that the only live blockers for the two scripts are:
  - `crates/commands/validation/src/contracts.rs`
  - `crates/commands/validation/tests/contracts_tests.rs`
- Reconfirm that no operator/runtime instruction from `C:\Users\tguis\copilot-instructions` needs to remain local once the wrappers are gone.
- Target paths:
  - `crates/commands/validation/src/contracts.rs`
  - `crates/commands/validation/tests/contracts_tests.rs`
  - `scripts/tests/`
  - `planning/completed/script-retirement-safety-matrix.md`
- Commands:
  - `rg -n "check-test-naming\.ps1|refactor_tests_to_aaa\.ps1" . -g "!planning/completed/**" -g "!planning/specs/completed/**"`
  - targeted upstream comparison against `C:\Users\tguis\copilot-instructions`
- Checkpoints:
  - the blocker list is still narrow and unchanged
  - no additional local contract unexpectedly depends on the two wrapper paths
- ✓ [2026-03-28 17:18] Reconfirmed that the only live blockers were the validation surface contract catalog and its tests.
- ✓ [2026-03-28 17:18] Reconfirmed against `C:\Users\tguis\copilot-instructions` that the two wrappers are implementation artifacts, not authoritative instruction sources.

### Task 2: Remove Legacy Contract References

Status: `[x]` Completed

- Narrow the validation surface contract catalog so it stops locking these two individual `.ps1` leaves as active local runtime obligations.
- Keep the Rust-native validation ownership explicit without preserving deleted wrapper paths in the live contract table.
- Target paths:
  - `crates/commands/validation/src/contracts.rs`
  - `crates/commands/validation/tests/contracts_tests.rs`
- Commands:
  - `cargo test -p nettoolskit-validation contracts_tests --quiet`
- Checkpoints:
  - contract totals still match the live validation-owned legacy inventory
  - no deleted path remains asserted in the live contract tests
- Commit checkpoint:
  - `refactor(validation): retire legacy test automation wrapper contracts`
- ✓ [2026-03-28 17:18] Removed the two legacy validation contract entries, deleted the `TestAutomationCommands` category from the live surface catalog, and rebaselined the locked total from `43` to `41`.

### Task 3: Delete The Two PowerShell Wrappers

Status: `[x]` Completed

- Delete:
  - `scripts/tests/check-test-naming.ps1`
  - `scripts/tests/refactor_tests_to_aaa.ps1`
- Confirm that the local repository relies on the Rust-native commands instead.
- Target paths:
  - `scripts/tests/check-test-naming.ps1`
  - `scripts/tests/refactor_tests_to_aaa.ps1`
  - any doc/runtime surface that still names them
- Commands:
  - `rg -n "check-test-naming\.ps1|refactor_tests_to_aaa\.ps1" .`
  - `git diff --check`
- Checkpoints:
  - the wrappers are fully removed from the live repository
  - no non-archival local path reference remains
- Commit checkpoint:
  - `refactor(validation): delete rust-covered test automation wrappers`
- ✓ [2026-03-28 17:18] Deleted `scripts/tests/check-test-naming.ps1` and `scripts/tests/refactor_tests_to_aaa.ps1`.
- ✓ [2026-03-28 17:18] Verified that only archived history plus this active phase still mention the deleted paths.

### Task 4: Update Retirement And Parity Artifacts

Status: `[x]` Completed

- Update the completed retirement matrix and supporting completed planning artifacts to reflect:
  - live inventory `143 -> 141`
  - retired slice `4 -> 6`
  - blocked tail reduction `110 -> 108`
- Record that the blockers for these two leaves are fully cleared and that they are no longer part of the live backlog.
- Target paths:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `planning/completed/rust-script-transcription-ownership-matrix.md`
  - `planning/completed/plan-instruction-parity-and-script-retirement.md`
- Commands:
  - `git diff --check`
- Checkpoints:
  - completed artifacts tell the truth about the live repository
  - no retired script is still presented as blocked in the completed audit bundle
- Commit checkpoint:
  - `docs(planning): record test automation wrapper retirement`
- ✓ [2026-03-28 17:18] Rebaselined the completed retirement matrix to `141` live scripts with `6` retired in this workstream and `108` still blocked pending consumer migration.
- ✓ [2026-03-28 17:18] Updated the completed audit bundle so the retired test automation wrappers are no longer presented as active blockers.

### Task 5: Validate And Decide The Next Slice

Status: `[x]` Completed

- Run focused validation for the validation crate and local retirement references.
- Re-rank the next deletion slice using the updated backlog state.
- Target paths:
  - `crates/commands/validation/**`
  - `planning/active/plan-script-retirement-phase-2.md`
  - `planning/specs/active/spec-script-retirement-phase-2.md`
- Commands:
  - `cargo test -p nettoolskit-validation --quiet`
  - `git diff --check`
- Checkpoints:
  - this phase is stable and ready to archive or continue
  - the next deletion slice is explicit instead of inferred
- ✓ [2026-03-28 17:18] Validation is green for the targeted contract slice and the broader validation crate.
- ✓ [2026-03-28 17:18] No new remove-now leaf is promoted yet; the next consumer sweep is now explicitly narrowed to `scripts/runtime/hooks/pre-tool-use.ps1` and `scripts/maintenance/trim-trailing-blank-lines.ps1`.

## Validation Checklist

- `rg -n "check-test-naming\.ps1|refactor_tests_to_aaa\.ps1" . -g "!planning/completed/**" -g "!planning/specs/completed/**"`
- `cargo test -p nettoolskit-validation contracts_tests --quiet`
- `cargo test -p nettoolskit-validation --quiet`
- `git diff --check`

## Recommended Specialist

- Primary: `dev-rust-engineer`
- Support:
  - `test-engineer`
  - `docs-release-engineer`

## Risks And Fallback

- Risk: deleting the wrappers without updating the validation contract catalog will leave the repository claiming live ownership of paths that no longer exist.
- Risk: archived planning may drift from the live repository if the retirement artifacts are not updated in the same slice.
- Fallback: if additional non-archival references appear during execution, stop deletion and downgrade this slice back to `retain until consumer migration completes`.

## Closeout Expectations

- Keep commits in English and split by stable slice.
- Update the retirement/planning bundle in the same phase as the deletion.
- Archive the active plan/spec only if the phase is fully validated and the next slice is either started or explicitly queued.