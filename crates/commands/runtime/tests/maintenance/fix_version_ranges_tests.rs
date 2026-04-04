//! Tests for runtime fix-version-ranges commands.

use nettoolskit_runtime::{
    invoke_fix_version_ranges, RuntimeFixVersionRangesRequest, RuntimeFixVersionRangesStatus,
};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn initialize_repo_root(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github")).expect(".github should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect(".codex should be created");
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

#[test]
fn test_invoke_fix_version_ranges_updates_explicit_versions() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    let project_path = repo.path().join("src/App/App.csproj");
    write_file(
        &project_path,
        r#"<Project>
  <ItemGroup>
    <PackageReference Include="AutoMapper" Version="13.0.1" />
  </ItemGroup>
</Project>
"#,
    );

    let result = invoke_fix_version_ranges(&RuntimeFixVersionRangesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        project_file: Some(project_path.clone()),
        ..RuntimeFixVersionRangesRequest::default()
    })
    .expect("fix version ranges should execute");

    assert_eq!(result.status, RuntimeFixVersionRangesStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.changed_projects.len(), 1);
    assert_eq!(result.changed_projects[0].adjustments.len(), 1);
    assert_eq!(
        fs::read_to_string(&project_path).expect("project should be readable"),
        r#"<Project>
  <ItemGroup>
    <PackageReference Include="AutoMapper" Version="[13.0.1,14.0.0)" />
  </ItemGroup>
</Project>
"#
    );
}

#[test]
fn test_invoke_fix_version_ranges_updates_existing_ranges() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    let project_path = repo.path().join("src/App/App.csproj");
    write_file(
        &project_path,
        r#"<Project>
  <ItemGroup>
    <PackageReference Include="MediatR" Version="[12.1.0,12.5.0)" />
  </ItemGroup>
</Project>
"#,
    );

    let result = invoke_fix_version_ranges(&RuntimeFixVersionRangesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..RuntimeFixVersionRangesRequest::default()
    })
    .expect("fix version ranges should execute");

    assert_eq!(result.status, RuntimeFixVersionRangesStatus::Passed);
    assert_eq!(result.changed_projects.len(), 1);
    assert_eq!(
        result.changed_projects[0].adjustments[0].updated_version,
        "[12.1.0,13.0.0)"
    );
}

#[test]
fn test_invoke_fix_version_ranges_dry_run_reports_without_writing() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    let project_path = repo.path().join("src/App/App.csproj");
    let original = r#"<Project>
  <ItemGroup>
    <PackageReference Include="FluentAssertions" Version="7.2.0" />
  </ItemGroup>
</Project>
"#;
    write_file(&project_path, original);

    let result = invoke_fix_version_ranges(&RuntimeFixVersionRangesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        project_file: Some(project_path.clone()),
        dry_run: true,
    })
    .expect("fix version ranges should execute");

    assert_eq!(result.status, RuntimeFixVersionRangesStatus::DryRun);
    assert_eq!(result.changed_projects.len(), 1);
    assert_eq!(
        fs::read_to_string(&project_path).expect("project should be readable"),
        original
    );
}