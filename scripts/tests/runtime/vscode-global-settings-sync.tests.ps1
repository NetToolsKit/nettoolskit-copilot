<#
.SYNOPSIS
    Runtime tests for global VS Code settings synchronization without external frameworks.

.DESCRIPTION
    Covers rendering and idempotent synchronization for
    `sync-vscode-global-settings.ps1`.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/vscode-global-settings-sync.tests.ps1

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

# Fails the current test when the supplied value is not null.
function Assert-Null {
    param(
        [object] $Actual,
        [string] $Message
    )

    if ($null -ne $Actual) {
        throw ("{0} Actual='{1}'" -f $Message, $Actual)
    }
}

# Writes deterministic UTF-8 test content to disk.
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
$scriptPath = Join-Path $resolvedRepoRoot 'scripts/runtime/sync-vscode-global-settings.ps1'

try {
    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    try {
        $workspaceVscodePath = Join-Path $tempRoot '.vscode'
        $globalUserPath = Join-Path $tempRoot 'Code\User'
        $templatePath = Join-Path $workspaceVscodePath 'settings.tamplate.jsonc'
        $targetPath = Join-Path $globalUserPath 'settings.json'

        Write-TextFile -Path $templatePath -Content @'
{
  "workbench.startupEditor": "welcomePage",
  "chat.restoreLastPanelSession": true,
  "chat.instructionsFilesLocations": {
    "%USERPROFILE%\\.github\\": true
  }
}
'@

        Write-TextFile -Path $targetPath -Content @'
{
  "workbench.startupEditor": "welcomePage"
}
'@

        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspaceVscodePath $workspaceVscodePath -GlobalVscodeUserPath $globalUserPath -CreateBackup | Out-Null

        $targetContent = Get-Content -Raw -LiteralPath $targetPath
        Assert-True -Condition ($targetContent -match '"workbench.startupEditor": "welcomePage"') -Message 'Global settings sync must render the standard welcome page startup value.'
        Assert-True -Condition ($targetContent -match '"chat.restoreLastPanelSession": true') -Message 'Global settings sync must preserve chat restore settings.'
        Assert-True -Condition ($targetContent -notmatch '%USERPROFILE%') -Message 'Global settings sync must replace runtime placeholders.'
        Assert-True -Condition ($targetContent -match [regex]::Escape('.github')) -Message 'Global settings sync must preserve rendered instruction paths.'

        $backupFiles = @(Get-ChildItem -LiteralPath $globalUserPath -Filter 'settings.json.*.bak' -File)
        Assert-Equal -Actual $backupFiles.Count -Expected 1 -Message 'Global settings sync must create one backup when requested.'

        $beforeSecondRun = Get-Content -Raw -LiteralPath $targetPath
        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspaceVscodePath $workspaceVscodePath -GlobalVscodeUserPath $globalUserPath | Out-Null
        $afterSecondRun = Get-Content -Raw -LiteralPath $targetPath

        Assert-Equal -Actual $afterSecondRun -Expected $beforeSecondRun -Message 'Global settings sync must be idempotent when target is already aligned.'

        $repoTemplatePath = Join-Path $resolvedRepoRoot '.vscode/settings.tamplate.jsonc'
        $repoTemplate = Get-Content -Raw -LiteralPath $repoTemplatePath | ConvertFrom-Json -Depth 200
        Assert-Equal -Actual $repoTemplate.PSObject.Properties['files.insertFinalNewline'].Value -Expected $false -Message 'Shared VS Code template must preserve the repository EOF policy.'
        Assert-Null -Actual $repoTemplate.PSObject.Properties['editor.defaultFormatter'] -Message 'Shared VS Code template must not define a global default formatter.'

        foreach ($languageScope in @('[javascript]', '[typescript]', '[html]', '[css]', '[scss]', '[vue]', '[json]', '[markdown]')) {
            $scopeValue = $repoTemplate.PSObject.Properties[$languageScope].Value
            Assert-True -Condition ($null -ne $scopeValue) -Message ("Shared VS Code template must keep the language scope {0}." -f $languageScope)
            Assert-Null -Actual $scopeValue.PSObject.Properties['editor.defaultFormatter'] -Message ("Shared VS Code template must not assign a shared default formatter for {0}." -f $languageScope)
        }
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    Write-Host '[OK] VS Code global settings sync tests passed.'
    exit 0
}
catch {
    Write-Host ("[FAIL] VS Code global settings sync tests failed: {0}" -f $_.Exception.Message)
    exit 1
}