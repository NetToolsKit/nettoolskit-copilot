<#
.SYNOPSIS
    Applies configured staged-file EOF hygiene during pre-commit.

.DESCRIPTION
    Reads the local EOF hygiene mode for the current repository/worktree and,
    when `autofix` is enabled, trims staged text files before the commit
    proceeds. Files with both staged and unstaged changes are treated as
    unsafe for automatic restaging and block the commit with a clear message.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.PARAMETER Verbose
    Shows detailed diagnostics.

.EXAMPLE
    pwsh -File scripts/git-hooks/invoke-pre-commit-eof-hygiene.ps1

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+, Git.
#>

param(
    [string] $RepoRoot,
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
. $script:CommonBootstrapPath -CallerScriptRoot $PSScriptRoot -Helpers @('console-style', 'repository-paths', 'git-hook-eof-settings')
$script:IsVerboseEnabled = [bool] $Verbose

# Writes verbose diagnostics only when requested.
function Write-VerboseLog {
    param(
        [string] $Message
    )

    if ($script:IsVerboseEnabled) {
        Write-StyledOutput ("[VERBOSE] {0}" -f $Message)
    }
}

# Normalizes repository-relative paths into Git CLI-safe slash format.
function Convert-ToGitRelativePath {
    param(
        [Parameter(Mandatory = $true)]
        [string] $ResolvedRepoRoot,
        [Parameter(Mandatory = $true)]
        [string] $FullPath
    )

    return ([System.IO.Path]::GetRelativePath($ResolvedRepoRoot, $FullPath) -replace '\\', '/')
}

# Returns staged file paths eligible for EOF hygiene.
function Get-StagedFilePathList {
    param(
        [string] $ResolvedRepoRoot
    )

    $statusOutput = & git -C $ResolvedRepoRoot diff --cached --name-only --diff-filter=ACMR -z 2>$null
    if ($LASTEXITCODE -ne 0) {
        throw "Could not enumerate staged files for repository: $ResolvedRepoRoot"
    }

    $relativePaths = @($statusOutput -split "`0" | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
    $resolvedPaths = foreach ($relativePath in $relativePaths) {
        Join-Path $ResolvedRepoRoot $relativePath
    }

    return @($resolvedPaths | Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } | Select-Object -Unique)
}

# Returns staged files that also contain unstaged modifications and are unsafe to auto-restage.
function Get-UnsafeMixedStageFileList {
    param(
        [string] $ResolvedRepoRoot,
        [string[]] $StagedFiles
    )

    $unsafeFiles = New-Object System.Collections.Generic.List[string]
    foreach ($fullPath in @($StagedFiles)) {
        $relativePath = Convert-ToGitRelativePath -ResolvedRepoRoot $ResolvedRepoRoot -FullPath $fullPath
        & git -C $ResolvedRepoRoot diff --quiet -- $relativePath
        if ($LASTEXITCODE -eq 1) {
            $unsafeFiles.Add($fullPath) | Out-Null
        }
        elseif ($LASTEXITCODE -gt 1) {
            throw "Could not inspect unstaged changes for '$relativePath'."
        }
    }

    return @($unsafeFiles)
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
Set-Location -Path $resolvedRepoRoot
$effectiveMode = Get-EffectiveGitHookEofMode -ResolvedRepoRoot $resolvedRepoRoot

Write-VerboseLog ("EOF hook mode: {0} ({1})" -f $effectiveMode.Name, $effectiveMode.Source)

if (-not $effectiveMode.AutoFixStagedFiles) {
    exit 0
}

$stagedFiles = @(Get-StagedFilePathList -ResolvedRepoRoot $resolvedRepoRoot)
if ($stagedFiles.Count -eq 0) {
    exit 0
}

$unsafeFiles = @(Get-UnsafeMixedStageFileList -ResolvedRepoRoot $resolvedRepoRoot -StagedFiles $stagedFiles)
if ($unsafeFiles.Count -gt 0) {
    Write-StyledOutput '[pre-commit] EOF autofix blocked: these files have both staged and unstaged changes.'
    foreach ($unsafeFile in $unsafeFiles) {
        Write-StyledOutput ("  - {0}" -f [System.IO.Path]::GetRelativePath($resolvedRepoRoot, $unsafeFile))
    }
    Write-StyledOutput '[pre-commit] Run `git trim-eof` manually or stage the full file before committing.'
    exit 1
}

$trimScriptPath = Join-Path $resolvedRepoRoot 'scripts\maintenance\trim-trailing-blank-lines.ps1'
if (-not (Test-Path -LiteralPath $trimScriptPath -PathType Leaf)) {
    throw "Missing trim script required by EOF autofix: $trimScriptPath"
}

Write-StyledOutput ('[pre-commit] EOF autofix mode active. Checking {0} staged file(s)...' -f $stagedFiles.Count)
& $trimScriptPath -LiteralPaths $stagedFiles
if ($LASTEXITCODE -ne 0) {
    throw "EOF autofix failed while trimming staged files (exit code: $LASTEXITCODE)"
}

foreach ($stagedFile in $stagedFiles) {
    $relativePath = Convert-ToGitRelativePath -ResolvedRepoRoot $resolvedRepoRoot -FullPath $stagedFile
    & git -C $resolvedRepoRoot add -- $relativePath
    if ($LASTEXITCODE -ne 0) {
        throw "Could not re-stage file after EOF autofix: $relativePath"
    }
}

Write-StyledOutput '[pre-commit] EOF autofix completed.'
exit 0