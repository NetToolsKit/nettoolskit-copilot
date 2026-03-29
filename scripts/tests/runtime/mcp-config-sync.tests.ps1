<#
.SYNOPSIS
    Runtime regression tests for mixed MCP catalog rendering and native
    VS Code template application.

.DESCRIPTION
    Verifies that the shared MCP catalog helpers can render a mixed catalog
    containing `stdio` and `http` servers and that the native runtime command
    can apply the resulting VS Code templates without any retired wrapper path.

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
. $script:CommonBootstrapPath -CallerScriptRoot $PSScriptRoot -Helpers @('repository-paths', 'runtime-paths')
. "$PSScriptRoot\..\..\common\mcp-runtime-catalog.ps1"
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

# Fails the current test when the text does not contain the expected fragment.
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
$runtimeBinaryPath = Resolve-NtkRuntimeBinaryPath -ResolvedRepoRoot $resolvedRepoRoot -RuntimePreference github
$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
$workspaceRoot = Join-Path $tempRoot 'repo'
$vscodeRoot = Join-Path $workspaceRoot '.vscode'
$githubRoot = Join-Path $workspaceRoot '.github'
$codexRoot = Join-Path $workspaceRoot '.codex'

try {
    New-Item -ItemType Directory -Path $vscodeRoot -Force | Out-Null
    New-Item -ItemType Directory -Path $githubRoot -Force | Out-Null
    New-Item -ItemType Directory -Path $codexRoot -Force | Out-Null

    $mixedCatalog = [pscustomobject]@{
        inputs = @(
            [pscustomobject]@{
                id = 'github-token'
                type = 'promptString'
                description = 'GitHub token'
                password = $true
            }
        )
        servers = @(
            [pscustomobject]@{
                id = 'microsoftdocs/mcp'
                codexName = 'microsoftdocs'
                targets = [pscustomobject]@{
                    vscode = [pscustomobject]@{
                        include = $true
                        enabledByDefault = $true
                    }
                    codex = [pscustomobject]@{
                        include = $true
                    }
                }
                definition = [pscustomobject]@{
                    type = 'http'
                    url = 'https://learn.microsoft.com/api/mcp'
                }
            }
            [pscustomobject]@{
                id = 'microsoft/playwright-mcp'
                codexName = 'playwright'
                targets = [pscustomobject]@{
                    vscode = [pscustomobject]@{
                        include = $true
                        enabledByDefault = $false
                    }
                    codex = [pscustomobject]@{
                        include = $true
                    }
                }
                definition = [pscustomobject]@{
                    type = 'stdio'
                    command = 'npx'
                    args = @('@playwright/mcp@latest')
                }
            }
            [pscustomobject]@{
                id = 'vscode-only/example'
                targets = [pscustomobject]@{
                    vscode = [pscustomobject]@{
                        include = $true
                        enabledByDefault = $false
                    }
                }
                definition = [pscustomobject]@{
                    type = 'http'
                    url = 'https://example.invalid/vscode-only'
                }
            }
        )
    }

    $catalogPath = Join-Path $githubRoot 'governance\mcp-runtime.catalog.json'
    $manifestPath = Join-Path $codexRoot 'mcp\servers.manifest.json'
    $targetConfigPath = Join-Path $codexRoot 'config.toml'
    New-Item -ItemType Directory -Path (Split-Path -Path $catalogPath -Parent) -Force | Out-Null
    New-Item -ItemType Directory -Path (Split-Path -Path $manifestPath -Parent) -Force | Out-Null
    $mixedCatalog | ConvertTo-Json -Depth 100 | Set-Content -LiteralPath $catalogPath -Encoding UTF8 -NoNewline
    Set-Content -LiteralPath (Join-Path $vscodeRoot 'settings.tamplate.jsonc') -Value '{ "editor.tabSize": 4 }' -Encoding UTF8 -NoNewline
    Set-Content -LiteralPath $targetConfigPath -Value "# local codex config`n[features]`nexperimental_windows_sandbox = true" -Encoding UTF8 -NoNewline

    & $runtimeBinaryPath runtime render-vscode-mcp-template --repo-root $workspaceRoot --catalog-path $catalogPath | Out-Null
    $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
    Assert-True -Condition ($exitCode -eq 0) -Message 'runtime render-vscode-mcp-template should succeed for a mixed MCP catalog.'

    & $runtimeBinaryPath runtime render-mcp-runtime-artifacts --repo-root $workspaceRoot --catalog-path $catalogPath --vscode-output-path (Join-Path $vscodeRoot 'mcp.tamplate.jsonc') --codex-output-path $manifestPath | Out-Null
    $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
    Assert-True -Condition ($exitCode -eq 0) -Message 'runtime render-mcp-runtime-artifacts should succeed for a mixed MCP catalog.'

    & $runtimeBinaryPath runtime sync-codex-mcp-config --repo-root $workspaceRoot --catalog-path $catalogPath --target-config-path $targetConfigPath --create-backup | Out-Null
    $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
    Assert-True -Condition ($exitCode -eq 0) -Message 'runtime sync-codex-mcp-config should succeed for a mixed MCP catalog.'

    & $runtimeBinaryPath runtime apply-vscode-templates --repo-root $workspaceRoot | Out-Null
    $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
    Assert-True -Condition ($exitCode -eq 0) -Message 'runtime apply-vscode-templates should succeed for rendered MCP templates.'

    $renderedSettings = Join-Path $vscodeRoot 'settings.json'
    $renderedMcp = Join-Path $vscodeRoot 'mcp.json'
    Assert-True -Condition (Test-Path -LiteralPath $renderedSettings -PathType Leaf) -Message 'runtime apply-vscode-templates did not write settings.json.'
    Assert-True -Condition (Test-Path -LiteralPath $renderedMcp -PathType Leaf) -Message 'runtime apply-vscode-templates did not write mcp.json.'
    Assert-True -Condition (Test-Path -LiteralPath $manifestPath -PathType Leaf) -Message 'runtime render-mcp-runtime-artifacts did not write the Codex manifest.'
    Assert-True -Condition (@(Get-ChildItem -LiteralPath $codexRoot -Filter 'config.toml.bak.*' -File).Count -eq 1) -Message 'runtime sync-codex-mcp-config should create one backup when --create-backup is used.'

    $content = Get-Content -LiteralPath $renderedMcp -Raw
    $manifestContent = Get-Content -LiteralPath $manifestPath -Raw
    $configContent = Get-Content -LiteralPath $targetConfigPath -Raw
    Assert-ContainsText -Content $content -ExpectedText 'microsoftdocs' -Message 'Rendered MCP output must include the HTTP server.'
    Assert-ContainsText -Content $content -ExpectedText 'playwright' -Message 'Rendered MCP output must include the stdio server.'
    Assert-ContainsText -Content $content -ExpectedText 'disabled' -Message 'Rendered MCP output must preserve disabled-by-default server state.'
    Assert-ContainsText -Content $manifestContent -ExpectedText 'microsoftdocs' -Message 'Rendered Codex manifest must include the HTTP server.'
    Assert-ContainsText -Content $manifestContent -ExpectedText 'playwright' -Message 'Rendered Codex manifest must include the stdio server.'
    Assert-ContainsText -Content $configContent -ExpectedText '[mcp_servers.microsoftdocs]' -Message 'Codex config must include the HTTP server block.'
    Assert-ContainsText -Content $configContent -ExpectedText '[mcp_servers.playwright]' -Message 'Codex config must include the stdio server block.'

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