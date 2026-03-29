//! Shared fixtures for architecture validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

pub fn initialize_architecture_boundaries_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    fs::create_dir_all(repo_root.join(".github/governance"))
        .expect("governance directory should be created");

    write_file(
        &repo_root.join(".github/governance/architecture-boundaries.baseline.json"),
        r#"{
  "rules": [
    {
      "id": "sample-boundary",
      "files": ["src/sample.rs"],
      "requiredPatterns": ["pub\\s+struct\\s+SampleBoundary;"],
      "forbiddenPatterns": ["ForbiddenDependency"],
      "allowedPatterns": [],
      "severity": "failure"
    }
  ]
}"#,
    );
    write_file(
        &repo_root.join("src/sample.rs"),
        "pub struct SampleBoundary;\n",
    );
}
