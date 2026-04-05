# VS Code Workspace Assets

> Versioned VS Code workspace templates and snippets for repository-managed authoring surfaces.

---

## Introduction

`definitions/providers/vscode/workspace/` stores the authoritative source for tracked VS Code workspace assets.

The rendered `.vscode/` surface is projected from that tree. Workspace settings, MCP templates, base workspace definitions, and snippet templates all stay versioned so the repository can regenerate local and global runtime files deterministically.

For `settings` and `mcp`, the repository uses `*.tamplate.jsonc` files to avoid forcing active workspace files (`settings.json`, `mcp.json`) into source control. For `.code-workspace`, the repository uses `base.code-workspace` as the shared inheritance baseline. For snippets, the repository uses `*.tamplate.code-snippets` files so the runtime can remove the `.tamplate` segment before writing the global VS Code profile.

---

## Features

- ✅ Template-first source for VS Code workspace settings and MCP projections
- ✅ Versioned base workspace definition for shared workspace defaults
- ✅ Versioned snippet templates for Copilot and Codex workflows
- ✅ Deterministic rendering into the tracked `.vscode/` surface
- ✅ Canonical catalog-driven MCP config via `definitions/providers/github/governance/mcp-runtime.catalog.json`
- ✅ Versioned VS Code profile baselines under `.vscode/profiles/`
- ✅ Local helper mirror for the rendered global MCP state

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
- [API Reference](#api-reference)
- [Build and Tests](#build-and-tests)
- [Contributing](#contributing)
- [Dependencies](#dependencies)
- [References](#references)
- [License](#license)

---

## Installation

No installation is required beyond PowerShell 7+ and VS Code.

---

## Quick Start

```powershell
ntk runtime apply-vscode-templates
pwsh -File .\scripts\runtime\sync-vscode-global-mcp.ps1 -CreateBackup
```

---

## Usage Examples

### Example 1: Validate templates and references

```powershell
ntk validation instructions --repo-root . --warning-only false
```

### Example 2: Apply active VS Code files from templates

```powershell
ntk runtime apply-vscode-templates --force
```

### Example 3: Synchronize canonical snippets into the global VS Code profile

```powershell
pwsh -File .\scripts\runtime\sync-vscode-global-snippets.ps1
```

### Example 4: Render the versioned settings template into the global VS Code profile

```powershell
pwsh -File .\scripts\runtime\sync-vscode-global-settings.ps1 -CreateBackup
```

### Example 5: Render the versioned MCP template into the global VS Code profile and refresh the local helper mirror

```powershell
pwsh -File .\scripts\runtime\sync-vscode-global-mcp.ps1 -CreateBackup
```

### Example 6: Generate a workspace from the shared base and settings baseline

```powershell
pwsh -File .\scripts\runtime\sync-workspace-settings.ps1 -WorkspacePath C:\Users\me\Projects\api.code-workspace -FolderPath src\Api
```

---

## API Reference

### Main Files

- `settings.tamplate.jsonc`: base VS Code and Copilot instruction-routing settings
- `mcp.tamplate.jsonc`: generated VS Code MCP projection rendered from `definitions/providers/github/governance/mcp-runtime.catalog.json`
- `base.code-workspace`: shared pseudo-inheritance source for `.code-workspace` files
- `snippets/codex-cli.tamplate.code-snippets`: versioned Codex CLI snippet template synchronized into the global profile
- `snippets/copilot.tamplate.code-snippets`: versioned Copilot workflow snippet template synchronized into the global profile

### Runtime Entry Points

- `ntk runtime apply-vscode-templates`: applies templates into active `settings.json` and `mcp.json`
- `scripts/runtime/sync-vscode-global-settings.ps1`: renders `settings.tamplate.jsonc` into the global VS Code user profile
- `scripts/runtime/sync-vscode-global-mcp.ps1`: renders the canonical MCP runtime catalog into the global VS Code user profile and refreshes `.vscode/mcp-vscode-global.json`
- `scripts/runtime/sync-vscode-global-snippets.ps1`: synchronizes canonical snippets into the global VS Code user profile
- `scripts/runtime/sync-workspace-settings.ps1`: merges `base.code-workspace` with target workspaces and regenerates the approved workspace `settings` block

### Profile and Helper Surfaces

- `.vscode/profiles/`: rendered repository surface projected from `definitions/providers/vscode/profiles/`
- `.vscode/profiles/README.md`: operator guide for profile creation and profile-driven MCP enablement
- `.vscode/mcp-vscode-global.json`: ignored local helper mirror generated from the canonical MCP runtime catalog

---

## Build and Tests

```powershell
ntk validation instructions --repo-root . --warning-only false
```

---

## Contributing

- Keep template files valid JSON or JSONC.
- Do not commit active `settings.json` or `mcp.json`.
- When VS Code MCP servers change, update `definitions/providers/github/governance/mcp-runtime.catalog.json` first, then regenerate the tracked runtime projections.
- Treat `definitions/providers/vscode/workspace/base.code-workspace` as the shared workspace inheritance baseline.
- Treat `definitions/providers/vscode/workspace/snippets/*.tamplate.code-snippets` as the versioned snippet source.
- Keep `.vscode/profiles/` versioned and internally consistent with the checked-in `profile-*.json` set.
- Keep `.vscode/mcp-vscode-global.json` ignored and generated.

---

## Dependencies

- Runtime: PowerShell 7+
- Tools: VS Code, GitHub Copilot extension, optional MCP server runtimes such as `npx`

---

## References

- `definitions/providers/github/governance/mcp-runtime.catalog.json`
- `definitions/providers/vscode/workspace/base.code-workspace`
- `definitions/providers/vscode/workspace/settings.tamplate.jsonc`
- `definitions/providers/vscode/workspace/snippets/codex-cli.tamplate.code-snippets`
- `definitions/providers/vscode/workspace/snippets/copilot.tamplate.code-snippets`
- `definitions/providers/vscode/profiles/README.md`
- `ntk runtime apply-vscode-templates`
- `scripts/runtime/render-vscode-workspace-surfaces.ps1`
- `scripts/runtime/sync-vscode-global-mcp.ps1`
- `scripts/runtime/sync-vscode-global-settings.ps1`
- `scripts/runtime/sync-vscode-global-snippets.ps1`
- `scripts/runtime/sync-workspace-settings.ps1`
- `ntk validation instructions --repo-root . --warning-only false`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---