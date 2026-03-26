//! Tests for runtime VS Code template application commands.

use nettoolskit_runtime::{invoke_apply_vscode_templates, RuntimeApplyVscodeTemplatesRequest};
use std::fs;
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_repo_layout(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
    fs::create_dir_all(repo_root.join(".vscode")).expect("vscode directory should be created");
}

#[test]
fn test_invoke_apply_vscode_templates_applies_missing_targets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo.path().join(".vscode/settings.tamplate.jsonc"),
        "{\n  \"editor.tabSize\": 4\n}",
    );
    write_file(
        &repo.path().join(".vscode/mcp.tamplate.jsonc"),
        "{\n  \"servers\": {}\n}",
    );

    let result = invoke_apply_vscode_templates(&RuntimeApplyVscodeTemplatesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..RuntimeApplyVscodeTemplatesRequest::default()
    })
    .expect("apply-vscode-templates should execute");

    assert_eq!(result.applied_count, 2);
    assert_eq!(result.skipped_count, 0);
    assert!(repo.path().join(".vscode/settings.json").is_file());
    assert!(repo.path().join(".vscode/mcp.json").is_file());
}

#[test]
fn test_invoke_apply_vscode_templates_skips_existing_targets_without_force() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo.path().join(".vscode/settings.tamplate.jsonc"),
        "{\n  \"editor.tabSize\": 4\n}",
    );
    write_file(
        &repo.path().join(".vscode/mcp.tamplate.jsonc"),
        "{\n  \"servers\": {}\n}",
    );
    write_file(
        &repo.path().join(".vscode/settings.json"),
        "{\n  \"editor.tabSize\": 2\n}",
    );

    let result = invoke_apply_vscode_templates(&RuntimeApplyVscodeTemplatesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        skip_mcp: true,
        ..RuntimeApplyVscodeTemplatesRequest::default()
    })
    .expect("apply-vscode-templates should execute");

    assert_eq!(result.applied_count, 0);
    assert_eq!(result.skipped_count, 1);
    let persisted = fs::read_to_string(repo.path().join(".vscode/settings.json"))
        .expect("settings target should be readable");
    assert!(persisted.contains("\"editor.tabSize\": 2"));
}

#[test]
fn test_invoke_apply_vscode_templates_overwrites_existing_targets_with_force() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo.path().join(".vscode/settings.tamplate.jsonc"),
        "{\n  \"editor.tabSize\": 4\n}",
    );
    write_file(
        &repo.path().join(".vscode/settings.json"),
        "{\n  \"editor.tabSize\": 2\n}",
    );

    let result = invoke_apply_vscode_templates(&RuntimeApplyVscodeTemplatesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        force: true,
        skip_mcp: true,
        ..RuntimeApplyVscodeTemplatesRequest::default()
    })
    .expect("apply-vscode-templates should execute");

    assert_eq!(result.applied_count, 1);
    assert_eq!(result.skipped_count, 0);
    let persisted = fs::read_to_string(repo.path().join(".vscode/settings.json"))
        .expect("settings target should be readable");
    assert!(persisted.contains("\"editor.tabSize\": 4"));
}