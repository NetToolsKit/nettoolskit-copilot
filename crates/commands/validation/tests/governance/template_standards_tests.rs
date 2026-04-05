//! Tests for `validate-template-standards`.

use nettoolskit_validation::{
    invoke_validate_template_standards, ValidateTemplateStandardsRequest, ValidationCheckStatus,
};
use std::fs;
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn write_governance_file(repo_root: &std::path::Path, file_name: &str, contents: &str) {
    write_file(
        &repo_root
            .join("definitions/providers/github/governance")
            .join(file_name),
        contents,
    );
    write_file(&repo_root.join(".github/governance").join(file_name), contents);
}

fn write_template_file(repo_root: &std::path::Path, file_name: &str, contents: &str) {
    write_file(
        &repo_root.join("definitions/templates/docs").join(file_name),
        contents,
    );
    write_file(&repo_root.join(".github/templates").join(file_name), contents);
}

fn initialize_repo_layout(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".github/templates"))
        .expect("templates directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
}

fn write_baseline(repo_root: &std::path::Path) {
    write_governance_file(
        repo_root,
        "template-standards.baseline.json",
        r#"{
  "version": 1,
  "requiredFiles": [
    "definitions/templates/docs/example.md"
  ],
  "templateRules": [
    {
      "path": "definitions/templates/docs/example.md",
      "requiredPatterns": ["^# Example", "Validation"],
      "forbiddenPatterns": ["Legacy"],
      "requiredPathReferences": ["definitions/providers/github/governance/template-standards.baseline.json"]
    }
  ]
}"#,
    );
}

#[test]
fn test_invoke_validate_template_standards_passes_for_valid_templates() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path());
    write_template_file(repo.path(), "example.md", "# Example\n\nValidation content.\n");

    let result = invoke_validate_template_standards(&ValidateTemplateStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateTemplateStandardsRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.templates_checked, 1);
    assert_eq!(result.rules_checked, 1);
}

#[test]
fn test_invoke_validate_template_standards_reports_missing_patterns_and_references() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_governance_file(
        repo.path(),
        "template-standards.baseline.json",
        r#"{
  "version": 1,
  "requiredFiles": [
    "definitions/templates/docs/example.md",
    "definitions/templates/docs/missing.md"
  ],
  "templateRules": [
    {
      "path": "definitions/templates/docs/example.md",
      "requiredPatterns": ["^# Example", "Validation"],
      "forbiddenPatterns": ["Legacy"],
      "requiredPathReferences": ["missing/path.md"]
    }
  ]
}"#,
    );
    write_template_file(repo.path(), "example.md", "# Example\n\nLegacy content. \n");

    let result = invoke_validate_template_standards(&ValidateTemplateStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateTemplateStandardsRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Required template not found")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Template contains trailing whitespace")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Template missing required pattern 'Validation'")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Template contains forbidden pattern 'Legacy'")));
}

#[test]
fn test_invoke_validate_template_standards_converts_required_findings_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path());
    write_template_file(repo.path(), "example.md", "# Example\n\nLegacy content.\n");

    let result = invoke_validate_template_standards(&ValidateTemplateStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateTemplateStandardsRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Template missing required pattern 'Validation'")));
}