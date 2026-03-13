<#
.SYNOPSIS
    Runtime tests for instruction architecture validation without external frameworks.

.DESCRIPTION
    Covers success, failure, and warning-only ownership-marker behavior for
    `validate-instruction-architecture.ps1`.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/instruction-architecture.tests.ps1

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
$scriptPath = Join-Path $resolvedRepoRoot 'scripts/validation/validate-instruction-architecture.ps1'

try {
    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    try {
        & $scriptPath -RepoRoot $resolvedRepoRoot -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 0 -Message 'Repository instruction architecture should pass.'

        $invalidManifestPath = Join-Path $tempRoot 'instruction-ownership.manifest.json'
        Write-TextFile -Path $invalidManifestPath -Content @'
{
  "version": 1,
  "layers": [
    {
      "id": "prompts",
      "description": "invalid test manifest",
      "pathPatterns": [".github/prompts/*"]
    }
  ]
}
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -ManifestPath $invalidManifestPath -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Manifest missing required layers should fail.'
        Remove-Item -LiteralPath $invalidManifestPath -Force

        $invalidAgentsPath = Join-Path $tempRoot 'AGENTS.md'
        Write-TextFile -Path $invalidAgentsPath -Content @'
# Temporary AGENTS

- This file intentionally omits repository-operating-model reference.
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -AgentsPath $invalidAgentsPath -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Missing global core architecture reference should fail.'
        Remove-Item -LiteralPath $invalidAgentsPath -Force

        $skillRoot = Join-Path $tempRoot 'skills'
        $skillPath = Join-Path $skillRoot 'sample\SKILL.md'
        Write-TextFile -Path $skillPath -Content @'
---
name: sample-skill
description: temporary skill without canonical repo-operating reference
---

# Sample Skill

Load `.github/AGENTS.md` and `.github/copilot-instructions.md`.
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -SkillRoot $skillRoot -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Missing skill canonical repository-operating reference should fail.'

        $promptRoot = Join-Path $tempRoot 'prompts'
        $promptPath = Join-Path $promptRoot 'ownership.prompt.md'
        Write-TextFile -Path $promptPath -Content @'
# Temporary prompt

This prompt claims to be the single source of truth for the whole repository.
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -PromptRoot $promptRoot -WarningOnly:$false -DetailedOutput | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 0 -Message 'Prompt ownership markers should warn but not fail.'

        $routePromptPath = Join-Path $tempRoot 'route-instructions.prompt.md'
        Write-TextFile -Path $routePromptPath -Content @'
---
description: Temporary route prompt
mode: ask
tools: ['readFile']
---

# Route Instructions

Use the routing catalog and return JSON.
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -RoutePromptPath $routePromptPath -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Route prompt without deterministic hard cap should fail.'
        Remove-Item -LiteralPath $routePromptPath -Force

        $templateRoot = Join-Path $tempRoot 'templates'
        $templatePath = Join-Path $templateRoot 'settings.tamplate.jsonc'
        Write-TextFile -Path $templatePath -Content @'
{
  "//": "temporary template that wrongly claims to be the single source of truth"
}
'@
        & $scriptPath -RepoRoot $resolvedRepoRoot -TemplateRoot $templateRoot -WarningOnly:$false | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 0 -Message 'Template ownership markers should warn but not fail.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force
        }
    }
}
finally {
    Set-Location -Path $resolvedRepoRoot
}