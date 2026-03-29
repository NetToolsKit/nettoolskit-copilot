# VS Code Workspace Assets

> Versioned VS Code workspace templates and snippets for repository-managed authoring surfaces.

---

## Introduction

`definitions/providers/vscode/workspace/` stores the authoritative source for tracked VS Code workspace assets.

The rendered `.vscode/` surface is projected from this tree. Workspace settings, MCP templates, base workspace definitions, and snippet templates all stay versioned here so the repository can regenerate local and global runtime files deterministically.

---

## Features

- ✅ Template-first source for VS Code workspace settings and MCP projections
- ✅ Versioned base workspace definition for shared workspace defaults
- ✅ Versioned snippet templates for Copilot and Codex workflows
- ✅ Deterministic rendering into the tracked `.vscode/` surface

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [References](#references)
- [License](#license)

---

## References

- `definitions/providers/vscode/workspace/base.code-workspace`
- `definitions/providers/vscode/workspace/settings.tamplate.jsonc`
- `definitions/providers/vscode/workspace/snippets/codex-cli.tamplate.code-snippets`
- `definitions/providers/vscode/workspace/snippets/copilot.tamplate.code-snippets`
- `definitions/providers/vscode/profiles/README.md`
- `scripts/runtime/apply-vscode-templates.ps1`
- `scripts/runtime/render-vscode-workspace-surfaces.ps1`
- `scripts/runtime/sync-vscode-global-mcp.ps1`
- `scripts/runtime/sync-vscode-global-settings.ps1`
- `scripts/runtime/sync-vscode-global-snippets.ps1`
- `scripts/runtime/sync-workspace-settings.ps1`
- `scripts/validation/validate-instructions.ps1`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---