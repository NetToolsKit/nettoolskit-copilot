<#
.SYNOPSIS
    Evaluates repository-owned agent pipeline contracts against versioned fixtures.
#>

param(
    [string] $RepoRoot,
    [string] $EvalsPath = '.codex/orchestration/evals/golden-tests.json',
    [string] $OutputPath = '.temp/agent-evals/pipeline-scorecard.json'
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
$resolvedEvalsPath = Resolve-RepoPath -Root $resolvedRepoRoot -Path $EvalsPath
$resolvedPipelinePath = Resolve-RepoPath -Root $resolvedRepoRoot -Path '.codex/orchestration/pipelines/default.pipeline.json'
$resolvedAgentsManifestPath = Resolve-RepoPath -Root $resolvedRepoRoot -Path '.codex/orchestration/agents.manifest.json'
$resolvedOutputPath = Resolve-FullPath -BasePath $resolvedRepoRoot -Candidate $OutputPath

$evals = Read-HardeningJsonFile -Path $resolvedEvalsPath
$pipeline = Read-HardeningJsonFile -Path $resolvedPipelinePath
$agentsManifest = Read-HardeningJsonFile -Path $resolvedAgentsManifestPath
$pipelineStageOrder = @($pipeline.stages | ForEach-Object { [string] $_.id })
$agentIds = @($agentsManifest.agents | ForEach-Object { [string] $_.id })

$caseResults = New-Object System.Collections.Generic.List[object]
foreach ($case in @($evals.cases)) {
    $failures = New-Object System.Collections.Generic.List[string]
    if ([string] $case.expectedPipelineId -ne [string] $pipeline.id) {
        $failures.Add(("Expected pipeline {0}, got {1}" -f [string] $case.expectedPipelineId, [string] $pipeline.id)) | Out-Null
    }

    $expectedOrder = @($case.expectedStageOrder | ForEach-Object { [string] $_ })
    if (($expectedOrder -join ',') -ne ($pipelineStageOrder -join ',')) {
        $failures.Add('Expected stage order does not match pipeline manifest.') | Out-Null
    }

    foreach ($requiredAgent in @($case.requiredAgents | ForEach-Object { [string] $_ })) {
        if (-not ($agentIds -contains $requiredAgent)) {
            $failures.Add(("Required agent missing from manifest: {0}" -f $requiredAgent)) | Out-Null
        }
    }

    $caseResults.Add([ordered]@{
            id = [string] $case.id
            status = if ($failures.Count -eq 0) { 'passed' } else { 'failed' }
            failures = @($failures.ToArray())
        }) | Out-Null
}

$scorecard = [ordered]@{
    generatedAt = (Get-Date).ToString('o')
    evalPath = Convert-ToRelativeRepoPath -Root $resolvedRepoRoot -Path $resolvedEvalsPath
    pipelineId = [string] $pipeline.id
    totalCases = $caseResults.Count
    passedCases = @($caseResults | Where-Object { $_.status -eq 'passed' }).Count
    failedCases = @($caseResults | Where-Object { $_.status -eq 'failed' }).Count
    cases = @($caseResults.ToArray())
}

Write-HardeningJsonFile -Path $resolvedOutputPath -Value $scorecard
$scorecard | ConvertTo-Json -Depth 50