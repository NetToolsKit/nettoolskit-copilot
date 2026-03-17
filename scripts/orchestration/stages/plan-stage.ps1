<#
.SYNOPSIS
    Produces planning artifacts for the multi-agent orchestration pipeline.

.DESCRIPTION
    Generates deterministic planning artifacts and, when enabled, dispatches the
    planner agent through the local Codex CLI to produce structured work items.

.PARAMETER RepoRoot
    Repository root path.

.PARAMETER RunDirectory
    Absolute run directory for generated artifacts.

.PARAMETER TraceId
    Unique trace identifier for the pipeline run.

.PARAMETER StageId
    Current stage identifier.

.PARAMETER AgentId
    Current agent identifier.

.PARAMETER RequestPath
    Path to the request text artifact.

.PARAMETER InputArtifactManifestPath
    Path to input artifact manifest (not required in this stage).

.PARAMETER OutputArtifactManifestPath
    Path where this stage writes its output artifact manifest.

.PARAMETER AgentsManifestPath
    Agent contract manifest path.

.PARAMETER DispatchMode
    Stage dispatch mode from pipeline execution settings.

.PARAMETER PromptTemplatePath
    Prompt template path for live dispatch.

.PARAMETER ResponseSchemaPath
    JSON schema path for live dispatch.

.PARAMETER DispatchCommand
    Codex CLI command name or absolute path.

.PARAMETER ExecutionBackend
    Runtime backend selection. `codex-exec` enables live dispatch.

.PARAMETER StageStatePath
    Optional path where stage execution metadata is written.

.PARAMETER DetailedOutput
    Shows detailed diagnostics.

.EXAMPLE
    pwsh -File .\scripts\orchestration\stages\plan-stage.ps1 -RepoRoot . -RunDirectory .temp\runs\trace-001 -TraceId trace-001 -StageId plan -AgentId planner -RequestPath .temp\runs\trace-001\request.txt -OutputArtifactManifestPath .temp\runs\trace-001\stages\plan\output-artifacts.json -ExecutionBackend codex-exec -DispatchMode codex-exec

.NOTES
    Falls back to deterministic scripted planning when live planner dispatch is disabled or fails.
#>

param(
    [Parameter(Mandatory = $true)] [string] $RepoRoot,
    [Parameter(Mandatory = $true)] [string] $RunDirectory,
    [Parameter(Mandatory = $true)] [string] $TraceId,
    [Parameter(Mandatory = $true)] [string] $StageId,
    [Parameter(Mandatory = $true)] [string] $AgentId,
    [Parameter(Mandatory = $true)] [string] $RequestPath,
    [string] $InputArtifactManifestPath,
    [Parameter(Mandatory = $true)] [string] $OutputArtifactManifestPath,
    [string] $AgentsManifestPath = '.codex/orchestration/agents.manifest.json',
    [string] $DispatchMode = 'scripted',
    [string] $PromptTemplatePath,
    [string] $ResponseSchemaPath,
    [string] $DispatchCommand = 'codex',
    [string] $ExecutionBackend = 'script-only',
    [string] $StageStatePath,
    [switch] $DetailedOutput
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$script:ConsoleStylePath = Join-Path $PSScriptRoot '..\common\console-style.ps1'
if (-not (Test-Path -LiteralPath $script:ConsoleStylePath -PathType Leaf)) {
    $script:ConsoleStylePath = Join-Path $PSScriptRoot '..\..\common\console-style.ps1'
}
if (Test-Path -LiteralPath $script:ConsoleStylePath -PathType Leaf) {
    . $script:ConsoleStylePath
}
$script:IsVerboseEnabled = [bool] $DetailedOutput

# Emits verbose diagnostics only when detailed output is enabled.
function Write-VerboseLog {
    param([string] $Message)

    if ($script:IsVerboseEnabled) {
        Write-StyledOutput ("[VERBOSE] {0}" -f $Message)
    }
}

# Resolves repository-relative paths into normalized absolute paths.
function Resolve-FullPath {
    param(
        [string] $BasePath,
        [string] $Candidate
    )

    if ([string]::IsNullOrWhiteSpace($Candidate)) {
        return $null
    }

    if ([System.IO.Path]::IsPathRooted($Candidate)) {
        return [System.IO.Path]::GetFullPath($Candidate)
    }

    return [System.IO.Path]::GetFullPath((Join-Path $BasePath $Candidate))
}

# Converts an absolute repository path into a stable relative artifact path.
function Convert-ToRelativeRepoPath {
    param(
        [string] $Root,
        [string] $Path
    )

    return [System.IO.Path]::GetRelativePath($Root, $Path) -replace '\\', '/'
}

# Builds a checksum-bearing artifact descriptor for the stage manifest.
function Get-ArtifactDescriptor {
    param(
        [string] $Name,
        [string] $Path,
        [string] $Root
    )

    $hash = Get-FileHash -LiteralPath $Path -Algorithm SHA256
    return [ordered]@{
        name = $Name
        path = (Convert-ToRelativeRepoPath -Root $Root -Path $Path)
        checksum = ("sha256:{0}" -f $hash.Hash.ToLowerInvariant())
    }
}

# Reads a JSON file using a repository-wide deep parse depth.
function Read-JsonFile {
    param([string] $Path)

    return (Get-Content -Raw -LiteralPath $Path | ConvertFrom-Json -Depth 200)
}

# Writes JSON deterministically without adding an implicit trailing newline.
function Write-JsonFile {
    param(
        [string] $Path,
        [object] $Value
    )

    $directory = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($directory)) {
        New-Item -ItemType Directory -Path $directory -Force | Out-Null
    }

    Set-Content -LiteralPath $Path -Value ($Value | ConvertTo-Json -Depth 100) -Encoding UTF8 -NoNewline
}

