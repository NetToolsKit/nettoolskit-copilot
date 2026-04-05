# GitHub Provider Definitions

> Authoritative GitHub/Copilot projection surfaces for the repository-owned `.github/` tree.

---

## Introduction

`definitions/providers/github/` stores the repository-authored provider surfaces that are rendered into `.github/`.

Canonical reusable assets now live under `definitions/instructions/`,
`definitions/templates/`, `definitions/agents/`, `definitions/skills/`, and
`definitions/hooks/`. `definitions/shared/` remains compatibility-only while
older consumers finish cutting over.

---

## Features

- ✅ Authoritative source for GitHub/Copilot runtime surfaces
- ✅ Separates managed root files, agents, chat modes, prompts, and hooks
- ✅ Keeps canonical instruction, template, agent, skill, and hook assets out of the provider-specific tree
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
- `definitions/README.md`
- `definitions/instructions/README.md`
- `definitions/templates/README.md`
- `definitions/shared/README.md`
- `scripts/runtime/render-github-instruction-surfaces.ps1`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---