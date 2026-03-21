<#
.SYNOPSIS
    Runs the recommended local onboarding flow for repository-managed runtime assets.

.DESCRIPTION
    Orchestrates the standard setup flow for this repository:
    - bootstrap shared `.github` and `.codex` runtime assets
    - optionally apply MCP configuration
    - render versioned global VS Code settings
    - synchronize versioned VS Code snippets
    - configure local Git hooks
    - run repository healthcheck

    The script is intentionally an orchestrator only. It reuses the versioned
    runtime scripts already maintained in this repository instead of duplicating logic.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.PARAMETER TargetGithubPath
    Optional runtime target path for .github assets.

.PARAMETER TargetCodexPath
    Optional runtime target path for .codex assets.

.PARAMETER TargetAgentsSkillsPath
    Optional runtime target path for picker-visible local skills.

.PARAMETER TargetCopilotSkillsPath
    Optional runtime target path for GitHub Copilot native personal skills.

.PARAMETER GlobalVscodeUserPath
    Optional VS Code global user settings folder.

.PARAMETER ValidationProfile
    Validation profile used by the final healthcheck. Defaults to `dev`.

.PARAMETER Mirror
    Enables mirror mode for bootstrap runtime sync.

.PARAMETER ApplyMcpConfig
    Applies MCP configuration during bootstrap.

.PARAMETER BackupMcpConfig
    Creates backup before applying MCP configuration.

.PARAMETER CreateSettingsBackup
    Creates a backup before rendering the global VS Code settings file.

.PARAMETER SkipGlobalSettings
    Skips global VS Code settings synchronization.

.PARAMETER SkipGlobalSnippets
    Skips global VS Code snippet synchronization.

.PARAMETER SkipGitHooks
    Skips local Git hook setup.

.PARAMETER SkipHealthcheck
    Skips the final healthcheck run.

.PARAMETER PreviewOnly
    Prints the planned steps and returns them without executing any changes.

.PARAMETER Verbose
    Shows detailed diagnostics.

.EXAMPLE
    pwsh -File scripts/runtime/install.ps1

.EXAMPLE
    pwsh -File scripts/runtime/install.ps1 -Mirror -ApplyMcpConfig -BackupMcpConfig -CreateSettingsBackup

.EXAMPLE
    pwsh -File C:\Users\tguis\copilot-instructions\scripts\runtime\install.ps1 -CreateSettingsBackup -ApplyMcpConfig -BackupMcpConfig

.EXAMPLE
    pwsh -File C:\Users\tguis\copilot-instructions\scripts\runtime\install.ps1 -RepoRoot C:\Users\tguis\copilot-instructions -PreviewOnly

.EXAMPLE
    pwsh -File scripts/runtime/install.ps1 -PreviewOnly

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot,
    [string] $TargetGithubPath,
    [string] $TargetCodexPath,
    [string] $TargetAgentsSkillsPath,
    [string] $TargetCopilotSkillsPath,
    [string] $GlobalVscodeUserPath,
    [string] $ValidationProfile = 'dev',
    [switch] $Mirror,
    [switch] $ApplyMcpConfig,
    [switch] $BackupMcpConfig,
    [switch] $CreateSettingsBackup,
    [switch] $SkipGlobalSettings,
    [switch] $SkipGlobalSnippets,
    [switch] $SkipGitHooks,
    [switch] $SkipHealthcheck,
    [switch] $PreviewOnly,
    [switch] $Verbose
)

$ErrorActionPreference = 'Stop'

