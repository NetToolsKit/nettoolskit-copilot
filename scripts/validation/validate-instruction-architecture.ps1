<#
.SYNOPSIS
    Validates instruction architecture ownership and boundary rules.

.DESCRIPTION
    Ensures the repository keeps a clear separation between:
    - global core files
    - repository operating model
    - domain instructions
    - cross-cutting policy files
    - prompts
    - templates
    - Codex skills
    - runtime/orchestration assets

    The validator is intentionally conservative:
    - missing required ownership references fail
    - suspicious policy ownership markers in prompts/templates/skills warn

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.PARAMETER ManifestPath
    Relative path to the instruction ownership manifest.

.PARAMETER AgentsPath
    Relative path to AGENTS.md.

.PARAMETER GlobalInstructionsPath
    Relative path to copilot-instructions.md.

.PARAMETER RoutingCatalogPath
    Relative path to instruction-routing.catalog.yml.

.PARAMETER PromptRoot
    Relative or absolute prompt root to scan.

.PARAMETER TemplateRoot
    Relative or absolute template root to scan.

.PARAMETER SkillRoot
    Relative or absolute skill root to scan.

.PARAMETER WarningOnly
    When true (default), failures are emitted as warnings and execution exits with code 0.

.PARAMETER DetailedOutput
    Prints file-level warning details.

.EXAMPLE
    pwsh -File scripts/validation/validate-instruction-architecture.ps1

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot,
    [string] $ManifestPath = '.github/governance/instruction-ownership.manifest.json',
    [string] $AgentsPath = '.github/AGENTS.md',
    [string] $GlobalInstructionsPath = '.github/copilot-instructions.md',
    [string] $RoutingCatalogPath = '.github/instruction-routing.catalog.yml',
    [string] $PromptRoot = '.github/prompts',
    [string] $TemplateRoot = '.github/templates',
    [string] $SkillRoot = '.codex/skills',
    [bool] $WarningOnly = $true,
    [switch] $DetailedOutput,
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
$script:IsWarningOnly = [bool] $WarningOnly
$script:IsDetailedOutputEnabled = [bool] $DetailedOutput
$script:Failures = New-Object System.Collections.Generic.List[string]
$script:Warnings = New-Object System.Collections.Generic.List[string]

# Writes verbose diagnostics.
function Write-VerboseLog {
    param(
        [string] $Message
    )

    if ($script:IsVerboseEnabled) {
        Write-StyledOutput ("[VERBOSE] {0}" -f $Message)
    }
}

# Registers a validation failure.
function Add-ValidationFailure {
    param(
        [string] $Message
    )

    if ($script:IsWarningOnly) {
        $script:Warnings.Add($Message) | Out-Null
        Write-StyledOutput ("[WARN] {0}" -f $Message)
        return
    }

    $script:Failures.Add($Message) | Out-Null
    Write-StyledOutput ("[FAIL] {0}" -f $Message)
}

