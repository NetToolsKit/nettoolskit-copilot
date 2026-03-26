//! Tests for public command re-exports.

use nettoolskit_commands::{nettoolskit_help, nettoolskit_manifest};
use std::path::PathBuf;

#[test]
fn test_help_manifest_info_is_constructible_via_commands_surface() {
    let info = nettoolskit_help::ManifestInfo {
        path: PathBuf::from("sample/ntk-manifest.yml"),
        project_name: "Sample".to_string(),
        language: "rust".to_string(),
        context_count: 2,
    };

    assert_eq!(info.project_name, "Sample");
    assert_eq!(info.language, "rust");
    assert_eq!(info.context_count, 2);
    assert!(info.path.ends_with("ntk-manifest.yml"));
}

#[test]
fn test_manifest_action_lookup_works_via_commands_surface() {
    let action = nettoolskit_manifest::get_action("check");

    assert_eq!(action, Some(nettoolskit_manifest::ManifestAction::Check));
}

#[test]
fn test_manifest_action_full_command_keeps_parent_prefix() {
    let action = nettoolskit_manifest::ManifestAction::Render;

    assert_eq!(action.full_command(), "/manifest render");
    assert_eq!(
        action.description(),
        "Preview generated files without creating them"
    );
}