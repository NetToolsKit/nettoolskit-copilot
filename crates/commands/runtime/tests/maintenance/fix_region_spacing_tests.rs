//! Tests for runtime fix-region-spacing commands.

use nettoolskit_runtime::{
    invoke_fix_region_spacing, RuntimeFixRegionSpacingRequest, RuntimeFixRegionSpacingStatus,
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
fn test_invoke_fix_region_spacing_updates_adjacent_regions() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    let file_path = repo.path().join("src/Sample.cs");
    write_file(
        &file_path,
        "class Sample\r\n{\r\n    #endregion\r\n    #region Build\r\n}\r\n",
    );

    let result = invoke_fix_region_spacing(&RuntimeFixRegionSpacingRequest {
        repo_root: Some(repo.path().to_path_buf()),
        path: Some(repo.path().join("src")),
        ..RuntimeFixRegionSpacingRequest::default()
    })
    .expect("fix region spacing should execute");

    assert_eq!(result.status, RuntimeFixRegionSpacingStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.changed_files, vec![file_path.clone()]);
    assert_eq!(
        fs::read_to_string(&file_path).expect("file should be readable"),
        "class Sample\r\n{\r\n    #endregion\r\n\r\n    #region Build\r\n}\r\n"
    );
}

#[test]
fn test_invoke_fix_region_spacing_dry_run_reports_changes_without_writing() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    let file_path = repo.path().join("src/Sample.cs");
    write_file(
        &file_path,
        "class Sample\n{\n    #endregion\n    #region Build\n}\n",
    );

    let result = invoke_fix_region_spacing(&RuntimeFixRegionSpacingRequest {
        repo_root: Some(repo.path().to_path_buf()),
        path: Some(file_path.clone()),
        dry_run: true,
    })
    .expect("fix region spacing should execute");

    assert_eq!(result.status, RuntimeFixRegionSpacingStatus::DryRun);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.changed_files, vec![file_path.clone()]);
    assert_eq!(
        fs::read_to_string(&file_path).expect("file should be readable"),
        "class Sample\n{\n    #endregion\n    #region Build\n}\n"
    );
}

#[test]
fn test_invoke_fix_region_spacing_ignores_non_csharp_files() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    let file_path = repo.path().join("notes.md");
    write_file(&file_path, "#endregion\n#region\n");

    let result = invoke_fix_region_spacing(&RuntimeFixRegionSpacingRequest {
        repo_root: Some(repo.path().to_path_buf()),
        path: Some(repo.path().to_path_buf()),
        ..RuntimeFixRegionSpacingRequest::default()
    })
    .expect("fix region spacing should execute");

    assert!(result.discovered_files.is_empty());
    assert!(result.changed_files.is_empty());
    assert_eq!(result.status, RuntimeFixRegionSpacingStatus::Passed);
}
