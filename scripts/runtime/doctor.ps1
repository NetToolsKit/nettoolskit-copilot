<#
.SYNOPSIS
    Diagnoses drift between repository source-of-truth assets and local runtime folders.

.DESCRIPTION
    Compares source and runtime file sets for:
    - .github -> ~/.github
    - scripts -> ~/.github/scripts
    - .codex/skills -> ~/.agents/skills
    - .github/skills -> ~/.copilot/skills
    - duplicate repo-managed skill folders that should not remain in ~/.codex/skills
    - .codex/mcp -> ~/.codex/shared-mcp
    - .codex/scripts (root tools) -> ~/.codex/shared-scripts
    - scripts/common -> ~/.codex/shared-scripts/common
    - scripts/security -> ~/.codex/shared-scripts/security
    - .codex/orchestration -> ~/.codex/shared-orchestration

    For each mapping, reports:
    - missing files in runtime
    - extra files in runtime
    - hash drift for files present on both sides

    Exit codes:
    - 0: no drift detected (or drift fixed with -SyncOnDrift)
    - 1: drift remains

.PARAMETER RepoRoot
    Optional repository root. If omitted, detects from script location and current directory.

.PARAMETER TargetGithubPath
    Runtime target path for .github assets. Defaults to <user-home>/.github.

.PARAMETER TargetCodexPath
    Runtime target path for .codex assets. Defaults to <user-home>/.codex.

.PARAMETER TargetAgentsSkillsPath
    Runtime target path for picker-visible local skills. Defaults to <user-home>/.agents/skills.

.PARAMETER TargetCopilotSkillsPath
    Runtime target path for GitHub Copilot native personal skills. Defaults to <user-home>/.copilot/skills.

.PARAMETER Detailed
    Prints file-level entries for missing, extra, and drifted files.

.PARAMETER SyncOnDrift
    When provided, runs scripts/runtime/bootstrap.ps1 if drift is detected and re-checks.

.PARAMETER StrictExtras
    Treats extra runtime files as drift failures. By default, extra files are reported but do not fail health.

.EXAMPLE
    pwsh -File scripts/runtime/doctor.ps1

.EXAMPLE
    pwsh -File scripts/runtime/doctor.ps1 -Detailed

.EXAMPLE
    pwsh -File scripts/runtime/doctor.ps1 -SyncOnDrift

