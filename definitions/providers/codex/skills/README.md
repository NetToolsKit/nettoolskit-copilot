# Shared Skills

> Repository-managed Codex skill definitions and their rendered runtime surface.

---

## Introduction

This folder stores the authored Codex skill contracts that are rendered into
`.codex/skills/` and consumed by the repository runtime. The skills are kept in
source control so routing, execution, and closeout behavior stay deterministic
and reviewable.

---

## Features

- ✅ Skills remain versioned and aligned with the repository instruction set
- ✅ The rendered `.codex/skills/` surface stays traceable to authored sources
- ✅ Shared router, runtime sync, and specialist workflows stay discoverable
- ✅ Starter and controller skills provide a deterministic Codex entry point

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [References](#references)
- [License](#license)

---

## References

- `.codex/README.md`
- `.codex/skills/`
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