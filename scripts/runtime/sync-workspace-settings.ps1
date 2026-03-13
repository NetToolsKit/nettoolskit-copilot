<#
.SYNOPSIS
    Generates or synchronizes efficient VS Code workspace settings from the shared template.

.DESCRIPTION
    VS Code does not provide native inheritance from an external `.vscode/settings.json`
    into `.code-workspace` files. This script implements repository-managed pseudo-inheritance
    by combining:
    - `.vscode/base.code-workspace`
    - `.vscode/settings.tamplate.jsonc`
    - `.github/governance/workspace-efficiency.baseline.json`

    The script keeps the workspace file lean:
    - preserves existing `folders` and workspace-specific top-level properties
    - merges shared recommendations from the base workspace `extensions` block
    - carries missing top-level defaults from the base workspace when creating/updating files
    - replaces only the `settings` object
    - copies required watcher/search excludes from the shared template
    - applies approved local throttles for multi-window Codex/Copilot usage

    When the workspace file does not exist, provide `-FolderPath` to create it.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.PARAMETER WorkspacePath
    Target `.code-workspace` file path. Can be relative to the repository root or absolute.

.PARAMETER FolderPath
    Folder paths used when creating a new workspace file. Ignored when the workspace already exists.

.PARAMETER BaselinePath
    Optional workspace-efficiency baseline path relative to repository root.

.PARAMETER SettingsTemplatePath
    Optional VS Code settings template path relative to repository root.

.PARAMETER BaseWorkspacePath
    Optional base workspace path relative to repository root.

.PARAMETER Verbose
    Shows detailed diagnostics.

.EXAMPLE
    pwsh -File scripts/runtime/sync-workspace-settings.ps1 -WorkspacePath .\workspaces\api.code-workspace -FolderPath src\Api

.EXAMPLE
    pwsh -File scripts/runtime/sync-workspace-settings.ps1 -WorkspacePath C:\Users\me\Projects\app.code-workspace

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot,
    [string] $WorkspacePath,
    [string[]] $FolderPath,
    [string] $BaselinePath = '.github/governance/workspace-efficiency.baseline.json',
    [string] $SettingsTemplatePath = '.vscode/settings.tamplate.jsonc',
    [string] $BaseWorkspacePath = '.vscode/base.code-workspace',
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

# Resolves one path from repository root.
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

# Reads one JSON or JSONC file.
function Read-JsonFile {
    param(
        [string] $Path,
        [string] $Label
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing ${Label}: $Path"
    }

    try {
        return Get-Content -Raw -LiteralPath $Path | ConvertFrom-Json -Depth 200
    }
    catch {
        throw "Invalid JSON/JSONC in ${Label}: $($Path) :: $($_.Exception.Message)"
    }
}

# Converts JSON-like objects to a property map keyed by property name.
function Convert-ToPropertyMap {
    param(
        [object] $Value
    )

    $result = [ordered]@{}
    if ($null -eq $Value) {
        return $result
    }

    if ($Value -is [System.Collections.IDictionary]) {
        foreach ($key in $Value.Keys) {
            $result[[string] $key] = $Value[$key]
        }

        return $result
    }

    foreach ($property in $Value.PSObject.Properties) {
        $result[[string] $property.Name] = $property.Value
    }

    return $result
}

# Gets a direct property value from a JSON-like object.
function Get-JsonPropertyValue {
    param(
        [object] $InputObject,
        [string] $PropertyName
    )

    if ($null -eq $InputObject) {
        return $null
    }

    if ($InputObject -is [System.Collections.IDictionary]) {
        if ($InputObject.Contains($PropertyName)) {
            return $InputObject[$PropertyName]
        }

        return $null
    }

    $property = $InputObject.PSObject.Properties[$PropertyName]
    if ($null -eq $property) {
        return $null
    }

    return $property.Value
}

