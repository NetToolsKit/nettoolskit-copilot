---
name: core-runtime-sync
description: Sync shared repo assets into local ~/.github and ~/.codex, and apply MCP settings from the shared canonical runtime catalog.
---

# Codex Runtime Sync

Use this skill to keep local runtime folders aligned with the versioned repository structure.

## Load minimal context first

1. Load `.github/AGENTS.md`, `.github/copilot-instructions.md`, and `.github/instructions/repository-operating-model.instructions.md`.
2. Use the repository operating model as the canonical source for runtime projection responsibilities.

This includes syncing the complete versioned `.github/` asset set (instructions, routing catalog, prompts, chatmodes, schemas, templates) into `~/.github`.
It also composes `~/.codex/shared-scripts` from:
- `.codex/scripts/` (compatibility wrappers that now delegate to native `ntk runtime` commands for MCP surfaces)
- `scripts/common/` (shared helpers)
- `scripts/security/` (shared security gates)

## Sync Shared Assets

```powershell
pwsh -File scripts/runtime/bootstrap.ps1
```

## Apply MCP Servers To Codex

```powershell
ntk runtime sync-codex-mcp-config --create-backup
```

## Render VS Code MCP From The Canonical Runtime Catalog

```powershell
ntk runtime render-vscode-mcp-template --output-path .vscode/mcp.tamplate.jsonc
```

## Source Of Truth

- `.github/governance/mcp-runtime.catalog.json`
- `.codex/mcp/servers.manifest.json` (generated Codex subset)
- `ntk runtime sync-codex-mcp-config`
- `ntk runtime render-vscode-mcp-template`
- `scripts/common/*` (runtime shared helpers)
- `scripts/security/*` (runtime shared security scripts)