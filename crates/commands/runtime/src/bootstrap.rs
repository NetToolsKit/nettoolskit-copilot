//! Runtime bootstrap orchestration for repository-managed runtime assets.

use anyhow::{anyhow, Context, Result};
use nettoolskit_core::runtime_execution::resolve_runtime_execution_context;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

use crate::error::RuntimeBootstrapCommandError;

/// Request payload for `bootstrap`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeBootstrapRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit GitHub runtime target path.
    pub target_github_path: Option<PathBuf>,
    /// Optional explicit Codex runtime target path.
    pub target_codex_path: Option<PathBuf>,
    /// Optional explicit picker-visible agent skills path.
    pub target_agents_skills_path: Option<PathBuf>,
    /// Optional explicit Copilot native skills path.
    pub target_copilot_skills_path: Option<PathBuf>,
    /// Optional explicit runtime profile name.
    pub runtime_profile: Option<String>,
    /// Optional explicit fallback runtime profile name.
    pub fallback_runtime_profile: Option<String>,
    /// Mirror target folders by removing files absent from the source.
    pub mirror: bool,
    /// Apply MCP configuration into the target Codex config file.
    pub apply_mcp_config: bool,
    /// Create a backup before applying MCP configuration.
    pub backup_config: bool,
}

/// Result payload for `bootstrap`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeBootstrapResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective runtime profile name.
    pub runtime_profile_name: String,
    /// Runtime profile catalog path used during resolution.
    pub runtime_profile_catalog_path: PathBuf,
    /// Whether the GitHub runtime surface was enabled.
    pub github_runtime_enabled: bool,
    /// Whether the Codex runtime surface was enabled.
    pub codex_runtime_enabled: bool,
    /// Whether the Claude runtime surface was enabled.
    pub claude_runtime_enabled: bool,
    /// Whether mirror mode was requested.
    pub mirror_mode: bool,
    /// Whether MCP config application was requested.
    pub apply_mcp_config: bool,
    /// Whether provider render was invoked.
    pub provider_rendered: bool,
    /// Whether MCP config application was delegated.
    pub mcp_config_applied: bool,
}

