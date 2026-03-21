<#
.SYNOPSIS
    Runtime tests for the repository onboarding/install orchestrator without external frameworks.

.DESCRIPTION
    Covers preview planning behavior for `scripts/runtime/install.ps1`.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/install-runtime.tests.ps1

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
# Fails the current test when the supplied condition is false.
function Assert-True {
    param(
        [bool] $Condition,
        [string] $Message
    )

    if (-not $Condition) {
        throw $Message
    }
}

# Fails the current test when the actual and expected values differ.
function Assert-Equal {
    param(
        [object] $Actual,
        [object] $Expected,
        [string] $Message
    )

    if ($Actual -ne $Expected) {
        throw ("{0} Expected='{1}' Actual='{2}'" -f $Message, $Expected, $Actual)
    }
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$scriptPath = Join-Path $resolvedRepoRoot 'scripts/runtime/install.ps1'

try {
    $previewResult = & $scriptPath -RepoRoot $resolvedRepoRoot -PreviewOnly

    Assert-True -Condition ($null -ne $previewResult) -Message 'Install preview must return a result object.'
    Assert-Equal -Actual $previewResult.previewOnly -Expected $true -Message 'Install preview must flag previewOnly.'
    Assert-Equal -Actual @($previewResult.steps).Count -Expected 5 -Message 'Install preview must plan the default onboarding steps.'
    Assert-Equal -Actual $previewResult.steps[0].name -Expected 'Bootstrap shared runtime assets' -Message 'Install preview must start with bootstrap.'
    Assert-Equal -Actual $previewResult.steps[4].name -Expected 'Run repository healthcheck' -Message 'Install preview must end with healthcheck.'
    Assert-True -Condition ($previewResult.steps[0].scriptPath -like '*scripts\runtime\bootstrap.ps1') -Message 'Install preview must reference bootstrap.ps1.'
    Assert-Equal -Actual $previewResult.summary.overallStatus -Expected 'preview' -Message 'Install preview must report preview summary status.'
    Assert-Equal -Actual $previewResult.issues.totalIssues -Expected 0 -Message 'Install preview must not report issues when only planning.'

    $reducedPreviewResult = & $scriptPath -RepoRoot $resolvedRepoRoot -PreviewOnly -SkipGlobalSettings -SkipGlobalSnippets -SkipGitHooks -SkipHealthcheck

    Assert-Equal -Actual @($reducedPreviewResult.steps).Count -Expected 1 -Message 'Install preview must honor skip switches.'
    Assert-Equal -Actual $reducedPreviewResult.steps[0].name -Expected 'Bootstrap shared runtime assets' -Message 'Reduced install preview must keep bootstrap.'
    Assert-Equal -Actual $reducedPreviewResult.summary.overallStatus -Expected 'preview' -Message 'Reduced install preview must report preview summary status.'
    Assert-Equal -Actual $reducedPreviewResult.issues.totalIssues -Expected 0 -Message 'Reduced install preview must not report issues when only planning.'

    Write-Host '[OK] runtime install tests passed.'
    exit 0
}
catch {
    Write-Host ("[FAIL] runtime install tests failed: {0}" -f $_.Exception.Message)
    exit 1
}