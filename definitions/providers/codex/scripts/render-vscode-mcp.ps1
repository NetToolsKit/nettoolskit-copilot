[CmdletBinding()]
param(
    [string] $RepoRoot,
    [string] $CatalogPath,
    [string] $OutputPath
)

$ErrorActionPreference = 'Stop'

$nativeArgs = @('runtime', 'render-vscode-mcp-template')
if ($PSBoundParameters.ContainsKey('RepoRoot') -and -not [string]::IsNullOrWhiteSpace($RepoRoot)) {
    $nativeArgs += @('--repo-root', $RepoRoot)
}
if ($PSBoundParameters.ContainsKey('CatalogPath') -and -not [string]::IsNullOrWhiteSpace($CatalogPath)) {
    $nativeArgs += @('--catalog-path', $CatalogPath)
}
if ($PSBoundParameters.ContainsKey('OutputPath') -and -not [string]::IsNullOrWhiteSpace($OutputPath)) {
    $nativeArgs += @('--output-path', $OutputPath)
}

& ntk @nativeArgs
exit $LASTEXITCODE