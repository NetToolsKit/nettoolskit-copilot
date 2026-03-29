//! Shared EOF hygiene settings resolution for runtime hook commands.

use anyhow::{anyhow, Context};
use nettoolskit_core::runtime_locations::resolve_user_home_path;
use serde_json::json;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_CATALOG_RELATIVE_PATH: &str = ".github/governance/git-hook-eof-modes.json";
const DEFAULT_GLOBAL_SETTINGS_RELATIVE_PATH: &str = ".codex/git-hook-eof-settings.json";

/// Effective EOF mode resolved for the current repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveGitHookEofMode {
    /// Resolved mode name.
    pub name: String,
    /// Whether staged files should be auto-fixed.
    pub auto_fix_staged_files: bool,
    /// Resolved scope name.
    pub scope: String,
    /// Resolution source identifier.
    pub source: String,
    /// Catalog path used for the resolution.
    pub catalog_path: PathBuf,
    /// Optional settings path selected by the resolution.
    pub settings_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GitHookEofCatalog {
    default_mode: String,
    default_scope: String,
    catalog_path: PathBuf,
    document: Value,
}

/// One supported EOF mode from the catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GitHookEofModeDefinition {
    /// Catalog mode name.
    pub name: String,
    /// Whether the mode enables staged-file autofix.
    pub auto_fix_staged_files: bool,
    /// Catalog path that defined the mode.
    pub catalog_path: PathBuf,
}

/// One supported EOF scope from the catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GitHookEofScopeDefinition {
    /// Catalog scope name.
    pub name: String,
    /// Catalog path that defined the scope.
    pub catalog_path: PathBuf,
}

/// Persisted EOF mode selection details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GitHookEofSelection {
    /// Selected mode definition.
    pub mode: GitHookEofModeDefinition,
    /// Selected scope definition.
    pub scope: GitHookEofScopeDefinition,
    /// Settings path written for the selection.
    pub settings_path: PathBuf,
}

/// Resolve the effective EOF mode using repository-local settings first,
/// global settings second, and catalog defaults last.
pub(crate) fn resolve_effective_git_hook_eof_mode(
    repo_root: &Path,
    catalog_path_override: Option<&Path>,
    global_settings_path_override: Option<&Path>,
) -> anyhow::Result<EffectiveGitHookEofMode> {
    let catalog = load_git_hook_eof_catalog(repo_root, catalog_path_override)?;
    let local_settings_path = resolve_local_git_hook_eof_settings_path(repo_root)?;
    if let Some(mode_name) = read_selected_mode(&local_settings_path)? {
        let mode = resolve_catalog_mode(&catalog, &mode_name)?;
        return Ok(EffectiveGitHookEofMode {
            name: mode_name,
            auto_fix_staged_files: mode,
            scope: "local-repo".to_string(),
            source: "local-settings".to_string(),
            catalog_path: catalog.catalog_path,
            settings_path: Some(local_settings_path),
        });
    }

    let global_settings_path =
        resolve_global_git_hook_eof_settings_path(global_settings_path_override)?;
    if let Some(mode_name) = read_selected_mode(&global_settings_path)? {
        let mode = resolve_catalog_mode(&catalog, &mode_name)?;
        return Ok(EffectiveGitHookEofMode {
            name: mode_name,
            auto_fix_staged_files: mode,
            scope: "global".to_string(),
            source: "global-settings".to_string(),
            catalog_path: catalog.catalog_path,
            settings_path: Some(global_settings_path),
        });
    }

    let auto_fix_staged_files = resolve_catalog_mode(&catalog, &catalog.default_mode)?;
    Ok(EffectiveGitHookEofMode {
        name: catalog.default_mode,
        auto_fix_staged_files,
        scope: catalog.default_scope,
        source: "catalog-default".to_string(),
        catalog_path: catalog.catalog_path,
        settings_path: None,
    })
}

