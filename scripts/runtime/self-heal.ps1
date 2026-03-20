<#
.SYNOPSIS
    Performs controlled self-healing for runtime and workspace agent assets.

.DESCRIPTION
    Executes a repair flow and validates final health:
    1) runtime bootstrap sync
    2) apply VS Code active files from templates
    3) run healthcheck and export status

    Produces:
    - console summary
    - structured JSON report
    - text log file

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.PARAMETER TargetGithubPath
    Runtime target path for .github assets. Defaults to <user-home>/.github.

.PARAMETER TargetCodexPath
    Runtime target path for .codex assets. Defaults to <user-home>/.codex.

.PARAMETER TargetAgentsSkillsPath
    Runtime target path for picker-visible local skills. Defaults to <user-home>/.agents/skills.

.PARAMETER Mirror
    Uses mirror mode for bootstrap sync.

.PARAMETER ApplyMcpConfig
    Applies MCP server settings into target Codex config.toml during bootstrap.

.PARAMETER BackupConfig
    Creates MCP config backup when -ApplyMcpConfig is used.

.PARAMETER ApplyVscodeTemplates
    Applies `.vscode` active files from templates.

.PARAMETER StrictExtras
    Fails healthcheck when runtime doctor detects extra files.

.PARAMETER OutputPath
    Path for JSON self-heal report. Defaults to .temp/self-heal-report.json.

.PARAMETER LogPath
    Path for text execution log. Defaults to .temp/logs/self-heal-<timestamp>.log.

.PARAMETER Verbose
    Shows detailed diagnostics.

.EXAMPLE
    pwsh -File scripts/runtime/self-heal.ps1

.EXAMPLE
    pwsh -File scripts/runtime/self-heal.ps1 -Mirror -StrictExtras

.EXAMPLE
    pwsh -File scripts/runtime/self-heal.ps1 -ApplyMcpConfig -BackupConfig

.EXAMPLE
    pwsh -File scripts/runtime/self-heal.ps1 -ApplyVscodeTemplates

.NOTES
    Version: 1.1
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot,
    [string] $TargetGithubPath,
    [string] $TargetCodexPath,
    [string] $TargetAgentsSkillsPath,
    [switch] $Mirror,
    [switch] $ApplyMcpConfig,
    [switch] $BackupConfig,
    [switch] $ApplyVscodeTemplates,
    [switch] $StrictExtras,
    [string] $OutputPath = '.temp/self-heal-report.json',
    [string] $LogPath,
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
$script:LogFilePath = $null
$script:IsVerboseEnabled = [bool] $Verbose

# -------------------------------
# Helpers
# -------------------------------
# Runs a script step and captures status, timing, and error metadata.
function Invoke-ScriptStep {
    param(
        [string] $Name,
        [string] $ScriptPath,
        [hashtable] $Arguments
    )

    $startedAt = Get-Date
    $status = 'failed'
    $exitCode = 1
    $errorMessage = $null

    if (-not (Test-Path -LiteralPath $ScriptPath -PathType Leaf)) {
        $errorMessage = "Script not found: $ScriptPath"
        Write-ExecutionLog -Level 'ERROR' -Message ("{0}: {1}" -f $Name, $errorMessage)
    }
    else {
        Write-ExecutionLog -Level 'INFO' -Message ("Starting step: {0}" -f $Name)

        try {
            & $ScriptPath @Arguments
            $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
            if ($exitCode -eq 0) {
                $status = 'passed'
                Write-ExecutionLog -Level 'OK' -Message ("Step passed: {0}" -f $Name)
            }
            else {
                Write-ExecutionLog -Level 'ERROR' -Message ("Step failed: {0} (exit code {1})" -f $Name, $exitCode)
            }
        }
        catch {
            $exitCode = 1
            $errorMessage = $_.Exception.Message
            Write-ExecutionLog -Level 'ERROR' -Message ("Step exception: {0} :: {1}" -f $Name, $errorMessage)
        }
    }

    $finishedAt = Get-Date
    $durationMs = [int] ($finishedAt - $startedAt).TotalMilliseconds
    $relativeScriptPath = [System.IO.Path]::GetRelativePath((Get-Location).Path, $ScriptPath)
    $argumentList = @()
    foreach ($entry in ($Arguments.GetEnumerator() | Sort-Object Name)) {
        $argumentList += ("-{0}={1}" -f $entry.Key, $entry.Value)
    }

    return [pscustomobject]@{
        name = $Name
        script = $relativeScriptPath
        arguments = $argumentList
        status = $status
        exitCode = $exitCode
        durationMs = $durationMs
        startedAt = $startedAt.ToString('o')
        finishedAt = $finishedAt.ToString('o')
        error = $errorMessage
    }
}

