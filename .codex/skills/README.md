# Shared Skills

> Repository-managed Codex skill definitions and their rendered runtime surface.

---

## Introduction

`definitions/providers/codex/skills/` stores the authoritative skill definitions aligned with `.github/instructions`.

The rendered runtime surface lives under `.codex/skills/`, and it is refreshed through `scripts/runtime/render-provider-skill-surfaces.ps1` before bootstrap and runtime sync consume it.

---

## Features

- ✅ Skills remain versioned and aligned with the repository instruction set
- ✅ The rendered `.codex/skills/` surface stays traceable to authored sources
- ✅ Shared router, runtime sync, and specialist workflows stay discoverable
- ✅ Starter and controller skills provide a deterministic Codex entry point
- ✅ Super Agent lifecycle, worktree isolation, and closeout automation stay repository-owned

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
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

```powershell
pwsh -File .\scripts\runtime\bootstrap.ps1
```

---

## Quick Start

```powershell
pwsh -File .\scripts\runtime\bootstrap.ps1
Get-ChildItem "$env:USERPROFILE\.agents\skills"
```

---

## Usage Examples

### Example 1: Software implementation

Use skills: `super-agent`, `brainstorm-spec-architect`, `plan-active-work-planner`, `context-token-optimizer`, then the routed specialist.

### Example 2: Testing and coverage

Use skill: `test-engineer`.

### Example 3: Runtime sync

Use skill: `core-runtime-sync`.

### Example 4: Isolated delivery and closeout

Use skills: `super-agent`, `worktree-isolation-engineer`, `release-closeout-engineer`.

---

## API Reference

### Core Skills

- `core-context-router`
- `core-runtime-sync`
- `brainstorm-spec-architect`
- `plan-active-work-planner`
- `context-token-optimizer`
- `worktree-isolation-engineer`
- `release-closeout-engineer`

### Specialist Skills

- `dev-software-engineer`
- `dev-dotnet-backend-engineer`
- `dev-frontend-vue-quasar-engineer`
- `dev-rust-engineer`
- `test-engineer`
- `review-code-engineer`
- `ops-devops-platform-engineer`
- `docs-release-engineer`
- `sec-security-vulnerability-engineer`
- `sec-api-performance-security-engineer`
- `obs-sre-observability-engineer`
- `privacy-compliance-engineer`
- `ops-resilience-chaos-engineer`

---

## Build and Tests

```powershell
Get-ChildItem .\.codex\skills -Recurse -Filter SKILL.md
pwsh -File .\scripts\runtime\bootstrap.ps1
```

---

## Contributing

- Keep skill names stable and descriptive.
- Treat `definitions/providers/codex/skills/` as the source of truth for the rendered `.codex/skills/` surface.
- Keep `SKILL.md` concise and reference existing instruction files.
- Update this README when adding or removing skills.

---

## Dependencies

- Runtime: PowerShell 7+ for bootstrap sync
- Codex runtime in `~/.codex`

---

## References

- `definitions/providers/codex/skills/`
- `.codex/skills/core-context-router/SKILL.md`
- `.codex/skills/core-runtime-sync/SKILL.md`
- `.github/instruction-routing.catalog.yml`
- `.github/instructions/nettoolskit-rules.instructions.md`
- `scripts/runtime/bootstrap.ps1`
- `scripts/runtime/render-provider-skill-surfaces.ps1`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---