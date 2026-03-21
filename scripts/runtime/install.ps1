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
    - configure global Git aliases
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

.PARAMETER RuntimeProfile
    Runtime activation profile. Supported values are defined in
    `.github/governance/runtime-install-profiles.json`. Defaults to the
    catalog default, which is currently `none`.

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
    [string] $RuntimeProfile,
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
. $script:CommonBootstrapPath -CallerScriptRoot $PSScriptRoot -Helpers @('console-style', 'repository-paths', 'runtime-paths', 'runtime-install-profiles')
$script:ScriptRoot = Split-Path -Path $PSCommandPath -Parent
$script:IsVerboseEnabled = [bool] $Verbose
$script:LogFilePath = $null

Initialize-ExecutionIssueTracking
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
        Write-ExecutionLog -Level 'INFO' -Message ("Starting install step: {0}" -f $Step.name)
        try {
            $stepArguments = @{}
            foreach ($property in $Step.arguments.GetEnumerator()) {
                $stepArguments[$property.Key] = $property.Value
            }

            & $Step.scriptPath @stepArguments | Out-Null
            $stepExitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
            if ($stepExitCode -eq 0) {
                $status = 'passed'
                Write-StyledOutput ("[OK] {0}" -f $Step.name) | Out-Host
                Write-ExecutionLog -Level 'OK' -Message ("Install step passed: {0}" -f $Step.name)
            }
            else {
                $status = 'failed'
                $errorMessage = "non-zero exit code: $stepExitCode"
                Write-StyledOutput ("[FAIL] {0}: {1}" -f $Step.name, $errorMessage) | Out-Host
                Write-ExecutionLog -Level 'ERROR' -Code 'INSTALL_STEP_FAILED' -Message ("{0}: {1}" -f $Step.name, $errorMessage)
            }
        }
        catch {
            $status = 'failed'
            $errorMessage = $_.Exception.Message
            Write-StyledOutput ("[FAIL] {0}: {1}" -f $Step.name, $errorMessage) | Out-Host
            Write-ExecutionLog -Level 'ERROR' -Code 'INSTALL_STEP_EXCEPTION' -Message ("{0}: {1}" -f $Step.name, $errorMessage)
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
$resolvedRuntimeProfile = Resolve-RuntimeInstallProfile -ResolvedRepoRoot $resolvedRepoRoot -ProfileName $RuntimeProfile
$steps = New-Object System.Collections.Generic.List[object]

if ($ApplyMcpConfig -and -not $resolvedRuntimeProfile.EnableCodexRuntime) {
    throw ("Runtime profile '{0}' does not enable the Codex runtime surface required by -ApplyMcpConfig." -f $resolvedRuntimeProfile.Name)
}

if ($resolvedRuntimeProfile.InstallBootstrap) {
    $bootstrapArguments = @{
        RepoRoot = $resolvedRepoRoot
        RuntimeProfile = $resolvedRuntimeProfile.Name
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
}

if ($resolvedRuntimeProfile.InstallGlobalVscodeSettings -and -not $SkipGlobalSettings) {
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

if ($resolvedRuntimeProfile.InstallGlobalVscodeSnippets -and -not $SkipGlobalSnippets) {
    $snippetArguments = @{
        RepoRoot = $resolvedRepoRoot
    }
    if (-not [string]::IsNullOrWhiteSpace($GlobalVscodeUserPath)) {
        $snippetArguments.GlobalVscodeUserPath = $GlobalVscodeUserPath
    }

    $steps.Add((New-InstallStep -Name 'Synchronize global VS Code snippets' -ScriptPath (Resolve-RepoPath -Root $resolvedRepoRoot -Path 'scripts/runtime/sync-vscode-global-snippets.ps1') -Arguments $snippetArguments)) | Out-Null
}

if ($resolvedRuntimeProfile.InstallLocalGitHooks -and -not $SkipGitHooks) {
    $steps.Add((New-InstallStep -Name 'Configure local Git hooks' -ScriptPath (Resolve-RepoPath -Root $resolvedRepoRoot -Path 'scripts/git-hooks/setup-git-hooks.ps1') -Arguments @{})) | Out-Null
}

if ($resolvedRuntimeProfile.InstallGlobalGitAliases -and -not $SkipGitHooks) {
    $globalAliasArguments = @{
        RepoRoot = $resolvedRepoRoot
        TargetCodexPath = if (-not [string]::IsNullOrWhiteSpace($TargetCodexPath)) { $TargetCodexPath } else { Join-Path (Resolve-UserHomePath) '.codex' }
    }
    $steps.Add((New-InstallStep -Name 'Configure global Git aliases' -ScriptPath (Resolve-RepoPath -Root $resolvedRepoRoot -Path 'scripts/git-hooks/setup-global-git-aliases.ps1') -Arguments $globalAliasArguments)) | Out-Null
}

if ($resolvedRuntimeProfile.InstallHealthcheck -and -not $SkipHealthcheck) {
    $healthcheckArguments = @{
        RepoRoot = $resolvedRepoRoot
        RuntimeProfile = $resolvedRuntimeProfile.Name
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
    $stepResult = Invoke-InstallStep -Step $step -Preview ([bool] $PreviewOnly)
    $results.Add($stepResult) | Out-Null

    if ((-not $PreviewOnly) -and $stepResult.status -eq 'failed') {
        break
    }
}

$resultItems = @($results.ToArray())
$stepItems = @($steps.ToArray())
$passedSteps = @($resultItems | Where-Object { $_.status -eq 'passed' }).Count
$failedSteps = @($resultItems | Where-Object { $_.status -eq 'failed' }).Count
$issueSummary = Write-ExecutionIssueSummary -Title 'Runtime install issue summary'
$overallStatus = if ($failedSteps -gt 0) { 'failed' } elseif ($PreviewOnly) { 'preview' } else { 'passed' }

Write-StyledOutput '' | Out-Host
Write-StyledOutput 'Runtime install summary' | Out-Host
Write-StyledOutput ("  Runtime profile: {0}" -f $resolvedRuntimeProfile.Name) | Out-Host
Write-StyledOutput ("  Profile catalog: {0}" -f $resolvedRuntimeProfile.CatalogPath) | Out-Host
Write-StyledOutput ("  Preview-only: {0}" -f ([bool] $PreviewOnly)) | Out-Host
Write-StyledOutput ("  Planned steps: {0}" -f $steps.Count) | Out-Host
Write-StyledOutput ("  Executed steps: {0}" -f $resultItems.Count) | Out-Host
Write-StyledOutput ("  Passed steps: {0}" -f $passedSteps) | Out-Host
Write-StyledOutput ("  Failed steps: {0}" -f $failedSteps) | Out-Host
Write-StyledOutput ("  Overall status: {0}" -f $overallStatus) | Out-Host

$output = [pscustomobject]@{
    previewOnly = [bool] $PreviewOnly
    repoRoot = $resolvedRepoRoot
    runtimeProfile = [pscustomobject]@{
        name = $resolvedRuntimeProfile.Name
        description = $resolvedRuntimeProfile.Description
        defaultProfile = $resolvedRuntimeProfile.DefaultProfile
        catalogPath = $resolvedRuntimeProfile.CatalogPath
    }
    steps = $stepItems
    results = $resultItems
    summary = [pscustomobject]@{
        overallStatus = $overallStatus
        runtimeProfile = $resolvedRuntimeProfile.Name
        plannedSteps = $steps.Count
        executedSteps = $resultItems.Count
        passedSteps = $passedSteps
        failedSteps = $failedSteps
    }
    issues = $issueSummary
}

$output

if ($failedSteps -gt 0 -and -not $PreviewOnly) {
    exit 1
}