# GitHub Provider Definitions

This tree is the authoritative source for repository-owned GitHub/Copilot
instruction, template, and hook surfaces that are rendered into `.github/`.

## Authoritative Coverage

- `root/` -> managed root files rendered into `.github/`
- `agents/` -> rendered into `.github/agents/`
- `chatmodes/` -> rendered into `.github/chatmodes/`
- `instructions/` -> rendered into `.github/instructions/`
- `ISSUE_TEMPLATE/` -> rendered into `.github/ISSUE_TEMPLATE/`
- `prompts/` -> rendered into `.github/prompts/`
- `hooks/` -> rendered into `.github/hooks/`
- `templates/` -> rendered into `.github/templates/`

## Projection Rules

- Edit these files here when changing GitHub/Copilot instruction or hook behavior.
- Regenerate the projected repo surface with:

```powershell
pwsh -File .\scripts\runtime\render-github-instruction-surfaces.ps1 -RepoRoot .
```

- `.github/` still keeps GitHub-native governance assets such as workflows,
  policies, schemas, runbooks, and governance catalogs authored in place. Only
  the provider-authored surfaces above are projected from
  `definitions/providers/github/`.