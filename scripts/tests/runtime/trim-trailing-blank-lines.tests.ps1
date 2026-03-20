<#
.SYNOPSIS
    Runtime tests for EOF normalization in trim-trailing-blank-lines.ps1.

.DESCRIPTION
    Verifies that the maintenance script:
    - removes trailing blank lines for default text files without adding a final newline
    - applies the same no-final-newline policy to Rust files

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/trim-trailing-blank-lines.tests.ps1

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Resolves the repository root for the current script or test fixture.
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

    $candidates += (Get-Location).Path

    foreach ($candidate in ($candidates | Select-Object -Unique)) {
        $current = $candidate
        for ($index = 0; $index -lt 6 -and -not [string]::IsNullOrWhiteSpace($current); $index++) {
            $hasLayout = (Test-Path -LiteralPath (Join-Path $current '.github')) -and (Test-Path -LiteralPath (Join-Path $current '.codex'))
            if ($hasLayout) {
                return $current
            }

            $current = Split-Path -Path $current -Parent
        }
    }

    throw 'Could not detect repository root containing both .github and .codex.'
}

# Fails the current test when the supplied condition is false.
function Assert-True {
    param(
        [bool] $Condition,
        [string] $Message
    )

    if (-not $Condition) {
        throw $Message
    }
}

# Fails the current test when the actual and expected values differ.
function Assert-Equal {
    param(
        [object] $Actual,
        [object] $Expected,
        [string] $Message
    )

    if ($Actual -ne $Expected) {
        throw ("{0} Expected='{1}' Actual='{2}'" -f $Message, $Expected, $Actual)
    }
}

# Writes deterministic UTF-8 test content to disk.
function Write-TextFile {
    param(
        [string] $Path,
        [string] $Content
    )

    $parent = Split-Path -Path $Path -Parent
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }

    [System.IO.File]::WriteAllText($Path, $Content, [System.Text.UTF8Encoding]::new($false))
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$scriptPath = Join-Path $resolvedRepoRoot 'scripts/maintenance/trim-trailing-blank-lines.ps1'

try {
    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    try {
        $dotnetFile = Join-Path $tempRoot 'sample.cs'
        Write-TextFile -Path $dotnetFile -Content "public sealed class Sample { }`n`n"

        & $scriptPath -Path $dotnetFile | Out-Null
        $dotnetText = [System.IO.File]::ReadAllText($dotnetFile)
        Assert-Equal -Actual $dotnetText -Expected 'public sealed class Sample { }' -Message 'Default files must end on the last content character with no final newline.'

        $rustFile = Join-Path $tempRoot 'lib.rs'
        Write-TextFile -Path $rustFile -Content "pub fn sample() {}`n`n"

        & $scriptPath -Path $rustFile | Out-Null
        $rustText = [System.IO.File]::ReadAllText($rustFile)
        Assert-Equal -Actual $rustText -Expected 'pub fn sample() {}' -Message 'Rust files must end on the last content character with no final newline.'
        Assert-True -Condition (-not $rustText.EndsWith("`n")) -Message 'Rust files must not keep a final newline under the repository policy.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    Write-Host '[OK] trim trailing blank lines tests passed.'
    exit 0
}
catch {
    Write-Host ("[FAIL] trim trailing blank lines tests failed: {0}" -f $_.Exception.Message)
    exit 1
}