# Merges string arrays while preserving first-seen order and uniqueness.
function Merge-UniqueStringValues {
    param(
        [object[]] $PrimaryValues,
        [object[]] $SecondaryValues
    )

    $items = New-Object System.Collections.Generic.List[string]
    foreach ($value in @($PrimaryValues) + @($SecondaryValues)) {
        $text = [string] $value
        if ([string]::IsNullOrWhiteSpace($text)) {
            continue
        }

        if (-not $items.Contains($text)) {
            $items.Add($text) | Out-Null
        }
    }

    return $items.ToArray()
}

# Merges workspace extension recommendations with the shared base workspace.
function Merge-WorkspaceExtensions {
    param(
        [object] $ExistingExtensions,
        [object] $BaseExtensions
    )

    $existingRecommendations = @((Get-JsonPropertyValue -InputObject $ExistingExtensions -PropertyName 'recommendations'))
    $baseRecommendations = @((Get-JsonPropertyValue -InputObject $BaseExtensions -PropertyName 'recommendations'))
    $existingUnwanted = @((Get-JsonPropertyValue -InputObject $ExistingExtensions -PropertyName 'unwantedRecommendations'))
    $baseUnwanted = @((Get-JsonPropertyValue -InputObject $BaseExtensions -PropertyName 'unwantedRecommendations'))

    [string[]] $mergedRecommendations = Merge-UniqueStringValues -PrimaryValues $existingRecommendations -SecondaryValues $baseRecommendations
    [string[]] $mergedUnwanted = Merge-UniqueStringValues -PrimaryValues $existingUnwanted -SecondaryValues $baseUnwanted

    if (@($mergedRecommendations).Count -eq 0 -and @($mergedUnwanted).Count -eq 0) {
        return $null
    }

    $extensions = [ordered]@{}
    if (@($mergedRecommendations).Count -gt 0) {
        $extensions['recommendations'] = $mergedRecommendations
    }

    if (@($mergedUnwanted).Count -gt 0) {
        $extensions['unwantedRecommendations'] = $mergedUnwanted
    }

    return [pscustomobject] $extensions
}

# Builds the allowed workspace settings subset from template and baseline.
function New-WorkspaceSettingsObject {
    param(
        [object] $SettingsTemplate,
        [object] $WorkspaceBaseline
    )

    $templateWatcherExclude = Convert-ToPropertyMap -Value (Get-JsonPropertyValue -InputObject $SettingsTemplate -PropertyName 'files.watcherExclude')
    $templateSearchExclude = Convert-ToPropertyMap -Value (Get-JsonPropertyValue -InputObject $SettingsTemplate -PropertyName 'search.exclude')
    $requiredSettings = Convert-ToPropertyMap -Value (Get-JsonPropertyValue -InputObject $WorkspaceBaseline -PropertyName 'requiredSettings')
    $recommendedSettings = Convert-ToPropertyMap -Value (Get-JsonPropertyValue -InputObject $WorkspaceBaseline -PropertyName 'recommendedSettings')
    $recommendedBounds = Convert-ToPropertyMap -Value (Get-JsonPropertyValue -InputObject $WorkspaceBaseline -PropertyName 'recommendedNumericUpperBounds')

    $watcherExclude = [ordered]@{}
    foreach ($key in @((Get-JsonPropertyValue -InputObject $requiredSettings['files.watcherExclude'] -PropertyName 'requiredKeys'))) {
        $keyName = [string] $key
        $watcherExclude[$keyName] = if ($templateWatcherExclude.Contains($keyName)) { [bool] $templateWatcherExclude[$keyName] } else { $true }
    }

    $searchExclude = [ordered]@{}
    foreach ($key in @((Get-JsonPropertyValue -InputObject $requiredSettings['search.exclude'] -PropertyName 'requiredKeys'))) {
        $keyName = [string] $key
        $searchExclude[$keyName] = if ($templateSearchExclude.Contains($keyName)) { [bool] $templateSearchExclude[$keyName] } else { $true }
    }

    return [pscustomobject] ([ordered]@{
        'git.autofetch' = [bool] $requiredSettings['git.autofetch']
        'git.openRepositoryInParentFolders' = 'never'
        'git.autorefresh' = [bool] $recommendedSettings['git.autorefresh']
        'extensions.autoUpdate' = [bool] $recommendedSettings['extensions.autoUpdate']
        'github.copilot.nextEditSuggestions.enabled' = [bool] $recommendedSettings['github.copilot.nextEditSuggestions.enabled']
        'scm.repositories.visible' = [int] $recommendedBounds['scm.repositories.visible']
        'chat.agent.maxRequests' = [int] $recommendedBounds['chat.agent.maxRequests']
        'files.watcherExclude' = [pscustomobject] $watcherExclude
        'search.exclude' = [pscustomobject] $searchExclude
    })
}

