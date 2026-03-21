# Plan: Shared Helper Loader Bootstrap

## Objective
- Centralize the repeated helper dot-sourcing bootstrap so scripts load shared common helpers through one reusable loader instead of duplicating path-resolution blocks.

## Scope
- `scripts/common/`
- `scripts/runtime/*.ps1`
- `scripts/security/*.ps1`
- `scripts/orchestration/**/*.ps1`
- `scripts/validation/*.ps1`
- `scripts/tests/runtime/*.ps1`
- targeted docs, changelog, and checksum manifest

## Spec
- `planning/specs/active/spec-shared-helper-loader-bootstrap.md`

## Tasks
1. Add a shared helper loader.
   - Target paths: `scripts/common/`
   - Checkpoint: one loader resolves common helper paths for repo and mirrored runtime layouts.
2. Replace duplicated import blocks.
   - Target paths: runtime, validation, orchestration, security, and runtime test scripts.
   - Checkpoint: scripts call the shared loader instead of inlining helper path detection.
3. Update docs and closeout artifacts.
   - Target paths: `scripts/README.md`, `CHANGELOG.md`, checksum manifest.
4. Validate the refactor.
   - Commands:
     - `pwsh -NoLogo -NoProfile -File scripts/validation/validate-powershell-standards.ps1 -RepoRoot . -SkipScriptAnalyzer`
     - `pwsh -NoLogo -NoProfile -File scripts/tests/runtime/runtime-scripts.tests.ps1 -RepoRoot .`
     - `pwsh -NoLogo -NoProfile -File scripts/validation/validate-runtime-script-tests.ps1 -RepoRoot . -WarningOnly:$false`
     - `pwsh -NoLogo -NoProfile -File scripts/validation/validate-shared-script-checksums.ps1 -RepoRoot . -WarningOnly:$false`
     - `pwsh -NoLogo -NoProfile -File scripts/validation/validate-all.ps1 -RepoRoot . -ValidationProfile dev`

## Risks
- Loader search order regressions in mirrored runtime directories.
- Partial replacement leaving mixed bootstrap patterns across scripts.

## Closeout Expectations
- Update changelog.
- Provide commit message.