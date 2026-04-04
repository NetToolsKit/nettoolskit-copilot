---
applyTo: "**/{pipeline,ci,cd,deploy,build,workflow,action}*/**/*.{yml,yaml,json,ps1,sh,dockerfile}"
priority: medium
---

# CI/CD And DevOps Platform

Use this instruction for pipeline architecture, stage boundaries, promotion
flows, artifact handling, and delivery strategy across CI/CD systems. Use
`ntk-runtime-workflow-generation.instructions.md` for GitHub Actions-specific
workflow authoring requirements, and use
`ntk-security-cicd-supply-chain-hardening.instructions.md` for trusted
workflow boundaries, action pinning, OIDC, runner isolation, and provenance
controls.

## Pipeline Structure

- Separate build, test, package, validation, and deploy concerns into explicit stages or jobs.
- Use fail-fast behavior where early failure avoids wasted compute.
- Keep environment promotion explicit instead of hiding it behind one monolithic job.
- Make rollback or recovery paths clear when deployment stages mutate live systems.
- Keep infrastructure-as-code and application delivery coordinated but not conflated.

## CI Baseline

- Run on pull requests and protected branches where appropriate.
- Keep build, test, quality, and security signals explicit.
- Use matrix execution only when real compatibility coverage is needed.
- Keep coverage, lint, and static analysis visible in pipeline outputs.
- Prefer deterministic commands over opaque wrapper chains.

## Build Optimization

- Cache dependency stores intentionally.
- Reuse immutable artifacts between later stages when safe.
- Prefer incremental work only when it does not compromise determinism.
- Keep build-time monitoring and artifact retention visible to operators.
- Avoid pipelines that rebuild the same deliverable repeatedly without reason.

## Testing And Release Gates

- Unit, integration, E2E, performance, and migration tests should run only where they add real release confidence.
- Keep test pyramid responsibilities explicit.
- Coordinate deployment gates with health checks, smoke checks, and rollback conditions.
- Use feature flags or staged rollout controls when delivery risk is material.

## Environments And Promotion

- Treat environment parity as an explicit concern.
- Keep configuration, secrets, and network boundaries environment-specific and reviewable.
- Use promotion gates, approvals, or deployment strategies intentionally.
- Prefer deployable artifacts that stay immutable across environments.
- Coordinate schema or migration changes with deployment order.

## Platform Guardrails

- Keep permissions least-privilege.
- Keep secrets out of logs and summaries.
- Prefer stable artifact names and locations.
- Avoid hidden side effects such as PR mutation or repository rewrites unless explicitly requested.
- Document the system-specific workflow constraints close to the implementation surface.