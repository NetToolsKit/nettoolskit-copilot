//! Shared fixtures for deploy preflight validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

pub fn initialize_deploy_preflight_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github"))
        .expect("github directory should be created for repository resolution");
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
}

pub fn write_valid_deploy_layout(repo_root: &Path) {
    write_file(
        &repo_root.join("docker/docker-compose.deploy.yaml"),
        "services:\n  api:\n    image: sample/api:latest\n",
    );
    write_file(
        &repo_root.join("docker/docker-compose.yaml"),
        "services:\n  api:\n    image: sample/api:latest\n",
    );
    write_file(
        &repo_root.join("docker/.env"),
        "API_PORT=5000\nAPI_PORT_HTTPS=5001\nSEQ_PORT=8082\n",
    );
    write_file(
        &repo_root.join("src/API/Dockerfile"),
        "FROM mcr.microsoft.com/dotnet/aspnet:8.0\n",
    );
}