# Expands a prompt template by replacing token placeholders with supplied values.
function Expand-Template {
    param(
        [string] $TemplateText,
        [hashtable] $Tokens
    )

    $rendered = $TemplateText
    foreach ($key in $Tokens.Keys) {
        $rendered = $rendered.Replace(("{{{0}}}" -f $key), [string] $Tokens[$key])
    }

    return $rendered
}

# Converts free-form request text into a stable plan filename slug.
function Convert-ToPlanSlug {
    param([string] $Text)

    $value = ($Text ?? '').ToLowerInvariant()
    $value = [regex]::Replace($value, '[^a-z0-9]+', '-')
    $value = $value.Trim('-')
    if ([string]::IsNullOrWhiteSpace($value)) {
        return 'planned-work'
    }

    if ($value.Length -gt 48) {
        return $value.Substring(0, 48).Trim('-')
    }

    return $value
}

# Retrieves a single agent contract from the orchestration manifest.
function Get-AgentContract {
    param(
        [string] $ManifestPath,
        [string] $TargetAgentId
    )

    $manifest = Read-JsonFile -Path $ManifestPath
    return @($manifest.agents | Where-Object { $_.id -eq $TargetAgentId } | Select-Object -First 1)
}

# Creates a deterministic fallback plan when live planner dispatch is unavailable.
function New-FallbackPlanResult {
    param(
        [string] $RequestText,
        [string[]] $AllowedPaths
    )

    return [ordered]@{
        objective = 'Deliver the requested change with deterministic validation.'
        scopeSummary = 'Fallback planning mode generated a single safe work item because live planner dispatch is disabled.'
        assumptions = @(
            'Repository context remains the primary source of truth.',
            'Execution should stay within the allowed repository paths.'
        )
        acceptanceCriteria = @(
            'Requested change is implemented with minimal safe scope.',
            'Relevant validation commands are executed before review.'
        )
        workItems = @(
            [ordered]@{
                id = 'implement-request'
                title = 'Implement requested change'
                description = $RequestText
                dependsOn = @()
                allowedPaths = $AllowedPaths
                deliverables = @(
                    'Requested code or documentation change',
                    'Validation-ready implementation log'
                )
                validationSteps = @(
                    'Run relevant targeted checks for the modified scope.',
                    'Prepare artifacts for validation and review stages.'
                )
                successCriteria = @(
                    'No unrelated files are changed.',
                    'Artifacts are ready for downstream validation.'
                )
            }
        )
        contextPaths = @(
            '.github/AGENTS.md',
            '.github/copilot-instructions.md',
            '.github/instructions/repository-operating-model.instructions.md'
        )
        validations = @(
            'Run focused validation for the changed scope.',
            'Run repository validation stage before review.'
        )
        risks = @(
            'Fallback mode may under-segment large requests.'
        )
        deliverySlices = @(
            [ordered]@{
                name = 'phase-1'
                goal = 'Single-task deterministic delivery path.'
            }
        )
    }
}

$resolvedRepoRoot = [System.IO.Path]::GetFullPath($RepoRoot)
$resolvedRunDirectory = [System.IO.Path]::GetFullPath($RunDirectory)
$resolvedRequestPath = [System.IO.Path]::GetFullPath($RequestPath)
$resolvedOutputManifestPath = [System.IO.Path]::GetFullPath($OutputArtifactManifestPath)
$resolvedAgentsManifestPath = Resolve-FullPath -BasePath $resolvedRepoRoot -Candidate $AgentsManifestPath
$resolvedPromptTemplatePath = Resolve-FullPath -BasePath $resolvedRepoRoot -Candidate $PromptTemplatePath
$resolvedResponseSchemaPath = Resolve-FullPath -BasePath $resolvedRepoRoot -Candidate $ResponseSchemaPath
$resolvedStageStatePath = if ([string]::IsNullOrWhiteSpace($StageStatePath)) { Join-Path $resolvedRunDirectory ('stages/{0}-state.json' -f $StageId) } else { Resolve-FullPath -BasePath $resolvedRepoRoot -Candidate $StageStatePath }

