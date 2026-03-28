//! Tests for runtime bootstrap commands.

use crate::sync::provider_surface_test_support::{
    initialize_minimal_mcp_runtime_catalog, initialize_minimal_provider_surface_projection,
};
use nettoolskit_runtime::{invoke_runtime_bootstrap, RuntimeBootstrapRequest};
use std::fs;
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn runtime_binary_name() -> &'static str {
    if cfg!(windows) {
        "ntk.exe"
    } else {
        "ntk"
    }
}

fn write_runtime_install_profile_catalog(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".github/governance/runtime-install-profiles.json"),
        r#"{"schemaVersion":1,"defaultProfile":"none","profiles":{"none":{"description":"none profile","install":{"bootstrap":false,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":false},"runtime":{"github":false,"codex":false,"claude":false}},"github":{"description":"github profile","install":{"bootstrap":true,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":true},"runtime":{"github":true,"codex":false,"claude":false}},"codex":{"description":"codex profile","install":{"bootstrap":true,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":true},"runtime":{"github":false,"codex":true,"claude":false}},"all":{"description":"all profile","install":{"bootstrap":true,"globalVscodeSettings":true,"globalVscodeSnippets":true,"localGitHooks":true,"globalGitAliases":true,"healthcheck":true},"runtime":{"github":true,"codex":true,"claude":true}}}}"#,
    );
}

fn initialize_repo_layout(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
    fs::create_dir_all(repo_root.join(".github/bin"))
        .expect("github runtime bin directory should be created");
    fs::create_dir_all(repo_root.join("scripts/runtime"))
        .expect("runtime directory should be created");
    fs::create_dir_all(repo_root.join("scripts/common"))
        .expect("common directory should be created");
    fs::create_dir_all(repo_root.join("scripts/security"))
        .expect("security directory should be created");
    fs::create_dir_all(repo_root.join("scripts/maintenance"))
        .expect("maintenance directory should be created");
    write_file(
        &repo_root.join(".github/bin").join(runtime_binary_name()),
        "binary",
    );
    write_runtime_install_profile_catalog(repo_root);
    initialize_minimal_provider_surface_projection(repo_root);
}

#[test]
fn test_invoke_runtime_bootstrap_syncs_github_runtime_and_removes_legacy_duplicates() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(&repo.path().join(".github/AGENTS.md"), "# Agents");

    let target_github = repo.path().join(".runtime/github");
    let target_copilot = repo.path().join(".runtime/copilot-skills");
    write_file(
        &target_github.join("skills/super-agent/SKILL.md"),
        "# duplicate github skill",
    );
    write_file(
        &target_copilot.join("using-super-agent/SKILL.md"),
        "# duplicate copilot skill",
    );

    let result = invoke_runtime_bootstrap(&RuntimeBootstrapRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_github_path: Some(target_github.clone()),
        target_copilot_skills_path: Some(target_copilot.clone()),
        runtime_profile: Some("github".to_string()),
        ..RuntimeBootstrapRequest::default()
    })
    .expect("bootstrap should execute");

    assert!(result.github_runtime_enabled);
    assert!(!result.codex_runtime_enabled);
    assert!(repo.path().join(".github/AGENTS.md").is_file());
    assert!(repo
        .path()
        .join(".github/prompts/route-instructions.prompt.md")
        .is_file());
    assert!(repo
        .path()
        .join(".vscode/profiles/profile-base.json")
        .is_file());
    assert!(repo
        .path()
        .join(".vscode/snippets/demo.tamplate.code-snippets")
        .is_file());
    assert!(!repo.path().join(".codex/scripts").exists());
    assert!(!repo.path().join(".claude/settings.json").exists());
    assert!(target_github.join("AGENTS.md").is_file());
    assert!(target_github
        .join("bin")
        .join(runtime_binary_name())
        .is_file());
    assert!(!target_github.join("skills/super-agent").exists());
    assert!(!target_copilot.join("using-super-agent").exists());
    assert!(result.provider_rendered);
}

