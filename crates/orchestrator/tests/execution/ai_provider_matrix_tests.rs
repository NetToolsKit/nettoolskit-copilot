//! Tests for the canonical free-provider matrix.

use nettoolskit_orchestrator::{
    classify_ai_free_provider, list_ai_free_provider_matrix_entries,
    list_compatible_ai_free_providers,
};

#[test]
fn test_list_ai_free_provider_matrix_entries_exposes_embedded_catalog() {
    let entries = list_ai_free_provider_matrix_entries();

    assert!(
        entries.len() >= 7,
        "expected the embedded free-provider matrix to expose the documented families"
    );
    assert!(
        entries.iter().any(|entry| entry.family_id == "openrouter"),
        "OpenRouter should exist in the embedded matrix"
    );
    assert!(
        entries
            .iter()
            .any(|entry| entry.family_id == "google-ai-studio"),
        "Google AI Studio should exist in the embedded matrix"
    );
}

#[test]
fn test_classify_ai_free_provider_prefers_endpoint_host_matches() {
    let classification = classify_ai_free_provider(
        "openai-compatible",
        Some("https://openrouter.ai/api/v1/chat/completions"),
    )
    .expect("OpenRouter endpoint should classify");

    assert_eq!(classification.family_id, "openrouter");
    assert_eq!(classification.integration_mode, "gateway/openai-compatible");
}

#[test]
fn test_list_compatible_ai_free_providers_matches_openai_compatible_runtime_mode() {
    let candidates = list_compatible_ai_free_providers(
        Some("gateway/openai-compatible"),
        &["openai-compatible".to_string(), "mock".to_string()],
    );

    assert!(
        candidates
            .iter()
            .any(|entry| entry.family_id == "openrouter"),
        "OpenRouter should be compatible with openai-compatible routing"
    );
    assert!(
        candidates.iter().any(|entry| entry.family_id == "groq"),
        "Groq should be compatible with openai-compatible routing"
    );
    assert!(
        !candidates
            .iter()
            .any(|entry| entry.family_id == "google-ai-studio"),
        "native-only providers should not appear for openai-compatible routing"
    );
}