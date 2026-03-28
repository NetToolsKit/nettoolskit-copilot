# Multi-Agent Orchestration

> Versioned contracts for deterministic Codex and Super Agent execution.

---

## Introduction

This folder defines the authored contract layer for multi-agent execution,
including agent manifests, pipeline definitions, prompts, templates, and eval
fixtures. The projected `.codex/orchestration/` tree is rendered from these
files and remains a derived surface.

---

## Features

- ✅ Versioned agent contracts with explicit roles, skills, tool scopes, and budgets
- ✅ Deterministic handoffs for brainstorm, planning, execution, review, and closeout
- ✅ Prompt, template, and eval artifacts stay reproducible across local and CI runs
- ✅ Projected `.codex/orchestration/` files stay traceable back to the authored source

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
- `.github/schemas/agent.contract.schema.json`
- `.github/schemas/agent.pipeline.schema.json`
- `.github/schemas/agent.stage-plan-result.schema.json`
- `.github/schemas/agent.stage-implementation-result.schema.json`
- `.github/schemas/agent.stage-review-result.schema.json`
- `.github/schemas/agent.handoff.schema.json`
- `.github/schemas/agent.run-artifact.schema.json`
- `.github/schemas/agent.evals.schema.json`
- `scripts/orchestration/engine/invoke-codex-dispatch.ps1`
- `scripts/orchestration/engine/invoke-task-worker.ps1`
- `scripts/validation/validate-agent-orchestration.ps1`
- `planning/README.md`
- `planning/specs/README.md`

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---