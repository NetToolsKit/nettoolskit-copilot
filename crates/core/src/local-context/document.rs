//! Document and chunk builders for the local context index.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use super::catalog::{
    local_context_index_file_candidates, resolve_local_context_index_root,
    LocalContextIndexCatalog, LocalContextIndexCatalogInfo,
};

/// Indexed file metadata persisted in the local context document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LocalContextIndexedFile {
    /// Repository-relative forward-slash file path.
    pub path: String,
    /// File content hash in `sha256:<hex>` format.
    pub hash: String,
    /// Last write time in RFC 3339 format.
    pub last_write_time_utc: String,
    /// File size in bytes.
    pub size_bytes: u64,
    /// Human-readable title derived from the file name or first heading.
    pub title: String,
    /// Ordered chunk identifiers emitted for the file.
    pub chunk_ids: Vec<String>,
}

/// Chunk classification persisted in the local context document.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LocalContextChunkKind {
    /// Heading-aware markdown chunk.
    Markdown,
    /// Plain text chunk.
    Text,
}

/// Searchable chunk persisted in the local context document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LocalContextChunk {
    /// Stable chunk identifier.
    pub id: String,
    /// Repository-relative forward-slash file path.
    pub path: String,
    /// Chunk classification.
    pub kind: LocalContextChunkKind,
    /// Optional markdown heading for markdown chunks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<String>,
    /// Canonical excerpt text.
    pub text: String,
    /// Lowercase search text used by lexical retrieval.
    pub search_text: String,
}

/// Persisted local context index document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LocalContextIndexDocument {
    /// Document schema version mirrored from the catalog.
    pub version: u32,
    /// Generation timestamp in RFC 3339 format.
    pub generated_at: String,
    /// Resolved repository root path.
    pub repo_root: String,
    /// Repository-relative catalog path.
    pub catalog_path: String,
    /// Number of chunks persisted in the document.
    pub chunk_count: usize,
    /// Indexed file entries.
    pub files: Vec<LocalContextIndexedFile>,
    /// Indexed chunks.
    pub chunks: Vec<LocalContextChunk>,
}

/// Report returned after building or refreshing the local context index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalContextIndexBuildReport {
    /// Resolved index root directory.
    pub index_root: PathBuf,
    /// Resolved persisted document path.
    pub index_path: PathBuf,
    /// Freshly built document.
    pub document: LocalContextIndexDocument,
    /// Total indexed files.
    pub indexed_file_count: usize,
    /// Number of files rebuilt in this pass.
    pub rebuilt_file_count: usize,
    /// Number of files reused from a prior persisted index.
    pub reused_file_count: usize,
}

/// Build local context chunks for a single repository file.
///
/// # Errors
///
/// Returns an error when the file cannot be read.
pub fn build_local_context_chunks_for_file(
    repo_root: &Path,
    file_path: &Path,
    catalog: &LocalContextIndexCatalog,
) -> Result<Vec<LocalContextChunk>> {
    let relative_path = file_path
        .strip_prefix(repo_root)
        .with_context(|| {
            format!(
                "failed to strip repository prefix '{}' from '{}'",
                repo_root.display(),
                file_path.display()
            )
        })?
        .to_string_lossy()
        .replace('\\', "/");
    let content = String::from_utf8_lossy(
        &fs::read(file_path)
            .with_context(|| format!("failed to read '{}'", file_path.display()))?,
    )
    .to_string();
    let lines: Vec<String> = content.lines().map(ToOwned::to_owned).collect();

    if is_markdown_like_path(&relative_path) {
        Ok(build_markdown_chunks(
            &relative_path,
            &lines,
            catalog.chunking.max_chars,
            catalog.chunking.max_lines,
        ))
    } else {
        Ok(build_text_chunks(
            &relative_path,
            &lines,
            catalog.chunking.max_chars,
            catalog.chunking.max_lines,
        ))
    }
}

