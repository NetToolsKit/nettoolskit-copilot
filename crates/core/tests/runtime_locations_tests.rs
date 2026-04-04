//! Tests for runtime location catalog helpers.

use nettoolskit_core::runtime_locations::{
    built_in_runtime_location_catalog, effective_runtime_locations, expand_runtime_location_path,
    resolve_configured_runtime_location, resolve_runtime_location_settings_path,
    RuntimeLocationOverrides,
};
use std::collections::BTreeMap;
use std::path::Path;

#[test]
fn test_built_in_runtime_location_catalog_exposes_expected_override_path() {
    let catalog = built_in_runtime_location_catalog();

    assert_eq!(catalog.schema_version, 1);
    assert_eq!(
        catalog.settings.user_override_relative_path,
        ".codex/runtime-location-settings.json"
    );
    assert_eq!(
        catalog.paths.get("githubRuntimeRoot"),
        Some(&"${HOME}/.github".to_string())
    );
}

#[test]
fn test_expand_runtime_location_path_replaces_home_tokens() {
    let home_path = Path::new("C:/Users/tester");

    let expanded = expand_runtime_location_path("${HOME}/.codex", home_path);

    assert_eq!(expanded, Path::new("C:/Users/tester/.codex"));
}

#[test]
fn test_resolve_runtime_location_settings_path_prefers_environment_override() {
    let catalog = built_in_runtime_location_catalog();
    let home_path = Path::new("C:/Users/tester");

    let settings_path = resolve_runtime_location_settings_path(
        &catalog,
        Some("D:/custom/runtime-location-settings.json"),
        home_path,
    );

    assert_eq!(
        settings_path,
        Path::new("D:/custom/runtime-location-settings.json")
    );
}

#[test]
fn test_resolve_configured_runtime_location_honors_precedence() {
    let catalog = built_in_runtime_location_catalog();
    let home_path = Path::new("C:/Users/tester");
    let mut overrides = RuntimeLocationOverrides::default();
    overrides.paths.insert(
        "codexRuntimeRoot".to_string(),
        "D:/override/.codex".to_string(),
    );

    let resolved = resolve_configured_runtime_location(
        "codexRuntimeRoot",
        Some("E:/env/.codex"),
        Path::new(".codex"),
        &catalog,
        &overrides,
        home_path,
    );

    assert_eq!(resolved, Path::new("E:/env/.codex"));
}

#[test]
fn test_effective_runtime_locations_resolve_all_named_paths() {
    let mut paths = BTreeMap::new();
    paths.insert(
        "githubRuntimeRoot".to_string(),
        "${HOME}/runtime/github".to_string(),
    );
    paths.insert(
        "codexRuntimeRoot".to_string(),
        "${HOME}/runtime/codex".to_string(),
    );
    paths.insert(
        "agentsSkillsRoot".to_string(),
        "${HOME}/runtime/agents/skills".to_string(),
    );
    paths.insert(
        "copilotSkillsRoot".to_string(),
        "${HOME}/runtime/copilot/skills".to_string(),
    );
    paths.insert(
        "codexGitHooksRoot".to_string(),
        "${HOME}/runtime/codex/git-hooks".to_string(),
    );
    paths.insert(
        "claudeRuntimeRoot".to_string(),
        "${HOME}/runtime/claude".to_string(),
    );
    let mut catalog = built_in_runtime_location_catalog();
    catalog.paths = paths;

    let locations = effective_runtime_locations(
        &catalog,
        &RuntimeLocationOverrides::default(),
        Path::new("C:/Users/tester"),
    );

    assert_eq!(
        locations.github_runtime_root,
        Path::new("C:/Users/tester/runtime/github")
    );
    assert_eq!(
        locations.codex_runtime_root,
        Path::new("C:/Users/tester/runtime/codex")
    );
    assert_eq!(
        locations.agents_skills_root,
        Path::new("C:/Users/tester/runtime/agents/skills")
    );
}