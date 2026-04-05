---
applyTo: "**/{sonar,sonarqube,analysis,quality,lint,static}*/**/*.{properties,yml,yaml,json,xml,config,cs,ts,js}"
priority: low
---

# Static Analysis and SonarQube

Use this instruction for SonarQube configuration, static-analysis scope,
quality profile expectations, exclusions, report import, and analysis
governance.

Use adjacent instructions for related concerns:

- `ntk-operations-ci-cd-devops.instructions.md` for generic pipeline stage policy
- `ntk-operations-workflow-generation.instructions.md` for GitHub Actions wiring and artifact publication
- `ntk-security-vulnerabilities.instructions.md` for dependency vulnerability and supply-chain scanning policy

## SonarQube Scope

- Keep SonarQube focused on static analysis, quality gates, and trend visibility.
- Treat analysis configuration as repository policy, not as ad-hoc per-branch experimentation.
- Keep project keys, inclusions, exclusions, and report paths explicit and reviewable.

## Project Configuration

- Maintain deterministic project metadata and source/test boundaries.
- Exclude generated artifacts, vendor content, transient build output, and known non-source directories intentionally.
- Keep language-specific analyzer configuration aligned with the actual stack in the repository.
- Prefer stable report paths so CI and local analysis remain predictable.

## Coverage and Report Import

- Import coverage reports only from supported, deterministic test outputs.
- Keep coverage exclusions explicit and justified.
- Normalize report generation paths before SonarQube ingestion rather than relying on fragile globbing.
- Treat coverage import as analysis input, not as the source of truth for test execution policy.

## Quality Gates

- Keep gate thresholds explicit for coverage, duplication, reliability, maintainability, and security review.
- Fail on conditions that represent real quality regressions, not vanity metrics.
- Review hotspots and code-smell policies with the same repository governance used for other release gates.
- Keep thresholds stable enough for branch protection and trend interpretation.

## Rule and Profile Governance

- Use approved analyzer sets and quality profiles per language.
- Keep custom rules limited, reviewable, and justified by repository policy.
- Avoid conflicting rule sets that duplicate the same concern with different semantics.
- Document deprecated or advisory-only rule classes instead of silently mixing them into blocking policy.

## Branch and PR Analysis

- Keep branch and pull-request analysis semantics explicit.
- Use decoration and quality-gate reporting when the hosting platform supports it.
- Do not treat branch-analysis wiring as a substitute for repository validation or tests.

## Reporting and Operations

- Keep dashboards, trend reports, and remediation queues tied to maintainable ownership.
- Track analysis duration and scanner performance when static analysis becomes materially slow.
- Keep technical debt reporting actionable and tied to real remediation paths.

## Verification

- Validate the SonarQube project configuration, report import paths, exclusions, and quality-gate expectations.
- Keep pipeline-specific execution details out of this instruction and in workflow or CI/CD guidance.