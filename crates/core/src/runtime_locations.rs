//! Runtime location catalog helpers.
//!
//! These helpers model the repository-owned runtime path catalog used by the
//! legacy PowerShell scripts so Rust commands can resolve the same effective
//! runtime roots deterministically.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::path_utils::repository::resolve_full_path;

/// Runtime location catalog settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeLocationCatalogSettings {
    /// Relative path to the optional user-local override settings document.
    pub user_override_relative_path: String,
}

/// Versioned runtime location catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeLocationCatalog {
    /// Schema version for the catalog contract.
    pub schema_version: u32,
    /// Catalog-level settings.
    pub settings: RuntimeLocationCatalogSettings,
    /// Named runtime root paths.
    pub paths: BTreeMap<String, String>,
}

/// Optional user-local runtime path overrides.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeLocationOverrides {
    /// Optional path overrides keyed by catalog name.
    #[serde(default)]
    pub paths: BTreeMap<String, String>,
}

/// Effective runtime locations resolved from the catalog and optional
/// overrides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveRuntimeLocations {
    /// `.github` runtime root.
    pub github_runtime_root: PathBuf,
    /// `.codex` runtime root.
    pub codex_runtime_root: PathBuf,
    /// `.agents/skills` discovery root.
    pub agents_skills_root: PathBuf,
    /// `.copilot/skills` discovery root.
    pub copilot_skills_root: PathBuf,
    /// `.codex/git-hooks` root.
    pub codex_git_hooks_root: PathBuf,
    /// `.claude` runtime root.
    pub claude_runtime_root: PathBuf,
}

/// Resolve the current user's home directory.
///
/// # Errors
///
/// Returns an error when no platform home directory can be determined.
pub fn resolve_user_home_path() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("USERPROFILE") {
        return Ok(PathBuf::from(path));
    }

    if let Some(path) = std::env::var_os("HOME") {
        return Ok(PathBuf::from(path));
    }

    Err(anyhow::anyhow!("could not resolve home path"))
}

/// Join path segments using the current platform separator rules.
#[must_use]
pub fn join_path_segments(base_path: &Path, segments: &[&str]) -> PathBuf {
    segments
        .iter()
        .fold(base_path.to_path_buf(), |current, segment| {
            if segment.trim().is_empty() {
                current
            } else {
                current.join(segment)
            }
        })
}

/// Return the built-in runtime location catalog.
#[must_use]
pub fn built_in_runtime_location_catalog() -> RuntimeLocationCatalog {
    let mut paths = BTreeMap::new();
    paths.insert(
        "githubRuntimeRoot".to_string(),
        "${HOME}/.github".to_string(),
    );
    paths.insert("codexRuntimeRoot".to_string(), "${HOME}/.codex".to_string());
    paths.insert(
        "agentsSkillsRoot".to_string(),
        "${HOME}/.agents/skills".to_string(),
    );
    paths.insert(
        "copilotSkillsRoot".to_string(),
        "${HOME}/.copilot/skills".to_string(),
    );
    paths.insert(
        "codexGitHooksRoot".to_string(),
        "${HOME}/.codex/git-hooks".to_string(),
    );
    paths.insert(
        "claudeRuntimeRoot".to_string(),
        "${HOME}/.claude".to_string(),
    );

    RuntimeLocationCatalog {
        schema_version: 1,
        settings: RuntimeLocationCatalogSettings {
            user_override_relative_path: ".codex/runtime-location-settings.json".to_string(),
        },
        paths,
    }
}

/// Resolve the default built-in override settings path.
///
/// # Errors
///
/// Returns an error when the user's home directory cannot be resolved.
pub fn resolve_built_in_runtime_location_settings_path() -> Result<PathBuf> {
    let home_path = resolve_user_home_path()?;
    Ok(join_path_segments(
        &home_path,
        &[".codex", "runtime-location-settings.json"],
    ))
}

/// Expand runtime path placeholders against the provided home path.
#[must_use]
pub fn expand_runtime_location_path(path_value: &str, home_path: &Path) -> PathBuf {
    let home_text = home_path.to_string_lossy();
    let expanded = path_value
        .replace("${HOME}", &home_text)
        .replace("$HOME", &home_text)
        .replace("%USERPROFILE%", &home_text)
        .replace("${USERPROFILE}", &home_text)
        .replace("${env:USERPROFILE}", &home_text)
        .replace("$env:USERPROFILE", &home_text);

    resolve_full_path(home_path, Path::new(&expanded))
}

/// Resolve the effective override settings path.
#[must_use]
pub fn resolve_runtime_location_settings_path(
    catalog: &RuntimeLocationCatalog,
    environment_override: Option<&str>,
    home_path: &Path,
) -> PathBuf {
    if let Some(environment_override) =
        environment_override.filter(|value| !value.trim().is_empty())
    {
        return resolve_full_path(home_path, Path::new(environment_override));
    }

    let relative_path = if catalog
        .settings
        .user_override_relative_path
        .trim()
        .is_empty()
    {
        ".codex/runtime-location-settings.json"
    } else {
        catalog.settings.user_override_relative_path.as_str()
    };

    expand_runtime_location_path(relative_path, home_path)
}

