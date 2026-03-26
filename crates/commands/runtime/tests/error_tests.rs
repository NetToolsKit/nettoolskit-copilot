//! Tests for runtime surface errors.

use nettoolskit_runtime::require_runtime_surface_contract;

#[test]
fn test_runtime_surface_error_mentions_missing_surface_id() {
    let error = require_runtime_surface_contract("missing-runtime")
        .expect_err("unknown runtime surface should fail");

    assert_eq!(
        error.to_string(),
        "unknown runtime surface contract: missing-runtime"
    );
}