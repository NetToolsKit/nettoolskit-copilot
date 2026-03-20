<#
.SYNOPSIS
    Shared repository path and logging helpers for runtime and validation scripts.

.DESCRIPTION
    Provides helper functions for:
    - repository root discovery
    - repository-relative path resolution
    - parent directory resolution
    - verbose diagnostics
    - structured execution logging

    Consumers are expected to dot-source `console-style.ps1` first and set:
    - `$script:ScriptRoot`
    - `$script:IsVerboseEnabled`
    - optionally `$script:LogFilePath`

.PARAMETER None
    This helper script does not require input parameters.

.EXAMPLE
    . ./scripts/common/console-style.ps1
    . ./scripts/common/repository-paths.ps1
    $root = Resolve-RepositoryRoot -RequestedRoot $RepoRoot

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param()

$ErrorActionPreference = 'Stop'

# Writes verbose diagnostics with a logical color label.
function Write-VerboseColor {
    param(
        [string] $Message,
        [ConsoleColor] $Color = [ConsoleColor]::Gray
    )

    if ($script:IsVerboseEnabled) {
        Write-StyledOutput ("[VERBOSE:{0}] {1}" -f $Color, $Message)
    }
}

# Resolves the repository root using explicit and fallback location candidates.
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
                Write-VerboseColor ("Repository root detected: {0}" -f $current) 'Green'
                return $current
            }

            $current = Split-Path -Path $current -Parent
        }
    }

    throw 'Could not detect repository root containing both .github and .codex.'
}

# Builds an absolute path from repository root and relative input path.
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

# Returns the parent directory for a given file path when available.
function Get-ParentDirectoryPath {
    param(
        [string] $Path
    )

    $parent = Split-Path -Path $Path -Parent
    if ([string]::IsNullOrWhiteSpace($parent)) {
        return $null
    }

    return $parent
}

# Writes execution log entries to console output and optional log file.
function Write-ExecutionLog {
    param(
        [string] $Level,
        [string] $Message
    )

    $timestamp = (Get-Date).ToString('o')
    $line = "[{0}] [{1}] {2}" -f $timestamp, $Level, $Message

    if ($null -ne $script:LogFilePath) {
        Add-Content -LiteralPath $script:LogFilePath -Value $line
    }

    Write-StyledOutput $line
}