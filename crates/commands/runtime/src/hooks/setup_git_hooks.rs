//! Runtime Git hook setup.

use anyhow::{anyhow, Context};
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_workspace_root};
use nettoolskit_core::runtime_locations::resolve_user_home_path;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
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
const DEFAULT_GLOBAL_HOOKS_RELATIVE_PATH: &str = ".codex/git-hooks";
const CATALOG_RELATIVE_PATH: &str = ".github/governance/git-hook-eof-modes.json";
const PRE_COMMIT_FILE_NAME: &str = "pre-commit";
const PRE_COMMIT_RUNNER_RELATIVE_PATH: &str = "scripts/git-hooks/invoke-pre-commit-eof-hygiene.ps1";

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
    /// Optional explicit managed global hooks path used for deterministic tests.
    pub git_hooks_path: Option<PathBuf>,
    /// Optional isolated global Git config path used for deterministic tests.
    pub git_config_global_path: Option<PathBuf>,
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
    /// Configured hook path when ownership is active.
    pub hooks_path: Option<String>,
    /// Optional isolated global Git config path used for the command.
    pub git_config_global_path: Option<PathBuf>,
}

/// Configure or remove repository-local or managed-global hook ownership.
///
/// # Errors
///
/// Returns [`RuntimeSetupGitHooksCommandError`] when workspace resolution,
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
    let git_config_global_path =
        resolve_optional_path(&repo_root, request.git_config_global_path.as_deref());
    prepare_git_config_parent(git_config_global_path.as_deref())
        .map_err(|source| RuntimeSetupGitHooksCommandError::ConfigureHooks { source })?;

    if request.uninstall {
        uninstall_git_hooks(
            &repo_root,
            resolved_scope.name.as_str(),
            request.git_hooks_path.as_deref(),
            git_config_global_path.as_deref(),
        )
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
            git_config_global_path,
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
        let global_hooks_path =
            resolve_global_hooks_path(&repo_root, request.git_hooks_path.as_deref())
                .map_err(|source| RuntimeSetupGitHooksCommandError::ConfigureHooks { source })?;
        install_managed_global_git_hooks(
            &repo_root,
            request.catalog_path.as_deref(),
            &global_hooks_path,
            git_config_global_path.as_deref(),
        )
        .map_err(|source| RuntimeSetupGitHooksCommandError::ConfigureHooks { source })?;
        unset_local_hooks_path(&repo_root)
            .map_err(|source| RuntimeSetupGitHooksCommandError::ConfigureHooks { source })?;
        remove_git_hook_eof_mode_selection(
            &repo_root,
            request.catalog_path.as_deref(),
            request.global_settings_path.as_deref(),
            Some(LOCAL_REPO_SCOPE),
        )
        .map_err(|source| RuntimeSetupGitHooksCommandError::PersistSettings { source })?;
        Some(global_hooks_path.display().to_string())
    };

    Ok(RuntimeSetupGitHooksResult {
        repo_root,
        scope_name: resolved_scope.name,
        mode_name: Some(mode_name),
        uninstall: false,
        settings_path,
        selection_persisted: should_persist_selection,
        hooks_path,
        git_config_global_path,
    })
}

fn uninstall_git_hooks(
    repo_root: &Path,
    scope_name: &str,
    requested_git_hooks_path: Option<&Path>,
    git_config_global_path: Option<&Path>,
) -> anyhow::Result<()> {
    match scope_name {
        LOCAL_REPO_SCOPE => unset_local_hooks_path(repo_root),
        GLOBAL_SCOPE => uninstall_managed_global_git_hooks(
            repo_root,
            requested_git_hooks_path,
            git_config_global_path,
        ),
        _ => Err(anyhow!("unsupported git hook scope '{scope_name}'")),
    }
}

fn set_local_hooks_path(repo_root: &Path, hooks_path: &str) -> anyhow::Result<()> {
    run_git_config(
        repo_root,
        &["config", "--local", "core.hooksPath", hooks_path],
        None,
    )
}

