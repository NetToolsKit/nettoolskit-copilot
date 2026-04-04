---
applyTo: ".github/workflows/*.{yml,yaml}"
priority: high
---

# CI/CD Supply Chain Hardening

Use this instruction for trusted-workflow design, GitHub Actions hardening,
runner isolation, artifact integrity, and release provenance. Keep general
pipeline structure in `ntk-runtime-ci-cd-devops.instructions.md`, and keep
GitHub Actions authoring mechanics in
`ntk-runtime-workflow-generation.instructions.md`.

## Primary External References

- Use OWASP CI/CD Security Cheat Sheet as the baseline for SCM hardening,
  pipeline isolation, secrets handling, least privilege, and visibility.
- Use OWASP Software Supply Chain Security Cheat Sheet for build-tool
  inventory, hardening, code signing, provenance, isolated builds, and
  dependency chain risk reduction.
- Use OWASP Dependency Graph & SBOM Best Practices Cheat Sheet for SBOM
  generation, signing, provenance, retention, and vulnerability triage.
- Use OWASP Secrets Management Cheat Sheet for CI/CD secret storage, rotation,
  least privilege, and fork-safe handling.

## Trusted Workflow Baseline

- Default to `pull_request`, not `pull_request_target`, for untrusted pull
  request validation.
- Treat `pull_request_target` as exceptional and high risk.
- Never combine `pull_request_target` with checkout or execution of untrusted
  pull-request head code, scripts, artifacts, or generated content.
- Require explicit, documented justification for any privileged workflow trigger
  that can touch secrets, protected branches, releases, or deployment
  environments.
- Keep workflow files under review/branch-protection policy equivalent to
  production code.

## Immutable Dependency And Action Pinning

- Pin third-party GitHub Actions to full 40-character commit SHAs.
- Do not rely on mutable refs such as `@main`, `@master`, `@latest`, `@v1`, or
  similar tags as the trust boundary.
- Pin container images by digest where image immutability matters.
- Keep lockfiles, dependency manifests, and action references reviewable and
  version controlled.
- Re-verify upstream publisher trust before bumping pinned actions or build
  tooling.

## Workflow Permissions And Identity

- Set workflow-level `permissions` to least privilege and narrow further per job
  only when required.
- Prefer short-lived federated identity (OIDC/workload identity federation) over
  long-lived secrets or PATs.
- Scope identities per environment, pipeline stage, and release surface.
- Keep release and deployment credentials unavailable to routine PR validation
  jobs.
- Require environment protection, explicit reviewers, and protected refs for
  mutation of production-facing assets.

## Runner And Execution Environment Hardening

- Prefer ephemeral, isolated runners for sensitive jobs.
- Separate build, test, release, and deploy runners when trust levels differ.
- Avoid interactive shell access or ad-hoc debugging on production-capable
  runners.
- Restrict outbound network access from runners where operationally feasible.
- Do not allow privileged containers, uncontrolled Docker socket access, or
  shared mutable caches across different trust zones unless explicitly justified
  and isolated.

## Secrets And Sensitive Material

- Never hardcode secrets in workflows, scripts, repo files, or generated
  artifacts.
- Do not print secrets, tokens, kubeconfigs, cloud credentials, or masked
  values to logs.
- Keep secret blast radius small; avoid "big shared secrets" across many
  pipelines or environments.
- Rotate credentials regularly and immediately after any supply-chain suspicion.
- Ensure forks, copied job definitions, or mirrored workflows cannot inherit
  privileged secrets implicitly.

## Safe Execution Rules

- Forbid `curl|bash`, `wget|sh`, `python -c`, `node -e`, or equivalent remote
  bootstrap patterns unless the source, checksum, and trust model are explicit
  and reviewed.
- Keep build scripts and workflow logic in source control; do not hide critical
  behavior in mutable external services.
- Validate all downloaded tools, scripts, and release assets with checksums,
  signatures, or attestations before execution.
- Prefer repository-owned scripts and pinned tool installers over ad-hoc shell
  pipelines.

## Artifact Integrity, SBOM, And Provenance

- Generate SBOMs during build, not as an afterthought, so exact resolved
  dependencies and metadata are captured.
- Publish at least one machine-readable SBOM per release in SPDX or CycloneDX
  format.
- Bind SBOMs to build artifacts using signatures or attestations.
- Generate and retain provenance/attestation metadata for release artifacts.
- Fail the build or release job if required SBOM or provenance evidence cannot
  be produced.

## Review, Monitoring, And Incident Readiness

- Audit workflow changes, action bumps, release workflow edits, and secret
  scope changes like production-sensitive changes.
- Keep logs long enough for incident investigation and alert on suspicious
  secret access, workflow mutation, or unexpected network egress.
- Review recent tags, releases, package publication credentials, and runner
  images after any supply-chain suspicion.
- Treat suspicious runner execution as potential credential compromise and
  rotate secrets before resuming normal operations.

## Required Verification Checks

- Scan workflows for:
  - `pull_request_target`
  - checkout of `${{ github.event.pull_request.head.sha }}`
  - mutable `uses:` references
  - overbroad `permissions`
  - suspicious remote bootstrap commands
- Verify action refs, tool downloads, and container images are pinned to
  immutable identifiers where required.
- Verify release flows emit SBOM and provenance evidence.
- Verify secrets, publish credentials, and deployment credentials stay scoped to
  the smallest possible blast radius.