/// Synchronize repository-managed runtime assets into the local runtime
/// folders.
///
/// # Errors
///
/// Returns [`RuntimeBootstrapCommandError`] when workspace resolution,
/// provider rendering, runtime sync, or MCP configuration application fails.
pub fn invoke_runtime_bootstrap(
    request: &RuntimeBootstrapRequest,
) -> Result<RuntimeBootstrapResult, RuntimeBootstrapCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeBootstrapCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let fallback_profile = request
        .fallback_runtime_profile
        .as_deref()
        .or(Some("all"));
    let context = resolve_runtime_execution_context(
        request.repo_root.as_deref(),
        request.runtime_profile.as_deref(),
        fallback_profile,
        request.target_github_path.as_deref(),
        request.target_codex_path.as_deref(),
        request.target_agents_skills_path.as_deref(),
        request.target_copilot_skills_path.as_deref(),
        None,
        &current_dir,
    )
    .map_err(|source| RuntimeBootstrapCommandError::ResolveExecutionContext { source })?;

    assert_directory_present(&context.sources.github_root, "source .github folder")
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
    assert_directory_present(&context.sources.codex_root, "source .codex folder")
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
    assert_directory_present(&context.sources.scripts_root, "source scripts folder")
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;

    invoke_provider_surface_render(
        &context.resolved_repo_root,
        context.runtime_profile.enable_codex_runtime,
        context.runtime_profile.enable_claude_runtime,
    )
    .map_err(|source| RuntimeBootstrapCommandError::RenderProviderSurfaces { source })?;

    if request.apply_mcp_config && !context.runtime_profile.enable_codex_runtime {
        return Err(RuntimeBootstrapCommandError::ApplyMcpConfig {
            source: anyhow!(
                "runtime profile '{}' does not enable the Codex runtime surface required by -ApplyMcpConfig",
                context.runtime_profile.name
            ),
        });
    }

    if context.runtime_profile.enable_github_runtime {
        sync_directory(
            &context.sources.github_root,
            &context.targets.github_runtime_root,
            request.mirror,
        )
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
        sync_directory(
            &context.sources.scripts_root,
            &context.targets.github_runtime_root.join("scripts"),
            request.mirror,
        )
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
        remove_legacy_starter_skill_duplicates(&[
            context.targets.github_runtime_root.join("skills"),
            context.targets.copilot_skills_root.clone(),
        ])
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
    }

    if context.runtime_profile.enable_codex_runtime {
        sync_agents_skills(
            &context.sources.codex_skills_root,
            &context.targets.agents_skills_root,
            request.mirror,
        )
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
        remove_managed_codex_skill_duplicates(
            &context.sources.codex_skills_root,
            &context.targets.codex_runtime_root.join("skills"),
        )
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
        sync_directory(
            &context.sources.codex_mcp_root,
            &context.targets.codex_runtime_root.join("shared-mcp"),
            request.mirror,
        )
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
        sync_directory(
            &context.sources.codex_scripts_root,
            &context.targets.codex_runtime_root.join("shared-scripts"),
            request.mirror,
        )
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
        sync_directory(
            &context.sources.common_scripts_root,
            &context.targets.codex_runtime_root.join("shared-scripts").join("common"),
            request.mirror,
        )
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
        sync_directory(
            &context.sources.security_scripts_root,
            &context.targets.codex_runtime_root.join("shared-scripts").join("security"),
            request.mirror,
        )
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
        sync_directory(
            &context.sources.maintenance_scripts_root,
            &context.targets
                .codex_runtime_root
                .join("shared-scripts")
                .join("maintenance"),
            request.mirror,
        )
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
        sync_directory(
            &context.sources.codex_orchestration_root,
            &context.targets.codex_runtime_root.join("shared-orchestration"),
            request.mirror,
        )
        .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;

        let shared_readme = context.sources.codex_root.join("README.md");
        if shared_readme.is_file() {
            let target_readme = context.targets.codex_runtime_root.join("README.shared.md");
            if let Some(parent) = target_readme.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create '{}'", parent.display()))
                    .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
            }

            fs::copy(&shared_readme, &target_readme)
                .with_context(|| {
                    format!(
                        "failed to copy '{}' to '{}'",
                        shared_readme.display(),
                        target_readme.display()
                    )
                })
                .map_err(|source| RuntimeBootstrapCommandError::SyncAssets { source })?;
        }
    }

    let mut mcp_config_applied = false;
    if request.apply_mcp_config {
        invoke_mcp_config_apply(
            &context.resolved_repo_root,
            &context.targets.codex_runtime_root,
            request.backup_config,
        )
        .map_err(|source| RuntimeBootstrapCommandError::ApplyMcpConfig { source })?;
        mcp_config_applied = true;
    }

    Ok(RuntimeBootstrapResult {
        repo_root: context.resolved_repo_root,
        runtime_profile_name: context.runtime_profile.name,
        runtime_profile_catalog_path: context.runtime_profile.catalog_path,
        github_runtime_enabled: context.runtime_profile.enable_github_runtime,
        codex_runtime_enabled: context.runtime_profile.enable_codex_runtime,
        claude_runtime_enabled: context.runtime_profile.enable_claude_runtime,
        mirror_mode: request.mirror,
        apply_mcp_config: request.apply_mcp_config,
        provider_rendered: true,
        mcp_config_applied,
    })
}

fn assert_directory_present(path: &Path, label: &str) -> Result<()> {
    if !path.is_dir() {
        return Err(anyhow!("{label} missing: {}", path.display()));
    }

    Ok(())
}