fn unset_local_hooks_path(repo_root: &Path) -> anyhow::Result<()> {
    let output = run_git(
        repo_root,
        &["config", "--local", "--unset", "core.hooksPath"],
        None,
    )?;
    if output.status.success() || output.status.code() == Some(5) {
        Ok(())
    } else {
        Err(render_git_failure(output))
    }
}

fn resolve_optional_path(repo_root: &Path, requested_path: Option<&Path>) -> Option<PathBuf> {
    requested_path.map(|path| resolve_full_path(repo_root, path))
}

fn prepare_git_config_parent(config_path: Option<&Path>) -> anyhow::Result<()> {
    let Some(config_path) = config_path else {
        return Ok(());
    };

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }

    Ok(())
}

fn resolve_global_hooks_path(
    repo_root: &Path,
    requested_path: Option<&Path>,
) -> anyhow::Result<PathBuf> {
    if let Some(requested_path) = requested_path {
        return Ok(resolve_full_path(repo_root, requested_path));
    }

    if let Some(path) = env::var_os("CODEX_GIT_HOOKS_PATH") {
        return Ok(PathBuf::from(path));
    }

    Ok(resolve_user_home_path()?.join(DEFAULT_GLOBAL_HOOKS_RELATIVE_PATH))
}

fn install_managed_global_git_hooks(
    repo_root: &Path,
    catalog_path_override: Option<&Path>,
    global_hooks_path: &Path,
    git_config_global_path: Option<&Path>,
) -> anyhow::Result<()> {
    let support_paths = resolve_global_hook_support_paths(repo_root, catalog_path_override)?;
    fs::create_dir_all(global_hooks_path)
        .with_context(|| format!("failed to create '{}'", global_hooks_path.display()))?;

    let pre_commit_path = global_hooks_path.join(PRE_COMMIT_FILE_NAME);
    fs::write(
        &pre_commit_path,
        build_managed_global_pre_commit_hook_content(
            &support_paths.runner_path,
            &support_paths.catalog_path,
        ),
    )
    .with_context(|| format!("failed to write '{}'", pre_commit_path.display()))?;
    make_hook_executable_on_unix(&pre_commit_path)?;

    let global_hooks_path_text = global_hooks_path.to_string_lossy().to_string();
    run_git_config(
        repo_root,
        &[
            "config",
            "--global",
            "core.hooksPath",
            &global_hooks_path_text,
        ],
        git_config_global_path,
    )?;

    Ok(())
}

fn uninstall_managed_global_git_hooks(
    repo_root: &Path,
    requested_git_hooks_path: Option<&Path>,
    git_config_global_path: Option<&Path>,
) -> anyhow::Result<()> {
    let managed_global_hooks_path = resolve_global_hooks_path(repo_root, requested_git_hooks_path)?;
    let current_global_hooks_path = read_git_config(
        repo_root,
        &["config", "--global", "--get", "core.hooksPath"],
        git_config_global_path,
    )?;
    if current_global_hooks_path
        .as_deref()
        .is_some_and(|current| paths_match(current, &managed_global_hooks_path))
    {
        let _ = run_git_config(
            repo_root,
            &["config", "--global", "--unset", "core.hooksPath"],
            git_config_global_path,
        );
    }

    if managed_global_hooks_path.is_dir() {
        fs::remove_dir_all(&managed_global_hooks_path).with_context(|| {
            format!("failed to remove '{}'", managed_global_hooks_path.display())
        })?;
    }

    Ok(())
}

