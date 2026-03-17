<#
.SYNOPSIS
    Runtime tests for the multi-agent orchestration engine without external frameworks.

.DESCRIPTION
    Validates Codex dispatch integration using a fake local Codex command so the
    orchestration stages can be tested deterministically without network access.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.
#>

param(
    [string] $RepoRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Resolve-RepositoryRoot {
    param([string] $RequestedRoot)

    $candidates = @()
    if (-not [string]::IsNullOrWhiteSpace($RequestedRoot)) {
        $candidates += (Resolve-Path -LiteralPath $RequestedRoot).Path
    }
    $candidates += (Get-Location).Path

    foreach ($candidate in ($candidates | Select-Object -Unique)) {
        $current = $candidate
        for ($index = 0; $index -lt 6 -and -not [string]::IsNullOrWhiteSpace($current); $index++) {
            if ((Test-Path -LiteralPath (Join-Path $current '.github')) -and (Test-Path -LiteralPath (Join-Path $current '.codex'))) {
                return $current
            }

            $current = Split-Path -Path $current -Parent
        }
    }

    throw 'Could not detect repository root containing both .github and .codex.'
}

function Assert-True {
    param(
        [bool] $Condition,
        [string] $Message
    )

    if (-not $Condition) {
        throw $Message
    }
}

function Assert-Equal {
    param(
        $Actual,
        $Expected,
        [string] $Message
    )

    if ($Actual -ne $Expected) {
        throw ("{0}. Expected '{1}', got '{2}'." -f $Message, $Expected, $Actual)
    }
}

function Write-JsonFile {
    param(
        [string] $Path,
        [object] $Value
    )

    $parent = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }

    Set-Content -LiteralPath $Path -Value ($Value | ConvertTo-Json -Depth 100) -Encoding UTF8 -NoNewline
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))

