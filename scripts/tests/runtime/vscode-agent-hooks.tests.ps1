<#
.SYNOPSIS
    Runtime tests for repository-owned VS Code agent hook scripts.

.DESCRIPTION
    Validates the simplified SessionStart, PreToolUse, and SubagentStart hook
    payload contracts used to bootstrap Copilot and Codex sessions inside
    VS Code.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing
    .github and .codex.

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
. $script:CommonBootstrapPath -CallerScriptRoot $PSScriptRoot -Helpers @('repository-paths')
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

# Creates and returns a disposable temporary workspace path.
function New-TemporaryWorkspacePath {
    $path = Join-Path ([System.IO.Path]::GetTempPath()) ('super-agent-workspace-' + [guid]::NewGuid().ToString('N'))
    [void] (New-Item -ItemType Directory -Path $path -Force)
    return $path
}

# Invokes a hook script with a compact JSON payload and returns the parsed result.
function Invoke-HookScript {
    param(
        [string] $ScriptPath,
        [hashtable] $Payload
    )

    $output = ($Payload | ConvertTo-Json -Depth 20 -Compress | & pwsh -NoLogo -NoProfile -File $ScriptPath)
    return ($output | ConvertFrom-Json -Depth 50)
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

$sessionResult = Invoke-HookScript -ScriptPath $sessionStartScript -Payload $sessionPayload
$preToolReplaceResult = Invoke-HookScript -ScriptPath $preToolUseScript -Payload $preToolReplacePayload
$preToolCreateResult = Invoke-HookScript -ScriptPath $preToolUseScript -Payload $preToolCreatePayload
$subagentResult = Invoke-HookScript -ScriptPath $subagentStartScript -Payload $subagentPayload

Assert-True ($sessionResult.hookSpecificOutput.hookEventName -eq 'SessionStart') 'SessionStart hook should return SessionStart payload.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match 'Selected startup controller: Super Agent \(\$super-agent\) via default') 'SessionStart hook should advertise the default startup controller.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match '\[Super Agent: ACTIVE \| controller=Super Agent \| skill=super-agent \| mode=workspace-adapter') 'SessionStart hook should expose a visible activation banner in workspace-adapter mode.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match 'first substantive assistant reply') 'SessionStart hook should require the one-time visibility confirmation in the first substantive reply.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match 'Super Agent lifecycle is mandatory') 'SessionStart hook should inject Super Agent lifecycle guidance.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match 'Planning root: planning/active -> planning/completed') 'SessionStart hook should use workspace planning roots in workspace-adapter mode.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match 'Spec root: planning/specs/active -> planning/specs/completed') 'SessionStart hook should use workspace spec roots in workspace-adapter mode.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match 'insert_final_newline = false') 'SessionStart hook should mention the repository EOF policy.'
Assert-True ([string] $sessionResult.hookSpecificOutput.additionalContext -match 'Keep non-versioned build outputs under \.build/ and deployment/runtime publish outputs under \.deployment/') 'SessionStart hook should mention the shared artifact layout policy.'

Assert-True ($preToolReplaceResult.hookSpecificOutput.hookEventName -eq 'PreToolUse') 'PreToolUse hook should return PreToolUse payload.'
Assert-True ([string] $preToolReplaceResult.hookSpecificOutput.additionalContext -match 'do not append a terminal newline') 'PreToolUse hook should remind the model about the EOF policy.'
Assert-True ([string] $preToolReplaceResult.hookSpecificOutput.updatedInput.newString -eq 'after') 'PreToolUse hook should strip a terminal newline from replaceString.newString.'
Assert-True ([string] $preToolCreateResult.hookSpecificOutput.updatedInput.content -eq "line one`nline two") 'PreToolUse hook should strip a terminal newline from createFile.content.'

Assert-True ($subagentResult.hookSpecificOutput.hookEventName -eq 'SubagentStart') 'SubagentStart hook should return SubagentStart payload.'
Assert-True ([string] $subagentResult.hookSpecificOutput.additionalContext -match 'reviewer') 'SubagentStart hook should mention the spawned worker type.'
Assert-True ([string] $subagentResult.hookSpecificOutput.additionalContext -match '\[Super Agent: ACTIVE \| controller=Super Agent \| skill=super-agent \| mode=workspace-adapter') 'SubagentStart hook should propagate the visibility banner in workspace-adapter mode.'
Assert-True ([string] $subagentResult.hookSpecificOutput.additionalContext -match 'Planning root: planning/active') 'SubagentStart hook should preserve workspace planning roots.'
Assert-True ([string] $subagentResult.hookSpecificOutput.additionalContext -match 'insert_final_newline = false') 'SubagentStart hook should preserve workspace EOF guidance.'

