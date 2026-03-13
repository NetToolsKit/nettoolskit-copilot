<#
.SYNOPSIS
    Runtime tests for workspace settings synchronization without external frameworks.

.DESCRIPTION
    Covers creation and update flows for `sync-workspace-settings.ps1`.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/workspace-settings-sync.tests.ps1

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
$scriptPath = Join-Path $resolvedRepoRoot 'scripts/runtime/sync-workspace-settings.ps1'

try {
    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    try {
        $baseWorkspace = Join-Path $tempRoot 'base.code-workspace'
        Write-TextFile -Path $baseWorkspace -Content @'
{
  "folders": [],
  "extensions": {
    "recommendations": [
      "mhutchie.git-graph"
    ]
  },
  "launch": {
    "configurations": []
  }
}
'@

        $existingWorkspace = Join-Path $tempRoot 'existing.code-workspace'
        Write-TextFile -Path $existingWorkspace -Content @'
{
  "folders": [
    { "name": "Api", "path": "src/Api" },
    { "name": "Ui", "path": "src/Ui" }
  ],
  "extensions": {
    "recommendations": [
      "rust-lang.rust-analyzer"
    ]
  },
  "settings": {
    "git.autofetch": true,
    "editor.fontSize": 99
  }
}
'@

        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspacePath $existingWorkspace -BaseWorkspacePath $baseWorkspace | Out-Null
        $existingDocument = Get-Content -Raw -LiteralPath $existingWorkspace | ConvertFrom-Json -Depth 100

        Assert-Equal -Actual @($existingDocument.folders).Count -Expected 2 -Message 'Existing workspace must preserve folders.'
        Assert-Equal -Actual @($existingDocument.extensions.recommendations).Count -Expected 2 -Message 'Existing workspace must merge extension recommendations with base workspace.'
        Assert-True -Condition (@($existingDocument.extensions.recommendations) -contains 'mhutchie.git-graph') -Message 'Existing workspace must inherit base workspace recommendations.'
        Assert-True -Condition (@($existingDocument.extensions.recommendations) -contains 'rust-lang.rust-analyzer') -Message 'Existing workspace must preserve workspace-specific recommendations.'
        Assert-Equal -Actual $existingDocument.settings.'git.autofetch' -Expected $false -Message 'Workspace sync must set git.autofetch to false.'
        Assert-Equal -Actual $existingDocument.settings.'git.openRepositoryInParentFolders' -Expected 'never' -Message 'Workspace sync must set parent-folder discovery to never.'
        Assert-Equal -Actual $existingDocument.settings.'git.autorefresh' -Expected $false -Message 'Workspace sync must set git.autorefresh to false.'
        Assert-Equal -Actual $existingDocument.settings.'extensions.autoUpdate' -Expected $false -Message 'Workspace sync must set extensions.autoUpdate to false.'
        Assert-Equal -Actual $existingDocument.settings.'github.copilot.nextEditSuggestions.enabled' -Expected $false -Message 'Workspace sync must set Copilot nextEditSuggestions to false.'
        Assert-Equal -Actual $existingDocument.settings.'workbench.startupEditor' -Expected 'welcomePage' -Message 'Workspace sync must preserve the standard welcome page startup value.'
        Assert-Equal -Actual $existingDocument.settings.'chat.emptyState.history.enabled' -Expected $true -Message 'Workspace sync must keep empty-state chat history enabled.'
        Assert-Equal -Actual $existingDocument.settings.'scm.repositories.visible' -Expected 4 -Message 'Workspace sync must cap visible repositories.'
        Assert-Equal -Actual $existingDocument.settings.'chat.agent.maxRequests' -Expected 400 -Message 'Workspace sync must cap chat.agent.maxRequests.'
        Assert-True -Condition ($null -eq $existingDocument.settings.PSObject.Properties['editor.fontSize']) -Message 'Workspace sync must not keep unrelated workspace-local settings.'
        Assert-Equal -Actual $existingDocument.settings.'files.watcherExclude'.'**/target/**' -Expected $true -Message 'Workspace sync must include required watcher exclude.'
        Assert-Equal -Actual $existingDocument.settings.'search.exclude'.'**/target' -Expected $true -Message 'Workspace sync must include required search exclude.'
        Assert-True -Condition ($null -ne $existingDocument.launch) -Message 'Workspace sync must inherit missing top-level defaults from the base workspace.'

        $newWorkspace = Join-Path $tempRoot 'new.code-workspace'
        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspacePath $newWorkspace -FolderPath 'src/Api', 'src/Ui' -BaseWorkspacePath $baseWorkspace | Out-Null
        $newDocument = Get-Content -Raw -LiteralPath $newWorkspace | ConvertFrom-Json -Depth 100

        Assert-Equal -Actual @($newDocument.folders).Count -Expected 2 -Message 'Workspace creation must create supplied folders.'
        Assert-Equal -Actual $newDocument.folders[0].path -Expected 'src/Api' -Message 'Workspace creation must preserve provided folder path.'
        Assert-Equal -Actual $newDocument.settings.'git.openRepositoryInParentFolders' -Expected 'never' -Message 'Workspace creation must set parent-folder discovery to never.'
        Assert-Equal -Actual $newDocument.settings.'workbench.startupEditor' -Expected 'welcomePage' -Message 'Workspace creation must default to the standard welcome page startup value.'
        Assert-Equal -Actual $newDocument.settings.'chat.emptyState.history.enabled' -Expected $true -Message 'Workspace creation must keep empty-state chat history enabled.'
        Assert-Equal -Actual $newDocument.settings.'search.exclude'.'**/bin' -Expected $true -Message 'Workspace creation must use normalized search excludes.'
        Assert-Equal -Actual @($newDocument.extensions.recommendations).Count -Expected 1 -Message 'Workspace creation must inherit extension recommendations from the base workspace.'
        Assert-True -Condition ($null -ne $newDocument.launch) -Message 'Workspace creation must carry top-level defaults from the base workspace.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    Write-Host '[OK] workspace settings sync tests passed.'
    exit 0
}
catch {
    Write-Host ("[FAIL] workspace settings sync tests failed: {0}" -f $_.Exception.Message)
    exit 1
}