.NOTES
    Version: 1.4
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot,
    [string] $TargetGithubPath,
    [string] $TargetCodexPath,
    [string] $TargetAgentsSkillsPath,
    [string] $TargetCopilotSkillsPath,
    [switch] $Detailed,
    [switch] $SyncOnDrift,
    [switch] $StrictExtras
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
# Builds a file hash inventory for drift comparison operations.
function Get-FileInventory {
    param(
        [string] $RootPath
    )

    $inventory = @{}
    if (-not (Test-Path -LiteralPath $RootPath)) {
        return $inventory
    }

    Get-ChildItem -LiteralPath $RootPath -Recurse -File | ForEach-Object {
        $relativePath = [System.IO.Path]::GetRelativePath($RootPath, $_.FullName)
        $hash = (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash
        $inventory[$relativePath] = $hash
    }

    return $inventory
}

# Returns repository-managed skill directory names from the versioned skill source root.
function Get-ManagedSkillNameList {
    param(
        [string] $SkillRoot
    )

    if (-not (Test-Path -LiteralPath $SkillRoot -PathType Container)) {
        return @()
    }

    return @(Get-ChildItem -LiteralPath $SkillRoot -Directory -Force | Select-Object -ExpandProperty Name | Sort-Object -Unique)
}

# Checks whether a relative path starts with or equals one of the provided prefixes.
function Test-PathPrefixMatch {
    param(
        [string] $RelativePath,
        [string[]] $Prefixes
    )

    $prefixList = @($Prefixes | Where-Object { -not [string]::IsNullOrWhiteSpace([string] $_) })
    if ($prefixList.Count -eq 0) {
        return $true
    }

    $normalizedPath = $RelativePath.Replace('\', '/')
    foreach ($prefix in $prefixList) {
        $normalizedPrefix = $prefix.Replace('\', '/').TrimEnd('/')
        if ($normalizedPath -eq $normalizedPrefix -or $normalizedPath.StartsWith(("{0}/" -f $normalizedPrefix), [System.StringComparison]::OrdinalIgnoreCase)) {
            return $true
        }
    }

    return $false
}

# Filters a file inventory by optional include and ignore prefix sets.
function Select-InventoryEntries {
    param(
        [hashtable] $Inventory,
        [string[]] $IncludePrefixes = @(),
        [string[]] $IgnorePrefixes = @()
    )

    $result = @{}
    $includeList = @($IncludePrefixes | Where-Object { -not [string]::IsNullOrWhiteSpace([string] $_) })
    $ignoreList = @($IgnorePrefixes | Where-Object { -not [string]::IsNullOrWhiteSpace([string] $_) })

    foreach ($entry in $Inventory.GetEnumerator()) {
        if ($includeList.Count -gt 0 -and -not (Test-PathPrefixMatch -RelativePath $entry.Key -Prefixes $includeList)) {
            continue
        }

        if ($ignoreList.Count -gt 0 -and (Test-PathPrefixMatch -RelativePath $entry.Key -Prefixes $ignoreList)) {
            continue
        }

        $result[$entry.Key] = $entry.Value
    }

    return $result
}

# Compares source and runtime inventories to detect missing, extra, and drifted files.
function Compare-Mapping {
    param(
        [string] $Name,
        [string] $SourcePath,
        [string] $TargetPath,
        [string[]] $IncludePrefixes = @(),
        [string[]] $IgnoreSourcePrefixes = @(),
        [string[]] $IgnoreExtraPrefixes = @(),
        [switch] $IncludeExtraRuntimeDrift
    )

    $sourceInventory = Get-FileInventory -RootPath $SourcePath
    $targetInventory = Get-FileInventory -RootPath $TargetPath

    $sourceInventory = Select-InventoryEntries -Inventory $sourceInventory -IncludePrefixes $IncludePrefixes -IgnorePrefixes $IgnoreSourcePrefixes
    $targetInventory = Select-InventoryEntries -Inventory $targetInventory -IncludePrefixes $IncludePrefixes

    $sourceKeys = @($sourceInventory.Keys | Sort-Object)
    $targetKeys = @($targetInventory.Keys | Sort-Object)

    $sourceKeySet = New-Object System.Collections.Generic.HashSet[string]([System.StringComparer]::OrdinalIgnoreCase)
    $targetKeySet = New-Object System.Collections.Generic.HashSet[string]([System.StringComparer]::OrdinalIgnoreCase)
    foreach ($key in $sourceKeys) { $sourceKeySet.Add($key) | Out-Null }
    foreach ($key in $targetKeys) { $targetKeySet.Add($key) | Out-Null }

    $missingInRuntime = New-Object System.Collections.Generic.List[string]
    foreach ($key in $sourceKeys) {
        if (-not $targetKeySet.Contains($key)) {
            $missingInRuntime.Add($key) | Out-Null
        }
    }

    $extraInRuntime = New-Object System.Collections.Generic.List[string]
    foreach ($key in $targetKeys) {
        if (-not $sourceKeySet.Contains($key)) {
            $shouldIgnore = $false
            foreach ($prefix in $IgnoreExtraPrefixes) {
                if ($key.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)) {
                    $shouldIgnore = $true
                    break
                }
            }

            if ($shouldIgnore) {
                continue
            }

            $extraInRuntime.Add($key) | Out-Null
        }
    }

    $driftedFiles = New-Object System.Collections.Generic.List[string]
    foreach ($key in $sourceKeys) {
        if (-not $targetKeySet.Contains($key)) {
            continue
        }

        if ($sourceInventory[$key] -ne $targetInventory[$key]) {
            $driftedFiles.Add($key) | Out-Null
        }
    }

    return [pscustomobject]@{
        Name = $Name
        SourcePath = $SourcePath
        TargetPath = $TargetPath
        SourceCount = $sourceKeys.Count
        TargetCount = $targetKeys.Count
        MissingInRuntime = @($missingInRuntime)
        ExtraInRuntime = @($extraInRuntime)
        DriftedFiles = @($driftedFiles)
        IsHealthy = ($missingInRuntime.Count -eq 0 -and $driftedFiles.Count -eq 0 -and ((-not $IncludeExtraRuntimeDrift) -or $extraInRuntime.Count -eq 0))
    }
}

# Reports repository-managed skill folders that should not remain duplicated in ~/.codex/skills.
function Test-CodexSkillDuplicateState {
    param(
        [string] $ManagedSkillRoot,
        [string] $CodexSkillsRoot
    )

    $managedSkillNames = @(Get-ManagedSkillNameList -SkillRoot $ManagedSkillRoot)
    $duplicates = New-Object System.Collections.Generic.List[string]

    if (Test-Path -LiteralPath $CodexSkillsRoot -PathType Container) {
        foreach ($skillName in $managedSkillNames) {
            $candidate = Join-Path $CodexSkillsRoot $skillName
            if (Test-Path -LiteralPath $candidate) {
                $duplicates.Add($skillName) | Out-Null
            }
        }

    }

    return [pscustomobject]@{
        Name = 'repo-managed skill duplicates in runtime .codex/skills'
        SourcePath = $ManagedSkillRoot
        TargetPath = $CodexSkillsRoot
        SourceCount = $managedSkillNames.Count
        TargetCount = if (Test-Path -LiteralPath $CodexSkillsRoot -PathType Container) { @(Get-ChildItem -LiteralPath $CodexSkillsRoot -Force).Count } else { 0 }
        MissingInRuntime = @()
        ExtraInRuntime = @($duplicates)
        DriftedFiles = @()
        IsHealthy = ($duplicates.Count -eq 0)
    }
}

# Prints a standardized summary for a single runtime mapping comparison.
function Write-MappingReport {
    param(
        [object] $Report,
        [switch] $DetailedReport
    )

    $statusText = if ($Report.IsHealthy) { 'OK' } else { 'DRIFT' }

    Write-StyledOutput ("[{0}] {1}" -f $statusText, $Report.Name)
    Write-StyledOutput ("  source: {0}" -f $Report.SourcePath)
    Write-StyledOutput ("  target: {0}" -f $Report.TargetPath)
    Write-StyledOutput ("  files: source={0} target={1}" -f $Report.SourceCount, $Report.TargetCount)
    Write-StyledOutput ("  missing in runtime: {0}" -f $Report.MissingInRuntime.Count)
    Write-StyledOutput ("  extra in runtime: {0}" -f $Report.ExtraInRuntime.Count)
    Write-StyledOutput ("  drifted files: {0}" -f $Report.DriftedFiles.Count)

    if ($DetailedReport) {
        foreach ($path in $Report.MissingInRuntime) {
            Write-StyledOutput ("    [missing] {0}" -f $path)
        }
        foreach ($path in $Report.ExtraInRuntime) {
            Write-StyledOutput ("    [extra]   {0}" -f $path)
        }
        foreach ($path in $Report.DriftedFiles) {
            Write-StyledOutput ("    [drift]   {0}" -f $path)
        }
    }
}

# Executes all runtime mapping drift checks and returns detailed reports.
function Invoke-Doctor {
    param(
        [string] $ResolvedRepoRoot
    )

    $managedSkillRoot = Join-Path $ResolvedRepoRoot '.codex\skills'
    $managedSkillPrefixes = @((Get-ManagedSkillNameList -SkillRoot $managedSkillRoot))

    $mappings = @(
        [pscustomobject]@{
            Name = '.github -> runtime'
            Source = Join-Path $ResolvedRepoRoot '.github'
            Target = $TargetGithubPath
            IgnoreExtraPrefixes = @('scripts\', 'scripts/')
        },
        [pscustomobject]@{
            Name = 'scripts -> runtime .github/scripts'
            Source = Join-Path $ResolvedRepoRoot 'scripts'
            Target = Join-Path $TargetGithubPath 'scripts'
            IgnoreExtraPrefixes = @()
        },
        [pscustomobject]@{
            Name = '.codex/skills -> runtime .agents/skills'
            Source = $managedSkillRoot
            Target = $TargetAgentsSkillsPath
            IncludePrefixes = $managedSkillPrefixes
            IgnoreSourcePrefixes = @('README.md')
            IgnoreExtraPrefixes = @((Get-ChildItem -LiteralPath $TargetAgentsSkillsPath -Directory -ErrorAction SilentlyContinue | Where-Object { $managedSkillPrefixes -notcontains $_.Name } | ForEach-Object { $_.Name }))
        },
        [pscustomobject]@{
            Name = '.github/skills -> runtime .copilot/skills'
            Source = Join-Path $ResolvedRepoRoot '.github\skills'
            Target = $TargetCopilotSkillsPath
            IgnoreExtraPrefixes = @()
        },
        [pscustomobject]@{
            Name = '.codex/mcp -> runtime'
            Source = Join-Path $ResolvedRepoRoot '.codex\mcp'
            Target = Join-Path $TargetCodexPath 'shared-mcp'
            IgnoreExtraPrefixes = @()
        },
        [pscustomobject]@{
            Name = '.codex/scripts (root tools) -> runtime'
            Source = Join-Path $ResolvedRepoRoot '.codex\scripts'
            Target = Join-Path $TargetCodexPath 'shared-scripts'
            IgnoreExtraPrefixes = @('common\', 'common/', 'security\', 'security/')
        },
        [pscustomobject]@{
            Name = 'scripts/common -> runtime'
            Source = Join-Path $ResolvedRepoRoot 'scripts\common'
            Target = Join-Path $TargetCodexPath 'shared-scripts\common'
            IgnoreExtraPrefixes = @()
        },
        [pscustomobject]@{
            Name = 'scripts/security -> runtime'
            Source = Join-Path $ResolvedRepoRoot 'scripts\security'
            Target = Join-Path $TargetCodexPath 'shared-scripts\security'
            IgnoreExtraPrefixes = @()
        },
        [pscustomobject]@{
            Name = '.codex/orchestration -> runtime'
            Source = Join-Path $ResolvedRepoRoot '.codex\orchestration'
            Target = Join-Path $TargetCodexPath 'shared-orchestration'
            IgnoreExtraPrefixes = @()
        }
    )

    $reports = @()
    foreach ($mapping in $mappings) {
        $includePrefixes = if ($null -ne $mapping.PSObject.Properties['IncludePrefixes']) { @($mapping.IncludePrefixes) } else { @() }
        $ignoreSourcePrefixes = if ($null -ne $mapping.PSObject.Properties['IgnoreSourcePrefixes']) { @($mapping.IgnoreSourcePrefixes) } else { @() }
        $reports += Compare-Mapping -Name $mapping.Name -SourcePath $mapping.Source -TargetPath $mapping.Target -IncludePrefixes $includePrefixes -IgnoreSourcePrefixes $ignoreSourcePrefixes -IgnoreExtraPrefixes $mapping.IgnoreExtraPrefixes -IncludeExtraRuntimeDrift:$StrictExtras
    }

    $reports += (Test-CodexSkillDuplicateState -ManagedSkillRoot $managedSkillRoot -CodexSkillsRoot (Join-Path $TargetCodexPath 'skills'))

    return $reports
}

# Checks whether runtime reports include any extra unmanaged files.
function Test-HasExtraRuntimeFile {
    param(
        [object[]] $Reports
    )

    return @($Reports | Where-Object { @($_.ExtraInRuntime).Count -gt 0 }).Count -gt 0
}

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
if ([string]::IsNullOrWhiteSpace($TargetCopilotSkillsPath)) {
    $TargetCopilotSkillsPath = Resolve-CopilotSkillsPath
}

Write-StyledOutput 'Runtime doctor report'
Write-StyledOutput ("  repo root: {0}" -f $resolvedRepoRoot)

$reports = Invoke-Doctor -ResolvedRepoRoot $resolvedRepoRoot
foreach ($report in $reports) {
    Write-MappingReport -Report $report -DetailedReport:$Detailed
}

$hasDrift = @($reports | Where-Object { -not $_.IsHealthy }).Count -gt 0
$hasExtras = Test-HasExtraRuntimeFile -Reports $reports

if ($hasDrift -and $SyncOnDrift) {
    Write-StyledOutput 'Drift detected. Running bootstrap sync...'
    $bootstrapScript = Join-Path $resolvedRepoRoot 'scripts\runtime\bootstrap.ps1'
    if (-not (Test-Path -LiteralPath $bootstrapScript)) {
        throw "Bootstrap script not found: $bootstrapScript"
    }

    & $bootstrapScript -RepoRoot $resolvedRepoRoot -TargetGithubPath $TargetGithubPath -TargetCodexPath $TargetCodexPath -TargetAgentsSkillsPath $TargetAgentsSkillsPath -TargetCopilotSkillsPath $TargetCopilotSkillsPath
    $reports = Invoke-Doctor -ResolvedRepoRoot $resolvedRepoRoot
    foreach ($report in $reports) {
        Write-MappingReport -Report $report -DetailedReport:$Detailed
    }
    $hasDrift = @($reports | Where-Object { -not $_.IsHealthy }).Count -gt 0
    $hasExtras = Test-HasExtraRuntimeFile -Reports $reports
}

Write-StyledOutput ''
$statusText = if ($hasDrift) {
    'detected'
}
elseif ($hasExtras) {
    'clean-with-extras'
}
else {
    'clean'
}

Write-StyledOutput ("Drift status: {0}" -f $statusText)

if ($hasExtras -and (-not $StrictExtras)) {
    Write-StyledOutput 'Runtime has extra files not tracked by source mappings.'
    Write-StyledOutput 'Use -Detailed to inspect extras and -StrictExtras to fail on extras.'
}

if ($hasDrift) {
    exit 1
}

exit 0