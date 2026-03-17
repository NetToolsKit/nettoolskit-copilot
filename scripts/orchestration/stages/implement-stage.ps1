<#
.SYNOPSIS
    Produces implementation artifacts for the multi-agent orchestration pipeline.

.DESCRIPTION
    Executes planned work items sequentially. When live dispatch is enabled,
    each work item is delegated to the local Codex CLI as an isolated task run.

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
    Path to input artifact manifest.

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
    pwsh -File .\scripts\orchestration\stages\implement-stage.ps1 -RepoRoot . -RunDirectory .temp\runs\trace-001 -TraceId trace-001 -StageId implement -AgentId executor -RequestPath .temp\runs\trace-001\request.txt -InputArtifactManifestPath .temp\runs\trace-001\stages\plan\output-artifacts.json -OutputArtifactManifestPath .temp\runs\trace-001\stages\implement\output-artifacts.json -ExecutionBackend codex-exec -DispatchMode codex-exec

.NOTES
    Enforces both agent-level and task-level allowed-path constraints before accepting changed files.
#>

param(
    [Parameter(Mandatory = $true)] [string] $RepoRoot,
    [Parameter(Mandatory = $true)] [string] $RunDirectory,
    [Parameter(Mandatory = $true)] [string] $TraceId,
    [Parameter(Mandatory = $true)] [string] $StageId,
    [Parameter(Mandatory = $true)] [string] $AgentId,
    [Parameter(Mandatory = $true)] [string] $RequestPath,
    [Parameter(Mandatory = $true)] [string] $InputArtifactManifestPath,
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

# Retrieves a single agent contract from the orchestration manifest.
function Get-AgentContract {
    param(
        [string] $ManifestPath,
        [string] $TargetAgentId
    )

    $manifest = Read-JsonFile -Path $ManifestPath
    return @($manifest.agents | Where-Object { $_.id -eq $TargetAgentId } | Select-Object -First 1)
}

# Converts a simple glob expression into a regex used for allowed-path checks.
function Convert-GlobToRegex {
    param([string] $Pattern)

    $normalized = ($Pattern -replace '\\', '/')
    $escaped = [regex]::Escape($normalized)
    $escaped = $escaped.Replace('\*\*', '.*')
    $escaped = $escaped.Replace('\*', '[^/]*')
    $escaped = $escaped.Replace('\?', '.')
    return ('^{0}$' -f $escaped)
}

# Validates whether a repository-relative path is allowed by at least one pattern.
function Test-IsPathAllowed {
    param(
        [string] $RelativePath,
        [string[]] $AllowedPatterns
    )

    foreach ($pattern in $AllowedPatterns) {
        $regex = Convert-GlobToRegex -Pattern $pattern
        if ($RelativePath -match $regex) {
            return $true
        }
    }

    return $false
}

# Captures current working-tree paths from git status for changed-file detection.
function Get-WorkingTreePathSet {
    param([string] $Root)

    $set = New-Object System.Collections.Generic.HashSet[string]([System.StringComparer]::OrdinalIgnoreCase)
    $statusLines = @(git -C "$Root" status --porcelain 2>$null)
    if ($LASTEXITCODE -ne 0) {
        return $set
    }

    foreach ($line in $statusLines) {
        if ([string]::IsNullOrWhiteSpace($line) -or $line.Length -lt 4) {
            continue
        }

        $pathText = $line.Substring(3).Trim()
        if ([string]::IsNullOrWhiteSpace($pathText)) {
            continue
        }

        if ($pathText.Contains('->')) {
            $pathText = $pathText.Split('->')[-1].Trim()
        }

        $set.Add(($pathText -replace '\\', '/')) | Out-Null
    }

    return $set
}

# Converts an artifact manifest into a name-to-absolute-path lookup table.
function Convert-ArtifactManifestToMap {
    param(
        [object] $Manifest,
        [string] $Root
    )

    $map = @{}
    foreach ($artifact in @($Manifest.artifacts)) {
        $name = [string] $artifact.name
        $path = [string] $artifact.path
        if ([string]::IsNullOrWhiteSpace($name) -or [string]::IsNullOrWhiteSpace($path)) {
            continue
        }

        $map[$name] = Resolve-FullPath -BasePath $Root -Candidate $path
    }

    return $map
}

# Creates a deterministic fallback task result when live executor dispatch is unavailable.
function New-FallbackTaskResult {
    param([string] $TaskId)

    return [ordered]@{
        taskId = $TaskId
        status = 'completed'
        summary = 'Fallback execution mode produced orchestration artifacts without live Codex dispatch.'
        changedFiles = @()
        validationsPerformed = @('Defer concrete validation to the validate stage.')
        residualRisks = @('No live executor dispatch occurred in script-only mode.')
        notes = @('Use -ExecutionBackend codex-exec to enable live task execution.')
        commitReady = $false
    }
}

$resolvedRepoRoot = [System.IO.Path]::GetFullPath($RepoRoot)
$resolvedRunDirectory = [System.IO.Path]::GetFullPath($RunDirectory)
$resolvedRequestPath = [System.IO.Path]::GetFullPath($RequestPath)
$resolvedInputManifestPath = [System.IO.Path]::GetFullPath($InputArtifactManifestPath)
$resolvedOutputManifestPath = [System.IO.Path]::GetFullPath($OutputArtifactManifestPath)
$resolvedAgentsManifestPath = Resolve-FullPath -BasePath $resolvedRepoRoot -Candidate $AgentsManifestPath
$resolvedPromptTemplatePath = Resolve-FullPath -BasePath $resolvedRepoRoot -Candidate $PromptTemplatePath
$resolvedResponseSchemaPath = Resolve-FullPath -BasePath $resolvedRepoRoot -Candidate $ResponseSchemaPath
$resolvedStageStatePath = if ([string]::IsNullOrWhiteSpace($StageStatePath)) { Join-Path $resolvedRunDirectory ('stages/{0}-state.json' -f $StageId) } else { Resolve-FullPath -BasePath $resolvedRepoRoot -Candidate $StageStatePath }

$stageArtifactsDirectory = Join-Path $resolvedRunDirectory 'artifacts'
$stageMetadataDirectory = Join-Path $resolvedRunDirectory ('stages/{0}' -f $StageId)
$tasksDirectory = Join-Path $stageMetadataDirectory 'tasks'
New-Item -ItemType Directory -Path $stageArtifactsDirectory -Force | Out-Null
New-Item -ItemType Directory -Path $tasksDirectory -Force | Out-Null

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

$inputManifest = Read-JsonFile -Path $resolvedInputManifestPath
$artifactMap = Convert-ArtifactManifestToMap -Manifest $inputManifest -Root $resolvedRepoRoot
$taskPlanDataPath = if ($artifactMap.ContainsKey('task-plan-data')) { [string] $artifactMap['task-plan-data'] } else { $null }
$contextPackPath = if ($artifactMap.ContainsKey('context-pack')) { [string] $artifactMap['context-pack'] } else { $null }
$routeSelectionPath = if ($artifactMap.ContainsKey('route-selection')) { [string] $artifactMap['route-selection'] } else { $null }
$specialistContextPackPath = if ($artifactMap.ContainsKey('specialist-context-pack')) { [string] $artifactMap['specialist-context-pack'] } else { $null }

$taskPlanData = if ($null -ne $taskPlanDataPath -and (Test-Path -LiteralPath $taskPlanDataPath -PathType Leaf)) { Read-JsonFile -Path $taskPlanDataPath } else { $null }
$contextPackJson = if ($null -ne $contextPackPath -and (Test-Path -LiteralPath $contextPackPath -PathType Leaf)) { Get-Content -Raw -LiteralPath $contextPackPath } else { '{}' }
$routeSelectionJson = if ($null -ne $routeSelectionPath -and (Test-Path -LiteralPath $routeSelectionPath -PathType Leaf)) { Get-Content -Raw -LiteralPath $routeSelectionPath } else { '{}' }
$specialistContextPackJson = if ($null -ne $specialistContextPackPath -and (Test-Path -LiteralPath $specialistContextPackPath -PathType Leaf)) { Get-Content -Raw -LiteralPath $specialistContextPackPath } else { $contextPackJson }
$taskPlanSummary = if ($null -ne $taskPlanData) { [string] $taskPlanData.scopeSummary } else { 'No structured task plan available.' }
$agentAllowedPaths = @($agent.allowedPaths | ForEach-Object { [string] $_ })
$shouldUseCodexDispatch = ($ExecutionBackend -eq 'codex-exec') -and ($DispatchMode -eq 'codex-exec') -and ($null -ne $taskPlanData)
$backendUsed = if ($shouldUseCodexDispatch) { 'codex-exec' } else { 'scripted' }

$taskRuns = New-Object System.Collections.Generic.List[object]
$allChangedFiles = New-Object System.Collections.Generic.HashSet[string]([System.StringComparer]::OrdinalIgnoreCase)
$dispatchErrors = New-Object System.Collections.Generic.List[string]

$workItems = if ($null -ne $taskPlanData) { @($taskPlanData.workItems) } else { @() }
if ($workItems.Count -eq 0) {
    $workItems = @(
        [pscustomobject]@{
            id = 'implement-request'
            title = 'Implement requested change'
            description = $requestContent
            allowedPaths = $agentAllowedPaths
            deliverables = @('Requested change')
            validationSteps = @('Run downstream validation stage.')
            successCriteria = @('No unrelated files are changed.')
            dependsOn = @()
        }
    )
}

$templateText = if ($shouldUseCodexDispatch) { Get-Content -Raw -LiteralPath $resolvedPromptTemplatePath } else { $null }
$dispatchScriptPath = Join-Path $resolvedRepoRoot 'scripts/orchestration/engine/invoke-codex-dispatch.ps1'

foreach ($workItem in $workItems) {
    $taskId = [string] $workItem.id
    $taskAllowedPaths = @($workItem.allowedPaths | ForEach-Object { [string] $_ })
    if ($taskAllowedPaths.Count -eq 0) {
        $taskAllowedPaths = $agentAllowedPaths
    }

    $taskDirectory = Join-Path $tasksDirectory $taskId
    New-Item -ItemType Directory -Path $taskDirectory -Force | Out-Null
    $taskStartedAt = Get-Date
    $taskResult = $null
    $newPaths = @()
    $reportedChangedFiles = @()
    $dispatchRecordRelativePath = $null
    $taskWarning = $null

    if ($shouldUseCodexDispatch) {
        try {
            $workItemJson = ($workItem | ConvertTo-Json -Depth 50)
            $renderedPrompt = Expand-Template -TemplateText $templateText -Tokens @{
                REQUEST_TEXT = $requestContent
                TASK_PLAN_SUMMARY = $taskPlanSummary
                CONTEXT_PACK_JSON = $contextPackJson
                ROUTE_SELECTION_JSON = $routeSelectionJson
                SPECIALIST_CONTEXT_PACK_JSON = $specialistContextPackJson
                WORK_ITEM_JSON = $workItemJson
                COMBINED_ALLOWED_PATHS = (($taskAllowedPaths | ForEach-Object { '- ' + $_ }) -join [Environment]::NewLine)
            }

            $dispatchPromptPath = Join-Path $taskDirectory 'executor-prompt.md'
            $dispatchResultPath = Join-Path $taskDirectory 'executor-result.json'
            $dispatchRecordPath = Join-Path $taskDirectory 'executor-dispatch.json'
            Set-Content -LiteralPath $dispatchPromptPath -Value $renderedPrompt -Encoding UTF8 -NoNewline

            $beforeSet = Get-WorkingTreePathSet -Root $resolvedRepoRoot
            $dispatchParams = @{
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
            & $dispatchScriptPath @dispatchParams
            $taskResult = Read-JsonFile -Path $dispatchResultPath
            $afterSet = Get-WorkingTreePathSet -Root $resolvedRepoRoot
            foreach ($pathAfter in $afterSet) {
                if (-not $beforeSet.Contains($pathAfter)) {
                    $newPaths += $pathAfter
                }
            }

            $reportedChangedFiles = @($taskResult.changedFiles | ForEach-Object { [string] $_ -replace '\\', '/' })
            $candidateChangedFiles = New-Object System.Collections.Generic.HashSet[string]([System.StringComparer]::OrdinalIgnoreCase)
            foreach ($path in $reportedChangedFiles + $newPaths) {
                if (-not [string]::IsNullOrWhiteSpace($path)) {
                    $candidateChangedFiles.Add($path) | Out-Null
                }
            }

            foreach ($candidatePath in $candidateChangedFiles) {
                if (-not (Test-IsPathAllowed -RelativePath $candidatePath -AllowedPatterns $agentAllowedPaths)) {
                    throw "Task $taskId changed path outside agent contract: $candidatePath"
                }

                if (-not (Test-IsPathAllowed -RelativePath $candidatePath -AllowedPatterns $taskAllowedPaths)) {
                    throw "Task $taskId changed path outside task scope: $candidatePath"
                }
            }

            $taskResult.changedFiles = @($candidateChangedFiles)
            $dispatchRecordRelativePath = Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $dispatchRecordPath
        }
        catch {
            $taskWarning = $_.Exception.Message
            $dispatchErrors.Add($taskWarning) | Out-Null
            $taskResult = [pscustomobject](New-FallbackTaskResult -TaskId $taskId)
            $taskResult.status = 'blocked'
            $taskResult.notes += $taskWarning
        }
    }
    else {
        $taskResult = [pscustomobject](New-FallbackTaskResult -TaskId $taskId)
    }

    foreach ($changedFile in @($taskResult.changedFiles)) {
        $allChangedFiles.Add([string] $changedFile) | Out-Null
    }

    $taskFinishedAt = Get-Date
    $taskRuns.Add([pscustomobject]@{
        taskId = $taskId
        title = [string] $workItem.title
        status = [string] $taskResult.status
        summary = [string] $taskResult.summary
        changedFiles = @($taskResult.changedFiles | ForEach-Object { [string] $_ })
        validationsPerformed = @($taskResult.validationsPerformed | ForEach-Object { [string] $_ })
        residualRisks = @($taskResult.residualRisks | ForEach-Object { [string] $_ })
        notes = @($taskResult.notes | ForEach-Object { [string] $_ })
        commitReady = [bool] $taskResult.commitReady
        startedAt = $taskStartedAt.ToString('o')
        finishedAt = $taskFinishedAt.ToString('o')
        durationMs = [int] ($taskFinishedAt - $taskStartedAt).TotalMilliseconds
        dispatchRecordPath = $dispatchRecordRelativePath
        warning = $taskWarning
    }) | Out-Null
}

$failedTasks = @($taskRuns | Where-Object { $_.status -ne 'completed' })
$changesetPath = Join-Path $stageArtifactsDirectory 'changeset.json'
$implementationLogPath = Join-Path $stageArtifactsDirectory 'implementation-log.md'
$implementationDispatchesPath = Join-Path $stageArtifactsDirectory 'implementation-dispatches.json'

$changeset = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    agentId = $AgentId
    backend = $backendUsed
    generatedAt = (Get-Date).ToString('o')
    summary = if ($failedTasks.Count -eq 0) { 'Implementation stage completed all planned work items.' } else { 'Implementation stage completed with blocked work items.' }
    sourceArtifacts = @($inputManifest.artifacts | ForEach-Object { [string] $_.name })
    workItemCount = $taskRuns.Count
    failedTaskCount = $failedTasks.Count
    changedFiles = @($allChangedFiles)
    tasks = $taskRuns
    commitReady = (@($taskRuns | Where-Object { -not $_.commitReady }).Count -eq 0) -and ($failedTasks.Count -eq 0)
}
Write-JsonFile -Path $changesetPath -Value $changeset

$implementationLog = @(
    ('# Implementation Log ({0})' -f $TraceId),
    '',
    ('- Stage: {0}' -f $StageId),
    ('- Agent: {0}' -f $AgentId),
    ('- Backend: {0}' -f $backendUsed),
    ('- GeneratedAt: {0}' -f (Get-Date).ToString('o')),
    ('- Work items: {0}' -f $taskRuns.Count),
    ('- Failed work items: {0}' -f $failedTasks.Count),
    '',
    '## Task Runs'
)
foreach ($taskRun in $taskRuns) {
    $implementationLog += ('### {0}: {1}' -f $taskRun.taskId, $taskRun.title)
    $implementationLog += ('- Status: {0}' -f $taskRun.status)
    $implementationLog += ('- Summary: {0}' -f $taskRun.summary)
    $implementationLog += ('- Changed files: {0}' -f ((@($taskRun.changedFiles) | ForEach-Object { [string] $_ }) -join ', '))
    $implementationLog += ('- Validation: {0}' -f ((@($taskRun.validationsPerformed) | ForEach-Object { [string] $_ }) -join '; '))
    $implementationLog += ('- Residual risks: {0}' -f ((@($taskRun.residualRisks) | ForEach-Object { [string] $_ }) -join '; '))
    if (-not [string]::IsNullOrWhiteSpace($taskRun.dispatchRecordPath)) {
        $implementationLog += ('- Dispatch record: {0}' -f $taskRun.dispatchRecordPath)
    }
    $implementationLog += ''
}
Set-Content -LiteralPath $implementationLogPath -Value ($implementationLog -join "`n") -Encoding UTF8 -NoNewline

$implementationDispatches = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    agentId = $AgentId
    backend = $backendUsed
    generatedAt = (Get-Date).ToString('o')
    tasks = $taskRuns
    warnings = @($dispatchErrors)
}
Write-JsonFile -Path $implementationDispatchesPath -Value $implementationDispatches

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
        (Get-ArtifactDescriptor -Name 'changeset' -Path $changesetPath -Root $resolvedRepoRoot),
        (Get-ArtifactDescriptor -Name 'implementation-log' -Path $implementationLogPath -Root $resolvedRepoRoot),
        (Get-ArtifactDescriptor -Name 'implementation-dispatches' -Path $implementationDispatchesPath -Root $resolvedRepoRoot)
    )
}
Write-JsonFile -Path $resolvedOutputManifestPath -Value $outputManifest

$stageState = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    agentId = $AgentId
    backend = $backendUsed
    dispatchCount = if ($backendUsed -eq 'codex-exec') { $taskRuns.Count } else { 0 }
    workItemCount = $taskRuns.Count
    changedFileCount = @($allChangedFiles).Count
    failedTaskCount = $failedTasks.Count
    promptTemplatePath = if ($backendUsed -eq 'codex-exec') { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedPromptTemplatePath } else { $null }
    responseSchemaPath = if ($backendUsed -eq 'codex-exec') { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedResponseSchemaPath } else { $null }
    taskStatePath = Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $implementationDispatchesPath
    warnings = @($dispatchErrors)
}
Write-JsonFile -Path $resolvedStageStatePath -Value $stageState

Write-VerboseLog ("Stage artifacts written: {0}" -f $resolvedOutputManifestPath)

if ($failedTasks.Count -gt 0) {
    exit 1
}

exit 0