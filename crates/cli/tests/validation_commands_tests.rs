//! Tests for executable validation command surfaces exposed by `ntk`.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use std::fs;
use std::path::Path;
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
    write_file(&repo_root.join("scripts/common/a.ps1"), "Write-Output 'a'\n");
    write_file(&repo_root.join("scripts/security/b.ps1"), "Write-Output 'b'\n");
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
    write_file(&repo_root.join("scripts/example.ps1"), "Write-Output 'example'\n");
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
        .args([
            "validation",
            "security-baseline",
            "--warning-only",
            "false",
        ])
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
        .args([
            "validation",
            "security-baseline",
            "--warning-only",
            "false",
        ])
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
        &repo.path().join(".github/governance/supply-chain.baseline.json"),
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