/// Build or refresh the full local context index document.
///
/// # Errors
///
/// Returns an error when repository files cannot be read or the index cannot be
/// written.
pub fn build_local_context_index(
    repo_root: &Path,
    catalog_info: &LocalContextIndexCatalogInfo,
    output_root: Option<&Path>,
    force_full_rebuild: bool,
) -> Result<LocalContextIndexBuildReport> {
    let index_root =
        resolve_local_context_index_root(repo_root, &catalog_info.catalog, output_root);
    let existing_index = if force_full_rebuild {
        None
    } else {
        read_local_context_index_document(&index_root)?
    };

    let mut existing_files = HashMap::new();
    let mut existing_chunks = HashMap::new();
    if let Some(existing_index) = &existing_index {
        for file_entry in &existing_index.files {
            existing_files.insert(file_entry.path.clone(), file_entry.clone());
        }
        for chunk_entry in &existing_index.chunks {
            existing_chunks.insert(chunk_entry.id.clone(), chunk_entry.clone());
        }
    }

    let mut file_entries = Vec::new();
    let mut chunk_entries = Vec::new();
    let mut reused_file_count = 0usize;
    let mut rebuilt_file_count = 0usize;

    for file_path in local_context_index_file_candidates(repo_root, &catalog_info.catalog)? {
        let relative_path = file_path
            .strip_prefix(repo_root)
            .with_context(|| {
                format!(
                    "failed to strip repository prefix '{}' from '{}'",
                    repo_root.display(),
                    file_path.display()
                )
            })?
            .to_string_lossy()
            .replace('\\', "/");
        let hash = format!("sha256:{}", compute_sha256_hex(&file_path)?);

        if let Some(existing_file_entry) = existing_files.get(&relative_path) {
            if existing_file_entry.hash == hash {
                reused_file_count += 1;
                file_entries.push(existing_file_entry.clone());
                for chunk_id in &existing_file_entry.chunk_ids {
                    if let Some(existing_chunk) = existing_chunks.get(chunk_id) {
                        chunk_entries.push(existing_chunk.clone());
                    }
                }
                continue;
            }
        }

        rebuilt_file_count += 1;
        let chunks = build_local_context_chunks_for_file(repo_root, &file_path, &catalog_info.catalog)?;
        let chunk_ids = chunks.iter().map(|chunk| chunk.id.clone()).collect::<Vec<_>>();
        let metadata = fs::metadata(&file_path)
            .with_context(|| format!("failed to inspect '{}'", file_path.display()))?;
        let title = derive_local_context_title(&file_path, &relative_path)?;

        chunk_entries.extend(chunks);
        file_entries.push(LocalContextIndexedFile {
            path: relative_path,
            hash,
            last_write_time_utc: format_system_time(metadata.modified().with_context(|| {
                format!("failed to read modification time for '{}'", file_path.display())
            })?)?,
            size_bytes: metadata.len(),
            title,
            chunk_ids,
        });
    }

    file_entries.sort_by(|left, right| left.path.cmp(&right.path));
    chunk_entries.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then_with(|| left.id.cmp(&right.id))
    });

    let repo_root = fs::canonicalize(repo_root)
        .with_context(|| format!("failed to canonicalize '{}'", repo_root.display()))?;
    let catalog_path = catalog_info
        .path
        .strip_prefix(&repo_root)
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| catalog_info.path.to_string_lossy().replace('\\', "/"));

    let document = LocalContextIndexDocument {
        version: catalog_info.catalog.version,
        generated_at: now_rfc3339()?,
        repo_root: repo_root.to_string_lossy().to_string(),
        catalog_path,
        chunk_count: chunk_entries.len(),
        files: file_entries,
        chunks: chunk_entries,
    };
    let index_path = write_local_context_index_document(&index_root, &document)?;

    Ok(LocalContextIndexBuildReport {
        index_root,
        index_path,
        indexed_file_count: document.files.len(),
        rebuilt_file_count,
        reused_file_count,
        document,
    })
}

/// Read a persisted local context index document when it exists.
///
/// Invalid documents are treated as missing so callers can rebuild safely.
///
/// # Errors
///
/// Returns an error only when the underlying file cannot be read.
pub fn read_local_context_index_document(
    index_root: &Path,
) -> Result<Option<LocalContextIndexDocument>> {
    let index_path = index_root.join("index.json");
    if !index_path.is_file() {
        return Ok(None);
    }

    let payload = fs::read_to_string(&index_path)
        .with_context(|| format!("failed to read '{}'", index_path.display()))?;
    match serde_json::from_str::<LocalContextIndexDocument>(&payload) {
        Ok(document) => Ok(Some(document)),
        Err(_) => Ok(None),
    }
}

/// Persist a local context index document.
///
/// # Errors
///
/// Returns an error when the output directory cannot be created or the
/// document cannot be written.
pub fn write_local_context_index_document(
    index_root: &Path,
    document: &LocalContextIndexDocument,
) -> Result<PathBuf> {
    fs::create_dir_all(index_root)
        .with_context(|| format!("failed to create index directory '{}'", index_root.display()))?;
    let index_path = index_root.join("index.json");
    let payload = serde_json::to_string_pretty(document)
        .context("failed to serialize local context index document")?;
    fs::write(&index_path, payload)
        .with_context(|| format!("failed to write '{}'", index_path.display()))?;

    Ok(index_path)
}

