# MCP Shared Config

> Authoritative Codex MCP catalog support docs and rendered configuration references.

---

## Introduction

`definitions/providers/codex/mcp/` stores the authoritative support files for the shared Codex MCP runtime surface.

The rendered `.codex/mcp/` surface is projected from that tree. The canonical server definitions live in `definitions/providers/github/governance/mcp-runtime.catalog.json`, and the native `ntk runtime render-mcp-runtime-artifacts` plus `ntk runtime sync-codex-mcp-config` commands keep Codex and VS Code MCP setups aligned without making the Codex subset the primary source of truth.

---

## Features

- ✅ Canonical MCP server definitions come from `definitions/providers/github/governance/mcp-runtime.catalog.json`
- ✅ Rendered Codex and VS Code support files stay versioned and reproducible
- ✅ Local compatibility wrappers remain thin and point back to the canonical runtime catalog
- ✅ Cross-links keep the projected `.codex/mcp/` surface traceable to its authored source

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

No package installation is required. Edit the canonical catalog and run the native `ntk runtime` commands.

---

## Quick Start

```powershell
ntk runtime render-vscode-mcp-template --output-path .\.vscode\mcp.tamplate.jsonc
ntk runtime sync-codex-mcp-config --create-backup
```

---

## Usage Examples

### Example 1: Generate VS Code MCP output

```powershell
ntk runtime render-vscode-mcp-template --output-path .\.vscode\mcp.tamplate.jsonc
```

### Example 2: Update local Codex MCP servers

```powershell
ntk runtime sync-codex-mcp-config --target-config-path "$env:USERPROFILE\.codex\config.toml" --create-backup
```

---

## API Reference

### Main Files

- `definitions/providers/github/governance/mcp-runtime.catalog.json`: source of truth for runtime server definitions
- `servers.manifest.json`: generated Codex subset used for Codex TOML sync
- `codex.config.template.toml`: reference template for local Codex config
- `vscode.mcp.template.json`: reference template for VS Code MCP config

### Runtime Entry Points

- `ntk runtime render-mcp-runtime-artifacts`: re-renders the tracked Codex MCP support surface
- `ntk runtime render-vscode-mcp-template`: generates the VS Code MCP projection from the canonical runtime catalog
- `ntk runtime sync-codex-mcp-config`: rewrites only the `[mcp_servers.*]` sections in the local Codex TOML config

---

## Build and Tests

```powershell
ntk runtime render-vscode-mcp-template --output-path .\.temp\vscode.mcp.generated.json
ntk runtime sync-codex-mcp-config --dry-run
```

---

## Contributing

- Add or update servers only in `definitions/providers/github/governance/mcp-runtime.catalog.json`.
- Keep backward-compatible field names unless migration is documented.
- Edit support docs and templates in `definitions/providers/codex/mcp/`, not directly in the projected `.codex/mcp/` copies.
- Regenerate both runtime projections after catalog changes with `ntk runtime render-mcp-runtime-artifacts`.

---

## Dependencies

- Runtime: PowerShell 7+
- Tooling examples: `npx` for stdio MCP servers

---

## References

- `definitions/providers/github/governance/mcp-runtime.catalog.json`
- `definitions/providers/codex/mcp/`
- `ntk runtime render-mcp-runtime-artifacts`
- `ntk runtime render-vscode-mcp-template`
- `ntk runtime sync-codex-mcp-config`
- `.codex/mcp/servers.manifest.json`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---