# Creates a new workspace document with supplied folders.
function New-WorkspaceDocument {
    param(
        [object] $BaseWorkspaceDocument,
        [string[]] $FolderPaths,
        [object] $SettingsObject
    )

    $folders = @()
    foreach ($item in @($FolderPaths)) {
        if ([string]::IsNullOrWhiteSpace([string] $item)) {
            continue
        }

        $folders += [pscustomobject] ([ordered]@{
            path = [string] $item
        })
    }

    $document = [ordered]@{
        folders = $folders
    }

    if ($null -ne $BaseWorkspaceDocument) {
        $baseExtensions = Merge-WorkspaceExtensions -ExistingExtensions $null -BaseExtensions (Get-JsonPropertyValue -InputObject $BaseWorkspaceDocument -PropertyName 'extensions')
        if ($null -ne $baseExtensions) {
            $document['extensions'] = $baseExtensions
        }

        foreach ($property in $BaseWorkspaceDocument.PSObject.Properties) {
            if ($property.Name -in @('folders', 'settings', 'extensions')) {
                continue
            }

            $document[$property.Name] = $property.Value
        }
    }

    $document['settings'] = $SettingsObject
    return [pscustomobject] $document
}

# Replaces the workspace settings while preserving existing top-level properties.
function Set-WorkspaceSettings {
    param(
        [object] $WorkspaceDocument,
        [object] $BaseWorkspaceDocument,
        [object] $SettingsObject
    )

    $ordered = [ordered]@{}
    $settingsInserted = $false
    $foldersSeen = $false
    $baseExtensions = if ($null -ne $BaseWorkspaceDocument) { Get-JsonPropertyValue -InputObject $BaseWorkspaceDocument -PropertyName 'extensions' } else { $null }
    $extensionsMerged = $false

    foreach ($property in $WorkspaceDocument.PSObject.Properties) {
        if ($property.Name -eq 'settings') {
            $ordered['settings'] = $SettingsObject
            $settingsInserted = $true
            continue
        }

        if ($property.Name -eq 'extensions') {
            $mergedExtensions = Merge-WorkspaceExtensions -ExistingExtensions $property.Value -BaseExtensions $baseExtensions
            if ($null -ne $mergedExtensions) {
                $ordered['extensions'] = $mergedExtensions
            }
            $extensionsMerged = $true
            continue
        }

        $ordered[$property.Name] = $property.Value
        if ($property.Name -eq 'folders') {
            $foldersSeen = $true
            if (-not $settingsInserted) {
                $ordered['settings'] = $SettingsObject
                $settingsInserted = $true
            }
        }
    }

    if (-not $foldersSeen) {
        $ordered['folders'] = @()
        if (-not $settingsInserted) {
            $ordered['settings'] = $SettingsObject
            $settingsInserted = $true
        }
    }

    if (-not $settingsInserted) {
        $ordered['settings'] = $SettingsObject
    }

    if (-not $extensionsMerged) {
        $mergedExtensions = Merge-WorkspaceExtensions -ExistingExtensions $null -BaseExtensions $baseExtensions
        if ($null -ne $mergedExtensions) {
            $ordered['extensions'] = $mergedExtensions
        }
    }

    if ($null -ne $BaseWorkspaceDocument) {
        foreach ($property in $BaseWorkspaceDocument.PSObject.Properties) {
            if ($property.Name -in @('folders', 'settings', 'extensions')) {
                continue
            }

            if (-not $ordered.Contains($property.Name)) {
                $ordered[$property.Name] = $property.Value
            }
        }
    }

    return [pscustomobject] $ordered
}

