//! Tests for `validate-xml-documentation`.

use std::fs;

use nettoolskit_validation::{
    invoke_validate_xml_documentation, ValidateXmlDocumentationRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_repo(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
}

#[test]
fn test_invoke_validate_xml_documentation_passes_for_documented_project() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo(repo.path());
    write_file(&repo.path().join("src/App/App.csproj"), "<Project />");
    write_file(
        &repo.path().join("src/App/ExampleService.cs"),
        r#"namespace Example.App;

/// <summary>
/// Example service.
/// </summary>
public class ExampleService
{
}
"#,
    );

    let result = invoke_validate_xml_documentation(&ValidateXmlDocumentationRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateXmlDocumentationRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.projects_checked, 1);
    assert_eq!(result.total_files, 1);
    assert_eq!(result.documented_files, 1);
    assert_eq!(result.missing_files, 0);
    assert!(result.failures.is_empty());
}

#[test]
fn test_invoke_validate_xml_documentation_reports_missing_summary() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo(repo.path());
    write_file(&repo.path().join("src/App/App.csproj"), "<Project />");
    write_file(
        &repo.path().join("src/App/ExampleService.cs"),
        r#"namespace Example.App;

public class ExampleService
{
}
"#,
    );

    let result = invoke_validate_xml_documentation(&ValidateXmlDocumentationRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateXmlDocumentationRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert_eq!(result.projects_checked, 1);
    assert_eq!(result.missing_files, 1);
    assert_eq!(result.coverage_percent, 0.0);
    assert_eq!(result.missing_entries[0].type_name, "ExampleService");
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Missing XML <summary>")));
}

#[test]
fn test_invoke_validate_xml_documentation_exports_missing_findings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo(repo.path());
    write_file(&repo.path().join("src/App/App.csproj"), "<Project />");
    write_file(
        &repo.path().join("src/App/ExampleService.cs"),
        r#"namespace Example.App;

public class ExampleService
{
}
"#,
    );
    let output_path = repo.path().join("docs/missing-documentation.json");

    let result = invoke_validate_xml_documentation(&ValidateXmlDocumentationRequest {
        repo_root: Some(repo.path().to_path_buf()),
        export_missing: true,
        output_path: Some(output_path.clone()),
        warning_only: true,
        ..ValidateXmlDocumentationRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(output_path.is_file());
    let document = fs::read_to_string(output_path).expect("export should exist");
    assert!(document.contains("\"missingDocumentation\""));
    assert!(document.contains("ExampleService"));
}
