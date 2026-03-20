<#
.SYNOPSIS
    Runtime tests for the versioned planning workspace structure.

.DESCRIPTION
    Validates the planning workspace contract by executing the repository
    planning structure validator in enforced mode.

.PARAMETER RepoRoot
    Optional repository root. If omitted, uses the current location.

.EXAMPLE
    pwsh -File scripts/tests/runtime/planning-structure.tests.ps1

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Resolves the repository root for the current script or test fixture.
function Resolve-RepositoryRoot {
    param([string] $RequestedRoot)

    if (-not [string]::IsNullOrWhiteSpace($RequestedRoot)) {
        return (Resolve-Path -LiteralPath $RequestedRoot).Path
    }

    return (Get-Location).Path
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$validationScriptPath = Join-Path $resolvedRepoRoot 'scripts/validation/validate-planning-structure.ps1'

& $validationScriptPath -RepoRoot $resolvedRepoRoot -WarningOnly:$false | Out-Null
$exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }

if ($exitCode -ne 0) {
    Write-Host '[FAIL] planning structure tests failed.'
    exit 1
}

Write-Host '[OK] planning structure tests passed.'
exit 0