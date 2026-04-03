//! Tests for `validate-routing-coverage`.

use nettoolskit_validation::{
    invoke_validate_routing_coverage, ValidateRoutingCoverageRequest, ValidationCheckStatus,
};
use std::fs;
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_repo_layout(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
}

#[test]
fn test_invoke_validate_routing_coverage_passes_for_valid_catalog_and_fixtures() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo.path().join(".github/instruction-routing.catalog.yml"),
        r#"version: 1
routing:
  - id: docs
    include:
      - path: instructions/docs/ntk-docs-readme.instructions.md
"#,
    );
    write_file(
        &repo
            .path()
            .join(".github/instructions/docs/ntk-docs-readme.instructions.md"),
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
      "expected_selected_paths": ["instructions/docs/ntk-docs-readme.instructions.md"]
    }
  ]
}"#,
    );

    let result = invoke_validate_routing_coverage(&ValidateRoutingCoverageRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateRoutingCoverageRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.routes_checked, 1);
    assert_eq!(result.cases_checked, 1);
}

#[test]
fn test_invoke_validate_routing_coverage_reports_missing_coverage_and_unknown_routes() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo.path().join(".github/instruction-routing.catalog.yml"),
        r#"version: 1
routing:
  - id: docs
    include:
      - path: instructions/docs/ntk-docs-readme.instructions.md
  - id: rust
    include:
      - path: instructions/rust.instructions.md
"#,
    );
    write_file(
        &repo
            .path()
            .join(".github/instructions/docs/ntk-docs-readme.instructions.md"),
        "# readme",
    );
    write_file(
        &repo
            .path()
            .join(".github/instructions/rust.instructions.md"),
        "# rust",
    );
    write_file(
        &repo
            .path()
            .join("scripts/validation/fixtures/routing-golden-tests.json"),
        r#"{
  "cases": [
    {
      "id": "broken-route",
      "expected_route_ids": ["missing"],
      "expected_selected_paths": ["instructions/docs/ntk-docs-readme.instructions.md"]
    }
  ]
}"#,
    );

    let result = invoke_validate_routing_coverage(&ValidateRoutingCoverageRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateRoutingCoverageRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("references unknown route id: missing")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Catalog route without fixture coverage: docs")));
}

#[test]
fn test_invoke_validate_routing_coverage_converts_required_findings_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo.path().join(".github/instruction-routing.catalog.yml"),
        r#"version: 1
routing:
  - id: docs
    include:
      - path: instructions/docs/ntk-docs-readme.instructions.md
"#,
    );
    write_file(
        &repo
            .path()
            .join(".github/instructions/docs/ntk-docs-readme.instructions.md"),
        "# readme",
    );
    write_file(
        &repo
            .path()
            .join("scripts/validation/fixtures/routing-golden-tests.json"),
        r#"{
  "cases": [
    {
      "id": "no-routes",
      "expected_route_ids": [],
      "expected_selected_paths": ["instructions/docs/ntk-docs-readme.instructions.md"]
    }
  ]
}"#,
    );

    let result = invoke_validate_routing_coverage(&ValidateRoutingCoverageRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateRoutingCoverageRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("has expected_selected_paths but no expected_route_ids")));
}
