//! Canonical offline/live harness descriptors for the free-provider matrix.
//!
//! The harness catalog keeps prompt fixtures and output contracts reusable
//! across provider families so live smoke checks can stay opt-in while
//! deterministic validation remains available in-repo.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Embedded document that stores the canonical free-provider harness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiFreeProviderHarnessDocument {
    /// Schema version for the embedded harness.
    pub version: u32,
    /// Shared prompt fixtures reused across provider cases.
    pub prompt_fixtures: Vec<AiFreeProviderHarnessPromptFixture>,
    /// Shared output contracts reused across provider cases.
    pub output_contracts: Vec<AiFreeProviderHarnessOutputContract>,
    /// Provider-family harness cases.
    pub provider_cases: Vec<AiFreeProviderHarnessCase>,
}

/// One shared prompt fixture for free-provider harness runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiFreeProviderHarnessPromptFixture {
    /// Stable prompt identifier.
    pub prompt_id: String,
    /// Short human-readable title.
    pub title: String,
    /// High-level request intent.
    pub intent: String,
    /// Prompt payload used by harness runs.
    pub prompt: String,
}

/// One reusable output contract for harness validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiFreeProviderHarnessOutputContract {
    /// Stable contract identifier.
    pub contract_id: String,
    /// Short human-readable title.
    pub title: String,
    /// Required markers that must appear in output to satisfy the contract.
    #[serde(default)]
    pub required_markers: Vec<String>,
}

/// Network mode for one provider-family harness case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiFreeProviderHarnessNetworkMode {
    /// Deterministic offline coverage using a local/mock adapter.
    OfflineDeterministic,
    /// Live-provider smoke checks that must be explicitly enabled by the operator.
    LiveOptIn,
}

/// One provider-family harness case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiFreeProviderHarnessCase {
    /// Stable provider-family id.
    pub family_id: String,
    /// Built-in profile to use when exercising the case.
    pub profile_id: String,
    /// Offline or live-opt-in network mode.
    pub network_mode: AiFreeProviderHarnessNetworkMode,
    /// Shared prompt fixture id.
    pub prompt_id: String,
    /// Shared output-contract id.
    pub output_contract_id: String,
    /// Latency budget expected for the live case.
    pub latency_budget_ms: u64,
    /// Whether streaming should be exercised for this case.
    pub supports_streaming: bool,
    /// Whether the case must include an error-path check.
    pub supports_error_path: bool,
    /// Short operator-facing notes.
    pub notes: String,
}

const EMBEDDED_FREE_PROVIDER_HARNESS: &str = include_str!(
    "../../../../definitions/templates/manifests/free-llm-provider-harness.catalog.json"
);

static FREE_PROVIDER_HARNESS: OnceLock<AiFreeProviderHarnessDocument> = OnceLock::new();

/// Return the embedded free-provider harness document.
#[must_use]
pub fn embedded_ai_free_provider_harness() -> &'static AiFreeProviderHarnessDocument {
    FREE_PROVIDER_HARNESS.get_or_init(load_embedded_free_provider_harness)
}

/// Find one provider-family harness case by family id.
#[must_use]
pub fn find_ai_free_provider_harness_case(family_id: &str) -> Option<AiFreeProviderHarnessCase> {
    let normalized_family_id = normalize_token(family_id);

    embedded_ai_free_provider_harness()
        .provider_cases
        .iter()
        .find(|entry| normalize_token(&entry.family_id) == normalized_family_id)
        .cloned()
}

/// Find one shared prompt fixture by id.
#[must_use]
pub fn find_ai_free_provider_harness_prompt(
    prompt_id: &str,
) -> Option<AiFreeProviderHarnessPromptFixture> {
    let normalized_prompt_id = normalize_token(prompt_id);

    embedded_ai_free_provider_harness()
        .prompt_fixtures
        .iter()
        .find(|entry| normalize_token(&entry.prompt_id) == normalized_prompt_id)
        .cloned()
}

/// Find one reusable output contract by id.
#[must_use]
pub fn find_ai_free_provider_harness_output_contract(
    contract_id: &str,
) -> Option<AiFreeProviderHarnessOutputContract> {
    let normalized_contract_id = normalize_token(contract_id);

    embedded_ai_free_provider_harness()
        .output_contracts
        .iter()
        .find(|entry| normalize_token(&entry.contract_id) == normalized_contract_id)
        .cloned()
}

/// Validate output text against one reusable harness contract.
///
/// # Errors
///
/// Returns an error when the contract is unknown or when one or more required
/// markers are missing from the output.
pub fn validate_ai_free_provider_harness_output(
    contract_id: &str,
    output: &str,
) -> Result<(), String> {
    let contract = find_ai_free_provider_harness_output_contract(contract_id)
        .ok_or_else(|| format!("unknown free-provider harness output contract `{contract_id}`"))?;

    let missing_markers = contract
        .required_markers
        .iter()
        .filter(|marker| !output.contains(marker.as_str()))
        .cloned()
        .collect::<Vec<_>>();

    if missing_markers.is_empty() {
        return Ok(());
    }

    Err(format!(
        "output does not satisfy contract `{}`; missing markers: {}",
        contract.contract_id,
        missing_markers.join(", ")
    ))
}

fn load_embedded_free_provider_harness() -> AiFreeProviderHarnessDocument {
    serde_json::from_str::<AiFreeProviderHarnessDocument>(EMBEDDED_FREE_PROVIDER_HARNESS)
        .unwrap_or_else(|error| panic!("invalid embedded free-provider harness: {error}"))
}

fn normalize_token(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}