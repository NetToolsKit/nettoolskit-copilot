<#
.SYNOPSIS
    Shared helpers for the provider surface projection catalog.

.DESCRIPTION
    Resolves the canonical provider-surface projection catalog used to describe
    repository-owned authored surfaces, generated exceptions, and the renderer
    entrypoints that project them into `.github/`, `.codex/`, `.claude/`, and
    `.vscode/`. Renderers may stay script-backed or dispatch through the native
    `ntk runtime render-provider-surfaces` contract.

.EXAMPLE
    . .\scripts\common\provider-surface-catalog.ps1
    $catalog = Read-ProviderSurfaceProjectionCatalog -RepoRoot (Get-Location)

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Returns one direct property value or a fallback default.
function Get-ProviderSurfaceCatalogOptionalValue {
    param(
        [object] $Object,
        [string] $PropertyName,
        [object] $DefaultValue = $null
    )

    if ($null -eq $Object -or [string]::IsNullOrWhiteSpace($PropertyName)) {
        return $DefaultValue
    }

    if ($Object -is [System.Collections.IDictionary]) {
        if ($Object.Contains($PropertyName)) {
            return $Object[$PropertyName]
        }

        return $DefaultValue
    }

    $property = $Object.PSObject.Properties[$PropertyName]
    if ($null -eq $property) {
        return $DefaultValue
    }

    return $property.Value
}

# Resolves the canonical provider-surface projection catalog path.
function Resolve-ProviderSurfaceProjectionCatalogPath {
    param(
        [Parameter(Mandatory = $true)]
        [string] $RepoRoot,
        [string] $CatalogPath
    )

    if ([string]::IsNullOrWhiteSpace($CatalogPath)) {
        return Join-Path $RepoRoot '.github\governance\provider-surface-projection.catalog.json'
    }

    if ([System.IO.Path]::IsPathRooted($CatalogPath)) {
        return [System.IO.Path]::GetFullPath($CatalogPath)
    }

    return [System.IO.Path]::GetFullPath((Join-Path $RepoRoot $CatalogPath))
}

# Reads the canonical provider-surface projection catalog.
function Read-ProviderSurfaceProjectionCatalog {
    param(
        [Parameter(Mandatory = $true)]
        [string] $RepoRoot,
        [string] $CatalogPath
    )

    $resolvedCatalogPath = Resolve-ProviderSurfaceProjectionCatalogPath -RepoRoot $RepoRoot -CatalogPath $CatalogPath
    if (-not (Test-Path -LiteralPath $resolvedCatalogPath -PathType Leaf)) {
        throw "Provider surface projection catalog not found: $resolvedCatalogPath"
    }

    try {
        $catalog = Get-Content -Raw -LiteralPath $resolvedCatalogPath | ConvertFrom-Json -Depth 200
    }
    catch {
        throw ("Invalid provider surface projection catalog '{0}': {1}" -f $resolvedCatalogPath, $_.Exception.Message)
    }

    if ($null -eq $catalog.renderers -or @($catalog.renderers).Count -eq 0) {
        throw "Provider surface projection catalog has no renderers: $resolvedCatalogPath"
    }

    if ($null -eq $catalog.surfaces -or @($catalog.surfaces).Count -eq 0) {
        throw "Provider surface projection catalog has no surfaces: $resolvedCatalogPath"
    }

    return [pscustomobject]@{
        Path = $resolvedCatalogPath
        Catalog = $catalog
    }
}

# Resolves one catalog-authored relative path against the repo root.
function Resolve-ProviderSurfaceCatalogPathValue {
    param(
        [Parameter(Mandatory = $true)]
        [string] $RepoRoot,
        [Parameter(Mandatory = $true)]
        [string] $PathValue
    )

    if ([System.IO.Path]::IsPathRooted($PathValue)) {
        return [System.IO.Path]::GetFullPath($PathValue)
    }

    return [System.IO.Path]::GetFullPath((Join-Path $RepoRoot $PathValue))
}

