<#
.SYNOPSIS
    Regenerates the shared script checksum manifest for external workflow consumption.

.DESCRIPTION
    Computes SHA256 checksums for repository-managed script roots and writes a
    deterministic manifest at `.github/governance/shared-script-checksums.manifest.json`.

    The manifest is intended for GitHub Actions workflows in external repositories
    that download scripts from this repository and verify integrity before execution.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.PARAMETER ManifestPath
    Manifest output path relative to repository root.

.PARAMETER IncludedRoots
    Script root folders (relative to repository root) included in checksum generation.

.PARAMETER Verbose
    Shows detailed diagnostics.

.EXAMPLE
    pwsh -File scripts/governance/update-shared-script-checksums-manifest.ps1

.EXAMPLE
    pwsh -File scripts/governance/update-shared-script-checksums-manifest.ps1 `
      -IncludedRoots scripts/common,scripts/security

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot,
    [string] $ManifestPath = '.github/governance/shared-script-checksums.manifest.json',
    [string[]] $IncludedRoots = @('scripts/common', 'scripts/security'),
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
$script:ScriptRoot = Split-Path -Path $PSCommandPath -Parent
$script:IsVerboseEnabled = [bool] $Verbose

# Writes verbose diagnostics.
function Write-VerboseLog {
    param(
        [string] $Message
    )

    if ($script:IsVerboseEnabled) {
        Write-StyledOutput ("[VERBOSE] {0}" -f $Message)
    }
}

# Resolves repository root from input and fallback candidates.
function Resolve-RepositoryRoot {
    param(
        [string] $RequestedRoot
    )

    $candidates = @()

    if (-not [string]::IsNullOrWhiteSpace($RequestedRoot)) {
        try {
            $candidates += (Resolve-Path -LiteralPath $RequestedRoot).Path
        }
        catch {
            throw "Invalid RepoRoot path: $RequestedRoot"
        }
    }

    if (-not [string]::IsNullOrWhiteSpace($script:ScriptRoot)) {
        $candidates += (Resolve-Path -LiteralPath (Join-Path $script:ScriptRoot '..\..')).Path
    }

    $candidates += (Get-Location).Path

    foreach ($candidate in ($candidates | Select-Object -Unique)) {
        $current = $candidate
        for ($index = 0; $index -lt 6 -and -not [string]::IsNullOrWhiteSpace($current); $index++) {
            $hasLayout = (Test-Path -LiteralPath (Join-Path $current '.github')) -and (Test-Path -LiteralPath (Join-Path $current '.codex'))
            if ($hasLayout) {
                Write-VerboseLog ("Repository root detected: {0}" -f $current)
                return $current
            }

            $current = Split-Path -Path $current -Parent
        }
    }

    throw 'Could not detect repository root containing both .github and .codex.'
}

# Resolves a repository-relative path into an absolute path.
function Resolve-RepoPath {
    param(
        [string] $Root,
        [string] $Path
    )

    if ([System.IO.Path]::IsPathRooted($Path)) {
        return [System.IO.Path]::GetFullPath($Path)
    }

    return [System.IO.Path]::GetFullPath((Join-Path $Root $Path))
}

# Converts absolute path to normalized repository-relative path.
function Convert-ToRelativePath {
    param(
        [string] $Root,
        [string] $Path
    )

    $relative = [System.IO.Path]::GetRelativePath($Root, $Path)
    return $relative.Replace('\', '/')
}

# Collects script files from included roots.
function Get-IncludedScriptFileList {
    param(
        [string] $Root,
        [string[]] $RootFolders
    )

    $fileSet = New-Object System.Collections.Generic.HashSet[string]([System.StringComparer]::OrdinalIgnoreCase)

    foreach ($rootFolder in $RootFolders) {
        $resolvedFolderPath = Resolve-RepoPath -Root $Root -Path $rootFolder
        if (-not (Test-Path -LiteralPath $resolvedFolderPath -PathType Container)) {
            throw ("Included root folder not found: {0}" -f $rootFolder)
        }

        Write-VerboseLog ("Scanning folder: {0}" -f $resolvedFolderPath)
        Get-ChildItem -LiteralPath $resolvedFolderPath -Recurse -File -Filter '*.ps1' | ForEach-Object {
            $fileSet.Add($_.FullName) | Out-Null
        }
    }

    return @($fileSet | Sort-Object)
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
Set-Location -Path $resolvedRepoRoot

$normalizedRoots = @($IncludedRoots | ForEach-Object { ([string] $_).Replace('\', '/') } | Sort-Object -Unique)
if ($normalizedRoots.Count -eq 0) {
    throw 'At least one IncludedRoots entry is required.'
}

$scriptFiles = Get-IncludedScriptFileList -Root $resolvedRepoRoot -RootFolders $normalizedRoots
if ($scriptFiles.Count -eq 0) {
    throw 'No .ps1 files found in IncludedRoots.'
}

$entryList = New-Object System.Collections.Generic.List[object]
foreach ($scriptFile in $scriptFiles) {
    $relativePath = Convert-ToRelativePath -Root $resolvedRepoRoot -Path $scriptFile
    $hash = (Get-FileHash -LiteralPath $scriptFile -Algorithm SHA256).Hash.ToLowerInvariant()
    $entryList.Add([ordered]@{
            path = $relativePath
            sha256 = $hash
        }) | Out-Null
}

$manifestObject = [ordered]@{
    version = 1
    sourceRepository = 'https://github.com/ThiagoGuislotti/copilot-instructions'
    hashAlgorithm = 'SHA256'
    includedRoots = $normalizedRoots
    entries = @($entryList | Sort-Object path)
}

$resolvedManifestPath = Resolve-RepoPath -Root $resolvedRepoRoot -Path $ManifestPath
$manifestDirectory = Split-Path -Path $resolvedManifestPath -Parent
if (-not [string]::IsNullOrWhiteSpace($manifestDirectory)) {
    New-Item -ItemType Directory -Path $manifestDirectory -Force | Out-Null
}

$manifestJson = $manifestObject | ConvertTo-Json -Depth 20
Set-Content -LiteralPath $resolvedManifestPath -Value $manifestJson -Encoding UTF8

Write-StyledOutput 'Shared script checksum manifest updated.'
Write-StyledOutput ("  manifest: {0}" -f $resolvedManifestPath)
Write-StyledOutput ("  roots: {0}" -f ($normalizedRoots -join ', '))
Write-StyledOutput ("  entries: {0}" -f $entryList.Count)

exit 0