# Shared POML Library

> Canonical POML prompt library for reusable repository prompts.

---

## Introduction

`definitions/shared/prompts/poml/` is the authoritative source for the shared
POML prompt library. The authored content here is projected into the tracked
prompt surface used by the repository tooling and provider workflows.

---

## Features

- ✅ Shared POML prompts are maintained in one canonical location
- ✅ The projected surface stays aligned with the authored source
- ✅ Provider-specific prompt entrypoints remain separate from shared prompt
  content

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

No separate installation step is required. The shared POML library is consumed directly from the repository root by projection workflows.

---

## Quick Start

Use this folder when prompt fragments must stay canonical in POML form and be projected into tracked provider/runtime surfaces.

---

## Usage Examples

- Add reusable POML prompt fragments here
- Keep provider-specific prompt wrappers outside this folder
- Re-render provider surfaces after edits with `ntk runtime render-provider-surfaces --repo-root .`

---

## API Reference

Primary authored surface:

- `definitions/shared/prompts/poml/`

Projected consumer surface:

- `.github/prompts/poml/`

---

## Build and Tests

Useful verification commands from the repository root:

```powershell
cargo run -q -p nettoolskit-cli -- validation instructions --repo-root . --warning-only false
cargo run -q -p nettoolskit-cli -- runtime render-provider-surfaces --repo-root .
```

---

## Contributing

Keep shared POML files canonical and provider-neutral. Do not duplicate the same prompt text across multiple projected provider folders when this shared lane can own it.

---

## Dependencies

These assets are consumed by:

- shared prompt projection flows
- projected `.github/prompts/poml/` surfaces
- provider/runtime workflows that require a shared POML library

---

## References

- Shared Prompt Assets: `definitions/shared/prompts/README.md`
- Shared Definitions: `definitions/shared/README.md`
- Projected Prompt Surface: `.github/prompts/poml/`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---