<#
.SYNOPSIS
    Produces release-closeout artifacts for the orchestration pipeline.

.DESCRIPTION
    Consolidates release-facing outputs after review, including a commit-ready
    summary, changelog summary, and planning transition metadata.

.PARAMETER RepoRoot
    Repository root used to resolve manifests, prompts, schemas, and artifacts.

.PARAMETER RunDirectory
    Pipeline run directory where stage artifacts and metadata are persisted.

.PARAMETER TraceId
    Stable execution trace identifier for the current pipeline run.

.PARAMETER StageId
    Stage identifier from the pipeline manifest.

.PARAMETER AgentId
    Agent contract identifier used for this stage.

.PARAMETER RequestPath
    Path to the normalized request payload for the run.

.PARAMETER InputArtifactManifestPath
    Artifact manifest produced by the previous stage.

.PARAMETER OutputArtifactManifestPath
    Artifact manifest written by this stage.

.PARAMETER AgentsManifestPath
    Relative or absolute path to the orchestration agent manifest.

.PARAMETER DispatchMode
    Dispatch mode declared by the pipeline stage contract.

.PARAMETER PromptTemplatePath
    Closeout prompt template used when live dispatch is enabled.

.PARAMETER ResponseSchemaPath
    JSON schema used to validate live closeout output.

.PARAMETER DispatchCommand
    Local command used to invoke Codex dispatch.

.PARAMETER ExecutionBackend
    Selected backend for the run, such as `script-only` or `codex-exec`.

.PARAMETER StageStatePath
    Optional override path for the persisted stage-state artifact.

.PARAMETER DetailedOutput
    Enables verbose diagnostics for stage execution.

.EXAMPLE
    pwsh -File scripts/orchestration/stages/closeout-stage.ps1 -RepoRoot . -RunDirectory .temp/runs/example -TraceId trace-1 -StageId closeout -AgentId release-engineer -RequestPath .temp/runs/example/request.md -InputArtifactManifestPath .temp/runs/example/review-output.json -OutputArtifactManifestPath .temp/runs/example/closeout-output.json

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
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

# Writes verbose diagnostics for closeout-stage execution.
function Write-VerboseLog {
    param([string] $Message)

    if ($script:IsVerboseEnabled) {
        Write-StyledOutput ("[VERBOSE] {0}" -f $Message)
    }
}

# Resolves relative stage paths against a base directory.
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

# Converts an absolute path into a repository-relative artifact path.
function Convert-ToRelativeRepoPath {
    param(
        [string] $Root,
        [string] $Path
    )

    return [System.IO.Path]::GetRelativePath($Root, $Path) -replace '\\', '/'
}

# Builds one artifact descriptor with checksum metadata.
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

# Reads a JSON file and returns the parsed object.
function Read-JsonFile {
    param([string] $Path)

    return (Get-Content -Raw -LiteralPath $Path | ConvertFrom-Json -Depth 200)
}

# Writes JSON content without a trailing newline.
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

# Replaces token placeholders in a prompt template.
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

# Loads one agent contract from the orchestration manifest.
function Get-AgentContract {
    param(
        [string] $ManifestPath,
        [string] $TargetAgentId
    )

    $manifest = Read-JsonFile -Path $ManifestPath
    return @($manifest.agents | Where-Object { $_.id -eq $TargetAgentId } | Select-Object -First 1)
}

# Converts an artifact manifest into a name-to-path lookup table.
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