# Builds a renderer lookup map keyed by renderer id.
function Get-ProviderSurfaceCatalogRendererMap {
    param(
        [Parameter(Mandatory = $true)]
        [object] $Catalog
    )

    $map = @{}
    foreach ($renderer in @($Catalog.renderers)) {
        $rendererId = [string] (Get-ProviderSurfaceCatalogOptionalValue -Object $renderer -PropertyName 'id' -DefaultValue '')
        if ([string]::IsNullOrWhiteSpace($rendererId)) {
            continue
        }

        $map[$rendererId] = $renderer
    }

    return $map
}

# Evaluates one bootstrap runtime condition from the projection catalog.
function Test-ProviderSurfaceBootstrapCondition {
    param(
        [Parameter(Mandatory = $true)]
        [string] $Condition,
        [bool] $EnableCodexRuntime = $false,
        [bool] $EnableClaudeRuntime = $false
    )

    switch ($Condition) {
        'always' { return $true }
        'codex' { return $EnableCodexRuntime }
        'claude' { return $EnableClaudeRuntime }
        'never' { return $false }
        default { return $false }
    }
}

# Converts one renderer default-argument object into a splattable hashtable.
function Convert-ProviderSurfaceArgumentMap {
    param(
        [object] $ArgumentObject
    )

    $argumentMap = @{}
    if ($null -eq $ArgumentObject) {
        return $argumentMap
    }

    foreach ($property in $ArgumentObject.PSObject.Properties) {
        $value = $property.Value
        if ($value -is [System.Array]) {
            $argumentMap[$property.Name] = @($value | ForEach-Object { $_ })
            continue
        }

        $argumentMap[$property.Name] = $value
    }

    return $argumentMap
}

# Resolves one native renderer metadata object from the catalog.
function Get-ProviderSurfaceCatalogNativeCommand {
    param(
        [object] $Renderer
    )

    return Get-ProviderSurfaceCatalogOptionalValue -Object $Renderer -PropertyName 'nativeCommand'
}

# Resolves the best available `ntk` runtime binary for native provider-surface renderer dispatch.
function Resolve-ProviderSurfaceNativeRuntimeBinaryPath {
    param(
        [Parameter(Mandatory = $true)]
        [string] $RepoRoot
    )

    if (-not (Get-Command -Name Resolve-NtkRuntimeBinaryPath -ErrorAction SilentlyContinue)) {
        $runtimePathsHelperPath = Join-Path $PSScriptRoot 'runtime-paths.ps1'
        if (-not (Test-Path -LiteralPath $runtimePathsHelperPath -PathType Leaf)) {
            throw ("Missing runtime paths helper for native provider-surface dispatch: {0}" -f $runtimePathsHelperPath)
        }

        . $runtimePathsHelperPath
    }

    $runtimeBinaryPath = Resolve-NtkRuntimeBinaryPath -ResolvedRepoRoot $RepoRoot -RuntimePreference github
    if (-not (Test-Path -LiteralPath $runtimeBinaryPath -PathType Leaf)) {
        throw ("Managed runtime binary not found for provider-surface dispatch: {0}" -f $runtimeBinaryPath)
    }

    return [System.IO.Path]::GetFullPath($runtimeBinaryPath)
}

# Invokes one native provider-surface renderer through the managed runtime binary.
function Invoke-ProviderSurfaceNativeRenderer {
    param(
        [Parameter(Mandatory = $true)]
        [string] $RepoRoot,
        [Parameter(Mandatory = $true)]
        [string] $RendererId,
        [Parameter(Mandatory = $true)]
        [object] $NativeCommand,
        [string] $CatalogPath,
        [string] $ConsumerName = 'direct',
        [bool] $EnableCodexRuntime = $false,
        [bool] $EnableClaudeRuntime = $false
    )

    $nativeKind = [string] (Get-ProviderSurfaceCatalogOptionalValue -Object $NativeCommand -PropertyName 'kind' -DefaultValue '')
    switch ($nativeKind) {
        'ntk-runtime-render-provider-surfaces' {
            $runtimeBinaryPath = Resolve-ProviderSurfaceNativeRuntimeBinaryPath -RepoRoot $RepoRoot
            $argumentList = @(
                'runtime',
                'render-provider-surfaces',
                '--repo-root',
                $RepoRoot,
                '--renderer-id',
                $RendererId,
                '--consumer-name',
                $ConsumerName
            )

            if (-not [string]::IsNullOrWhiteSpace($CatalogPath)) {
                $argumentList += @('--catalog-path', $CatalogPath)
            }

            if ($EnableCodexRuntime) {
                $argumentList += '--enable-codex-runtime'
            }

            if ($EnableClaudeRuntime) {
                $argumentList += '--enable-claude-runtime'
            }

            & $runtimeBinaryPath @argumentList | Out-Null
            $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
            if ($exitCode -ne 0) {
                throw ("Native provider surface renderer '{0}' failed. ExitCode={1}" -f $RendererId, $exitCode)
            }

            return [pscustomobject]@{
                DispatchKind = 'native-runtime'
                RuntimeBinaryPath = $runtimeBinaryPath
                ScriptPath = $null
                ExitCode = $exitCode
            }
        }
        default {
            throw ("Unsupported provider surface native command kind '{0}' for renderer '{1}'." -f $nativeKind, $RendererId)
        }
    }
}

