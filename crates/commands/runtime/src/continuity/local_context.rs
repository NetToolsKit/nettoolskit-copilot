//! Runtime-backed local context index commands.

use nettoolskit_core::local_context::{
    build_local_context_index, read_local_context_index_catalog, read_local_context_index_document,
    resolve_local_context_index_root, resolve_local_context_memory_db_path,
    search_local_context_index_document, search_local_context_sqlite_index, LocalContextSearchHit,
    LocalContextSqliteQueryRequest,
};
use nettoolskit_core::path_utils::repository::resolve_workspace_root;
use std::env;
use std::path::PathBuf;

use crate::error::LocalContextCommandError;

/// Request payload for `update-local-context-index`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UpdateLocalContextIndexRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit catalog path.
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit output root.
    pub output_root: Option<PathBuf>,
    /// Rebuild every file even when a prior persisted index exists.
    pub force_full_rebuild: bool,
}

/// Result payload for `update-local-context-index`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateLocalContextIndexResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved catalog path.
    pub catalog_path: PathBuf,
    /// Resolved index root.
    pub index_root: PathBuf,
    /// Persisted document path.
    pub index_path: PathBuf,
    /// Resolved SQLite memory root.
    pub memory_root: PathBuf,
    /// Resolved SQLite memory database path.
    pub memory_db_path: PathBuf,
    /// Total indexed files.
    pub indexed_file_count: usize,
    /// Files rebuilt during the current update.
    pub rebuilt_file_count: usize,
    /// Files reused from the previous persisted document.
    pub reused_file_count: usize,
    /// Total chunk count in the resulting document.
    pub chunk_count: usize,
}

/// Request payload for `query-local-context-index`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryLocalContextIndexRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Query text executed against the local context index.
    pub query_text: String,
    /// Optional explicit catalog path.
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit output root.
    pub output_root: Option<PathBuf>,
    /// Optional explicit top limit.
    pub top: Option<usize>,
    /// Repository-relative paths excluded from ranking.
    pub exclude_paths: Vec<String>,
    /// Optional repository-relative path prefix filter.
    pub path_prefix: Option<String>,
    /// Optional heading substring filter.
    pub heading_contains: Option<String>,
    /// Force the legacy JSON compatibility path instead of the default SQLite recall path.
    pub use_json_index: bool,
}

/// Retrieval backend used for `query-local-context-index`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalContextQueryBackend {
    /// Default SQLite-backed repo-local memory recall.
    SqliteDefault,
    /// Explicit JSON compatibility fallback.
    JsonCompatibility,
}

impl LocalContextQueryBackend {
    /// Stable user-facing label for CLI/reporting output.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SqliteDefault => "sqlite-default",
            Self::JsonCompatibility => "json-compatibility",
        }
    }
}

/// Result payload for `query-local-context-index`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryLocalContextIndexResult {
    /// Retrieval backend that answered the query.
    pub backend: LocalContextQueryBackend,
    /// Query text executed against the index.
    pub query: String,
    /// Effective top limit used by the query.
    pub top: usize,
    /// Persisted document path.
    pub index_path: PathBuf,
    /// Resolved SQLite memory database path.
    pub memory_db_path: PathBuf,
    /// Number of ranked hits returned.
    pub result_count: usize,
    /// Ranked search hits.
    pub hits: Vec<LocalContextSearchHit>,
}

/// Request payload for `update-local-memory`.
pub type UpdateLocalMemoryRequest = UpdateLocalContextIndexRequest;

/// Result payload for `update-local-memory`.
pub type UpdateLocalMemoryResult = UpdateLocalContextIndexResult;

/// Request payload for `query-local-memory`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryLocalMemoryRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Query text executed against the local memory store.
    pub query_text: String,
    /// Optional explicit catalog path.
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit output root.
    pub output_root: Option<PathBuf>,
    /// Optional explicit top limit.
    pub top: Option<usize>,
    /// Repository-relative paths excluded from ranking.
    pub exclude_paths: Vec<String>,
    /// Optional repository-relative path prefix filter.
    pub path_prefix: Option<String>,
    /// Optional heading substring filter.
    pub heading_contains: Option<String>,
}

/// Result payload for `query-local-memory`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryLocalMemoryResult {
    /// Query text executed against the SQLite memory store.
    pub query: String,
    /// Effective top limit used by the query.
    pub top: usize,
    /// Resolved SQLite memory database path.
    pub memory_db_path: PathBuf,
    /// Number of ranked hits returned.
    pub result_count: usize,
    /// Ranked search hits.
    pub hits: Vec<LocalContextSearchHit>,
}

/// Build or refresh the local context index.
///
/// # Errors
///
/// Returns [`LocalContextCommandError`] when workspace resolution, catalog
/// loading, or index building fails.
pub fn update_local_context_index(
    request: &UpdateLocalContextIndexRequest,
) -> Result<UpdateLocalContextIndexResult, LocalContextCommandError> {
    let current_dir =
        env::current_dir().map_err(|source| LocalContextCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        })?;
    let repo_root = resolve_workspace_root(request.repo_root.as_deref(), Some(&current_dir))
        .map_err(|source| LocalContextCommandError::ResolveWorkspaceRoot { source })?;
    let catalog_info =
        read_local_context_index_catalog(&repo_root, request.catalog_path.as_deref())
            .map_err(|source| LocalContextCommandError::ReadCatalog { source })?;
    let report = build_local_context_index(
        &repo_root,
        &catalog_info,
        request.output_root.as_deref(),
        request.force_full_rebuild,
    )
    .map_err(|source| LocalContextCommandError::BuildIndex { source })?;

    Ok(UpdateLocalContextIndexResult {
        repo_root,
        catalog_path: catalog_info.path,
        index_root: report.index_root,
        index_path: report.index_path,
        memory_root: report.memory_root,
        memory_db_path: report.memory_db_path,
        indexed_file_count: report.indexed_file_count,
        rebuilt_file_count: report.rebuilt_file_count,
        reused_file_count: report.reused_file_count,
        chunk_count: report.document.chunk_count,
    })
}

