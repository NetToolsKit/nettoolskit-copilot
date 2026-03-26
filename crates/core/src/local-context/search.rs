//! Deterministic lexical retrieval for the local context index.

use regex::Regex;
use std::collections::{BTreeSet, HashSet};
use std::sync::OnceLock;

use super::document::LocalContextIndexDocument;

/// Ranked search hit returned from the local context index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalContextSearchHit {
    /// Stable chunk identifier.
    pub id: String,
    /// Repository-relative forward-slash file path.
    pub path: String,
    /// Optional markdown heading.
    pub heading: Option<String>,
    /// Deterministic lexical score.
    pub score: i32,
    /// Chunk excerpt.
    pub excerpt: String,
}

/// Search an in-memory local context index document.
#[must_use]
pub fn search_local_context_index_document(
    query_text: &str,
    index_document: &LocalContextIndexDocument,
    top: usize,
    exclude_paths: &[String],
) -> Vec<LocalContextSearchHit> {
    if query_text.trim().is_empty() {
        return Vec::new();
    }

    let effective_top = top.max(1);
    let excluded_paths = exclude_paths
        .iter()
        .map(|path| path.replace('\\', "/"))
        .collect::<HashSet<_>>();

    let mut hits = index_document
        .chunks
        .iter()
        .filter_map(|chunk| {
            let chunk_path = chunk.path.replace('\\', "/");
            if excluded_paths.contains(&chunk_path) {
                return None;
            }

            let score = local_context_chunk_score(query_text, &chunk_path, chunk.heading.as_deref(), &chunk.search_text);
            (score > 0).then(|| LocalContextSearchHit {
                id: chunk.id.clone(),
                path: chunk_path,
                heading: chunk.heading.clone(),
                score,
                excerpt: chunk.text.clone(),
            })
        })
        .collect::<Vec<_>>();

    hits.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.path.cmp(&right.path))
    });
    hits.truncate(effective_top);
    hits
}

fn local_context_chunk_score(
    query_text: &str,
    path: &str,
    heading: Option<&str>,
    search_text: &str,
) -> i32 {
    let mut score = 0i32;
    let query_lower = query_text.to_ascii_lowercase();
    let path_lower = path.to_ascii_lowercase();
    let heading_lower = heading.unwrap_or_default().to_ascii_lowercase();
    let search_text = search_text.to_ascii_lowercase();

    if path_lower.contains(&query_lower) {
        score += 24;
    }

    if !heading_lower.is_empty() && heading_lower.contains(&query_lower) {
        score += 18;
    }

    for term in local_context_search_terms(query_text) {
        if path_lower.contains(&term) {
            score += 10;
        }

        if !heading_lower.is_empty() && heading_lower.contains(&term) {
            score += 8;
        }

        let match_count = search_text.match_indices(&term).count();
        if match_count > 0 {
            score += (match_count as i32 * 2).min(12);
        }
    }

    score
}

fn local_context_search_terms(text: &str) -> BTreeSet<String> {
    static TOKEN_REGEX: OnceLock<Regex> = OnceLock::new();

    TOKEN_REGEX
        .get_or_init(|| {
            Regex::new(r"[a-z0-9][a-z0-9._/-]{1,63}")
                .expect("local context token regex should be valid")
        })
        .find_iter(&text.to_ascii_lowercase())
        .map(|capture| capture.as_str().to_string())
        .filter(|term| term.len() >= 2)
        .collect()
}