# Selects the renderers enabled for one consumer view of the catalog.
function Get-ProviderSurfaceProjectionRenderers {
    param(
        [Parameter(Mandatory = $true)]
        [object] $Catalog,
        [string[]] $RendererIds = @(),
        [string] $ConsumerName = 'direct',
        [bool] $EnableCodexRuntime = $false,
        [bool] $EnableClaudeRuntime = $false
    )

    $requestedRendererIds = @($RendererIds | Where-Object { -not [string]::IsNullOrWhiteSpace($_) } | Select-Object -Unique)
    $selectedRenderers = New-Object System.Collections.Generic.List[object]

    foreach ($renderer in @($Catalog.renderers)) {
        $rendererId = [string] (Get-ProviderSurfaceCatalogOptionalValue -Object $renderer -PropertyName 'id' -DefaultValue '')
        if ($requestedRendererIds.Count -gt 0 -and -not ($requestedRendererIds -contains $rendererId)) {
            continue
        }

        $consumers = Get-ProviderSurfaceCatalogOptionalValue -Object $renderer -PropertyName 'consumers'
        $consumer = Get-ProviderSurfaceCatalogOptionalValue -Object $consumers -PropertyName $ConsumerName
        if ($null -eq $consumer) {
            continue
        }

        $enabled = [bool] (Get-ProviderSurfaceCatalogOptionalValue -Object $consumer -PropertyName 'enabled' -DefaultValue $false)
        if (-not $enabled) {
            continue
        }

        if ($ConsumerName -eq 'bootstrap') {
            $condition = [string] (Get-ProviderSurfaceCatalogOptionalValue -Object $consumer -PropertyName 'condition' -DefaultValue 'always')
            if (-not (Test-ProviderSurfaceBootstrapCondition -Condition $condition -EnableCodexRuntime:$EnableCodexRuntime -EnableClaudeRuntime:$EnableClaudeRuntime)) {
                continue
            }
        }

        $selectedRenderers.Add($renderer) | Out-Null
    }

    return @(
        $selectedRenderers.ToArray() |
            Sort-Object {
                $consumer = Get-ProviderSurfaceCatalogOptionalValue -Object (Get-ProviderSurfaceCatalogOptionalValue -Object $_ -PropertyName 'consumers') -PropertyName $ConsumerName
                [int] (Get-ProviderSurfaceCatalogOptionalValue -Object $consumer -PropertyName 'order' -DefaultValue 0)
            }, {
                [string] (Get-ProviderSurfaceCatalogOptionalValue -Object $_ -PropertyName 'id' -DefaultValue '')
            }
    )
}

