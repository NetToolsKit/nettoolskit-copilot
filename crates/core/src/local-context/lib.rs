//! Repository-owned local context index utilities.
//!
//! This module provides the deterministic local-first indexing and retrieval
//! primitives that replace the PowerShell local context index helpers.

mod catalog;
mod document;
mod search;
mod sqlite;

pub use catalog::{
    local_context_index_file_candidates, local_context_index_path_included,
    read_local_context_index_catalog, resolve_local_context_index_catalog_path,
    resolve_local_context_index_root, LocalContextIndexCatalog, LocalContextIndexCatalogInfo,
    LocalContextIndexChunking, LocalContextIndexQueryDefaults,
};
pub use document::{
    build_local_context_chunks_for_file, build_local_context_index,
    read_local_context_index_document, write_local_context_index_document, LocalContextChunk,
    LocalContextChunkKind, LocalContextIndexBuildReport, LocalContextIndexDocument,
    LocalContextIndexedFile,
};
pub use search::{search_local_context_index_document, LocalContextSearchHit};
pub use sqlite::{
    initialize_local_context_memory_store, prune_local_context_memory_events,
    record_local_context_memory_event, resolve_local_context_memory_db_path,
    resolve_local_context_memory_paths, resolve_local_context_memory_root,
    search_local_context_sqlite_index, upsert_local_context_memory_session,
    write_local_context_sqlite_index, LocalContextMemoryEventRecord, LocalContextMemoryPaths,
    LocalContextMemorySchemaReport, LocalContextMemorySessionRecord,
    LocalContextSqliteQueryRequest, LocalContextSqliteWriteReport,
    LOCAL_CONTEXT_MEMORY_DB_FILE_NAME, LOCAL_CONTEXT_MEMORY_DIR_NAME,
    LOCAL_CONTEXT_MEMORY_SCHEMA_VERSION,
};
