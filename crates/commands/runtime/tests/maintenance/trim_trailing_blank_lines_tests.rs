//! Tests for runtime trim-trailing-blank-lines commands.

use nettoolskit_runtime::{
    invoke_trim_trailing_blank_lines, RuntimeTrimTrailingBlankLinesRequest,
    RuntimeTrimTrailingBlankLinesStatus,
};
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn initialize_repo_root(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github")).expect(".github should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect(".codex should be created");
}

fn initialize_git_repo(repo_root: &Path) {
    let output = Command::new("git")
        .arg("init")
        .arg(repo_root)
        .output()
        .expect("git init should execute");
    assert!(output.status.success(), "git init should succeed");
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn write_editorconfig(repo_root: &Path) {
    write_file(
        &repo_root.join(".editorconfig"),
        "root = true\n\n[*]\ninsert_final_newline = false\n\n[*.{rs,toml,lock}]\ninsert_final_newline = true\n",
    );
}

#[test]
fn test_invoke_trim_trailing_blank_lines_trims_literal_paths() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    write_editorconfig(repo.path());
    write_file(&repo.path().join("notes.md"), "alpha   \n\n\n");
    write_file(
        &repo.path().join("src/lib.rs"),
        "pub fn sample() {}\r\n\r\n",
    );

    let result = invoke_trim_trailing_blank_lines(&RuntimeTrimTrailingBlankLinesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        literal_paths: vec![repo.path().join("notes.md"), repo.path().join("src/lib.rs")],
        ..RuntimeTrimTrailingBlankLinesRequest::default()
    })
    .expect("trim trailing blank lines should execute");

    assert_eq!(result.status, RuntimeTrimTrailingBlankLinesStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.changed_files.len(), 2);
    assert_eq!(
        fs::read_to_string(repo.path().join("notes.md")).expect("file should be readable"),
        "alpha"
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("src/lib.rs")).expect("file should be readable"),
        "pub fn sample() {}\r\n"
    );
}

#[test]
fn test_invoke_trim_trailing_blank_lines_check_only_reports_changes_without_writing() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    write_editorconfig(repo.path());
    write_file(&repo.path().join("notes.md"), "alpha   \n\n\n");

    let result = invoke_trim_trailing_blank_lines(&RuntimeTrimTrailingBlankLinesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        literal_paths: vec![repo.path().join("notes.md")],
        check_only: true,
        ..RuntimeTrimTrailingBlankLinesRequest::default()
    })
    .expect("trim trailing blank lines should execute");

    assert_eq!(
        result.status,
        RuntimeTrimTrailingBlankLinesStatus::CheckOnly
    );
    assert_eq!(result.exit_code, 1);
    assert_eq!(result.changed_files.len(), 1);
    assert_eq!(
        fs::read_to_string(repo.path().join("notes.md")).expect("file should be readable"),
        "alpha   \n\n\n"
    );
}

#[test]
fn test_invoke_trim_trailing_blank_lines_supports_git_changed_only() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    initialize_git_repo(repo.path());
    write_editorconfig(repo.path());
    write_file(&repo.path().join("notes.md"), "alpha   \n\n\n");

    let result = invoke_trim_trailing_blank_lines(&RuntimeTrimTrailingBlankLinesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        git_changed_only: true,
        ..RuntimeTrimTrailingBlankLinesRequest::default()
    })
    .expect("trim trailing blank lines should execute");

    assert!(result
        .changed_files
        .iter()
        .any(|path| path.ends_with("notes.md")));
    assert_eq!(
        fs::read_to_string(repo.path().join("notes.md")).expect("file should be readable"),
        "alpha"
    );
}