# Registers a validation warning.
function Add-ValidationWarning {
    param(
        [string] $Message
    )

    $script:Warnings.Add($Message) | Out-Null
    Write-StyledOutput ("[WARN] {0}" -f $Message)
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

# Resolves a path from repo root.
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

# Reads and parses JSON from file path.
function Read-JsonFile {
    param(
        [string] $Path,
        [string] $Label
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        Add-ValidationFailure ("Missing {0}: {1}" -f $Label, $Path)
        return $null
    }

    try {
        return Get-Content -Raw -LiteralPath $Path | ConvertFrom-Json -Depth 200
    }
    catch {
        Add-ValidationFailure ("Invalid JSON in {0}: {1}" -f $Label, $_.Exception.Message)
        return $null
    }
}

# Reads a required text file.
function Read-TextFile {
    param(
        [string] $Path,
        [string] $Label
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        Add-ValidationFailure ("Missing {0}: {1}" -f $Label, $Path)
        return $null
    }

    return Get-Content -Raw -LiteralPath $Path
}

# Converts input to a string array.
function ConvertTo-StringArray {
    param(
        [object] $Value
    )

    if ($null -eq $Value) {
        return @()
    }

    if ($Value -is [string]) {
        return @([string] $Value)
    }

    return @($Value | ForEach-Object { [string] $_ })
}

# Reads an optional property value from a loose object.
function Get-OptionalPropertyValue {
    param(
        [object] $InputObject,
        [string] $Name
    )

    if ($null -eq $InputObject) {
        return $null
    }

    $property = $InputObject.PSObject.Properties[$Name]
    if ($null -eq $property) {
        return $null
    }

    return $property.Value
}

# Converts an absolute path to a normalized repository-relative path.
function Get-NormalizedRelativePath {
    param(
        [string] $Root,
        [string] $Path
    )

    return [System.IO.Path]::GetRelativePath($Root, $Path).Replace('\', '/')
}

# Matches a repository-relative path against a manifest pattern.
function Test-PathPatternMatch {
    param(
        [string] $RelativePath,
        [string] $Pattern
    )

    return $RelativePath.ToLowerInvariant() -like $Pattern.Replace('\', '/').ToLowerInvariant()
}

# Expands manifest patterns into the matching file set for one layer.
function Get-MatchingFilesForLayer {
    param(
        [string] $Root,
        [object] $Layer
    )

    $allFiles = Get-ChildItem -Path $Root -Recurse -File
    $matches = New-Object System.Collections.Generic.List[string]
    $pathPatterns = @(ConvertTo-StringArray -Value (Get-OptionalPropertyValue -InputObject $Layer -Name 'pathPatterns'))
    $excludePatterns = @(ConvertTo-StringArray -Value (Get-OptionalPropertyValue -InputObject $Layer -Name 'excludePatterns'))

    foreach ($file in $allFiles) {
        $relativePath = Get-NormalizedRelativePath -Root $Root -Path $file.FullName
        $isMatch = $false

        foreach ($pattern in $pathPatterns) {
            if (Test-PathPatternMatch -RelativePath $relativePath -Pattern $pattern) {
                $isMatch = $true
                break
            }
        }

        if (-not $isMatch) {
            continue
        }

        $isExcluded = $false
        foreach ($excludePattern in $excludePatterns) {
            if (Test-PathPatternMatch -RelativePath $relativePath -Pattern $excludePattern) {
                $isExcluded = $true
                break
            }
        }

        if (-not $isExcluded) {
            $matches.Add($file.FullName) | Out-Null
        }
    }

    return @($matches | Select-Object -Unique)
}

# Validates manifest version, required layers, and layer shape.
function Test-ManifestStructure {
    param(
        [object] $Manifest
    )

    if ($null -eq $Manifest) {
        return
    }

    if ([int] $Manifest.version -lt 1) {
        Add-ValidationFailure 'Instruction ownership manifest version must be >= 1.'
    }

    $layers = @($Manifest.layers)
    if ($layers.Count -eq 0) {
        Add-ValidationFailure 'Instruction ownership manifest must define at least one layer.'
        return
    }

    $requiredLayerIds = @(
        'global-core',
        'repository-operating-model',
        'cross-cutting-policies',
        'domain-instructions',
        'prompts',
        'templates',
        'codex-skills',
        'orchestration',
        'runtime-projection'
    )

    $seenIds = @{}
    foreach ($layer in $layers) {
        $layerId = [string] $layer.id
        if ([string]::IsNullOrWhiteSpace($layerId)) {
            Add-ValidationFailure 'Instruction ownership manifest contains a layer without id.'
            continue
        }

        if ($seenIds.ContainsKey($layerId)) {
            Add-ValidationFailure ("Instruction ownership manifest contains duplicate layer id: {0}" -f $layerId)
            continue
        }

        $seenIds[$layerId] = $true

        $patterns = @(ConvertTo-StringArray -Value (Get-OptionalPropertyValue -InputObject $layer -Name 'pathPatterns'))
        if ($patterns.Count -eq 0) {
            Add-ValidationFailure ("Layer '{0}' must define at least one path pattern." -f $layerId)
        }
    }

    foreach ($requiredLayerId in $requiredLayerIds) {
        if (-not $seenIds.ContainsKey($requiredLayerId)) {
            Add-ValidationFailure ("Instruction ownership manifest is missing required layer '{0}'." -f $requiredLayerId)
        }
    }
}

# Detects exact file ownership overlap across architecture layers.
function Test-LayerOverlap {
    param(
        [string] $Root,
        [object[]] $Layers
    )

    $ownership = @{}

    foreach ($layer in $Layers) {
        $layerId = [string] $layer.id
        $files = @(Get-MatchingFilesForLayer -Root $Root -Layer $layer)
        foreach ($file in $files) {
            $relativePath = Get-NormalizedRelativePath -Root $Root -Path $file
            if ($ownership.ContainsKey($relativePath)) {
                Add-ValidationFailure ("File is claimed by multiple architecture layers: {0} -> {1}, {2}" -f $relativePath, $ownership[$relativePath], $layerId)
            }
            else {
                $ownership[$relativePath] = $layerId
            }
        }
    }
}

# Validates that the global core references the required centralized files.
function Test-GlobalCoreReferences {
    param(
        [string] $AgentsContent,
        [string] $GlobalContent,
        [string] $RoutingContent
    )

    $requiredPatterns = @(
        'instructions/repository-operating-model\.instructions\.md',
        'instructions/authoritative-sources\.instructions\.md'
    )

    foreach ($pattern in $requiredPatterns) {
        if ($AgentsContent -notmatch $pattern) {
            Add-ValidationFailure ("AGENTS.md is missing required architecture reference: {0}" -f $pattern)
        }

        if ($GlobalContent -notmatch $pattern) {
            Add-ValidationFailure ("copilot-instructions.md is missing required architecture reference: {0}" -f $pattern)
        }

        if ($RoutingContent -notmatch $pattern) {
            Add-ValidationFailure ("instruction-routing.catalog.yml is missing required architecture reference: {0}" -f $pattern)
        }
    }
}

# Scans prompts, templates, or skills for ownership markers that should stay elsewhere.
function Test-OwnershipMarkers {
    param(
        [string] $Root,
        [string] $TargetRoot,
        [string[]] $ForbiddenMarkers,
        [string] $Label
    )

    if (-not (Test-Path -LiteralPath $TargetRoot)) {
        Add-ValidationFailure ("Missing {0} root: {1}" -f $Label, $TargetRoot)
        return
    }

    $files = Get-ChildItem -Path $TargetRoot -Recurse -File
    foreach ($file in $files) {
        $content = Get-Content -Raw -LiteralPath $file.FullName
        $relativePath = Get-NormalizedRelativePath -Root $Root -Path $file.FullName

        foreach ($marker in $ForbiddenMarkers) {
            if ($content -match [regex]::Escape($marker)) {
                Add-ValidationWarning ("{0} may be owning policy instead of behavior: {1} -> marker '{2}'" -f $Label, $relativePath, $marker)
            }
        }
    }
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
Set-Location -Path $resolvedRepoRoot

$resolvedManifestPath = Resolve-RepoPath -Root $resolvedRepoRoot -Path $ManifestPath
$resolvedAgentsPath = Resolve-RepoPath -Root $resolvedRepoRoot -Path $AgentsPath
$resolvedGlobalInstructionsPath = Resolve-RepoPath -Root $resolvedRepoRoot -Path $GlobalInstructionsPath
$resolvedRoutingCatalogPath = Resolve-RepoPath -Root $resolvedRepoRoot -Path $RoutingCatalogPath
$resolvedPromptRoot = Resolve-RepoPath -Root $resolvedRepoRoot -Path $PromptRoot
$resolvedTemplateRoot = Resolve-RepoPath -Root $resolvedRepoRoot -Path $TemplateRoot
$resolvedSkillRoot = Resolve-RepoPath -Root $resolvedRepoRoot -Path $SkillRoot

$manifest = Read-JsonFile -Path $resolvedManifestPath -Label 'instruction ownership manifest'
Test-ManifestStructure -Manifest $manifest

if ($null -ne $manifest) {
    Test-LayerOverlap -Root $resolvedRepoRoot -Layers @($manifest.layers)
}

$agentsContent = Read-TextFile -Path $resolvedAgentsPath -Label 'AGENTS.md'
$globalInstructionsContent = Read-TextFile -Path $resolvedGlobalInstructionsPath -Label 'copilot-instructions.md'
$routingCatalogContent = Read-TextFile -Path $resolvedRoutingCatalogPath -Label 'instruction routing catalog'

if ($null -ne $agentsContent -and $null -ne $globalInstructionsContent -and $null -ne $routingCatalogContent) {
    Test-GlobalCoreReferences `
        -AgentsContent $agentsContent `
        -GlobalContent $globalInstructionsContent `
        -RoutingContent $routingCatalogContent
}

if ($null -ne $manifest) {
    foreach ($layer in @($manifest.layers)) {
        $forbiddenMarkers = @(ConvertTo-StringArray -Value (Get-OptionalPropertyValue -InputObject $layer -Name 'forbiddenOwnershipMarkers'))
        if ($forbiddenMarkers.Count -eq 0) {
            continue
        }

        switch ([string] $layer.id) {
            'prompts' {
                Test-OwnershipMarkers -Root $resolvedRepoRoot -TargetRoot $resolvedPromptRoot -ForbiddenMarkers $forbiddenMarkers -Label 'Prompt file'
            }
            'templates' {
                Test-OwnershipMarkers -Root $resolvedRepoRoot -TargetRoot $resolvedTemplateRoot -ForbiddenMarkers $forbiddenMarkers -Label 'Template file'
            }
            'codex-skills' {
                Test-OwnershipMarkers -Root $resolvedRepoRoot -TargetRoot $resolvedSkillRoot -ForbiddenMarkers $forbiddenMarkers -Label 'Skill file'
            }
        }
    }
}

Write-Host ''
Write-Host 'Instruction architecture validation summary'
Write-Host ("  Warning-only mode: {0}" -f $script:IsWarningOnly)
Write-Host ("  Manifest path: {0}" -f $ManifestPath)
Write-Host ("  Warnings: {0}" -f $script:Warnings.Count)
Write-Host ("  Failures: {0}" -f $script:Failures.Count)

if ($script:Failures.Count -gt 0 -and -not $script:IsWarningOnly) {
    exit 1
}

exit 0