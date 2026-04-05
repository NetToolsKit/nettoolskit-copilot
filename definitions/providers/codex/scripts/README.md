# Codex Scripts

> Repository-managed Codex helper scripts and runtime entrypoints.

---

## Introduction

`definitions/providers/codex/scripts/` stores the authoritative wrapper documentation for the rendered Codex script surface.

`scripts/runtime/bootstrap.ps1` composes `~/.codex/shared-scripts` from `.codex/scripts/`, `scripts/common/`, `scripts/security/`, and `scripts/maintenance/`. The `.codex/scripts/` files remain compatibility wrappers only, but the MCP entrypoints now delegate to native `ntk runtime` commands.

---

## Features

- ✅ Repository-owned script wrappers remain versioned and reviewable
- ✅ Rendered `.codex/scripts/` files stay traceable to authored sources
- ✅ Bootstrap, routing, and validation helpers keep the Codex runtime aligned
- ✅ Shared script contracts stay discoverable from a single reference surface

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

No additional installation is required beyond PowerShell.

---

## Quick Start

```powershell
ntk runtime sync-codex-mcp-config --create-backup
ntk runtime render-vscode-mcp-template --output-path .\.vscode\mcp.tamplate.jsonc
pwsh -File .\scripts\runtime\bootstrap.ps1
```

---

## Usage Examples

### Example 1: Preview Codex MCP changes

```powershell
ntk runtime sync-codex-mcp-config --dry-run
```

### Example 2: Apply MCP config with backup

```powershell
ntk runtime sync-codex-mcp-config --target-config-path "$env:USERPROFILE\.codex\config.toml" --create-backup
```

### Example 3: Render VS Code MCP output

```powershell
ntk runtime render-vscode-mcp-template --output-path .\.vscode\mcp.tamplate.jsonc
```

---

## API Reference

### Runtime Entry Points

- `ntk runtime sync-codex-mcp-config`: applies the Codex MCP subset into the local Codex config
- `ntk runtime render-vscode-mcp-template`: renders the VS Code MCP projection from the canonical runtime catalog
- `scripts/runtime/bootstrap.ps1`: composes the shared Codex script runtime

### Shared Security Gates

- `scripts/security/Invoke-PreBuildSecurityGate.ps1`
- `scripts/security/Install-SecurityAuditPrerequisites.ps1`
- `scripts/security/Invoke-VulnerabilityAudit.ps1`
- `scripts/security/Invoke-FrontendPackageVulnerabilityAudit.ps1`
- `scripts/security/Invoke-RustPackageVulnerabilityAudit.ps1`

---

## Build and Tests

```powershell
ntk runtime render-vscode-mcp-template --output-path .\.temp\vscode.mcp.generated.json
ntk runtime sync-codex-mcp-config --dry-run
```

---

## Contributing

- Keep wrapper behavior thin and deterministic.
- Edit the authoritative copies under `definitions/providers/codex/scripts/`, then re-render `.codex/scripts/`.
- Preserve local user configuration outside MCP sections.
- Update `.codex/mcp/README.md` when wrapper behavior changes.

---

## Dependencies

- Runtime: PowerShell 7+
- Optional tooling from manifest server definitions, for example `npx`

---

## References

- `definitions/providers/github/governance/mcp-runtime.catalog.json`
- `definitions/providers/codex/scripts/`
- `.codex/mcp/README.md`
- `scripts/runtime/bootstrap.ps1`
- `ntk runtime render-vscode-mcp-template`
- `ntk runtime sync-codex-mcp-config`
- `ntk validation shell-hooks --repo-root .`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---