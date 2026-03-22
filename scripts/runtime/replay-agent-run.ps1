<#
.SYNOPSIS
    Summarizes a recorded Super Agent run from trace, policy, and checkpoint artifacts.
#>

param(
    [string] $RepoRoot,
    [Parameter(Mandatory = $true)] [string] $RunDirectory,
    [string] $OutputPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

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
. $script:CommonBootstrapPath -CallerScriptRoot $PSScriptRoot -Helpers @('repository-paths', 'agent-runtime-hardening')

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$resolvedRunDirectory = Resolve-FullPath -BasePath $resolvedRepoRoot -Candidate $RunDirectory
$runArtifactPath = Join-Path $resolvedRunDirectory 'run-artifact.json'
$traceRecordPath = Join-Path $resolvedRunDirectory 'trace-record.json'
$policyEvaluationsPath = Join-Path $resolvedRunDirectory 'policy-evaluations.json'
$checkpointStatePath = Join-Path $resolvedRunDirectory 'checkpoint-state.json'

foreach ($requiredPath in @($runArtifactPath, $traceRecordPath, $policyEvaluationsPath, $checkpointStatePath)) {
    if (-not (Test-Path -LiteralPath $requiredPath -PathType Leaf)) {
        throw "Required run artifact not found: $requiredPath"
    }
}

$runArtifact = Read-HardeningJsonFile -Path $runArtifactPath
$traceRecord = Read-HardeningJsonFile -Path $traceRecordPath
$policyRecord = Read-HardeningJsonFile -Path $policyEvaluationsPath
$checkpointState = Read-HardeningJsonFile -Path $checkpointStatePath

$summary = [ordered]@{
    traceId = [string] $runArtifact.traceId
    pipelineId = [string] $runArtifact.pipelineId
    status = [string] $runArtifact.status
    stageCount = [int] $runArtifact.summary.stageCount
    totalDurationMs = [int] $runArtifact.summary.totalDurationMs
    policyWarningCount = [int] $runArtifact.summary.policyWarningCount
    policyBlockCount = [int] $runArtifact.summary.policyBlockCount
    lastSuccessfulStageId = [string] $checkpointState.lastSuccessfulStageId
    resumableFromStageId = [string] $checkpointState.resumableFromStageId
    stages = @($traceRecord.stages | ForEach-Object {
            [ordered]@{
                stageId = [string] $_.stageId
                status = [string] $_.status
                durationMs = [int] $_.durationMs
                effectiveModel = [string] $_.model.effectiveModel
                policyWarnings = [int] $_.policy.warningCount
                policyBlocks = [int] $_.policy.blockCount
            }
        })
    policyRuleIds = @($policyRecord.evaluations | ForEach-Object { [string] $_.ruleId } | Select-Object -Unique)
}

if (-not [string]::IsNullOrWhiteSpace($OutputPath)) {
    $resolvedOutputPath = Resolve-FullPath -BasePath $resolvedRepoRoot -Candidate $OutputPath
    Write-HardeningJsonFile -Path $resolvedOutputPath -Value $summary
}

$summary | ConvertTo-Json -Depth 50