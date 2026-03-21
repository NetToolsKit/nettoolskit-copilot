<#
.SYNOPSIS
    Runtime tests for repository-owned VS Code agent hook scripts.

.DESCRIPTION
    Validates the SessionStart, PreToolUse, and SubagentStart hook payload
    contracts used to bootstrap Copilot and Codex sessions inside VS Code.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/vscode-agent-hooks.tests.ps1

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

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$sessionStartScript = Join-Path $resolvedRepoRoot '.github/hooks/scripts/session-start.ps1'
$preToolUseScript = Join-Path $resolvedRepoRoot '.github/hooks/scripts/pre-tool-use.ps1'
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

$preToolReplacePayload = [ordered]@{
    cwd = $resolvedRepoRoot
    sessionId = 'session-123'
    hookEventName = 'PreToolUse'
    tool_name = 'replaceString'
    tool_use_id = 'tool-123'
    tool_input = [ordered]@{
        filePath = (Join-Path $resolvedRepoRoot 'README.md')
        oldString = 'before'
        newString = "after`r`n"
    }
}

$preToolCreatePayload = [ordered]@{
    cwd = $resolvedRepoRoot
    sessionId = 'session-123'
    hookEventName = 'PreToolUse'
    tool_name = 'createFile'
    tool_use_id = 'tool-456'
    tool_input = [ordered]@{
        filePath = (Join-Path $resolvedRepoRoot '.temp/hook-test.txt')
        content = "line one`nline two`n"
    }
}

$sessionOutput = ($sessionPayload | ConvertTo-Json -Depth 20 -Compress | & pwsh -NoLogo -NoProfile -File $sessionStartScript)
$preToolReplaceOutput = ($preToolReplacePayload | ConvertTo-Json -Depth 20 -Compress | & pwsh -NoLogo -NoProfile -File $preToolUseScript)
$preToolCreateOutput = ($preToolCreatePayload | ConvertTo-Json -Depth 20 -Compress | & pwsh -NoLogo -NoProfile -File $preToolUseScript)
$subagentOutput = ($subagentPayload | ConvertTo-Json -Depth 20 -Compress | & pwsh -NoLogo -NoProfile -File $subagentStartScript)

$sessionResult = $sessionOutput | ConvertFrom-Json -Depth 50
$preToolReplaceResult = $preToolReplaceOutput | ConvertFrom-Json -Depth 50
$preToolCreateResult = $preToolCreateOutput | ConvertFrom-Json -Depth 50
$subagentResult = $subagentOutput | ConvertFrom-Json -Depth 50

Assert-True ($sessionResult.hookSpecificOutput.hookEventName -eq 'SessionStart') 'SessionStart hook should return SessionStart payload.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match 'Super Agent lifecycle is mandatory') 'SessionStart hook should inject Super Agent bootstrap context.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match 'Selected startup controller: Super Agent \(\$super-agent\) via default') 'SessionStart hook should advertise the default startup controller.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match '\.build/') 'SessionStart hook should mention the artifact layout policy.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match 'insert_final_newline = false') 'SessionStart hook should mention the repository EOF policy.'
Assert-True ($preToolReplaceResult.hookSpecificOutput.hookEventName -eq 'PreToolUse') 'PreToolUse hook should return PreToolUse payload.'
Assert-True ([string] $preToolReplaceResult.hookSpecificOutput.additionalContext -match 'do not append a terminal newline') 'PreToolUse hook should remind the model about the EOF policy.'
Assert-True ([string] $preToolReplaceResult.hookSpecificOutput.updatedInput.newString -eq 'after') 'PreToolUse hook should strip a terminal newline from replaceString.newString.'
Assert-True ([string] $preToolCreateResult.hookSpecificOutput.updatedInput.content -eq "line one`nline two") 'PreToolUse hook should strip a terminal newline from createFile.content.'
Assert-True ($subagentResult.hookSpecificOutput.hookEventName -eq 'SubagentStart') 'SubagentStart hook should return SubagentStart payload.'
Assert-True ([string] $subagentResult.hookSpecificOutput.additionalContext -match 'reviewer') 'SubagentStart hook should mention the spawned worker type.'
Assert-True ([string] $subagentResult.hookSpecificOutput.additionalContext -match 'do not append a terminal newline') 'SubagentStart hook should mention the repository EOF policy.'

try {
    [Environment]::SetEnvironmentVariable('COPILOT_SUPER_AGENT_SKILL', 'using-super-agent', 'Process')
    [Environment]::SetEnvironmentVariable('COPILOT_SUPER_AGENT_NAME', 'Using Super Agent', 'Process')

    $overrideOutput = ($sessionPayload | ConvertTo-Json -Depth 20 -Compress | & pwsh -NoLogo -NoProfile -File $sessionStartScript)
    $overrideResult = $overrideOutput | ConvertFrom-Json -Depth 50

    Assert-True ([string] $overrideResult.hookSpecificOutput.additionalContext -match 'Selected startup controller: Using Super Agent \(\$using-super-agent\) via environment-override') 'SessionStart hook should honor environment overrides for the startup controller.'
}
finally {
    [Environment]::SetEnvironmentVariable('COPILOT_SUPER_AGENT_SKILL', $null, 'Process')
    [Environment]::SetEnvironmentVariable('COPILOT_SUPER_AGENT_NAME', $null, 'Process')
}

Write-Host 'VS Code agent hook runtime tests passed.'