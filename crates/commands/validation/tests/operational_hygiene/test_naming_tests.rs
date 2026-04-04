//! Tests for operational_hygiene::test_naming module.

use nettoolskit_validation::{
    invoke_validate_test_naming, ValidateTestNamingRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::test_naming_fixtures::{initialize_test_naming_repo, write_file};

#[test]
fn test_invoke_validate_test_naming_passes_for_matching_projects() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_test_naming_repo(repo.path());
    write_test_project(
        repo.path(),
        "src/App.Tests/App.Tests.csproj",
        "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>",
    );
    write_test_project(
        repo.path(),
        "src/Other.UnitTests/Other.UnitTests.csproj",
        "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>",
    );
    write_file(
        &repo.path().join("src/App.Tests/Tests/FeatureTests.cs"),
        r#"using System.Threading.Tasks;
using Xunit;

namespace Example.App.Tests;

public class FeatureTests
{
    [Fact]
    public async Task Feature_Context_Result_One()
    {
        await Task.CompletedTask;
    }

    public void HelperMethod()
    {
    }
}
"#,
    );
    write_file(
        &repo.path().join("src/Other.UnitTests/Tests/OtherTests.cs"),
        r#"using Xunit;

namespace Example.Other.Tests;

public class OtherTests
{
    [Fact]
    public void Other_Context_Result_One()
    {
    }
}
"#,
    );

    let result = invoke_validate_test_naming(&ValidateTestNamingRequest {
        repo_root: Some(repo.path().to_path_buf()),
        projects: Some(vec!["App.Tests".to_string()]),
        warning_only: false,
        ..ValidateTestNamingRequest::default()
    })
    .expect("test naming validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.test_projects_checked, 1);
    assert_eq!(result.test_files_checked, 1);
    assert_eq!(result.test_methods_checked, 1);
    assert_eq!(result.violations_found, 0);
}

#[test]
fn test_invoke_validate_test_naming_reports_underscore_violations() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_test_naming_repo(repo.path());
    write_test_project(
        repo.path(),
        "src/App.Tests/App.Tests.csproj",
        "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>",
    );
    write_file(
        &repo.path().join("src/App.Tests/Tests/FeatureTests.cs"),
        r#"using Xunit;

namespace Example.App.Tests;

public class FeatureTests
{
    [Fact]
    public void FeatureContextResult()
    {
    }

    [Fact]
    public void Feature_Context_Result_One()
    {
    }
}
"#,
    );

    let result = invoke_validate_test_naming(&ValidateTestNamingRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateTestNamingRequest::default()
    })
    .expect("test naming validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.test_projects_checked, 1);
    assert_eq!(result.test_files_checked, 1);
    assert_eq!(result.test_methods_checked, 2);
    assert_eq!(result.violations_found, 1);
    assert!(result.failures.iter().any(|message| {
        message.contains("Test method name violates underscore convention")
            && message.contains("FeatureContextResult")
    }));
}

#[test]
fn test_invoke_validate_test_naming_converts_violations_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_test_naming_repo(repo.path());
    write_test_project(
        repo.path(),
        "src/App.Tests/App.Tests.csproj",
        "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>",
    );
    write_file(
        &repo.path().join("src/App.Tests/Tests/FeatureTests.cs"),
        r#"using Xunit;

namespace Example.App.Tests;

public class FeatureTests
{
    [Fact]
    public void FeatureContextResult()
    {
    }
}
"#,
    );

    let result = invoke_validate_test_naming(&ValidateTestNamingRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateTestNamingRequest::default()
    })
    .expect("test naming validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("FeatureContextResult")));
}

fn write_test_project(repo_root: &std::path::Path, relative_path: &str, contents: &str) {
    write_file(&repo_root.join(relative_path), contents);
}