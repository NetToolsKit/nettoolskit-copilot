//! Tests for executable validation command surfaces exposed by `ntk`.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Command as ProcessCommand;
use tempfile::TempDir;

fn ntk() -> Command {
    cargo_bin_cmd!("ntk")
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_validation_repo_root(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github")).expect(".github should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect(".codex should be created");
}

fn initialize_security_baseline_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(&repo_root.join("CODEOWNERS"), "* @example\n");
    write_file(&repo_root.join(".github/AGENTS.md"), "# Agents\n");
    write_file(
        &repo_root.join(".github/copilot-instructions.md"),
        "# Copilot\n",
    );
    write_file(
        &repo_root.join(".github/governance/security-baseline.json"),
        r#"{
  "version": 1,
  "requiredFiles": ["CODEOWNERS", ".github/AGENTS.md"],
  "requiredDirectories": [".github/governance", "scripts/validation"],
  "scanExtensions": [".md", ".ps1"],
  "excludedPathGlobs": [".temp/**"],
  "forbiddenPathGlobs": ["**/*.key"],
  "forbiddenContentPatterns": [
    {
      "id": "private-key-block",
      "pattern": "-----BEGIN PRIVATE KEY-----",
      "severity": "failure"
    },
    {
      "id": "hardcoded-password-assignment",
      "pattern": "(?i)(password|passwd|pwd)\\s*[:=]\\s*[\"'](?!\\*{3}|changeme|password|example|your-password)[^\"']{8,}[\"']",
      "severity": "warning"
    }
  ],
  "allowedContentPatterns": [
    "(?i)example-password"
  ]
}"#,
    );
    write_file(
        &repo_root.join("scripts/validation/validate-agent-hooks.ps1"),
        "Write-Output 'ok'\n",
    );
    write_file(&repo_root.join("README.md"), "# Repo\n");
}

fn initialize_shared_script_checksums_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(
        &repo_root.join("scripts/common/a.ps1"),
        "Write-Output 'a'\n",
    );
    write_file(
        &repo_root.join("scripts/security/b.ps1"),
        "Write-Output 'b'\n",
    );
    write_file(
        &repo_root.join(".github/governance/shared-script-checksums.manifest.json"),
        r#"{
  "version": 1,
  "sourceRepository": "https://example.invalid/repo",
  "hashAlgorithm": "SHA256",
  "includedRoots": [
    "scripts/common",
    "scripts/security"
  ],
  "entries": [
    {
      "path": "scripts/common/a.ps1",
      "sha256": "5bf6ac0a30397ddeb64d29e038e66b27f9e79d7fccb3029be82fc763997cbadb"
    },
    {
      "path": "scripts/security/b.ps1",
      "sha256": "3f54252b5c9557fc0c76168aaf530a1339b27138593bec12dccc4a196e966897"
    }
  ]
}"#,
    );
}

fn initialize_supply_chain_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(
        &repo_root.join(".github/governance/supply-chain.baseline.json"),
        r#"{
  "version": 1,
  "sbomOutputPath": ".temp/audit/sbom.latest.json",
  "licenseEvidencePath": ".temp/audit/licenses.latest.json",
  "requireLicenseEvidence": false,
  "warnOnMissingLicenseEvidence": false,
  "warnOnEmptyDependencySet": false,
  "excludedPathGlobs": [
    ".git/**",
    ".temp/**",
    "**/bin/**",
    "**/obj/**",
    "**/.vs/**"
  ],
  "blockedDependencyPatterns": [
    "(?i)^event-stream$"
  ],
  "sensitiveDependencyPatterns": [
    "(?i)^log4j(?:-.*)?$"
  ]
}"#,
    );
    write_file(
        &repo_root.join("package.json"),
        r#"{
  "dependencies": {
    "chalk": "^5.0.0"
  },
  "devDependencies": {
    "vitest": "^2.1.0"
  }
}"#,
    );
    write_file(
        &repo_root.join("Cargo.toml"),
        r#"[package]
name = "fixture"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#,
    );
    write_file(
        &repo_root.join("src/App/App.csproj"),
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <ItemGroup>
    <PackageReference Include="MediatR" Version="12.0.1" />
  </ItemGroup>
</Project>"#,
    );
    write_file(
        &repo_root.join("Directory.Packages.props"),
        r#"<Project>
  <ItemGroup>
    <PackageReference Include="Serilog" Version="4.0.0" />
  </ItemGroup>
</Project>"#,
    );
}

fn initialize_powershell_standards_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(
        &repo_root.join("scripts/runtime/install.ps1"),
        r#"<#
.SYNOPSIS
Installs runtime assets.

.DESCRIPTION
Ensures runtime assets are present.

.PARAMETER RepoRoot
Optional repository root.

.EXAMPLE
pwsh -File scripts/runtime/install.ps1

.NOTES
Version: 1.0
#>

param(
    [string] $RepoRoot
)

$ErrorActionPreference = 'Stop'

# Returns a sample value.
function Get-ExampleValue {
    param()

    return 'ok'
}
"#,
    );
}

fn initialize_warning_baseline_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(
        &repo_root.join(".github/governance/warning-baseline.json"),
        r#"{
  "version": 1,
  "maxTotalWarnings": 3,
  "scanRoot": "scripts",
  "maxWarningsByRule": {
    "PSAvoidUsingWriteHost": 2,
    "PSUseSingularNouns": 1
  }
}"#,
    );
    write_file(
        &repo_root.join("scripts/example.ps1"),
        "Write-Output 'example'\n",
    );
}

