//! Shared fixtures for security baseline validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

pub fn initialize_security_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    write_repo_file(repo_root, "CODEOWNERS", "* @example\n");
    write_repo_file(repo_root, ".github/AGENTS.md", "# Agents\n");
    write_repo_file(repo_root, ".github/copilot-instructions.md", "# Copilot\n");
    write_repo_file(
        repo_root,
        ".github/governance/security-baseline.json",
        r#"{
  "version": 1,
  "requiredFiles": ["CODEOWNERS", ".github/AGENTS.md"],
  "requiredDirectories": [".github/governance", "scripts/validation"],
  "scanExtensions": [".md", ".ps1"],
  "excludedPathGlobs": [".temp/**"],
  "forbiddenPathGlobs": ["**/*.key"],
  "forbiddenContentPatterns": [
    {
      "id": "private-key-block",
      "pattern": "-----BEGIN PRIVATE KEY-----",
      "severity": "failure"
    },
    {
      "id": "hardcoded-password-assignment",
      "pattern": "(?i)(password|passwd|pwd)\\s*[:=]\\s*[\"'](?!\\*{3}|changeme|password|example|your-password)[^\"']{8,}[\"']",
      "severity": "warning"
    }
  ],
  "allowedContentPatterns": [
    "(?i)example-password"
  ]
}"#,
    );
    write_repo_file(repo_root, "scripts/validation/validate-agent-hooks.ps1", "Write-Output 'ok'\n");
    write_repo_file(repo_root, "README.md", "# Repo\n");
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