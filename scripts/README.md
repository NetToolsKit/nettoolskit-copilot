# Scripts

> Repository-owned runtime commands for bootstrap, projection, validation, diagnostics, and maintenance.

---

## Introduction

`scripts/` is the runtime surface for repository operations. It renders projected surfaces, applies runtime configuration, validates policy and docs, and keeps maintenance tasks deterministic. The Rust crates own the primary implementation surfaces and the native `ntk runtime` / `ntk validation` commands are the preferred operator contracts.

Authoritative non-code assets live under `definitions/`. Provider and runtime folders such as `.github/`, `.codex/`, `.claude/`, and `.vscode/` are generated surfaces that these scripts render, sync, and validate.

---

## Features

- ✅ Bootstrap and sync repository runtime surfaces from versioned assets through native runtime commands and targeted wrapper launchers where needed.
- ✅ Render projected provider and editor surfaces from canonical definitions.
- ✅ Validate README, instruction, policy, and workspace standards with the Rust validation boundary.
- ✅ Run health, remediation, security, and maintenance entrypoints through native commands where available.
- ✅ Keep operational commands deterministic and script-driven.

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Quick Start](#quick-start)
- [Build and Tests](#build-and-tests)
- [References](#references)
- [License](#license)

---

## Quick Start

```powershell
ntk runtime doctor --repo-root . --detailed
ntk runtime healthcheck --repo-root . --runtime-profile all --validation-profile dev
ntk validation all --repo-root . --validation-profile dev
ntk validation readme-standards --repo-root .
```

---

## Build and Tests

```powershell
pwsh -File .\scripts\runtime\bootstrap.ps1
pwsh -File .\scripts\runtime\render-provider-surfaces.ps1 -RepoRoot .
pwsh -File .\scripts\runtime\self-heal.ps1 -StrictExtras
ntk runtime doctor --repo-root . --detailed
ntk validation all --repo-root . --validation-profile release
ntk validation readme-standards --repo-root .
```

---

## References

- [Repository README](../README.md)
- [Planning README](../planning/README.md)
- [Definitions README](../definitions/README.md)
- [AGENTS](../.github/AGENTS.md)
- [Copilot Instructions](../.github/copilot-instructions.md)
- [Bootstrap](runtime/bootstrap.ps1)
- [Render Provider Surfaces](runtime/render-provider-surfaces.ps1)
- `ntk runtime doctor --repo-root . --detailed`
- `ntk runtime healthcheck --repo-root . --runtime-profile all --validation-profile release`
- [Self-Heal](runtime/self-heal.ps1)
- Native Validate All: `ntk validation all --repo-root . --validation-profile release`
- Native README Standards Check: `ntk validation readme-standards --repo-root .`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---