# Runtime Drift Runbook

## Scope

Use this runbook when runtime folders (`~/.github`, `~/.codex`) diverge from repository source-of-truth.

## Detect

```powershell
ntk runtime doctor --repo-root . --detailed
```

## Repair

1. Sync runtime using mirror mode:

```powershell
pwsh -File .\scripts\runtime\bootstrap.ps1 -Mirror
```

2. Re-check drift:

```powershell
ntk runtime doctor --repo-root . --detailed
```

3. Run healthcheck:

```powershell
ntk runtime healthcheck --repo-root . --runtime-profile all --validation-profile dev
```

## Cleanup

```powershell
pwsh -File .\scripts\runtime\clean-codex-runtime.ps1 -IncludeSessions -SessionRetentionDays 30 -LogRetentionDays 30 -Apply
```

## Notes

- Healthcheck runs in warning-only mode by default.
- Use strict enforcement only when explicit governance requires it.