<#
.SYNOPSIS
    Runtime tests for the native `ntk validation instruction-architecture`
    surface without external frameworks.

.DESCRIPTION
    Covers success, failure, and warning-only ownership-marker behavior for
    instruction architecture validation.

.PARAMETER RepoRoot
    Optional repository root. If omitted, auto-detects a root containing .github and .codex.

.EXAMPLE
    pwsh -File scripts/tests/runtime/instruction-architecture.tests.ps1

.NOTES
    Version: 1.0
    Requirements: PowerShell 7+.
#>

param(
    [string] $RepoRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$script:ScriptRoot = Split-Path -Path $PSCommandPath -Parent
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
. $script:CommonBootstrapPath -CallerScriptRoot $PSScriptRoot -Helpers @('repository-paths', 'runtime-paths')

function Assert-ExitCode {
    param(
        [int] $ExitCode,
        [int] $Expected,
        [string] $Message
    )

    if ($ExitCode -ne $Expected) {
        throw $Message
    }
}

function Write-TextFile {
    param(
        [string] $Path,
        [string] $Content
    )

    $parent = Split-Path -Path $Path -Parent
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }

    Set-Content -LiteralPath $Path -Value $Content
}

function Initialize-ValidInstructionArchitectureRepo {
    param(
        [string] $Root
    )

    Write-TextFile -Path (Join-Path $Root '.github/governance/instruction-ownership.manifest.json') -Content @'
{
  "version": 1,
  "intentionalGlobalExceptions": [
    {
      "concern": "Global context must remain visible.",
      "ownedBy": "agent-control"
    }
  ],
  "architectureConstraints": {
    "globalCoreMaxChars": {
      "AGENTS.md": 14000,
      "copilot-instructions.md": 14000
    },
    "routing": {
      "maxAlwaysFiles": 10,
      "maxSelectedFiles": 5,
      "requiredAlwaysPaths": [
        "AGENTS.md",
        "copilot-instructions.md",
        "instructions/agents/ntk-agents-super-agent.instructions.md",
        "instructions/core/ntk-core-repository-operating-model.instructions.md",
        "instructions/core/ntk-core-artifact-layout.instructions.md",
        "instructions/process/planning/ntk-process-subagent-planning-workflow.instructions.md",
        "instructions/process/planning/ntk-process-workflow-optimization.instructions.md",
        "instructions/core/ntk-core-authoritative-sources.instructions.md",
        "instructions/operations/automation/ntk-runtime-powershell-execution.instructions.md",
        "instructions/process/delivery/ntk-process-feedback-changelog.instructions.md"
      ]
    }
  },
  "layers": [
    {
      "id": "global-core",
      "pathPatterns": [
        ".github/AGENTS.md",
        ".github/copilot-instructions.md"
      ]
    },
    {
      "id": "agent-control",
      "pathPatterns": [
        ".github/instructions/agents/*.instructions.md"
      ]
    },
    {
      "id": "repository-operating-model",
      "pathPatterns": [
        ".github/instructions/core/ntk-core-repository-operating-model.instructions.md"
      ]
    },
    {
      "id": "cross-cutting-policies",
      "pathPatterns": [
        ".github/instructions/core/ntk-core-authoritative-sources.instructions.md",
        ".github/governance/*",
        ".github/policies/*"
      ]
    },
    {
      "id": "domain-instructions",
      "pathPatterns": [
        ".github/instructions/*.instructions.md"
      ],
      "excludePatterns": [
        ".github/instructions/core/ntk-core-authoritative-sources.instructions.md",
        ".github/instructions/agents/ntk-agents-super-agent.instructions.md",
        ".github/instructions/core/ntk-core-repository-operating-model.instructions.md"
      ]
    },
    {
      "id": "prompts",
      "pathPatterns": [
        ".github/prompts/*"
      ],
      "forbiddenOwnershipMarkers": [
        "single source of truth",
        "global rules live here",
        "always applied"
      ]
    },
    {
      "id": "templates",
      "pathPatterns": [
        ".github/templates/*",
        ".vscode/*.tamplate.jsonc"
      ],
      "forbiddenOwnershipMarkers": [
        "single source of truth",
        "global rules live here",
        "always applied"
      ]
    },
    {
      "id": "codex-skills",
      "pathPatterns": [
        ".codex/skills/*/SKILL.md"
      ],
      "forbiddenOwnershipMarkers": [
        "single source of truth",
        "global rules live here"
      ]
    },
    {
      "id": "orchestration",
      "pathPatterns": [
        "scripts/orchestration/*"
      ]
    },
    {
      "id": "runtime-projection",
      "pathPatterns": [
        "scripts/runtime/*"
      ]
    }
  ]
}
'@
    Write-TextFile -Path (Join-Path $Root '.github/AGENTS.md') -Content @'
# AGENTS

Use `instructions/core/ntk-core-repository-operating-model.instructions.md`.
Use `instructions/core/ntk-core-authoritative-sources.instructions.md`.
'@
    Write-TextFile -Path (Join-Path $Root '.github/copilot-instructions.md') -Content @'
# Global Instructions

Use `instructions/core/ntk-core-repository-operating-model.instructions.md`.
Use `instructions/core/ntk-core-authoritative-sources.instructions.md`.
'@
    Write-TextFile -Path (Join-Path $Root '.github/instruction-routing.catalog.yml') -Content @'
always:
  - path: AGENTS.md
  - path: copilot-instructions.md
  - path: instructions/agents/ntk-agents-super-agent.instructions.md
  - path: instructions/core/ntk-core-repository-operating-model.instructions.md
  - path: instructions/core/ntk-core-artifact-layout.instructions.md
  - path: instructions/process/planning/ntk-process-subagent-planning-workflow.instructions.md
  - path: instructions/process/planning/ntk-process-workflow-optimization.instructions.md
  - path: instructions/core/ntk-core-authoritative-sources.instructions.md
  - path: instructions/operations/automation/ntk-runtime-powershell-execution.instructions.md
  - path: instructions/process/delivery/ntk-process-feedback-changelog.instructions.md
'@
    Write-TextFile -Path (Join-Path $Root '.github/prompts/route-instructions.prompt.md') -Content @'
---
description: Route a request
mode: ask
tools: ['readFile']
---

# Route Instructions

Hard cap: at most 5 selected instruction files (excluding mandatory).
'@
    Write-TextFile -Path (Join-Path $Root '.github/prompts/example.prompt.md') -Content @'
---
description: Example prompt
mode: ask
tools: ['readFile']
---

# Example Prompt

Use the routing catalog.
'@
    Write-TextFile -Path (Join-Path $Root '.github/templates/example.md') -Content "# Example Template`n`nUse this as a reusable artifact."
    Write-TextFile -Path (Join-Path $Root '.github/policies/example.policy.md') -Content '# Example Policy'
    Write-TextFile -Path (Join-Path $Root '.github/instructions/core/ntk-core-repository-operating-model.instructions.md') -Content '# Repository Operating Model'
    Write-TextFile -Path (Join-Path $Root '.github/instructions/core/ntk-core-authoritative-sources.instructions.md') -Content '# Authoritative Sources'
    Write-TextFile -Path (Join-Path $Root '.github/instructions/agents/ntk-agents-super-agent.instructions.md') -Content '# Super Agent'
    Write-TextFile -Path (Join-Path $Root '.github/instructions/core/ntk-core-artifact-layout.instructions.md') -Content '# Artifact Layout'
    Write-TextFile -Path (Join-Path $Root '.github/instructions/process/planning/ntk-process-subagent-planning-workflow.instructions.md') -Content '# Subagent Planning Workflow'
    Write-TextFile -Path (Join-Path $Root '.github/instructions/process/planning/ntk-process-workflow-optimization.instructions.md') -Content '# Workflow Optimization'
    Write-TextFile -Path (Join-Path $Root '.github/instructions/operations/automation/ntk-runtime-powershell-execution.instructions.md') -Content '# PowerShell Execution'
    Write-TextFile -Path (Join-Path $Root '.github/instructions/process/delivery/ntk-process-feedback-changelog.instructions.md') -Content '# Feedback Changelog'
    Write-TextFile -Path (Join-Path $Root '.codex/skills/sample/agents/openai.yaml') -Content @'
display_name: Sample Skill
short_description: Example
default_prompt: $sample
'@
    Write-TextFile -Path (Join-Path $Root '.codex/skills/sample/SKILL.md') -Content @'
---
name: sample
description: sample skill
---

# Sample Skill

Load `ntk-core-repository-operating-model.instructions.md`.
'@
    Write-TextFile -Path (Join-Path $Root 'scripts/orchestration/example.ps1') -Content "Write-Host 'orchestration'"
    Write-TextFile -Path (Join-Path $Root 'scripts/runtime/bootstrap.ps1') -Content "Write-Host 'runtime'"
}

