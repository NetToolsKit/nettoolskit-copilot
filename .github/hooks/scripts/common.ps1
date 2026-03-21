Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Reads the JSON payload emitted by the VS Code hook runtime from STDIN.
function Read-HookInput {
    $raw = [Console]::In.ReadToEnd()
    if ([string]::IsNullOrWhiteSpace($raw)) {
        return [pscustomobject]@{}
    }

    return $raw | ConvertFrom-Json -Depth 50
}

# Resolves the workspace display name from a repository path.
function Get-WorkspaceName {
    param(
        [string] $WorkspacePath
    )

    if ([string]::IsNullOrWhiteSpace($WorkspacePath)) {
        return 'unknown-workspace'
    }

    return [System.IO.Path]::GetFileName($WorkspacePath.TrimEnd('\', '/'))
}

# Resolves the current Git branch for the workspace when Git is available.
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

# Checks whether a repository-relative path exists inside the current workspace.
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

# Safely reads an optional property from the hook payload object.
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

# Resolves the repository-owned hooks root from the current script location.
function Resolve-HooksRootPath {
    return (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
}

# Resolves the repository default insert_final_newline policy from the workspace .editorconfig.
function Get-WorkspaceInsertFinalNewlinePolicy {
    param(
        [string] $WorkspacePath
    )

    if ([string]::IsNullOrWhiteSpace($WorkspacePath)) {
        return $null
    }

    $editorConfigPath = Join-Path $WorkspacePath '.editorconfig'
    if (-not (Test-Path -LiteralPath $editorConfigPath -PathType Leaf)) {
        return $null
    }

    foreach ($line in (Get-Content -LiteralPath $editorConfigPath)) {
        $trimmed = [string] $line
        $trimmed = $trimmed.Trim()

        if ([string]::IsNullOrWhiteSpace($trimmed)) {
            continue
        }

        if ($trimmed.StartsWith('#') -or $trimmed.StartsWith(';')) {
            continue
        }

        if ($trimmed -match '^insert_final_newline\s*=\s*(true|false)$') {
            return [System.Convert]::ToBoolean($Matches[1])
        }
    }

    return $null
}

# Detects whether the current workspace provides a local Super Agent adapter surface.
function Get-SuperAgentWorkspaceSurface {
    param(
        [string] $WorkspacePath
    )

    $hasWorkspaceAgents = Test-WorkspacePath -WorkspacePath $WorkspacePath -RelativePath '.github/AGENTS.md'
    $hasWorkspaceInstructions = Test-WorkspacePath -WorkspacePath $WorkspacePath -RelativePath '.github/copilot-instructions.md'
    $hasWorkspaceRouteCatalog = Test-WorkspacePath -WorkspacePath $WorkspacePath -RelativePath '.github/instruction-routing.catalog.yml'
    $hasWorkspaceRoutePrompt = Test-WorkspacePath -WorkspacePath $WorkspacePath -RelativePath '.github/prompts/route-instructions.prompt.md'
    $hasPlanningWorkspace = Test-WorkspacePath -WorkspacePath $WorkspacePath -RelativePath 'planning/README.md'
    $hasSpecWorkspace = Test-WorkspacePath -WorkspacePath $WorkspacePath -RelativePath 'planning/specs/README.md'

    $workspaceMode = if ($hasWorkspaceAgents -and $hasWorkspaceInstructions) {
        'workspace-adapter'
    }
    else {
        'global-runtime'
    }

    $planningActivePath = if ($hasPlanningWorkspace) { 'planning/active' } else { '.build/super-agent/planning/active' }
    $planningCompletedPath = if ($hasPlanningWorkspace) { 'planning/completed' } else { '.build/super-agent/planning/completed' }
    $specActivePath = if ($hasSpecWorkspace) { 'planning/specs/active' } else { '.build/super-agent/specs/active' }
    $specCompletedPath = if ($hasSpecWorkspace) { 'planning/specs/completed' } else { '.build/super-agent/specs/completed' }

    $instructionLoadMessage = if ($workspaceMode -eq 'workspace-adapter') {
        'Load workspace .github/AGENTS.md and .github/copilot-instructions.md first.'
    }
    else {
        'No local workspace adapter detected: load runtime AGENTS.md and copilot-instructions.md from ~/.github first.'
    }

    $routingMessage = if (($workspaceMode -eq 'workspace-adapter') -and $hasWorkspaceRouteCatalog -and $hasWorkspaceRoutePrompt) {
        'Routing mode: use workspace static routing with .github/instruction-routing.catalog.yml and .github/prompts/route-instructions.prompt.md.'
    }
    elseif ($workspaceMode -eq 'workspace-adapter') {
        'Routing mode: no local static routing surface detected; build a minimal context pack from the workspace files you are touching.'
    }
    else {
        'Routing mode: global-runtime. Do not assume the runtime repository routing catalog or repository-operating-model.instructions.md applies to this workspace; build a minimal local context pack from the target repo.'
    }

    $closeoutMessage = if ($workspaceMode -eq 'workspace-adapter') {
        'Closeout uses workspace conventions, including README and CHANGELOG updates when the local repo requires them.'
    }
    else {
        'Closeout stays generic in global-runtime mode: always prepare a commit message and only update README or CHANGELOG when the target repo already uses them.'
    }

    return [pscustomobject]@{
        WorkspaceMode          = $workspaceMode
        HasWorkspaceAdapter    = ($workspaceMode -eq 'workspace-adapter')
        HasWorkspaceRouteCatalog = $hasWorkspaceRouteCatalog
        HasWorkspaceRoutePrompt = $hasWorkspaceRoutePrompt
        HasPlanningWorkspace   = $hasPlanningWorkspace
        HasSpecWorkspace       = $hasSpecWorkspace
        PlanningActivePath     = $planningActivePath
        PlanningCompletedPath  = $planningCompletedPath
        SpecActivePath         = $specActivePath
        SpecCompletedPath      = $specCompletedPath
        InstructionLoadMessage = $instructionLoadMessage
        RoutingMessage         = $routingMessage
        CloseoutMessage        = $closeoutMessage
    }
}

# Builds a workspace-aware EOF policy guidance string for startup and edit hooks.
function Get-WorkspaceEofPolicyMessage {
    param(
        [string] $WorkspacePath
    )

    $insertFinalNewline = Get-WorkspaceInsertFinalNewlinePolicy -WorkspacePath $WorkspacePath
    if ($insertFinalNewline -eq $false) {
        return 'Workspace EOF policy: preserve exact file EOF, and do not append a terminal newline because .editorconfig uses insert_final_newline = false.'
    }

    if ($insertFinalNewline -eq $true) {
        return 'Workspace EOF policy: preserve exact file EOF and keep a terminal newline where .editorconfig uses insert_final_newline = true.'
    }

    return 'Workspace EOF policy: preserve the current EOF state of touched files and do not change terminal newline behavior unless the workspace defines a narrower rule.'
}

# Returns true when the supplied path resolves inside the current workspace.
function Test-WorkspaceManagedFilePath {
    param(
        [string] $WorkspacePath,
        [string] $FilePath
    )

    if ([string]::IsNullOrWhiteSpace($WorkspacePath) -or [string]::IsNullOrWhiteSpace($FilePath)) {
        return $false
    }

    try {
        $resolvedWorkspace = [System.IO.Path]::GetFullPath($WorkspacePath)
        $resolvedFile = if ([System.IO.Path]::IsPathRooted($FilePath)) {
            [System.IO.Path]::GetFullPath($FilePath)
        }
        else {
            [System.IO.Path]::GetFullPath((Join-Path $resolvedWorkspace $FilePath))
        }
    }
    catch {
        return $false
    }

    $separator = [System.IO.Path]::DirectorySeparatorChar
    $workspacePrefix = $resolvedWorkspace.TrimEnd('\', '/') + $separator
    $comparison = if ($IsWindows) { [System.StringComparison]::OrdinalIgnoreCase } else { [System.StringComparison]::Ordinal }

    return $resolvedFile.StartsWith($workspacePrefix, $comparison)
}

# Removes terminal newline sequences from a tool text payload while preserving internal line breaks.
function Remove-TerminalNewline {
    param(
        [AllowNull()]
        [string] $Text
    )

    if ($null -eq $Text) {
        return $null
    }

    return ($Text -replace '(\r\n|\n)+$','')
}

# Clones a hook tool input payload into a mutable object for normalization.
function Copy-ToolInputObject {
    param(
        [object] $ToolInput
    )

    if ($null -eq $ToolInput) {
        return $null
    }

    return ($ToolInput | ConvertTo-Json -Depth 100 | ConvertFrom-Json -Depth 100)
}

# Normalizes supported edit-tool payloads to remove terminal newlines when the workspace policy forbids them.
function Get-EofNormalizedToolInput {
    param(
        [string] $WorkspacePath,
        [string] $ToolName,
        [object] $ToolInput
    )

    $insertFinalNewline = Get-WorkspaceInsertFinalNewlinePolicy -WorkspacePath $WorkspacePath
    if ($insertFinalNewline -ne $false) {
        return $null
    }

    $updatedInput = Copy-ToolInputObject -ToolInput $ToolInput
    if ($null -eq $updatedInput) {
        return $null
    }

    $changed = $false

    switch ($ToolName) {
        'createFile' {
            if (Test-WorkspaceManagedFilePath -WorkspacePath $WorkspacePath -FilePath ([string] $updatedInput.filePath)) {
                $normalizedContent = Remove-TerminalNewline -Text ([string] $updatedInput.content)
                if ($normalizedContent -cne [string] $updatedInput.content) {
                    $updatedInput.content = $normalizedContent
                    $changed = $true
                }
            }
        }
        'insertEdit' {
            if (Test-WorkspaceManagedFilePath -WorkspacePath $WorkspacePath -FilePath ([string] $updatedInput.filePath)) {
                $normalizedCode = Remove-TerminalNewline -Text ([string] $updatedInput.code)
                if ($normalizedCode -cne [string] $updatedInput.code) {
                    $updatedInput.code = $normalizedCode
                    $changed = $true
                }
            }
        }
        'replaceString' {
            if (Test-WorkspaceManagedFilePath -WorkspacePath $WorkspacePath -FilePath ([string] $updatedInput.filePath)) {
                $normalizedNewString = Remove-TerminalNewline -Text ([string] $updatedInput.newString)
                if ($normalizedNewString -cne [string] $updatedInput.newString) {
                    $updatedInput.newString = $normalizedNewString
                    $changed = $true
                }
            }
        }
        'multiReplaceString' {
            foreach ($replacement in @($updatedInput.replacements)) {
                if (-not (Test-WorkspaceManagedFilePath -WorkspacePath $WorkspacePath -FilePath ([string] $replacement.filePath))) {
                    continue
                }

                $normalizedNewString = Remove-TerminalNewline -Text ([string] $replacement.newString)
                if ($normalizedNewString -cne [string] $replacement.newString) {
                    $replacement.newString = $normalizedNewString
                    $changed = $true
                }
            }
        }
    }

    if (-not $changed) {
        return $null
    }

    return $updatedInput
}

# Returns true when the tool participates in repository file creation or editing.
function Test-EofSensitiveToolName {
    param(
        [string] $ToolName
    )

    return @('applyPatch', 'createFile', 'insertEdit', 'replaceString', 'multiReplaceString') -contains $ToolName
}

# Reads a JSON file when it exists and returns $null when absent.
function Read-OptionalJsonFile {
    param(
        [string] $Path
    )

    if ([string]::IsNullOrWhiteSpace($Path)) {
        return $null
    }

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        return $null
    }

    return (Get-Content -Raw -LiteralPath $Path | ConvertFrom-Json -Depth 50)
}

# Loads the shared Super Agent selector configuration from the repository hooks folder.
function Get-SuperAgentSelectorConfig {
    $hooksRoot = Resolve-HooksRootPath
    $selectorPath = Join-Path $hooksRoot 'super-agent.selector.json'
    $selector = Read-OptionalJsonFile -Path $selectorPath

    if ($null -eq $selector) {
        throw ("Missing or invalid selector configuration: {0}" -f $selectorPath)
    }

    return $selector
}

# Resolves the optional local override file path under the user ~/.github/hooks runtime.
function Resolve-SuperAgentSelectorOverridePath {
    param(
        [string] $RelativePath
    )

    if ([string]::IsNullOrWhiteSpace($RelativePath)) {
        return $null
    }

    $userProfile = $env:USERPROFILE
    if ([string]::IsNullOrWhiteSpace($userProfile)) {
        $userProfile = $HOME
    }

    if ([string]::IsNullOrWhiteSpace($userProfile)) {
        return $null
    }

    return (Join-Path (Join-Path $userProfile '.github\hooks') $RelativePath)
}

# Builds the normalized selected startup-controller record.
function New-SuperAgentSelection {
    param(
        [string] $SkillName,
        [string] $DisplayName,
        [string] $Source
    )

    if ([string]::IsNullOrWhiteSpace($SkillName)) {
        throw 'Startup controller skill name cannot be empty.'
    }

    if ([string]::IsNullOrWhiteSpace($DisplayName)) {
        $DisplayName = $SkillName
    }

    if ([string]::IsNullOrWhiteSpace($Source)) {
        $Source = 'default'
    }

    return [pscustomobject]@{
        SkillName   = [string] $SkillName
        DisplayName = [string] $DisplayName
        Source      = [string] $Source
    }
}

# Resolves the startup-controller selection using repo default, local override, then env override.
function Resolve-SuperAgentSelection {
    $selector = Get-SuperAgentSelectorConfig
    $selection = New-SuperAgentSelection `
        -SkillName ([string] $selector.defaultAgent.skillName) `
        -DisplayName ([string] $selector.defaultAgent.displayName) `
        -Source 'default'

    $overridePath = Resolve-SuperAgentSelectorOverridePath -RelativePath ([string] $selector.overrideSources.localOverrideFile)
    $localOverride = Read-OptionalJsonFile -Path $overridePath
    if (($null -ne $localOverride) -and -not [string]::IsNullOrWhiteSpace([string] $localOverride.skillName)) {
        $selection = New-SuperAgentSelection `
            -SkillName ([string] $localOverride.skillName) `
            -DisplayName ([string] $localOverride.displayName) `
            -Source 'local-override'
    }

    $envSkillVariable = [string] $selector.overrideSources.environment.skillVariable
    $envDisplayVariable = [string] $selector.overrideSources.environment.displayVariable
    $envSkillValue = if ([string]::IsNullOrWhiteSpace($envSkillVariable)) { $null } else { [Environment]::GetEnvironmentVariable($envSkillVariable) }
    $envDisplayValue = if ([string]::IsNullOrWhiteSpace($envDisplayVariable)) { $null } else { [Environment]::GetEnvironmentVariable($envDisplayVariable) }

    if (-not [string]::IsNullOrWhiteSpace($envSkillValue)) {
        $selection = New-SuperAgentSelection `
            -SkillName $envSkillValue `
            -DisplayName $envDisplayValue `
            -Source 'environment-override'
    }

    return $selection
}

# Formats the skill token consistently for bootstrap messages.
function Format-SkillToken {
    param(
        [string] $SkillName
    )

    if ([string]::IsNullOrWhiteSpace($SkillName)) {
        return '<unspecified-skill>'
    }

    return ('$' + $SkillName)
}

# Builds the SessionStart bootstrap context injected into VS Code agent sessions.
function New-SessionContextString {
    param(
        [object] $Payload
    )

    $selection = Resolve-SuperAgentSelection
    $workspacePath = [string] (Get-OptionalPayloadProperty -Payload $Payload -Name 'cwd')
    $workspaceName = Get-WorkspaceName -WorkspacePath $workspacePath
    $branchName = Get-GitBranchName -WorkspacePath $workspacePath
    $workspaceSurface = Get-SuperAgentWorkspaceSurface -WorkspacePath $workspacePath
    $eofMessage = Get-WorkspaceEofPolicyMessage -WorkspacePath $workspacePath

    $segments = New-Object System.Collections.Generic.List[string]
    $segments.Add(('Workspace: {0}' -f $workspaceName)) | Out-Null

    if (-not [string]::IsNullOrWhiteSpace($branchName)) {
        $segments.Add(('Branch: {0}' -f $branchName)) | Out-Null
    }

    $segments.Add(('Selected startup controller: {0} ({1}) via {2}.' -f $selection.DisplayName, (Format-SkillToken -SkillName $selection.SkillName), $selection.Source)) | Out-Null
    $segments.Add(('Workspace mode: {0}.' -f $workspaceSurface.WorkspaceMode)) | Out-Null
    $segments.Add([string] $workspaceSurface.InstructionLoadMessage) | Out-Null
    $segments.Add([string] $workspaceSurface.RoutingMessage) | Out-Null
    $segments.Add(('Planning root: {0} -> {1}.' -f $workspaceSurface.PlanningActivePath, $workspaceSurface.PlanningCompletedPath)) | Out-Null
    $segments.Add(('Spec root: {0} -> {1}.' -f $workspaceSurface.SpecActivePath, $workspaceSurface.SpecCompletedPath)) | Out-Null
    $segments.Add([string] $workspaceSurface.CloseoutMessage) | Out-Null
    $segments.Add('Super Agent lifecycle is mandatory for change-bearing work: intake -> planning -> spec when needed -> specialist -> test -> review -> closeout -> planning update.') | Out-Null
    $segments.Add($eofMessage) | Out-Null
    $segments.Add('Use workspace context first and official docs second.') | Out-Null
    $segments.Add('Keep non-versioned build outputs under .build/ and deployment/runtime publish outputs under .deployment/.') | Out-Null

    return ($segments -join ' ')
}

# Builds the SubagentStart bootstrap context injected into worker sessions.
function New-SubagentContextString {
    param(
        [object] $Payload
    )

    $selection = Resolve-SuperAgentSelection
    $workspacePath = [string] (Get-OptionalPayloadProperty -Payload $Payload -Name 'cwd')
    $workspaceSurface = Get-SuperAgentWorkspaceSurface -WorkspacePath $workspacePath
    $eofMessage = Get-WorkspaceEofPolicyMessage -WorkspacePath $workspacePath
    $agentType = [string] (Get-OptionalPayloadProperty -Payload $Payload -Name 'agent_type')
    if ([string]::IsNullOrWhiteSpace($agentType)) {
        $agentType = 'subagent'
    }

    return ('Startup controller: {0} ({1}) via {2}. Workspace mode: {3}. {4} {5} Planning root: {6}. Spec root: {7}. {8} Super Agent bootstrap is active. Current worker type: {9}. {10} Keep scope minimal, follow the active routing decision for this workspace, respect planning/spec artifacts, avoid write-scope conflicts, validate before completion, and return structured output only for the assigned slice.' -f $selection.DisplayName, (Format-SkillToken -SkillName $selection.SkillName), $selection.Source, $workspaceSurface.WorkspaceMode, $workspaceSurface.InstructionLoadMessage, $workspaceSurface.RoutingMessage, $workspaceSurface.PlanningActivePath, $workspaceSurface.SpecActivePath, $workspaceSurface.CloseoutMessage, $agentType, $eofMessage)
}

# Builds the PreToolUse payload used to normalize repository edit tool inputs before disk writes occur.
function New-PreToolUseResult {
    param(
        [object] $Payload
    )

    $toolName = [string] (Get-OptionalPayloadProperty -Payload $Payload -Name 'tool_name')
    $toolInput = Get-OptionalPayloadProperty -Payload $Payload -Name 'tool_input'
    $workspacePath = [string] (Get-OptionalPayloadProperty -Payload $Payload -Name 'cwd')

    $hookSpecificOutput = [ordered]@{
        hookEventName = 'PreToolUse'
    }

    if (Test-EofSensitiveToolName -ToolName $toolName) {
        $hookSpecificOutput.additionalContext = Get-WorkspaceEofPolicyMessage -WorkspacePath $workspacePath

        $updatedInput = Get-EofNormalizedToolInput -WorkspacePath $workspacePath -ToolName $toolName -ToolInput $toolInput
        if ($null -ne $updatedInput) {
            $hookSpecificOutput.updatedInput = $updatedInput
        }
    }

    return [ordered]@{
        hookSpecificOutput = $hookSpecificOutput
    }
}