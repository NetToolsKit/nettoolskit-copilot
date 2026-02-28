---
applyTo: ".github/workflows/*.{yml,yaml}"
priority: high
---

# Purpose
- Standardize workflow generation for enterprise-grade CI/CD with deterministic security, testing, vulnerability, and code-quality coverage.
- Keep workflows auditable, reproducible, and safe-by-default.

# Mandatory Workflow Security Baseline
- Pin all `uses:` actions by full commit SHA and keep the source version in inline comments.
- Use least-privilege `permissions` at workflow level and override per job only when needed.
- Define `concurrency` with `cancel-in-progress: true` to prevent stale executions.
- Set explicit `timeout-minutes` for every job.
- Avoid dynamic script downloads during workflow execution unless checksum-verified.
- Never print secrets or tokens to logs; never echo full environment values containing credentials.
- Do not add automatic PR creation/merge actions unless explicitly requested by the user.

# Required Quality and Validation Coverage
- Every validation-oriented workflow must include:
- Build/Validation gate: run deterministic repository validation (`scripts/validation/validate-all.ps1` with selected profile).
- Test gate: run unit/integration/E2E checks relevant to the stack.
- Security gate: run baseline security checks and dependency vulnerability checks.
- Code-quality gate: run static analysis and style/lint checks.
- Artifact evidence: publish reports/logs for traceability.

# Vulnerability and Supply-Chain Gates
- For dependency security, prefer the shared pre-build gate:
```powershell
pwsh -NoLogo -NoProfile -File ./scripts/security/Invoke-PreBuildSecurityGate.ps1 `
  -RepoRoot $PWD `
  -WarningOnly:$true `
  -AllowMissingCargoAudit
```
- Use stack auto-detection and skip non-applicable ecosystems (`-SkipDotnet`, `-SkipFrontend`, `-SkipRust`) when absent.
- Include dependency review for PR flows when applicable (`actions/dependency-review-action`).
- For supply-chain observability, include SBOM generation and optional provenance attestation.

# Code Quality and Test Gate Expectations
- Run repository validation profile matching the workflow purpose (`dev`, `release`, or `enforced`).
- Ensure test execution is explicit and deterministic (no hidden defaults).
- Publish machine-readable outputs (JSON/SARIF/coverage) and human-readable summaries.
- Prefer warning-only mode for observability workflows unless the user explicitly requests blocking enforcement.

# Trigger and Execution Patterns
- Validation workflows:
- `pull_request`, `push` (main/master), `workflow_dispatch`.
- Observability workflows:
- `schedule` + `workflow_dispatch`; optional `push` for fast feedback.
- Use matrix builds only when real compatibility coverage is needed.
- Keep job names stable for branch protection and governance baselines.

# Logging and Evidence
- Always upload relevant artifacts with retention policy.
- Append concise execution summaries to `GITHUB_STEP_SUMMARY` when useful.
- Keep report paths stable under `.temp/` for script compatibility.

# GitHub Actions Structure Reference
```yaml
name: Example Validation Workflow

on:
  pull_request:
  push:
    branches: [main, master]
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: example-validation-${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  validate:
    runs-on: ubuntu-latest
    timeout-minutes: 20
    steps:
      - name: Checkout
        uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd # v6.0.2

      - name: Validate all (warning-only)
        shell: pwsh
        run: |
          pwsh -NoLogo -NoProfile -File ./scripts/validation/validate-all.ps1 `
            -RepoRoot $PWD `
            -ValidationProfile release `
            -WarningOnly:$true `
            -OutputPath ./.temp/audit/validate-all.latest.json

      - name: Security gate (warning-only)
        shell: pwsh
        run: |
          pwsh -NoLogo -NoProfile -File ./scripts/security/Invoke-PreBuildSecurityGate.ps1 `
            -RepoRoot $PWD `
            -WarningOnly:$true `
            -AllowMissingCargoAudit

      - name: Upload artifacts
        if: always()
        uses: actions/upload-artifact@bbbca2ddaa5d8feaa63e36b76fdaad77386f024f # v7.0.0
        with:
          name: validation-artifacts-${{ github.run_id }}
          path: |
            .temp/audit/
            .temp/vulnerability-audit/
          if-no-files-found: warn
          retention-days: 14
```

# Authoring Checklist
- Pinned action SHAs.
- Minimal permissions.
- Explicit timeout/concurrency.
- Tests + security + vulnerability + quality coverage.
- Artifact upload and summary.
- No hidden side effects (no PR auto-creation, no destructive repository mutation).