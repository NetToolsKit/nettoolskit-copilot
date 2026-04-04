//! Tests for standards::dotnet_standards module.

use nettoolskit_validation::{
    invoke_validate_dotnet_standards, ValidateDotnetStandardsRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::standards_fixtures::{
    initialize_dotnet_standards_repo, write_dotnet_template_file,
};

#[test]
fn test_invoke_validate_dotnet_standards_passes_for_valid_templates() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_dotnet_standards_repo(repo.path());

    let result = invoke_validate_dotnet_standards(&ValidateDotnetStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidateDotnetStandardsRequest::default()
    })
    .expect("dotnet standards validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.templates_checked, 4);
}

#[test]
fn test_invoke_validate_dotnet_standards_reports_missing_required_template() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_dotnet_standards_repo(repo.path());
    std::fs::remove_file(
        repo.path()
            .join(".github/templates/dotnet-class-template.cs"),
    )
    .expect("required template should be removed");

    let result = invoke_validate_dotnet_standards(&ValidateDotnetStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidateDotnetStandardsRequest::default()
    })
    .expect("dotnet standards validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Required .NET template not found")));
}

#[test]
fn test_invoke_validate_dotnet_standards_reports_missing_required_pattern() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_dotnet_standards_repo(repo.path());
    write_dotnet_template_file(
        repo.path(),
        "dotnet-interface-template.cs",
        r#"namespace [Namespace];

/// <summary>
/// Example interface template.
/// </summary>
public interface ExampleInterface
{
}
"#,
    );

    let result = invoke_validate_dotnet_standards(&ValidateDotnetStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidateDotnetStandardsRequest::default()
    })
    .expect("dotnet standards validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result.failures.iter().any(|message| {
        message.contains("Template missing required pattern")
            && message.contains("dotnet-interface-template.cs")
    }));
}

#[test]
fn test_invoke_validate_dotnet_standards_warns_for_missing_xml_summary() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_dotnet_standards_repo(repo.path());
    write_dotnet_template_file(
        repo.path(),
        "dotnet-class-template.cs",
        r#"namespace [Namespace];

public class [ClassName]
{
}
"#,
    );

    let result = invoke_validate_dotnet_standards(&ValidateDotnetStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidateDotnetStandardsRequest::default()
    })
    .expect("dotnet standards validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Template missing XML <summary> section")));
}

#[test]
fn test_invoke_validate_dotnet_standards_reports_whitespace_hygiene_failures() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_dotnet_standards_repo(repo.path());
    write_dotnet_template_file(
        repo.path(),
        "dotnet-unit-test-template.cs",
        "namespace [Namespace];\n\n/// <summary>\n/// Example unit test template.\n/// </summary>\n[TEST_CLASS]\npublic class [ClassName]Tests\n{\n\t[Fact]\n    public void TestName()    \n    {\n    }\n}\n",
    );

    let result = invoke_validate_dotnet_standards(&ValidateDotnetStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidateDotnetStandardsRequest::default()
    })
    .expect("dotnet standards validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Template contains tab character")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Template contains trailing whitespace")));
}