//! Shared fixtures for release validation tests.

use std::fs;
use std::path::Path;
use std::process::Command;

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

pub fn initialize_release_provenance_repo(repo_root: &Path) {
    initialize_release_governance_repo(repo_root);
    write_repo_file(
        repo_root,
        ".github/governance/release-provenance.baseline.json",
        r#"{
  "version": 1,
  "releaseBranch": "main",
  "requireCleanWorktree": false,
  "warnOnDirtyWorktree": false,
  "requireAuditReport": false,
  "warnOnMissingOptionalAuditReport": false,
  "warnOnAuditCommitMismatch": true,
  "changelogPath": "CHANGELOG.md",
  "validateAllCommand": "ntk validation all",
  "requiredValidationChecks": [
    "validate-release-governance",
    "validate-release-provenance"
  ],
  "requiredEvidenceFiles": [
    "CHANGELOG.md",
    "CODEOWNERS",
    ".github/governance/release-governance.md",
    ".github/governance/release-provenance.baseline.json"
  ]
}"#,
    );
}

pub fn initialize_git_repository(repo_root: &Path) -> String {
    run_git(repo_root, &["init", "-b", "main"]);
    run_git(
        repo_root,
        &["config", "user.email", "fixtures@example.invalid"],
    );
    run_git(repo_root, &["config", "user.name", "Fixture User"]);
    run_git(repo_root, &["add", "."]);
    run_git(repo_root, &["commit", "-m", "Initial release fixtures"]);

    run_git_capture(repo_root, &["rev-parse", "HEAD"])
}

pub fn write_audit_report(
    repo_root: &Path,
    relative_path: &str,
    commit: &str,
    overall_status: &str,
) {
    write_repo_file(
        repo_root,
        relative_path,
        &format!(
            r#"{{
  "generatedAt": "2026-03-27T13:43:00Z",
  "summary": {{
    "overallStatus": "{overall_status}"
  }},
  "git": {{
    "commit": "{commit}"
  }}
}}"#
        ),
    );
}

fn run_git(repo_root: &Path, arguments: &[&str]) {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(arguments)
        .status()
        .expect("git command should start");
    assert!(
        status.success(),
        "git command should succeed: {:?}",
        arguments
    );
}

fn run_git_capture(repo_root: &Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(arguments)
        .output()
        .expect("git command should start");
    assert!(
        output.status.success(),
        "git command should succeed: {:?}",
        arguments
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}