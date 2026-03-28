//! Repository-owned local context index utilities.
//!
//! This module provides the deterministic local-first indexing and retrieval
//! primitives that replace the PowerShell local context index helpers.

mod catalog;
mod document;
mod search;

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
