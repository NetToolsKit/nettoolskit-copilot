---
name: sec-api-performance-security-engineer
description: Build and harden high-performance APIs with strong security controls, endpoint budgets, rate limiting, and resilience patterns. Use when tasks involve API latency/throughput tuning, abuse resistance, authz hardening, or API release gates.
---

# API Performance Security Engineer

## Load context first

1. `.github/AGENTS.md`
2. `.github/copilot-instructions.md`
3. `.github/instructions/core/ntk-core-repository-operating-model.instructions.md`

## Instruction pack

- `.github/instructions/security/ntk-security-api-high-performance.instructions.md`
- `.github/instructions/data/ntk-data-orm.instructions.md`
- `.github/instructions/architecture/backend/ntk-backend-dotnet-csharp.instructions.md` (when .NET is in scope)
- `.github/instructions/security/ntk-security-vulnerabilities.instructions.md`

## Claude-native execution

Run as a `general-purpose` agent within the Super Agent pipeline.

## Execution workflow

1. Define endpoint performance and abuse-control objectives.
2. Apply request efficiency, query optimization, and caching controls.
3. Enforce authentication, authorization, validation, and rate-limiting rules.
4. Add resilience patterns and idempotency where retry is possible.
5. Validate with performance, security, and vulnerability checks before release.

## Validation examples

```powershell
ntk validation security-baseline --warning-only false
dotnet test --filter "Category=Unit"
```