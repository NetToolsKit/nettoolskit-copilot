<#
.SYNOPSIS
    Runtime tests for the multi-agent orchestration engine without external frameworks.
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
$createdCompletedPlanPath = $null

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
    elseif ($allInput -match '# Router Stage Contract') {
        $payload = [ordered]@{
            summary = 'Router selected a backend-oriented specialist focus.'
            recommendedSpecialistSkill = 'dev-dotnet-backend-engineer'
            recommendedSpecialistFocus = '.NET backend implementation and validation.'
            contextPaths = @(
                '.github/AGENTS.md',
                '.github/copilot-instructions.md',
                '.github/instructions/subagent-planning-workflow.instructions.md'
            )
            tokenBudgetGuidance = @('Load only routed files.', 'Keep prompts focused on the current work item.')
            executionNotes = @('Apply the routed focus to all work items.')
            validationFocus = @('Run validation after implementation.')
            closeoutExpectations = @('Prepare commit and changelog summary.')
            shouldRunTester = $true
            readmeImpact = $true
            changelogImpact = $true
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
    elseif ($allInput -match '# Closeout Stage Contract') {
        $payload = [ordered]@{
            status = 'ready-for-commit'
            summary = 'Mock closeout produced commit-ready artifacts.'
            readmeActions = @('README already aligned for mock flow.')
            commitMessage = 'feat: close orchestration smoke test'
            changelogSummary = 'Close the orchestration smoke test plan.'
            followUps = @()
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

    $runDirectory = Join-Path $tempRoot 'run'
    New-Item -ItemType Directory -Path (Join-Path $runDirectory 'artifacts') -Force | Out-Null
    New-Item -ItemType Directory -Path (Join-Path $runDirectory 'stages') -Force | Out-Null

    $requestPath = Join-Path $runDirectory 'artifacts/request.md'
    Set-Content -LiteralPath $requestPath -Value 'Implement enterprise orchestration support.' -Encoding UTF8 -NoNewline

    $planOutputManifestPath = Join-Path $runDirectory 'stages/plan-output.json'
    & (Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/plan-stage.ps1') `
        -RepoRoot $resolvedRepoRoot `
        -RunDirectory $runDirectory `
        -TraceId 'run-test' `
        -StageId 'plan' `
        -AgentId 'planner' `
        -RequestPath $requestPath `
        -OutputArtifactManifestPath $planOutputManifestPath `
        -DispatchMode 'codex-exec' `
        -PromptTemplatePath '.codex/orchestration/prompts/planner-stage.prompt.md' `
        -ResponseSchemaPath '.github/schemas/agent.stage-plan-result.schema.json' `
        -DispatchCommand $fakeCodexPath `
        -ExecutionBackend 'codex-exec' | Out-Null
    Assert-Equal -Actual ([int] $LASTEXITCODE) -Expected 0 -Message 'Plan stage should succeed with fake Codex.'

    $planManifest = Get-Content -Raw -LiteralPath $planOutputManifestPath | ConvertFrom-Json -Depth 100
    $planArtifacts = @{}
    foreach ($artifact in @($planManifest.artifacts)) {
        $planArtifacts[[string] $artifact.name] = Join-Path $resolvedRepoRoot ([string] $artifact.path)
    }
    $taskPlanData = Get-Content -Raw -LiteralPath $planArtifacts['task-plan-data'] | ConvertFrom-Json -Depth 100
    Assert-Equal -Actual @($taskPlanData.workItems).Count -Expected 2 -Message 'Plan stage should produce two work items in fake flow.'
    Assert-True (Test-Path -LiteralPath $planArtifacts['active-plan'] -PathType Leaf) 'Plan stage should produce an active plan file.'

    $routeInputManifestPath = Join-Path $runDirectory 'stages/route-input.json'
    Write-JsonFile -Path $routeInputManifestPath -Value ([ordered]@{
            traceId = 'run-test'
            stageId = 'route'
            agentId = 'router'
            producedAt = (Get-Date).ToString('o')
            artifacts = @($planManifest.artifacts)
        })
    $routeOutputManifestPath = Join-Path $runDirectory 'stages/route-output.json'
    & (Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/route-stage.ps1') `
        -RepoRoot $resolvedRepoRoot `
        -RunDirectory $runDirectory `
        -TraceId 'run-test' `
        -StageId 'route' `
        -AgentId 'router' `
        -RequestPath $requestPath `
        -InputArtifactManifestPath $routeInputManifestPath `
        -OutputArtifactManifestPath $routeOutputManifestPath `
        -DispatchMode 'codex-exec' `
        -PromptTemplatePath '.codex/orchestration/prompts/router-stage.prompt.md' `
        -ResponseSchemaPath '.github/schemas/agent.stage-route-result.schema.json' `
        -DispatchCommand $fakeCodexPath `
        -ExecutionBackend 'codex-exec' | Out-Null
    Assert-Equal -Actual ([int] $LASTEXITCODE) -Expected 0 -Message 'Route stage should succeed with fake Codex.'

    $routeManifest = Get-Content -Raw -LiteralPath $routeOutputManifestPath | ConvertFrom-Json -Depth 100
    $routeArtifacts = @{}
    foreach ($artifact in @($routeManifest.artifacts)) {
        $routeArtifacts[[string] $artifact.name] = Join-Path $resolvedRepoRoot ([string] $artifact.path)
    }
    $routeSelection = Get-Content -Raw -LiteralPath $routeArtifacts['route-selection'] | ConvertFrom-Json -Depth 100
    Assert-Equal -Actual $routeSelection.recommendedSpecialistSkill -Expected 'dev-dotnet-backend-engineer' -Message 'Route stage should surface the routed specialist skill.'

    $implementInputManifestPath = Join-Path $runDirectory 'stages/implement-input.json'
    Write-JsonFile -Path $implementInputManifestPath -Value ([ordered]@{
            traceId = 'run-test'
            stageId = 'implement'
            agentId = 'specialist'
            producedAt = (Get-Date).ToString('o')
            artifacts = @($planManifest.artifacts + $routeManifest.artifacts)
        })
    $implementOutputManifestPath = Join-Path $runDirectory 'stages/implement-output.json'
    & (Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/implement-stage.ps1') `
        -RepoRoot $resolvedRepoRoot `
        -RunDirectory $runDirectory `
        -TraceId 'run-test' `
        -StageId 'implement' `
        -AgentId 'specialist' `
        -RequestPath $requestPath `
        -InputArtifactManifestPath $implementInputManifestPath `
        -OutputArtifactManifestPath $implementOutputManifestPath `
        -DispatchMode 'codex-exec' `
        -PromptTemplatePath '.codex/orchestration/prompts/executor-task.prompt.md' `
        -ResponseSchemaPath '.github/schemas/agent.stage-implementation-result.schema.json' `
        -DispatchCommand $fakeCodexPath `
        -ExecutionBackend 'codex-exec' | Out-Null
    Assert-Equal -Actual ([int] $LASTEXITCODE) -Expected 0 -Message 'Implement stage should succeed with fake Codex.'

    $implementManifest = Get-Content -Raw -LiteralPath $implementOutputManifestPath | ConvertFrom-Json -Depth 100
    $implementArtifacts = @{}
    foreach ($artifact in @($implementManifest.artifacts)) {
        $implementArtifacts[[string] $artifact.name] = Join-Path $resolvedRepoRoot ([string] $artifact.path)
    }
    $dispatches = Get-Content -Raw -LiteralPath $implementArtifacts['implementation-dispatches'] | ConvertFrom-Json -Depth 100
    Assert-Equal -Actual @($dispatches.tasks).Count -Expected 2 -Message 'Implement stage should dispatch each planned work item.'

    $validateInputManifestPath = Join-Path $runDirectory 'stages/validate-input.json'
    Write-JsonFile -Path $validateInputManifestPath -Value ([ordered]@{
            traceId = 'run-test'
            stageId = 'validate'
            agentId = 'tester'
            producedAt = (Get-Date).ToString('o')
            artifacts = @($implementManifest.artifacts + $routeManifest.artifacts)
        })
    $validateOutputManifestPath = Join-Path $runDirectory 'stages/validate-output.json'
    & (Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/validate-stage.ps1') `
        -RepoRoot $resolvedRepoRoot `
        -RunDirectory $runDirectory `
        -TraceId 'run-test' `
        -StageId 'validate' `
        -AgentId 'tester' `
        -RequestPath $requestPath `
        -InputArtifactManifestPath $validateInputManifestPath `
        -OutputArtifactManifestPath $validateOutputManifestPath | Out-Null
    Assert-Equal -Actual ([int] $LASTEXITCODE) -Expected 0 -Message 'Validate stage should succeed.'

    $validateManifest = Get-Content -Raw -LiteralPath $validateOutputManifestPath | ConvertFrom-Json -Depth 100
    $validateArtifacts = @{}
    foreach ($artifact in @($validateManifest.artifacts)) {
        $validateArtifacts[[string] $artifact.name] = Join-Path $resolvedRepoRoot ([string] $artifact.path)
    }

    $reviewInputManifestPath = Join-Path $runDirectory 'stages/review-input.json'
    Write-JsonFile -Path $reviewInputManifestPath -Value ([ordered]@{
            traceId = 'run-test'
            stageId = 'review'
            agentId = 'reviewer'
            producedAt = (Get-Date).ToString('o')
            artifacts = @(
                [ordered]@{ name = 'changeset'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $implementArtifacts['changeset']) -replace '\\', '/' },
                [ordered]@{ name = 'validation-report'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $validateArtifacts['validation-report']) -replace '\\', '/' }
            )
        })
    $reviewOutputManifestPath = Join-Path $runDirectory 'stages/review-output.json'
    & (Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/review-stage.ps1') `
        -RepoRoot $resolvedRepoRoot `
        -RunDirectory $runDirectory `
        -TraceId 'run-test' `
        -StageId 'review' `
        -AgentId 'reviewer' `
        -RequestPath $requestPath `
        -InputArtifactManifestPath $reviewInputManifestPath `
        -OutputArtifactManifestPath $reviewOutputManifestPath `
        -DispatchMode 'codex-exec' `
        -PromptTemplatePath '.codex/orchestration/prompts/reviewer-stage.prompt.md' `
        -ResponseSchemaPath '.github/schemas/agent.stage-review-result.schema.json' `
        -DispatchCommand $fakeCodexPath `
        -ExecutionBackend 'codex-exec' | Out-Null
    Assert-Equal -Actual ([int] $LASTEXITCODE) -Expected 0 -Message 'Review stage should succeed with fake Codex.'

    $reviewManifest = Get-Content -Raw -LiteralPath $reviewOutputManifestPath | ConvertFrom-Json -Depth 100
    $reviewArtifacts = @{}
    foreach ($artifact in @($reviewManifest.artifacts)) {
        $reviewArtifacts[[string] $artifact.name] = Join-Path $resolvedRepoRoot ([string] $artifact.path)
    }

    $closeoutInputManifestPath = Join-Path $runDirectory 'stages/closeout-input.json'
    Write-JsonFile -Path $closeoutInputManifestPath -Value ([ordered]@{
            traceId = 'run-test'
            stageId = 'closeout'
            agentId = 'release-engineer'
            producedAt = (Get-Date).ToString('o')
            artifacts = @(
                [ordered]@{ name = 'route-selection'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $routeArtifacts['route-selection']) -replace '\\', '/' },
                [ordered]@{ name = 'changeset'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $implementArtifacts['changeset']) -replace '\\', '/' },
                [ordered]@{ name = 'validation-report'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $validateArtifacts['validation-report']) -replace '\\', '/' },
                [ordered]@{ name = 'review-report'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $reviewArtifacts['review-report']) -replace '\\', '/' },
                [ordered]@{ name = 'decision-log'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $reviewArtifacts['decision-log']) -replace '\\', '/' },
                [ordered]@{ name = 'active-plan'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $planArtifacts['active-plan']) -replace '\\', '/' }
            )
        })
    $closeoutOutputManifestPath = Join-Path $runDirectory 'stages/closeout-output.json'
    & (Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/closeout-stage.ps1') `
        -RepoRoot $resolvedRepoRoot `
        -RunDirectory $runDirectory `
        -TraceId 'run-test' `
        -StageId 'closeout' `
        -AgentId 'release-engineer' `
        -RequestPath $requestPath `
        -InputArtifactManifestPath $closeoutInputManifestPath `
        -OutputArtifactManifestPath $closeoutOutputManifestPath `
        -DispatchMode 'codex-exec' `
        -PromptTemplatePath '.codex/orchestration/prompts/closeout-stage.prompt.md' `
        -ResponseSchemaPath '.github/schemas/agent.stage-closeout-result.schema.json' `
        -DispatchCommand $fakeCodexPath `
        -ExecutionBackend 'codex-exec' | Out-Null
    Assert-Equal -Actual ([int] $LASTEXITCODE) -Expected 0 -Message 'Closeout stage should succeed with fake Codex.'

    $closeoutManifest = Get-Content -Raw -LiteralPath $closeoutOutputManifestPath | ConvertFrom-Json -Depth 100
    $closeoutArtifacts = @{}
    foreach ($artifact in @($closeoutManifest.artifacts)) {
        $closeoutArtifacts[[string] $artifact.name] = Join-Path $resolvedRepoRoot ([string] $artifact.path)
    }
    $closeoutReport = Get-Content -Raw -LiteralPath $closeoutArtifacts['closeout-report'] | ConvertFrom-Json -Depth 100
    $completedPlanMetadata = Get-Content -Raw -LiteralPath $closeoutArtifacts['completed-plan'] | ConvertFrom-Json -Depth 100
    if (-not [string]::IsNullOrWhiteSpace([string] $completedPlanMetadata.completedPlanPath)) {
        $createdCompletedPlanPath = Join-Path $resolvedRepoRoot ([string] $completedPlanMetadata.completedPlanPath)
    }
    Assert-Equal -Actual $closeoutReport.status -Expected 'ready-for-commit' -Message 'Closeout stage should be commit-ready in fake flow.'
    Assert-True (-not (Test-Path -LiteralPath $planArtifacts['active-plan'] -PathType Leaf)) 'Closeout stage should move the active plan out of plans-active.'

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
    if (-not [string]::IsNullOrWhiteSpace($createdCompletedPlanPath) -and (Test-Path -LiteralPath $createdCompletedPlanPath -PathType Leaf)) {
        Remove-Item -LiteralPath $createdCompletedPlanPath -Force -ErrorAction SilentlyContinue
    }

    $completedPlansGitKeepPath = Join-Path $resolvedRepoRoot '.temp/planning/plans-completed/.gitkeep'
    if (-not (Test-Path -LiteralPath $completedPlansGitKeepPath -PathType Leaf)) {
        New-Item -ItemType File -Path $completedPlansGitKeepPath -Force | Out-Null
    }

    if (Test-Path -LiteralPath $tempRoot) {
        Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
    }
}