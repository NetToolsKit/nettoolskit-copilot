# Multi-Agent Orchestration

> Versioned contracts and baseline artifacts for deterministic multi-agent execution.

---

## Introduction

This folder defines the contract layer for planner, context-token-optimizer, specialist, tester, reviewer, and release-closeout collaboration. The objective is to keep multi-agent behavior auditable, reproducible, and validation-driven across local runtime and CI workflows.

---

## Features

- ✅ Versioned agent contracts with explicit roles, tool scopes, and budgets
- ✅ Deterministic pipeline orchestration with planner -> context-token-optimizer -> specialist -> tester -> reviewer -> release-closeout handoffs and completion criteria
- ✅ Real sequential stage dispatch through `codex exec` with schema-validated outputs
- ✅ Versioned planning artifacts under `planning/` for active and completed plan history
- ✅ Standard run artifacts for traceability and post-run analysis
- ✅ Persisted run state for retry diagnostics and execution auditing under `.temp/runs/<traceId>/`
- ✅ Golden eval fixtures for regression checks on orchestration behavior
- ✅ Schema-based validation integrated with repository quality gates

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

---

## Installation

No additional package installation is required. Use PowerShell 7+ and run commands from repository root.

---

## Quick Start

```powershell
pwsh -File .\scripts\validation\validate-agent-orchestration.ps1
pwsh -File .\scripts\runtime\run-agent-pipeline.ps1 -RequestText "Run orchestration smoke test" -ExecutionBackend codex-exec
```

---

## Usage Examples

### Example 1: Validate Orchestration Contracts

```powershell
pwsh -File .\scripts\validation\validate-agent-orchestration.ps1
```

### Example 2: Run Full Healthcheck Including Orchestration

```powershell
pwsh -File .\scripts\runtime\healthcheck.ps1 -StrictExtras
```

### Example 3: Execute Default Pipeline With Live Sequential Dispatch

```powershell
pwsh -File .\scripts\runtime\run-agent-pipeline.ps1 `
  -RequestText "Implement and validate multi-agent flow" `
  -ExecutionBackend codex-exec
```

### Example 4: Inspect Persisted Run Artifacts

```powershell
Get-Content -Raw .\.temp\runs\<traceId>\run-artifact.json
Get-Content -Raw .\.temp\runs\<traceId>\run-state.json
```

---

## API Reference

### Contract Files

- `agents.manifest.json`: canonical agent contract (roles, skills, tools, allowed paths, budgets, fallback links).
- `pipelines/default.pipeline.json`: baseline execution graph with stage order, handoffs, and completion criteria.
- `prompts/*.prompt.md`: structured planner, router, specialist, reviewer, and closeout prompts rendered by the live execution backend.
- `templates/handoff.template.json`: canonical stage handoff artifact shape.
- `templates/run-artifact.template.json`: canonical run summary artifact shape.
- `evals/golden-tests.json`: deterministic orchestration regression fixtures.

### Validation Scope

`scripts/validation/validate-agent-orchestration.ps1` validates:
- JSON schema contracts under `.github/schemas/`
- Cross-file integrity (agent IDs, skills, pipeline stages, handoffs, artifacts)
- Stage execution script existence and wiring (`execution.scriptPath`)
- Live dispatch wiring for `codex-exec` stages (`promptTemplatePath`, `responseSchemaPath`)
- Template and eval fixture consistency

### Runtime Model

- `scripts/runtime/run-agent-pipeline.ps1` supports two backends:
  - `script-only`: deterministic synthetic execution for offline contract smoke tests
  - `codex-exec`: real sequential dispatch through the local Codex CLI
- `scripts/orchestration/engine/invoke-codex-dispatch.ps1` is the adapter that renders prompts, invokes Codex, captures JSON output, and persists dispatch logs.
- Each pipeline run writes:
  - `run-artifact.json`: summarized execution result
  - `run-state.json`: mutable state snapshot for stage-by-stage diagnostics
  - `artifacts/*`: stage outputs and structured handoff assets

---

## Build and Tests

```powershell
# validate orchestration contracts only
pwsh -File .\scripts\validation\validate-agent-orchestration.ps1

# execute live sequential stages through codex exec
pwsh -File .\scripts\runtime\run-agent-pipeline.ps1 -RequestText "Smoke run" -ExecutionBackend codex-exec

# run end-to-end checks including orchestration, policy, and release governance
pwsh -File .\scripts\runtime\healthcheck.ps1 -StrictExtras
```

---

## Contributing

- Keep contract changes additive and versioned.
- Prefer explicit allowlists (`allowedPaths`) over broad wildcards.
- Keep fallback chains deterministic and short.
- Update related schema and validation logic in the same change when required.

---

## Dependencies

- Runtime: PowerShell 7+
- Live backend: local `codex` CLI available in `PATH` for `-ExecutionBackend codex-exec`
- Repository validators: scripts under `scripts/validation/` and `scripts/runtime/`

---

## References

- `.codex/README.md`
- `.github/schemas/agent.contract.schema.json`
- `.github/schemas/agent.pipeline.schema.json`
- `.github/schemas/agent.stage-plan-result.schema.json`
- `.github/schemas/agent.stage-implementation-result.schema.json`
- `.github/schemas/agent.stage-review-result.schema.json`
- `.github/schemas/agent.handoff.schema.json`
- `.github/schemas/agent.run-artifact.schema.json`
- `.github/schemas/agent.evals.schema.json`
- `scripts/orchestration/engine/invoke-codex-dispatch.ps1`
- `scripts/validation/validate-agent-orchestration.ps1`