/// Build or refresh the repository-local SQLite memory snapshot.
///
/// # Errors
///
/// Returns [`LocalContextCommandError`] when workspace resolution, catalog
/// loading, or index building fails.
pub fn update_local_memory(
    request: &UpdateLocalMemoryRequest,
) -> Result<UpdateLocalMemoryResult, LocalContextCommandError> {
    update_local_context_index(request)
}

/// Query the persisted local context index.
///
/// # Errors
///
/// Returns [`LocalContextCommandError`] when workspace resolution, catalog
/// loading, or index loading fails, or when no persisted index exists.
pub fn query_local_context_index(
    request: &QueryLocalContextIndexRequest,
) -> Result<QueryLocalContextIndexResult, LocalContextCommandError> {
    if request.query_text.trim().is_empty() {
        return Err(LocalContextCommandError::EmptyQuery);
    }

    let current_dir =
        env::current_dir().map_err(|source| LocalContextCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        })?;
    let repo_root = resolve_workspace_root(request.repo_root.as_deref(), Some(&current_dir))
        .map_err(|source| LocalContextCommandError::ResolveWorkspaceRoot { source })?;
    let catalog_info =
        read_local_context_index_catalog(&repo_root, request.catalog_path.as_deref())
            .map_err(|source| LocalContextCommandError::ReadCatalog { source })?;
    let index_root = resolve_local_context_index_root(
        &repo_root,
        &catalog_info.catalog,
        request.output_root.as_deref(),
    );
    let index_path = index_root.join("index.json");
    let effective_top = request
        .top
        .unwrap_or(catalog_info.catalog.query_defaults.top)
        .max(1);
    let memory_db_path =
        resolve_local_context_memory_db_path(&repo_root, request.output_root.as_deref());

    let (backend, hits) = if request.use_json_index {
        let index_document = read_local_context_index_document(&index_root)
            .map_err(|source| LocalContextCommandError::ReadIndex { source })?
            .ok_or_else(|| LocalContextCommandError::IndexNotFound {
                index_path: index_path.display().to_string(),
            })?;
        (
            LocalContextQueryBackend::JsonCompatibility,
            search_local_context_index_document(
                &request.query_text,
                &index_document,
                effective_top,
                &request.exclude_paths,
            ),
        )
    } else {
        let hits = search_local_context_sqlite_index(
            &repo_root,
            request.output_root.as_deref(),
            &LocalContextSqliteQueryRequest {
                query_text: request.query_text.clone(),
                top: effective_top,
                exclude_paths: request.exclude_paths.clone(),
                path_prefix: request.path_prefix.clone(),
                heading_contains: request.heading_contains.clone(),
            },
        )
        .map_err(|source| LocalContextCommandError::ReadMemory { source })?
        .ok_or_else(|| LocalContextCommandError::MemoryNotFound {
            db_path: memory_db_path.display().to_string(),
        })?;
        (LocalContextQueryBackend::SqliteDefault, hits)
    };

    Ok(QueryLocalContextIndexResult {
        backend,
        query: request.query_text.clone(),
        top: effective_top,
        index_path,
        memory_db_path,
        result_count: hits.len(),
        hits,
    })
}

/// Query the repository-local SQLite memory snapshot.
///
/// # Errors
///
/// Returns [`LocalContextCommandError`] when workspace resolution or catalog
/// loading fails, or when no SQLite snapshot exists yet.
pub fn query_local_memory(
    request: &QueryLocalMemoryRequest,
) -> Result<QueryLocalMemoryResult, LocalContextCommandError> {
    if request.query_text.trim().is_empty() {
        return Err(LocalContextCommandError::EmptyQuery);
    }

    let current_dir =
        env::current_dir().map_err(|source| LocalContextCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        })?;
    let repo_root = resolve_workspace_root(request.repo_root.as_deref(), Some(&current_dir))
        .map_err(|source| LocalContextCommandError::ResolveWorkspaceRoot { source })?;
    let catalog_info =
        read_local_context_index_catalog(&repo_root, request.catalog_path.as_deref())
            .map_err(|source| LocalContextCommandError::ReadCatalog { source })?;
    let effective_top = request
        .top
        .unwrap_or(catalog_info.catalog.query_defaults.top)
        .max(1);
    let memory_db_path =
        resolve_local_context_memory_db_path(&repo_root, request.output_root.as_deref());
    let hits = search_local_context_sqlite_index(
        &repo_root,
        request.output_root.as_deref(),
        &LocalContextSqliteQueryRequest {
            query_text: request.query_text.clone(),
            top: effective_top,
            exclude_paths: request.exclude_paths.clone(),
            path_prefix: request.path_prefix.clone(),
            heading_contains: request.heading_contains.clone(),
        },
    )
    .map_err(|source| LocalContextCommandError::ReadMemory { source })?
    .ok_or_else(|| LocalContextCommandError::MemoryNotFound {
        db_path: memory_db_path.display().to_string(),
    })?;

    Ok(QueryLocalMemoryResult {
        query: request.query_text.clone(),
        top: effective_top,
        memory_db_path,
        result_count: hits.len(),
        hits,
    })
}
