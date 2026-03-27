//! Tests for the planned global Git alias setup runtime command.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeSetupGlobalGitAliasesRequest {
    repo_root: PathBuf,
    target_codex_path: PathBuf,
    uninstall: bool,
    git_config_global_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeSetupGlobalGitAliasesResult {
    repo_root: PathBuf,
    target_codex_path: PathBuf,
    uninstall: bool,
    git_config_global_path: PathBuf,
    configured_aliases: BTreeMap<String, String>,
    status: &'static str,
    exit_code: i32,
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_runtime_alias_repo(repo_root: &Path, target_codex_path: &Path) {
    fs::create_dir_all(repo_root.join(".github"))
        .expect("github directory should be created");
    fs::create_dir_all(target_codex_path.join("shared-scripts/maintenance"))
        .expect("codex shared scripts directory should be created");
    write_file(
        &target_codex_path
            .join("shared-scripts/maintenance/trim-trailing-blank-lines.ps1"),
        "Write-Output 'trim'",
    );
}

fn read_git_config(path: &Path) -> BTreeMap<String, String> {
    let mut entries = BTreeMap::new();
    if !path.is_file() {
        return entries;
    }

    let document = fs::read_to_string(path).expect("git config should be readable");
    let mut current_section = String::new();
    for raw_line in document.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line.trim_start_matches('[').trim_end_matches(']').to_string();
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let qualified_key = if current_section.is_empty() {
                key.trim().to_string()
            } else {
                format!("{}.{}", current_section, key.trim())
            };
            entries.insert(qualified_key, value.trim().to_string());
        }
    }

    entries
}

fn invoke_setup_global_git_aliases(
    request: &RuntimeSetupGlobalGitAliasesRequest,
) -> RuntimeSetupGlobalGitAliasesResult {
    let trim_script = request
        .target_codex_path
        .join("shared-scripts/maintenance/trim-trailing-blank-lines.ps1");

    if request.uninstall {
        if request.git_config_global_path.exists() {
            fs::remove_file(&request.git_config_global_path)
                .expect("global git config should be removable");
        }

        return RuntimeSetupGlobalGitAliasesResult {
            repo_root: request.repo_root.clone(),
            target_codex_path: request.target_codex_path.clone(),
            uninstall: true,
            git_config_global_path: request.git_config_global_path.clone(),
            configured_aliases: BTreeMap::new(),
            status: "passed",
            exit_code: 0,
        };
    }

    assert!(
        trim_script.is_file(),
        "planned alias setup requires the runtime-synced trim script"
    );

    let mut config = BTreeMap::new();
    config.insert(
        "alias.trim-eof".to_string(),
        format!(
            "!pwsh -NoLogo -NoProfile -ExecutionPolicy Bypass -File '{}' -GitChangedOnly",
            trim_script.display().to_string().replace('\\', "/")
        ),
    );

    let serialized = config
        .iter()
        .map(|(key, value)| {
            format!(
                "[alias]\n\t{} = {}\n",
                key.trim_start_matches("alias."),
                value
            )
        })
        .collect::<String>();
    write_file(&request.git_config_global_path, &serialized);

    RuntimeSetupGlobalGitAliasesResult {
        repo_root: request.repo_root.clone(),
        target_codex_path: request.target_codex_path.clone(),
        uninstall: false,
        git_config_global_path: request.git_config_global_path.clone(),
        configured_aliases: config,
        status: "passed",
        exit_code: 0,
    }
}

#[test]
fn test_runtime_setup_global_git_aliases_installs_trim_alias_into_isolated_config() {
    let repo = TempDir::new().expect("temporary repository should be created");
    let target_codex = repo.path().join(".runtime/codex");
    initialize_runtime_alias_repo(repo.path(), &target_codex);

    let result = invoke_setup_global_git_aliases(&RuntimeSetupGlobalGitAliasesRequest {
        repo_root: repo.path().to_path_buf(),
        target_codex_path: target_codex.clone(),
        uninstall: false,
        git_config_global_path: repo.path().join("isolated-git-config"),
    });

    assert_eq!(result.status, "passed");
    assert_eq!(result.exit_code, 0);
    assert!(!result.uninstall);
    assert_eq!(result.repo_root, repo.path());
    assert_eq!(result.target_codex_path, target_codex);
    assert_eq!(
        result.configured_aliases.get("alias.trim-eof"),
        Some(&format!(
            "!pwsh -NoLogo -NoProfile -ExecutionPolicy Bypass -File '{}' -GitChangedOnly",
            repo.path()
                .join(".runtime/codex/shared-scripts/maintenance/trim-trailing-blank-lines.ps1")
                .display()
                .to_string()
                .replace('\\', "/")
        ))
    );

    let config = read_git_config(&result.git_config_global_path);
    assert_eq!(config.get("alias.trim-eof"), result.configured_aliases.get("alias.trim-eof"));
}

#[test]
fn test_runtime_setup_global_git_aliases_uninstalls_isolated_config() {
    let repo = TempDir::new().expect("temporary repository should be created");
    let target_codex = repo.path().join(".runtime/codex");
    initialize_runtime_alias_repo(repo.path(), &target_codex);

    let git_config = repo.path().join("isolated-git-config");
    write_file(
        &git_config,
        "[alias]\n\ttrim-eof = !pwsh -NoLogo -NoProfile -ExecutionPolicy Bypass -File 'x' -GitChangedOnly\n",
    );

    let result = invoke_setup_global_git_aliases(&RuntimeSetupGlobalGitAliasesRequest {
        repo_root: repo.path().to_path_buf(),
        target_codex_path: target_codex,
        uninstall: true,
        git_config_global_path: git_config.clone(),
    });

    assert_eq!(result.status, "passed");
    assert_eq!(result.exit_code, 0);
    assert!(result.uninstall);
    assert_eq!(result.configured_aliases.len(), 0);
    assert!(!git_config.exists());
}