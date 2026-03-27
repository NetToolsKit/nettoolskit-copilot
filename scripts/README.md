# Scripts

> Repository-owned operational entrypoints for bootstrap, projection, validation, health, and maintenance.

---

## Introduction

`scripts/` is the supported execution layer for repository operations. It renders projected surfaces, applies runtime configuration, validates policy and docs, and keeps maintenance tasks deterministic.

Authoritative non-code assets live under `definitions/`. Provider and runtime folders such as `.github/`, `.codex/`, `.claude/`, and `.vscode/` are generated surfaces that these scripts render, sync, and validate.

---

## Features

- ✅ Bootstrap and sync repository runtime surfaces from versioned assets
- ✅ Render projected provider and editor surfaces from canonical definitions
- ✅ Validate README, instruction, policy, and workspace standards
- ✅ Run health, remediation, security, and maintenance entrypoints
- ✅ Keep operational commands deterministic and script-driven

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

- [.github/AGENTS.md](.github/AGENTS.md)
- [.github/copilot-instructions.md](.github/copilot-instructions.md)
- [.github/instructions/readme.instructions.md](.github/instructions/readme.instructions.md)
- [.github/instructions/nettoolskit-rules.instructions.md](.github/instructions/nettoolskit-rules.instructions.md)
- [definitions/README.md](definitions/README.md)
- [planning/README.md](planning/README.md)
- [scripts/runtime/bootstrap.ps1](scripts/runtime/bootstrap.ps1)
- [scripts/runtime/render-provider-surfaces.ps1](scripts/runtime/render-provider-surfaces.ps1)
- [scripts/runtime/healthcheck.ps1](scripts/runtime/healthcheck.ps1)
- [scripts/runtime/self-heal.ps1](scripts/runtime/self-heal.ps1)
- [scripts/validation/validate-all.ps1](scripts/validation/validate-all.ps1)
- [scripts/validation/validate-readme-standards.ps1](scripts/validation/validate-readme-standards.ps1)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---