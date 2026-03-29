# VS Code Profiles

> Versioned VS Code profile baselines with profile-aware MCP selection.

---

## Introduction

`definitions/providers/vscode/profiles/` stores the authoritative `profile-*.json` definitions for repository-managed VS Code profile baselines.

The rendered repository surface lives under `.vscode/profiles/`, and `scripts/runtime/setup-vscode-profiles.ps1` is the operator entrypoint that applies a selected profile set and can drive MCP enablement.

Profiles are where the repository chooses which MCP servers should stay enabled for the current VS Code setup. The canonical MCP template still lives in `.vscode/mcp.tamplate.jsonc`, but each profile can override server enablement through its `mcp.servers.<name>.enabled` map.

---

## Features

- ✅ Versioned profile baselines for VS Code runtime selection
- ✅ Profile-aware MCP enable and disable mapping
- ✅ Rendered repository surface under `.vscode/profiles/`
- ✅ Operator entrypoint for listing, previewing, and applying profiles
- ✅ Deterministic MCP selection without hand-editing the base template

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Quick Start](#quick-start)
- [MCP Behavior](#mcp-behavior)
- [Profile Schema](#profile-schema)
- [References](#references)
- [License](#license)

---

## Quick Start

List available profiles:

```powershell
pwsh -File .\scripts\runtime\setup-vscode-profiles.ps1 -ListProfiles
```

Create one profile and sync the global MCP selection from it:

```powershell
pwsh -File .\scripts\runtime\setup-vscode-profiles.ps1 -ProfileName Frontend -CreateMcpBackup
```

Create multiple profiles but force one of them to drive MCP enablement:

```powershell
pwsh -File .\scripts\runtime\setup-vscode-profiles.ps1 -ProfileName Base,Frontend -McpProfileName Frontend -CreateMcpBackup
```

Preview without changing anything:

```powershell
pwsh -File .\scripts\runtime\setup-vscode-profiles.ps1 -DryRun -ProfileName "Backend .NET"
```

---

## MCP Behavior

The setup flow delegates MCP sync to `scripts/runtime/sync-vscode-global-mcp.ps1`.

That means:

- `%APPDATA%\Code\User\mcp.json` is regenerated from the tracked MCP template
- `.vscode/mcp-vscode-global.json` is refreshed as the ignored local helper mirror
- the selected profile can enable or disable MCP servers without changing the base template
- stable `${input:...}` auth ids let VS Code reuse securely stored credentials after the first prompt

If multiple profiles are selected and no explicit `-McpProfileName` is supplied, MCP sync is skipped because the choice is ambiguous.

---

## Profile Schema

Minimal shape:

```json
{
  "name": "Frontend",
  "description": "Vue and browser tooling profile.",
  "extends": "Base",
  "extensions": [
    "vue.volar"
  ],
  "mcp": {
    "servers": {
      "io.github.github/github-mcp-server": { "enabled": true },
      "microsoft/playwright-mcp": { "enabled": true },
      "com.microsoft/azure": { "enabled": false }
    }
  }
}
```

---

## References

- `definitions/providers/vscode/profiles/profile-base.json`
- `definitions/providers/vscode/profiles/profile-ai-data.json`
- `definitions/providers/vscode/profiles/profile-backend-dotnet.json`
- `definitions/providers/vscode/profiles/profile-backend-java.json`
- `definitions/providers/vscode/profiles/profile-devops.json`
- `definitions/providers/vscode/profiles/profile-frontend.json`
- `definitions/providers/vscode/workspace/README.md`
- `definitions/providers/vscode/workspace/base.code-workspace`
- `definitions/providers/vscode/workspace/settings.tamplate.jsonc`
- `scripts/runtime/setup-vscode-profiles.ps1`
- `scripts/runtime/sync-vscode-global-mcp.ps1`
- `.vscode/mcp.tamplate.jsonc`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---