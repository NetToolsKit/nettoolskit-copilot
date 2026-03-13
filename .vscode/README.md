# VS Code Workspace Assets

> Versioned VS Code templates and snippets for shared Copilot/Codex workflows.

---

## Introduction

This folder stores repository-managed VS Code assets used to bootstrap local and global runtime configuration.

For `settings` and `mcp`, the repository uses `*.tamplate.jsonc` files to avoid forcing active workspace files (`settings.json`, `mcp.json`) into the repository.

For `.code-workspace`, the repository uses `base.code-workspace` as the shared inheritance baseline for workspace-level recommendations and top-level defaults.

For snippets, the repository uses `*.tamplate.code-snippets` files so they follow the same template-first versioning pattern used by `settings` and `mcp`. During sync, the `.tamplate` segment is removed before writing to the global VS Code profile.

---

## Features

- ✅ Template-first settings via `settings.tamplate.jsonc`
- ✅ Template-first MCP config via `mcp.tamplate.jsonc`
- ✅ Base workspace pseudo-inheritance via `base.code-workspace`
- ✅ Template-first Copilot/Codex snippets under `snippets/`
- ✅ Validation integrated in `scripts/validation/validate-instructions.ps1`

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
- [API Reference](#api-reference)
- [Build and Tests](#build-and-tests)
- [Contributing](#contributing)
- [Dependencies](#dependencies)
- [References](#references)

---

## Installation

No installation is required beyond PowerShell 7+ and VS Code.

---

## Quick Start

Regenerate MCP template from the shared manifest:

```powershell
pwsh -File .\.codex\scripts\render-vscode-mcp.ps1 -OutputPath .\.vscode\mcp.tamplate.jsonc
pwsh -File .\scripts\runtime\apply-vscode-templates.ps1
```

---

## Usage Examples

### Example 1: Validate templates and references

```powershell
pwsh -File .\scripts\validation\validate-instructions.ps1
```

### Example 2: Inspect available snippets

```powershell
Get-ChildItem .\.vscode\snippets\*.tamplate.code-snippets
```

### Example 3: Apply active VS Code files from templates

```powershell
pwsh -File .\scripts\runtime\apply-vscode-templates.ps1 -Force
```

### Example 4: Synchronize canonical snippets into the global VS Code profile

```powershell
pwsh -File .\scripts\runtime\sync-vscode-global-snippets.ps1
```

### Example 5: Generate a workspace from the shared base and settings baseline

```powershell
pwsh -File .\scripts\runtime\sync-workspace-settings.ps1 -WorkspacePath C:\Users\me\Projects\api.code-workspace -FolderPath src\Api
```

---

## API Reference

- `settings.tamplate.jsonc`: base VS Code/Copilot instruction-routing settings.
- `mcp.tamplate.jsonc`: base VS Code MCP servers map derived from `.codex/mcp/servers.manifest.json`.
- `base.code-workspace`: shared pseudo-inheritance source for `.code-workspace` files.
- `snippets/codex-cli.tamplate.code-snippets`: versioned Codex CLI snippet template synchronized into the global profile.
- `snippets/copilot.tamplate.code-snippets`: versioned Copilot chat/workflow snippet template synchronized into the global profile.
- `scripts/runtime/apply-vscode-templates.ps1`: applies templates into active `settings.json` and `mcp.json`.
- `scripts/runtime/sync-vscode-global-snippets.ps1`: synchronizes canonical snippets into the global VS Code user profile.
- `scripts/runtime/sync-workspace-settings.ps1`: merges `base.code-workspace` with target workspaces and regenerates the approved workspace `settings` block.

---

## Build and Tests

```powershell
pwsh -File .\scripts\validation\validate-instructions.ps1
```

---

## Contributing

- Keep template files valid JSON/JSONC.
- Do not commit active `settings.json` or `mcp.json`.
- When MCP servers change, regenerate `mcp.tamplate.jsonc` from manifest.
- Apply templates locally when needed with `scripts/runtime/apply-vscode-templates.ps1`.
- Treat `base.code-workspace` as the shared workspace inheritance baseline.
- Treat `.vscode/snippets/*.tamplate.code-snippets` as the versioned snippet source and sync them into the global profile when they change.

---

## Dependencies

- Runtime: PowerShell 7+.
- Tools: VS Code, GitHub Copilot extension, optional MCP server runtimes (`npx`, etc.).

---

## References

- `.codex/mcp/servers.manifest.json`
- `.codex/scripts/render-vscode-mcp.ps1`
- `.vscode/base.code-workspace`
- `scripts/runtime/apply-vscode-templates.ps1`
- `scripts/runtime/sync-vscode-global-snippets.ps1`
- `scripts/runtime/sync-workspace-settings.ps1`
- `scripts/validation/validate-instructions.ps1`