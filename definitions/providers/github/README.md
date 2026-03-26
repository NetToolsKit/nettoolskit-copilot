# GitHub Provider Definitions

This tree is the authoritative source for repository-owned GitHub/Copilot instruction
and hook surfaces that are rendered into `.github/`.

## Authoritative Coverage

- `root/` -> managed root files rendered into `.github/`
- `agents/` -> rendered into `.github/agents/`
- `instructions/` -> rendered into `.github/instructions/`
- `prompts/` -> rendered into `.github/prompts/`
- `hooks/` -> rendered into `.github/hooks/`

## Projection Rules

- Edit these files here when changing GitHub/Copilot instruction or hook behavior.
- Regenerate the projected repo surface with:

```powershell
pwsh -File .\scripts\runtime\render-github-instruction-surfaces.ps1 -RepoRoot .
```

- `.github/` still keeps GitHub-native assets such as workflows, policies, schemas,
  runbooks, and templates authored in place. Only the instruction/runtime-oriented
  surfaces above are projected from `definitions/providers/github/`.