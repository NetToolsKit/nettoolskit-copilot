<#
.SYNOPSIS
    Removes trailing blank lines and trailing whitespace from text files.

.DESCRIPTION
    Scans text files (skipping binaries and excluded folders) and normalizes the end-of-file:
        - Removes trailing spaces/tabs at the end of the file.
        - Removes every blank line after the last line of content.
        - Applies the repository EOF policy:
          - the repository default keeps no final newline
          - narrower `.editorconfig` overrides can require one terminal newline for specific file types

    When executed inside a Git repository, it prefers `git ls-files` to respect ignore rules.
    Otherwise it falls back to a filesystem scan starting from -Path (or the current directory)
    while skipping common build/tooling folders.

    In `-GitChangedOnly` mode, discovery is limited to the current Git status set
    so the script only trims files that Git currently reports as modified,
    added, copied, renamed, or untracked.

.PARAMETER Path
    Root folder to scan or a single file path. Defaults to the current directory when omitted.

.PARAMETER LiteralPaths
    Explicit file paths to trim. When provided, discovery/scanning is skipped
    and only these files are processed.

.PARAMETER CheckOnly
    Only checks and lists files that would be fixed. Returns exit code 1 when changes are required.

.PARAMETER GitChangedOnly
    Limits discovery to files currently reported by `git status`.
    Deleted paths are ignored because they no longer exist on disk.

.PARAMETER Verbose
    Prints detailed logs of processed files, decisions and errors.

.EXAMPLE
    Normalizes all supported files under the current directory.
    pwsh -File scripts/maintenance/trim-trailing-blank-lines.ps1

.EXAMPLE
    Lists files requiring adjustments without modifying them.
    pwsh -File scripts/maintenance/trim-trailing-blank-lines.ps1 -Path "C:\repo" -CheckOnly

.EXAMPLE
    Runs with verbose logging so you can audit which files changed.
    pwsh -File scripts/maintenance/trim-trailing-blank-lines.ps1 -Path "C:\repo" -Verbose

.EXAMPLE
    Targets a single file and trims trailing whitespace/blank lines in-place.
    pwsh -File scripts/maintenance/trim-trailing-blank-lines.ps1 -Path "scripts/README.md"

.EXAMPLE
    Lists and trims only the files that Git currently marks as changed.
    pwsh -File scripts/maintenance/trim-trailing-blank-lines.ps1 -GitChangedOnly

.NOTES
    Version: 1.5
    Requirements: PowerShell 7+, Git CLI (optional, for faster discovery).
#>