# Returns projected surface entries with optional authority/validation filtering.
function Get-ProviderSurfaceProjectionSurfaces {
    param(
        [Parameter(Mandatory = $true)]
        [object] $Catalog,
        [bool] $DefinitionsOnly = $false,
        [bool] $ValidationEnabledOnly = $false
    )

    $surfaces = foreach ($surface in @($Catalog.surfaces)) {
        $authority = [string] (Get-ProviderSurfaceCatalogOptionalValue -Object $surface -PropertyName 'authority' -DefaultValue '')
        if ($DefinitionsOnly -and $authority -ne 'definitions') {
            continue
        }

        if ($ValidationEnabledOnly) {
            $validation = Get-ProviderSurfaceCatalogOptionalValue -Object $surface -PropertyName 'validation'
            if (-not [bool] (Get-ProviderSurfaceCatalogOptionalValue -Object $validation -PropertyName 'enabled' -DefaultValue $false)) {
                continue
            }
        }

        $surface
    }

    return @($surfaces)
}

# Invokes the selected projection renderers from the canonical catalog.
function Invoke-ProviderSurfaceProjectionRenderers {
    param(
        [Parameter(Mandatory = $true)]
        [string] $RepoRoot,
        [Parameter(Mandatory = $true)]
        [object] $Catalog,
        [string] $CatalogPath,
        [string[]] $RendererIds = @(),
        [string] $ConsumerName = 'direct',
        [bool] $EnableCodexRuntime = $false,
        [bool] $EnableClaudeRuntime = $false,
        [bool] $RenderVerbose = $false
    )

    $renderers = Get-ProviderSurfaceProjectionRenderers -Catalog $Catalog -RendererIds $RendererIds -ConsumerName $ConsumerName -EnableCodexRuntime:$EnableCodexRuntime -EnableClaudeRuntime:$EnableClaudeRuntime
    $results = New-Object System.Collections.Generic.List[object]

    foreach ($renderer in @($renderers)) {
        $rendererId = [string] (Get-ProviderSurfaceCatalogOptionalValue -Object $renderer -PropertyName 'id' -DefaultValue '')
        $nativeCommand = Get-ProviderSurfaceCatalogNativeCommand -Renderer $renderer
        if ($null -ne $nativeCommand) {
            $nativeResult = Invoke-ProviderSurfaceNativeRenderer `
                -RepoRoot $RepoRoot `
                -RendererId $rendererId `
                -NativeCommand $nativeCommand `
                -CatalogPath $CatalogPath `
                -ConsumerName $ConsumerName `
                -EnableCodexRuntime:$EnableCodexRuntime `
                -EnableClaudeRuntime:$EnableClaudeRuntime

            $results.Add([pscustomobject]@{
                    Id = $rendererId
                    ScriptPath = $nativeResult.ScriptPath
                    RuntimeBinaryPath = $nativeResult.RuntimeBinaryPath
                    DispatchKind = $nativeResult.DispatchKind
                    Consumer = $ConsumerName
                    ExitCode = $nativeResult.ExitCode
                }) | Out-Null

            continue
        }

        $scriptPathValue = [string] (Get-ProviderSurfaceCatalogOptionalValue -Object $renderer -PropertyName 'scriptPath' -DefaultValue '')
        if ([string]::IsNullOrWhiteSpace($scriptPathValue)) {
            throw ("Provider surface renderer '{0}' must define either scriptPath or nativeCommand." -f $rendererId)
        }

        $scriptPath = Resolve-ProviderSurfaceCatalogPathValue -RepoRoot $RepoRoot -PathValue $scriptPathValue
        if (-not (Test-Path -LiteralPath $scriptPath -PathType Leaf)) {
            throw ("Provider surface renderer not found: {0}" -f $scriptPath)
        }

        $arguments = Convert-ProviderSurfaceArgumentMap -ArgumentObject (Get-ProviderSurfaceCatalogOptionalValue -Object $renderer -PropertyName 'defaultArgs')
        $arguments['RepoRoot'] = $RepoRoot
        $arguments['Verbose'] = [bool] $RenderVerbose

        & $scriptPath @arguments | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        if ($exitCode -ne 0) {
            throw ("Provider surface renderer '{0}' failed. ExitCode={1}" -f $rendererId, $exitCode)
        }

        $results.Add([pscustomobject]@{
                Id = $rendererId
                ScriptPath = $scriptPath
                RuntimeBinaryPath = $null
                DispatchKind = 'script-path'
                Consumer = $ConsumerName
                ExitCode = $exitCode
            }) | Out-Null
    }

    return @($results.ToArray())
}