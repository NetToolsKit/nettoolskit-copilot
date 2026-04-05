# Shared Prompt Assets

> Reusable prompt sources shared across repository surfaces.

---

## Introduction

`definitions/shared/prompts/` stores prompt assets that are authored once and
projected into tracked prompt surfaces. It exists so shared prompt content stays
canonical and does not drift between provider-specific folders.

---

## Features

- ✅ Shared prompt sources stay centralized in one authored location
- ✅ POML content is projected into `.github/prompts/poml/`
- ✅ Provider folders remain focused on provider-specific prompt entrypoints
- ✅ The folder supports prompt reuse without duplicating source text

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
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

No separate installation step is required. These prompt assets are consumed directly from the repository root by projection and provider-surface workflows.

---

## Quick Start

Author shared prompt content here when the same source text must be reused across multiple provider surfaces.

---

## Usage Examples

- Keep reusable prompt text under `definitions/shared/prompts/`
- Keep shared POML prompt sources under `definitions/shared/prompts/poml/`
- Re-render provider surfaces after prompt changes with `ntk runtime render-provider-surfaces --repo-root .`

---

## API Reference

Current shared prompt lanes:

- `definitions/shared/prompts/`
- `definitions/shared/prompts/poml/`

---

## Build and Tests

Useful verification commands from the repository root:

```powershell
cargo run -q -p nettoolskit-cli -- validation instructions --repo-root . --warning-only false
cargo run -q -p nettoolskit-cli -- runtime render-provider-surfaces --repo-root .
```

---

## Contributing

Keep shared prompt assets provider-neutral. Provider-specific wrappers and entrypoints should stay in provider trees or projected runtime surfaces.

---

## Dependencies

These assets are consumed by:

- shared prompt projection flows
- GitHub/Codex/Claude provider mirrors
- authoring workflows that reuse canonical prompt text

---

## References

- [Shared POML Library](poml/README.md)
- [Shared Definitions](../README.md)
- [Projected Prompt Surface](../../../.github/prompts/poml/)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---