# Produces the deterministic fallback closeout result when live dispatch is unavailable.
function New-FallbackCloseoutResult {
    param(
        [string] $ReviewDecision,
        [int] $FailedChecks
    )

    if ($ReviewDecision -eq 'blocked' -or $FailedChecks -gt 0) {
        return [ordered]@{
            status = 'blocked'
            summary = 'Closeout is blocked because review or validation did not pass cleanly.'
            readmeActions = @('Do not update release-facing documentation until the blocking issues are resolved.')
            commitMessage = 'fix: resolve blocking review findings before closeout'
            changelogSummary = 'Blocked closeout due to pending validation or review findings.'
            followUps = @(
                'Resolve failed validation checks.',
                'Address reviewer follow-up items before closing the plan.'
            )
        }
    }

    return [ordered]@{
        status = 'ready-for-commit'
        summary = 'Closeout is ready for commit and plan completion.'
        readmeActions = @('Review whether README changes are required for the delivered scope and keep them aligned when applicable.')
        commitMessage = 'feat: finalize planned delivery with validated review closeout'
        changelogSummary = 'Finalize the validated delivery and close the active implementation plan.'
        followUps = @()
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

$inputManifest = Read-JsonFile -Path $resolvedInputManifestPath
$artifactMap = Convert-ArtifactManifestToMap -Manifest $inputManifest -Root $resolvedRepoRoot
$normalizedRequestPath = if ($artifactMap.ContainsKey('normalized-request')) { [string] $artifactMap['normalized-request'] } else { $null }
$routeSelectionPath = if ($artifactMap.ContainsKey('route-selection')) { [string] $artifactMap['route-selection'] } else { $null }
$changesetPath = if ($artifactMap.ContainsKey('changeset')) { [string] $artifactMap['changeset'] } else { $null }
$validationReportPath = if ($artifactMap.ContainsKey('validation-report')) { [string] $artifactMap['validation-report'] } else { $null }
$reviewReportPath = if ($artifactMap.ContainsKey('review-report')) { [string] $artifactMap['review-report'] } else { $null }
$decisionLogPath = if ($artifactMap.ContainsKey('decision-log')) { [string] $artifactMap['decision-log'] } else { $null }
$activePlanPath = if ($artifactMap.ContainsKey('active-plan')) { [string] $artifactMap['active-plan'] } else { $null }

if ($null -ne $normalizedRequestPath -and (Test-Path -LiteralPath $normalizedRequestPath -PathType Leaf)) {
    $normalizedRequestContent = (Get-Content -Raw -LiteralPath $normalizedRequestPath).Trim()
    if (-not [string]::IsNullOrWhiteSpace($normalizedRequestContent)) {
        $requestContent = $normalizedRequestContent
    }
}

$routeSelectionJson = if ($null -ne $routeSelectionPath -and (Test-Path -LiteralPath $routeSelectionPath -PathType Leaf)) { Get-Content -Raw -LiteralPath $routeSelectionPath } else { '{}' }
$changesetJson = if ($null -ne $changesetPath -and (Test-Path -LiteralPath $changesetPath -PathType Leaf)) { Get-Content -Raw -LiteralPath $changesetPath } else { '{}' }
$validationReport = if ($null -ne $validationReportPath -and (Test-Path -LiteralPath $validationReportPath -PathType Leaf)) { Read-JsonFile -Path $validationReportPath } else { $null }
$validationReportJson = if ($null -ne $validationReportPath -and (Test-Path -LiteralPath $validationReportPath -PathType Leaf)) { Get-Content -Raw -LiteralPath $validationReportPath } else { '{}' }
$reviewReportText = if ($null -ne $reviewReportPath -and (Test-Path -LiteralPath $reviewReportPath -PathType Leaf)) { Get-Content -Raw -LiteralPath $reviewReportPath } else { '' }
$decisionLogText = if ($null -ne $decisionLogPath -and (Test-Path -LiteralPath $decisionLogPath -PathType Leaf)) { Get-Content -Raw -LiteralPath $decisionLogPath } else { '' }

$failedChecks = if ($null -ne $validationReport) { [int] $validationReport.summary.failedChecks } else { 1 }
$reviewDecision = if ($decisionLogText -match 'Decision:\s*(?<decision>[a-z-]+)') { $Matches['decision'] } else { 'blocked' }

$shouldUseCodexDispatch = ($ExecutionBackend -eq 'codex-exec') -and ($DispatchMode -eq 'codex-exec')
$backendUsed = if ($shouldUseCodexDispatch) { 'codex-exec' } else { 'scripted' }
$dispatchError = $null
$closeoutResult = $null
$dispatchRecordPath = Join-Path $stageMetadataDirectory 'closeout-dispatch.json'
$dispatchResultPath = Join-Path $stageMetadataDirectory 'closeout-result.json'
$dispatchPromptPath = Join-Path $stageMetadataDirectory 'closeout-prompt.md'

if ($shouldUseCodexDispatch) {
    try {
        $templateText = Get-Content -Raw -LiteralPath $resolvedPromptTemplatePath
        $renderedPrompt = Expand-Template -TemplateText $templateText -Tokens @{
            REQUEST_TEXT = $requestContent
            ROUTE_SELECTION_JSON = $routeSelectionJson
            CHANGESET_JSON = $changesetJson
            VALIDATION_REPORT_JSON = $validationReportJson
            REVIEW_REPORT_TEXT = $reviewReportText
            DECISION_LOG_TEXT = $decisionLogText
        }
        Set-Content -LiteralPath $dispatchPromptPath -Value $renderedPrompt -Encoding UTF8 -NoNewline

        $dispatchScriptPath = Join-Path $resolvedRepoRoot 'scripts/orchestration/engine/invoke-codex-dispatch.ps1'
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
        $closeoutResult = Read-JsonFile -Path $dispatchResultPath
    }
    catch {
        $dispatchError = $_.Exception.Message
        Write-StyledOutput ("[WARN] Closeout live dispatch failed. Falling back to scripted mode. {0}" -f $dispatchError)
        $backendUsed = 'scripted'
    }
}

if ($null -eq $closeoutResult) {
    $closeoutResult = New-FallbackCloseoutResult -ReviewDecision $reviewDecision -FailedChecks $failedChecks
}

$closeoutReportPath = Join-Path $stageArtifactsDirectory 'closeout-report.json'
$releaseSummaryPath = Join-Path $stageArtifactsDirectory 'release-summary.md'
$completedPlanMetadataPath = Join-Path $stageArtifactsDirectory 'completed-plan.json'

$completedPlanPath = $null
$planMoved = $false
if ($closeoutResult.status -eq 'ready-for-commit' -and -not [string]::IsNullOrWhiteSpace($activePlanPath) -and (Test-Path -LiteralPath $activePlanPath -PathType Leaf)) {
    $completedPlansDirectory = Join-Path $resolvedRepoRoot 'planning/completed'
    New-Item -ItemType Directory -Path $completedPlansDirectory -Force | Out-Null
    $completedPlanPath = Join-Path $completedPlansDirectory ([System.IO.Path]::GetFileName($activePlanPath))
    Move-Item -LiteralPath $activePlanPath -Destination $completedPlanPath -Force
    $planMoved = $true
}

Write-JsonFile -Path $closeoutReportPath -Value $closeoutResult

$releaseSummary = @(
    ('# Release Summary ({0})' -f $TraceId),
    '',
    ('- Stage: {0}' -f $StageId),
    ('- Agent: {0}' -f $AgentId),
    ('- Backend: {0}' -f $backendUsed),
    ('- Status: {0}' -f [string] $closeoutResult.status),
    ('- GeneratedAt: {0}' -f (Get-Date).ToString('o')),
    '',
    '## Summary',
    ('- {0}' -f [string] $closeoutResult.summary),
    '',
    '## Commit Message',
    ('- `{0}`' -f [string] $closeoutResult.commitMessage),
    '',
    '## Changelog Summary',
    ('- {0}' -f [string] $closeoutResult.changelogSummary),
    '',
    '## README Actions'
)
$releaseSummary += @($closeoutResult.readmeActions | ForEach-Object { '- ' + [string] $_ })
$releaseSummary += @(
    '',
    '## Follow-Ups'
)
$releaseSummary += @($closeoutResult.followUps | ForEach-Object { '- ' + [string] $_ })
$releaseSummary += @(
    '',
    ('- Active plan moved: {0}' -f $planMoved)
)
Set-Content -LiteralPath $releaseSummaryPath -Value ($releaseSummary -join "`n") -Encoding UTF8 -NoNewline

$completedPlanMetadata = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    agentId = $AgentId
    moved = $planMoved
    sourcePlanPath = if (-not [string]::IsNullOrWhiteSpace($activePlanPath)) { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $activePlanPath } else { $null }
    completedPlanPath = if (-not [string]::IsNullOrWhiteSpace($completedPlanPath)) { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $completedPlanPath } else { $null }
}
Write-JsonFile -Path $completedPlanMetadataPath -Value $completedPlanMetadata

$outputManifest = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    agentId = $AgentId
    producedAt = (Get-Date).ToString('o')
    artifacts = @(
        (Get-ArtifactDescriptor -Name 'closeout-report' -Path $closeoutReportPath -Root $resolvedRepoRoot),
        (Get-ArtifactDescriptor -Name 'release-summary' -Path $releaseSummaryPath -Root $resolvedRepoRoot),
        (Get-ArtifactDescriptor -Name 'completed-plan' -Path $completedPlanMetadataPath -Root $resolvedRepoRoot)
    )
}
Write-JsonFile -Path $resolvedOutputManifestPath -Value $outputManifest

$stageState = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    agentId = $AgentId
    backend = $backendUsed
    dispatchCount = if ($backendUsed -eq 'codex-exec') { 1 } else { 0 }
    completedPlanMoved = $planMoved
    promptTemplatePath = if ($backendUsed -eq 'codex-exec') { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedPromptTemplatePath } else { $null }
    responseSchemaPath = if ($backendUsed -eq 'codex-exec') { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedResponseSchemaPath } else { $null }
    dispatchRecordPath = if ((Test-Path -LiteralPath $dispatchRecordPath -PathType Leaf)) { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $dispatchRecordPath } else { $null }
    warning = $dispatchError
}
Write-JsonFile -Path $resolvedStageStatePath -Value $stageState

Write-VerboseLog ("Stage artifacts written: {0}" -f $resolvedOutputManifestPath)

if ($closeoutResult.status -eq 'blocked') {
    exit 1
}

exit 0