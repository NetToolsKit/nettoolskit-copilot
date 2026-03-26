<#
.SYNOPSIS
    Renders repository-owned GitHub/Copilot instruction surfaces from the authoritative definitions tree.

.DESCRIPTION
    Mirrors the authoritative GitHub provider tree plus shared instruction assets
    into the tracked `.github/` instruction/runtime surfaces:
    - managed provider root files (`AGENTS.md`, `COMMANDS.md`,
      `copilot-instructions.md`, `instruction-routing.catalog.yml`)
    - `.github/agents/`
    - `.github/chatmodes/`
    - `.github/instructions/` from `definitions/shared/instructions/`
    - `.github/prompts/`
    - `.github/hooks/`
    - `.github/templates/` from `definitions/shared/templates/`

    GitHub-native repository/community assets such as `.github/ISSUE_TEMPLATE/`,
    `.github/PULL_REQUEST_TEMPLATE.md`, `.github/dependabot.yml`, and
    `.github/dependency-review-config.yml` stay authored directly in `.github/`.

.PARAMETER RepoRoot
    Optional repository root. Auto-detected when omitted.

.PARAMETER SourceRoot
    Optional override path to the authoritative GitHub provider source tree.
    Defaults to `<RepoRoot>/definitions/providers/github`.

.PARAMETER SharedRoot
    Optional override path to the shared authoritative definitions tree used for
    projected instructions and templates.
    Defaults to `<RepoRoot>/definitions/shared`.

.PARAMETER OutputRoot
    Optional override path for the rendered `.github/` surface.
    Defaults to `<RepoRoot>/.github`.

.PARAMETER Verbose
    Shows detailed diagnostics.

.EXAMPLE
    pwsh -File scripts/runtime/render-github-instruction-surfaces.ps1 -RepoRoot .

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot,
    [string] $SourceRoot,
    [string] $SharedRoot,
    [string] $OutputRoot,
    [switch] $Verbose
)

$ErrorActionPreference = 'Stop'

$script:CommonBootstrapPath = Join-Path $PSScriptRoot '../common/common-bootstrap.ps1'
if (-not (Test-Path -LiteralPath $script:CommonBootstrapPath -PathType Leaf)) {
    $script:CommonBootstrapPath = Join-Path $PSScriptRoot '../../common/common-bootstrap.ps1'
}
if (-not (Test-Path -LiteralPath $script:CommonBootstrapPath -PathType Leaf)) {
    $script:CommonBootstrapPath = Join-Path $PSScriptRoot '../../shared-scripts/common/common-bootstrap.ps1'
}
if (-not (Test-Path -LiteralPath $script:CommonBootstrapPath -PathType Leaf)) {
    throw "Missing shared common bootstrap helper: $script:CommonBootstrapPath"
}
. $script:CommonBootstrapPath -CallerScriptRoot $PSScriptRoot -Helpers @('console-style', 'repository-paths')
$script:IsVerboseEnabled = [bool] $Verbose

# Resolves one GitHub authoritative source or projected output path.
function Resolve-GithubSurfacePath {
    param(
        [string] $ResolvedRepoRoot,
        [string] $RequestedPath,
        [string] $DefaultRelativePath
    )

    if ([string]::IsNullOrWhiteSpace($RequestedPath)) {
        return Join-Path $ResolvedRepoRoot $DefaultRelativePath
    }

    if ([System.IO.Path]::IsPathRooted($RequestedPath)) {
        return [System.IO.Path]::GetFullPath($RequestedPath)
    }

    return [System.IO.Path]::GetFullPath((Join-Path $ResolvedRepoRoot $RequestedPath))
}

