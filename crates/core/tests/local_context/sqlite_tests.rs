//! Tests for repository-local SQLite memory foundations.

use nettoolskit_core::local_context::{
    initialize_local_context_memory_store, resolve_local_context_memory_db_path,
    resolve_local_context_memory_paths, resolve_local_context_memory_root,
    LOCAL_CONTEXT_MEMORY_DB_FILE_NAME, LOCAL_CONTEXT_MEMORY_DIR_NAME,
    LOCAL_CONTEXT_MEMORY_SCHEMA_VERSION,
};
use rusqlite::Connection;
use tempfile::TempDir;

#[test]
fn test_resolve_local_context_memory_paths_uses_repo_temp_by_default() {
    let repo_root = TempDir::new().expect("temporary directory should be created");

    let memory_root = resolve_local_context_memory_root(repo_root.path(), None);
    let db_path = resolve_local_context_memory_db_path(repo_root.path(), None);
    let paths = resolve_local_context_memory_paths(repo_root.path(), None);

    assert_eq!(
        memory_root,
        repo_root
            .path()
            .join(".temp")
            .join(LOCAL_CONTEXT_MEMORY_DIR_NAME)
    );
    assert_eq!(
        db_path,
        repo_root
            .path()
            .join(".temp")
            .join(LOCAL_CONTEXT_MEMORY_DIR_NAME)
            .join(LOCAL_CONTEXT_MEMORY_DB_FILE_NAME)
    );
    assert_eq!(paths.memory_root, memory_root);
    assert_eq!(paths.db_path, db_path);
}

#[test]
fn test_resolve_local_context_memory_paths_honors_relative_override() {
    let repo_root = TempDir::new().expect("temporary directory should be created");

    let paths =
        resolve_local_context_memory_paths(repo_root.path(), Some("custom/memory".as_ref()));

    assert_eq!(
        paths.memory_root,
        repo_root.path().join("custom").join("memory")
    );
    assert_eq!(
        paths.db_path,
        repo_root
            .path()
            .join("custom")
            .join("memory")
            .join(LOCAL_CONTEXT_MEMORY_DB_FILE_NAME)
    );
}

#[test]
fn test_initialize_local_context_memory_store_creates_schema_and_metadata() {
    let repo_root = TempDir::new().expect("temporary directory should be created");

    let report =
        initialize_local_context_memory_store(repo_root.path(), None).expect("schema should init");

    assert!(report.memory_root.is_dir());
    assert!(report.db_path.is_file());
    assert_eq!(report.schema_version, LOCAL_CONTEXT_MEMORY_SCHEMA_VERSION);

    let connection =
        Connection::open(&report.db_path).expect("sqlite database should be reopenable");
    let schema_version: String = connection
        .query_row(
            "SELECT value FROM schema_metadata WHERE key = 'schemaVersion'",
            [],
            |row| row.get(0),
        )
        .expect("schema metadata should exist");
    assert_eq!(
        schema_version,
        LOCAL_CONTEXT_MEMORY_SCHEMA_VERSION.to_string()
    );

    let tables = [
        "documents",
        "files",
        "chunks",
        "chunk_fts",
        "events",
        "sessions",
        "artifacts",
    ];
    for table_name in tables {
        let exists: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE name = ?1",
                [table_name],
                |row| row.get(0),
            )
            .expect("sqlite master query should succeed");
        assert_eq!(exists, 1, "expected table `{table_name}` to exist");
    }
}

#[test]
fn test_initialize_local_context_memory_store_is_idempotent() {
    let repo_root = TempDir::new().expect("temporary directory should be created");

    let first =
        initialize_local_context_memory_store(repo_root.path(), None).expect("first init passes");
    let second =
        initialize_local_context_memory_store(repo_root.path(), None).expect("second init passes");

    assert_eq!(first.memory_root, second.memory_root);
    assert_eq!(first.db_path, second.db_path);
    assert_eq!(first.schema_version, second.schema_version);
}
