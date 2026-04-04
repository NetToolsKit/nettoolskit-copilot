//! Shared fixtures for test naming validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

pub fn initialize_test_naming_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github"))
        .expect("github directory should be created for repository resolution");
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
}