/// Resolve the repository-local settings path backed by `.git/`.
pub(crate) fn resolve_local_git_hook_eof_settings_path(
    repo_root: &Path,
) -> anyhow::Result<PathBuf> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("rev-parse")
        .arg("--git-path")
        .arg("codex-hook-eof-settings.json")
        .output()
        .with_context(|| {
            format!(
                "failed to resolve git EOF settings path for '{}'",
                repo_root.display()
            )
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(anyhow!(if stderr.is_empty() {
            "git rev-parse --git-path failed".to_string()
        } else {
            stderr
        }));
    }

    let raw_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if raw_path.is_empty() {
        return Err(anyhow!(
            "git rev-parse --git-path returned an empty settings path"
        ));
    }

    let path = PathBuf::from(raw_path);
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(repo_root.join(path))
    }
}

/// Resolve the machine-global settings path for EOF mode selection.
pub(crate) fn resolve_global_git_hook_eof_settings_path(
    override_path: Option<&Path>,
) -> anyhow::Result<PathBuf> {
    if let Some(override_path) = override_path {
        return Ok(override_path.to_path_buf());
    }

    if let Some(path) = std::env::var_os("CODEX_GIT_HOOK_EOF_SETTINGS_PATH") {
        return Ok(PathBuf::from(path));
    }

    let home_path = resolve_user_home_path()?;
    Ok(home_path.join(DEFAULT_GLOBAL_SETTINGS_RELATIVE_PATH))
}

/// Resolve one catalog mode, falling back to the catalog default when omitted.
pub(crate) fn resolve_git_hook_eof_mode(
    repo_root: &Path,
    catalog_path_override: Option<&Path>,
    mode_name: Option<&str>,
) -> anyhow::Result<GitHookEofModeDefinition> {
    let catalog = load_git_hook_eof_catalog(repo_root, catalog_path_override)?;
    let effective_mode_name = mode_name
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(&catalog.default_mode);
    let auto_fix_staged_files = resolve_catalog_mode(&catalog, effective_mode_name)?;

    Ok(GitHookEofModeDefinition {
        name: effective_mode_name.to_string(),
        auto_fix_staged_files,
        catalog_path: catalog.catalog_path,
    })
}

/// Resolve one catalog scope, falling back to the catalog default when omitted.
pub(crate) fn resolve_git_hook_eof_scope(
    repo_root: &Path,
    catalog_path_override: Option<&Path>,
    scope_name: Option<&str>,
) -> anyhow::Result<GitHookEofScopeDefinition> {
    let catalog = load_git_hook_eof_catalog(repo_root, catalog_path_override)?;
    let effective_scope_name = scope_name
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(&catalog.default_scope);
    resolve_catalog_scope(&catalog, effective_scope_name)?;

    Ok(GitHookEofScopeDefinition {
        name: effective_scope_name.to_string(),
        catalog_path: catalog.catalog_path,
    })
}

/// Resolve the settings path for one explicit scope.
pub(crate) fn resolve_git_hook_eof_settings_path_for_scope(
    repo_root: &Path,
    catalog_path_override: Option<&Path>,
    global_settings_path_override: Option<&Path>,
    scope_name: Option<&str>,
) -> anyhow::Result<PathBuf> {
    let scope = resolve_git_hook_eof_scope(repo_root, catalog_path_override, scope_name)?;
    if scope.name == "global" {
        resolve_global_git_hook_eof_settings_path(global_settings_path_override)
    } else {
        resolve_local_git_hook_eof_settings_path(repo_root)
    }
}