# Mirrors one authoritative GitHub directory into its projected surface.
function Invoke-SurfaceMirror {
    param(
        [Parameter(Mandatory = $true)]
        [string] $SourcePath,
        [Parameter(Mandatory = $true)]
        [string] $DestinationPath
    )

    if (-not (Test-Path -LiteralPath $SourcePath -PathType Container)) {
        throw "Missing GitHub surface source: $SourcePath"
    }

    New-Item -ItemType Directory -Path $DestinationPath -Force | Out-Null
    Get-ChildItem -LiteralPath $DestinationPath -Force -ErrorAction SilentlyContinue | ForEach-Object {
        Remove-Item -LiteralPath $_.FullName -Recurse -Force -ErrorAction Stop
    }

    Get-ChildItem -LiteralPath $SourcePath -Force | ForEach-Object {
        Copy-Item -LiteralPath $_.FullName -Destination $DestinationPath -Recurse -Force
    }
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$resolvedSourceRoot = Resolve-GithubSurfacePath -ResolvedRepoRoot $resolvedRepoRoot -RequestedPath $SourceRoot -DefaultRelativePath 'definitions/providers/github'
$resolvedSharedRoot = Resolve-GithubSurfacePath -ResolvedRepoRoot $resolvedRepoRoot -RequestedPath $SharedRoot -DefaultRelativePath 'definitions/shared'
$resolvedOutputRoot = Resolve-GithubSurfacePath -ResolvedRepoRoot $resolvedRepoRoot -RequestedPath $OutputRoot -DefaultRelativePath '.github'

$rootSourcePath = Join-Path $resolvedSourceRoot 'root'
$managedRootFiles = @(Get-ChildItem -LiteralPath $rootSourcePath -File -Force -ErrorAction Stop)

Start-ExecutionSession `
    -Name 'render-github-instruction-surfaces' `
    -RootPath $resolvedRepoRoot `
    -Metadata ([ordered]@{
            'Source root' = $resolvedSourceRoot
            'Shared root' = $resolvedSharedRoot
            'Output root' = $resolvedOutputRoot
        }) `
    -IncludeMetadataInDefaultOutput | Out-Null

foreach ($rootFile in $managedRootFiles) {
    Copy-Item -LiteralPath $rootFile.FullName -Destination (Join-Path $resolvedOutputRoot $rootFile.Name) -Force
}

$directorySpecs = @(
    [pscustomobject]@{ Name = 'agents'; Source = Join-Path $resolvedSourceRoot 'agents'; Destination = Join-Path $resolvedOutputRoot 'agents' }
    [pscustomobject]@{ Name = 'chatmodes'; Source = Join-Path $resolvedSourceRoot 'chatmodes'; Destination = Join-Path $resolvedOutputRoot 'chatmodes' }
    [pscustomobject]@{ Name = 'instructions'; Source = Join-Path $resolvedSharedRoot 'instructions'; Destination = Join-Path $resolvedOutputRoot 'instructions' }
    [pscustomobject]@{ Name = 'prompts'; Source = Join-Path $resolvedSourceRoot 'prompts'; Destination = Join-Path $resolvedOutputRoot 'prompts' }
    [pscustomobject]@{ Name = 'hooks'; Source = Join-Path $resolvedSourceRoot 'hooks'; Destination = Join-Path $resolvedOutputRoot 'hooks' }
    [pscustomobject]@{ Name = 'templates'; Source = Join-Path $resolvedSharedRoot 'templates'; Destination = Join-Path $resolvedOutputRoot 'templates' }
)

foreach ($directorySpec in $directorySpecs) {
    Invoke-SurfaceMirror -SourcePath $directorySpec.Source -DestinationPath $directorySpec.Destination
    Write-VerboseColor ("Rendered GitHub surface: {0} -> {1}" -f $directorySpec.Source, $directorySpec.Destination) 'Gray'
}

$fileCount = @(
    foreach ($directorySpec in $directorySpecs) {
        @(Get-ChildItem -LiteralPath $directorySpec.Destination -Recurse -File -ErrorAction SilentlyContinue).Count
    }
) | Measure-Object -Sum

Write-StyledOutput ''
Write-StyledOutput 'GitHub instruction surface render summary'
Write-StyledOutput ("  Source: {0}" -f $resolvedSourceRoot)
Write-StyledOutput ("  Destination: {0}" -f $resolvedOutputRoot)
Write-StyledOutput ("  Managed root files: {0}" -f $managedRootFiles.Count)
Write-StyledOutput ("  Rendered directories: {0}" -f $directorySpecs.Count)
Write-StyledOutput ("  Rendered files: {0}" -f $fileCount.Sum)

Complete-ExecutionSession -Name 'render-github-instruction-surfaces' -Status 'passed' -Summary ([ordered]@{
        'Managed root files' = $managedRootFiles.Count
        'Rendered directories' = $directorySpecs.Count
        'Rendered files' = $fileCount.Sum
    }) | Out-Null

exit 0