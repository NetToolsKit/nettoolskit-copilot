//! Tests for validation surface errors.

use nettoolskit_validation::require_validation_surface_contract;

#[test]
fn test_validation_surface_error_mentions_missing_surface_id() {
    let error = require_validation_surface_contract("missing-validation")
        .expect_err("unknown validation surface should fail");

    assert_eq!(
        error.to_string(),
        "unknown validation surface contract: missing-validation"
    );
}