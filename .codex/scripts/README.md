# Codex Scripts

> Repository-managed Codex helper scripts and runtime entrypoints.

---

## Introduction

`definitions/providers/codex/scripts/` stores the authoritative wrapper documentation for the rendered Codex script surface.

`scripts/runtime/bootstrap.ps1` composes `~/.codex/shared-scripts` from `.codex/scripts/`, `scripts/common/`, `scripts/security/`, and `scripts/maintenance/`. The `.codex/scripts/` files remain compatibility wrappers only.

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
pwsh -File .\scripts\runtime\sync-codex-mcp-config.ps1 -CreateBackup
pwsh -File .\scripts\runtime\render-vscode-mcp-template.ps1 -OutputPath .\.vscode\mcp.tamplate.jsonc
pwsh -File .\scripts\runtime\bootstrap.ps1
```

---

## Usage Examples

### Example 1: Preview Codex MCP changes

```powershell
pwsh -File .\scripts\runtime\sync-codex-mcp-config.ps1 -DryRun
```

### Example 2: Apply MCP config with backup

```powershell
pwsh -File .\scripts\runtime\sync-codex-mcp-config.ps1 -TargetConfigPath "$env:USERPROFILE\.codex\config.toml" -CreateBackup
```

### Example 3: Render VS Code MCP output

```powershell
pwsh -File .\scripts\runtime\render-vscode-mcp-template.ps1 -OutputPath .\.vscode\mcp.tamplate.jsonc
```

---

## API Reference

### Runtime Entry Points

- `scripts/runtime/sync-codex-mcp-config.ps1`: applies the Codex MCP subset into the local Codex config
- `scripts/runtime/render-vscode-mcp-template.ps1`: renders the VS Code MCP projection from the canonical runtime catalog
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
pwsh -File .\scripts\runtime\render-vscode-mcp-template.ps1 -OutputPath .\.temp\vscode.mcp.generated.json
pwsh -File .\scripts\runtime\sync-codex-mcp-config.ps1 -DryRun
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

- `.github/governance/mcp-runtime.catalog.json`
- `definitions/providers/codex/scripts/`
- `.codex/mcp/README.md`
- `scripts/runtime/bootstrap.ps1`
- `scripts/runtime/render-vscode-mcp-template.ps1`
- `scripts/runtime/sync-codex-mcp-config.ps1`
- `scripts/validation/validate-shell-hooks.ps1`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---