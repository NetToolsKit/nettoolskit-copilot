//! Shared fixtures for release validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
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

pub fn initialize_release_governance_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    write_repo_file(
        repo_root,
        "CHANGELOG.md",
        r#"# Changelog

[2.0.0] - 2026-03-20
[1.9.0] - 2026-02-10
"#,
    );
    write_repo_file(
        repo_root,
        "CODEOWNERS",
        "* @example\n.github/ @example\n.githooks/ @example\nscripts/ @example\n",
    );
    write_repo_file(
        repo_root,
        ".github/governance/release-governance.md",
        r#"# Release Governance

## Scope

Scope.

## Branch Protection

Branch protection.

## CODEOWNERS

Owners.

## Release Checklist

Checklist.

## Rollback

Rollback.
"#,
    );
    write_repo_file(
        repo_root,
        ".github/governance/branch-protection.baseline.json",
        r#"{
  "schemaVersion": 1,
  "repository": "example/repo",
  "branch": "main",
  "protection": {
    "required_status_checks": {
      "strict": true,
      "contexts": [
        "Validate Instructions Runtime and Policies"
      ]
    },
    "enforce_admins": true,
    "required_pull_request_reviews": {
      "dismiss_stale_reviews": true,
      "require_code_owner_reviews": true,
      "required_approving_review_count": 1
    }
  }
}"#,
    );
}