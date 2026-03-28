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
