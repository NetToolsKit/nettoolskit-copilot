<#
.SYNOPSIS
    Produces normalized request artifacts for the Super Agent intake stage.

.DESCRIPTION
    Runs the repository-owned Super Agent intake lifecycle before planning begins.
    When live dispatch is enabled, invokes the Super Agent through the local
    Codex CLI and persists a normalized request plus intake metadata.

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
    Path to the request artifact used as the initial intake payload.

.PARAMETER InputArtifactManifestPath
    Optional path to an upstream artifact manifest. The intake stage can start without it.

.PARAMETER OutputArtifactManifestPath
    Artifact manifest written by this stage.

.PARAMETER AgentsManifestPath
    Relative or absolute path to the orchestration agent manifest.

.PARAMETER DispatchMode
    Dispatch mode declared by the pipeline stage contract.

.PARAMETER PromptTemplatePath
    Intake prompt template used when live dispatch is enabled.

.PARAMETER ResponseSchemaPath
    JSON schema used to validate live intake output.

.PARAMETER DispatchCommand
    Local command used to invoke Codex dispatch.

.PARAMETER ExecutionBackend
    Selected backend for the run, such as `script-only` or `codex-exec`.

.PARAMETER StageStatePath
    Optional override path for the persisted stage-state artifact.

.PARAMETER DetailedOutput
    Enables verbose diagnostics for stage execution.

.EXAMPLE
    pwsh -File scripts/orchestration/stages/intake-stage.ps1 -RepoRoot . -RunDirectory .temp/runs/example -TraceId trace-1 -StageId intake -AgentId super-agent -RequestPath .temp/runs/example/request.md -OutputArtifactManifestPath .temp/runs/example/intake-output.json -ExecutionBackend codex-exec -DispatchMode codex-exec

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
$script:RepositoryHelpersPath = Join-Path $PSScriptRoot '..\..\common\repository-paths.ps1'
if (-not (Test-Path -LiteralPath $script:RepositoryHelpersPath -PathType Leaf)) {
    throw "Missing shared repository helper: $script:RepositoryHelpersPath"
}
. $script:RepositoryHelpersPath
$script:IsVerboseEnabled = [bool] $DetailedOutput

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

# Converts free-text work intent into a stable planning slug.
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

# Creates a deterministic fallback intake result when live dispatch is unavailable.
function New-FallbackIntakeResult {
    param([string] $RequestText)

    $trimmed = ($RequestText ?? '').Trim()
    $normalized = if ([string]::IsNullOrWhiteSpace($trimmed)) { 'No request content provided.' } else { $trimmed }
    $isInformational = $normalized -match '^(what|why|how|list|show|explain)\b' -and $normalized -notmatch '\b(add|adjust|change|create|delete|edit|fix|implement|move|refactor|remove|rename|sync|update|write)\b'
    $slug = Convert-ToPlanSlug -Text $normalized

    return [ordered]@{
        stage = 'super-agent-intake'
        normalizedRequest = $normalized
        changeBearing = (-not $isInformational)
        planningRequired = (-not $isInformational)
        workstreamSlug = $slug
        explicitWorkItems = @($normalized)
        constraints = @(
            'Preserve repository instructions, policies, and validation coverage.',
            'Use repository context first and official sources second.'
        )
        risks = @(
            'Skipping planning or review would violate the repository lifecycle.'
        )
        notes = @(
            'Execution remains sequential unless later planning proves tasks are parallel-safe.'
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
$dispatchRecordPath = Join-Path $stageMetadataDirectory 'super-agent-dispatch.json'
$dispatchResultPath = Join-Path $stageMetadataDirectory 'super-agent-result.json'
$dispatchPromptPath = Join-Path $stageMetadataDirectory 'super-agent-prompt.md'
$dispatchError = $null
$intakeResult = $null
$backendUsed = if ($shouldUseCodexDispatch) { 'codex-exec' } else { 'scripted' }

if ($shouldUseCodexDispatch) {
    try {
        if (-not (Test-Path -LiteralPath $resolvedPromptTemplatePath -PathType Leaf)) {
            throw "Super Agent prompt template not found: $resolvedPromptTemplatePath"
        }

        if (-not (Test-Path -LiteralPath $resolvedResponseSchemaPath -PathType Leaf)) {
            throw "Super Agent response schema not found: $resolvedResponseSchemaPath"
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
        $intakeResult = Read-JsonFile -Path $dispatchResultPath
    }
    catch {
        $dispatchError = $_.Exception.Message
        Write-StyledOutput ("[WARN] Super Agent live dispatch failed. Falling back to scripted mode. {0}" -f $dispatchError)
        $backendUsed = 'scripted'
    }
}

if ($null -eq $intakeResult) {
    $intakeResult = New-FallbackIntakeResult -RequestText $requestContent
}

$normalizedRequestPath = Join-Path $stageArtifactsDirectory 'normalized-request.md'
$intakeReportPath = Join-Path $stageArtifactsDirectory 'intake-report.json'
Set-Content -LiteralPath $normalizedRequestPath -Value ([string] $intakeResult.normalizedRequest) -Encoding UTF8 -NoNewline
Write-JsonFile -Path $intakeReportPath -Value $intakeResult

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
        (Get-ArtifactDescriptor -Name 'normalized-request' -Path $normalizedRequestPath -Root $resolvedRepoRoot),
        (Get-ArtifactDescriptor -Name 'intake-report' -Path $intakeReportPath -Root $resolvedRepoRoot)
    )
}
Write-JsonFile -Path $resolvedOutputManifestPath -Value $outputManifest

$stageState = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    agentId = $AgentId
    backend = $backendUsed
    dispatchCount = if ($backendUsed -eq 'codex-exec') { 1 } else { 0 }
    planningRequired = [bool] $intakeResult.planningRequired
    changeBearing = [bool] $intakeResult.changeBearing
    promptTemplatePath = if ($backendUsed -eq 'codex-exec') { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedPromptTemplatePath } else { $null }
    responseSchemaPath = if ($backendUsed -eq 'codex-exec') { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedResponseSchemaPath } else { $null }
    dispatchRecordPath = if ((Test-Path -LiteralPath $dispatchRecordPath -PathType Leaf)) { Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $dispatchRecordPath } else { $null }
    warning = $dispatchError
}
Write-JsonFile -Path $resolvedStageStatePath -Value $stageState

Write-VerboseLog ("Stage artifacts written: {0}" -f $resolvedOutputManifestPath)
exit 0