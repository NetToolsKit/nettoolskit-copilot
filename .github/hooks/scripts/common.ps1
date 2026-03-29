Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Resolve-ProjectedRuntimeHookScriptPath {
    param(
        [Parameter(Mandatory = $true)]
        [string] $ScriptName
    )

    $candidatePaths = @(
        (Join-Path $PSScriptRoot (Join-Path '..\..\..\scripts\runtime\hooks' $ScriptName)),
        (Join-Path $PSScriptRoot (Join-Path '..\..\scripts\runtime\hooks' $ScriptName)),
        (Join-Path $PSScriptRoot (Join-Path '..\..\..\..\..\scripts\runtime\hooks' $ScriptName))
    )

    foreach ($candidatePath in $candidatePaths) {
        if (Test-Path -LiteralPath $candidatePath -PathType Leaf) {
            return [System.IO.Path]::GetFullPath($candidatePath)
        }
    }

    throw ("Unable to locate canonical runtime hook script '{0}' from '{1}'." -f $ScriptName, $PSScriptRoot)
}

function Resolve-ProjectedRuntimeBinaryPath {
    if (-not [string]::IsNullOrWhiteSpace($env:CODEX_NTK_RUNTIME_BIN_PATH)) {
        $overridePath = [System.IO.Path]::GetFullPath($env:CODEX_NTK_RUNTIME_BIN_PATH)
        if (Test-Path -LiteralPath $overridePath -PathType Leaf) {
            return $overridePath
        }
    }

    $binaryName = if ($IsWindows) { 'ntk.exe' } else { 'ntk' }
    $candidatePaths = @(
        (Join-Path $PSScriptRoot (Join-Path '..\..\bin' $binaryName)),
        (Join-Path $PSScriptRoot (Join-Path '..\..\..\bin' $binaryName)),
        (Join-Path $PSScriptRoot (Join-Path '..\..\..\.build\target\debug' $binaryName)),
        (Join-Path $PSScriptRoot (Join-Path '..\..\..\..\..\.build\target\debug' $binaryName))
    )

    foreach ($candidatePath in $candidatePaths) {
        if (Test-Path -LiteralPath $candidatePath -PathType Leaf) {
            return [System.IO.Path]::GetFullPath($candidatePath)
        }
    }

    $ntkCommand = Get-Command ntk -ErrorAction SilentlyContinue
    if ($null -ne $ntkCommand -and -not [string]::IsNullOrWhiteSpace([string] $ntkCommand.Source)) {
        return [System.IO.Path]::GetFullPath([string] $ntkCommand.Source)
    }

    throw ("Unable to locate managed runtime binary '{0}' from '{1}'." -f $binaryName, $PSScriptRoot)
}