fn initialize_policy_command_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(&repo_root.join("README.md"), "# Repo\n");
    write_file(
        &repo_root.join("scripts/runtime/install.ps1"),
        "Write-Output 'install'\n",
    );
    write_file(&repo_root.join(".githooks/pre-commit"), "#!/bin/sh\n");
    write_file(&repo_root.join(".githooks/post-commit"), "#!/bin/sh\n");
    write_file(
        &repo_root.join(".github/policies/baseline.policy.json"),
        r#"{
  "id": "repository-baseline",
  "requiredFiles": ["README.md", "scripts/runtime/install.ps1"],
  "requiredDirectories": [".github/policies", ".githooks"],
  "forbiddenFiles": ["forbidden.txt"],
  "requiredGitHooks": ["pre-commit", "post-commit"]
}"#,
    );
}

fn initialize_release_governance_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(
        &repo_root.join("CHANGELOG.md"),
        r#"# Changelog

[2.0.0] - 2026-03-20
[1.9.0] - 2026-02-10
"#,
    );
    write_file(
        &repo_root.join("CODEOWNERS"),
        "* @example\n.github/ @example\n.githooks/ @example\nscripts/ @example\n",
    );
    write_file(
        &repo_root.join(".github/governance/release-governance.md"),
        r#"# Release Governance

## Scope

Scope.

## Branch Protection

Branch protection.

## CODEOWNERS

Owners.

## Release Checklist

Checklist.

## Rollback

Rollback.
"#,
    );
    write_file(
        &repo_root.join(".github/governance/branch-protection.baseline.json"),
        r#"{
  "schemaVersion": 1,
  "repository": "example/repo",
  "branch": "main",
  "protection": {
    "required_status_checks": {
      "strict": true,
      "contexts": [
        "Validate Instructions Runtime and Policies"
      ]
    },
    "enforce_admins": true,
    "required_pull_request_reviews": {
      "dismiss_stale_reviews": true,
      "require_code_owner_reviews": true,
      "required_approving_review_count": 1
    }
  }
}"#,
    );
}

fn initialize_release_provenance_repo_root(repo_root: &Path) {
    initialize_release_governance_repo_root(repo_root);
    write_file(
        &repo_root.join(".github/governance/release-provenance.baseline.json"),
        r#"{
  "version": 1,
  "releaseBranch": "main",
  "requireCleanWorktree": false,
  "warnOnDirtyWorktree": false,
  "requireAuditReport": false,
  "warnOnMissingOptionalAuditReport": false,
  "warnOnAuditCommitMismatch": true,
  "changelogPath": "CHANGELOG.md",
  "validateAllPath": "scripts/validation/validate-all.ps1",
  "requiredValidationChecks": [
    "validate-release-governance",
    "validate-release-provenance"
  ],
  "requiredEvidenceFiles": [
    "CHANGELOG.md",
    "CODEOWNERS",
    ".github/governance/release-governance.md",
    ".github/governance/release-provenance.baseline.json"
  ]
}"#,
    );
    write_file(
        &repo_root.join("scripts/validation/validate-all.ps1"),
        "$definitions = @(\n    @{ name = 'validate-release-governance' },\n    @{ name = 'validate-release-provenance' }\n)\n",
    );
}

fn initialize_compatibility_lifecycle_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(
        &repo_root.join("COMPATIBILITY.md"),
        r#"# Compatibility

## Support Lifecycle and EOL
Reference date for status labels in this table: **January 15, 2025**.

| Minor | GA date | Active support until | Maintenance support until | EOL date | Status |
| --- | --- | --- | --- | --- | --- |
| 1.2 | January 1, 2024 | February 1, 2025 | March 1, 2025 | March 2, 2025 | Active |
"#,
    );
}

fn initialize_dotnet_standards_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(
        &repo_root.join(".github/templates/dotnet-class-template.cs"),
        r#"namespace [Namespace];

/// <summary>
/// Template summary.
/// </summary>
public class [ClassName]
{
}
"#,
    );
    write_file(
        &repo_root.join(".github/templates/dotnet-interface-template.cs"),
        r#"namespace [Namespace];

/// <summary>
/// Template summary.
/// </summary>
public interface [InterfaceName]
{
}
"#,
    );
    write_file(
        &repo_root.join(".github/templates/dotnet-unit-test-template.cs"),
        r#"namespace [Namespace];

/// <summary>
/// Template summary.
/// </summary>
[TEST_CLASS]
public class [ClassName]
{
    [Fact]
    public void Test()
    {
    }
}
"#,
    );
    write_file(
        &repo_root.join(".github/templates/dotnet-integration-test-template.cs"),
        r#"namespace [Namespace];

/// <summary>
/// Template summary.
/// </summary>
public class [ClassName]
{
    private readonly IMediator _mediator;

    [Test]
    public void Test()
    {
    }
}
"#,
    );
}

fn initialize_authoritative_source_policy_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(
        &repo_root.join(".github/governance/authoritative-source-map.json"),
        r#"{
  "version": 1,
  "defaultPolicy": {
    "repositoryContextFirst": true
  },
  "stackRules": [
    { "id": "dotnet", "displayName": ".NET", "keywords": ["dotnet"], "officialDomains": ["learn.microsoft.com"] },
    { "id": "github-copilot", "displayName": "GitHub Copilot", "keywords": ["copilot"], "officialDomains": ["docs.github.com"] },
    { "id": "vscode", "displayName": "VS Code", "keywords": ["vscode"], "officialDomains": ["code.visualstudio.com"] },
    { "id": "rust", "displayName": "Rust", "keywords": ["rust"], "officialDomains": ["doc.rust-lang.org"] },
    { "id": "vue", "displayName": "Vue", "keywords": ["vue"], "officialDomains": ["vuejs.org"] },
    { "id": "quasar", "displayName": "Quasar", "keywords": ["quasar"], "officialDomains": ["quasar.dev"] },
    { "id": "docker", "displayName": "Docker", "keywords": ["docker"], "officialDomains": ["docs.docker.com"] },
    { "id": "kubernetes", "displayName": "Kubernetes", "keywords": ["kubernetes"], "officialDomains": ["kubernetes.io"] },
    { "id": "postgresql", "displayName": "PostgreSQL", "keywords": ["postgresql"], "officialDomains": ["postgresql.org"] },
    { "id": "openai", "displayName": "OpenAI", "keywords": ["openai"], "officialDomains": ["platform.openai.com"] }
  ]
}"#,
    );
    write_file(
        &repo_root.join(".github/instructions/authoritative-sources.instructions.md"),
        r#"# Authoritative Sources

Use `.github/governance/authoritative-source-map.json`.
Repository context first.
Use official documentation.
Use community sources only as fallback.
"#,
    );
    write_file(
        &repo_root.join(".github/AGENTS.md"),
        r#"# AGENTS

Use `instructions/authoritative-sources.instructions.md`.
Use `.github/governance/authoritative-source-map.json`.
"#,
    );
    write_file(
        &repo_root.join(".github/copilot-instructions.md"),
        r#"# Global Instructions

Use `instructions/authoritative-sources.instructions.md`.
Use `.github/governance/authoritative-source-map.json`.
"#,
    );
    write_file(
        &repo_root.join(".github/instruction-routing.catalog.yml"),
        "always:\n  - path: instructions/authoritative-sources.instructions.md\n",
    );
}

