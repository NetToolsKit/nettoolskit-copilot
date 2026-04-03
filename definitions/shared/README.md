# Shared Definitions

> Canonical shared assets reused across multiple repository surfaces.

---

## Introduction

`definitions/shared/` contains repository-owned source artifacts that are shared
across provider and runtime surfaces. The folder is the authoritative input for
the instruction, prompt, and template content that gets projected into the
tracked workspace surfaces.

---

## Features

- ✅ Shared instruction assets are authored once and projected into
  `.github/instructions/`
- ✅ Shared prompt assets are authored once and projected into
  `.github/prompts/poml/`
- ✅ Shared templates stay reusable instead of being duplicated across provider
  folders
- ✅ The folder keeps canonical documentation separate from generated or
  provider-specific runtime surfaces

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [References](#references)
- [License](#license)

---

## References

- [definitions/shared/prompts/README.md](prompts/README.md)
- [definitions/shared/prompts/poml/README.md](prompts/poml/README.md)
- [Repository README Rules](../../.github/instructions/docs/ntk-docs-repository-readme-overrides.instructions.md)
- [README Template](../../.github/templates/readme-template.md)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---
