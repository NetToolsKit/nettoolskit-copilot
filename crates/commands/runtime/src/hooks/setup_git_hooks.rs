//! Runtime Git hook setup.

use anyhow::{anyhow, Context};
use nettoolskit_core::path_utils::repository::resolve_workspace_root;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Output};

use crate::error::RuntimeSetupGitHooksCommandError;
use crate::hooks::eof_settings::{
    persist_git_hook_eof_mode_selection, remove_git_hook_eof_mode_selection,
    resolve_effective_git_hook_eof_mode, resolve_git_hook_eof_scope,
    resolve_git_hook_eof_settings_path_for_scope,
};

const LOCAL_REPO_SCOPE: &str = "local-repo";
const GLOBAL_SCOPE: &str = "global";
const LOCAL_HOOKS_PATH: &str = ".githooks";

/// Request payload for `setup-git-hooks`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeSetupGitHooksRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit EOF mode to persist.
    pub eof_hygiene_mode: Option<String>,
    /// Optional explicit EOF scope to persist.
    pub eof_hygiene_scope: Option<String>,
    /// Remove the selected hook ownership instead of configuring it.
    pub uninstall: bool,
    /// Optional explicit EOF catalog path.
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit global settings path used for deterministic tests.
    pub global_settings_path: Option<PathBuf>,
}

/// Result payload for `setup-git-hooks`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSetupGitHooksResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved scope name for the operation.
    pub scope_name: String,
    /// Selected mode name when the command configured hooks.
    pub mode_name: Option<String>,
    /// Whether uninstall semantics were used.
    pub uninstall: bool,
    /// Resolved settings path for the selected scope.
    pub settings_path: PathBuf,
    /// Whether the command wrote a selection file.
    pub selection_persisted: bool,
    /// Configured local hooks path when local ownership is active.
    pub hooks_path: Option<String>,
}

/// Configure or remove repository-local hook ownership and EOF selection state.
///
/// # Errors
///
/// Returns [`RuntimeSetupGitHooksCommandError`] when repository resolution,
/// catalog/scope validation, settings persistence, or Git hook-path updates
/// cannot complete.
pub fn invoke_setup_git_hooks(
    request: &RuntimeSetupGitHooksRequest,
) -> Result<RuntimeSetupGitHooksResult, RuntimeSetupGitHooksCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeSetupGitHooksCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_workspace_root(request.repo_root.as_deref(), Some(&current_dir))
        .map_err(|source| RuntimeSetupGitHooksCommandError::ResolveWorkspaceRoot { source })?;
    let requested_scope = request
        .eof_hygiene_scope
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(LOCAL_REPO_SCOPE);
    let resolved_scope = resolve_git_hook_eof_scope(
        &repo_root,
        request.catalog_path.as_deref(),
        Some(requested_scope),
    )
    .map_err(|source| RuntimeSetupGitHooksCommandError::ResolveMode { source })?;
    let settings_path = resolve_git_hook_eof_settings_path_for_scope(
        &repo_root,
        request.catalog_path.as_deref(),
        request.global_settings_path.as_deref(),
        Some(resolved_scope.name.as_str()),
    )
    .map_err(|source| RuntimeSetupGitHooksCommandError::PersistSettings { source })?;

    if request.uninstall {
        uninstall_git_hooks(&repo_root, resolved_scope.name.as_str())
            .map_err(|source| RuntimeSetupGitHooksCommandError::ConfigureHooks { source })?;
        remove_git_hook_eof_mode_selection(
            &repo_root,
            request.catalog_path.as_deref(),
            request.global_settings_path.as_deref(),
            Some(resolved_scope.name.as_str()),
        )
        .map_err(|source| RuntimeSetupGitHooksCommandError::PersistSettings { source })?;

        return Ok(RuntimeSetupGitHooksResult {
            repo_root,
            scope_name: resolved_scope.name,
            mode_name: None,
            uninstall: true,
            settings_path,
            selection_persisted: false,
            hooks_path: None,
        });
    }

    let effective_mode = resolve_effective_git_hook_eof_mode(
        &repo_root,
        request.catalog_path.as_deref(),
        request.global_settings_path.as_deref(),
    )
    .map_err(|source| RuntimeSetupGitHooksCommandError::ResolveMode { source })?;
    let mode_name = request
        .eof_hygiene_mode
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(effective_mode.name.as_str())
        .to_string();
    let should_persist_selection = request
        .eof_hygiene_mode
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
        || request
            .eof_hygiene_scope
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());

    if should_persist_selection {
        persist_git_hook_eof_mode_selection(
            &repo_root,
            request.catalog_path.as_deref(),
            request.global_settings_path.as_deref(),
            Some(mode_name.as_str()),
            Some(resolved_scope.name.as_str()),
        )
        .map_err(|source| RuntimeSetupGitHooksCommandError::PersistSettings { source })?;
    }

    let hooks_path = if resolved_scope.name == LOCAL_REPO_SCOPE {
        set_local_hooks_path(&repo_root, LOCAL_HOOKS_PATH)
            .map_err(|source| RuntimeSetupGitHooksCommandError::ConfigureHooks { source })?;
        Some(LOCAL_HOOKS_PATH.to_string())
    } else {
        unset_local_hooks_path(&repo_root)
            .map_err(|source| RuntimeSetupGitHooksCommandError::ConfigureHooks { source })?;
        remove_git_hook_eof_mode_selection(
            &repo_root,
            request.catalog_path.as_deref(),
            request.global_settings_path.as_deref(),
            Some(LOCAL_REPO_SCOPE),
        )
        .map_err(|source| RuntimeSetupGitHooksCommandError::PersistSettings { source })?;
        None
    };

    Ok(RuntimeSetupGitHooksResult {
        repo_root,
        scope_name: resolved_scope.name,
        mode_name: Some(mode_name),
        uninstall: false,
        settings_path,
        selection_persisted: should_persist_selection,
        hooks_path,
    })
}

fn uninstall_git_hooks(repo_root: &std::path::Path, scope_name: &str) -> anyhow::Result<()> {
    match scope_name {
        LOCAL_REPO_SCOPE => unset_local_hooks_path(repo_root),
        GLOBAL_SCOPE => Ok(()),
        _ => Err(anyhow!("unsupported git hook scope '{scope_name}'")),
    }
}

fn set_local_hooks_path(repo_root: &std::path::Path, hooks_path: &str) -> anyhow::Result<()> {
    run_git_config(
        repo_root,
        &["config", "--local", "core.hooksPath", hooks_path],
    )?;
    Ok(())
}

fn unset_local_hooks_path(repo_root: &std::path::Path) -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["config", "--local", "--unset", "core.hooksPath"])
        .output()
        .with_context(|| {
            format!(
                "failed to start local git hook uninstall for '{}'",
                repo_root.display()
            )
        })?;
    if output.status.success() || output.status.code() == Some(5) {
        Ok(())
    } else {
        Err(render_git_failure(output))
    }
}

fn run_git_config(repo_root: &std::path::Path, arguments: &[&str]) -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(arguments)
        .output()
        .with_context(|| format!("failed to start git with args {:?}", arguments))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(render_git_failure(output))
    }
}

fn render_git_failure(output: Output) -> anyhow::Error {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let message = if !stderr.is_empty() {
        stderr
    } else if !stdout.is_empty() {
        stdout
    } else {
        format!("git exited with {}", output.status)
    };

    anyhow!(message)
}