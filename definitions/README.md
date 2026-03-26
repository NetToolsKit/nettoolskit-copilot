# Definitions Tree

`definitions/` contains repository-owned authoritative non-code assets that are
projected into provider/runtime surfaces such as `.github/`, `.codex/`, `.claude/`,
and `.vscode/`.

## Rules

- Treat `definitions/` as authoritative when a corresponding renderer exists.
- Treat projected surfaces as generated outputs; do not hand-edit them.
- Keep executable entrypoints under `scripts/`.
- Keep provider/runtime folders focused on projected assets, not original logic.
- Reserve `src/` for executable engine code and `tests/` for its future test suite.

## Current coverage

- `definitions/providers/github/{root,agents,instructions,prompts,chatmodes,hooks}/` -> projected instruction/runtime surface in `.github/`
- `definitions/providers/codex/skills/` -> `.codex/skills/`
- `definitions/providers/codex/orchestration/` -> `.codex/orchestration/`
- `definitions/providers/claude/skills/` -> `.claude/skills/`
- `definitions/providers/claude/runtime/` -> `.claude/`
- `definitions/providers/vscode/profiles/` -> `.vscode/profiles/`
- `definitions/providers/vscode/workspace/` -> selected authored `.vscode/` assets (`README.md`, `base.code-workspace`, `settings.tamplate.jsonc`, `snippets/`)

GitHub-native repository governance assets such as workflows, schemas, policies,
templates, and runbooks remain authored directly in `.github/`; only the
instruction/runtime-oriented GitHub surfaces above are projected from
`definitions/providers/github/`.

Use:

```powershell
pwsh -File .\scripts\runtime\render-github-instruction-surfaces.ps1 -RepoRoot .
pwsh -File .\scripts\runtime\render-provider-skill-surfaces.ps1 -RepoRoot .
pwsh -File .\scripts\runtime\render-codex-orchestration-surfaces.ps1 -RepoRoot .
pwsh -File .\scripts\runtime\render-claude-runtime-surfaces.ps1 -RepoRoot .
pwsh -File .\scripts\runtime\render-vscode-profile-surfaces.ps1 -RepoRoot .
pwsh -File .\scripts\runtime\render-vscode-workspace-surfaces.ps1 -RepoRoot .
```