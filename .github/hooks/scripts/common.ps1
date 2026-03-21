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

    $segments.Add(('Selected startup controller: {0} ({1}) via {2}.' -f $selection.DisplayName, (Format-SkillToken -SkillName $selection.SkillName), $selection.Source)) | Out-Null
    $segments.Add('Super Agent lifecycle is mandatory for change-bearing work: intake -> planning -> spec when needed -> specialist -> test -> review -> closeout -> planning update.') | Out-Null
    $segments.Add('Repository EOF policy is strict: preserve exact file EOF, and do not append a terminal newline because .editorconfig uses insert_final_newline = false by default.') | Out-Null
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

# Builds the SubagentStart bootstrap context injected into worker sessions.
function New-SubagentContextString {
    param(
        [object] $Payload
    )

    $selection = Resolve-SuperAgentSelection
    $agentType = [string] (Get-OptionalPayloadProperty -Payload $Payload -Name 'agent_type')
    if ([string]::IsNullOrWhiteSpace($agentType)) {
        $agentType = 'subagent'
    }

    return ('Startup controller: {0} ({1}) via {2}. Super Agent bootstrap is active. Current worker type: {3}. Preserve repository EOF policy exactly: do not append a terminal newline unless a file-specific rule explicitly requires it. Keep scope minimal, follow the repository routing result, respect planning/spec artifacts, avoid write-scope conflicts, validate before completion, and return structured output only for the assigned slice.' -f $selection.DisplayName, (Format-SkillToken -SkillName $selection.SkillName), $selection.Source, $agentType)
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
        $hookSpecificOutput.additionalContext = 'Repository EOF policy is strict for this workspace: preserve exact EOF state and do not append a terminal newline because .editorconfig uses insert_final_newline = false by default.'

        $updatedInput = Get-EofNormalizedToolInput -WorkspacePath $workspacePath -ToolName $toolName -ToolInput $toolInput
        if ($null -ne $updatedInput) {
            $hookSpecificOutput.updatedInput = $updatedInput
        }
    }

    return [ordered]@{
        hookSpecificOutput = $hookSpecificOutput
    }
}