/// Resolve a single named runtime location using the same precedence as the
/// PowerShell helper: environment, user override, catalog, then fallback.
#[must_use]
pub fn resolve_configured_runtime_location(
    path_key: &str,
    environment_value: Option<&str>,
    fallback_path: &Path,
    catalog: &RuntimeLocationCatalog,
    overrides: &RuntimeLocationOverrides,
    home_path: &Path,
) -> PathBuf {
    if let Some(environment_value) = environment_value.filter(|value| !value.trim().is_empty()) {
        return expand_runtime_location_path(environment_value, home_path);
    }

    if let Some(override_value) = overrides.paths.get(path_key) {
        if !override_value.trim().is_empty() {
            return expand_runtime_location_path(override_value, home_path);
        }
    }

    if let Some(catalog_value) = catalog.paths.get(path_key) {
        if !catalog_value.trim().is_empty() {
            return expand_runtime_location_path(catalog_value, home_path);
        }
    }

    resolve_full_path(home_path, fallback_path)
}

/// Resolve the full set of named runtime locations.
#[must_use]
pub fn effective_runtime_locations(
    catalog: &RuntimeLocationCatalog,
    overrides: &RuntimeLocationOverrides,
    home_path: &Path,
) -> EffectiveRuntimeLocations {
    EffectiveRuntimeLocations {
        github_runtime_root: resolve_configured_runtime_location(
            "githubRuntimeRoot",
            None,
            Path::new(".github"),
            catalog,
            overrides,
            home_path,
        ),
        codex_runtime_root: resolve_configured_runtime_location(
            "codexRuntimeRoot",
            None,
            Path::new(".codex"),
            catalog,
            overrides,
            home_path,
        ),
        agents_skills_root: resolve_configured_runtime_location(
            "agentsSkillsRoot",
            None,
            Path::new(".agents/skills"),
            catalog,
            overrides,
            home_path,
        ),
        copilot_skills_root: resolve_configured_runtime_location(
            "copilotSkillsRoot",
            None,
            Path::new(".copilot/skills"),
            catalog,
            overrides,
            home_path,
        ),
        codex_git_hooks_root: resolve_configured_runtime_location(
            "codexGitHooksRoot",
            None,
            Path::new(".codex/git-hooks"),
            catalog,
            overrides,
            home_path,
        ),
        claude_runtime_root: resolve_configured_runtime_location(
            "claudeRuntimeRoot",
            None,
            Path::new(".claude"),
            catalog,
            overrides,
            home_path,
        ),
    }
}

/// Resolve the `.github` runtime root using built-in catalog defaults.
#[must_use]
pub fn resolve_github_runtime_path() -> PathBuf {
    let home_path = resolve_user_home_path().unwrap_or_else(|_| PathBuf::from("."));
    effective_runtime_locations(
        &built_in_runtime_location_catalog(),
        &RuntimeLocationOverrides::default(),
        &home_path,
    )
    .github_runtime_root
}

/// Resolve the `.codex` runtime root using built-in catalog defaults.
#[must_use]
pub fn resolve_codex_runtime_path() -> PathBuf {
    let home_path = resolve_user_home_path().unwrap_or_else(|_| PathBuf::from("."));
    effective_runtime_locations(
        &built_in_runtime_location_catalog(),
        &RuntimeLocationOverrides::default(),
        &home_path,
    )
    .codex_runtime_root
}

/// Resolve the `.agents/skills` runtime root using built-in catalog defaults.
#[must_use]
pub fn resolve_agents_skills_path() -> PathBuf {
    let home_path = resolve_user_home_path().unwrap_or_else(|_| PathBuf::from("."));
    effective_runtime_locations(
        &built_in_runtime_location_catalog(),
        &RuntimeLocationOverrides::default(),
        &home_path,
    )
    .agents_skills_root
}

/// Resolve the `.copilot/skills` runtime root using built-in catalog defaults.
#[must_use]
pub fn resolve_copilot_skills_path() -> PathBuf {
    let home_path = resolve_user_home_path().unwrap_or_else(|_| PathBuf::from("."));
    effective_runtime_locations(
        &built_in_runtime_location_catalog(),
        &RuntimeLocationOverrides::default(),
        &home_path,
    )
    .copilot_skills_root
}

/// Resolve the `.claude` runtime root using built-in catalog defaults.
#[must_use]
pub fn resolve_claude_runtime_path() -> PathBuf {
    let home_path = resolve_user_home_path().unwrap_or_else(|_| PathBuf::from("."));
    effective_runtime_locations(
        &built_in_runtime_location_catalog(),
        &RuntimeLocationOverrides::default(),
        &home_path,
    )
    .claude_runtime_root
}