fn invoke_provider_surface_render(
    repo_root: &Path,
    enable_codex_runtime: bool,
    enable_claude_runtime: bool,
) -> Result<()> {
    let render_script_path = repo_root.join("scripts/runtime/render-provider-surfaces.ps1");
    if !render_script_path.is_file() {
        return Err(anyhow!(
            "provider surface render dispatcher missing: {}",
            render_script_path.display()
        ));
    }

    let output = Command::new("pwsh")
        .arg("-NoLogo")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(&render_script_path)
        .arg("-RepoRoot")
        .arg(repo_root)
        .arg("-ConsumerName")
        .arg("bootstrap")
        .arg("-EnableCodexRuntime")
        .arg(enable_codex_runtime.to_string())
        .arg("-EnableClaudeRuntime")
        .arg(enable_claude_runtime.to_string())
        .output()
        .with_context(|| format!("failed to execute '{}'", render_script_path.display()))?;
    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Err(anyhow!(
        "provider surface render dispatch failed before bootstrap sync. {}",
        if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            format!("exit code {}", output.status.code().unwrap_or(1))
        }
    ))
}

fn invoke_mcp_config_apply(repo_root: &Path, codex_path: &Path, backup_config: bool) -> Result<()> {
    let sync_script = repo_root.join("scripts/runtime/sync-codex-mcp-config.ps1");
    let catalog_path = repo_root.join(".github/governance/mcp-runtime.catalog.json");
    let target_config = codex_path.join("config.toml");

    if !sync_script.is_file() {
        return Err(anyhow!("MCP sync script missing: {}", sync_script.display()));
    }
    if !catalog_path.is_file() {
        return Err(anyhow!(
            "MCP runtime catalog missing: {}",
            catalog_path.display()
        ));
    }
    if !target_config.is_file() {
        return Err(anyhow!(
            "target Codex config missing: {}",
            target_config.display()
        ));
    }

    let mut command = Command::new("pwsh");
    command
        .arg("-NoLogo")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(&sync_script)
        .arg("-CatalogPath")
        .arg(&catalog_path)
        .arg("-TargetConfigPath")
        .arg(&target_config);
    if backup_config {
        command.arg("-CreateBackup");
    }

    let output = command
        .output()
        .with_context(|| format!("failed to execute '{}'", sync_script.display()))?;
    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Err(anyhow!(
        "MCP config apply failed. {}",
        if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            format!("exit code {}", output.status.code().unwrap_or(1))
        }
    ))
}

fn sync_agents_skills(source_root: &Path, destination_root: &Path, mirror_mode: bool) -> Result<()> {
    if !source_root.is_dir() {
        return Ok(());
    }

    fs::create_dir_all(destination_root)
        .with_context(|| format!("failed to create '{}'", destination_root.display()))?;
    let source_skill_names = fs::read_dir(source_root)
        .with_context(|| format!("failed to enumerate '{}'", source_root.display()))?
        .filter_map(std::result::Result::ok)
        .filter_map(|entry| {
            entry
                .file_type()
                .ok()
                .filter(|file_type| file_type.is_dir())
                .and_then(|_| entry.file_name().to_str().map(ToOwned::to_owned))
        })
        .collect::<BTreeSet<_>>();

    if mirror_mode && destination_root.is_dir() {
        for entry in fs::read_dir(destination_root)
            .with_context(|| format!("failed to enumerate '{}'", destination_root.display()))?
            .filter_map(std::result::Result::ok)
        {
            if !entry.file_type().is_ok_and(|file_type| file_type.is_dir()) {
                continue;
            }

            let Some(name) = entry.file_name().to_str().map(ToOwned::to_owned) else {
                continue;
            };
            if !source_skill_names.contains(&name) {
                fs::remove_dir_all(entry.path()).with_context(|| {
                    format!("failed to remove '{}'", entry.path().display())
                })?;
            }
        }
    }

    for skill_name in source_skill_names {
        sync_directory(
            &source_root.join(&skill_name),
            &destination_root.join(&skill_name),
            mirror_mode,
        )?;
    }

    let destination_readme = destination_root.join("README.md");
    if destination_readme.exists() {
        fs::remove_file(&destination_readme).with_context(|| {
            format!("failed to remove '{}'", destination_readme.display())
        })?;
    }

    Ok(())
}

