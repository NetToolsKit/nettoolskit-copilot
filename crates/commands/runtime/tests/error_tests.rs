//! Tests for runtime surface errors.

use anyhow::anyhow;
use nettoolskit_runtime::{require_runtime_surface_contract, LocalContextCommandError};
use std::error::Error;

#[test]
fn test_runtime_surface_error_mentions_missing_surface_id() {
    let error = require_runtime_surface_contract("missing-runtime")
        .expect_err("unknown runtime surface should fail");

    assert_eq!(
        error.to_string(),
        "unknown runtime surface contract: missing-runtime"
    );
}

#[test]
fn test_local_context_command_error_mentions_missing_index_path() {
    let error = LocalContextCommandError::IndexNotFound {
        index_path: "C:/repo/.temp/context-index/index.json".to_string(),
    };

    assert_eq!(
        error.to_string(),
        "local context index not found: C:/repo/.temp/context-index/index.json"
    );
}

#[test]
fn test_local_context_command_error_preserves_source_message() {
    let error = LocalContextCommandError::BuildIndex {
        source: anyhow!("boom"),
    };

    assert_eq!(error.to_string(), "failed to build local context index");
    assert_eq!(
        error.source().expect("source should be preserved").to_string(),
        "boom"
    );
}