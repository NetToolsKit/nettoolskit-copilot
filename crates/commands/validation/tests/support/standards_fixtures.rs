//! Shared fixtures for standards validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

pub fn initialize_dotnet_standards_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    fs::create_dir_all(repo_root.join(".github/templates"))
        .expect("template directory should be created");

    write_dotnet_template_file(
        repo_root,
        "dotnet-class-template.cs",
        r#"namespace [Namespace];

/// <summary>
/// Example class template.
/// </summary>
public class [ClassName]
{
}
"#,
    );
    write_dotnet_template_file(
        repo_root,
        "dotnet-interface-template.cs",
        r#"namespace [Namespace];

/// <summary>
/// Example interface template.
/// </summary>
public interface [InterfaceName]
{
}
"#,
    );
    write_dotnet_template_file(
        repo_root,
        "dotnet-unit-test-template.cs",
        r#"namespace [Namespace];

/// <summary>
/// Example unit test template.
/// </summary>
[TEST_CLASS]
public class [ClassName]Tests
{
    [Fact]
    public void TestName()
    {
    }
}
"#,
    );
    write_dotnet_template_file(
        repo_root,
        "dotnet-integration-test-template.cs",
        r#"namespace [Namespace];

/// <summary>
/// Example integration test template.
/// </summary>
public class [ClassName]IntegrationTests
{
    private readonly IMediator _mediator;

    [Test]
    public void TestName()
    {
    }
}
"#,
    );
}

pub fn write_dotnet_template_file(repo_root: &Path, file_name: &str, contents: &str) {
    write_file(
        &repo_root.join(".github/templates").join(file_name),
        contents,
    );
}