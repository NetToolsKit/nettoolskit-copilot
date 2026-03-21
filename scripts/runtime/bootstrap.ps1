<#
.SYNOPSIS
    Syncs repository-managed .github and .codex assets into the local runtime folders.

.DESCRIPTION
    Detects the repository root and copies shared assets to:
    - <user-home>/.github
    - <user-home>/.github/scripts
    - <user-home>/.agents/skills
    - <user-home>/.codex/shared-mcp
    - <user-home>/.codex/shared-scripts
    - <user-home>/.codex/shared-orchestration

    Runtime .github/scripts are synchronized from:
    - scripts (repository root scripts)

    Shared-scripts are synchronized from:
    - .codex/scripts (MCP utility scripts and docs)
    - scripts/common (shared PowerShell helpers)
    - scripts/security (shared security audit gates)

    When -ApplyMcpConfig is specified, applies MCP servers from the shared manifest
    into the local Codex config.toml file.

.PARAMETER RepoRoot
    Optional repository root. If omitted, the script detects root from the script location.

.PARAMETER TargetGithubPath
    Target path for .github runtime assets. Defaults to <user-home>/.github.

.PARAMETER TargetCodexPath
    Target path for .codex runtime assets other than picker-visible skills. Defaults to <user-home>/.codex.

.PARAMETER TargetAgentsSkillsPath
    Target path for picker-visible local skills. Defaults to <user-home>/.agents/skills.

.PARAMETER Mirror
    Mirrors target folders (removes files not present in source) when supported by the sync mode.

.PARAMETER ApplyMcpConfig
    Applies mcp_servers blocks from .codex/mcp/servers.manifest.json into target config.toml.

.PARAMETER BackupConfig
    Creates backup before applying MCP config (used with -ApplyMcpConfig).

.PARAMETER Verbose
    Shows detailed sync diagnostics.

.EXAMPLE
    pwsh -File ./scripts/runtime/bootstrap.ps1

.EXAMPLE
    pwsh -File ./scripts/runtime/bootstrap.ps1 -Mirror

.EXAMPLE
    pwsh -File ./scripts/runtime/bootstrap.ps1 -ApplyMcpConfig -BackupConfig

.NOTES
    Version: 1.4
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
$script:RobocopyCommand = Get-Command -Name 'robocopy' -ErrorAction SilentlyContinue
$script:RobocopyBaseArgs = @(
    '/R:2',
    '/W:1',
    '/NFL',
    '/NDL',
    '/NJH',
    '/NJS',
    '/MT:8'
)

# -------------------------------
# Helpers
# -------------------------------
# Validates that a required file path exists before execution continues.
function Assert-PathPresent {
    param(
        [string] $Path,
        [string] $Label
    )

    if (-not (Test-Path -LiteralPath $Path)) {
        throw ("Missing {0}: {1}" -f $Label, $Path)
    }
}

