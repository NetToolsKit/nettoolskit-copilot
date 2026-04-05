# Definitions Tree

> Repository-owned canonical non-code assets consumed by crates, runtime projections, and provider surfaces.

---

## Introduction

`definitions/` is the canonical root for repository-owned assets that are not application source code but still drive behavior across the workspace.

This tree now separates five canonical authored lanes:

- `instructions/` for stable engineering and governance rules
- `templates/` for reusable authored artifacts used in generation and documentation flows
- `agents/` for controller and specialist orchestration contracts
- `skills/` for reusable specialist capability packs
- `hooks/` for lifecycle-triggered runtime behaviors

`providers/` remains the consumer and projection side of the model.

---

## Features

- ✅ Canonical roots for instructions, templates, agents, skills, and hooks
- ✅ Clear separation between authored definitions and provider-specific consumers
- ✅ Stable path contract for crate-driven generation and projection flows
- ✅ Canonical `definitions/templates/` now carries migrated docs/codegen templates plus the mirrored .NET scaffold tree
- ✅ Transitional compatibility while legacy `definitions/shared/` and root `templates/` are retired safely

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
  - [Architecture](#architecture)
  - [Canonical Roots](#canonical-roots)
  - [Provider Consumers](#provider-consumers)
  - [Migration Policy](#migration-policy)
- [References](#references)
- [License](#license)

---

### Architecture

```mermaid
graph TD
    DEFINITIONS["definitions/"]
    INSTR["instructions/"]
    TEMPLATES["templates/"]
    AGENTS["agents/"]
    SKILLS["skills/"]
    HOOKS["hooks/"]
    PROVIDERS["providers/"]
    LEGACY["shared/ (legacy compatibility)"]
    RENDER["ntk runtime render-provider-surfaces"]
    GITHUB["provider surfaces"]

    DEFINITIONS --> INSTR
    DEFINITIONS --> TEMPLATES
    DEFINITIONS --> AGENTS
    DEFINITIONS --> SKILLS
    DEFINITIONS --> HOOKS
    DEFINITIONS --> PROVIDERS
    DEFINITIONS --> LEGACY
    INSTR --> RENDER
    TEMPLATES --> RENDER
    AGENTS --> RENDER
    SKILLS --> RENDER
    HOOKS --> RENDER
    PROVIDERS --> RENDER
    RENDER --> GITHUB
```

---

## Canonical Roots

- `instructions/` holds the shallow canonical rules board organized by `governance`, `development`, `operations`, `security`, and `data`.
- `templates/` holds canonical authored templates organized by `codegen`, `docs`, `manifests`, `prompts`, and `workflows`.
- `agents/` holds controller and specialist agent definitions such as `super-agent`, `planner`, `reviewer`, and `implementer`.
- `skills/` holds reusable capability packs grouped by engineering role.
- `hooks/` holds lifecycle entrypoints such as `session-start`, `pre-tool-use`, `subagent-start`, and `stop`.

Each canonical root should be authored here first, then projected or consumed elsewhere.

---

## Provider Consumers

`definitions/providers/` remains the consumer side of the model.

- `github/` contains `.github/`-oriented surfaces and compatibility consumers.
- `vscode/` contains workspace and editor consumers.
- `codex/` contains Codex runtime, MCP, orchestration, and skill consumers.
- `claude/` contains Claude runtime and skill consumers.

Provider trees should consume canonical definitions instead of inventing parallel sources of truth.

---

## Migration Policy

This root is mid-migration and currently preserves compatibility on purpose.

- `definitions/shared/` remains available as the legacy canonical surface until all consumers are realigned.
- Root `templates/` remains available until its authored content is migrated into `definitions/templates/`.
- The reorganization must prefer copy-then-cutover over destructive moves so documents are not lost during path normalization.
- Human-facing examples belong in `docs/samples/`; canonical reusable authored assets belong in `definitions/templates/`.

---

## References

- [definitions/instructions/README.md](instructions/README.md)
- [definitions/templates/README.md](templates/README.md)
- [definitions/agents/README.md](agents/README.md)
- [definitions/skills/README.md](skills/README.md)
- [definitions/hooks/README.md](hooks/README.md)
- [definitions/providers/README.md](providers/README.md)
- [definitions/shared/README.md](shared/README.md)
- [provider-surface-projection.catalog.json](providers/github/governance/provider-surface-projection.catalog.json)
- `ntk runtime render-provider-surfaces --repo-root .`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---