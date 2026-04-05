---
name: privacy-compliance-engineer
description: Design and enforce privacy and data protection controls across application, API, and database layers. Use when tasks involve PII handling, retention/deletion policies, consent, data minimization, auditability, or compliance-oriented architecture changes.
---

# Privacy Compliance Engineer

## Load minimal context first

1. Load `.github/AGENTS.md`, `.github/copilot-instructions.md`, and `.github/instructions/governance/ntk-governance-repository-operating-model.instructions.md`.
2. Route with `.github/instruction-routing.catalog.yml`.
3. Keep only mandatory files plus privacy and impacted domain packs.

## Privacy instruction pack

- `.github/instructions/data/ntk-data-privacy-compliance.instructions.md`
- `.github/instructions/security/ntk-security-vulnerabilities.instructions.md`
- `.github/instructions/data/ntk-data-database.instructions.md` (when data schema/queries are in scope)
- `.github/instructions/development/ntk-development-persistence-orm.instructions.md` (when API/service logic is in scope)

## Execution workflow

1. Identify personal data classes, flows, and processing purpose.
2. Apply minimization, masking, encryption, and least-privilege controls.
3. Define retention and deletion behavior with auditable evidence.
4. Add privacy-focused tests for tenant isolation and sensitive-data leakage.
5. Report residual compliance risks with explicit owners and deadlines.

## Validation examples

```powershell
ntk validation security-baseline --warning-only false
ntk validation release-provenance --warning-only false
```