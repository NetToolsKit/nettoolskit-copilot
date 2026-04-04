---
applyTo: ".github/workflows/*.{yml,yaml}"
priority: high
---

# Workflow Generation

Use this instruction for GitHub Actions workflow authoring. General CI/CD stage
and promotion guidance belongs in `ntk-runtime-ci-cd-devops.instructions.md`.
Use `ntk-security-cicd-supply-chain-hardening.instructions.md` for trusted
workflow boundaries, `pull_request_target` restrictions, immutable action
pinning, OIDC, runner isolation, SBOM, and provenance controls.

## Purpose

- standardize GitHub Actions workflows for deterministic validation and delivery
- keep workflow authoring auditable, reproducible, and safe by default
- preserve repository-owned quality, security, and artifact evidence gates

## Shared Script Source Policy For External Repositories

- For GitHub Actions workflows in other repositories, do not copy shared scripts into the target repository.
- Consume shared scripts from the approved source at a pinned ref.
- Pin by immutable commit SHA or approved release tag.
- Verify downloaded script checksums before execution.
- Execute downloaded scripts from runner temp/workspace, not from ad-hoc locations.

## Mandatory GitHub Actions Baseline

- Pin all `uses:` actions by full commit SHA.
- Keep source versions in inline comments where that improves reviewability.
- Use least-privilege `permissions` at workflow level and narrow further per job only when needed.
- Define `concurrency` with `cancel-in-progress: true` when stale runs should not survive.
- Set explicit `timeout-minutes` for every job.
- Avoid dynamic script downloads unless checksum-verified.
- Never print secrets or tokens to logs.
- Do not add automatic PR creation or merge behavior unless explicitly requested.

## Required Validation Coverage

Validation-oriented workflows must explicitly include:

- repository validation
- test execution for the relevant stack
- security and vulnerability checks
- code quality or style checks
- artifact or report evidence for traceability

Prefer native `ntk validation` surfaces where the repository already provides them.

## Vulnerability And Supply-Chain Gates

- Prefer the repository security gate scripts when the workspace owns them.
- For external repositories, fetch the same gate from the approved shared source with checksum validation.
- Use stack auto-detection and skip non-applicable ecosystems explicitly.
- Include dependency review, SBOM generation, or provenance artifacts when the workflow scope requires them.

## Trigger And Execution Patterns

- Validation workflows normally support `pull_request`, `push`, and `workflow_dispatch`.
- Observability or audit workflows may add `schedule`.
- Keep job names stable for branch protection and governance baselines.
- Use matrix builds only when they provide real compatibility or platform coverage.

## Logging And Evidence

- Upload relevant artifacts with an explicit retention period.
- Append concise summaries to `GITHUB_STEP_SUMMARY` when useful.
- Keep generated report paths stable under `.temp/` when repository scripts expect them.
- Prefer machine-readable output plus short human-readable summaries.

## Authoring Checklist

- pinned action SHAs
- minimal permissions
- explicit timeout and concurrency
- explicit validation, security, and quality coverage
- artifact upload and summary
- no hidden destructive repository mutation