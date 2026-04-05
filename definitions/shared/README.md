# Shared Definitions

> Legacy compatibility surface preserved while canonical roots move to shallow top-level lanes under `definitions/`.

---

## Introduction

`definitions/shared/` remains available to avoid breaking legacy consumers
while canonical authored roots have already moved to:

- `definitions/instructions/`
- `definitions/templates/`
- `definitions/agents/`
- `definitions/skills/`
- `definitions/hooks/`

Do not treat this folder as the long-term target structure for new authored
content when an equivalent canonical root already exists.

---

## Features

- ✅ Preserves existing consumers while canonical paths are being introduced
- ✅ Keeps older projections functioning during migration
- ✅ Reduces risk of document loss by using copy-then-cutover instead of destructive moves
- ✅ Provides a compatibility checkpoint while providers are realigned

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
  - [Architecture](#architecture)
  - [Shared Asset Boundaries](#shared-asset-boundaries)
  - [Projection Contract](#projection-contract)
- [References](#references)
- [License](#license)

---

### Architecture

```mermaid
graph TD
    SHARED["definitions/shared/"]
    LEGACY_INSTR["legacy instructions/"]
    LEGACY_PROMPTS["legacy prompts/"]
    LEGACY_TEMPLATES["legacy templates/"]
    CANONICAL["definitions/* canonical roots"]
    PROVIDERS["provider consumers"]

    SHARED --> LEGACY_INSTR
    SHARED --> LEGACY_PROMPTS
    SHARED --> LEGACY_TEMPLATES
    CANONICAL --> PROVIDERS
    SHARED -. compatibility .-> CANONICAL
```

---

## Shared Asset Boundaries

`definitions/shared/` should now be treated as a migration compatibility lane.

- keep existing consumers working until their canonical root is in place
- prefer authoring new root-level canonical content under `definitions/`
- do not create new long-lived taxonomy branches here when an equivalent root
  exists already

---

## Projection Contract

`definitions/shared/` is now compatibility-first.

- Transitional compatibility source: `definitions/shared/`
- Preferred canonical source: `definitions/instructions/`, `definitions/templates/`, `definitions/agents/`, `definitions/skills/`, `definitions/hooks/`
- Remaining authored shared prompt lane: `definitions/shared/prompts/` and `definitions/shared/prompts/poml/`
- Projected runtime surface: `.github/`, `.codex/`, `.claude/`, `.vscode/`
- Ownership and projection rules: `definitions/providers/github/governance/provider-surface-projection.catalog.json`
- Naming contract: semantic domain folders plus stable `ntk-*` file names for
  instruction assets

Do not delete or force-move existing shared content until all known consumers
have been realigned.

---

## References

- [definitions/README.md](../README.md)
- [definitions/instructions/README.md](../instructions/README.md)
- [definitions/templates/README.md](../templates/README.md)
- [definitions/agents/README.md](../agents/README.md)
- [definitions/skills/README.md](../skills/README.md)
- [definitions/hooks/README.md](../hooks/README.md)
- [definitions/providers/README.md](../providers/README.md)
- [definitions/shared/instructions/README.md](instructions/README.md)
- [definitions/shared/prompts/README.md](prompts/README.md)
- [definitions/shared/prompts/poml/README.md](prompts/poml/README.md)
- [Repository README Rules](../../.github/instructions/docs/ntk-docs-repository-readme-overrides.instructions.md)
- [README Template](../templates/docs/readme-template.md)
- [provider-surface-projection.catalog.json](../providers/github/governance/provider-surface-projection.catalog.json)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---