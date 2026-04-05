//! Tests for normalized AI provider adapter contracts.

use nettoolskit_orchestrator::{
    ai_provider_adapter_descriptor_for_id, mock_ai_provider_adapter_descriptor,
    openai_compatible_provider_adapter_descriptor, AiProviderAuthKind, AiProviderTransportKind,
};

#[test]
fn test_mock_adapter_descriptor_is_local_and_authless() {
    let descriptor = mock_ai_provider_adapter_descriptor();

    assert_eq!(descriptor.provider_id, "mock");
    assert_eq!(descriptor.transport, AiProviderTransportKind::LocalMock);
    assert_eq!(descriptor.auth, AiProviderAuthKind::None);
    assert!(descriptor.supports_streaming);
    assert!(!descriptor.supports_usage_reporting);
    assert!(!descriptor.supports_fallback_output);
}

#[test]
fn test_openai_compatible_adapter_descriptor_reports_expected_capabilities() {
    let descriptor = openai_compatible_provider_adapter_descriptor();

    assert_eq!(descriptor.provider_id, "openai-compatible");
    assert_eq!(
        descriptor.transport,
        AiProviderTransportKind::OpenAiCompatibleChat
    );
    assert_eq!(descriptor.auth, AiProviderAuthKind::BearerApiKey);
    assert!(descriptor.supports_streaming);
    assert!(descriptor.supports_usage_reporting);
    assert!(descriptor.supports_fallback_output);
}

#[test]
fn test_ai_provider_adapter_descriptor_for_id_is_case_insensitive() {
    let descriptor = ai_provider_adapter_descriptor_for_id("OPENAI")
        .expect("openai-compatible descriptor should resolve");

    assert_eq!(descriptor.provider_id, "openai-compatible");
}