<#
.SYNOPSIS
    Runtime tests for EOF normalization through the native `ntk runtime trim-trailing-blank-lines` surface.

.DESCRIPTION
    Verifies that the maintenance script:
    - removes trailing blank lines for default text files without adding a final newline
    - keeps a final newline for Rust files when the workspace `.editorconfig` requires it
    - can limit trimming to files currently reported as changed by Git

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

$script:ScriptRoot = Split-Path -Path $PSCommandPath -Parent
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
. $script:CommonBootstrapPath -CallerScriptRoot $PSScriptRoot -Helpers @('repository-paths', 'runtime-paths')
# Fails the current runtime test when the supplied condition is false.
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

# Initializes a deterministic temporary git repository for runtime tests.
function Initialize-GitRepository {
    param(
        [string] $Path
    )

    & git -C $Path init | Out-Null
    & git -C $Path config core.hooksPath .git/hooks | Out-Null
    & git -C $Path config core.autocrlf false | Out-Null
    & git -C $Path config user.name 'Test User' | Out-Null
    & git -C $Path config user.email 'test@example.com' | Out-Null
}

# Creates the minimal runtime markers required by the Rust trim command.
function Initialize-RuntimeRepoRoot {
    param(
        [string] $Path
    )

    New-Item -ItemType Directory -Path (Join-Path $Path '.github') -Force | Out-Null
    New-Item -ItemType Directory -Path (Join-Path $Path '.codex') -Force | Out-Null
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$runtimeBinaryPath = Resolve-NtkRuntimeBinaryPath -ResolvedRepoRoot $resolvedRepoRoot -RuntimePreference codex

try {
    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    try {
        Initialize-RuntimeRepoRoot -Path $tempRoot
        Write-TextFile -Path (Join-Path $tempRoot '.editorconfig') -Content @"
root = true

[*]
insert_final_newline = false

[*.{rs,toml,lock}]
insert_final_newline = true
"@

        $dotnetFile = Join-Path $tempRoot 'sample.cs'
        Write-TextFile -Path $dotnetFile -Content "public sealed class Sample { }`n`n"

        & $runtimeBinaryPath runtime trim-trailing-blank-lines --repo-root $tempRoot --path $dotnetFile | Out-Null
        $dotnetText = [System.IO.File]::ReadAllText($dotnetFile)
        Assert-Equal -Actual $dotnetText -Expected 'public sealed class Sample { }' -Message 'Default files must end on the last content character with no final newline.'

        $rustFile = Join-Path $tempRoot 'lib.rs'
        Write-TextFile -Path $rustFile -Content "pub fn sample() {}`n`n"

        & $runtimeBinaryPath runtime trim-trailing-blank-lines --repo-root $tempRoot --path $rustFile | Out-Null
        $rustText = [System.IO.File]::ReadAllText($rustFile)
        Assert-Equal -Actual $rustText -Expected "pub fn sample() {}`n" -Message 'Rust files must keep one final newline when the workspace .editorconfig requires it.'
        Assert-True -Condition $rustText.EndsWith("`n") -Message 'Rust files must keep a final newline when .editorconfig sets insert_final_newline = true.'

        $gitRepoRoot = Join-Path $tempRoot 'git-repo'
        [void] (New-Item -ItemType Directory -Path $gitRepoRoot -Force)
        Initialize-RuntimeRepoRoot -Path $gitRepoRoot
        Initialize-GitRepository -Path $gitRepoRoot

        $changedFile = Join-Path $gitRepoRoot 'changed.cs'
        $cleanTrackedFile = Join-Path $gitRepoRoot 'clean.cs'
        $untrackedChangedFile = Join-Path $gitRepoRoot 'new.md'

        Write-TextFile -Path $changedFile -Content 'public sealed class Changed { }'
        Write-TextFile -Path $cleanTrackedFile -Content "public sealed class Clean { }`n`n"
        & git -C $gitRepoRoot add changed.cs clean.cs | Out-Null
        & git -C $gitRepoRoot commit -m 'initial' | Out-Null

        Write-TextFile -Path $changedFile -Content "public sealed class Changed { }`n`n"
        Write-TextFile -Path $untrackedChangedFile -Content "# new file`n`n"

        $gitModeOutput = & $runtimeBinaryPath runtime trim-trailing-blank-lines --repo-root $gitRepoRoot --git-changed-only
        $changedText = [System.IO.File]::ReadAllText($changedFile)
        $cleanTrackedText = [System.IO.File]::ReadAllText($cleanTrackedFile)
        $untrackedChangedText = [System.IO.File]::ReadAllText($untrackedChangedFile)

        Assert-Equal -Actual $changedText -Expected 'public sealed class Changed { }' -Message 'GitChangedOnly mode must trim modified tracked files.'
        Assert-Equal -Actual $cleanTrackedText -Expected "public sealed class Clean { }`n`n" -Message 'GitChangedOnly mode must not touch clean tracked files that are absent from git status.'
        Assert-Equal -Actual $untrackedChangedText -Expected '# new file' -Message 'GitChangedOnly mode must trim untracked files that appear in git status.'
        Assert-True -Condition (($gitModeOutput -join "`n") -match 'Git changed files mode: enabled') -Message 'GitChangedOnly mode should announce that git-status-based discovery is active.'
        Assert-True -Condition (($gitModeOutput -join "`n") -match 'changed\.cs') -Message 'GitChangedOnly mode should list modified tracked files selected for trim.'
        Assert-True -Condition (($gitModeOutput -join "`n") -match 'new\.md') -Message 'GitChangedOnly mode should list untracked files selected for trim.'
        Assert-True -Condition (-not (($gitModeOutput -join "`n") -match 'clean\.cs')) -Message 'GitChangedOnly mode should not list clean tracked files.'

        $singleChangeRepoRoot = Join-Path $tempRoot 'single-change-git-repo'
        [void] (New-Item -ItemType Directory -Path $singleChangeRepoRoot -Force)
        Initialize-RuntimeRepoRoot -Path $singleChangeRepoRoot
        Initialize-GitRepository -Path $singleChangeRepoRoot

        $singleChangedFile = Join-Path $singleChangeRepoRoot 'single.cs'
        Write-TextFile -Path $singleChangedFile -Content 'public sealed class Single { }'
        & git -C $singleChangeRepoRoot add single.cs | Out-Null
        & git -C $singleChangeRepoRoot commit -m 'initial' | Out-Null

        Write-TextFile -Path $singleChangedFile -Content "public sealed class Single { }`n`n"

        $singleGitModeOutput = & $runtimeBinaryPath runtime trim-trailing-blank-lines --repo-root $singleChangeRepoRoot --git-changed-only
        $singleChangedText = [System.IO.File]::ReadAllText($singleChangedFile)

        Assert-Equal -Actual $singleChangedText -Expected 'public sealed class Single { }' -Message 'GitChangedOnly mode must trim a single modified tracked file without failing scalar Count access.'
        Assert-True -Condition (($singleGitModeOutput -join "`n") -match 'Files found: 1') -Message 'GitChangedOnly mode should report a single changed file cleanly.'
        Assert-True -Condition (($singleGitModeOutput -join "`n") -match 'single\.cs') -Message 'GitChangedOnly mode should list the single changed tracked file.'
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