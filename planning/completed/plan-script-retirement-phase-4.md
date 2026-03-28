# Plan: Script Retirement Phase 4

Generated: 2026-03-28 19:18

## Status

- LastUpdated: 2026-03-28 19:18
- Objective: retire the next runtime continuity and workspace-template PowerShell leaves by replacing their live consumers with native `ntk runtime` entrypoints and then deleting the local wrappers safely.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, keeping planning active and updated while each stable phase is committed separately.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-4.md`
- Inputs:
  - `C:\Users\tguis\copilot-instructions`
  - `planning/completed/plan-script-retirement-phase-3.md`
  - `planning/specs/completed/spec-script-retirement-phase-3.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- Current Slice Snapshot:
  - local `scripts/**/*.ps1` inventory is `135`
  - `scripts/runtime/update-local-context-index.ps1` and `scripts/runtime/query-local-context-index.ps1` are retired locally; continuity consumers now execute through `ntk runtime update-local-context-index` and `ntk runtime query-local-context-index`
  - `scripts/runtime/export-planning-summary.ps1` is retired locally; planning handoff flows now execute through `ntk runtime export-planning-summary`
  - `scripts/runtime/apply-vscode-templates.ps1` is retired locally; runtime housekeeping, self-heal, and VS Code guidance now execute through `ntk runtime apply-vscode-templates`
  - upstream `C:\Users\tguis\copilot-instructions` still carries the four PowerShell leaves, but local canonical behavior now lives in Rust-owned runtime entrypoints plus explicit retained compatibility wrappers only where policy still requires them

## Scope Summary

This phase targeted the smallest next runtime-owned leaves whose canonical behavior already exists in Rust:

1. `scripts/runtime/update-local-context-index.ps1`
2. `scripts/runtime/query-local-context-index.ps1`
3. `scripts/runtime/export-planning-summary.ps1`
4. `scripts/runtime/apply-vscode-templates.ps1`

The phase is complete only if:

- a native `ntk runtime` entrypoint exists for each in-scope behavior
- live runtime and authored consumers stop hardcoding those exact `.ps1` paths
- PowerShell parity tests stop validating the legacy script path as the canonical executable contract
- the four leaves can be deleted safely or explicitly reclassified with concrete retained-wrapper rationale

## Ordered Tasks

### Task 1: Freeze The Consumer Surface

Status: `[x]` Completed

- Record the exact live consumers that still hardcode the four target paths.
- Separate blockers into:
  - runtime housekeeping and self-heal flows
  - authored instruction and README surfaces
  - validation inventory and policy references
  - runtime parity tests and bootstrap assumptions
- Target paths:
  - `scripts/runtime/**`
  - `scripts/tests/runtime/**`
  - `scripts/validation/**`
  - `definitions/**`
  - `.github/**`
  - `crates/commands/runtime/tests/**`
- Commands:
  - `rg -n "update-local-context-index\\.ps1|query-local-context-index\\.ps1|export-planning-summary\\.ps1|apply-vscode-templates\\.ps1" . -g "!planning/completed/**" -g "!planning/specs/completed/**"`
- Checkpoints:
  - every blocker is concrete and path-specific
  - no remaining blocker is justified only by historical planning text
- ✓ [2026-03-28 19:18] Revalidated that the only live blockers were runtime housekeeping, self-heal, authored docs, validation inventory, and parity tests that still encoded the exact `.ps1` paths.

### Task 2: Extend The Native Runtime CLI Surface

Status: `[x]` Completed

- Add executable `ntk runtime` entrypoints for:
  - `update-local-context-index`
  - `query-local-context-index`
  - `export-planning-summary`
  - `apply-vscode-templates`
- Keep the CLI contract narrow and reuse the existing runtime crate request/result types.
- Target paths:
  - `crates/cli/src/**`
  - `crates/cli/tests/**`
  - `crates/commands/runtime/src/continuity/**`
  - `crates/commands/runtime/src/sync/**`
- Checkpoints:
  - each in-scope behavior is executable through `ntk runtime ...`
  - CLI tests cover success-path invocation for the new commands
- ✓ [2026-03-28 19:18] Added `ntk runtime update-local-context-index`, `query-local-context-index`, `export-planning-summary`, and `apply-vscode-templates` with CLI coverage in `crates/cli/tests/runtime_commands_tests.rs`.

### Task 3: Repoint Consumers And Tests

Status: `[x]` Completed

