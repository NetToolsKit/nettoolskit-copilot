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

$script:ScriptRoot = Split-Path -Path $PSCommandPath -Parent
$script:RepositoryHelpersPath = Join-Path $script:ScriptRoot '..\..\common\repository-paths.ps1'
if (-not (Test-Path -LiteralPath $script:RepositoryHelpersPath -PathType Leaf)) {
    throw "Missing shared repository helper: $script:RepositoryHelpersPath"
}
. $script:RepositoryHelpersPath
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