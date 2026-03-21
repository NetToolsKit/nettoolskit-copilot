<#
.SYNOPSIS
    Configures local Git hooks for instruction/policy validation and runtime sync workflows.

.DESCRIPTION
    Sets `core.hooksPath` to `.githooks` for this repository and verifies required hook files:
    - pre-commit
    - post-commit
    - post-merge
    - post-checkout

    On Linux/macOS, marks hook scripts as executable.

    When `-Uninstall` is used, removes local `core.hooksPath` configuration.

.PARAMETER RepoRoot
    Optional repository root. If omitted, the script detects root from script location.

.PARAMETER EofHygieneMode
    Local pre-commit EOF hygiene mode for this clone/worktree. Supported
    values are defined in `.github/governance/git-hook-eof-modes.json`.

.PARAMETER EofHygieneScope
    Scope for persisting the EOF hygiene mode selection. Supported values are
    defined in `.github/governance/git-hook-eof-modes.json`.

.PARAMETER Uninstall
    Removes local `core.hooksPath` configuration instead of setting it.

.PARAMETER Verbose
    Shows detailed diagnostics.

.EXAMPLE
    pwsh -File scripts/git-hooks/setup-git-hooks.ps1

.EXAMPLE
    pwsh -File scripts/git-hooks/setup-git-hooks.ps1 -RepoRoot C:\repo\copilot-instructions

.EXAMPLE
    pwsh -File scripts/git-hooks/setup-git-hooks.ps1 -Uninstall

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+, Git.
#>

param(
    [string] $RepoRoot,
    [string] $EofHygieneMode,
    [string] $EofHygieneScope,
    [switch] $Uninstall,
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
$script:ScriptRoot = Split-Path -Path $PSCommandPath -Parent
$script:IsVerboseEnabled = [bool] $Verbose

# -------------------------------
# Helpers
# -------------------------------
# Validates that a required command is available in the current environment.
function Assert-CommandAvailable {
    param(
        [string] $CommandName
    )

    if ($null -eq (Get-Command $CommandName -ErrorAction SilentlyContinue)) {
        throw ("Required command not found: {0}" -f $CommandName)
    }
}

# Validates that a required file path exists before execution continues.
function Assert-PathPresent {
    param(
        [string] $Path,
        [string] $Label
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw ("Missing {0}: {1}" -f $Label, $Path)
    }
}

# Applies executable permission to hook files on Unix-like systems.
function Invoke-HookExecutabilityUpdate {
    param(
        [string[]] $HookPaths
    )

    if (-not ($IsLinux -or $IsMacOS)) {
        return
    }

    foreach ($hookPath in $HookPaths) {
        & chmod +x $hookPath
        if ($LASTEXITCODE -ne 0) {
            throw ("Failed to set executable permission: {0}" -f $hookPath)
        }
    }
}

# -------------------------------
# Main execution
# -------------------------------
$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
Set-Location -Path $resolvedRepoRoot
Assert-CommandAvailable -CommandName 'git'

$gitRoot = (& git -C $resolvedRepoRoot rev-parse --show-toplevel 2>$null)
if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($gitRoot)) {
    throw ("Current folder is not a Git repository: {0}" -f $resolvedRepoRoot)
}

if ($Uninstall) {
    $resolvedRemovalScope = if (-not [string]::IsNullOrWhiteSpace($EofHygieneScope)) {
        Resolve-GitHookEofScope -ResolvedRepoRoot $resolvedRepoRoot -ScopeName $EofHygieneScope
    }
    else {
        Resolve-GitHookEofScope -ResolvedRepoRoot $resolvedRepoRoot -ScopeName 'local-repo'
    }

    & git -C $resolvedRepoRoot config --local --unset core.hooksPath 2>$null
    if ($LASTEXITCODE -ne 0) {
        Write-StyledOutput 'No local core.hooksPath configured.'
        $removedSettingsPath = Remove-GitHookEofModeSelection -ResolvedRepoRoot $resolvedRepoRoot -ScopeName $resolvedRemovalScope.Name
        if (Test-Path -LiteralPath $removedSettingsPath -PathType Leaf) {
            Write-StyledOutput ("Warning: could not remove local EOF hook settings: {0}" -f $removedSettingsPath)
        }
        exit 0
    }

    $removedSettingsPath = Remove-GitHookEofModeSelection -ResolvedRepoRoot $resolvedRepoRoot -ScopeName $resolvedRemovalScope.Name
    Write-StyledOutput 'Removed local Git hook path (core.hooksPath).'
    Write-StyledOutput ("Removed EOF hook settings ({0}): {1}" -f $resolvedRemovalScope.Name, $removedSettingsPath)
    exit 0
}

