//! Tests for repository-local SQLite memory foundations.

use nettoolskit_core::local_context::{
    build_local_context_index, initialize_local_context_memory_store,
    prune_local_context_memory_events, record_local_context_memory_event,
    resolve_local_context_memory_db_path, resolve_local_context_memory_paths,
    resolve_local_context_memory_root, search_local_context_sqlite_index,
    upsert_local_context_memory_session, LocalContextIndexCatalog, LocalContextIndexCatalogInfo,
    LocalContextIndexChunking, LocalContextIndexQueryDefaults, LocalContextMemoryEventRecord,
    LocalContextMemorySessionRecord, LocalContextSqliteQueryRequest,
    LOCAL_CONTEXT_MEMORY_DB_FILE_NAME, LOCAL_CONTEXT_MEMORY_DIR_NAME,
    LOCAL_CONTEXT_MEMORY_SCHEMA_VERSION,
};
use rusqlite::Connection;
use std::fs;
use tempfile::TempDir;

fn sample_catalog() -> LocalContextIndexCatalog {
    LocalContextIndexCatalog {
        version: 1,
        index_root: ".temp/context-index".to_string(),
        max_file_size_kb: 32,
        chunking: LocalContextIndexChunking {
            max_chars: 160,
            max_lines: 6,
        },
        query_defaults: LocalContextIndexQueryDefaults { top: 5 },
        include_globs: vec![
            "planning/**/*.md".to_string(),
            "scripts/**/*.ps1".to_string(),
        ],
        exclude_globs: vec![".temp/**".to_string()],
    }
}

fn write_governance_file(repo_root: &std::path::Path, file_name: &str, contents: &str) {
    let canonical_dir = repo_root.join("definitions/providers/github/governance");
    fs::create_dir_all(&canonical_dir).expect("canonical catalog directory should be created");
    fs::write(canonical_dir.join(file_name), contents).expect("canonical file should be written");

    let legacy_dir = repo_root.join(".github/governance");
    fs::create_dir_all(&legacy_dir).expect("legacy catalog directory should be created");
    fs::write(legacy_dir.join(file_name), contents).expect("legacy file should be written");
}

fn write_catalog(repo_root: &std::path::Path) -> LocalContextIndexCatalogInfo {
    let catalog_path =
        repo_root.join("definitions/providers/github/governance/local-context-index.catalog.json");
    write_governance_file(
        repo_root,
        "local-context-index.catalog.json",
        r#"{"version":1,"indexRoot":".temp/context-index","maxFileSizeKb":32,"chunking":{"maxChars":160,"maxLines":6},"queryDefaults":{"top":5},"includeGlobs":["planning/**/*.md","scripts/**/*.ps1"],"excludeGlobs":[".temp/**"]}"#,
    );

    LocalContextIndexCatalogInfo {
        path: catalog_path,
        catalog: sample_catalog(),
    }
}

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

#[test]
fn test_search_local_context_sqlite_index_returns_ranked_hits() {
    let repo_root = TempDir::new().expect("temporary directory should be created");
    let catalog_info = write_catalog(repo_root.path());
    fs::create_dir_all(repo_root.path().join("planning/active"))
        .expect("planning directory should be created");
    fs::create_dir_all(repo_root.path().join("scripts/runtime"))
        .expect("runtime directory should be created");
    fs::write(
        repo_root.path().join("planning/active/plan.md"),
        "# Rust migration\n\nSQLite local memory replaces JSON-only recall.",
    )
    .expect("plan file should be written");
    fs::write(
        repo_root.path().join("scripts/runtime/demo.ps1"),
        "Write-Output 'local memory sqlite recall'",
    )
    .expect("script file should be written");

    build_local_context_index(repo_root.path(), &catalog_info, None, false)
        .expect("local context build should succeed");

    let hits = search_local_context_sqlite_index(
        repo_root.path(),
        None,
        &LocalContextSqliteQueryRequest {
            query_text: "rust migration".to_string(),
            top: 5,
            exclude_paths: Vec::new(),
            path_prefix: None,
            heading_contains: None,
        },
    )
    .expect("sqlite query should succeed")
    .expect("sqlite memory store should exist");

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].path, "planning/active/plan.md");
    assert_eq!(hits[0].heading.as_deref(), Some("Rust migration"));
}

