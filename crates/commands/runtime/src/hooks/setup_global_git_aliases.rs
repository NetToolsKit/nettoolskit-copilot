//! Runtime global Git alias setup.

use anyhow::{anyhow, Context};
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use nettoolskit_core::runtime_locations::resolve_codex_runtime_path;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::RuntimeSetupGlobalGitAliasesCommandError;

const TRIM_ALIAS_NAME: &str = "trim-eof";
const RUNTIME_BINARY_DIRECTORY: &str = "bin";

/// Request payload for `setup-global-git-aliases`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeSetupGlobalGitAliasesRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit Codex runtime root.
    pub target_codex_path: Option<PathBuf>,
    /// Remove managed aliases instead of configuring them.
    pub uninstall: bool,
    /// Optional isolated global Git config path used for deterministic tests.
    pub git_config_global_path: Option<PathBuf>,
}

/// Result payload for `setup-global-git-aliases`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSetupGlobalGitAliasesResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved Codex runtime root.
    pub target_codex_path: PathBuf,
    /// Whether the operation performed uninstall semantics.
    pub uninstall: bool,
    /// Optional isolated global Git config path used for the command.
    pub git_config_global_path: Option<PathBuf>,
    /// Managed aliases present after the command completes.
    pub configured_aliases: BTreeMap<String, String>,
    /// Managed aliases removed by the command.
    pub removed_aliases: Vec<String>,
}

/// Configure or remove managed global Git aliases.
///
/// # Errors
///
/// Returns [`RuntimeSetupGlobalGitAliasesCommandError`] when repository or
/// runtime path resolution fails, when the runtime-synced `ntk` binary is
/// missing, or when `git config --global` fails.
pub fn invoke_setup_global_git_aliases(
    request: &RuntimeSetupGlobalGitAliasesRequest,
) -> Result<RuntimeSetupGlobalGitAliasesResult, RuntimeSetupGlobalGitAliasesCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeSetupGlobalGitAliasesCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(
            |source| RuntimeSetupGlobalGitAliasesCommandError::ResolveWorkspaceRoot { source },
        )?;
    let target_codex_path =
        resolve_target_codex_path(&repo_root, request.target_codex_path.as_deref()).map_err(
            |source| RuntimeSetupGlobalGitAliasesCommandError::ResolveTargetCodexPath { source },
        )?;
    let git_config_global_path =
        resolve_optional_path(&repo_root, request.git_config_global_path.as_deref());

    prepare_git_config_parent(git_config_global_path.as_deref()).map_err(|source| {
        RuntimeSetupGlobalGitAliasesCommandError::PrepareGitConfigPath { source }
    })?;

    let managed_aliases = build_managed_alias_map(&target_codex_path)?;
    if request.uninstall {
        let mut removed_aliases = Vec::new();
        for alias_name in managed_aliases.keys() {
            if let Some(removed_alias) =
                uninstall_alias(alias_name, git_config_global_path.as_deref()).map_err(
                    |source| RuntimeSetupGlobalGitAliasesCommandError::ConfigureAliases { source },
                )?
            {
                removed_aliases.push(removed_alias);
            }
        }

        return Ok(RuntimeSetupGlobalGitAliasesResult {
            repo_root,
            target_codex_path,
            uninstall: true,
            git_config_global_path,
            configured_aliases: BTreeMap::new(),
            removed_aliases,
        });
    }

    for (alias_name, alias_command) in &managed_aliases {
        set_alias(alias_name, alias_command, git_config_global_path.as_deref()).map_err(
            |source| RuntimeSetupGlobalGitAliasesCommandError::ConfigureAliases { source },
        )?;
    }

    let mut configured_aliases = BTreeMap::new();
    for alias_name in managed_aliases.keys() {
        if let Some(value) =
            read_alias(alias_name, git_config_global_path.as_deref()).map_err(|source| {
                RuntimeSetupGlobalGitAliasesCommandError::ConfigureAliases { source }
            })?
        {
            configured_aliases.insert(alias_name.clone(), value);
        }
    }

    Ok(RuntimeSetupGlobalGitAliasesResult {
        repo_root,
        target_codex_path,
        uninstall: false,
        git_config_global_path,
        configured_aliases,
        removed_aliases: Vec::new(),
    })
}

fn resolve_target_codex_path(
    repo_root: &Path,
    requested_path: Option<&Path>,
) -> anyhow::Result<PathBuf> {
    match requested_path {
        Some(path) => Ok(resolve_full_path(repo_root, path)),
        None => Ok(resolve_codex_runtime_path()),
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

fn build_managed_alias_map(
    target_codex_path: &Path,
) -> Result<BTreeMap<String, String>, RuntimeSetupGlobalGitAliasesCommandError> {
    let runtime_binary_path = runtime_binary_path(target_codex_path);
    if !runtime_binary_path.is_file() {
        return Err(
            RuntimeSetupGlobalGitAliasesCommandError::TrimScriptNotFound {
                trim_script_path: runtime_binary_path.display().to_string(),
            },
        );
    }

    let mut aliases = BTreeMap::new();
    aliases.insert(
        TRIM_ALIAS_NAME.to_string(),
        format!(
            "!'{}' runtime trim-trailing-blank-lines --repo-root \"$(git rev-parse --show-toplevel 2>/dev/null || pwd)\" --git-changed-only",
            normalize_shell_path(&runtime_binary_path)
        ),
    );
    Ok(aliases)
}

fn runtime_binary_path(runtime_root: &Path) -> PathBuf {
    runtime_root
        .join(RUNTIME_BINARY_DIRECTORY)
        .join(runtime_binary_file_name())
}

fn runtime_binary_file_name() -> &'static str {
    if cfg!(windows) { "ntk.exe" } else { "ntk" }
}

fn normalize_shell_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn set_alias(
    alias_name: &str,
    alias_command: &str,
    config_path: Option<&Path>,
) -> anyhow::Result<()> {
    run_git_config_command(
        config_path,
        &[format!("alias.{alias_name}"), alias_command.to_string()],
        true,
    )
    .map(|_| ())
}

fn uninstall_alias(alias_name: &str, config_path: Option<&Path>) -> anyhow::Result<Option<String>> {
    if read_alias(alias_name, config_path)?.is_none() {
        return Ok(None);
    }

    run_git_config_command(
        config_path,
        &["--unset-all".to_string(), format!("alias.{alias_name}")],
        true,
    )?;
    Ok(Some(alias_name.to_string()))
}

fn read_alias(alias_name: &str, config_path: Option<&Path>) -> anyhow::Result<Option<String>> {
    let output = run_git_config_command(
        config_path,
        &["--get".to_string(), format!("alias.{alias_name}")],
        false,
    )?;
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

fn run_git_config_command(
    config_path: Option<&Path>,
    arguments: &[String],
    require_success: bool,
) -> anyhow::Result<std::process::Output> {
    let mut command = Command::new("git");
    command.arg("config").arg("--global").args(arguments);
    if let Some(config_path) = config_path {
        command.env("GIT_CONFIG_GLOBAL", config_path);
    }

    let output = command
        .output()
        .with_context(|| format!("failed to start git config with args {:?}", arguments))?;
    if require_success && !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let message = if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            format!("git config exited with {}", output.status)
        };
        return Err(anyhow!(message));
    }

    Ok(output)
}
