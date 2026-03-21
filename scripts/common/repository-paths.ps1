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

# Resets the runtime issue registry used by execution logging.
function Initialize-ExecutionIssueTracking {
    $script:ExecutionIssuesBySignature = @{}
    $script:ExecutionIssuesById = [ordered]@{}
    $script:ExecutionIssueSequence = @{
        warning = 0
        error = 0
    }
}

# Maps log levels to supported issue severity buckets.
function Get-ExecutionIssueSeverity {
    param(
        [string] $Level
    )

    $normalizedLevel = [string] $Level
    switch ($normalizedLevel.ToUpperInvariant()) {
        'WARN' { return 'warning' }
        'WARNING' { return 'warning' }
        'ERROR' { return 'error' }
        'FAIL' { return 'error' }
        default { return $null }
    }
}

# Creates the next issue id for the supplied severity.
function New-ExecutionIssueId {
    param(
        [ValidateSet('warning', 'error')]
        [string] $Severity
    )

    if ($null -eq $script:ExecutionIssueSequence) {
        Initialize-ExecutionIssueTracking
    }

    $script:ExecutionIssueSequence[$Severity] = [int] $script:ExecutionIssueSequence[$Severity] + 1
    $prefix = if ($Severity -eq 'error') { 'ERR' } else { 'WRN' }
    return ("{0}{1:D3}" -f $prefix, $script:ExecutionIssueSequence[$Severity])
}

# Registers a deduplicated warning/error issue for later summary output.
function Register-ExecutionIssue {
    param(
        [string] $Level,
        [string] $Code,
        [string] $Message
    )

    $severity = Get-ExecutionIssueSeverity -Level $Level
    if ([string]::IsNullOrWhiteSpace($severity)) {
        return $null
    }

    if ($null -eq $script:ExecutionIssuesBySignature -or $null -eq $script:ExecutionIssuesById -or $null -eq $script:ExecutionIssueSequence) {
        Initialize-ExecutionIssueTracking
    }

    $normalizedCode = if ([string]::IsNullOrWhiteSpace($Code)) {
        if ($severity -eq 'error') { 'UNSPECIFIED_ERROR' } else { 'UNSPECIFIED_WARNING' }
    }
    else {
        ([string] $Code).Trim().ToUpperInvariant()
    }

    $normalizedMessage = if ([string]::IsNullOrWhiteSpace($Message)) { '' } else { ([string] $Message).Trim() }
    $signature = "{0}|{1}|{2}" -f $severity, $normalizedCode, $normalizedMessage

    if ($script:ExecutionIssuesBySignature.ContainsKey($signature)) {
        $existingId = [string] $script:ExecutionIssuesBySignature[$signature]
        $existingIssue = $script:ExecutionIssuesById[$existingId]
        $existingIssue.occurrences = [int] $existingIssue.occurrences + 1
        $existingIssue.lastSeenAt = (Get-Date).ToString('o')
        return $existingIssue
    }

    $issueId = New-ExecutionIssueId -Severity $severity
    $issue = [pscustomobject]@{
        id = $issueId
        severity = $severity
        code = $normalizedCode
        message = $normalizedMessage
        occurrences = 1
        firstSeenAt = (Get-Date).ToString('o')
        lastSeenAt = (Get-Date).ToString('o')
    }

    $script:ExecutionIssuesBySignature[$signature] = $issueId
    $script:ExecutionIssuesById[$issueId] = $issue
    return $issue
}

# Returns the current execution issue registry as a summary object.
function Get-ExecutionIssueSummary {
    if ($null -eq $script:ExecutionIssuesById) {
        Initialize-ExecutionIssueTracking
    }

    $issues = @($script:ExecutionIssuesById.Values)
    $warningCount = @($issues | Where-Object { $_.severity -eq 'warning' }).Count
    $errorCount = @($issues | Where-Object { $_.severity -eq 'error' }).Count
    $warningMeasure = @($issues | Where-Object { $_.severity -eq 'warning' } | Measure-Object -Property occurrences -Sum) | Select-Object -First 1
    $errorMeasure = @($issues | Where-Object { $_.severity -eq 'error' } | Measure-Object -Property occurrences -Sum) | Select-Object -First 1
    $warningOccurrences = if ($null -eq $warningMeasure -or $null -eq $warningMeasure.Sum) { 0 } else { [int] $warningMeasure.Sum }
    $errorOccurrences = if ($null -eq $errorMeasure -or $null -eq $errorMeasure.Sum) { 0 } else { [int] $errorMeasure.Sum }

    return [pscustomobject]@{
        warnings = $warningCount
        errors = $errorCount
        warningOccurrences = $warningOccurrences
        errorOccurrences = $errorOccurrences
        totalIssues = $issues.Count
        totalOccurrences = $warningOccurrences + $errorOccurrences
        issues = @(
            $issues |
                Sort-Object -Property @{
                    Expression = { if ($_.severity -eq 'error') { 0 } else { 1 } }
                }, @{
                    Expression = { $_.id }
                }
        )
    }
}

# Writes the deduplicated warning/error issue summary for the current run.
function Write-ExecutionIssueSummary {
    param(
        [string] $Title = 'Execution issue summary'
    )

    $summary = Get-ExecutionIssueSummary
    $summaryLines = New-Object System.Collections.Generic.List[string]

    $summaryLines.Add('') | Out-Null
    $summaryLines.Add($Title) | Out-Null
    $summaryLines.Add('  Severity counts') | Out-Null
    $summaryLines.Add('  Severity  Distinct  Occurrences') | Out-Null
    $summaryLines.Add(("  Err       {0,-8} {1}" -f $summary.errors, $summary.errorOccurrences)) | Out-Null
    $summaryLines.Add(("  Wrn       {0,-8} {1}" -f $summary.warnings, $summary.warningOccurrences)) | Out-Null
    $summaryLines.Add(("  Total     {0,-8} {1}" -f $summary.totalIssues, $summary.totalOccurrences)) | Out-Null

    if ($summary.totalIssues -eq 0) {
        $summaryLines.Add('  Issues') | Out-Null
        $summaryLines.Add('    none') | Out-Null
    }
    else {
        $summaryLines.Add('  Issues') | Out-Null
        foreach ($issue in $summary.issues) {
            $summaryLines.Add(("    {0} | {1} | occurrences={2}" -f $issue.id, $issue.code, $issue.occurrences)) | Out-Null
        }
    }

    foreach ($line in $summaryLines) {
        Write-StyledOutput $line | Out-Host
    }

    if ($null -ne $script:LogFilePath) {
        foreach ($line in $summaryLines) {
            Add-Content -LiteralPath $script:LogFilePath -Value $line
        }
    }

    return $summary
}

# Writes execution log entries to console output and optional log file.
function Write-ExecutionLog {
    param(
        [string] $Level,
        [string] $Message,
        [string] $Code
    )

    $timestamp = (Get-Date).ToString('o')
    $issue = Register-ExecutionIssue -Level $Level -Code $Code -Message $Message
    $line = if ($null -eq $issue) {
        "[{0}] [{1}] {2}" -f $timestamp, $Level, $Message
    }
    else {
        "[{0}] [{1}] [{2}] [{3}] {4}" -f $timestamp, $Level, $issue.id, $issue.code, $Message
    }

    if ($null -ne $script:LogFilePath) {
        Add-Content -LiteralPath $script:LogFilePath -Value $line
    }

    Write-StyledOutput $line | Out-Host
}