# Phase 21: Security And Governance Consumer Sweep

Generated: 2026-04-05

## Status

- LastUpdated: 2026-04-05 16:30
- Objective: execute the next domain-level consumer sweep for `scripts/security/*.ps1` and `scripts/governance/*.ps1`, prove whether any leaf is safe to retire, and keep the shared checksum baseline correct if deletions happen.
- Normalized Request: continue the script-retirement planning flow after Phase 20, keep planning updated, and commit each stable phase separately.
- Active Branch: `docs/planning-gap-workstreams`
- Spec Path: `planning/specs/active/spec-script-retirement-phase-21-security-governance-sweep.md`
- Inputs:
  - `planning/active/plan-repository-consolidation-continuity.md`
  - `planning/specs/active/spec-repository-consolidation-continuity.md`
  - `planning/completed/script-retirement-safety-matrix.md`
  - `planning/completed/rust-script-parity-ledger.md`
  - `definitions/providers/github/governance/shared-script-checksums.manifest.json`
  - `scripts/security/*.ps1`
  - `scripts/governance/*.ps1`

## Scope Summary

1. `scripts/security/Install-SecurityAuditPrerequisites.ps1`
2. `scripts/security/Invoke-CiPreBuildSecuritySnapshot.ps1`
3. `scripts/security/Invoke-FrontendPackageVulnerabilityAudit.ps1`
4. `scripts/security/Invoke-PreBuildSecurityGate.ps1`
5. `scripts/security/Invoke-RustPackageVulnerabilityAudit.ps1`
6. `scripts/security/Invoke-VulnerabilityAudit.ps1`
7. `scripts/governance/set-branch-protection.ps1`
8. `scripts/governance/update-shared-script-checksums-manifest.ps1`

This phase is complete only if:

- every security/governance leaf is classified with concrete local-consumer evidence
- no delete is attempted without zero non-self consumer proof
- any deletion affecting `scripts/security/*.ps1` updates `definitions/providers/github/governance/shared-script-checksums.manifest.json` in the same slice
- the continuity workstream can treat Phase 21 as explicit evidence instead of an abstract pending bucket

## Ordered Tasks

### Task 1: Freeze The Security/Governance Inventory And Search Surface

Status: `[ ]` Pending

- Lock the 8-script working set above.
- Reuse deterministic search commands for every candidate:
  - `rg -n "<script-name>" definitions crates planning scripts docs`
  - `rg -n "shared-script-checksums.manifest.json|supply-chain|release-governance" definitions crates planning scripts docs`
- Confirm the authored governance surfaces that can block deletion:
  - `definitions/providers/github/governance/shared-script-checksums.manifest.json`
  - `definitions/providers/github/policies/*.json`
  - `definitions/providers/github/governance/*.json`

### Task 2: Execute The Security-Domain Consumer Sweep

Status: `[ ]` Pending

- Target paths:
  - `scripts/security/*.ps1`
- Expected blocker classes:
  - checksum/governance manifest entries
  - security policy baselines
  - supply-chain and release validation fixtures
  - retained security/runtime smoke tests
- Deliverables:
  - exact zero-consumer list for deletable security leaves
  - retained-blocker list for non-deletable security leaves
  - same-slice checksum-manifest update if any security leaf is deleted
- Commit checkpoint:
  - `docs(runtime-retirement): record Phase 21 security-domain consumer proof and checksum-baseline result`

### Task 3: Execute The Governance-Domain Consumer Sweep

Status: `[ ]` Pending

- Target paths:
  - `scripts/governance/*.ps1`
- Expected blocker classes:
  - governance policies and release baselines
  - branch-protection and release workflow guidance
  - validation fixtures and retained runtime/operator smoke tests
- Deliverables:
  - exact zero-consumer list for deletable governance leaves
  - retained-blocker list for non-deletable governance leaves
  - same-slice re-points for any deleted governance leaf
- Commit checkpoint:
  - `docs(runtime-retirement): record Phase 21 governance-domain consumer proof for governance wrappers`

### Task 4: Rebaseline And Close Out Phase 21

Status: `[ ]` Pending

- After every executed slice:
  - update `planning/completed/script-retirement-safety-matrix.md`
  - update `planning/completed/rust-script-parity-ledger.md`
  - update `planning/active/plan-repository-consolidation-continuity.md`
- If any security leaf is deleted:
  - update `definitions/providers/github/governance/shared-script-checksums.manifest.json`
- Closeout checkpoint:
  - move this plan/spec to `planning/completed/` and `planning/specs/completed/`

## Validation Checklist

- [ ] targeted `rg` consumer sweep across `definitions/`, `crates/`, `planning/`, `scripts/`, and `docs/`
- [ ] `cargo run -q -p nettoolskit-cli -- validation supply-chain --repo-root . --warning-only false`
- [ ] `cargo run -q -p nettoolskit-cli -- validation policy --repo-root .`
- [ ] `cargo run -q -p nettoolskit-cli -- validation instructions --repo-root . --warning-only false`
- [ ] `cargo run -q -p nettoolskit-cli -- validation planning-structure --repo-root . --warning-only false`
- [ ] `pwsh -NoProfile -File .\scripts\security\Invoke-RustPackageVulnerabilityAudit.ps1 -RepoRoot $PWD -ProjectPath . -FailOnSeverities Critical,High`
- [ ] `git diff --check`

## Risks And Fallbacks

- Security leaves may remain pinned by the authored checksum manifest even when native parity already exists.
- `Invoke-RustPackageVulnerabilityAudit.ps1` can stay intentionally retained longer than other leaves because it is still the explicit operator-facing audit launcher.
- Governance leaves may still be pinned by release/baseline evidence and should close as audit-only rather than force a delete.

## Closeout Expectations

- Phase 21 may close as audit-only if every leaf still has a live local consumer.
- The checksum manifest must never drift from the live `scripts/security/*.ps1` set.
- Phase 22 should not start until Phase 21 records a stable blocker or deletion baseline for the security/governance domain.