$resolvedRepoRoot = Resolve-RepositoryRoot -RequestedRoot $RepoRoot
$runtimeBinaryPath = Resolve-RepositoryRuntimeBinaryPath -ResolvedRepoRoot $resolvedRepoRoot

try {
    $tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString('N'))
    try {
        $fixtureRepoRoot = Join-Path $tempRoot 'instruction-architecture-fixture'
        Initialize-ValidInstructionArchitectureRepo -Root $fixtureRepoRoot

        & $runtimeBinaryPath 'validation' 'instruction-architecture' '--repo-root' $fixtureRepoRoot '--warning-only' 'false' | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 0 -Message 'Repository instruction architecture should pass.'

        $invalidManifestPath = Join-Path $tempRoot 'instruction-ownership.manifest.json'
        Write-TextFile -Path $invalidManifestPath -Content @'
{
  "version": 1,
  "layers": [
    {
      "id": "prompts",
      "description": "invalid test manifest",
      "pathPatterns": [".github/prompts/*"]
    }
  ]
}
'@
        & $runtimeBinaryPath 'validation' 'instruction-architecture' '--repo-root' $fixtureRepoRoot '--manifest-path' $invalidManifestPath '--warning-only' 'false' | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Manifest missing required layers should fail.'
        Remove-Item -LiteralPath $invalidManifestPath -Force

        $invalidAgentsPath = Join-Path $tempRoot 'AGENTS.md'
        Write-TextFile -Path $invalidAgentsPath -Content @'
# Temporary AGENTS

- This file intentionally omits repository-operating-model reference.
'@
        & $runtimeBinaryPath 'validation' 'instruction-architecture' '--repo-root' $fixtureRepoRoot '--agents-path' $invalidAgentsPath '--warning-only' 'false' | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Missing global core architecture reference should fail.'
        Remove-Item -LiteralPath $invalidAgentsPath -Force

        $skillRoot = Join-Path $tempRoot 'skills'
        $skillPath = Join-Path $skillRoot 'sample\SKILL.md'
        Write-TextFile -Path $skillPath -Content @'
---
name: sample-skill
description: temporary skill without canonical repo-operating reference
---

# Sample Skill

Load `.github/AGENTS.md` and `.github/copilot-instructions.md`.
'@
        & $runtimeBinaryPath 'validation' 'instruction-architecture' '--repo-root' $fixtureRepoRoot '--skill-root' $skillRoot '--warning-only' 'false' | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Missing skill canonical repository-operating reference should fail.'

        $promptRoot = Join-Path $tempRoot 'prompts'
        $promptPath = Join-Path $promptRoot 'ownership.prompt.md'
        Write-TextFile -Path $promptPath -Content @'
# Temporary prompt

This prompt claims to be the single source of truth for the whole repository.
'@
        & $runtimeBinaryPath 'validation' 'instruction-architecture' '--repo-root' $fixtureRepoRoot '--prompt-root' $promptRoot '--warning-only' 'false' | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 0 -Message 'Prompt ownership markers should warn but not fail.'

        $routePromptPath = Join-Path $tempRoot 'route-instructions.prompt.md'
        Write-TextFile -Path $routePromptPath -Content @'
---
description: Temporary route prompt
mode: ask
tools: ['readFile']
---

# Route Instructions

Use the routing catalog and return JSON.
'@
        & $runtimeBinaryPath 'validation' 'instruction-architecture' '--repo-root' $fixtureRepoRoot '--route-prompt-path' $routePromptPath '--warning-only' 'false' | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 1 -Message 'Route prompt without deterministic hard cap should fail.'
        Remove-Item -LiteralPath $routePromptPath -Force

        $templateRoot = Join-Path $tempRoot 'templates'
        $templatePath = Join-Path $templateRoot 'settings.tamplate.jsonc'
        Write-TextFile -Path $templatePath -Content @'
{
  "//": "temporary template that wrongly claims to be the single source of truth"
}
'@
        & $runtimeBinaryPath 'validation' 'instruction-architecture' '--repo-root' $fixtureRepoRoot '--template-root' $templateRoot '--warning-only' 'false' | Out-Null
        $exitCode = if ($null -eq $LASTEXITCODE) { 0 } else { [int] $LASTEXITCODE }
        Assert-ExitCode -ExitCode $exitCode -Expected 0 -Message 'Template ownership markers should warn but not fail.'
    }
    finally {
        if (Test-Path -LiteralPath $tempRoot) {
            Remove-Item -LiteralPath $tempRoot -Recurse -Force
        }
    }

    Write-Host '[OK] instruction architecture tests passed.'
    exit 0
}
catch {
    Write-Host ("[FAIL] instruction architecture tests failed: {0}" -f $_.Exception.Message)
    exit 1
}