try {
    New-Item -ItemType Directory -Path $tempRoot -Force | Out-Null
    $fakeCodexRunnerPath = Join-Path $tempRoot 'fake-codex-runner.ps1'
    $fakeCodexPath = Join-Path $tempRoot 'fake-codex.cmd'
    $fakeCodex = @'
param(
    [Parameter(ValueFromPipeline = $true)] [string] $IgnoredPipelineInput,
    [string] $RawArgs
)

begin {
    $allInput = ''
}
process {
    if ($null -ne $IgnoredPipelineInput) {
        $allInput += $IgnoredPipelineInput + "`n"
    }
}
end {
    if ([string]::IsNullOrWhiteSpace($allInput)) {
        $allInput = [Console]::In.ReadToEnd()
    }

    $RemainingArgs = @()
    if (-not [string]::IsNullOrWhiteSpace($RawArgs)) {
        $RemainingArgs = @($RawArgs.Split(' ', [System.StringSplitOptions]::RemoveEmptyEntries))
    }

    $outputPath = $null
    for ($i = 0; $i -lt $RemainingArgs.Count; $i++) {
        if ($RemainingArgs[$i] -eq '-o' -and ($i + 1) -lt $RemainingArgs.Count) {
            $outputPath = $RemainingArgs[$i + 1]
        }
    }

    if ([string]::IsNullOrWhiteSpace($outputPath)) {
        throw 'Fake codex did not receive -o output path.'
    }

    if ($allInput -match '# Planner Stage Contract') {
        $payload = [ordered]@{
            objective = 'Deliver the requested change with deterministic validation.'
            scopeSummary = 'Planner produced two sequential work items for the request.'
            assumptions = @('Repository rules stay authoritative.')
            acceptanceCriteria = @('Changes are implemented.', 'Validation artifacts are ready.')
            workItems = @(
                [ordered]@{
                    id = 'task-one'
                    title = 'Prepare first change'
                    description = 'First planned task.'
                    dependsOn = @()
                    allowedPaths = @('scripts/**', '.github/**')
                    deliverables = @('First deliverable')
                    validationSteps = @('Run focused checks.')
                    successCriteria = @('First task completes cleanly.')
                },
                [ordered]@{
                    id = 'task-two'
                    title = 'Prepare second change'
                    description = 'Second planned task.'
                    dependsOn = @('task-one')
                    allowedPaths = @('scripts/**', '.github/**')
                    deliverables = @('Second deliverable')
                    validationSteps = @('Run focused checks.')
                    successCriteria = @('Second task completes cleanly.')
                }
            )
            contextPaths = @('.github/AGENTS.md', '.github/copilot-instructions.md')
            validations = @('Run repository validation stage.')
            risks = @('None in mock flow.')
            deliverySlices = @(
                [ordered]@{ name = 'phase-1'; goal = 'Plan and execute sequential tasks.' }
            )
        }
    }
    elseif ($allInput -match '# Executor Task Contract') {
        $taskId = 'task-generic'
        if ($allInput -match '"id"\s*:\s*"(?<taskId>[a-z0-9-]+)"') {
            $taskId = $Matches['taskId']
        }

        $payload = [ordered]@{
            taskId = $taskId
            status = 'completed'
            summary = "Completed $taskId in fake execution mode."
            changedFiles = @()
            validationsPerformed = @('Prepared downstream validation artifacts.')
            residualRisks = @()
            notes = @('Mock executor produced no file changes.')
            commitReady = $true
        }
    }
    elseif ($allInput -match '# Reviewer Stage Contract') {
        $payload = [ordered]@{
            decision = 'approved'
            summary = 'Mock reviewer approved the change set.'
            findings = @()
            requiredFollowUps = @()
            recommendation = 'Proceed with delivery.'
        }
    }
    else {
        throw 'Fake codex received an unknown prompt contract.'
    }

    Set-Content -LiteralPath $outputPath -Value ($payload | ConvertTo-Json -Depth 100) -Encoding UTF8 -NoNewline
}
'@
    Set-Content -LiteralPath $fakeCodexRunnerPath -Value $fakeCodex -Encoding UTF8 -NoNewline
    $fakeCodexCmd = "@echo off`r`nsetlocal`r`nset FAKE_CODEX_ARGS=%*`r`npwsh -NoProfile -File `"$fakeCodexRunnerPath`" -RawArgs `"%FAKE_CODEX_ARGS%`"`r`n"
    Set-Content -LiteralPath $fakeCodexPath -Value $fakeCodexCmd -Encoding ASCII -NoNewline

    $dispatchScriptPath = Join-Path $resolvedRepoRoot 'scripts/orchestration/engine/invoke-codex-dispatch.ps1'
    $dispatchPromptPath = Join-Path $tempRoot 'dispatch-prompt.md'
    $dispatchSchemaPath = Join-Path $resolvedRepoRoot '.github/schemas/agent.stage-plan-result.schema.json'
    $dispatchResultPath = Join-Path $tempRoot 'dispatch-result.json'
    $dispatchRecordPath = Join-Path $tempRoot 'dispatch-record.json'
    Set-Content -LiteralPath $dispatchPromptPath -Value '# Planner Stage Contract' -Encoding UTF8 -NoNewline

    & $dispatchScriptPath -RepoRoot $resolvedRepoRoot -TraceId 'run-test' -StageId 'plan' -AgentId 'planner' -PromptPath $dispatchPromptPath -ResponseSchemaPath $dispatchSchemaPath -ResultPath $dispatchResultPath -DispatchRecordPath $dispatchRecordPath -CommandName $fakeCodexPath | Out-Null
    $dispatchExit = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
    Assert-Equal -Actual $dispatchExit -Expected 0 -Message 'Dispatch helper should succeed with fake Codex.'
    Assert-True (Test-Path -LiteralPath $dispatchResultPath -PathType Leaf) 'Dispatch helper did not create result file.'

    $runDirectory = Join-Path $tempRoot 'run'
    New-Item -ItemType Directory -Path (Join-Path $runDirectory 'artifacts') -Force | Out-Null
    New-Item -ItemType Directory -Path (Join-Path $runDirectory 'stages') -Force | Out-Null

    $requestPath = Join-Path $runDirectory 'artifacts/request.md'
    Set-Content -LiteralPath $requestPath -Value 'Implement enterprise orchestration support.' -Encoding UTF8 -NoNewline

    $planOutputManifestPath = Join-Path $runDirectory 'stages/plan-output.json'
    $planStageScript = Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/plan-stage.ps1'
    & $planStageScript -RepoRoot $resolvedRepoRoot -RunDirectory $runDirectory -TraceId 'run-test' -StageId 'plan' -AgentId 'planner' -RequestPath $requestPath -OutputArtifactManifestPath $planOutputManifestPath -DispatchMode 'codex-exec' -PromptTemplatePath '.codex/orchestration/prompts/planner-stage.prompt.md' -ResponseSchemaPath '.github/schemas/agent.stage-plan-result.schema.json' -DispatchCommand $fakeCodexPath -ExecutionBackend 'codex-exec' | Out-Null
    $planExit = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
    Assert-Equal -Actual $planExit -Expected 0 -Message 'Plan stage should succeed with fake Codex.'

    $planManifest = Get-Content -Raw -LiteralPath $planOutputManifestPath | ConvertFrom-Json -Depth 100
    $planArtifacts = @{}
    foreach ($artifact in @($planManifest.artifacts)) {
        $planArtifacts[[string] $artifact.name] = Join-Path $resolvedRepoRoot ([string] $artifact.path)
    }
    $taskPlanData = Get-Content -Raw -LiteralPath $planArtifacts['task-plan-data'] | ConvertFrom-Json -Depth 100
    Assert-Equal -Actual @($taskPlanData.workItems).Count -Expected 2 -Message 'Plan stage should produce two work items in fake flow.'

    $implementOutputManifestPath = Join-Path $runDirectory 'stages/implement-output.json'
    $implementInputManifestPath = Join-Path $runDirectory 'stages/implement-input.json'
    $implementInputManifest = [ordered]@{
        traceId = 'run-test'
        stageId = 'implement'
        agentId = 'executor'
        producedAt = (Get-Date).ToString('o')
        artifacts = @($planManifest.artifacts)
    }
    Write-JsonFile -Path $implementInputManifestPath -Value $implementInputManifest

    $implementStageScript = Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/implement-stage.ps1'
    & $implementStageScript -RepoRoot $resolvedRepoRoot -RunDirectory $runDirectory -TraceId 'run-test' -StageId 'implement' -AgentId 'executor' -RequestPath $requestPath -InputArtifactManifestPath $implementInputManifestPath -OutputArtifactManifestPath $implementOutputManifestPath -DispatchMode 'codex-exec' -PromptTemplatePath '.codex/orchestration/prompts/executor-task.prompt.md' -ResponseSchemaPath '.github/schemas/agent.stage-implementation-result.schema.json' -DispatchCommand $fakeCodexPath -ExecutionBackend 'codex-exec' | Out-Null
    $implementExit = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
    Assert-Equal -Actual $implementExit -Expected 0 -Message 'Implement stage should succeed with fake Codex.'

    $implementManifest = Get-Content -Raw -LiteralPath $implementOutputManifestPath | ConvertFrom-Json -Depth 100
    $implementArtifacts = @{}
    foreach ($artifact in @($implementManifest.artifacts)) {
        $implementArtifacts[[string] $artifact.name] = Join-Path $resolvedRepoRoot ([string] $artifact.path)
    }
    $dispatches = Get-Content -Raw -LiteralPath $implementArtifacts['implementation-dispatches'] | ConvertFrom-Json -Depth 100
    Assert-Equal -Actual @($dispatches.tasks).Count -Expected 2 -Message 'Implement stage should dispatch each planned work item.'

    $validationReportPath = Join-Path $runDirectory 'artifacts/validation-report.json'
    Write-JsonFile -Path $validationReportPath -Value ([ordered]@{
        traceId = 'run-test'
        stageId = 'validate'
        agentId = 'tester'
        summary = [ordered]@{ totalChecks = 1; failedChecks = 0; overallStatus = 'passed' }
        checks = @()
    })

    $reviewInputManifestPath = Join-Path $runDirectory 'stages/review-input.json'
    $reviewInputManifest = [ordered]@{
        traceId = 'run-test'
        stageId = 'review'
        agentId = 'reviewer'
        producedAt = (Get-Date).ToString('o')
        artifacts = @(
            [ordered]@{ name = 'changeset'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $implementArtifacts['changeset']) -replace '\\', '/' },
            [ordered]@{ name = 'validation-report'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $validationReportPath) -replace '\\', '/' }
        )
    }
    Write-JsonFile -Path $reviewInputManifestPath -Value $reviewInputManifest

    $reviewOutputManifestPath = Join-Path $runDirectory 'stages/review-output.json'
    $reviewStageScript = Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/review-stage.ps1'
    & $reviewStageScript -RepoRoot $resolvedRepoRoot -RunDirectory $runDirectory -TraceId 'run-test' -StageId 'review' -AgentId 'reviewer' -RequestPath $requestPath -InputArtifactManifestPath $reviewInputManifestPath -OutputArtifactManifestPath $reviewOutputManifestPath -DispatchMode 'codex-exec' -PromptTemplatePath '.codex/orchestration/prompts/reviewer-stage.prompt.md' -ResponseSchemaPath '.github/schemas/agent.stage-review-result.schema.json' -DispatchCommand $fakeCodexPath -ExecutionBackend 'codex-exec' | Out-Null
    $reviewExit = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
    Assert-Equal -Actual $reviewExit -Expected 0 -Message 'Review stage should succeed with fake Codex.'

    $reviewManifest = Get-Content -Raw -LiteralPath $reviewOutputManifestPath | ConvertFrom-Json -Depth 100
    Assert-Equal -Actual @($reviewManifest.artifacts).Count -Expected 2 -Message 'Review stage should produce both review artifacts.'

    Write-Host '[OK] agent orchestration engine tests passed.'
    exit 0
}
catch {
    $message = $_.Exception.Message
    $trace = $_.ScriptStackTrace
    if ([string]::IsNullOrWhiteSpace($trace)) {
        Write-Host ("[FAIL] agent orchestration engine tests failed: {0}" -f $message)
    }
    else {
        Write-Host ("[FAIL] agent orchestration engine tests failed: {0}`n{1}" -f $message, $trace)
    }
    exit 1
}
finally {
    if (Test-Path -LiteralPath $tempRoot) {
        Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
    }
}