<#
.SYNOPSIS
    Runtime tests for the repository-owned Super Agent entrypoint scripts.

.DESCRIPTION
    Validates the thin Super Agent entry commands that wrap brainstorming,
    planning, execution, and parallel dispatch flows.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/super-agent-entrypoints.tests.ps1

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot
)

$ErrorActionPreference = 'Stop'

$script:ScriptRoot = Split-Path -Path $PSCommandPath -Parent
$script:RepositoryHelpersPath = Join-Path $script:ScriptRoot '..\..\common\repository-paths.ps1'
if (-not (Test-Path -LiteralPath $script:RepositoryHelpersPath -PathType Leaf)) {
    throw "Missing shared repository helper: $script:RepositoryHelpersPath"
}
. $script:RepositoryHelpersPath
# Fails the current test when the actual and expected values differ.
function Assert-Equal {
    param([object] $Actual, [object] $Expected, [string] $Message)
    if ($Actual -ne $Expected) {
        throw ("{0}. Expected '{1}', got '{2}'." -f $Message, $Expected, $Actual)
    }
}

# Fails the current test when the supplied condition is false.
function Assert-True {
    param([bool] $Condition, [string] $Message)
    if (-not $Condition) {
        throw $Message
    }
}

$resolvedRepoRoot = if ([string]::IsNullOrWhiteSpace($RepoRoot)) { (Resolve-Path (Join-Path $PSScriptRoot '..\..\..')).Path } else { (Resolve-Path $RepoRoot).Path }

try {
    $brainstorm = (& (Join-Path $resolvedRepoRoot 'scripts/runtime/invoke-super-agent-brainstorm.ps1') -RepoRoot $resolvedRepoRoot -RequestText 'brainstorm request' -PreviewOnly) | ConvertFrom-Json -Depth 50
    Assert-Equal $brainstorm.mode 'brainstorm' 'Brainstorm wrapper should expose the brainstorm mode.'
    Assert-Equal $brainstorm.parameters.StopAfterStageId 'spec' 'Brainstorm wrapper should stop after spec.'

    $plan = (& (Join-Path $resolvedRepoRoot 'scripts/runtime/invoke-super-agent-plan.ps1') -RepoRoot $resolvedRepoRoot -RequestText 'plan request' -PreviewOnly) | ConvertFrom-Json -Depth 50
    Assert-Equal $plan.mode 'plan' 'Plan wrapper should expose the plan mode.'
    Assert-Equal $plan.parameters.StopAfterStageId 'plan' 'Plan wrapper should stop after plan.'

    $execute = (& (Join-Path $resolvedRepoRoot 'scripts/runtime/invoke-super-agent-execute.ps1') -RepoRoot $resolvedRepoRoot -RequestText 'execute request' -PreviewOnly) | ConvertFrom-Json -Depth 50
    Assert-Equal $execute.mode 'execute' 'Execute wrapper should expose the execute mode.'
    Assert-True (-not $execute.parameters.PSObject.Properties['StopAfterStageId']) 'Execute wrapper should run the full lifecycle.'

    $parallel = (& (Join-Path $resolvedRepoRoot 'scripts/runtime/invoke-super-agent-parallel-dispatch.ps1') -RepoRoot $resolvedRepoRoot -RequestText 'parallel request' -PreviewOnly) | ConvertFrom-Json -Depth 50
    Assert-Equal $parallel.mode 'parallel-dispatch' 'Parallel wrapper should expose the parallel-dispatch mode.'
    Assert-Equal $parallel.parameters.ExecutionBackend 'codex-exec' 'Parallel wrapper should force codex-exec.'

    Write-Host '[OK] super-agent entrypoint tests passed.'
    exit 0
}
catch {
    Write-Host ("[FAIL] super-agent entrypoint tests failed: {0}" -f $_.Exception.Message)
    exit 1
}