fn initialize_instruction_architecture_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(
        &repo_root.join(".github/governance/instruction-ownership.manifest.json"),
        r#"{
  "version": 1,
  "intentionalGlobalExceptions": [
    {
      "concern": "Global context must remain visible.",
      "ownedBy": "global-core"
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
        "instructions/super-agent.instructions.md",
        "instructions/repository-operating-model.instructions.md",
        "instructions/artifact-layout.instructions.md",
        "instructions/subagent-planning-workflow.instructions.md",
        "instructions/workflow-optimization.instructions.md",
        "instructions/authoritative-sources.instructions.md",
        "instructions/powershell-execution.instructions.md",
        "instructions/feedback-changelog.instructions.md"
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
      "id": "repository-operating-model",
      "pathPatterns": [
        ".github/instructions/repository-operating-model.instructions.md"
      ]
    },
    {
      "id": "cross-cutting-policies",
      "pathPatterns": [
        ".github/instructions/super-agent.instructions.md",
        ".github/instructions/authoritative-sources.instructions.md",
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
        ".github/instructions/authoritative-sources.instructions.md",
        ".github/instructions/super-agent.instructions.md",
        ".github/instructions/repository-operating-model.instructions.md"
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
}"#,
    );
    write_file(
        &repo_root.join(".github/AGENTS.md"),
        r#"# AGENTS

Use `instructions/repository-operating-model.instructions.md`.
Use `instructions/authoritative-sources.instructions.md`.
"#,
    );
    write_file(
        &repo_root.join(".github/copilot-instructions.md"),
        r#"# Global Instructions

Use `instructions/repository-operating-model.instructions.md`.
Use `instructions/authoritative-sources.instructions.md`.
"#,
    );
    write_file(
        &repo_root.join(".github/instruction-routing.catalog.yml"),
        r#"always:
  - path: AGENTS.md
  - path: copilot-instructions.md
  - path: instructions/super-agent.instructions.md
  - path: instructions/repository-operating-model.instructions.md
  - path: instructions/artifact-layout.instructions.md
  - path: instructions/subagent-planning-workflow.instructions.md
  - path: instructions/workflow-optimization.instructions.md
  - path: instructions/authoritative-sources.instructions.md
  - path: instructions/powershell-execution.instructions.md
  - path: instructions/feedback-changelog.instructions.md
"#,
    );
    write_file(
        &repo_root.join(".github/prompts/route-instructions.prompt.md"),
        r#"---
description: Route a request
mode: ask
tools: ['readFile']
---

# Route Instructions

Hard cap: at most 5 selected instruction files (excluding mandatory).
"#,
    );
    write_file(
        &repo_root.join(".github/prompts/example.prompt.md"),
        r#"---
description: Example prompt
mode: ask
tools: ['readFile']
---

# Example Prompt

Use the routing catalog.
"#,
    );
    write_file(
        &repo_root.join(".github/templates/example.md"),
        "# Example Template\n\nUse this as a reusable artifact.\n",
    );
    write_file(
        &repo_root.join(".github/policies/example.policy.md"),
        "# Example Policy\n",
    );
    write_file(
        &repo_root.join(".github/instructions/repository-operating-model.instructions.md"),
        "# Repository Operating Model\n",
    );
    write_file(
        &repo_root.join(".github/instructions/authoritative-sources.instructions.md"),
        "# Authoritative Sources\n",
    );
    write_file(
        &repo_root.join(".github/instructions/super-agent.instructions.md"),
        "# Super Agent\n",
    );
    write_file(
        &repo_root.join(".github/instructions/artifact-layout.instructions.md"),
        "# Artifact Layout\n",
    );
    write_file(
        &repo_root.join(".github/instructions/subagent-planning-workflow.instructions.md"),
        "# Subagent Planning Workflow\n",
    );
    write_file(
        &repo_root.join(".github/instructions/workflow-optimization.instructions.md"),
        "# Workflow Optimization\n",
    );
    write_file(
        &repo_root.join(".github/instructions/powershell-execution.instructions.md"),
        "# PowerShell Execution\n",
    );
    write_file(
        &repo_root.join(".github/instructions/feedback-changelog.instructions.md"),
        "# Feedback Changelog\n",
    );
    write_file(
        &repo_root.join(".github/instructions/example-domain.instructions.md"),
        "# Domain Instruction\n",
    );
    write_file(
        &repo_root.join(".codex/skills/sample/SKILL.md"),
        r#"---
name: sample-skill
description: sample skill
---

# Sample Skill

Load `repository-operating-model.instructions.md`.
"#,
    );
    write_file(
        &repo_root.join("scripts/orchestration/example.ps1"),
        "Write-Host 'orchestration'\n",
    );
    write_file(
        &repo_root.join("scripts/runtime/bootstrap.ps1"),
        "Write-Host 'runtime'\n",
    );
}

#[allow(dead_code)]
fn initialize_git_repository(repo_root: &Path) -> String {
    run_git(repo_root, &["init", "-b", "main"]);
    run_git(
        repo_root,
        &["config", "user.email", "fixtures@example.invalid"],
    );
    run_git(repo_root, &["config", "user.name", "Fixture User"]);
    run_git(repo_root, &["add", "."]);
    run_git(repo_root, &["commit", "-m", "Initial release fixtures"]);

    run_git_capture(repo_root, &["rev-parse", "HEAD"])
}

fn write_audit_report(repo_root: &Path, relative_path: &str, commit: &str, overall_status: &str) {
    write_file(
        &repo_root.join(relative_path),
        &format!(
            r#"{{
  "generatedAt": "2026-03-27T13:43:00Z",
  "summary": {{
    "overallStatus": "{overall_status}"
  }},
  "git": {{
    "commit": "{commit}"
  }}
}}"#
        ),
    );
}