$globalWorkspacePath = New-TemporaryWorkspacePath

try {
    [void] (New-Item -ItemType Directory -Path (Join-Path $globalWorkspacePath 'src') -Force)
    Set-Content -LiteralPath (Join-Path $globalWorkspacePath 'README.md') -Value '# temp workspace' -NoNewline

    $globalSessionPayload = [ordered]@{
        cwd = $globalWorkspacePath
        source = 'new'
        sessionId = 'session-global'
        hookEventName = 'SessionStart'
    }

    $globalSubagentPayload = [ordered]@{
        cwd = $globalWorkspacePath
        sessionId = 'session-global'
        hookEventName = 'SubagentStart'
        agent_id = 'subagent-global'
        agent_type = 'implementer'
    }

    $globalSessionResult = Invoke-HookScript -ScriptPath $sessionStartScript -Payload $globalSessionPayload
    $globalSubagentResult = Invoke-HookScript -ScriptPath $subagentStartScript -Payload $globalSubagentPayload

    Assert-True ([string] $globalSessionResult.hookSpecificOutput.additionalContext -match 'Workspace mode: global-runtime') 'SessionStart hook should advertise global-runtime mode when no local adapter exists.'
    Assert-True ([string] $globalSessionResult.hookSpecificOutput.additionalContext -match '\[Super Agent: ACTIVE \| controller=Super Agent \| skill=super-agent \| mode=global-runtime') 'SessionStart hook should expose a visible activation banner in global-runtime mode.'
    Assert-True ([string] $globalSessionResult.hookSpecificOutput.additionalContext -match 'load runtime AGENTS\.md and copilot-instructions\.md from ~/.github first') 'SessionStart hook should fall back to runtime instructions in global-runtime mode.'
    Assert-True ([string] $globalSessionResult.hookSpecificOutput.additionalContext -match '\.build/super-agent/planning/active -> \.build/super-agent/planning/completed') 'SessionStart hook should use .build planning roots in global-runtime mode.'
    Assert-True ([string] $globalSessionResult.hookSpecificOutput.additionalContext -match '\.build/super-agent/specs/active -> \.build/super-agent/specs/completed') 'SessionStart hook should use .build spec roots in global-runtime mode.'
    Assert-True ([string] $globalSessionResult.hookSpecificOutput.additionalContext -match 'Do not assume the runtime repository routing catalog') 'SessionStart hook should block runtime repo routing assumptions in global-runtime mode.'
    Assert-True (-not ([string] $globalSessionResult.hookSpecificOutput.additionalContext -match 'insert_final_newline = false')) 'SessionStart hook should not claim insert_final_newline = false when the workspace has no matching .editorconfig rule.'

    Assert-True ([string] $globalSubagentResult.hookSpecificOutput.additionalContext -match 'Workspace mode: global-runtime') 'SubagentStart hook should propagate global-runtime mode.'
    Assert-True ([string] $globalSubagentResult.hookSpecificOutput.additionalContext -match 'implementer') 'SubagentStart hook should mention the worker type in global-runtime mode.'
    Assert-True ([string] $globalSubagentResult.hookSpecificOutput.additionalContext -match '\.build/super-agent/planning/active') 'SubagentStart hook should preserve .build planning roots in global-runtime mode.'
    Assert-True ([string] $globalSubagentResult.hookSpecificOutput.additionalContext -match '\[Super Agent: ACTIVE \| controller=Super Agent \| skill=super-agent \| mode=global-runtime') 'SubagentStart hook should propagate the visibility banner in global-runtime mode.'
}
finally {
    if (Test-Path -LiteralPath $globalWorkspacePath) {
        Remove-Item -LiteralPath $globalWorkspacePath -Recurse -Force
    }
}

try {
    [Environment]::SetEnvironmentVariable('COPILOT_SUPER_AGENT_SKILL', 'super-agent', 'Process')
    [Environment]::SetEnvironmentVariable('COPILOT_SUPER_AGENT_NAME', 'Custom Super Agent', 'Process')

    $overrideResult = Invoke-HookScript -ScriptPath $sessionStartScript -Payload $sessionPayload
    Assert-True ([string] $overrideResult.hookSpecificOutput.additionalContext -match 'Selected startup controller: Custom Super Agent \(\$super-agent\) via environment-override') 'SessionStart hook should honor environment overrides for the startup controller.'
}
finally {
    [Environment]::SetEnvironmentVariable('COPILOT_SUPER_AGENT_SKILL', $null, 'Process')
    [Environment]::SetEnvironmentVariable('COPILOT_SUPER_AGENT_NAME', $null, 'Process')
}

Write-Host 'VS Code agent hook runtime tests passed.'