$hooksDirectory = Join-Path $resolvedRepoRoot '.githooks'
if (-not (Test-Path -LiteralPath $hooksDirectory -PathType Container)) {
    throw ("Missing hook directory: {0}" -f $hooksDirectory)
}

$requiredHooks = @(
    'pre-commit',
    'post-commit',
    'post-merge',
    'post-checkout'
)

$hookPaths = New-Object System.Collections.Generic.List[string]
foreach ($hookName in $requiredHooks) {
    $hookPath = Join-Path $hooksDirectory $hookName
    Assert-PathPresent -Path $hookPath -Label ("hook file '{0}'" -f $hookName)
    $hookPaths.Add($hookPath) | Out-Null
}

& git -C $resolvedRepoRoot config --local core.hooksPath '.githooks'
if ($LASTEXITCODE -ne 0) {
    throw 'Failed to configure local Git hook path.'
}

$effectiveEofSelection = Get-EffectiveGitHookEofMode -ResolvedRepoRoot $resolvedRepoRoot
$eofScopeSelection = if (-not [string]::IsNullOrWhiteSpace($EofHygieneScope)) {
    Resolve-GitHookEofScope -ResolvedRepoRoot $resolvedRepoRoot -ScopeName $EofHygieneScope
}
else {
    $null
}
$shouldPersistEofSelection = (-not [string]::IsNullOrWhiteSpace($EofHygieneMode)) -or ($null -ne $eofScopeSelection)
$eofModeSelection = if ($shouldPersistEofSelection) {
    $targetModeName = if (-not [string]::IsNullOrWhiteSpace($EofHygieneMode)) { $EofHygieneMode } else { $effectiveEofSelection.Name }
    $targetScopeName = if ($null -ne $eofScopeSelection) { $eofScopeSelection.Name } else { 'local-repo' }
    $selection = Set-GitHookEofModeSelection -ResolvedRepoRoot $resolvedRepoRoot -ModeName $targetModeName -ScopeName $targetScopeName
    if ($selection.Scope.Name -eq 'global') {
        Remove-GitHookEofModeSelection -ResolvedRepoRoot $resolvedRepoRoot -ScopeName 'local-repo' | Out-Null
    }

    $selection
}
else {
    [pscustomobject]@{
        Mode = (Resolve-GitHookEofMode -ResolvedRepoRoot $resolvedRepoRoot -ModeName $effectiveEofSelection.Name)
        Scope = (Resolve-GitHookEofScope -ResolvedRepoRoot $resolvedRepoRoot -ScopeName $effectiveEofSelection.Scope)
        SettingsPath = $effectiveEofSelection.SettingsPath
        Source = $effectiveEofSelection.Source
    }
}
Invoke-HookExecutabilityUpdate -HookPaths $hookPaths.ToArray()

$configuredPath = (& git -C $resolvedRepoRoot config --local --get core.hooksPath 2>$null)
if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($configuredPath)) {
    throw 'Could not read configured core.hooksPath.'
}

Write-StyledOutput 'Git hooks configured successfully.'
Write-StyledOutput ("  repo: {0}" -f $gitRoot)
Write-StyledOutput ("  core.hooksPath: {0}" -f $configuredPath)
Write-StyledOutput ("  pre-commit: .githooks/pre-commit (EOF mode={0}; scope={1}; runs validate-all with profile=dev, warning-only, best effort)" -f $eofModeSelection.Mode.Name, $eofModeSelection.Scope.Name)
if ($null -ne $eofModeSelection.SettingsPath) {
    Write-StyledOutput ("  EOF hook settings ({0}): {1}" -f $eofModeSelection.Scope.Name, $eofModeSelection.SettingsPath)
}
else {
    Write-StyledOutput ("  EOF hook settings source: {0}" -f $eofModeSelection.Source)
}
Write-StyledOutput '  post-commit: .githooks/post-commit (syncs ~/.github and ~/.codex via scripts/runtime/bootstrap.ps1)'
Write-StyledOutput '  post-merge: .githooks/post-merge (runs validate-all with profile=release, warning-only, best effort)'
Write-StyledOutput '  post-checkout: .githooks/post-checkout (runs validate-all with profile=dev, warning-only, best effort)'
Write-StyledOutput '  skip sync (temporary): set CODEX_SKIP_POST_COMMIT_SYNC=1'
Write-StyledOutput '  optional MCP apply on manifest change: set CODEX_APPLY_MCP_ON_POST_COMMIT=1'
Write-StyledOutput '  MCP apply backup default: CODEX_BACKUP_MCP_CONFIG=1 (set 0 to disable backup)'

exit 0