$stageArtifactsDirectory = Join-Path $resolvedRunDirectory 'artifacts'
$stageMetadataDirectory = Join-Path $resolvedRunDirectory ('stages/{0}' -f $StageId)
New-Item -ItemType Directory -Path $stageArtifactsDirectory -Force | Out-Null
New-Item -ItemType Directory -Path $stageMetadataDirectory -Force | Out-Null

$requestContent = if (Test-Path -LiteralPath $resolvedRequestPath -PathType Leaf) {
    (Get-Content -Raw -LiteralPath $resolvedRequestPath).Trim()
}
else {
    'No request content provided.'
}

$agent = Get-AgentContract -ManifestPath $resolvedAgentsManifestPath -TargetAgentId $AgentId
if ($null -eq $agent) {
    throw ("Agent contract not found for stage {0}: {1}" -f $StageId, $AgentId)
}

$allowedPaths = @($agent.allowedPaths | ForEach-Object { [string] $_ })
$shouldUseCodexDispatch = ($ExecutionBackend -eq 'codex-exec') -and ($DispatchMode -eq 'codex-exec')
$dispatchRecordPath = Join-Path $stageMetadataDirectory 'planner-dispatch.json'
$dispatchResultPath = Join-Path $stageMetadataDirectory 'planner-result.json'
$dispatchPromptPath = Join-Path $stageMetadataDirectory 'planner-prompt.md'
$dispatchError = $null
$planResult = $null
$backendUsed = if ($shouldUseCodexDispatch) { 'codex-exec' } else { 'scripted' }

if ($shouldUseCodexDispatch) {
    try {
        if (-not (Test-Path -LiteralPath $resolvedPromptTemplatePath -PathType Leaf)) {
            throw "Planner prompt template not found: $resolvedPromptTemplatePath"
        }

        if (-not (Test-Path -LiteralPath $resolvedResponseSchemaPath -PathType Leaf)) {
            throw "Planner response schema not found: $resolvedResponseSchemaPath"
        }

        $templateText = Get-Content -Raw -LiteralPath $resolvedPromptTemplatePath
        $renderedPrompt = Expand-Template -TemplateText $templateText -Tokens @{
            REQUEST_TEXT = $requestContent
            AGENT_ALLOWED_PATHS = (($allowedPaths | ForEach-Object { '- ' + $_ }) -join [Environment]::NewLine)
        }
        Set-Content -LiteralPath $dispatchPromptPath -Value $renderedPrompt -Encoding UTF8 -NoNewline

        $dispatchScriptPath = Join-Path $resolvedRepoRoot 'scripts/orchestration/engine/invoke-codex-dispatch.ps1'
        $dispatchParameters = @{
            RepoRoot = $resolvedRepoRoot
            WorkingDirectory = $resolvedRepoRoot
            TraceId = $TraceId
            StageId = $StageId
            AgentId = $AgentId
            PromptPath = $dispatchPromptPath
            ResponseSchemaPath = $resolvedResponseSchemaPath
            ResultPath = $dispatchResultPath
            DispatchRecordPath = $dispatchRecordPath
            CommandName = $DispatchCommand
            Model = [string] $agent.model
            DetailedOutput = [bool] $DetailedOutput
        }
        & $dispatchScriptPath @dispatchParameters
        $planResult = Read-JsonFile -Path $dispatchResultPath
    }
    catch {
        $dispatchError = $_.Exception.Message
        Write-StyledOutput ("[WARN] Planner live dispatch failed. Falling back to scripted mode. {0}" -f $dispatchError)
        $planResult = $null
        $backendUsed = 'scripted'
    }
}

if ($null -eq $planResult) {
    $planResult = New-FallbackPlanResult -RequestText $requestContent -AllowedPaths $allowedPaths
}

$taskPlanDataPath = Join-Path $stageArtifactsDirectory 'task-plan.json'
$taskPlanPath = Join-Path $stageArtifactsDirectory 'task-plan.md'
$contextPackPath = Join-Path $stageArtifactsDirectory 'context-pack.json'
$planningDirectory = Join-Path $resolvedRepoRoot '.temp/planning/plans-active'
New-Item -ItemType Directory -Path $planningDirectory -Force | Out-Null
$planSlug = Convert-ToPlanSlug -Text $requestContent
$existingActivePlan = Get-ChildItem -LiteralPath $planningDirectory -File -Filter ("plan-*-{0}.md" -f $planSlug) |
    Sort-Object LastWriteTimeUtc -Descending |
    Select-Object -First 1