fn run_git(repo_root: &Path, arguments: &[&str]) {
    let status = ProcessCommand::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(arguments)
        .status()
        .expect("git command should start");
    assert!(
        status.success(),
        "git command should succeed: {:?}",
        arguments
    );
}

fn run_git_capture(repo_root: &Path, arguments: &[&str]) -> String {
    let output = ProcessCommand::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(arguments)
        .output()
        .expect("git command should start");
    assert!(
        output.status.success(),
        "git command should succeed: {:?}",
        arguments
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn initialize_agent_skill(repo_root: &Path, skill_name: &str) {
    write_file(
        &repo_root.join(format!(".codex/skills/{skill_name}/SKILL.md")),
        &format!(
            "---\nname: {skill_name}\n---\nReference .github/AGENTS.md\nReference .github/copilot-instructions.md\nReference .github/instruction-routing.catalog.yml\nReference .github/instructions/repository-operating-model.instructions.md\n"
        ),
    );
    write_file(
        &repo_root.join(format!(".codex/skills/{skill_name}/agents/openai.yaml")),
        "name: openai\n",
    );
}

fn valid_agents_manifest_json() -> &'static str {
    r#"{
  "version": 1,
  "agents": [
    {
      "id": "super-agent",
      "role": "planner",
      "skill": "super-agent",
      "allowedPaths": [".github/**", ".codex/**", "planning/**", "scripts/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 16, "maxDurationMinutes": 15, "maxFileEdits": 8, "maxTokens": 45000 },
      "fallbackAgentId": "planner"
    },
    {
      "id": "brainstormer",
      "role": "planner",
      "skill": "brainstorm-spec-architect",
      "allowedPaths": [".github/**", ".codex/**", "planning/**", "scripts/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 18, "maxDurationMinutes": 20, "maxFileEdits": 10, "maxTokens": 55000 },
      "fallbackAgentId": "planner"
    },
    {
      "id": "planner",
      "role": "planner",
      "skill": "plan-active-work-planner",
      "allowedPaths": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 20, "maxDurationMinutes": 20, "maxFileEdits": 12, "maxTokens": 70000 },
      "fallbackAgentId": "router"
    },
    {
      "id": "router",
      "role": "router",
      "skill": "context-token-optimizer",
      "allowedPaths": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 15, "maxDurationMinutes": 15, "maxFileEdits": 8, "maxTokens": 50000 },
      "fallbackAgentId": "specialist"
    },
    {
      "id": "specialist",
      "role": "specialist",
      "skill": "dev-software-engineer",
      "allowedPaths": ["src/**", "modules/**", "samples/**", "tests/**", "scripts/**", ".github/**", ".codex/**", ".temp/**", "README.md", "CHANGELOG.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "approvalRequired": true,
      "approvalInstructions": "approval required",
      "budget": { "maxSteps": 45, "maxDurationMinutes": 50, "maxFileEdits": 40, "maxTokens": 180000 },
      "fallbackAgentId": "tester"
    },
    {
      "id": "tester",
      "role": "tester",
      "skill": "test-engineer",
      "allowedPaths": ["tests/**", "src/**", "modules/**", "samples/**", "scripts/**", ".github/**", ".temp/**"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 25, "maxDurationMinutes": 30, "maxFileEdits": 20, "maxTokens": 120000 },
      "fallbackAgentId": "reviewer"
    },
    {
      "id": "reviewer",
      "role": "reviewer",
      "skill": "review-code-engineer",
      "allowedPaths": ["src/**", "modules/**", "samples/**", "planning/**", "scripts/**", ".github/**", ".codex/**", ".temp/**", "README.md", "CHANGELOG.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 20, "maxDurationMinutes": 25, "maxFileEdits": 10, "maxTokens": 90000 },
      "fallbackAgentId": "release-engineer"
    },
    {
      "id": "release-engineer",
      "role": "release",
      "skill": "release-closeout-engineer",
      "allowedPaths": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "approvalRequired": true,
      "approvalInstructions": "approval required",
      "budget": { "maxSteps": 20, "maxDurationMinutes": 20, "maxFileEdits": 15, "maxTokens": 80000 }
    }
  ]
}"#
}

fn valid_permission_matrix_json() -> &'static str {
    r#"{
  "version": 1,
  "defaultWarningOnly": true,
  "globalRules": {
    "requiredBlockedCommandPrefixes": ["git reset --hard", "git checkout --"],
    "allowedStageScriptPrefixes": ["scripts/orchestration/stages/"]
  },
  "agents": [
    { "agentId": "super-agent", "role": "planner", "skill": "super-agent", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/intake-stage.ps1"], "requiredBudget": { "maxSteps": 16, "maxDurationMinutes": 15, "maxFileEdits": 8, "maxTokens": 45000 } },
    { "agentId": "brainstormer", "role": "planner", "skill": "brainstorm-spec-architect", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/spec-stage.ps1"], "requiredBudget": { "maxSteps": 18, "maxDurationMinutes": 20, "maxFileEdits": 10, "maxTokens": 55000 } },
    { "agentId": "planner", "role": "planner", "skill": "plan-active-work-planner", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/plan-stage.ps1"], "requiredBudget": { "maxSteps": 20, "maxDurationMinutes": 20, "maxFileEdits": 12, "maxTokens": 70000 } },
    { "agentId": "router", "role": "router", "skill": "context-token-optimizer", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/route-stage.ps1"], "requiredBudget": { "maxSteps": 15, "maxDurationMinutes": 15, "maxFileEdits": 8, "maxTokens": 50000 } },
    { "agentId": "specialist", "role": "specialist", "skill": "dev-software-engineer", "allowedPathGlobs": ["src/**", "modules/**", "samples/**", "tests/**", "scripts/**", ".github/**", ".codex/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/implement-stage.ps1"], "requiredBudget": { "maxSteps": 45, "maxDurationMinutes": 50, "maxFileEdits": 40, "maxTokens": 180000 } },
    { "agentId": "tester", "role": "tester", "skill": "test-engineer", "allowedPathGlobs": ["tests/**", "src/**", "modules/**", "samples/**", "scripts/**", ".github/**", ".temp/**"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/validate-stage.ps1"], "requiredBudget": { "maxSteps": 25, "maxDurationMinutes": 30, "maxFileEdits": 20, "maxTokens": 120000 } },
    { "agentId": "reviewer", "role": "reviewer", "skill": "review-code-engineer", "allowedPathGlobs": ["src/**", "modules/**", "samples/**", "planning/**", "scripts/**", ".github/**", ".codex/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/review-stage.ps1"], "requiredBudget": { "maxSteps": 20, "maxDurationMinutes": 25, "maxFileEdits": 10, "maxTokens": 90000 } },
    { "agentId": "release-engineer", "role": "release", "skill": "release-closeout-engineer", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/closeout-stage.ps1"], "requiredBudget": { "maxSteps": 20, "maxDurationMinutes": 20, "maxFileEdits": 15, "maxTokens": 80000 } }
  ]
}"#
}

fn valid_pipeline_manifest_json() -> &'static str {
    r#"{
  "id": "default-dev-flow",
  "version": 1,
  "description": "Validation fixture for agent orchestration.",
  "runtime": {
    "policyCatalogPath": ".github/governance/agent-runtime-policy.catalog.json",
    "modelRoutingCatalogPath": ".github/governance/agent-model-routing.catalog.json"
  },
  "stages": [
    { "id": "intake", "agentId": "super-agent", "mode": "plan", "execution": { "scriptPath": "scripts/orchestration/stages/intake-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/super-agent-intake-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-intake-result.schema.json" }, "inputArtifacts": ["request"], "outputArtifacts": ["normalized-request", "intake-report"], "onFailure": "retry-once" },
    { "id": "spec", "agentId": "brainstormer", "mode": "plan", "execution": { "scriptPath": "scripts/orchestration/stages/spec-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/spec-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-spec-result.schema.json" }, "inputArtifacts": ["request", "normalized-request", "intake-report"], "outputArtifacts": ["spec-summary", "active-spec"], "onFailure": "retry-once" },
    { "id": "plan", "agentId": "planner", "mode": "plan", "execution": { "scriptPath": "scripts/orchestration/stages/plan-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/planner-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-plan-result.schema.json" }, "inputArtifacts": ["request", "normalized-request", "intake-report", "spec-summary", "active-spec"], "outputArtifacts": ["task-plan", "task-plan-data", "context-pack", "active-plan"], "onFailure": "stop" },
    { "id": "route", "agentId": "router", "mode": "execute", "execution": { "scriptPath": "scripts/orchestration/stages/route-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/router-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-route-result.schema.json" }, "inputArtifacts": ["task-plan-data", "context-pack", "active-plan"], "outputArtifacts": ["route-selection", "specialist-context-pack"], "onFailure": "retry-once" },
    { "id": "implement", "agentId": "specialist", "mode": "execute", "execution": { "scriptPath": "scripts/orchestration/stages/implement-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/executor-task.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-implementation-result.schema.json" }, "inputArtifacts": ["task-plan", "task-plan-data", "context-pack", "route-selection", "specialist-context-pack", "active-plan"], "outputArtifacts": ["changeset", "implementation-log", "task-review-report"], "onFailure": "retry-once" },
    { "id": "validate", "agentId": "tester", "mode": "validate", "execution": { "scriptPath": "scripts/orchestration/stages/validate-stage.ps1", "dispatchMode": "scripted" }, "inputArtifacts": ["changeset", "implementation-log", "task-review-report"], "outputArtifacts": ["validation-report"], "onFailure": "retry-once" },
    { "id": "review", "agentId": "reviewer", "mode": "review", "execution": { "scriptPath": "scripts/orchestration/stages/review-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/reviewer-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-review-result.schema.json" }, "inputArtifacts": ["changeset", "validation-report", "task-review-report", "active-plan"], "outputArtifacts": ["review-report", "decision-log"], "onFailure": "stop" },
    { "id": "closeout", "agentId": "release-engineer", "mode": "review", "execution": { "scriptPath": "scripts/orchestration/stages/closeout-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/closeout-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-closeout-result.schema.json" }, "inputArtifacts": ["changeset", "validation-report", "review-report", "decision-log", "active-plan"], "outputArtifacts": ["closeout-report", "release-summary", "completed-plan"], "onFailure": "stop" }
  ],
  "handoffs": [
    { "fromStage": "intake", "toStage": "spec", "requiredArtifacts": ["normalized-request", "intake-report"] },
    { "fromStage": "spec", "toStage": "plan", "requiredArtifacts": ["spec-summary", "active-spec"] },
    { "fromStage": "plan", "toStage": "route", "requiredArtifacts": ["task-plan-data", "context-pack", "active-plan"] },
    { "fromStage": "route", "toStage": "implement", "requiredArtifacts": ["route-selection", "specialist-context-pack"] },
    { "fromStage": "implement", "toStage": "validate", "requiredArtifacts": ["changeset", "implementation-log", "task-review-report"] },
    { "fromStage": "validate", "toStage": "review", "requiredArtifacts": ["validation-report"] },
    { "fromStage": "review", "toStage": "closeout", "requiredArtifacts": ["review-report", "decision-log"] }
  ],
  "completionCriteria": {
    "requiredStages": ["intake", "spec", "plan", "route", "implement", "validate", "review", "closeout"],
    "requiredArtifacts": ["intake-report", "spec-summary", "validation-report", "review-report", "decision-log", "closeout-report", "release-summary"]
  }
}"#
}

fn valid_eval_fixtures_json() -> &'static str {
    r#"{
  "version": 1,
  "cases": [
    {
      "id": "feature-implementation",
      "expectedPipelineId": "default-dev-flow",
      "expectedStageOrder": ["intake", "spec", "plan", "route", "implement", "validate", "review", "closeout"],
      "requiredAgents": ["super-agent", "brainstormer", "planner", "router", "specialist", "tester", "reviewer", "release-engineer"]
    }
  ]
}"#
}

fn valid_handoff_template_json() -> &'static str {
    r#"{
  "fromStage": "plan",
  "toStage": "implement",
  "artifacts": [
    { "name": "task-plan", "path": ".temp/task-plan.md", "checksum": "sha256:replace-me" }
  ]
}"#
}

fn valid_run_artifact_template_json() -> &'static str {
    r#"{
  "stages": [
    { "stageId": "intake", "agentId": "super-agent" },
    { "stageId": "spec", "agentId": "brainstormer" },
    { "stageId": "plan", "agentId": "planner" },
    { "stageId": "route", "agentId": "router" },
    { "stageId": "implement", "agentId": "specialist" },
    { "stageId": "validate", "agentId": "tester" },
    { "stageId": "review", "agentId": "reviewer" },
    { "stageId": "closeout", "agentId": "release-engineer" }
  ],
  "summary": {
    "stageCount": 8
  }
}"#
}

