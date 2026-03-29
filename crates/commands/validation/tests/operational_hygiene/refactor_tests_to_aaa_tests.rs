//! Tests for the standalone AAA refactor helper.

use nettoolskit_validation::{
    invoke_refactor_tests_to_aaa, RefactorTestsToAaaRequest, RefactorTestsToAaaStatus,
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
fn test_invoke_refactor_tests_to_aaa_rewrites_test_file() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    let test_file = repo.path().join("crates/example/tests/sample_tests.rs");
    write_file(
        &test_file,
        r#"#[test]
fn sample_test() {
    // ==========
    let _lock = env_lock();
    let feature = Features::load();
    assert_eq!(feature, 1);
}
"#,
    );

    let result = invoke_refactor_tests_to_aaa(&RefactorTestsToAaaRequest {
        repo_root: Some(repo.path().to_path_buf()),
        test_file: test_file.clone(),
        dry_run: false,
    })
    .expect("AAA refactor should execute");

    assert_eq!(result.status, RefactorTestsToAaaStatus::Passed);
    assert!(result.changed);
    assert_eq!(result.removed_separator_comments, 1);
    assert_eq!(result.inserted_arrange_markers, 1);
    assert_eq!(result.inserted_act_markers, 1);
    assert_eq!(result.inserted_assert_markers, 1);
    let content = fs::read_to_string(&test_file).expect("test file should be readable");
    assert!(content.contains("// Arrange"));
    assert!(content.contains("// Act"));
    assert!(content.contains("// Assert"));
    assert!(!content.contains("// =========="));
}

#[test]
fn test_invoke_refactor_tests_to_aaa_dry_run_reports_without_writing() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    let test_file = repo.path().join("crates/example/tests/sample_tests.rs");
    let original = r#"#[test]
fn sample_test() {
    let feature = Features::load();
    assert_eq!(feature, 1);
}
"#;
    write_file(&test_file, original);

    let result = invoke_refactor_tests_to_aaa(&RefactorTestsToAaaRequest {
        repo_root: Some(repo.path().to_path_buf()),
        test_file: test_file.clone(),
        dry_run: true,
    })
    .expect("AAA refactor should execute");

    assert_eq!(result.status, RefactorTestsToAaaStatus::DryRun);
    assert!(result.changed);
    assert_eq!(
        fs::read_to_string(&test_file).expect("test file should be readable"),
        original
    );
}

#[test]
fn test_invoke_refactor_tests_to_aaa_leaves_existing_aaa_file_unchanged() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    let test_file = repo.path().join("crates/example/tests/sample_tests.rs");
    let original = r#"#[test]
fn sample_test() {
    // Arrange
    let feature = Features::load();

    // Act
    let actual = feature;

    // Assert
    assert_eq!(actual, 1);
}
"#;
    write_file(&test_file, original);

    let result = invoke_refactor_tests_to_aaa(&RefactorTestsToAaaRequest {
        repo_root: Some(repo.path().to_path_buf()),
        test_file: test_file.clone(),
        dry_run: false,
    })
    .expect("AAA refactor should execute");

    assert!(!result.changed);
    assert_eq!(
        fs::read_to_string(&test_file).expect("test file should be readable"),
        original
    );
}
