<#
.SYNOPSIS
    Validates the versioned planning workspace structure under .temp/planning.

.DESCRIPTION
    Enforces the repository contract for versioned planning artifacts under
    `.temp/planning`, including required directories, tracked placeholder files,
    and `.gitignore` exceptions that keep planning versioned while the rest of
    `.temp` stays ignored.

.PARAMETER RepoRoot
    Repository root used to resolve the planning workspace structure.

.PARAMETER WarningOnly
    When true, structural failures are emitted as warnings and exit code remains 0.

.EXAMPLE
    pwsh -File scripts/validation/validate-planning-structure.ps1 -RepoRoot . -WarningOnly:$false

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot,
    [bool] $WarningOnly = $true
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Resolves the repository root used for planning workspace checks.
function Resolve-RepositoryRoot {
    param([string] $RequestedRoot)

    if (-not [string]::IsNullOrWhiteSpace($RequestedRoot)) {
        return (Resolve-Path -LiteralPath $RequestedRoot).Path
    }

    return (Get-Location).Path
}

# Records either a failure or a warning based on the selected validation mode.
function Add-ValidationMessage {
    param(
        [string] $Message,
        [System.Collections.Generic.List[string]] $Warnings,
        [System.Collections.Generic.List[string]] $Failures,
        [bool] $WarningOnlyMode
    )

    if ($WarningOnlyMode) {
        $Warnings.Add($Message) | Out-Null
        Write-Host ("[WARN] {0}" -f $Message)
    }
    else {
        $Failures.Add($Message) | Out-Null
        Write-Host ("[FAIL] {0}" -f $Message)
    }
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$warnings = New-Object System.Collections.Generic.List[string]
$failures = New-Object System.Collections.Generic.List[string]

$requiredFiles = @(
    '.temp/planning/README.md',
    '.temp/planning/plans-active/.gitkeep',
    '.temp/planning/plans-completed/.gitkeep'
)

foreach ($relativePath in $requiredFiles) {
    $absolutePath = Join-Path $resolvedRepoRoot $relativePath
    if (-not (Test-Path -LiteralPath $absolutePath -PathType Leaf)) {
        Add-ValidationMessage -Message ("Missing required planning file: {0}" -f $relativePath) -Warnings $warnings -Failures $failures -WarningOnlyMode $WarningOnly
    }
}

$requiredDirectories = @(
    '.temp/planning',
    '.temp/planning/plans-active',
    '.temp/planning/plans-completed'
)

foreach ($relativePath in $requiredDirectories) {
    $absolutePath = Join-Path $resolvedRepoRoot $relativePath
    if (-not (Test-Path -LiteralPath $absolutePath -PathType Container)) {
        Add-ValidationMessage -Message ("Missing required planning directory: {0}" -f $relativePath) -Warnings $warnings -Failures $failures -WarningOnlyMode $WarningOnly
    }
}

$gitIgnorePath = Join-Path $resolvedRepoRoot '.gitignore'
if (Test-Path -LiteralPath $gitIgnorePath -PathType Leaf) {
    $gitIgnoreContent = Get-Content -Raw -LiteralPath $gitIgnorePath
    foreach ($requiredPattern in @('.temp/*', '!.temp/planning/', '!.temp/planning/**')) {
        if ($gitIgnoreContent -notmatch [regex]::Escape($requiredPattern)) {
            Add-ValidationMessage -Message (".gitignore missing planning pattern: {0}" -f $requiredPattern) -Warnings $warnings -Failures $failures -WarningOnlyMode $WarningOnly
        }
    }
}
else {
    Add-ValidationMessage -Message 'Missing .gitignore for planning workspace validation.' -Warnings $warnings -Failures $failures -WarningOnlyMode $WarningOnly
}

Write-Host ''
Write-Host 'Planning structure validation summary'
Write-Host ("  Warnings: {0}" -f $warnings.Count)
Write-Host ("  Failures: {0}" -f $failures.Count)

if ($failures.Count -gt 0) {
    exit 1
}

Write-Host 'Planning workspace validation passed.'
exit 0