fn initialize_agent_contract_command_repo_root(repo_root: &Path) {
    initialize_validation_repo_root(repo_root);
    write_file(&repo_root.join(".github/AGENTS.md"), "# Agents\n");
    write_file(
        &repo_root.join(".github/copilot-instructions.md"),
        "# Copilot Instructions\n",
    );
    write_file(
        &repo_root.join(".github/instruction-routing.catalog.yml"),
        "version: 1\nroutes: []\n",
    );
    write_file(
        &repo_root.join(".github/instructions/repository-operating-model.instructions.md"),
        "# Repository Operating Model\n",
    );
    write_file(
        &repo_root.join(".github/governance/agent-runtime-policy.catalog.json"),
        r#"{ "version": 1, "rules": [] }"#,
    );
    write_file(
        &repo_root.join(".github/governance/agent-model-routing.catalog.json"),
        r#"{ "version": 1, "rules": [] }"#,
    );
    write_file(
        &repo_root.join(".codex/orchestration/agents.manifest.json"),
        valid_agents_manifest_json(),
    );
    write_file(
        &repo_root.join(".github/governance/agent-skill-permissions.matrix.json"),
        valid_permission_matrix_json(),
    );
    write_file(
        &repo_root.join(".codex/orchestration/pipelines/default.pipeline.json"),
        valid_pipeline_manifest_json(),
    );
    write_file(
        &repo_root.join(".codex/orchestration/evals/golden-tests.json"),
        valid_eval_fixtures_json(),
    );

    for skill_name in [
        "super-agent",
        "brainstorm-spec-architect",
        "plan-active-work-planner",
        "context-token-optimizer",
        "dev-software-engineer",
        "test-engineer",
        "review-code-engineer",
        "release-closeout-engineer",
    ] {
        initialize_agent_skill(repo_root, skill_name);
    }

    for relative_path in [
        "scripts/common/agent-runtime-hardening.ps1",
        "scripts/runtime/run-agent-pipeline.ps1",
        "scripts/runtime/resume-agent-pipeline.ps1",
        "scripts/runtime/replay-agent-run.ps1",
        "scripts/runtime/evaluate-agent-pipeline.ps1",
        "scripts/orchestration/engine/invoke-codex-dispatch.ps1",
        "scripts/orchestration/engine/invoke-task-worker.ps1",
        "scripts/orchestration/stages/intake-stage.ps1",
        "scripts/orchestration/stages/spec-stage.ps1",
        "scripts/orchestration/stages/plan-stage.ps1",
        "scripts/orchestration/stages/route-stage.ps1",
        "scripts/orchestration/stages/implement-stage.ps1",
        "scripts/orchestration/stages/validate-stage.ps1",
        "scripts/orchestration/stages/review-stage.ps1",
        "scripts/orchestration/stages/closeout-stage.ps1",
    ] {
        write_file(&repo_root.join(relative_path), "Write-Output 'ok'\n");
    }

    for relative_path in [
        ".codex/orchestration/prompts/super-agent-intake-stage.prompt.md",
        ".codex/orchestration/prompts/spec-stage.prompt.md",
        ".codex/orchestration/prompts/planner-stage.prompt.md",
        ".codex/orchestration/prompts/router-stage.prompt.md",
        ".codex/orchestration/prompts/executor-task.prompt.md",
        ".codex/orchestration/prompts/task-spec-review.prompt.md",
        ".codex/orchestration/prompts/task-quality-review.prompt.md",
        ".codex/orchestration/prompts/reviewer-stage.prompt.md",
        ".codex/orchestration/prompts/closeout-stage.prompt.md",
    ] {
        write_file(&repo_root.join(relative_path), "# Prompt\n");
    }

    for relative_path in [
        ".codex/orchestration/templates/trace-record.template.json",
        ".codex/orchestration/templates/policy-evaluations.template.json",
        ".codex/orchestration/templates/checkpoint-state.template.json",
    ] {
        write_file(&repo_root.join(relative_path), r#"{ "type": "object" }"#);
    }
    write_file(
        &repo_root.join(".codex/orchestration/templates/handoff.template.json"),
        valid_handoff_template_json(),
    );
    write_file(
        &repo_root.join(".codex/orchestration/templates/run-artifact.template.json"),
        valid_run_artifact_template_json(),
    );

    for relative_path in [
        ".github/schemas/agent.contract.schema.json",
        ".github/schemas/agent.pipeline.schema.json",
        ".github/schemas/agent.handoff.schema.json",
        ".github/schemas/agent.run-artifact.schema.json",
        ".github/schemas/agent.evals.schema.json",
        ".github/schemas/agent.stage-intake-result.schema.json",
        ".github/schemas/agent.stage-spec-result.schema.json",
        ".github/schemas/agent.stage-plan-result.schema.json",
        ".github/schemas/agent.stage-route-result.schema.json",
        ".github/schemas/agent.stage-implementation-result.schema.json",
        ".github/schemas/agent.stage-review-result.schema.json",
        ".github/schemas/agent.stage-closeout-result.schema.json",
        ".github/schemas/agent.task-review-result.schema.json",
        ".github/schemas/agent.trace-record.schema.json",
        ".github/schemas/agent.policy-evaluation.schema.json",
        ".github/schemas/agent.checkpoint-state.schema.json",
    ] {
        write_file(&repo_root.join(relative_path), r#"{ "type": "object" }"#);
    }
}

#[test]
fn test_validation_audit_ledger_reports_pass_for_missing_ledger() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validation_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "audit-ledger"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Entries checked: 0"));
}

#[test]
fn test_validation_architecture_boundaries_reports_pass_for_matching_baseline() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validation_repo_root(repo.path());
    write_file(
        &repo
            .path()
            .join(".github/governance/architecture-boundaries.baseline.json"),
        r#"{
  "rules": [
    {
      "id": "readme-contract",
      "files": ["README.md"],
      "requiredPatterns": ["Native validation boundary"],
      "severity": "failure"
    }
  ]
}"#,
    );
    write_file(
        &repo.path().join("README.md"),
        "# Demo\n\nNative validation boundary is documented here.\n",
    );

    ntk()
        .current_dir(repo.path())
        .args(["validation", "architecture-boundaries"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Rules checked: 1"));
}

#[test]
fn test_validation_authoritative_source_policy_reports_pass_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_authoritative_source_policy_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "validation",
            "authoritative-source-policy",
            "--warning-only",
            "false",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Stack rules checked: 10"))
        .stdout(predicate::str::contains("Instruction files scanned: 0"));
}

