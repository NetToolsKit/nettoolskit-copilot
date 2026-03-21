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
$script:CommonBootstrapPath = Join-Path $PSScriptRoot '..\common\common-bootstrap.ps1'
if (-not (Test-Path -LiteralPath $script:CommonBootstrapPath -PathType Leaf)) {
    $script:CommonBootstrapPath = Join-Path $PSScriptRoot '..\..\common\common-bootstrap.ps1'
}
if (-not (Test-Path -LiteralPath $script:CommonBootstrapPath -PathType Leaf)) {
    $script:CommonBootstrapPath = Join-Path $PSScriptRoot '..\..\shared-scripts\common\common-bootstrap.ps1'
}
if (-not (Test-Path -LiteralPath $script:CommonBootstrapPath -PathType Leaf)) {
    throw "Missing shared common bootstrap helper: $script:CommonBootstrapPath"
}
. $script:CommonBootstrapPath -CallerScriptRoot $PSScriptRoot -Helpers @('repository-paths')
# Fails the current runtime test when the supplied condition is false.
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

# Fails the current test when the supplied script block does not throw.
function Assert-ThrowsLike {
    param(
        [scriptblock] $Action,
        [string] $ExpectedPattern,
        [string] $Message
    )

    try {
        & $Action
    }
    catch {
        if ($_.Exception.Message -match $ExpectedPattern) {
            return
        }

        throw ("{0} Expected pattern='{1}' Actual='{2}'" -f $Message, $ExpectedPattern, $_.Exception.Message)
    }

    throw $Message
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$scriptPath = Join-Path $resolvedRepoRoot 'scripts/runtime/install.ps1'

try {
    $previewResult = & $scriptPath -RepoRoot $resolvedRepoRoot -PreviewOnly

    Assert-True -Condition ($null -ne $previewResult) -Message 'Install preview must return a result object.'
    Assert-Equal -Actual $previewResult.previewOnly -Expected $true -Message 'Install preview must flag previewOnly.'
    Assert-Equal -Actual $previewResult.runtimeProfile.name -Expected 'none' -Message 'Install preview must default to the non-intrusive none profile.'
    Assert-Equal -Actual @($previewResult.steps).Count -Expected 0 -Message 'Install preview must not plan any steps for the default none profile.'
    Assert-Equal -Actual $previewResult.summary.overallStatus -Expected 'preview' -Message 'Install preview must report preview summary status.'
    Assert-Equal -Actual $previewResult.issues.totalIssues -Expected 0 -Message 'Install preview must not report issues when only planning.'

    $allPreviewResult = & $scriptPath -RepoRoot $resolvedRepoRoot -PreviewOnly -RuntimeProfile all

    Assert-Equal -Actual $allPreviewResult.runtimeProfile.name -Expected 'all' -Message 'Install preview must honor explicit all profile selection.'
    Assert-Equal -Actual @($allPreviewResult.steps).Count -Expected 6 -Message 'Install preview must plan the full onboarding steps for profile all.'
    Assert-Equal -Actual $allPreviewResult.steps[0].name -Expected 'Bootstrap shared runtime assets' -Message 'All-profile install preview must start with bootstrap.'
    Assert-Equal -Actual $allPreviewResult.steps[4].name -Expected 'Configure global Git aliases' -Message 'All-profile install preview must include global Git alias setup after local hooks.'
    Assert-Equal -Actual $allPreviewResult.steps[5].name -Expected 'Run repository healthcheck' -Message 'All-profile install preview must end with healthcheck.'
    Assert-True -Condition ($allPreviewResult.steps[0].scriptPath -like '*scripts\runtime\bootstrap.ps1') -Message 'All-profile install preview must reference bootstrap.ps1.'
    Assert-Equal -Actual $allPreviewResult.summary.overallStatus -Expected 'preview' -Message 'All-profile install preview must report preview summary status.'
    Assert-Equal -Actual $allPreviewResult.issues.totalIssues -Expected 0 -Message 'All-profile install preview must not report issues when only planning.'

    $githubPreviewResult = & $scriptPath -RepoRoot $resolvedRepoRoot -PreviewOnly -RuntimeProfile github

    Assert-Equal -Actual @($githubPreviewResult.steps).Count -Expected 2 -Message 'GitHub-profile install preview must plan bootstrap plus healthcheck only.'
    Assert-Equal -Actual $githubPreviewResult.steps[0].name -Expected 'Bootstrap shared runtime assets' -Message 'GitHub-profile install preview must keep bootstrap.'
    Assert-Equal -Actual $githubPreviewResult.steps[1].name -Expected 'Run repository healthcheck' -Message 'GitHub-profile install preview must keep healthcheck.'

    $codexPreviewResult = & $scriptPath -RepoRoot $resolvedRepoRoot -PreviewOnly -RuntimeProfile codex

    Assert-Equal -Actual @($codexPreviewResult.steps).Count -Expected 2 -Message 'Codex-profile install preview must plan bootstrap plus healthcheck only.'
    Assert-Equal -Actual $codexPreviewResult.steps[0].name -Expected 'Bootstrap shared runtime assets' -Message 'Codex-profile install preview must keep bootstrap.'
    Assert-Equal -Actual $codexPreviewResult.steps[1].name -Expected 'Run repository healthcheck' -Message 'Codex-profile install preview must keep healthcheck.'

    Assert-ThrowsLike -Action { & $scriptPath -RepoRoot $resolvedRepoRoot -PreviewOnly -RuntimeProfile github -ApplyMcpConfig } -ExpectedPattern 'does not enable the Codex runtime surface' -Message 'Install preview must reject MCP apply when the selected profile does not enable Codex.'

    $reducedPreviewResult = & $scriptPath -RepoRoot $resolvedRepoRoot -PreviewOnly -RuntimeProfile all -SkipGlobalSettings -SkipGlobalSnippets -SkipGitHooks -SkipHealthcheck

    Assert-Equal -Actual @($reducedPreviewResult.steps).Count -Expected 1 -Message 'Install preview must honor skip switches on top of profile all.'
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