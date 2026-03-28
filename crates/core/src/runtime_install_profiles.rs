//! Runtime install profile catalog helpers.
//!
//! These helpers port the shared PowerShell runtime-install profile contract so
//! Rust runtime commands can resolve the same profile-driven behavior.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Install toggles exposed by one runtime profile.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInstallProfileInstall {
    /// Whether bootstrap/runtime projection should run.
    #[serde(default)]
    pub bootstrap: bool,
    /// Whether global VS Code settings should be installed.
    #[serde(default)]
    pub global_vscode_settings: bool,
    /// Whether global VS Code snippets should be installed.
    #[serde(default)]
    pub global_vscode_snippets: bool,
    /// Whether local git hooks should be installed.
    #[serde(default)]
    pub local_git_hooks: bool,
    /// Whether global git aliases should be installed.
    #[serde(default)]
    pub global_git_aliases: bool,
    /// Whether healthcheck should run.
    #[serde(default)]
    pub healthcheck: bool,
}

/// Runtime toggles exposed by one runtime profile.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInstallProfileRuntime {
    /// Whether the GitHub runtime is enabled.
    #[serde(default)]
    pub github: bool,
    /// Whether the Codex runtime is enabled.
    #[serde(default)]
    pub codex: bool,
    /// Whether the Claude runtime is enabled.
    #[serde(default)]
    pub claude: bool,
}

/// Raw profile node from the versioned catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInstallProfileNode {
    /// Human-readable profile description.
    pub description: String,
    /// Install behavior toggles.
    #[serde(default)]
    pub install: RuntimeInstallProfileInstall,
    /// Runtime surface toggles.
    #[serde(default)]
    pub runtime: RuntimeInstallProfileRuntime,
}

/// Versioned runtime install profile catalog document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInstallProfileCatalog {
    /// Schema version.
    pub schema_version: u32,
    /// Default profile name.
    pub default_profile: String,
    /// Profile entries keyed by name.
    pub profiles: BTreeMap<String, RuntimeInstallProfileNode>,
}

/// Catalog payload paired with the resolved file path it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeInstallProfileCatalogInfo {
    /// Resolved catalog path.
    pub path: PathBuf,
    /// Default profile name.
    pub default_profile: String,
    /// Available profile names in deterministic order.
    pub profile_names: Vec<String>,
    /// Parsed catalog payload.
    pub catalog: RuntimeInstallProfileCatalog,
}

/// Resolved profile contract used by runtime commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRuntimeInstallProfile {
    /// Selected profile name.
    pub name: String,
    /// Human-readable profile description.
    pub description: String,
    /// Catalog path used to resolve the profile.
    pub catalog_path: PathBuf,
    /// Catalog default profile.
    pub default_profile: String,
    /// Available profile names.
    pub available_profiles: Vec<String>,
    /// Install bootstrap toggle.
    pub install_bootstrap: bool,
    /// Install global VS Code settings toggle.
    pub install_global_vscode_settings: bool,
    /// Install global VS Code snippets toggle.
    pub install_global_vscode_snippets: bool,
    /// Install local git hooks toggle.
    pub install_local_git_hooks: bool,
    /// Install global git aliases toggle.
    pub install_global_git_aliases: bool,
    /// Install healthcheck toggle.
    pub install_healthcheck: bool,
    /// Enable GitHub runtime toggle.
    pub enable_github_runtime: bool,
    /// Enable Codex runtime toggle.
    pub enable_codex_runtime: bool,
    /// Enable Claude runtime toggle.
    pub enable_claude_runtime: bool,
}

/// Resolve the catalog path for versioned runtime install profiles.
#[must_use]
pub fn runtime_install_profile_catalog_path(resolved_repo_root: &Path) -> PathBuf {
    resolved_repo_root.join(".github/governance/runtime-install-profiles.json")
}

/// Load the runtime install profile catalog.
///
/// # Errors
///
/// Returns an error when the catalog is missing or invalid.
pub fn read_runtime_install_profile_catalog(
    resolved_repo_root: &Path,
) -> Result<RuntimeInstallProfileCatalogInfo> {
    let catalog_path = runtime_install_profile_catalog_path(resolved_repo_root);
    let payload = fs::read_to_string(&catalog_path).with_context(|| {
        format!(
            "missing runtime install profile catalog '{}'",
            catalog_path.display()
        )
    })?;
    let catalog =
        serde_json::from_str::<RuntimeInstallProfileCatalog>(&payload).with_context(|| {
            format!(
                "invalid runtime install profile catalog '{}'",
                catalog_path.display()
            )
        })?;

    if catalog.default_profile.trim().is_empty() {
        return Err(anyhow!(
            "runtime install profile catalog is missing defaultProfile: {}",
            catalog_path.display()
        ));
    }

    let mut profile_names = catalog.profiles.keys().cloned().collect::<Vec<_>>();
    profile_names.sort();
    if profile_names.is_empty() {
        return Err(anyhow!(
            "runtime install profile catalog does not define any profiles: {}",
            catalog_path.display()
        ));
    }

    Ok(RuntimeInstallProfileCatalogInfo {
        path: catalog_path,
        default_profile: catalog.default_profile.clone(),
        profile_names,
        catalog,
    })
}

/// Resolve the effective runtime install profile for a command invocation.
///
/// # Errors
///
/// Returns an error when the requested profile does not exist or the catalog
/// cannot be read.
pub fn resolve_runtime_install_profile(
    resolved_repo_root: &Path,
    profile_name: Option<&str>,
    fallback_profile_name: Option<&str>,
) -> Result<ResolvedRuntimeInstallProfile> {
    let catalog_info = read_runtime_install_profile_catalog(resolved_repo_root)?;
    let effective_profile_name = profile_name
        .filter(|value| !value.trim().is_empty())
        .or_else(|| fallback_profile_name.filter(|value| !value.trim().is_empty()))
        .unwrap_or(&catalog_info.default_profile);

    let profile = catalog_info
        .catalog
        .profiles
        .get(effective_profile_name)
        .ok_or_else(|| {
            anyhow!(
                "unknown runtime profile '{}'. Valid profiles: {}",
                effective_profile_name,
                catalog_info.profile_names.join(", ")
            )
        })?;

    Ok(ResolvedRuntimeInstallProfile {
        name: effective_profile_name.to_string(),
        description: profile.description.clone(),
        catalog_path: catalog_info.path,
        default_profile: catalog_info.default_profile,
        available_profiles: catalog_info.profile_names,
        install_bootstrap: profile.install.bootstrap,
        install_global_vscode_settings: profile.install.global_vscode_settings,
        install_global_vscode_snippets: profile.install.global_vscode_snippets,
        install_local_git_hooks: profile.install.local_git_hooks,
        install_global_git_aliases: profile.install.global_git_aliases,
        install_healthcheck: profile.install.healthcheck,
        enable_github_runtime: profile.runtime.github,
        enable_codex_runtime: profile.runtime.codex,
        enable_claude_runtime: profile.runtime.claude,
    })
}
