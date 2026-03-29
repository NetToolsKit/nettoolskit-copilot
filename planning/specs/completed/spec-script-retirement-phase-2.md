# Script Retirement Phase 2

Generated: 2026-03-28 21:43

## Objective

Retire the next safe pair of local PowerShell wrappers:

1. `scripts/tests/check-test-naming.ps1`
2. `scripts/tests/refactor_tests_to_aaa.ps1`

without losing repository guidance, validation behavior, or contract clarity.

## Context

The completed retirement audit proved that the local repository should not delete `scripts/` in bulk. It also proved that these two wrappers are different from the higher-risk blocked domains:

- both behaviors already exist natively in `crates/commands/validation`
- neither wrapper is retained by policy
- the only remaining live blockers are contract metadata and contract tests that still encode the old `.ps1` paths

`C:\Users\tguis\copilot-instructions` remains an upstream comparison source for instruction content, but not the local source of runtime truth. For this slice, the external repo matters only to confirm that removing the wrappers does not remove unique instruction intent.

## Current Slice Summary

- live local script inventory: `141`
- current removal target count: `2`
- expected live inventory after the slice: `141`
- current known blockers: none in live code/tests; the contract cleanup and wrapper deletion are complete in this phase
- current native Rust owners:
  - `crates/commands/validation/src/operational_hygiene/test_naming.rs`
  - `crates/commands/validation/src/operational_hygiene/refactor_tests_to_aaa.rs`
- next smallest blocked leaves after this phase:
  - `scripts/runtime/hooks/pre-tool-use.ps1`
  - `scripts/maintenance/trim-trailing-blank-lines.ps1`

## Design Decisions

1. Treat the Rust-native validation commands as the canonical runtime surfaces for test naming and AAA refactor behavior.
2. Remove the two wrappers only after the contract catalog stops presenting their `.ps1` paths as live local obligations.
3. Update the completed retirement bundle in the same slice so the repository history does not leave retired paths marked as still blocked.
4. Preserve `C:\Users\tguis\copilot-instructions` as a reference source only; do not sync or mirror the upstream PowerShell scripts back into the local repo.
5. Treat the deleted wrapper paths as retired local compatibility debt, not as missing instruction surfaces.

## Non-Goals

- deleting any higher-risk script in this same slice
- reopening the closed migration-wave planning bundle
- changing retained-wrapper policy for runtime hooks, git hooks, deploy wrappers, or runtime parity harness scripts

## Risks

- the validation contract table may currently serve as human-facing migration evidence, so removing entries must keep the remaining totals coherent
- archived retirement artifacts can become misleading if they are not updated together with the code change
- upstream `copilot-instructions` still contains these scripts, but that upstream presence is not a reason to keep local duplicates when Rust is canonical locally

## Alternatives Considered

### Alternative 1: Keep The Wrappers But Mark Them As Deprecated

Rejected. These wrappers are already redundant locally and the remaining blockers are metadata-only.

### Alternative 2: Delete The Wrappers Without Touching Completed Planning

Rejected. That would immediately make the completed retirement audit inaccurate.

### Alternative 3: Wait And Delete Them As Part Of A Larger `scripts/tests` Sweep

Rejected. The blocker surface is already minimal, and delaying would preserve dead wrappers for no technical benefit.

## Acceptance Criteria

1. The two `.ps1` files are removed from the live repository.
2. No non-archival local file still references either deleted path.
3. `crates/commands/validation` tests remain green.
4. The completed retirement matrix and supporting artifacts reflect the new inventory and retired count.
5. The next retirement slice is identified before this phase is closed.

## Planning Readiness

- implemented-validated

## Recommended Specialist Focus

- `dev-rust-engineer`
- `test-engineer`
- `docs-release-engineer`