//! Tests for executable runtime command surfaces exposed by `ntk`.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn ntk() -> Command {
    cargo_bin_cmd!("ntk")
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_git_repo(repo_root: &Path) {
    let output = std::process::Command::new("git")
        .arg("init")
        .arg(repo_root)
        .output()
        .expect("git init should execute");
    assert!(output.status.success(), "git init should succeed");
}

fn initialize_runtime_repo_root(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github")).expect(".github should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect(".codex should be created");
}

#[test]
fn test_runtime_pre_tool_use_emits_hook_specific_output_json() {
    let workspace = TempDir::new().expect("temporary workspace should be created");
    write_file(
        &workspace.path().join(".editorconfig"),
        "root = true\n\n[*]\ninsert_final_newline = false\n",
    );
    let payload = json!({
        "cwd": workspace.path(),
        "tool_name": "createFile",
        "tool_input": {
            "filePath": "README.md",
            "content": "# Title\n"
        }
    });

    ntk()
        .args(["runtime", "pre-tool-use"])
        .write_stdin(payload.to_string())
        .assert()
        .success()
        .stdout(predicate::str::contains(r##""hookEventName":"PreToolUse""##))
        .stdout(predicate::str::contains(r##""updatedInput":{"content":"# Title""##))
        .stdout(predicate::str::contains(r##""filePath":"README.md""##));
}

#[test]
fn test_runtime_trim_trailing_blank_lines_reports_git_changed_only_files() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_file(
        &repo.path().join(".editorconfig"),
        "root = true\n\n[*]\ninsert_final_newline = false\n",
    );
    initialize_git_repo(repo.path());
    write_file(&repo.path().join("changed.cs"), "public sealed class Changed { }\n\n");

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "trim-trailing-blank-lines", "--git-changed-only"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Git changed files mode: enabled"))
        .stdout(predicate::str::contains("Files found: 2"))
        .stdout(predicate::str::contains(".editorconfig"))
        .stdout(predicate::str::contains("changed.cs"));

    assert_eq!(
        fs::read_to_string(repo.path().join("changed.cs")).expect("changed file should be readable"),
        "public sealed class Changed { }"
    );
}

#[test]
fn test_runtime_trim_trailing_blank_lines_supports_plain_git_repos_without_runtime_markers() {
    let repo = TempDir::new().expect("temporary repository should be created");
    write_file(
        &repo.path().join(".editorconfig"),
        "root = true\n\n[*]\ninsert_final_newline = false\n",
    );
    initialize_git_repo(repo.path());
    write_file(&repo.path().join("changed.cs"), "public sealed class Changed { }\n\n");

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "trim-trailing-blank-lines", "--git-changed-only"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Files found: 2"))
        .stdout(predicate::str::contains(".editorconfig"))
        .stdout(predicate::str::contains("changed.cs"));

    assert_eq!(
        fs::read_to_string(repo.path().join("changed.cs")).expect("changed file should be readable"),
        "public sealed class Changed { }"
    );
}
