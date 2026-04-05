---
applyTo: "scripts/**/*.ps1"
priority: high
---

# PowerShell Script Creation

Use this instruction for creating or refactoring PowerShell scripts under
`scripts/`. Use `ntk-runtime-powershell-execution.instructions.md` for runtime
execution behavior and path-safety expectations.

## Purpose

Standardize repository PowerShell scripts so they keep the same structure,
safety, and operator-facing behavior across runtime, validation, maintenance,
and orchestration surfaces.

## Required Starting Point

- Start from `definitions/templates/codegen/powershell-script-template.ps1` for every new script.
- Do not assemble a full script from inline examples alone.
- For AI-assisted generation, prefer `.github/prompts/create-powershell-script.prompt.md` first.

## Required Script Shape

- Start with comment-based help.
- Document `SYNOPSIS`, `DESCRIPTION`, `PARAMETER`, `EXAMPLE`, and `NOTES`.
- Keep the `param` block at the top of the executable body.
- Set `ErrorActionPreference = 'Stop'` unless a narrower operational reason exists.
- Organize code into:
  - Helpers
  - Main execution
  - Summary/exit

## Function Authoring Rules

- Use approved PowerShell verbs and descriptive names.
- Add a short comment above every function declaration when the purpose is not obvious from the signature.
- Keep functions focused and single-purpose.
- Avoid duplicating path resolution logic across helpers.
- Prefer shared helper patterns over one-off inline path and logging code.

## Parameter And Validation Rules

- Accept explicit parameters instead of hidden global state where practical.
- Validate required paths before processing.
- Use `Test-Path -LiteralPath` for filesystem checks.
- Use explicit switch parameters for optional behaviors.
- Return `0` for success and `1` for validation or runtime failure unless a narrower contract requires a different code.

## Root Detection For Repo Scripts

- When script behavior depends on repository layout, accept an optional `RepoRoot` parameter.
- Resolve and validate the requested root deterministically.
- Do not assume the caller launched the script from the correct directory.
- Reuse a shared helper such as `Set-CorrectWorkingDirectory` when possible.

## Logging And Summaries

- Use stable, structured message prefixes for warnings and failures.
- Keep success summaries at the end with counts or final status when applicable.
- Support verbose diagnostics through a switch and helper logger.
- Never print secrets or sensitive configuration values.

## Mutation Safety

- Prefer safe default modes.
- Add `DryRun` or equivalent preview support for destructive or bulk-rewrite scripts.
- Use `Force` only when the caller explicitly requests it.
- Wrap critical mutations in `try/catch`.
- Keep scripts idempotent when feasible.

## File And Encoding Rules

- Use UTF-8 for file writes unless the target format requires otherwise.
- Keep PowerShell files UTF-8 without BOM unless a file-specific format requires BOM.
- Respect the repository EOF policy from `.editorconfig`.
- Do not leave trailing blank lines at EOF.