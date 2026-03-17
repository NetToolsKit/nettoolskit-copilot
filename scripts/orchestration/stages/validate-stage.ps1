<#
.SYNOPSIS
    Executes repository validation checks and produces validation artifacts.

.DESCRIPTION
    Runs deterministic validation scripts and writes:
    - validation-report json artifact
    - output artifact manifest for handoff
    - stage state metadata for orchestration observability

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

.PARAMETER StageStatePath
    Optional path where stage execution metadata is written.

.PARAMETER DetailedOutput
    Shows detailed diagnostics.

.EXAMPLE
    pwsh -File .\scripts\orchestration\stages\validate-stage.ps1 -RepoRoot . -RunDirectory .temp\runs\trace-001 -TraceId trace-001 -StageId validate -AgentId tester -RequestPath .temp\runs\trace-001\request.txt -InputArtifactManifestPath .temp\runs\trace-001\stages\implement\output-artifacts.json -OutputArtifactManifestPath .temp\runs\trace-001\stages\validate\output-artifacts.json

.NOTES
    This stage stays scripted in phase 1 and acts as the deterministic quality gate before review.
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
    [string] $AgentsManifestPath,
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

# Executes one repository validation script and returns structured timing/output metadata.
function Invoke-ValidationScript {
    param(
        [string] $Name,
        [string] $ScriptPath,
        [string] $Root
    )

    $startedAt = Get-Date
    $status = 'failed'
    $exitCode = 1
    $errorMessage = $null

    try {
        & $ScriptPath -RepoRoot $Root | Out-Host
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        if ($exitCode -eq 0) {
            $status = 'passed'
        }
    }
    catch {
        $exitCode = 1
        $errorMessage = $_.Exception.Message
    }

    $finishedAt = Get-Date
    return [pscustomobject]@{
        name = $Name
        script = (Convert-ToRelativeRepoPath -Root $Root -Path $ScriptPath)
        status = $status
        exitCode = $exitCode
        startedAt = $startedAt.ToString('o')
        finishedAt = $finishedAt.ToString('o')
        durationMs = [int] ($finishedAt - $startedAt).TotalMilliseconds
        error = $errorMessage
    }
}

$resolvedRepoRoot = [System.IO.Path]::GetFullPath($RepoRoot)
$resolvedRunDirectory = [System.IO.Path]::GetFullPath($RunDirectory)
$resolvedRequestPath = [System.IO.Path]::GetFullPath($RequestPath)
$resolvedInputManifestPath = [System.IO.Path]::GetFullPath($InputArtifactManifestPath)
$resolvedOutputManifestPath = [System.IO.Path]::GetFullPath($OutputArtifactManifestPath)
$resolvedStageStatePath = if ([string]::IsNullOrWhiteSpace($StageStatePath)) { Join-Path $resolvedRunDirectory ('stages/{0}-state.json' -f $StageId) } else { [System.IO.Path]::GetFullPath($StageStatePath) }

$stageArtifactsDirectory = Join-Path $resolvedRunDirectory 'artifacts'
New-Item -ItemType Directory -Path $stageArtifactsDirectory -Force | Out-Null

$validationScripts = @(
    [ordered]@{ Name = 'validate-instructions'; Path = (Join-Path $resolvedRepoRoot 'scripts/validation/validate-instructions.ps1') },
    [ordered]@{ Name = 'validate-policy'; Path = (Join-Path $resolvedRepoRoot 'scripts/validation/validate-policy.ps1') },
    [ordered]@{ Name = 'validate-agent-orchestration'; Path = (Join-Path $resolvedRepoRoot 'scripts/validation/validate-agent-orchestration.ps1') },
    [ordered]@{ Name = 'validate-planning-structure'; Path = (Join-Path $resolvedRepoRoot 'scripts/validation/validate-planning-structure.ps1') },
    [ordered]@{ Name = 'validate-release-governance'; Path = (Join-Path $resolvedRepoRoot 'scripts/validation/validate-release-governance.ps1') }
)

$results = New-Object System.Collections.Generic.List[object]
foreach ($scriptEntry in $validationScripts) {
    if (-not (Test-Path -LiteralPath $scriptEntry.Path -PathType Leaf)) {
        $results.Add([pscustomobject]@{
            name = $scriptEntry.Name
            script = (Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $scriptEntry.Path)
            status = 'failed'
            exitCode = 1
            startedAt = (Get-Date).ToString('o')
            finishedAt = (Get-Date).ToString('o')
            durationMs = 0
            error = 'Validation script not found.'
        }) | Out-Null
        continue
    }

    $results.Add((Invoke-ValidationScript -Name $scriptEntry.Name -ScriptPath $scriptEntry.Path -Root $resolvedRepoRoot)) | Out-Null
}

$failedChecks = @($results | Where-Object { $_.status -ne 'passed' }).Count
$inputArtifactCount = 0
if (Test-Path -LiteralPath $resolvedInputManifestPath -PathType Leaf) {
    $inputManifest = Get-Content -Raw -LiteralPath $resolvedInputManifestPath | ConvertFrom-Json -Depth 100
    $inputArtifactCount = @($inputManifest.artifacts).Count
}
$validationReportPath = Join-Path $stageArtifactsDirectory 'validation-report.json'

$validationReport = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    agentId = $AgentId
    generatedAt = (Get-Date).ToString('o')
    summary = [ordered]@{
        totalChecks = $results.Count
        failedChecks = $failedChecks
        overallStatus = if ($failedChecks -eq 0) { 'passed' } else { 'failed' }
    }
    inputs = [ordered]@{
        requestPath = Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedRequestPath
        inputArtifactManifestPath = Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedInputManifestPath
        inputArtifactCount = $inputArtifactCount
    }
    checks = $results
}
Write-JsonFile -Path $validationReportPath -Value $validationReport

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
        (Get-ArtifactDescriptor -Name 'validation-report' -Path $validationReportPath -Root $resolvedRepoRoot)
    )
}
Write-JsonFile -Path $resolvedOutputManifestPath -Value $outputManifest

$stageState = [ordered]@{
    traceId = $TraceId
    stageId = $StageId
    agentId = $AgentId
    backend = 'scripted'
    dispatchCount = 0
    validationCount = $results.Count
    failedChecks = $failedChecks
}
Write-JsonFile -Path $resolvedStageStatePath -Value $stageState

Write-VerboseLog ("Stage artifacts written: {0}" -f $resolvedOutputManifestPath)

if ($failedChecks -gt 0) {
    exit 1
}

exit 0