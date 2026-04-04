//! Tests for error surfaces reachable through the command aggregator.

use nettoolskit_commands::{
    nettoolskit_manifest::ManifestError, nettoolskit_runtime, nettoolskit_validation,
};
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
    assert!(solution_error
        .to_string()
        .contains("solution root not found"));
}

#[test]
fn test_runtime_and_validation_unknown_surface_errors_render_requested_id() {
    let runtime_error = nettoolskit_runtime::require_runtime_surface_contract("missing-runtime")
        .expect_err("missing runtime surface should fail");
    let validation_error =
        nettoolskit_validation::require_validation_surface_contract("missing-validation")
            .expect_err("missing validation surface should fail");

    assert!(runtime_error.to_string().contains("missing-runtime"));
    assert!(validation_error.to_string().contains("missing-validation"));
}