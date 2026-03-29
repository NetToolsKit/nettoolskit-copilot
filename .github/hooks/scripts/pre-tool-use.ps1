Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. (Join-Path $PSScriptRoot 'common.ps1')
$runtimeBinaryPath = Resolve-ProjectedRuntimeBinaryPath
$rawInput = [Console]::In.ReadToEnd()
if ([string]::IsNullOrWhiteSpace($rawInput)) {
    $rawInput = '{}'
}

$output = $rawInput | & $runtimeBinaryPath runtime pre-tool-use @args
$exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
if ($exitCode -ne 0) {
    exit $exitCode
}

$output