/// Persist one explicit mode selection for the requested scope.
pub(crate) fn persist_git_hook_eof_mode_selection(
    repo_root: &Path,
    catalog_path_override: Option<&Path>,
    global_settings_path_override: Option<&Path>,
    mode_name: Option<&str>,
    scope_name: Option<&str>,
) -> anyhow::Result<GitHookEofSelection> {
    let mode = resolve_git_hook_eof_mode(repo_root, catalog_path_override, mode_name)?;
    let scope = resolve_git_hook_eof_scope(repo_root, catalog_path_override, scope_name)?;
    let settings_path = resolve_git_hook_eof_settings_path_for_scope(
        repo_root,
        catalog_path_override,
        global_settings_path_override,
        Some(scope.name.as_str()),
    )?;

    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }

    let payload = json!({
        "schemaVersion": 1,
        "selectedMode": mode.name,
        "selectedScope": scope.name,
        "catalogPath": mode.catalog_path,
    });
    let document = serde_json::to_string(&payload).context("failed to serialize EOF selection")?;
    fs::write(&settings_path, document)
        .with_context(|| format!("failed to write '{}'", settings_path.display()))?;

    Ok(GitHookEofSelection {
        mode,
        scope,
        settings_path,
    })
}

/// Remove the persisted mode selection for the requested scope when present.
pub(crate) fn remove_git_hook_eof_mode_selection(
    repo_root: &Path,
    catalog_path_override: Option<&Path>,
    global_settings_path_override: Option<&Path>,
    scope_name: Option<&str>,
) -> anyhow::Result<PathBuf> {
    let settings_path = resolve_git_hook_eof_settings_path_for_scope(
        repo_root,
        catalog_path_override,
        global_settings_path_override,
        scope_name,
    )?;
    if settings_path.is_file() {
        fs::remove_file(&settings_path)
            .with_context(|| format!("failed to remove '{}'", settings_path.display()))?;
    }

    Ok(settings_path)
}

fn load_git_hook_eof_catalog(
    repo_root: &Path,
    override_path: Option<&Path>,
) -> anyhow::Result<GitHookEofCatalog> {
    let catalog_path = match override_path {
        Some(path) => path.to_path_buf(),
        None => repo_root.join(DEFAULT_CATALOG_RELATIVE_PATH),
    };
    let document = fs::read_to_string(&catalog_path)
        .with_context(|| format!("failed to read '{}'", catalog_path.display()))?;
    let value: Value = serde_json::from_str(&document)
        .with_context(|| format!("failed to parse '{}'", catalog_path.display()))?;

    let default_mode = value
        .get("defaultMode")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("EOF catalog is missing defaultMode"))?
        .to_string();
    let default_scope = value
        .get("defaultScope")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("EOF catalog is missing defaultScope"))?
        .to_string();

    Ok(GitHookEofCatalog {
        default_mode,
        default_scope,
        catalog_path,
        document: value,
    })
}

fn read_selected_mode(settings_path: &Path) -> anyhow::Result<Option<String>> {
    if !settings_path.is_file() {
        return Ok(None);
    }

    let document = fs::read_to_string(settings_path)
        .with_context(|| format!("failed to read '{}'", settings_path.display()))?;
    let value: Value = serde_json::from_str(&document)
        .with_context(|| format!("failed to parse '{}'", settings_path.display()))?;

    Ok(value
        .get("selectedMode")
        .and_then(Value::as_str)
        .filter(|mode| !mode.trim().is_empty())
        .map(ToOwned::to_owned))
}

fn resolve_catalog_mode(catalog: &GitHookEofCatalog, mode_name: &str) -> anyhow::Result<bool> {
    catalog
        .document
        .get("modes")
        .and_then(|modes| modes.get(mode_name))
        .and_then(|mode| mode.get("autoFixStagedFiles"))
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            anyhow!(
                "unknown EOF mode '{mode_name}' in '{}'",
                catalog.catalog_path.display()
            )
        })
}

fn resolve_catalog_scope(catalog: &GitHookEofCatalog, scope_name: &str) -> anyhow::Result<()> {
    let scope_exists = catalog
        .document
        .get("scopes")
        .and_then(|scopes| scopes.get(scope_name))
        .is_some();
    if scope_exists {
        Ok(())
    } else {
        Err(anyhow!(
            "unknown EOF scope '{scope_name}' in '{}'",
            catalog.catalog_path.display()
        ))
    }
}
