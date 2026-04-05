//! Deterministic fake Codex runner used by the parity harness.

use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn create_fake_codex_runner(temp_root: &Path) -> PathBuf {
    let runner_path = temp_root.join("fake-codex-runner.ps1");
    let command_path = temp_root.join("fake-codex.cmd");

    fs::write(&runner_path, fake_codex_runner_script()).expect("fake runner should be written");
    fs::write(&command_path, fake_codex_command_wrapper(&runner_path))
        .expect("fake command wrapper should be written");

    command_path
}

fn fake_codex_command_wrapper(runner_path: &Path) -> String {
    format!(
        "@echo off\r\nsetlocal\r\nset FAKE_CODEX_ARGS=%*\r\npwsh -NoLogo -NoProfile -ExecutionPolicy Bypass -File \"{}\" -RawArgs \"%FAKE_CODEX_ARGS%\"\r\n",
        runner_path.display()
    )
}

fn fake_codex_runner_script() -> &'static str {
    r###"
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
        $workstreamSlug = 'implement-enterprise-orchestration-support'
        if ($allInput -match 'Implement closeout smoke orchestration support\.') {
            $workstreamSlug = 'implement-closeout-smoke-orchestration-support'
        }

        $payload = [ordered]@{
            stage = 'brainstorm-spec'
            status = 'required'
            specRequired = $true
            workstreamSlug = $workstreamSlug
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
                            command = 'ntk validation runtime-script-tests --repo-root . --warning-only false'
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
                            command = 'ntk validation runtime-script-tests --repo-root . --warning-only false'
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
                            command = 'ntk validation runtime-script-tests --repo-root . --warning-only false'
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
                            command = 'ntk validation runtime-script-tests --repo-root . --warning-only false'
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
                '.github/instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md'
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
        $normalizedRequest = 'Implement enterprise orchestration support.'
        $workstreamSlug = 'implement-enterprise-orchestration-support'
        if ($allInput -match 'Implement closeout smoke orchestration support\.') {
            $normalizedRequest = 'Implement closeout smoke orchestration support.'
            $workstreamSlug = 'implement-closeout-smoke-orchestration-support'
        }

        $payload = [ordered]@{
            stage = 'super-agent-intake'
            normalizedRequest = $normalizedRequest
            changeBearing = $true
            planningRequired = $true
            clarificationRequired = $false
            canProceedSafely = $true
            workstreamSlug = $workstreamSlug
            explicitWorkItems = @('Normalize request', 'Plan execution')
            clarificationQuestions = @()
            clarificationReason = $null
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
                    path = '.temp/agent-orchestration-engine-smoke/README.md'
                    summary = 'Refresh the temporary README from the closeout stage.'
                    content = "# Runtime Smoke README`n`nCloseout automation updated this README during the orchestration smoke test."
                }
            )
            commitMessage = 'feat: close orchestration smoke test'
            changelogSummary = 'Close the orchestration smoke test plan.'
            changelogUpdate = [ordered]@{
                apply = $true
                path = '.temp/agent-orchestration-engine-smoke/CHANGELOG.md'
                summary = 'Record closeout documentation automation coverage in the temporary changelog.'
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
"###
}