#[test]
fn test_validation_authoritative_source_policy_reports_failure_for_invalid_agents_reference() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_authoritative_source_policy_repo_root(repo.path());
    write_file(
        &repo.path().join(".github/AGENTS.md"),
        "# Temporary AGENTS\n\nThis file intentionally omits the required reference.\n",
    );

    ntk()
        .current_dir(repo.path())
        .args([
            "validation",
            "authoritative-source-policy",
            "--warning-only",
            "false",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("Status: failed"))
        .stdout(predicate::str::contains(
            "AGENTS.md is missing required pattern",
        ));
}

#[test]
fn test_validation_instruction_architecture_reports_pass_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_instruction_architecture_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "validation",
            "instruction-architecture",
            "--warning-only",
            "false",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Layers checked: 9"))
        .stdout(predicate::str::contains("Prompt files scanned: 2"))
        .stdout(predicate::str::contains("Template files scanned: 1"))
        .stdout(predicate::str::contains("Skill files scanned: 1"));
}

#[test]
fn test_validation_instruction_architecture_reports_failure_for_missing_route_hard_cap() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_instruction_architecture_repo_root(repo.path());
    write_file(
        &repo
            .path()
            .join(".github/prompts/route-instructions.prompt.md"),
        r#"---
description: Temporary route prompt
mode: ask
tools: ['readFile']
---

# Route Instructions

Use the routing catalog and return JSON.
"#,
    );

    ntk()
        .current_dir(repo.path())
        .args([
            "validation",
            "instruction-architecture",
            "--warning-only",
            "false",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("Status: failed"))
        .stdout(predicate::str::contains(
            "Route prompt is missing deterministic hard-cap text",
        ));
}

