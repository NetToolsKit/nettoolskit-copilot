# GitHub Provider Definitions

> Authoritative GitHub/Copilot projection surfaces for the repository-owned `.github/` tree.

---

## Introduction

`definitions/providers/github/` stores the repository-authored provider surfaces that are rendered into `.github/`.

Shared instructions and reusable templates live under `definitions/shared/`. GitHub-native repository assets that are intentionally maintained in place stay in `.github/` and are not projected from this tree.

---

## Features

- ✅ Authoritative source for GitHub/Copilot runtime surfaces
- ✅ Separates managed root files, agents, chat modes, prompts, and hooks
- ✅ Keeps shared instruction and prompt assets out of the provider-specific tree
- ✅ Renders into `.github/` through a single repository-owned entrypoint

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [References](#references)
- [License](#license)

---

## References

- `definitions/providers/github/root/`
- `definitions/providers/github/agents/`
- `definitions/providers/github/chatmodes/`
- `definitions/providers/github/prompts/`
- `definitions/providers/github/hooks/`
- `definitions/shared/README.md`
- `scripts/runtime/render-github-instruction-surfaces.ps1`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---