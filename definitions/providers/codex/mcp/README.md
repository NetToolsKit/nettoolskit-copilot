# MCP Shared Config

> Authoritative Codex MCP catalog support docs and rendered configuration references.

---

## Introduction

This folder is the authored source for the shared Codex MCP runtime catalog, its
template files, and the projected support surface rendered into `.codex/mcp/`.
The canonical server definitions live in the repository governance catalog, and
these files keep the local Codex support surface aligned with that source of
truth.

---

## Features

- ✅ Canonical MCP server definitions come from `.github/governance/mcp-runtime.catalog.json`
- ✅ Rendered Codex and VS Code support files stay versioned and reproducible
- ✅ Local compatibility wrappers remain thin and point back to the canonical runtime catalog
- ✅ Cross-links keep the projected `.codex/mcp/` surface traceable to its authored source

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [References](#references)
- [License](#license)

---

## References

- `.github/governance/mcp-runtime.catalog.json`
- `scripts/runtime/render-mcp-runtime-artifacts.ps1`
- `scripts/runtime/render-vscode-mcp-template.ps1`
- `scripts/runtime/sync-codex-mcp-config.ps1`
- `.codex/mcp/`
- `.codex/README.md`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---