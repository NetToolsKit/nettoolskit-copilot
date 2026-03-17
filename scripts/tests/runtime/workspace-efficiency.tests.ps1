<#
.SYNOPSIS
    Runtime tests for workspace efficiency validation without external frameworks.

.DESCRIPTION
    Covers success, failure, and warning-only heuristics for
    `validate-workspace-efficiency.ps1`.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/workspace-efficiency.tests.ps1

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

function Assert-ExitCode {
    param(
        [int] $ExitCode,
        [int] $Expected,
        [string] $Message
    )

    if ($ExitCode -ne $Expected) {
        throw $Message
    }
}

function Write-TextFile {
    param(
        [string] $Path,
        [string] $Content
    )

    $parent = Split-Path -Path $Path -Parent
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }

    Set-Content -LiteralPath $Path -Value $Content
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$scriptPath = Join-Path $resolvedRepoRoot 'scripts/validation/validate-workspace-efficiency.ps1'

try {
    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    try {
        $validWorkspace = Join-Path $tempRoot 'valid.code-workspace'
        Write-TextFile -Path $validWorkspace -Content @'
{
  "folders": [
    { "path": "App" }
  ],
  "settings": {
    "chat.agent.maxRequests": 200
  }
}
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspaceSearchRoot $validWorkspace -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 0 -Message 'Valid workspace should pass.'

        $templateBaselinePath = Join-Path $tempRoot 'workspace-template.baseline.json'
        Write-TextFile -Path $templateBaselinePath -Content @'
{
  "version": 1,
  "templateWorkspacePaths": [
    "template-base.code-workspace"
  ],
  "requiredSettings": {},
  "forbiddenSettings": {},
  "recommendedSettings": {},
  "recommendedNumericUpperBounds": {},
  "heuristics": {}
}
'@
        $templateWorkspace = Join-Path $tempRoot 'template-base.code-workspace'
        Write-TextFile -Path $templateWorkspace -Content @'
{
  "folders": [],
  "extensions": {
    "recommendations": [
      "mhutchie.git-graph"
    ]
  }
}
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -BaselinePath $templateBaselinePath -WorkspaceSearchRoot $templateWorkspace -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 0 -Message 'Template workspace without settings should pass when declared in baseline.'
        Remove-Item -LiteralPath $templateWorkspace -Force
        Remove-Item -LiteralPath $templateBaselinePath -Force

        $missingSettingsWorkspace = Join-Path $tempRoot 'missing-settings.code-workspace'
        Write-TextFile -Path $missingSettingsWorkspace -Content @'
{
  "folders": [
    { "path": "App" }
  ]
}
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspaceSearchRoot $missingSettingsWorkspace -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Workspace without settings should fail.'
        Remove-Item -LiteralPath $missingSettingsWorkspace -Force

        $forbiddenSettingWorkspace = Join-Path $tempRoot 'forbidden-setting.code-workspace'
        Write-TextFile -Path $forbiddenSettingWorkspace -Content @'
{
  "folders": [
    { "path": "App" }
  ],
  "settings": {
    "git.openRepositoryInParentFolders": "always",
    "chat.agent.maxRequests": 200
  }
}
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspaceSearchRoot $forbiddenSettingWorkspace -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Workspace with forbidden parent-folder discovery should fail.'
        Remove-Item -LiteralPath $forbiddenSettingWorkspace -Force

        $duplicateWorkspace = Join-Path $tempRoot 'duplicate-paths.code-workspace'
        Write-TextFile -Path $duplicateWorkspace -Content @'
{
  "folders": [
    { "path": "App" },
    { "path": "./App" }
  ],
  "settings": {
    "chat.agent.maxRequests": 200
  }
}
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspaceSearchRoot $duplicateWorkspace -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Workspace with duplicate folder paths should fail.'
        Remove-Item -LiteralPath $duplicateWorkspace -Force

        $warningWorkspace = Join-Path $tempRoot 'warning-only.code-workspace'
        Write-TextFile -Path $warningWorkspace -Content @'
{
  "folders": [
    { "path": "C:/Users/example/.codex" },
    { "path": "C:/Users/example/.github" },
    { "path": "C:/Users/example/copilot-instructions" },
    { "path": "AppA" },
    { "path": "AppB" }
  ],
  "settings": {
    "chat.agent.maxRequests": 200
  }
}
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspaceSearchRoot $warningWorkspace -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 0 -Message 'Workspace heuristic warnings should not fail validation.'

        $redundantSettingWorkspace = Join-Path $tempRoot 'redundant-setting.code-workspace'
        Write-TextFile -Path $redundantSettingWorkspace -Content @'
{
  "folders": [
    { "path": "App" }
  ],
  "settings": {
    "extensions.autoUpdate": false,
    "chat.agent.maxRequests": 200
  }
}
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspaceSearchRoot $redundantSettingWorkspace -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Workspace with redundant global settings should fail.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    Write-Host '[OK] workspace efficiency tests passed.'
    exit 0
}
catch {
    Write-Host ("[FAIL] workspace efficiency tests failed: {0}" -f $_.Exception.Message)
    exit 1
}