param (
    [string] $Path,
    [string[]] $LiteralPaths,
    [switch] $Verbose,
    [switch] $CheckOnly,
    [switch] $GitChangedOnly
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
. $script:CommonBootstrapPath -CallerScriptRoot $PSScriptRoot -Helpers @('console-style', 'repository-paths')
$script:IsVerboseEnabled = [bool] $Verbose

# -------------------------------
# Helpers
# -------------------------------
# Writes output text using ANSI color sequences when available.
function Write-ColorLine {
    param(
        [string] $Message,
        [ConsoleColor] $Color = [ConsoleColor]::Gray
    )

    if ($null -eq $PSStyle) {
        Microsoft.PowerShell.Utility\Write-Output $Message
        return
    }

    $ansiColor = switch ($Color) {
        ([ConsoleColor]::Blue) { $PSStyle.Foreground.Blue; break }
        ([ConsoleColor]::Cyan) { $PSStyle.Foreground.Cyan; break }
        ([ConsoleColor]::Green) { $PSStyle.Foreground.Green; break }
        ([ConsoleColor]::Yellow) { $PSStyle.Foreground.Yellow; break }
        ([ConsoleColor]::Red) { $PSStyle.Foreground.Red; break }
        ([ConsoleColor]::DarkGray) { $PSStyle.Foreground.BrightBlack; break }
        default { $PSStyle.Foreground.White }
    }

    Microsoft.PowerShell.Utility\Write-Output ("{0}{1}{2}" -f $ansiColor, $Message, $PSStyle.Reset)
}

# Resolves the repository root by searching for known repository markers.
function Get-RepoRoot ([string] $startPath) {
    try {
        $resolved = if ([string]::IsNullOrWhiteSpace($startPath)) {
            (Get-Location).Path
        } else {
            (Resolve-Path -LiteralPath $startPath).Path
        }

        $gitRoot = (git -C "$resolved" rev-parse --show-toplevel 2>$null)
        if ($LASTEXITCODE -eq 0 -and $gitRoot) {
            return $gitRoot
        }

        return $resolved
    }
    catch {
        return (Get-Location).Path
    }
}

# Returns true when the Git CLI is available for repository-aware discovery.
function Test-GitAvailable {
    $gitCommand = Get-Command git -ErrorAction SilentlyContinue
    return ($null -ne $gitCommand)
}

# Expands a single `{a,b}` EditorConfig brace pattern into flat patterns.
function Expand-EditorConfigBracePattern {
    param(
        [string] $Pattern
    )

    if ([string]::IsNullOrWhiteSpace($Pattern)) {
        return @($Pattern)
    }

    $openIndex = $Pattern.IndexOf('{')
    if ($openIndex -lt 0) {
        return @($Pattern)
    }

    $closeIndex = $Pattern.IndexOf('}', $openIndex + 1)
    if ($closeIndex -lt 0) {
        return @($Pattern)
    }

    $prefix = $Pattern.Substring(0, $openIndex)
    $suffix = $Pattern.Substring($closeIndex + 1)
    $inner = $Pattern.Substring($openIndex + 1, $closeIndex - $openIndex - 1)
    $expandedPatterns = New-Object System.Collections.Generic.List[string]

    foreach ($part in ($inner -split ',')) {
        $trimmedPart = [string] $part
        $trimmedPart = $trimmedPart.Trim()
        if (-not [string]::IsNullOrWhiteSpace($trimmedPart)) {
            $expandedPatterns.Add(('{0}{1}{2}' -f $prefix, $trimmedPart, $suffix)) | Out-Null
        }
    }

    if ($expandedPatterns.Count -eq 0) {
        return @($Pattern)
    }

    return @($expandedPatterns)
}

# Converts a minimal EditorConfig glob pattern into a regex matcher.
function Convert-EditorConfigGlobToRegex {
    param(
        [string] $Pattern
    )

    $normalizedPattern = ([string] $Pattern) -replace '\\', '/'
    $escapedPattern = [Regex]::Escape($normalizedPattern)
    $escapedPattern = $escapedPattern -replace '\\\*\\\*', '.*'
    $escapedPattern = $escapedPattern -replace '\\\*', '[^/]*'
    $escapedPattern = $escapedPattern -replace '\\\?', '[^/]'

    return ('^{0}$' -f $escapedPattern)
}

# Returns true when an EditorConfig section pattern applies to the target file.
function Test-EditorConfigPatternMatch {
    param(
        [string] $RelativePath,
        [string] $FileName,
        [string] $Pattern
    )

    $normalizedRelativePath = ([string] $RelativePath) -replace '\\', '/'
    foreach ($expandedPattern in (Expand-EditorConfigBracePattern -Pattern $Pattern)) {
        $normalizedPattern = ([string] $expandedPattern) -replace '\\', '/'
        $candidate = if ($normalizedPattern.Contains('/')) {
            $normalizedRelativePath
        }
        else {
            $FileName
        }

        if ($candidate -match (Convert-EditorConfigGlobToRegex -Pattern $normalizedPattern)) {
            return $true
        }
    }

    return $false
}

# Resolves the effective insert_final_newline policy for a file from the workspace .editorconfig.
function Get-InsertFinalNewlinePolicyForFile {
    param(
        [string] $WorkspaceRoot,
        [string] $FullPath
    )

    if ([string]::IsNullOrWhiteSpace($WorkspaceRoot) -or [string]::IsNullOrWhiteSpace($FullPath)) {
        return $null
    }

    $editorConfigPath = Join-Path $WorkspaceRoot '.editorconfig'
    if (-not (Test-Path -LiteralPath $editorConfigPath -PathType Leaf)) {
        return $null
    }

    $relativePath = [System.IO.Path]::GetRelativePath($WorkspaceRoot, $FullPath) -replace '\\', '/'
    $fileName = [System.IO.Path]::GetFileName($FullPath)
    $currentPattern = $null
    $effectivePolicy = $null

    foreach ($line in (Get-Content -LiteralPath $editorConfigPath)) {
        $trimmed = [string] $line
        $trimmed = $trimmed.Trim()

        if ([string]::IsNullOrWhiteSpace($trimmed)) {
            continue
        }

        if ($trimmed.StartsWith('#') -or $trimmed.StartsWith(';')) {
            continue
        }

        if ($trimmed.StartsWith('[') -and $trimmed.EndsWith(']')) {
            $currentPattern = $trimmed.Substring(1, $trimmed.Length - 2).Trim()
            continue
        }

        if ($trimmed -notmatch '^insert_final_newline\s*=\s*(true|false)$') {
            continue
        }

        $policy = [System.Convert]::ToBoolean($Matches[1])
        if ([string]::IsNullOrWhiteSpace($currentPattern)) {
            $effectivePolicy = $policy
            continue
        }

        if (Test-EditorConfigPatternMatch -RelativePath $relativePath -FileName $fileName -Pattern $currentPattern) {
            $effectivePolicy = $policy
        }
    }

    return $effectivePolicy
}

# Checks whether a path is located under any configured excluded directory.
function Test-IsUnderExcludedDir ([string] $fullPath, [string[]] $excludeDirs, [string] $root) {
    $norm = [IO.Path]::GetFullPath($fullPath).TrimEnd('\', '/')

    foreach ($d in $excludeDirs) {
        try {
            $exp = [IO.Path]::GetFullPath((Join-Path -Path $root -ChildPath $d)).TrimEnd('\', '/')
            $expWithSlash = $exp + '\'
            $expWithAltSlash = $exp + '/'

            if (
                $norm.Equals($exp, [System.StringComparison]::OrdinalIgnoreCase) -or
                $norm.StartsWith($expWithSlash, [System.StringComparison]::OrdinalIgnoreCase) -or
                $norm.StartsWith($expWithAltSlash, [System.StringComparison]::OrdinalIgnoreCase)
            ) {
                return $true
            }
        }
        catch {
            Write-VerboseColor ("Skipping excluded directory candidate due path resolution failure: {0}" -f $d) 'Yellow'
        }
    }

    return $false
}

# Checks whether a file should be processed based on directory and extension filters.
function Test-IsProcessableFile ([string] $fullPath, [string[]] $excludedExtensions, [string[]] $excludeDirs, [string] $root) {
    if (Test-IsUnderExcludedDir -fullPath $fullPath -excludeDirs $excludeDirs -root $root) {
        return $false
    }

    $extension = [IO.Path]::GetExtension($fullPath)
    if ([string]::IsNullOrWhiteSpace($extension)) {
        return $true
    }

    return ($excludedExtensions -notcontains $extension.ToLowerInvariant())
}

# Discovers only files that currently appear in Git status for the repository.
function Get-GitChangedFiles {
    param(
        [string] $Root
    )

    if (-not (Test-GitAvailable)) {
        throw 'GitChangedOnly requires git to be available on PATH.'
    }

    $statusOutput = git -C "$Root" status --porcelain=v1 -z --untracked-files=all 2>$null
    if ($LASTEXITCODE -ne 0) {
        throw "GitChangedOnly requires a valid git repository root: $Root"
    }

    $entries = @($statusOutput -split "`0" | Where-Object { $_ -ne '' })
    $selectedFiles = New-Object System.Collections.Generic.List[string]

    for ($index = 0; $index -lt $entries.Count; $index++) {
        $entry = [string] $entries[$index]
        if ($entry.Length -lt 4) {
            continue
        }

        $statusCode = $entry.Substring(0, 2)
        $relativePath = $entry.Substring(3)

        if ($statusCode[0] -eq 'D' -or $statusCode[1] -eq 'D') {
            continue
        }

        if ($statusCode[0] -eq 'R' -or $statusCode[1] -eq 'R' -or $statusCode[0] -eq 'C' -or $statusCode[1] -eq 'C') {
            if (($index + 1) -ge $entries.Count) {
                continue
            }

            $index++
            $relativePath = [string] $entries[$index]
        }

        if ([string]::IsNullOrWhiteSpace($relativePath)) {
            continue
        }

        $fullPath = Join-Path -Path $Root -ChildPath $relativePath
        if (Test-Path -LiteralPath $fullPath -PathType Leaf) {
            $selectedFiles.Add($fullPath) | Out-Null
        }
    }

    return @($selectedFiles | Select-Object -Unique)
}

# -------------------------------
# Configuration (extensions and exclusions)
# -------------------------------
$BinaryExtensions = @(
    '.dll', '.exe', '.pdb', '.png', '.jpg', '.jpeg', '.gif', '.ico',
    '.zip', '.7z', '.rar', '.pdf', '.mp4', '.mp3', '.wav', '.ogg', '.webp', '.bmp',
    '.ttf', '.otf', '.woff', '.woff2', '.snk', '.nupkg', '.sln'
)

$ExcludeDirs = @(
    'bin', 'obj', '.git', 'node_modules', '.vs', '.idea',
    '.build', '.deployment', 'artifacts', 'target'
)
$explicitLiteralPaths = @($LiteralPaths | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
$hasLiteralPaths = $explicitLiteralPaths.Count -gt 0

# -------------------------------
# Discover files (single-file mode or repo scanning)
# -------------------------------
$files = @()

if ($hasLiteralPaths) {
    $files = @(
        $explicitLiteralPaths |
            ForEach-Object { Resolve-Path -LiteralPath $_ -ErrorAction Stop } |
            ForEach-Object { $_.Path }
    )

    $root = if ($files.Count -gt 0) {
        Get-RepoRoot -startPath (Split-Path -Path $files[0] -Parent)
    }
    else {
        Get-RepoRoot -startPath ((Get-Location).Path)
    }
    Write-ColorLine -Message ("Root: {0}" -f $root) -Color Blue
    Write-ColorLine -Message 'Explicit file list mode: enabled' -Color Cyan
}
elseif ($Path -and (Test-Path -LiteralPath $Path -PathType Leaf)) {
    # Single file mode
    $fullFile = (Resolve-Path -LiteralPath $Path).Path
    $root     = [System.IO.Path]::GetDirectoryName($fullFile)

    Write-ColorLine -Message ("Root: {0}" -f $root) -Color Blue

    $files = @($fullFile)
}
else {
    # Repository or directory mode
    $root = Get-RepoRoot -startPath $Path

    Write-ColorLine -Message ("Root: {0}" -f $root) -Color Blue

    if ($GitChangedOnly) {
        try {
            $files = @(
                Get-GitChangedFiles -Root $root |
                    Where-Object {
                        Test-IsProcessableFile -fullPath $_ -excludedExtensions $BinaryExtensions -excludeDirs $ExcludeDirs -root $root
                    }
            )

            Write-ColorLine -Message 'Git changed files mode: enabled' -Color Cyan
        }
        catch {
            throw ("Failed to discover changed Git files: {0}" -f $_.Exception.Message)
        }
    }
    else {
        try {
            # Git method: tracked + untracked (non-ignored)
            $gitFiles = git -C "$root" ls-files -z --cached -o --exclude-standard 2>$null
            if ($LASTEXITCODE -eq 0 -and $gitFiles) {
                $files = ($gitFiles -split "`0") |
                    Where-Object { $_ -ne '' } |
                    ForEach-Object { Join-Path -Path $root -ChildPath $_ }

                # Apply directory and extension exclusions to git list as well.
                $files = $files |
                    Where-Object {
                        Test-IsProcessableFile -fullPath $_ -excludedExtensions $BinaryExtensions -excludeDirs $ExcludeDirs -root $root
                    }
            }
        }
        catch {
            Write-VerboseColor 'Git file discovery failed; falling back to filesystem scan.' 'Yellow'
        }

        if (-not $files) {
            # Filesystem method
            $files = Get-ChildItem -Path $root -Recurse -File |
                Where-Object {
                    Test-IsProcessableFile -fullPath $_.FullName -excludedExtensions $BinaryExtensions -excludeDirs $ExcludeDirs -root $root
                } |
                ForEach-Object { $_.FullName }
        }
    }
}

$files = @($files)
$workspaceRoot = Get-RepoRoot -startPath $root

Start-ExecutionSession `
    -Name 'trim-trailing-blank-lines' `
    -RootPath $root `
    -Metadata ([ordered]@{
            'Check-only' = [bool] $CheckOnly
            'Git changed only' = [bool] $GitChangedOnly
            'Explicit file mode' = [bool] $hasLiteralPaths
        }) `
    -IncludeMetadataInDefaultOutput | Out-Null

Write-ColorLine -Message ("Files found: {0}" -f $files.Count) -Color Yellow

if (($GitChangedOnly -or $hasLiteralPaths) -and $files.Count -gt 0) {
    $selectionHeading = if ($hasLiteralPaths) {
        'Explicit files selected for trim:'
    }
    else {
        'Git changed files selected for trim:'
    }

    Write-ColorLine -Message $selectionHeading -Color Cyan
    foreach ($selectedFile in $files) {
        Write-ColorLine -Message ("  - {0}" -f [IO.Path]::GetRelativePath($root, $selectedFile)) -Color DarkGray
    }
}

# -------------------------------
# Processing
# -------------------------------
$changed = New-Object System.Collections.Generic.List[string]

# Regex patterns to clean EOF regardless of line ending type; \z ensures true end-of-string
$trailNlPattern = '([ \t]*(?:\r\n|\n|\r))+\z'
$trailWsPattern = '[ \t]+\z'

foreach ($fullPath in $files) {
    try {
        $relative = [IO.Path]::GetRelativePath($root, $fullPath)

        $text = Get-Content -Raw -LiteralPath $fullPath -ErrorAction Stop
        if ($null -eq $text) {
            continue
        }

        $updated = $text -replace $trailNlPattern, ''
        $updated = $updated -replace $trailWsPattern, ''
        $insertFinalNewline = Get-InsertFinalNewlinePolicyForFile -WorkspaceRoot $workspaceRoot -FullPath $fullPath
        $preferredNewline = if ($text.Contains("`r`n")) {
            "`r`n"
        }
        elseif ($text.Contains("`n")) {
            "`n"
        }
        elseif ($text.Contains("`r")) {
            "`r"
        }
        else {
            "`n"
        }

        if ($insertFinalNewline -and -not [string]::IsNullOrEmpty($updated)) {
            $updated = '{0}{1}' -f $updated, $preferredNewline
        }

        if ($updated -ne $text) {
            if (-not $CheckOnly) {
                # Write exactly the computed content; do not add extra newline; UTF-8
                Set-Content -LiteralPath $fullPath -Value $updated -NoNewline -Encoding UTF8
            }

            $changed.Add($fullPath) | Out-Null
            Write-VerboseColor ("Fixed EOF for: {0}" -f $relative) 'Green'
        }
        else {
            Write-VerboseColor ("OK: {0}" -f $relative) 'Green'
        }
    }
    catch {
        Write-VerboseColor ("ERROR: {0} [{1}]" -f $_.Exception.Message, $fullPath) 'Red'
    }
}

# -------------------------------
# Summary and exit code
# -------------------------------
Write-StyledOutput ("Changed files: {0}" -f $changed.Count)

if ($Verbose -and $changed.Count -gt 0) {
    $changed | ForEach-Object { Write-StyledOutput $_ }
}

if ($CheckOnly) {
    $checkOnlyStatus = if ($changed.Count -gt 0) { 'warning' } else { 'passed' }
    Complete-ExecutionSession -Name 'trim-trailing-blank-lines' -Status $checkOnlyStatus -Summary ([ordered]@{
            'Files found' = $files.Count
            'Changed files' = $changed.Count
        }) | Out-Null
    if ($changed.Count -gt 0) {
        exit 1
    }
    else {
        exit 0
    }
}

Complete-ExecutionSession -Name 'trim-trailing-blank-lines' -Status 'passed' -Summary ([ordered]@{
        'Files found' = $files.Count
        'Changed files' = $changed.Count
    }) | Out-Null

exit 0