#[test]
fn test_search_local_context_sqlite_index_honors_filters() {
    let repo_root = TempDir::new().expect("temporary directory should be created");
    let catalog_info = write_catalog(repo_root.path());
    fs::create_dir_all(repo_root.path().join("planning/active"))
        .expect("planning directory should be created");
    fs::write(
        repo_root.path().join("planning/active/plan.md"),
        "# Wave one\n\nSQLite local memory query command.",
    )
    .expect("plan file should be written");
    fs::write(
        repo_root.path().join("planning/active/notes.md"),
        "# Backlog\n\nSQLite local memory future events.",
    )
    .expect("notes file should be written");

    build_local_context_index(repo_root.path(), &catalog_info, None, false)
        .expect("local context build should succeed");

    let hits = search_local_context_sqlite_index(
        repo_root.path(),
        None,
        &LocalContextSqliteQueryRequest {
            query_text: "sqlite local memory".to_string(),
            top: 5,
            exclude_paths: vec!["planning/active/notes.md".to_string()],
            path_prefix: Some("planning/active/".to_string()),
            heading_contains: Some("wave".to_string()),
        },
    )
    .expect("sqlite query should succeed")
    .expect("sqlite memory store should exist");

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].path, "planning/active/plan.md");
    assert_eq!(hits[0].heading.as_deref(), Some("Wave one"));
}

#[test]
fn test_record_local_context_memory_event_persists_and_prunes_expired_rows() {
    let repo_root = TempDir::new().expect("temporary directory should be created");
    initialize_local_context_memory_store(repo_root.path(), None).expect("schema should init");

    record_local_context_memory_event(
        repo_root.path(),
        None,
        &LocalContextMemoryEventRecord {
            event_id: "event-old".to_string(),
            session_id: None,
            source_kind: "tests".to_string(),
            payload_json: r#"{"summary":"old"}"#.to_string(),
            created_at_unix_ms: 100,
            expires_at_unix_ms: Some(150),
        },
    )
    .expect("old event should persist");
    record_local_context_memory_event(
        repo_root.path(),
        None,
        &LocalContextMemoryEventRecord {
            event_id: "event-new".to_string(),
            session_id: Some("session-1".to_string()),
            source_kind: "tests".to_string(),
            payload_json: r#"{"summary":"new"}"#.to_string(),
            created_at_unix_ms: 140,
            expires_at_unix_ms: Some(400),
        },
    )
    .expect("new event should persist");

    let pruned = prune_local_context_memory_events(repo_root.path(), None, 300)
        .expect("expired events should prune");
    assert!(pruned <= 1);

    let db_path = resolve_local_context_memory_db_path(repo_root.path(), None);
    let connection = Connection::open(db_path).expect("sqlite database should be reopenable");
    let event_count = connection
        .query_row("SELECT COUNT(*) FROM events", [], |row| {
            row.get::<_, i64>(0)
        })
        .expect("event count should load");
    let session_id: String = connection
        .query_row(
            "SELECT session_id FROM events WHERE event_id = 'event-new'",
            [],
            |row| row.get(0),
        )
        .expect("event row should load");

    assert_eq!(event_count, 1);
    assert_eq!(session_id, "session-1");
}

#[test]
fn test_upsert_local_context_memory_session_replaces_summary() {
    let repo_root = TempDir::new().expect("temporary directory should be created");
    initialize_local_context_memory_store(repo_root.path(), None).expect("schema should init");

    upsert_local_context_memory_session(
        repo_root.path(),
        None,
        &LocalContextMemorySessionRecord {
            session_id: "session-1".to_string(),
            session_kind: "ai-session".to_string(),
            summary: Some("first summary".to_string()),
            created_at_unix_ms: 10,
            updated_at_unix_ms: 20,
        },
    )
    .expect("session should persist");
    upsert_local_context_memory_session(
        repo_root.path(),
        None,
        &LocalContextMemorySessionRecord {
            session_id: "session-1".to_string(),
            session_kind: "ai-session".to_string(),
            summary: Some("updated summary".to_string()),
            created_at_unix_ms: 10,
            updated_at_unix_ms: 30,
        },
    )
    .expect("session should upsert");

    let db_path = resolve_local_context_memory_db_path(repo_root.path(), None);
    let connection = Connection::open(db_path).expect("sqlite database should be reopenable");
    let summary: String = connection
        .query_row(
            "SELECT summary FROM sessions WHERE session_id = 'session-1'",
            [],
            |row| row.get(0),
        )
        .expect("session summary should load");
    let updated_at: String = connection
        .query_row(
            "SELECT updated_at FROM sessions WHERE session_id = 'session-1'",
            [],
            |row| row.get(0),
        )
        .expect("session updated timestamp should load");

    assert_eq!(summary, "updated summary");
    assert_eq!(updated_at, "30");
}