fn build_markdown_chunks(
    relative_path: &str,
    lines: &[String],
    max_chars: usize,
    max_lines: usize,
) -> Vec<LocalContextChunk> {
    let mut chunks = Vec::new();
    let mut current_heading = String::new();
    let mut buffer = Vec::<String>::new();
    let mut chunk_index = 0usize;

    for line in lines {
        if let Some(heading) = markdown_heading(line) {
            if let Some(chunk) =
                flush_markdown_chunk(relative_path, &current_heading, &buffer, chunk_index)
            {
                chunks.push(chunk);
                chunk_index += 1;
            }

            current_heading = heading;
            buffer.clear();
            continue;
        }

        buffer.push(line.clone());
        let current_text = normalize_context_text(&buffer.join("\n"));
        if buffer.len() >= max_lines || current_text.len() >= max_chars {
            if let Some(chunk) =
                flush_markdown_chunk(relative_path, &current_heading, &buffer, chunk_index)
            {
                chunks.push(chunk);
                chunk_index += 1;
            }
            buffer.clear();
        }
    }

    if let Some(chunk) = flush_markdown_chunk(relative_path, &current_heading, &buffer, chunk_index)
    {
        chunks.push(chunk);
    }

    chunks
}

fn build_text_chunks(
    relative_path: &str,
    lines: &[String],
    max_chars: usize,
    max_lines: usize,
) -> Vec<LocalContextChunk> {
    let mut chunks = Vec::new();
    let mut buffer = Vec::<String>::new();
    let mut chunk_index = 0usize;

    for line in lines {
        buffer.push(line.clone());
        let current_text = normalize_context_text(&buffer.join("\n"));
        if buffer.len() >= max_lines || current_text.len() >= max_chars {
            if let Some(chunk) = flush_text_chunk(relative_path, &buffer, chunk_index) {
                chunks.push(chunk);
                chunk_index += 1;
            }
            buffer.clear();
        }
    }

    if let Some(chunk) = flush_text_chunk(relative_path, &buffer, chunk_index) {
        chunks.push(chunk);
    }

    chunks
}

fn flush_markdown_chunk(
    relative_path: &str,
    heading: &str,
    buffer: &[String],
    chunk_index: usize,
) -> Option<LocalContextChunk> {
    if buffer.is_empty() {
        return None;
    }

    let text = normalize_context_text(&buffer.join("\n"));
    if text.is_empty() {
        return None;
    }

    let heading = heading.trim();
    Some(LocalContextChunk {
        id: format!("{relative_path}::{chunk_index}"),
        path: relative_path.to_string(),
        kind: LocalContextChunkKind::Markdown,
        heading: if heading.is_empty() {
            None
        } else {
            Some(heading.to_string())
        },
        search_text: normalize_context_text(&format!("{heading} {text}").to_lowercase()),
        text,
    })
}

fn flush_text_chunk(
    relative_path: &str,
    buffer: &[String],
    chunk_index: usize,
) -> Option<LocalContextChunk> {
    if buffer.is_empty() {
        return None;
    }

    let text = normalize_context_text(&buffer.join("\n"));
    if text.is_empty() {
        return None;
    }

    Some(LocalContextChunk {
        id: format!("{relative_path}::{chunk_index}"),
        path: relative_path.to_string(),
        kind: LocalContextChunkKind::Text,
        heading: None,
        search_text: normalize_context_text(&format!("{relative_path} {text}").to_lowercase()),
        text,
    })
}

fn markdown_heading(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let hashes = trimmed.chars().take_while(|character| *character == '#').count();
    if !(1..=6).contains(&hashes) {
        return None;
    }

    let heading = trimmed[hashes..].trim();
    if heading.is_empty() {
        None
    } else {
        Some(heading.to_string())
    }
}

fn is_markdown_like_path(path: &str) -> bool {
    matches!(
        Path::new(path)
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.to_ascii_lowercase()),
        Some(extension) if extension == "md" || extension == "markdown"
    )
}

fn normalize_context_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn derive_local_context_title(file_path: &Path, relative_path: &str) -> Result<String> {
    if is_markdown_like_path(relative_path) {
        let content = String::from_utf8_lossy(
            &fs::read(file_path)
                .with_context(|| format!("failed to read '{}'", file_path.display()))?,
        )
        .to_string();
        for line in content.lines() {
            if let Some(heading) = markdown_heading(line) {
                return Ok(heading);
            }
        }
    }

    Ok(file_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_string())
}

fn compute_sha256_hex(path: &Path) -> Result<String> {
    let bytes =
        fs::read(path).with_context(|| format!("failed to read '{}' for hashing", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

fn now_rfc3339() -> Result<String> {
    Ok(OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .context("failed to format generation timestamp")?)
}

fn format_system_time(system_time: std::time::SystemTime) -> Result<String> {
    Ok(OffsetDateTime::from(system_time)
        .format(&Rfc3339)
        .context("failed to format file timestamp")?)
}