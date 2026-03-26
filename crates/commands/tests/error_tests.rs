//! Tests for error surfaces reachable through the command aggregator.

use nettoolskit_commands::nettoolskit_manifest::ManifestError;
use std::io;
use std::path::PathBuf;

#[test]
fn test_manifest_error_other_variant_is_reachable_via_commands_surface() {
    let error = ManifestError::Other("detail".to_string());

    assert_eq!(error.to_string(), "detail");
}

#[test]
fn test_manifest_error_from_string_preserves_message() {
    let error = ManifestError::from("commands aggregator");

    assert_eq!(error.to_string(), "commands aggregator");
}

#[test]
fn test_manifest_error_io_backed_variants_render_expected_context() {
    let read_error = ManifestError::ReadError {
        path: "ntk-manifest.yml".to_string(),
        source: io::Error::other("denied"),
    };
    let solution_error = ManifestError::SolutionNotFound {
        path: PathBuf::from("repo"),
    };

    assert!(read_error.to_string().contains("failed to read manifest"));
    assert!(read_error.to_string().contains("ntk-manifest.yml"));
    assert!(solution_error.to_string().contains("solution root not found"));
}