$script:ConsoleStylePath = Join-Path $PSScriptRoot '..\common\console-style.ps1'
if (-not (Test-Path -LiteralPath $script:ConsoleStylePath -PathType Leaf)) {
    $script:ConsoleStylePath = Join-Path $PSScriptRoot '..\..\common\console-style.ps1'
}
if (Test-Path -LiteralPath $script:ConsoleStylePath -PathType Leaf) {
    . $script:ConsoleStylePath
}
$script:RuntimePathsPath = Join-Path $PSScriptRoot '..\common\runtime-paths.ps1'
if (-not (Test-Path -LiteralPath $script:RuntimePathsPath -PathType Leaf)) {
    $script:RuntimePathsPath = Join-Path $PSScriptRoot '..\..\common\runtime-paths.ps1'
}
if (Test-Path -LiteralPath $script:RuntimePathsPath -PathType Leaf) {
    . $script:RuntimePathsPath
}
else {
    throw "Missing shared runtime path helper: $script:RuntimePathsPath"
}
$script:RepositoryHelpersPath = Join-Path $PSScriptRoot '..\common\repository-paths.ps1'
if (-not (Test-Path -LiteralPath $script:RepositoryHelpersPath -PathType Leaf)) {
    $script:RepositoryHelpersPath = Join-Path $PSScriptRoot '..\..\common\repository-paths.ps1'
}
if (Test-Path -LiteralPath $script:RepositoryHelpersPath -PathType Leaf) {
    . $script:RepositoryHelpersPath
}
else {
    throw "Missing shared repository helper: $script:RepositoryHelpersPath"
}
$script:ScriptRoot = Split-Path -Path $PSCommandPath -Parent
$script:IsVerboseEnabled = [bool] $Verbose

# Writes verbose diagnostics.
function Write-VerboseLog {
    param(
        [string] $Message
    )

    if ($script:IsVerboseEnabled) {
        Write-StyledOutput ("[VERBOSE] {0}" -f $Message) | Out-Host
    }
}

# Builds a deterministic install step contract.
function New-InstallStep {
    param(
        [string] $Name,
        [string] $ScriptPath,
        [hashtable] $Arguments
    )

    return [pscustomobject]@{
        name = $Name
        scriptPath = $ScriptPath
        arguments = $Arguments
    }
}

