//! Tests for `validate-instructions`.

use nettoolskit_validation::{
    ValidateInstructionsRequest, ValidationCheckStatus, invoke_validate_instructions,
};
use std::fs;
use tempfile::TempDir;

use crate::support::instruction_graph_fixtures::{
    initialize_validate_instructions_repo, write_file,
};

#[test]
fn test_invoke_validate_instructions_passes_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validate_instructions_repo(repo.path());

    let result = invoke_validate_instructions(&ValidateInstructionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("validation should execute");

    assert_eq!(
        result.status,
        ValidationCheckStatus::Passed,
        "failures={:?} warnings={:?}",
        result.failures,
        result.warnings
    );
    assert_eq!(result.exit_code, 0);
    assert!(result.required_files_checked >= 19);
    assert!(result.catalog_paths_checked >= 10);
    assert!(result.json_files_checked >= 8);
    assert!(result.markdown_files_checked >= 6);
    assert!(result.markdown_links_checked >= 4);
    assert_eq!(result.routing_routes_checked, 1);
    assert_eq!(result.routing_cases_checked, 1);
    assert_eq!(result.skills_checked, 1);
    assert_eq!(result.skill_files_checked, 1);
    assert_eq!(result.openai_files_checked, 1);
    assert!(result.failures.is_empty());
}

#[test]
fn test_invoke_validate_instructions_reports_missing_required_file() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validate_instructions_repo(repo.path());
    fs::remove_file(repo.path().join(".github/AGENTS.md"))
        .expect("temporary AGENTS file should be removed");

    let result = invoke_validate_instructions(&ValidateInstructionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(
        result
            .failures
            .iter()
            .any(|message| message.contains("Required file not found: .github/AGENTS.md"))
    );
}

#[test]
fn test_invoke_validate_instructions_reports_broken_catalog_path() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validate_instructions_repo(repo.path());
    write_file(
        &repo.path().join(".github/instruction-routing.catalog.yml"),
        "always:\n  - path: missing.file\n",
    );

    let result = invoke_validate_instructions(&ValidateInstructionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(
        result
            .failures
            .iter()
            .any(|message| message.contains("Catalog path not found: missing.file"))
    );
}

#[test]
fn test_invoke_validate_instructions_reports_broken_markdown_link() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validate_instructions_repo(repo.path());
    write_file(
        &repo.path().join("README.md"),
        "# Example\n\nSee [Missing](missing.md).\n",
    );

    let result = invoke_validate_instructions(&ValidateInstructionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(
        result
            .failures
            .iter()
            .any(|message| message.contains("Broken markdown link"))
    );
}

#[test]
fn test_invoke_validate_instructions_reports_missing_openai_yaml() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validate_instructions_repo(repo.path());
    fs::remove_file(repo.path().join(".codex/skills/sample/agents/openai.yaml"))
        .expect("temporary openai.yaml should be removed");

    let result = invoke_validate_instructions(&ValidateInstructionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(
        result
            .failures
            .iter()
            .any(|message| message.contains("Skill missing agents/openai.yaml"))
    );
}

#[test]
fn test_invoke_validate_instructions_reports_workspace_template_divergence() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validate_instructions_repo(repo.path());
    write_file(
        &repo.path().join(".vscode/settings.tamplate.jsonc"),
        r#"{
  "files.exclude": {
    "**/.git": false
  },
  "extensions.autoUpdate": true
}"#,
    );

    let result = invoke_validate_instructions(&ValidateInstructionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result.failures.iter().any(|message| {
        message.contains("Workspace efficiency baseline requires files.exclude entry")
            || message.contains("Workspace recommended setting 'extensions.autoUpdate' diverges")
    }));
}

#[test]
fn test_invoke_validate_instructions_reports_broken_snippet_reference() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validate_instructions_repo(repo.path());
    write_file(
        &repo
            .path()
            .join(".vscode/snippets/copilot.tamplate.code-snippets"),
        r#"{
  "Broken": {
    "prefix": "copilot",
    "body": [
      "Open .github/missing.md"
    ]
  }
}"#,
    );

    let result = invoke_validate_instructions(&ValidateInstructionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(
        result
            .failures
            .iter()
            .any(|message| message.contains("Broken snippet path"))
    );
}

#[test]
fn test_invoke_validate_instructions_reports_broken_routing_fixture() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validate_instructions_repo(repo.path());
    write_file(
        &repo
            .path()
            .join("scripts/validation/fixtures/routing-golden-tests.json"),
        r#"{
  "cases": [
    {
      "id": "broken-route",
      "expected_route_ids": ["missing-route"],
      "expected_selected_paths": ["instructions/repository-operating-model.instructions.md"]
    }
  ]
}"#,
    );

    let result = invoke_validate_instructions(&ValidateInstructionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.routing_routes_checked, 1);
    assert_eq!(result.routing_cases_checked, 1);
    assert!(result.failures.iter().any(|message| {
        message.contains("Fixture case 'broken-route' references unknown route id: missing-route")
    }));
}

#[test]
fn test_invoke_validate_instructions_converts_failures_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_validate_instructions_repo(repo.path());
    fs::remove_file(repo.path().join(".github/AGENTS.md"))
        .expect("temporary AGENTS file should be removed");

    let result = invoke_validate_instructions(&ValidateInstructionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.failures.is_empty());
    assert!(
        result
            .warnings
            .iter()
            .any(|message| message.contains("Required file not found: .github/AGENTS.md"))
    );
}
