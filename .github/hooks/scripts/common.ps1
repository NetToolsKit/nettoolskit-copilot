Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Read-HookInput {
    $raw = [Console]::In.ReadToEnd()
    if ([string]::IsNullOrWhiteSpace($raw)) {
        return [pscustomobject]@{}
    }

    return $raw | ConvertFrom-Json -Depth 50
}

function Get-WorkspaceName {
    param(
        [string] $WorkspacePath
    )

    if ([string]::IsNullOrWhiteSpace($WorkspacePath)) {
        return 'unknown-workspace'
    }

    return [System.IO.Path]::GetFileName($WorkspacePath.TrimEnd('\', '/'))
}

function Get-GitBranchName {
    param(
        [string] $WorkspacePath
    )

    if ([string]::IsNullOrWhiteSpace($WorkspacePath)) {
        return $null
    }

    $gitCommand = Get-Command git -ErrorAction SilentlyContinue
    if ($null -eq $gitCommand) {
        return $null
    }

    try {
        $branch = (& git -C $WorkspacePath branch --show-current 2>$null)
        if ([string]::IsNullOrWhiteSpace($branch)) {
            return $null
        }

        return [string] $branch
    }
    catch {
        return $null
    }
}

function Test-WorkspacePath {
    param(
        [string] $WorkspacePath,
        [string] $RelativePath
    )

    if ([string]::IsNullOrWhiteSpace($WorkspacePath)) {
        return $false
    }

    $candidatePath = Join-Path $WorkspacePath $RelativePath
    return (Test-Path -LiteralPath $candidatePath)
}

function Get-OptionalPayloadProperty {
    param(
        [object] $Payload,
        [string] $Name
    )

    if ($null -eq $Payload) {
        return $null
    }

    $property = $Payload.PSObject.Properties[$Name]
    if ($null -eq $property) {
        return $null
    }

    return $property.Value
}

function New-SessionContextString {
    param(
        [object] $Payload
    )

    $workspacePath = [string] (Get-OptionalPayloadProperty -Payload $Payload -Name 'cwd')
    $workspaceName = Get-WorkspaceName -WorkspacePath $workspacePath
    $branchName = Get-GitBranchName -WorkspacePath $workspacePath
    $hasWorkspaceAgents = Test-WorkspacePath -WorkspacePath $workspacePath -RelativePath '.github/AGENTS.md'
    $hasWorkspaceInstructions = Test-WorkspacePath -WorkspacePath $workspacePath -RelativePath '.github/copilot-instructions.md'
    $hasPlanningWorkspace = Test-WorkspacePath -WorkspacePath $workspacePath -RelativePath 'planning/README.md'
    $hasSpecWorkspace = Test-WorkspacePath -WorkspacePath $workspacePath -RelativePath 'planning/specs/README.md'

    $segments = New-Object System.Collections.Generic.List[string]
    $segments.Add(('Workspace: {0}' -f $workspaceName)) | Out-Null

    if (-not [string]::IsNullOrWhiteSpace($branchName)) {
        $segments.Add(('Branch: {0}' -f $branchName)) | Out-Null
    }

    if ($hasWorkspaceAgents -and $hasWorkspaceInstructions) {
        $segments.Add('Load workspace .github/AGENTS.md and .github/copilot-instructions.md first.') | Out-Null
    }
    else {
        $segments.Add('Load runtime AGENTS.md and copilot-instructions.md from ~/.github first.') | Out-Null
    }

    $segments.Add('Super Agent lifecycle is mandatory for change-bearing work: intake -> planning -> spec when needed -> specialist -> test -> review -> closeout -> planning update.') | Out-Null
    $segments.Add('Use repository context first and official docs second.') | Out-Null
    $segments.Add('Keep non-versioned build outputs under .build/ and deployment/runtime publish outputs under .deployment/.') | Out-Null

    if ($hasPlanningWorkspace) {
        $segments.Add('Planning workspace detected: use planning/active and planning/completed for versioned plans.') | Out-Null
    }

    if ($hasSpecWorkspace) {
        $segments.Add('Specification workspace detected: use planning/specs/active before planning for non-trivial design-bearing work.') | Out-Null
    }

    return ($segments -join ' ')
}

function New-SubagentContextString {
    param(
        [object] $Payload
    )

    $agentType = [string] (Get-OptionalPayloadProperty -Payload $Payload -Name 'agent_type')
    if ([string]::IsNullOrWhiteSpace($agentType)) {
        $agentType = 'subagent'
    }

    return ('Super Agent bootstrap is active. Current worker type: {0}. Keep scope minimal, follow the repository routing result, respect planning/spec artifacts, avoid write-scope conflicts, validate before completion, and return structured output only for the assigned slice.' -f $agentType)
}