# Executes or previews a planned install step.
function Invoke-InstallStep {
    param(
        [pscustomobject] $Step,
        [bool] $Preview
    )

    $startedAt = Get-Date
    $status = 'preview'
    $errorMessage = $null

    if ($Preview) {
        Write-StyledOutput ("[PLAN] {0}" -f $Step.name) | Out-Host
    }
    else {
        Write-StyledOutput ("[STEP] {0}" -f $Step.name) | Out-Host
        try {
            $stepArguments = @{}
            foreach ($property in $Step.arguments.GetEnumerator()) {
                $stepArguments[$property.Key] = $property.Value
            }

            & $Step.scriptPath @stepArguments | Out-Null
            $status = 'passed'
            Write-StyledOutput ("[OK] {0}" -f $Step.name) | Out-Host
        }
        catch {
            $status = 'failed'
            $errorMessage = $_.Exception.Message
            Write-StyledOutput ("[FAIL] {0}: {1}" -f $Step.name, $errorMessage) | Out-Host
            throw
        }
    }

    return [pscustomobject]@{
        name = $Step.name
        scriptPath = $Step.scriptPath
        status = $status
        startedAt = $startedAt.ToString('o')
        completedAt = (Get-Date).ToString('o')
        error = $errorMessage
    }
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$steps = New-Object System.Collections.Generic.List[object]

$bootstrapArguments = @{
    RepoRoot = $resolvedRepoRoot
}
if (-not [string]::IsNullOrWhiteSpace($TargetGithubPath)) {
    $bootstrapArguments.TargetGithubPath = $TargetGithubPath
}
if (-not [string]::IsNullOrWhiteSpace($TargetCodexPath)) {
    $bootstrapArguments.TargetCodexPath = $TargetCodexPath
}
if (-not [string]::IsNullOrWhiteSpace($TargetAgentsSkillsPath)) {
    $bootstrapArguments.TargetAgentsSkillsPath = $TargetAgentsSkillsPath
}
if (-not [string]::IsNullOrWhiteSpace($TargetCopilotSkillsPath)) {
    $bootstrapArguments.TargetCopilotSkillsPath = $TargetCopilotSkillsPath
}
if ($Mirror) {
    $bootstrapArguments.Mirror = $true
}
if ($ApplyMcpConfig) {
    $bootstrapArguments.ApplyMcpConfig = $true
}
if ($BackupMcpConfig) {
    $bootstrapArguments.BackupConfig = $true
}

$steps.Add((New-InstallStep -Name 'Bootstrap shared runtime assets' -ScriptPath (Resolve-RepoPath -Root $resolvedRepoRoot -Path 'scripts/runtime/bootstrap.ps1') -Arguments $bootstrapArguments)) | Out-Null

if (-not $SkipGlobalSettings) {
    $settingsArguments = @{
        RepoRoot = $resolvedRepoRoot
    }
    if (-not [string]::IsNullOrWhiteSpace($GlobalVscodeUserPath)) {
        $settingsArguments.GlobalVscodeUserPath = $GlobalVscodeUserPath
    }
    if ($CreateSettingsBackup) {
        $settingsArguments.CreateBackup = $true
    }

    $steps.Add((New-InstallStep -Name 'Render global VS Code settings' -ScriptPath (Resolve-RepoPath -Root $resolvedRepoRoot -Path 'scripts/runtime/sync-vscode-global-settings.ps1') -Arguments $settingsArguments)) | Out-Null
}

if (-not $SkipGlobalSnippets) {
    $snippetArguments = @{
        RepoRoot = $resolvedRepoRoot
    }
    if (-not [string]::IsNullOrWhiteSpace($GlobalVscodeUserPath)) {
        $snippetArguments.GlobalVscodeUserPath = $GlobalVscodeUserPath
    }

    $steps.Add((New-InstallStep -Name 'Synchronize global VS Code snippets' -ScriptPath (Resolve-RepoPath -Root $resolvedRepoRoot -Path 'scripts/runtime/sync-vscode-global-snippets.ps1') -Arguments $snippetArguments)) | Out-Null
}

if (-not $SkipGitHooks) {
    $steps.Add((New-InstallStep -Name 'Configure local Git hooks' -ScriptPath (Resolve-RepoPath -Root $resolvedRepoRoot -Path 'scripts/git-hooks/setup-git-hooks.ps1') -Arguments @{})) | Out-Null
}

if (-not $SkipHealthcheck) {
    $healthcheckArguments = @{
        RepoRoot = $resolvedRepoRoot
        ValidationProfile = $ValidationProfile
        WarningOnly = $true
    }
    if (-not [string]::IsNullOrWhiteSpace($TargetGithubPath)) {
        $healthcheckArguments.TargetGithubPath = $TargetGithubPath
    }
    if (-not [string]::IsNullOrWhiteSpace($TargetCodexPath)) {
        $healthcheckArguments.TargetCodexPath = $TargetCodexPath
    }
    if (-not [string]::IsNullOrWhiteSpace($TargetAgentsSkillsPath)) {
        $healthcheckArguments.TargetAgentsSkillsPath = $TargetAgentsSkillsPath
    }
    if (-not [string]::IsNullOrWhiteSpace($TargetCopilotSkillsPath)) {
        $healthcheckArguments.TargetCopilotSkillsPath = $TargetCopilotSkillsPath
    }

    $steps.Add((New-InstallStep -Name 'Run repository healthcheck' -ScriptPath (Resolve-RepoPath -Root $resolvedRepoRoot -Path 'scripts/runtime/healthcheck.ps1') -Arguments $healthcheckArguments)) | Out-Null
}

$results = New-Object System.Collections.Generic.List[object]
foreach ($step in $steps) {
    $results.Add((Invoke-InstallStep -Step $step -Preview ([bool] $PreviewOnly))) | Out-Null
}

$resultItems = @($results.ToArray())
$stepItems = @($steps.ToArray())

Write-StyledOutput '' | Out-Host
Write-StyledOutput 'Runtime install summary' | Out-Host
Write-StyledOutput ("  Preview-only: {0}" -f ([bool] $PreviewOnly)) | Out-Host
Write-StyledOutput ("  Planned steps: {0}" -f $steps.Count) | Out-Host
Write-StyledOutput ("  Executed steps: {0}" -f @($resultItems | Where-Object { $_.status -eq 'passed' }).Count) | Out-Host

[pscustomobject]@{
    previewOnly = [bool] $PreviewOnly
    repoRoot = $resolvedRepoRoot
    steps = $stepItems
    results = $resultItems
}