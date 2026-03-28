# Plan: Script Retirement Phase 3

Generated: 2026-03-28 18:41

## Status

- LastUpdated: 2026-03-28 18:41
- Objective: retire the next two locally blocked PowerShell leaves by replacing their live consumers with native `ntk runtime` entrypoints and then deleting the local wrappers safely.
- Normalized Request: continue deleting local PowerShell scripts in favor of Rust-native behavior, but only after the live consumer chain stops hardcoding the legacy `.ps1` entrypoints.
- Active Branch: `feature/instruction-runtime-retirement-audit`
- Spec Path: `planning/specs/completed/spec-script-retirement-phase-3.md`
- Inputs:
  - `C:\Users\tguis\copilot-instructions`
  - `planning/completed/plan-script-retirement-phase-2.md`
  - `planning/specs/completed/spec-script-retirement-phase-2.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- Current Slice Snapshot:
  - local `scripts/**/*.ps1` inventory is `139`
  - `scripts/runtime/hooks/pre-tool-use.ps1` is retired locally; `.github/hooks/scripts/pre-tool-use.ps1` and the provider-authored wrapper now dispatch to `ntk runtime pre-tool-use`
  - `scripts/maintenance/trim-trailing-blank-lines.ps1` is retired locally; git hooks, `git trim-eof`, and bootstrap-projected runtimes now dispatch to `ntk runtime trim-trailing-blank-lines`
  - upstream `C:\Users\tguis\copilot-instructions` still carries both scripts, but local canonical behavior now lives in Rust-owned runtime surfaces plus explicit compatibility wrappers

## Scope Summary

This phase does not start from another low-coupling delete-now leaf. It addresses the smallest remaining leaves whose blockers are concrete and already known:

1. `scripts/runtime/hooks/pre-tool-use.ps1`
2. `scripts/maintenance/trim-trailing-blank-lines.ps1`

The phase is complete only if:

- live runtime and validation consumers stop depending on those exact `.ps1` paths
- the provider/runtime hook chain points to a Rust-native boundary or an explicit retained compatibility surface
- git hook and alias flows stop requiring `trim-trailing-blank-lines.ps1` as their canonical implementation path
- the two leaves can either be deleted safely or downgraded explicitly to retained-wrapper policy with clear rationale

## Ordered Tasks

### Task 1: Freeze The Blocker Surface

Status: `[x]` Completed

- Record the exact consumer files that still hardcode the two target paths.
- Separate blockers into:
  - runtime hook bootstrap
  - validation contracts/checks
  - PowerShell runtime smoke tests
  - Rust hook setup/alias code
- Target paths:
  - `.github/hooks/**`
  - `definitions/providers/github/hooks/**`
  - `scripts/validation/**`
  - `scripts/git-hooks/**`
  - `scripts/tests/runtime/**`
  - `crates/commands/validation/**`
  - `crates/commands/runtime/**`
- Commands:
  - `rg -n "pre-tool-use\\.ps1|trim-trailing-blank-lines\\.ps1" . -g "!planning/completed/**" -g "!planning/specs/completed/**"`
- Checkpoints:
  - no blocker remains generic or inferred
  - each retained reference is tied to a concrete migration action
- ✓ [2026-03-28 18:41] Revalidated that the remaining blockers were limited to wrapper bootstrap, git-hook and alias launch paths, and parity tests.

### Task 2: Design The Native Hook And EOF Entry Contract

Status: `[x]` Completed

- Decide the canonical Rust-facing invocation path for:
  - `PreToolUse`
  - EOF trim / git alias / pre-commit hygiene
- Preferred outcome:
  - provider hook wrappers remain tiny launch surfaces only
  - behavior lives in Rust and no longer depends on repo-local PowerShell leaves
- Target paths:
  - `planning/specs/active/spec-script-retirement-phase-3.md`
  - `.github/hooks/scripts/pre-tool-use.ps1`
  - `definitions/providers/github/hooks/scripts/pre-tool-use.ps1`
  - `crates/commands/runtime/src/hooks/**`
- Checkpoints:
  - one canonical invocation path exists for each behavior
  - compatibility wrappers and canonical implementations are clearly separated
- ✓ [2026-03-28 18:41] Added executable `ntk runtime` entrypoints for `pre-tool-use`, `trim-trailing-blank-lines`, `pre-commit-eof-hygiene`, `setup-git-hooks`, and `setup-global-git-aliases`.

### Task 3: Repoint Consumers And Tests

Status: `[x]` Completed

- Update the live consumers to stop hardcoding:
  - `scripts/runtime/hooks/pre-tool-use.ps1`
  - `scripts/maintenance/trim-trailing-blank-lines.ps1`
- Include:
  - validation scripts or Rust validation checks
  - runtime hook bootstrap/projection assets
  - Rust hook setup code and tests
  - runtime PowerShell parity tests that still assert the old paths
- Target paths:
  - `.github/hooks/**`
  - `definitions/providers/github/hooks/**`
  - `scripts/validation/**`
  - `scripts/git-hooks/**`
  - `scripts/tests/runtime/**`
  - `crates/commands/runtime/**`
  - `crates/commands/validation/**`
- Commands:
  - targeted `cargo test`
  - targeted PowerShell runtime tests when still relevant
- Checkpoints:
  - no live consumer still requires the old `.ps1` path
  - tests assert the new contract instead of the deleted leaves
- ✓ [2026-03-28 18:41] Repointed GitHub hook wrappers, git-hook setup, global aliases, bootstrap runtime projection, and runtime parity tests to the native `ntk runtime` boundary.

### Task 4: Retire Or Explicitly Reclassify The Leaves

Status: `[x]` Completed

- Delete the leaves if all live consumers are cleared.
- If deletion is still blocked after the consumer refactor, reclassify them explicitly as retained wrappers with concrete rationale.
- Update:
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
- Checkpoints:
  - the matrix reflects reality
  - no ambiguous “blocked” state remains for these two leaves
- ✓ [2026-03-28 18:41] Deleted `scripts/runtime/hooks/pre-tool-use.ps1` and `scripts/maintenance/trim-trailing-blank-lines.ps1` after live consumers, tests, and validation inventory stopped requiring the local leaf paths.

### Task 5: Validate And Queue The Next Domain

Status: `[x]` Completed

- Run the relevant Rust and runtime validations.
- Decide the next domain-level sweep after these hook/EOF leaves.
- Checkpoints:
  - the phase ends with an explicit next queue
  - the repo remains stable and clean
- ✓ [2026-03-28 18:41] Verified the phase with targeted Rust/runtime tests, vulnerability audit, `validate-instructions`, and `git diff --check`. The next queue moves from leaf retirement to domain-level proof for `scripts/common/*.ps1`, `scripts/runtime/*.ps1` excluding retained hooks, and `scripts/validation/*.ps1`.

## Validation Checklist

- `rg -n "scripts/runtime/hooks/pre-tool-use\\.ps1|scripts/maintenance/trim-trailing-blank-lines\\.ps1" . -g "!planning/completed/**" -g "!planning/specs/completed/**"`
- targeted `cargo test -p nettoolskit-runtime --quiet`
- targeted `cargo test -p nettoolskit-validation --quiet`
- `pwsh -File .\\scripts\\security\\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- `git diff --check`

## Recommended Specialist

- Primary: `dev-rust-engineer`
- Support:
  - `test-engineer`
  - `docs-release-engineer`

## Risks And Fallback

- Risk: the hook bootstrap chain may still require a PowerShell launch surface even after Rust owns the behavior.
- Risk: git alias and pre-commit flows may depend on a script path that is mirrored into shared runtime directories rather than executed through a Rust CLI boundary.
- Fallback: if either leaf still cannot be deleted cleanly after consumer refactoring, reclassify it from `retain until consumer migration completes` to `retain wrapper intentionally` instead of forcing deletion.

## Outcome

- `scripts/runtime/hooks/pre-tool-use.ps1`: retired locally
- `scripts/maintenance/trim-trailing-blank-lines.ps1`: retired locally
- Remaining retained wrappers are now explicit policy exceptions or higher-level compatibility surfaces, not unresolved leaf blockers.

## Closeout Expectations

- Keep commits in English and slice-oriented.
- Update the retirement matrix and parity ledger in the same phase as any deletion or reclassification.
- Do not claim the leaf is deletable until both Rust and PowerShell parity tests agree on the new contract.