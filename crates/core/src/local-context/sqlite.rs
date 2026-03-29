//! SQLite-backed local memory foundations for repository continuity.

use anyhow::{Context, Result};
use regex::Regex;
use rusqlite::{params, params_from_iter, types::Value, Connection};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use super::document::LocalContextIndexDocument;
use super::search::LocalContextSearchHit;
use crate::path_utils::repository::resolve_full_path;

/// Relative directory used by the repository-local SQLite memory store.
pub const LOCAL_CONTEXT_MEMORY_DIR_NAME: &str = "context-memory";
/// SQLite file name used by the repository-local SQLite memory store.
pub const LOCAL_CONTEXT_MEMORY_DB_FILE_NAME: &str = "context.db";
/// Schema version recorded in the repository-local SQLite memory store.
pub const LOCAL_CONTEXT_MEMORY_SCHEMA_VERSION: u32 = 1;

const LOCAL_CONTEXT_MEMORY_SCHEMA: &str = r#"
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS schema_metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS documents (
    document_id TEXT PRIMARY KEY,
    repo_root TEXT NOT NULL,
    catalog_path TEXT NOT NULL,
    generated_at TEXT NOT NULL,
    version INTEGER NOT NULL,
    chunk_count INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS files (
    path TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    hash TEXT NOT NULL,
    last_write_time_utc TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    title TEXT NOT NULL,
    FOREIGN KEY(document_id) REFERENCES documents(document_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS chunks (
    chunk_id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    path TEXT NOT NULL,
    kind TEXT NOT NULL,
    heading TEXT,
    text TEXT NOT NULL,
    search_text TEXT NOT NULL,
    FOREIGN KEY(document_id) REFERENCES documents(document_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS events (
    event_id TEXT PRIMARY KEY,
    repo_root TEXT NOT NULL,
    session_id TEXT,
    source_kind TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    created_at TEXT NOT NULL,
    expires_at TEXT
);

CREATE TABLE IF NOT EXISTS sessions (
    session_id TEXT PRIMARY KEY,
    repo_root TEXT NOT NULL,
    session_kind TEXT NOT NULL,
    summary TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS artifacts (
    artifact_id TEXT PRIMARY KEY,
    repo_root TEXT NOT NULL,
    artifact_kind TEXT NOT NULL,
    reference_path TEXT,
    title TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE VIRTUAL TABLE IF NOT EXISTS chunk_fts USING fts5(
    chunk_id UNINDEXED,
    path,
    heading,
    text,
    search_text,
    tokenize = 'unicode61'
);

CREATE INDEX IF NOT EXISTS idx_documents_repo_root
    ON documents (repo_root);
CREATE INDEX IF NOT EXISTS idx_files_document_id
    ON files (document_id);
CREATE INDEX IF NOT EXISTS idx_chunks_document_id
    ON chunks (document_id);
CREATE INDEX IF NOT EXISTS idx_chunks_path
    ON chunks (path);
CREATE INDEX IF NOT EXISTS idx_events_repo_root_created_at
    ON events (repo_root, created_at);
CREATE INDEX IF NOT EXISTS idx_sessions_repo_root_updated_at
    ON sessions (repo_root, updated_at);
CREATE INDEX IF NOT EXISTS idx_artifacts_repo_root_created_at
    ON artifacts (repo_root, created_at);
"#;

/// Resolved repository-local SQLite memory store paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalContextMemoryPaths {
    /// Resolved repository-local memory root directory.
    pub memory_root: PathBuf,
    /// Resolved SQLite database path.
    pub db_path: PathBuf,
}

/// Result payload returned after schema initialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalContextMemorySchemaReport {
    /// Resolved repository-local memory root directory.
    pub memory_root: PathBuf,
    /// Resolved SQLite database path.
    pub db_path: PathBuf,
    /// Schema version written to metadata.
    pub schema_version: u32,
}

/// Report returned after persisting the SQLite mirror of the local-context index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalContextSqliteWriteReport {
    /// Resolved repository-local memory root directory.
    pub memory_root: PathBuf,
    /// Resolved SQLite database path.
    pub db_path: PathBuf,
    /// Stable snapshot identifier written to the `documents` table.
    pub document_id: String,
    /// Number of files mirrored into SQLite.
    pub file_count: usize,
    /// Number of chunks mirrored into SQLite.
    pub chunk_count: usize,
}

/// Request payload for querying the repository-local SQLite memory store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalContextSqliteQueryRequest {
    /// Query text executed against the SQLite memory store.
    pub query_text: String,
    /// Maximum number of hits to return.
    pub top: usize,
    /// Repository-relative paths excluded from ranking.
    pub exclude_paths: Vec<String>,
    /// Optional repository-relative path prefix filter.
    pub path_prefix: Option<String>,
    /// Optional heading substring filter.
    pub heading_contains: Option<String>,
}

/// Resolve the repository-local SQLite memory root.
#[must_use]
pub fn resolve_local_context_memory_root(repo_root: &Path, output_root: Option<&Path>) -> PathBuf {
    match output_root {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => resolve_full_path(repo_root, path),
        None => repo_root.join(".temp").join(LOCAL_CONTEXT_MEMORY_DIR_NAME),
    }
}

/// Resolve the repository-local SQLite memory database path.
#[must_use]
pub fn resolve_local_context_memory_db_path(
    repo_root: &Path,
    output_root: Option<&Path>,
) -> PathBuf {
    resolve_local_context_memory_root(repo_root, output_root)
        .join(LOCAL_CONTEXT_MEMORY_DB_FILE_NAME)
}

/// Resolve both repository-local SQLite memory paths.
#[must_use]
pub fn resolve_local_context_memory_paths(
    repo_root: &Path,
    output_root: Option<&Path>,
) -> LocalContextMemoryPaths {
    let memory_root = resolve_local_context_memory_root(repo_root, output_root);
    let db_path = memory_root.join(LOCAL_CONTEXT_MEMORY_DB_FILE_NAME);

    LocalContextMemoryPaths {
        memory_root,
        db_path,
    }
}

/// Initialize the repository-local SQLite memory schema.
///
/// # Errors
///
/// Returns an error when the target directory cannot be created, the database
/// cannot be opened, or the schema bootstrap fails.
pub fn initialize_local_context_memory_store(
    repo_root: &Path,
    output_root: Option<&Path>,
) -> Result<LocalContextMemorySchemaReport> {
    let paths = resolve_local_context_memory_paths(repo_root, output_root);
    let _connection = open_local_context_memory_connection(&paths)?;

    Ok(LocalContextMemorySchemaReport {
        memory_root: paths.memory_root,
        db_path: paths.db_path,
        schema_version: LOCAL_CONTEXT_MEMORY_SCHEMA_VERSION,
    })
}

/// Persist the current local-context snapshot into the repository-local SQLite store.
///
/// # Errors
///
/// Returns an error when the SQLite store cannot be initialized or the snapshot
/// rows cannot be mirrored.
pub fn write_local_context_sqlite_index(
    repo_root: &Path,
    output_root: Option<&Path>,
    document: &LocalContextIndexDocument,
) -> Result<LocalContextSqliteWriteReport> {
    let paths = resolve_local_context_memory_paths(repo_root, output_root);
    let mut connection = open_local_context_memory_connection(&paths)?;
    let document_id = build_local_context_document_id(document);

    let transaction = connection
        .transaction()
        .context("failed to open local memory write transaction")?;

    transaction
        .execute("DELETE FROM chunk_fts", [])
        .context("failed to clear local memory chunk FTS rows")?;
    transaction
        .execute("DELETE FROM documents", [])
        .context("failed to clear local memory documents")?;

    transaction
        .execute(
            r#"
            INSERT INTO documents (
                document_id,
                repo_root,
                catalog_path,
                generated_at,
                version,
                chunk_count
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                document_id.as_str(),
                document.repo_root.as_str(),
                document.catalog_path.as_str(),
                document.generated_at.as_str(),
                i64::from(document.version),
                i64::try_from(document.chunk_count).context("chunk count exceeds i64 range")?,
            ],
        )
        .context("failed to insert local memory document snapshot")?;

    {
        let mut file_statement = transaction
            .prepare_cached(
                r#"
                INSERT INTO files (
                    path,
                    document_id,
                    hash,
                    last_write_time_utc,
                    size_bytes,
                    title
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
            )
            .context("failed to prepare local memory file statement")?;
        for file in &document.files {
            file_statement
                .execute(params![
                    file.path.as_str(),
                    document_id.as_str(),
                    file.hash.as_str(),
                    file.last_write_time_utc.as_str(),
                    i64::try_from(file.size_bytes).context("file size exceeds i64 range")?,
                    file.title.as_str(),
                ])
                .with_context(|| format!("failed to insert local memory file '{}'", file.path))?;
        }
    }

    {
        let mut chunk_statement = transaction
            .prepare_cached(
                r#"
                INSERT INTO chunks (
                    chunk_id,
                    document_id,
                    path,
                    kind,
                    heading,
                    text,
                    search_text
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                "#,
            )
            .context("failed to prepare local memory chunk statement")?;
        let mut fts_statement = transaction
            .prepare_cached(
                r#"
                INSERT INTO chunk_fts (chunk_id, path, heading, text, search_text)
                VALUES (?1, ?2, ?3, ?4, ?5)
                "#,
            )
            .context("failed to prepare local memory chunk FTS statement")?;

        for chunk in &document.chunks {
            let kind = match chunk.kind {
                super::document::LocalContextChunkKind::Markdown => "markdown",
                super::document::LocalContextChunkKind::Text => "text",
            };
            chunk_statement
                .execute(params![
                    chunk.id.as_str(),
                    document_id.as_str(),
                    chunk.path.as_str(),
                    kind,
                    chunk.heading.as_deref(),
                    chunk.text.as_str(),
                    chunk.search_text.as_str(),
                ])
                .with_context(|| format!("failed to insert local memory chunk '{}'", chunk.id))?;
            fts_statement
                .execute(params![
                    chunk.id.as_str(),
                    chunk.path.as_str(),
                    chunk.heading.as_deref(),
                    chunk.text.as_str(),
                    chunk.search_text.as_str(),
                ])
                .with_context(|| format!("failed to insert local memory FTS row '{}'", chunk.id))?;
        }
    }

    transaction
        .commit()
        .context("failed to commit local memory snapshot")?;

    Ok(LocalContextSqliteWriteReport {
        memory_root: paths.memory_root,
        db_path: paths.db_path,
        document_id,
        file_count: document.files.len(),
        chunk_count: document.chunks.len(),
    })
}

/// Query the repository-local SQLite memory snapshot when it exists.
///
/// # Errors
///
/// Returns an error when the SQLite database cannot be opened or queried.
pub fn search_local_context_sqlite_index(
    repo_root: &Path,
    output_root: Option<&Path>,
    request: &LocalContextSqliteQueryRequest,
) -> Result<Option<Vec<LocalContextSearchHit>>> {
    let paths = resolve_local_context_memory_paths(repo_root, output_root);
    if !paths.db_path.is_file() {
        return Ok(None);
    }

    let connection = Connection::open(&paths.db_path).with_context(|| {
        format!(
            "failed to open local memory database '{}'",
            paths.db_path.display()
        )
    })?;
    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .context("failed to configure local memory SQLite session")?;

    let match_expression = build_local_context_match_expression(&request.query_text);
    if match_expression.is_empty() {
        return Ok(Some(Vec::new()));
    }

    let effective_top = request.top.max(1);
    let mut sql = String::from(
        r#"
        SELECT
            chunks.chunk_id,
            chunks.path,
            chunks.heading,
            chunks.text
        FROM chunk_fts
        INNER JOIN chunks ON chunks.chunk_id = chunk_fts.chunk_id
        WHERE chunk_fts MATCH ?
        "#,
    );

    let mut parameters = vec![Value::from(match_expression)];
    if let Some(path_prefix) = request
        .path_prefix
        .as_ref()
        .filter(|value| !value.trim().is_empty())
    {
        sql.push_str(" AND chunks.path LIKE ?");
        parameters.push(Value::from(format!(
            "{}%",
            normalize_local_context_path(path_prefix)
        )));
    }
    if let Some(heading_contains) = request
        .heading_contains
        .as_ref()
        .filter(|value| !value.trim().is_empty())
    {
        sql.push_str(" AND COALESCE(chunks.heading, '') LIKE ?");
        parameters.push(Value::from(format!("%{}%", heading_contains.trim())));
    }
    for excluded_path in request
        .exclude_paths
        .iter()
        .map(|path| normalize_local_context_path(path))
    {
        sql.push_str(" AND chunks.path <> ?");
        parameters.push(Value::from(excluded_path));
    }
    sql.push_str(" ORDER BY bm25(chunk_fts), chunks.path, chunks.chunk_id LIMIT ?");
    parameters.push(Value::from(
        i64::try_from(effective_top).context("local memory top exceeds i64 range")?,
    ));

    let mut statement = connection
        .prepare(&sql)
        .context("failed to prepare local memory SQLite query")?;
    let rows = statement
        .query_map(params_from_iter(parameters), |row| {
            Ok(LocalContextSearchHit {
                id: row.get(0)?,
                path: row.get(1)?,
                heading: row.get(2)?,
                score: 0,
                excerpt: row.get(3)?,
            })
        })
        .context("failed to execute local memory SQLite query")?;

    let mut hits = rows
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("failed to map local memory SQLite hits")?;
    let total_hits = i32::try_from(hits.len()).unwrap_or(i32::MAX);
    for (index, hit) in hits.iter_mut().enumerate() {
        hit.score = total_hits.saturating_sub(i32::try_from(index).unwrap_or(i32::MAX));
    }

    Ok(Some(hits))
}

fn open_local_context_memory_connection(paths: &LocalContextMemoryPaths) -> Result<Connection> {
    if let Some(parent) = paths.db_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create local memory directory '{}'",
                parent.display()
            )
        })?;
    }

    let connection = Connection::open(&paths.db_path).with_context(|| {
        format!(
            "failed to open local memory database '{}'",
            paths.db_path.display()
        )
    })?;
    connection
        .execute_batch(LOCAL_CONTEXT_MEMORY_SCHEMA)
        .context("failed to initialize local memory schema")?;
    connection
        .execute(
            r#"
            INSERT INTO schema_metadata (key, value)
            VALUES ('schemaVersion', ?1)
            ON CONFLICT(key) DO UPDATE SET value = excluded.value
            "#,
            [LOCAL_CONTEXT_MEMORY_SCHEMA_VERSION.to_string()],
        )
        .context("failed to persist local memory schema version")?;
    Ok(connection)
}

fn build_local_context_document_id(document: &LocalContextIndexDocument) -> String {
    format!(
        "local-context:{}:{}",
        document.version, document.generated_at
    )
}

fn build_local_context_match_expression(query_text: &str) -> String {
    let terms = local_context_query_terms(query_text)
        .into_iter()
        .map(|term| format!("\"{}\"", term.replace('"', "\"\"")))
        .collect::<Vec<_>>();
    terms.join(" OR ")
}

fn local_context_query_terms(text: &str) -> Vec<String> {
    static TOKEN_REGEX: OnceLock<Regex> = OnceLock::new();

    TOKEN_REGEX
        .get_or_init(|| {
            Regex::new(r"[a-z0-9][a-z0-9._/-]{1,63}")
                .expect("local context SQLite token regex should be valid")
        })
        .find_iter(&text.to_ascii_lowercase())
        .map(|capture| capture.as_str().to_string())
        .collect()
}

fn normalize_local_context_path(path: &str) -> String {
    path.replace('\\', "/")
}
