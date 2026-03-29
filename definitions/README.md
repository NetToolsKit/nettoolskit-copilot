# Definitions Tree

> Repository-owned authoritative non-code assets projected into provider and editor surfaces.

---

## Introduction

`definitions/` contains the source-of-truth assets that render into repository-managed surfaces such as `.github/`, `.codex/`, `.claude/`, and `.vscode/`.

The authoritative projection map between authored definitions, generated exceptions, projected destinations, and renderer ownership lives in `.github/governance/provider-surface-projection.catalog.json`.

---

## Features

- ✅ Shared definitions reused across provider and runtime surfaces
- ✅ Provider-specific authored assets kept separate from generated projections
- ✅ Catalog-driven rendering for repository-owned runtime surfaces
- ✅ Generated outputs kept out of hand-edit paths

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [References](#references)
- [License](#license)

---

## References

- [definitions/shared/README.md](definitions/shared/README.md)
- [definitions/shared/prompts/README.md](definitions/shared/prompts/README.md)
- [definitions/shared/prompts/poml/README.md](definitions/shared/prompts/poml/README.md)
- [definitions/providers/github/README.md](definitions/providers/github/README.md)
- [definitions/providers/vscode/profiles/README.md](definitions/providers/vscode/profiles/README.md)
- [definitions/providers/vscode/workspace/README.md](definitions/providers/vscode/workspace/README.md)
- [definitions/providers/codex/mcp/README.md](definitions/providers/codex/mcp/README.md)
- [definitions/providers/codex/orchestration/README.md](definitions/providers/codex/orchestration/README.md)
- [definitions/providers/codex/scripts/README.md](definitions/providers/codex/scripts/README.md)
- [definitions/providers/codex/skills/README.md](definitions/providers/codex/skills/README.md)
- [.github/governance/provider-surface-projection.catalog.json](.github/governance/provider-surface-projection.catalog.json)
- `ntk runtime render-provider-surfaces --repo-root .`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---