# -------------------------------
# Main execution
# -------------------------------
$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
Set-Location -Path $resolvedRepoRoot

$userHome = Resolve-UserHomePath
if ([string]::IsNullOrWhiteSpace($TargetGithubPath)) {
    $TargetGithubPath = Join-Path $userHome '.github'
}
if ([string]::IsNullOrWhiteSpace($TargetCodexPath)) {
    $TargetCodexPath = Join-Path $userHome '.codex'
}
if ([string]::IsNullOrWhiteSpace($TargetAgentsSkillsPath)) {
    $TargetAgentsSkillsPath = Resolve-AgentsSkillsPath
}

$resolvedOutputPath = Resolve-RepoPath -Root $resolvedRepoRoot -Path $OutputPath

$resolvedLogPath = if ([string]::IsNullOrWhiteSpace($LogPath)) {
    $timestampToken = Get-Date -Format 'yyyyMMdd-HHmmss'
    Resolve-RepoPath -Root $resolvedRepoRoot -Path (".temp/logs/self-heal-{0}.log" -f $timestampToken)
}
else {
    Resolve-RepoPath -Root $resolvedRepoRoot -Path $LogPath
}

$outputParent = Get-ParentDirectoryPath -Path $resolvedOutputPath
if (-not [string]::IsNullOrWhiteSpace($outputParent)) {
    New-Item -ItemType Directory -Path $outputParent -Force | Out-Null
}

$logParent = Get-ParentDirectoryPath -Path $resolvedLogPath
if (-not [string]::IsNullOrWhiteSpace($logParent)) {
    New-Item -ItemType Directory -Path $logParent -Force | Out-Null
}
Set-Content -LiteralPath $resolvedLogPath -Value ("# self-heal log`n# generatedAt={0}" -f (Get-Date).ToString('o'))
$script:LogFilePath = $resolvedLogPath

Write-ExecutionLog -Level 'INFO' -Message ("Repo root: {0}" -f $resolvedRepoRoot)
Write-ExecutionLog -Level 'INFO' -Message ("Output report: {0}" -f $resolvedOutputPath)
Write-ExecutionLog -Level 'INFO' -Message ("Log file: {0}" -f $resolvedLogPath)

$steps = New-Object System.Collections.Generic.List[object]

$bootstrapScript = Join-Path $resolvedRepoRoot 'scripts/runtime/bootstrap.ps1'
$applyVscodeTemplatesScript = Join-Path $resolvedRepoRoot 'scripts/runtime/apply-vscode-templates.ps1'
$healthcheckScript = Join-Path $resolvedRepoRoot 'scripts/runtime/healthcheck.ps1'

$bootstrapArgs = @{
    RepoRoot = $resolvedRepoRoot
    TargetGithubPath = $TargetGithubPath
    TargetCodexPath = $TargetCodexPath
    TargetAgentsSkillsPath = $TargetAgentsSkillsPath
}
if ($Mirror) {
    $bootstrapArgs.Mirror = $true
}
if ($ApplyMcpConfig) {
    $bootstrapArgs.ApplyMcpConfig = $true
}
if ($BackupConfig) {
    $bootstrapArgs.BackupConfig = $true
}

