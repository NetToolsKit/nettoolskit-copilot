//! Tests for runtime local context commands.

use nettoolskit_runtime::{
    query_local_context_index, query_local_memory, update_local_context_index, update_local_memory,
    LocalContextCommandError, LocalContextQueryBackend, QueryLocalContextIndexRequest,
    QueryLocalMemoryRequest, UpdateLocalContextIndexRequest, UpdateLocalMemoryRequest,
};
use std::fs;
use tempfile::TempDir;

fn write_catalog(repo_root: &std::path::Path) {
    let catalog_dir = repo_root.join(".github/governance");
    fs::create_dir_all(&catalog_dir).expect("catalog directory should be created");
    fs::write(
        catalog_dir.join("local-context-index.catalog.json"),
        r#"{"version":1,"indexRoot":".temp/context-index","maxFileSizeKb":16,"chunking":{"maxChars":120,"maxLines":4},"queryDefaults":{"top":3},"includeGlobs":["planning/**/*.md","scripts/**/*.ps1"],"excludeGlobs":[".temp/**"]}"#,
    )
    .expect("catalog should be written");
}

#[test]
fn test_update_local_context_index_builds_index_and_query_returns_hits() {
    let repo = TempDir::new().expect("temporary repository should be created");
    write_catalog(repo.path());
    fs::create_dir_all(repo.path().join("planning/active"))
        .expect("planning directory should be created");
    fs::create_dir_all(repo.path().join("scripts/runtime"))
        .expect("runtime directory should be created");
    fs::write(
        repo.path().join("planning/active/plan.md"),
        "# Wave 1\nRust migration for local context index",
    )
    .expect("plan file should be written");
    fs::write(
        repo.path().join("scripts/runtime/demo.ps1"),
        "Write-Output 'rust migration'",
    )
    .expect("script file should be written");

    let update_result = update_local_context_index(&UpdateLocalContextIndexRequest {
        repo_root: Some(repo.path().to_path_buf()),
        catalog_path: None,
        output_root: None,
        force_full_rebuild: false,
    })
    .expect("index update should succeed");

    assert_eq!(update_result.indexed_file_count, 2);
    assert_eq!(update_result.rebuilt_file_count, 2);
    assert_eq!(update_result.reused_file_count, 0);
    assert!(update_result.index_path.is_file());
    assert!(update_result.memory_db_path.is_file());

    let query_result = query_local_context_index(&QueryLocalContextIndexRequest {
        repo_root: Some(repo.path().to_path_buf()),
        query_text: "rust migration".to_string(),
        catalog_path: None,
        output_root: None,
        top: None,
        exclude_paths: Vec::new(),
        path_prefix: Some("planning/".to_string()),
        heading_contains: Some("wave".to_string()),
        use_json_index: false,
    })
    .expect("query should succeed");

    assert_eq!(
        query_result.backend,
        LocalContextQueryBackend::SqliteDefault
    );
    assert_eq!(query_result.top, 3);
    assert_eq!(query_result.result_count, 1);
    assert_eq!(query_result.hits[0].path, "planning/active/plan.md");
    assert!(query_result.memory_db_path.is_file());
}

#[test]
fn test_update_local_memory_builds_sqlite_store_and_query_returns_hits() {
    let repo = TempDir::new().expect("temporary repository should be created");
    write_catalog(repo.path());
    fs::create_dir_all(repo.path().join("planning/active"))
        .expect("planning directory should be created");
    fs::create_dir_all(repo.path().join("scripts/runtime"))
        .expect("runtime directory should be created");
    fs::write(
        repo.path().join("planning/active/plan.md"),
        "# Wave 1\nSQLite local memory powers continuity recall",
    )
    .expect("plan file should be written");
    fs::write(
        repo.path().join("scripts/runtime/demo.ps1"),
        "Write-Output 'continuity recall'",
    )
    .expect("script file should be written");

    let update_result = update_local_memory(&UpdateLocalMemoryRequest {
        repo_root: Some(repo.path().to_path_buf()),
        catalog_path: None,
        output_root: None,
        force_full_rebuild: false,
    })
    .expect("local memory update should succeed");

    assert!(update_result.index_path.is_file());
    assert!(update_result.memory_db_path.is_file());

    let query_result = query_local_memory(&QueryLocalMemoryRequest {
        repo_root: Some(repo.path().to_path_buf()),
        query_text: "sqlite continuity".to_string(),
        catalog_path: None,
        output_root: None,
        top: Some(2),
        exclude_paths: Vec::new(),
        path_prefix: Some("planning/".to_string()),
        heading_contains: Some("wave".to_string()),
    })
    .expect("local memory query should succeed");

    assert_eq!(query_result.top, 2);
    assert_eq!(query_result.result_count, 1);
    assert_eq!(query_result.hits[0].path, "planning/active/plan.md");
    assert!(query_result.memory_db_path.is_file());
}

