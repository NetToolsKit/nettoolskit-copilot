//! Shared fixtures for policy validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

pub fn initialize_policy_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    write_repo_file(repo_root, "README.md", "# Repo\n");
    write_repo_file(repo_root, "scripts/runtime/install.ps1", "Write-Output 'install'\n");
    write_repo_file(repo_root, ".githooks/pre-commit", "#!/bin/sh\n");
    write_repo_file(repo_root, ".githooks/post-commit", "#!/bin/sh\n");
    write_policy_file(
        repo_root,
        "baseline.policy.json",
        r#"{
  "id": "repository-baseline",
  "requiredFiles": ["README.md", "scripts/runtime/install.ps1"],
  "requiredDirectories": [".github/policies", ".githooks"],
  "forbiddenFiles": ["forbidden.txt"],
  "requiredGitHooks": ["pre-commit", "post-commit"]
}"#,
    );
}

pub fn initialize_compatibility_lifecycle_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    fs::create_dir_all(repo_root.join(".github"))
        .expect("github directory should be created for repository resolution");
    write_compatibility_file(
        repo_root,
        "COMPATIBILITY.md",
        "January 15, 2025",
        &[
            "| 1.2 | January 1, 2024 | February 1, 2025 | March 1, 2025 | March 2, 2025 | Active |",
        ],
    );
}

pub fn write_repo_file(repo_root: &Path, relative_path: &str, contents: &str) {
    write_file(&repo_root.join(relative_path), contents);
}

pub fn remove_repo_path(repo_root: &Path, relative_path: &str) {
    let path = repo_root.join(relative_path);
    if path.is_dir() {
        fs::remove_dir_all(&path).expect("directory should be removed");
    } else if path.is_file() {
        fs::remove_file(&path).expect("file should be removed");
    }
}

pub fn write_policy_file(repo_root: &Path, file_name: &str, contents: &str) {
    write_repo_file(
        repo_root,
        &format!(".github/policies/{file_name}"),
        contents,
    );
}

pub fn write_compatibility_file(
    repo_root: &Path,
    relative_path: &str,
    reference_date: &str,
    rows: &[&str],
) {
    let mut lines = vec![
        "# Compatibility".to_string(),
        String::new(),
        "## Support Lifecycle and EOL".to_string(),
        format!(
            "Reference date for status labels in this table: **{reference_date}**."
        ),
        String::new(),
        "| Minor | GA date | Active support until | Maintenance support until | EOL date | Status |"
            .to_string(),
        "| --- | --- | --- | --- | --- | --- |".to_string(),
    ];
    lines.extend(rows.iter().map(|row| (*row).to_string()));
    lines.push(String::new());

    write_repo_file(repo_root, relative_path, &lines.join("\n"));
}