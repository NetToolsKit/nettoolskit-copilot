//! Canonical free-provider matrix for AI runtime evaluation and reporting.
//!
//! The matrix captures operator-facing assumptions for free/provider-preview
//! surfaces without coupling orchestration to one vendor implementation.

use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Embedded document that stores the canonical free-provider matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiFreeProviderMatrixDocument {
    /// Schema version for the embedded matrix.
    pub version: u32,
    /// Declared free/provider-preview families.
    pub providers: Vec<AiFreeProviderCatalogEntry>,
}

/// One free-provider family classification entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiFreeProviderCatalogEntry {
    /// Stable family id.
    pub family_id: String,
    /// Human-readable title.
    pub title: String,
    /// High-level platform grouping such as gateway, api, infra, or orchestrator.
    pub platform_type: String,
    /// Integration-mode summary used in diagnostics and docs.
    pub integration_mode: String,
    /// Operator-facing support tier.
    pub support_tier: String,
    /// Short stability label.
    pub stability_label: String,
    /// Runtime compatibility tags used to derive compatible free-provider families.
    #[serde(default)]
    pub compatibility_tags: Vec<String>,
    /// Aliases that can map persisted provider ids to this family.
    #[serde(default)]
    pub provider_aliases: Vec<String>,
    /// Endpoint hosts that can map live configuration to this family.
    #[serde(default)]
    pub endpoint_hosts: Vec<String>,
    /// Estimated free-tier quota hint.
    pub quota_hint: String,
    /// Short operator caveat or note.
    pub operator_note: String,
}

const EMBEDDED_FREE_PROVIDER_MATRIX: &str = include_str!(
    "../../../../definitions/templates/manifests/free-llm-provider-matrix.catalog.json"
);

static FREE_PROVIDER_MATRIX: OnceLock<Vec<AiFreeProviderCatalogEntry>> = OnceLock::new();

/// Return the immutable embedded free-provider matrix.
#[must_use]
pub fn list_ai_free_provider_matrix_entries() -> &'static [AiFreeProviderCatalogEntry] {
    FREE_PROVIDER_MATRIX
        .get_or_init(load_embedded_free_provider_matrix)
        .as_slice()
}

/// Classify one provider id and/or endpoint against the embedded free-provider matrix.
#[must_use]
pub fn classify_ai_free_provider(
    provider_id: &str,
    endpoint: Option<&str>,
) -> Option<AiFreeProviderCatalogEntry> {
    let normalized_provider_id = normalize_token(provider_id);
    let normalized_host = endpoint.and_then(normalize_endpoint_host);

    list_ai_free_provider_matrix_entries()
        .iter()
        .find(|entry| {
            normalized_host.as_ref().is_some_and(|host| {
                entry
                    .endpoint_hosts
                    .iter()
                    .map(|value| normalize_token(value))
                    .any(|candidate| host == &candidate || host.ends_with(&candidate))
            }) || entry
                .provider_aliases
                .iter()
                .map(|value| normalize_token(value))
                .any(|candidate| candidate == normalized_provider_id)
        })
        .cloned()
}

/// List free-provider families compatible with a runtime mode/provider chain.
#[must_use]
pub fn list_compatible_ai_free_providers(
    provider_mode: Option<&str>,
    provider_chain: &[String],
) -> Vec<AiFreeProviderCatalogEntry> {
    let normalized_mode = provider_mode.map(normalize_token);
    let normalized_chain = provider_chain
        .iter()
        .map(|value| normalize_token(value))
        .collect::<Vec<_>>();

    list_ai_free_provider_matrix_entries()
        .iter()
        .filter(|entry| {
            entry.compatibility_tags.iter().any(|tag| {
                let normalized_tag = normalize_token(tag);
                normalized_mode
                    .as_ref()
                    .is_some_and(|mode| mode.contains(&normalized_tag))
                    || normalized_chain.iter().any(|provider_id| {
                        provider_id.contains(&normalized_tag)
                            || (normalized_tag == "openai-compatible" && provider_id == "openai")
                    })
            })
        })
        .cloned()
        .collect()
}

fn load_embedded_free_provider_matrix() -> Vec<AiFreeProviderCatalogEntry> {
    let document =
        serde_json::from_str::<AiFreeProviderMatrixDocument>(EMBEDDED_FREE_PROVIDER_MATRIX)
            .unwrap_or_else(|error| panic!("invalid embedded free-provider matrix: {error}"));
    document.providers
}

fn normalize_endpoint_host(endpoint: &str) -> Option<String> {
    let trimmed = endpoint.trim();
    if trimmed.is_empty() {
        return None;
    }

    Url::parse(trimmed)
        .ok()
        .and_then(|url| url.host_str().map(normalize_token))
        .or_else(|| Some(normalize_token(trimmed)))
}

fn normalize_token(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}