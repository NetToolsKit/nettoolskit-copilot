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

fn write_local_context_catalog(repo_root: &Path) {
    write_file(
        &repo_root.join(".github/governance/local-context-index.catalog.json"),
        r#"{"version":1,"indexRoot":".temp/context-index","maxFileSizeKb":64,"chunking":{"maxChars":400,"maxLines":20},"queryDefaults":{"top":3},"includeGlobs":["README.md","planning/**/*.md","scripts/**/*.ps1",".github/**/*.md"],"excludeGlobs":[".temp/**"]}"#,
    );
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

#[test]
fn test_runtime_update_local_context_index_builds_the_index_document() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_local_context_catalog(repo.path());
    write_file(
        &repo.path().join("README.md"),
        "# Demo\n\nContinuity summary for the local context index.",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "update-local-context-index"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Local context index updated:"))
        .stdout(predicate::str::contains("Files indexed:"));

    assert!(repo.path().join(".temp/context-index/index.json").is_file());
}

#[test]
fn test_runtime_query_local_context_index_supports_json_output() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_local_context_catalog(repo.path());
    write_file(
        &repo.path().join("README.md"),
        "# Demo\n\nContinuity summary for the local context index.",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "update-local-context-index"])
        .assert()
        .success();

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "query-local-context-index",
            "--query-text",
            "continuity summary",
            "--json-output",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""resultCount":1"#))
        .stdout(predicate::str::contains(r#""path":"README.md""#));
}

#[test]
fn test_runtime_export_planning_summary_prints_active_plan_context() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_local_context_catalog(repo.path());
    write_file(
        &repo.path().join("planning/active/plan-wave4.md"),
        "# Wave 4 Plan\n\n- Status: in progress\n- Current focus: retire continuity wrappers.\n",
    );
    write_file(
        &repo.path().join("planning/specs/active/spec-wave4.md"),
        "# Wave 4 Spec\n\nObjective: move continuity execution to ntk runtime.\n",
    );
    write_file(
        &repo.path().join("README.md"),
        "# Runtime Rewrite\n\nRetire continuity wrappers with native entrypoints.",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "update-local-context-index"])
        .assert()
        .success();

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "export-planning-summary", "--print-only"])
        .assert()
        .success()
        .stdout(predicate::str::contains("## Active Plans"))
        .stdout(predicate::str::contains("Wave 4 Plan"))
        .stdout(predicate::str::contains("## Resume Instructions"));
}

#[test]
fn test_runtime_apply_vscode_templates_copies_workspace_templates() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_file(
        &repo.path().join(".vscode/settings.tamplate.jsonc"),
        "{\n  \"editor.tabSize\": 4\n}",
    );
    write_file(
        &repo.path().join(".vscode/mcp.tamplate.jsonc"),
        "{\n  \"servers\": []\n}",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "apply-vscode-templates"])
        .assert()
        .success()
        .stdout(predicate::str::contains("VS Code template apply summary"))
        .stdout(predicate::str::contains("applied: 2"));

    assert!(repo.path().join(".vscode/settings.json").is_file());
    assert!(repo.path().join(".vscode/mcp.json").is_file());
}
