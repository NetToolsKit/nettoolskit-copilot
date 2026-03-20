<#
.SYNOPSIS
    Runtime tests for the CI pre-build security snapshot wrapper.

.DESCRIPTION
    Verifies the repository-owned CI wrapper can execute in warning-only mode
    against this repository root and return a success code without duplicating
    stack-detection logic in workflows.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/ci-security-snapshot.tests.ps1

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Resolve-RepositoryRoot {
    param(
        [string] $RequestedRoot
    )

    $candidates = @()

    if (-not [string]::IsNullOrWhiteSpace($RequestedRoot)) {
        try {
            $candidates += (Resolve-Path -LiteralPath $RequestedRoot).Path
        }
        catch {
            throw "Invalid RepoRoot path: $RequestedRoot"
        }
    }

    $candidates += (Get-Location).Path

    foreach ($candidate in ($candidates | Select-Object -Unique)) {
        $current = $candidate
        for ($index = 0; $index -lt 6 -and -not [string]::IsNullOrWhiteSpace($current); $index++) {
            if ((Test-Path -LiteralPath (Join-Path $current '.github')) -and (Test-Path -LiteralPath (Join-Path $current '.codex'))) {
                return $current
            }

            $current = Split-Path -Path $current -Parent
        }
    }

    throw 'Could not detect repository root containing both .github and .codex.'
}

function Assert-True {
    param(
        [bool] $Condition,
        [string] $Message
    )

    if (-not $Condition) {
        throw $Message
    }
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$scriptPath = Join-Path $resolvedRepoRoot 'scripts/security/Invoke-CiPreBuildSecuritySnapshot.ps1'

try {
    Assert-True -Condition (Test-Path -LiteralPath $scriptPath -PathType Leaf) -Message 'CI security snapshot script must exist.'

    & $scriptPath -RepoRoot $resolvedRepoRoot -WarningOnly:$true -AllowMissingCargoAudit
    $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }

    Assert-True -Condition ($exitCode -eq 0) -Message 'CI security snapshot wrapper must succeed in warning-only mode.'

    Write-Host '[OK] CI security snapshot tests passed.'
    exit 0
}
catch {
    Write-Host ("[FAIL] CI security snapshot tests failed: {0}" -f $_.Exception.Message)
    exit 1
}