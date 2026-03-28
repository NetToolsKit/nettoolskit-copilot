//! Tests for executable validation command surfaces exposed by `ntk`.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
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