# Writes one workspace document as JSON.
function Write-WorkspaceDocument {
    param(
        [string] $Path,
        [object] $Document
    )

    $parent = Split-Path -Path $Path -Parent
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }

    $json = $Document | ConvertTo-Json -Depth 100
    Set-Content -LiteralPath $Path -Value $json
}

if ([string]::IsNullOrWhiteSpace($WorkspacePath)) {
    throw 'WorkspacePath is required.'
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
Set-Location -Path $resolvedRepoRoot

$resolvedWorkspacePath = Resolve-RepoPath -Root $resolvedRepoRoot -Path $WorkspacePath
$resolvedBaselinePath = Resolve-RepoPath -Root $resolvedRepoRoot -Path $BaselinePath
$resolvedSettingsTemplatePath = Resolve-RepoPath -Root $resolvedRepoRoot -Path $SettingsTemplatePath
$resolvedBaseWorkspacePath = Resolve-RepoPath -Root $resolvedRepoRoot -Path $BaseWorkspacePath

$workspaceBaseline = Read-JsonFile -Path $resolvedBaselinePath -Label 'workspace-efficiency baseline'
$settingsTemplate = Read-JsonFile -Path $resolvedSettingsTemplatePath -Label 'VS Code settings template'
$baseWorkspaceDocument = Read-JsonFile -Path $resolvedBaseWorkspacePath -Label 'base workspace'
$workspaceSettings = New-WorkspaceSettingsObject -SettingsTemplate $settingsTemplate -WorkspaceBaseline $workspaceBaseline

$workspaceExists = Test-Path -LiteralPath $resolvedWorkspacePath -PathType Leaf
if ($workspaceExists) {
    $workspaceDocument = Read-JsonFile -Path $resolvedWorkspacePath -Label 'workspace file'
    $updatedDocument = Set-WorkspaceSettings -WorkspaceDocument $workspaceDocument -BaseWorkspaceDocument $baseWorkspaceDocument -SettingsObject $workspaceSettings
    Write-WorkspaceDocument -Path $resolvedWorkspacePath -Document $updatedDocument
    Write-StyledOutput ("[OK] Workspace settings synchronized: {0}" -f $resolvedWorkspacePath)
    Write-StyledOutput ("Folders preserved: {0}" -f @($updatedDocument.folders).Count)
}
else {
    if (@($FolderPath).Count -eq 0) {
        throw 'Workspace file does not exist. Provide -FolderPath to create a new workspace.'
    }

    $newDocument = New-WorkspaceDocument -BaseWorkspaceDocument $baseWorkspaceDocument -FolderPaths $FolderPath -SettingsObject $workspaceSettings
    Write-WorkspaceDocument -Path $resolvedWorkspacePath -Document $newDocument
    Write-StyledOutput ("[OK] Workspace created with synchronized settings: {0}" -f $resolvedWorkspacePath)
    Write-StyledOutput ("Folders created: {0}" -f @($newDocument.folders).Count)
}

Write-StyledOutput ''
Write-StyledOutput 'Workspace settings sync summary'
Write-StyledOutput ("  Repo root: {0}" -f $resolvedRepoRoot)
Write-StyledOutput ("  Workspace path: {0}" -f $resolvedWorkspacePath)
Write-StyledOutput ("  Baseline: {0}" -f $resolvedBaselinePath)
Write-StyledOutput ("  Template: {0}" -f $resolvedSettingsTemplatePath)
Write-StyledOutput ("  Base workspace: {0}" -f $resolvedBaseWorkspacePath)

exit 0