#[test]
fn test_validation_compatibility_lifecycle_policy_reports_pass_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_compatibility_lifecycle_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "validation",
            "compatibility-lifecycle-policy",
            "--warning-only",
            "false",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Reference date: 2025-01-15"))
        .stdout(predicate::str::contains("Rows checked: 1"));
}

#[test]
fn test_validation_dotnet_standards_reports_pass_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_dotnet_standards_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "dotnet-standards"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Templates checked: 4"));
}

#[test]
fn test_validation_routing_coverage_reports_pass_for_matching_catalog_and_fixture() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validation_repo_root(repo.path());
    write_file(
        &repo.path().join(".github/instruction-routing.catalog.yml"),
        r#"version: 1
routing:
  - id: docs
    include:
      - path: instructions/readme.instructions.md
"#,
    );
    write_file(
        &repo
            .path()
            .join(".github/instructions/readme.instructions.md"),
        "# readme",
    );
    write_file(
        &repo
            .path()
            .join("scripts/validation/fixtures/routing-golden-tests.json"),
        r#"{
  "cases": [
    {
      "id": "docs-route",
      "expected_route_ids": ["docs"],
      "expected_selected_paths": ["instructions/readme.instructions.md"]
    }
  ]
}"#,
    );

    ntk()
        .current_dir(repo.path())
        .args(["validation", "routing-coverage"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Routes checked: 1"))
        .stdout(predicate::str::contains("Cases checked: 1"));
}

#[test]
fn test_validation_security_baseline_reports_pass_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_security_baseline_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "security-baseline", "--warning-only", "false"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Files scanned: 4"));
}