fn remove_managed_codex_skill_duplicates(
    managed_source_root: &Path,
    codex_skills_root: &Path,
) -> Result<()> {
    if !codex_skills_root.is_dir() || !managed_source_root.is_dir() {
        return Ok(());
    }

    for skill_name in fs::read_dir(managed_source_root)
        .with_context(|| format!("failed to enumerate '{}'", managed_source_root.display()))?
        .filter_map(std::result::Result::ok)
        .filter_map(|entry| {
            entry
                .file_type()
                .ok()
                .filter(|file_type| file_type.is_dir())
                .and_then(|_| entry.file_name().to_str().map(ToOwned::to_owned))
        })
    {
        let duplicate_path = codex_skills_root.join(&skill_name);
        if duplicate_path.exists() {
            fs::remove_dir_all(&duplicate_path).with_context(|| {
                format!("failed to remove '{}'", duplicate_path.display())
            })?;
        }
    }

    let managed_readme = managed_source_root.join("README.md");
    let duplicate_readme = codex_skills_root.join("README.md");
    if managed_readme.is_file()
        && duplicate_readme.is_file()
        && fs::read(&managed_readme)? == fs::read(&duplicate_readme)?
    {
        fs::remove_file(&duplicate_readme).with_context(|| {
            format!("failed to remove '{}'", duplicate_readme.display())
        })?;
    }

    Ok(())
}

fn remove_legacy_starter_skill_duplicates(skill_roots: &[PathBuf]) -> Result<()> {
    for skill_root in skill_roots {
        if !skill_root.is_dir() {
            continue;
        }

        for skill_name in ["super-agent", "using-super-agent"] {
            let candidate_path = skill_root.join(skill_name);
            if candidate_path.exists() {
                fs::remove_dir_all(&candidate_path).with_context(|| {
                    format!("failed to remove '{}'", candidate_path.display())
                })?;
            }
        }
    }

    Ok(())
}

fn sync_directory(source: &Path, destination: &Path, mirror_mode: bool) -> Result<()> {
    if !source.exists() {
        return Ok(());
    }

    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create '{}'", destination.display()))?;

    let source_files = WalkDir::new(source)
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("failed to enumerate '{}'", source.display()))?
        .into_iter()
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();

    let expected_files = source_files
        .iter()
        .map(|path| {
            path.strip_prefix(source)
                .map(|relative| relative.to_path_buf())
                .with_context(|| {
                    format!(
                        "failed to compute relative path for '{}' from '{}'",
                        path.display(),
                        source.display()
                    )
                })
        })
        .collect::<Result<BTreeSet<_>>>()?;

    for source_file in &source_files {
        let relative_path = source_file
            .strip_prefix(source)
            .with_context(|| {
                format!(
                    "failed to compute relative path for '{}' from '{}'",
                    source_file.display(),
                    source.display()
                )
            })?;
        let destination_file = destination.join(relative_path);
        if let Some(parent) = destination_file.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create '{}'", parent.display()))?;
        }

        fs::copy(source_file, &destination_file).with_context(|| {
            format!(
                "failed to copy '{}' to '{}'",
                source_file.display(),
                destination_file.display()
            )
        })?;
    }

    if mirror_mode {
        remove_mirrored_extras(destination, &expected_files)?;
    }

    Ok(())
}

fn remove_mirrored_extras(destination: &Path, expected_files: &BTreeSet<PathBuf>) -> Result<()> {
    if !destination.is_dir() {
        return Ok(());
    }

    let destination_files = WalkDir::new(destination)
        .contents_first(true)
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("failed to enumerate '{}'", destination.display()))?;

    for entry in destination_files {
        let path = entry.path();
        if path == destination {
            continue;
        }

        let relative_path = path
            .strip_prefix(destination)
            .with_context(|| {
                format!(
                    "failed to compute relative path for '{}' from '{}'",
                    path.display(),
                    destination.display()
                )
            })?
            .to_path_buf();

        if entry.file_type().is_file() && !expected_files.contains(&relative_path) {
            fs::remove_file(path)
                .with_context(|| format!("failed to remove '{}'", path.display()))?;
        } else if entry.file_type().is_dir() {
            let is_empty = fs::read_dir(path)
                .with_context(|| format!("failed to enumerate '{}'", path.display()))?
                .next()
                .is_none();
            if is_empty {
                fs::remove_dir(path)
                    .with_context(|| format!("failed to remove '{}'", path.display()))?;
            }
        }
    }

    Ok(())
}