<#
.SYNOPSIS
    Runtime tests for repository-owned VS Code agent hook scripts.
#>

param(
    [string] $RepoRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Resolve-RepositoryRoot {
    param([string] $RequestedRoot)

    $candidates = @()
    if (-not [string]::IsNullOrWhiteSpace($RequestedRoot)) {
        $candidates += (Resolve-Path -LiteralPath $RequestedRoot).Path
    }
    $candidates += (Get-Location).Path

    foreach ($candidate in ($candidates | Select-Object -Unique)) {
        $current = $candidate
        for ($index = 0; $index -lt 6 -and -not [string]::IsNullOrWhiteSpace($current); $index++) {
            if ((Test-Path -LiteralPath (Join-Path $current '.github')) -and (Test-Path -LiteralPath (Join-Path $current '.codex'))) {
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

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$sessionStartScript = Join-Path $resolvedRepoRoot '.github/hooks/scripts/session-start.ps1'
$subagentStartScript = Join-Path $resolvedRepoRoot '.github/hooks/scripts/subagent-start.ps1'

$sessionPayload = [ordered]@{
    cwd = $resolvedRepoRoot
    source = 'new'
    sessionId = 'session-123'
    hookEventName = 'SessionStart'
}

$subagentPayload = [ordered]@{
    cwd = $resolvedRepoRoot
    sessionId = 'session-123'
    hookEventName = 'SubagentStart'
    agent_id = 'subagent-123'
    agent_type = 'reviewer'
}

$sessionOutput = ($sessionPayload | ConvertTo-Json -Depth 20 -Compress | & pwsh -NoLogo -NoProfile -File $sessionStartScript)
$subagentOutput = ($subagentPayload | ConvertTo-Json -Depth 20 -Compress | & pwsh -NoLogo -NoProfile -File $subagentStartScript)

$sessionResult = $sessionOutput | ConvertFrom-Json -Depth 50
$subagentResult = $subagentOutput | ConvertFrom-Json -Depth 50

Assert-True ($sessionResult.hookSpecificOutput.hookEventName -eq 'SessionStart') 'SessionStart hook should return SessionStart payload.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match 'Super Agent lifecycle is mandatory') 'SessionStart hook should inject Super Agent bootstrap context.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match '\.build/') 'SessionStart hook should mention the artifact layout policy.'
Assert-True ($subagentResult.hookSpecificOutput.hookEventName -eq 'SubagentStart') 'SubagentStart hook should return SubagentStart payload.'
Assert-True ([string] $subagentResult.hookSpecificOutput.additionalContext -match 'reviewer') 'SubagentStart hook should mention the spawned worker type.'

Write-Host 'VS Code agent hook runtime tests passed.'