#[test]
fn test_validation_security_baseline_reports_warning_for_allowlisted_first_match_and_real_secret_later(
) {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_security_baseline_repo_root(repo.path());
    write_file(
        &repo.path().join("docs/notes.md"),
        "password = \"example-password\"\npassword = \"supersecret1\"\n",
    );

    ntk()
        .current_dir(repo.path())
        .args(["validation", "security-baseline", "--warning-only", "false"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: warning"))
        .stdout(predicate::str::contains("supersecret1"));
}

#[test]
fn test_validation_powershell_standards_reports_pass_for_valid_scripts() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_powershell_standards_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "validation",
            "powershell-standards",
            "--skip-script-analyzer",
            "--warning-only",
            "false",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Files checked: 1"));
}

#[test]
fn test_validation_shared_script_checksums_reports_pass_for_valid_manifest() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_shared_script_checksums_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "validation",
            "shared-script-checksums",
            "--warning-only",
            "false",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Manifest entries: 2"))
        .stdout(predicate::str::contains("Current entries: 2"));
}

#[test]
fn test_validation_supply_chain_reports_pass_for_valid_manifests() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_supply_chain_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "supply-chain", "--warning-only", "false"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Dependency manifests: 4"))
        .stdout(predicate::str::contains("Packages discovered: 5"))
        .stdout(predicate::str::contains("SBOM path:"));
}

#[test]
fn test_validation_supply_chain_fails_when_required_license_evidence_path_is_missing() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_supply_chain_repo_root(repo.path());
    write_file(
        &repo
            .path()
            .join(".github/governance/supply-chain.baseline.json"),
        r#"{
  "version": 1,
  "sbomOutputPath": ".temp/audit/sbom.latest.json",
  "requireLicenseEvidence": true,
  "warnOnMissingLicenseEvidence": false,
  "warnOnEmptyDependencySet": false,
  "excludedPathGlobs": [
    ".git/**",
    ".temp/**",
    "**/bin/**",
    "**/obj/**",
    "**/.vs/**"
  ],
  "blockedDependencyPatterns": [],
  "sensitiveDependencyPatterns": []
}"#,
    );

    ntk()
        .current_dir(repo.path())
        .args(["validation", "supply-chain", "--warning-only", "false"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("Status: failed"))
        .stdout(predicate::str::contains(
            "License evidence path is required but missing or empty.",
        ));
}

#[test]
fn test_validation_warning_baseline_reports_pass_for_matching_report() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_warning_baseline_repo_root(repo.path());
    write_file(
        &repo.path().join(".temp/audit/analyzer-warning-report.json"),
        &json!([
            {
                "RuleName": "PSAvoidUsingWriteHost",
                "ScriptPath": "scripts/example.ps1"
            }
        ])
        .to_string(),
    );

    ntk()
        .current_dir(repo.path())
        .args([
            "validation",
            "warning-baseline",
            "--analyzer-report-path",
            ".temp/audit/analyzer-warning-report.json",
            "--warning-only",
            "false",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Total warnings: 1"));
}

#[test]
fn test_validation_policy_reports_pass_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_policy_command_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "policy"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Policies checked: 1"));
}

#[test]
fn test_validation_agent_skill_alignment_reports_pass_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_command_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "agent-skill-alignment"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Agents checked: 8"))
        .stdout(predicate::str::contains("Stage checks: 8"))
        .stdout(predicate::str::contains("Eval case checks: 1"));
}

#[test]
fn test_validation_agent_permissions_reports_pass_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_command_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "agent-permissions", "--warning-only", "false"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Agents checked: 8"))
        .stdout(predicate::str::contains("Stage checks: 8"));
}

#[test]
fn test_validation_agent_orchestration_reports_pass_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_command_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "agent-orchestration"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Agents checked: 8"))
        .stdout(predicate::str::contains("Stage checks: 8"));
}

#[test]
fn test_validation_release_governance_reports_pass_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_governance_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "validation",
            "release-governance",
            "--warning-only",
            "false",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Latest changelog version: 2.0.0"));
}

#[test]
fn test_validation_release_provenance_reports_pass_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_provenance_repo_root(repo.path());
    let head_commit = initialize_git_repository(repo.path());
    write_audit_report(
        repo.path(),
        ".temp/audit-report.json",
        &head_commit,
        "passed",
    );

    ntk()
        .current_dir(repo.path())
        .args([
            "validation",
            "release-provenance",
            "--warning-only",
            "false",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Checks declared: 2"))
        .stdout(predicate::str::contains("Checks found in validate-all: 2"))
        .stdout(predicate::str::contains("Git available: true"));
}

#[test]
fn test_validation_release_provenance_require_audit_report_reports_warning_when_missing() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_release_provenance_repo_root(repo.path());
    initialize_git_repository(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "release-provenance", "--require-audit-report"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: warning"))
        .stdout(predicate::str::contains("Require audit report: true"))
        .stdout(predicate::str::contains("Required audit report not found"));
}

#[test]
fn test_validation_policy_reports_pass_for_valid_policy_directory() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_policy_command_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "policy"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Policies checked: 1"));
}

#[test]
fn test_validation_agent_skill_alignment_reports_pass_for_valid_contract_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_command_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "agent-skill-alignment"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Agents checked: 8"))
        .stdout(predicate::str::contains("Stage checks: 8"))
        .stdout(predicate::str::contains("Eval case checks: 1"));
}

#[test]
fn test_validation_agent_permissions_reports_pass_for_valid_contract_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_command_repo_root(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["validation", "agent-permissions", "--warning-only", "false"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Agents checked: 8"))
        .stdout(predicate::str::contains("Stage checks: 8"));
}