# Synchronizes directories with Copy-Item when robocopy is unavailable.
function Invoke-FallbackSync {
    param(
        [string] $Source,
        [string] $Destination,
        [switch] $MirrorMode
    )

    New-Item -ItemType Directory -Path $Destination -Force | Out-Null

    if ($MirrorMode -and (Test-Path -LiteralPath $Destination)) {
        Get-ChildItem -LiteralPath $Destination -Force -ErrorAction SilentlyContinue | ForEach-Object {
            Remove-Item -LiteralPath $_.FullName -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    $sourceItems = Get-ChildItem -LiteralPath $Source -Force -ErrorAction SilentlyContinue
    foreach ($item in $sourceItems) {
        Copy-Item -LiteralPath $item.FullName -Destination $Destination -Recurse -Force
    }
}

# Invokes robocopy with the shared repository sync defaults.
function Invoke-RobocopySync {
    param(
        [string] $Source,
        [string] $Destination,
        [switch] $MirrorMode,
        [string[]] $AdditionalArgs
    )

    New-Item -ItemType Directory -Path $Destination -Force | Out-Null

    $mode = if ($MirrorMode) { '/MIR' } else { '/E' }
    $robocopyArgs = @(
        $Source,
        $Destination,
        $mode
    ) + $script:RobocopyBaseArgs

    if (@($AdditionalArgs).Count -gt 0) {
        $robocopyArgs += $AdditionalArgs
    }

    & $script:RobocopyCommand.Source @robocopyArgs | Out-Null
    if ($LASTEXITCODE -ge 8) {
        throw "robocopy failed for '$Source' -> '$Destination' (exit code: $LASTEXITCODE)"
    }

    Write-VerboseColor ("Synced with robocopy: {0} -> {1}" -f $Source, $Destination) 'Gray'
}

# Synchronizes source and destination directories with robocopy or fallback mode.
function Invoke-DirectorySync {
    param(
        [string] $Source,
        [string] $Destination,
        [switch] $MirrorMode
    )

    if (-not (Test-Path -LiteralPath $Source)) {
        Write-VerboseColor ("Skipping missing source: {0}" -f $Source) 'Yellow'
        return
    }

    if ($null -ne $script:RobocopyCommand) {
        Invoke-RobocopySync -Source $Source -Destination $Destination -MirrorMode:$MirrorMode
        return
    }

    Write-VerboseColor 'robocopy not found; using Copy-Item fallback sync.' 'Yellow'
    Invoke-FallbackSync -Source $Source -Destination $Destination -MirrorMode:$MirrorMode
    Write-VerboseColor ("Synced with fallback copy: {0} -> {1}" -f $Source, $Destination) 'Gray'
}

# Returns repository-managed skill directory names from the versioned source root.
function Get-ManagedSkillNameList {
    param(
        [string] $SourceRoot
    )

    if (-not (Test-Path -LiteralPath $SourceRoot -PathType Container)) {
        return @()
    }

    return @(Get-ChildItem -LiteralPath $SourceRoot -Directory -Force | Select-Object -ExpandProperty Name | Sort-Object -Unique)
}

# Synchronizes repository-owned skill directories into the local `.agents/skills` picker path.
function Invoke-AgentsSkillSync {
    param(
        [string] $SourceRoot,
        [string] $DestinationRoot,
        [switch] $MirrorMode
    )

    if (-not (Test-Path -LiteralPath $SourceRoot -PathType Container)) {
        Write-VerboseColor ("Skipping missing agents skill source root: {0}" -f $SourceRoot) 'Yellow'
        return
    }

    New-Item -ItemType Directory -Path $DestinationRoot -Force | Out-Null

    if ($null -ne $script:RobocopyCommand) {
        Invoke-RobocopySync -Source $SourceRoot -Destination $DestinationRoot -MirrorMode:$MirrorMode -AdditionalArgs @('/XF', 'README.md')

        $destinationReadme = Join-Path $DestinationRoot 'README.md'
        if (Test-Path -LiteralPath $destinationReadme -PathType Leaf) {
            Remove-Item -LiteralPath $destinationReadme -Force -ErrorAction Stop
        }

        return
    }

    $sourceSkillDirectories = @(Get-ChildItem -LiteralPath $SourceRoot -Directory -Force)
    foreach ($skillDirectory in $sourceSkillDirectories) {
        $targetSkillPath = Join-Path $DestinationRoot $skillDirectory.Name
        Invoke-DirectorySync -Source $skillDirectory.FullName -Destination $targetSkillPath -MirrorMode:$MirrorMode
    }
}

# Removes repository-managed skill duplicates from the local `.codex/skills` runtime root.
function Remove-ManagedCodexSkillDuplicates {
    param(
        [string] $ManagedSourceRoot,
        [string] $CodexSkillsRoot
    )

    if (-not (Test-Path -LiteralPath $CodexSkillsRoot -PathType Container)) {
        return
    }

    foreach ($skillName in (Get-ManagedSkillNameList -SourceRoot $ManagedSourceRoot)) {
        $duplicateSkillPath = Join-Path $CodexSkillsRoot $skillName
        if (Test-Path -LiteralPath $duplicateSkillPath) {
            Remove-Item -LiteralPath $duplicateSkillPath -Recurse -Force -ErrorAction Stop
        }
    }

    $managedReadme = Join-Path $ManagedSourceRoot 'README.md'
    $duplicateReadme = Join-Path $CodexSkillsRoot 'README.md'
    if ((Test-Path -LiteralPath $managedReadme -PathType Leaf) -and (Test-Path -LiteralPath $duplicateReadme -PathType Leaf)) {
        $managedHash = (Get-FileHash -LiteralPath $managedReadme -Algorithm SHA256).Hash
        $duplicateHash = (Get-FileHash -LiteralPath $duplicateReadme -Algorithm SHA256).Hash
        if ($managedHash -eq $duplicateHash) {
            Remove-Item -LiteralPath $duplicateReadme -Force -ErrorAction Stop
        }
    }
}

# Applies MCP manifest settings to target Codex config.toml.
function Invoke-McpConfigApply {
    param(
        [string] $ResolvedRepoRoot,
        [string] $CodexPath,
        [switch] $CreateBackup
    )

    $syncScript = Join-Path $ResolvedRepoRoot '.codex\scripts\sync-mcp-to-codex-config.ps1'
    $manifest = Join-Path $ResolvedRepoRoot '.codex\mcp\servers.manifest.json'
    $targetConfig = Join-Path $CodexPath 'config.toml'

    Assert-PathPresent -Path $syncScript -Label 'MCP sync script'
    Assert-PathPresent -Path $manifest -Label 'MCP manifest'
    Assert-PathPresent -Path $targetConfig -Label 'target Codex config'

    $syncArgs = @{
        ManifestPath = $manifest
        TargetConfigPath = $targetConfig
    }

    if ($CreateBackup) {
        $syncArgs.CreateBackup = $true
    }

    & $syncScript @syncArgs
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

$sourceGithub = Join-Path $resolvedRepoRoot '.github'
$sourceCodex = Join-Path $resolvedRepoRoot '.codex'
$sourceScripts = Join-Path $resolvedRepoRoot 'scripts'
$sourceCodexScripts = Join-Path $sourceCodex 'scripts'
$sourceCommonScripts = Join-Path $sourceScripts 'common'
$sourceSecurityScripts = Join-Path $sourceScripts 'security'

Assert-PathPresent -Path $sourceGithub -Label 'source .github folder'
Assert-PathPresent -Path $sourceCodex -Label 'source .codex folder'
Assert-PathPresent -Path $sourceScripts -Label 'source scripts folder'

Invoke-DirectorySync -Source $sourceGithub -Destination $TargetGithubPath -MirrorMode:$Mirror
Invoke-DirectorySync -Source $sourceScripts -Destination (Join-Path $TargetGithubPath 'scripts') -MirrorMode:$Mirror
Invoke-AgentsSkillSync -SourceRoot (Join-Path $sourceCodex 'skills') -DestinationRoot $TargetAgentsSkillsPath -MirrorMode:$Mirror
Remove-ManagedCodexSkillDuplicates -ManagedSourceRoot (Join-Path $sourceCodex 'skills') -CodexSkillsRoot (Join-Path $TargetCodexPath 'skills')
Invoke-DirectorySync -Source (Join-Path $sourceCodex 'mcp') -Destination (Join-Path $TargetCodexPath 'shared-mcp') -MirrorMode:$Mirror
Invoke-DirectorySync -Source $sourceCodexScripts -Destination (Join-Path $TargetCodexPath 'shared-scripts') -MirrorMode:$Mirror
Invoke-DirectorySync -Source $sourceCommonScripts -Destination (Join-Path $TargetCodexPath 'shared-scripts\common') -MirrorMode:$Mirror
Invoke-DirectorySync -Source $sourceSecurityScripts -Destination (Join-Path $TargetCodexPath 'shared-scripts\security') -MirrorMode:$Mirror
Invoke-DirectorySync -Source (Join-Path $sourceCodex 'orchestration') -Destination (Join-Path $TargetCodexPath 'shared-orchestration') -MirrorMode:$Mirror

$sharedReadme = Join-Path $sourceCodex 'README.md'
if (Test-Path -LiteralPath $sharedReadme) {
    New-Item -ItemType Directory -Path $TargetCodexPath -Force | Out-Null
    Copy-Item -LiteralPath $sharedReadme -Destination (Join-Path $TargetCodexPath 'README.shared.md') -Force
}

Write-StyledOutput 'Sync complete.'
Write-StyledOutput ("  .github -> {0}" -f $TargetGithubPath)
Write-StyledOutput ("  scripts -> {0}" -f (Join-Path $TargetGithubPath 'scripts'))
Write-StyledOutput ("  .codex/skills (source only; runtime duplicates removed from {0})" -f (Join-Path $TargetCodexPath 'skills'))
Write-StyledOutput ("  .agents/skills (canonical picker/runtime skills) -> {0}" -f $TargetAgentsSkillsPath)
Write-StyledOutput ("  .codex/mcp -> {0}" -f (Join-Path $TargetCodexPath 'shared-mcp'))
Write-StyledOutput ("  .codex/scripts + scripts/common + scripts/security -> {0}" -f (Join-Path $TargetCodexPath 'shared-scripts'))
Write-StyledOutput ("  .codex/orchestration -> {0}" -f (Join-Path $TargetCodexPath 'shared-orchestration'))

if ($ApplyMcpConfig) {
    Invoke-McpConfigApply -ResolvedRepoRoot $resolvedRepoRoot -CodexPath $TargetCodexPath -CreateBackup:$BackupConfig
}

exit 0