#[test]
fn test_invoke_runtime_bootstrap_syncs_codex_runtime_assets_and_removes_duplicates() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo.path().join(".codex/skills/runtime-skill/SKILL.md"),
        "# runtime-skill",
    );
    write_file(&repo.path().join(".codex/mcp/catalog.json"), "{}");
    write_file(&repo.path().join(".codex/README.md"), "# shared codex");
    write_file(
        &repo.path().join("scripts/common/common-bootstrap.ps1"),
        "Write-Output 'common'",
    );
    write_file(
        &repo.path().join("scripts/security/audit.ps1"),
        "Write-Output 'security'",
    );
    write_file(
        &repo.path().join("scripts/maintenance/cleanup.ps1"),
        "Write-Output 'maintenance'",
    );

    let target_codex = repo.path().join(".runtime/codex");
    let target_agents = repo.path().join(".runtime/agents-skills");
    write_file(
        &target_codex.join("skills/runtime-skill/SKILL.md"),
        "# duplicate runtime skill",
    );
    write_file(&target_codex.join("skills/README.md"), "# ignored");

    let result = invoke_runtime_bootstrap(&RuntimeBootstrapRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_codex_path: Some(target_codex.clone()),
        target_agents_skills_path: Some(target_agents.clone()),
        runtime_profile: Some("codex".to_string()),
        ..RuntimeBootstrapRequest::default()
    })
    .expect("bootstrap should execute");

    assert!(!result.github_runtime_enabled);
    assert!(result.codex_runtime_enabled);
    assert!(repo.path().join(".github/AGENTS.md").is_file());
    assert!(repo
        .path()
        .join(".vscode/profiles/profile-base.json")
        .is_file());
    assert!(repo.path().join(".codex/scripts/root-tool.ps1").is_file());
    assert!(repo.path().join(".codex/orchestration/flow.md").is_file());
    assert!(repo
        .path()
        .join(".codex/skills/runtime-skill/SKILL.md")
        .is_file());
    assert!(!repo.path().join(".claude/settings.json").exists());
    assert!(target_agents.join("runtime-skill/SKILL.md").is_file());
    assert!(!target_codex.join("skills/runtime-skill").exists());
    assert!(target_codex.join("shared-mcp/README.md").is_file());
    assert!(target_codex.join("shared-scripts/root-tool.ps1").is_file());
    assert!(target_codex
        .join("shared-scripts/common/common-bootstrap.ps1")
        .is_file());
    assert!(target_codex
        .join("shared-scripts/security/audit.ps1")
        .is_file());
    assert!(target_codex
        .join("shared-scripts/maintenance/cleanup.ps1")
        .is_file());
    assert!(target_codex.join("shared-orchestration/flow.md").is_file());
    assert!(target_codex.join("README.shared.md").is_file());
}

#[test]
fn test_invoke_runtime_bootstrap_renders_claude_runtime_surface_when_profile_enables_claude() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(&repo.path().join(".github/AGENTS.md"), "# Agents");
    write_file(&repo.path().join(".codex/README.md"), "# shared codex");

    let result = invoke_runtime_bootstrap(&RuntimeBootstrapRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_github_path: Some(repo.path().join(".runtime/github")),
        target_codex_path: Some(repo.path().join(".runtime/codex")),
        target_agents_skills_path: Some(repo.path().join(".runtime/agents-skills")),
        target_copilot_skills_path: Some(repo.path().join(".runtime/copilot-skills")),
        runtime_profile: Some("all".to_string()),
        ..RuntimeBootstrapRequest::default()
    })
    .expect("bootstrap should execute");

    assert!(result.github_runtime_enabled);
    assert!(result.codex_runtime_enabled);
    assert!(result.claude_runtime_enabled);
    assert!(repo.path().join(".claude/settings.json").is_file());
}

#[test]
fn test_invoke_runtime_bootstrap_mirror_mode_removes_extra_files() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(&repo.path().join(".github/AGENTS.md"), "# Agents");

    let target_github = repo.path().join(".runtime/github");
    write_file(&target_github.join("extra-file.md"), "extra");
    write_file(
        &target_github.join("scripts/runtime/extra-script.ps1"),
        "Write-Output 'extra'",
    );

    invoke_runtime_bootstrap(&RuntimeBootstrapRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_github_path: Some(target_github.clone()),
        runtime_profile: Some("github".to_string()),
        mirror: true,
        ..RuntimeBootstrapRequest::default()
    })
    .expect("bootstrap should execute");

    assert!(!target_github.join("extra-file.md").exists());
    assert!(!target_github
        .join("scripts/runtime/extra-script.ps1")
        .exists());
}

#[test]
fn test_invoke_runtime_bootstrap_applies_mcp_config_when_requested() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    initialize_minimal_mcp_runtime_catalog(repo.path());

    let target_codex = repo.path().join(".runtime/codex");
    write_file(
        &target_codex.join("config.toml"),
        "model = \"gpt-5\"\n\n[tools]\nsearch = true",
    );

    let result = invoke_runtime_bootstrap(&RuntimeBootstrapRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_codex_path: Some(target_codex.clone()),
        runtime_profile: Some("codex".to_string()),
        apply_mcp_config: true,
        backup_config: true,
        ..RuntimeBootstrapRequest::default()
    })
    .expect("bootstrap should execute");

    assert!(result.mcp_config_applied);
    let updated = fs::read_to_string(target_codex.join("config.toml"))
        .expect("updated config should be readable");
    assert!(updated.contains("model = \"gpt-5\""));
    assert!(updated.contains("[tools]"));
    assert!(updated.contains("[mcp_servers.microsoftdocs]"));
    assert!(updated.contains("url = \"https://learn.microsoft.com/api/mcp\""));
    assert!(updated.contains("[mcp_servers.playwright]"));
    assert!(updated.contains("command = \"npx\""));
    assert!(!updated.contains("vscode-only"));
    let backup_files = fs::read_dir(&target_codex)
        .expect("target codex directory should be readable")
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .filter(|file_name| file_name.starts_with("config.toml.bak."))
        .collect::<Vec<_>>();
    assert_eq!(backup_files.len(), 1);
}