#[test]
fn test_query_local_context_index_requires_existing_index() {
    let repo = TempDir::new().expect("temporary repository should be created");
    write_catalog(repo.path());

    let error = query_local_context_index(&QueryLocalContextIndexRequest {
        repo_root: Some(repo.path().to_path_buf()),
        query_text: "missing".to_string(),
        catalog_path: None,
        output_root: None,
        top: Some(1),
        exclude_paths: Vec::new(),
        path_prefix: None,
        heading_contains: None,
        use_json_index: true,
    })
    .expect_err("query should fail when index does not exist");

    match error {
        LocalContextCommandError::IndexNotFound { index_path } => {
            assert!(
                index_path.ends_with(".temp\\context-index\\index.json")
                    || index_path.ends_with(".temp/context-index/index.json")
            );
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn test_query_local_memory_requires_existing_sqlite_store() {
    let repo = TempDir::new().expect("temporary repository should be created");
    write_catalog(repo.path());

    let error = query_local_memory(&QueryLocalMemoryRequest {
        repo_root: Some(repo.path().to_path_buf()),
        query_text: "missing".to_string(),
        catalog_path: None,
        output_root: None,
        top: Some(1),
        exclude_paths: Vec::new(),
        path_prefix: None,
        heading_contains: None,
    })
    .expect_err("query should fail when sqlite store does not exist");

    match error {
        LocalContextCommandError::MemoryNotFound { db_path } => {
            assert!(
                db_path.ends_with(".temp\\context-memory\\context.db")
                    || db_path.ends_with(".temp/context-memory/context.db")
            );
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn test_query_local_context_index_rejects_empty_query() {
    let error = query_local_context_index(&QueryLocalContextIndexRequest {
        repo_root: None,
        query_text: "   ".to_string(),
        catalog_path: None,
        output_root: None,
        top: None,
        exclude_paths: Vec::new(),
        path_prefix: None,
        heading_contains: None,
        use_json_index: false,
    })
    .expect_err("empty query should fail");

    assert!(matches!(error, LocalContextCommandError::EmptyQuery));
}

#[test]
fn test_query_local_context_index_can_force_json_compatibility_path() {
    let repo = TempDir::new().expect("temporary repository should be created");
    write_catalog(repo.path());
    fs::create_dir_all(repo.path().join("planning/active"))
        .expect("planning directory should be created");
    fs::write(
        repo.path().join("planning/active/plan.md"),
        "# Wave 1\nRust compatibility fallback remains available",
    )
    .expect("plan file should be written");

    let update_result = update_local_context_index(&UpdateLocalContextIndexRequest {
        repo_root: Some(repo.path().to_path_buf()),
        catalog_path: None,
        output_root: None,
        force_full_rebuild: false,
    })
    .expect("index update should succeed");
    fs::remove_file(&update_result.memory_db_path).expect("sqlite memory db should be removable");

    let query_result = query_local_context_index(&QueryLocalContextIndexRequest {
        repo_root: Some(repo.path().to_path_buf()),
        query_text: "compatibility fallback".to_string(),
        catalog_path: None,
        output_root: None,
        top: Some(1),
        exclude_paths: Vec::new(),
        path_prefix: None,
        heading_contains: None,
        use_json_index: true,
    })
    .expect("compatibility JSON query should succeed");

    assert_eq!(
        query_result.backend,
        LocalContextQueryBackend::JsonCompatibility
    );
    assert_eq!(query_result.result_count, 1);
    assert_eq!(query_result.hits[0].path, "planning/active/plan.md");
}