# Scripts

> Repository-owned compatibility wrappers for bootstrap, projection, validation, health, and maintenance.

---

## Introduction

`scripts/` is the compatibility layer for repository operations. It renders projected surfaces, applies runtime configuration, validates policy and docs, and keeps maintenance tasks deterministic. The Rust crates own the primary implementation surfaces; these wrappers remain for shell-based entrypoints and fallback.

Authoritative non-code assets live under `definitions/`. Provider and runtime folders such as `.github/`, `.codex/`, `.claude/`, and `.vscode/` are generated surfaces that these scripts render, sync, and validate.

---

## Features

- ✅ Bootstrap and sync repository runtime surfaces from versioned assets through compatibility wrappers.
- ✅ Render projected provider and editor surfaces from canonical definitions.
- ✅ Validate README, instruction, policy, and workspace standards with the Rust validation boundary.
- ✅ Run health, remediation, security, and maintenance wrappers when shell invocation is required.
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
pwsh -File .\scripts\runtime\bootstrap.ps1
pwsh -File .\scripts\runtime\healthcheck.ps1 -StrictExtras
pwsh -File .\scripts\validation\validate-all.ps1 -ValidationProfile dev
pwsh -File .\scripts\validation\validate-readme-standards.ps1
```

---

## Build and Tests

```powershell
pwsh -File .\scripts\runtime\bootstrap.ps1
pwsh -File .\scripts\runtime\render-provider-surfaces.ps1 -RepoRoot .
pwsh -File .\scripts\runtime\self-heal.ps1 -StrictExtras
pwsh -File .\scripts\validation\validate-all.ps1 -ValidationProfile release
pwsh -File .\scripts\validation\validate-readme-standards.ps1
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
- [Healthcheck](runtime/healthcheck.ps1)
- [Self-Heal](runtime/self-heal.ps1)
- [Validate All](validation/validate-all.ps1)
- [Validate README Standards](validation/validate-readme-standards.ps1)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---