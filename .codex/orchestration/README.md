# Multi-Agent Orchestration

> Versioned contracts for deterministic Codex and Super Agent execution.

---

## Introduction

`definitions/providers/codex/orchestration/` stores the authoritative contract files for multi-agent execution.

The rendered `.codex/orchestration/` surface is projected from that tree. It defines the contract layer for Super Agent intake, brainstorming/spec, planning, context routing, specialist execution, testing, review, and closeout so multi-agent behavior remains auditable, reproducible, and validation-driven.

---

## Features

- ✅ Versioned agent contracts with explicit roles, skills, tool scopes, and budgets
- ✅ Deterministic handoffs for brainstorm, planning, execution, review, and closeout
- ✅ Prompt, template, and eval artifacts stay reproducible across local and CI runs
- ✅ Projected `.codex/orchestration/` files stay traceable back to the authored source
- ✅ Versioned planning artifacts under `planning/` plus versioned spec artifacts under `planning/specs/`

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

No additional package installation is required. Use PowerShell 7+ and run commands from repository root.

---

## Quick Start

```powershell
ntk validation agent-orchestration
ntk runtime render-provider-surfaces --repo-root . --renderer-id codex-orchestration-surfaces
pwsh -File .\scripts\runtime\run-agent-pipeline.ps1 -RequestText "Run orchestration smoke test" -ExecutionBackend codex-exec
```

---

## Usage Examples

### Example 1: Validate orchestration contracts

```powershell
ntk validation agent-orchestration
```

### Example 2: Execute the default pipeline with live dispatch

```powershell
pwsh -File .\scripts\runtime\run-agent-pipeline.ps1 -RequestText "Implement and validate multi-agent flow" -ExecutionBackend codex-exec
```

### Example 3: Create an isolated worktree first

```powershell
pwsh -File .\scripts\runtime\new-super-agent-worktree.ps1 -WorktreeName "feature-slice"
```

---

## API Reference

### Contract Files

- `agents.manifest.json`: canonical agent contract with roles, skills, tools, allowed paths, and budgets
- `pipelines/default.pipeline.json`: baseline execution graph with stage order, handoffs, and completion criteria
- `prompts/*.prompt.md`: structured intake, spec, planner, router, specialist, reviewer, and closeout prompts
- `templates/handoff.template.json`: canonical stage handoff artifact shape
- `templates/run-artifact.template.json`: canonical run summary artifact shape
- `evals/golden-tests.json`: deterministic orchestration regression fixtures

### Runtime Entry Points

- `scripts/runtime/run-agent-pipeline.ps1`: supports `script-only` and `codex-exec` backends
- `scripts/runtime/new-super-agent-worktree.ps1`: creates isolated git worktrees for risky or parallelized execution flows
- `scripts/runtime/invoke-super-agent-brainstorm.ps1`
- `scripts/runtime/invoke-super-agent-plan.ps1`
- `scripts/runtime/invoke-super-agent-execute.ps1`
- `scripts/runtime/invoke-super-agent-parallel-dispatch.ps1`
- `scripts/orchestration/engine/invoke-codex-dispatch.ps1`
- `scripts/orchestration/engine/invoke-task-worker.ps1`

---

## Build and Tests

```powershell
ntk validation agent-orchestration
pwsh -File .\scripts\runtime\run-agent-pipeline.ps1 -RequestText "Smoke run" -ExecutionBackend codex-exec
pwsh -File .\scripts\tests\runtime\super-agent-worktree.tests.ps1 -RepoRoot .
pwsh -File .\scripts\tests\runtime\super-agent-entrypoints.tests.ps1 -RepoRoot .
```

---

## Contributing

- Keep contract changes additive and versioned.
- Prefer explicit allowlists over broad wildcards.
- Keep fallback chains deterministic and short.
- Update related schema and validation logic in the same change when required.

---

## Dependencies

- Runtime: PowerShell 7+
- Live backend: local `codex` CLI available in `PATH` for `-ExecutionBackend codex-exec`

---

## References

- `.github/schemas/agent.contract.schema.json`
- `.github/schemas/agent.pipeline.schema.json`
- `.github/schemas/agent.handoff.schema.json`
- `.github/schemas/agent.run-artifact.schema.json`
- `definitions/providers/codex/orchestration/`
- `scripts/orchestration/engine/invoke-codex-dispatch.ps1`
- `scripts/orchestration/engine/invoke-task-worker.ps1`
- `ntk validation agent-orchestration`
- `planning/README.md`
- `planning/specs/README.md`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---