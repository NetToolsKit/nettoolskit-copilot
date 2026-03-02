<#
.SYNOPSIS
    Runtime tests for critical scripts without external test frameworks.

.DESCRIPTION
    Validates script contracts and smoke behavior for runtime scripts used
    by bootstrap, healthcheck, self-heal, and cleanup flows.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/runtime-scripts.tests.ps1

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

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

    $candidates += (Get-Location).Path

    foreach ($candidate in ($candidates | Select-Object -Unique)) {
        $current = $candidate
        for ($index = 0; $index -lt 6 -and -not [string]::IsNullOrWhiteSpace($current); $index++) {
            $hasLayout = (Test-Path -LiteralPath (Join-Path $current '.github')) -and (Test-Path -LiteralPath (Join-Path $current '.codex'))
            if ($hasLayout) {
                return $current
            }

            $current = Split-Path -Path $current -Parent
        }
    }

    throw 'Could not detect repository root containing both .github and .codex.'
}

function Assert-True {
    param(
        [bool] $Condition,
        [string] $Message
    )

    if (-not $Condition) {
        throw $Message
    }
}

function Assert-Contains {
    param(
        [string[]] $Collection,
        [string] $Value,
        [string] $Message
    )

    if (-not ($Collection -contains $Value)) {
        throw $Message
    }
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$runtimeScriptRoot = Join-Path $resolvedRepoRoot 'scripts/runtime'

$exitCode = 0

try {
    $scriptPath = Join-Path $runtimeScriptRoot 'bootstrap.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'RepoRoot' -Message 'bootstrap missing RepoRoot parameter.'
    Assert-Contains -Collection $keys -Value 'TargetGithubPath' -Message 'bootstrap missing TargetGithubPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetCodexPath' -Message 'bootstrap missing TargetCodexPath parameter.'
    Assert-Contains -Collection $keys -Value 'Mirror' -Message 'bootstrap missing Mirror parameter.'

    $scriptPath = Join-Path $runtimeScriptRoot 'healthcheck.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'ValidationProfile' -Message 'healthcheck missing ValidationProfile parameter.'
    Assert-Contains -Collection $keys -Value 'WarningOnly' -Message 'healthcheck missing WarningOnly parameter.'
    Assert-Contains -Collection $keys -Value 'TargetGithubPath' -Message 'healthcheck missing TargetGithubPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetCodexPath' -Message 'healthcheck missing TargetCodexPath parameter.'

    $scriptPath = Join-Path $runtimeScriptRoot 'self-heal.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'Mirror' -Message 'self-heal missing Mirror parameter.'
    Assert-Contains -Collection $keys -Value 'ApplyMcpConfig' -Message 'self-heal missing ApplyMcpConfig parameter.'
    Assert-Contains -Collection $keys -Value 'TargetGithubPath' -Message 'self-heal missing TargetGithubPath parameter.'
    Assert-Contains -Collection $keys -Value 'TargetCodexPath' -Message 'self-heal missing TargetCodexPath parameter.'

    $scriptPath = Join-Path $runtimeScriptRoot 'clean-codex-runtime.ps1'
    $command = Get-Command -Name $scriptPath -ErrorAction Stop
    $keys = @($command.Parameters.Keys)
    Assert-Contains -Collection $keys -Value 'CodexHome' -Message 'clean-codex-runtime missing CodexHome parameter.'
    Assert-Contains -Collection $keys -Value 'IncludeSessions' -Message 'clean-codex-runtime missing IncludeSessions parameter.'
    Assert-Contains -Collection $keys -Value 'SessionRetentionDays' -Message 'clean-codex-runtime missing SessionRetentionDays parameter.'
    Assert-Contains -Collection $keys -Value 'LogRetentionDays' -Message 'clean-codex-runtime missing LogRetentionDays parameter.'

    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    $targetGithub = Join-Path $tempRoot '.github'
    $targetCodex = Join-Path $tempRoot '.codex'
    $scriptPath = Join-Path $runtimeScriptRoot 'bootstrap.ps1'
    try {
        & $scriptPath -RepoRoot $resolvedRepoRoot -TargetGithubPath $targetGithub -TargetCodexPath $targetCodex -Mirror | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-True ($exitCode -eq 0) 'bootstrap smoke test failed.'
        Assert-True (Test-Path -LiteralPath $targetGithub -PathType Container) 'bootstrap did not create target github folder.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetCodex 'skills') -PathType Container) 'bootstrap did not sync skills folder.'
        Assert-True (Test-Path -LiteralPath (Join-Path $targetCodex 'shared-scripts') -PathType Container) 'bootstrap did not sync shared-scripts folder.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    $codexHome = Join-Path $tempRoot '.codex'
    $tmpDir = Join-Path $codexHome 'tmp'
    $logDir = Join-Path $codexHome 'log'
    $sessionsDir = Join-Path $codexHome 'sessions'
    $oldLog = Join-Path $logDir 'old.log'
    $oldSession = Join-Path $sessionsDir 'old-session.json'
    $tempFile = Join-Path $tmpDir 'temp.txt'
    $scriptPath = Join-Path $runtimeScriptRoot 'clean-codex-runtime.ps1'
    try {
        New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null
        New-Item -ItemType Directory -Path $logDir -Force | Out-Null
        New-Item -ItemType Directory -Path $sessionsDir -Force | Out-Null
        Set-Content -LiteralPath $tempFile -Value 'temp'
        Set-Content -LiteralPath $oldLog -Value 'log'
        Set-Content -LiteralPath $oldSession -Value 'session'
        $expiredDate = (Get-Date).AddDays(-10)
        (Get-Item -LiteralPath $oldLog).LastWriteTime = $expiredDate
        (Get-Item -LiteralPath $oldSession).LastWriteTime = $expiredDate
        & $scriptPath -CodexHome $codexHome -IncludeSessions -SessionRetentionDays 1 -LogRetentionDays 1 -Apply | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-True ($exitCode -eq 0) 'clean-codex-runtime smoke test failed.'
        Assert-True (-not (Test-Path -LiteralPath $tempFile)) 'clean-codex-runtime did not remove tmp file.'
        Assert-True (-not (Test-Path -LiteralPath $oldLog)) 'clean-codex-runtime did not remove old log.'
        Assert-True (-not (Test-Path -LiteralPath $oldSession)) 'clean-codex-runtime did not remove old session.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    Write-Host '[OK] runtime script tests passed.'
    exit 0
}
catch {
    $message = $_.Exception.Message
    $trace = $_.ScriptStackTrace
    if ([string]::IsNullOrWhiteSpace($trace)) {
        Write-Host ("[FAIL] runtime script tests failed: {0}" -f $message)
    }
    else {
        Write-Host ("[FAIL] runtime script tests failed: {0}`n{1}" -f $message, $trace)
    }
    exit 1
}