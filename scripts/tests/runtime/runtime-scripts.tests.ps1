<#
.SYNOPSIS
    Runtime tests for critical scripts without external test frameworks.

.DESCRIPTION
    Validates script contracts and smoke behavior for runtime scripts used
    by bootstrap, healthcheck, self-heal, and cleanup flows.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/runtime-scripts.tests.ps1

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
. $script:CommonBootstrapPath -CallerScriptRoot $PSScriptRoot -Helpers @('repository-paths')
# Fails the current runtime test when the supplied condition is false.
function Assert-True {
    param(
        [bool] $Condition,
        [string] $Message
    )

    if (-not $Condition) {
        throw $Message
    }
}

# Fails the current test when the collection does not contain the expected value.
function Assert-Contains {
    param(
        [string[]] $Collection,
        [string] $Value,
        [string] $Message
    )

    if (-not ($Collection -contains $Value)) {
        throw $Message
    }
}

# Fails the current test when the collection does not contain any expected value.
function Assert-ContainsAny {
    param(
        [string[]] $Collection,
        [string[]] $ExpectedValues,
        [string] $Message
    )

    foreach ($expectedValue in @($ExpectedValues)) {
        if ($Collection -contains $expectedValue) {
            return
        }
    }

    throw $Message
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$runtimeScriptRoot = Join-Path $resolvedRepoRoot 'scripts/runtime'

$exitCode = 0

try {
    $scriptPath = Join-Path $runtimeScriptRoot 'bootstrap.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'RepoRoot' -Message 'bootstrap missing RepoRoot parameter.'
    Assert-Contains -Collection $keys -Value 'TargetGithubPath' -Message 'bootstrap missing TargetGithubPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetCodexPath' -Message 'bootstrap missing TargetCodexPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetAgentsSkillsPath' -Message 'bootstrap missing TargetAgentsSkillsPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetCopilotSkillsPath' -Message 'bootstrap missing TargetCopilotSkillsPath parameter.'
    Assert-Contains -Collection $keys -Value 'RuntimeProfile' -Message 'bootstrap missing RuntimeProfile parameter.'
    Assert-Contains -Collection $keys -Value 'Mirror' -Message 'bootstrap missing Mirror parameter.'

    $scriptPath = Join-Path $runtimeScriptRoot 'healthcheck.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'ValidationProfile' -Message 'healthcheck missing ValidationProfile parameter.'
    Assert-Contains -Collection $keys -Value 'WarningOnly' -Message 'healthcheck missing WarningOnly parameter.'
    Assert-Contains -Collection $keys -Value 'TargetGithubPath' -Message 'healthcheck missing TargetGithubPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetCodexPath' -Message 'healthcheck missing TargetCodexPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetAgentsSkillsPath' -Message 'healthcheck missing TargetAgentsSkillsPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetCopilotSkillsPath' -Message 'healthcheck missing TargetCopilotSkillsPath parameter.'
    Assert-Contains -Collection $keys -Value 'RuntimeProfile' -Message 'healthcheck missing RuntimeProfile parameter.'

    $scriptPath = Join-Path $runtimeScriptRoot 'self-heal.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'Mirror' -Message 'self-heal missing Mirror parameter.'
    Assert-Contains -Collection $keys -Value 'ApplyMcpConfig' -Message 'self-heal missing ApplyMcpConfig parameter.'
    Assert-Contains -Collection $keys -Value 'TargetGithubPath' -Message 'self-heal missing TargetGithubPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetCodexPath' -Message 'self-heal missing TargetCodexPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetAgentsSkillsPath' -Message 'self-heal missing TargetAgentsSkillsPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetCopilotSkillsPath' -Message 'self-heal missing TargetCopilotSkillsPath parameter.'
    Assert-Contains -Collection $keys -Value 'RuntimeProfile' -Message 'self-heal missing RuntimeProfile parameter.'

    $scriptPath = Join-Path $runtimeScriptRoot 'clean-codex-runtime.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'CodexHome' -Message 'clean-codex-runtime missing CodexHome parameter.'
    Assert-Contains -Collection $keys -Value 'IncludeSessions' -Message 'clean-codex-runtime missing IncludeSessions parameter.'
    Assert-Contains -Collection $keys -Value 'SessionRetentionDays' -Message 'clean-codex-runtime missing SessionRetentionDays parameter.'
    Assert-Contains -Collection $keys -Value 'LogRetentionDays' -Message 'clean-codex-runtime missing LogRetentionDays parameter.'

    $scriptPath = Join-Path $runtimeScriptRoot 'run-agent-pipeline.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'RequestText' -Message 'run-agent-pipeline missing RequestText parameter.'
    Assert-Contains -Collection $keys -Value 'ExecutionBackend' -Message 'run-agent-pipeline missing ExecutionBackend parameter.'
    Assert-Contains -Collection $keys -Value 'DispatchCommand' -Message 'run-agent-pipeline missing DispatchCommand parameter.'
    Assert-Contains -Collection $keys -Value 'ApprovedStageIds' -Message 'run-agent-pipeline missing ApprovedStageIds parameter.'
    Assert-Contains -Collection $keys -Value 'ApprovedAgentIds' -Message 'run-agent-pipeline missing ApprovedAgentIds parameter.'
    Assert-Contains -Collection $keys -Value 'ApprovedBy' -Message 'run-agent-pipeline missing ApprovedBy parameter.'
    Assert-Contains -Collection $keys -Value 'ApprovalJustification' -Message 'run-agent-pipeline missing ApprovalJustification parameter.'
    Assert-Contains -Collection $keys -Value 'WriteRunState' -Message 'run-agent-pipeline missing WriteRunState parameter.'
    Assert-Contains -Collection $keys -Value 'StopAfterStageId' -Message 'run-agent-pipeline missing StopAfterStageId parameter.'
    Assert-Contains -Collection $keys -Value 'StartAtStageId' -Message 'run-agent-pipeline missing StartAtStageId parameter.'
    Assert-Contains -Collection $keys -Value 'ResumeFromRunDirectory' -Message 'run-agent-pipeline missing ResumeFromRunDirectory parameter.'
    Assert-Contains -Collection $keys -Value 'PolicyCatalogPath' -Message 'run-agent-pipeline missing PolicyCatalogPath parameter.'
    Assert-Contains -Collection $keys -Value 'ModelRoutingCatalogPath' -Message 'run-agent-pipeline missing ModelRoutingCatalogPath parameter.'

    $scriptPath = Join-Path $runtimeScriptRoot 'resume-agent-pipeline.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'RunDirectory' -Message 'resume-agent-pipeline missing RunDirectory parameter.'
    Assert-Contains -Collection $keys -Value 'StartAtStageId' -Message 'resume-agent-pipeline missing StartAtStageId parameter.'
    Assert-Contains -Collection $keys -Value 'ApprovedAgentIds' -Message 'resume-agent-pipeline missing ApprovedAgentIds parameter.'

    $scriptPath = Join-Path $runtimeScriptRoot 'replay-agent-run.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'RunDirectory' -Message 'replay-agent-run missing RunDirectory parameter.'
    Assert-Contains -Collection $keys -Value 'OutputPath' -Message 'replay-agent-run missing OutputPath parameter.'

    $scriptPath = Join-Path $runtimeScriptRoot 'evaluate-agent-pipeline.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'EvalsPath' -Message 'evaluate-agent-pipeline missing EvalsPath parameter.'
    Assert-Contains -Collection $keys -Value 'OutputPath' -Message 'evaluate-agent-pipeline missing OutputPath parameter.'

    $scriptPath = Join-Path $resolvedRepoRoot 'scripts/git-hooks/setup-global-git-aliases.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'RepoRoot' -Message 'setup-global-git-aliases missing RepoRoot parameter.'
    Assert-Contains -Collection $keys -Value 'TargetCodexPath' -Message 'setup-global-git-aliases missing TargetCodexPath parameter.'
    Assert-Contains -Collection $keys -Value 'Uninstall' -Message 'setup-global-git-aliases missing Uninstall parameter.'

    $scriptPath = Join-Path $resolvedRepoRoot 'scripts/git-hooks/setup-git-hooks.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'RepoRoot' -Message 'setup-git-hooks missing RepoRoot parameter.'
    Assert-Contains -Collection $keys -Value 'EofHygieneMode' -Message 'setup-git-hooks missing EofHygieneMode parameter.'
    Assert-Contains -Collection $keys -Value 'EofHygieneScope' -Message 'setup-git-hooks missing EofHygieneScope parameter.'
    Assert-Contains -Collection $keys -Value 'Uninstall' -Message 'setup-git-hooks missing Uninstall parameter.'

    $scriptPath = Join-Path $runtimeScriptRoot 'install.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'RuntimeProfile' -Message 'install missing RuntimeProfile parameter.'
    Assert-Contains -Collection $keys -Value 'GitHookEofMode' -Message 'install missing GitHookEofMode parameter.'
    Assert-Contains -Collection $keys -Value 'GitHookEofScope' -Message 'install missing GitHookEofScope parameter.'

    $operationalScriptDirectories = @(
        (Join-Path $resolvedRepoRoot 'scripts/runtime'),
        (Join-Path $resolvedRepoRoot 'scripts/validation'),
        (Join-Path $resolvedRepoRoot 'scripts/git-hooks'),
        (Join-Path $resolvedRepoRoot 'scripts/governance'),
        (Join-Path $resolvedRepoRoot 'scripts/maintenance'),
        (Join-Path $resolvedRepoRoot 'scripts/security')
    )
    foreach ($scriptDirectory in $operationalScriptDirectories) {
        foreach ($operationalScript in @(Get-ChildItem -LiteralPath $scriptDirectory -Filter *.ps1 -File | Sort-Object Name)) {
            $command = Get-Command -Name $operationalScript.FullName -ErrorAction Stop
            $keys = @($command.Parameters.Keys)
            Assert-ContainsAny -Collection $keys -ExpectedValues @('Verbose', 'DetailedLogs', 'DetailedOutput') -Message ("Operational script is missing a verbose-style parameter: {0}" -f $operationalScript.FullName)
        }
    }

    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    $targetGithub = Join-Path $tempRoot '.github'
    $targetCodex = Join-Path $tempRoot '.codex'
    $targetAgentsSkills = Join-Path $tempRoot '.agents\skills'
    $targetCopilotSkills = Join-Path $tempRoot '.copilot\skills'
    $targetCodexSkills = Join-Path $targetCodex 'skills'
    $scriptPath = Join-Path $runtimeScriptRoot 'bootstrap.ps1'
    try {
        New-Item -ItemType Directory -Path (Join-Path $targetCodexSkills 'super-agent') -Force | Out-Null
        New-Item -ItemType Directory -Path (Join-Path $targetCodexSkills '.system') -Force | Out-Null
        Set-Content -LiteralPath (Join-Path $targetCodexSkills 'super-agent\SKILL.md') -Value 'duplicate'
        Set-Content -LiteralPath (Join-Path $targetCodexSkills '.system\SKILL.md') -Value 'system'

        & $scriptPath -RepoRoot $resolvedRepoRoot -TargetGithubPath $targetGithub -TargetCodexPath $targetCodex -TargetAgentsSkillsPath $targetAgentsSkills -TargetCopilotSkillsPath $targetCopilotSkills -Mirror | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-True ($exitCode -eq 0) 'bootstrap smoke test failed.'
        Assert-True (Test-Path -LiteralPath $targetGithub -PathType Container) 'bootstrap did not create target github folder.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetGithub 'agents\super-agent.agent.md') -PathType Leaf) 'bootstrap did not project the repository-owned Copilot agent profile.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetAgentsSkills 'super-agent\SKILL.md') -PathType Leaf) 'bootstrap did not project picker-visible skills.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetCopilotSkills 'super-agent\SKILL.md') -PathType Leaf) 'bootstrap did not project native Copilot skills.'
        Assert-True (-not (Test-Path -LiteralPath (Join-Path $targetCodexSkills 'super-agent'))) 'bootstrap did not remove duplicate repo-managed super-agent from .codex/skills.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetCodexSkills '.system\SKILL.md') -PathType Leaf) 'bootstrap should preserve unmanaged/system skills in .codex/skills.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetCodex 'shared-scripts') -PathType Container) 'bootstrap did not sync shared-scripts folder.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetCodex 'shared-scripts\maintenance\trim-trailing-blank-lines.ps1') -PathType Leaf) 'bootstrap did not project maintenance trim script into shared-scripts.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    $targetGithub = Join-Path $tempRoot '.github'
    $targetCodex = Join-Path $tempRoot '.codex'
    $targetAgentsSkills = Join-Path $tempRoot '.agents\skills'
    $targetCopilotSkills = Join-Path $tempRoot '.copilot\skills'
    $scriptPath = Join-Path $runtimeScriptRoot 'bootstrap.ps1'
    try {
        & $scriptPath -RepoRoot $resolvedRepoRoot -TargetGithubPath $targetGithub -TargetCodexPath $targetCodex -TargetAgentsSkillsPath $targetAgentsSkills -TargetCopilotSkillsPath $targetCopilotSkills -RuntimeProfile github | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-True ($exitCode -eq 0) 'bootstrap github-profile smoke test failed.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetGithub 'agents\super-agent.agent.md') -PathType Leaf) 'bootstrap github profile must project repository-owned GitHub runtime files.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetCopilotSkills 'super-agent\SKILL.md') -PathType Leaf) 'bootstrap github profile must project native Copilot skills.'
        Assert-True (-not (Test-Path -LiteralPath $targetAgentsSkills)) 'bootstrap github profile must not project Codex picker-visible skills.'
        Assert-True (-not (Test-Path -LiteralPath (Join-Path $targetCodex 'shared-scripts'))) 'bootstrap github profile must not project Codex shared scripts.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    $targetGithub = Join-Path $tempRoot '.github'
    $targetCodex = Join-Path $tempRoot '.codex'
    $targetAgentsSkills = Join-Path $tempRoot '.agents\skills'
    $targetCopilotSkills = Join-Path $tempRoot '.copilot\skills'
    $scriptPath = Join-Path $runtimeScriptRoot 'bootstrap.ps1'
    try {
        & $scriptPath -RepoRoot $resolvedRepoRoot -TargetGithubPath $targetGithub -TargetCodexPath $targetCodex -TargetAgentsSkillsPath $targetAgentsSkills -TargetCopilotSkillsPath $targetCopilotSkills -RuntimeProfile codex | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-True ($exitCode -eq 0) 'bootstrap codex-profile smoke test failed.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetAgentsSkills 'super-agent\SKILL.md') -PathType Leaf) 'bootstrap codex profile must project picker-visible skills.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetCodex 'shared-scripts\maintenance\trim-trailing-blank-lines.ps1') -PathType Leaf) 'bootstrap codex profile must project Codex shared scripts.'
        Assert-True (-not (Test-Path -LiteralPath $targetGithub)) 'bootstrap codex profile must not project the GitHub runtime root.'
        Assert-True (-not (Test-Path -LiteralPath $targetCopilotSkills)) 'bootstrap codex profile must not project native Copilot skills.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    $codexHome = Join-Path $tempRoot '.codex'
    $tmpDir = Join-Path $codexHome 'tmp'
    $logDir = Join-Path $codexHome 'log'
    $sessionsDir = Join-Path $codexHome 'sessions'
    $oldLog = Join-Path $logDir 'old.log'
    $oldSession = Join-Path $sessionsDir 'old-session.json'
    $tempFile = Join-Path $tmpDir 'temp.txt'
    $scriptPath = Join-Path $runtimeScriptRoot 'clean-codex-runtime.ps1'
    try {
        New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null
        New-Item -ItemType Directory -Path $logDir -Force | Out-Null
        New-Item -ItemType Directory -Path $sessionsDir -Force | Out-Null
        Set-Content -LiteralPath $tempFile -Value 'temp'
        Set-Content -LiteralPath $oldLog -Value 'log'
        Set-Content -LiteralPath $oldSession -Value 'session'
        $expiredDate = (Get-Date).AddDays(-10)
        (Get-Item -LiteralPath $oldLog).LastWriteTime = $expiredDate
        (Get-Item -LiteralPath $oldSession).LastWriteTime = $expiredDate
        & $scriptPath -CodexHome $codexHome -IncludeSessions -SessionRetentionDays 1 -LogRetentionDays 1 -Apply | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-True ($exitCode -eq 0) 'clean-codex-runtime smoke test failed.'
        Assert-True (-not (Test-Path -LiteralPath $tempFile)) 'clean-codex-runtime did not remove tmp file.'
        Assert-True (-not (Test-Path -LiteralPath $oldLog)) 'clean-codex-runtime did not remove old log.'
        Assert-True (-not (Test-Path -LiteralPath $oldSession)) 'clean-codex-runtime did not remove old session.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    $codexHome = Join-Path $tempRoot '.codex'
    $tmpDir = Join-Path $codexHome 'tmp'
    $logDir = Join-Path $codexHome 'log'
    $sessionsDir = Join-Path $codexHome 'sessions'
    $scriptPath = Join-Path $runtimeScriptRoot 'clean-codex-runtime.ps1'
    try {
        New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null
        New-Item -ItemType Directory -Path $logDir -Force | Out-Null
        New-Item -ItemType Directory -Path $sessionsDir -Force | Out-Null
        & $scriptPath -CodexHome $codexHome -IncludeSessions -SessionRetentionDays 1 -LogRetentionDays 1 -Apply | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-True ($exitCode -eq 0) 'clean-codex-runtime must tolerate empty log/session collections.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    Write-Host '[OK] runtime script tests passed.'
    exit 0
}
catch {
    $message = $_.Exception.Message
    $trace = $_.ScriptStackTrace
    if ([string]::IsNullOrWhiteSpace($trace)) {
        Write-Host ("[FAIL] runtime script tests failed: {0}" -f $message)
    }
    else {
        Write-Host ("[FAIL] runtime script tests failed: {0}`n{1}" -f $message, $trace)
    }
    exit 1
}