- Update live consumers to stop hardcoding the in-scope `.ps1` paths.
- Include:
  - `scripts/runtime/invoke-super-agent-housekeeping.ps1`
  - `scripts/runtime/self-heal.ps1`
  - authored docs and instruction surfaces under `definitions/**` and `.github/**`
  - runtime PowerShell parity tests and validation inventory
- Target paths:
  - `scripts/runtime/**`
  - `scripts/tests/runtime/**`
  - `scripts/validation/**`
  - `definitions/**`
  - `.github/**`
- Commands:
  - targeted `cargo test`
  - targeted PowerShell runtime tests for continuity and template flows
- Checkpoints:
  - no live consumer still requires the old `.ps1` path
  - tests assert the native executable contract rather than the deleted leaf file
- ✓ [2026-03-28 19:18] Repointed runtime housekeeping, self-heal, authored instruction surfaces, VS Code and Claude runtime guidance, validation inventory, PowerShell parity tests, and Rust fixtures to the native `ntk runtime` boundary.

### Task 4: Retire Or Reclassify The Leaves

Status: `[x]` Completed

- Delete the leaves if all live consumers are cleared.
- If deletion is still blocked after consumer refactoring, reclassify the leaf explicitly as `retain wrapper intentionally` with concrete rationale.
- Update:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- Checkpoints:
  - the matrix reflects the executed state of each in-scope leaf
  - no ambiguous blocked state remains for any of the four targets
- ✓ [2026-03-28 19:18] Deleted `scripts/runtime/update-local-context-index.ps1`, `scripts/runtime/query-local-context-index.ps1`, `scripts/runtime/export-planning-summary.ps1`, and `scripts/runtime/apply-vscode-templates.ps1` after live consumers, docs, validation, and parity tests stopped requiring the local leaf paths.

### Task 5: Validate And Prepare The Next Domain

Status: `[x]` Completed

- Run the relevant Rust, PowerShell, validation, and security checks.
- Decide whether the next queue should move to:
  - runtime diagnostics (`doctor`, `healthcheck`, `self-heal`)
  - validation shell retirement
  - common helper retirement
- Checkpoints:
  - the phase ends with an explicit next queue
  - the repository remains stable and clean
- ✓ [2026-03-28 19:18] Verified the phase with targeted CLI/runtime tests, PowerShell runtime parity tests, validation checks, vulnerability audit, and `git diff --check`. The next queue moves to the next small runtime or validation consumer-cutover slice rather than reopening already retired continuity/template leaves.

## Validation Checklist

- `rg -n "update-local-context-index\\.ps1|query-local-context-index\\.ps1|export-planning-summary\\.ps1|apply-vscode-templates\\.ps1" . -g "!planning/completed/**" -g "!planning/specs/completed/**"`
- `cargo test -p nettoolskit-cli --quiet`
- `cargo test -p nettoolskit-runtime --quiet`
- targeted PowerShell runtime tests for continuity/template flows
- `pwsh -File .\\scripts\\validation\\validate-instructions.ps1`
- `pwsh -File .\\scripts\\security\\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- `git diff --check`

## Recommended Specialist

- Primary: `dev-rust-engineer`
- Support:
  - `test-engineer`
  - `docs-release-engineer`

## Risks And Fallback

- Risk: authored docs and provider guidance may still intentionally document the PowerShell wrappers instead of the native runtime binary.
- Risk: `export-planning-summary` and local-context commands are used as safe continuity surfaces, so CLI output changes could break operator expectations or tests.
- Risk: `apply-vscode-templates` may still be referenced as a standalone shell action even if `self-heal` already owns the underlying behavior natively.
- Fallback: if any leaf remains blocked after consumer cutover, reclassify it as an intentional retained wrapper and move the unresolved behavior to the next domain plan instead of forcing deletion.

## Outcome

- `scripts/runtime/update-local-context-index.ps1`: retired locally
- `scripts/runtime/query-local-context-index.ps1`: retired locally
- `scripts/runtime/export-planning-summary.ps1`: retired locally
- `scripts/runtime/apply-vscode-templates.ps1`: retired locally
- The live local `scripts/**/*.ps1` estate is now reduced from `139` to `135`.
- Remaining runtime retirement work now shifts from continuity/template leaves to the next consumer-coupled runtime or validation slices.

## Closeout Expectations

- Keep commits in English and slice-oriented.
- Update the retirement matrix and parity ledger in the same phase as any deletion or reclassification.
- Do not claim a leaf is deletable until both the Rust and PowerShell parity checks agree on the new contract.