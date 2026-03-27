# VS Code Profiles

> Versioned VS Code profile baselines with profile-aware MCP selection.

---

## Introduction

`definitions/providers/vscode/profiles/` stores the authoritative `profile-*.json` definitions for repository-managed VS Code profile baselines.

The rendered repository surface lives under `.vscode/profiles/`, and `scripts/runtime/setup-vscode-profiles.ps1` is the operator entrypoint that applies a selected profile set and can drive MCP enablement.

---

## Features

- ✅ Versioned profile baselines for VS Code runtime selection
- ✅ Profile-aware MCP enable and disable mapping
- ✅ Rendered repository surface under `.vscode/profiles/`
- ✅ Operator entrypoint for listing, previewing, and applying profiles

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Quick Start](#quick-start)
- [References](#references)
- [License](#license)

---

## Quick Start

List the available profiles:

```powershell
pwsh -File .\scripts\runtime\setup-vscode-profiles.ps1 -ListProfiles
```

Preview a profile selection without changing files:

```powershell
pwsh -File .\scripts\runtime\setup-vscode-profiles.ps1 -DryRun -ProfileName "Backend .NET"
```

Apply one profile and back up the MCP selection:

```powershell
pwsh -File .\scripts\runtime\setup-vscode-profiles.ps1 -ProfileName Frontend -CreateMcpBackup
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