fn resolve_global_hook_support_paths(
    repo_root: &Path,
    catalog_path_override: Option<&Path>,
) -> anyhow::Result<GlobalHookSupportPaths> {
    let catalog_path = resolve_full_path(
        repo_root,
        catalog_path_override.unwrap_or_else(|| Path::new(CATALOG_RELATIVE_PATH)),
    );
    if !catalog_path.is_file() {
        return Err(anyhow!(
            "missing global hook catalog '{}'",
            catalog_path.display()
        ));
    }

    let runner_path = repo_root.join(PRE_COMMIT_RUNNER_RELATIVE_PATH);
    if !runner_path.is_file() {
        return Err(anyhow!(
            "missing global hook runner '{}'",
            runner_path.display()
        ));
    }

    Ok(GlobalHookSupportPaths {
        runner_path,
        catalog_path,
    })
}

fn build_managed_global_pre_commit_hook_content(runner_path: &Path, catalog_path: &Path) -> String {
    format!(
        "#!/usr/bin/env sh\nset -eu\n\nREPO_ROOT=\"$(git rev-parse --show-toplevel 2>/dev/null || pwd)\"\ncd \"$REPO_ROOT\"\n\nexport CODEX_GIT_HOOK_EOF_CATALOG_PATH='{1}'\n\nif command -v pwsh >/dev/null 2>&1; then\n  if ! pwsh -NoLogo -NoProfile -ExecutionPolicy Bypass -File '{0}' -RepoRoot \"$REPO_ROOT\"; then\n    echo \"[pre-commit] Error: EOF hygiene hook failed.\" >&2\n    exit 1\n  fi\n  exit 0\nfi\n\nif command -v powershell >/dev/null 2>&1; then\n  if ! powershell -NoLogo -NoProfile -ExecutionPolicy Bypass -File '{0}' -RepoRoot \"$REPO_ROOT\"; then\n    echo \"[pre-commit] Error: EOF hygiene hook failed.\" >&2\n    exit 1\n  fi\n  exit 0\nfi\n\necho \"[pre-commit] Warning: PowerShell not found. EOF hygiene skipped.\" >&2\nexit 0\n",
        normalize_shell_path(runner_path),
        normalize_shell_path(catalog_path),
    )
}

fn make_hook_executable_on_unix(path: &Path) -> anyhow::Result<()> {
    if cfg!(windows) {
        return Ok(());
    }

    let output = Command::new("chmod")
        .args(["+x", path.to_string_lossy().as_ref()])
        .output()
        .with_context(|| format!("failed to start chmod for '{}'", path.display()))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(render_git_failure(output))
    }
}

fn normalize_shell_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn read_git_config(
    repo_root: &Path,
    arguments: &[&str],
    git_config_global_path: Option<&Path>,
) -> anyhow::Result<Option<String>> {
    let output = run_git(repo_root, arguments, git_config_global_path)?;
    if !output.status.success() {
        return Ok(None);
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

fn paths_match(candidate: &str, expected: &Path) -> bool {
    let candidate_path = PathBuf::from(candidate.trim());
    let normalized_candidate = candidate_path.canonicalize().unwrap_or(candidate_path);
    let normalized_expected = expected
        .canonicalize()
        .unwrap_or_else(|_| expected.to_path_buf());
    normalized_candidate == normalized_expected
}

fn run_git_config(
    repo_root: &Path,
    arguments: &[&str],
    git_config_global_path: Option<&Path>,
) -> anyhow::Result<()> {
    let output = run_git(repo_root, arguments, git_config_global_path)?;
    if output.status.success() {
        Ok(())
    } else {
        Err(render_git_failure(output))
    }
}

fn run_git(
    repo_root: &Path,
    arguments: &[&str],
    git_config_global_path: Option<&Path>,
) -> anyhow::Result<Output> {
    let mut command = Command::new("git");
    command.arg("-C").arg(repo_root).args(arguments);
    if let Some(git_config_global_path) = git_config_global_path {
        command.env("GIT_CONFIG_GLOBAL", git_config_global_path);
    }

    command
        .output()
        .with_context(|| format!("failed to start git with args {:?}", arguments))
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

#[derive(Debug, Clone)]
struct GlobalHookSupportPaths {
    runner_path: PathBuf,
    catalog_path: PathBuf,
}
