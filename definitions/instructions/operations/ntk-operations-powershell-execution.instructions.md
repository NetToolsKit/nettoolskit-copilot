---
applyTo: "**/*.ps1"
priority: high
---

# PowerShell Execution

Use this instruction for executing, invoking, or operationally maintaining
existing PowerShell scripts. Use
`ntk-operations-powershell-script-creation.instructions.md` for authoring or
refactoring script files.

## Repository Root Detection

- Never assume the caller is already at the repository root.
- Detect and validate the repository root before any repo-relative operation.
- Prefer a shared helper such as `Set-CorrectWorkingDirectory` or equivalent deterministic logic.
- Use `Test-Path` and `Resolve-Path` before changing directories.
- Keep repository-root discovery bounded and explicit.

## Path Safety

- Build paths with `Join-Path`.
- Use `-LiteralPath` for filesystem operations when paths may contain special characters.
- Avoid hardcoded absolute paths.
- Validate target paths before mutation or navigation.
- Prefer repo-relative paths once the root is confirmed.

## Runtime Error Handling

- Wrap critical operations in `try/catch`.
- Use meaningful failure messages and non-zero exit codes for execution failure.
- Use `Write-Error` for hard failures and `Write-Warning` for recoverable issues.
- Keep success summaries explicit and short.
- Do not swallow exceptions silently.

## Mutation Safety

- Prefer safe default behavior.
- Require explicit switches for destructive or bulk-changing operations.
- Add `DryRun` or equivalent preview modes when a script mutates many files or resources.
- Use `-Force` only when the caller opted into it or the workflow explicitly requires it.
- Validate targets before delete/move/replace behavior.

## Logging And Diagnostics

- Keep log prefixes stable and operator-readable.
- Support `-Verbose` or an equivalent diagnostic mode.
- Do not print secrets, tokens, or full sensitive configuration values.
- Keep output deterministic so runtime wrappers and validation flows can parse it reliably.

## Cross-Platform And Compatibility

- Consider Windows PowerShell vs PowerShell 7+ behavior where relevant.
- Normalize path handling and case-sensitivity assumptions.
- Avoid platform-specific shell shortcuts unless the script is intentionally platform-bound.
- Keep execution guidance aligned with repository runtime wrappers and native `ntk` commands.

## Performance

- Avoid broad `Get-ChildItem -Recurse` scans without early filters.
- Cache expensive path or config lookups when repeated.
- Limit search depth or scope when the repo layout is known.
- Use `-ErrorAction SilentlyContinue` only when the failure mode is expected and safe.

## Security

- Sanitize user-controlled inputs before use.
- Avoid `Invoke-Expression` or equivalent dynamic execution unless the workflow truly requires it.
- Run with the least privilege required for the task.
- Validate file and directory targets before mutation.
- Keep secrets and sensitive material out of logs and summaries.