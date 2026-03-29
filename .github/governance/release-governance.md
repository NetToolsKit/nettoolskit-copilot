# Release Governance

## Scope

This repository uses local-first governance for instruction and runtime assets.
Validation is deterministic and runs through the native `ntk validation` surface plus the remaining scripted runtime entrypoints under `scripts/runtime`.

## Branch Protection

Branch protection baseline is versioned in `.github/governance/branch-protection.baseline.json`.
Apply or validate baseline drift with:

```powershell
pwsh -File .\scripts\governance\set-branch-protection.ps1
pwsh -File .\scripts\governance\set-branch-protection.ps1 -Apply
```

Notes:
- Branch protection mutation is opt-in (`-Apply`).
- Script requires `gh` CLI authenticated with permissions to manage branch protection.
- Keep `required_status_checks.contexts` aligned with active GitHub Actions job names.

## CODEOWNERS

`CODEOWNERS` is mandatory and validated by:
- `ntk validation policy` (file presence)
- `ntk validation release-governance --warning-only false` (rule quality checks)

At minimum:
- Catch-all owner rule (`* owner`)
- Governance path ownership for `.github/`, `.githooks/`, and `scripts/`

## Contribution Intake

Community intake artifacts are part of the release governance baseline:
- `CONTRIBUTING.md`
- `.github/PULL_REQUEST_TEMPLATE.md`
- `.github/ISSUE_TEMPLATE/*`

These files are validated through the governance and provenance baselines so onboarding and contribution flow do not drift away from the versioned runtime model.

## Release Checklist

1. Run baseline validations:
   - `pwsh -File .\scripts\validation\validate-instructions.ps1`
   - `ntk validation policy`
   - `ntk validation security-baseline --warning-only false`
   - `ntk validation agent-permissions --warning-only false`
   - `ntk validation supply-chain --warning-only false`
   - `ntk validation warning-baseline --warning-only false`
   - `ntk validation agent-orchestration`
   - `ntk validation agent-hooks --repo-root . --warning-only false`
   - `ntk validation shell-hooks --repo-root . --warning-only false`
   - `ntk validation runtime-script-tests --repo-root . --warning-only false`
   - `ntk validation release-governance --warning-only false`
   - `ntk validation release-provenance --warning-only false`
   - `ntk validation audit-ledger --warning-only false`
   - `pwsh -File .\scripts\validation\validate-all.ps1 -ValidationProfile release`
2. Confirm branch protection drift is zero:
   - `pwsh -File .\scripts\governance\set-branch-protection.ps1`
3. Update `CHANGELOG.md` with semantic version entry `[X.Y.Z] - YYYY-MM-DD`.
4. Create tag `copilot-vX.Y.Z` after merge to default branch.
5. Export audit package:
   - `pwsh -File .\scripts\validation\export-audit-report.ps1 -StrictExtras`

## Rollback

1. Revert faulty commit(s) in a dedicated fix branch.
2. Re-run all validation scripts.
3. Re-apply branch protection baseline if drift was introduced.
4. Publish corrective changelog entry and matching tag.