$activePlanPath = if ($null -ne $existingActivePlan) {
    $existingActivePlan.FullName
}
else {
    Join-Path $planningDirectory ("plan-{0}-{1}.md" -f $TraceId, $planSlug)
}

Write-JsonFile -Path $taskPlanDataPath -Value $planResult

$taskPlanMarkdown = @(
    ('# Task Plan ({0})' -f $TraceId),
    '',
    ('- Stage: {0}' -f $StageId),
    ('- Agent: {0}' -f $AgentId),
    ('- Backend: {0}' -f $backendUsed),
    ('- GeneratedAt: {0}' -f (Get-Date).ToString('o')),
    '',
    '## Objective',
    ('- {0}' -f [string] $planResult.objective),
    '',
    '## Scope Summary',
    ('- {0}' -f [string] $planResult.scopeSummary),
    '',
    '## Acceptance Criteria'
)
$taskPlanMarkdown += @($planResult.acceptanceCriteria | ForEach-Object { '- ' + [string] $_ })
$taskPlanMarkdown += @(
    '',
    '## Work Items'
)
foreach ($workItem in @($planResult.workItems)) {
    $taskPlanMarkdown += ('### {0}: {1}' -f [string] $workItem.id, [string] $workItem.title)
    $taskPlanMarkdown += ('- Description: {0}' -f [string] $workItem.description)
    $taskPlanMarkdown += ('- Allowed paths: {0}' -f ((@($workItem.allowedPaths) | ForEach-Object { [string] $_ }) -join ', '))
    $taskPlanMarkdown += ('- Deliverables: {0}' -f ((@($workItem.deliverables) | ForEach-Object { [string] $_ }) -join '; '))
    $taskPlanMarkdown += ('- Validation: {0}' -f ((@($workItem.validationSteps) | ForEach-Object { [string] $_ }) -join '; '))
    $taskPlanMarkdown += ''
}
$taskPlanMarkdown += @(
    '## Risks'
)
$taskPlanMarkdown += @($planResult.risks | ForEach-Object { '- ' + [string] $_ })
Set-Content -LiteralPath $taskPlanPath -Value ($taskPlanMarkdown -join "`n") -Encoding UTF8 -NoNewline
Set-Content -LiteralPath $activePlanPath -Value ($taskPlanMarkdown -join "`n") -Encoding UTF8 -NoNewline

$contextPack = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    generatedAt = (Get-Date).ToString('o')
    mandatoryContext = @(
        '.github/AGENTS.md',
        '.github/copilot-instructions.md'
    )
    contextPaths = @($planResult.contextPaths | ForEach-Object { [string] $_ })
    validations = @($planResult.validations | ForEach-Object { [string] $_ })
    requestPath = Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedRequestPath
    notes = @(
        'Route before execute using instruction-routing catalog.',
        'Use repository context first and official docs only when external behavior is uncertain.'
    )
}
Write-JsonFile -Path $contextPackPath -Value $contextPack

$outputManifestDirectory = Split-Path -Parent $resolvedOutputManifestPath
if (-not [string]::IsNullOrWhiteSpace($outputManifestDirectory)) {
    New-Item -ItemType Directory -Path $outputManifestDirectory -Force | Out-Null
}

$outputManifest = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    agentId = $AgentId
    producedAt = (Get-Date).ToString('o')
    artifacts = @(
        (Get-ArtifactDescriptor -Name 'task-plan' -Path $taskPlanPath -Root $resolvedRepoRoot),
        (Get-ArtifactDescriptor -Name 'task-plan-data' -Path $taskPlanDataPath -Root $resolvedRepoRoot),
        (Get-ArtifactDescriptor -Name 'context-pack' -Path $contextPackPath -Root $resolvedRepoRoot),
        (Get-ArtifactDescriptor -Name 'active-plan' -Path $activePlanPath -Root $resolvedRepoRoot)
    )
}
Write-JsonFile -Path $resolvedOutputManifestPath -Value $outputManifest

$stageState = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    agentId = $AgentId
    backend = $backendUsed
    dispatchCount = if ($backendUsed -eq 'codex-exec') { 1 } else { 0 }
    workItemCount = @($planResult.workItems).Count
    promptTemplatePath = if ($backendUsed -eq 'codex-exec') { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedPromptTemplatePath } else { $null }
    responseSchemaPath = if ($backendUsed -eq 'codex-exec') { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedResponseSchemaPath } else { $null }
    dispatchRecordPath = if ((Test-Path -LiteralPath $dispatchRecordPath -PathType Leaf)) { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $dispatchRecordPath } else { $null }
    activePlanPath = Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $activePlanPath
    warning = $dispatchError
}
Write-JsonFile -Path $resolvedStageStatePath -Value $stageState

Write-VerboseLog ("Stage artifacts written: {0}" -f $resolvedOutputManifestPath)
exit 0