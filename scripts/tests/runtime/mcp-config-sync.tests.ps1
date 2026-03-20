<#
.SYNOPSIS
    Runtime regression tests for mixed MCP manifest rendering.

.DESCRIPTION
    Verifies that `.codex/scripts/sync-mcp-to-codex-config.ps1` can render a
    mixed manifest containing `stdio` and `http` servers without requiring
    every optional property to exist on every server entry.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/mcp-config-sync.tests.ps1

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

function Assert-ContainsText {
    param(
        [string] $Content,
        [string] $ExpectedText,
        [string] $Message
    )

    if (-not $Content.Contains($ExpectedText)) {
        throw $Message
    }
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$scriptPath = Join-Path $resolvedRepoRoot '.codex/scripts/sync-mcp-to-codex-config.ps1'
$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
$manifestPath = Join-Path $tempRoot 'servers.manifest.json'
$configPath = Join-Path $tempRoot 'config.toml'

try {
    New-Item -ItemType Directory -Path $tempRoot -Force | Out-Null

    @'
{
  "version": 1,
  "servers": [
    {
      "name": "playwright",
      "type": "stdio",
      "command": "npx",
      "args": ["@playwright/mcp@latest"]
    },
    {
      "name": "microsoftdocs",
      "type": "http",
      "url": "https://learn.microsoft.com/api/mcp"
    }
  ]
}
'@ | Set-Content -LiteralPath $manifestPath

    @'
model = "gpt-5"

[tools]
search = true
'@ | Set-Content -LiteralPath $configPath

    & $scriptPath -ManifestPath $manifestPath -TargetConfigPath $configPath -CreateBackup | Out-Null
    $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
    Assert-True -Condition ($exitCode -eq 0) -Message 'sync-mcp-to-codex-config should succeed for mixed stdio/http manifests.'

    $content = Get-Content -LiteralPath $configPath -Raw
    Assert-ContainsText -Content $content -ExpectedText '[mcp_servers.playwright]' -Message 'Rendered config must include the stdio MCP server block.'
    Assert-ContainsText -Content $content -ExpectedText 'command = "npx"' -Message 'Rendered config must include the stdio command.'
    Assert-ContainsText -Content $content -ExpectedText '[mcp_servers.microsoftdocs]' -Message 'Rendered config must include the http MCP server block.'
    Assert-ContainsText -Content $content -ExpectedText 'url = "https://learn.microsoft.com/api/mcp"' -Message 'Rendered config must include the http URL.'

    $backupFiles = @(Get-ChildItem -LiteralPath $tempRoot -Filter 'config.toml.bak.*' -File)
    Assert-True -Condition ($backupFiles.Count -eq 1) -Message 'sync-mcp-to-codex-config should create one backup when -CreateBackup is specified.'

    Write-Host '[OK] MCP config sync tests passed.'
    exit 0
}
catch {
    Write-Host ("[FAIL] MCP config sync tests failed: {0}" -f $_.Exception.Message)
    exit 1
}
finally {
    if (Test-Path -LiteralPath $tempRoot) {
        Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
    }
}