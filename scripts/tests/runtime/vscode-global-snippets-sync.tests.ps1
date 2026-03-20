<#
.SYNOPSIS
    Runtime tests for global VS Code snippet synchronization without external frameworks.

.DESCRIPTION
    Covers create, update, and preserve-extra behavior for
    `sync-vscode-global-snippets.ps1`.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/vscode-global-snippets-sync.tests.ps1

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
$scriptPath = Join-Path $resolvedRepoRoot 'scripts/runtime/sync-vscode-global-snippets.ps1'

try {
    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    try {
        $workspaceVscode = Join-Path $tempRoot 'workspace-vscode'
        $globalUser = Join-Path $tempRoot 'global-user'
        $sourceSnippets = Join-Path $workspaceVscode 'snippets'
        $targetSnippets = Join-Path $globalUser 'snippets'

        Write-TextFile -Path (Join-Path $sourceSnippets 'alpha.tamplate.code-snippets') -Content @'
{
  "Alpha": {
    "prefix": "alpha",
    "body": [
      "value-a"
    ]
  }
}
'@
        Write-TextFile -Path (Join-Path $sourceSnippets 'beta.tamplate.code-snippets') -Content @'
{
  "Beta": {
    "prefix": "beta",
    "body": [
      "value-b"
    ]
  }
}
'@

        Write-TextFile -Path (Join-Path $targetSnippets 'alpha.code-snippets') -Content @'
{
  "Alpha": {
    "prefix": "alpha-old",
    "body": [
      "stale"
    ]
  }
}
'@
        Write-TextFile -Path (Join-Path $targetSnippets 'custom.code-snippets') -Content @'
{
  "Custom": {
    "prefix": "custom",
    "body": [
      "keep-me"
    ]
  }
}
'@

        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspaceVscodePath $workspaceVscode -GlobalVscodeUserPath $globalUser | Out-Null

        $alpha = Get-Content -Raw -LiteralPath (Join-Path $targetSnippets 'alpha.code-snippets')
        $beta = Get-Content -Raw -LiteralPath (Join-Path $targetSnippets 'beta.code-snippets')
        $custom = Get-Content -Raw -LiteralPath (Join-Path $targetSnippets 'custom.code-snippets')

        Assert-True -Condition ($alpha -match 'value-a') -Message 'Alpha snippet must be updated from source.'
        Assert-True -Condition ($beta -match 'value-b') -Message 'Missing source snippet must be created in global path.'
        Assert-True -Condition ($custom -match 'keep-me') -Message 'Unrelated global snippet must be preserved.'

        & $scriptPath -RepoRoot $resolvedRepoRoot -WorkspaceVscodePath $workspaceVscode -GlobalVscodeUserPath $globalUser | Out-Null
        $finalFiles = @(Get-ChildItem -LiteralPath $targetSnippets -Filter '*.code-snippets' -File)
        Assert-Equal -Actual $finalFiles.Count -Expected 3 -Message 'Second sync must remain idempotent and preserve custom snippets.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    Write-Host '[OK] vscode global snippet sync tests passed.'
    exit 0
}
catch {
    Write-Host ("[FAIL] vscode global snippet sync tests failed: {0}" -f $_.Exception.Message)
    exit 1
}