$bootstrapStep = @(Invoke-ScriptStep -Name 'runtime-bootstrap' -ScriptPath $bootstrapScript -Arguments $bootstrapArgs) | Select-Object -Last 1
if ($null -ne $bootstrapStep) {
    $steps.Add($bootstrapStep) | Out-Null
}

if ($ApplyVscodeTemplates) {
    $vscodeStep = @(Invoke-ScriptStep -Name 'apply-vscode-templates' -ScriptPath $applyVscodeTemplatesScript -Arguments @{ RepoRoot = $resolvedRepoRoot; Force = $true }) | Select-Object -Last 1
    if ($null -ne $vscodeStep) {
        $steps.Add($vscodeStep) | Out-Null
    }
}
else {
    Write-ExecutionLog -Level 'WARN' -Message 'Skipping VS Code templates apply (enable with -ApplyVscodeTemplates).'
}

$healthcheckReportPath = Resolve-RepoPath -Root $resolvedRepoRoot -Path '.temp/healthcheck-report.json'
$healthcheckLogPath = Resolve-RepoPath -Root $resolvedRepoRoot -Path '.temp/logs/healthcheck-from-self-heal.log'
$healthcheckArgs = @{
    RepoRoot = $resolvedRepoRoot
    TargetGithubPath = $TargetGithubPath
    TargetCodexPath = $TargetCodexPath
    TargetAgentsSkillsPath = $TargetAgentsSkillsPath
    OutputPath = $healthcheckReportPath
    LogPath = $healthcheckLogPath
}
if ($StrictExtras) {
    $healthcheckArgs.StrictExtras = $true
}

$healthcheckStep = @(Invoke-ScriptStep -Name 'healthcheck' -ScriptPath $healthcheckScript -Arguments $healthcheckArgs) | Select-Object -Last 1
if ($null -ne $healthcheckStep) {
    $steps.Add($healthcheckStep) | Out-Null
}

$passedSteps = @($steps | Where-Object { $_.status -eq 'passed' }).Count
$failedSteps = @($steps | Where-Object { $_.status -ne 'passed' }).Count
$overallStatus = if ($failedSteps -eq 0) { 'passed' } else { 'failed' }

$healthcheckSummary = $null
if (Test-Path -LiteralPath $healthcheckReportPath -PathType Leaf) {
    try {
        $healthcheckSummary = Get-Content -Raw -LiteralPath $healthcheckReportPath | ConvertFrom-Json -Depth 100
    }
    catch {
        Write-ExecutionLog -Level 'WARN' -Message ("Could not parse healthcheck report: {0}" -f $healthcheckReportPath)
    }
}

$report = [ordered]@{
    schemaVersion = 1
    generatedAt = (Get-Date).ToString('o')
    repoRoot = $resolvedRepoRoot
    targets = [ordered]@{
        github = $TargetGithubPath
        codex = $TargetCodexPath
        agentsSkills = $TargetAgentsSkillsPath
    }
    options = [ordered]@{
        mirror = [bool] $Mirror
        applyMcpConfig = [bool] $ApplyMcpConfig
        backupConfig = [bool] $BackupConfig
        applyVscodeTemplates = [bool] $ApplyVscodeTemplates
        strictExtras = [bool] $StrictExtras
    }
    summary = [ordered]@{
        totalSteps = $steps.Count
        passedSteps = $passedSteps
        failedSteps = $failedSteps
        overallStatus = $overallStatus
    }
    steps = $steps.ToArray()
    healthcheck = $healthcheckSummary
    logPath = $resolvedLogPath
}

$reportJson = $report | ConvertTo-Json -Depth 100
Set-Content -LiteralPath $resolvedOutputPath -Value $reportJson

Write-ExecutionLog -Level 'INFO' -Message ("Self-heal summary: total={0} passed={1} failed={2}" -f $steps.Count, $passedSteps, $failedSteps)
Write-ExecutionLog -Level 'INFO' -Message ("Self-heal report generated: {0}" -f $resolvedOutputPath)

if ($overallStatus -ne 'passed') {
    exit 1
}

exit 0