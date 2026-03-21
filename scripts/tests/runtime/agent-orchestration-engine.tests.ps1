<#
.SYNOPSIS
    Runtime tests for the multi-agent orchestration engine without external frameworks.

.DESCRIPTION
    Validates end-to-end scripted orchestration behavior, artifact generation,
    and closeout side effects for the repository-owned agent pipeline.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/agent-orchestration-engine.tests.ps1

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$script:ScriptRoot = Split-Path -Path $PSCommandPath -Parent
$script:RepositoryHelpersPath = Join-Path $script:ScriptRoot '..\..\common\repository-paths.ps1'
if (-not (Test-Path -LiteralPath $script:RepositoryHelpersPath -PathType Leaf)) {
    throw "Missing shared repository helper: $script:RepositoryHelpersPath"
}
. $script:RepositoryHelpersPath
# Fails the current test when the supplied condition is false.
function Assert-True {
    param(
        [bool] $Condition,
        [string] $Message
    )

    if (-not $Condition) {
        throw $Message
    }
}

# Fails the current test when the actual and expected values differ.
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

# Writes deterministic JSON test content to disk.
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
$createdCompletedSpecPath = $null
$originalPlanningReadmeContent = $null
$originalChangelogContent = $null

try {
    New-Item -ItemType Directory -Path $tempRoot -Force | Out-Null
    $planningReadmePath = Join-Path $resolvedRepoRoot 'planning/README.md'
    $changelogPath = Join-Path $resolvedRepoRoot 'CHANGELOG.md'
    $originalPlanningReadmeContent = Get-Content -Raw -LiteralPath $planningReadmePath
    $originalChangelogContent = Get-Content -Raw -LiteralPath $changelogPath
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

    if ($allInput -match '# Specification Stage Contract') {
        $payload = [ordered]@{
            stage = 'brainstorm-spec'
            status = 'required'
            specRequired = $true
            workstreamSlug = 'implement-enterprise-orchestration-support'
            specSummary = 'A versioned design checkpoint is required before planning.'
            designDecisions = @(
                'Keep a versioned spec separate from the execution plan.',
                'Lock the normalized request before creating work items.'
            )
            alternativesConsidered = @('Skipping the spec and planning directly from intake.')
            assumptions = @('Repository policy remains authoritative.')
            risks = @('Skipping the spec would blur design and execution intent.')
            acceptanceCriteria = @('A spec file exists.', 'Planning receives the spec summary.')
            planningReadiness = 'ready-for-plan'
            recommendedSpecialists = @('dev-dotnet-backend-engineer')
            notes = @('Mock spec agent prepared the workstream for planning.')
        }
    }
    elseif ($allInput -match '# Planner Stage Contract') {
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
                    targetPaths = @('scripts/orchestration/stages/plan-stage.ps1')
                    commands = @(
                        [ordered]@{
                            purpose = 'targeted validation'
                            command = 'pwsh -File scripts/validation/validate-runtime-script-tests.ps1 -RepoRoot . -WarningOnly:$false'
                            expectedOutcome = 'Runtime tests pass after the first task.'
                        }
                    )
                    checkpoints = @(
                        [ordered]@{
                            name = 'first-task-baseline'
                            expectedOutcome = 'expected-verified'
                            evidence = 'Planner confirmed the first task scope and target file before implementation.'
                        },
                        [ordered]@{
                            name = 'first-task-green'
                            expectedOutcome = 'expected-pass'
                            command = 'pwsh -File scripts/validation/validate-runtime-script-tests.ps1 -RepoRoot . -WarningOnly:$false'
                            evidence = 'Runtime tests pass after the first task.'
                        }
                    )
                    commitCheckpoint = [ordered]@{
                        scope = 'task'
                        when = 'After the first task is validated.'
                        suggestedMessage = 'feat: complete first orchestration task checkpoint'
                    }
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
                    targetPaths = @('scripts/orchestration/stages/implement-stage.ps1')
                    commands = @(
                        [ordered]@{
                            purpose = 'targeted validation'
                            command = 'pwsh -File scripts/validation/validate-runtime-script-tests.ps1 -RepoRoot . -WarningOnly:$false'
                            expectedOutcome = 'Runtime tests pass after the second task.'
                        }
                    )
                    checkpoints = @(
                        [ordered]@{
                            name = 'second-task-baseline'
                            expectedOutcome = 'expected-verified'
                            evidence = 'Planner confirmed the second task scope and target file before implementation.'
                        },
                        [ordered]@{
                            name = 'second-task-green'
                            expectedOutcome = 'expected-pass'
                            command = 'pwsh -File scripts/validation/validate-runtime-script-tests.ps1 -RepoRoot . -WarningOnly:$false'
                            evidence = 'Runtime tests pass after the second task.'
                        }
                    )
                    commitCheckpoint = [ordered]@{
                        scope = 'slice'
                        when = 'After the second task is validated and the slice is stable.'
                        suggestedMessage = 'feat: complete second orchestration task checkpoint'
                    }
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
    elseif ($allInput -match '# Super Agent Intake Stage Contract') {
        $payload = [ordered]@{
            stage = 'super-agent-intake'
            normalizedRequest = 'Implement enterprise orchestration support.'
            changeBearing = $true
            planningRequired = $true
            workstreamSlug = 'implement-enterprise-orchestration-support'
            explicitWorkItems = @('Normalize request', 'Plan execution')
            constraints = @('Preserve repository policy and validation gates.')
            risks = @('Skipping planning would violate the lifecycle.')
            notes = @('Use sequential execution unless the planner proves tasks are parallel-safe.')
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
    elseif ($allInput -match '# Task Spec Review Contract') {
        $payload = [ordered]@{
            reviewType = 'spec-compliance'
            decision = 'approved'
            summary = 'Mock spec review approved the task.'
            findings = @()
            followUps = @()
        }
    }
    elseif ($allInput -match '# Task Code Quality Review Contract') {
        $payload = [ordered]@{
            reviewType = 'code-quality'
            decision = 'approved'
            summary = 'Mock code-quality review approved the task.'
            findings = @()
            followUps = @()
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
            readmeUpdates = @(
                [ordered]@{
                    path = 'planning/README.md'
                    summary = 'Refresh planning README from the closeout stage.'
                    content = "# Planning Workspace`n`nCloseout automation updated this README during the orchestration smoke test."
                }
            )
            commitMessage = 'feat: close orchestration smoke test'
            changelogSummary = 'Close the orchestration smoke test plan.'
            changelogUpdate = [ordered]@{
                apply = $true
                path = 'CHANGELOG.md'
                summary = 'Record closeout documentation automation coverage.'
                entry = "## [9.9.9] - 2026-03-20`n`n### Changed`n- Added smoke-test coverage for closeout-driven README and CHANGELOG updates."
            }
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

    $intakeOutputManifestPath = Join-Path $runDirectory 'stages/intake-output.json'
    & (Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/intake-stage.ps1') `
        -RepoRoot $resolvedRepoRoot `
        -RunDirectory $runDirectory `
        -TraceId 'run-test' `
        -StageId 'intake' `
        -AgentId 'super-agent' `
        -RequestPath $requestPath `
        -OutputArtifactManifestPath $intakeOutputManifestPath `
        -DispatchMode 'codex-exec' `
        -PromptTemplatePath '.codex/orchestration/prompts/super-agent-intake-stage.prompt.md' `
        -ResponseSchemaPath '.github/schemas/agent.stage-intake-result.schema.json' `
        -DispatchCommand $fakeCodexPath `
        -ExecutionBackend 'codex-exec' | Out-Null
    Assert-Equal -Actual ([int] $LASTEXITCODE) -Expected 0 -Message 'Intake stage should succeed with fake Codex.'

    $intakeManifest = Get-Content -Raw -LiteralPath $intakeOutputManifestPath | ConvertFrom-Json -Depth 100
    $intakeArtifacts = @{}
    foreach ($artifact in @($intakeManifest.artifacts)) {
        $intakeArtifacts[[string] $artifact.name] = Join-Path $resolvedRepoRoot ([string] $artifact.path)
    }
    $intakeReport = Get-Content -Raw -LiteralPath $intakeArtifacts['intake-report'] | ConvertFrom-Json -Depth 100
    Assert-True ([bool] $intakeReport.planningRequired) 'Intake stage should require planning in the fake flow.'

    $specOutputManifestPath = Join-Path $runDirectory 'stages/spec-output.json'
    & (Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/spec-stage.ps1') `
        -RepoRoot $resolvedRepoRoot `
        -RunDirectory $runDirectory `
        -TraceId 'run-test' `
        -StageId 'spec' `
        -AgentId 'brainstormer' `
        -RequestPath $requestPath `
        -InputArtifactManifestPath $intakeOutputManifestPath `
        -OutputArtifactManifestPath $specOutputManifestPath `
        -DispatchMode 'codex-exec' `
        -PromptTemplatePath '.codex/orchestration/prompts/spec-stage.prompt.md' `
        -ResponseSchemaPath '.github/schemas/agent.stage-spec-result.schema.json' `
        -DispatchCommand $fakeCodexPath `
        -ExecutionBackend 'codex-exec' | Out-Null
    Assert-Equal -Actual ([int] $LASTEXITCODE) -Expected 0 -Message 'Spec stage should succeed with fake Codex.'

    $specManifest = Get-Content -Raw -LiteralPath $specOutputManifestPath | ConvertFrom-Json -Depth 100
    $specArtifacts = @{}
    foreach ($artifact in @($specManifest.artifacts)) {
        $specArtifacts[[string] $artifact.name] = Join-Path $resolvedRepoRoot ([string] $artifact.path)
    }
    $specSummary = Get-Content -Raw -LiteralPath $specArtifacts['spec-summary'] | ConvertFrom-Json -Depth 100
    Assert-True ([bool] $specSummary.specRequired) 'Spec stage should require a versioned spec in the fake flow.'
    Assert-True (Test-Path -LiteralPath $specArtifacts['active-spec'] -PathType Leaf) 'Spec stage should produce an active spec file.'

    $planOutputManifestPath = Join-Path $runDirectory 'stages/plan-output.json'
    & (Join-Path $resolvedRepoRoot 'scripts/orchestration/stages/plan-stage.ps1') `
        -RepoRoot $resolvedRepoRoot `
        -RunDirectory $runDirectory `
        -TraceId 'run-test' `
        -StageId 'plan' `
        -AgentId 'planner' `
        -RequestPath $requestPath `
        -InputArtifactManifestPath $specOutputManifestPath `
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
    Assert-Equal -Actual @($taskPlanData.workItems[0].targetPaths).Count -Expected 1 -Message 'Plan stage should emit target paths per work item.'
    Assert-Equal -Actual @($taskPlanData.workItems[0].commands).Count -Expected 1 -Message 'Plan stage should emit explicit commands per work item.'
    Assert-Equal -Actual @($taskPlanData.workItems[0].checkpoints).Count -Expected 2 -Message 'Plan stage should emit checkpoints per work item.'
    Assert-Equal -Actual ([string] $taskPlanData.workItems[0].commitCheckpoint.scope) -Expected 'task' -Message 'Plan stage should emit commit checkpoints per work item.'
    Assert-True (Test-Path -LiteralPath $planArtifacts['active-plan'] -PathType Leaf) 'Plan stage should produce an active plan file.'

    $routeInputManifestPath = Join-Path $runDirectory 'stages/route-input.json'
    Write-JsonFile -Path $routeInputManifestPath -Value ([ordered]@{
            traceId = 'run-test'
            stageId = 'route'
            agentId = 'router'
            producedAt = (Get-Date).ToString('o')
            artifacts = @($intakeManifest.artifacts + $specManifest.artifacts + $planManifest.artifacts)
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
            artifacts = @($intakeManifest.artifacts + $specManifest.artifacts + $planManifest.artifacts + $routeManifest.artifacts)
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
    Assert-True ($implementArtifacts.ContainsKey('task-review-report')) 'Implement stage should emit the task-review-report artifact.'

    $validateInputManifestPath = Join-Path $runDirectory 'stages/validate-input.json'
    Write-JsonFile -Path $validateInputManifestPath -Value ([ordered]@{
            traceId = 'run-test'
            stageId = 'validate'
            agentId = 'tester'
            producedAt = (Get-Date).ToString('o')
            artifacts = @($implementManifest.artifacts + $routeManifest.artifacts + $planManifest.artifacts)
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
                [ordered]@{ name = 'spec-summary'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $specArtifacts['spec-summary']) -replace '\\', '/' },
                [ordered]@{ name = 'active-spec'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $specArtifacts['active-spec']) -replace '\\', '/' },
                [ordered]@{ name = 'changeset'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $implementArtifacts['changeset']) -replace '\\', '/' },
                [ordered]@{ name = 'validation-report'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $validateArtifacts['validation-report']) -replace '\\', '/' },
                [ordered]@{ name = 'route-selection'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $routeArtifacts['route-selection']) -replace '\\', '/' },
                [ordered]@{ name = 'task-review-report'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $implementArtifacts['task-review-report']) -replace '\\', '/' },
                [ordered]@{ name = 'active-plan'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $planArtifacts['active-plan']) -replace '\\', '/' }
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
                [ordered]@{ name = 'spec-summary'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $specArtifacts['spec-summary']) -replace '\\', '/' },
                [ordered]@{ name = 'active-spec'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $specArtifacts['active-spec']) -replace '\\', '/' },
                [ordered]@{ name = 'route-selection'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $routeArtifacts['route-selection']) -replace '\\', '/' },
                [ordered]@{ name = 'changeset'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $implementArtifacts['changeset']) -replace '\\', '/' },
                [ordered]@{ name = 'validation-report'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $validateArtifacts['validation-report']) -replace '\\', '/' },
                [ordered]@{ name = 'task-review-report'; path = [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $implementArtifacts['task-review-report']) -replace '\\', '/' },
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
    $readmeUpdatesReport = Get-Content -Raw -LiteralPath $closeoutArtifacts['readme-updates'] | ConvertFrom-Json -Depth 100
    $changelogUpdateReport = Get-Content -Raw -LiteralPath $closeoutArtifacts['changelog-update'] | ConvertFrom-Json -Depth 100
    $completedPlanMetadata = Get-Content -Raw -LiteralPath $closeoutArtifacts['completed-plan'] | ConvertFrom-Json -Depth 100
    if (-not [string]::IsNullOrWhiteSpace([string] $completedPlanMetadata.completedPlanPath)) {
        $createdCompletedPlanPath = Join-Path $resolvedRepoRoot ([string] $completedPlanMetadata.completedPlanPath)
    }
    if (-not [string]::IsNullOrWhiteSpace([string] $completedPlanMetadata.completedSpecPath)) {
        $createdCompletedSpecPath = Join-Path $resolvedRepoRoot ([string] $completedPlanMetadata.completedSpecPath)
    }
    Assert-Equal -Actual $closeoutReport.status -Expected 'ready-for-commit' -Message 'Closeout stage should be commit-ready in fake flow.'
    Assert-True ([bool] $readmeUpdatesReport.updated) 'Closeout stage should report applied README updates in fake flow.'
    Assert-True ([bool] $changelogUpdateReport.applied) 'Closeout stage should report an applied changelog update in fake flow.'
    Assert-True ((Get-Content -Raw -LiteralPath $planningReadmePath) -match 'Closeout automation updated this README') 'Closeout stage should rewrite the planning README in fake flow.'
    Assert-True ((Get-Content -Raw -LiteralPath $changelogPath).StartsWith("## [9.9.9] - 2026-03-20", [System.StringComparison]::Ordinal)) 'Closeout stage should prepend the changelog entry in fake flow.'
    Assert-True (-not (Test-Path -LiteralPath $planArtifacts['active-plan'] -PathType Leaf)) 'Closeout stage should move the active plan out of planning/active.'
    Assert-True (-not (Test-Path -LiteralPath $specArtifacts['active-spec'] -PathType Leaf)) 'Closeout stage should move the active spec out of planning/specs/active.'

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
    if ($null -ne $originalPlanningReadmeContent) {
        Set-Content -LiteralPath (Join-Path $resolvedRepoRoot 'planning/README.md') -Value $originalPlanningReadmeContent -Encoding UTF8 -NoNewline
    }
    if ($null -ne $originalChangelogContent) {
        Set-Content -LiteralPath (Join-Path $resolvedRepoRoot 'CHANGELOG.md') -Value $originalChangelogContent -Encoding UTF8 -NoNewline
    }
    if (-not [string]::IsNullOrWhiteSpace($createdCompletedPlanPath) -and (Test-Path -LiteralPath $createdCompletedPlanPath -PathType Leaf)) {
        Remove-Item -LiteralPath $createdCompletedPlanPath -Force -ErrorAction SilentlyContinue
    }
    if (-not [string]::IsNullOrWhiteSpace($createdCompletedSpecPath) -and (Test-Path -LiteralPath $createdCompletedSpecPath -PathType Leaf)) {
        Remove-Item -LiteralPath $createdCompletedSpecPath -Force -ErrorAction SilentlyContinue
    }

    if (Test-Path -LiteralPath $tempRoot) {
        Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
    }
}