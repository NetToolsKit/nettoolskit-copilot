[CmdletBinding()]
param(
    [string] $RepoRoot,
    [string] $CatalogPath,
    [string] $ManifestPath,
    [string] $TargetConfigPath,
    [switch] $CreateBackup,
    [switch] $DryRun
)

$ErrorActionPreference = 'Stop'

$nativeArgs = @('runtime', 'sync-codex-mcp-config')
if ($PSBoundParameters.ContainsKey('RepoRoot') -and -not [string]::IsNullOrWhiteSpace($RepoRoot)) {
    $nativeArgs += @('--repo-root', $RepoRoot)
}
if ($PSBoundParameters.ContainsKey('CatalogPath') -and -not [string]::IsNullOrWhiteSpace($CatalogPath)) {
    $nativeArgs += @('--catalog-path', $CatalogPath)
}
if ($PSBoundParameters.ContainsKey('ManifestPath') -and -not [string]::IsNullOrWhiteSpace($ManifestPath)) {
    $nativeArgs += @('--manifest-path', $ManifestPath)
}
if ($PSBoundParameters.ContainsKey('TargetConfigPath') -and -not [string]::IsNullOrWhiteSpace($TargetConfigPath)) {
    $nativeArgs += @('--target-config-path', $TargetConfigPath)
}
if ($PSBoundParameters.ContainsKey('CreateBackup') -and $CreateBackup) {
    $nativeArgs += '--create-backup'
}
if ($PSBoundParameters.ContainsKey('DryRun') -and $DryRun) {
    $nativeArgs += '--dry-run'
}

& ntk @nativeArgs
exit $LASTEXITCODE