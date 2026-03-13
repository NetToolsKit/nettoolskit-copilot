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
            $hasLayout = (Test-Path -LiteralPath (Join-Path $current '.github')) -and (Test-Path -LiteralPath (Join-Path $current '.codex'))
            if ($hasLayout) {
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

    $reducedPreviewResult = & $scriptPath -RepoRoot $resolvedRepoRoot -PreviewOnly -SkipGlobalSettings -SkipGlobalSnippets -SkipGitHooks -SkipHealthcheck

    Assert-Equal -Actual @($reducedPreviewResult.steps).Count -Expected 1 -Message 'Install preview must honor skip switches.'
    Assert-Equal -Actual $reducedPreviewResult.steps[0].name -Expected 'Bootstrap shared runtime assets' -Message 'Reduced install preview must keep bootstrap.'

    Write-Host '[OK] runtime install tests passed.'
    exit 0
}
catch {
    Write-Host ("[FAIL] runtime install tests failed: {0}" -f $_.Exception.Message)
    exit 1
}